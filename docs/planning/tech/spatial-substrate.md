# Spatial substrate — geo data kinds, representations & indexing

**Status:** draft v0.1 · **Date:** 2026-10-17
**Scope:** how spatial information of every kind — dense fields, discrete
features, situated observations, and the geodetic frame itself — is
represented, indexed, and joined across ecopunk's containers (`.q42` / `.m64` /
`.10d` / GPU texture arrays), and how the graph plane and the tensor plane stay
bound to each other.

Grounded in: [map-layers.md](map-layers.md) (the *governed-layer* model —
identity, licence, provenance, families; this doc is the *substrate* beneath
it), [../data-sources/australia.md](../data-sources/australia.md) (source
register), [../domain-model.md](../domain-model.md) §5 (spatio-temporal engine
fit), [ontology-stack.md](ontology-stack.md) (GeoSPARQL/LOD position),
`docs/plans/earth-terrain-architecture-and-spatial-formats.md` and
`earth-terrain-continuum-implementation-plan.md` (Phase 3 tasks 3.1–3.3).

---

## 1. The motivating observation, sharpened

The prompt for this doc: a DEM GeoTIFF is a *heavy* artefact — millions of
elevation samples — while most other GIS layers (vectors, classifications,
registries, observations) are naturally *graph-shaped* (GeoSPARQL / geo-RDF
ontologies). That observation is correct but incomplete in a useful way:
**"raster vs vector vs graph" are serialisations, not ontologies.** Underneath,
spatial claims exist in three distinct modes:

| Mode | What is being claimed | What has identity | Canonical question | Natural home |
|---|---|---|---|---|
| **Field** | "quantity `v` holds at position `p` for all `p` in a domain" | the *field* as a whole; cells have no identity | "what is the value here?" | dense arrays: `.m64` chunks, `.10d` FIEL, GPU textures |
| **Feature** | "entity `X` exists and occupies geometry `G`" | the *thing*; geometry is one attribute among many | "what is here / what contains this?" | `.q42` NQuins — `geo:Feature` + `geo:hasGeometry`/`geo:asWKT` |
| **Event** | "agent/instrument `O` observed result `R` at `p`, time `t`" | the *act of observation*; position and time are attributes | "what was seen where, when, by whom?" | `.q42` NQuins — `sosa:Observation` / DwC + point index |

A DEM cell is not a thing; a road's centreline is not a table of heights; an
occurrence record is not a point-cloud sample. Each mode has a *different*
identity model, and the identity model is what decides the representation —
not the file format it arrived in.

Plus a fourth thing that is neither field, feature, nor event:

| **Frame** | "position `p` *means* this geodetic place / custodial Country / DGGS cell" | the frame itself is the entity | "what does this coordinate *mean*?" | geodetic quins + container headers (`E64Header.spatial_origin`, `srid_geodetic_code`) |

### The governing rule

The architecture's master-constituent paradigm applies to the whole spatial
stack:

> **`.q42` is the semantic spine; every dense payload is a graph-addressed,
> provenance-bearing artefact.**

A `.m64` DEM tile or a GPU geology texture is a *compiled derivative* of an
ingestion act — its identity, coverage footprint, licence union, transform
chain, and temporal scope live in quins. Conversely the graph never tries to
*be* the raster: 48-byte NQuins describe fields and address them; they do not
hold per-cell values. Pixels for eyes, quins for truth — and every byte of
payload reachable from a governed identity.

## 2. The geodetic frame (what a coordinate *means*)

Before any payload: position is meaningless without a declared frame. The
substrate carries five cooperating frame elements:

