# Implementation strategy — RD&D frame & priorities

**Status:** draft v0.3 · the frame plus a proposed priority ordering
(**proposed — awaiting your confirmation/adjustment**). v0.3 inserts the
game-demo remediation prerequisite (Phase 0A, per
[tech/game-demo-remediation.md](tech/game-demo-remediation.md)).

## 1. The RD&D driver: `game-demo`

The first implementation focus is **`C:\github\game-demo`** (Maslows
Challenge) — deliberately dual-purpose:

- a **useful product** in its own right (Australia-first browser/WASM game);
- the **development test-case** — its requirements force QualiaDB
  functionality to be *actually implemented and functioning*, not merely
  specified. Its written requirements (ecology contract, procedural pipeline,
  modality adoption in doc 40) are the demand curve ecopunk and the engine
  build against.

The dependency triangle to keep straight:

```
        game-demo (product + test-case)
          consumes ▲        ▲ forces
          packs    │        │ real impl
                   │        │
   ecopunk ── produces packs/corpus ──► qualiaDB (engine)
        ▲                                  ▲
        │  hero assets, authored media     │  capabilities
        └── poet + poet authoring studios ─┘  land upstream
```

**ecopunk's role in the loop:** the sourced, licensed, provenance-bearing
corpus — locality profiles, species selections, layer packs, style-tiered
assets — that game-demo's pipeline consumes. Where game-demo needs capability
X, that is *engine work* (the QualiaDB-only contract); where it needs *data*
Y, that is ecopunk work.

## 2. The authoring uplift (model quality)

Model quality uplift runs through the **Poet authoring studio suite** (per-medium
tools — see `tech/interfaces.md` §6) + agent-friendly authoring upstream.
ecopunk's stake: our style-tier quality gates (`assets/style-and-scale.md`)
define the *bar*; the *studio tools produce the assets that meet it.

## 3. Delivery targets (from style-and-scale §8)

Mobile/tablet · desktop/laptop · AR (elevation-aware anchors) · **film/TV
linear HD** — declared per artifact; linear output is a first-class product.

## 4. Proposed priority order (v0.3 — for confirmation)

Ordered so each phase proves the next thing game-demo needs:

### Phase 0 — Foundations in writing (now, near-done)
Planning docs complete enough to work against; gap register consolidated
(`engine-requirements.md`); namespace/PID decision **G16** — the one pending
user decision blocking vocabulary work. G16 blocks *new minting* of
vocabularies — it does not block repairs, diagnosis, or native work such as
the remediation below.

### Phase 0A — Game-demo stabilization & qualification (prerequisite)
The driver/test-case must be *reproducibly measurable and correct* before
pipeline breadth means anything: findings F1–F10 and work packages **R0–R4**
in [tech/game-demo-remediation.md](tech/game-demo-remediation.md) —
reproducible browser baseline + correctness fixes (R0), loader/scene
lifecycle (R1), hot-path and Vibe-boundary corrections (R2), verified pack
service (R3, [tech/pack-delivery.md](tech/pack-delivery.md)), and
Webizen-governed workers (R4, [tech/webizen-execution.md](tech/webizen-execution.md)).
Stabilization acceptance before breadth work resumes:

- compatible artifacts (same pin across builds) with a **verified textured
  pack load**;
- a **race-safe loader** with a real first-present receipt;
- **no non-state recompiles or text reparse in the frame path**, with the
  animation corrections qualified (F2–F4);
- **named-device browser performance evidence** against agreed budgets —
  captured pixels/frame data, not Node checks;
- the **first approved illustrated-realism hero scene** per R5 — this gate
  stands even though broader R5 content work continues afterwards.

R0–R4 deliver their services incrementally; the runtime-worker
OffscreenCanvas/shared-memory extensions and the optional R6 lane are **not**
gates on the earlier fixes.

### Phase 1 — First pipeline end-to-end (thin slice)
**Goal:** one reviewed pack consumed by game-demo — proof of the whole loop.

