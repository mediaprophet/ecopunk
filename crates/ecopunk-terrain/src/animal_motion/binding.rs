//! Source-to-canonical clip binding. Kept separate from the allocation-free sampler.
use super::*;
use crate::{
    swamp_wallaby_anatomical_r3::Source,
    wallaby_rig::{self, RigLimits, TangentPolicy},
};
use qualia_core_db::{
    container_10d::skeletal_container::read_skeletal_section,
    q_hash,
    render::skeletal::{Joint, Motor},
};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
pub(super) fn joint_id(scope: &str, id: &str) -> u64 {
    q_hash(&format!(
        "urn:ecopunk:authored-rig:{:x}:joint:{:x}",
        Sha256::digest(scope.as_bytes()),
        Sha256::digest(id.as_bytes())
    ))
}
pub(super) fn admit(
    model: &[u8],
    source: &Source,
    rig_limits: RigLimits,
    limits: MotionLimits,
    cancel: &AtomicBool,
) -> Result<MotionPlayer, String> {
    if model.len() > rig_limits.max_container_bytes.min(42 * 1024 * 1024) {
        return Err("Motion model exceeds container budget".into());
    }
    if cancel.load(Ordering::Acquire) {
        return Err("Motion admission cancelled".into());
    }
    let motion = source
        .motion
        .as_ref()
        .ok_or("No authored motion in source")?;
    validation::validate(source, motion, limits)?;
    let section = read_skeletal_section(model).map_err(|e| format!("Motion SKL1 read: {e:?}"))?;
    let counts = section.counts();
    if counts.vertices > limits.max_vertices || counts.joints != source.joints.len() {
        return Err("Motion source/model counts differ".into());
    }
    let mut joints = Vec::new();
    let mut ids = Vec::new();
    joints
        .try_reserve_exact(counts.joints)
        .map_err(|_| "Motion joint allocation refused")?;
    ids.try_reserve_exact(counts.joints)
        .map_err(|_| "Motion identifier allocation refused")?;
    for (i, j) in source.joints.iter().enumerate() {
        let world = j.position_m.map(f64::from);
        let parent = j
            .parent
            .map(|p| source.joints[p].position_m.map(f64::from))
            .unwrap_or([0.; 3]);
        let expected = Joint {
            parent: j.parent.map(|v| v as u16),
            rest_local: Motor::from_rotation_translation(
                [1., 0., 0., 0.],
                std::array::from_fn(|a| world[a] - parent[a]),
            ),
            inverse_bind: Motor::from_rotation_translation([1., 0., 0., 0.], world.map(|v| -v)),
        };
        let actual = section.joint(i).ok_or("Missing motion joint")?;
        let id = joint_id(&source.identifier, &j.id);
        if section.joint_id(i) != Some(id)
            || actual.parent != expected.parent
            || actual.rest_local != expected.rest_local
            || actual.inverse_bind != expected.inverse_bind
        {
            return Err("Motion source/model named rest binding differs".into());
        }
        joints.push(actual);
        ids.push(id);
    }
    let compiled = compile(
        source,
        &joints,
        counts.joints,
        counts.joints,
        limits,
        cancel,
    )?;
    let rest = section.clip(0).ok_or("Missing static rest clip")?;
    let rest_id = q_hash(&format!(
        "urn:ecopunk:authored-rig:{:x}:static-rest-hold:v1",
        Sha256::digest(source.identifier.as_bytes())
    ));
    if (
        rest.id,
        rest.duration,
        rest.first_track,
        rest.track_count,
        rest.looping,
    ) != (rest_id, 1., 0, counts.joints as u32, false)
    {
        return Err("Motion static rest inventory differs".into());
    }
    for (i, joint) in joints.iter().enumerate() {
        let track = section.track(i).ok_or("Missing rest track")?;
        let key = section.key(i).ok_or("Missing rest key")?;
        if (track.joint, track.first_key, track.key_count) != (i as u16, i as u32, 1)
            || key.time != 0.
            || key.local != joint.rest_local
        {
            return Err("Motion retained rest keys differ".into());
        }
    }
    if counts.clips != compiled.clips.len() + 1
        || counts.tracks != compiled.tracks.len() + counts.joints
        || counts.keys != compiled.keys.len() + counts.joints
    {
        return Err("Motion source/model clip inventory differs".into());
    }
    for (i, c) in compiled.clips.iter().enumerate() {
        let a = section.clip(i + 1).ok_or("Missing source clip")?;
        if (a.id, a.duration, a.first_track, a.track_count, a.looping)
            != (c.id, c.duration, c.first_track, c.track_count, c.looping)
        {
            return Err("Motion source clip binding differs".into());
        }
    }
    for (i, t) in compiled.tracks.iter().enumerate() {
        let a = section
            .track(counts.joints + i)
            .ok_or("Missing source track")?;
        if (a.joint, a.first_key, a.key_count) != (t.joint, t.first_key, t.key_count) {
            return Err("Motion source track binding differs".into());
        }
    }
    for (i, k) in compiled.keys.iter().enumerate() {
        let a = section.key(counts.joints + i).ok_or("Missing source key")?;
        if a.time != k.time || a.local != k.local {
            return Err("Motion source key binding differs".into());
        }
    }
    let mut source_hash = BoundedHash {
        hasher: Sha256::new(),
        remaining: limits.cold_bytes.min(2 * 1024 * 1024),
    };
    serde_json::to_writer(&mut source_hash, source)
        .map_err(|_| "Motion evaluated source digest exceeds byte budget")?;
    let source_digest: [u8; 32] = source_hash.hasher.finalize().into();
    let mut player =
        wallaby_rig::admit_model(model, rig_limits, TangentPolicy::RegenerateFromRetainedUv)?;
    let mut clips = Vec::new();
    clips
        .try_reserve_exact(motion.clips.len())
        .map_err(|_| "Motion clip allocation refused")?;
    for c in &motion.clips {
        let mut contacts = Vec::new();
        contacts
            .try_reserve_exact(c.contacts.len())
            .map_err(|_| "Motion contact allocation refused")?;
        for contact in &c.contacts {
            contacts.push(runtime::BoundContact {
                indices: [
                    validation::index(source, &contact.chain[0])?,
                    validation::index(source, &contact.chain[1])?,
                    validation::index(source, &contact.chain[2])?,
                ],
                // v1 validation requires each supported hip parent Some(0).
                parent: 0,
                source: contact.clone(),
            });
        }
        clips.push(runtime::BoundClip {
            id: clip_id(&source.identifier, &c.id),
            name: c.id.clone(),
            duration: c.duration_s,
            looping: c.looping,
            contacts,
        });
    }
    player
        .select_clip(clips[0].id, 0.)
        .map_err(|e| format!("Motion select: {e:?}"))?;
    if cancel.load(Ordering::Acquire) {
        return Err("Motion admission cancelled".into());
    }
    Ok(MotionPlayer {
        player,
        ids,
        clips,
        selected: 0,
        extractor: qualia_core_db::render::skeletal::RootMotionExtractor::default(),
        last_time: 0.,
        retired: false,
        model_sha256: Sha256::digest(model).into(),
        source_sha256: source_digest,
    })
}
struct BoundedHash {
    hasher: Sha256,
    remaining: usize,
}
impl std::io::Write for BoundedHash {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.remaining {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Source digest cap",
            ));
        }
        self.remaining -= bytes.len();
        self.hasher.update(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
