# Data sources — Australia (discovery register)

**Status:** discovery draft v0.1 · **Date checked:** 2026-10-12 · **Scope lead:** Australia-first, per project decision

Australia is the pilot region: it has unusually strong open biodiversity infrastructure
(NCRIS-funded ALA/TERN/IMOS), authoritative national taxonomies (NSL), an active government
linked-data programme (AGLDWG / `linked.data.gov.au`), and state/territory atlases that expose
real APIs. This file records what was actually found; nothing here is assumed.

**Legend** — Access: `open` | `registered` | `request` | `restricted`.
Licence: recorded as published; **verify at acquisition time** — licences change and
record-level licences often differ from portal-level statements.

---

## 1. National aggregators

| Source | What | Access & format | Licence | ecopunk notes |
|---|---|---|---|---|
| **Atlas of Living Australia (ALA)** — ala.org.au | Australia's GBIF node; 100M+ occurrence records, 153k+ named species. Services: Biocache (occurrences), BIE (species pages), Collectory (datasets/institutions), Spatial Portal (layers + analysis), Lists, Images, Profiles ("Flora of Australia"), BioCollect (citizen science), Sensitive Data Service | REST/JSON APIs (`api.ala.org.au`, `biocache-ws.ala.org.au`), OpenAPI docs at docs.ala.org.au; bulk CSV (Full DwC / "ALA Legacy" incl. QA assertions + spatial layer joins); DwC-A per-dataset; `galah` R/Python clients; download DOIs; **protected APIs need JWT** (sensitive data) | Site CC-BY 3.0 AU; **record-level licences vary** (dcterms:license per record — CC0/CC-BY/CC-BY-NC present) | Primary spine for AU occurrences. Sensitive-species coordinates are obfuscated by ALA (matches our sensitivity model). Image service carries per-image licence + attribution. Download DOIs must be preserved as provenance. |
| **GBIF** — gbif.org | Global mirror of AU records (ALA is a node; iNaturalist AU flows through GBIF) | REST API, DwC-A downloads, DOI per download | CC0/CC-BY/CC-BY-NC per record | Use ALA for AU-specific fields; GBIF for cross-border/global coverage. |
| **data.gov.au** | Federal CKAN portal aggregating Commonwealth + state datasets (incl. DCCEEW, GA, BOM, IMOS metadata) | CKAN API (`data.gov.au/data/api/3`), resource files: CSV/GeoJSON/SHP/GeoTIFF/WMS | Mostly CC-BY 4.0 (per-dataset field) | Catalogue-of-catalogues; harvest DCAT metadata for discovery even when payload lives elsewhere. |
| **NationalMap / Terria** | Federation of AU geospatial web services | WMS/WFS/ESRI REST federation | varies | Useful for layer enumeration; not a primary store. |

## 2. Federal science & environment (DCCEEW, CSIRO, GA, BOM)

| Source | What | Access & format | Licence | ecopunk notes |
|---|---|---|---|---|
| **DCCEEW Find Environmental Data** | IBRA v7 (89 bioregions / 419 subregions), CAPAD (protected areas terrestrial+marine), NVIS (major vegetation groups/subgroups, extant + pre-1750), EPBC Referrals spatial, heritage (AHDb) | `gis.environment.gov.au` ArcGIS REST/MapServer + WMS; downloadable GIS packages (SHP/GDB) | Open (verify per dataset; historically CC-BY) | IBRA/NVIS are our **ecoregion/vegetation scaffold** (game-demo already uses them as context layers). |
| **SPRAT + SNES** | Species Profile & Threats Database (EPBC listed species profiles, conservation advices, recovery plans, threatening processes); SNES = modelled distributions for listed species | SPRAT report tool (HTML/CSV export); SNES GIS grids | Open public products | **SNES grids are pre-generalised to 1 km (~10 km for sensitive species)** — Australia's own sensitivity model; mirror this granularity, never claim finer precision. |
| **National Species List (NSL / auNSL)** — biodiversity.org.au/nsl | Authoritative AU taxonomies: **AFD** (fauna, ~130,056 spp in 4,536 families — CSV bulk + LSID GUIDs), **APC/APNI** (vascular plants), AusFungi, AusMoss/Bryophytes, AusLichen, Algae lists | CSV bulk downloads per taxon group; REST-ish taxon endpoints (`taxa/{id}/{resource}`); ALA's name index consumes it | Open (ABRS/DCCEEW; verify reuse terms) | **Our AU taxonomic backbone.** Keep NSL LSIDs as canonical identifiers; ALA merges NSL→NZOR→CoL in that priority order. A "National Species List Data Model" ontology is already registered on the AGLDWG catalogue — adopt rather than re-model. |
| **Geoscience Australia** | DEM/elevation (SRTM-derived), coastlines, hydrology (Geofabric), Sentinel/Landsat via DEA | ELVIS download, WMS/WCS/THREDDS | CC-BY 4.0 | Terrain + hydrology context layers. |
| **Digital Atlas of Australia** — `digital.atlas.gov.au/search` | GA's national foundation spatial catalogue — searchable discovery across admin boundaries, elevation, transport, imagery, placenames, landcover etc. | web catalogue + linked services (verify per-dataset endpoints/licences) | per-dataset (mostly CC-BY 4.0; verify) | *Added 2026-10-12 per user.* Discovery hub for the geodetic/terrain/hydro/jurisdictional layer families — register its datasets through the same descriptor pipeline. |
| **Digital Earth Australia (DEA)** | Analysis-ready satellite products: fractional cover, Water Observations from Space, land cover, coastal (DEA Coastlines) | OGC WMS/WCS, STAC API (`explorer.dea.ga.gov.au/stac`), S3 open bucket (`dea-public-data`), Jupyter sandbox | CC-BY 4.0 | Environmental time-series layers for macro-scale views; STAC JSON-LD items link directly to Cloud-Optimised GeoTIFFs (COGs). |
| **BOM** — bom.gov.au | Climate grids: SILO daily interpolations, AWAP, ACORN-SAT station series | SILO API (registered key), FTP/grid files, NetCDF | SILO: registered access; check redistribution | Climate layers for species×climate modelling. Verify licence before repackaging. |
| **data.csiro.au** | CSIRO Data Access Portal — collections incl. plant/entomology datasets, soils | REST API, files | Mostly CC-BY | ANIC insect collection data flows to ALA/OZCAM anyway; portal has extras. |

