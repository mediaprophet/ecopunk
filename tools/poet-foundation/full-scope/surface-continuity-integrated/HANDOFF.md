# Surface continuity implementation candidate

Status: staged source only. No compiler, tests, GPU, build, exports, production asset or canonical source changes by this worker.

## Current implementation traced

Poet `surfaces/uv_atlas.rs::apply` already remaps authored normals by Core atlas `source_vertices` and retains original raw source geometry. When authored normals are absent it returns None; `assembly/admission.rs` then permits flat-normal expansion. That can introduce faceting across a deliberately smooth surface. Current `assembly/compile.rs` emits normals but passes no tangent stream to `.10d` compilation. The GPU upload owner in Core already generates UV-aligned tangents through `render/mesh_tangent_frames::tangents_from_uvs_into` when no explicit tangents are supplied; this is existing functionality, not a missing engine.

## Staged changes and installation mapping

- `mesh_tangent_frames.rs` -> QualiaDB `crates/qualia-core-db/src/render/mesh_tangent_frames.rs`: additive caller-buffered strict admission wrapper using the existing generator. Rejects nonfinite/degenerate geometry and UVs, unreferenced vertices, mixed UV orientation without source splits, overflowed/undefined accumulated frames. The published output is untouched on every returned failure. Scratch/temporary may change. Existing legacy API and fallback behavior are unchanged.
- `uv_atlas.rs` -> Poet `native/src/graphical/surfaces/uv_atlas.rs`: explicit version 2 policy. Version 1 retains its existing behavior. Version 2 preserves supplied normals; otherwise it calls existing DDG face/vertex normals on the UNSPLIT source, maps them through Core atlas provenance, then generates strict tangents on the split derivative. Native `.10d` and source storage are unchanged. The `AtlasApplied.tangents` field is a derived result, not an authored master field.
- `uv_atlas_controls.rs` -> Poet `native/src/graphical/authoring/uv_atlas_controls.rs`: explicit native opt-in checkbox for smooth shading across UV seams, with existing accepted-settings load/source action.
- `uv_atlas_tests.rs` -> Poet `native/src/graphical/surfaces/uv_atlas_tests.rs`: v2 roundtrip/continuity and v1-preservation source cases. Strict wrapper tests are appended in the Core source candidate. Tests are not executed.

Root owns canonical registration/install and must preserve preimages and reconcile other source changes. No new module registration is required; strict API is added to the existing public module.

## Resource and preservation contract

Atlas wrapper adds logical element-payload reservation `36*source_vertices + 204*source_triangles` bytes and structural-work reserve `8*source_vertices + 432*source_triangles` before Core unwrap. The original wrapper reservation is retained. This counts additional normal scratch, remapped frame buffers and flat indices conservatively; it is not a hard allocator/process-memory ceiling. New frame allocations are fallible, but Vec capacity and allocator metadata may exceed requested length. Existing atlas allocation behavior remains outside this added claim.

Normals are computed on original topology before UV cuts, preventing UV cuts alone from introducing a smooth-normal discontinuity. Authored normals and intentional hard-edge source vertex splits take precedence. Tangents remain vertex/corner aligned and mirrored orientation must already be split. Unsupported tangent/skin/morph/extra-UV source records remain refused by existing assembly schema; there is no implemented arbitrary attribute remapping or hard-group authoring migration here.

The current CompiledAssembly type has no tangent field. Existing assembly consumers use the new normals, but do not yet retain AtlasApplied.tangents as a `.10d` tangent sidecar. GPU upload already regenerates tangent frames using the existing Core owner; do NOT claim explicit authored tangent source preservation or frame-sidecar persistence from this staging. A follow-up consumer must either carry the derivative stream or derive world-space tangents at compile using the same bounded owner, while preserving fallback semantics for untextured parts.

This does not solve junction self-intersection, multi-branch source correspondence, general remesh/skin migration, production texture distortion limits or biological acceptance. Atlas explicit cuts and packing remain existing harmonic disk-chart behavior, not an automatic tree unwrap. Tests/builds are deferred under the user's instruction.

## Original source pins

- Poet uv_atlas.rs: AA7C5EC4B1027BBBBD8EACF5AF53A29F5B5911A82974781E0C672B48E6DB4C01
- Poet uv_atlas_tests.rs: 29BF04946F862100D3A083297C3472E6CBFA4526185AEFBED482C048C81AD863
- Poet uv_atlas_controls.rs: 7BCFB93F7DF5A2DB1784176AF4983A13964816F200F5D270BDC9344AFB43116D
- Core mesh_tangent_frames.rs: DAAAFB9B8CC78B1553FE276AAE82412B1EA41D6F9E69A632581AF5B3E8BDA01B