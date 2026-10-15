# QualiaDB vs traditional semantic stacks — why the engine matters for ecopunk

**Status:** draft v0.1 · companion to `engine-capabilities.md` (the inventory) ·
purpose: make legible *what is different*, for architecture and for anyone
evaluating why this project runs on QualiaDB rather than a conventional
semantic-web stack.

**Honesty rule:** traditional stacks are genuinely good at what they do. This
document names where they lead as well as where they end. Every QualiaDB claim
here is marked **[verified]** (source read this survey), **[doc]** (documented,
not yet re-verified), or **[design]** (architecture commitment, evidence pending).

---

## 1. What a "traditional semantic stack" looks like

The conventional architecture ecopunk *would* have been built on:

```
┌────────────────────────────────────────────────────┐
│ Web app / UI (JS framework)                        │
├────────────────────────────────────────────────────┤
│ Application tier (Python/Java/Node)                │
│  · business logic · authz · scientific compute      │
│  · file/assets · ML · rendering · workflow          │
├────────────────────────────────────────────────────┤
│ Semantic tier                                       │
│  · triplestore (Jena/GraphDB/Virtuoso/Stardog)      │
│  · SPARQL endpoint · OWL/DL reasoner (HermiT/Pellet)│
│  · SHACL validator · RDF import/export             │
└────────────────────────────────────────────────────┘
```

The semantic tier is a *database*: it stores triples, answers queries, validates
shapes. **Everything else** — the science, the permissions model, the media, the
app — lives in the application tier or scattered services, and the graph holds
at most *pointers* to it. The browser is a thin client that talks HTTP to the
endpoint.

QualiaDB's design is a different partition: **the governed compute substrate is
inside the data engine**, and the engine compiles to WASM so the *substrate
itself* can run in a browser tab or on an edge device.

## 2. Dimension-by-dimension

| Dimension | Traditional semantic stack | QualiaDB |
|---|---|---|
| Core record | triple/quad, variable-size | 48-byte `NQuin`, fixed-layout, parity-checked **[verified]** |
| Query | SPARQL (mature optimizers, billion-triple scale) | SPARQL library + mini-parser + QISP/VibeScript; bounded arenas **[doc]** |
| Reasoning | OWL DL tableau (HermiT/Pellet-class), rule engines | Subsumption DL, ASP stable models, LTL/CTL, N3 rules incl. **Defeater `^>`**; *no full tableau prover* **[verified]** |
| Shape validation | SHACL (external validator) | SHACL → SlgOpcode **compiler** — constraints become executable opcodes + per-domain SHACL libraries **[verified]** |
| Permissions | endpoint ACLs / app code | **deontic modality in the record** — Obligation/Permission/Prohibition are quins evaluated per-verdict; `entity_view::rights_filter` per-observer views **[verified]** |
| Temporal | versioning conventions / named graphs (bolt-on) | Allen algebra + RCC8 + LTL as engine modalities; `TemporalInterval` reasoning **[verified]** |
| Legal/rights semantics | none (text in literals) | **Hohfeld jural algebra**, STIT agency, capacity, delegation chains, contract formation, breach records → court-admissible **[verified]** |
| Scientific compute | export to Python/notebook | in-engine: bioinformatics, organic chemistry, geospatial, ML suite, statistics, symbolic math, units **[verified]** |
| Numeric substrate | none (strings/numbers as literals) | tensor/field containers (`.10d` FIEL), 42 MB Sentinel arenas, SIMD kernels, GPU dispatch via `compute_bridge` **[verified]** |
| 3D/media | URL pointers to external files | `.10d` geometry/skeletal/field sections + `.hmc` packs + portal renderer + skeletal player **[verified]** |
| Sensitive data | generalization by hand / access tiers | `NetworkDisclosureRegistry` consent-gated egress; `StewardContract` quorum unlocks; explicit-denial guard; DP + BFV homomorphic privacy **[verified]** |
| Claims & hedging | everything asserted or absent | epistemic certainty bands (assertive↔doxastic), **RDF-star allegations ≠ facts**, `SocraticDegradation` of medical/legal claims **[verified]** |
| Machine→human boundary | none | `ontology_align` **type-forbidden** from asserting `exactMatch` — human ratification is a type-level invariant **[verified]** |
| Provenance | PROV-O data (if authored) | provenance *fields* in every quin (context/metadata/parity), `text_span` anchors, `source_audit`, asset envelopes w/ licence inheritance **[verified]** |
| Licensing | portal-level note | machine-checkable `LicencePolicy`, **unknown licence fails closed**, derived works inherit union of obligations **[verified]** |
| Commons economics | none | `value_flow`: capped-ROI royalty pools that **discharge at threshold then free globally** **[verified]** |
| Execution model | server-side daemon | local-first volumes; WASM profiles → **the engine runs in the browser** **[verified]** |
| Identity | user accounts at the app tier | DID fabric, k-of-n relational recovery, ZCAP-LD delegation **[verified]** |
| Network | HTTP endpoints | QDNF fabric: authority, contracts, evidence, policy labels, replication, raw-ethernet + acoustic-BLE bearers **[verified]** |
| AI boundary | external LLM calls | `inference_guard` graph-backed boundary; local GGUF inference incl. browser kernels; receipts & budgets **[verified]** |

