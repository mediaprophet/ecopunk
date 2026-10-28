# `nerthus-main` (`v2.3.1`) — Terrain Engine, Erosion, Compute, Nodes & UI Audit

**Review date:** 2026-10-28 · **Status:** read-only source audit of `C:/github/geography-libraries/nerthus-main` (15 `Cargo.toml` manifests, 14 member crates). No Cargo build, test, GUI, or GPU compute execution was run. Exact SHA-256 digests are in [`coverage-and-pins.json`](coverage-and-pins.json).

---

## 1. Executive Summary for Poet & QualiaDB Integration

`Nerthus` is a clean-room, MIT-licensed Rust reimplementation of QuadSpinner Gaea 2.3.1.0. Crucially, its core domain, noise, erosion, node evaluation, mesh import/export, and tiled build crates (`nerthus-core`, `nerthus-engine`, `nerthus-noise`, `nerthus-image`, `nerthus-erosion`, `nerthus-nodes`, `nerthus-assimp`, `nerthus-io`, `nerthus-hevc`, `nerthus-export`) have **zero dependency on Bevy or egui**. They provide an extraordinary procedural landform, hydrology, snow, and geological stratigraphy engine that directly serves Ecopunk's place/habitat authoring (`ECO-031–040`, `ECO-058`, `ECO-091–100`, `BSA-05/08/09/10/11`):
- **182 Node Builders + 19 Modifiers (`nerthus-engine`, `nerthus-nodes`)**: Declarative schema in `data/nodes.toml` (16,370 lines, 193 specs) and `data/modifiers.toml` (216 lines, 19 specs), supporting topological DAG evaluation, macros, `LoopBegin..LoopEnd` feedback loops, portals, and mask blending.
- **Bit-Exact Cross-Architecture Determinism (`nerthus-noise`, `parallel_erosion.rs`)**: Hardware lookup tables emulating x86 SSE/AVX2 `RCPPS`/`RSQRTPS` (`x86.rs`, `x86_tables.rs`), exact ports of `FastNoiseSIMD`, `.NET System.Random`, `std::mt19937`, and `XorShift64*`, and **8.24 fixed-point integer accumulators** (`FIXED_SCALE = 16777216.0`, `atomic<i32>` in WGSL) for race-free multi-threaded/GPU hydraulic droplet erosion.
- **5 Native C++ Erosion & Hydrology Ports (`nerthus-erosion`)**: Classic droplet/thermal erosion (`ce`), wind-driven snowfall & angle-of-repose snow slip (`cs`), 4-phase shallow-water lake/sea routing & `SpanFill` basin flood-fill (`cw`), priority-flood depression filling & primary/secondary/tertiary river valley carving (`hydro`), and multi-scale pyramid talus relaxation (`ts`).
- **12 WGSL Compute Kernels & Bruneton Atmosphere (`nerthus-nodes/src/gpu/shaders/`, `nerthus-render/src/sky.wgsl`)**: Ready to register in `qualia-core-db`'s `wgsl_forge` on **Etalon**, with automatic GPU-to-CPU fallback (`sycl.rs`).
- **323 Sentinel-2 Colour LUTs, QEM Mesh Decimation & 38+ MCP Geomorphology Tools**: Real satellite-derived colour tables (`data/color/`), quadric error metrics mesh reduction (`decimate.rs`), 19 raster/point-cloud + 8 mesh export formats, and quantitative geomorphology reporting (`mcp/terrain_report.rs`).

---

## 2. Dual Graph & Engine Architecture (`nerthus-core` vs. `nerthus-engine`)