| Element | Choice | Where recorded | Notes |
|---|---|---|---|
| Horizontal CRS (interchange) | `CRS84` / WGS84 lon-lat — OGC & GeoJSON default | `srid_geodetic_code` = 4326 | What WFS/WCS sources typically emit |
| Horizontal CRS (AU authoritative) | GDA2020 `EPSG:7844`; MGA zones `EPSG:78xx` for metre-accurate local math | `srid_geodetic_code` = 7844 / zone code | GDA2020 is **plate-fixed at epoch 2020.0**; WGS84 is dynamic — the pair diverges ~7 cm/yr. For ~m-scale DEM/draping this is tolerable; for cm-scale LiDAR and cadastre it is not — record the *epoch* of any high-precision coordinate |
| Vertical datum | AHD (`EPSG:5711`) for AU terrain; ellipsoidal `h` for raw GNSS/LiDAR | `E64Header.elevation_datum = [base_altitude_m, scale]`; strata records are mm-relative | AHD↔ellipsoid needs AUSGeoid2020 separation `N`; ~1–2 m of Australian tilt — silently mixing them is a *data bug*, not a precision issue |
| Chunk-local tangent frame | f32 metre offsets from `E64Header.spatial_origin` | every `E64PointRecord` / `.m64` chunk | Dual-scale pattern: geodetic truth in header+graph, fast local math in records |
| Discrete global cell | **AusPix DGGS** (rHEALPix; already on the AGLDWG register) — candidate canonical cell vocabulary; H3/S2 alternates | quin literals (`env:inCell`) + derived chunk keys | DGGS gives every place a *nameable entity* — the bridge between graph addressing and array indexing (§5, §6) |

Non-negotiable: **Hilbert order is a storage-order key, not a reference
system.** `M64ChunkDirectoryEntry.spatial_hilbert_index` exists for mmap
locality (1-D sort approximating 2-D adjacency); it answers "which bytes" — the
frame above answers "which place."

## 3. The data-kind register

Everything in `map-layers.md` §2 lands on this substrate. Columns:

- **Mode** — field / feature / event / frame (§1)
- **Wire form** — how the data actually arrives (§4)
- **Substrate form** — the canonical ecopunk representation
- **Index** — which spatial index answers position queries (§6)

