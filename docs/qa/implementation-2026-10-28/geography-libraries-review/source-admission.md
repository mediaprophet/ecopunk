# Source Admission & Dependency Inventory: `EarthCraft`, `Nerthus` and `RustGIS`

**Review date:** 2026-10-28 (v1.1 — updated with `RustGIS-main`) · **Status:** read-only discovery, license/notice verification, and candidate crate classification across all **37 `Cargo.toml` manifests** in `C:/github/geography-libraries`; no extraction, build, or implementation is executed by this document.

---

## 1. Scope, Provenance & Workspace Topology

We audited all three workspaces under `C:/github/geography-libraries`:
1. **`C:/github/geography-libraries/EarthCraft-main`**:
   - **Version**: `0.2.0` (`edition = "2024"`, `rust-version = "1.95"`, `resolver = "3"`).
   - **License**: `MIT` (`Copyright (c) 2026 Cleber`, `LICENSE:1–22`).
   - **Manifests**: 5 `Cargo.toml` files (1 workspace root + 3 library crates under `crates/*` + 1 binary crate under `apps/earthcraft`).
   - **Safety Lints**: Workspace-wide `unsafe_code = "forbid"` (`Cargo.toml:65`), with a single narrow `#![allow(unsafe_code)]` in `apps/earthcraft/src/app/menu/windows.rs:2` for Win32 menu bar cleanup (`SetMenu`).
2. **`C:/github/geography-libraries/nerthus-main`**:
   - **Version**: `2.3.1` in crate manifests / `1.0.0` in root workspace header (`edition = "2024"`, `rust-version = "1.99"`, `resolver = "3"`).
   - **License**: `MIT` (`Copyright (c) 2026 Dane Madsen <danemadsen96@gmail.com>`, `LICENSE:1–21`).
   - **Clean-Room Context**: Clean-room Rust reimplementation of QuadSpinner Gaea 2.3.1.0 (`README.md:7–13`), interoperable with Gaea 2 `.terrain` (Newtonsoft JSON) and legacy Gaea 1 `.tor` (XML/ZIP) and `.gpr` (NRBF) formats.
   - **Manifests**: 15 `Cargo.toml` files (1 workspace root + 14 member crates under `crates/*`).
3. **`C:/github/geography-libraries/RustGIS-main`**:
   - **Version**: `0.4.0` (`edition = "2024"`, `rust-version = "1.90"` in root / `"1.88"` in members, `resolver = "3"`).
   - **License**: `Apache-2.0` (`LICENSE:1–178`, `THIRD-PARTY-LICENSES.txt:1–11135`, `deny.toml:1–32`).
   - **Safety Lints**: Workspace-wide `unsafe_code = "forbid"` (`Cargo.toml:112`) and `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]` across all headless crates.
   - **Manifests**: 17 `Cargo.toml` files (1 workspace root + 11 library/UI crates under `crates/*` + 4 app crates under `apps/*` + `xtask`).

Exact file sizes, line counts, and SHA-256 digests for all **37 manifests** (`5` EarthCraft + `15` Nerthus + `17` RustGIS) are tabulated below and recorded in [`coverage-and-pins.json`](coverage-and-pins.json).

---

## 2. Complete Manifest Inventory (37 `Cargo.toml` Files)

### 2.1 `EarthCraft-main` Manifests (5 Files)