## 2A. 3D Terrain, Bare-Earth Elevation, LiDAR & Canopy Height Payloads (NSW & National)

Because serialising millions of $X, Y, Z$ LiDAR vertices or 1-metre elevation grid cells directly into RDF triples creates massive bloat, Australian spatial agencies separate the **Semantic Control Plane** (GeoSPARQL / DCAT / STAC JSON-LD / AusPIX DGGS) from the **Binary Elevation Payload Plane** (Cloud-Optimised GeoTIFFs and Cloud-Optimised Point Clouds fetched via HTTP `Range: bytes=...` requests):

| Source / Service | Semantic / Catalogue Layer | Binary Payload & Resolution | Datum / CRS | Licence | ecopunk substrate role |
|---|---|---|---|---|---|
| **ELVIS (Elevation and Depth — FSDF)** — `elevation.fsdf.org.au` | ISO 19115 / DCAT-AP metadata profiles in `data.gov.au` and Geoscience Australia's persistent catalogue (`pid.geoscience.gov.au`) | **1 m to 5 m Bare-Earth LiDAR DEMs** (GeoTIFF / COG) and **Classified LAS/LAZ / COPC Point Clouds** (ASPRS classes: `2 = Ground`, `3/4/5 = Low/Med/High Vegetation`, `6 = Building`, `9 = Water`) | GDA2020 (`EPSG:7844` / MGA55 `EPSG:7855`, MGA56 `EPSG:7856`), AHD (`EPSG:5711`) | CC-BY 4.0 | Primary high-resolution bare-earth terrain mesh (`.m64` `surface_elevation_mm`) and rock-outcrop / canopy return source (`.m64` Mode `0x00` `E64PointRecord`) across NSW corridors. |
| **NSW Spatial Services** — `portal.spatial.nsw.gov.au` / `data.nsw.gov.au` | DCAT-compliant dataset descriptions linking to OGC WCS/WFS and S3-hosted tile indexes | **1 m Cloud-Optimised GeoTIFF (COG) DEMs** in $1\text{ km} \times 1\text{ km}$ tiles, plus statewide **NSW 2 m Contours & 5 m DEM** | GDA2020 / AHD (`EPSG:5711`) | CC-BY 4.0 | Direct HTTP byte-range (`Range: bytes=...`) streaming of $512 \times 512$ elevation windows per AusPIX DGGS cell without downloading multi-gigabyte regional archives. |
| **FABDEM (Forest And Buildings removed Copernicus DEM)** — Bristol / Planetary Computer STAC | STAC JSON-LD (`stac:Item` / `stac:hasAsset`) | **~30 m Bare-Earth DEM COG** with tree-canopy and building height artifacts machine-learning stripped | WGS84 / GDA2020 aligned | CC-BY-NC-SA 4.0 / verify commercial vs commons lane | Essential bare-earth fallback for remote NSW backcountry where airborne LiDAR has not yet been flown, preventing artificial 20 m "hills" under dense *Eucalyptus* canopy. |
| **CSIRO / GA 1-Second SRTM DEM-H & DEM-S** | Linked via GA persistent URI catalogue (`pid.geoscience.gov.au/dataset/ga/72759`) | **~30 m Hydrologically Enforced DEM (DEM-H)** and Smoothed DEM (DEM-S) in COG/GeoTIFF | GDA94/GDA2020, AHD | CC-BY 4.0 | Permissive-commons (`CC-BY 4.0`) bare-earth and drainage-enforced fallback guaranteeing monotonically descending river networks when merged with NSW Geofabric hydrology. |
| **Fused NASA GEDI + ICESat-2 / Meta-WRI Global Canopy Height** | STAC API (Microsoft Planetary Computer / DEA / TERN Landscapes) | **1 m to 30 m Canopy Height Model (CHM) COGs** ($\text{CHM} = \text{DSM} - \text{DEM}$) | WGS84 / GDA2020 | CC-BY 4.0 / Open | Provides real-world vertical vegetation height ($H_{\text{canopy}}$ in metres) per grid cell to parameterise *Corymbia* / *Eucalyptus* allometric stage and crown height in `.10d` `PROC1`/`EONT1`. |
| **SLGA (Soil and Landscape Grid of Australia)** — CSIRO / TERN | ANSIS RDF ontology + STAC / WCS | **90 m (and 30 m v2) 6-depth-layer soil attribute COGs** (`0–5 cm`, `5–15 cm`, `15–30 cm`, `30–60 cm`, `60–100 cm`, `100–200 cm`: bulk density, clay/silt/sand %, organic carbon, pH, available water capacity, depth to regolith) | GDA2020 | CC-BY 4.0 | Compiles directly into `.m64` Mode `0x02` `M64StrataColumnRecord` 6-layer subterranean stacks and `qualia-sim::geomechanics` Bekker/Mohr-Coulomb soil physics parameters. |

