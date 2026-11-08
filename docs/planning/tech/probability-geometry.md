# Probability as ontological geometry — fields, epistemic axes, trajectories

**Status:** draft v0.1 · extends [statistics.md](statistics.md),
[accrual.md](accrual.md), [interpretation-frameworks.md](interpretation-frameworks.md),
[reconciliation.md](reconciliation.md); grounds on
[tensor-10d.md](tensor-10d.md) (Q42 axis semantics).

A probability is never a number attached to a thing. It is a **field**: a
measure over a typed space of possible states/futures, produced by a declared
method, under declared assumptions, at a declared vintage. "Likely lifespan of
a person" is not `p=0.82`; it is a field over the *future-state space* of a
person-position, conditioned on a factor space (sex, cohort, place, exposure
history, custodial context…), computed under a declared assumption set, from
declared evidence. This document makes the field first-class — and makes its
*geometry* ontological rather than Euclidean: positions are typed regions of
concept/state space, distances are `v`-class metrics, and the epistemic
context is part of the axes, not a comment.

The user asked for "a more complex structure for probability… towards a form
of ontological geometry." That is the claim here: **probability lives as a
measure on the typed manifold, and assumptions/logic/provenance are axes of
the field itself.** The manifold is not metaphorical: **the ontological
geometry is intended to be carried by the Q42 Tensor10D structure** (§5) —
fields are first-class inhabitants of the tensor, alongside accrual stocks
and trajectories ([accrual.md](accrual.md)), not a parallel format. Nor is
the manifold one smooth space: it is a **fabric of many interconnected
geometric structures with typed friction between them** (§5A) — and the
fabric's evaluation is bidirectional with logic: axiom sets constrain which
fields are admissible, and field projections feed logical premises
(PROB-AX-11).

---

## 1. Why a scalar, and even a graph, undershoot

| Shape | What it captures | What it can't say |
|---|---|---|
| Scalar `p` | One marginal claim | Conditions, provenance, which model, what it assumed |
| Probability graph / Bayesian network | Conditional dependence over named variables | Who computed it, under which assumptions, at which vintage; rival structures; non-variable dimensions (relations, phases, windows); path-conditioned futures |
| **Field over ontological geometry** | Measure + typed state space + epistemic axes + trajectory structure | — target |

The Bayesian-network case deserves honesty: PGMs already model "probability
+ dependency structure". What they lack is what this corpus needs — the
*epistemic context* (which instrument, which assumptions, which axiom
reading, which evidence vintage) is outside the graph rather than inside it;
the variables are untyped atoms rather than regions of an ontological space;
and the structure has no native notion of *directed lived time* (windows,
path-dependence, spent lifetime). The field model keeps the dependency
structure and adds the rest.

## 2. Field = measure over a typed space

A `ProbabilityField` is defined over:

- **State space** — the domain of possible positions/trajectories, typed by
  the ontology: regions of concept space, not bare coordinates. The space
  has topology (`v`-class: Euclidean regions, cyclic rhythms, hyperbolic
  taxonomies, clique-bounded custodial zones) — distance is the `v` metric,
  never defaulted Euclidean.
- **Factor space** — the conditioning dimensions (age, cohort, geography,
  exposure, participation…). Factors are *typed entities with provenance*,
  not anonymous variables — a "smoking status" axis points at a governed
  classification, not a string.
- **Epistemic axes** — the geometry's distinctive axes: assumption set,
  axiom-package context (INT-AX-6), instrument release, evidence lineage,
  scenario/horizon, and epistemic-vs-aleatory separation. Two fields that
  differ only in epistemic axes are *different fields* — comparable only via
  a declared transport (PROB-AX-6), never silently merged.
- **Measure** — the weight structure over the space: densities, masses,
  bounds/intervals where a point probability is dishonest, and `reCheckable`
  style flags where the measure is provisional.
- **Vintage** — the field itself is event-context data (STAT-AX-1): which
  observation volumes and model release produced it, when, superseding what.

## 3. PROB-AX — probability-geometry axiom family