| Kind | Mode | AU source candidates | Wire form | Substrate form | Index | Notes |
|---|---|---|---|---|---|---|
| Geodetic frame (datums, geoid, control marks, placenames) | frame | GA geodesy, Gazetteer, G-NAF | REST/CSV/WFS | frame quins + header fields | DGGS | The substrate's substrate — everything inherits it |
| **Terrain DEM** | field | GA ELVIS: SRTM 1″ (~30 m) DEM-S/DSM; 5 m LiDAR DEMs where available | WCS `GetCoverage` → GeoTIFF | `.m64` strata columns (`surface_elevation_mm`) + `.10d` FIEL heightfield | Hilbert chunks | Task 3.1. DEM cells are *field samples* — see open q1 on FIEL-vs-record landing |
| LiDAR point clouds | field (sampled) | ELVIS/state LiDAR | LAZ / COPC | `.m64` Mode 0x00 `E64PointRecord` (has ASPRS class + 8-band spectral already) | Hilbert chunks | Return-level detail (ground vs veg strata) feeds both terrain and canopy |
| Bathymetry | field | AusSeabed, GA marine | GeoTIFF/NetCDF | `.10d` FIEL | Hilbert | Continues the terrain field below 0 m AHD |
| Satellite imagery / radiometry | field | DEA products (WOFS, fractional cover, landcover), GIBS | WMS/WMTS + STAC + NetCDF | texture array (display) + `.10d` FIEL/`HyperspectralCube` (analysis) | tile matrix + Hilbert | Two products, two fates — §3.1 |
| **Surface geology** | feature (+ field drape) | GSNSW Seamless Geology, GA | WMS tiles + **WFS unit polygons w/ attributes** | vector quins (`env:GeologicUnit` via GeoSPARQL) + texture-array drape + strata-column LUT classes | DGGS + quadtree | Task 3.2. The semantic payload is the *vector*, not the pixels — §3.2 |
| Subsurface strata / regolith | field | GSNSW horizons, ANSIS soil landscapes | WFS + ANSIS RDF (AGLDWG soil ontology exists) | `.m64` Mode 0x02 `M64StrataColumnRecord` 6-layer stack + LUT indices | Hilbert | Unit-code → `BankedConstituentLut` mapping is the compile step |
| Hydrography | feature | Geofabric (AHGF: streams, catchments, aquifers), BOM AWRA | WFS/GeoJSON | vector quins → corridor splines + strata `surface_fluid_*` fields | DGGS + quadtree | Task 3.3. Creeks are both *features* (identity: this watercourse) and *field drivers* (flow depth/velocity in strata columns) |
| Transport corridors | feature | state roads, fire trails, OSM | WFS/GeoJSON | vector quins → Hermite spline conformer → heightfield modification | DGGS | Task 3.3. Corridor impressions are *derived edits* — provenance-linked, never baked into source terrain |
| Boundaries (all kinds) | feature | IBRA v7, NVIS, CAPAD, NNTT, AIATSIS, cadastre | WFS/SHP/GeoJSON | vector quins, DOM-AX-3 kind-family, validity intervals | quadtree | §3.3 — temporal versioning is structural |
| Occurrences | event | ALA/BioNet/WildNet | REST/DwC-A/OData | point quins + (x,y,t) quadtree; generalised per sensitivity | quadtree + DGGS | §3.4 — the SNES 1 km model operates at index resolution |
| Species distributions (modelled) | field/feature | SNES/SPRAT grids, expert maps | grids/WFS | raster quins or FIEL + confidence class | DGGS | Interpretation layers — never presented as observations |
| Climate/environmental grids | field | BOM AWAP/SILO, DEA time-series | NetCDF/WCS/THREDDS | `.10d` FIEL (x,y,t) | Hilbert + t | Phenology & SDM inputs; time-series fields are 3-D |
| Sensor/plot observations | event | TERN (EcoPlots, OzFlux, EcoAcoustics), IMOS | **SPARQL / SOSA RDF** | direct `.q42` ingest (TERN Ontology + GeoSPARQL + QUDT) | quadtree | The cleanest path: graph in, graph stored |
| Vegetation/landcover classes | feature+field | NVIS, DEA landcover | WFS + raster | vector quins + class rasters | DGGS | Placement constraints for flora; class-code → LUT like geology |
| Socioeconomic geographies | feature | ASGS (SA1–SA4, LGA, states) | WFS/SHP | vector quins + `qb:` observations keyed by region IRI | quadtree | statistics.md family — measures are events, geographies are features |
| AR-anchored content | event/feature | community-authored | `PhysicalAnchor` | (x,y,z,t) records + overlay graph | 3-D index | Elevation first-class: anchors resolve to *volumes* |
| Geological deep time | frame/feature | NSW Seamless Geology horizons | graph | Level-4 construct contexts + strata columns | DGGS + t | Formations carry ages in Ma — semantic `t`, not sim ticks |

### 3.1 Imagery has two fates — keep them apart

A satellite scene is simultaneously (a) a *backdrop picture* and (b) a
*measurement field*. The display path (WMTS tile → texture array → drape) and
the analytical path (radiance values → FIEL/`HyperspectralCube` → σ-axis
spectra) are different layers with different licences of honesty: the drape
may be pan-sharpened and styled; the field must stay in physical units.
Register both; never infer (b) from (a)'s pixels.

### 3.2 Geology is a vector with a picture, not a picture

GSNSW Seamless Geology's truth layer is the unit polygon + attribute table
(unit code, lithology, age, rank). The WMS tile is a *portrait* of it — styled,
generalised, projected for display. Task 3.2's "map geological unit codes to
`BankedConstituentLut` mineral indices" therefore **cannot come from WMS
pixels**: unit codes live in the WFS feature attributes. The correct shape is
dual-path ingestion: WMS → texture array (eyes); WFS → quins (truth) →
classification quins → LUT indices (physics). A third path — WCS or the
seamless-geology *raster* product — can rasterise unit codes per-cell for the
strata-column compile step.

### 3.3 Boundaries are versioned relations, not lines

Per DOM-AX-3 and map-layers §4: `(kind, geometry-version, validity-interval,
authority)`. IBRA 1998 ≠ IBRA 2026 — same entity family, different geometry
versions chained by `prov:wasRevisionOf`. Spatial index entries carry the
validity interval so `asOf` queries stay indexable. Country/custodial kinds
are relations, not parcels (SOC-AX-2): store the relation quins; any polygon
is a cartographic convenience layered on top.

