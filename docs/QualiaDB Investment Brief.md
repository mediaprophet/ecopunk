# QualiaDB — Briefing for Philanthropic Funders

*Prepared 2026-10-17. Every substantive claim below was verified against the project source
(`C:\github\qualiadb`, branch `0.0.40`) using a machine-generated audit of **5,056 source files /
54,077 functions** plus manual source inspection. Where the code implements a mechanism but
nothing yet calls it, we say so — the project's own documentation uses the same evidence
discipline, and we have kept it.*

---

## 1. Executive summary

**QualiaDB is a deterministic semantic engine for the living world** — a single computational
substrate in which data, rules, rights, evidence, geometry, and physics coexist inside one
48-byte record (an `NQuin`), evaluated by bounded, attested reasoning that runs natively,
at the edge, and **entirely inside a web browser** via WebAssembly.

Most databases store facts. QualiaDB also stores — and enforces — **what facts mean, what they
cost, who may use them, and under what obligations**. It was built because living-world value
(biomass, habitat, hydrological function, custodial relation) is not legible to today's
software, and commons cannot be governed at the speed and scale software operates at without
machine-checkable obligations.

The project is the work principally of a single creator (Timothy Holborn, Webizen), developed
in the open under **CC BY-NC-ND 4.0** — a non-commercial, commons-aligned license. It is real,
unusually deep, and honestly documented: the repository maintains its own divergence log of
where documents disagree with code, labels progress reports "Honest Progress," and marks
incomplete surfaces as such.

**Bottom line for funders:** the hard, differentiated parts exist and are verified —
deterministic logic across ~45 families, an ecological accrual engine, a commons-obligation
economic model, and a large scientific-computing stack. What is missing is mostly *wiring,
packaging, and demonstration* — the kind of work grant funding classically accelerates:
turning a vast, real engine into a usable public good.

---

## 2. The problems it addresses

### 2.1 Living-world value is not machine-legible

Natural-capital accounting today is a spreadsheet exercise — estimates, periodic reports,
unverifiable claims. QualiaDB's **accrual modality** (`modalities/accrual.rs`) treats standing
stocks as first-class semantic objects with *multi-dimensional time-depth*: biomass, habitat
structure, hydrological function, genetic distinctness, and custodial relation, each with
formation-vs-destruction asymmetry and six irreversibility classes. A machine can finally
reason over "how long did this take to form, how fast can it be destroyed, and what cannot
be rebuilt" — the asymmetry that ecological economics has always lacked in executable form.

### 2.2 Commons cannot be governed at software speed

Licenses, obligations, and provenance today are legal text detached from the assets they
govern. QualiaDB implements an **obligation-cost / commons-sunset economy**: every asset can
carry a production-cost ledger; payments accumulate against it; when the cap is met, the
obligation is discharged and the asset's licence class transitions to a permissive commons —
*automatically, inside the data format itself* (`CommonsObligation` in the bundle manifest,
`discharge_to_commons()`). Anti-usury ceilings, EROI thermodynamic-viability gating, and
generational royalty trees are implemented in the same zero-allocation arithmetic.

### 2.3 AI systems cannot say "I don't know"

The engine treats **uncertainty as a first-class citizen**: interval type-2 fuzzy sets
("about 0.7, but unsure how sure"), evidential (μ, λ) packing bound to verifiable
credentials, Socratic degradation (emitting probabilistic maps *and questions* rather than
unauthorized directives), and paraconsistent retention of real conflicts. For funders
concerned with accountable AI, this is a working alternative to hallucination-prone systems —
one where every answer carries provenance, sensitivity class, and contractual context in the
record itself.

### 2.4 Public-interest software should not need a datacenter

The full engine compiles to a browser WASM bundle. A community ranger station, a classroom,
or a citizen-science project can run the *same deterministic semantics* with no server, no
subscription, and no data leaving the device — with deliberate design choices like **no
hardware fingerprinting** baked into the observer model.

---

## 3. What it is, technically

- **NQuin** — every semantic datum is one fixed 48-byte record (subject / predicate / object /
  context / metadata / parity). Fixed layout means GPU-uploadable, SIMD-friendly, zero-copy,
  and hashable end-to-end.
- **Bounded reasoning** — a 42 MiB `SlgArena` sentinel bounds hot-path evaluation; a two-tier
  allocation contract (zero-heap kernels vs. budgeted cold construction) is enforced by
  measured `assert_zero_alloc` tests, not convention.
- **~45 logic families, composed not co-located** — deontic × temporal (LTL) × epistemic ×
  paraconsistent × description-logic × ASP × linear × defeasible, evaluated over the same
  NQuins, with verdicts mappable to runtime actions.
- **One ABI everywhere** — the same engine runs native, WASM, and edge; 87+ WASM exports are
  already live-callable from a browser page, and the WASM bridge spans bio, chemistry,
  geometry, governance, logic, medical, semantic, and numerics domains.
