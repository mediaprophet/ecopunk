# RUN-06B — source-safe native Studio edits

2026-10-24. **Historical staging/test record; exact candidate subsequently
applied by the supervisor.** See [applied-source verification](../applied-sources.json)
and the [batch integration record](../README.md) for later target qualification.
The staging manifests and test receipts below retain their original state.
This repairs actual
source restoration in the existing canonical Studio store. It does not deliver
graphical/cooperative authoring, an organic editor, an HMC embodiment or a model.

## Evidence and chosen contribution

The RUN-06A CADCraft inspection did not establish an absent 3D engine. Canonical
QualiaDB has the following relevant public source. Paths below are relative to
`C:/github/qualiaDB/crates/qualia-core-db/src/`; pinned files are recorded in
`source-evidence.json`.

| Source/API | Actual scope and remaining seam |
| --- | --- |
| `specialized_libs/computational_geometry/authoring.rs`: `drag_vertex`, `boolean_op`, `Scene`, `SceneNode`, `compose_trs` | Mesh edits/transforms and composition exist. Boolean dispatch uses `boolean_3`, with exact orientation classification and approximate f64 construction. This is not proof of a complete organic editor or cooperative document. |
| `.../authoring_loft.rs`: `loft_convex_sections_into` | Caller-owned closed-loft output from 2–256 convex XY sections, 3–128 points per ring and strictly increasing Z. Useful organic/assembly surface construction; not arbitrary sculpting, topology editing or production quality acceptance. |
| `.../surface_mesh_processing.rs`; `.../nary_csg.rs` | Area/volume measurement; n-ary **2D** polygon CSG. Neither file alone establishes a 3D Boolean deficit or a whole organic modeling capability. |
| `render/surface_authoring.rs`: `SurfaceRecipe`, `generate_surface_maps_into`, `planar_uvs_into`, `encode_surface_png_into` | Deterministic editable parameters and colour/normal/ORM outputs exist. Current map builder admits sides 8–1024. This is a scoped operator limit, not Ecopunk's master-quality ceiling. |
| `render/skeletal/authoring.rs`: `weights_from_segments_into`, `remap_skin_weights_into` | Existing bounded skin-authoring/seam-remap path; not evidence of all rig/clip/deformation authoring obligations. |
| `hypermedia_authoring/world3d.rs`: `Object3D`, `Material3D`, `Scene3D` | Identified object/parent/mesh/material, transform, camera/light/rig/clip records and editing methods exist. No integrated source revision/undo/Poet viewport was established by these records. |
| `poet_host/invoke/geometry/{mod,wave27_host,wave28_host,wave29_host}.rs` | Public Vibe geometry seams already include extrusion, tree skeleton/colonisation, growth, skinning, swung surfaces and pipe radii. Geometry output is not automatically installed in a revisioned native scene document. |
| `poet_host/invoke/manifold/project.rs`: `project` | Presentation coordinate projection exists. Returned coordinates and `honesty` text do not demonstrate a mounted graphical native viewport. |
| `render/compile_10d.rs`: `compile_asset`, `CompiledAsset`; `render/asset_package.rs`: `build_hmc_bundle` | Actual source-asset → binary `.10d`/facts/images → HMC resource path exists. Preservation of this Studio source document, project contributions, review and recovery through that path remains unqualified. |

`poet/native` is the inspected CLI. Poet's existing `wasm/src/tool_chest` intent
bus/manifold/scene container semantics remain recovery inputs. The inspected
`webizen-os-shell` Poet stage is descriptive; Aether's `packages/native` is the
Blitz/Vello candidate, distinct from its webview `desktop`. This work neither
mounts nor globally disproves an existing native application. Chora is untouched
and remains outside all source/licensing consolidation.

The concrete defect was `studio_store.rs::undo_artifact`: it popped a string,
decremented the clock and inserted `_undo_last`, leaving edited title, nodes and
properties unchanged. Redo also changed only markers. The retained baseline
regression executes that exact source and fails because the title stays edited.
This is a source-loss/recovery blocker before native geometry controls can be
trusted, so the bounded contribution repairs this existing state owner.

## Candidate behavior

`candidate/studio_store.rs` retains the store and existing public call shapes.
`studio_journal.rs` holds typed source snapshots beside each artifact in the same
locked store entry. `StudioArtifact` retains its original public source shape;
there is no second database, renderer or private wire codec. Undo/redo restore title, nodes
and properties. Every successful edit/undo/redo advances the existing 29-bit
revision; clock exhaustion is refused rather than wrapped. History remains
append-only. Typed identifiers/domain/routing/sensitivity/context are protected
from source-edit callbacks and never restored from snapshots.

`mutate_artifact_at`, `undo_artifact_at` and `redo_artifact_at` compare the expected
revision under the same existing mutex as activation. `Studio.edit`, `Studio.undo`
and `Studio.redo` use those paths; an optional positive 29-bit
`expected_revision` enables conflict rejection. Malformed supplied values fail.
Omitting it preserves legacy callers and provides no stale-command protection.
The old replacement `upsert_artifact` remains unchecked and invalidates snapshots
so they cannot accidentally restore a replaced source.

