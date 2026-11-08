# Ecopunk whole-biosphere uplift — outstanding outcomes and qualification work

**Updated:** 28 October 2026, Australia/Sydney. Documentation and scoped source reconciliation; no new build, test, export, acquisition or implementation batch was run for this revision.

Ecopunk's scope is the living Earth and its coupled environment: flora, vertebrates and invertebrates, funga, microorganisms, communities/habitats, landforms/geology/soil, fresh and marine waters, atmosphere/weather and space weather. Made-world objects, people and civics scenarios retain their existing requirements. Botanical work is one starting case, not an architectural restriction or permission boundary.

This is a current outstanding-outcome view under the [governing implementation plan](../implementation-plan.md), [BSA programme](biosphere-earth-and-anatomy-capabilities.md) and [120 ECO packets](ecopunk-production-backlog.md). It does not create another independent backlog or reset saved work. Read historical gap statements against later source receipts and executions. **Outstanding** below means the stated product outcome still needs reconciliation, implementation where actually missing, or qualification; it does not mean every underlying algorithm is absent.

## 1. Shared critical path

| Work to finish | Reuse / existing basis | Evidence needed to close the outcome |
|---|---|---|
| **Restore and demonstrate the intended Poet workspace continuity.** Preserve tool chest, manifolds, graph/source, spatial editing, inspection and existing useful services. | Original Poet/Webizen Desktop routes, Rust PoetWorkbench and canonical Poet; [incident/recovery record](../../qa/implementation-2026-10-27/human-authority-round1/poet-continuity-incident-and-remediation.md). BSA-10, IP-03 C1–C8. | Actual user project open → select/edit → visible preview → undo → native save → fresh reopen → contributor/reviewer decision. API or offscreen diagnostics alone do not prove this. |
| **Reconcile the updated native format/resource path.** Establish actual supported M64/Q42/10d/HMC profiles and consumers without another serializer. | Existing native readers/writers, [format foundation](format-sdk-foundation.md), latest upstream fixes and prior format executions. BSA-01/11, ECO-021–030. | Exact producer/dependency/consumer versions; semantic roundtrip of supported geometry, sampled fields, time series, materials, identifiers and provenance; explicit unsupported/lossy cases. Current M64 export pause remains until ready. |
| **Source-qualified corpus acquisition and semantic resolution.** Broaden dataset/media coverage by subject and system. | [Australian](../data-sources/australia.md) and [Earth/space](../data-sources/earth-space.md) registers, native adapters and Q42 lexicon/provenance owners. | Real source records with rights, namespace/schema mapping, units/time/frame, uncertainty and preserved identifiers; distinguish duplicate/syndicated evidence and conflicting taxon concepts. |
| **Usable symbolic inference over that corpus.** Select existing logic/solver operations appropriate to each question. | [Source-to-inference contract](evidence-led-asset-production.md#7-source-namespaces-and-native-symbolic-inference); existing modalities, native host and Vibe paths. | Actual premises/rules/query → inspectable result and available derivation evidence; conflict/unknown handling, budget outcome and invalidation after source changes. No replacement graph store or logic runtime. |
| **Editable connected geometry and preserved attributes.** Finish and qualify the selected branch/organ/joint/surface operations. | Existing Core geometry, native botanical/rig controls, installed junction work and preserved general-branch/surface staging. BSA-03/05/06. | Reachable authoring actions; branch/joint correspondence, normal/UV/tangent continuity, authored attributes/weights and identifiers surviving edits and replay. Reinspect current source before redispatching older defects. |
| **Real resource and job accounting.** Bound CPU/GPU work, scratch/residency and cancellation at actual owner seams. | Existing scheduler/budget/streaming owners and installed junction reservation work. | Operation-specific measured work/memory and boundary refusal, cancellation/recovery, bounded resource closure and truthful incomplete results. Logical payload counters are not whole-process memory limits. |
| **Human-centred cooperative authority and history.** Preserve the principal, scoped delegation, attribution and review. | Existing authority, projects, source/revision/checkpoint and review mechanisms. BSA-10, AUTH-H1–H6. | Real contributor/reviewer actions and revocation/conflict/recovery cases where applicable. Inference cannot grant permissions or award human acceptance. Delegated-agent qualification is not an extra prerequisite to a separately admitted direct human edit. |
| **One reviewed source-to-consumer path.** Ensure the visible scene uses the reviewed asset and corpus revision. | Existing HMC resolution, native renderer and game-demo consumers. ECO-021–030/101–110. | Bind actual loaded resources/derivatives to exact source/output identifiers; remove stale/default fallback from qualified paths; assess presentation and runtime behaviour separately. |

These shared items enable useful assets incrementally. Finishing every species, every scientific solver or payment rails is not a prerequisite to the first bounded production case.

## 2. Domain work that must remain visible

| Domain | Outstanding production and qualification obligations | Existing allocation |
|---|---|---|
| **Trees and other vascular plants** | Reviewed root/stem/branch junctions and bark/leaf/reproductive organs; variation, developmental states, seasonal appearance, deformation and relevant physiology. Finish real editable branch/surface continuity rather than adding foliage to hide defects. | Existing white-gum dossier; BSA-03–06/08/10–12, ECO-061–080. |
| **Other photosynthetic life** | Evidence-based frond/thallus/colony forms, attachment and life histories where applicable; do not impose the tree grammar. Resolve actual taxon concepts and required scales. | BSA domain census and existing geometry/field owners. |
| **Vertebrate animals** | Coherent external/internal anatomy, skin/fur/feathers/scales, rig/weights/muscle or morph correspondence, locomotion/contact, behaviour, vocalisation and life stages for selected subjects. Reuse saved rig preview/stepping repairs; do not call those absent again. | Existing wallaby case and skeletal/IK/physics/audio owners; BSA-03–08/10–12, ECO-081–090. |
| **Insects and other invertebrates** | Species-specific segmentation, appendages/joints, soft-body or exoskeleton, wings/optics, metamorphosis, locomotion and relevant interactions; qualify aquatic and terrestrial cases independently. | BSA-03–08 and existing geometry, optics, deformation and simulation owners. |
| **Funga** | Evidence-linked hyphae/mycelia and fruiting bodies, substrates, reproductive/developmental states and decomposition/symbiotic relations. Neither a branching kernel nor a mushroom mesh proves the complete lifecycle. | Existing BSA census, branching/surface/field/domain mechanisms. |
| **Microorganisms and microscopic communities** | Source-scaled cells/colonies/biofilms, applicable internal structures, development/motion and environmental relationships; calibrated microscopy/reconstruction and uncertainty. Avoid invented anatomy behind an attractive shell. | BSA-01/02/04/07/08/12; existing vision/volume and biological owners. |
| **Terrain, geography, geology and soils** | Real source coverage, CRS/vertical datum, physical scale, strata/materials/cavities where required, erosion/deposition, surface/subsurface continuity and ecological interfaces. A labelled synthetic heightfield is not a recognisable real place. Reconcile the geo/spatial work currently being updated. | BSA-09, ECO-031–040/091–100, [spatial substrate](spatial-substrate.md). |
| **Freshwater, wetlands, groundwater, coasts and oceans** | Catchments/connectivity, flow/levels, bathymetry/tides where applicable, transport/wetness, aquatic habitats and source-backed boundary conditions. Separate visual water from qualified hydrology/ocean dynamics. | BSA-06/09; existing fluid, field, geospatial and simulation owners. |
| **Atmosphere, weather and climate context** | Time/location-specific wind, precipitation, temperature/humidity, radiation/clouds, atmospheric composition and forcing needed by the case; distinguish observations, forecasts, reanalysis and authored scenarios. Assess cloud/light/audio appearance separately from model validity. | BSA-05/09, Earth/space register; existing field/spectral/weather/simulation owners. |
| **Space weather and solar-terrestrial dynamics** | Acquire source-qualified solar/geomagnetic/ionospheric time series and model epochs; integrate only scientifically qualified couplings into physical fields, sky/aurora and relevant organism/technology cases. More detail in section 3. | Expanded BSA-09; existing `qualia-sim::space_weather`, Vibe/native host and Earth/space source register. |
| **Habitats, communities and ecological processes** | Place real organisms in supported distributions and conditions; trophic/symbiotic relationships, pollination/dispersal, decomposition, disturbance, fire and recovery where required. Distinguish individual development from population/community dynamics and distinguish hypotheses from measured relations. | BSA-04/06/09/12, existing domain/solver owners and complete model dossiers. |
| **People, made-world objects and civics scenarios** | Preserve people/object/place identity, materials, operation and interactions; make problems, proposed interventions, trade-offs and uncertainty understandable. Assess downstream learning/scenario outcomes using actual participants/review. | Existing [made-world contract](../made-world.md), high-grade/civics contract and game-demo work. |
| **Comparative anatomy and scientific education** | Correct sections/volumes, registration/labels/measurements and normal variation; declared educational purpose and qualified review. Education acceptance does not establish clinical validity. | BSA-07/08/12 and BSA-EDU-01–08; existing reconstruction, volume and inspection owners. |

Detailed organs, profiles, clips, transitions, behaviours and scenarios remain in the pinned complete-model catalogue and subject-specific dossier. This table does not replace or waive them.

## 3. Space weather: existing work and the remaining seam

The existing [Earth/space register](../data-sources/earth-space.md) already distinguishes static snapshots, versioned models, API captures and derived windows. Retain those modes, but a versioned model must include its declared validity interval; deterministic evaluation outside it does not establish physical validity. Historical endpoint/licence shorthand requires per-product verification.

The existing [Biosphere algorithms specification v2](Biosphere%20algorithms%20Specifications%20v2.md) also describes broader biological, geodynamic and cosmic cases. Its proposed container/layout and capability assertions must be reconciled with canonical format owners and the latest upstream M64 changes before implementation or completion claims. Neither that specification nor this list authorizes replacing the actual native resource stack.

Read-only source inspection in this revision found:

- `qualia-sim/src/space_weather.rs`: public solar-cycle and solar-wind coupling functions, state types for solar wind/IMF, geomagnetic and atmospheric electrical quantities, and unit-test bodies. The functions use simplified parameterised relationships, including drought/lightning outputs. Those outputs need independent scientific justification/calibration before affecting qualified ecology or fire scenarios; they are not observations merely because a function returns them.
- `qualia-mcp/src/tools/sim_space_weather.rs`: existing wrappers call those simulation functions. Their transport representation does not replace the native Q42 state/knowledge owner, and wrapper presence is not evidence of execution or model validity.
- `qualia-core-db/src/poet_host/invoke/studio_domains.rs::sim_space_weather`: the inspected adapter echoes selected inputs, classifies a storm using one threshold and returns fixed `magnetopause_re = 8.6` and `tec_tecu = 31.4`. This function body is not a call to the simulation owner. Trace its dispatch and consumers, then adapt the existing owner at that seam; do not present fixed outputs as measured dynamics. Verify storm-scale classification against the selected official scale before using it.
- `vibe/src/catalog/ids.rs`: `Simulation.space_weather` is already an operation identifier. Catalogue presence does not prove complete native source-to-consumer integration.

**Outstanding:** select actual observation/reanalysis/model products, preserve UTC/sample interval/latency and forecast issue/valid time, frames and units; distinguish solar-wind measurements at a spacecraft from Earth-arrival estimates; retain missing/quality-flagged records and model epochs. Separate solar geometry and lunar tides from solar activity, plasma/IMF, geomagnetic disturbance, ionosphere/thermosphere, energetic particles and radiation. Couple each supported effect through explicit physical parameters and applicable time/space scales, with benchmark/reference comparisons and disclosed approximation limits. Treat proposed ecological/behavioural causal links as hypotheses until independently supported for that case.

Official discovery/qualification inputs checked on 28 October:

- [NOAA SWPC phenomena](https://www.spaceweather.gov/phenomena) describes solar wind, geomagnetic storms, ionosphere, energetic particles and aurora as distinct phenomena. A single light position or Kp number cannot stand in for this full set.
- [Bureau of Meteorology Space Weather API](https://sws-data.sws.bom.gov.au/) documents Australian/regional or station index products and magnetic/aurora notices, including historical index access. It requires registration/API credentials. Discovery is not an acquisition or a request to create credentials.

No space-weather source test, host operation, historical replay or scientific comparison was executed in this documentation revision. Existing test bodies check selected model behaviour, not independent observational validation. Complete end-to-end readiness remains unassessed.

## 4. Quality and delivery obligations across all domains

1. Establish source rights and reference quality, including independent evaluation material where available; preserve source namespaces and derivation lineage in Q42/HMC.
2. Author high-grade editable masters with connected parts, supported states, materials and relevant motion/sound. Use measured comparisons and attributed artistic/domain decisions; polygon count and an opaque quality score are insufficient.
3. Preserve semantic selection, measurements and source/parameter links through pose, representation/LOD changes and save/reopen. Keep the full corpus distinct from a bounded resident resource closure.
4. Produce explicit film, game and cartoon derivatives with documented conversion loss, budgets and source lineage. An early study or one accepted view does not complete all profiles.
5. Demonstrate actual native presentation and the real downstream game-demo consumer with the exact reviewed content. Assess recognisable places, ecological relationships and civics scenario comprehension.
6. Maintain every applicable catalogue/expanded obligation, unresolved issue and genuine review result. Do not infer acceptance, source rights, authority or payment from a generated label or receipt.

## 5. Practical order for uplifting the whole system

Keep the governing IP phase order. First reconcile the current native workspace and upstream M64/geo changes; finish the already identified shared authoring/corpus/inference seams. Then run bounded end-to-end cases in parallel domain lanes: the existing botanical case, the existing wallaby case, a real place with soil/water/weather context, and source-qualified space-weather historical forcing where applicable. Select small funga, invertebrate and microscopic cases to expose different structural/scale requirements without pretending that another tree qualifies them.

Research and reference curation can proceed alongside coding. Resume tests/builds only under the user's current coding-complete instruction and coordinated serial Cargo ownership. Resume the paused asset export when the upstream M64 update is ready. Do not expand each initial case into finishing all of biology before useful assets appear. Broader coverage advances through accepted cases and reusable native capabilities, with the remaining cases visible.

## 6. Evidence and limits of this revision

This view reconciles the BSA programme, original product handoff, governing plan, ECO backlog, source registers, [nine-location review](../../qa/implementation-2026-10-26/biosphere-integration-review/README.md), [later authoring evidence](../../qa/implementation-2026-10-27/full-scope-round1/continuation-authoring-presentation.md), original-workspace recovery record and current namespace/inference contract. Prior rig stepping/preview, source/review/concurrency repairs, material controls, GPU checks and rebuild receipts remain evidence for their recorded snapshots; they must not be relisted wholesale as missing. Later source-only junction/surface work is not automatically qualified by an earlier build.

The nine-location review includes bounded bodies/callers in domains, modalities, solvers, specialized_libs, wgsl_forge, qualia-vision, qualia-audio, webizen-render and semantic_instruments, plus sibling owners such as qualia-sim. It explicitly does not cover every implementation. This revision does not extend that into a full semantic audit or declare unreviewed functionality absent. The space-weather body observations above have exact file pins below; broader rows are production/qualification obligations anchored to existing plans, not fresh missing-code findings.

### Working-source pins

SHA-256 pins of inspected files; these identify source bytes, not executed qualification.

| Source under `C:/github/qualiaDB/` | SHA-256 |
|---|---|
| `crates/qualia-sim/src/space_weather.rs` | `6b56d986c3aef6fe75ca04b97eedd25a0d14541dc212d3e922e1f1425d3096c7` |
| `crates/qualia-mcp/src/tools/sim_space_weather.rs` | `5a1cc7ed64de19ce8796ac4daf68e35078ed43743afab9807b161652285260d6` |
| `crates/qualia-core-db/src/poet_host/invoke/studio_domains.rs` | `4abca8d9d0b4f4463bdf59fc2bba299c0e81f2374c6b76b0ce124c736f89ab12` |
| `crates/vibe/src/catalog/ids.rs` | `63e7205ba400889a4898a0d5937e8ac8bf7616148c48d459eabf038770296919` |
