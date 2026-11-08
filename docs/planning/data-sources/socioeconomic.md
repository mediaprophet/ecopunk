# Data sources — socioeconomic & official statistics (discovery register)

**Status:** discovery draft v0.1 · **Date checked:** 2026-10-12 · **Scope:** public
statistical data about people, economies, institutions, and their environmental
interaction — AU-first, then international systems.

The biosphere register ([australia.md](australia.md)) covers living-world data.
This register covers the **statistical shadow of the sociosphere**: the governed,
methodology-bearing numbers institutions publish about populations, economies,
health, land use, and environment-economy accounts. The data *model* these feed
(Data Cube / SDMX, vintages, confidentiality, claims-vs-facts) is specified in
[../tech/statistics.md](../tech/statistics.md); the sociosphere axioms they must
respect are in [../sociosphere.md](../sociosphere.md).

**Legend** — Access: `open` | `registered` | `request` | `restricted` |
`licensed` (paid/contractual). Licence recorded as published on the date checked;
**verify at acquisition time** — licences change and record-level terms differ
from portal-level statements.

---

## 1. AU national statistics infrastructure

| Source | What | Access & format | Licence | ecopunk notes |
|---|---|---|---|---|
| **ABS Data API** — data.api.abs.gov.au | The Commonwealth statistical spine: Census, labour force, CPI, national accounts, population, health/social surveys — exposed as SDMX **dataflows** | **SDMX 2.1 REST** (verified): `GET /rest/data/{agency},{flow},{version}/{dataKey}`, SDMX-ML/JSON/CSV; structure queries for DSDs/codelists; **no API key** since Nov-2024 (register-of-interest); OpenAPI def on api.gov.au | CC-BY (ABS standard; verify per dataset) | **Canonical SDMX-shaped source.** Dataflow/DSD/codelist structures ingest as vocabulary; observations slice by key. First target for the statistics ingest path. |
| **ABS ASGS** (Australian Statistical Geography Standard) | The statistical geography framework: **Edition 3 (Jul 2021–Jun 2026)**: 368,286 Mesh Blocks → SA1→SA2→SA3→SA4→GCCSA→STE→AUS; plus Indigenous Structure (ILOC/IARE/IREG), Remoteness, UCL/SUA, and non-ABS structures (postal, LGA, electoral, **drainage divisions**) built from whole Mesh Blocks | Digital boundaries: Shapefile/GeoPackage downloads; allocation files (xlsx) carry **`ASGS_LOCI_URI_2021` linked-data URIs** (verified); ASGS Editions 1–2 remain as historical versions | CC-BY | **Statistical geography is a DOM-AX-3 border kind-family**: editions version on a 5-year cycle; concordances between editions are lossy proposals. The Loc-I URIs are government-minted PIDs — namespace durability precedent (cf. G16). Drainage-division membership is the direct bio↔soc join point. |
| **ABS environmental-economic accounts** | **National Ecosystem Accounts** (SEEA EA; experimental, 2022-23 FY release 2026-04-23 — verified) and **National Land Account** (land use/cover/tenure/value) — ecosystems classified by **IUCN GET** realms/biomes/EFGs; ecosystem extent/condition/services (carbon storage, storm protection) in physical + monetary terms | ABS releases (tables + methodology publications); via ABS site/data API | CC-BY | **The official biosphere↔socioeconomic bridge.** Ecosystem extent/condition/service accounts are statistics *about* `env:`/`bio:` entities under the SEEA framework — ingest as the bridge schema, not parallel data. GBR experimental accounts add social/Indigenous values. |
| **AIHW** — Australian Institute of Health and Welfare | Health & welfare national collections: hospitals (admitted patient, ED, elective), disease registers, mortality, welfare services, housing/homelessness, **Closing the Gap reporting** | MyHospitals API — **open, no auth**, JSON/CSV (verified); ~240 datasets on data.gov.au (mixed licence states — verify per dataset); AIHW **linked data assets** programme for record-level linkage (access by approval — restricted) | Mostly open/CC-BY at portal tier; linked microdata restricted | Population-health statistics as aggregates; the linked-data assets are a microdata sensitivity exemplar — never ingest unit records. |
| **Centre for Population** — population.gov.au | National population projections, regional population, migration | Publications + data downloads | CC-BY (verify) | Population projections are *modelled claims* — vintage + methodology entities. |
| **Jobs and Skills Australia** | Labour market analysis, skills priority lists, employment projections | data + reports via jobsandskills.gov.au | CC-BY (verify) | Occupation-level stats keyed to ANZSCO. |
| **RBA** — rba.gov.au | Monetary/financial statistics: rates, credit, exchange, inflation expectations | Statistical tables (xlsx/csv) + snapshot API | CC-BY-style (verify) | Macro-economic context layer. |
| **ATO** | Taxation statistics — income, tax, business by geography | Annual taxation statistics publications (xlsx/csv) | open gov | Finely-geographed income stats; aggregated/confidentialised only. |
| **ABARES** — agriculture.gov.au/abares | Agricultural/forestry/fisheries economics, **land use mapping (ACLUMP/catchment-scale)**, farm surveys, commodity statistics | Data downloads + spatial services | CC-BY (verify) | **Direct biosphere bridge**: land use × IBRA/NVIS intersections; commodity ↔ harvest acts (DOM-AX-1). |
| **ACARA / education data** | NAPLAN, school profile statistics (My School) | Downloads + My School extracts | mixed; privacy-gated detail | Educational statistics; school-level data carries suppression rules. |
| **AEC / electoral** | Enrolment, results, referendum data by division | Tally room data + GIS boundaries | open | Political-geography layer (electoral borders are a DOM-AX-3 kind). |
| **State crime/justice bureaus** | BOCSAR (NSW), CSA (VIC), QGSO (QLD) etc. | Published tables/APIs vary by state | mixed | Justice-domain statistics; strong suppression conventions. |
| **State data portals** | data.nsw/vic/qld/wa/sa/tas/nt/act — housing, health, education, transport statistics | CKAN + spatial services | mostly CC-BY | Same rubric as australia.md §4. |