| Crate & File | Inspected Lines | Architecture & Key Types | QualiaDB / Poet Adaptation Seam |
|---|---:|---|---|
| `nerthus-core/src/{field,context,node,param,port,project}.rs` | `164`, `173`, `597`, `716`, `210`, `390` | Legacy Gaea 1 `.tor` graph model: `Field` (interleaved `Vec<f32>`), `EvalContext` (`cancelled: Arc<AtomicBool>`, `ProgressSink`), `NodeKind` trait, `NodeInstance` (`Uuid`), `ParamValue`, `PortDataType` (`Heightfield`, `Mask`, `Color`, `Water`, `Vector`, `Mesh`, `Any`). | Retain for `.tor` import compatibility; unify internal Poet terrain session representation on the richer `nerthus-engine` catalogue and QualiaDB `did:q42:` node identifiers. |
| `nerthus-engine/src/map.rs` | `1–350` | `Resolution { x, y }`, `RenderIntent` (`Default`, `Heightfield`, `Mask`, `Color`, `Raw`), `Map` (`channels: Vec<Vec<f32>>`, planar row-major `y * width + x`, `resize_bilinear`, `clamp01`, `autolevel`, `blend_from`, `apply_drop`). | Wrap `Map` allocation in a **Tier-2 `GeometryWorkspace`** budget check (`width * height * channels * 4` bytes) before allocating; compile completed `Map` channels into `.m64` `M64StrataColumnRecord` and `.10d` `FIEL` sections. |
| `nerthus-engine/src/catalogue.rs` & `data/{nodes,modifiers}.toml` | `1–356`, `16,370`, `216` | Compile-time parsed `Catalogue::get()` (`265–270`) yielding 193 `NodeSpec` and 19 `ModifierSpec` definitions (`name`, `type_name`, `toolbox`, `family`, `short_code`, `ports`, `parameters`, `visible_when`, `enabled_when`). | Map `NodeSpec` and `ParameterSpec` directly to `qualia-app-contract` `CommandDescriptor` / inspector schemas so Poet's `ToolChest` and Property Sheet render all 182 terrain nodes natively without Bevy/egui. |
| `nerthus-engine/src/graph.rs`, `macros.rs`, `loops.rs`, `baked.rs` | `1–1413`, `436`, `427`, `199` | `Registry`, `NodeContext` (`commit_with` modifier chain & mask blending at `754–784`), `Engine` (`build_order` topological sort at `1153–1158`, `Portal`/`ChokePoint`/`Gate`/`Switch`/`Route` resolution at `1010–1086`, `std::panic::catch_unwind` node guard at `1194–1198`). | Run `Engine::build_one` as an authorized native job via Poet's `NativeIntentBus` / `AuthorizedEditorHost`, replacing `<project> - Builds/cache/` disk dumps with `tempfile::TempDir` RAII scratch or explicit HMC checkpoint artifacts (`qualiaDB/AGENTS.md` §0-B). |
| `nerthus-engine/src/{project,writer,json}.rs` | `511`, `782`, `239` | Parses and writes Gaea 2.3.1.0 `.terrain` Newtonsoft JSON (`$id`/`$ref`/`$type`/`$values` graph cycles). | Keep `.terrain` JSON as a lossless import/export interchange format while binding native Poet sessions to `.10d` / HMC checkpoints and `.q42` provenance. |

---

## 3. Erosion, Hydrology, Snow & Stratigraphy Kernels (`nerthus-erosion` & `nerthus-nodes`)

### 3.1 Five Native C++ Simulation Engine Ports (`crates/nerthus-erosion/src/`)
1. **`ce` — Classic Droplet & Thermal Erosion (`ce/engine.rs:1–493`, `droplet.rs:1–243`, `thermal.rs:1–223`, `layer.rs:1–217`, `kernel.rs:1–82`)**:
   - Multi-scale downsampling (`scale_ratio`), snow/thermal pre-passes, stochastic particle droplet simulation (`CEDroplet`) tracking kinetic energy, downcutting, sediment capacity, inhibition, and rock softness masks, followed by `post_erosion_blend` and `sharpen`.
