# HMC embodiment package — shared logical contract

**Status:** specification draft v0.1 · **Updated:** 2026-10-24.
This specifies logical content and required operations. It does not mint a new
binary format, ratify wire keys, or claim that the APIs already exist.

Companions: [architecture](../../architecture.md), [species embodiment](species-embodiment-spec.md),
[made world](../../made-world.md), [per-asset requirements](digital-twin-asset-requirements.md),
[format pipeline](../pipeline.md), [style/scale](style-and-scale.md).

## 1. Role of the formats

HMC is the distribution unit for an entire embodiment. It carries intact native
payloads under the qualified HMC codec; it is not a renamed mesh or an external
URL list. Q42 carries graph data and its interpretation context; 10D carries
applicable compiled geometry/fields/material/rig resources. Vibe sources and
compiled behaviour, media, provenance and other resources accompany them.
Preserve existing codec responsibilities and custom-format semantics.

Ecopunk's current format planning selects core QBDL for HMC. The previously
recorded QBDL/ZIP discrepancy remains an upstream compatibility decision; the
suffix alone establishes no interoperability. Family metadata must be agreed
with the codec owner without silently changing binary layouts or tensor axes.

## 2. Identifiers and logical descriptor

The following groups MUST be discoverable through native engine inspection.
Names below describe concepts, not ratified field names or new ontology IRIs.

