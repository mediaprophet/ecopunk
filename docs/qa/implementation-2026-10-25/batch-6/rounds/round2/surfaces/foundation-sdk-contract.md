# Foundation SDK production contract — proposed before implementation

2026-10-25; requested Core `.10d` implementation scope; root approved the registry
and narrow transactional MRP1 decoder repair before Core source writes.

Register built-in MID2 field kinds `FIELD_KIND_SURFACE_FRAMES = 0x0200` and
`FIELD_KIND_MORPH_TARGETS = 0x0201`; bounded source/standard search found no
existing assignment. Generic vector fields are never interpreted as normals.
These values select existing NRM1/FRM1 and MRP1 codecs, respectively. They do
not allocate new outer section tags, codec versions, or persistence formats.
Existing MID2 readers without these assignments reject UnsupportedFieldKind;
upgrade the reader or explicitly supply its supported extension-kind allow-list.
No existing default writer or legacy container bytes change.

Proposed public API under `container_10d::typed_payloads`:

- `TypedPayloadResolver::new(data, supported_extension_kinds, field_scratch,
  out_fields)` validates the header/section table and, when present, all MID2
  kind/range references before exposing caller-buffered decoded records.
- `resolver.field(field_id)` returns a borrowed exact payload slice plus its
  validated TypedFieldRecord; no magic scanning or payload copying.
- `resolver.surface_frames()` and `resolver.morph_targets()` select the unique
  registered kind and reject ambiguous duplicate matches. Declared payloads
  must bind a nonempty FieldSidecar range; empty declarations fail clearly.
- No MID2: the surface reader preserves the established optional whole-sidecar
  NRM1/FRM1 path. MID2 present: only typed SurfaceFrames references select
  normals/tangents, with no guessing or legacy fallback.
- `read_container_mesh_frames(data, expected_vertices)` uses bounded fixed
  record/scratch arrays for selection, then existing decode_mesh_frames and
  vertex-count validation. Existing allocating decoded-stream return is retained.
- `read_container_morph_targets_into(data, expected_vertices, out_deltas)` uses
  the same selection and existing caller-buffered MRP1 decoder, with preflight
  header/count validation before output is modified. MID2 declares production
  morph semantics; absent MID2 supports an explicitly recognized whole MRP1
  sidecar. With MID2 present, unrelated fields never trigger morph inference.

MorphTargets uses a mixed schema. Its declaration has unit_id=0; position deltas
inherit mesh coordinate convention, while normal/tangent deltas are unitless.
Scientific field units are preserved independently.

Errors distinguish bad envelope, unsupported manifest kind/range, ambiguous
kind, malformed codec, mismatched vertex count, and insufficient caller buffer.
Resolver hot selection allocates nothing. Existing envelope CRC rules remain
the owner; this scope does not silently strengthen required legacy sealing.

Delegate render/compile_10d.rs frame reads to the shared SDK. Replace portal's
whole-sidecar frame decode with the same public SDK after the mesh is known;
keep new behavior in the focused module. Track the existing oversized render
files for decomposition; do not grow additional lifecycle behavior in them.

Regression uses existing encoders: frames + MRP1 + a scientific descriptor or
raster, one FieldSidecar, MID2 exact ranges, alternate subpayload orders.
Validate exact independent decode and high-level normal use, legacy behavior,
unsupported kinds, invalid ranges, duplicate frame-kind ambiguity, malformed
codec, vertex mismatch and caller-buffer transactional refusal. Root owns Cargo
execution; source/test presence alone proves no runtime or model acceptance.

The legacy StableEntityId spelling and equality APIs remain. Rustdoc now
distinguishes an opaque stored handle from verified issuer/global uniqueness;
qualified referent resolution belongs to native graph/authority mechanisms.
The wider identifiers/API migration remains outside this allocation.

Canonical section parsing now owns unaligned-table errors and header/table
overlap validation through a focused child module. It preserves valid payloads
before a later table and empty section offsets. The typed resolver delegates to
that canonical reader; it contains no parallel private envelope guard/parser.