| Manifest Path (relative to `EarthCraft-main`) | Package Name | Version | Lines | Bytes | SHA-256 | Primary Role |
|---|---|---|---:|---:|---|---|
| `Cargo.toml` | *(workspace root)* | `0.2.0` | 77 | 2,617 | `a3fdb9476935bb5bb14432118bacfee95da488023fefcc32afa134236d4d7d09` | Workspace definition, dependency pins, `unsafe_code = "forbid"` |
| `crates/earthcraft-core/Cargo.toml` | `earthcraft-core` | `0.2.0` | 26 | 532 | `4a9cddc82bef10802e2e53e7580dad27a1bda0cca3ffd94e1360fbb32fb62d6a` | Pure geodesy (`Wgs84`/`PerfectSphere`), coordinates (`DD`/`DMS`/`DDM`/`UTM`/`UPS`/`MGRS`), geodesic `Ruler`, `Camera`, `FlyAnim`, `CelestialFrame`, `OrbitElements`, `Places` KML tree, `ControllerProfile`, Fluent i18n |
| `crates/earthcraft-data/Cargo.toml` | `earthcraft-data` | `0.2.0` | 36 | 722 | `9ab83c720c242f318ba6670edc9f064946cf06257187dc90b5f4162c7aa84ede` | Tile providers, `moka` L1 + `rusqlite` `.mbtiles` L2 cache, guarded KML/KMZ & `.earthplanet` I/O, administrative boundaries, prefix search + Nominatim/Photon, live aircraft/AIS/satellites/weather/roads/traffic |
| `crates/earthcraft-render/Cargo.toml` | `earthcraft-render` | `0.2.0` | 30 | 656 | `b4764f84653c3f84ef0a11f0a8630c84e2ba80a98bbedb9a1ee6d49ef7a3525d` | `wide::f64x4` + `rayon` CPU ray-casting globe renderer, Mapterhorn Terrarium DEM ray-displacement, experimental `wgpu` overview compute shader + CPU seam/limb repair, `geo-index`/`rstar` spatial overlay & label indexing |
| `apps/earthcraft/Cargo.toml` | `earthcraft` | `0.2.0` | 58 | 2,260 | `a782b61ece3d5aee224f7a97cf1b7681c18e2765817d361c2ca09586cfb8e488` | Desktop `eframe`/`egui` application shell, native menus (`muda`), dialogs (`rfd`), gamepad polling (`gilrs`) |

### 2.2 `nerthus-main` Manifests (15 Files)