## 2B. Linked-Data Spatial Topology & Bush Infrastructure RDF Services (NSW)

To populate ground-level open-world bush infrastructure (NPWS and Forestry Corporation campsites, unsealed 4WD fire trails, locked gates, river crossings, gullies, and mountain peaks) without manual placement:

| Service / Endpoint | Ontology / Vocabulary | Content & Spatial Coverage | Access & Format | Licence | ecopunk substrate role |
|---|---|---|---|---|---|
| **AusPIX DGGS** — `linked.data.gov.au/def/auspix` & `pid.geoscience.gov.au/dggs/ausPIX/` | AusPIX Ontology (`apo:Cell`, `apo:CellAggregate`, `apo:hasNeighbourUp/Down/Left/Right`, `geo:hasGeometry`) | Hierarchical rHEALPix Discrete Global Grid System dividing Australia into deterministic resolution cells (Level 9 $\approx 600\text{ m}$, Level 10 $\approx 200\text{ m}$, Level 11 $\approx 65\text{ m}$) | Linked Data API / Prez RDF + deterministic mathematical coordinate-to-cell projection | CC-BY 4.0 | **Canonical spatial join key** bridging `.q42` semantic queries and `.m64` Hilbert terrain chunks (`spatial-substrate.md` §2, §6). |
| **Australian Hydrological Geospatial Fabric (Geofabric)** — `linked.data.gov.au/dataset/geofabric` | AHGF Ontology + GeoSPARQL (`geo:wktLiteral`) | Authoritative surface hydrology, perennial/ephemeral river networks, waterbodies, billabongs, and catchment boundaries across NSW | Prez Linked Data API / WFS / GeoSPARQL | CC-BY 4.0 | Carves hydrologically accurate riverbeds into `.m64` terrain via `HermiteCorridorSegment`s and drives `qualia-sim::hydro` 3D Archimedes buoyancy & shallow-water flow. |
| **Composite Gazetteer of Australia (Place Names RDF)** — `linked.data.gov.au/def/placenames` | Place Names Ontology + GeoSPARQL | Connects NSW Geographical Names Board (GNB) entities (bush reserves, state forests, gullies, passes, peaks, creeks) to formal URIs, jurisdictional provenance, and WKT geometries | Prez Linked Data API / SPARQL | CC-BY 4.0 | Anchors named places, study sites, and topographic landmarks in `.q42`. |
| **QLever / `osm2rdf` OpenStreetMap Australia Extract** — `qlever.cs.uni-freiburg.de/osm-planet` | `osmkey:` / `osmtag:` + GeoSPARQL (`geo:hasGeometry` / `geo:asWKT`) | Weekly RDF Turtle extract of OpenStreetMap Australia (`aus.osm.ttl.bz2`, ~2.5B triples) + live GeoSPARQL endpoint. Maps `tourism=camp_site`, `highway=track`, `surface=unpaved`, `tracktype=grade1..5`, `barrier=gate`, `waterway=ford`, `operator=NSW National Parks and Wildlife Service` to exact WKT linestrings and polygons | Public GeoSPARQL endpoint + bulk `.ttl.bz2` ingest into `.q42` | ODbL 1.0 (attribute OpenStreetMap contributors; keep ODbL layer provenance distinct in manifest) | Drives procedural 4WD fire-trail banking (`conform_terrain_height_to_corridor`), bush campsite layouts, gates, and drivable multi-kilometre road networks across NSW. |
| **Wikidata Federated National Parks & Reserves** — `query.wikidata.org` | Wikidata (`wdt:P31 wd:Q46169`, `wdt:P2044` elevation, `wdt:P625` coords, OSM relation links) | NSW National Parks, Nature Reserves, State Forests, and World Heritage Areas | SPARQL 1.1 Federation (`SERVICE <https://query.wikidata.org/sparql>`) | CC0 1.0 | Cross-links NPWS reserves with OSM nodes, ALA species checklists, and Wikipedia/Commons media. |

