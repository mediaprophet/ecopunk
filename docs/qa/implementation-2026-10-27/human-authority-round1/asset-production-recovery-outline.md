# From the Poet continuity incident to Ecopunk asset production

Prepared 28 October 2026. Companion to the [incident and remediation record](poet-continuity-incident-and-remediation.md). This is an actionable recovery outline under the existing [governing implementation plan](../../../planning/implementation-plan.md), not another replacement architecture.

## 1. Start with the intended product

Recover the actual Poet workflow attached to Webizen Desktop and the existing Rust/Dioxus `PoetWorkbench`. Reconcile it with canonical `C:/github/poet` source and the published tool-chest/manifold interface. Preserve menus, containers, selection, inspection, history, media/tools and engine services while integrating new authoring operations. Source modularisation into separate repositories must preserve product behavior and improve dependency/build boundaries; it must not produce a different application by accident.

**First reviewable result:** the actual source/launch/state/service chain is identified, and every required authoring action has a named existing host and operation. Stop assigning additional UI to the diagnostic workbench as if that already satisfied continuity.

## 2. Correct the concrete authoring blockers

1. Implement a reachable new-part/junction-adoption transaction. Current empty-part requirements contradict normal geometry admission. Preserve existing nonempty geometry and source identifiers.
2. Finish graph-aware branching beyond one four-organ junction, with explicit per-branch taper/radii, deterministic regeneration, appropriate clearance/contact treatment and no accidental adoption of proposal nodes.
3. Integrate branch normals, authored hard/smooth groups, UV seams/atlas correspondence and tangent frames using existing QualiaDB owners. Preserve authored streams or require an explicit supported migration. Complete the bark/leaf/material tools required by the first botanical case.
4. Complete shared work, dispatch and actual working-set accounting through edit, source evaluation, inspection, rendering handoff and replay. Logical element counts must not masquerade as a hard process-memory cap. Make useful edits and fresh reopen fit an explicitly selected profile.
5. Connect these operations to the original Poet controls and Vibe access, source/HMC history, undo, review and recovery. Reuse existing authority/rights owners for direct and delegated routes as applicable.

**Exit:** the full selected native workflow is in canonical source with no disconnected module, unreachable precondition or known guaranteed-refusal path. Tests and independent expected results are written. This is source completion, not verified functionality.

## 3. Use the species-information resources already registered

The main starting references are:

- [Australian data-source register](../../../planning/data-sources/australia.md), especially TERN at line 63 and ontology/format notes around lines 148–189.
- [East-coast ecological acquisition plan](../../../plans/east-coast-australia-ecological-dataset-acquisition-plan.md), especially ontology and species linked-data tables around lines 423–470.
- [Native acquisition/validation/package pipeline](../../../planning/tech/pipeline.md).
- [Existing tree and wallaby reference/rights research](reference-resolution.md).

| Information needed | Existing source families | Structured content to retain | What it can support in an asset |
|---|---|---|---|
| Taxon and name concepts | NSL, APC/APNI, AFD; qualified ALA/GBIF links | Source taxon-concept and name identifiers, authority, accepted/synonym status, version and qualified mapping | The subject and its classification, with species/subspecies distinctions intact. A matching label is insufficient. |
| Occurrence/specimen | ALA/GBIF, state systems, AVH/OZCAM, vouchers | Darwin Core occurrence/event/organism/material/specimen identifiers, date, location precision, determination and record licence | Locality/individual evidence and scope of a representation; an occurrence is not automatically the depicted individual. |
| Plot and ecological context | TERN EcoPlots/AusPlots, SuperSites, relevant OzFlux/landscape products | TERN/SOSA observation, feature of interest, sample, observed property, method/procedure, result, time, geometry and units | Habitat, vegetation structure, associated taxa and environmental conditions. Plot context is not a complete organism model. |
| Quantitative traits | AusTraits and appropriate primary measurements | Trait definition, value/distribution, unit, method, biological state, source/specimen and uncertainty | Bounded authoring/simulation parameters with explicit provenance; no unsupported universal dimensions. |
| Diagnostic form | EUCLID, PlantNET, Flora of Australia, suitable expert/collection records | Diagnostic anatomical statements, taxonomic scope, views, measurement basis, uncertainty and source revision | Branch/leaf/bark/flower/developmental anatomy as applicable to the chosen tree. |
| Visual/audio references | TERN EcoImages/EcoAcoustics, ALA/collections and other permitted sources | Media identifiers, creator, exact version, subject/location/time, view, licence, attribution and access limits | Material/shape/audio references within their actual terms; access to a thumbnail is not derivative clearance. |
| Meaning and lineage | Existing TERN, Darwin Core, plant/anatomy, PROV-O, GeoSPARQL, QUDT and media vocabularies | Versioned IRIs, types, relationships, typed literals, units, evidence and derivation records | A queryable knowledge-bearing embodiment linked to editable geometry and resources. |

