# Review and agent handoff: Biosphere algorithms Specifications

**Date:** 2026-10-26. **Status:** source-backed review and proposed documentation
reconciliation. No format/engine change, build/test, export, asset production or
acceptance is activated. The original [Biosphere algorithms Specifications](../../Biosphere%20algorithms%20Specifications.md)
is preserved unchanged: 547 lines, SHA256
`883fe0ecf692d6a9b6d55870ef28c729df125fc550e5f506e20343176d5c5afc`.
Physical line references below refer to that version.

**Recommendation:** retain the document as a broad design/research input, with
the corrections below attached whenever assigning it to an agent. Its vision
fits the intended ecosystem; its assertions are not yet a reliable implemented
capability map or binary-format contract. Do not implement its numbered layouts,
performance promises or scientific conclusions as if source and evidence already
established them. Useful new requirements can be incorporated under the existing
[governing plan](../implementation-plan.md), [biosphere contract](biosphere-earth-and-anatomy-capabilities.md),
[graph/cue/streaming contract](biosphere-graph-spectra-and-streaming.md) and
[Poet adaptation guide](poet-capability-adaptation-guide.md).

## Considerations to incorporate

1. **Connected biological and environmental embodiments.** Expand the existing
   plant/animal cases with explicit fungal, arthropod, microbial, soil/water and
   anatomical interfaces when applicable. Preserve whole/part/developmental
   identifiers and environmental dependencies in the complete HMC graph/resources.
2. **Authored and generative sources together.** Keep procedural recipes,
   parameters, measured/acquired sources, authored meshes/materials/rigs/clips,
   simulation restart state and reproducible derivatives. Their usefulness is
   complementary; a ban on meshes, recordings or authored clips would reduce
   quality and inspectability. Neither representation alone proves completeness.
3. **Multiscale inspection as an explicit product workflow.** The proposed
   Specimen Loupe is useful: select a subject/part, inspect sourced representations
   and uncertainty, choose a bounded resource closure and isolate experiments.
   Implement through existing Poet Manifold/container/selection/authority owners,
   with native user and agent operations, not a new app or hidden biology store.
4. **Typed cues and model boundaries.** Record quantity, units, frame/time,
   calibration, source, receiver, response and validity domain. Separate measured,
   reconstructed, authored, simulated, inferred and illustrative content; retain
   unknown/nonvisible bands rather than filling them with plausible values.
5. **Portable bytes and measured runtime layouts.** Explicit hot/cold budgets,
   reproducible cooking, optional target-specific data layouts and incremental
   residency are useful. Preserve current compatible files and qualified scopes;
   choose optimisations from measured selected workloads.
6. **Optional research and assistance.** Preserve frontier physics, quantum
   biology, model fitting and assisted exploration with explicit scientific/evidence
   status. Advanced models, model weights, NPUs and live QPUs are not universal
   prerequisites for high-grade authored masters or independent human operation.

These extend existing BSA/BGS/PA/PC/TOOL/ECO cases. They are not a replacement
phase order, silent catalogue scope reduction or a claim of product readiness.

## Corrections an implementing agent must consider