## 2. Statistical classifications & concordances (AU + international)

Classifications are **governed vocabularies**, not lookup tables — they version,
and cross-classification mappings are correspondence proposals with known loss.
Model family: **XKOS** (SKOS extension for statistical classifications — levels,
correspondences, explanatory notes).

| Classification | Owner | International counterpart | Notes |
|---|---|---|---|
| **ANZSIC** (industry) | ABS | ISIC (UN) | Supply-chain/MRIO joins run on industry codes. |
| **ANZSCO** (occupation) | ABS/Stats NZ | ISCO (ILO) | Labour statistics dimension. |
| **ANZSOC** (offence) | ABS | ICCS (UNODC) | Justice statistics. |
| **ASCL** (language) | ABS | ISO 639 | Joins to AUSTLANG — language codes are sociosphere entities (SOC-AX-9). |
| **SACC** (country of birth) | ABS | UN M49 | Migration statistics. |
| **ASGS structures** | ABS | — | Geography-as-classification; above. |
| **IUCN GET** (ecosystems) | IUCN/UN SEEA | national ecosystem typologies | **The ecosystem classification used by ABS NEA** — shared spine with `bio:`/`env:`. |

## 3. International statistical systems

| Source | What | Access & format | Licence (checked 2026-10-12) | ecopunk notes |
|---|---|---|---|---|
| **UN SDG Global Database** — unstats.un.org | 200+ SDG indicators, country×year series, disaggregations | **SDMX API** + `sdgapi` REST + SDMX metadata API; DSD versioned via SDMX Global Registry; quarterly update cycle with archived releases | UN open-data terms (verify) | The global indicator framework — goal/target/indicator hierarchy is itself a governed classification. Versioned releases = vintage snapshots (STAT-AX-4). |
| **UNdata / UNSD** | National accounts, trade, demography, energy | UNdata portal + SDMX browser/API | UN terms | Broad cross-country context. |
| **World Bank** — data.worldbank.org | WDI (~1,400 indicators), plus sectoral collections | REST API (XML/JSON), bulk download, Data Catalog | **CC-BY 4.0 default for WBG-produced datasets; third-party datasets carry own licences — check each** (verified) | The biggest open indicator corpus. Record-level licence distinction applies — same rule as ALA records. |
| **OECD Data Explorer** — data-explorer.oecd.org | OECD-wide statistics incl. **TiVA** (trade-in-value-added — open MRIO-adjacent) | **SDMX REST** (`sdmx.oecd.org/public/rest`), free; **rate limit 60 downloads/hr, no VPN/anonymised traffic** (verified) | OECD T&C; per-dataset | TiVA gives the open-licence supply-chain/footprint substrate the commercial MRIOs don't. Respect the rate limit in acquisition jobs. |
| **IMF Data** | IFS, BOP, WEO, financial soundness | SDMX API + bulk | IMF terms | Macro/finance indicators. |
| **Eurostat** | EU statistics | SDMX REST + bulk TSV | CC-BY-style EU open data | Model SDMX implementation (ILOSTAT gateway is Eurostat NSI-derived). |
| **ILOSTAT** — ilostat.ilo.org | Labour statistics worldwide | **SDMX REST** (`sdmx.ilo.org/rest`) + bulk csv.gz + dictionaries; multi-format bulk (parquet/feather/rds) | ILO terms (verify) | Labour dimension corpus. |
| **WHO GHO** | Global Health Observatory indicators | OData/Athena API | WHO terms | Health indicator corpus. |
| **FAOSTAT** | Food/agriculture statistics, 245+ countries, 1961→ | REST API + bulk downloads | **CC-BY-4.0 current** (FAO statistical DB policy; **legacy pre-2019-11-30 content may carry CC BY-NC-SA 3.0 IGO** — check per dataset; verified both clauses) | Agriculture↔biosphere join; licence history needs per-dataset check at ingest. |
| **UNESCO UIS** | Education/science/culture indicators | API + bulk | UNESCO terms | Education domain. |
| **UNDP HDI** | Human Development Index + components | Downloads | UNDP terms | Composite-index exemplar — an index is a methodology, not a measurement (STAT-AX-6). |
| **UN Comtrade** | Merchandise/services trade by partner/commodity | API (free tier + subscription tiers) | Comtrade licence (verify tier) | Trade flows — commodification acts at international scale (DOM-AX-1). |
| **Our World in Data** — ourworldindata.org | Curated indicators + provenance-rich metadata | **Grapher API**: `{slug}.csv` / `.metadata.json` / `.zip` (verified) | **CC-BY 4.0 for OWID-produced work; third-party data keeps originator licence — metadata names it** (verified) | Best-practice precedent: per-variable source+licence metadata is exactly our provenance contract. |
| **IHME GBD** | Global Burden of Disease | GHDx registered download | Registered; non-commercial-ish terms (verify) | Health burden estimates — modelled claims (STAT-AX-9). |
| **V-Dem / Polity** | Democracy/governance indicator suites | Downloads; V-Dem has dataset + codebook | V-Dem: custom open licence | Governance/institutional influence indicators. |
| **WID.world** | World Inequality Database | Download tables | verify | Inequality series. |