TERN's [Linked Data site](https://linkeddata.tern.org.au/) and [ontology source](https://github.com/ternaustralia/ontology_tern) supply information-model and shape resources. Verify actual current dataset endpoints rather than blindly copying an old GraphDB URL. Record each item's terms; the [TERN licensing page](https://www.tern.org.au/datalicence/) describes a default and supported historical licences, not uniform clearance for every item.

Not all these sources publish RDF. Reuse existing native parsers/adapters for the actual transport: RDF/Turtle, SPARQL results, Darwin Core archives, CSV, REST payloads or referenced dense/media resources. Native authoritative persistence remains QualiaDB/Q42/HMC. Source JSON does not justify inventing a private JSON project database.

## 4. Build one traceable species evidence slice

Choose the first bounded production subject: a taxon-level representation or an identified individual with a defined locality/state. Preserve the existing white-gum focus and its unresolved identification; do not silently substitute another eucalyptus.

For each selected item, use the existing pipeline:

1. **Discover:** find a small set of relevant records; verify source schema, endpoint and identifier meaning.
2. **Snapshot:** retain a dated, digest-bound source/version and access/rights record in the established acquisition location. Preserve raw source unchanged where permitted.
3. **Review:** record applicable rights, sensitivity, cultural restrictions, quality and relevance. Unknowns remain explicit. No paid/provider or external communication is implied.
4. **Map:** retain source vocabulary and provenance; express explicit native mappings with units, frames, dates and identifier distinctions. Separate taxon, specimen, organism, observation and depiction.
5. **Validate:** apply the pinned applicable shapes and native integrity checks. Validate semantics and unit conversions, not merely parser success. Do not rewrite source ontology meaning silently to fit an app structure.
6. **Query:** retrieve the trait/observation/media evidence needed by the selected organ/state. Return unknowns and conflicting evidence honestly.
7. **Bind:** link editable organs, geometry/material parameters and source resources to evidence. Sampling-edge indexes are not anatomical identifiers. Artist choices and inferred parameters must be labelled as such.
8. **Package:** retain the complete relevant Q42 graph/resources, editable source and derivation lineage in HMC, with permitted public/restricted handling.

There are existing reuse seams: [GBIF occurrence adapter](C:/github/qualiaDB/crates/qualia-core-db/src/domains/geospatial/adapters/gbif_adapter.rs:49), [SPARQL adapter](C:/github/qualiaDB/crates/qualia-core-db/src/domains/geospatial/adapters/sparql_adapter.rs:32), and [typed metric-trait gate](C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/metric_trait_gate.rs:14). The inspected SPARQL adapter's generic variable/hash mapping does not establish preservation of RDF term kinds, datatype/language, blank nodes, graphs, units or TERN shape semantics. Trace and improve existing owners as necessary; do not declare ingestion complete from their names.

**Reviewable result:** one species/organ/state evidence slice can be queried in QualiaDB and follows its source, rights, units and geometry parameter into HMC. Reference gathering and tooling can proceed in parallel; unsupported biology does not become an invented fact.

## 5. Verify the completed coding and show the real Poet

Respect Timothy's condition: no Poet tests or rebuild until all currently requested coding is complete. A four-organ subset or source-only receipt cannot waive that condition. Reconcile the active work list explicitly before starting verification.

Then one coordinator pins the source/dependency closure, isolates target/feature builds, checks disk/process ownership, runs the relevant serial regressions, fixes actual failures, preserves the old executable and rebuilds Poet from the qualified snapshot. Verify actual GPU preview, source/attribute retention, HMC save/fresh reopen, repeated edits, stale/denied operations and review target binding.

Show the intended tool-chest/manifold workspace and the real action sequence: open admitted project → create/select plant part → edit branch/radius/material → see the result → undo/redo → save → close → fresh reopen → continue editing → review. Record executable and artifact versions. Automated diagnostics, actual operator use and domain/visual acceptance are distinct results.

## 6. Start producing the asset

After the applicable tool path works and the user releases production HOLD, create the first editable botanical master with evidence-linked branches, leaves/bark and applicable states. Review silhouette, junction form, shading/texture continuity and biological claims against the selected references. Repair concrete tool defects exposed by the work; do not retreat into unrelated suite development.

Maintain the complete dossier/claim ledger from the start. Initial authoring does not require payment rails, a new personhood provider, a spreadsheet project, completion of all species, or already-finished film/game/cartoon deliveries. Required rights apply when source material is used; visual/domain review can occur during production.

Extend the same pipeline to wallaby production using existing rig/animation/deformation and behaviour owners. Complete packages require all applicable expanded cases, editable knowledge/resources and reviewed film, game and cartoon embodiments; the game derivative must run in its actual consumer. A first botanical asset remains a milestone, not completion of the whole mission.

## 7. Small managed swarm, concrete deliverables

| Role | Work | Output |
|---|---|---|
| Coordinator/integrator | Original-host continuity, shared APIs/state, source/manifest saves, resource accounting and serial qualification | A coherent canonical native workflow and exact source/executable/evidence record. |
| Existing-host/source worker | Recover intended Poet mounts and expose source/geometry/persistence actions; fix unreachable adoption | Real controls in the intended workspace, preserving useful old behavior. |
| Geometry/surface worker | Selected plant operations and branch/normal/UV/tangent quality through QualiaDB | Public reusable kernels and connected authoring operations, with preservation cases. |
| Independent reviewer | Check contracts, entry-point reachability, source/attribute/authority preservation and evidence claims | Concrete findings closed before acceptance; no self-awarded readiness. |

Use a freed worker slot for a bounded lower-cost reference/index task. Keep at most four active slots, one owner for shared code and no parallel Cargo. Data research must reuse the existing register rather than restart a global search. A handoff or staged patch is intermediate work; the deliverable is a usable operation and ultimately a reviewed Ecopunk asset.

This outline records proposed work. It does not claim the recovery, ingestion, native usability or asset production has already occurred.