| Manifest Path (relative to `nerthus-main`) | Package Name | Version | Lines | Bytes | SHA-256 | Primary Role |
|---|---|---|---:|---:|---|---|
| `Cargo.toml` | *(workspace root)* | `1.0.0` | 164 | 6,116 | `64a8b5def8c5724e024499f45ee32341692f6a359d34785fcdfd1136705b4911` | Workspace root (14 crates), shared dependency specs, release LTO |
| `crates/nerthus-core/Cargo.toml` | `nerthus-core` | `2.3.1` | 19 | 468 | `dd63f232b27713abb7516dcbab0e0670a10e8f70adff8913691536c07de8635b` | Gaea 1 `.tor` graph model: `Field`, `NodeInstance`, `ParamValue`, `PortSet`, `Project` |
| `crates/nerthus-engine/Cargo.toml` | `nerthus-engine` | `2.3.1` | 21 | 696 | `cef52fb91c534a9d7eae318b1d11b2ffeda07ff2b219481bf999ac292ccdc0ae` | Gaea 2.3.1.0 `.terrain` engine: planar `Map`, `Catalogue` (`nodes.toml` + `modifiers.toml`), Newtonsoft JSON loader/writer, JACE expressions, macros, loops, topological `Engine` |
| `crates/nerthus-noise/Cargo.toml` | `nerthus-noise` | `2.3.1` | 11 | 280 | `10b3c3bcf6b52815765f3b906229e524486b75abe8b390a57b7333ef4182e614` | `FastNoiseSIMD` port, `Perlin`, `Simplex`, `Cellular`, `Gabor`, `.NET Random`, `Mt19937`, `XorShift64Star`, x86 SSE/AVX2 `RCPPS`/`RSQRTPS` bit-exact LUTs |
| `crates/nerthus-image/Cargo.toml` | `nerthus-image` | `2.3.1` | 13 | 308 | `d189758155f7e39630cddea527d79a7395576acfc13932c744ef8e9a525b8df4` | ImageMagick 7 Q16 HDRI raster processing (colorspace, morphology, convolve, distort, seam carving, octree quantization) |
| `crates/nerthus-erosion/Cargo.toml` | `nerthus-erosion` | `2.3.1` | 15 | 324 | `2e204491fc85c962ec71b648d09aea21d8c4a10cb487b93b48d5ce6730860194` | 5 C++ erosion/hydrology engine ports: `ce` (droplet/thermal), `cs` (snow), `cw` (shallow water + `SpanFill`), `hydro` (priority-flood drainage & rivers), `ts` (thermal shaper) |
| `crates/nerthus-compute/Cargo.toml` | `nerthus-compute` | `2.3.1` | 27 | 665 | `48d7d8e011b868a64822362b24165f9010f9df485f6fd9f6b1724cfc04397dad` | `wgpu 30.0.1` compute runtime (`GpuDevice`), adapter enumeration, batching (`DISPATCHES_PER_SUBMISSION = 64`), plus `nvidia_profile.rs` (to exclude) |
| `crates/nerthus-nodes/Cargo.toml` | `nerthus-nodes` | `2.3.1` | 29 | 755 | `8084291c3e06a47fbdb1a4a194a827ff69f25c179a26e15434b608fc54f0938c` | 182 node builders + 19 modifiers, 8.24 fixed-point `ParallelErosion_Erode` (CPU + WGSL), 12 WGSL compute shaders, `sycl.rs` fallback, Delaunator, QEM mesh decimation, 323 Sentinel-2 CLUTs |
| `crates/nerthus-assimp/Cargo.toml` | `nerthus-assimp` | `2.3.1` | 13 | 313 | `31073f4bf3d81956ca8de28fb958073e4ea1bafa292d3797a120f651751191d1` | Clean-room Assimp 5.0.0 mesh importer (`OBJ`, `PLY`, `STL`, `3DS`, `LWO`, `DXF`, `OFF`, `NFF`, `RAW`, `COB`, `SMD`, `glTF`/`GLB`, `FBX` ASCII/binary) |
| `crates/nerthus-io/Cargo.toml` | `nerthus-io` | `2.3.1` | 50 | 1,329 | `1e4daa1e85bed7305956465450b398f31d3c009d98141a6e812039c49df870cc` | `.tor` XML/ZIP project I/O, `.gpr` NRBF preset reader, 100+ format raster reader/writer (`EXR`, `TIFF`, `PSD`, `DNG`/`RAW`, `HEIF`/`AVIF`, `JP2`, `JXL`, `SVG`, `DDS`, `HDR`) |
| `crates/nerthus-hevc/Cargo.toml` | `nerthus-hevc` | `2.3.1` | 10 | 258 | `609dbf4c157824cbcc389d42f469861facfe8eea7d0589c01c672edcdf47b4eb` | Clean-room `libde265 1.0.16` intra-slice HEVC (H.265) still-image decoder |
| `crates/nerthus-export/Cargo.toml` | `nerthus-export` | `2.3.1` | 28 | 821 | `06189974ae06341e60888e489047cd509fdbd6b9602d4840ea2407e8d8cfd4d9` | Build & export manager: 19 raster/point-cloud formats, 8 mesh formats, tiled/split builds, post-process modifiers, DCC bridge scripts |
| `crates/nerthus-render/Cargo.toml` | `nerthus-render` | `2.3.1` | 12 | 261 | `13c19329f81773b0dbc2767343e2e7334ddaf95157fb25f5dfd20990df2ea7e0` | Bevy `0.20.0` 3D terrain viewport plugin (`TerrainMaterial` `terrain.wgsl`, Bruneton `SkyMaterial` `sky.wgsl`, water plane, bounds, camera) |
| `crates/nerthus-app/Cargo.toml` | `nerthus-app` | `2.3.1` | 73 | 2,559 | `d94e8419ded9fdd743761b58831d8ccc6b0a50630aae97424294f7aca929611b` | Desktop/WASM GUI binary (`nerthus`) on `bevy 0.20.0` + `bevy_egui 0.43.1`, plus embedded HTTP MCP server (`src/mcp/`) |
| `crates/nerthus-cli/Cargo.toml` | `nerthus-cli` | `2.3.1` | 25 | 585 | `e3b692081e56668fafe0ead122ca79269a4cc0b5865ce3c07120a4ac278ef766` | Headless CLI build runner (`nerthus-swarm`) compatible with `Gaea.Swarm.exe` flags |