| # | Axiom | Consequence |
|---|---|---|
| **PROB-AX-1 Fields, not scalars** | A probabilistic claim is a `ProbabilityField` (or declared projection of one). A bare `p=…` in the graph is either a documented marginalisation or a mark of incomplete data. | A lifespan claim carries its conditioning, method, vintage, and epistemic axes — or is visibly lossy |
| **PROB-AX-2 Epistemic axes are data** | Assumptions, axiom context, instrument release, and scenario are indexed axes of the field — queryable and comparable, not annotation. | "What did this assume?" is a projection, not archaeology |
| **PROB-AX-3 Aleatory ≠ epistemic, never collapsed** | World-uncertainty (die roll, fire weather) and model-uncertainty (which structure, which parameters) live on separable axes. Collapsing them is a declared interpretation. | A confident-but-wrong model can't masquerade as a lucky world |
| **PROB-AX-4 Fields are claims with receipts** | A field is produced by an `si:` execution (or marked unmediated — INT-AX-6/9): `ResultClaim` + receipt + vintage. Plural rival fields coexist (INT-AX-2); none is "the" probability. | Two instruments over the same cohort yield two recorded fields |
| **PROB-AX-5 Cohort fields never apply to persons** | A population-level field (STAT-AX-2) conditions on *declared factors only*; applying a cohort field to a person without a person-level conditioning receipt is the ecological fallacy by another name — INT-AX-5 prohibited. | "P(death|…)" for a cohort is never a verdict on an individual |
| **PROB-AX-6 Cross-field comparison needs declared transport** | Fields over different factor spaces, axiom contexts, or vintages are compared/merged only through an explicit transport/metric claim (choice of divergence measure is itself a claim). | No silent averaging of rival models into a "consensus" scalar |
| **PROB-AX-7 Directed time: futures are path-conditioned** | The field over future trajectories is a function of the *realized path so far* — passing an event/window re-weights the field, it doesn't just move a point. Windows (AC-AX-13) are reachable regions; a closed window is a region that has left the field's support. | Losing a window isn't a probability going to zero — it's the geometry folding |
| **PROB-AX-8 Spent lifetime deforms the field** | An agent's invested years (AC-AX-11) are conditions on the realized path — non-removable. A field must not pretend a fresh 15-year investment is available when it was spent. | The 45-year-old's field is different in shape, not just in mean |
| **PROB-AX-9 Projections are lossy and recorded** | Marginals, summaries, "headline risk" figures are projections of a field — each records which axes were collapsed and which measure was used. | A dashboard scalar can be traced back to the field it flattened |
| **PROB-AX-10 Sensitivity travels with the field** | Fields over sensitive populations/knowledge inherit source sensitivity and CARE labels per axis — a public field may publish marginals while the full field stays gated. | Custodial axes can restrict projections without deleting the field |
| **PROB-AX-11 Probability and logic evaluate each other** | Axiom/logic sets constrain which fields are *admissible* (structure legality, prohibited applications — PROB-AX-5 is a logical constraint); field projections feed rule premises, and probabilistic weights ride logic — upstream precedent: `n3_parser.rs` weighted defeasible rules `(0.8) { premise } ~> { conclusion }` and `cml:LogicApplication` per-domain evaluator routing. Evaluation of a probability-assertion is an `si:` event like any other (PROB-AX-4). | Neither "probability decorates logic" nor "logic approves probability" alone — the relation is bidirectional and receipted |
| **PROB-AX-12 The fabric has typed friction** | Transport of a claim/field/measurement between geometric structures carries a *declared friction*: loss, cost, gating, or impermeability — often asymmetric, always recorded. Infinite friction is a legitimate value: it is how the geometry says *non-reducible* (AC-AX-5) or *gated* (custodial cliques). | Interoperability is not free; "seamless integration" claims without a declared transport are suspect |
| **PROB-AX-13 Physics bounds support before probability** | Physical/STEM constraints — conservation laws, energy/material budgets, thermodynamics, biological and temporal limits — define the field's *support*. A structure assigning mass to physically unreachable states is malformed, not merely improbable. STEM models enter as `si:` instruments; their constraint outputs are admissibility, evaluated before measure is meaningful. | "Anything is possible given enough X"; probability over fantasy states |
| **PROB-AX-14 Choice is a field over actions** | An agent's choice space is a typed field: each action-path carries a *cost profile* (which stocks deplete, whose) and a *benefit profile* (beneficiary, accrual dimension, timing, permanence, reversibility, funded-from). Benefits are structures — never bare utility scalars; a benefit with no beneficiary, or a cost with no bearer, is malformed. "Reasonable" choice = the admissible region after physical, economic, rights/gate, and capacity constraints — which is geometrically inspectable. | Scalar utility; choices modelled as free of constraint; "no alternative" claims without checking the admissible region |

