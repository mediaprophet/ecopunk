# Statistics — socioeconomic observation model, process, and logic

**Status:** draft v0.1 (concept, recorded 2026-10-12)
**Feeds:** [../sociosphere.md](../sociosphere.md), [../domain-model.md](../domain-model.md),
[ontology-stack.md](ontology-stack.md), [pipeline.md](pipeline.md),
[governance.md](governance.md), [engine-requirements.md](engine-requirements.md)
**Source register:** [../data-sources/socioeconomic.md](../data-sources/socioeconomic.md)

---

## 1. The claim being modelled

Official statistics are the **statistical shadow of the sociosphere**: governed,
methodology-bearing observations about *populations, economies, areas, and
cohorts* — never about persons. They are not raw facts: every number is produced
by a defined instrument (a census, a survey, an accounts framework), under a
published methodology, against a versioned classification and a versioned
geography, and is subject to revision and confidentiality processing.

Two observations anchor the design, in the same style as sociosphere.md:

1. **A statistic is an act of measurement with a lineage.** "Unemployment,
   SA4=Melbourne-Inner, 2025-Q3" is meaningless without its dataflow, DSD,
   codelists, collection method, estimation method, and vintage. The number is
   the leaf; the structure is the truth.
2. **Influence is a claim, not an observation.** Correlations, causal
   attributions, "drivers", and policy effects are *claims* asserted by a source
   (a report, a study, a methodology) under an evidence class. They are stored as
   claims — contested pairs coexist — and never collapse into the observation
   layer.

Consequence: statistics enter the graph through the **RDF Data Cube / SDMX
shape** — observation = value + dimension vector + attribute set + measure +
dataset — with the SDMX structural artefacts (dataflow, DSD, codelists,
concept scheme, agency, version) ingested as first-class vocabulary.

## 2. Core model — Data Cube / SDMX alignment

| Concept | Binding | Notes |
|---|---|---|
| `qb:DataSet` / SDMX dataflow | `stat:DataSet` (subclass) | Identified by `{agencyId, dataflowId, version}` — the version is part of identity |
| `qb:Observation` | `stat:Observation` | Value + dimension key + attributes + unit + vintage marker |
| `qb:DataStructureDefinition` | `stat:DSD` | Dimension/measure/attribute declarations — ingested as vocabulary, not config |
| SDMX codelists / concept schemes | `skos:ConceptScheme` + `xkos:` extensions | Codes are concepts in governed schemes (STAT-AX-8) |
| `qb:Slice` | series key (SDMX series) | Time-series slices = observation groups sharing all-but-time dimensions |
| Units / measures | **QUDT** (already in stack) | Same path as TERN result units |
| Measure types | `stat:level`, `stat:rate`, `stat:index`, `stat:share`, `stat:rank` | Index/rank are *derived* — methodology mandatory (STAT-AX-6) |
| Geographic dimension | versioned `Border`/geography entities | ASGS codes resolve to edition-scoped regions (DOM-AX-3) |
| Confidentiality marks | `stat:confidentiality` attribute family | suppressed / perturbed / not-published-not-suppressed / modelled (STAT-AX-5) |
| Provenance | PROV-O + `asset_envelope` | retrieval, query string, snapshot hash, release vintage |

## 3. STAT-AX — statistics axiom family

