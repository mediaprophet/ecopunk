# Poet comprehensive authoring: source review and refactoring plan

**Revision:** 2026-10-26 · **Status:** research/documentation; no new code,
refactoring, build, runtime, provider invocation or acceptance is claimed.

Poet was intended to become a comprehensive authoring environment for **people
and agents**, not merely a CAD viewport or an API used to generate assets.
The original Tool Chest, Constructs, Manifolds, content containers, contextual
tools, inspectors and stage/timelines remain the product continuity baseline.
The source tools contribute internal capabilities and editing behaviour to Poet;
their independent app shells and former product names are migration references.

This review expands [PC-01–08](poet-craft-consolidation.md) and
[TOOL-01–12](assets/production-toolchain-spec.md), under the single
[implementation plan](../implementation-plan.md). It uses the
[QualiaDB audit](../../qa/implementation-2026-10-26/biosphere-integration-review/README.md)
and [focused biological route study](../../qa/implementation-2026-10-26/documentation-revision/native-route-study.md).
It does not replace the existing phase order, model catalogue or source owners.

## Additional source and adaptation method — EffectCraft

The additional source `C:/github/storytold/effectcraft-main` supplies compositing,
motion-graphics, layers/keyframes and effect-control candidates. Its
[bounded review](../../qa/implementation-2026-10-26/effectcraft-review/README.md)
extends the original eleven-family source study; historical counts/read coverage
remain unchanged. It expands PA-06/PA-12 and relevant PA-01/02/11 cases, rather
than creating a separate product or a new governing phase.

Use the [requirements-first adaptation guide](poet-capability-adaptation-guide.md)
for this and earlier source families. It supplies real existing Poet/QualiaDB
implementation examples and the older POET-STU/ST requirements they serve.
A Construct is the observer/agent's coherent scope, not a source application,
composition or project. Domain state, host mechanisms, shared semantics and
human presentation must be reconciled explicitly. Source app architecture and
branding do not supersede those requirements. No compositor integration is
claimed by this review.

## What the new review establishes

| Source review | Scope and result |
|---|---|
| [Current Poet human/agent routes](../../qa/implementation-2026-10-26/poet-tooling-review/poet-authoring.md) | Current shared/native/WASM composition, intent, command, source/job, cooperative and MCP paths; distinguishes mounted spatial functionality from held/unqualified other document surfaces. |
| [Image, RAW, vector, audio, film](../../qa/implementation-2026-10-26/poet-tooling-review/media-tools.md) | Source document engines, edits/history, processing/render/export, automation and interaction patterns across five repositories, with exact bounded coverage. |
| [Text, PDF, grid, presentation, optional generation and CAD](../../qa/implementation-2026-10-26/poet-tooling-review/document-data-tools.md) | Existing models/commands and selected operation bodies across six repositories, including session/protection/history, cell edits, PDF batch actions, headless presentation and generation enqueue paths. |
| [Shared application contract](../../qa/implementation-2026-10-26/poet-tooling-review/qualia-app-contract-source-review.json) | `qualia-app-contract` already exists, with command/context/session/surface/job/gesture/delegation types and `AppAdapter`. It must be reconciled and reused, not scheduled as a nonexistent DTO library. |

These are source findings, not complete suite parity or a new execution result.
Read coverage and source pins accompany each report. README/module claims and
function bodies have different evidence levels. Source tests were not run here.

### Stale findings corrected

- Native Poet is no longer CLI-only. It has built native graphical authoring,
  existing source/session/geometry/material/resource controls, viewport and
  cooperative projections with retained scoped execution evidence.
- The current native `poet_workspace/intents.rs` implements the shared IntentBus.
  Its model reuses original Construct/Studio seeds. Earlier statements that no
  native implementation existed are superseded for this selected route.
- Those facts do not establish the full original workspace or every document
  editor. Held/partial Scene, Audio and other seeds, command descriptions or
  callbacks must not be counted as working editors.
- Native UV/texture/detail/material-map work exists. Older factor-only/rejected
  texture observations are historical; arbitrary complete-document, topology,
  rigged-section and material preservation remain separately qualified.
- The earlier CAD-heavy production focus understated the available source tools.
  Word, PDF, grid, deck and media authoring are substantive parts of Poet's scope.

## Full capability map: reusable source and intended Poet surface

Each row is a required capability family to reconcile item by item. This table
does not claim every advertised feature or codec was inspected or works in Poet.
Reports identify actual read scope and remaining unknowns.

