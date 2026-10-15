# Fair compensation — work type × place × agent kind

**Status:** draft v0.1 (concept, recorded 2026-10-12)
**Feeds:** [cooperative-layer.md](cooperative-layer.md) §2/§3,
[contribution-chains.md](contribution-chains.md) §4,
[statistics.md](statistics.md), [../sociosphere.md](../sociosphere.md) SOC-AX-14
**Source register:** [../data-sources/socioeconomic.md](../data-sources/socioeconomic.md) §3A/§3B

---

## 1. The claim being modelled

Contributors accrue obligation costs (cooperative-layer §3); "fair" is the
normative question of *what that obligation should be* for a unit of work. The
model required by the project direction has three modulation axes:

1. **Work type** — the contribution-kind register (contribution-chains §1) gives
   labour classes: field collection, identification, custodial knowledge,
   laboratory analysis, curation, translation, stewardship/hosting, review.
   Different classes carry different benchmark anchors.
2. **Place** — the same work deserves equivalent *real* compensation across
   places, not equal nominal amounts: purchasing-power parity adjusts the unit
   (ICP PPP factors), living-wage benchmarks set the human floor, and national
   minima supply statutory floors.
3. **Agent kind** — SOC-AX-14 categories modulate *who* the obligation serves:
   natural persons earn labour compensation; legal persons channel to their
   human constituents; artificial agents accrue to their accountable principal —
   *no anonymous-agent discount* (cooperative-layer §2 verified rule).

The engine already provides the substrate: `royalty(base, agent_multiplier)`,
`commons_cost` capped-ROI discharge-to-free, `Contribution` author-chains,
labour quins, `pc:Micropayment` rails. What this document adds is the
**benchmark-anchored rate layer** — the data-driven answer to "what is fair".

## 2. COMP-AX — compensation axiom family

| Axiom | Statement | Rejects |
|---|---|---|
| **COMP-AX-1 Fairness is a computed norm, not a market price** | The payable obligation for work = `base_rate(work_class) × place_factor(place, time) × agent_profile(kind)` bounded by floors/ceilings; it is *derived from published benchmarks*, never set by the platform's say-so or by what a buyer can extract. | "What the market bears" as fairness; platform-declared rates |
| **COMP-AX-2 Real-equivalence across place** | PPP conversion (ICP `PA.NUS.PPP` family, OECD/Eurostat programmes) normalises to purchasing-power terms; place resolves to the contributor's *declared region*, not their IP/VPN guesswork — the recorded place is evidence, verified where required. | Nominal equality ("same $ for everyone" or "everyone gets local minimum") |
| **COMP-AX-3 Living-wage floor for human labour** | Where a benchmark exists (Anker/GLWC, national living-wage campaigns, statutory minima — FWC awards in AU), human labour compensation never prices below it; below-floor work is *gifted labour* (COMP-AX-6), not discounted labour. | Poverty-wage pricing; minimum-wage as ceiling |
| **COMP-AX-4 Work-class parameters are declared, reviewable vocabulary** | Skill/hazard/custodial weights live in the labour-class vocabulary (`stat:`/`eco:` terms), versioned and ratified like any other vocabulary — never hidden constants. | Opaque rate tables |
| **COMP-AX-5 Agent-kind resolves to humans** | Natural person → direct labour obligation. Legal person → obligation resolves to constituent human labour + legitimate cost recovery (the entity itself cannot "earn" — SOC-AX-14). Artificial agent → accrues to accountable principal; *resource costs* (compute/energy/storage) are reimbursement entries, never labour. | Corporations earning wages; bots claiming labour pay; anonymous-agent rates |
| **COMP-AX-6 Gifting is legitimate; discounting is not** | Contributors may freely gift work (waive obligation) — consent-structured (`spc:` consent + asymmetric-power guards; a waiver under power imbalance is invalid). The floor protects against *exploitation*, not against generosity. | Waivers extracted by platform design |
| **COMP-AX-7 Benchmarks enter through the statistics layer** | PPP factors, wage benchmarks, minima are *statistical observations* — vintage, methodology, source-provenance per STAT-AX; never hardcoded. A compensation computation cites `{benchmark, version, vintage}`. | Constants in config; stale rate tables |
| **COMP-AX-8 Fixed-cost rule retained** | Biosphere/humanitarian-class work keeps `commons_cost` (production × (1+capped ROI)) → discharge → globally free; fair-compensation governs the *obligation during accrual*, not perpetual rent. | Perpetual per-use rent on humanitarian corpora |
| **COMP-AX-9 Obligation is a claim, not an invoice** | The computation produces a *payable-obligation claim* against a use/act — discharged through `pc:Micropayment`/ledger receipt (G11), negotiable by relation terms (`spc:` Employment/cooperative use-cases), auditable end-to-end. | Silent non-payment; unverifiable "we pay fairly" |
| **COMP-AX-10 Rates are contested-able** | The benchmark set, the work-class weights, and the computed result are all contestable records (CONTEST machinery) — a contributor can contest *the classification of their work* as first-class dispute. | Rates as take-it-or-leave-it |
| **COMP-AX-11 The ledger counts stock drawdown, not just cash** | Work priced below the viability floor doesn't become cheap — it becomes *subsidised by worker stock* (health, savings, tools, spent years — AC-AX-11/12). That drawdown is recorded as a cost *of the project*, attributed to the project relation. "Self-funded"/underpaid work is a `LossEvent`-bearing subsidy, not a discount. | Books that balance because the workers' bodies absorbed the difference |
| **COMP-AX-12 The delivery structure is evidence** | The geometry of how a project was staffed and compensated — rates vs floors, consent quality of waivers, risk/insurance placement — is a preserved, discoverable record. A later ownership/profit claim does not erase the obligation records the structure generated; contestability attaches to the *structure*, not just individual payslips. | "I own the outputs" retroactively launders "I didn't pay the inputs" |