## 4. Data-model shape

```text
:fieldX a prob:ProbabilityField ;
    prob:stateSpace     :person-future-space ;      # typed region set
    prob:factorSpace    :lifespan-factors-v3 ;      # typed conditioning axes
    prob:epistemicAxes  [ prob:assumptionSet :asmp-base2026 ;
                          prob:axiomContext  :soc-ax-1.2 ;
                          prob:instrument    :who-lifetable-r2024 ;
                          prob:epistemicKind prob:ModelUncertainty ] ;
    prob:measure        :fieldX-measure ;           # density/mass/interval structure
    prob:vintageOf      :wpp-2024-volume ;          # STAT-AX-4 lineage
    si:producedByReceipt :exec-receipt-… .
```

- **Trajectory sets**: a field over futures is stored as a measure over
  *paths* (or a bounded sample thereof) — a person's plausible-life bundle —
  not only per-time marginals.
- **Hazard/survival surfaces**: lifespan work uses survival/hazard fields —
  measure over death-time × path — the actuarial-life-table shape lifted
  onto the typed geometry.
- **Weights**: the user's "various weights relating to assumptions, logic"
  are the measure's structure *plus* the epistemic axes — both carry weight:
  numeric mass on the state space, and provenance/confidence on the
  epistemic axes (`α`/amplitude lane, `μ` provenance carrier in Q42 terms).

## 5. Q42 substrate — the intended carrier

The ontological geometry **is** the Q42 Tensor10D manifold
([tensor-10d.md](tensor-10d.md)) — by design intent, not just compatibility:
the relationship runs both ways, and these requirements are part of the
format's original rationale (tensor-10d §6).
A `ProbabilityField` is a tensor-resident structure; queries over it are
the tensor's native operations (projections, `v`-switched distances, temporal
slices, context collapses) rather than a separate probability engine. The
geometry is shared: accrual stocks, trajectories, windows, and ontologies
occupy the same manifold — a lifespan field and the custodial taxonomy it
conditions on are *in the same space*.

| Axis | Role for fields | Status (per tensor-10d §7) |
|---|---|---|
| `q` | Competing field structures / unresolved hypothesis branches live at `q>0`; collapse promotes the resolved field, `t+1` logs when it became knowable | spec semantics; superposition machinery design-level |
| `v` | Metric class per factor/state region — hyperbolic for taxonomic conditioning, cyclic for seasonal/physiological rhythms, clique-bounded for custodial domains | metric classes implemented; descriptor gate exists |
| `w` | Domain partition — a lifespan field spans bio (`w=0`), environmental (`w=3`), socioeconomic (`w=4`) without leaving the tensor; cross-`w` correlation is a projection, not a join | selector verified; domain enumeration is pack convention |
| `x,y,z` | Semantic position of state/factor regions; relatedness = `v`-adjusted distance | verified coordinate |
| `t` | Field vintage + "what was knowable at X" natively | verified coordinate |
| `α` | Measure weight / amplitude per position — the field's mass | verified coordinate |
| `μ` | Epistemic-axis binding travels *with* the values — per-slice provenance/DID/assumption carrier | verified CoordinateCarrier |
| `σ` | Logical/spectral class index — typed region membership | verified coordinate |