- **VibeScript** — a companion language/VM (`vibe` crate) with a **fail-closed capability
  sandbox**: ~1,086 invoke IDs across ~90 families (statistics 108, economics 107,
  computational geometry 94, machine learning 81, render 60…), each requiring an explicit
  `using {Family}` grant. Sandboxed agentic computation by construction, not by policy.
- **Formats** — `.q42` semantic volumes, `.10d` tensor/mesh containers (with a new
  hyperspectral-cube section type), `.p64` model weights (byte-exact GGUF/safetensors
  compiler), `.hmc` hypermedia bundles, `VBC1` bytecode.

---

## 4. What it does today — verified capability domains

Counts below come from the tree-sitter audit (54,077 functions; ~29% carry documentation).

| Domain | What exists | Evidence status |
|---|---|---|
| **Ecological entity model** | `EONT1` entity ontology (taxon, lifecycle stage, energy/damage/hydration/wear, ≤256 affordances); `DevelopmentalKeyframeRecord` ontogeny tracking; `EthogramRegistry` binding affordances ↔ behavioral HFSM ↔ motor clips; species/ecology SHACL validation pack; TK/BC (Traditional Knowledge / Biocultural) label enforcement | Implemented; ethogram/keyframes are real kernels, some consumers still unwired |
| **Accrual & living-world value** | Standing-stock accrual with time-depth, irreversibility classes | Implemented kernel (not yet exposed in any demo) |
| **Commons economics** | Obligation cost, pool accumulation, `is_commons_discharged` sunset, royalty trees, usury guards, EROI gating, `CommonsObligation` manifest persistence, QDNF meter→pool binding | Mechanisms implemented; **no production callers yet** — test-invoked |
| **Payments** | Real ILP-STREAM micropayments (payment-pointer resolution, Nym mixnet option, stablecoin fallbacks); **every payment written as an N-Quad receipt into `.q42`**; verify-before-pay distributed jobs; 12% commons-tithe tax router | Implemented paths |
| **Logic & reasoning** | ~45 families; SHACL→opcode compiler; N3 rules (strict/defeasible/defeater/linear); 21 families live-demoable in browser today | Implemented; ~21 with public demos |
| **Scientific compute** | 911 numerics + 913 computational-geometry functions: exact predicates, botanical kernels (L-systems, space colonization, tree skeletonization, canopy radiation), XPBD soft-body, arrangements, CSG, SVD/eigen/stats/CAS/transforms/units | Implemented; repo's own review finds the geometry stack *exceeds* PlantGL in exact predicates |
| **Physics & engineering** | 593 functions: nbody, molecular dynamics, CFD (Burgers + D2Q9 LBM), EMF, boids flocking, truss/frame FEM, Newmark-β dynamics, modal/FRF vibration, reliability | Implemented, mostly library-only |
| **Economics library** | ~33-file suite: environmental resource economics, spatial economics, game theory, mechanism/market design, welfare | Implemented, library-only |
| **Sensory** | Audio: 1,502-function crate (full MIR stack, MIDI 1/2, bounded production engine, AQA) + holosonic wavefield synthesis + HRTF + bioacoustics; vision: native CV (watershed, Voronoi-Otsu) → `VisionQuin` records; spectral: Jakob-Hanika reflectance basis, metamer, gamut | Implemented; **zero public demos** — see §6 |
| **Rendering** | wgpu 30 backend; spectral/PBR path; skeletal animation; portal pipeline; holographic 3D lane (new); `.10d` compiled-asset path | Implemented; partially WASM-exposed |
| **Networking** | QDNF peer fabric (323 files), `did:q42` topological pointers, sync/replication, Solid/LDP bridge crate | Implemented; docs-level pages only |
| **Inference** | 271-file inference stack; streaming Qwen runtime; P64-packaged models; WASM LLM demos | Implemented, demoed |
| **Identity & rights** | DIDs, verifiable credentials, Ed25519 author-scoped Merkle roots, delegated access with expiry, suspended-transaction consensus queues | Implemented |

---

## 5. What makes it different

| Compared to… | QualiaDB |
|---|---|
| Knowledge graphs / triplestores | Facts carry geometry, physics, rights, and obligations *in the same record*; reasoning is bounded and attested, not open-ended query answering |
| Game engines / GIS | Entities are semantic records with provenance, lifecycle, and affordances — not disposable scene objects; the "map" and the "ledger" are one system |
| Smart-contract platforms | Deterministic bounded evaluation (42 MiB sentinel, zero-heap kernels) instead of gas-metered global consensus; obligations discharge to commons instead of extracting rent forever |
| Document AI / LLM agents | Uncertainty, provenance, and refusal are encoded in the data model; the engine emits evidence, not just tokens |
| Natural-capital tools | Accrual is *computed* — formation-time asymmetry and irreversibility classes are evaluable semantics, not report prose |