| Axiom | Statement | Analogue it rejects |
|---|---|---|
| **STAT-AX-1 An observation is an event-context record** | Every observation carries (who measured, when, methodology version, instrument, release vintage). A number without lineage is a claim, not a statistic. | Bare KPI values; "the number" detached from its production |
| **STAT-AX-2 Statistics are about populations** | Observations attach to populations/areas/cohorts — never to identified individuals. Microdata stays outside the corpus; aggregates only (SOC-AX-8; GOV-SOV-7 for the identity gate). | Person-attached stats; re-identification joins |
| **STAT-AX-3 Statistical geography is versioned infrastructure** | ASGS editions, electoral redistributions, postcode geographies are `Border` kind-family members (DOM-AX-3); **concordances between editions are lossy proposals** with documented error, never silent re-mapping. | Treating SA2@2016 = SA2@2021; geography as static lookup |
| **STAT-AX-4 Vintages are first-class** | Revised series keep every released vintage; "as reported at date X" is a queryable state (real-time data). DOM-AX-2 validity intervals apply to *release state* as well as world state. | Latest-only databases that silently rewrite history |
| **STAT-AX-5 Confidentiality marks are data** | Suppressed/perturbed/confidentialised cells are *statements* — the mark travels with the cell; a suppressed value is never back-filled as an observation. Modelled fills are flagged `modelled` with method provenance. | Missingness treated as absence; silent imputation |
| **STAT-AX-6 Indices are derived claims** | CPI, SEIFA, HDI, rankings are *constructed* — they bind to a methodology entity + version + weighting scheme; re-basing/chaining is provenance, not a new index. A trend in a chained index is a claim about the chain, not the world. | Index number treated as a measurement |
| **STAT-AX-7 Influences are claims, not facts** | Correlation, causal, and policy-effect claims carry (source, evidence class, epistemic band, method); contested claims coexist (paraconsistent modality); corpus-derived correlations are *proposals* for ratification (alignment-rule precedent). | "Data shows X causes Y" as assertion |
| **STAT-AX-8 Classifications are governed vocabularies** | ANZSIC/ANZSCO/ISIC/ISCO/SACC/IUCN-GET are versioned concept schemes (XKOS); cross-scheme mappings are correspondence *proposals* with loss/error metadata. | Flat codelists; silent code translation |
| **STAT-AX-9 Aggregation is lossy and documented** | Every aggregate carries its aggregation path (units → small-area → region); small-area estimates and MRIO footprints are *modelled* — flagged with method + uncertainty where published. | Aggregates as raw truth; footprints as measurements |
| **STAT-AX-10 Statistics about peoples carry sovereignty conditions** | Statistical products concerning Indigenous peoples/Country follow community data-governance principles (Maiam nayri Wingara; CARE); sovereignty review lane applies to packs in this domain (review-workflow.md). | Population statistics as politically neutral facts |

## 4. Process — the statistics ingest profile (pipeline S1–S9 specialisation)

```
S1 ACQUIRE    dataflow discovery → structure harvest (DSD, codelists, concepts,
              agencies) THEN observation slices — structure before data, always.
              Snapshot = {dataflow id+version, query key, retrieval timestamp,
              response hash}. OECD/ILO rate limits → scheduled, resumable jobs.
S2 STAGE      inbox per {agency}/{dataflow}/{version}/{slice-date}
S3 REVIEW     licence tier (per dataset, not portal), confidentiality semantics
              preserved, sovereignty lane check (Indigenous geographies →
              sovereignty review), methodology completeness
S4 NORMALISE  DSD/codelists → vocab layer (XKOS-shaped); observations →
              qb:-shaped NQuins; ASGS codes → edition-scoped region IRIs;
              attributes incl. confidentiality marks preserved per cell
S5 VALIDATE   SHACL: cube well-formedness (every obs has all required dims,
              dims resolve to codelist members), unit presence (QUDT),
              vintage stamping, no unmark suppression
S6 COMPILE    per-acquisition-unit volumes: {agency × dataflow × version ×
              release-vintage} — a revised release is a NEW volume linked by
              wasRevisionOf, never a mutation (STAT-AX-4)
S7 PACK       stats packs carry: observations + DSD/codelist vocab closure +
              methodology docs (doc entries) + concordance proposals
S8 PUBLISH    lane per licence cohort; microdata-derived never published
S9 LEDGER     capability note: SDMX ingest version, codelist coverage
```

**Reuse of existing machinery:** `csv_mapper`/`json_mapper` handle table → RDF
once a per-source profile exists; the new work is the **SDMX structure layer**
(G38) — dataflow/DSD/codelist discovery and versioned structure records.
`network_quanta` ScanBudget bounds the slice-fetch jobs; pack delivery uses the
same range-backed service as everything else.

## 5. Biosphere interaction — the environment-economy join

The user-flagged interaction is structural, not incidental:

| Bridge | Mechanism | Status |
|---|---|---|
| **SEEA ecosystem accounts** (ABS NEA, land accounts) | Statistics *about* `env:`/`bio:` assets — extent, condition, services — classified by **IUCN GET** (shared spine with the biosphere layers). Ingest as the official bridge schema. | AU products verified (NEA 2022-23; Land Account); ingest path = G38 family |
| **Land use × ecosystem** | ABARES catchment land-use ∩ IBRA/NVIS — spatial join layers, same geoprocessing as ecological layers | data + geometry paths exist |
| **Commodification chains** | Harvest/trade `Act`s (DOM-AX-1) link living entities → commodities → trade/industry statistics (ANZSIC codes, Comtrade flows) | model exists; statistics bind at the `Act` edge |
| **Footprint / embodied impact** | Consumption → production → environmental pressure via MRIO (OECD TiVA open; Eora licensed — register only) | **modelled claims** — STAT-AX-7/9; carry `{model, version, satellite, uncertainty}` |
| **Population × place** | ASGS regions ↔ IBRA/drainage divisions: the people-side of "who lives in this bioregion" | geography-join only; never person-level |
| **Policy influence claims** | "MPAs change fisheries employment" style claims → claim entities w/ evidence class, contested pairs kept | epistemic machinery verified in-engine |

**Guardrail:** SEEA monetary valuations of ecosystem services are *methodology
outputs*, stored as index-family claims (STAT-AX-6) — the graph records the
valuation *and* that it was produced by a chosen method, never "ecosystem worth
= $X" as a bare property.

## 6. Interface consequences

- **Encyclopedia:** population/area boxes cite dataflow + vintage inline;
  contested claims render as contested (epistemic band UI, G18).
- **Explorer/map layers:** choropleth layers are *statistics layers* —
  dimension vectors selectable; geography edition shown honestly
  ("2021 ASGS boundaries").
- **Educational surface:** "how this number was made" — the DSD, instrument,
  and revision history are navigable content, not footnotes.
- **No person-pivots:** interfaces never join a statistic to an individual
  record or imply one (STAT-AX-2; rendering-honesty rule like SOC rendering).

## 6A. Interpretation binding

Statistics' outputs — correlations, derived indicators, cross-dataset reads —
are produced by **interpretation events**, not absorbed as facts. See
[interpretation-frameworks.md](interpretation-frameworks.md): every derived
result binds `{data slice × instrument release × entry point × inputs}` through
an `si:ExecutionReceipt`; rival instruments over the same data produce
coexisting claims (INT-AX-2), and axiom context (which of SOC/STAT/DOM/COMP
packages the reading ran under) is itself a declared dependency (INT-AX-6).
A statistical claim without an instrument link is the informal-reading class —
weakest provenance, still first-class.

**Probabilistic claims are fields.** Any statistic presented as a probability —
lifespan, incidence, risk — is a `ProbabilityField` or a *declared projection*
of one (PROB-AX-1/9): conditioning factors, epistemic axes (assumptions,
instrument release, axiom context), and vintage travel with the value. See
[probability-geometry.md](probability-geometry.md). Cohort fields never apply
to a person without a person-level conditioning receipt (PROB-AX-5 extends
STAT-AX-2).

## 7. Open questions

1. `stat:` minted namespace vs extending `qb:`/`sdmx:` directly — lean: subclass
   Data Cube, mint only confidentiality/vintage machinery SDMX-RDF lacks.
2. SDMX → QB bridge implementation owner: upstream QualiaDB ingest profile vs
   ecopunk mapping scripts (G38).
3. Revision storage granularity: volume-per-vintage is clean but multiplies
   volumes for monthly series — partition by {dataflow × year-vintage-cohort}?
4. Concordance representation: `xkos:correspondence` entities vs named-graph
   proposals — needs a test against ASGS Ed2→Ed3 mesh-block concordances.
5. Which statistical products get first-class UI (population pyramids, time
   series) vs reference-only (data layers) — product-surfaces iteration.
6. Do seea-account cells reference `bio:`/`env:` entities directly (dual-typed
   subject) or via account-area geometry joins? The NEA IUCN-GET classification
   suggests entity-level joins are feasible — test at ingest.