| Review ID / source lines | Required consideration | Evidence and owner |
|---|---|---|
| BR-01 / 12 versus 244–269 | Reconcile removal and retention of M64. Preserve existing M64 consumers and HMC's intact-file packaging; investigate decomposition only as a versioned measured option. | TECH-01; R-04/06, BS-30/60, IP-01/02. |
| BR-02 / 15,125–145,254,515 | Correct incompatible `.10d` assignments: 14 is UV/texture coordinates, 22 is voxels; 5 is reserved temporal; 18 uses PROC; 13 uses MAT4; 17 is affordance/entity data. Proposed MGEN/PHAS/STR0 or clip/development semantics need an explicit compatible profile. | TECH-02–04; current SectionType/codecs; PA-R01/11, R-02/03/04. No silent renumbering or reinterpretation. |
| BR-03 / 18,23,26,169–197,262 | Current QBDL version/header/alignment and `.10d` descriptors differ from the proposed new ABI. Universal 64-byte records, 256-byte offsets, 16-KiB pages and an encoding flag are not current general contracts. | TECH-05/10; format owners, R-04/06. Preserve old/unknown data and version extension proposals. |
| BR-04 / 18,26,184,241,268 | Existing range CRC/SHA checks are not Bao proofs; selected M64 reader accepts a whole supplied slice. Current variant resolver scans/allocates, rather than guaranteeing O(1). | TECH-06/07; R-04/06, BS-30. Qualify trust roots, acquisition/read granularity and selected-resource work before streaming/performance claims. |
| BR-05 / 32–48 | Keep Tensor10D roles, referenced payloads and inference-internal second-ten semantics. Do not put an arbitrary spectrum or exact u64 digest into a scalar f32 lane, or equate epistemic curvature/density with physical tissue. A branch index does not establish empirical truth or quantum amplitude. | TECH-08/09; SCI-01–03; R-02/03/05 and actual axis/profile owners. Preserve multiple qualified identifiers. |
| BR-06 / 151–197,201–242 | Treat allocation, speed, zero-copy, silicon dispatch and quantum caching as operation/backend/profile claims requiring evidence. Preserve developed Etalon capabilities; a failed path/token search is not a missing-engine finding. | TECH-09–13; existing Etalon handoff, R-04/06. Existing device/provider interfaces first; no duplicate HAL or speculative cache implementation. |
| BR-07 / 60,330–341 | `pipe_model_radii` is an implemented geometric exponent-accumulation operator. The inspected API does not take the claimed hydraulic/N-body inputs. The stated Weibull formula also fails its P50 definition. | ALG-01; SCI-12; R-02/05, BS-10A/B/E; reuse existing sim metamer/leaf owners and review units/calibration. |
| BR-08 / 287,352–355,423 | Existing differential growth performs 3D rest-edge growth/constraint corrections and local splits. The attributed full cotangent/tensor/reaction-diffusion surface model is not established by that body. | ALG-02; R-02, BS-10C/E. Preserve rest state, topology and organ/UV/rig correspondence; do not rewrite the useful kernel from an overstatement. |
| BR-09 / 377–395 | Curve extraction does not supply an anatomical rig/SKL1/skin/clip automatically. The selected PBD wrapper loses previous-position state across calls and does not expose the asserted compliant-tissue parameters. DQS/CPG names do not establish accepted locomotion or volume preservation. | ALG-03/04; SCI-12; R-03, BS-20A–E. Reuse actual skeletal/sim owners and reviewed connected master sources. |
| BR-10 / 216,408–410,515 | Selected flocking body is O(n²), with in-place order-dependent updates. Morton-grid/read-old-write-next behaviour is not present in that operation. Fungal/flight/pheromone integrations remain case-specific source/domain questions. | ALG-05 and domain findings; BSA-03–06/09, relevant future bounded research. Locate other owners before allocating replacement algorithms. |
| BR-11 / 64–121, 227–242 | Separate measured phenomena, conditional models, debated hypotheses and unsupported organismal extrapolations. Fix misidentified citations; preserve research options with falsifiable cases and uncertainty. | SCI-01–12 and source bibliography checks. No blanket rejection of quantum research and no claim of validated organism/medical response. |
| BR-12 / 433–469,473–499 | Distinguish taxon concepts, specimens, organisms, parts, observations, representations and releases. SHACL validates declared shapes; graph grounding/grammar limits do not establish truth or zero hallucinations. Complete knowledge is a finite corpus/release snapshot. | Ontology review; R-01/02/03/05/06, BGS-01/03/05/06 and species/HMC contracts. |
| BR-13 / 434,449,507 | Source/custodian permissions, labels, no-charge treatment and bounded obligations require existing qualified terms and authority. Do not invent a universal 10× cap, perpetual exemption, adopted agreement, payment or discharge from ontology/label/signature presence. | Ontology review; native project/commons/instrument and AGR/credit owners. No legal conclusion or rail implementation in this review. |
| BR-14 / 264,513–516 | Draft changes the working whitegum to E. pauciflora and introduces other reference species. Retain the current mountain-whitegum/wallaby brief until sourced taxon/specimen reconciliation; common names do not establish equivalence. Preserve broader examples as later cases. | ALG/ontology reviews; R-01 and existing dossiers. No silent source/corpus/species substitution. |
| BR-15 / 16,24,489–516 | Vibe remains source-evaluated with pinned 0.1/proposed 0.2 distinct. Optional bytecode/AI/SDK does not replace actual native human+agent authoring, editable recovery and contributor/reviewer work. | TECH-12; PA-R01/02/11/12, IP-03 C1–C8; existing native examples and actual evidence. |
| BR-16 / 50–52,199–205,309–330,509–516 | Fix duplicate/out-of-order headings and distinguish requirements, source facts, proposals and forecasts. Reconcile the four-phase roadmap under the single IP plan; ten species/all domains/quantum precomputation are not new serial prerequisites. | All reports; R-06. Retain every broader obligation and case without delaying an unrelated bounded native workflow. |