## 3. NCRIS observing infrastructure (TERN / IMOS / Bioplatforms)

| Source | What | Access & format | Licence | ecopunk notes |
|---|---|---|---|---|
| **TERN** — tern.org.au | Terrestrial Ecosystem Research Network: AusPlots/EcoPlots (plot surveys), SuperSites, OzFlux towers, EcoImages (photopoints, leaf/flora images), EcoAcoustics (audio) | portal.tern.org.au discovery; per-app portals (ecoplots/ecoimages/ecoacoustics.tern.org.au); **RDF via TERN Ontology + GraphDB SPARQL** (graphdb.tern.org.au); SHaRED submission tooling | TERN data licence (CC-BY-style; verify per product) | **The best-modeled AU ecology data** — already OWL/SHACL RDF aligned to SOSA/GeoSPARQL/QUDT. Direct pipeline to `.q42`. EcoImages/EcoAcoustics are media sources for asset packs. |
| **TERN Linked Data** — linkeddata.tern.org.au | TERN Ontology (OWL + SHACL profiles) + controlled vocabularies (observable properties, feature types, instruments, methods) | Turtle source at github.com/ternaustralia/ontology_tern; w3id.org/tern namespace | Open (CC-BY) | **Core LOD ontology for AU ecology.** Extends SOSA/SSN/SSN-ext + PROV-O + GeoSPARQL + QUDT. Its SHACL shapes give us free validation profiles for survey data. |
| **IMOS / AODN** — imos.org.au, portal.aodn.org.au | Integrated Marine Observing System: NRS time-series, AUV/glider, moorings, animal telemetry (ATN/IMOS ATF), Reef monitoring | AODN Portal, THREDDS (NetCDF/OPeNDAP), S3 direct, WMS | IMOS licence (CC-BY attribution; verify) | Marine counterpart to TERN — macro marine layers + animal tracking for the micro/telemetry collections. |
| **Australian Microbiome Initiative** — australianmicrobiome.com | National microbial sequence datasets: BASE (soils), Marine Microbes, plus IMOS MMI | Bioplatforms Australia Data Portal (registered), amplicon/metagenome files, contextual metadata | Registered access (terms apply) | **Microscopic end of the scale ladder** — soil/marine microbiome occurrence + sequence data. Access terms need review (registration, attribution). |
| **OzFlux / TERN flux** | Eddy-covariance flux tower time-series | via TERN portal / OzFlux | CC-BY-ish | Ecosystem-function time series. |

## 4. State & territory biodiversity atlases

| Jurisdiction | System | Access & format | Notes |
|---|---|---|---|
| **Commonwealth/ALA** | (above) | — | Aggregation layer |
| **NSW** | BioNet (Atlas + Species Sightings, Systematic Flora Survey, Vegetation Classification, NSW Landscapes, Threatened Biodiversity, Species Names) + SEED portal (data.seed.nsw.gov.au) | **OData API** (api token; tiered access incl. sensitive), Power Query templates, BioNet Atlas UI exports | Tiered access levels are a worked example of sensitivity classes in the wild. SEED is CKAN. |
| **QLD** | WildNet (sightings, species profiles, lists, multimedia) + DES open data | **Swagger REST API** (wildnet-pub.science-data.qld.gov.au): species/sightings/species-list downloads, CSV; Biomaps/WetlandMaps/Queensland Globe views; "published" record subset on data.qld.gov.au | API email-registration expected for long-term use. Sensitive list exists separately. |
| **VIC** | Victorian Biodiversity Atlas (VBA) via NatureKit; Data Vic | VBA exports/registered access; FIS/flora-fauna spatial layers WFS | VBA is the authoritative VIC occurrence set; verify public vs registered tiers. **To verify.** |
| **WA** | DBCA NatureMap / NatureMapr; SLIP geospatial; data.wa.gov.au | NatureMap registered downloads; SLIP services | **To verify.** |
| **SA** | BDBSA (Biological Databases of South Australia) via NatureMaps/Enviro Data SA + data.sa.gov.au | Exports + spatial services | **BDBSA already has a TERN-Ontology RDF demo** (ternaustralia/bdbsa-data-demo + GraphDB endpoint) and BDBSA vocabularies on the AGLDWG register — the cleanest state→RDF precedent. |
| **TAS** | Natural Values Atlas (NVA/DPIPWE) + the LIST (data.tas.gov.au spatial) | NVA reports/registered; LIST services | **To verify.** |
| **NT** | NRETAS / NR Maps + data.nt.gov.au | Map services, reports | **To verify.** |
| **ACT** | ACTmapi + data.act.gov.au | Spatial services + CKAN | **To verify.** |