| Internal Poet capability | Existing source functions to reuse/review | Integrated human surface and agent operation | Qualification boundary |
|---|---|---|---|
| Raster/image/material sources | Photo-origin layer/document, masks, edits/history, adjustment/composition, image IO/export and automation. | Layered image canvas, selection/brush/mask/adjustment/history, native command discovery and the same session edits for agents. | Not a mesh editor. Preserve colour/data-map roles, source layers, UV/material bindings and exact native reopen/export. |
| RAW/capture development | Light-origin non-destructive settings, RAW decode/demosaic, colour/lens/local-mask pipeline, catalogue/metadata, previews and history. | Reference/capture browser, development controls, metadata/source provenance, bounded preview/export jobs. | This is photographic development, not 3D lighting. Declared capture/calibration and original data survive derivatives. |
| Vector/illustration | Vector-origin paths, objects/artboards/layers, text/symbols, appearance/effects, command registry and structural history. | Vector canvas, guides/snaps/tools, selection/inspector, plate/diagram/label editing and agent commands. | Projected surface-of-revolution output is not general solid anatomy. Enhanced Metafile interchange must not be confused with the ecosystem's physical EMF framework. |
| Audio and spectral editing | Sound-origin tracks/clips/mixer/MIDI, edits/history, block processing, automation/routing; existing Qualia audio/DSP/media services. | Audio-session container, waveform/spectral/track view, timeline/mixer/meters, emitter/event links and headless editing. | Headless transport requests are not audible playback. Sample/time/units, actual device output and spectral interpretation need separate proof. |
| Film/sequence/compositing | Film-origin rational/integer time, timeline edits, transitions/captions, compositing/grade/audio and export/cancel jobs. | Sequence/stage/timeline, trim and review controls, scopes/captions, render/export queue; agent edits same sequence. | NLE composition is not the 3D scene renderer. Preserve frame/sample alignment, colour, passes and resource lineage. |
| Text/semantic documents | Word-origin document blocks/styles/stories/tables/comments/revisions, layout/references and protected command/history session. | Document/CML/Markdown or other declared views, text selection, formatting/citations/review and native page output. | Preserve rich editable semantics and revision history; do not flatten to HTML, PDF or a generic form. Semantic annotations bind to Q42 without replacing the document engine. |
| PDF/reference/review | PDF-origin page/model/edit, annotations/forms and batch/action/export/preflight candidates. | PDF content container, pages/bookmarks, evidence annotations/forms, review/plate/export controls and permitted agent actions. | Sampled batch actions do not prove all annotation/MCP/signature workflows. Signed/derived documents, OCR and destructive edits require exact supported profiles and loss records. |
| Spreadsheet/data/formula | Grid-origin workbook/cell model, formula/function registry, edit/recalculate/protection and data/chart/pivot candidates. | Grid container with range selection, formula bar, cell edits and data/trait/BOM/chart inspection for people and agents. | Preserve formulas, types, units, protected ranges and recalc semantics. A spreadsheet view is not the authoritative Q42 claim or rights store. |
| Presentations/storyboards | Deck-origin slide/object/text/table/chart/layout/master/theme/media/transition source and headless session routes. | Slide/scene navigator, presentation canvas, object tools, notes, layout/theme and transitions plus agent commands. | Headless pointer/command support does not prove native Poet controls or presenter accessibility. Retain editable linked media and scene lineage. |
| CAD/constraints/assemblies | CAD-origin drafting selection/snaps/constraints/dimensions/parameters plus existing Qualia geometry and current Poet spatial libraries. | Sketch/drawing/spatial contexts, part hierarchy, dimensions/constraints/material/BOM/ports and command/gesture parity. | Do not infer full 3D solids from 2D `Solid` or constraint names. Keep organic/rig/assembly contracts and exact source dimensions. |
| Optional generative assistance | Art-origin references/prompts/models/accounts, generation request/queue/result/library/lightbox UI and existing provider clients. | Optional authorized assistance inside the current Poet document/reference context, with budget, progress/cancel and provenance. | Generation pages or enqueue success do not establish editable geometry, completed output, source rights or quality. No paid provider call or mandatory provider dependency is introduced. |
| Graph/science/logic/package authoring | Existing Qualia Q42/query/ontology/modality/solver/simulation/vision/audio/instrument services and HMC/.10d/M64 owners. | Graph/trait/rule/shape inspectors, typed physical/spectral data, behaviour/state, source REPL, package/resources/claims/review in the same project. | Reuse actual owner semantics, evidence and units. A decoder, demo runner, receipt, colour projection or rule result cannot establish executed physiology, authority or complete knowledge. |

## Reuse boundaries against the audited QualiaDB owners