## 3. The structural differentiators (why it's not "SPARQL + extras")

**D1 — Semantics and compute share one record.** A medicinal claim, the SMILES
of the compound behind it, the SMILES parser that validates it, the licence
governing both, and the certainty band on the claim all live in the same
48-byte substrate. In a traditional stack these live in four different systems
joined by convention.

**D2 — Governance is a logic, not a middleware.** Rights aren't filters bolted
on a query result; Obligation/Permission/Prohibition are first-class quins with
a verdict engine, Hohfeld correlatives that detect *structural gaps* (a right
with no duty-bearer), and `PreventiveBlock` semantics that halt before harm for
non-derogable violations. Steward contracts, quorum unlocks, and explicit
denial are graph primitives — which is what custodianship needs, not a role
table.

**D3 — Bounded determinism instead of unbounded services.** 42 MB Sentinel
arenas, fixed-capacity zero-heap kernels, fail-closed budgets — every function
class is classified `HotZeroHeap`/`ColdBounded`. The tradeoff is real (see §4),
but it's what makes the same code legally runnable in a tab, on a phone, on a
mesh node.

**D4 — The browser is a first-class runtime, not a client.** Five WASM profiles
(`wasm-webcivics` → `portal` → `wasm-full`), a `wasm_bridge` exporting
cas/linalg/stats/transforms/units/bio/chemistry/medical/governance to JS, async
adapter fetches, WebGPU portal rendering, browser LLM kernels. *Verified counts:
469 `gpu-runtime` gates, 406 `wasm-scientific` gates — the gating discipline is
applied, not aspirational.*

**D5 — Evidence and hedging are native.** μ/λ evidential packing, certainty
bands, allegations-as-quoted-statements, attestations/VCs — a knowledge
ecosystem needs "we believe / the Elder states / the record claims / the
model predicts" as distinct values, not one assert bit.

**D6 — The machine proposes, the human disposes — enforced by types.**
`ProposedStatus` has no `ExactMatch` constructor. SHACL and the type system
carry governance; it isn't a wiki page of conventions.

**D7 — Dense and semantic payloads are siblings, not separate worlds.** `.q42`
volumes + `.10d` sections + `.hmc` packs share identity, provenance, and
licence machinery — a species page, its geometry, its audio, and its rights
ship in one governed package.

## 4. Where traditional stacks still lead — honest section

| Area | Reality |
|---|---|
| **Billion-triple query optimization** | Mature triplestores have decades of join-order/IO optimization over persistent indices. QualiaDB's model is bounded arenas + partitioned volume families — a different design point; large-scale ad-hoc analytics over a single monolithic graph is not its target. |
| **Full OWL DL tableau reasoning** | HermiT/Pellet-class complete DL reasoning does not exist in-engine. QualiaDB offers DAG subsumption, ASP, LTL, SHACL compilation — different, composable machinery; full-DL ontologies would need profiling/downstream mapping. |
| **Standards conformance breadth** | RDF 1.1/SPARQL 1.1/OWL profile conformance-suite status per format is **[to verify]** per serialiser; don't assume W3C-suite completeness. |
| **Ecosystem/tooling** | Protégé, triplestore admin tooling, LOD publishing pipelines, `rdflib`/`Jena` ecosystems are mature. QualiaDB tooling (CLI, MCP, QApps) is newer. |
| **Adapters at depth** | The adapter *framework* is real and consent-gated; several adapters are shallow or self-described stubs (e.g. GBIF). Depth varies — verify per source. |