### 3.4 Sensitivity is a resolution, not an omission

Occurrence/event coordinates exist at multiple precisions (raw → 1 km →
~10 km). The substrate's answer: the *same* record carries a generalisation
history and the published pack only contains the coarse form — SNES's own
model. Raw coordinates live in the authoring snapshot / Sanctuary lane only.
This is an index-level decision: publish the DGGS cell at coarser resolution,
not a different entity.

## 4. Protocol → representation contract

OGC service families deliver *different ontological things* — the ingest
adapter's first job is to not confuse them:

| Service / format | What it actually delivers | Lands as | Rule |
|---|---|---|---|
| **WCS** (`GetCoverage`) | the coverage itself — georeferenced values in source CRS | `.m64`/`.10d` FIEL field | Task 3.1's path. GeoTIFF is the container, not the semantics |
| **WMS / WMTS** (`GetMap`/`GetTile`) | a *rendered picture* — styled, projected, generalised | GPU texture array, display path only | Never parse truth from pixels; classification must come from the vector/coverage source |
| **WFS / OGC API Features** | the features — geometry + attribute table | vector quins + WKT literals | Task 3.3's path; geometry validity receipts at ingest |
| **SOSA/SSN, SensorThings API** | observations bound to feature-of-interest + sensor | event quins | TERN already ships this shape |
| **GeoSPARQL endpoint** | the graph itself | direct `.q42` ingest | QualiaDB executes GeoSPARQL natively — zero impedance |
| **STAC / CSW / DCAT** | catalogues of assets + metadata | `source_catalogue` descriptors | Discovery only; `Unverified` until licence+schema review |
| **NetCDF / Zarr / OPeNDAP / THREDDS** | n-dimensional arrays (x,y,z,t × bands) | `.10d` FIEL | Climate/ocean grids; registered engine gap |
| **LAZ / COPC** | point clouds w/ classes, returns, intensity | `.m64` Mode 0x00 | `E64PointRecord` already shaped for ASPRS classes |
| **GeoPackage / FlatGeobuf / GeoJSON** | vector exchange files | vector quins | Treat as WFS output that arrived by post |
| **PMTiles / MBTiles** | display tile pyramids | texture array / external refs | Same honesty rule as WMS |
| **3D Tiles / CityGML / glTF** | scene products | export *targets*, not ingest sources | Our derivative direction: constituents → scenes |

## 5. The bridge: graph ↔ tensor binding

The two planes must answer each other at query time without heap traversal of
RDF in hot loops. Three mechanisms do the binding:

1. **Manifest quins per artefact.** Every `.m64` file, `.10d` section, and
   texture-array layer emits a manifest entry at compile time: identity
   (`eco:SpatialAsset/<id>`), `prov:wasDerivedFrom` the source snapshot +
   query, coverage footprint (WKT bbox or polygon), SRID + epoch, licence
   union, validity/observation interval, Hilbert index range, and the chunk
   table location. The `.hmc` pack is then self-describing: enumerate quins →
   know every payload before touching a byte of it.
2. **Shared spatial join keys.** The same spatial key exists in both planes:
   `spatial_hilbert_index` (u64 in the chunk directory ↔ literal in quins) and
   DGGS cell IDs (`env:inCell` assertions ↔ derivable chunk keys). GeoSPARQL
   `sfWithin`/`RCC8` evaluate in the graph; results resolve to byte ranges.
3. **The LUT pattern, generalised.** `BankedConstituentLut` already compiles
   `.q42` material recipes into 8-bit GPU indices, paged per region via
   `lut_bank_epoch_id`. The same compile step maps *any* classification —
   geology unit codes, NVIS groups, soil orders — into indices carried by
   binary records. Bank 0 = global baseline; Bank 1 = this region's classes;
   Bank 2 = blends. Records store `u8`; the graph stores what `u8` means.

