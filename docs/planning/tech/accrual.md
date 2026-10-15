# Accrual — stocks, time-depth, asymmetry, and irreversibility

**Status:** draft v0.1 (concept, recorded 2026-10-12)
**Feeds:** [../domain-model.md](../domain-model.md),
[statistics.md](statistics.md), [interpretation-frameworks.md](interpretation-frameworks.md),
[reconciliation.md](reconciliation.md), [../sociosphere.md](../sociosphere.md)

---

## 1. The claim being modelled

The user put it precisely: *a tree may take 30 years to grow but only 2 nights
to burn — and this isn't simply "carbon" or "time" or "energy".*

The dimension being named is **accrual**: what a thing is as a consequence of
time — accumulated organisation across *multiple, non-commensurable
dimensions*. A mature tree is simultaneously a carbon stock, a water-cycle
participant, a habitat structure (hollows take ~120 years), a genetic-information
carrier, a phenological clock, a custodial-relation bearer, and a knowledge
referent. None of those dimensions reduce to each other, and none reduce to a
price. Destroying the tree destroys *all of them at once*; regrowing it restores
them on wildly different horizons — biomass in decades, hollows in a century,
genetic distinctness possibly never.

Resilience theory calls these **slow variables** (conservation→release
asymmetry is the adaptive cycle's core dynamic); natural-capital accounting
calls part of this **critical natural capital** (non-substitutable stock).
Neither name carries the whole thing — so the model holds it as a
multi-dimensional stock with *time-depth*, not as a scalar on a single axis.

## 2. AC-AX — accrual axiom family

| Axiom | Statement | Rejects |
|---|---|---|
| **AC-AX-1 Stocks and flows are distinct kinds** | A *stock* (a standing thing with accrual history — tree, aquifer, soil profile, language community, trust relationship) is a different entity class from a *flow* (harvest, discharge, transaction). Confusing them is a category error; a flow's ledger never substitutes for a stock's state. | Treating inventories as income; "growth" as stock health |
| **AC-AX-2 Time-depth is a first-class attribute** | Accrual age / maturity class / formation horizon are recorded properties. An old-growth individual ≠ a sapling of the same taxon — same species, different stock state. DOM-AX-4's scale ladder has a **temporal twin**. | Species-count equivalence; "a tree is a tree" |
| **AC-AX-3 Asymmetry is recorded, not implied** | Formation-time vs destruction-time is a declared, queryable ratio per stock class (30 yr growth / 2 nights fire; centuries of soil / one erosive season; generations of language / one generation of transmission-break). | Loss recorded as symmetric to gain |
| **AC-AX-4 Irreversibility is classified** | `reversible` / `slow-reversible` (within planning horizons) / `horizon-exceeding` (recovery longer than a human planning scale — old-growth, deep aquifer) / `absolute` (extinction, language death, erased custodial knowledge). The class is a property of the stock+context, not a slogan. | "Renewable vs non-renewable" binary |
| **AC-AX-5 Accrual is multi-dimensional and non-collapsible** | Each stock carries a *profile* of accrual dimensions — biomass, structure, genetic diversity, hydrological function, custodial relation, knowledge linkage. Aggregating them into one scalar (a "carbon value", a dollar figure) is a *derived claim* (STAT-AX-6 methodology; INT-AX-3 result claim), never the stock itself. | Carbon-credit reductionism; single-metric ecosystem value |
| **AC-AX-6 Loss is an event; accrual is a process** | Destruction is recorded as an event (time, cause, agent/instrument, extent, receipts); accrual is a longitudinal record. The graph can answer "what was lost in the 2 nights" *and* "what 30 years built". | Loss as a silent state-change |
| **AC-AX-7 Renewal rates vs extraction rates are a stated pair** | Where use is legitimate (DOM-AX-1 acts), the model carries the *rate pair*: extraction rate vs regeneration rate per stock — sustainability questions become comparisons of two recorded rates, not moral adjectives. | "Sustainable" as an unparameterised label |
| **AC-AX-8 Some accruals are non-substitutable** | Critical-stock designation is a declared property with evidence — offsetting a horizon-exceeding stock against a commensurable-looking claim elsewhere is a *contestable interpretation*, flagged, never an assumed wash. | Generic offset equivalence ("plant a sapling, clear an old-growth") |
| **AC-AX-9 The sociosphere accrues too** | The same pattern models knowledge traditions, languages, trust, standing, custodial relationship — slow-accrual, fast-destroyable, dimensioned. SOC-AX knowledge systems and AUSTLANG languages are stocks. | Social assets as instant/commodity |
| **AC-AX-10 Degraded ≠ absent** | A burnt forest is a *changed stock* (post-fire state with its own accrual trajectory — regeneration phases), not a deletion. DOM-AX-2 applies: the prior state remains queryable history; the new state is a new accrual track. | Binary present/destroyed state |
| **AC-AX-11 Spent lifetime is non-renewable** | An agent's invested years are *consumed* by the act — the spender's lifetime budget does not regenerate. A parent who raised a child for 15 years cannot recover that time; the investor-side asymmetry is distinct from the stock-side one (AC-AX-3). Applies to persons, communities, and ecological investors alike (a forest "spends" decades of succession). | Time as fungible budget; "lost productivity" framings |
| **AC-AX-12 Constitutive investment can't be detached** | Some accruals *are* the shared time — a child–parent bond, a language community, a custodial relationship. The investment is not separable from what it formed; it cannot be refunded, transferred, or re-performed by another agent. Such accruals may be *acknowledged* — never *compensated as equivalent*. | Compensation-as-restitution for lived time |
| **AC-AX-13 Windows are stock-critical events** | Formation and investment are window-bound — childhood years, succession phases, intergenerational transmission openings, fire-weather windows. A closed window is itself a recordable loss: the parent who loses their child at 45 has lost the child *and* the window — two losses, both first-class. | Only entities as losses; "try again later" assumptions |
| **AC-AX-14 Replacement is a new accrual, never restoration** | A replanted forest, a subsequent child, a rebuilt tradition begins a *new accrual track* — it does not continue or restore the prior one. Claims of restoration/replacement are recorded as new-accrual-start, and "offset" or "replant" equivalence is a contestable interpretation (AC-AX-8), not a fact. | Substitution accounting; "we'll just replant it" |

**Terminology note (user question, 2026-10-12):** *non-fungible* is the right
direction but undershoots — spent time fails interchangeability four ways:
**non-renewable** (the spender's budget never refills), **non-transferable**
(an accrual stays bound to who invested), **constitutive** (the time *is* the
thing formed), and **window-bound** (the act exists only within its temporal
window). For the stock side the right term is **non-substitutable** (AC-AX-8).
The model needs all five distinctions; "non-fungible" names only the first.

## 3. Data-model consequences

- `eco:Stock` (or per-domain `bio:`/`env:`/`soc:` subclasses) carries:
  `accrualProfile` (dimension map), `accrualSince` / maturity class,
  `formationHorizon`, `destructionTimescale`, `irreversibilityClass`,
  `renewalRate`/`extractionRate` where applicable.
- `eco:LossEvent` — event entity: interval, cause (fire/flood/clearing/
  policy/extraction), extent geometry, affected stocks, receipts, and the
  *link to reconciliation* (reconciliation.md).
- Derived valuations ("this forest is worth $X carbon") route through the
  interpretation layer as instrument outputs — the stock graph keeps the
  dimensions; instruments produce the scalars.
- Seasonal/phenological calendars (SOC-AX-6) are accrual clocks — the seasonal
  instruments in interpretation-frameworks §7 read stocks through them.

## 3A. The investor side — time as spent life

AC-AX-1–10 describe the stock. AC-AX-11–14 describe the **spender**. The
pairing matters for both domains:

- **Sociosphere (SOC-AX):** a person's spent lifetime is the paradigmatic
  non-renewable accrual *input*. Raising a child, holding custodianship,
  carrying a language — these are window-bound, constitutive investments.
  The graph should be able to say honestly: this obligation can be *discharged
  or acknowledged*, but the spent years are not a recoverable quantity.
- **Biosphere:** a stand of trees, a soil profile, a reef — the "investor" is
  the place itself and the living community; its spent decades don't come
  back either.
- **Made world:** even artefacts carry constitutive time — a craft tradition's
  embodied skill, a codebase's decades of accrued correctness.

**Compensation consequence:** fair compensation (COMP-AX) discharges the
*obligation* for work; it never claims to restore the *time*. Where a loss is
of the constitutive/irreversible class, the honest ledger entry is
**unpayable-in-kind** — obligation recorded against dimension lost
(accrual §5-5), not a dollar equivalence.

## 4. Worked example — the 30-year tree

```
bio:Individual/Stand #t442
  eco:accrualProfile  { biomass: 8.2t, hollows: 3 (est. 120yr),
                        geneticLineage: regional-provenance A,
                        custodialRelation: (nation, story-anchor),
                        phenologyClock: flowering-ref for 4 spp }
  eco:accrualSince    ~1996
  eco:formationHorizon 30–120yr (dimension-dependent)
  eco:irreversibilityClass { biomass: slow-reversible;
                             hollows: horizon-exceeding;
                             lineage: absolute }
  ── LossEvent: bushfire, 2026-10-09..10 (2 nights) ──
  post-state: changed stock, new accrual track begins;
  prior state retained (DOM-AX-2); interpretations citing
  the stand get re-check flags (reconciliation §4)
```

## 5. Open questions

1. Dimension registry — is the `accrualProfile` dimension vocabulary open
   (any claimant may add a dimension) or governed (fixed families)? Lean:
   open-with-registration, since custodial dimensions are exactly the ones a
   closed list would miss.
2. Quantification honesty — many dimensions have no defensible number
   (custodial relation). Representation must allow *presence + qualitative
   state* without fabricating magnitudes.
3. Whether `Stock` is an entity class or an aspect — likely an aspect
   (role-of-entity-in-time), same pattern as borders being entities but
   "being bordered" being a relation.
4. Extraction-rate bookkeeping — who/what records the rate pair for
   commons-lane resources (which instrument reports it; cadence).
5. Interaction with compensation — §3A resolves the direction: obligation
   attaches to the *dimension lost*, and constitutive/irreversible losses are
   recorded **unpayable-in-kind** (acknowledged, not equated). Open: how the
   unpayable class renders in obligation ledgers (a balance that can't be
   settled but stays on record?).
6. Interaction with probability fields — windows are *reachable regions* in a
   field over futures and spent lifetime deforms that field (PROB-AX-7/8 in
   [probability-geometry.md](probability-geometry.md)). A `LossEvent` is thus
   also a re-weighting event on the affected stocks' future fields; whether
   field-updates propagate automatically or via instrument re-execution is
   open (ties to REC-AX-5 propagation).