## 3A. Demographics & vital statistics

Population stocks, structures, vital events, and projections — the demographic
substrate. **Projections are modelled claims** (STAT-AX-9): they carry scenario,
methodology and vintage; a projection *revises*, which is exactly why
STAT-AX-4 vintage storage matters.

| Source | What | Access & format | Licence (checked 2026-10-12) | ecopunk notes |
|---|---|---|---|---|
| **UN World Population Prospects (WPP) 2024** — population.un.org/wpp | The global demographic reference: estimates 1950→present, projections →2100, 237 countries/areas; single-year age×time intervals (since 2022 rev); built on 1,910 censuses + 3,189 surveys + vital registration | Data Portal + **open API** + CSV/Excel bulk; Special Aggregates | **CC BY 3.0 IGO** (verified) | Estimates *and* projections distinguished — projections are scenario claims, not observations. Edition-based releases (28th ed.) = vintage snapshots. |
| **ABS Estimated Resident Population (ERP)** | Quarterly AU population by age/sex/geography (SA2+); Census base populations | ABS Data API (SDMX) + publications | CC-BY | The AU demographic spine — joins to ASGS regions. |
| **ABS Census of Population and Housing** | 5-yearly full enumeration — demographic, social, household composition detail | Data API + TableBuilder (registered microdata tier) | CC-BY aggregates; microdata restricted | Mesh-block-limited release; suppression conventions apply (STAT-AX-5). |
| **ABS Births/Deaths/Marriages + cause of death** | Vital-statistics registrations | ABS publications/API | CC-BY | Vital event flows — the stock-change counterpart to census stocks. |
| **UN International Migrant Stock / IOM DTM** | Migrant stock by origin×destination; displacement tracking | Downloads/API | UN/IOM terms (verify) | Migration flow layer. |
| **OECD migration / DIOC** | Migration statistics, emigrant stocks | OECD SDMX API (60/hr limit) | OECD terms | Cross-check vs national sources. |
| **Wittgenstein Centre / IIASA** | Human-capital projections (pop × education attainment), SSP-linked scenarios | WIC data explorer/downloads | verify | Education-attained population projections — forward-looking modelled claims. |
| **IHME population forecasts** (GBD-linked) | Fertility/mortality/migration scenario forecasts | GHDx registered | registered terms | Alternative projection family — contested-with-WPP is a *feature*: rival projections coexist as claims. |

