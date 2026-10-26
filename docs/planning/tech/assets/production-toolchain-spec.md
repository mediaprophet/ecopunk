# Native production toolchain — comprehensive Poet authoring

**Status:** specification draft v0.2 · **Reconciled:** 2026-10-26.
Scope: what is needed to produce complete species and made-world HMCs through
the existing local authoring suite. This is read-only source/document inspection;
no sibling repository was edited, built, launched or runtime-qualified here.

The [current comprehensive authoring review](../poet-comprehensive-authoring-review.md)
adds bounded function/caller studies across all eleven source families, current
native/shared human/agent Poet routes and the existing `qualia-app-contract`.
Initial source observations below are historical where superseded by that review
and later saved native evidence. Source presence, exposed headless commands,
native integrated controls and accepted production remain separate statuses.

Companions: [architecture](../../architecture.md), [package contract](asset-package-spec.md),
[completion claims](model-completion-claims.md), [per-model template](model-production-template/README.md).

**Product boundary:** MIT-licensed source capabilities and useful editing/UI
behaviour are refactored into internal **Poet** functionality. The earlier
`C:/github/craft` layout is migration context to reconcile with current owners,
not a separate product or blanket relocation prerequisite.
[poet-craft-consolidation.md](../poet-craft-consolidation.md) governs this
repository/UI migration. Independent studio windows are migration/reference
inputs, not the intended product interface.

The [controlled framework and environment specification](../webizen-environment-framework.md)
supplies the common foundation: Aether (the Dioxus fork), Webizen Desktop native launcher/
host, and shared Poet native/WASM projections. Native-assisted WASM reaches the
same qualified domain services where required; it does not duplicate editing
policy or replace native production-path qualification. Tool handoffs must
declare their execution location and provider/version/availability requirements.

The separate [Chora engine](../chora-engine-boundary.md) is excluded from Craft
extraction. Its implementation remains in `C:/github/chora`; host-side adapters
use generic interfaces with its source/license provenance preserved separately.

The [master production backlog](../ecopunk-production-backlog.md) connects
TOOL-01–12 to the stack/content/release sequence. The
[Etalon handoff](../etalon-capability-handoff.md) identifies implemented GPU
extensions and existing consumer source; inspect their required qualified scope
before commissioning another kernel or attributing poor art to missing hardware APIs.

## 1. Answer and scope

The [consolidated implementation plan](../../implementation-plan.md) makes
native cooperative authoring an operational prerequisite: IP-03 C1–C8 must
demonstrate shared definition, authorized contributions/review, real editing,
revision/conflict handling, reproducible output, recovery and usability. Source
APIs/CLI automation and completed engine tasks do not alone establish this
capability. Finish the required missing operator/binding/host workflow and
produce the defined high-grade result; broader suite work remains allocated.

Use the suite's relevant native engines and authoring capabilities rather than
reimplementing its work in Ecopunk. A model does not need every application to
run: a reproducible Vibe/native command chain can invoke an engine headlessly;
an artist edits the same sources through Poet's integrated domain surfaces.
Both paths retain the same document/entity identifiers, source lineage and output
contract. Existing studio shells remain references while functions are ported.

The suite contains substantial specialised tools, but inspection did not
establish an integrated film-quality organic modeling, rigging and HMC release
workflow. That workflow must be enabled and qualified. A GUI, command count,
generated image or “3D” label does not establish the required editable model.
No JavaScript/Node/V8 workflow is permitted as a dependency of this production
path. JSON messages over native commands/MCP are data, not JavaScript execution.

## 2. What is actually present

Paths below are relative to `C:/github/`. Claims are limited to the inspected
source/documents; README feature claims have not been promoted to runtime proof.
The older `*_studio` names in Ecopunk planning need mapping to these actual repos.

