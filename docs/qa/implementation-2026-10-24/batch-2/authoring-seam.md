# RUN-06A — native authoring seam

**Inspection only · 2026-10-24.** No sibling source was changed, moved, built or
run. This identifies the smallest credible starting seam after the Vibe session
checker; it does not claim usable native authoring or IP-03 completion.

## Finding

The inspected Poet/CADCraft/host paths did not establish a complete native Poet,
Aether and Craft 3D authoring seam. Within that limited inspection, one candidate
is **one Poet scene/document container backed
by a single revisioned source document, with CADCraft's existing editable
`Drawing`/constraint model as a bounded 2D construction layer**. It is a concrete
recovery/editor slice, but it cannot itself represent a production organism,
assembly, material graph, or HMC geometry. Treat it as an interface prototype
only until the missing 3D geometry/material authority is selected from the
Qualia owners and qualified. Do not call it the reference asset authoring path.

**Supervisor qualification:** this inspection did not audit the canonical Qualia
3D authoring kernels. RUN-06B subsequently found existing loft/surface/part and
material APIs and is investigating the real Studio source-store undo/revision
path. This 2D candidate is not the governing first implementation slice and
cannot establish that 3D capability is absent. Exceptional 3D master production
remains the priority; see `native-3d/` in this batch for the bounded contribution.

Why this is the smallest credible seam:

* Poet's current native package remains the Vibe CLI; `C:/github/poet/wasm/src/tool_chest/core/registry.rs::ManifoldSeed` and `.../manifolds/studio.rs::studio_manifold_seed` already describe a shared studio surface with `dual_studio`, `scene_view`, and `audio_session`. `wasm/src/tool_chest/core/intent_bus.rs` defines structured Vibe intents, target identifiers and provenance. These are recovery/contract inputs; the browser UI is DOM/web-sys and is not the native renderer.
* Aether is the specified shared UI framework, but the inspected Poet native target is not a graphical Aether host. The exact mounted native app, Aether version/backend, document binding and renderer path are missing. A `scene_view` seed alone is just layout data.
* CADCraft `C:/github/poet-cadcraft/crates/doc/src/lib.rs::Drawing` owns a copy-on-write `EntityStore`; `EntityStore::clone` is explicitly documented as a cheap undo snapshot. `Drawing` retains layers, blocks, layouts, views, constraints and parameters. `cadcraft-constraints/src/lib.rs::solve/add/set_parameter` edits its parametric 2D model. `EntityKind` includes `Polyline3d` and `Face3d`, but this is a CAD drawing representation: its own architecture/gaps say no 3D solid/surface/mesh model, and its `Solid` is 2D. These pieces are a reusable document/constraint engine, not an organic/assembly modeler.
* Material ownership is absent from this seam. CADCraft entity color/transparency is drafting display metadata, not semantic PBR material/map authoring. PhotoCraft's image/mask processing is a likely later source adapter, not yet a demonstrated material document round-trip.

## State owners and cooperative path

Keep one document authority. The CADCraft `Drawing` (or a future chosen 3D
document) owns editable geometry/parameters; add an adapter-owned `document_id`,
source revision, part/subject identifier associations, provenance and immutable
checkpoint references around it. Do not store parallel geometry in a Poet UI
signal, Aether component tree, browser manifold, `wellfair` contribution record,
or viewport cache. Viewport selection/camera is projection state; preview output
is derived and must point back to the source revision.

An existing native cooperative path is present in Webizen Desktop, but not wired
to an asset document: `src/commands/wellfair/projects.rs` exposes
`wellfair_add_project`, `wellfair_add_project_membership` (steward/contributor/
observer), and `wellfair_add_contribution`; `work_items.rs` exposes
`wellfair_add_work_item`, `wellfair_add_work_item_status`, and
`wellfair_work_item_board` (including `in_review`). They commit through the
existing host and `wellfare_core`/`qualia_cooperative_core` record types. This
is project/contribution/work tracking, not source revision, reviewer decision,
or authorization enforcement for CAD edits. Existing `poet.rs::poet_eval` and
`vibe_host.rs::vibe_check` / `vibe_capability_invoke` are Vibe entry points, not
document operations. `poet_daemon.rs::http_invoke` is a daemon bridge, not proof
of native GUI integration. Add a typed adapter across these owners; do not
duplicate the project or contribution ledger.

## One bounded vertical slice to target

Use a **single identified tree branch-and-trunk source** as the first editable
organic case, with a small identified manufactured bracket/joint document as the
assembly control. Neither may be reduced to a capsule/primitive generator. After
the session checker, implement a native Poet scene document command path that
opens and checkpoints one source revision; edits selected geometry through a
Craft/Qualia public operator; previews the actual resulting mesh/materials in
the qualified native viewport; records a contributor checkpoint and reviewer
decision through existing project/work-item ownership; then reproduces the
identified HMC root and a downstream preview from that checkpoint. Before the
tree case can be represented as 3D, choose and bind the actual organic geometry
operator and material source; CADCraft is only the 2D/parametric document control
case. Keep tree and bracket sources, IDs, authorship, target profiles and review
status distinct. Master quality is not bounded by game-demo budgets.