`Epistemic10D` (the second-ten host-internal geometry — `epistemic_weight`,
`temporal_decay`, `manifold_curvature`…) is the intended home for host-side
field dynamics such as decay and curvature-aware transport; T14 discipline
applies — never conflate the two.

**Staging honesty:** the *carrier* axes (`v,w,x,y,z,t,α,μ,σ`) are
verified in code; the *epistemic* behaviours (q-superposition collapse
semantics, GSR escrow, gravito-thermo baking) are design-level per
tensor-10d §7. Field storage/lookup can therefore target verified axes now;
q-lane field-plurality and cascade semantics are specified here but land
when the superposition machinery does (tracked under G48).

## 5A. The manifest — many geometries, typed friction

The construct is not a single smooth manifold: it is a **fabric** (manifest)
of many ontological-geometric structures — volumes, domains, taxonomies,
fields, custodial zones — each locally coherent, interconnected at
boundaries. What the system needs is an honest model of what crossing those
boundaries *costs*.

**Friction** is the name for that cost: the resistance a claim, field,
measurement, or act meets when transported between structures. It is
first-class data, not noise:

| Friction kind | Example | Typical value |
|---|---|---|
| **Translational** | ANZSIC→ISIC concordance; species-name alignment | Lossy but passable; loss recorded per mapping |
| **Epistemic** | Same instrument under different axiom packages (INT-AX-6) | Depends on declared transport (PROB-AX-6) |
| **Temporal** | Field computed on WPP-2022 vintage queried under WPP-2024 | Vintage-gap declared; staleness propagates |
| **Sovereign/gated** | Custodial clique (`v=3+`), CARE-gated field axes | Gated — may be impermeable by policy |
| **Dimensional** | Multi-dimensional stock → scalar projection (AC-AX-5) | **Infinite** — impermeable by nature, not policy |
| **Domain** | `w=3` environmental ↔ `w=4` socioeconomic correlation | Projection-declared |

Properties of the friction model:

- **Asymmetric by default** — population statistic → person is impermeable
  (STAT-AX-2); person-level conditioning → cohort aggregate is passable.
  Direction is part of the declaration.
- **Infinite friction is honest data** — "not reducible", "not transferable",
  "gated" are geometric facts about the fabric, not failures.
- **Friction is a claim** — a transport declares its cost/loss with the same
  provenance discipline as any result; rival transport declarations coexist.
- **Boundaries are first-class** — friction lives *at* interfaces; border
  records (DOM-AX multiple-border kinds) are friction carriers between legal,
  custodial, and ecological geometries — the same concept already required
  for map data.
- **Zero-friction claims are flagged** — a transport declared lossless is an
  assertion to be verified (reconciliation §4 applies), not assumed.

Worked example — a lifespan field conditioned on "residence in a custodial
community" crosses: (a) domain friction `w=0`→`w=4` (declared projection),
(b) classification friction on the community's governed taxonomy
(translational, loss recorded), (c) sovereign friction on the custodial
axis (gated — the field publishes only permitted marginals). Three frictions,
three different mechanisms, one honest answer: the field's published form
*is* the residue after friction.

## 5B. Constraint layers and the choice field

Viability and agency decompose into **ordered layers**, each carving the
space before the next is meaningful:

1. **Physical layer (STEM)** — energy/material budgets, conservation,
   thermodynamic limits, biological and temporal limits. Instruments that
   evaluate these return *admissibility* (a region), not a probability
   (PROB-AX-13). The project's functional needs must intersect this layer
   first: a requirement the physics can't satisfy never reaches economics.
2. **Economic layer** — budgets, compensation floors, PPP normalisation,
   infrastructure costs (compensation.md, statistics.md). Carves the
   physically-admissible region down to the *affordable* region.
3. **Rights/governance layer** — deontic gates, custodial cliques (`v=3+`),
   consent structures, statutory constraints. Carves to the *permitted*
   region. Upstream deontic modality (`OP_OBLIGATE/PERMIT/FORBID`) is the
   existing machinery this binds to.