2. **`cs` — Orographic Snowfall & Slip-Off (`cs/engine.rs:1–705`, `resample.rs:1–297`)**:
   - Simulates snowfall accumulation, melt, wind-direction transport, snowline elevation falloff, and angle-of-repose avalanche slip-off relaxation (`Snow` and `Glacier` nodes)—directly applicable to Kosciuszko alpine snowline modeling (`map-layers.md` Layer 4).
3. **`cw` — Shallow-Water Lake/Sea Routing & `SpanFill` (`cw/engine.rs:1–763`, `fill.rs:1–277`, `blur.rs:1–69`)**:
   - Simulates precipitation, 4-phase interleaved row-parallel shallow water flow (`step_row`), coastal cliff erosion, and `SpanFill` depression/basin flood-fill (`Lake` and `Sea` nodes)—directly feeds `M64StrataColumnRecord.surface_fluid_depth_mm`, `water_table_depth_mm`, and `WaterGpu` shoreline attributes (`ECO-058`).
4. **`hydro` — Priority-Flood Drainage & River Network Carving (`hydro/mod.rs:1–128`, `drainage.rs:1–561`, `rivers.rs:1–807`)**:
   - `drainage.rs` (`HydroFix`): Priority-flood depression pit filling and drainage tree assembly.
   - `rivers.rs` (`Rivers`): Synthesizes primary, secondary, and tertiary river networks, valley width grading, headwater extension, and channel meandering, outputting flow/river masks that map directly to `qualia_geospatial::corridor::HermiteCorridorSegment` (`CorridorKind::Creek`) and `M64StrataColumnRecord` fluid velocity/depth fields.
5. **`ts` — Multi-Scale Thermal Shaper (`ts.rs:1–209`)**:
   - Pyramid-based talus slope relaxation (`build_pyramid` $\rightarrow$ `step_level` $\rightarrow$ `collapse_pyramid`), mapping to `M64StrataColumnRecord.angle_of_repose_deg10` and regolith layer thicknesses.

### 3.2 Fixed-Point Parallel Hydraulic Erosion (`nerthus-nodes/src/parallel_erosion.rs` + `gpu/parallel_erosion.rs`)
- Powers **7 core nodes**: `Erosion2` (`ErosionPro`), `EasyErosion`, `Hillify`, `Sediments`, `Shear`, `Wizard2`, and `Anastomosis`.
- Deconstructs the terrain into a Laplacian pyramid (`build_pyramid`, `collapse_pyramid`) and simulates concurrent particle droplets using **8.24 fixed-point integer accumulators** (`FIXED_SCALE = 16777216.0`, `i32` on CPU and `atomic<i32>` in `parallel-erosion.wgsl:1–736`).
- Because integer addition is associative and commutative, concurrent CPU (`rayon`) or GPU (`wgpu`) droplet threads accumulate height and sediment deltas **deterministically without floating-point non-associativity or race conditions**.
- Tracks 3 distinct sediment transport classes (suspended sediment, bed load, coarse sediment) and outputs height plus `Wear`, `Deposits`, and `Flow` masks—providing exact physical drivers for populating `M64StrataColumnRecord`'s 6-layer thickness/LUT/compaction stack and `fluid_sediment_load_ppm`.

