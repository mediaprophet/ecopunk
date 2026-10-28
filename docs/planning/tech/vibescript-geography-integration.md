# VibeScript Geography, Terrain & GIS Integration Specification (`VIBE-GEO-0.2`)

- **Date**: 2026-10-28
- **Status**: Comprehensive Language, Catalog, Effect, Schema & REPL Integration Specification for `GEO-SUPERIORITY-v2`
- **Governing Specifications**:
  - [`docs/planning/tech/vibescript-0.2-specification.md`](vibescript-0.2-specification.md)
  - [`docs/planning/tech/vibescript-0.2-source-and-conformance.md`](vibescript-0.2-source-and-conformance.md)
  - [`docs/planning/tech/vibescript-0.2-repl-and-host-contract.md`](vibescript-0.2-repl-and-host-contract.md)
  - [`docs/planning/tech/geography-swarm-implementation-plan.md`](geography-swarm-implementation-plan.md)
- **Inspected `crates/vibe` Seams (`C:/github/qualiaDB/crates/vibe/src/`)**:
  - [`lib.rs`](file:///C:/github/qualiaDB/crates/vibe/src/lib.rs#L1-L188) (`LANGUAGE_VERSION = "vibe-0.1"`, `HOST_VERSION = "vibe-host-0.1"`, `eval_cell`, `parse_program`, `check_program`, `Engine`)
  - [`catalog/mod.rs`](file:///C:/github/qualiaDB/crates/vibe/src/catalog/mod.rs#L1-L150) & [`catalog/ids.rs`](file:///C:/github/qualiaDB/crates/vibe/src/catalog/ids.rs#L1-L1264) (`ALL_INVOKE_IDS`, `canonical_id`, `granted_covers`)
  - [`catalog/specialized.rs`](file:///C:/github/qualiaDB/crates/vibe/src/catalog/specialized.rs#L1-L150) (`try_invoke` for `LocalHost` kernels)
  - [`capability_schema.rs`](file:///C:/github/qualiaDB/crates/vibe/src/capability_schema.rs#L1-L765) (`SchemaEntry`, `SchemaArg`, `EffectClass`, `HonestyLabel`, `all_schemas()`)
  - [`effects.rs`](file:///C:/github/qualiaDB/crates/vibe/src/effects.rs#L1-L88) (`Effect::Pure < Hot < Cold < Async < External`, `binding_effect`, `capability_for`)
  - [`bind/dispatch.rs`](file:///C:/github/qualiaDB/crates/vibe/src/bind/dispatch.rs#L1-L200) (`dispatch<H: Host>`)
  - [`topology_decl.rs`](file:///C:/github/qualiaDB/crates/vibe/src/topology_decl.rs#L1-L180) (`GeometryDecl`, `HarbouredGraphDecl`, `TopologyInvokeDescriptor`)
  - [`quantity.rs`](file:///C:/github/qualiaDB/crates/vibe/src/quantity.rs) & [`reactive_cell.rs`](file:///C:/github/qualiaDB/crates/vibe/src/reactive_cell.rs)

---

## 1. Why VibeScript Makes QualiaDB Superior to `EarthCraft`, `Nerthus`, and `RustGIS`

Each of the three reference repositories suffers from fragmented, non-auditable, or insecure scripting and automation surfaces:

| Reference Repo | Legacy Scripting / Automation Mechanism | Architectural Weakness Replaced by VibeScript |
| :--- | :--- | :--- |
| **`EarthCraft`** (`v0.2.0`) | Hardcoded Rust `egui` callbacks (`app.rs`); no expression language, no scriptable camera tours, no programmable ephemeris or bookmark automation. | Replaced by **VibeScript clocked tweens (`from ~ to over duration ease name`)**, `present` view declarations, and `Pure`/`Hot` `Geodesy.*` cells evaluated directly from `.vibe` source with **zero Rust/Cargo rebuilds**. |
| **`Nerthus`** (`v2.3.1`) | Imperative JSON graph serializer (`GraphSerializer`) + **unauthenticated localhost TCP JSON-RPC server (`127.0.0.1:47110`)** exposing 26 untyped string commands (`nerthus_create_node`, `nerthus_erode`, etc.). | Replaced by **VibeScript pipeline expressions (`value \|> Terrain.erode_fixed824(...)`)**, declarative `geometry <Name> : StrataColumnComplex { ... }` blocks, and capability-gated `Terrain.*` (`Effect::Cold`, `HonestyLabel::CapabilityLease`) running over `AuthorizedEditorHost` and `NativeIntentBus`. |
| **`RustGIS`** (`v0.4.0`) | Three separate ad-hoc evaluators: (1) string-split Field Calculator, (2) string-split Raster Calculator, (3) JSON `crates/modelbuilder` DAG + **unauthenticated localhost HTTP MCP (`127.0.0.1:7879`)**. | Replaced by **one unified, type-checked, effect-checked, budget-bounded VibeScript engine**: pure cell formulas (`= (nir - red) / (nir + red)`) for field/raster math, `GisIo.*` / `GisGp.*` pipelines for geoprocessing DAGs, and native `graph? { ... }` + `obligate`/`permit`/`forbid` for GeoSPARQL + Deontic spatial covenants. |

---

## 2. Five VibeScript Integration Surfaces Across Rounds 1–10

To ensure VibeScript is updated systematically alongside every Rust engine module (rather than bolted on as an afterthought), every swarm round (`R1`–`R10`) includes a dedicated **`geo-vibe-worker` (`Model: "flash"`) packet (`R1-V` through `R10-V`)**:

### Surface A — Geodetic, Hydrological & Cartographic Units in `crates/vibe/src/quantity.rs` (`Round 1: R1-V`)
VibeScript 0.2 (§3 & §4) requires that quantity suffixes on numeric literals resolve against registered unit tokens with dimensional compatibility checking (preventing unit-mismatch bugs like adding meters to degrees or mixing arcseconds with radians):
- **Length / Elevation**: `mm`, `cm`, `m`, `km`, `nmi` (nautical mile $= 1852\text{ m}$)
- **Angle / Geodesy**: `deg`, `rad`, `arcmin`, `arcsec`, `mrad`
- **Area**: `m2`, `ha` ($10^4\text{ m}^2$), `km2` ($10^6\text{ m}^2$)
- **Hydrology & Erosion**: `m3_s` (discharge $\text{m}^3/\text{s}$), `mm_hr` (rainfall intensity), `kg_m2_s` (sediment flux)
- **Solar & Radiometry**: `W_m2` (irradiance), `Wh_m2` (insolation), `K` / `degC`
- **Cartographic Print Layout**: `pt` ($1/72\text{ in}$), `mm_print`, `dpi`

### Surface B — Declarative Geospatial `geometry` & `topology` Complexes in `crates/vibe/src/topology_decl.rs` (`Rounds 2–5: R2-V..R5-V`)
Extends [`topology_decl.rs`](file:///C:/github/qualiaDB/crates/vibe/src/topology_decl.rs#L1-L180) (`geometry <Name> : <ComplexKind> { ... graph <G> : <Morphology> { ... } }`) with four geospatial `ComplexKind`s that lower deterministically into `Topology.define_geometry`, `Topology.harbour_graph`, `Terrain.*`, `GisGp.*`, and `Poet.topology_bind`:

1. **`StrataColumnComplex`** (`.m64` Mode `0x02` `M64StrataColumnRecord` 6-layer geological strata + `BankedConstituentLut` Bank 1 Sentinel-2 materials):
   ```vibe
   geometry KosciuszkoAlpine : StrataColumnComplex {
       manifold: "WGS84_GDA2020_MGA55";
       metric: "KarneyGeodesic";
       cells_source: "hmc://kosciuszko/strata_0x02.m64";
       operators: ["Terrain.erode_fixed824", "Terrain.hydro_breach_d8", "Terrain.simulate_snow"];

       graph DrainageD8 : DirectedAcyclic {
           role: "Hydrology";
           anchors: ["SnowyRiverHeadwater", "BlueLakeCirque"];
           weights: "Discharge_m3_s";
           invariants: ["AcyclicFlow", "MassConservation"];
           boundary { sources: "AlpineRidgePrecipitation"; sinks: "SnowyCatchmentOutlet"; }
       }
   }
   ```
2. **`LidarPointComplex`** (`.m64` Mode `0x00` `E64PointRecord` COPC/LAZ point clouds with ASPRS ground/canopy classification graphs).
3. **`CadastralTopologyComplex`** (planar shared-vertex parcel/biome boundary complexes lowering to `GisGp.topology_validate` with invariants `MustNotOverlap`, `MustNotHaveGaps`, `MustNotHaveDangles`).
4. **`TransportNetworkComplex`** (`RoadGraph` / utility / wildlife corridor graphs with `Dijkstra` / `A*` geodesic weights).

### Surface C — Six New Catalog Families in `catalog/ids.rs`, `capability_schema.rs` & `effects.rs` (`Rounds 1–10: R1-V..R10-V`)
Every operation is registered in both `qualia-core-db/src/poet_host/invoke/ids.rs` and `crates/vibe/src/catalog/ids.rs`, with an explicit `SchemaEntry` in `crates/vibe/src/capability_schema/` and an explicit effect classification in `crates/vibe/src/effects.rs`:

| Family (`using <Family>;`) | Round | Effect Class (`effects.rs`) | Honesty Label (`capability_schema.rs`) | Bound Methods (`Family.method`) |
| :--- | :---: | :--- | :--- | :--- |
| **`Geodesy.*`** (16 methods) | **R1 (`R1-V`)** | `Effect::Pure` (math/CRS/DGGS) / `Effect::Hot` (`SpatialRTree64` slice queries) | `HonestyLabel::Always` | `Geodesy.wgs84_inverse`, `Geodesy.wgs84_direct`, `Geodesy.wgs84_intermediate`, `Geodesy.polygon_area_m2`, `Geodesy.geodetic_to_ecef`, `Geodesy.ecef_to_geodetic`, `Geodesy.enu_basis`, `Geodesy.parse_coord`, `Geodesy.format_mgrs`, `Geodesy.crs_transform`, `Geodesy.ausgeoid_undulation`, `Geodesy.lonlat_to_a5`, `Geodesy.a5_to_lonlat`, `Geodesy.rtree_query_bbox`, `Geodesy.sun_ephemeris`, `Geodesy.topographic_insolation`, `Geodesy.orbit_propagate` |
| **`Terrain.*`** (15 methods) | **R2 & R3 (`R2-V`, `R3-V`)** | `Effect::Cold` (bounded in `GeometryWorkspace` 256 MiB) | `HonestyLabel::CapabilityLease` | `Terrain.noise`, `Terrain.primitive`, `Terrain.adjust`, `Terrain.filter`, `Terrain.select_mask`, `Terrain.combine`, `Terrain.eval_dag`, `Terrain.erode_fixed824`, `Terrain.hydro_breach_d8`, `Terrain.stream_power_incision`, `Terrain.span_fill_lakes`, `Terrain.simulate_snow`, `Terrain.thermal_shaper`, `Terrain.compile_m64_strata`, `Terrain.qa_report` |
| **`GisIo.*`** (15 methods) | **R4 & R5 (`R4-V`, `R5-V`)** | `Effect::Cold` (local stream/parse) / `Effect::External` (OGC/STAC remote queries) | `HonestyLabel::CapabilityLease` | `GisIo.read_lidar_m64`, `GisIo.classify_ground_csf`, `GisIo.rasterize_dsm_dtm_chm`, `GisIo.read_filegdb`, `GisIo.read_geopackage`, `GisIo.read_flatgeobuf`, `GisIo.read_shapefile`, `GisIo.read_geoparquet`, `GisIo.read_kml_guarded`, `GisIo.read_osm_pbf`, `GisIo.read_dxf`, `GisIo.read_pmtiles`, `GisIo.decode_mvt`, `GisIo.ogc_query`, `GisIo.read_datacube_10d` |
| **`GeoSparql.*`** (6 methods) | **R5 (`R5-V`)** | `Effect::Pure` (topological predicates) / `Effect::Read` (`.q42` frame scan) | `HonestyLabel::CapabilityLease` | `GeoSparql.compile_features_to_quins`, `GeoSparql.sf_intersects`, `GeoSparql.sf_within`, `GeoSparql.sf_contains`, `GeoSparql.sf_touches`, `GeoSparql.evaluate_deontic_covenant` |
| **`GisGp.*`** (16 methods) | **R6 & R7 (`R6-V`, `R7-V`)** | `Effect::Cold` (`GeometryWorkspace` arena) | `HonestyLabel::CapabilityLease` | `GisGp.overlay`, `GisGp.buffer`, `GisGp.dissolve`, `GisGp.simplify`, `GisGp.hull`, `GisGp.voronoi_delaunay`, `GisGp.spatial_join`, `GisGp.kde`, `GisGp.kriging`, `GisGp.spatial_autocorrelation` (Moran's $I$ / Getis-Ord $G_i^*$), `GisGp.viewshed`, `GisGp.morphometry`, `GisGp.map_algebra`, `GisGp.zonal_stats`, `GisGp.watershed_twi_spi`, `GisGp.network_route`, `GisGp.topology_validate_repair` |
| **`Cartography.*` & `WgslForge.*`** (10 methods) | **R8, R9 & R10 (`R8-V`..`R10-V`)** | `Effect::Cold` (layout/shader compile) / `Effect::Write` (CoW edit) | `HonestyLabel::CapabilityLease` | `WgslForge.geo_dispatch`, `Cartography.cow_undo`, `Cartography.cow_redo`, `Cartography.move_shared_vertex`, `Cartography.geodesic_ruler`, `Cartography.classify_choropleth`, `Cartography.simulate_cvd`, `Cartography.compose_layout`, `Cartography.export_pdf_svg_gltf`, `Render.geo_viewport_configure` |

### Surface D — Pure Reactive Cells (`= expr`) & Map Algebra Lowering (`Round 7: R7-V`)
In `RustGIS`, raster band math (`(B8 - B4) / (B8 + B4)`) and vector field calculations use fragile ad-hoc string parsers. In `QualiaDB` + `Poet`:
- Any VibeScript pure cell (`= (band_nir - band_red) / (band_nir + band_red)`) is parsed and type-checked by `vibe::parse_cell` and `vibe::check_cell` (`Effect::Pure`).
- `R7-V` adds a deterministic AST-to-Map-Algebra compiler (`crates/vibe/src/geo_map_algebra.rs`, `< 350` lines) that lowers checked VibeScript pure arithmetic/conditional ASTs (`BinOp`, `UnOp`, `ExprKind::If`) directly into the zero-alloc stack bytecode consumed by `R7-C` (`gp_map_algebra_zonal.rs`) and `R8-C` (`geo_viewshed_algebra_kernels.rs`)!
- Result: Users write standard VibeScript formulas in Poet's inspector or REPL, and the exact same checked AST runs at native SIMD/GPU speed across millions of DEM/multispectral pixels with **zero per-script Rust/Cargo/WASM compilation** (`REQ-VIBE-001`, `VBC-001`).

### Surface E — Native GeoSPARQL + Deontic + Temporal Propositions in VibeScript (`Round 5: R5-V`)
VibeScript natively supports graph queries (`graph? { ... }`), RDF-star triple terms (`<<(s p o)>>`), and modal/deontic propositions (`obligate`, `permit`, `forbid`, `always`, `eventually`). `R5-V` wires `GeoSparql.*` into `LocalHost` and `Poet` so authors can write:

```vibe
using Geodesy;
using GisIo;
using GeoSparql;

effect cold fn verify_alpine_covenant(parcel_fgb: string, lidar_copc: string) -> Result<Record, string> {
    let parcels = GisIo.read_flatgeobuf(path: parcel_fgb, crs: "EPSG:7855")?;
    let quins = GeoSparql.compile_features_to_quins(features: parcels, context: <did:q42:kosciuszko_national_park>)?;
    let verdict = GeoSparql.evaluate_deontic_covenant(
        quins: quins,
        zone_predicate: <geo:sfWithin>,
        norm_opcode: "forbid",
        action: <ecopunk:clear_snow_gum_canopy>
    )?;
    return Ok(verdict);
}
```

---

## 3. Round-by-Round VibeScript Worker Packets (`R1-V` through `R10-V`)

Each round adds one disjoint `< 500` LOC leaf file in `C:/github/qualiaDB/crates/vibe/src/` plus `.vibe` conformance fixtures in `C:/github/qualiaDB/crates/vibe/fixtures/geography/`:

| Packet | Round | Owner (`Model: "flash"`) | Target Files (`< 500` LOC each) | Conformance Cases Covered (`vibescript-0.2-source-and-conformance.md`) |
| :--- | :---: | :--- | :--- | :--- |
| **`R1-V`** | **1** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_geodesy_entries.rs` + `crates/vibe/fixtures/geography/geodesy_crs_ephemeris.vibe` | `VBC-001` (no-build source eval), `VBC-005` (exact numeric/NaN), `VBC-030` (geodetic units/frames in `quantity.rs` + `Geodesy.*` `Pure`/`Hot` schemas) |
| **`R2-V`** | **2** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_terrain_node_entries.rs` + `crates/vibe/fixtures/geography/terrain_dag_pipeline.vibe` | `VBC-004` (`\|>` pipeline chaining over `Terrain.noise \|> Terrain.primitive \|> Terrain.combine`), `VBC-011` (`Cold` effect checking), `VBC-013` (`GeometryWorkspace` budget traps) |
| **`R3-V`** | **3** | `geo-vibe-worker` | `crates/vibe/src/topology_decl_geo.rs` + `crates/vibe/fixtures/geography/strata_column_complex.vibe` | `VBC-015` (multi-identifier subject & `.m64` `0x02` `StrataColumnComplex` lowering), `VBC-024` (`Terrain.erode_fixed824` & `Terrain.compile_m64_strata` descriptor parity) |
| **`R4-V`** | **4** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_gis_io_entries.rs` + `crates/vibe/fixtures/geography/lidar_vector_ingest.vibe` | `VBC-009` (`Result<T,E>` & `?` propagation on malformed/bomb KML or missing FileGDB), `VBC-012` (capability lease enforcement for `GisIo.*`) |
| **`R5-V`** | **5** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_sparql_deontic_entries.rs` + `crates/vibe/fixtures/geography/geosparql_deontic_covenant.vibe` | `VBC-014` (one graph snapshot & unforgeable `Receipt`), `VBC-015` (`did:q42` + `EPSG` + `A5` identifier survival), `VBC-030` (`obligate`/`permit`/`forbid` + `GeoSparql.*`) |
| **`R6-V`** | **6** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_vector_gp_entries.rs` + `crates/vibe/fixtures/geography/vector_geoprocessing.vibe` | `VBC-011` & `VBC-013` (`GisGp.overlay`, `GisGp.buffer`, `GisGp.spatial_join`, `GisGp.kde` effect/budget enforcement) |
| **`R7-V`** | **7** | `geo-vibe-worker` | `crates/vibe/src/geo_map_algebra.rs` + `crates/vibe/fixtures/geography/map_algebra_kriging_network.vibe` | `VBC-001` & `VBC-016` (Pure reactive cell `= (nir - red) / (nir + red)` lowered to zero-alloc Map Algebra bytecode; Kriging, Moran's $I$, `TWI`, and network routing descriptors) |
| **`R8-V`** | **8** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_wgsl_forge_entries.rs` + `crates/vibe/fixtures/geography/wgsl_forge_terrain_gpu.vibe` | `VBC-027` & `VBC-029` (Native/WASM/CPU-oracle equivalence for `WgslForge.geo_dispatch` erosion/viewshed/algebra kernels) |
| **`R9-V`** | **9** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_cartography_poet_entries.rs` + `crates/vibe/fixtures/geography/poet_cartography_live_edit.vibe` | `VBC-017` (`present` & `bind a <-> b` for Poet terrain/GIS controls), `VBC-019`–`VBC-021` (persistent REPL live edit + `PutAside` CoW undo/redo), `VBC-025` (cooperative review) |
| **`R10-V`** | **10** | `geo-vibe-worker` | `crates/vibe/src/capability_schema/geo_viewport_entries.rs` + `crates/vibe/fixtures/geography/kosciuszko_e2e_authoring.vibe` | `VBC-018` (`FlyAnim` clocked camera tween `from ~ to over duration ease`), `VBC-031` (End-to-end no-build native Poet + VibeScript Kosciuszko authoring & HMC reproduction) |