Each referent may have several identifiers under
[DOM-AX-7](../../domain-model.md#dom-ax-7-multiple-identifiers-can-refer-to-one-subject).
Logical descriptors retain identifier associations and source-qualified mappings,
not just a single entity key. Keep record/root/part/release identifiers and byte
digests distinct from all identifiers associated with the depicted subject.

| Group | Required content |
|---|---|
| Release | Package identifiers, digest, family, version, base/previous release, schema/codec/profile requirements and build provenance. |
| Referent | Stable entity identifiers and kind; species concept or made-world design/instance; labels and explicit equivalences. |
| Inventory | Every entry's role, type, length, digest, dependency references and declared rights/access conditions. |
| Graph | Q42 graph roots/partitions, lexicon and vocabulary/shapes closure, provenance/evidence and the release's coverage inventory. |
| Logical roots | Named complete objects, available variants and part membership; a root may span several payloads. |
| Components | Stable part IDs, parent/attachment relations, frames, transforms, units, constraints and role. |
| Representations | Depicted referent/parts, style, scale, resolution, state/variant applicability, resource membership and qualification. |
| Operation | Applicable physical/behavioural/animation/audio definitions, preconditions, capabilities and state schema. |
| Source | Editable native sources/recipes, deterministic parameters, derivation lineage and permitted reference/evidence content. |
| Admission | Required/optional distinctions, bounded dependency traversal, resource estimates, compatibility and transactional replacement rules. |
| Review | Distinct format, semantic, physical, visual, scientific and native-runtime evidence/status. |

Identifiers layers stay separate: entity ID → definition/version → package digest;
instance ID → state revision/journal; part ID → representation fragment.
Changing a texture or package digest does not create a different species or
manufactured vehicle. An authored synthetic instance must declare its origin.

## 3. Package completeness and resource selection

| ID | Requirement | Acceptance case |
|---|---|---|
| REQ-HMC-001 | A complete embodiment MUST carry its graph, sources and all required definition/representation dependencies for the declared release. | Disconnect the network: native inspection and declared operations resolve from the admitted release. External labels are not required to interpret core IDs. |
| REQ-HMC-002 | The manifest MUST enumerate complete logical roots, parts, representations, shared resources and exact dependencies. | Select the whole tree or vehicle: all required parts load with transforms applied once. A first-entry preview is insufficient. |
| REQ-HMC-003 | Full package contents MUST be distinct from the resident working set. | Inspect graph inventory without decoding all meshes; select one representation without loading incompatible alternatives. Bounds do not silently delete graph content. |
| REQ-HMC-004 | Graph queries and resource discovery MUST operate through native Q42/HMC/Vibe capabilities. | A native consumer finds a named part, queries its facts and admits its depiction without JS, a private JSON graph or an app-owned HMC parser. |
| REQ-HMC-005 | Each selected representation MUST retain referent/part identifiers and declare compatible stage, sex/phenotype where applicable, rig, behaviour, damage and fidelity states. | Reject an incompatible rig/state combination; style switching retains the admitted instance state. |
| REQ-HMC-006 | Entry validation, format checks and capability admission MUST precede execution/use. | Corruption, missing members, cycles, conflicting IDs, bad units, unsupported formats and exhausted budgets fail with an identified dependency path. |
| REQ-HMC-007 | Published definitions MUST remain immutable; mutable observations and session/instance state MUST bind to a release through the qualified state/journal path. | Simulation/repair does not rewrite the distribution HMC; wrong-base journals are rejected. |
| REQ-HMC-008 | Rights and sensitivity MUST apply per graph/resource partition and survive selective access. | A required restricted resource is gated; it is not silently omitted while claiming a complete public release. |
| REQ-HMC-009 | Native authoring, compilation and packaging MUST reproduce the declared release without Node/JS/V8. | A documented native invocation regenerates inventory, digests, graph and resources from retained source. |
| REQ-HMC-010 | Family/schema migration MUST preserve stable identifiers, lineage and compatibility evidence. | Old releases remain interpretable under their declared codec/profile; no extension reinterprets old fields silently. |
| REQ-HMC-011 | Complete model packages MUST expose a versioned per-deliverable claim/evidence/review inventory, covering film, open-world game and cartoon representations and all applicable movement/behaviour cases. | The [completion gate](model-completion-claims.md) resolves every required row/case and binds the final package/content evidence; no unchecked omitted deliverables or self-awarded human approval. |

## 4. Alternate representations and extensions

**Master/derivative direction:** [REQ-QUAL-001–005](../high-grade-production-and-civics-experience.md)
requires accepted high-grade masters, editable inputs and explicit delivery
derivation. Bind each derived representation/reduced export to its master/source
revision, settings/tool versions, target, output identifiers and preservation/loss
evidence. Preserve the full embodiment; state reduced-export coverage and base
dependencies explicitly. Exact field encoding remains subject to the shared
descriptor/codec review, not a private per-consumer format.

Several graphical representations can coexist inside the same HMC: captured,
realistic-CG, illustrated-CG such as the Community Grounds reference, stylized,
scientific cutaway, schematic and symbolic. Each depicts declared entities or
parts; none owns a competing copy of the species/design facts. Geometry LOD,
microscopic scale and artistic style are independent selections.

The complete release inventories its available representations and their
dependencies. Later depictions may be distributed as an associated extension
HMC, explicitly bound to the base embodiment identifiers and compatible release.
Such an extension is a representation release, not a new complete species
embodiment. To claim a self-contained complete combined release, the resulting
HMC must include its required base graph/resources; unresolved external bases
or URLs do not satisfy that claim. A cached extension may reuse verified base
bytes in memory without requiring duplicate resident copies.

If a representation intentionally omits or exaggerates detail, it declares that
scope and its interaction limits. It does not rewrite authoritative anatomy,
dimensions, physical constraints or component membership. New data/behaviour
assertions are versioned graph changes with provenance, not hidden art edits.

## 5. Native operation contract

The upstream interface must support these operations; exact signatures and
binding names are open, and the table is not an implemented API catalog.

| Operation | Required result |
|---|---|
| Inspect package | Verified release identifiers, family, inventory, graph coverage, roots, variants, dependencies and limitations. |
| Open graph view | Capability-scoped Q42 view with lexicon/ontology context, provenance and completeness record. |
| Resolve entity/representation | Stable referent and part mapping, compatible variant selection and exact bounded dependency plan. |
| Admit root | Verified transactional whole-object admission, retaining the old root if replacement fails. |
| Instantiate | New/recovered instance identifiers and state with explicit real/synthetic referent and base definition. |
| Invoke/query | Governed action intent or graph query through the owning native capability, returning actual result/refusal. |
| Change presentation | Style/LOD/inspection changes preserving entity, part mappings and admitted state. |
| Unload/checkpoint | Bounded resource release or state persistence with an explicit durability result. |

Resource planning, range/cache, graph reading, rendering and state control must
remain engine services. HMC index ranges and Q42 internal BIDX ranges are
different address spaces; neither a raw range response nor a parsed header is
proof of authenticated resource admission.

Cycle checks target invalid assembly containment or unbounded dependency
traversal. They must not reject valid cyclic graph knowledge, food webs,
service circuits or kinematic loops; those have their own bounded semantics.

## 6. Specification acceptance and open decisions

The family contract must express one species embodiment and one multi-part
vehicle/building without inventing a second format. Review graph completeness,
root/part identifiers, selective admission, variant compatibility, offline
interpretability, revisions and negative cases before implementation allocation.

Open: family metadata encoding/location; native inspector/resolver signatures;
schema and vocabulary versioning; extension composition; resource-limit model;
rights-partition storage. Qualified runtime evidence is separate future work.
