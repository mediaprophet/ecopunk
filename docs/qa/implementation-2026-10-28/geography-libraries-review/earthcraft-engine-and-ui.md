# `EarthCraft-main` (`v0.2.0`) — Engine, Geodesy, Data, Renderer & UI Audit

**Review date:** 2026-10-28 · **Status:** read-only source audit of `C:/github/geography-libraries/EarthCraft-main` (157 total files: 5 `Cargo.toml` manifests, 108 Rust files, 1 WGSL shader, 2 Fluent `.ftl` catalogs). No Cargo build, test, GUI, or network request was run. Exact SHA-256 digests are in [`coverage-and-pins.json`](coverage-and-pins.json).

---

## 1. Executive Summary for Poet & QualiaDB Integration

`EarthCraft` is a pure-Rust (`unsafe_code = "forbid"`), MIT-licensed 3D planetary globe explorer. Its core library (`earthcraft-core`) and selected spatial/caching/rendering modules (`earthcraft-data`, `earthcraft-render`) provide production-ready mathematical and geospatial building blocks that directly upgrade `qualia-geospatial`, `qualia-core-db`, and native Poet:
- **Ellipsoidal & Spherical Geodesy**: WGS84 (G2296) oblate ellipsoid ($a = 6378.137\text{ km}$, $f = 1/298.257223563$) and IUGG mean sphere ($R = 6371.0088\text{ km}$), Bowring closed-form geodetic inversion, ray-ellipsoid intersection, and Karney's ellipsoidal geodesic inverse/direct path solver (`geographiclib-rs`).
- **Comprehensive Coordinate Systems**: Bidirectional parsing and formatting for `DD`, `DMS`, `DDM`, `UTM`/`UPS` (with Norway `32V` and Svalbard `31X–37X` special zones), and `MGRS` (`1 m` precision).
- **Astronomical Ephemeris & Satellite Propagation**: IAU 2006/2000A precession-nutation (`sofars::c2t06a`), ICRS-to-Galactic sky transformation (`CelestialFrame`), solar terminator (`sun_dir`), and Newton-Raphson Keplerian two-body satellite propagator (`OrbitElements`).
- **Interactive Geodesic Ruler & Guarded KML/KMZ Tree**: Multi-path ellipsoidal distance/bearing measurement with 128-step undo/redo (`Ruler`), hierarchical KML 2.2 `<Folder>`/`<Placemark>` management (`Places`), and anti-zip-bomb / XML entity-expansion security guards (`kml/guard.rs`).
- **Ray-Anchored Globe Camera & Hybrid SIMD/GPU Rendering**: Cursor-locked ray-ellipsoid pan/zoom (`camera.rs`), smoothstep geodesic `FlyAnim` (`flight.rs`), `wide::f64x4` 4-lane SIMD ray-casting (`rays.rs`), Mapterhorn Terrarium DEM ray-refinement (`terrain.rs`), and hybrid f32 GPU compute overview + f64 CPU horizon/seam precision repair (`gpu/overview.rs`, `gpu/repair.rs`).

---

## 2. `crates/earthcraft-core`: Geodesy, Coordinates, Camera, Celestial & Domain State

