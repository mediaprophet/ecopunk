# Reconciliation — internal view, external view, and the temporal process between

**Status:** draft v0.1 (concept, recorded 2026-10-12)
**Feeds:** [../domain-model.md](../domain-model.md) DOM-AX-2/3,
[statistics.md](statistics.md) (STAT-AX-4 vintages),
[accrual.md](accrual.md) (loss events), [pipeline.md](pipeline.md),
[interpretation-frameworks.md](interpretation-frameworks.md) (re-check flags)

---

## 1. The claim being modelled

The graph holds an **internal view** — what the corpus asserts: observations,
claims, packs, derived interpretations, each at its source vintage. The world
holds the **external view** — what upstream sources currently publish, and what
ground truth currently *is*. These two are never identical, and the distance
between them is itself data.

Reconciliation is the governed, temporal process that measures that distance
and acts on it. It is not "sync" — sync overwrites. Reconciliation **compares,
records, and routes**: converged states, recorded divergences, deferrals, and
contests are all legitimate outcomes; a forced agreement is not.

The temporal dynamics matter in both directions and on multiple timescales:

- **Source cadence** — an upstream release changes (a new ASGS edition, a
  revised statistical vintage, a new taxonomic list).
- **World events** — a fire, flood, clearing event, or de-listing changes
  ground truth faster than any release cycle (the 2-night burn against the
  30-year accrual — [accrual.md](accrual.md)).
- **Interpretation staleness** — a result claim read a *vintage*; when that
  vintage reconciles forward, every claim citing it becomes re-checkable.

## 2. The two views

| View | Contents | Truth status |
|---|---|---|
| **Internal** | What this node/pack asserts — ingested volumes, ratified claims, instrument results, computed state, each stamped with `{source, version, vintage}` | *Held* — honest about provenance, not guaranteed current |
| **External** | What sources assert now (latest release/API state) and what events evidence (sensor/observation feeds) | *Asserted-elsewhere* — authoritative only within its own provenance; still subject to licence, sensitivity, quality gates |

Neither view outranks the other structurally: a newer external release is
*evidence for* update, not authority to overwrite (licence/sovereignty review
still applies); an internal contest can legitimately hold a divergence open
against upstream (e.g. custodial knowledge contesting an institutional record —
SOC-AX-7).

## 3. REC-AX — reconciliation axiom family

| Axiom | Statement | Rejects |
|---|---|---|
| **REC-AX-1 Reconciliation is an event with a record** | Each reconciliation run is a first-class `ReconciliationEvent` — `{views compared, basis, outcome, actor/instrument, timestamp}` — in the provenance graph, not a log line. | Silent sync; invisible refreshes |
| **REC-AX-2 Outcomes are typed** | `converged` / `diverged` (recorded divergence — both sides retained) / `deferred` (scheduled retry) / `contested` (dispute opened). Divergence is a stable state, not a failure to be forced. | Reconciliation ≡ overwrite; divergence ≡ error |
| **REC-AX-3 Staleness is a queryable property** | Every holding carries `reconciledAt` + expected-cadence; an *unreconciled-age* is computable at read time and surfaced to consumers. A pack claiming pre-burn forest state carries its staleness honestly until reconciled. | Freshness assumed from recency of *install* |
| **REC-AX-4 Event-driven beats cadence** | Disturbance event feeds (fire/flood/clearing sensors) trigger reconciliation of *affected holdings* by spatial/temporal overlap — not the whole corpus, and not on a calendar. | Polling everything; reconciling nothing until release day |
| **REC-AX-5 Change propagates as re-check, not rewrite** | A reconciled-forward source marks dependent claims `reCheckable` via their execution receipts — instruments may be re-run to produce *new* claims linked `wasRevisionOf`; originals are never edited (DOM-AX-2 + INT-AX-9). | In-place result mutation |
| **REC-AX-6 Both directions reconcile** | **Inbound**: external→graph (new release, new event). **Outbound**: graph→world (our claims checked against ground truth — "the graph says forest; the satellite says burnt"). Outbound checks are auditable acts too. | Only watching upstream, never checking ourselves |
| **REC-AX-7 Reconciliation respects the gates** | An external change does not bypass licence, sensitivity, sovereignty, or review gates — a reconciled-forward source produces an *update proposal* that re-enters the pipeline (S3), never auto-promotes. | "Newer = auto-installed" |
| **REC-AX-8 Reconciliation latency is declared** | Expected reconciliation lag per holding class is policy data (a fire-relevant claim has a tighter expected lag than a name-synonym list); breach of expected lag is itself a surfaced condition. | Pretending the corpus is current |

