# Map layers — the layer model for ecopunk packs

**Status:** draft v0.1 · scope: how spatial information is structured into
**independent, provenance-bearing layers** inside ecopunk volumes and packs,
and how downstream systems (game-demo's procedural environments, exploration
and encyclopedic interfaces, modelling instruments) consume them.

Grounded in: `data-sources/australia.md` (source candidates),
`tech/governance.md` (licence lanes, sensitivity, provenance chain),
`tech/pipeline.md` (S0–S9), `tech/engine-capabilities.md` §4a (adapters,
spatio-temporal index, scale continuum), and `game-demo` planning docs
03/29/30 + `docs/art/australian-ecology-data-contract.md` (the reference
consumer).

---

## 1. The core rule: a layer is an independent governed object

A "map" is never one dataset. It is a **composition of layers**, and each
layer is a separable governed object with its own:

| Property | Rule |
|---|---|
| **Identity** | stable layer ID (namespace + name + version) — versions, not overwrites |
| **Provenance** | source dataset, retrieval date/query/digest, source CRS + declared target CRS, transform chain, importer+version |
| **Licence** | per-layer `LicencePolicy` — **layers never silently inherit each other's licences** (game-demo rule, adopted) |
| **Temporal scope** | validity interval(s) and/or observation dates; historical layers are first-class |
| **Sensitivity** | class + generalisation history (e.g., SNES 1 km grid); raw coordinates stay in the authoring snapshot, not the shipped layer |
| **Coverage/extent** | declared bounds + resolution/scale denominator + "not valid below scale X" honesty notes |
| **Separation** | raw source vs derived interpretation are *different layers*; overlays (game scenario, user annotation, simulation output) never edit source layers |
| **Representation** | declared payload type — see §3 |

game-demo doc 29's discipline is adopted verbatim: *"an OSM
`amenity=community_centre` tag is evidence of a mapped tag, not proof the
building is open"* — source facts and interpreted roles are different objects.

## 2. Layer families (ecopunk registry)

| Family | Example layers | AU source candidates | Representation | Primary consumers |
|---|---|---|---|---|
| **geodetic** | ellipsoid/CRS frames, datums | GA geodesy | `geo:` graph + ReferenceFrame | everything (substrate) |
| **terrain** | DEM heightfields, slope/aspect/contours, bathymetry | ELVIS/GA DEM, SRTM | `.10d` FIEL / terrain tiles | scene, SDM, atlas |
| **hydro** | waterways, waterbodies, coastlines, drainage basins | GA hydrography, BOM AWRA | vector quins + mesh | scene, routes, ecology |
| **jurisdictional** | states, LGAs, postcodes, electorates — *versioned over time* | ABS, PSMA | vector quins (border kind-family) | atlas, governance views |
| **historical-borders** | superseded boundaries, former counties/colonies | state archives, NNTT | vector quins + validity intervals | historical interface |
| **ecological** | IBRA regions/subregions, subregions' descriptions | DCCEEW IBRA v7 (89/419) | vector quins | context layers, profiles |
| **vegetation** | NVIS major vegetation groups/subgroups, floristics | DCCEEW NVIS products | raster/vector quins | placement constraints |
| **protected** | CAPAD terrestrial/marine, IPAs | DCCEEW CAPAD | vector quins + rights overlays | governance + scene |
| **occurrence** | ALA/GBIF records (generalised), survey points | ALA, BioNet, WildNet | point quins + aggregates | profiles, encyclopedia |
| **distribution** | modelled ranges, expert maps (SPAT/SPRAT) | SPRAT, state lists | vector/raster + confidence | encyclopedia |
| **climate/env** | temperature/rainfall/evap grids, fire history | BOM AWAP, DEA, NAFI | `.10d` FIEL rasters | SDM, backdrop, phenology |
| **soil/land** | soil types, land capability, geology | ANSIS, GA | vector/raster | modelling |
| **imagery** | basemap rasters, satellite mosaics | **NASA GIBS** (in-engine chora), DEA | raster tiles (external refs or baked) | backdrop |
| **Country/sovereignty** | AIATSIS language areas, IPA estate, native-title determinations | AIATSIS, NNTT, DCCEEW | vector quins + rights/deontic overlays — *Country is relation, not parcel* | sovereignty views |
| **sociosphere** | settlements, services, project sites, population aggregates | ABS, civics.au outputs | point/vector quins | atlas, scenarios |
| **overlay** | user/game/simulation entities — *never edits source* | consumer-authored | separate graph space | consumers |
| **AR-anchored** | messages/data left *in-air* or bound to physical objects — anchored annotations, waypoints, community marks | user/community-authored | `PhysicalAnchor` records (UWB/VPS, Spatial Web Anchoring spec) + overlay graph space | AR surfaces, field use |

**AR elevation matters.** `domains/geospatial/ar_anchor.rs` implements
`PhysicalAnchor` (virtual XYZ ↔ physical UWB-beacon/VPS anchor, confidence,
active state) + `ArDeviceState` (camera pose + tracking confidence + anchors).
Anchored content is **3D by nature**: a message pinned "at this tree" resolves
to a *volume*, not a polygon footprint — so AR layers index against
`(x,y,z)` + `t`, and **elevation is a first-class query term**, not a display
hint. Anchors carry governance like any layer: who may place (canvas_rights /
location triggers), visibility classes (public/custodial/private messages),
persistence, and contest — a contested anchor is a record, not an edit.

