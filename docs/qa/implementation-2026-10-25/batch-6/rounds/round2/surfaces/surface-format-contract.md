# Bounded native surface / format contract investigation

2026-10-25; source inspection only. No SDK/engine changes or Cargo checks.
The latest instruction holds implementation pending ecosystem-format review.
No new model/asset, human acceptance, or model-completion claim is made.

## Demonstrated transport loss

- `C:/github/poet/native/src/graphical/spatial_host.rs::mesh_value`
  emits only `vertices` and `triangles` from Core `render::assets::Mesh`.
- `native/src/graphical/assembly/admission.rs::geometry_counts` already
  validates optional vertex-aligned `normals` and `uv0` streams.
- `crates/spatial-authoring/src/assembly.rs::compile_assembly` uses supplied
  normals; absence expands every triangle to flat-shaded separate vertices.
- `spatial-authoring/src/frames.rs::normal` already reuses Core
  `GpuInstanceRecord::normal_from_local` for inverse-transpose transformation.

## Existing reusable public APIs and owners

All Core paths below are relative to
`C:/github/qualiaDB/crates/qualia-core-db/src/`.

- `render/mesh_surface_frames.rs::vertex_normals_into` and
  `vertex_normals_from_indices_into`: area-weighted shared-index normals.
  Distinct authored indices retain hard seams; shared caps/corners would smooth.
- `render/mesh_surface_frames.rs::box_project_mesh_with_sources_into`:
  geometry-aligned UV charts, supplied normals, unchanged triangle order,
  source-vertex lineage for duplicated seam vertices, caller-owned buffers.
  Worst case is three output vertices per triangle; kernel rejects >262,144.
- `render/mesh_tangent_frames.rs::tangents_from_uvs_into`: UV-aligned tangents,
  explicit handedness, caller-owned output and six-float scratch per vertex.
- `container_10d/normal_section.rs::encode_mesh_frames/decode_mesh_frames`:
  exact legacy magic `NRM1`; combined normal/tangent magic `FRM1`.
  Normals-only encoding deliberately remains `NRM1` for byte stability.
- `container_10d/texture_coordinate_section.rs` exposes `UV01` through
  `encode_texture_coordinate_section`, `decode_texture_coordinates_into`,
  `texture_coordinate_section_count`, and checked `encoded_len`.
- `container_10d/manifold_identity_container.rs` exposes
  `encode_container_with_manifold_identity[_extensions]` and
  `decode_container_manifold_identity`: typed section-ID + byte-range references.
  Payload schema owners still validate the actual selected payload semantics.

## Proposed contract, pending format-owner review

- Preserve existing default operator geometry and transport for reproducibility.
- An explicit versioned policy must declare smoothing domains, hard boundaries,
  UV mapping and scale; reject unknown policies and inapplicable parameters.
- Reuse existing `.10d` sections/typed range references; do not create a private
  persisted surface container or infer a new m64/e64 format from file suffixes.
- Keep source vertex lineage when splitting UV/normal seams so identifiers,
  skin/morph/scientific vertex fields cannot silently become misaligned.
- Shared profile sweep caps must separate from side surfaces; polygon corners
  must remain hard unless the source explicitly describes a continuous curve.
  Blanket shared-index smoothing is insufficient. A general public hard-boundary
  split/primitive-surface contract was not found by the bounded render/CG search.

## Budgets and missing qualification

- Preflight expanded vertices, triangle indices, normal/UV/tangent streams,
  source lineage, chart tables, input/output coexistence and kernel scratch.
- Also preflight Vibe transport before allocation: nested `Value::List` records
  cost row Values plus scalar Values, not merely raw float bytes. Use checked
  `size_of::<vibe::Value>()` arithmetic and reserve record/list overhead.
- Existing `GeometryLimits::admit` estimates 64 bytes per vertex and triangle;
  that does not establish a bound for eight transported surface scalar lanes.
- Geometry host work accounting must debit expanded emitted vertex count.
- Required regression cases: curved continuity; hard cap/corner separation;
  unchanged winding/shape; finite useful UVs; source lineage; nonuniform normal
  transformation; invalid policy; budget refusal before allocation/mutation;
  legacy byte stability; normal+morph+scientific payload coexistence.
- Existing generic envelope range validation is source evidence, not proof that
  every direct FieldSidecar consumer supports multiplexed payloads. Audit those
  consumers before selecting a public regression case or declaring coexistence.
