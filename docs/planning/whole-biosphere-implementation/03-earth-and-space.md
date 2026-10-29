# Earth and space implementation packets

**State:** research and handoff revision, 2026-10-28. This document assigns
future implementation work; it does not activate a build, data acquisition,
test run, export or implementation batch. It reconciles the governing
[implementation plan](../implementation-plan.md), [biosphere programme](../tech/biosphere-earth-and-anatomy-capabilities.md),
[outstanding outcomes](../tech/whole-biosphere-uplift-outstanding-work.md),
[spatial substrate](../tech/spatial-substrate.md) and
[Earth/space data register](../data-sources/earth-space.md).

Use the shared [corpus and inference contract](01-corpus-and-inference.md)
for invalid-value estimation, method selection, private/proprietary/classified
inputs, query cohorts, access control, uncertainty, provenance and output
sharing. The present packets specify Earth/space implications only; they must
not fork or weaken those rules. In particular, invalid source values remain
stored as received and marked invalid. A separate graph-function estimate may
be emitted only from an adequate comparable cohort, with declared user-selected
or user-defined method and complete cohort/frame/unit/time/context lineage. If
there is no adequate cohort, leave the result unresolved. Never estimate a
forecast storm index by taking a median across unlike events, or merge values
whose coordinate frames, physical units, time meanings or contexts differ.

## Evidence boundary and source review

The source register is a list of candidate acquisition surfaces, not an audit
that every endpoint, product, service agreement, entitlement or current
response was verified. No source acquisition was performed for this revision.
For every selected product, the implementation records product and schema
revision, source URL/service and request, observation/issue/retrieval times,
coverage, spatial/vertical frame, units, quality flags, uncertainty, rights,
attribution and response/content digest. Preserve an original source response
and any transformation as separate evidence. Observed measurements,
reanalysis, hindcasts, forecasts, model output and authored/illustrative
scenarios remain distinguishable in Q42; forecast issue time and valid time
must not be collapsed into one timestamp.

Read-only source review for this handoff was against Ecopunk `HEAD`
`c1ac16c33dceac75ab77893b9ecb5e59bf8d51fc` and QualiaDB `HEAD`
`3c0f3938d6d18dc7fc23447136716af65281b55a`, with QualiaDB working-tree status
recorded as dirty at review time. SHA-256 pins below therefore identify the
inspected bytes even if a checkout later changes. These are source observations,
not successful runtime executions.

Inspected QualiaDB scope:

- `qualia-geospatial/src/lib.rs` exports geodetic frame, GeoTIFF/M64, WFS,
  WMS slot streamer, corridors, manifests and Hilbert APIs. `src/frame.rs`
  defines WGS84/GDA2020/AHD/ellipsoidal identifiers, epoch, local-mm
  projection/unprojection, DGGS hash and conversion to Core's
  `SpatiotemporalPoint`. The local projection explicitly says it is an
  equirectangular approximation for footprints below 50 km; its hash helper
  must be qualified against the declared DGGS before being treated as a
  canonical cell identifier.
- `qualia-geospatial/src/geotiff.rs` includes a public elevation raster
  ingestion API. Its test bodies assert unsupported codecs fail without
  mutating output, raw M64 roundtrip/readability for a 1×1 fixture, and
  rejection of invalid dimensions. These are source test assertions only; they
  were not run in this task. The public legacy compiler is the one used by the
  inspected Ecopunk producer. The separate opt-in `src/raster_m64/writer.rs`
  `compile_raster_to_m64_v3` is a bounded caller-buffer writer for raw,
  elevation-only strata: it sets surface elevation, leaves other strata fields
  unpopulated and does not emit a general gridded FIEL. **Update (2026-10-28 Round 6):**
  `compile_raster_to_m64_v3` is wired into `poet/src/host/checkpoint.rs::CheckpointService::create_georaster` / `edit_georaster` and has been executed and verified across 6 reference `.hmc` packages (`1` chunk, `4,096` CRC-32C verified records, fresh-process reopen via `M64Reader`). Sourced multi-band/FIEL terrain and non-synthetic catchment DEM inputs remain open under `EAS-02`.