| Repository | Inspected evidence | Appropriate production role / limitation |
|---|---|---|
| `poet` | Current graphical/session/intent/MCP/workspace sources and retained native execution reports in the comprehensive review | Native source/spatial/material/resource editing and human/agent service paths exist. Full original workspace, all document kinds and actual operator acceptance remain incomplete. Canonical development is this workspace; retained copies remain recovery inputs. |
| `poet-artcraft` | `artcraft-native/README.md`; `src/pages/mod.rs`; `src/app.rs` dispatch | Native image/video/audio generation client; `coming_soon` handles other pages. The general README advertises image-to-3D and scene staging, but this is not proof of native production mesh editing, retopology or rig authoring. Optional provider generation requires separately authorised calls and explicit provenance. |
| `poet-cadcraft` | `docs/architecture.md`; `docs/gaps.md`; `crates/doc/src/entity.rs::EntityKind`; `crates/mcp/src/lib.rs` | Drafting, dimensions, constraints and manufactured-form references. The inspected model has polylines and 3D faces; `Solid` is labelled **2D Solid**. Current architecture explicitly lists no 3D solid/surface/mesh model. Qualify/add the design/assembly/solid editing seam rather than assuming AutoCAD-parity branding supplies it. |
| `poet-vectorcraft` | `crates/three-d/{Cargo.toml,src/lib.rs}`; `docs/architecture.md` | Vector plates, diagrams, profile curves and stylized graphic resources. The inspected 3D crate projects surfaces of revolution into vector polygons; its own comment says these are not solid modeling inputs. This does not establish a general editable character model or HMC mesh exporter. |
| `poet-photocraft` | `docs/architecture.md`; `crates/automation/src/lib.rs` | Raster/material source authoring, masks, texture/image preparation. Native engine/automation/headless modules exist. Need the semantic material-map/colour-space/export adapter and proven source round-trip; not by itself a mesh/rig authoring application. |
| `poet-lightcraft` | `docs/architecture.md`; `crates/mcp/src/lib.rs` | RAW development, photographic catalog, colour/metadata and reference preparation. It is a Lightroom-class photo tool, **not a 3D lighting studio**. Native capture-development outputs need preserved provenance and interpretation. |
| `poet-soundcraft` | `docs/architecture.md`; `crates/automation/src/lib.rs` | Calls, operating sounds, foley, loops, event-aligned audio and mixes. Native/headless automation exists in source; the inspected headless backend drops transport requests and has no audio engine. A headless command result therefore cannot establish audible playback. |
| `poet-filmcraft` | `docs/architecture.md`; `crates/automation/src/lib.rs` | Edit, composite, grade and assemble rendered sequences and sound, with exact time. Headless/bridge engine paths exist in source. It is an NLE, not proof that the upstream 3D renderer supplies film geometry, shading, grooming or render passes. |
| `poet-wordcraft`, `poet-pdfcraft` | Current document/data review follows rich document/session/history/MCP, PDF command metadata and batch action bodies | Substantive text/semantic document and PDF source/edit/review capability families are reusable product work to reconcile and integrate, beyond dossier export. Integrate real native document/page/annotation controls and the same authorized domain operations for agents; preserve source and supported formats. |
| `poet-gridcraft`, `poet-deckcraft` | Current review follows formula registry/cell-edit and structured slide/headless session bodies | Spreadsheet/data/formula/chart and presentation/storyboard editing belong in Poet. Native cell/range/slide/object interaction and agent parity remain to qualify; these editors do not replace Q42 facts/rights or complete-model claims. |
| `qualiaDB`, `etalon` and owning custom libraries | Existing Ecopunk capability/gap register; focused prior renderer inspection | Shared graph/codecs/geometry/simulation/rendering and hardware execution. Reuse and qualify actual public capabilities; existing source presence is not a complete artist-facing workflow. This turn did not reaudit every kernel. |

The additional [EffectCraft review](../../../qa/implementation-2026-10-26/effectcraft-review/README.md)
covers compositor/motion-graphics source outside the eleven historical roots.
Use its actual layer/keyframe/effect/session bodies and the
[ecosystem adaptation examples](../poet-capability-adaptation-guide.md) when
allocating PA-06/TOOL-09/10 work. Selection enablement, standalone save/render and
MCP control do not establish native Poet authority, HMC continuity or acceptance.

The existing cross-repository programme is in
`qualiaDB/docs/work-in-progress/POET_UPDATED_IMPLEMENTATION_PLAN_2026-09-05.md`
(ST programme and ST-N) and its execution playbook. It already proposes app
adapters, shared commands/context, supervision and Vibe migration. The proposals
are inputs to this enablement work, not evidence that adapters are installed.
In the initial 24 October manifest-only search, no direct Qualia/Vibe/HMC dependency matches were found in the six inspected
Craft workspace Cargo manifests; that limited search does not rule out external
adapters/protocols or prove a lack of interoperability elsewhere. The current
shared application contract and existing native IntentBus must now be reconciled
before proposing new bridge types or claiming all integration is absent.

## 3. Required authoring flow

```text
Reference/data acquisition -> Q42 species/design corpus + identifiers/part map
  -> native editable geometry/CAD/procedural source
  -> topology/UV/material/rig/behaviour/media authoring
  -> derived film + open-world + cartoon representation sets
  -> native per-root compile/validate + graph/resource reconciliation
  -> complete HMC + claims/evidence inventory
  -> native inspect/query/operate/render + review
  -> admitted release; instance state in the qualified journal
```

Every handoff carries entity/part IDs, source revision and digest, units/frame,
selection/context, style/target, compatibility, colour/time conventions,
rights/provenance, losses, dependencies and the affected deliverable IDs.
Never flatten a design/rig to a screenshot or discard parts just to cross tools.
Any imported format is a declared conversion step, not the master authority.

## 4. Enablement work packets

All packets are **open requirements**. Owners are proposed library/repository
boundaries. Complete a narrow production chain; unrelated suite feature parity
is not a prerequisite. Upstream implementation authority remains with Timothy.