## 3B. Wages, PPP & living-wage benchmarks (fair-compensation substrate)

The data behind [../tech/compensation.md](../tech/compensation.md): PPP converts
place, living-wage benchmarks set floors, earnings series supply context, and
statutory minima bound the legal floor.

| Source | What | Access & format | Licence (checked 2026-10-12) | ecopunk notes |
|---|---|---|---|---|
| **World Bank ICP PPP** — via WDI | `PA.NUS.PPP` (GDP-level) + household-consumption PPP + price level indices; ICP 2021 cycle: 176 economies + imputed for 19; 2018–20 time series | WDI API (`/indicator/PA.NUS.PPP`), DataBank | CC-BY 4.0 | **ICP's own caveat (verified): PPPs are statistical *estimates*, "should be treated as approximations"** — a documented STAT-AX-9 modelled-observation; record the cycle vintage. |
| **OECD / Eurostat PPP programmes** | Regional PPP benchmarks feeding ICP for OECD/EU economies | OECD SDMX API | OECD/EU terms | Cross-check + finer EU coverage. |
| **Global Living Wage Coalition — Anker** | Living-wage/income benchmarks by region; **Anker Reference Values** (regression-based, annually updated); methodology e-book public; estimates "freely available online" per GLWC | Benchmark reports per country/region | freely published; **per-benchmark licence to verify** | The living-wage floor source. Reference Values are *modelled* (STAT-AX-9); full studies are measured benchmarks — the two evidence classes stay distinct. |
| **Living Wage Foundation (UK) / MIT Living Wage Calculator (US)** | National living-wage benchmarks | Published rates | verify | National benchmark precedents; modelled from cost baskets. |
| **WageIndicator** | Living wages + minimum wages + collective agreements, global coverage | Portal/API terms | verify coverage/licence | Broad fill where GLWC studies absent — verify terms before ingest. |
| **ILOSTAT earnings & minimum wage stats** | Mean/median earnings, minimum-wage series, working-poverty indicators | SDMX REST + bulk (verified §3) | ILO terms | Wage context + statutory-minima cross-check. |
| **ABS Employee Earnings & Hours / AWE / WPI** | AU wage levels, indices, earnings by industry/occupation (ANZSIC/ANZSCO keyed) | ABS Data API | CC-BY | AU wage benchmarks join to classifications. |
| **Fair Work Commission** | National minimum wage + award rates (AU statutory floor) | FWC decisions + published tables | public statutory | The AU legal floor — statutory instrument, cited with decision + commencement date. |
| **Numbeo / crowdsourced cost-of-living** | City-level price indices | portal; commercial terms | **restricted/commercial** | Flag: crowdsourced + commercial — register only; prefer ICP/benchmark sources. |