- `qualia-core-db/src/modalities/spatiotemporal_predicates.rs` owns the native
  spatiotemporal point/interval and NQuin compilation types; `domains/geospatial`
  owns spatial indexes. The chosen event/field/frame and validity semantics
  must be tested through these owners, not by inventing an Ecopunk parallel
  index. The frame-to-point helper currently passes Unix nanoseconds and local
  coordinates; this does not by itself specify leap-second handling, model
  time, forecast valid time or vertical transform provenance.
- `qualia-sim/src/space_weather.rs` exposes `evaluate_solar_cycle_phase` and
  `evaluate_space_weather_coupling`, with value structs for solar cycle, wind,
  storm and atmospheric electric circuit. Inspected unit-test bodies exercise
  expected seasonal/cycle trends, southward-IMF storm response, GCR cycle
  modulation and positive dry-lightning susceptibility. They are not
  independently calibrated against observation/reference datasets, and were
  not executed here. Formula comments and fixed defaults do not make outputs
  observations.
- `qualia-sim/src/hydro.rs` provides analytical Gerstner surface waves,
  buoyancy and locomotion/submersion mechanisms; it is not a catchment,
  groundwater, sediment or ocean-circulation validation result. `atmosphere.rs`
  and `geomechanics.rs` expose atmospheric and soil mechanics APIs and
  parameter presets; actual source-calibrated profiles and product-specific
  use remain unassessed. Reuse these owners where their declared mathematics
  matches a case and route missing mechanisms to their appropriate owners.
- `qualia-mcp/src/tools/sim_space_weather.rs` calls the `qualia-sim` functions.
  This establishes an existing wrapper caller, not the native Poet/QApp path.
  `qualia-core-db/src/poet_host/invoke/mod.rs` dispatches
  `ids::SIM_SPACE_WEATHER` to `studio_domains::sim_space_weather`; that
  function in `studio_domains.rs` defaults absent inputs, classifies at a Kp
  threshold and returns fixed `magnetopause_re=8.6` and `tec_tecu=31.4`. The
  inspected body does not call `qualia-sim`. Vibe catalogue/dispatch presence
  is not proof of execution or an integrated state owner.

An existing Ecopunk producer is
`crates/ecopunk-terrain/src/bin/compile_east_coast_substrate.rs`: it reads a
local elevation binary, but if it is absent it fills all 4096 samples with
`385_000` mm before calling `compile_raster_to_m64` and writing
`assets/terrain/talbingo_catchment.m64`. A format receipt and compile assertions
must not be presented as source-backed Talbingo terrain when this fallback
path supplied the samples. Remove the silent substitute from qualified source
production: require an identified source snapshot or produce an explicit
incomplete/unassessed result. Preserve prior execution receipts as evidence of
the exact run they describe and reconcile their input hashes before reuse.

### Selected file pins

SHA-256 (hex) of the inspected working bytes:

| Repository path | SHA-256 |
|---|---|
| QualiaDB `crates/qualia-geospatial/src/frame.rs` | `0063167be2a114a35e2fa968199a03c105e8ffcc97b9b7f2f025188ff57a9664` |
| QualiaDB `crates/qualia-geospatial/src/geotiff.rs` | `c21fffa0283234e42f7c8785bddda19b875c809b728d52c9da6d6912d55fad76` |
| QualiaDB `crates/qualia-geospatial/src/raster_m64/writer.rs` | `440d8e6db0cd3cf674a5113acff61b842c1cc295fdb31e793e64e7bdba93b3f0` |
| QualiaDB `crates/qualia-core-db/src/modalities/spatiotemporal_predicates.rs` | `9065265abc22cb0f349e11040781b69dd7f5b8220cb874f6564746837f350151` |
| QualiaDB `crates/qualia-sim/src/space_weather.rs` | `6b56d986c3aef6fe75ca04b97eedd25a0d14541dc212d3e922e1f1425d3096c7` |
| QualiaDB `crates/qualia-sim/src/hydro.rs` | `ccd5be171ccb266cb80b0bd4626bfadb84fdbe275ca1348072f63664b4a29d10` |
| QualiaDB `crates/qualia-sim/src/atmosphere.rs` | `2ab5d6cf5215cb411a89476e0b15cd50a333b4c8cb2eee40bc632190d0f2d723` |
| QualiaDB `crates/qualia-sim/src/geomechanics.rs` | `6527fe8055c85eed1eb3bdcde433784ba887defc9b458545a1986d93761b00af` |
| QualiaDB `crates/qualia-geospatial/src/lib.rs` | `fbd513c24a8197661fb435c25b94901fc76e019faf5e840e53618a8d59094bee` |
| QualiaDB `crates/qualia-geospatial/src/wfs.rs` | `19d8a154ae5eff1069c7d35f9bef4052093120788363e3c2ec1fb04ec5f2ab7c` |
| QualiaDB `crates/qualia-geospatial/src/wms_streamer.rs` | `b5120eb5da9bb1c60068cafbd02b184630232f8e739a3ad1a0c02b17a6912840` |
| QualiaDB `crates/qualia-geospatial/src/manifest.rs` | `fbf39d0121f2fead3ac3f8c4029f06b14395d48943b1b1dcc323b65c10fe28ac` |
| QualiaDB `crates/qualia-mcp/src/tools/sim_space_weather.rs` | `5a1cc7ed64de19ce8796ac4daf68e35078ed43743afab9807b161652285260d6` |
| QualiaDB `crates/qualia-core-db/src/poet_host/invoke/studio_domains.rs` | `4abca8d9d0b4f4463bdf59fc2bba299c0e81f2374c6b76b0ce124c736f89ab12` |
| QualiaDB `crates/qualia-core-db/src/poet_host/invoke/mod.rs` | `977b81ef5c52fc616561fe3dd980a7d2f4003ee621e6147f9c6ee2887c8ed56b` |
| Ecopunk `crates/ecopunk-terrain/src/bin/compile_east_coast_substrate.rs` | `b45d9771088749d734b3fc1c14a49ba83f365e7b348b2b7e8e83cb01e32c2cad` |