### 2.3 `RustGIS-main` Manifests (17 Files)

| Manifest Path (relative to `RustGIS-main`) | Package Name | Version | Lines | Bytes | SHA-256 | Primary Role |
|---|---|---|---:|---:|---|---|
| `Cargo.toml` | *(workspace root)* | `0.4.0` | 138 | 5,816 | `d5e37e82fd3252b069cdbe8993efde37048e2a4826275eb987b79c807168d6f5` | Workspace root (16 members), `Apache-2.0`, `unsafe_code = "forbid"`, panic/unwrap deny lints |
| `crates/geom/Cargo.toml` | `rustgis-geom` | `0.4.0` | 14 | 302 | `d19ed8aa67fb2206af494cd42a940847508ec0bc1194f91d43df397cd6b85944` | WGS84/planetary geodesy (`geographiclib-rs`), Karney polygon area, 12 `BODIES`, Web Mercator, Sinusoidal/Goode Homolosine, ear-clip triangulation |
| `crates/data/Cargo.toml` | `rustgis-data` | `0.4.0` | 74 | 2,901 | `235a2bc6618d87ee61b550d8d17eca0715a09be03eebdc0f8a42fc9c5e1935f8` | 16 vector/point-cloud formats (`GeoJSON`, `Shapefile`, `GeoPackage`, `GeoParquet`, `FlatGeobuf`, `FileGDB`, `LAS`/`LAZ`/`COPC`, `OSM PBF`, `DXF`), OGC web services (`WFS`, `OGC API Features`, `WMS`, `WMTS`, `WCS`, `CSW`, `SOS`, `SensorThings`, `STAC`, `ArcGIS REST`), `GeoTIFF`/`COG`/`Zarr v2/v3`/`NetCDF-3`/`GRIB2`, `proj4rs` + WKT/ESRI CRS |
| `crates/map/Cargo.toml` | `rustgis-map` | `0.4.0` | 26 | 912 | `d2a3c6f040f7afc4d314d4e1b4410599e24aaec8efbdaeb247317e13f7b48267` | `MapDocument`, `Layer`, `Symbology` (Graduated Jenks, Bivariate, Heatmap, FlowMap, MIL-STD-2525D), 18 `PRESETS` projections, `GlobeView`, `PageLayout` + `MapSeries`, `Report`, `Presentation`, `Notebook`, QGIS `.qgs`/`.qgz`/`.qml` |
| `crates/analysis/Cargo.toml` | `rustgis-analysis` | `0.4.0` | 19 | 466 | `26620cfcbcf6784cdfc8c7f6333f818876a32e6589eb95efd06eb56b0231ee57` | 21 GP tools (`buffer`, `clip`, `erase`, `intersect`, `union`, `dissolve`, `spatialJoin`, `zonalStats`), Ordinary `kriging`, `viewshed`, `contours`, Dijkstra `RoadGraph`, 6 `topology` rules/fixes, `a5` pentagonal DGGS, 2-layer `Mlp` neural classifier |
| `crates/render/Cargo.toml` | `rustgis-render` | `0.4.0` | 31 | 853 | `5e9e1e03a47564dfd84893dded280ef8675ca40d0205bf7d574792e38bb85a5a` | Dual CPU (`tiny-skia`) & camera-relative `f64`$\rightarrow$`f32` `lyon` GPU tessellator (`gpu_scene.rs`, `Capped<B>`), `cosmic-text` complex-script/RTL curved labels, `ColorVision` deficiency simulation, PDF/SVG/GeoTIFF/glTF/OBJ exporters |
| `crates/engine/Cargo.toml` | `rustgis-engine` | `0.4.0` | 42 | 1,393 | `c31b691cde7654bc2977b1091e025a692b2a2e387d35c56d7fb2afcf6f4e880c` | Command-centric `Session`, `Registry`, 51 command modules, `PutAside` CoW delta undo/redo, `.rgis` ZIP project I/O, `Model` geoprocessing DAG |
| `crates/ui-egui/Cargo.toml` | `rustgis-ui-egui` | `0.4.0` | 50 | 1,664 | `d0ac60f275f94488343607ad2a5748bfce3bc6bb947a6a256ac4ee959e5b0fad` | `egui 0.37` / `egui-wgpu 0.37` GUI shell, 11-tab Ribbon, 26 RightPanes, `vertexedit.rs` shared-vertex topological editor, 19-locale i18n (with RTL Persian) |
| `crates/geolibre/Cargo.toml` | `rustgis-geolibre` | `0.4.0` | 25 | 947 | `1990cdbf936dd09eba3043655920607fe28e652fc1cb4cd1944f96591d0c1665` | Bridge to 1,000+ GeoLibre/Whitebox (`geolibre` CLI / WASI `.wasm`) terrain, hydrology, LiDAR, and SAR tools |
| `crates/plugins/Cargo.toml` | `rustgis-plugins` | `0.4.0` | 42 | 1,607 | `55342bc28729befe413c5da82de922e7fd5e9ea93f92d8f127ede61ae40def21` | `wasmi 0.40` sandboxed WebAssembly plugin runtime (64 MiB memory cap, 500M fuel metering, capability-gated host calls) |
| `crates/tiles/Cargo.toml` | `rustgis-tiles` | `0.4.0` | 40 | 1,479 | `2754de7cd481ddfe8a4974e01f0e658d90fbecf6b2df606ce72c7c2f0739a7f3` | Zero-protoc Mapbox Vector Tile v2.1 decoder (`mvt.rs`), PMTiles v3 Hilbert directory reader (`pmtiles.rs`), MapLibre GL Style v8 evaluator (`style.rs`), AWS SigV4/GCS/Azure signer (`s3.rs`) |
| `crates/assistant/Cargo.toml` | `rustgis-assistant` | `0.4.0` | 36 | 1,360 | `a30e0c5382e0864440961e7d87f7667a4c12b18da007a1f854d8cd331ddb003e` | Model Context Protocol (`mcp.rs`, JSON-RPC 2.0 `2024-11-05`) server & LLM client (`client.rs`) |
| `apps/rustgis/Cargo.toml` | `rustgis` | `0.4.0` | 51 | 2,530 | `ffdb5c8b7641c96cc9100530495971cce6a3eb75b6bd6cde02c54d3d3089e574` | Native desktop `eframe 0.37` binary with `127.0.0.1:7879` MCP attach bridge |
| `apps/rustgis-cli/Cargo.toml` | `rustgis-cli` | `0.4.0` | 16 | 398 | `e5310a4cc350274f9a47c696f9692d2f9ea4b24b940cfe3e05bd20d7bfca30de` | Headless CLI (`commands`, `exec`, `script`, `convert`, `render`, `inspect`, `server --attach`) |
| `apps/rustgis-python/Cargo.toml` | `rustgis-python` | `0.4.0` | 26 | 655 | `6cac5b3183cf9baed3eabd8a23e6f507af975f5f05aa29bf9b284703baa397c7` | PyO3 (`0.24`, `abi3-py39`) Python bindings for `Session` and GP tools |
| `apps/rustgis-web/Cargo.toml` | `rustgis-web` | `0.4.0` | 29 | 1,503 | `452a21cb2b9422da053043c284d4a2a7b47874825efcdb3e61df39662f34eca6` | `wasm32-unknown-unknown` browser app (`eframe` WebGPU/WebGL, OPFS project cache) |
| `xtask/Cargo.toml` | `xtask` | `0.4.0` | 9 | 135 | `bf54c562cfdf88b4649c799e2b68157f9cd107f805ef0b64a0e2667b3b78ec58` | Packaging & build automation |