Worked flows:

- **Graph → tensor:** "features within this IBRA subregion" → GeoSPARQL in
  `.q42` → subregion geometry → DGGS cells / Hilbert range → manifest quins →
  `byte_offset` slices → mmap `.m64` chunks. No raster scan, no string parsing.
- **Tensor → graph:** user clicks a strata column → `chunk_uuid` + `lut_index`
  → manifest quin → `env:GeologicUnit` entity → its provenance, age,
  custodianship, related literature. The voxel *explains itself*.

## 6. Indexing strategy — four indexes, four jobs

| Index | Answers | Lives in | Notes |
|---|---|---|---|
| **Hilbert u64** | "which bytes are near these bytes" — mmap/Bao-slice locality | `.m64` chunk directory | Storage order only; 64×64-cell chunks per Task 3.1 |
| **Quadtree (x,y,t0,t1)** | "which events/points sit in this box+span" | `.q42` side index (map-layers §3) | Point/event plane; carries validity |
| **DGGS cells (AusPix)** | "which layers cover this *named place*" — cross-layer joins | graph (first-class entities) | The join vocabulary between field and feature planes; also the natural *generalisation* dial (§3.4) |
| **Tile matrix (z/x/y)** | "which rendered tile for this viewport" | texture array addressing / pack delivery | Display path; WMTS-compatible |

The `.10d` manifold's own `x,y,z,t` axes underlie all four — these indexes are
*addressing schemes over the same embedding*, not four worlds. Z is handled
per-mode: strata columns carry depth implicitly (layer thickness), LiDAR
carries `z`, AR anchors index 3-D directly, and the boundary/occurrence
planes are mostly z-agnostic (a species occurs in a cell, not at an altitude —
until habitat modelling needs it).

## 7. Time across the substrate

Each mode has a different temporal semantics — do not flatten them:

- **Fields:** observation/vintage time (a 2009 SRTM scene), and *data currency*
  (DEA time-series are (x,y,t) fields, not layers per epoch).
- **Features:** validity intervals (DOM-AX-2/3) — borders, geology editions,
  custodial determinations; queried `asOf`.
- **Events:** `resultTime`/occurrence date — fixed point observations;
  sensor series are event streams.
- **Constructs:** Level-4 historical/deep-time scenarios (1850 clearing,
  1974 flood, 300 Ma batholith) live in isolated `context` hashes with affine
  `T = T_base + κt` mappings — never asserted into q=0 ground truth.

## 8. Sensitivity, sovereignty & scale-honesty at the substrate

- **Generalisation is index resolution** (§3.4): Sanctuary volumes may hold
  raw; Commons packs ship coarse DGGS cells only. Fail closed at pack build.
- **Country is relation**: `soc:` custodial/custodianship quins are primary;
  rendered boundaries are conveniences with explicit authority + contest
  status.
- **Scale honesty** (map-layers review gate): a context layer declares "not
  valid below scale X"; a 30 m DEM never claims 5 m terrain; a modelled
  distribution never wears an observation's epistemic class.
- **Provenance is passive but mandatory**: `provenance_did_hash` on every
  binary record + manifest quins per artefact — attribution costs nothing at
  runtime and is never optional at compile time.

## 9. Phase 3 implications

Concrete consequences for the geospatial module the plan creates
(`crates/ecopunk-terrain/src/geospatial/`):

- **Task 3.1 `geotiff.rs`:** ingest WCS/GeoTIFF → partition 64×64-cell chunks
  → Hilbert sort → `.m64` + chunk directory. *Also required by §5:* emit
  manifest quins (coverage WKT, SRID=7844/7855-as-acquired, AHD note,
  retrieval digest). Decide FIEL-vs-record landing for cells (open q1).
- **Task 3.2 `wms_streamer.rs`:** WMS tiles → texture array is the *display*
  half. Pair it with a WFS/classification ingest path for GSNSW unit
  attributes so unit-code → `BankedConstituentLut` mapping is semantic, not
  pixel-scraped (§3.2). Tile-matrix addressing for the array.
