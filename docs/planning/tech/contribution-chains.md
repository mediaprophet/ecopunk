# Contribution chains — many roads into one body of knowledge

**Status:** draft v0.1 · scope: the multi-kind contribution model — how photos,
preparation knowledge, lab chemistry, literature links and other contribution
kinds converge on the *same entity*, each with its own epistemic class,
governance lane, tooling, and obligation-cost accrual.

**The model:** the **entity is the convergence point**; contributions are typed,
provenance-bearing edges onto it. A person photographs a plant; someone else —
somewhere else, never having met the first — contributes the preparation
knowledge; a third does the organic chemistry; a fourth links the literature
about the compound found in it. Nobody coordinated. The *derivation graph*
does the coordination, and the royalty tree pays it.

---

## 1. Contribution kinds (register — extensible)

| Kind | Example | Epistemic class (`epistemic.rs`) | Governance lane | Tooling (verified) |
|---|---|---|---|---|
| **Observation/media** | photo of a plant, frog-call recording | Observation record + `proposesClass` *proposal* (Inferred) until human-confirmed | specimen coords may be sensitive → generalised | vision detector→`semantic.rs` observation quins; `ma-ont` fragments; audio `tf_surface` |
| **Identification/verification** | "yes, that's *Banksia integrifolia*" | human ratification — upgrades the proposal | reviewer-attributed | correction stream (`humanCorrects`/`humanRejects`); `ontology_align` ratify path |
| **Experiential / traditional knowledge** | how to prepare it for a medicinal use | banded/honoured — **never flattened to directive** | custodial-gated; language-bound (DOM-AX-5) | `epistemic` certainty bands; `epistemic_boundaries` Socratic degradation; consent scopes |
| **Laboratory / scientific analysis** | SMILES + descriptors of the compound found in it | Instrument-derived (Objective-class with device provenance) | licence + method provenance | `organic_chemistry` (SMILES→MW/LogP/TPSA…); `chebi_parse` map/query |
| **Literature curation** | links a published article about the compound | curated reference edge — *the link, not the text* (licence-bound) | source licence governs | `resource_catalog`; DCAT/VoID descriptors |
| **Linguistic / naming** | names in local languages | lexical facts — language-tag is semantic | community lexicon governance | OntoLex-lemon entries; `lexicon` multi-modal tokenisation (oral/phonetic/ceremonial) |
| **Curation/moderation** | labels, cleans, resolves disputes | meta-acts — recorded as labour | accountable roles | `labelledBy`/`moderatedBy`/`cleanedBy` labour quins |
| **Stewardship / hosting** | seeds a pack, relays QDNF traffic | service acts | commons economics | `pc:Micropayment` rails; pack redundancy accounting |

New kinds are **vocabulary additions, not schema changes** — the taxonomy is
open (same rule as `spc:` use-cases and `taxonomy.rs`).

## 2. The convergence graph

```
            ┌──── photo + proposesClass ──── human confirms ────┐
            │                                                  ▼
   plant in place/time ────────────────► TAXON entity ◄──────── identification act
            │                                                  ▲
   preparation method (custodial, language-bound) ──asserted──►│  use-knowledge edge
            │                                                  │
            └──── specimen → lab analysis ─► COMPOUND entity ◄─┤  wasDerivedFrom
                                                 ▲             │
                            literature link ─────┴─────────────┘
```

Every edge is a PROV-O activity in `PROVENANCE_CONTEXT`:
`wasGeneratedBy agent_did`, `wasDerivedFrom` upstream, activity
label/start/end, plus the labour predicates. **Disputes are first-class** —
`CONTEST_CONTEXT` carries `contestedBy/contestReason/resolvedBy` into the
Merkle-DAG, so a disputed identification or contested preparation claim is a
*recorded contest*, not an edit war.

## 3. Epistemic discipline per kind

The engine's three-tier epistemics keep the kinds honest *without* ranking the
contributors:

- **Objective** — `wasGeneratedBy` CryptoSensor/signed instrument event
  (lab result, signed observation).
- **Inferred** — `wasDerivedFrom` an AgentQuery (classifier proposal, a
  chemistry inference) — stays a proposal until ratified.
- **Hearsay** — reported-knowledge edges keep their informant DID — "elder X
  says this preparation…" is attributed, not anonymous.

Traditional/experiential knowledge rides **certainty bands** (`epistemic` +
`modal-junctures`), and medicinal-use claims route through
`epistemic_boundaries` → probabilistic maps + Socratic prompts — never a
flattened "take X for Y" directive. The contribution is *honoured by the
system's refusal to over-claim it*.

## 4. Obligation-cost wiring (the economics)

Each contribution kind is a `Contribution` record in the author chain
(`projects.rs`); derivation edges (`wasDerivedFrom`) define the royalty tree:

- The medicinal-pack consumer's payment flows **up the tree**: photographer →
  identifier → knowledge custodian → chemist → literature curator — by
  `ancestor_royalty` geometric split, no central accountant.
- Disconnected contributors merge safely (add-wins-by-id), so a contributor
  who only ever submits once, from a node that syncs a year later, still gets
  their chain position and obligation share.
- Biosphere/humanitarian-class work keeps the **fixed-cost rule**
  (`commons_cost` capped-ROI → discharge → globally free).
- Hosting/steward acts accrue `pc:Micropayment` — infrastructure contributors
  are in the same graph as knowledge contributors.
- **What a contribution is *due*** is computed, not declared — the
  fair-compensation model in [compensation.md](compensation.md) prices work by
  labour class × place (PPP/living-wage floor) × agent kind, anchored to
  published benchmarks (COMP-AX). The labour quin records the work; the
  compensation claim is the derived obligation.

## 5. Design rules

1. **Entity-convergent, contribution-divergent** — never require contributors
   to coordinate; the derivation graph converges for them.
2. **Kinds differ; dignity doesn't** — the photograph, the preparation
   knowledge, and the lab paper are *different epistemic classes*, but each
   contributor's labour-quin and chain position are equivalent in standing.
3. **Proposals, not assertions** — machine-derived edges (classifier,
   alignment, extraction) stay proposals until a human ratifies.
4. **Contests are records** — disputes persist with reason + resolution,
   keeping the chain auditable rather than silently rewritten.
5. **Contribution ≠ exposure** — sensitive kinds (specimen coords, custodial
   knowledge) keep their lane obligations even while accruing obligation
   costs; contribution to the commons never forfeits governance.

## 6. Open questions

1. Contribution-kind ↔ pack-section mapping (does each kind get a manifest
   section, or is it query-derivable from the graph?)
2. The contributor-role taxonomy — map to the ~17 `AgencyDomain`s and define
   `eco:` roles (collector / verifier / custodian / analyst / curator /
   translator / steward).
3. Whether a *single-act* contribution (one photo, never returns) can accrue
   meaningful obligation without a standing project — chain-only vs
   project-keyed contributions.
4. Contest-resolution governance — who holds `resolvedBy` standing per
   contribution kind (taxonomic authority? custodial body? project quorum?).