---

## 3. Candidate Crate Classification Matrix (All Three Workspaces)

| Crate | Workspace | License | Initial Disposition | Target QualiaDB / Poet / Craft Owner |
|---|---|---|---|---|
| `earthcraft-core` | `EarthCraft` | MIT | **Reuse / Adapt** | `qualia-geospatial` (Karney WGS84 geodesics, Bowring inversion, `DD`/`DMS`/`DDM`/`UTM`/`UPS`/`MGRS`, geodesic `Ruler`) + `qualia-core-db` (`CelestialFrame` with SOFA notice, `sun_dir`, `OrbitElements`, `FlyAnim`) |
| `earthcraft-data` | `EarthCraft` | MIT | **Adapt Bounded Subsets** | `qualia-geospatial` (guarded KML/KMZ parser `kml/guard.rs`, prefix search, `.mbtiles` cache pattern); **exclude `updater.rs`**; keep live HTTP feeds optional |
| `earthcraft-render` | `EarthCraft` | MIT | **Adapt via Seam** | `qualia-geospatial` (`geo-index`/`rstar` `BoundaryIndex` & `LabelIndex`) + `qualia-core-db/src/render` (`wide::f64x4` ray-ellipsoid/DEM refinement & hybrid GPU/CPU limb/seam repair on Etalon) |
| `apps/earthcraft` | `EarthCraft` | MIT | **Reference Only** | Map 28 commands, coordinate readout bar, geodesic ruler workflow, and KML Places tree into native Poet Aether views |
| `nerthus-noise` | `Nerthus` | MIT | **Reuse / Adapt** | `qualia-core-db` / `qualia-geospatial` procedural terrain synthesis (deterministic `FastNoiseSIMD`, `Perlin`, `Simplex`, `Cellular`, `Gabor`, x86 `RCPPS`/`RSQRTPS` LUTs) |
| `nerthus-erosion` | `Nerthus` | MIT | **Reuse / Adapt (Tier-2 Bounded)** | `qualia-geospatial` / `qualia-sim` cold terrain builders (`ce` droplet/thermal, `cs` snow, `cw` lake/sea `SpanFill`, `hydro` priority-flood drainage & river carving, `ts` thermal shaper) |
| `nerthus-engine` | `Nerthus` | MIT | **Adapt via Seam (Tier-2 Bounded)** | `qualia-geospatial` / Poet terrain graph session (`Catalogue`, topological DAG evaluation, `.terrain` JSON I/O, wrapped in `GeometryWorkspace` byte budget) |
| `nerthus-nodes` | `Nerthus` | MIT | **Reuse / Adapt (Tier-2 + WGSL Forge)** | `qualia-geospatial` + `wgsl_forge` (182 node builders, 19 modifiers, 8.24 fixed-point `ParallelErosion_Erode`, 12 WGSL compute shaders, QEM decimation, Sentinel-2 CLUTs) |
| `nerthus-compute` | `Nerthus` | MIT | **Adapt via Seam (Exclude `nvidia_profile.rs`)** | Rebind `GpuDevice` batching (`DISPATCHES_PER_SUBMISSION = 64`) to Etalon / `wgsl_forge`; **delete `nvidia_profile.rs`** |
| `nerthus-export` | `Nerthus` | MIT | **Adapt Bounded Subsets** | `qualia-geospatial` / Poet export producers (tiled builds, `mesher.rs`, `EXR`/`TIFF32`/`PNG16`/`RAW16` heightfield & mask export, bridged to `.m64` `M64StrataColumnRecord` and `.10d` `FIEL`/`MESH`) |
| `nerthus-render` | `Nerthus` | MIT | **Adapt Shaders / Reference Plugin** | Adapt `sky.wgsl` (Bruneton BSD-3-Clause notice preserved) and `terrain.wgsl` (`Data`/`Clay`/`Realistic` modes) into `QualiaPortal`; do not import Bevy `0.20` plugin |
| `nerthus-app`, `nerthus-cli` | `Nerthus` | MIT | **Reference Only (Adapt `terrain_report.rs`)** | Adapt `mcp/terrain_report.rs` geomorphology metrics into `NativeProject::mcp_backend()`; **exclude `file_association.rs` and `nahimic.rs`** |
| `rustgis-geom` | `RustGIS` | Apache-2.0 | **Reuse / Adapt** | `qualia-geospatial` (Karney `geodesic_polygon_area_m2`, 12 planetary `BODIES`, `ear_clip` hole-bridging polygon triangulator) |
| `rustgis-data` | `RustGIS` | Apache-2.0 | **Reuse / Adapt (Tier-2 + Streaming)** | `qualia-geospatial`: stream `lidar.rs` (`laz` LAS/LAZ/COPC) into `.m64` `E64PointRecord`; emit `.q42` `NQuin` `geo:Feature`/`sosa:Observation` from `GeoPackage`, `GeoParquet`, `FlatGeobuf`, `FileGDB`, `OSM PBF`, `WFS 2.0`, `OGC API Features`, `SOS`/`SensorThings`; compile `cog.rs`, `WCS`, and `zarr.rs` (`Zarr v2/v3`, `NetCDF-3`, `GRIB2`) into `GRM3`/`.10d` `FIEL`; reuse `crs.rs` (`proj4rs` + WKT1/WKT2/ESRI) |
| `rustgis-analysis` | `RustGIS` | Apache-2.0 | **Reuse / Adapt (Tier-2 Bounded)** | `qualia-geospatial` / `qualia-core-db` cold analysis operators (`buffer`, `clip`, `intersect`, `dissolve`, `spatialJoin`, `zonalStats`, Ordinary `kriging`, `viewshed`, `contours`, Dijkstra `RoadGraph`, 6 `topology.rs` rules/auto-fixes, `a5` DGGS) inside `GeometryWorkspace` |
| `rustgis-tiles` | `RustGIS` | Apache-2.0 | **Reuse / Adapt** | `qualia-geospatial` / `webizen-render` (`MVT` v2.1 protobuf decoder `mvt.rs`, `PMTiles` v3 Hilbert archive reader `pmtiles.rs`, `MapLibre` GL style evaluator `style.rs`) |
| `rustgis-render` | `RustGIS` | Apache-2.0 | **Adapt via Seam** | `webizen-render` / `QualiaPortal` (camera-relative `f64`$\rightarrow$`f32` `lyon` tessellator `gpu_scene.rs` with `Capped<B>` vertex budget, `cosmic-text` curved/RTL labels, `ColorVision` deficiency filters, `pdf.rs`/`svg.rs`/`export_scene_gltf`) |
| `rustgis-map` | `RustGIS` | Apache-2.0 | **Adapt via Seam** | Poet Geospatial/Document/Presentation Manifolds (`Symbology`, 18 `PRESETS` projections, `PageLayout` + `MapSeries`, QGIS `.qgs`/`.qgz`/`.qml` import/export) |
| `rustgis-engine` | `RustGIS` | Apache-2.0 | **Adapt Bounded Patterns** | Poet domain session (`PutAside` CoW delta undo/redo `MapMeta`/`LayersMeta`/`OneLayer` and `Model` geoprocessing DAG `model.rs`); bind command execution to `AuthorizedEditorHost` |
| `rustgis-geolibre` | `RustGIS` | Apache-2.0 | **Optional Adapter / Hold** | Optional external/WASI runner for 1,000+ GeoLibre/Whitebox tools using `tempfile::TempDir` RAII cleanup (`qualiaDB/AGENTS.md` §0-B) |
| `rustgis-plugins`, `rustgis-assistant`, `rustgis-ui-egui`, `apps/rustgis*` | `RustGIS` | Apache-2.0 | **Reference Only (Adapt `vertexedit.rs`)** | Adapt `ui-egui/src/vertexedit.rs` (`edit.moveSharedVertex` + `DragOwner`) into Poet's vector/corridor editor; route MCP tools through `NativeProject::mcp_backend()` rather than `127.0.0.1:7879` |
