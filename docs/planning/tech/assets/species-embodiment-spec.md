# Species embodiment — complete knowledge and resources in HMC

**Status:** specification draft v0.1 · **Updated:** 2026-10-24.
The complete-species-HMC requirement records Timothy's instruction. The logical
structure below is proposed for upstream schema/API review, not implemented
conformance or a reduction of the species knowledge scope.

Companions: [shared package contract](asset-package-spec.md),
[domain model](../../domain-model.md), [digital-twin requirements](digital-twin-asset-requirements.md).

## 1. The unit being packaged

A species HMC carries the **entire embodiment available to Ecopunk about that
species at a stated revision**: its graph data, evidence and interpretation
context, representations, authored operational models, media and dependencies.
It is the resource from which an application obtains what it needs. A game,
atlas, scientific inspector and film tool select different resources from the
same embodiment rather than maintaining separate species definitions.

The root denotes an explicit taxon concept/circumscription and its provenance.
Names and synonyms are labels/references; they do not collapse differing taxon
concepts automatically. Particular specimens, organisms, occurrences and
simulation individuals have separate identifiers linked to that concept.
Type-level facts must remain distinguishable from observations of individuals.

## 2. Knowledge coverage and completeness

“All available” MUST NOT become “the few traits required by this demo.” Each
release enumerates its acquisition/corpus revision, covered sources, records,
partitions, derived assertions, unresolved conflicts and known gaps. It includes
all species-relevant graph data held in that corpus revision, including the
provenance/evidence needed to distinguish incompatible or uncertain assertions.
This is a finite, auditable snapshot; it does not claim to know every fact that
exists anywhere or to have completed future acquisition.

| ID | Requirement | Acceptance case |
|---|---|---|
| REQ-SP-001 | One complete species release MUST contain the available species graph and embodiment resources inside HMC. | Reconcile the package inventory with the species corpus snapshot: every in-scope graph partition and required resource is accounted for. |
| REQ-SP-002 | Completeness MUST be evidenced by an explicit inclusion rule, source/corpus revision and reconciliation record. | A newly acquired trait, call recording or observation appears in the next release or has a stated unresolved packaging/access issue; it cannot disappear because the viewer does not use it. |
| REQ-SP-003 | Uncertainty, contradictions, historical assertions and unknown values MUST survive packaging. | Queries return attributed competing assertions and valid/observation times; they do not select an arbitrary unqualified “truth.” |
| REQ-SP-004 | Graph interpretation context MUST be available offline. | Names, relations, units, lexicon, necessary vocabulary/shapes and provenance resolve without external ontology/DNS fetches. |
| REQ-SP-005 | A rights-filtered or incomplete derivative MUST identify its coverage and MUST NOT claim the unrestricted complete embodiment. | Restricted records remain access-controlled; public exports state exclusions without leaking protected contents. Missing authority to distribute does not authorize copying. |
| REQ-SP-006 | Referenced ecological partners and places MUST retain resolvable identifiers and relation evidence. | A pollinator edge names its target and supporting context. The species HMC need not recursively contain every other species' entire HMC; any data required for a declared offline operation is included in its closure. |
| REQ-SP-007 | Representation selection MUST preserve the taxon/individual distinction and shared graph identifiers. | Illustrated and scientific views query the same species facts; switching style does not change an individual's sex, stage, injury or location. |
| REQ-SP-008 | Unsupported biology or representation MUST remain explicit. | An absent female form, organ reconstruction, behaviour model or molecular detail is unknown/unavailable, not fabricated or silently treated as inapplicable. |

Gated original releases may contain controlled partitions under the qualified
native rights mechanism. A public derivative may exclude protected content with
an appropriate non-disclosing coverage record. If the required rights mechanism
is unavailable, the affected release remains blocked/unqualified; incompleteness
must not be hidden by changing the meaning of “complete.”

## 3. Species knowledge groups

The graph MUST inventory these groups as applicable, available, unknown or
inapplicable. Available data is included even when no graphics use it yet.

| Group | Included knowledge/resources |
|---|---|
| Identifiers and taxonomy | Concept and name history, classifications, synonyms/mappings, source identifiers and disagreements. |
| Morphology and anatomy | Dimensions/traits, body plan, parts/tissues, anatomical and diagnostic descriptions, measured distributions and uncertainty. |
| Development and reproduction | Life stages, growth, phenology, reproductive strategies, applicable sex/phenotype variation, conditions and drivers. |
| Physiology and behaviour | Resource needs, metabolism, movement, senses, calls, affordances, environmental response and declared model limits. |
| Ecology | Habitat, community membership, trophic/host/pollination relations, environmental tolerances and evidence. |
| Distribution and observations | Occurrences/specimens, ranges, time/place, environmental context, sensitivity and observation provenance. |
| Molecular/cellular | Available chemistry, genes/proteins, cellular/tissue models and relevant evidence, with appropriate scale and claim boundaries. |
| Conservation and human relations | Attributed status/assessments, interactions, custodial/use knowledge and its authority/conditions; graph inclusion never transfers ownership of that knowledge. |
| Representations and media | Whole organism and parts; model variants, source recipes, materials, rigs/clips, audio, photographs, video, diagrams, descriptions and reference evidence as distributable. |
| Provenance and operation | Sources, transformations, rights, reviews, model/rule versions, graph shapes and native capability requirements. |