| Packet | What must be enabled | Proposed owner | Evidence required to close |
|---|---|---|---|
| TOOL-01 | Versioned capability inventory per studio/host, distinguishing documented, source-present, callable, runtime-qualified and unsupported. | Poet/app adapters | Discover command/schema/profile/version, run an allowed operation and inspect actual output; unavailable operations never masquerade as completed work. |
| TOOL-02 | Shared entity/document/part/selection/asset references and typed command/job/receipt contract, exposed through Vibe. | Existing ST contract owners + app adapters | Same identified construct crosses two engines, can be edited, undone/reopened and traced to output without ID loss or private JS state. |
| TOOL-03 | An actual native 3D authoring workspace: object/part selection, transform/edit, procedural parameters, geometry inspection, source save, undo, viewport and incremental preview. | Poet/native 3D host + geometry/render library | An artist or agent edits a complete organism and an assembly and reopens their editable sources; no blockout-only or screenshot-only completion claim. Decide the owning workspace from existing code before adding a new app. |
| TOOL-04 | Organic geometry surface/volume editing, curve/branch generation, remeshing/retopology, UVs, displacement/groom/foliage support as required by chosen subjects. | Shared Qualia geometry/surface libraries; native authoring adapter | Neutral-light closeups and deformation cases on real deliverable subjects, with source/operator history, topology/UV checks and reproducible derivation. Missing grooming/subdivision/sculpt tools remain explicit work. |
| TOOL-05 | Parametric solid/surface design and assembly authoring: dimensions, constraints, part IDs, joints, material/BOM and port data. | CADCraft adapter + owning CAD/geometry libraries | Native vehicle/building components remain editable; controlled tessellation/conversion retains dimensions/parts and reports losses. Do not interpret 2D SOLID or a projected vector face as a solid-modeling implementation. |
| TOOL-06 | Material/look development and capture pipeline: typed colour/data maps, procedural sources, shader/material graphs where needed, calibrated preview and source interchange. | PhotoCraft/LightCraft/VectorCraft adapters + Qualia materials/rendering | Source-to-material round-trip; role/colour-space/mip fidelity; distinct surface appearance under neutral, backlit and production light; retain physical/render distinction. |
| TOOL-07 | Rig, skin, morph, attachment, clip, contact and transition authoring; native playback/scrub/retarget and variant compatibility. | Skeletal/animation libraries + native authoring tools | Editable rig and all enumerated clips/transitions for film/game/cartoon; extreme-pose deformation, contacts, state mapping and reduced-motion evidence. A pose generator is not an animation library. |
| TOOL-08 | Behaviour/lifecycle/physics graph authoring and inspection: stimuli, states, actions, resource/energy accounting, stochastic seeds, environment coupling and journal. | Vibe/Q42/qualia-sim/owning domain libraries | Native governed stimulus -> admitted state change -> animation/audio response -> replay, including refusal/cancellation. A clip's motion is not the behaviour or its biological/operational effect. |
| TOOL-09 | Sound and film handoffs: exact event/sample/frame time, editable source, stems, emitter bindings, rendering/capture passes and colour metadata. | SoundCraft/FilmCraft + Qualia audio/rendering | Reopen an object-linked session/sequence, render/export, and verify event sync and native playback. Film-quality 3D images must originate in a qualified 3D renderer; NLE export alone does not qualify them. |
| TOOL-10 | Three representation delivery profiles with independent budgets/quality criteria, derivations and compatibility: film, open-world game and cartoon. | Native compiler/rendering + Poet chain | Each has editable source, selected roots, materials/rigs/motion coverage, tested output and human visual review; cartoon specifies its film/realtime target and is not a cheap LOD. |
| TOOL-11 | Native compile/pack/inspect: complete Q42 corpus/interpretation, components and variant inventory, resource closure, HMC integrity, offline inspection and incremental rebuild. | Existing codec/asset/compiler/Poet owners | One flora, one fauna and one manufactured embodiment go from source to native-use HMC; required graph/resources reconcile; all representations resolve with stable IDs and journal separation. |
| TOOL-12 | Per-deliverable claim capture, evidence indexing and completion gate integrated with agent task finalization, package validation and release. | Native validation/supervision + Poet review UI | Missing/duplicate/stale claims, unsupported N/A, evidence mismatch or open required rows prevent a complete status. Agents cannot self-award human approval. See the completion contract. |

## 5. Qualification and allocation

Start with identifiers/handoff contracts and a native authoring slice. Then trace
one complete plant, animal and vehicle through the tools actually required,
using the per-model deliverable ledger. At each missing seam, create a bounded
upstream packet identifying source evidence, required interface, owner, formats,
inputs/outputs, cancellation/budgets and acceptance cases. Do not report every
studio “required” just because it is installed.

Some work can use a qualified native headless engine before its Poet controls
are complete; capabilities needing visual authoring still need an inspectable
native Poet surface. Rendering quality, command correctness, graph completeness and source
recoverability are separate gates. No build/install/provider/network action is
performed or implied by this specification.

Open decisions: precise owner/location for the complete organic modeling and
animation workspace; qualified CAD solid path; cross-app metadata/undo/selection
schema; native HMC import/export APIs; renderer capabilities and image/sound
fidelity per target; validator enforcement and human review authority. Existing
names/features remain claims until the particular production chain is qualified.
