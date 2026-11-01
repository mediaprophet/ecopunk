use super::{add, filled, mul, RigLimits, TangentPolicy};
use qualia_core_db::{
    container_10d::{
        self as native, skeletal_container::read_skeletal_section, Container10dHeader,
        MeshFrameDecodeLimits, SectionType,
    },
    render::skeletal::{SkeletalPlayer, MAX_JOINTS, MAX_VERTICES},
};

/// Cold canonical resource admission. No source IDs, rights or biological
/// validity are inferred. The caller retains source/package authority checks.
/// The returned existing player supports explicit joint deltas; those runtime
/// controls are not automatically authored motion or persisted source edits.
pub fn admit_model(
    bytes: &[u8],
    limits: RigLimits,
    tangents_policy: TangentPolicy,
) -> Result<SkeletalPlayer, String> {
    if bytes.len() > limits.max_container_bytes
        || bytes.len() > 42 * 1024 * 1024
        || bytes.len() > limits.cold_bytes
    {
        return Err("Wallaby rig resource exceeds caller admission budget".into());
    }
    let rig =
        read_skeletal_section(bytes).map_err(|e| format!("Wallaby canonical rig read: {e:?}"))?;
    let counts = rig.counts();
    let n = counts.vertices;
    if n > limits.max_vertices.min(MAX_VERTICES)
        || counts.joints > limits.max_joints.min(MAX_JOINTS)
    {
        return Err("Wallaby rig exceeds selected vertex/joint budget".into());
    }
    let header =
        Container10dHeader::parse(bytes).map_err(|e| format!("Wallaby .10d header: {e:?}"))?;
    let sections = native::parse_section_table(bytes, &header)
        .map_err(|e| format!("Wallaby .10d table: {e:?}"))?;
    let mut meshes = sections
        .iter()
        .filter(|s| s.typ() == Some(SectionType::QuantizedMesh));
    let mesh = meshes.next().ok_or("Wallaby rig lacks canonical mesh")?;
    if meshes.next().is_some() {
        return Err("Wallaby rig has ambiguous bind meshes".into());
    }
    let start = usize::try_from(mesh.byte_offset).map_err(|_| "Wallaby mesh offset overflow")?;
    let end = add(
        start,
        usize::try_from(mesh.byte_length).map_err(|_| "Wallaby mesh size overflow")?,
    )?;
    let payload = bytes
        .get(start..end)
        .ok_or("Wallaby mesh outside container")?;
    let (mesh_header, needed) =
        native::parse_mesh_header(payload).map_err(|e| format!("Wallaby mesh header: {e:?}"))?;
    let indices = mul(mesh_header.triangle_count as usize, 3)?;
    if mesh_header.vertex_count as usize != n
        || needed != payload.len()
        || indices == 0
        || indices > limits.max_indices.min(MAX_VERTICES * 6)
    {
        return Err("Wallaby rig bind mesh count/layout refused".into());
    }
    // Conservative incremental logical envelope for decoded mesh/frames/UV,
    // flattened indices and Core's owned player. Not measured allocator/RSS.
    let required = add(
        bytes.len(),
        add(
            mul(n, 320)?,
            add(
                mul(indices, 12)?,
                add(
                    mul(counts.joints, 4096)?,
                    add(
                        mul(counts.keys, 80)?,
                        add(mul(counts.tracks, 16)?, 64 * 1024)?,
                    )?,
                )?,
            )?,
        )?,
    )?;
    if required > limits.cold_bytes {
        return Err("Wallaby rig readback/player exceeds caller cold budget".into());
    }
    let decoded =
        native::decode_mesh_section(payload).map_err(|e| format!("Wallaby mesh decode: {e:?}"))?;
    let frame_limits = MeshFrameDecodeLimits {
        max_vertices: n,
        max_payload_bytes: bytes.len(),
        max_decoded_bytes: mul(n, 28)?,
    };
    let layout = native::inspect_container_mesh_frames(bytes, n, frame_limits)
        .map_err(|e| format!("Wallaby canonical frame inspection: {e:?}"))?
        .ok_or("Wallaby rig requires retained normal frames")?;
    if !layout.has_normals {
        return Err("Wallaby rig requires normals; no invented frame stream".into());
    }
    if layout.has_tangents && tangents_policy == TangentPolicy::RefuseStoredTangents {
        return Err("Stored tangent preservation is unsupported by the current player; select explicit runtime regeneration or retain the source".into());
    }
    let mut normals = filled(n, [0.; 3])?;
    let mut tangents = filled(if layout.has_tangents { n } else { 0 }, [0.; 4])?;
    native::read_container_mesh_frames_into(bytes, n, frame_limits, &mut normals, &mut tangents)
        .map_err(|e| format!("Wallaby canonical frame decode: {e:?}"))?;
    // Canonical _into decoding validates both present streams. Only the explicit
    // regeneration policy permits omitting stored tangents from player inputs;
    // the native resource still retains those exact bytes.
    drop(tangents);
    let mut uv = filled(n, [0.; 2])?;
    native::read_texture_coordinate_section_into(bytes, n, &mut uv)
        .map_err(|e| format!("Wallaby retained UV0: {e:?}"))?;
    let mut flat = filled(indices, 0u32)?;
    for (out, t) in flat.chunks_exact_mut(3).zip(&decoded.triangles) {
        out.copy_from_slice(t);
    }
    let player = SkeletalPlayer::from_section_with_budget(
        &rig,
        &decoded.positions,
        &normals,
        &uv,
        &flat,
        limits.player_bytes,
    )
    .map_err(|e| format!("Wallaby canonical player admission: {e:?}"))?;
    // SKL1 validates inverse-bind consistency. Verify the actual quantized bind
    // frame survives rest skinning within the admitted weight/float tolerance.
    // This checks rest only, never biological bending or collision/contact.
    if player
        .positions()
        .iter()
        .zip(&decoded.positions)
        .any(|(a, b)| {
            a.iter()
                .zip(b)
                .any(|(x, y)| !x.is_finite() || (*x - *y).abs() > 2e-5 * y.abs().max(1.))
        })
        || player.normals().iter().zip(&normals).any(|(a, b)| {
            a.iter()
                .zip(b)
                .any(|(x, y)| !x.is_finite() || (*x - *y).abs() > 2e-4)
        })
    {
        return Err("Wallaby rest skinning differs from canonical bind frame".into());
    }
    Ok(player)
}