## Packet EAS-01 — shared spatial, temporal and units contract

**Order:** prerequisite for EAS-02 through EAS-04. **Owners:** QualiaDB
geospatial/Core graph and modalities own public frame, units, query and
serialization contracts; qualia-sim owns domain calculation semantics; Poet
native host supplies user-facing selection and review. Ecopunk supplies
source-specific profiles and acceptance cases. Do not create a second frame,
units, graph or time engine.

**Work:** pin the exact current frame/container APIs and establish a versioned
Earth/space observation, field and model metadata profile over Q42 and the
existing M64/FIEL/voxel/time-series owners. Specify horizontal CRS and epoch,
vertical datum/geoid transform, local frame extent and error bounds, axis order
and handedness, field cell registration, units/dimensions, uncertainty and
missing/invalid flags. Define UTC instants, source time scale, leap-second
table/version, interval closure, sampling cadence, observation time, issue
time, forecast valid time, retrieval time, model epoch/validity interval and
separate simulation/deep-time mappings. Retain source identifiers and qualified
mappings; CRS codes or identical coordinates do not prove equivalent
referents. Decide the graph-addressed DGGS cell vocabulary only after
qualifying the current `to_dggs_cell_hash` implementation against the named
standard and cross-layer test vectors. Hilbert remains byte/storage order.

**Seam and acceptance:** Core's graph/NQuin and modality APIs hold authoritative
observations, field manifests and frame/time relationships; M64/FIEL hold
bounded dense payloads referenced from that graph. Reuse the existing
`GeodeticFrame` and `SpatiotemporalPoint` seams, and make transforms explicit,
versioned operations. Positive cases cover surveyed coordinates at chunk
origin/edge, CRS and epoch transform roundtrip, AHD/ellipsoid conversion,
grid-cell centers/edges, UTC and source-specific time boundaries, and exact
unit conversion. Negative cases reject missing/unsupported CRS, datum/epoch
mixing, unknown units, reversed axes, invalid coordinates, out-of-validity
models and ambiguous instants before writing output. Compare transformed
coordinates and time conversions with independently published controls; set
numeric tolerances before execution and record actual measurements. Provenance
must include input/output digests, transforms and method revision. No tolerance
or successful run is claimed by this handoff.

## Packet EAS-02 — sourced terrain, geology and soil fields

**Dependencies:** EAS-01 and shared corpus contract. **Owners:**
`qualia-geospatial` for raster/vector ingest, frame-aware manifests and M64;
Core Q42/10D FIEL/schema owners for semantic features and dense-field payloads;
qualia-sim geomechanics/soil owners for calibrated physical operations; Poet
native authoring/presentation owns selection and review. Ecopunk owns selected
places, source rights/profile, scenario truth and complete output dossier.