The nine-location study is the reuse baseline, with its stated sampling limits.
These allocations compare responsibilities; they do not assert every desired
operator was reviewed, exposed or executed. Follow the cited source/caller pins
before changing a specific operation.

| Existing owner | Required relationship to integrated Poet tools |
|---|---|
| `qualia-core-db/src/domains` | Reuse actual typed domain data/calculations and their units/evidence. Image, document or grid content can reference them without becoming scientific authority. |
| `qualia-core-db/src/modalities` | Preserve actual physical/spectral/field representations and source/receiver interpretation across inspectors, visual/audio projections and package resources. Do not reduce nonvisible signals to RGB or conflate acoustic pressure with electromagnetic propagation. |
| `qualia-core-db/src/solvers` | Map required constraints, geometry, physics and logic operations to their actual owner/export/caller. Reuse matching implementations; inspect semantics before replacing an existing CAD/domain solver. |
| `qualia-core-db/src/specialized_libs` | Locate the existing numerical/geometry/scientific primitive required by a tool; qualify inputs/outputs and precision. A broad library catalogue is neither a missing-function list nor permission to rewrite the document engine. |
| `qualia-core-db/src/wgsl_forge` | Reuse actual GPU operator generation/registration/execution where present; prove the selected host binding and fallback. GPU source is not automatic UI or device execution evidence. |
| `qualia-vision` | Reuse actual image/analysis/calibration services where their semantics match. Preserve layered-image and RAW development source/history; analysis results do not replace authoring or establish biological truth. |
| `qualia-audio` | Reuse actual feature/DSP/streaming and semantic owners with explicit sample/time/units. Track/mixer/session authoring and audible device transport still need their own demonstrated route. |
| `webizen-render` / developed Etalon | Reuse existing scene/material/viewport/frame consumers and hardware capability evidence. Keep image compositing, NLE composition and presentation rendering distinct from 3D rendering. |
| `qualia-core-db/src/semantic_instruments` | Reuse native provenance/contribution/rights/review links and qualified authority. An editor history, generated agreement or receipt does not establish adopted terms, human review or payment. |
| Existing Q42/HMC/.10d and `qualia-sim` owners | Bind source document/part/organ/rig/behaviour references and their derivatives to complete embodiments and actual resource/streaming APIs. Reuse the focused plant/rig studies; do not create parallel stores, codecs or simulation kernels. |

## Common architecture: one product, authoritative domain documents

Poet coordinates project, document/subject/part selection, tool discovery,
commands, provenance, review and views. Each domain document keeps **one
authoritative engine/session**, with typed references and revisions across tools.
Do not replace the media/text/grid engines with a spatial-script state store or
create a second hidden state copy inside an embedded foreign app.

Reconcile existing `qualia-app-contract::AppAdapter`/descriptors with Poet's
actual IntentBus, ContextRef, Tool metadata/registry and native owner. They are
existing related interfaces, not automatically identical wire contracts. Preserve
exact legacy IDs as compatibility references; new public naming belongs to Poet.
Native and WASM adapters share semantics with explicit provider/feature differences.

Important current contract limits: trait defaults can report `Available`, return
revision zero/Null/empty snapshots or success for restore/cancel/close without
domain behaviour. A descriptor or implemented trait is not proof of an operation.
Every admitted adapter must declare supported operations and demonstrate actual
state changes, recovery and error behaviour. A receipt DTO does not sign itself
or authenticate the supplied actor; authority remains with native services.

The refactoring unit includes document/model state, operations, history,
serializer/codecs, render/preview, export, jobs, selection and UI semantics.
Reuse shared numerical/media primitives from QualiaDB when contracts actually
match; similarity of names does not justify replacing a domain engine. Keep
required notices and data compatibility. Broad manifest relocation is not the
first deliverable and Chora stays outside this consolidation boundary.

## Human and agent authoring must converge on the same operation

The required path is:

```text
human gesture/menu/palette OR agent Vibe/MCP/command intent
-> actual current project/document/selection and expected revision
-> native authentication/authorization, capability and work/budget admission
-> one existing domain session operation
-> actual revision, undo/history/job outcome and resource changes
-> native editor/preview + machine-readable inspection of that same result
-> durable source/package and contributor/reviewer record
```

Same operation does not mean equal authority. An agent cannot acquire human
review/adoption rights by setting actor fields, calling MCP, or replaying a UI
gesture. A UI form and a headless endpoint must use the current authorized owner.
Unsupported operations remain unavailable with a useful reason; no placeholder
receipt or external app launch counts as native Poet authoring.

