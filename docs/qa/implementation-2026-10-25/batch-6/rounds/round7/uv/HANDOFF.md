# Round 7 UV domain staging

Status: **staged, not applied or compiled**. The eight new files are pinned in
[target-manifest.json](target-manifest.json); [before-state.json](before-state.json)
records absent target preimages. No existing sibling file was changed. The only
executed code check was successful Rust 2021 rustfmt parsing/formatting, recorded
in [rustfmt-result.json](rustfmt-result.json). Test definitions are not passing
test results. Production assets, credentials and human review are untouched.

## Existing owners and bounded contract

Inspection found publicly reexported
`qualia_core_db::specialized_libs::computational_geometry::harmonic_parameterize_disk`
in `ddg_operators.rs`: ordered boundary pinned by source edge lengths to a circle,
uniform iterative harmonic interior, caller UV output, bounded-by-input internal
scratch, and explicit iteration count. Its implementation initializes its output
on every call. It has no atlas, seam cutting, packing, caller scratch limit or
cancellation parameter. Searches of computational geometry, Poet reusable native
libraries and Craft found no existing complete seam/chart/atlas public operator.

The staged `computational_geometry::uv_atlas` module supplies this missing general
operator in the canonical owner. It composes the existing solver, sorted directed
edge incidence, corner disjoint sets and face disjoint sets. Cutting a nonseparating
seam duplicates corner fans inside a single chart, rather than merely duplicating
whole face components. It checks oriented two-face edge incidence, one simple
boundary and post-cut Euler characteristic one. Closed, hole-bearing or malformed
charts refuse and request additional explicit cuts. No automatic seam selection,
conformal/optimal parameterization or general topology completion is claimed.

Each chart's circle map is uniformly scaled so its UV-pixel area matches source
surface area times requested density squared. Deterministic source-chart-order
shelf packing reserves square chart rectangles with explicit positive pixel
gutters. It never shrinks density to force a fit. This packing can refuse an atlas
that a more sophisticated packer could fit. UV orientation/positive area and
pairwise positive-area triangle overlap are checked before and after f32 delivery;
overlap decisions reuse canonical `orientation_2`. Delivered chart statistics
report actual f32 UV area and the minimum/maximum per-triangle area-derived texel
density. These are distortion diagnostics, not conformality or anisotropy bounds.
The density unit is **part-local source metres before assembly TRS**.

The public function has immutable inputs and caller-buffered public outputs.
Every returned error leaves all caller buffers unchanged. Input triangle order,
winding and source-vertex lineage survive cutting. The caller-selected scratch
and work limits are checked before allocating internal construction state.
The public `preflight_counts` reports the conservative `3 * triangle_count`
split-vertex maximum after checking counts, options, scratch and work.

Cancellation probes bracket chart solves and run during topology/overlap passes,
including the final pre-copy probe. The existing solver does not support stopping
within a solve, so worst cancellation latency includes one whole admitted chart
solve. There is no wall-clock deadline or parallel worker implementation. This is
Tier-2 ColdBounded construction outside `SlgArena`, never an evaluator hot loop.
The scratch estimate is conservative admission accounting, not a measured peak
heap benchmark or an allocator-failure recovery claim.

## Stable Poet adapter

New `native/src/graphical/surfaces/uv_atlas.rs` exposes:

- `AtlasDraft { version, seams, atlas_size, gutter_pixels, texels_per_metre,
  iterations }`, with strict serde and Vibe record parsing. Version 1 identifies
  this bounded harmonic-disk recipe; a different version refuses.
- `source(&AtlasDraft) -> Result<String,String>` and
  `from_value(&Value) -> Result<AtlasDraft,String>`.
- `preflight(&AtlasDraft, vertices, triangles, AtlasLimits) -> Result<usize,String>`;
  charges converted input, caller outputs and remapped derivative plus Core's
  independent construction budget.
- `apply(&AtlasDraft, &Mesh, Option<&[[f32;3]]>, AtlasLimits,
  &dyn Fn()->bool) -> Result<AtlasApplied,String>`.
