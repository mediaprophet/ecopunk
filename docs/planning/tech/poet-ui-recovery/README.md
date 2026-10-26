# Poet UI/UX migration inventory

**Status:** bounded source inventory; every migration row remains `pending`. This document and its machine index are planning/evidence aids. They do not establish parity, recovery, implementation, runtime behavior, human acceptance, or exhaustive UI coverage.

Poet is the one authoring product. Craft repositories preserve source behavior and useful editing functions for integration into existing Poet constructs, manifolds, content containers, Tool Chest, command/context surfaces, inspectors and timelines. Former Craft application shells are migration references; imported libraries support Poet implementation and do not replace Poet with another UI or suite.

## Governing migration rules

The [Poet/Craft consolidation specification](../poet-craft-consolidation.md) §3 requires each source command, panel, gesture, shortcut, context-menu item, dialog, view and accessibility behavior to map to a Poet destination and disposition. Its §4 makes Poet own authoring navigation/composition while integrating the useful medium-specific functionality. PC-01 through PC-07 remain open; file presence, selected source files, or this inventory do not meet those gates.

The machine index, [ui-function-inventory.json](ui-function-inventory.json), names all eleven `C:/github/poet-*craft` repositories, selected real UI/registry source units, and concrete feature rows. It also retains cross-repository rows for all eight required UI categories. The rows are deliberately pending because no Poet parity implementation or validation is claimed by this read-only inventory.

The separate [Craft manifest inventory](../craft-consolidation/source-inventory.json) enumerates 324 Cargo manifest candidates across eleven repositories (271 under crate trees). It is not an exhaustive UI function inventory, dependency resolution, source-history review, or proof that every manifest is a reusable production library.

## Source coverage boundary

Each contributing repository has a current source documentation baseline except ArtCraft, whose index cites its README and React/Tauri frontend instead of a `docs/ui-parity.md`. Several parity pages are dated October 10–11, use different reference products and methods, and include self-reported percentages and known missing/partial behavior. They are useful source pointers, not comparable readiness measurements or verified behavior in this pass.

The source index lists representative UI roots and command registries, not every file. The following item-level families still need to be enumerated from actual source and reconciled against tests, manifests, release notes and local-only work plans:

- Every command/action ID, aliases, menu/palette/toolbar/Tool Chest exposure and invocation path.
- Every panel, dock, inspector, timeline, container, enabled/disabled state and context association.
- Every selection, pointer/keyboard/touch gesture, modifier, shortcut, focus/IME, preview/commit/cancel and undo/recovery transition.
- Every context-menu location/item, dialog/pane, view and view-state transition.
- Every accessibility tree/label, keyboard-only workflow, focus order, screen-reader behavior, contrast/text-scale/reduced-motion option and assistive alternative.
- Format/import/export, persistence, session, source revision, provenance, provider, platform and capability seams for each feature.

This inventory is therefore not a “recovered” feature list. Item-level mapping remains required before declaring any UI family retained, ported, combined or replaced. Rows use `pending` until a concrete Poet target, source behavior comparison and actual evidence exist.

## Historical growing-station slice and current native reconciliation

The initial growing-station slice selected hierarchy/part selection, numeric
TRS/constraints and materials/UV inspection. Its older observations are superseded
where the [current Poet review](../../../qa/implementation-2026-10-26/poet-tooling-review/poet-authoring.md)
and [Round 9 evidence](../../../qa/implementation-2026-10-26/round9/progress.md)
record actual later work:

- Source/part selection, typed native intents, shared-model projection, numeric
  edits, retained viewport and source-bound frames are implemented in the selected
  native route. This is not full Manifold/multidocument parity or a 3D gizmo.
- Surface maps, UV projection/transform/atlas and texture/detail editing exist;
  the older factor-only/rejected-texture statement is no longer current. General
  layer/image/RAW editor parity and arbitrary rig/topology preservation remain open.
- Native contributor/reviewer projections and source/HMC recovery have scoped
  software evidence. Actual human operation and acceptance remain open.

The [comprehensive authoring review](../poet-comprehensive-authoring-review.md)
restores the full image/RAW/vector/audio/film/text/PDF/grid/deck/spatial/optional
assistance scope for people and agents. The rack is an engineering fixture,
not a reason to restrict Poet to CAD or replace the original workspace.

## Existing Poet reuse hooks and integration seams

Reuse the existing typed product model:

- `C:/github/poet/wasm/src/tool_chest/core/construct.rs::ConstructSeed` for project/observer scope.
- `C:/github/poet/wasm/src/tool_chest/core/registry.rs::ManifoldSeed, SeedContainer, ContainerKind` for workspace composition and a 3D/assembly content container plus panels.
- `C:/github/poet/wasm/src/tool_chest/core/tool.rs::Tool/ToolMetadata` and registry/tool-chain/toolbox models for actions and Tool Chest placement.
- `C:/github/poet/wasm/src/tool_chest/core/intent_bus.rs::VibeScriptPayload/IntentBus` for versioned commands, target context, capability gating, provenance and receipts.
- Existing Poet native authoring/session and QualiaDB material, assembly, renderer and checkpoint/HMC owners for one canonical source-to-output state.

The current native `poet_workspace/intents.rs` implements the original shared
IntentBus, and `native_dispatch.rs` routes selected authenticated actions through
it. `poet_workspace/model.rs` reuses Construct/Studio seeds. The older inventory's
"no native IntentBus" finding is superseded for this selected source/spatial
route. Browser `WasmIntentBus` and current native adapters remain target-specific;
this is not a claim of all-document/native/WASM parity.

`qualia-app-contract` already provides application adapter, command/context,
session/surface/gesture/job and delegation contracts. Reconcile it with existing
Poet types and actual domain engines; do not create a parallel DTO crate. Trait
methods and availability defaults are not functional editor or recovery proof.
Canonical documents belong to their authoritative domain owners; container
view/layout state or content HTML does not become their source database.

## Remaining work

The original 30 feature/category parity rows remain pending; selected current native wiring/evidence is recorded separately so those pending rows are not mistaken for wholly missing functionality. The inventory must be expanded from representative source pointers to actual item-level registries and behavior, with source/version/test references and an explicit Poet disposition for each row. Then integrate and qualify the first native source-to-output vertical slice through existing Poet navigation/selection/commands, including contributor/reviewer source work and required human usability acceptance. Image, vector, audio, film, document, grid, deck, PDF and light domains remain visible in the index and are not dropped by focusing first on the growing station.

## Additional compositor source — 26 October

[EffectCraft](../../../qa/implementation-2026-10-26/effectcraft-review/README.md)
is a twelfth source root, outside the historical eleven `poet-*craft` roots.
The original thirty pending rows remain; eight additional category rows cover
this source with explicit unknowns. Its eight feature groups are not the same
as the eight governing UI categories. Context-menu/dialog details and
accessibility still need item-level investigation and actual native evidence.
Use the [requirements-first guide](../poet-capability-adaptation-guide.md) and
existing Poet/QualiaDB implementations, preserving the observer/Manifold/container
referents and existing functionality. Native compositor integration is open.
