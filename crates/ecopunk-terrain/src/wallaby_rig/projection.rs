use super::{add, admit_model, filled, mul, RigLimits, TangentPolicy};
use crate::swamp_wallaby_anatomical_r3::Candidate;
use qualia_core_db::{
    container_10d::{
        skeletal_container::{self, SkeletalContainerError},
        skeletal_section::{self, ClipRecord, SkeletalData, SkeletalSection},
        SectionTableError,
    },
    q_hash,
    render::skeletal::{
        authoring::remap_skin_weights_into, Joint, Key, Motor, SkinWeight, Track, MAX_JOINTS,
        MAX_VERTICES,
    },
};
use sha2::{Digest, Sha256};

pub(crate) struct PreparedRig {
    pub metadata: serde_json::Value,
    payload: Vec<u8>,
    vertices: usize,
    rest_clip: u64,
}

/// `lineage` is the existing concatenated per-material render-to-skin mapping,
/// in exactly the order consumed by compile_review_parts. It is not a weld.
pub(crate) fn prepare(
    candidate: &Candidate,
    lineage: &[u32],
    source_sha256: &str,
    limits: RigLimits,
) -> Result<PreparedRig, String> {
    let source = &candidate.source;
    let count = source.joints.len();
    let n = lineage.len();
    let original = candidate.skin.positions.len();
    if source.identifier.is_empty()
        || source.identifier.len() > 1024
        || source_sha256.len() != 64
        || !source_sha256.bytes().all(|b| b.is_ascii_hexdigit())
        || source.joints.iter().any(|j| j.id.len() > 1024)
        || count == 0
        || count > limits.max_joints.min(MAX_JOINTS)
        || n == 0
        || n > limits.max_vertices.min(MAX_VERTICES)
        || original > 750_000
        || candidate.skin.joint_indices.len() != original
        || candidate.skin.joint_weights.len() != original
        || lineage.iter().any(|&v| v as usize >= original)
    {
        return Err("Wallaby rig source/lineage count refused".into());
    }
    let motion_limits = crate::animal_motion::MotionLimits::default();
    let motion_reservation = if source.motion.is_some() {
        motion_limits.cold_bytes
    } else {
        0
    };
    let counts = skeletal_section::Counts {
        joints: count,
        vertices: n,
        clips: 1,
        tracks: count,
        keys: count,
    };
    let payload_bytes = counts
        .encoded_len()
        .ok_or("Wallaby rig exceeds canonical SKL1 counts")?;
    let cold = add(
        add(mul(original, 24)?, mul(n, 24)?)?,
        add(payload_bytes, mul(count, 2048)?)?,
    )?;
    if add(cold, motion_reservation)? > limits.cold_bytes {
        return Err("Wallaby rig preparation exceeds caller cold byte budget".into());
    }
    let scope = format!("{:x}", Sha256::digest(source.identifier.as_bytes()));
    let rest_uri = format!("urn:ecopunk:authored-rig:{scope}:static-rest-hold:v1");
    let rest_clip = q_hash(&rest_uri);
    let identity = Motor::identity();
    let mut joints = filled(
        count,
        Joint {
            parent: None,
            rest_local: identity,
            inverse_bind: identity,
        },
    )?;
    let mut ids = filled(count, 0u64)?;
    let mut tracks = filled(
        count,
        Track {
            joint: 0,
            first_key: 0,
            key_count: 1,
        },
    )?;
    let mut keys = filled(
        count,
        Key {
            time: 0.,
            local: identity,
        },
    )?;
    let mut mappings = Vec::new();
    for (i, j) in source.joints.iter().enumerate() {
        if j.parent.is_some_and(|p| p >= i)
            || j.parent.is_none() != (i == 0)
            || j.id.is_empty()
            || j.position_m.iter().any(|v| !v.is_finite() || v.abs() > 3.)
        {
            return Err("Wallaby rig invalid authored joint hierarchy/frame".into());
        }
        // Widen BEFORE subtracting: local translations compose to the retained
        // f32 source-world landmarks in the same f64 Motor algebra as Core.
        let world = j.position_m.map(f64::from);
        let parent = j
            .parent
            .map(|p| source.joints[p].position_m.map(f64::from))
            .unwrap_or([0.; 3]);
        let local = std::array::from_fn::<_, 3, _>(|a| world[a] - parent[a]);
        joints[i] = Joint {
            parent: j.parent.map(|p| p as u16),
            rest_local: Motor::from_rotation_translation([1., 0., 0., 0.], local),
            inverse_bind: Motor::from_rotation_translation([1., 0., 0., 0.], world.map(|v| -v)),
        };
        let uri = format!(
            "urn:ecopunk:authored-rig:{scope}:joint:{:x}",
            Sha256::digest(j.id.as_bytes())
        );
        ids[i] = q_hash(&uri);
        if ids[i] == 0 || ids[..i].contains(&ids[i]) {
            return Err("Wallaby rig scoped joint identifier collision".into());
        }
        tracks[i] = Track {
            joint: i as u16,
            first_key: i as u32,
            key_count: 1,
        };
        keys[i] = Key {
            time: 0.,
            local: joints[i].rest_local,
        };
        mappings.push(serde_json::json!({"source_index":i,"source_id":j.id,"render_uri":uri,"render_id":ids[i],
            "source_parent":j.parent,"source_world_position_m":j.position_m}));
    }
    let zero = SkinWeight {
        joints: [0; 4],
        weights: [1., 0., 0., 0.],
    };
    let mut original_skin = filled(original, zero)?;
    for (i, out) in original_skin.iter_mut().enumerate() {
        *out = SkinWeight {
            joints: candidate.skin.joint_indices[i],
            weights: candidate.skin.joint_weights[i],
        };
    }
    let mut skin = filled(n, zero)?;
    remap_skin_weights_into(&original_skin, lineage, &mut skin)
        .map_err(|e| format!("Wallaby exact weight remap: {e:?}"))?;
    // No renormalization, nearest-joint reassignment, or dropped influences.
    let mut clips = vec![ClipRecord {
        id: rest_clip,
        duration: 1.,
        first_track: 0,
        track_count: count as u32,
        looping: false,
    }];
    let motion = crate::animal_motion::compile(
        source,
        &joints,
        tracks.len(),
        keys.len(),
        motion_limits,
        &std::sync::atomic::AtomicBool::new(false),
    )?;
    clips
        .try_reserve_exact(motion.clips.len())
        .map_err(|_| "Motion clip allocation refused")?;
    tracks
        .try_reserve_exact(motion.tracks.len())
        .map_err(|_| "Motion track allocation refused")?;
    keys.try_reserve_exact(motion.keys.len())
        .map_err(|_| "Motion key allocation refused")?;
    clips.extend_from_slice(&motion.clips);
    tracks.extend_from_slice(&motion.tracks);
    keys.extend_from_slice(&motion.keys);
    let payload_bytes = skeletal_section::Counts {
        joints: count,
        vertices: n,
        clips: clips.len(),
        tracks: tracks.len(),
        keys: keys.len(),
    }
    .encoded_len()
    .ok_or("Authored motion exceeds canonical SKL1 counts")?;
    if add(cold, add(motion_reservation, payload_bytes)?)? > limits.cold_bytes {
        return Err("Authored motion rig cold reservation exhausted".into());
    }
    let data = SkeletalData {
        joints: &joints,
        joint_ids: &ids,
        skin: &skin,
        clips: &clips,
        tracks: &tracks,
        keys: &keys,
    };
    let mut payload = filled(payload_bytes, 0u8)?;
    let written = skeletal_section::encode_skeletal_section_into(&data, &mut payload)
        .map_err(|e| format!("Wallaby SKL1 admission: {e:?}"))?;
    payload.truncate(written);
    let parsed =
        SkeletalSection::parse(&payload, n).map_err(|e| format!("Wallaby SKL1 readback: {e:?}"))?;
    for (i, w) in skin.iter().enumerate() {
        let decoded = parsed.skin(i).ok_or("Wallaby SKL1 weight row absent")?;
        if decoded.joints != w.joints || decoded.weights != w.weights {
            return Err("Wallaby SKL1 changed authored influences".into());
        }
    }
    let metadata = serde_json::json!({"schema":"ecopunk.wallaby-rest-rig/v1","resource":"review/model.10d",
        "codec":"SKL1","source_sha256":source_sha256,"source_identifier":source.identifier,
        "joint_mapping":mappings,"render_vertices":n,"render_weight_mapping":"exact existing per-material source_skin_vertices order",
        "basis":"world-aligned identity joint rotation; authored landmark translations; not anatomical joint axes",
        "rest_clip":{"id":rest_clip,"uri":rest_uri,"duration_seconds":1.0,"keys_per_joint":1,"looping":false,
            "meaning":"format-required static hold; no biological motion or measured duration"},
        "authored_motion":motion.metadata,"scope":"external skin including authored coat; osteology/myology remain retained anatomy resources",
        "deformation":"existing Core rigid-joint linear blend skinning; authored weights unchanged",
        "tangent_policy":"explicit runtime regeneration from retained UV0 and decoded bind/pose frames; stored resource tangents retained but not preserved by player",
        "acceptance":"technical candidate only; bends/contact/groom/biological and visual review incomplete"});
    Ok(PreparedRig {
        metadata,
        payload,
        vertices: n,
        rest_clip,
    })
}