There is no requirement to invent every group for every species. Missing data
is visible work in the acquisition/specification register. Known data is not
discarded because its relevant simulation capability is unimplemented.

## 4. Organism, instance and representations

| Object | Identifiers / ownership of facts |
|---|---|
| Taxon concept | Species knowledge root; facts qualified by evidence/time/context. |
| Organism template | Sourced or authored parameter/rule set; declares which traits it approximates. |
| Real specimen/individual | Particular subject with observations, capture rights and actual state history. |
| Synthetic instance | New instance ID, seed, environment and admitted state; is not an occurrence observation. |
| Organ/part | Stable part definition/instance IDs with anatomical membership and physical frame. |
| Representation | Depicts the concept, individual or parts at a style/scale/fidelity; does not become the taxon. |

Variant dimensions include artistic style; world/organ/tissue scale; device and
geometry/texture/rig LOD; stage/season; and sourced sex/phenotype/reproductive
form. These dimensions are independent and compatibility is explicit. An
illustrated tree and scientific cutaway can coexist while sharing botanical
identifiers; a juvenile state is not a low-detail adult.

The supplied Community Grounds image illustrates a desired representational
family: coherent illustrated landscape, recognisable trees and people, vehicles
and working infrastructure. It is a visual reference, not evidence of species
identifiers, anatomy, locality, engineering performance or rights to redistribute
any depicted subject. Alternate styles follow the same identifiers contract.

## 5. Logical species HMC layout (illustrative)

```text
<species-release>.hmc
  release + inventory + completeness/reconciliation record
  graph/     Q42 species corpus, observations, evidence and lexicon
  ontology/  required interpretation context and validation shapes
  source/    native editable sources, procedural recipes and parameters
  roots/     organism/part/instance-template membership and variants
  depiction/ 10D world, organism, organ and scientific representations
  surface/   declared material/texture/field resources
  motion/    rigs, clips, morphs and compatibility definitions
  behaviour/ Vibe/rules, physical/lifecycle models and capability declarations
  media/     calls, photographs, video, plates and documents
  rights/    provenance, access conditions, attribution and review evidence
```

Paths are explanatory, not required wire paths; graphs may be partitioned over
multiple Q42 entries. The native consumer discovers the graph, asks for the
entity/part/representation it needs, obtains an exact dependency plan and admits
only that working set. Full-file install and authenticated selective acquisition
are delivery choices over the same complete release.

Apply [BGS-01–12 and BGS-Q1–8](../biosphere-graph-spectra-and-streaming.md) for
existing Q42/HMC reuse, physical spectra/cues and inspection-driven detail.
Closely inspecting a fish may select scale resources; insects in grass and
lichen on rock retain their own referents and evidenced relationships rather
than becoming LODs of their substrate. Magnification, graph queries and sensory
mode select compatible resources without fabricating new biological facts or
reducing the complete package. Exact ranges, authentication, cancellation and
CPU/GPU lifetime must be demonstrated independently of a complete-file preview.

## 6. Acceptance scenarios and open decisions

Specify one complete flora species and one complete fauna species. For each,
review a graph inventory, whole organism/part membership, film/open-world-game/
cartoon representations plus the applicable scientific inspection forms,
applicable life/sex variants, native graph queries and governed interactions,
offline dependencies and revision/state separation. Negative scenarios cover
omitted graph partitions, wrong taxon binding, incompatible rig/stage, corrupt
resources and a denied restricted record. These are future qualification cases,
not tests run by writing this specification.

Completion must account for the entire [per-model deliverable catalogue](model-production-template/README.md)
and its expanded clip/behaviour/variant/profile cases under
[model-completion-claims.md](model-completion-claims.md). The small scenarios
above are workflow fixtures; one query/interaction is not evidence that all
species movements, behaviour or package obligations are delivered.

Open: precise species-corpus inclusion/reconciliation query, taxon-concept
alignment policy, restricted-partition encoding, metadata schema and native
resource-discovery API. Upstream allocation is tracked in G71–G74.