- `AtlasApplied { mesh, normals, uv0, source_vertices, triangle_charts, charts }`.
  Normals duplicate exactly through the same source map as positions.

The current native assembly's admitted geometry schema is exactly `vertices`,
`triangles`, optional vertex-aligned `normals` and optional vertex-aligned `uv0`.
The raw geometry record remains the source master; the recipe derives render/
`.10d` geometry without rewriting the original topology, normals or UV0. Existing
assembly compilation attaches canonical UV/MAT sections, and existing HMC/checkpoint
owners retain raw source. No new scene codec, renderer or JavaScript is introduced.

The narrow Mesh API cannot discover discarded attributes. **Integration must keep
the existing unknown-geometry-field refusal before narrowing records**, including
tangents, skin, morphs, additional UV channels and unsupported extension streams.
Future support must map every corresponding stream through corner splits rather
than dropping them. Simultaneous non-null planar projection and atlas must refuse;
there is no implicit precedence. A nonidentity affine UV transform needs explicit
post-transform qualification or refusal while atlas diagnostics are presented as
final coverage/gutter/density evidence.

## Supervisor integration ownership

The root owns shared-file changes and native routing/UI. Required module hooks:
`pub mod uv_atlas;` in canonical computational geometry and native Poet surfaces.
Register `tests/uv_atlas_native.rs` as an actual public consumer test target in
Poet's selected Cargo manifest. The adapter's three native tests self-register
under its cfg-test submodule. Core's internal tests are additional unexecuted
coverage and are not a substitute for compiling the public consumer.

Root admission must charge worst-case corner expansion across the complete
assembly before cloning, and preserve the existing strict geometry-field
admission. Root source editing must change only the `uv_atlas` recipe while
retaining every unrelated part/source field and reject conflicting planar/affine
state. Root compilation and preview must use this same adapter. Root owns the
explicit selected-part inspection job and its stale-source guards; atlas solving
must not run for all parts during every UI render. Root also owns controls,
native preview, current project/session permission checks and durable
contribution/review/save/reopen verification.

The focused `uv_atlas/manifest.rs` supplies ColdBounded scalar metadata;
[registration.patch](registration.patch) gives exact minimal module/registry
hooks for root integration. Existing `capability_manifests/registry.rs` exceeds
the owner's file-size threshold; the one-line link to a focused body avoids
putting more implementation there, and decomposition remains an owner task.
The scalar capability is approximate metric/tolerance determinism across
platforms; same input/source and machine repeatability has a regression
definition, not an executed result. The constant's Foundation maturity is the
intended post-compilation state; this staged source is not compiled. Maturity
cannot be Verified/Done here, and Implemented requires the stated own tests.

## Regression definitions and remaining evidence

The ten public canonical cases cover curved disk positive coverage/density and
determinism; multiple explicit cut charts with disjoint gutter rectangles;
nonseparating cylinder seam with duplicated endpoints inside one chart; closed
tetrahedron refusal and explicit cuts; disconnected islands and insufficient
caller output; f32 delivery collapse; malformed indices/degeneracy/nonfinite/
nonmanifold input; malformed seams, budget, oversubscribed density and cancellation;
and cancellation at the last probe preserving the complete output buffers.
An arbitrary non-power-of-two resolution fixture checks delivered pixel-space
gutter containment with an explicit 0.001-pixel f32 tolerance. Reserved chart
rectangles remain exact integers; exact real-number normalized UV gutter is not
claimed. Texture edge dilation/filtering and mip padding remain separate work.

Poet's three cases cover exact normal/position source mapping and immutable raw
geometry, strict recipe/version/size refusal, and a recipe surviving actual
persistent Vibe source evaluation across edits. These do not establish durable
project save/reopen or actual native contributor/reviewer use. No build/test/export
was run by this worker. The source must undergo independent review, selected
compilation and actual regression execution before any applied/qualified claim.

All FND-01–06 and IP-03 C1–C8 remain open. No model/package completion or human
acceptance is implied. Film, open-world game and cartoon packages remain
incomplete; the existing 193 catalogue and eight subject-specific claims remain
unassessed and unchanged. Production assets remain on hold.