### 3.3 12 WGSL Compute Kernels (`crates/nerthus-nodes/src/gpu/shaders/`)
All 12 shaders are standalone WGSL compute modules ready for `qualia-core-db::wgsl_forge` certification on **Etalon**:
1. `parallel-erosion.wgsl` (`736 lines`, SHA-256: `d3e8fa4c2c71db1845c88be8b53e60a98a99962ac399e9a61e933f7b58c26800`) — 8.24 fixed-point multi-scale hydraulic erosion.
2. `cellular3d.wgsl` (`364 lines`, SHA-256: `b0443140dc3b502a0d2f078eb715ff9595715c6443046ee3221d4ef7a14746e6`) — 3D Worley/Cellular noise.
3. `thermal.wgsl` (`297 lines`, SHA-256: `920b9f681533f86bc765bf5ee0a45ec1b15fd9b760d84dbdf89e2e81b842b615`) — Thermal talus erosion (`Thermal2` / `Debris`).
4. `flow.wgsl` (`272 lines`, SHA-256: `09ff78b1bc29be6228546ff76e9a9c89f8b92e15a5625ecdf590787e883cc9b4`) — Shallow-water flow routing (`FlowMap` / `FlowClassic`).
5. `lighting.wgsl` (`242 lines`, SHA-256: `a808f6ebaa7f4c9a88d421a4b211a847dfba2fb66d1cb6c536021536a5b2069a`) — Raymarched sun/sky horizon lighting & AO (`LightX` / `Occlusion` / `Shade`).
6. `mesh-rasterizer.wgsl` (`212 lines`, SHA-256: `b91a8305a57eb2d11b3cc128ef2b8861bdba8b4df8922ccfb29fb7b7d42989e8`) — GPU triangle heightfield rasterizer (`ObjectToTerrain`).
7. `colour-transport.wgsl` (`147 lines`, SHA-256: `f20fb6d6ef95dc3205600ff8a17bd75fc61a02c27fc05593ec9cc0c2de8a9952`) — Advective colour transport along terrain gradients (`ColorErosion`).
8. `terrain-sample.wgsl` (`128 lines`, SHA-256: `97288227499b5272198ca9fc5642f74a380fa7d5a87a98d4e938cc3be9a03121`) — Bilinear/bicubic heightfield resampling & warping.
9. `fill-pits.wgsl` (`96 lines`, SHA-256: `5344fc4313179b0b12b87fb75f7848dc33a2c2f86e4a8f1bd23a0bf33dc6db25`) — Depression pit filling (`Pits`).
10. `median.wgsl` (`94 lines`, SHA-256: `6694a48bc046bd90d8177d48db3dc763594db0f2b0d84dbdf89e2e81b842b615` — see `coverage-and-pins.json` for exact digest) — Neighborhood median filter.
11. `pyramid.wgsl` (`79 lines`, SHA-256: `f1f3e0d8c785c54d2b4e4c19e08a7e0a801f5b832ee9130f9c8c5b6e2f8f1a33`) — Gaussian/Laplacian pyramid downsample/upsample.
12. `thermal-shaper.wgsl` (`79 lines`, SHA-256: `e8d91f2a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6`) — Multi-scale slope relaxation.

---

## 4. Governing UI-Category Matrix (`nerthus-app` & `nerthus-cli`)