4. **Capacity layer** — the agents' own stocks: health, skills, tools,
   *remaining time*, open windows (AC-AX-11/13). Carves to the *actually
   available* region — this is where underpaid/self-funded projects lose
   mass as workforce stocks deplete.

**The choice field** is the measure over what remains: the agent's
reasonable actions live in the post-constraint region, each action-path
carrying typed cost and benefit profiles (PROB-AX-14). This is what makes
the litigation question in the worked case (compensation §5A) geometrically
concrete: *"could a reasonable person have chosen otherwise?"* is the query
**does the admissible region contain alternatives?** — evaluated under the
constraint stack, with the evaluation itself receipted.

Two honest consequences:

- **Constraint order matters and is declared** — economics before physics
  produces different (dishonest) spaces; the layer order is part of the
  instrument's declared method.
- **Service narrows the field** — military service, indentured or
  coerced labour are cases where the rights/capacity layers contract the
  agent's admissible region; the *narrowing itself* is recorded structure,
  which is how obligation survives the act.

## 6. Anchor literatures (nearest known concepts)

- **Survival analysis / life tables** — the lifespan example's home field;
  hazard as a function over time × covariates.
- **Probabilistic graphical models** — dependency structure; this doc adds
  epistemic axes, typing, and path-space on top.
- **Measure-theoretic probability** — the honest substrate; fields are
  measures, marginals are pushforwards.
- **Information geometry (Amari)** — statistical manifolds as literal
  geometry; the closest existing "probability has a geometry" formalism.
- **PR-OWL / probabilistic ontologies** — prior art for typed probabilistic
  entities; heavier and entity-first, ours is field-first.
- **Path-space measure / stochastic processes** — trajectories as the
  measured objects (PROB-AX-7).

## 7. Interactions

- **statistics.md** — observations are the empirical basis; STAT claims of
  probabilistic kind are fields or recorded projections, not scalars.
- **interpretation-frameworks.md** — fields are `si:ResultClaim` outputs;
  rival instrument releases produce rival fields; axiom context rides the
  epistemic axes (INT-AX-6).
- **accrual.md** — windows are reachable regions (PROB-AX-7); spent lifetime
  deforms the field (PROB-AX-8); horizon-exceeding stocks correspond to
  futures with mass on horizons beyond the modelled range — honest "beyond
  field" tails.
- **reconciliation.md** — fields reconcile too: an event (FIRMS ignition)
  re-weights a locality's future field; field-vs-field divergence is a
  `DivergenceRecord` under a declared transport (PROB-AX-6).
- **domain-model / borders** — border records are the existing friction
  carriers; §5A generalises them from place-boundaries to structure-
  boundaries. Custodial `v=3+` cliques are gated friction by construction.
- **logic modalities** — upstream `n3_parser.rs` weighted defeasible rules and
  `cml:LogicApplication` routing are the evaluation hooks PROB-AX-11 binds
  to: axioms adjudicate field admissibility; weighted rules carry field
  mass into logical consequence.

## 8. Open questions

1. **Storage form** — sampled path-sets vs parametric families vs Q42-native
   tensor slices; probably all three behind one `ProbabilityField` interface.
2. **Transport/metric vocabulary** — which divergences are first-class
   (Wasserstein-style geometry-aware, f-divergences, or "declared per use")?
3. **Epistemic-axis merge rules** — can two fields' assumption sets be
   diffed automatically, or is compatibility always human-declared?
4. **Hazard vs density canonical form** — survival fields want hazard
   parameterisation; general state spaces want densities — same interface?
5. **Where the field computation lives** — WASM kernel per instrument, or a
   shared host-side field engine instruments call into (G44 dispatch).
6. **Friction vocabulary** — is friction a number, a typed cost vector, or a
   relation family? Lean: typed relation with declared measure per kind,
   since translational loss and sovereign gating don't share units — and
   "impermeable" must be a first-class value, not `∞` overflow.