## 5. Collections, names & traits

| Source | What | Access & format | Licence | ecopunk notes |
|---|---|---|---|---|
| **AVH (Australasian Virtual Herbarium)** — via ALA/OZCAM | Herbarium specimen records (CANB, MEL, NSW, BRI, PERTH, AD, HO, DNA…) | Through ALA Biocache + AVH portal; DwC | per record | Specimen records + images. |
| **OZCAM** — via ALA | Zoological collections of AU museums | Through ALA Biocache | per record | Fauna specimen spine. |
| **AusTraits** — austraits.org | Harmonised AU plant traits: 500+ traits, 30k+ taxa, 400+ sources | Zenodo releases: CSV/parquet/Rds + dictionary; R package; app.austraits.org portal | **CC-BY 4.0** | Trait layer for modelling + encyclopedia. Cite Falster/Gallagher et al. + original sources. |
| **Flora of Australia profiles** — profiles.ala.org.au | Species/taxon descriptive treatments (morphology, distribution, maps) for AU flora | ALA Profiles service | CC-BY | **Ready-made encyclopedic content** — long-form per-taxon text. |
| **ABRS resources** | AusGrass keys, lichen checklists, bioregion reports | Web/CSV | Open | Identification-key content for explorer interfaces. |
| **Australian Microbiome / Bioplatforms** | (§3 above) | | | |
| **BOLD** — boldsystems.org | DNA barcodes incl. AU specimens | API + record downloads | mixed/research | Barcode↔taxon linkage; check per-record terms. |
| **ARGA** — Australian Reference Genome Atlas | Reference genomes for AU biota | arga.org.au portal + index | open | Genomic anchor for micro-scale collection. **To verify formats.** |

## 5B. Unified Species Trait & Biophysical Parameter Spine (Dimensional & Physics Governance)

A foundational design rule of `ecopunk` (see [`digital-twin-asset-requirements.md`](../tech/assets/digital-twin-asset-requirements.md) `DTA-ID-05`, `DTA-SEM-05`, `DTA-PHY-01..09`) is that **3D geometry bounds, allometric growth curves, wood/tissue density, mass, and physics colliders are never arbitrary artist or procedural magic numbers**. Every living organism (`bio:DarwinianTaxon`) and every manufactured field artefact (`made:Artefact`) derives its canonical SI dimensions (`1.0 unit = 1.0 m`) and physical mass (`kg`) directly from QUDT-typed `.q42` trait statements compiled from authoritative trait databases:

| Taxonomic Group | Primary Trait & Morphological Sources | Canonical QUDT-Typed Parameters Extracted into `.q42` | Engine / Physics Consumer |
|---|---|---|---|
| **Australian Vascular Flora** (e.g. *Corymbia aparrerinja*, *Eucalyptus pauciflora*, *E. delegatensis*, *Banksia integrifolia*) | **AusTraits** (`austraits.org`), **APC / Flora of Australia** (`profiles.ala.org.au`), **TERN AusPlots** | `plant_height_max_m`, `plant_height_typical_m`, `stem_diameter_max_m` (DBH), `wood_density_kg_per_m3`, `leaf_length_mean_mm`, `leaf_width_mean_mm`, `specific_leaf_area_m2_per_kg`, `bark_thickness_mean_mm`, `root_depth_max_m`, `seed_mass_mean_mg` (see [`entities/taxa/corymbia-aparrerinja/traits.json`](../../../entities/taxa/corymbia-aparrerinja/traits.json)) | `.10d` `PROC1` Huxley allometry & Pipe Model ($\eta = 2.49$), `qualia-sim::metamer_fspm` stem modulus-of-rupture & wind deflection, `DTA-ID-05` mesh AABB gate |
| **Australian Mammals (Marsupials, Monotremes, Eutherians)** (e.g. *Macropus giganteus*, *Osphranter rufus*, *Vombatus ursinus*, *Phascolarctos cinereus*, *Ornithorhynchus anatinus*) | **AFD (Australian Faunal Directory)**, **OZCAM morphometrics**, **PanTHERIA / COMBINE / AusMammal trait compilations** | `adult_body_mass_kg` (min/mean/max by sex), `standing_height_m`, `shoulder_height_m`, `head_body_length_m`, `tail_length_m`, `pes_length_mm`, `stride_frequency_hz`, `achilles_tendon_stiffness_n_per_m`, `locomotion_mode` | `qualia-sim::physics3d::XpbdWorld3D` rigid/ragdoll mass & inertia tensor, `FaunaSkeletalRig10d` bone lengths ($L_1, L_2$) for Two-Bone & FABRIK Motor IK, `DTA-ID-05` mesh AABB gate |
| **Australian Avifauna** (e.g. *Aquila audax*, *Dromaius novaehollandiae*, *Casuarius casuarius*, *Menura novaehollandiae*, *Dacelo novaeguineae*) | **AVONET** (global morphological & ecological bird dataset, CC-BY 4.0), **HANZAB / BirdLife Australia**, **AFD** | `body_mass_g`, `wingspan_m`, `wing_chord_mm`, `tarsus_length_mm`, `culmen_length_mm`, `tail_length_mm`, `hand_wing_index` | Aerodynamic lift/glide polar in `qualia-sim`, skeletal wing/tarsus bone lengths, `DTA-ID-05` mesh AABB gate |
| **Australian Herpetofauna & Actinopterygii** (e.g. *Varanus varius*, *Pseudophryne corroboree*, *Maccullochella peelii*) | **Meiri / Feldman Squamate & Amphibian trait datasets**, **AFD**, **Fishes of Australia (OZCAM)** | `snout_vent_length_mm` (SVL), `total_length_m`, `body_mass_g`, `thermal_critical_min_max_c`, `buoyancy_neutral_depth_m` | Undulatory/sprawling gait stride lengths, ectothermic activity envelopes, 3D Archimedes buoyancy (`qualia-sim::hydro`), `DTA-ID-05` mesh AABB gate |
| **Manufactured Field Vehicles & Infrastructure** (`made:Artefact`, DOM-AX-1) | Engineering specification sheets (e.g. NPWS 4WD Field Utility / LandCruiser 70-series / Survey Rover reference envelope) | `overall_length_m` (`4.85 m`), `overall_width_m` (`1.88 m`), `overall_height_m` (`1.92 m`), `wheelbase_m` (`2.85 m`), `track_width_m` (`1.55 m`), `wheel_radius_m` (`0.41 m`), `curb_mass_kg` (`2,240 kg`), `suspension_stiffness_n_per_m` | `qualia-sim::kinematics::RoverState` 4-wheel ray/sphere suspension, `qualia-sim::geomechanics` Bekker wheel sinkage, `DTA-ID-05` relative scale invariant |

## 5A. Sociosphere & Indigenous knowledge sources

See [../sociosphere.md](../sociosphere.md) for the axiom family these feed — place-as-
relation (Country ≠ property), standing axes, custodianship, phenological knowledge.

| Source | What | Access & format | Licence/sensitivity | ecopunk notes |
|---|---|---|---|---|
| **AIATSIS** | AUSTLANG (Indigenous languages + varieties + places), Map of Indigenous Australia (nations/language-group regions), subject thesaurus | Web portals; some downloadable datasets; thesaurus | Verify per dataset | Nations/language regions model Country-scale relations — **not** cadastral boundaries; render rules differ (SOC-AX-2). |
| **NNTT registers** | Native Title determinations + ILUAs — spatial custodianship/agreement overlays | National Native Title Tribunal spatial layers/search | Public register; sensitive details redacted | Legal overlays *over* Country, distinct entities from Country itself. |
| **Indigenous Protected Areas** | Voluntary Indigenous-managed conservation areas | Inside CAPAD download | per CAPAD | Custodianship exemplar for the relation family. |
| **Indigenous seasonal calendars** | Community phenology (Tiwi 3-season, Nyoongar 6, Gooniyandi, Ngadju…) — species/weather/activity phases | Published materials via CSIRO/BOM/community projects | Community co-owned — **negotiate; TK labels apply** | Phenology = structured ecological observation (SOC-AX-6). High-value modelling data; high-sensitivity handling. |
| **I-Tracker / ranger programs** | Biocultural monitoring data by Indigenous ranger groups | NAILSMA / program partners | Community-owned | Community data stewardship precedent. **To verify access.** |
| **Local Contexts Hub** | TK/BC label registry + label semantics | localcontexts.org | Label framework | Machine-readable label → deontic condition mapping. |
| **Heritage registers (state/fed)** | Registered sites/places | Mixed; most detail restricted | **Restricted** — canonical sensitivity case | Registry presence yes; coordinates/details typically withheld — proves the sensitivity model. |
| **Oral history / AIATSIS collections** | Recorded knowledge, elders' testimony | Reading-room/access-request | Restricted | Knowledge-lineage provenance (SOC open q4). |
| **Public-domain ethnographic works** | e.g. Maiden, *The Useful Native Plants of Australia* (1889); Roth bulletins; early ethnographies | Digitised books (Trove, Biodiversity Heritage Library, archive.org) | **Public domain** (pre-1929) | Commons-lane seed for use-knowledge (DOM-AX-5) — dated colonial-era framings need honest context labelling, and never stand in for community-held knowledge. |
| **Historical boundary/jurisdiction data** | Past colonies, shires, pastoral districts, native-title history | NNTT historical records; state archives; AUS geospatial history (AHB) | mixed | Feeds DOM-AX-2/3 versioned borders. **To verify availability as spatial data.** |
| **Biodiversity Heritage Library / Trove** | Digitised AU taxonomic literature, field guides, expedition reports | OCR'd texts, APIs | PD / CC mixed | Encyclopedic + historical baseline content. |
| **OGTR — Office of the Gene Technology Regulator** | GM organism register: approved dealings (DIR), GMO crops/organisms released in AU | ogtr.gov.au register lists/search | Public register | The `engineered` lineage registry for DOM-AX-1 — engineered organisms carry regulator + IP entanglement at creation. **To verify bulk access.** |

