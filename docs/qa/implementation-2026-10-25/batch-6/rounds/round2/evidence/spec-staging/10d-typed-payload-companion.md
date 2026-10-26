# `.10d` typed-payload companion — staged for review

**Status:** source-grounded draft; not a new canonical standard. Installed Core SDK behavior is described below. Selected Win64 little-endian native conformance passed in [the Round 2 log](../../native-conformance.log). This does not pass FND-03 or any product gate.

## Outer container and registration model

- `.10d` outer header versions 1 and 2 are supported; current writers emit v2. `MID2` is a registered section only in v2. The encoder/parser reject a `ManifoldIdentityV2` section under outer v1; no outer-version bump is required for the registered MID2 field-kind additions below.
- The outer section table has one `FieldSidecar` entry (section type 8) and one `ManifoldIdentityV2` entry (section type 15). Canonical encoding sorts distinct section types, rejects duplicate outer types, validates section ranges/alignment/CRC, and retains the existing parser contract.
- `FIELD_KIND_SURFACE_FRAMES` (`0x0200`) and `FIELD_KIND_MORPH_TARGETS` (`0x0201`) are MID2 `TypedFieldRecord.kind` values, not outer `SectionType` values or separate payload formats. Their codec bytes and scientific-field payload ranges can occupy nonoverlapping slices of the single `FieldSidecar` section.

## MID2 manifold-identity manifest

The v2 manifest has a 112-byte header and up to 256 fixed 48-byte field records (12,400 bytes maximum). Header offsets: `MID2` magic `[0..4]`; version 2 `[4..6]`; header size `[6..8]`; total length `[8..12]`; field count `[12..14]`; coordinate count `[14]`; flags `[15]`; stable entity identifier bytes `[16..32]`; profile, domain and coordinate-convention identifiers `[32..56]`; ten `f32` coordinates `[56..96]`; optional external digest/index hint `[96..112]`. Coordinate order is `[q, v, w, x, y, z, t, α, μ, σ]`.

Each 48-byte `TypedFieldRecord` stores `field_id` at `[0..8]`, `kind` `[8..10]`, `flags` `[10..12]`, `unit_id` `[12..20]`, `payload_section_id` `[20..24]`, reserved zero `[24..28]`, `payload_offset` `[28..36]`, `payload_length` `[36..44]`, and reserved zero `[44..48]`. IDs are nonzero, unique and strictly ascending. A populated range names a real outer section and uses checked offset-plus-length arithmetic; empty declarations use a zero section identifier and zero offset/length. `StableEntityId` is an identifier value, not proof of issuer authority. An external digest is not the entity identifier. `unit_id` is distinct from legacy FIEL `unit_hash`.

## Typed payloads within FieldSidecar

| MID2 kind | Existing payload | Binding and reader behavior |
|---|---|---|
| `FIELD_KIND_SURFACE_FRAMES` (`0x0200`) | `NRM1` normals or `FRM1` interleaved normals/tangents. NRM1 uses a 16-byte header plus 4 bytes per vertex; FRM1 uses the 16-byte header plus 4 bytes per normal and/or 8 bytes per tangent per vertex. | Unique kind; `payload_section_id` is FieldSidecar and `unit_id` is zero. The reader checks declared/decoded vertex count. A generic `Vector` field is never guessed to be normals. |
| `FIELD_KIND_MORPH_TARGETS` (`0x0201`) | `MRP1`: 32-byte header and 16-byte quantized delta records; current limits are 256 targets and 1,048,576 deltas. | Unique kind; FieldSidecar, `unit_id=0`. Position deltas use the mesh coordinate convention; normal/tangent deltas are dimensionless. Reader checks vertex count and validates deltas before publishing output. |
| Scientific kinds such as scalar/vector/tensor/spectral/EMF | FIEL v0 descriptors plus raster metadata and `f32` sample arrays where present. Public codecs include `encode_field_section_with_rasters`, `decode_field_section`, and `decode_field_raster_samples`. | Preserve the descriptor's `name_hash`, `unit_hash`, field type/support/representation and sample range. Preserve MID2 `unit_id` separately; do not infer equality, convert units, or discard scientific samples when adding frames/morphs. |

FIEL v0's 12-byte header and 32-byte descriptors coexist with raster encoding APIs. Earlier Rustdoc claiming that v0 contains no sample bytes is stale relative to these APIs and must not be used as capability evidence. Raster grid metadata is 16 bytes (`width`, `height`, `sample_count`, reserved), followed by little-endian `f32` samples. The legacy FIEL unit hash is not a registry identifier.

## Resolution, legacy paths and refusal behavior

`TypedPayloadResolver::new` parses the outer header and canonical section table, then validates the MID2 manifest and ranges before changing caller `out_fields`; caller scratch may change on error. Selection returns borrowed byte ranges. `surface_frames()` and `morph_targets()` require a unique matching typed kind and valid section/unit binding. Generic fields are selected by `field_id`; unknown extension kinds fail closed unless explicitly allowed by the decoder.

When MID2 is absent, the existing readers may recognize a whole-FieldSidecar legacy NRM1/FRM1 or MRP1 payload. When MID2 is present, an absent registered kind means no such payload; the reader does not guess from unrelated field bytes or magic. `read_container_morph_targets_into` checks the declared vertex count, and MRP1 decode validates/preflights its records and output capacity before publishing caller output. Preserve these transactional refusal cases.

Older MID2 readers without the `0x0200`/`0x0201` kind registrations reject those records as unsupported kinds; they must not reinterpret or silently skip them. The outer v2 version and 112/48-byte MID2 layout remain unchanged. HMC/Q42 graph knowledge, resource identifiers, provenance and whole-embodiment coverage remain under the HMC owner and are not replaced by this sidecar map.

## Source references and evidence boundary

- Outer table and version policy: `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/{header.rs,section.rs,section/table_validation.rs}`.
- MID2 constants, record layout and validation: `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/manifold_identity_v2.rs`.
- Typed selection and high-level readers: `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/typed_payloads/{mod.rs,resolver.rs,readers.rs}`.
- Existing codec layouts: `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/{normal_section.rs,morph_section.rs,field_section.rs}`.
- Authored SDK cases: `C:/github/qualiaDB/crates/qualia-core-db/tests/typed_field_payloads.rs`. The 9 selected tests passed, including coexistence in either slice order, legacy fallback, unsupported kinds, count checks and transactional output cases; the full Round 2 result is recorded in [native-conformance.log](../../native-conformance.log).