Every selected document workflow must cover command discovery/availability,
typed inputs/results, direct manipulation where relevant, selection/focus,
preview/commit/cancel, undo/redo, import/open/save/export, revision conflicts,
jobs/cancellation/stale results, error recovery, provenance and review.
Cross-document actions need explicit dependency/transaction semantics; do not
promise atomic global undo where the current engine contracts cannot provide it.

Original UI behaviour must be catalogued across commands, panels, gestures,
shortcuts, context menus, dialogs, views and accessibility. Keyboard/IME/focus,
screen-reader labels, text scale, contrast, reduced motion and accessible
alternatives are requirements, not optional substitutes for agent APIs.

## Required work, preserving existing allocations

These PA IDs expand PC/TOOL/IP-03 cases; they are not another phase order.

| ID | Work and output required | Existing allocation |
|---|---|---|
| PA-01 | Reconcile actual shared contracts, descriptor/availability/target/gesture/session/receipt/job/delegation semantics and source authority. Freeze compatible adapter behaviour, including refusal of unimplemented defaults. | PC-04; TOOL-01/02; ENG-08/09/13. |
| PA-02 | Recover full Construct/Manifold/container navigation and document-kind routing, Tool Chest/menu/palette, context/selection, inspectors/timelines and saved workspace layouts. Retain existing native spatial work. | PC-01/05; TOOL-02/03; IP-03 C1/C3/C8. |
| PA-03 | Raster/RAW reference-to-editable-layer/material workflow through existing image engines, domain history and native controls, with typed exports/colour/provenance. | PC-06; TOOL-06. |
| PA-04 | Vector/diagram/plate editing, source-linked labels and exact dimensional/export semantics through Poet controls and agent commands. | PC-06; TOOL-03/06. |
| PA-05 | Audio session/track/spectral/mixer/emitter editing, jobs and actual playback/output, with native user and agent parity. | PC-06; TOOL-09. |
| PA-06 | Film/timeline/compositing/grade/caption/audio and render/export queue, preserving exact frame/sample/colour/resource lineage. | PC-06/07; TOOL-09/10. |
| PA-07 | Rich text/semantic document and PDF/reference annotation/review workflows, native editing/accessibility, durable source and explicit lossy derivatives. | PC-06; TOOL-02/11/12. |
| PA-08 | Spreadsheet formulas/data/units/protection/charts with native cell/range interaction and source-to-Q42 relation boundaries. | PC-06; TOOL-02/11. |
| PA-09 | Editable presentation/storyboard/slide/scene links, native object/notes/theme/transition/presenter controls and agent operations. | PC-06; TOOL-02/09/10. |
| PA-10 | Optional generation/reference assistance with explicit provider/capability/budget/source/result/provenance states and no false completion. | PC-06; TOOL-01/02/12. |
| PA-11 | Native CAD/organic/material/rig/behaviour and scene authoring from existing owners; preserve current BS-10/20/30/40 source and package contracts. | PC-07; TOOL-03–08/11; existing biosphere jobs. |
| PA-12 | Cross-domain human/agent, authority, undo/reopen, contribution/reviewer and accessible failure/recovery qualification. Full UI/function dispositions require evidence, not a row count. | PC-05/06/08; TOOL-12; IP-03 C1–C8. |

The [central job register](../biosphere-work-packets.json) adds independent
PA-R research handoffs for these areas. Current scope is documentation/research;
none activates implementation, moves source or runs Cargo. Workers own disjoint
findings/handoff files; one coordinator owns shared product/schema/manifest changes
and serial meaningful checks when implementation resumes.

## Delivery order without shrinking the product

First resolve common authoring contracts and the required native workspace
connections. Then implement and qualify coherent document workflows using existing
engines. Image/reference/material, vector annotation, biological/spatial/rig,
graph/package and cooperation are near-term Ecopunk dependencies; other media,
documents, grids and presentation work can be assigned in parallel once shared
interfaces are clear. They remain explicit product work, not silently dropped.

Do not require complete parity of every suite feature before a tree/wallaby
workflow can pass its declared gate. Equally, do not call **comprehensive Poet**
complete when only spatial source editing works. Full-product acceptance requires
all agreed domain workflows and inventoried UI cases, actual users and agents,
native/WASM capability declarations and source/knowledge/resource continuity.

For each selected workflow retain exact source/dependency/input/output IDs,
actual human and agent invocations, expected versus observed state/history,
saved editable source, reopened result, native output and failures. Rights,
human acceptance, complete species packages and film/game/cartoon claims remain
separate. No new whole-suite or model acceptance is established by this review.