## 6. Citizen science & media

| Source | What | Access & format | Licence | ecopunk notes |
|---|---|---|---|---|
| **iNaturalist AU** | Community observations + CC-licensed photos; feeds ALA/GBIF | iNat API; GBIF export | per-observation (CC-BY/CC0/CC-BY-NC) | Photo source for asset references; honour per-photo licence. |
| **BioCollect (ALA)** | Project-based citizen science (ACSA hub) | ALA APIs | open-ish | Survey-structured observations. |
| **FrogID (Australian Museum)** | Frog call observations + audio | via AM/ALA | research terms | Acoustic + occurrence. **To verify access.** |
| **Aussie Bird Count / BirdLife Australia** | Bird surveys | Birdata portal/exports | registered | **To verify.** |
| **DigiVol / AVH imaging** | Digitised specimen labels + specimen imagery | via ALA images | per image | Media + transcription provenance. |
| **TERN EcoImages / EcoAcoustics** | Photopoint, leaf, organism images; long-form soundscape audio | ecoimages/ecoacoustics portals | TERN licence | **Direct media assets for packs** (real AU field imagery/audio). |
| **Wikimedia Commons / Wikipedia** | Species photos + encyclopedic text, incl. strong AU coverage | MediaWiki API, Commons categories | CC-BY-SA / CC0 / PD mixed | Text + media for encyclopedic interface; per-file licence tracking required. |

## 7. LOD-cloud & ontology layer (alignment targets, not data sources per se)

| Vocabulary/ontology | Role | Status/notes |
|---|---|---|
| **TERN Ontology** — `w3id.org/tern/ontologies/tern/` | Site/survey/observation/sample information model | OWL + SHACL; profile of SOSA/SSN/SSN-ext + PROV-O + GeoSPARQL + QUDT. GitHub source, CC-BY. **Adopt as the survey-data spine.** |
| **Darwin Core (dwc:)** | Occurrence/taxon exchange standard | ALA/GBIF native; TDWG. Required for interoperability. |
| **SOSA/SSN + SSN-ext** | Observation/sampling semantics | W3C; TERN's base. |
| **GeoSPARQL** | Spatial features/geometries | TERN uses it; QualiaDB executes GeoSPARQL natively — direct fit. |
| **QUDT** | Units/quantities | TERN results carry QUDT units. |
| **PROV-O** | Provenance chains | Bundled in QualiaDB already. |
| **National Species List Data Model** | AU taxon/name model | Registered on AGLDWG catalogue (`catalogue.linked.data.gov.au`) — **adopt for names backbone.** |
| **AGLDWG register ontologies** | AU gov LOD: ANSIS soil ontology, Australian Soil Classification, Plot Ontology, BDBSA vocabs, Queensland Spatial Information Supermodel, Plant Health Surveillance, G-NAF, FSDF, GeoSPARQL extensions, AusPix DGGS | All in `/def/` persistent namespace — **namespace persistence handled by government** (50-year-durability friendly). |
| **ENVO / PATO / UBERON / PO / NCBITaxon / ChEBI / FOODON / GO** | Environment, phenotype, anatomy, plant, taxon, chemical, food, gene ontologies | OBO family; fill domain gaps TERN/DwC don't cover (esp. micro-scale). ChEBI already bundled in qualiaDB manifests. |
| **Wikidata / Wikipedia / Open Tree of Life** | Hub identifiers, encyclopedic text, phylogeny | CC0 / CC-BY-SA / mixed. Wikidata QIDs = durable cross-source join keys. |
| **PLAZI TreatmentBank** | Taxonomic treatments as RDF (SPARQL endpoint exists) | Rich AU taxonomic literature → LOD. **To verify endpoint coverage.** |
| **DCAT + VocPub profile** | Dataset/vocabulary cataloguing | AGLDWG catalogue conventions; use for our own dataset manifests. |
| **DWC-DP / Camtrap DP / Humboldt Core** | New TDWG packages: survey design, camera traps, extended survey metadata | Emerging standards — track for survey/camera-trap data (TERN EcoAcoustics/EcoImages fit here). ALA chairs TDWG groups; alignment matters. |
| **Audubon Core** | Media/metadata standard | ALA participates in its interest group; use for media assets in packs. |

