# Zero-V8 QualiaDB, VibeScript & Custom Container Architecture Report

**Date:** 2026-10-20  
**Repository:** `c:\github\ecopunk`  
**Reference Specifications:**
- [`docs/planning/tech/development-modes.md`](file:///c:/github/ecopunk/docs/planning/tech/development-modes.md)
- [`docs/planning/tech/engine-requirements.md`](file:///c:/github/ecopunk/docs/planning/tech/engine-requirements.md)
- [`docs/planning/tech/game-demo-remediation.md`](file:///c:/github/ecopunk/docs/planning/tech/game-demo-remediation.md)
- [`docs/planning/tech/webizen-execution.md`](file:///c:/github/ecopunk/docs/planning/tech/webizen-execution.md)
- [`docs/planning/tech/pack-delivery.md`](file:///c:/github/ecopunk/docs/planning/tech/pack-delivery.md)

---

## 1. Executive Summary

In accordance with the architectural mandate in [`docs/planning/tech/development-modes.md`](file:///c:/github/ecopunk/docs/planning/tech/development-modes.md) §1 (*"Qualia-native environments run VibeScript, not JavaScript... do not inject a JavaScript substitute"*), this report documents:

1. **What Has Been Implemented 100% Zero-V8 in `c:\github\ecopunk`:**
   - All custom browser JavaScript engines (`viewer/*.js`, `viewer/data/*.js`) and Node.js V8 scripts (`scripts/*.mjs`) have been **deleted**.
   - The entire ecological, botanical, zoological, geospatial, semantic, and GPU rendering pipeline in [`crates/ecopunk-terrain`](file:///c:/github/ecopunk/crates/ecopunk-terrain/src/lib.rs) now executes **100% natively in Rust + QualiaDB + VibeScript (`vibe-0.1`) + `wgpu` (`PortalGpu` / `VolumetricRenderer`) with ZERO V8, ZERO Node.js, and ZERO Python**.
2. **Where the V8 Boundary Exists & Why Standard Commodity Browsers Cannot Yet Run Zero-V8 Without Either the Native Webizen/Chora/Poet Host or Upstream Engine Gaps (`G10`, `G23`, `G33`, `G34`, `G50`, `G52–G65`):**
   - **Mode 1 — Qualia-Native Execution (`PortalGpu`, `VolumetricRenderer`, `qualia-cli`, Chora/Poet Desktop/Mobile Shell):** **0% V8.** Runs native Rust, `vibe::Engine` / `vibe::eval_function` + `PoetSnapshot` (no JIT, no V8), custom binary containers (`.hmc`, `.10d`, `.q42`, `.m64`, `.e64`), and native `wgpu` (`Vulkan` / `DX12` / `Metal`).
   - **Mode 2 — Standard-Web Browser Projection (Unmodified Chrome / Edge / Safari / Firefox on GitHub Pages):** Commodity web browsers hardcode a JavaScript VM (V8 in Chromium, JavaScriptCore in WebKit, SpiderMonkey in Firefox) as the only W3C-standardized gatekeeper to DOM `<canvas>`, `navigator.gpu` (WebGPU), and `WebAssembly.instantiateStreaming`. Even when 99.9% of the runtime is compiled to WebAssembly (`wasm32-unknown-unknown`), an unmodified commodity browser still invokes its JS engine to execute the `wasm-bindgen` glue table that forwards `navigator.gpu` and `<canvas>` handles into WASM. Eliminating V8 *inside the browser* requires running inside the **native Webizen / Chora / Poet browser host** (`PortalGpu::new_surface(hwnd, ...)`).

---

## 2. Implemented 100% Zero-V8 Native Pipeline (`crates/ecopunk-terrain`)

### 2.1 VibeScript (`vibe-0.1`) Execution & Darwin Core Semantic Sealing (`PoetSnapshot`)
- **Modules Certified & Executed Without V8:**
  1. [`vibe/east_coast_biosphere_statements.vibe`](file:///c:/github/ecopunk/vibe/east_coast_biosphere_statements.vibe) — Evaluated directly by [`vibe::load_program`](file:///C:/github/qualiaDB/crates/vibe/src/lib.rs) and [`vibe::eval_function`](file:///C:/github/qualiaDB/crates/vibe/src/lib.rs) against [`qualia_core_db::poet_host::PoetSnapshot`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/mod.rs). Calls `quin.statement(subject, predicate, object, context: 0)` to hash IRIs via `generate_60bit_token` and seal 64-bit parity via `NQuin::calculate_parity`.
  2. [`entities/taxa/corymbia-aparrerinja/corymbia-aparrerinja.vibe`](file:///c:/github/ecopunk/entities/taxa/corymbia-aparrerinja/corymbia-aparrerinja.vibe) — Certified via `vibe::parse` and `vibe::check_module`.
  3. [`vibe/east_coast_flora_305.vibe`](file:///c:/github/ecopunk/vibe/east_coast_flora_305.vibe) — 305-species botanical and leaf biophysics schema module.
  4. [`vibe/east_coast_fauna_164.vibe`](file:///c:/github/ecopunk/vibe/east_coast_fauna_164.vibe) — 164-species zoological and biomechanical gait schema module.
  5. [`vibe/lithology_pedology_hydrology.vibe`](file:///c:/github/ecopunk/vibe/lithology_pedology_hydrology.vibe) — 6-layer SLGA soil horizon, lithology, and radiative hydrology schema module.
- **SHACL Validation:** Every VibeScript-emitted taxon and asset binding is verified in-process via [`qualia_core_db::query::shacl_compiler::validate_shacl_property`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/query/shacl_compiler.rs) (`ShaclConstraint::MinCount(1)`, `ShaclConstraint::Datatype(ShaclDatatype::Integer)`).

### 2.2 Custom QualiaDB Binary Formats Sealed Natively (`.hmc`, `.10d`, `.q42`, `.m64`, `ConstituentLUT`)

| Binary Format | Magic / Version | Engine Producer / Verifier | Output Artifact |
| :--- | :--- | :--- | :--- |
| **`.hmc` Holistic Media Container** | `QBDL` v1 | [`BundleWriter`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/bundle.rs) / [`BundleReader::verify_entry`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/bundle.rs) | `assets/packs/ecopunk-east-coast.hmc` |
| **`.10d` Manifold Geometry & Material Container** | `10D\0` v1 (`MESH`, `MAT4`, `UV01`, `FNRM`, `SRD1`, `PROV`) | [`compile_mesh_to_10d_with_extras_and_frames`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/compile_10d.rs), [`attach_material_section`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/material_section.rs), [`attach_texture_coordinate_section`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/mod.rs) | `assets/terrain/sector_*.10d`, `assets/terrain/water_*.10d` |
| **`.q42` v4 Semantic Volume** | `Q42\0` v4 (`Q42LEX`, `BIDX`, Merkle Root) | [`StreamingQ42VolumeWriter`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/q42/q42_volume.rs) & [`write_sorted_quins_volume`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/q42/q42_volume.rs) | `assets/terrain/east_coast_biosphere_v4.q42`, `assets/terrain/east_coast_strata.q42` |
| **`.m64` Matter-64 Geodetic Strata** | `M64\0` Mode `0x02` (`SRID_GDA2020`, `VERTICAL_DATUM_AHD`) | [`qualia_geospatial::compile_raster_to_m64`](file:///C:/github/qualiaDB/crates/qualia-geospatial/src/lib.rs) | `assets/terrain/talbingo_catchment.m64` |
| **16-KB GPU `ConstituentLUT`** | 256 × 64B (`16,384` B) | [`BankedConstituentLut::new_with_terrestrial_defaults`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/q42/m64_matter/banked_lut.rs) | `assets/terrain/constituent_lut_16kb.bin` |

### 2.3 Native Computational Geometry, Botanical Biophysics & `PortalGpu` (`wgpu`) Rendering
In [`crates/ecopunk-terrain/src/scene_builder_10d.rs`](file:///c:/github/ecopunk/crates/ecopunk-terrain/src/scene_builder_10d.rs):
- **Botanical Crown Envelopes & 3D Falcate Foliage:** Uses [`botanical_hulls::asymmetric_hull_into`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/botanical_hulls.rs) (Cescatti 6-point asymmetric crown hulls) combined with explicit 3D phyllotactic falcate leaf blades (`double_sided: true`, `wind_animated: true` in `MAT4`), *Dicksonia antarctica* bipinnate fern fronds, *Xanthorrhoea* grass tree needle skirts & flowering scapes, Cosserat tussock grasses (*Themeda triandra*, *Poa sieberiana*), and submerged *Vallisneria australis* ribbon weed.
- **Rotation-Minimizing Bishop Frame Extrusion:** Uses [`sweep_extrusion::extrude_profile_along_path`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/sweep_extrusion.rs) (`compute_bishop_frames`, Wang et al. 2008 Double-Reflection Method) for contorted snow-gum/ghost-gum trunks, boughs, and articulated 3D Australian fauna (*Macropus giganteus*, *Vombatus ursinus*, *Ornithorhynchus anatinus*, *Aquila audax*, *Maccullochella peelii*).
- **AG-12 Zero-Heap Vegetation Interaction:** Executes [`VegetationEvaluator::update_instances`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/vegetation.rs) for dynamic collider deflection and wind spring-damper bending.
- **AG-10 Refractive Shoreline Water & Submerged 3D Pebbles/Cobbles:** Executes [`PortalGpu::upload_water_geometry`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/water.rs) (which runs `shore_topology_into` + `shore_distance_accelerated_into` BVH shoreline SDF distance) at `WATER_QUALITY_CINEMATIC` (`2`) over 220+ 3D riverbed cobbles and variegated quartz/jasper/basalt pebbles.
- **AG-22 Environment Probes, Cascaded Shadows & SSAO:** Binds [`EnvironmentProbeSet`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/lighting/environment_lighting.rs), two-cascade directional sun shadows (`set_shadows_enabled(true)`), half-resolution depth/normal SSAO (`set_screen_space_ao_enabled(true)`), and analytic sky (`set_sky_preset`) on [`PortalGpu::new_offscreen`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/portal_gpu/constructors.rs).

---

## 3. Why a 100% Zero-V8 Pipeline Cannot Run Inside an Unmodified Commodity Web Browser (And How QualiaDB Solves It)

Per [`docs/planning/tech/development-modes.md`](file:///c:/github/ecopunk/docs/planning/tech/development-modes.md) §§1–3 and [`docs/planning/tech/engine-requirements.md`](file:///c:/github/ecopunk/docs/planning/tech/engine-requirements.md), it is essential to distinguish between **Qualia-Native Execution (Mode 1)** and **Standard-Web Browser Delivery (Mode 2)**:

### 3.1 The Commodity Browser Sandbox Constraint (W3C / Chromium V8 Boundary)
1. **No Direct OS/GPU Syscalls from HTML:** An unmodified commodity browser (Chrome, Edge, Brave, Firefox, Safari) opening an `index.html` page on GitHub Pages does **not** allow raw native binaries, `.hmc` containers, or `.vibe` scripts to claim a GPU swapchain directly from HTML.
2. **WebAssembly Host Imports Are Routed Through the Browser's JS Engine:** In standard W3C WebAssembly 1.0/2.0 (without the native Webizen host), a `.wasm` module has no direct access to `<canvas>` or `navigator.gpu` (`GPUAdapter`, `GPUDevice`, `GPUQueue`, `GPURenderPassEncoder`). When [`qualia-core-db` (`features = ["portal", "wasm-full"]`)](file:///C:/github/qualiaDB/crates/qualia-core-db/Cargo.toml) or [`rolling-commons-shell`](file:///C:/github/game-demo/crates/rolling-commons-shell/Cargo.toml) compiles to `wasm32-unknown-unknown`, `wgpu`'s `BROWSER_WEBGPU` backend emits `wasm-bindgen` imports (`wbg_*`). Inside Chrome/Edge, **V8** parses and executes that generated `wasm-bindgen` JS glue file to instantiate the `.wasm` module and forward WebGPU/DOM handle tables across the host boundary.
3. **Mode 2 Standard-Web Projection Gap (`G10` in [`engine-requirements.md`](file:///c:/github/ecopunk/docs/planning/tech/engine-requirements.md#L48-L55)):**
   - As documented in [`docs/planning/tech/development-modes.md`](file:///c:/github/ecopunk/docs/planning/tech/development-modes.md#L19-L29):
     > *"Standard-web outputs (HTML, CSS, DOM/WebGPU glue, WASM bootstrap, Service Worker / Cache API plumbing, standard JSON-LD/RDFa heads) are **build artefacts** emitted by projecting `.vibe` + `.q42` + `.10d` + `.hmc` through the Qualia build toolchain (`qapp-packager`, `vibe-wasm`, `webizen-web`). They are not hand-authored source."*
   - And [`docs/planning/tech/engine-requirements.md` §3 `G10`](file:///c:/github/ecopunk/docs/planning/tech/engine-requirements.md#L53) records that the automated **`vibe` → standard-web projection toolchain** (compiling `.vibe` UI/control modules into a zero-hand-written-JS standard-web projection bundle) is an open upstream toolchain requirement (`G10`).

### 3.2 Open Upstream QualiaDB Engine Gaps Identified in [`engine-requirements.md`](file:///c:/github/ecopunk/docs/planning/tech/engine-requirements.md)
Even within the native/WASM QualiaDB engine (`C:\github\qualiaDB`, which is read-only per [`development-modes.md` §5](file:///c:/github/ecopunk/docs/planning/tech/development-modes.md#L46-L56)), the planning docs identify the following upstream engine boundaries that we have worked around cleanly in `ecopunk-terrain`:

| Gap ID | Upstream Engine Location | Technical Limitation & Status | How `ecopunk-terrain` Handles It Today |
| :--- | :--- | :--- | :--- |
| **`G10`** | `crates/vibe`, `crates/vibe-wasm` | No automated `vibe` → standard-web HTML/WASM projection bundler yet; browser WASM still uses `wasm-bindgen` JS glue on V8. | `ecopunk-terrain` runs 100% Zero-V8 natively via `PortalGpu::new_offscreen` / `VolumetricRenderer` (`wgpu` native Vulkan/DX12/Metal). |
| **`G23`** | `crates/qualia-core-db/src/container_10d/section.rs` | `EntityOntology` (`EONT`) section wire size constant (`ENTITY_ONTOLOGY_WIRE_SIZE = 104`) disagrees with `EntityOntologyHeader` (`92` B), causing `encode_entity_ontology_section` to panic if an `EONT` section is attached directly. | Follows [`game-demo/crates/rolling-commons-shell/src/asset_compile.rs`](file:///C:/github/game-demo/crates/rolling-commons-shell/src/asset_compile.rs#L96-L100): seals entity ontology & provenance inside `ProvenanceSidecar::semantic_metadata` (`PROV`) + `.hmc` `sem/east_coast_biosphere.q42`. |
| **`G33` / `G34`** | `crates/qualia-core-db/src/container_10d/impostor_section.rs` | `IMPO` section header size check (`36` vs `44` bytes) and `encode_container` ascending `SectionType` sort (`QuantizedMesh = 1` before `Impostor8D = 12`) prevent embedding `IMPO` inside the same `.10d` file as `QuantizedMesh`. | Emits `.10d` hero/sector meshes (`QuantizedMesh` + `MAT4` + `UV01` + `FNRM` + `SRD1` + `PROV`) and keeps LOD/water entries as separate verified `.hmc` entries (`10d/sector-*.10d`, `10d/water-*.10d`). |
| **`G50` / `AG-14`** | `crates/qualia-core-db/src/render/gpu/mod.rs` | `PortalGpu` holds one resident multi-submesh `MeshGpu` (`MAX_GPU_MATERIAL_DRAWS = 65,536`) + one instanced stream (`mesh_instances`) + one `WaterGpu` surface (`water`) rather than an arbitrary multi-mesh scene graph table. | `scene_builder_10d.rs` compiles each sector into a unified multi-submesh `MAT4` `.10d` container (with separate `SubmeshRange`s and `MaterialRecord`s for Terrain/Strata, 3D Lithology/Pebbles, Tree Trunks, 3D Leaf Blades/Grasses, and 3D Fauna) plus a separate `.10d` water surface loaded via `upload_water_geometry` / `load_hmc_water_asset`. |
| **`G52–G55`** | `crates/qualia-core-db/src/render/gpu/portal_gpu/init.rs` | `PortalGpu::upload_mesh_colored_with_frames_and_materials_and_uv0` accepts full `SRD1` vertex colours + `FNRM` normals + `UV01` UVs + `MAT4` materials, whereas `VolumetricRenderer::load_10d_asset` currently passes `None` for the `FNRM` normal sidecar and only populates vertex `colors` when `Tensor10DNodes` are present. | `scene_builder_10d.rs` seals `SRD1`, `FNRM`, `UV01`, and `MAT4` into the `.10d` files AND passes the decoded `&sector.colors`, `Some(&sector.normals)`, `Some(&sector.uvs)`, `&materials`, and `&sector.ranges` directly to `PortalGpu::upload_mesh_colored_with_frames_and_materials_and_uv0`. |

---

## 4. Upstream Recommendations for `C:\github\qualiaDB`

To make `VolumetricRenderer::load_hmc_asset` and browser/native hosts preserve 100% of `.10d` surface readings and normals without caller intervention:
1. **In `crates/webizen-render/src/volumetric.rs` (`VolumetricRenderer::load_10d_asset`):**
   - Decode `SectionType::SurfaceReading` (`SRD1`) via `container_10d::surface_reading::decode_surface_reading` when `nodes.is_empty()` so per-vertex botanical/pedological colours are preserved automatically by `load_hmc_asset`.
   - Decode `SectionType::FieldSidecar` (`FNRM`) via `qualia_core_db::render::compile_10d::decode_10d_mesh_frames` and forward `frames.normals` and `frames.tangents` into `self.inner.upload_mesh_colored_with_frames_and_materials_and_uv0(...)`.
2. **In `crates/qualia-core-db/src/container_10d/entity_ontology_section.rs` (`G23`) & `impostor_section.rs` (`G33`):**
   - Align `ENTITY_ONTOLOGY_WIRE_SIZE` (`92` B) and `IMPOSTOR_HEADER_SIZE` (`44` B) with their struct encoders so `EONT` and `IMPO` sections can be sealed inside `.10d` containers without sidecar workarounds.
3. **In `crates/vibe` / `crates/webizen-web` (`G10`):**
   - Provide the native Webizen/Chora desktop window runner (`VolumetricRenderer::new_surface(hwnd, width, height, particle_cap)`) as the primary Zero-V8 interactive viewer, and add a `vibe` → WASM/WebGPU projection target for Mode 2 standard-web deployment.