**Work:** choose one real reference catchment/place from a source product whose
licence, coverage, native CRS/datum/epoch, scale and resolution are checked.
Keep measurements (DEM/LiDAR/imagery), feature claims (geological units,
faults, hydrography, boundaries) and modelled interpretations separate. Ingest
the feature attributes and their geometry into Q42; styled WMS/WMTS pixels may
be display derivatives but cannot supply lithology/unit truth. Map sourced
geologic class and soil/stratum measurements to graph-linked, typed layers with
uncertainty and intervals; do not infer a soil profile from elevation or fill
unpopulated M64 strata with presets. Reuse the raster M64 compiler for its
bounded documented elevation role. Route a general multi-band/time gridded
field-to-FIEL writer through the existing owning Core/10D schema rather than
adding a replacement terrain format. Store compiled display terrain and
scientific field separately with derivation lineage. Connect source-backed
soil state to the existing `SoilMechanicsProfile` and relevant native
`qualia-sim` geomechanics calls only after units and subject conditions are
mapped. Invalid source values may receive a separately identified estimate
under 01's user-selected/user-defined graph-function policy; no source claim is
overwritten, and incompatible soils, geological units, depths or epochs cannot
form one comparison cohort.

**Current integration seam:** public `compile_raster_to_m64` and the newer
opt-in v3 writer exist; the inspected Ecopunk producer calls the former and
substitutes a flat 385 m constant when its local binary is missing. First
qualify and correct that product source selection. `SpatialAssetManifest`, WFS
`GeologicUnitRecord`/`GeologicVectorLayer`, WMS tile-slot management and current
Poet spatial workbench are reuse points to trace in implementation; their
presence does not prove rights-cleared endpoint acquisition, complete vector
mapping, FIEL writing or human authoring flow. Integrate producer → Q42 feature
and M64/FIEL resource manifest → native Webizen/Poet spatial query/selection →
actual game-demo consumer using one exact source and output revision.

**Acceptance:** publish an input manifest with product/version and rights;
survey-control checks for CRS, vertical datum, units, pixel registration,
nodata masks and edge seams; independent control elevations/geology attributes;
soil-depth/class and property comparison with explicit uncertainty; feature
topology and source-to-derived graph link checks; M64/Q42/FIEL fresh-process
reopen and semantic query; safe failure for missing/malformed/unsupported data
without fabricated fallback; and a native human selects a point/polygon to see
the matching sourced field, feature, units, scale, confidence and transform.
Assess spatial science separately from visual quality. Record precision and
LOD truth (a coarse DEM cannot claim fine terrain). Do not mark a place
accepted until a geoscience/soil reviewer inspects the actual output. Existing
M64 codec and HMC checks in prior QA remain prior-snapshot evidence only; they
do not establish this sourced Earth case.

## Packet EAS-03 — freshwater, marine and atmosphere coupling

**Dependencies:** EAS-01 and EAS-02 for the selected catchment/coast. **Owners:**
QualiaDB graph/field/query and existing water-rendering owners; qualia-sim
`hydro.rs`, `atmosphere.rs`, and applicable fluid/physics owners; Poet's native
field, boundary-condition and timeline controls. Ecopunk supplies the
catchment, station/product selections, environmental dossier and scenarios.

**Work:** acquire source-qualified catchment/hydrography, water level/flow,
groundwater or soil-moisture, precipitation, temperature, wind, humidity,
radiation and, for coastal/marine scope, bathymetry/tide/current/salinity as
needed for the stated case. Distinguish observations, forecasts, reanalysis,
derived estimates and illustrative boundary conditions. Preserve temporal
coverage gaps and quality flags. Model data should be admitted into graph-linked
typed fields with physical units, acquisition frame and interval; do not infer
an unmeasured flow from a visual water plane. Existing analytical Gerstner
waves and buoyancy can serve visual/small-scale interactions where their
assumptions apply; they do not replace runoff, catchment routing, groundwater,
sediment or ocean circulation. Define each solver/approximation, domain,
boundary condition and coupling to soil, erosion, vegetation and light. Use
invalid-value estimates only with comparable stations/products and user
selected, provenance-recorded methods from the shared contract; estimate water
level or rainfall only within aligned basins/seasons/units/time window and
instrument semantics, otherwise unresolved.

