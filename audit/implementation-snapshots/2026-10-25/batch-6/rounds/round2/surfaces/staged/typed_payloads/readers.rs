//! Codec adapters over typed selection. Decoded surface streams retain the
//! existing allocating API; payload selection and morph output are zero-heap.
use super::{TypedPayloadError, TypedPayloadResolver};
use crate::container_10d::{
    decode_mesh_frames, decode_morph_section_into, MeshFrameStreams, MorphSectionHeader,
    QuantizedMorphDelta, TypedFieldRecord, MANIFOLD_IDENTITY_V2_MAX_FIELDS,
    MORPH_SECTION_HEADER_SIZE,
};

/// Decode the unique SurfaceFrames field or the legacy single frame payload.
/// With MID2 present, absent SurfaceFrames means no frame stream: never guess.
pub fn read_container_mesh_frames(
    data: &[u8],
    expected_vertices: usize,
) -> Result<Option<MeshFrameStreams>, TypedPayloadError> {
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let resolver = TypedPayloadResolver::new(data, &[], &mut scratch, &mut fields)?;
    let payload = if resolver.has_manifest() {
        resolver.surface_frames()?.map(|p| p.bytes)
    } else { resolver.legacy_frames() };
    let Some(payload) = payload else { return Ok(None); };
    // Both existing frame headers put vertex count at byte 8. Refuse mismatched
    // counts before invoking the existing bounded allocating decoder.
    if payload.len() < crate::container_10d::NORMAL_SECTION_HEADER_SIZE {
        return Err(TypedPayloadError::Frames(crate::container_10d::NormalSectionError::HeaderTooShort));
    }
    if !crate::container_10d::is_normal_section(payload) {
        return Err(TypedPayloadError::Frames(crate::container_10d::NormalSectionError::BadMagic));
    }
    let declared = u32::from_le_bytes(payload[8..12].try_into().expect("checked frame header")) as usize;
    if declared != expected_vertices {
        return Err(TypedPayloadError::VertexCount { expected: expected_vertices, got: declared });
    }
    let frames = decode_mesh_frames(payload).map_err(TypedPayloadError::Frames)?;
    let count = frames.normals.as_ref().map(Vec::len)
        .or_else(|| frames.tangents.as_ref().map(Vec::len)).unwrap_or(0);
    if count != expected_vertices {
        return Err(TypedPayloadError::VertexCount { expected: expected_vertices, got: count });
    }
    Ok(Some(frames))
}

/// Decode a production MID2 MorphTargets field into caller storage. The shared
/// MRP1 codec validates every record before publishing any caller output.
pub fn read_container_morph_targets_into(
    data: &[u8],
    expected_vertices: usize,
    out_deltas: &mut [QuantizedMorphDelta],
) -> Result<Option<(MorphSectionHeader, usize)>, TypedPayloadError> {
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let resolver = TypedPayloadResolver::new(data, &[], &mut scratch, &mut fields)?;
    let Some(payload) = resolver.morph_targets()? else { return Ok(None); };
    if payload.bytes.len() >= MORPH_SECTION_HEADER_SIZE {
        let header: MorphSectionHeader = bytemuck::pod_read_unaligned(&payload.bytes[..MORPH_SECTION_HEADER_SIZE]);
        if header.vertex_count as usize != expected_vertices {
            return Err(TypedPayloadError::VertexCount { expected: expected_vertices, got: header.vertex_count as usize });
        }
    }
    decode_morph_section_into(payload.bytes, out_deltas).map(Some).map_err(TypedPayloadError::Morph)
}