## 4. Input-output & footprint tables (economy → biosphere pressure)

| Source | What | Access | Licence | Notes |
|---|---|---|---|---|
| **OECD TiVA** | Trade in value added — origin of value in final demand, by industry×country | OECD SDMX API (open) | OECD terms | **The open MRIO-adjacent option** — use before any commercial table. |
| **Eora / Eora26** — worldmrio.com, eora.org | Global MRIO, 190 countries, ~226M transactions, 1990–2022+, environmental satellite accounts (GHG, water, land, labour, HANPP) | Download per licence | **`licensed` — commercial subscription; free academic tier exists** (verified). NOT open data | Rich footprint substrate but **cannot enter a Commons pack**; may inform eco-spec of what an open equivalent must cover. Australian-built (KGM/UNSW lineage). |
| **EXIOBASE** | Global MRIO w/ environmental extensions | Registered download (verify current terms) | Registration/licence terms | Alternative footprint model; verify licence at acquisition. |
| **WIOD** | World Input-Output Database | Downloads | verify | Smaller coverage, academic. |
| **Figaro** | Eurostat EU inter-country supply-use tables | Eurostat open | EU open | EU-scope only. |
| **National IO tables** (ABS input-output, part of national accounts) | Australian supply-use/IO tables | ABS releases | CC-BY | AU domestic substrate — open; combine with TiVA for global reach. |

**Design consequence (STAT-AX-7):** footprint/embodied-impact numbers are
*modelled claims* — every MRIO-derived figure carries `{model, version, year,
satellite account, method}` provenance and an epistemic band; never asserted as
measured fact.

## 5. Microdata & survey sources (restricted tier — provenance of method)

| Source | What | Access | Notes |
|---|---|---|---|
| **ABS TableBuilder / DataLab** | Census & survey microdata, user-tabulated | `registered`/`restricted` (paid TableBuilder tiers; DataLab is secure-environment) | **Do not ingest unit records.** Registering proves the access-tier boundary exists; aggregates only, with confidentiality marks intact. |
| **HILDA** (Melbourne Institute) | Longitudinal household panel | `restricted` (application + fees + conditions) | Same rule — metadata/methodology register entry only. |
| **ADA — Australian Data Archive** (ANU) | Social-science survey archive | `registered`/mixed | Catalogue entry; dataset-level terms. |
| **AIHW linked data assets** | Person-level linked health/welfare records | `restricted` (approval-gated) | The linkage-infrastructure precedent; we carry its *existence* in the register, not its rows. |
| **IPUMS** (international census microdata) | Harmonised census samples worldwide | `registered` (free registration, terms) | Register for method lineage only. |

**Why register restricted sources at all:** the access tier itself is data — it
evidences the microdata/macrodata boundary our confidentiality model must encode
(STAT-AX-5) and keeps the catalogue honest about what we deliberately do *not*
hold.

## 6. "Influences" — evidence & claims sources

The user asked for data about **influences** — how factors shape outcomes. The
honest modelling position (STAT-AX-7): influences are **claims with evidence
classes**, not observations. Sources are therefore the *claim-bearers*:

| Source class | Examples | Handling |
|---|---|---|
| Government/parliamentary reports | Closing the Gap annual reports, Productivity Commission reviews, Royal Commissions, AIHW welfare reports | Cited claims — source document as `prov:` entity, claim quins carry epistemic band + section/page provenance |
| Peer-reviewed syntheses | Systematic reviews/meta-analyses (Crossref/OpenAlex metadata) | Claims with stronger evidence class; contested pairs coexist (paraconsistent modality) |
| Official methodology docs | ABS conceptual frameworks, SDG metadata repository, OECD handbooks | The methodology entities statistics hang off (STAT-AX-6) |
| Indigenous data-governance instruments | **Maiam nayri Wingara Indigenous Data Sovereignty principles**, Lowitja Institute guidance | Governance documents — inform deontic conditions on statistics *about* peoples (STAT-AX-10) |