| Module / File | Inspected Lines | Key Types & Functions | QualiaDB / Poet Adaptation Seam |
|---|---:|---|---|
| `src/earth_model.rs` | `1–280` | `EarthModel::{Wgs84, PerfectSphere}`, `unit_position(lat, lon)`, `surface_normal(lat, lon)`, `geodetic_degrees(point)` (Bowring closed-form inversion, `116–143`), `intersect_ray(origin, dir)` (`161–190`), `distance_km(a, b)`, `initial_bearing(a, b)`, `sample_path(a, b, steps)` (`192–279` via `geographiclib_rs::Geodesic::wgs84()`) | Expose Karney ellipsoidal geodesics, Bowring inversion, and ray-ellipsoid intersection inside `qualia-geospatial::frame` alongside `GeodeticFrame` (`EPSG:7844` GDA2020 + `EPSG:5711` AHD). |
| `src/math.rs` | `1–150` | `sun_dir(unix_secs)` (`27–61`), `haversine_km(a, b)` (`64–73`), `bearing_deg(a, b)` (`76–84`), `sample_great_circle(a, b, steps)` (`87–116`) | Bind `sun_dir(unix_secs)` to `qualia-sim::atmosphere` and `QualiaPortal` solar forcing (`BSA-10`) so scene lighting matches exact locality date/time. |
| `src/coordinates.rs`, `src/coordinates/input.rs`, `src/coordinates/display.rs` | `1–158`, `1–246`, `1–243` | `CoordinateFormat::{DecimalDegrees, DegreesMinutesSeconds, DegreesDecimalMinutes, UtmUps, Mgrs}`, `parse_coordinate_query(query)` (`input.rs:28–56`), `format_coordinates(lat, lon, format)` (`display.rs`) | Integrate into `qualia-geospatial` and Poet's spatial status bar / search bar so operators and agents can enter or inspect `DD`, `DMS`, `DDM`, `UTM` (e.g., MGA Zone 55 `55H` for Kosciuszko), or `MGRS` coordinates. |
| `src/camera.rs` & `src/flight.rs` | `1–527`, `1–136` | `Camera`, `Basis` (`pick`, `project_unit`, `ray_ndc`), `pan_by_pixels_for_model` (`76–119`, `solve_camera_keep_anchor`), `zoom_at_ndc_for_model` (`121–157`), `FlyAnim` (`flight.rs:6–68`, smoothstep $t^2(3-2t)$ geodesic path + parabolic log-altitude arc) | Drive `qualia_core_db::domains::geospatial::scale_continuum` (`requires_micro_transition`) and Poet's 3D viewport camera for seamless planetary $\rightarrow$ regional $\rightarrow$ local walk/lab transitions (`ECO-091/096–100`). |
| `src/celestial.rs` & `src/orbit.rs` | `1–86`, `1–160` | `CelestialFrame::from_unix_seconds(unix_secs)` (`20–72`, `sofars::c2t06a` IAU 2006/2000A precession-nutation + ICRS $\rightarrow$ Galactic $R_{\text{gal}}$), `OrbitElements::propagate(unix_seconds)` (`42–135`), `ground_track` | Orient `QualiaPortal` night-sky / Milky Way backdrop accurately in time and space (preserving `sofars-LICENSE.txt`); optional satellite pass prediction for Earth-observation / telemetry scenarios. |
| `src/ruler.rs` & `src/units/ruler.rs` | `1–301`, `1–298` | `Ruler` (`18–246`, multi-path `Vec<Vec<(f64, f64)>>`, `MAX_HISTORY = 128` undo/redo, drag coalescing `begin_move`/`move_point`/`end_move`, `distance_km`, `last_leg`), `RulerUnit` (9 metric/imperial/nautical/angular units) | Mount as a native Poet spatial measurement tool (`BSA-08`, `ECO-091/098`), emitting typed IntentBus commands and binding measured paths to `.q42` provenance when saved. |
| `src/placemark/tree.rs` | `1–205` | `Places`, `PlaceNode`, `PlaceItem::{Folder, Marker}`, recursive visibility, search, cycle-checked `move_into` | Map KML folders and placemarks to `did:q42:` feature identifiers (`geo:Feature` + `GeodeticCoord`) in Poet's Places/Outliner panel. |
| `src/controller.rs`, `src/language.rs`, `src/localization.rs` | `1–286`, `1–209`, `1–87` | `ControllerProfile` (5 axes with radial/axial deadzones & power curves `1.0..=3.0`, 19 button mappings), `Lang` (`en.ftl`, `pt-BR.ftl`), `fold_name` diacritic folding | Reuse gamepad response-curve math (`ControllerProfile`) and diacritic-insensitive place search folding (`fold_name`). |

---

## 3. `crates/earthcraft-data`: Caching, Guarded KML/KMZ, Boundaries & Live Feeds

1. **Two-Tier Tile Cache (`src/tiles/{mod,cache,disk,warmup}.rs`, `src/tiles/disk/mbtiles/{mod,budget}.rs`)**:
   - **L1 Memory Cache**: `moka::sync::Cache<TileKey, Arc<Tile>>` weighted by byte size (`256 * 256 * 4 = 262,144 B` per RGBA8 tile, configurable `64..=4096 MiB`).
   - **L2 Disk Cache (`MbtilesStore`)**: Per-provider SQLite `.mbtiles` store (`tiles/disk/mbtiles.rs:1–325`) with TMS $y$-flip (`(1 << z) - 1 - y`), WAL mode, and automatic budget pruning (`budget.rs:1–146`, evicting oldest-accessed batches + `PRAGMA incremental_vacuum`).
   - **Camera-Centered Priority Queue (`cache.rs:88–165`)**: Prioritizes tile requests by zoom level and squared distance from the viewport center.
2. **Hardened KML/KMZ & `.earthplanet` Security Guard (`src/kml/{guard,kmz,mod}.rs`, `src/planet.rs`)**:
   - `kml/guard.rs:1–121` enforces fail-closed bounds before parsing untrusted files:
     - Max KML payload: `32 MiB`.
     - Max KMZ archive: `64 MiB`, max ZIP entries: `256`, max compression ratio: `100:1`.
     - Rejects XML DTD / entity expansion attacks (`<!DOCTYPE`, `<!ENTITY` in `reject_xml_declarations`).
   - **Adaptation**: Directly reusable in `qualia-geospatial` and Poet's `ResourceImportHost` for safe KML/KMZ feature ingestion.