- **Task 3.3 `corridor.rs`:** vectors arrive as quins (WFS→WKT→ingest), not
  raw GeoJSON structs in hot paths; Hermite cross-sections write into the
  field plane (heightfield deltas + strata `surface_fluid_*` for creeks) as
  *derived edits* with `prov` links to the source features.
- **Suggested module skeleton:** `geospatial/{mod,frame,hilbert,geotiff,
  wms_streamer,wfs,corridor,manifest,dggs}.rs` — `frame` (CRS/datum/epoch),
  `manifest` (bridge quins), `dggs` (cell vocabulary) are new vs the plan's
  three files; keep zero-heap hot-path rules (caller buffers, `Pod` records).
- **Register gaps upstream** per convention: general raster→FIEL ingest,
  WFS→quin adapter, DGGS indexing, GDA2020 epoch handling — these go in
  `engine-requirements.md`, not around.

## 10. Compression, codec hardware & the synthesis ladder

Video-codec lineage (MPEG-4/JPEG2000 → HEVC/AV1) maps onto the substrate in
three distinct ways — the skeleton/mesh/representation intuition translates
directly:

| Layer | Holo term | Contents | Compression economy |
|---|---|---|---|
| Program | *emitter instructions* | graph quins, vector features, LUT recipes, seeds | always lossless — small and authoritative |
| Forcefield | *the field you stand on* | heightfields, strata, LiDAR, moisture | lossless or bounded-error — physics reads truth |
| Projection | *what you see* | WMS drapes, styled tiles, imagery | freely lossy — already a derivative |

### 10.1 The wavelet lineage is already a terrain format

Wavelet multiresolution coding is *structurally identical* to a terrain LOD
pyramid: coarse approximation + per-level detail coefficients = clipmap base +
refinement rings. The GIS lineage is long (MrSID, ECW, JPEG2000/GMLJP2); its
current industrial form is **HTJ2K** (JPEG 2000 Part 15): integer or float
wavelet, lossless-or-lossy, resolution- and precinct-progressive, ~10× faster
than classic JPEG2000, adopted by national geospatial archives. For `.m64`
Hilbert chunks it offers the property video codecs lack: **random-access
decode at any resolution level** — the Bao-slice / range-fetch delivery model
made of codec pieces.

### 10.2 Hardware video silicon — what it honestly buys

Consumer GPUs ship dedicated decode blocks (NVDEC, Quick Sync, VideoToolbox,
VCN; browser path: WebCodecs → `copyExternalImageToTexture` → WebGPU texture).
Honest accounting:

- **Profiles are the gate.** Typical silicon decodes 4:2:0 8/10-bit
  H.264/HEVC/AV1. The profiles field data wants — 4:4:4, 12-bit, normative
  lossless — exist on some chips but are *not* universal. Hardware lossy
  decode for the projection layer: assume yes. Hardware lossless for
  forcefield payloads: verify per target; HTJ2K on CPU is the deterministic
  fallback.
- **Chroma subsampling is a band-alignment bug.** 4:2:0 corrupts multispectral
  alignment. Pragmatic packings: 10-bit luma plane = one u10 field channel
  decoded everywhere; chroma planes carry secondary low-res channels; or
  Terrain-RGB-style bit packing into a still-image codec.
- **GOP structure fights random access.** Video is sequential; chunks are
  random. Resolution: **the Hilbert chunk is the GOP boundary** — intra-coded
  per chunk (HEVC/AV1 all-intra), inter-coded only along a temporal axis that
  genuinely exists (§10.3).

### 10.3 Where video prediction is genuinely optimal

**(x,y,t) fields are video.** DEA WOFS, BOM climate series, fire progression:
temporal inter-frame prediction is exactly what these codecs were built for —
an AV1/HEVC GOP over a time-series decodes to texture-array slices at hardware
speed. The one place motion compensation is a *correct* model of the data.
Scalable extensions (H.264 SVC, HEVC SHVC) generalise the same idea across
resolution instead of time: base layer + predicted enhancement layers, which
is the codec-native form of the LOD pyramid — but SHVC decoder support is
uneven enough that HTJ2K stays the safer field transport.

