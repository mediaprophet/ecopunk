$ErrorActionPreference = 'Stop'
$root = Join-Path $PSScriptRoot 'after/crates/qualia-core-db/src'
function Update([string]$rel, [scriptblock]$transform) {
    $path = Join-Path $root $rel
    $original = [IO.File]::ReadAllText($path)
    $changed = & $transform $original
    [IO.File]::WriteAllText($path, $changed, [Text.UTF8Encoding]::new($false))
}
Update 'container_10d/normal_section.rs' {
    param($text)
    $text = $text.Replace('use bytemuck::{bytes_of, from_bytes, Pod, Zeroable};', @'
use bytemuck::{bytes_of, Pod, Zeroable};

mod frame_decode;
pub use frame_decode::{
    decode_mesh_frames_into, decode_mesh_frames_with_limits, inspect_mesh_frames,
    MeshFrameDecodeLimits, MeshFrameLayout,
};
'@)
    $text = $text.Replace('    EmptyFrame,', @'
    EmptyFrame,
    PayloadTooLarge { needed: usize, max: usize },
    DecodedTooLarge { needed: usize, max: usize },
    AllocationFailed,
'@)
    $start = $text.IndexOf('pub fn decode_mesh_frames(bytes:')
    $end = $text.IndexOf('/// Encode normalized vertex normals', $start)
    if ($start -lt 0 -or $end -lt 0) { throw 'missing decoder boundary' }
    $text = $text.Substring(0,$start) + @'
pub fn decode_mesh_frames(bytes: &[u8]) -> Result<MeshFrameStreams, NormalSectionError> {
    decode_mesh_frames_with_limits(bytes, MeshFrameDecodeLimits::COMPATIBILITY)
}

'@ + $text.Substring($end)
    $start = $text.IndexOf('/// Decode the canonical octahedral normal sidecar.')
    $end = $text.IndexOf('/// Decode vertex normals from', $start)
    if ($start -lt 0 -or $end -lt 0) { throw 'missing legacy decoder boundary' }
    $text.Substring(0,$start) + $text.Substring($end)
}
Update 'container_10d/typed_payloads/readers.rs' {
    param($text)
    $start = $text.IndexOf('/// Decode a MID2 MorphTargets')
    if ($start -lt 0) { throw 'missing morph boundary' }
    @'
//! Caller-buffered morph codec adapter over typed selection.
use super::{TypedPayloadError, TypedPayloadResolver};
use crate::container_10d::{
    MANIFOLD_IDENTITY_V2_MAX_FIELDS, MORPH_SECTION_HEADER_SIZE,
    MorphSectionHeader, QuantizedMorphDelta, TypedFieldRecord, decode_morph_section_into,
};

'@ + $text.Substring($start)
}
Update 'container_10d/typed_payloads/mod.rs' {
    param($text)
    $text = $text.Replace('mod readers;', 'mod frames;' + "`n" + 'mod readers;')
    $text.Replace('pub use readers::{read_container_mesh_frames, read_container_morph_targets_into};', @'
pub use frames::{
    inspect_container_mesh_frames, read_container_mesh_frames,
    read_container_mesh_frames_into, read_container_mesh_frames_with_limits,
};
pub use readers::read_container_morph_targets_into;
'@)
}
Update 'container_10d/mod.rs' {
    param($text)
    $text = $text.Replace('    decode_mesh_frames, decode_normals, decode_tangents, encode_mesh_frames, encode_normals,', @'
    decode_mesh_frames, decode_mesh_frames_into, decode_mesh_frames_with_limits,
    inspect_mesh_frames, MeshFrameDecodeLimits, MeshFrameLayout,
    decode_normals, decode_tangents, encode_mesh_frames, encode_normals,
'@)
    $text.Replace('    read_container_mesh_frames, read_container_morph_targets_into, TypedPayload,', @'
    inspect_container_mesh_frames, read_container_mesh_frames,
    read_container_mesh_frames_into, read_container_mesh_frames_with_limits,
    read_container_morph_targets_into, TypedPayload,
'@)
}
Update 'render/compile_10d.rs' {
    param($text)
    $text.Replace('crate::container_10d::read_container_mesh_frames(container_10d, expected_vertex_count)', @'
crate::container_10d::read_container_mesh_frames_with_limits(
        container_10d, expected_vertex_count,
        crate::container_10d::MeshFrameDecodeLimits::COMPATIBILITY,
    )
'@)
}
Update 'render/portal/mod.rs' {
    param($text)
    $text.Replace('container_10d::read_container_mesh_frames(&bytes_mut, mesh.vertex_count())', @'
container_10d::read_container_mesh_frames_with_limits(
            &bytes_mut, mesh.vertex_count(),
            container_10d::MeshFrameDecodeLimits {
                max_vertices: container_10d::MAX_NORMAL_COUNT,
                max_payload_bytes: 64 * 1024 * 1024,
                max_decoded_bytes: 256 * 1024 * 1024,
            },
        )
'@)
}
Update 'render/portal/scene_mesh.rs' {
    param($text)
    $text = $text.Replace('match crate::render::compile_10d::decode_10d_normals(&bytes, mesh.positions.len()) {', @'
match container_10d::read_container_mesh_frames_with_limits(
                &bytes, mesh.positions.len(),
                container_10d::MeshFrameDecodeLimits {
                    max_vertices: container_10d::MAX_NORMAL_COUNT,
                    max_payload_bytes: 64 * 1024 * 1024,
                    max_decoded_bytes: 256 * 1024 * 1024,
                },
            ).map(|frames| frames.and_then(|frames| frames.normals)) {
'@)
    $text
}