Correlation findings from our own corpus are *derived proposals* — they emit
`closeMatch`-style claim candidates for human ratification, never asserted facts
(alignment-rule precedent, ontology-stack §1).

## 7. Format inventory (observed across statistical sources)

| Format | Seen at | Handling path |
|---|---|---|
| SDMX-ML/-JSON/-CSV (+ DSDs/codelists) | ABS, OECD, ILO, UN SDG, Eurostat, IMF | **New ingest family needed** — structure artefacts → vocab layer; observations → Data Cube-shaped NQuins (G38) |
| RDF Data Cube (qb:) native | (some national stats portals publish QB; verify per source) | Direct ingest where present |
| xlsx/csv statistical tables | ABS products, ATO, JSA, AIHW | csv_mapper w/ per-product profile — header/codelist semantics need the DSD-aware path, not generic CSV mapping |
| Shapefile/GeoPackage boundaries | ASGS digital boundaries | → WKT/GeoSPARQL geometry, versioned `Border` entities |
| xlsx allocation/concordance files | ASGS allocation, correspondence tables | → `xkos:` correspondence quins as *proposals* |
| Bulk csv.gz + dictionary CSVs | ILOSTAT, FAOSTAT | Dictionary files are the codelist — ingest first, then observations |
| OData | NSW BioNet (already registered); some stats portals | OData fetcher → mapper |
| Microdata formats (SAS/Stata/SPSS, DDI-Codebook) | TableBuilder, HILDA, IPUMS | **Register only** — no unit-record ingest path; DDI metadata may be ingested as method documentation |

## 8. Sensitivity & sovereignty notes

- **Populations, not persons.** Statistical observations describe cohorts/areas;
  attaching them to individuals is both statistically wrong and prohibited by
  SOC-AX-8/GOV-SOV-7. The NQuin sensitivity context applies: microdata-derived
  anything stays Classified-tier.
- **Suppressed cells carry meaning.** Small-cell suppression and ABS
  perturbation are *published* confidentiality marks — they travel with the
  value as attribute data (STAT-AX-5); never back-filled by estimation and
  marked as observed.
- **Small-area estimates are modelled.** SA2-level estimates published by
  agencies (PHIDU, NATSEM-style) are modelled products — methodology entity +
  epistemic band mandatory.
- **Indigenous data sovereignty.** Statistics *about* Indigenous peoples sit
  under community-governed data principles (Maiam nayri Wingara; CARE).
  Statistical products for Indigenous geographies (ILOC/IARE/IREG) get the
  sovereignty review lane, same as custodial content (review-workflow.md).
- **Licence nuance verified 2026-10-12:** FAOSTAT's CC-BY-4.0 applies to current
  datasets; its 2019-era CC BY-NC-SA 3.0 IGO licence can still attach to older
  corpora — check per dataset. World Bank's CC-BY-4.0 covers WBG-produced
  datasets only; third-party datasets in the catalog keep origin terms. Eora is
  commercial — the footprint domain's open path runs through OECD TiVA + ABS IO.

## 9. Open questions / next discovery steps

1. ABS data API rate limits / fair-use posture for bulk dataflow harvest
   (no key required; still needs a polite-fetch policy + register-of-interest entry).
2. Whether ABS publishes any QB (RDF Data Cube) endpoints natively, or SDMX→QB
   mapping is always our step (G38).
3. ASGS Loc-I URI resolution endpoints — do the `ASGS_LOCI_URI_2021` identifiers
   still dereference (linked.fsdf.org.au / Loc-I infrastructure status)?
4. AIHW data.gov.au dataset-level licences (portal claims vs dataset fields).
5. Australian Data Archive / Indigenous-studies archives (AIATSIS statistical
   surveys) — register entries under sovereignty lane.
6. Which OECD datasets' CC status permits pack redistribution (TiVA terms at
   acquisition).
7. State-level environmental-economic accounts (e.g. catchment-scale land
   accounts) beyond the ABS national products.
8. UN Comtrade free-tier terms vs subscription — which tier permits derived-pack
   redistribution.