3. **Administrative Boundaries & Offline/Online Search (`src/boundaries/*`, `src/search.rs`, `src/search/{prefix,local}.rs`)**:
   - Ingests GeoJSON / Natural Earth / GeoBoundaries polylines (`ADM0`, `ADM1`, `ADM2`) and builds an in-memory prefix/trigram index (`PrefixIndex` in `search/prefix.rs:1–188`) for zero-latency offline place lookup, with optional online Photon/Nominatim fallback.
4. **Live Telemetry Feeds (`src/{aircraft,vessels,satellites,roads,traffic}.rs`, `src/live_maps/*`)**:
   - Decoders for OpenSky/ADSB.lol aircraft (`aircraft/decode.rs`), OpenSeaFeed/Digitraffic marine AIS (`vessels.rs`), CelesTrak GP JSON (`satellites/decode.rs`), Overpass OSM highways (`roads.rs`), TomTom flow segments (`traffic.rs`), and RainViewer radar (`live_maps/mod.rs`).
   - **Adaptation**: Keep strictly optional behind authorized Webizen host network policies; map admitted live observations to `sosa:Observation` event NQuins (`spatial-substrate.md` §1).

---

## 4. `crates/earthcraft-render`: SIMD Ray-Caster, DEM Displacement, Hybrid GPU & R-Tree Overlays

1. **`wide::f64x4` CPU SIMD Ellipsoidal Ray-Caster (`src/lib.rs:1–365`, `src/rays.rs:1–304`, `src/pixel.rs:1–264`)**:
   - Renders in parallel 8-row horizontal strips (`ROWS_PER_TILE_BATCH = 8`) via `rayon`, specialized via `render_row<const ELLIPSOID: bool>`.
   - `RayCaster` (`rays.rs:10–197`) evaluates 4 pixels per loop using `wide::f64x4` double-precision SIMD lanes (`cast_sphere_lanes` / `cast_ellipsoid_lanes`), eliminating single-precision jitter at high zoom levels.
   - Thread-local `Local` tile map (`lib.rs:64–131`) caches `Arc<Tile>` pointers per 8-row strip to avoid atomic contention on the global cache.
2. **Terrain DEM Ray-Displacement (`src/terrain.rs:1–129`)**:
   - `TerrainSampler::displace` (`terrain.rs:58–122`) decodes Mapterhorn Terrarium elevation tiles ($\text{elev\_m} = (R \cdot 256 + G + B/256) - 32768$, matching `qualia_geospatial::decode_terrarium_mm`), performs 2 iterative ray-march refinements along the view ray (` altitude <= 150 km`), and computes finite-difference east/north slopes for hillshading.
3. **Experimental GPU Overview Renderer + CPU Limb/Seam Repair (`src/gpu/{mod,overview,repair}.rs`, `src/gpu/shaders/overview.wgsl:1–129`)**:
   - `overview.wgsl` (`@compute @workgroup_size(256)`) casts f32 rays against a 2048-slot GPU hash table backed by a `256x256` `texture_2d_array<f32>` (up to 512 layers = 128 MiB) and tracks missing tile distances via `atomicMin`.
   - **Hybrid Precision Pattern (`gpu/repair.rs:1–68`)**: Because f32 rays lose precision near the planetary horizon limb (`disc < 0.01`) and at tile UV borders (`< 0.0001`), `overview.wgsl` tags those pixels with `0u` and `repair.rs` recomputes only those boundary pixels on the CPU in `wide::f64x4` double precision.
4. **Spatial & Screen-Space R-Tree Indexing (`src/overlays/{spatial,labels,grid,ruler}.rs`)**:
   - `BoundaryIndex` (`overlays/spatial.rs:141–202`): Static Hilbert-packed R-tree (`geo_index::rtree::RTree<f64>`) over 3D trigonometric bounding boxes (`sin/cos` lat/lon), queried by spherical cap chord radius and refined against the horizon plane + 4 frustum planes.
   - `LabelIndex` (`overlays/labels.rs:16–39`): Dynamic screen-space `rstar::RTree<LabelRect>` preventing label overlap (`MAX_VISIBLE_LABELS = 500`, prioritized by `Country > State > Metro > City`).

---

## 5. Governing UI-Category Matrix (`apps/earthcraft`)

