# EffectCraft admission and Poet adaptation review

**Date:** 2026-10-26. **Current scope:** source research, requirements reconciliation,
documentation and prepared handoffs. No engine/source refactoring, build, test,
export, GUI, network/provider invocation or asset production occurred. No native
compositor, product or model acceptance is claimed.

The additional source is `C:/github/storytold/effectcraft-main`. It provides useful
compositing/motion-graphics document, layer, keyframe, effect, command, history and
render/export capabilities to adapt into existing Poet requirements. Reuse is
**requirements-first**: source applications accelerate the ecosystem; their shell,
authority, scripting and file/runtime assumptions do not define its replacement.

Start with the [adaptation guide](../../../planning/tech/poet-capability-adaptation-guide.md).
It includes earlier POET-STU/ST requirements, observer/Manifold/container semantics,
real current implementation examples and one future editable composition case.
It applies to earlier source-tool families as well as this additional source.
The [comprehensive authoring review](../../../planning/tech/poet-comprehensive-authoring-review.md)
and governing implementation phase order remain in force.

## What was actually studied

| Saved evidence | Bounded inspection | What remains unproven |
|---|---|---|
| [Source admission](source-admission.md) / [coverage](source-admission.coverage.json) | Static parsing of 38 visible manifests, 37 packages, 351 direct dependency edges and 691 lock records; source licence/notice/asset/brand declarations. No implementation bodies. | Checkout provenance unavailable; exact remote pins versus related local forks, transitive source/licence/feature closure and runtime remain unverified. Counts are declarations, not reusable or working feature counts. |
| [Engine routes](engine-authoring.md) / [coverage](engine-authoring.coverage.json) | Fourteen selected implementation/caller files tracing composition/layer/effect/key edits, history/revision, frame/save/reopen and selected agent/export producer routes. API/test/metadata discovery is separate. | No new execution, every-effect/codec qualification, whole job lifecycle/cancellation, native adapter, HMC continuity or human acceptance. |
| [UI interactions](ui-functions.md) / [coverage](ui-functions.coverage.json) | Eight selected source-body files, representative command/panel/timeline/property/graph/history/job behaviours; separate matrix for eight governing UI categories. | Exhaustive command/UI parity, context-menu/dialog details, accessibility and actual native user operation. WASM seed observations are not native defaults. |
| [Existing ecosystem examples](ecosystem-examples.coverage.json) | Eight current production implementation excerpts, two selected test-source excerpts, six earlier specification excerpts and one module-exposure excerpt. | Not new test execution or proof those domain-specific interfaces already support compositions. Examples require explicit adaptation. |
| [Independent guide review](independent-review.md) | Document-only check of requirements, referents, Vibe/Aether/Etalon boundaries, human/agent workflow and evidence claims. | Does not re-audit all implementation or establish product acceptance. Root resolutions below address its documentation findings. |

Exact whole-file hashes/ranges pin the inspected source versions; they do not
mean whole-file semantic inspection or a clean/reproducible checkout. Metadata
was parsed directly without Cargo resolution. The historical eleven-root /
324-manifest inventory is preserved separately. This is an additional twelfth
source root, not a fresh audit of every earlier tool.

## Reusable capability and concrete integration limits

- `comp.new`, `layer.newSolid`, `effect.apply` and `prop.addKey` lead to actual
  source model/history/revision and compositor/render code. Specific effect
  kernels and output quality still require selected oracles.
- Lower `Session` defaults omit expression/export/acceleration hosts, while the
  complete EffectCraft host installs Boa expressions/scripts. Preserve imported
  expressions inertly and hold unsupported execution until an explicit compatible
  Vibe/native profile exists. Rust implementation alone does not remove JavaScript.
- Command enablement and the sampled default-unconfined MCP backend do not
  provide native Poet principal/revision/rights admission. The existing shared
  NativeProject/AuthorizedEditorHost/IntentBus examples are the integration guide.
- `.ecproj` serializes project/view; replacing/reopening the project clears domain
  session history. Preserve full editable sources and native history, identifiers,
  resource/graph/provenance and contributor/reviewer records explicitly through
  the existing package owners. Ordinary save/reopen is not complete HMC recovery.
- Registry wgpu, patched egui/eframe/winit, direct file services, plugin loader and
  export paths need specific Etalon/Aether/host reconciliation. Keep source notices;
  restricted source brand marks are not automatically imported into Poet.
- Cancellation has selected producer-body support but not a fully reviewed
  queue/MCP/cleanup contract. Display actual acknowledged outcomes only.

The related `C:/github/poet-filmcraft` source already exists and has earlier review
evidence. It must not be assumed equal to EffectCraft's locked remote FilmCraft
revision. The admission report distinguishes local references from verified pins.
No absence of a differently named checkout justifies building replacement codecs.

## Handoff and review resolutions

Use the existing [job register](../../../planning/biosphere-work-packets.json):
PA-R01 reconciles shared domain contracts/authority, PA-R02 consumes the workspace
and UI mapping, PA-R06 specifies the selected composition workflow alongside the
separate NLE/scene-render owners, and PA-R12 reconciles cooperative qualification.
No extra full-product phase or broad new audit is created. Every source-family
job now names an existing implementation example, its limits and required changes.
Workers keep disjoint outputs; shared source/manifest changes and serial meaningful
checks belong to the coordinator when implementation resumes.

The independent review's missing-index finding is resolved by this file; its
ambiguous spectral abbreviation is expanded in the guide, which now directly
links the [governing UI-category matrix](ui-functions.md#governing-ui-category-coverage).
Context menus, dialogs and accessibility remain explicit unknowns. These are
documentation resolutions, not runtime fixes or accepted UI cases.

[Verification receipt](review-verification.json) records source-pin and document/
register consistency checks. Full PC/TOOL/PA, IP-03 C1–C8 and foundation gates
remain open. Species film/open-world-game/cartoon packages remain incomplete;
asset-production HOLD is unchanged. Payment rails remain later work.