---

## 6. Honest maturity & risk assessment

Sophisticated funders should weigh these — the project's own docs are candid about most:

- **Scale vs. wiring.** The codebase is enormous (54k functions) but much of the deepest
  machinery is *library-only* — real tested code with no production caller yet. Flagship
  examples: `discharge_to_commons` and meter→pool accumulation are test-invoked only.
- **Solo-author concentration.** Essentially one creator; governance/bus-factor and
  community adoption are open questions a funding program could directly address.
- **Demo gap.** The website demos the semantic/LLM story well but shows *nothing* of the
  living-world stack — accrual, botanical kernels, physics, audio, vision, econ-suite
  (documented in `qualiadb-website-update.md`, also generated 2026-10-17).
- **Documentation coverage.** ~29% of functions carry docs; large areas are
  code-self-describing only.
- **License.** CC BY-NC-ND 4.0 fits a commons/public-good mission but restricts commercial
  adoption and derivative works — funders expecting permissive open source should discuss
  licensing intent with the author.
- **Test claims.** Test counts in project docs are quoted from the repo's own review files,
  not independently re-run in this audit; the audit verified *existence and wiring*, not
  behavioral correctness.

---

## 7. Alignment with philanthropic program areas

| Program area | Relevant, verified machinery |
|---|---|
| Natural capital / conservation | Accrual stocks (biomass, habitat, hydrological function), ecology SHACL shapes, hyperspectral reflectance sections, geospatial/GeoSPARQL, environmental-resource economics |
| Indigenous data sovereignty | **TK/BC label validation** (Local Contexts Traditional Knowledge & Biocultural notices) enforced in governance code; deontic access lanes; author-scoped Merkle attestation |
| Digital commons governance | Obligation-cost model, commons-sunset discharge, usury ceilings, royalty trees, credential-bound permissions — obligations travel *inside* the asset format |
| Accountable / public-interest AI | Bounded deterministic reasoning, evidential uncertainty, provenance-in-record, fail-closed capability sandbox for agent compute |
| Open science infrastructure | 87+ live browser compute exports (stats, linalg, CAS, numerics, units, transforms, graph), deterministic physics/geometry kernels, units with CODATA constants |
| Edge / offline-first access | Full engine in browser WASM; no server required; no hardware fingerprinting by design |

---

## 8. How to verify (due-diligence checklist)

1. **Run the live demos** — `docs/` is a static site: `compute-engine.html` (87 WASM calls),
   `modalities-showcase.html` (21 logic families), `science-playground.html`,
   `vibe/playground.html`, `playground/` (SPARQL, SHACL, RDF→Q42 comparator, 3D anatomy).
2. **Check the machine registry** — `audit-docs/FUNCTIONS.md` (54,077 functions with
   `path:line`) is a grep-able existence index; `audit-docs/CAPABILITIES.md` maps domains.
3. **Read the divergence log** — `docs/manuals/standards/q42-implementation-divergence.md`:
   the project's own record of where its documents disagree with code.
4. **Read `AGENTS.md` §0–§2** — the immovable engineering contract (NQuin layout, two-tier
   allocation, opcode ranges) and the "do not re-implement" register.
5. **Spot-check a claim** — pick any function name in this brief and grep the registry for
   its `path:line`, then read the source. The audit made every claim in this document
   traceable that way.
6. **Independently re-run tests** — quoted test counts are the project's own; a funding
   diligence pass should execute `cargo test` on the checkout being evaluated.

---

## 9. What funding would plausibly unlock

Grounded in verified gaps, not speculation:

- **Invocation wiring** — production callers for mechanisms that are complete but dormant
  (commons discharge, meter→pool economics, accrual surfaces).
- **Public demonstration** — the living-world demos (flora entities, accrual, audio MIR,
  vision, econ sandbox) that make the engine legible to non-specialists.
- **SDK & consumer boundary** — the repo's own GA/SDK work packages identify a missing
  versioned release boundary for application developers.
- **Documentation lift** — from 29% documented functions toward a maintainable public API
  surface; the generated registry already provides the skeleton.
- **Governance & community** — contributor pathways, licensing review, stewardship structure
  appropriate to a commons-mission public good.

---

## 10. Documents in this briefing pack

- *This brief* — purpose, capabilities, risks, alignment.
- **`QualiaDB Verified Inventory.md`** — the data appendix: audit numbers, module/crate
  maps, demo inventory, gap register.
- **`QualiaDB Capability Reference.md`** — the 2,000-line technical audit for a funder's
  technical reviewer (same evidence levels).
- **`qualiadb-website-update.md`** (in the repo) — the public-demonstration gap analysis.

*Method note: the audit was generated by a tree-sitter extraction tool (`rust_audit`,
Rust/Python/WGSL) over the 2026-10-17 working tree; it verifies existence, location, and
wiring — not behavioral correctness. Claims about test results are the project's own.*