## 3. The computation shape

```
obligation(work_record) =
    base_rate[labour_class]                       ← work-type axis
  × place_factor[region, vintage]                 ← PPP / living-wage / minima
  × agent_profile[agent_kind]                     ← SOC-AX-14 modulation
  subject to: living_wage_floor[region] (humans)
              statutory_floor[jurisdiction]       (where applicable)
              commons_cost_cap                    (biosphere/humanitarian class)
```

Every term is a graph reference: `labour_class` is a vocabulary term, `region`
is an edition-scoped geography (DOM-AX-3/ASGS or admin), the factor is a
benchmark observation with vintage. A computed obligation is a *derived claim*
with full `prov:` — recomputable, auditable, contestable.

## 4. Data dependencies (register refs)

| Need | Sources (all in socioeconomic.md) | Status |
|---|---|---|
| PPP factors | World Bank ICP (`PA.NUS.PPP` + household-consumption PPP), OECD/Eurostat PPP programmes | verified open (ICP caveat: PPPs are *estimates* — STAT-AX-9) |
| Living-wage benchmarks | Global Living Wage Coalition / Anker (methodology + benchmarks freely published; Reference Values are regression-modelled → STAT-AX-9); Living Wage Foundation (UK); MIT LWC (US) | registered; per-benchmark licence verify at acquisition |
| Statutory minima | ILO minimum-wage statistics; Fair Work Commission awards (AU); national equivalents | open/statutory sources |
| Earnings context | ILOSTAT earnings module; ABS AWE/Census income | verified SDMX/API |
| Demographic context | UN WPP 2024 (CC BY 3.0 IGO, open API — verified); ABS ERP/Census; Wittgenstein/IIASA | registered in socioeconomic §3A |

## 5. Interaction notes

- **With cooperative-layer:** this doc supplies the `base` and `agent_multiplier`
  *values* the verified `royalty()` machinery consumes — it doesn't replace it.
- **With contribution-chains §4:** labour quins record the work *done*;
  compensation claims are the derived *due* — same records, two projections.
- **With STAT-AX:** benchmark vintage matters — a 2021 PPP factor applied to
  2026 work is a declared approximation (vintage gap is recorded, not hidden).
