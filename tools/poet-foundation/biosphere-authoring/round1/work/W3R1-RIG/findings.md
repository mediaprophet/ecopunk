# W3R1-RIG staged native rig binding

2026-10-26. Reviewed successor of the initial isolated implementation for
coordinator review. **Tests NOT RUN.**
No canonical Poet/Core source, Cargo manifest, renderer, controller, HMC writer,
production asset, external provider, commit or publication was changed. The
source files below are staged only; native integration and claims are open.

## Exact source seam and inspected scope

| Owner/API or caller | Inspected scope | SHA-256 |
|---|---|---|
| Core `container_10d::skeletal_section::{SkeletalSection,SkeletalData,ClipRecord}` | `mod.rs` whole; `binary.rs` lines 1–184 (validated parse/access/decode) | `ffb869b632b3bf324f08fdc86e0e60c245b7606237b4a06e59e93b2e3549f46f`; `8f665503f8e3509ab1aa1c67cbe7920a9723ddd3d7f82283d8cddb3c64d067d8` |
| Core `skeletal_container::read_skeletal_section` | `skeletal_container.rs` lines 1–101: sealed envelope, section and actual mesh-vertex association | `eba193a209d9ba832a4bbf4095a2343a7d8df02da42253b0dd22f72677d56f15` |
| Core skeletal `weights_from_segments_into`/`remap_skin_weights_into` | `authoring.rs` whole | `0d0193c2617b9a396ffd345e5221fc867ef64cc326918fbb3070e5e172339736` |
| Core `SkeletalPlayer` | `player.rs` whole: budget admission, clip/seek/joint controls, immutable bind frame, CPU positions/normals/tangents | `289e3656722da0852229342ca2426cdb9c613d1a001f62f05b28f6a817cff78b` |
| Core `PortalGpu::write_mesh_animated_frames` | `animated_upload.rs` whole: positions, normals, tangents, bounds and visibility | `83e705c4f1732224e33578d866120e952cba4cc3b0886fd89c92102c5596ea69` |
| Core portal existing consumer | `scene_mesh.rs` lines 70–170, 275–354; `scene_animation.rs` lines 100–175 | `3ac2407910cb9d18555fddfd76fbcd158ed640e6547aada758b3c78bb4fda58f`; `e73bb8f95211c9260c991eecf05d8c264dc001a8cabf0dccd62aa4c5ffa0d8ba` |
| Native Poet selected owner | `graphical/mod.rs` whole; `preview.rs` lines 1–55 and mesh/Portal references; `viewport_provider.rs` lines 1–125; renderer file located, upload owner to be rechecked by coordinator | `0603b4665a2e499339f313c24408e53c3a380d7b4cfb73c285e32435e93c0736`; `1ca1b62e39375e808acdda34cf2e972d633a6285c22398faf5f7834bf9fc668b`; `187b42d683694d3298aa9ed7da0ddd0e941fc30dbcad6009d02e058587aec5d6`; `2f97ea57e6f23b7fa551c563d6dc359fb154d0a9ef16a09adaa9e914c40e0ded` |

Applicable contracts read: `docs/planning/biosphere-authoring-round1.md`,
`docs/qa/implementation-2026-10-26/documentation-revision/native-route-study.md`,
`requirements-realignment.md`, `docs/planning/tech/assets/species-embodiment-spec.md`,
`docs/planning/tech/poet-capability-adaptation-guide.md`, and supplied AGENTS.md.
The existing SKL1 parser/player and .10d envelope are reused. No codec,
skinning engine, player or second renderer was introduced.

## Staged API

- `rig_binding::admit_bound_rig` takes a live `SourceRef`, durable
  `RigSourceDescriptor`, exact resolver key, selected sealed `.10d` bytes,
  `BindMesh` (positions, normals, UV0, local indices), viewport span and
  resident budget. It checks source stamp; selected resource SHA-256; checked
  SKL1 and QuantizedMesh; decoded rest coordinates and local triangle order;
  normal/UV source digest; exact joint/clip IDs; identity vertex lineage; and
  Core budgeted player construction. Core weight remap verifies the admitted
  identity path. Unknown remeshing, seams/flat-normal expansion and count/index
  mismatch refuse.
