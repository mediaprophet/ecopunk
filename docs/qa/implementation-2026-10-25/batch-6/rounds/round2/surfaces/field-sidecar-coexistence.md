# FieldSidecar coexistence: public-call regression proposal

2026-10-25; source inspection only; no test executed or engine patch applied.
Core paths are under `C:/github/qualiaDB/crates/qualia-core-db/src/`.

The envelope already supports typed byte-range references. Its current normal
consumers do not use them. This is a consumer/SDK integration gap; this finding
does not establish that a new persistence format is required.

## Exact existing calls

1. `container_10d::normal_section::encode_mesh_frames(normals, tangents, out)`
   produces legacy `NRM1` or combined `FRM1` bytes.
2. `container_10d::morph_section::encode_morph_section(header, deltas, out)`
   produces `MRP1`; `decode_morph_section_into(payload, out_deltas)` reads it.
3. `container_10d::field_section::encode_field_section(entries, out)` produces
   scientific field descriptors; `decode_field_section(bytes)` reads them.
   Sample-bearing rasters use `encode_field_section_with_rasters` and
   `decode_field_raster_samples` instead of inventing a float payload layout.
4. `container_10d::manifold_identity_container::
   encode_container_with_manifold_identity_extensions` accepts one
   `SectionInput { section_type: SectionType::FieldSidecar, ... }` and several
   `TypedFieldRecord` ranges selecting independently encoded subpayloads.
5. `decode_container_manifold_identity(data, header, supported_extension_kinds,
   field_scratch, out_fields)` validates section IDs and byte ranges.
6. `container_10d::section::parse_section_table` finds the sidecar descriptor;
   apply its byte offset plus each validated field range to select payloads.
7. Call the existing normal/morph/scientific decoders on those exact slices.

## Existing mechanisms and limits

- `manifold_identity_v2.rs` has built-in scalar/vector/tensor/spectral/EMF
  kinds and an explicit extension-kind allow-list. It does not declare registered
  normal, tangent, or morph kinds. Do not silently label every vector a normal.
- `TypedFieldRecord` holds stable field ID, kind, flags, unit/convention ID,
  section ID, offset and length. Stable field IDs must be strictly ascending.
- `section.rs` rejects duplicate SectionType values. Two separate FieldSidecar
  sections cannot currently carry normal and morph payloads independently.
- `morph_section.rs` aliases `SectionType::MorphTargets` to `FieldSidecar` (8).
  Its separate `MORPH_TARGETS_DISCRIMINANT = 0x0013` constant is not an assigned
  outer section tag; outer tag 19 is `ImpostorAtlas`. Preserve exact spellings.
- The manifest validates bounds, not kind-specific semantics, overlap policy,
  vertex counts, unit meaning, or source lineage of selected subpayloads.
- `normal_section.rs::decode_mesh_frames` requires an exact NRM1/FRM1 length.

## Public regression and owning patch decision

- Encode a three-vertex triangle's normal frames, one sparse morph, and one
  scientific descriptor/raster using the existing codecs above. Concatenate
  their bytes into one caller-owned sidecar with nonoverlapping ranges.
- Reference the ranges through MID2. Use explicitly test-local allowed extension
  kinds until the format owner allocates production semantics; do not publish
  those fixture values as a registry. Verify each decoded slice independently.
- Repeat with different subpayload order. Verify source bytes and vertex fields
  survive; reject invalid ranges, unavailable extension kind, malformed codec,
  inconsistent vertex count, and undersized caller buffers transactionally.
- Current `render/compile_10d.rs::decode_10d_mesh_frames` (635ff) inspects and
  decodes the entire sidecar. Normal-first concatenation fails exact length;
  normal-after-other data is skipped. `render/portal/mod.rs` (1330ff) has the
  same whole-sidecar assumption. These are inspection predictions, unexecuted.
- Owning change: Core public typed-payload selection/validation and registered
  surface/morph semantics, then shared native/WASM consumers. Poet should call
  that shared API; it must not own a parallel magic-scanning reader or renderer.
- Keep legacy one-payload normal containers byte-identical. Validate coexistence
  before primitive surface SDK work or any new asset. m64/e64 ownership remains
  under separate investigation; no new format has been created here.
