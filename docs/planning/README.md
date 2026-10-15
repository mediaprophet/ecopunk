# ecopunk — Planning & Specification Index

**Status:** Draft v0.1 · **Started:** 2026-10-12 · **Horizon:** 50+ year system lifetime

This directory is the working space for planning, architecture, and specification of the
**ecopunk** project: the production of biosphere-related assets, datasets, ontologies, and
structured resources for the **QualiaDB** ecosystem, plus the interfaces that make them usable.

The planning corpus is expected to be built over multiple sessions. When it is complete and
stable it will be used to derive a formal **implementation plan**. Proposed strategies and a
remediation plan now exist (`implementation-strategy.md`,
`tech/game-demo-remediation.md`) but are **not yet implemented or ratified** — see
`roadmap.md` for framing only.

---

## Design posture

Two decisions govern everything below:

1. **Comprehensiveness over early scoping.** Requirements are recorded even when no
   implementation is feasible yet. Nothing is rejected for being "too big" — it is deferred,
   not deleted.
2. **50-year+ durability.** Every format, identifier, licence choice, and packaging decision
   is evaluated against multi-decade survivability: institutional churn, format rot,
   namespace persistence, and hardware evolution.

---

## Document map

### Core planning

| File | Contents |
|---|---|
| [charter.md](charter.md) | Vision, purpose, relationship to QualiaDB, governing principles, non-goals |
| [implementation-strategy.md](implementation-strategy.md) | RD&D frame + proposed phase order — game-demo as driver/test-case, Phase 0A remediation prerequisite, Phase 1 thin-slice target, poet-*craft uplift | draft v0.3 |
| [requirements.md](requirements.md) | Comprehensive requirements register (REQ-* IDs, all domains) |
| [architecture.md](architecture.md) | Layered system architecture, pipeline, package types, distribution model |
| [domain-model.md](domain-model.md) | Domain model — living/made partition, scale ladder, border kind-family, entity & relation classes, spatio-temporal rules | draft v0.1 |
| [sociosphere.md](sociosphere.md) | The sociosphere layer — place-as-relation (Country ≠ property), standing axes, custodianship, identity-by-relation, language-as-meaning | draft v0.2 |
| [roadmap.md](roadmap.md) | Phase framing only — the real implementation plan is derived later | planned |

### Data sources (`data-sources/`)

| File | Contents | Status |
|---|---|---|
| [README.md](data-sources/README.md) | Evaluation rubric, registry schema, acquisition workflow | draft |
| [australia.md](data-sources/australia.md) | AU-first discovery register — ALA/TERN/DCCEEW/NSL/states/IMOS + LOD ontology layer | active discovery |
| [socioeconomic.md](data-sources/socioeconomic.md) | Socioeconomic & official statistics — ABS/ASGS/SEEA, AIHW, classifications, UN/World Bank/OECD/ILO/FAO/WHO/OWID, MRIO footprint tables, restricted-microdata register | draft v0.1 |
| `global.md` | Global sources (GBIF, CoL, WoRMS/OBIS, IUCN, OToL, WDPA, climate, media, 3D) | planned |
| `licensing.md` | Licence policy detail (absorbed into tech/governance.md for now) | planned |

### Technical specifications (`tech/`)

