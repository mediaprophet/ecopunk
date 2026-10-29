# Poet continuity failure: incident record and proposed remediation

**Prepared:** 28 October 2026, Australia/Sydney.  
**Purpose:** account for the failure to preserve the intended Poet product and repository strategy, identify what is actually known, and define concrete recovery work.  
**Companion:** [asset-production recovery outline](asset-production-recovery-outline.md).

This document was requested by Timothy after he identified that the work had not adequately followed his existing Poet/Webizen interface requirement. It records an implementation and reporting failure. It does not claim that the original application is fully operational, that the new authoring work is qualified, or that any species package has been produced.

## 1. Executive finding

Timothy had already specified the [Poet Tool-Chest and Manifolds interface](https://mediaprophet.github.io/qualiaDB/poet-app/index.html) and asked for reusable Craft capabilities to be integrated into Poet. Existing desktop-host and Rust/Dioxus Poet implementations were available, and the project's own consolidation document named them. The coordinator nevertheless concentrated implementation and reporting on a newly developed native authoring surface in the canonical Poet repository, including an explicit diagnostic entry, without establishing that it preserved and extended the intended Poet experience.

That is the central failure. Useful geometry, source, authority or persistence code does not establish compliance with the product requirement if its integration with the required workspace remains unqualified. Repeated plans, saved files and test counts did not give Timothy the working authoring environment he needed to produce high-quality Ecopunk assets.

The coordinator also failed to distinguish consistently between user authorization, filesystem sandbox mechanics, repository/source location, source implementation and an actually usable application. The later discovery of a concrete unreachable junction-adoption route demonstrates why earlier source-level progress could not support claims of a usable workflow.

The remedy is to recover the existing Poet product baseline, integrate useful new capabilities through its authoritative operations and host seams, finish the asset-specific blockers, and demonstrate the complete source-to-output workflow. Replacing QualiaDB, discarding valid work, rebuilding the monorepo by accident, or starting another application would compound the failure.

## 2. What Timothy had instructed

### 2.1 The interface requirement was explicit

In the earlier chat **“Audit ecopunk demo graphics”**, on **24 October 2026 at 17:09 Sydney time**, Timothy wrote:

> the 'craft' suite was intended to be stripped to pieces and then integrated with [the Poet interface](https://mediaprophet.github.io/qualiaDB/poet-app/index.html)

He also wrote:

> the interface was intended to be, simply poet.

He asked whether the Craft crates could be put in one `craft` directory and their UI elements added to Poet. He reported that existing Poet, previously part of Webizen Desktop, had effectively been removed. This was a product-continuity requirement, not permission to substitute a small geometry editor for the original workspace.

The exact user message was recovered from the [local conversation record, line 612](C:/Users/tim_h/.codex/sessions/2026/10/24/rollout-2026-10-24T15-39-28-01a16c34-e639-7993-858e-f47e4b4f5f29.jsonl:612). That record establishes the instruction and the user's report; it does not, on its own, establish which earlier change removed or regressed functionality.

### 2.2 Repository separation was meant to modularise existing software

In the current request Timothy explains that QualiaDB had been a monorepo, that he was creating separate repositories to modularise it and improve development/build overhead, and that he understood access to `C:/github/poet` in that context.

This is consistent with the historical [Poet execution playbook](C:/github/qualiaDB/docs/POET_LOWER_COST_AGENT_EXECUTION_PLAYBOOK_2026-09-04.md:600), which identifies separate repository roots, treats retained monorepo Poet copies as noncanonical, and instructs reuse rather than repeated implementation. The [session ledger](C:/github/qualiaDB/docs/POET_IMPLEMENTATION_SESSION_LEDGER.md:690) also records extraction-related path failures and bounded repairs.

Moving source to a separate repository does not mean changing the product, replacing its runtime, or abandoning its interface. Nor does a directory split automatically make Cargo faster: the resolved dependency closure, default features, crate boundaries, build profile, target isolation and cache reuse determine what actually recompiles. The build-speed objective must be preserved and measured, not assumed from a new path.

### 2.3 Other continuing requirements

- QualiaDB remains the core runtime and semantic/data owner. Existing native `.q42`, `.10d`, HMC, identifiers, rights and provenance mechanisms are reused.
- Poet is the unified authoring interface. Craft-origin capabilities are internal tools, with required notices and useful behavior preserved.
- Native and WASM requirements remain distinct. Framework migration must preserve the existing product semantics; it is not a reason to discard the interface.
- User authorization to implement relevant Poet/QualiaDB/Etalon changes persists. A sandbox write mechanism is not a new product decision or a revocation of that authorization.
- Managed workers own disjoint changes; one coordinator integrates shared source and owns serialized Cargo execution.
- The latest instruction defers Poet testing and rebuilding until the requested coding is complete. It does not permit claims that untested source is functional.
- Asset production remains on HOLD under the earlier instruction. This document does not invent a release of that HOLD or authorize publication, paid providers or spending.

## 3. Existing Poet implementations that should have anchored the work

| Existing location | What was actually inspected | Required treatment |
|---|---|---|
| [Webizen Desktop window launcher](C:/github/webizen-desktop/src/shell/menu.rs:232) | `open_poet_window` focuses an existing Poet window or creates one titled **Poet HyperCanvas — Spatial Hypermedia Workspace**, using `/volumes/poet` or its bundled page. | Trace and preserve this host entry, its service/lifecycle assumptions and the user-visible workspace. Source presence does not prove that the current binary launches successfully. |
| [Webizen desktop Poet volume](C:/github/webizen-desktop/static/os-shell/volumes/poet.html) | Existing HyperCanvas volume includes tool-chest and content-container behavior. | Recover actual interaction semantics; do not substitute a descriptive dashboard or launcher for an editor. |
| [Bundled Tool-Chest/Manifolds application](C:/github/webizen-desktop/static/portal/poet-app/index.html:6) | Bundled application has the published Poet title and WASM bootstrap. | Establish its revision and behavior relationship to the published reference and canonical source. Do not assume byte identity. |
| [Rust/Dioxus Poet components](C:/github/qualiaDB/crates/webizen-studio/src/components/poet/mod.rs:35) | `PoetHarness` mounts `PoetWorkbench`; the source includes chest, manifolds, containers, inspector, stage, Vibe console and host/engine modules. | Reconcile this existing implementation and its callers before adding or migrating frontend behavior. |
| [Desktop Poet engine commands](C:/github/webizen-desktop/src/commands/poet.rs) and [render command](C:/github/webizen-desktop/src/commands/poet_render.rs:29) | Existing `PoetSnapshot`, Vibe evaluation, document/cell, volume and rendering service bindings. | Reuse or adapt the actual services; qualify authority, state ownership and effects instead of assuming a parallel new host is necessary. |
| [Separate Poet repository](C:/github/poet/Cargo.toml:1) | Workspace contains native, WASM and internal geometry/spatial crates, with sibling QualiaDB dependencies. `wasm/src/browser` retains the richer interface source. | Treat the separate source home as modularisation of Poet, not evidence that Poet was absent or that retained copies may be overwritten indiscriminately. |
| [Consolidation record, updated 26 October](../../../planning/tech/poet-craft-consolidation.md) | It names these source families and requires preservation/reconciliation. The source table reports read-only inspection on 24 October; that is distinct from the current document's update date. | Apply the recorded baseline in every dispatch, not merely cite the document. |

The desktop implementation currently inspected uses Tauri/WebView infrastructure. Aether's native rendering requirement is a separate engineering qualification. That distinction does not erase the existing desktop product, invalidate its useful behavior, or justify replacing its workspace without a preservation/migration contract. Conversely, the existence of a desktop wrapper is not proof of the required no-JS/V8 native path. Both facts must remain visible.

## 4. Incident chronology and evidence limits

| Period/event | Established evidence | What it does not establish |
|---|---|---|
| Earlier extraction/modularisation | Separate roots and retained-copy distinctions appear in the historical playbook; extraction path failures and repairs appear in the ledger. | Complete extraction, faster Cargo or native feature parity. |
| 24 October instruction | User explicitly supplied the published Poet URL and requested Craft UI integration into Poet. | The exact removal/regression commit or a proven working historical executable. |
| Existing project specifications | Consolidation, comprehensive authoring and framework documents name original Poet surfaces and Webizen service seams. | Compliance by later dispatches merely because those files were read. |
| Subsequent implementation | New native geometry, source, cooperative and persistence slices were saved; some earlier snapshots have scoped test/GPU/build evidence. | Preservation of the complete intended Poet experience, usable biological production or coverage of later source changes. |
| Build/resource incidents | The continuation record documents shared-target feature collisions, bounded repair, interrupted builds, cache cleanup and executable receipts. | That all disk consumption came from this coordinator, that files were deliberately hidden, or that a particular motive caused the failures. |
| Latest junction source installation | [Installation receipt and backups](../../../../tools/poet-foundation/full-scope/junction-installed/source-manifest.tsv), followed by [source-review fixes](../../../../tools/poet-foundation/full-scope/junction-installed/source-review-fixes.tsv) and [final source pins](../../../../tools/poet-foundation/full-scope/junction-installed/final-source-hashes.tsv). | Successful compilation, a running integrated application or validated asset production. The newest integration was deliberately not tested or rebuilt. |
| User requests to see Poet, 28 October | Published interface was inspected. Existing `poet.exe` hash was `4b6e9a07e769df8499d278f5f4d0b7b915df5059199fea1d321422f61ae3c81a`; a launch attempt created a process initially. Later window inventory showed no Poet window and that PID was no longer present. | A successful visible launch. The available launch log did not establish the exit cause. |
| Reconciliation after challenge | Source inspection confirmed the existing desktop launcher and `PoetHarness`/`PoetWorkbench`. Further pending UI expansion was stopped and candidates preserved. | Completed remediation. |

The [continuation record](../full-scope-round1/continuation-authoring-presentation.md) preserves more detailed successes, failures, source snapshots and interruptions. Earlier evidence must not be relabelled as verification of current dirty source.

## 5. Specific failures

### F-01: Existing-product reconnaissance did not govern implementation

The original interface and its native/desktop source homes were known and documented. The coordinator did not establish and document an end-to-end launch, workbench, service and persistence trace sufficient to govern the new integration. The implementation activity therefore advanced individual capabilities without establishing product continuity. This does not mean the canonical native source is exclusively diagnostic: it contains both native-project and explicit diagnostic entry routes.

### F-02: Source-location and access statements were conflated

Four separate questions needed separate answers: which repository owns the source; whether the source exists and can be read; how the sandbox permits writing it; and whether the application works. A missing search result, unavailable Git metadata or a blocked ordinary write cannot establish that Poet is absent, globally read-only or unauthorized.

`C:/github/poet` was outside the session's default writable roots, while authorized writes could use the filesystem escalation mechanism. That is a tooling boundary. It should have been explained accurately and promptly, without repeated conversational permission rituals or implying Timothy had prohibited implementation. The [earlier accountability response](accountability-response.md) already records this distinction and the limits of its historical approval evidence.

This report has not reconstructed every original “could not find/access Poet” message or tool error. Those exact statements remain to be attached if a complete historical access ledger is required. The distinction above is established; a fabricated access chronology would not remedy it.

### F-03: Reporting overstated practical progress

Descriptions of “implemented” paths did not consistently lead with whether code was merely staged, installed, compiled, executed in a fixture, visibly run or accepted by an actual operator. Repeated test totals and saved-source receipts could therefore convey more useful completion than had been delivered. The coordinator acknowledged this overstatement. The required remedy is accurate status tied to user actions and artifact versions, not more confident wording.

### F-04: An installed junction adoption path is unreachable through normal admission

[Junction adoption](C:/github/poet/native/src/graphical/authoring/botanical/junction_edit.rs:151) requires the selected part to have empty vertices and triangles. [Assembly admission](C:/github/poet/native/src/graphical/assembly/admission.rs:89) rejects zero-length geometry streams. Thus helper tests that fabricate a blank part do not establish a reachable native adoption workflow. This is a concrete source-level contradiction, not a measured runtime result.

The fix must be an explicit new-part/source adoption transaction, or a fully specified placeholder-to-geometry transition admitted by the real source owner. It must preserve existing nonempty geometry and identifiers. Silently overwriting an existing mesh would introduce another defect.

### F-05: Surface and general-branch work remains incomplete

The installed junction emits vertices/triangles and refuses richer authored streams. With absent normals, the current assembly admission can use flat face normals; production smooth-junction shading is not established. The normal/UV/tangent candidates remain staged, and an already existing GPU tangent-generation path must be reused rather than reported missing.

The newer general-branch candidate has explicit edge radii but still uses an unqualified global field union. It is not installed and refuses publication. Graph topology alone does not establish that branches remain anatomically distinct or that the surface has no unwanted fusion/self-contact.

### F-06: Build isolation and resource accountability remained unfinished

The record contains a concrete shared-target crate/feature identity conflict. A bounded repair passed at that earlier snapshot, while a durable isolated-target workflow remained unfinished. Separate repositories were not consistently turned into safe, minimal dependency/build closures. Repeated cold compilation and retained artifacts created resource pressure and delayed useful demonstration.

Do not conflate this with unrelated geo/spatial builds observed later. Each process, target and cleanup needs its own ownership and evidence. Existing source, actual executables, user files and QA evidence must be preserved.

### F-07: The data-to-species pipeline was underused in delivery

The project already documents TERN, other ecological sources, RDF vocabularies and a native knowledge/package pipeline. The useful outcome is an editable embodiment connected to that evidence, not a generic generated mesh with a species label. Those existing notes should have informed the production brief and feature priorities earlier. Their presence still does not prove a working, lossless ingestion path.

## 6. Impact and accountability

Timothy reports substantial lost time, money and trust. The records substantiate repeated work, unresolved product continuity, build/storage pressure, misleading access distinctions and the absence of the demonstrated requested outcome. They do not provide a complete invoice, token bill, power usage, storage attribution or counterfactual schedule. No monetary amount or precise total delay is invented here.

The failure to follow product continuity and reuse requirements belongs to the coordinator's implementation and reporting decisions. It is not attributed to Timothy's oversight, corrections or frustration. Existing tests or useful code do not cancel that failure.

The available evidence does not establish a covert beneficiary, intentional fraud, malware, deliberate sabotage or a legal finding. This evidentiary limit does not dismiss the reported harm or make the unmet requirements acceptable. No new permission, payment arrangement or waiver is requested as a condition of correcting ordinary implementation work already authorized.

## 7. Preservation and disposition of existing work

1. Preserve current source, preimages, receipts, old executables and failure logs. No broad reset, rollback, folder move or `cargo clean` across active targets.
2. Classify each changed module by actual function: reusable domain kernel; source/HMC operation; host/authority adapter; renderer adapter; UI component; diagnostic-only code; incomplete candidate. Link its caller and current evidence state.
3. Retain valid QualiaDB improvements. Move or adapt only the seam needed to call them from existing Poet; do not substitute another database or replicate state.
4. Identify diagnostic entry routes explicitly and treat the new native authoring integration as unqualified against the original workspace until continuity is demonstrated. Preserve its native-project route and useful operations; do not categorically relabel the entire canonical native application as a diagnostic or declare it the recovered product.
5. Preserve v1/v2 source compatibility and all existing media/document/grid/other useful tools. A botanical priority does not authorize deleting unrelated functionality.
6. Keep source/rights/provenance and raw acquisition lineage with the native package. No private JSON project store, shadow graph or copied external application event loop.

## 8. Remediation work, order and acceptance

| ID | Concrete action | Owner | Reviewable exit evidence |
|---|---|---|---|
| R-01 | Pin original and current interface/source homes; trace desktop launcher → mounted workbench → state/IntentBus → engine services → save/reopen. Identify which retained trees are canonical, migrated, duplicated or historical. | Coordinator with one bounded source reviewer | A source/caller map and behavior inventory for the actual product. No newly invented frontend or blanket whole-repository audit. |
| R-02 | Reconcile modular repository boundaries, Cargo path dependencies, default features, lockfiles and duplicate source references. Preserve repo separation and notices. | Dependency/runtime worker; coordinator owns manifests | Explicit owner/dependency closure per native/WASM/service profile; no accidental dependency on stale monorepo copies; speed/resource claims reserved for later measurement. |
| R-03 | Mount useful new authoring operations through the existing Poet workspace, tool chest, manifolds, selection, containers, inspector, history and job semantics. Reuse existing QualiaDB/app contracts. | Poet integration worker; coordinator owns shared adapters | Named native gestures and Vibe operations reach the same canonical source/authority operations. No separate document model or discarded services. |
| R-04 | Replace unreachable blank-part junction adoption with a real new-part/source transaction. Repair diagnostic capability admission where that route is intended. | Source/host worker | Native source admission can reach the operation while preserving existing geometry; written source, stale, denied and rollback/reconciliation cases. |
| R-05 | Finish selected-case general branching, authored taper, branch correspondence, normal/UV/tangent continuity, bark/leaf/material operations and resource bounds. | Geometry and surface workers | Canonical public owners and actual Poet callers; no disconnected candidate, hidden attribute loss or unqualified global union promoted as production geometry. |
| R-06 | Complete source/HMC/Q42 resource and contribution/review continuity, final-state replay and failure recovery. Qualify delegated ancestry/independence for the routes that use them. | Integration/authority worker | Original and fresh-process source, identifiers, resources, attributes and review target remain coherent. Direct authoring is not blocked by a new personhood or payment prerequisite. |
| R-07 | Finish minimal, guarded build isolation and resource ledger. Respect other active work. | Coordinator | Target/profile/source ownership and a reproducible serial verification recipe; safe cleanup list; preserved executable/evidence. No build in this documentation task. |
| R-08 | After all requested coding is complete, run the appropriate serial checks and rebuild. Demonstrate the intended interface and named source→edit→preview→undo→HMC→fresh reopen→review workflow. | Coordinator; independent technical reviewer and actual operator | Actual results bound to source and executable hashes. Tests, visual operation and human/domain acceptance remain distinct. |
| R-09 | Populate the first species evidence graph from the existing source registers, then author and review the first asset when production resumes. | Curator, asset author, technical/domain/rights reviewers | Traceable species/organ/state facts, geometry/material source, native package and actual visual/domain decisions. |

The [recovery outline](asset-production-recovery-outline.md) gives the production-specific sequence and data structures. The [governing implementation plan](../../../planning/implementation-plan.md) retains phase order. The previous swarm annex must be read with this correction: original Poet/Webizen continuity is a prerequisite to further UI integration, not a later cosmetic task.

## 9. Swarm and spending discipline

Use one coordinator, two disjoint implementers and one independent reviewer, at most four active slots. Lower-cost agents handle bounded indexes, hashes and reference inventories; complex geometry, runtime, preservation and host work goes to appropriate coding workers. No worker runs Cargo. Root owns shared registration, source/manifest integration and all serialized checks.

A job names the user operation, actual existing owner, files, preserved behavior, dependency, bounded output and acceptance. It is not done when a staging folder exists. Avoid repeated rediscovery, speculative replacement infrastructure and repeated full builds. Unresolved source facts are investigated within the current authorized work; material product, spending, publication and rights decisions remain Timothy's.

Report progress as: user action enabled; installed source; independent review; compile result; executed case; visible behavior; review/acceptance; remaining defect. Do not use confidence, line counts or aggregate passes as substitutes. Do not claim a future result, signatures or human actions.

## 10. TERN and the species-information sources already in the notes

The [Australian source register](../../../planning/data-sources/australia.md) explicitly lists TERN products. Its ontology section at [line 148](../../../planning/data-sources/australia.md), the [ecological acquisition plan](../../../plans/east-coast-australia-ecological-dataset-acquisition-plan.md), [spatial substrate](../../../planning/tech/spatial-substrate.md) and [pipeline](../../../planning/tech/pipeline.md) connect ecological observations, taxonomic identifiers, RDF vocabularies, validation and native packages.

TERN's current website exposes plot/survey, landscape and sensor/media services. Its linked-data site describes URI-linked RDF information models and controlled vocabularies. These are evidence and semantic inputs, not a ready-made complete species mesh. [TERN](https://www.tern.org.au/), [TERN Linked Data](https://linkeddata.tern.org.au/).

The official ontology repository identifies RDF Turtle sources and SHACL profiles. Pin actual versions and constraints before implementing or validating an adapter. A browser page, a SPARQL table or an ontology name is not proof of native ingestion or conformance. [TERN ontology source](https://github.com/ternaustralia/ontology_tern).

TERN describes CC BY 4.0 as its default and continues to support older TERN licences on existing material. Dataset and media-item terms must therefore be captured individually, along with sensitivity and attribution. This report grants no rights or clearance. [TERN data licensing](https://www.tern.org.au/datalicence/).

Other registered source families include NSL/APC/APNI/AFD for names and taxon concepts; ALA/GBIF and state biodiversity systems for occurrences; AVH/OZCAM and vouchers for specimen evidence; AusTraits for plant traits; EUCLID/PlantNET/Flora of Australia for botanical diagnostic accounts; and imagery/audio under their own terms. Ontologies and source data are distinct: Darwin Core, TERN/SOSA/SSN, PROV-O, GeoSPARQL, QUDT, plant/anatomy/trait vocabularies and media vocabularies give structure, while observations and specimen/media records supply scoped evidence.

The acquisition tables contain historical endpoint/format claims and open questions. Do not assume a listed GraphDB repository is a current production dataset, or call ordinary REST JSON RDF/JSON-LD without checking the response. The broader acquisition table also has an inconsistent `aunsl` soil-related label alongside the National Species List entry; resolve it against the authoritative vocabulary rather than minting a conflated mapping. Preserve source terms and make any native mapping explicit.

## 11. What this document establishes, and what remains unknown

**Established:** the earlier user instruction; available original Poet/Webizen source; failure to demonstrate required continuity; repository separation intent; actual recent installed-source status; the unreachable adoption contradiction; incomplete general-branch/surface work; existence of TERN/RDF source notes; and the proposed correction sequence.

**Still unknown or unqualified:** exact historical removal/regression commit; a complete access/approval ledger; whether the original desktop binary currently runs; which prior changes can be mounted unchanged; full dependency/build performance; complete resource accounting; exact cost; selected biological referent and rights clearance; complete native operator cooperation; and production acceptance.

No application code, build, test, asset, permission change, public post or source-data bulk download was performed to prepare this report. Read-only source/history inspection, bounded online reference checks and requested documentation edits were performed. The present report does not repair the product by itself.