- `RigSourceDescriptor` is strict serializable source metadata: scoped source,
  part, bind and resource keys; SHA-256 evidence; exact joint/clip identifiers;
  vertex lineage; selected clip, seek and joint deltas. It contains no player
  buffers. `controls::apply_control` changes the actual Core player and a
  descriptor edit candidate; `descriptor_source_literal` supplies JSON for the
  existing expected-source/HMC route. The shared owner must authorize, check
  current source revision, and promote or reject that edit. JSON alone grants
  no authority. The Dioxus `RigControls` exposes clip, seek and joint translation
  actions to the same callback; it is not mounted yet.
- `AnimatedRigPreview::advance(dt, reduced_motion)` returns `AnimatedFrame`
  borrowed positions/normals/tangents and Core `PlaybackReceipt`.
  `write_frame(&mut PortalGpu, compiled_span)` calls the existing Core GPU
  writer. `frame()` now refuses a previously sampled frame after its source
  stamp retires, and mutable rig access invalidates its last receipt. The
  wrapper checks source freshness and span equality; shared
  renderer must prove compiled rest content and actor-range ownership.
- Admission checks the caller's working budget before Core section parsing for
  selected resource bytes plus a bounded section table, then counts selected
  resource bytes, section table, decoded mesh, temporary skin vectors,
  descriptor lineage and Core player estimate before allocating those cold
  vectors. The remaining budget is passed to Core's player. The caller sets
  this working limit; no new production-master ceiling is imposed. Nonzero
  authored clip fades are refused because the source descriptor has no
  transition snapshot for faithful reopen.

## Coordinator integration and required proof

1. Copy the five staged Rust files into new canonical Poet graphical paths.
   Register `pub mod rig_binding;` and `pub mod animated_rig_preview;` in
   `graphical/mod.rs`. Keep canonical Core unchanged.
2. Resolve the selected immutable HMC `.10d` resource under existing project
   authority. Persist descriptor as source-owned metadata with expected source
   revision, not as a private preview cache. On reopen, obtain a fresh
   `SourceRef`, same selected key/digest and source-owned bind/normal/UV
   descriptors; call admission again. Preserve other HMC graph/resources,
   rights and original rest mesh. Current selected MAT4 dependent-section
   refusal remains.
3. In shared assembly/renderer code, prove **before actor admission** that the
   compiled part's `first_vertex..first_vertex+vertex_count` is a nonoverlapping
   exact rest slice of the selected QuantizedMesh, with identical local triangle
   order and explicit scene index offsets, and its normals/UV0 match the
   source-owned descriptor. Pass that verified `ViewportSpan` and decoded
   `BindMesh`; never infer lineage from matching counts. This profile requires
   `vertex_lineage[i] == i`; any seam expansion, flat-normal expansion, atlas
   duplication or remesh needs a later qualified explicit normal/UV/weight
   correspondence and separate admission. Preserve original bind buffers when
   uploading posed frames.
4. Retain `AnimatedRigPreview` per selected actor, reject retired source,
   cancellation or changed range before publication, call `advance`, and for
   GPU path call `write_frame` with the compiler-proven span. Bind Dioxus
   `RigControls` and MCP actions only through existing authenticated
   AuthoringEdit/expected-source route; stage literal, validate and submit,
   then reconstruct on accepted source. Test normals/tangents, draw bounds,
   picking, reduced motion, stale rejection, save/reopen and reviewer flow.
5. Run serial Poet checks only after shared registration. Staged source tests
   are not executed evidence. The fixture builds a three-vertex triangle with
   Core `compile_mesh_to_10d_with_nodes`, attaches one valid SKL1 clip/one
   joint through `attach_skeletal_section_into`, then exercises the actual
   player and frame slices, small budgets, stale source refusal and nonzero
   fade refusal; no wallaby morphology claim follows.

## Status and remaining IDs

Source implementation staged; execution **NOT RUN**; human/domain acceptance
**unassessed**; asset production **HOLD**. W3R1-RIG advances bounded BS-20A
admission and first BS-20B/C adapter code. BS-20B compiled topology/weight
correspondence and correction, BS-20C actual mounted viewport/control/picking,
BS-20D HMC rigged rebuild, BS-20E contributor/reviewer use, IP-03 C1–C8 and
film/game/cartoon accepted wallaby model claims all remain open.