## 3. Representation choices per payload type

| Payload | Canonical form | Notes |
|---|---|---|
| Point records (occurrences, sites) | quins + quadtree (x,y,t0,t1) index | sensitive coords → generalised + flag |
| Vector features (borders, roads, regions) | vector quins, source identity retained (`vector_scene` precedent) | geometry validity receipts at ingest |
| Gridded fields (DEM, climate, veg cover) | `.10d` FIEL sections (scalar/vector/tensor over cells) | DEM path exists (`dem_adapter`); general raster→FIEL is a registered gap |
| Meshes (terrain surface, scene geometry) | `.10d` mesh sections | terrain_pipeline: DEM→mesh→geodetic anchor |
| Raster imagery | `.10d` FIEL or external tile refs in pack manifest | licence per tile source |
| Symbolic layers (route graphs, place hierarchies) | pure quins | routes = first-class edges (game-demo contract) |
| Overlay/simulation | separate graph space + link quins | never mutates source layers |

## 4. Temporal rules (borders & everything else)

- Every layer version carries `valid_from`/`valid_to`; observation layers carry
  `observed_at`. A query "what was the IBRA boundary in 1998" is a *layer
  version query*, not a special case.
- Border kinds coexist (DOM-AX-3): jurisdictional, ecological, custodial,
  linguistic, cadastral — no kind outranks another; all carry authority +
  provenance.
- Historical layers are not archives — they're queryable layers whose validity
  interval has closed.

## 5. The game-demo consumption contract (reference downstream)

game-demo is the first known consumer and its requirements are already written
down — ecopunk layer packs should satisfy:

| game-demo need (doc 29/30, ecology contract) | ecopunk layer that feeds it |
|---|---|
| "versioned locality profile, not a single national species list" | `occurrence` + `distribution` + `ecological` layers packaged as *locality profiles* |
| "IBRA and NVIS provide context at their published product scales; neither is a substitute for local species observations" | separate `ecological`/`vegetation` context layers vs `occurrence` observation layers — scale honesty in metadata |
| "filter ALA downloads to the place boundary in-source; preserve DOI, query, per-record licence" | pipeline S2 acquisition contract; `asset_envelope` carries licence union |
| "independent layers with their own CRS, date, licence, transformations" | §1 — structural rule |
| "deterministic instance positions inside valid areas, respecting roads/buildings/water/slopes/exclusions" | `vegetation`/`landcover` layers as placement constraints + `terrain` for slope gates |
| "select gumtrees/shrubs/crops through biome/season profiles rather than generic green props" | species×biome×season profile layers → `.10d` flora asset selection |
| "sensitive coordinates never exposed in game" | generalised `occurrence` layers only |
| "source facts separate from game roles; overlays never edit geography" | `overlay` family is a distinct graph space |
| "declarative render profile cannot alter IDs/coordinates/route graph" | style is a consumer-side profile; layers are style-neutral |

Also relevant: doc 40 shows game-demo adopting the *same* modalities stack
(deontic for park-law rules, Allen for diurnal/seasonal timing, REA
socioeconomics for resource balances, A* over track graphs, units for telemetry) — ecopunk
layers carrying time/duration/units semantics will be directly consumable.

## 6. Pipeline implications (amends `pipeline.md`, to fold in)

- **S2 acquire** — the `geospatial/adapters` framework covers WMS/WFS (vector
  layers), CKAN (discovery), OPeNDAP (NetCDF climate), STAC (DEA imagery),
  DEM (Terrarium), SPARQL (TERN/Wikidata). ALA uses its download API (per
  ecology contract: filter at source, preserve DOI).
- **S5 normalise** — every source lands as a declared-representation layer;
  CRS transform + validity + licence + sensitivity recorded here.
- **S7 compile** — layer → quins (vector/point), `.10d` FIEL (fields), mesh
  (terrain); quadtree + spatio-temporal index entries emitted.
- **S8 pack** — packs declare their layer inventory in the manifest; layers are
  separately addressable entries (range-fetch per `.hmc` entry).
- **Review gate** — scale-honesty checks: no context layer presented as
  observation; no generalised layer presented as precise; no profile presented
  as complete coverage.

## 7. Open questions

1. Canonical layer-ID scheme: `eco:layer/<family>/<name>@<version>` vs
   digest-named? (decide with namespace strategy — governance §open).
2. Do overlay layers ever get *promoted* to source layers (community-verified
   records → governed source)? Stewardship/attestation path needed.
3. FIEL raster chunking for very large grids (continental climate) — tile
   scheme vs the 16-tile DEM fetch cap.
4. Layer dependency declaration (a `profile` depends on `occurrence@v` +
   `ibra@v7`) — manifest link vocabulary, probably reuse `asset_envelope`
   `derived_from`.
5. How Country/sovereignty layers encode *relations* rather than polygons —
   deferred to `ontology-stack.md` (SOC-AX-2).