- **With demographics:** population structure (age, participation) contextualises
  labour-supply and *who* contributes — context for claims, never a parameter
  pricing a person (STAT-AX-2 guard applies).
- **With accrual:** compensation discharges the *obligation* for work; it never
  claims to restore *spent lifetime* (AC-AX-11/12). Losses of constitutive or
  irreversible class — invested years, horizon-exceeding stocks — are recorded
  **unpayable-in-kind**, not equated to a rate.

## 5A. Worked case — the exploitative project

A project owner holds a **demand-side structure**: budget, requirements,
functional needs, circumstances — one geometric shape. The resources sought
(workers) each hold **supply-side structures**: subsistence floors (food,
shelter, tool costs), capacity, risk/insurance factors, spent-lifetime
budgets, windows. Viability is a *compatibility question between geometries*
— friction analysis ([probability-geometry.md](probability-geometry.md) §5A),
not a spreadsheet line:

1. **Below-floor demand is an impermeable ask.** "I want a lot of work, paid
   below what it costs workers to feed/shelter/equip themselves" doesn't
   describe a cheaper price — it describes a transfer the workers' stocks
   must absorb (COMP-AX-11). The project books show low cost; the workers'
   ledgers show drawdown: health, savings, years. The viability analysis is
   only honest if the drawdown appears *in the project's cost structure*.
2. **Workforce variance makes viability a field.** Members differ in cost,
   capacity, quality, insurance/risk factors — "can this be done, and done
   fairly?" is a `ProbabilityField` over outcome space
   `{feasible-fair, feasible-depleting, infeasible, contested}`, conditioned
   on workforce composition — not a yes/no. Risk factors are conditioning
   axes with provenance, per PROB-AX-2. Constraint layers
   (physics → economics → rights → capacity, probability-geometry §5B)
   carve the field's support before its measure: physical infeasibility
   is malformed, economic nonviability is carved, capacity depletion
   shrinks what's left — "no alternative but to underpay" is a claim
   about the admissible region and is inspectable as such.
3. **Purpose changes the axiom context, not the physics.** A human-rights /
   community project and a for-profit project evaluate under different axiom
   packages (COMP-AX-8 benevolent lane; COMP-AX-6 gifted labour) — gifting is
   legitimate *when consent-structured*. But neither context erases recorded
   drawdown, and a waiver extracted under power imbalance is invalid in both.
   Diminished capacity to continue ("workers burn out and can't go on") is a
   stock loss with downstream costs — the broader costs land on the workers'
   stocks and the commons, not the project budget.
4. **The strategy's geometry is litigable evidence.** An internal claim of
   "full ownership of profits" coexists with the preserved record of the
   structure that produced them (COMP-AX-12). Litigation by state or other
   parties is an *external reconciliation event* — the owner's internal view
   meeting an external normative reading (reconciliation.md REC-AX-2/6).
   The litigation outcome itself lives in the viability field as a
   contested-outcome axis.
5. **Military-service analogy.** Service expends exactly this class of
   accrual — non-renewable years, health, windows, risk exposure. Society's
   responses (veteran care, honours) are *acknowledgment*, not equivalence —
   the unpayable-in-kind entry (accrual §3A). The ledger shape is identical;
   the model is designed to say so.

## 6. Open questions

1. Labour-class vocabulary — initial classes from contribution-kinds;
   which anchor each to a benchmark family (occupational wage series vs
   living-wage vs professional rates)? Needs a first taxonomy pass.
2. `base_rate` governance — who ratifies the rate table (community review lane?
   cooperative membership?) and at what cadence — the table is a *governance
   object*, probably pack-versioned with the labour vocab.
3. Place granularity — PPP is national-level in ICP; subnational adjustment
   (regional living wage, remote-AU cost loading) needs a declared extension
   rule; don't fabricate precision the source lacks.
4. Legal-person channel — how constituent labour inside a contracting entity is
   evidenced (payroll equivalence? declared split?) — likely a
   `ThirdPartyAnchoredObligation`-style requirement on the entity.
5. Exchange-rate volatility vs PPP lag — which rate applies at discharge time;
   probably the *vintage-aligned* factor with the timestamp recorded.