/// Existing native attachment preserves geometry, normal/UV/material sections.
/// Admission uses the actual quantized bind mesh, not pre-encode source positions.
pub(crate) fn attach(
    bytes: &[u8],
    rig: &PreparedRig,
    limits: RigLimits,
) -> Result<Vec<u8>, String> {
    if bytes.len() > limits.max_container_bytes {
        return Err("Wallaby .10d exceeds caller container cap".into());
    }
    let needed =
        match skeletal_container::attach_skeletal_section_into(bytes, &rig.payload, &mut []) {
            Err(SkeletalContainerError::Sections(SectionTableError::OutputBufferTooSmall {
                needed,
                ..
            })) => needed,
            Ok(n) => n,
            Err(e) => return Err(format!("Wallaby SKL1 attachment: {e:?}")),
        };
    if needed > limits.max_container_bytes
        || add(add(bytes.len(), rig.payload.len())?, needed)? > limits.cold_bytes
    {
        return Err("Wallaby rig attachment exceeds caller byte budget".into());
    }
    let mut out = filled(needed, 0u8)?;
    let written = skeletal_container::attach_skeletal_section_into(bytes, &rig.payload, &mut out)
        .map_err(|e| format!("Wallaby SKL1 attachment: {e:?}"))?;
    out.truncate(written);
    let mut admission_limits = limits;
    admission_limits.cold_bytes = limits
        .cold_bytes
        .checked_sub(add(bytes.len(), rig.payload.len())?)
        .ok_or("Wallaby rig attachment reservation exhausted")?;
    let player = admit_model(
        &out,
        admission_limits,
        TangentPolicy::RegenerateFromRetainedUv,
    )?;
    if player.positions().len() != rig.vertices || !player.has_clip(rig.rest_clip) {
        return Err("Wallaby player lost rest rig correspondence".into());
    }
    Ok(out)
}