## 5. What this unlocks for ecopunk specifically

- **A pack that is simultaneously dataset, app, geometry, and licence** —
  encyclopedic interfaces ship as governed `.hmc` bundles, not as a web app
  that *references* data.
- **Rights that travel with the records** — per-record licences, sensitivity
  classes, TK conditions, and steward contracts ride the quins into every
  derived work; the union-of-obligations inheritance is structural.
- **Medicinal/knowledge claims with correct epistemics** — certainty bands +
  Socratic degradation mean a phytochemical claim renders as an evidence map,
  not a directive; Indigenous teachings keep their qualification semantics.
- **Micro↔macro in one substrate** — `ReferenceFrame` scale continuum +
  `.10d` fields + bioinformatics + chemistry put a molecule and a biome in the
  same governed graph — no traditional stack has a *scale ladder*.
- **In-browser sovereignty** — Sanctuary volumes queried into private contexts
  can run *where the person's data lives* — on their device — because the
  engine runs there.
- **Scientific instruments as graph citizens** — an SDM (Gaussian process), a
  sequence alignment, a NetCDF subset, an ontology alignment — all invocable
  against governed data with receipts, not exported to an ungoverned notebook.

## 6. Architectural consequence for ecopunk

The traditional-stack instinct is: *database + application tier + thin client*.
For ecopunk that instinct is wrong twice — the "application tier" capabilities
(science, governance, rendering, inference) exist in-engine, and the client is
not thin. So:

1. **Don't re-implement engine capabilities as pipeline services** — wrap them
   as `semantic_instruments`/QApps with contracts and receipts.
2. **Design to the modality model** — our axioms (custodianship, standing,
   living/made, borders) should compile to deontic/jural/epistemic quins +
   SHACL, not live in application if-statements.
3. **Plan for partitioned volume families**, not one big graph — the bounded
   model wants source×lane×region partitioning (matches pipeline §5 sizing).
4. **Treat the WASM profile ladder as the interface contract** — each
   interface family declares which profile it needs; Sanctuary-first features
   target in-browser execution by default.

## 7. Topology assessment — it is not a plugin (qualitative)

Considered qualitatively, the designed topology differs from traditional
internet/web/AI/OS ecosystems along most of the axes those ecosystems are
*defined* by — which is why "adapt it into a conventional model" is not really
available. It may serve as an **upgrade path** for them; it cannot be a plugin
for them:

| Axis | Traditional ICT topology | This topology |
|---|---|---|
| Record substrate | Files/rows/blobs; semantics layered on top | 48-byte NQuin — sensitivity class and routing lane are *bits in the datum itself* |
| Compute | Unbounded services over data | Bounded, deterministic kernels inside the data substrate |
| Authority | Platform accounts; provider is root | Human-rooted delegation chains (SOC-AX-14); agents always downstream |
| Economics | SaaS billing / advertising enclosure | Obligation pools, category-scoped royalties, micropayments, discharge-to-free |
| Distribution | DNS-anchored HTTP endpoints | Multi-carrier: content-digest + `did:` identity, HTTP as timestamped alias, DHT/DLT/sneakernet |
| Identity | URL/account *is* identity | Identifier is a pointer; identity is relational (SOC-AX-8/10) |
| Epistemics | Boolean rows; confidence as metadata | Objective/Inferred/Hearsay tiers, certainty bands, proposals, recorded contests |
| Time | Mutable current state | Everything versioned; historical states first-class |
| Law/policy | Middleware/advisory layer | Deontic+jural modalities in the evaluation plane — fail-closed |
| Experience | Pages/apps atop data | Manifold worlds — one graph renders as scene, pane, page, sonified state, game asset |

**The asymmetry that makes it "not a plugin":** this topology can *consume*
traditional data faithfully (adapters emit provenance quins; the ontology
registry ingests LOD vocabularies; HTTP URIs remain first-class aliases). The
reverse fails — a rights layer bolted onto a conventional triplestore is
*advisory middleware*; here the guarantee lives in the record's bit layout and
the evaluator's gates. Guarantees that live in the substrate do not survive
translation into a substrate that lacks them. Inward fidelity is possible;
outward fidelity is lossy by construction.

**Honest caveat:** this is an assessment of the *designed* topology — the
codebase is explicitly unfinished (stub subsystems are flagged throughout the
capability map), so the claim is "the architecture is genuinely different in
kind", not "the product is complete".