### 10.4 Vector = model-based coding for space

The skeleton→mesh→paint analogy has direct codec precedent: MPEG-4's
model-based coding (Face/Body Animation) and AV1's normative film-grain
synthesis — *ship parameters, synthesize at decode*. Applied geospatially:

- **Resolution independence.** A Hermite roadbed spline is ~KB evaluated at
  any zoom; the rasterized equivalent is MB. Task 3.3's conformer already is
  this: store the vector, evaluate into the field at load.
- **SDF pattern.** Coarse field + analytic vector boundary → crisp edges at
  any scale (coastlines, geology contacts) without megapixel truth.
- **Detail synthesis at the edge.** Geology polygon + unit-class LUT +
  per-class noise seed → synthesized strata/soil texture: the same
  "compress the program, regenerate the pixels" economy as grain synthesis,
  and the same master-constituent rule the stack already runs on.
- **Geometry as image.** Geometry-image encoding (mesh → parametric image) →
  image/video codecs can compress `MSH0` patches for transport; GPU
  tessellation expands on arrival.

### 10.5 Rules of engagement

1. `M64ChunkDirectoryEntry.compression_flags` gains codec kinds beyond
   `0=mmap / 1=LZ4 / 2=Bao` — e.g. HTJ2K-lossless (truth fields), HEVC/AV1
   intra (drapes), AVIF (still tiles). The chunk stays the random-access unit.
2. Forcefield payloads: integer wavelet lossless, or bounded error declared in
   the manifest (`env:maxErrorM`); strata records' mm quantization is already
   the *sanctioned* lossy step — a codec must not add silent error on top.
3. Projection payloads: any lossy codec the display path supports.
4. Encode once at pack build (external compute lane, pipeline §7A); decode at
   mount/stream — encode is the expensive side and it is offline.
5. Record codec + parameters in the artefact manifest quins — compression is
   a transform in the provenance chain like any other.

## 11. Open questions

1. **DEM landing form** — strata-column `surface_elevation_mm` vs `.10d`
   `FIEL` heightfield vs both? FIEL is the analytic-surface form (watertight
   eval, slope/aspect); strata columns carry the fluid+subsurface stack.
   Likely: FIEL for the surface, strata for what's beneath — needs a compile
   rule, not a choice per cell.
2. **Canonical graph cell key** — AusPix DGGS vs raw Hilbert vs both
   (DGGS for graph joins, Hilbert for byte order)? GeoSPARQL 1.1's DGGS
   relations favour AusPix as the named entity; Hilbert stays internal.
3. **GDA2020 epoch discipline** — cm-precision LiDAR needs coordinate epochs;
   30 m DEM doesn't. Where does `epoch` live — per-layer manifest only, or
   per-chunk?
4. **Vertical datum of strata records** — `surface_elevation_mm` relative to
   AHD vs ellipsoid: declare per-layer manifest; is one canonical internal
   choice wanted (suggest: AHD for AU packs, ellipsoid only at ingest edge)?
5. **WFS adapter home** — plan assigns "generic raster/point importers" to
   qualiaDB upstream; does a GeoSPARQL/WFS→quin adapter also belong upstream,
   with ecopunk carrying only AU-source profiles?
6. **Imagery analysis path** — when does a satellite layer justify
   `HyperspectralCube`/FIEL ingest (σ-axis truth) vs staying a display drape?
   Needs a per-product policy (WOFS/fractional-cover: yes; basemap: no).
7. **Codec × silicon matrix** — which profiles have normative-lossless
   *hardware* decode paths on which targets (NVDEC/QSV/VideoToolbox/WebCodecs)?
   Where does HTJ2K-on-CPU become the streaming bottleneck? Needs a
   verification pass before `compression_flags` values are frozen.
8. **Time-series transport** — AV1/HEVC GOPs vs per-epoch FIEL sections for
   (x,y,t) fields: delivery efficiency vs analysis fidelity; probably both
   (GOP for streaming, FIEL export on demand) — decide per product family.