- **P-data**: *one locality-profile pack* for game-demo's sample corridor —
  licence-clear (P0/P1) sources only: NSL names backbone (subset), IBRA7/NVIS
  context layers, a bounded ALA occurrence slice, TERN vegetation.
- **P-pipeline**: LOD vocab registry extension (Darwin Core, TERN, ENVO,
  QUDT) → occurrence→quins mapper → `.q42` compile → `.hmc` pack → pack
  manifest + review-gate pass.
- **P-quality**: **one hero species family** (the bottlebrush/banksia/
  grass-tree/saltbush/kangaroo-paw group game-demo already uses
  illustratively — replace illustrative with sourced+styled) in T3,
  desktop+mobile delivery classes.
- **Depends on:** G1 (only if raster layers needed — can defer), G16
  (namespace), review-workflow fast lane.

### Phase 2 — Contribution loop live
- **P-interfaces**: observation+identification contribution kind through
  Poet (`vision` invoke) → proposals → review fast-lane → pack deltas —
  the smallest complete contribution loop.
- **P-pipeline**: media capture → fragment quins (ma-ont) — needs G6
  temporal-encoding unify.
- **P-economics**: the contribution loop's obligation accounting runs on the
  fair-compensation model ([tech/compensation.md](tech/compensation.md),
  G43) — labour classes + PPP/living-wage benchmarks + agent-kind modulation
  feeding the verified `royalty()`/`commons_cost` machinery.
- **P-network**: seed+cache node (pack availability) — testnet first (G13).

### Phase 3 — Layer breadth + quality uplift
- More layer families (terrain `.10d` exists; historical borders; Country
  sovereignty layer with custodial process — sovereign lane).
- **Socioeconomic statistics layer** ([tech/statistics.md](tech/statistics.md)):
  ABS Data API SDMX ingest (G38), ASGS editioned geographies, one SEEA/land-
  account bridge pack (G41), statistical claims stored under STAT-AX epistemic
  rules.
- **Reconciliation hooks** ([tech/reconciliation.md](tech/reconciliation.md)):
  staleness metadata on holdings + one event-triggered reconciliation path
  (FIRMS/DEA disturbance → affected-locality re-check flags) as the corpus
  freshness proof (G46).
- Hero-first expansion to more families; AR delivery class + anchored layers
  (G22); bioacoustic packs (G2+G25).

### Phase 4 — Products & network
- Chora `biosphere_world` bound to reviewed packs (G21); encyclopedia
  surface (webcivics); node roles expand (resolve, edge); micropayment rail
  when G11 lands; film/TV linear deliverables as quality demonstrator.

## 5. Constraints that shape sequencing

- Stubs are being eliminated upstream by another agent — sequence *around*
  confirmed-implemented capability, not against the stub list.
- Licence review gates are per-source; the first packs should pick
  *already-clear* licence lanes (P0/P1) to prove the pipeline end-to-end.
- The webcivics participation tier defines the minimum-spec device envelope;
  mobile delivery class is the harder constraint.
- **Range-addressable packs change the consumption pattern — with honest
  scoping.** `Q42ByteRange` + `BIDX` + HTTP `Range`/`206` + trustless IPFS
  partial-CAR (`q42/volume/`, `webtorrent_routes.rs`) are verified substrate —
  but BIDX is the `.q42` object→block index, **not** the `.hmc` QBDL entry
  index, and the range source is native (`cfg !wasm32`); browser integration
  is open. Consumption is **both** explicit install *and* streaming through
  one verified service ([tech/pack-delivery.md](tech/pack-delivery.md)) — the
  current game-demo still does an unintegrated whole-pack fetch (remediation
  F5). Phase-1 acceptance should include a range-fetched layer read where the
  service supports it, not just a whole-pack load — it also makes
  partial-volume cache nodes useful from day one (network-nodes §1).