## 8. Format inventory (observed across AU sources)

| Format | Seen at | Handling path |
|---|---|---|
| Darwin Core Archive (DwC-A zip: CSV + meta.xml + EML) | ALA per-dataset, GBIF | Parse meta.xml → csv_mapper → emit DwC IRIs |
| CSV (+ JSON REST) | ALA APIs, WildNet, AFD, SPRAT, AusTraits | csv_mapper/json_mapper → vocab mapping |
| OData | NSW BioNet | Custom fetcher → JSON → mapper |
| RDF/Turtle + SPARQL | TERN GraphDB, AGLDWG, PLAZI | **Direct ingest** (Turtle supported by qualia-cli) |
| ArcGIS REST / WMS / WFS | DCCEEW, GA, states | Layer→vector extraction → GeoSPARQL/GeoJSON → KML/WKT ingest (KML detected by qualia-cli) |
| Shapefile / FileGDB / GeoJSON | DCCEEW downloads, states | Transform → WKT geometries |
| GeoTIFF / NetCDF / THREDDS | DEA, BOM, IMOS, TERN raster | **Dense raster sidecar needed** — engine gap (no raster→10d/q42 path today) |
| RDS / Parquet | AusTraits | Parquet→CSV/JSON bridge then mapper |
| Audio (WAV/FLAC) + imagery | EcoAcoustics, EcoImages, ALA images, FrogID | Media asset references (Audubon Core metadata) |
| Sequence (FASTA/FASTQ, BIOM) | Microbiome, BOLD, ARGA | qualiaDB bioinformatics primitives exist (FASTA validation, k-mers) — map sequences as dense-asset refs |

## 9. Sensitivity & sovereignty notes (AU-specific)

- **SNES/SPRAT**: Commonwealth pre-generalises threatened-species grids (1 km; ~10 km for state-classified sensitive species). Our packs must treat *raw* state coordinates for these taxa as classified — keep coarse products only in public packs.
- **ALA Sensitive Data Service**: exists precisely because states hold different sensitive lists — a per-jurisdiction sensitivity registry we must model, not a single flag.
- **NSW BioNet tiered access** and **WildNet published-subset model** show authorities already separate public vs restricted records — mirror that split in pack builds.
- **Indigenous knowledge**: ALA has an Indigenous Ecological Knowledge programme; TK/BC labels and CARE principles apply to any community-sourced ecological knowledge. Do not ingest IEK datasets without label-aware handling. (ALA's "Acknowledgement of Country" + IEK programme page is the entry point.)
- **Record-level licences inside ALA**: per-record `dcterms:license` must survive into our provenance graph — CC-BY-NC records cannot enter a permissive-commons pack.

## 10. Open questions / next discovery steps

1. Verify VIC (VBA/NatureKit), WA (NatureMap/SLIP), TAS (NVA), NT (NRETAS), ACT public vs registered tiers — marked *To verify* above.
2. TERN GraphDB: is there a general production SPARQL endpoint beyond demo repos? What datasets are loaded? (graphdb.tern.org.au public repo list.)
3. AFD/APC bulk download licence text — is it explicitly CC-BY or "open gov" unlabelled?
4. AusTraits → trait ontology mapping: PATO/TO terms already assigned in dictionary?
5. Microbiome Initiative terms — registration scope, attribution requirements, per-project licences.
6. IMOS animal telemetry (ATN/IMOS ATF) — redistribution terms for derived tracks.
7. DEA/BOM licence detail for repackaging derived rasters.
8. Flora of Australia profiles — is content bulk-downloadable, and under what licence vs site CC-BY?
9. Whether linked.data.gov.au will mint `/def/` PIDs for ecopunk artefacts (AGLDWG offers assistance) — strong 50-year namespace option.
10. iNaturalist-AU photo licence distribution — what fraction is Commons-safe?