**Seam and acceptance:** graph feature/event ownership stays in Q42; continuous
fields use existing FIEL/M64/voxel owners; `qualia-sim` consumes bounded
declared inputs and returns derived state with method/parameter lineage. Poet
exposes product time, missing periods, frame/units, boundary conditions and
scenario-vs-observation state, and invokes the same native operation. Exact
acceptance cases include conservation/residual checks appropriate to the
selected solver, holdout station/gauge comparisons over a declared period,
time-step and boundary sensitivity, dry/wet and upstream/downstream cases,
coast datum/tide alignment where used, negative tests for incompatible frames,
bad units, gaps and out-of-validity forecasts, and cancel/budget bounds. A
hydrology or marine reviewer compares actual reconstructed fields and outcomes
with source and reference evidence. A rendered water surface, atmospheric
shader or prior source-level test body is not validation.

## Packet EAS-04 — space weather and solar-terrestrial integration

**Dependencies:** EAS-01 and the shared corpus/method contract. **Owners:**
`qualia-sim::space_weather` owns calculations; Core Q42/temporal graph and
field owners hold observations/models/derived states; Core Poet host invoke
dispatch is the native operation seam; Vibe owns user-selected orchestration;
Poet/Webizen presents provenance and effect. Ecopunk selects a bounded
historical interval or live/replay use case and its independent references.

**Work:** adapt the existing `qualia-sim` solar-cycle and solar-wind functions
through `studio_domains::sim_space_weather` (dispatched at
`poet_host/invoke/mod.rs` for `Simulation.space_weather`) rather than writing a
second algorithm. Replace the inspected fixed `magnetopause_re` and `tec_tecu`
receipt values with typed results produced by the selected existing owner or
explicitly unassessed/unsupported results. The current sim function has
parameterized cycle curves, an estimated Kp/Dst/Newell response and speculative
GCR/global-electric-circuit/dry-lightning links. Preserve those calculations
as model output only after each quantity/relationship, parameter and scope is
validated; do not label these values observations. Qualify each output and any
physical effect separately before coupling into atmosphere, fire or ecology.

Create selected official product profiles from the Earth/space register only
after endpoint, terms, credential requirements and product metadata are
verified. Distinguish upstream solar-wind spacecraft measurements from
Earth-arrival estimates; retain GSM/other magnetic frame, units, cadence,
quality state, capture timestamp and propagation assumptions. Preserve NOAA
SWPC/BOM/GFZ or other source product identifiers and licence/attribution as
actually confirmed. Solar geometry/irradiance, plasma and IMF forcing,
geomagnetic indices, ionosphere/thermosphere and energetic-particle/radiation
are separate observables/models. Store event/observation captures and rolling
windows as time-qualified evidence; store WMM/IGRF and other deterministic
models with coefficient edition and validity interval. Forecast products need
issue and valid times, horizon, model cycle and forecast/reference comparison.
Do not use median estimates across different storms or cycles for a missing
forecast quantity. Averages/estimates are available only through the explicit
method and cohort semantics in 01.

**Acceptance:** invoke from the native Poet/Vibe path and show that inputs
reach the pinned `qualia-sim` owner; prove fixed constants no longer masquerade
as computed data. Replay an acquired, rights-cleared historical window against
independent source products with unit/frame/time conversion and declared
metrics/tolerances; include quiet, storm, northward/southward IMF, missing and
flagged measurements, stale/invalid model epoch, issue/valid-time boundary and
unsupported product cases. Include a deterministic input replay and output
derivation record, user-selected model/method, resource/cancel outcomes, and
source provenance through Q42 and native presentation. Science review must
cover each calibrated index/field and each claimed terrestrial coupling.
Biological effects (including drought/lightning) require a separate literature
and case evidence review. No host operation, source acquisition, replay or
scientific validation was executed by this handoff; current end-to-end status
remains unassessed.

## Shared release evidence for these packets

For each implementation assignment, include only the owning seam and disjoint
files, pin current source/consumer/relevant tests before editing, and record
whether claims are source-inspected, wired, executed or human-reviewed.
Implementation starts only under the governing plan's coordinator and current
activation state. Preserve the producer and consumer dependency direction;
native Poet/QApp operation must not require MCP, JavaScript, Node or V8. A
future evidence bundle binds each HMC/output/field to exact source, graph,
transform, model and consumer revisions. No endpoint list, API wrapper, source
test, rendered map, stale receipt or fixed host response alone closes an
acceptance case.