| Governing UI Category | Inspected Source Evidence (`apps/earthcraft/src/`) | Poet Destination & Adaptation Status |
|---|---|---|
| **1. Commands** | 28 unified menu/toolbar actions across 8 sections (`Application`, `File`, `Edit`, `View`, `Add`, `Tools`, `Window`, `Help`) in `app/menu/commands.rs:41–180` and `app/menu/mod.rs:64–156`. | Map geodetic, ruler, placemark, layer, and fly-to commands into Poet's `ToolChest` and `NativeIntentBus` (`qualia-app-contract` `CommandDescriptor`). Exclude `Quit` and self-update actions. |
| **2. Panels** | Top bar (`ui/bars.rs:14–121`), bottom coordinate/altitude/heading status bar (`ui/bars.rs:123–187`, `ui/coordinate_display.rs`), and collapsible left sidebar (`ui/sidebar.rs:1–185`) with `Explore`, `Layers`, and pin-able `Places` outline panels (`ui/sidebar/{explore,layers,places}.rs`). | Integrate coordinate/altitude/datum readout into Poet's spatial viewport status bar; mount `Layers` and KML `Places` inside Poet's spatial/environment Manifold containers. |
| **3. Gestures** | Ray-anchored ground pan (`globe.rs:43–52`), heading rotate drag (`globe.rs:53–65`), cursor-anchored exponential zoom (`globe.rs:66–98`), double-click `FlyAnim` (`globe.rs:189–200`), on-canvas compass/pan/zoom HUD (`ui/navigation.rs:26–344`), ruler vertex click/drag/delete (`globe/ruler_input.rs:1–259`), and KML drag-and-drop (`app/files.rs:155–204`). | Emit typed Poet viewport/gesture intents (`qualia-app-contract` gesture/session contract) so interactive ruler edits and camera moves share state with agent tools. |
| **4. Shortcuts** | `Cmd/Ctrl+O/S/Shift+S` (KML I/O), `Cmd/Ctrl+Shift+E` (Screenshot), `Cmd/Ctrl+Z/Y` (Ruler undo/redo in `ruler_history.rs:47–61`), `Cmd/Ctrl+Shift+P/N` (Add Placemark/Folder), `Cmd/Ctrl+Alt+B` (Sidebar), `Arrow` keys (pan), `+/-`/`PageUp/Down` (zoom), `N` (North reset) in `globe.rs:99–144`. | Reconcile with Poet's global shortcut table inside the spatial Manifold; preserve `N` (North reset) and ruler undo/redo semantics. |
| **5. Context Menus** | Globe right-click menu (`globe.rs:433–496`: formatted coordinate header + `Add placemark here`, `Fly to here`, `Measure from/to here`, `Copy coordinates`) and Places tree right-click menu (`ui/places.rs:145–156`: `Fly to`, `Delete`). | High-utility spatial context menu pattern; adapt directly into Poet's 3D/2D geospatial viewport context menu. |
| **6. Dialogs & Settings** | Live-filtered searchable Settings window (`ui/settings.rs:117–235`), Exact Ruler Points Editor (`ui/ruler/points.rs:13–231`, 8-decimal `{:.8}` precision), Camera Altitude Editor (`ui/altitude.rs:34–154`), Solar System / `.earthplanet` Explorer (`app/explorer.rs:1–294`), and Live Object telemetry cards (`app/live_layers.rs:171–331`). | Adapt the 8-decimal Ruler Points Editor and exact Altitude/Coordinate input popovers as Poet inspector popovers. |
| **7. Views** | Adaptive-resolution 3D WGS84/Sphere globe view (`globe.rs:25–505`, 50% scale during drag, 100% after `250 ms` idle), planetary UV-sphere view (`planet.rs:1–121`), and Easter-egg 3D Flat Earth view (`flat_earth/scene.rs:1–312`). | Reuse adaptive interaction resolution and `BoundaryIndex`/`LabelIndex` overlay projection in Poet's spatial viewport; omit the Flat Earth Easter egg. |
| **8. Accessibility & Localization** | Embedded bilingual Fluent catalogs (`en.ftl`, `pt-BR.ftl`, 472 keys each), independent UI Scale (`0.75–2.0x`) and Map Label Scale (`0.75–2.0x`), custom font selection (`fontdb`), vector icons with `egui::WidgetInfo::selected` accessibility state (`rail.rs:26–33`), and full `gilrs` gamepad remapping. **Note**: Despite `README.md:89` mentioning narrated tours, **no TTS/audio synthesis crate exists in `EarthCraft-main` source**. | Reuse `ControllerProfile` gamepad curve math and `WidgetInfo` semantic labeling patterns; provide any voice/narration capabilities via `qualia-audio` rather than EarthCraft. |