## 4. The temporal dynamics

```
        ┌── source-cadence trigger (release calendar / detected new vintage)
        ├── event trigger (disturbance feed ∩ holding extent)
        ├── access trigger (read-time staleness check on critical paths)
        └── manual/contest trigger (a human contests currency)
                        ▼
        ReconciliationEvent ──► outcome: converged | diverged | deferred | contested
                        ▼
        converged  → update proposal → S3 gate → new state (wasRevisionOf)
        diverged   → divergence record (both views held, aged, visible)
        deferred   → scheduled retry + staleness continues accruing
        contested  → review lane; the dispute is the state until resolved
```

**The tree worked-example end-to-end:** the pack asserts a mature stand;
DEA/FIRMS hotspots intersect the stand's extent → event-triggered
reconciliation → burnt-area confirmation layers (they are *provisional* by
design — "unverified burnt area" is the product's own honesty) → a
`LossEvent` record + post-state stock (AC-AX-10, degraded≠absent) → dependent
interpretation receipts flagged `reCheckable` → consumers see staleness
throughout the lag rather than a pretend-current map.

## 5. Event-source register (external-view feeds)

| Source | What | Access | Notes |
|---|---|---|---|
| **NASA FIRMS** | Active-fire hotspots, VIIRS/MODIS, NRT + historical | Free `MAP_KEY` API (`/api/area/csv/...`), 10-min rate windows — verified 2026-10-12 | Global disturbance trigger feed |
| **DEA Hotspots** | AU national hotspots, ~10-min update cadence | Open (WMS/WFS + downloads) | AU primary event feed — verified |
| **DEA Burnt Area / Characteristic Layers** | Same-day provisional burn detection, Sentinel-2 | Open, DEA Sandbox/Maps | Provisional by design — reconciliation *confirms*, the feed *signals* |
| **State fire/land services** | Incident maps, prescribed burns, clearing notifications | per-state | Some event truth is institutional, not remote-sensed |
| **Source release calendars** | ABS/OECD release schedules, census cycles, ASGS editions | published calendars | Cadence triggers + expected-lag policy inputs |

## 6. Fit to existing machinery

- DOM-AX-2 (nothing overwrites) + CRDT/Merkle history — divergence records are
  durable by default; converged updates are new states.
- Allen intervals + `asOf` queries — "what did we believe about X at T" is a
  query, not a reconstruction.
- `si:` receipts make dependent-invalidation mechanical — the receipt knows
  which vintage it read.
- `network_quanta` budgets bound reconciliation sweeps like any other work;
  event-triggered reconciliation runs as governed jobs (webizen-execution
  lanes — a reconciliation sweep is an ontology/data-lane task).

## 7. Open questions

1. Holding→feed binding — how does a holding declare which external feeds can
   trigger its reconciliation (extent keying: geometry ∩ feed event; entity
   keying: taxon list change; keying is per-holding policy data)?
2. Divergence aging — does a long-open divergence decay claim confidence
   (epistemic band downgrade with age) or stay stable until resolved? Lean:
   visible age, stable band — aging is presentation of the record, not decay
   of the claim.
3. Reconciliation of *interpretations* vs *observations* at different cadences
   — observations reconcile to sources; interpretations reconcile when their
   *inputs* moved (transitive staleness through receipt chains).
4. Who can contest a reconciliation outcome (review-workflow standing matrix —
   divergences on custodial content route sovereign lane).
5. Offline-partition semantics — a node offline for a year reconciles in bulk;
   add-wins-by-id holds for records but divergence *detection* cost scales —
   probably a reconciliation-cursor per source.