| File | Contents | Status |
|---|---|---|
| [engine-capabilities.md](tech/engine-capabilities.md) | Surveyed QualiaDB capability map — ~2,509 src files / 334 dirs + sibling crates, per-area, with honesty status | draft v0.2 |
| [engine-comparison.md](tech/engine-comparison.md) | QualiaDB vs traditional semantic stacks — dimension table, 7 structural differentiators, honest gaps, ecopunk consequences | draft v0.1 |
| [capability-spotlights.md](tech/capability-spotlights.md) | 14 concrete scenarios (S1–S14): goal → traditional path → QualiaDB path → modules → evidence | draft v0.1 |
| [wasm-edge.md](tech/wasm-edge.md) | The browser/WASM dimension — profile ladder, JS surface, portal renderer, native-only boundaries, participation-is-deployment tiers (webcivics = for-purpose entry) | draft v0.2 |
| [map-layers.md](tech/map-layers.md) | The map-layer model — independent governed layers, family registry, representations, temporal rules, game-demo consumer contract | draft v0.1 |
| [classifier-packs.md](tech/classifier-packs.md) | Decentralized classifier support in commons packs — HMC payload inventory, SPARQL-MM media model, P64-canonical rule, correction federating | draft v0.1 |
| [durability-and-distribution.md](tech/durability-and-distribution.md) | Identifiers beyond DNS — dual addressing, `did:qi`/QDNF, DHT/DLT/offline distribution, `pc:Micropayment` infrastructure economics | draft v0.1 |
| [cooperative-layer.md](tech/cooperative-layer.md) | Social commons (`spc:`) + cooperative economics — persistence classes, category-scoped fees, contribution→obligation accounting, royalty trees | draft v0.1 |
| [contribution-chains.md](tech/contribution-chains.md) | Multi-kind contributions converging on entities — photos, preparation knowledge, lab chemistry, literature; epistemic classes, labour quins, obligation flow-up | draft v0.1 |
| [tensor-10d.md](tech/tensor-10d.md) | The 10D Q42 tensor spec reconciled with code — axis semantics (q superposition, v metrics, w domains, αμσ spectral), GSR escrow, gravito-thermo baking | draft v0.1 |
| [interfaces.md](tech/interfaces.md) | The explore↔author loop — Chora (commons canvas, `biosphere_world` flagship) + Poet (tool-chest, invoke domains, receipts→labour quins) + wiring work | draft v0.1 |
| [development-modes.md](tech/development-modes.md) | VibeScript-first rule (JS never a fallback — gaps go upstream) + native vs standard-web browser modes | draft v0.1 |
| [network-nodes.md](tech/network-nodes.md) | Node roles (cache/seed/relay/resolve/custody/attest/edge), deployable testnet, stewardship-transfer as designed success | draft v0.1 |
| [pipeline.md](tech/pipeline.md) | Source → `.q42` / `.hmc` pipeline — stages S0–S9, artefact paths, partitioning, tooling | draft v0.1 |
| [governance.md](tech/governance.md) | Data-governance requirements — licence tiers/lanes, sensitivity, provenance, honesty, sovereignty, durability, integrity, review gates | draft v0.1 |
| [ontology-stack.md](tech/ontology-stack.md) | Vocabulary architecture — L0–L4 layers, LOD-cloud processing, inference ladder, decentralised-training hooks, namespace/PID strategy | draft v0.1 |
| [engine-requirements.md](tech/engine-requirements.md) | Consolidated gap register G1–G51 — formats, semantics, network, governance, interfaces, media, trust, runtime/execution, statistics, interpretation, temporal dynamics, probability fields | draft v0.1 |
| [review-workflow.md](tech/review-workflow.md) | The human gate — ratification ladder, standing matrix, review lanes, anti-abuse, reviewer surfaces | draft v0.1 |
| [product-surfaces.md](tech/product-surfaces.md) | End-user products — encyclopedia, explorer, educational, field/AR, cinematic, sanctuary companion; invariants + dependencies | draft v0.1 |
| [threat-model.md](tech/threat-model.md) | Adversarial threats T1–T12 — sybil, poisoning, abuse, capture, exfiltration; structural answers vs residual work | draft v0.1 |
| [game-demo-remediation.md](tech/game-demo-remediation.md) | Game-demo findings and proposed remediation — confirmed defects, source risks, browser evidence pending; findings F1–F10, work packages R0–R6 | draft v0.1 |
| [webizen-execution.md](tech/webizen-execution.md) | Proposed Webizen-governed worker orchestration — admission, lanes, persistent workers, job/result descriptors | draft v0.1 |
| [pack-delivery.md](tech/pack-delivery.md) | Proposed verified pack acquisition — stream/download/hybrid/import modes, bounded acquisition, loader lifecycle | draft v0.1 |
| [statistics.md](tech/statistics.md) | Statistics model — STAT-AX-1–10 axiom family, Data Cube/SDMX alignment, vintages/confidentiality/classifications, claims-vs-facts, SEEA biosphere bridge | draft v0.1 |
| [compensation.md](tech/compensation.md) | Fair compensation — COMP-AX-1–12: work-type × place × agent-kind obligation computation; stock-drawdown ledger, delivery-structure-as-evidence, exploitative-project worked case | draft v0.1 |
| [interpretation-frameworks.md](tech/interpretation-frameworks.md) | Interpretation layer — INT-AX-1–10: bound executions over `si:` semantic instruments, axiom packages, plural/contested readings, claims-never-facts | draft v0.1 |
| [accrual.md](tech/accrual.md) | Accrual — AC-AX-1–14: stocks with time-depth, non-collapsible accrual dimensions, formation/destruction asymmetry, irreversibility classes, spent-lifetime & window-bound losses | draft v0.1 |
| [reconciliation.md](tech/reconciliation.md) | Internal↔external reconciliation — REC-AX-1–8: typed outcomes, staleness as data, event-driven triggers (FIRMS/DEA), re-check propagation, gate-respecting updates | draft v0.1 |
| [probability-geometry.md](tech/probability-geometry.md) | Probability as ontological geometry on Q42 — PROB-AX-1–14: fields over state/factor/epistemic axes, path-conditioned futures, logic↔field evaluation, friction fabric, constraint layers & choice fields | draft v0.1 |

### Asset specifications (`tech/assets/`)

| File | Contents |
|---|---|
| [asset-package-spec.md](tech/assets/asset-package-spec.md) | Asset package layout — semantic manifest, geometry sidecars, LODs, provenance | planned |
| [style-and-scale.md](tech/assets/style-and-scale.md) | Style tiers T5–T0 (captured-real → symbolic) × scale rungs (molecule → biome), quality gates, delivery-target classes (mobile/desktop/AR/film) | draft v0.1 |
| [media-types.md](tech/assets/media-types.md) | Media beyond 3D — bioacoustic, field video, stills, documents/plates, specimen scans, sensor series; truth-layer + fragment rules | draft v0.1 |

---

## Working conventions

- **Requirement IDs** (`REQ-<area>-<nnn>`) are stable once published; never renumber — retire.
- Every doc carries a status line (`draft` / `review` / `agreed`) and an **Open questions**
  section where unresolved decisions accumulate rather than being silently assumed.
- Provenance and licensing claims cite the source document/URL and the date checked. Do not
  assert a licence from memory — record what was actually verified.
- Documents describe *target* state. Where QualiaDB capability is missing, the gap is logged
  in [engine-requirements.md](tech/engine-requirements.md) rather than assumed away.
- One topic per file; files stay under ~500 lines. Split by lifecycle, not arbitrarily.