This is a proposed implementation seam and acceptance chain, not an assertion
that those operations are presently available. Required 3D operator/material,
native preview, source-to-HMC codec, shared authorization/revision semantics and
review UI remain blockers. If the Qualia owner cannot identify an existing
public 3D operator and source-to-HMC handoff, stop at the interface contract and
escalate owner/API selection; do not fill the hole with a local primitive
generator or renderer-specific substitute.

## IP-03 C1–C8 status from inspected source

All eight remain **unqualified**. Existing project/work-item records are not
execution evidence for this slice.

| Case | Current evidence / exact deficit |
|---|---|
| C1 definition | Project and work-item records exist; no shared versioned brief bound to the document, source/parts, target criteria and decisions. |
| C2 cooperation | Membership/contribution commands exist; no demonstrated distinct contributor/reviewer sessions authorized on the same source authority. |
| C3 real authoring | CADCraft has editable 2D entities/constraints; no native graphical Poet/Aether controls, qualified 3D organic/assembly operators, semantic materials or native viewport. |
| C4 revisions | CADCraft snapshots can support in-memory undo; no persistent document revision/checkpoint/reopen, attribution-preserving reviewer flow or integrated UI. |
| C5 conflict safety | No revision-checked document commands, stale-edit rejection, or late/cancelled job protection wired to source commits. |
| C6 production | No demonstrated source → admitted HMC/root → downstream preview path from this authoring document; invalid-resource diagnosis absent. |
| C7 recovery | No demonstrated disconnect/restart/rejoin recovery of document sources, pending edits, decisions and authority. |
| C8 usability/evidence | Manifold seed/CLI/host commands do not show usable native authoring or retained capture/input/result/human-acceptance evidence. |

## Next code edits (not performed)

1. **After the checker:** in Poet's shared native document/session contract, define
   `DocumentRef { subject identifiers, source revision, expected revision }`,
   command receipts and checkpoint/reopen semantics. Preserve multiple qualified
   identifiers; use the engine's authoritative project record/role checks.
2. Add a headless adapter for CADCraft `Drawing` open/edit/clone/commit that
   accepts expected revision and produces a source digest/checkpoint; use the
   existing solver/operators and test stale edits. Keep this clearly 2D until a
   selected public 3D engine contract is available.
3. Bind Poet's shared `scene_view`/tool-chest command model to that same session;
   mount it in an actual Aether native host and provide selection, property
   inspection, undo, save/reopen and incremental preview controls. A manifold
   seed or placeholder controls do not pass acceptance.
4. Add the native geometry/material adapter only after the owning Qualia API is
   selected; preserve editable operator/material sources and source-to-output
   IDs rather than baking a mesh. The tree and bracket control cases must both
   exercise the real path.
5. Bind checkpoint/review actions to Webizen's existing project membership,
   contribution and work-item commands; add revision-bound review decisions,
   restart recovery and reproducible HMC/preview evidence. Keep contribution
   records and source rights visible; no payment rail is needed for this slice.

## Boundary and provenance

`poet`, `poet-cadcraft`, `webizen-desktop`, `rust-app-framework` (Aether) and
`qualiaDB` remain separate source/authority owners. The Craft consolidation
specification does not authorize moving source or altering licenses; preserve
CADCraft copyright/notice and source history if a later authorized migration is
planned. Poet Tool-Chest/Manifolds and old UI are recovery references, not proof
of the missing native projection. Chora remains separate and is not part of this
seam. The static Craft inventory is discovery metadata, not library qualification.

## Pinned source files

* Ecopunk contracts: `docs/planning/architecture.md`,
  `docs/planning/tech/development-modes.md`, `docs/planning/made-world.md`,
  `docs/planning/tech/poet-craft-consolidation.md`,
  `docs/planning/tech/assets/production-toolchain-spec.md`,
  `docs/planning/implementation-plan.md` (IP-03 C1–C8), and
  `docs/planning/tech/craft-consolidation/source-inventory.json`.
* Poet: `C:/github/poet/wasm/src/tool_chest/core/registry.rs`,
  `.../core/intent_bus.rs`, `.../manifolds/studio.rs`,
  `C:/github/poet/native/Cargo.toml` (CLI target).
* Craft: `C:/github/poet-cadcraft/crates/doc/src/lib.rs`,
  `.../doc/src/entity.rs`, `.../doc/src/store.rs`,
  `.../constraints/src/lib.rs`, `C:/github/poet-cadcraft/docs/architecture.md`,
  `.../docs/gaps.md`.
* Host: `C:/github/webizen-desktop/src/commands/poet.rs`, `.../vibe_host.rs`,
  `.../poet_daemon.rs`, `.../wellfair/projects.rs`,
  `.../wellfair/work_items.rs`.

No source history, runtime behavior, hardware, human review, HMC output or gate
pass was established by this read-only inspection.