The [technical review](../../qa/implementation-2026-10-26/biosphere-specification-review/technical-reconciliation.md),
[algorithm review](../../qa/implementation-2026-10-26/biosphere-specification-review/algorithms-and-production.md),
[ontology/workflow review](../../qa/implementation-2026-10-26/biosphere-specification-review/ontology-and-handoff.md)
and [scientific review](../../qa/implementation-2026-10-26/biosphere-specification-review/scientific-claims.md)
provide exact source/paper references, limitations and proposed corrections.
They are bounded reviews, not a complete engine, scientific literature or
transitive dependency audit. Read the source-pinned evidence before declaring an
operator absent or commissioning new code.

Broader kingdom, scientific, astrophysical and research capabilities remain
explicit programme work, qualified by their actual cases and evidence. Keeping
them outside an unrelated first-output dependency does not delete them or lower
the accepted master-quality, complete-embodiment or three-target obligations.

## Copyable brief for another agent

> Review the pinned original Biosphere algorithms Specifications alongside
> `C:/github/ecopunk/docs/planning/tech/biosphere-algorithms-specification-review.md`.
> Treat its normative/implemented wording as proposed material to reconcile, not
> new authority to change source or formats. Read Ecopunk AGENTS, the governing
> implementation plan, relevant species/made-world contract and existing code
> audits. Preserve current native Poet work and use the requirements-first
> adaptation guide's actual ecosystem examples.
>
> Work only on the assigned existing R/PA-R/BS case and exact owned output paths.
> First trace its selected operation through current implementation, export,
> caller, prior receipts and tests. Reuse reviewed reads; inspect only unresolved
> seams. Record source hashes/ranges and classify documented/source-present/
> bound/executed/domain-reviewed/human-accepted separately. Follow BR-01–16 and
> the relevant TECH/ALG/SCI findings; unsupported or unknown cases remain visible.
>
> Specify the smallest compatible contract/action that enables the required
> native user and agent workflow. Keep source/part/resource lineage, units,
> precision, state/history, rights, failure recovery and complete HMC content.
> Keep style, LOD, biological stage, simulation fidelity and delivery target
> independent. Preserve authored/acquired masters and generative alternatives.
> Record exact proposed owner/files, unresolved decisions, evidence/oracle,
> blockers and next bounded action in the existing handoff template.
>
> Current direction is documentation/research: no engine writes, Cargo/build/
> tests/exports, GUI/Computer Use, provider/QPU calls, source moves or asset
> production. Implementation needs a resumed implementation assignment and the
> applicable source permissions; document review alone does not activate it.
> Never overwrite another active owner's outputs, invent acceptance or create
> replacement graph/geometry/rig/format/authentication infrastructure from a
> narrow search. One coordinator owns shared integration and serial checks.

## Existing assignments that consume the review

| Existing job | Bounded deliverable informed by this review |
|---|---|
| R-01 | Resolve exact reference taxon/specimen/corpus, sourced morphology/parameters, rights and unknowns. Keep existing dossiers and all target obligations. |
| R-02 | Map persistent organ/metamer/leaf state and authored graph edits to actual geometric/sim owners, units and source/reopen lineage; development simulation remains a separate case. |
| R-03 | Freeze actual connected mesh/rig/weight/clip correspondence, state/preservation and native animated preview contract. Biological anatomy/locomotion review is separate from an engineering rig oracle. |
| R-04 | Reconcile actual M64/.10d/HMC formats and selected Q42-resource acquisition/verification closure; propose extensions without ABI collisions or whole-file reads called streaming. |
| R-05 | Define one genuinely supported cue/model chain with typed quantities, domain/range/calibration/uncertainty, receiver and sourced response. Keep frontier alternatives explicitly research-scoped. |
| R-06 | Reconcile all evidence and proposed contracts against BR-01–16, current plan and native C1–C8; no repeated builds or acceptance from report counts. |
| PA-R01/02/06/11/12 | Reuse native owner/IntentBus/domain-adapter examples for contextual source/property/timeline/graph/loupe controls and actual same-state human/agent/cooperative authoring. EffectCraft's useful interaction/code candidates remain available where they serve those requirements. |

This review is an input to the [existing register](../biosphere-work-packets.json),
not another programme or phase. No jobs are dispatched by being listed.
Full foundation/Poet product gates, accepted biological masters, complete
film/open-world-game/cartoon packages and game-demo consumer acceptance remain
open. Asset-production HOLD and later payment-rail priority are unchanged.
See the [review evidence index](../../qa/implementation-2026-10-26/biosphere-specification-review/README.md)
for source coverage, independent document review and final consistency receipt.