`configure_artifact_journal` is a typed **native host Rust API**, not a new Vibe
capability or an authority grant. It admits deliberate caller-selected larger
source/journal/step/audit/field budgets without discarding retained state. Defaults
are 1 MiB source charge, 16 MiB retained journal charge, 64 source snapshots,
256 audit events and 16,384 fields. The charge includes UTF-8 bytes and a
conservative 256-byte per-field allowance; it is not a measured allocator or
whole-process memory bound. Native edit callbacks remain trusted code: their
transient allocations are not intercepted. Retained source/state is admitted
before activation; script/provider authority cannot expand these limits by an
ordinary source edit. Full high-grade masters may be deliberately admitted by
the owning host and are not required to fit these defaults.

Generic string `properties` remain editable source. They are not a typed rights,
identifier-association or contribution ledger. This repair cannot infer which
arbitrary property names carry authoritative provenance; those records must stay
with the existing authoritative project/rights owners. Tests preserve fixture
subject/part/author fields, but do not prove adoption, rights or compensation.

## Verification and limits

`harness/lib.rs` compiles exact candidate store/domain modules and canonical
argument helpers with real cached `qualia_core_db` and Vibe artifacts. Its kernel
calls use the actual core's public loft/surface APIs. No substitute geometry,
type declarations, checker or runtime is supplied. `run-checks.ps1` records exact
compiler/runtime logs; `verification.json` pins the artifacts and final results.
This qualifies isolated candidate behavior against those cached dependencies;
current whole-core compilation/linkage after upstream apply remains unrun.

Final focused journal tests: **10 passed**. All mounted store/domain/regression
tests: **16 passed, 1 failed**. The failed existing
`test_import_cml_and_csv` expects inline CML `stroke` parsing; the exact original
module reproduces the same failure. Original undo regression also fails because
the source title is not restored; the candidate passes. This repair does not fix
or waive the separate CML importer defect. Logs retain the failures explicitly.

The concurrent regression releases two native threads from one barrier against
the same base revision with distinct edits. Exactly one edit succeeds; the stale
callback never runs. Winning source/audit and the single undo snapshot are
preserved. This proves the local atomic revision primitive only; network,
membership, contributor roles and reviewer authority remain unqualified.

The test-only source fixture edits one loft control point and roughness, then
regenerates geometry and ORM maps. Undo reproduces the original output; redo
reproduces the edited output. This is kernel/source regression evidence, **not**
an installed organic production binding, rendered preview, export or model.
`StudioArtifact::export_format("10d"/"e64")` returns a JSON envelope; it must not
be treated as the binary `.10d` or HMC proof described above.

The existing `studio_domains.rs` receipt helper also preserves placeholder
results. `Vector.path_boolean` reports 64 vertices; `Vector.bezier_offset` reports
18 segments; `Vector.svg_compile` reports 12 nodes; `Vector.extrude_3d` reports
240 triangles/128 vertices. `Cad.sketch_constrain` reports convergence/zero DOF;
`Cad.brep_solid` reports 6 faces/12 edges/8 vertices; `Cad.dxf_dwg_io` reports
320 entities/5 layers; `Cad.dimension_measure` reports 25.4 and 90 degrees.
These functions do not call the corresponding production kernels. Their
existing receipts/parity flags are not geometry, solver, format, measurement,
Craft, export or human-review qualification. Document/RAW/timeline fixed-result
handlers likewise remain unqualified and unchanged by this repair.

`poet_host/invoke/render/scene_graph.rs` also distinguishes planning/data
records from execution: `Scene.create`, `Scene.add_node`, `Scene.set_transform`
and `Scene.set_mesh` return records rather than installing a revisioned source
scene; `Scene.render` explicitly returns a planning request and does not call
the GPU frame path. Some neighboring functions, such as IK, do call real
kernels. Do not infer their status from the namespace alone. The next geometry
binding must route authored profile/part data through actual extrusion/loft/3D
Boolean/measurement/solver APIs and bind revisioned output to the real native
GPU upload/frame path. Packaging must use the actual binary compiler/HMC APIs,
preserving editable sources, rather than the placeholder Studio exporters.

The store is in memory. Durable checkpoint/reopen, explicit native source-recipe
schema/operator bindings, source→HMC editable-source retention, cancellation/job
activation, project/reviewer authorization and a usable Aether/Poet viewport
remain open. Existing unchecked get/edit/upsert receipt producers are not
silently upgraded to cooperative transactions. Full rights/graph/embodiment
content is not claimed to reside in this store.

Next integration should bind an existing organic/assembly operator to a defined
canonical source recipe and revision, then derive native preview/10D/HMC through
existing owners, rejecting stale output. Its controls belong in Poet's existing
scene/manifold, with project roles and review actions supplied by existing host
services. No new 2D app/prototype is a substitute for that work. IP-03 C1–C8,
ECO-034/044 and TOOL-04–07/11–12 remain open beyond this bounded repair. No model
package was created or changed; film/game/cartoon and human acceptance are not
claimed complete.