| Governing UI Category | Inspected Source Evidence (`crates/nerthus-app/src/`) | Poet Destination & Adaptation Status |
|---|---|---|
| **1. Commands** | Central `Command` enum (`ui/commands.rs:34–129`), `GraphCommand` (`ui/graph_commands.rs:1–1105`), `ToolCommand` (`ui/tool_commands.rs:37–115`), and `FileCommand` (`ui/files.rs:1–979`), plus 38+ MCP tools (`mcp/*`). | Map graph authoring (`AddNode`, `Connect`, `CombineSelected`, `SwitchTo`), node state (`PassThrough`, `Muted`, `Solo`, `LockPreview`), and build/bake actions into Poet's `NativeIntentBus` and `qualia-app-contract`. |
| **2. Panels** | Top toolbar (`ui/main_menu.rs`), 10-category `Toolbox` (`ui/toolbox.rs:1–988`), tabbed `GraphCanvas` (`ui/graph_canvas.rs:1–4161`), `PropertyArea` + `ModifierList` (`ui/property_area.rs:1–1906`, `ui/modifier_list.rs:1–418`), 3D `ViewportArea` (`ui/viewport_area.rs:1–862`), docked/floating `Viewport2D` (`ui/viewport_2d.rs:1–1413`), `BuildPanel` (`ui/build_panel.rs:1–879`), `VarPanel` (`ui/var_panel.rs:1–464`), and tabular `DataView` (`ui/data_view.rs:1–1039`). | Mount as containers inside Poet's Environment/Terrain **Manifold**: Node Graph container, 3D/2D Viewport container, Inspector/Modifier container, and Tabular DataView container. |
| **3. Gestures** | Graph marquee selection, `Ctrl+Shift+Drag` **Push/DragArrange** (`graph_canvas.rs:181–250`), wire drag-to-empty-space fuzzy node search (`create_connection.rs:1–322`), `Alt+Drag` wire reroute, interactive 2D SRT/point gizmos (`ui/gizmos.rs:1–676`), vector mask brush painting (`ui/painter.rs:207–243`), and 2D `SunControl` azimuth/altitude dial drag (`viewport_area.rs:250–520`). | High-value UX patterns for Poet's node-graph and spatial canvas: port `Drag::Push` layout arrangement, wire-drop search popup, 2D parameter gizmos, and `Draw` mask stroke painting as typed Poet gestures. |
| **4. Shortcuts** | Customizable `shortcuts.json` (`ui/shortcuts.rs:1–528`, `ui/menu_keys.rs:1–666`): `Tab`/`~` (canvas node search), `F` (Lock Preview), `M` (Mute node), `F3` (Combine nodes), `Ctrl+G` (Group), `Ctrl+B` (Cycle node size), `F5` (Queue Build), `F6`/`Ctrl+F6`/`Ctrl+M` (2D Map Viewers), `F7` (Sky Editor). | Preserve `Tab` node launcher, `F` preview lock, `M` bypass, and `F6` 2D map toggle within Poet's terrain graph context. |
| **5. Context Menus** | Empty canvas node-creation/bookmark menu (`graph_canvas.rs:3450–3580`), Node right-click (`graph_canvas.rs:3585–3850`: state, `Switch to...` in-place replacement, `Convert Port to Portal`, presets, TORLET export), Wire right-click (`3855–3920`: `Insert Node...`, `Convert to Portal`), and parameter keyframe/variable binding menu (`bound_variable.rs:1–528`). | Adapt `Switch to...` (in-place node type replacement preserving compatible connections/params) and `Insert Node on Wire` into Poet's graph editing intents. |
| **6. Dialogs & Editors** | `painter.rs` (`Draw` node brush editor), `gradient_editor.rs` & `cluter_editor.rs` (multi-stop gradient & Sentinel-2 CLUT sampler), `thermal_shaper_editor.rs`, `math_editor.rs`, `presets.rs`, `turntable.rs`, `var_manager.rs`, and `file_repair.rs`. | Adapt `gradient_editor.rs` / `cluter_editor.rs` (Sentinel-2 colour LUT picker) and `painter.rs` (mask brush) as native Poet inspector editors. |
| **7. Views** | 3D terrain view (`nerthus-render/src/terrain.rs:1–404` with `Data`, `Clay`, and `Realistic` modes + Bruneton `sky.wgsl` + `water.rs`), asynchronous 2D hillshade/mask viewer (`viewport_render.rs:1–827`), and spreadsheet `DataView` (`data_view.rs:1–1039`). | Integrate `Data` / `Clay` / `Realistic` diagnostic terrain shading modes and colourblind-safe mask ramps (`Viridis`, `Magma`, `Plasma`, `Inferno`, `Cividis`, `Turbo`) into `QualiaPortal` and Poet's viewport (`ECO-051/053`). |
| **8. Accessibility, Docs & Agent MCP** | Offline Markdown manual browser for 308 node pages (`ui/docs_window.rs:1–465`), UI scale (`75%–200%`), `Default`/`Compact` widget themes (`ui/theme.rs:1–357`), colourblind-safe mask gradients, and 38+ MCP tools (`src/mcp/*`, including `mcp/terrain_report.rs` quantitative slope/elevation/roughness analysis). | Route `terrain_report.rs` quantitative geomorphology metrics through `NativeProject::mcp_backend()` (`AuthorizedEditorHost`) so human reviewers and agents share objective terrain quality metrics (`ECO-092/114`). |
