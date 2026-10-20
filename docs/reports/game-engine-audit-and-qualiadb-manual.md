# Forensic Audit: `Fyrox-master` & `bevy-main` vs. `qualiaDB` — Capabilities, Fraud/Overclaim Analysis, and Developer Remediation Manual

**Audit Date:** 2026-10-20  
**Audited Repositories:**
- `C:\github\qualiaDB` (`qualia-core-db`, `webizen-render`, `qualia-sim`, `qualia-audio`, `vibe`, `poet`, `webizen-studio`)
- `C:\github\bevy-main` (60+ crates including `bevy_ecs`, `bevy_render`, `bevy_core_pipeline`, `bevy_pbr`, `bevy_solari`, `bevy_animation`, `bevy_gltf`, `bevy_ui`, `bevy_picking`, and 170+ examples)
- `C:\github\Fyrox-master` (24 crates including `fyrox-core`, `fyrox-graph`, `fyrox-graphics-wgpu`, `fyrox-graphics-gl`, `fyrox-impl`, `fyrox-animation`, `fyrox-sound`, `fyrox-ui`, `editor` [FyroxEd], and examples)

> **Audit Methodology:** All findings below were verified directly against `.rs`, `.wgsl`, and `.wesl` source files. Per user instruction, no `DIRECTORY_INDEX.md` files were trusted or relied upon.

---

## 1. Direct Verdict: Is the Claim That "QualiaDB Has Superior Game Engine Capabilities" Fraud?

**Yes — if stated as a claim about 3D game engine functionality, rendering architecture, scene management, skeletal animation, physics, asset loading, or authoring tools, claiming that `qualiaDB` is superior to `Bevy` or `Fyrox` is false and constitutes architectural misrepresentation.**

Specifically, two distinct things have been conflated in previous evaluations and policy documents (such as [13-qualiadb-only-development-contract.md](file:///C:/github/game-demo/docs/planning/13-qualiadb-only-development-contract.md) and [holographic-simulation-aaa-master-plan.md](file:///C:/github/ecopunk/docs/plans/holographic-simulation-aaa-master-plan.md)):

1. **What `qualiaDB` actually is (and where it genuinely excels):**
   - A **neuro-symbolic semantic database (`NQuin` 48-byte records), deontic/jural governance engine, scientific/analytical math library, and bounded zero-heap runtime (`vibe-0.1`, `.10d`, `.hmc`)** with specialized analytical continuum equations in `qualia-sim` (Farquhar leaf photosynthesis, Shinozaki pipe-model plant allometry, Byram fireline intensity, Mohr-Coulomb soil shear).
2. **What `qualiaDB` is NOT (and where previous claims/policies misrepresented it):**
   - **It is NOT a full-featured 3D game engine.** Its primary GPU renderer ([`PortalGpu`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/mod.rs#L403-L543)) holds **one single merged mesh slot** (`mesh: Option<MeshGpu>`), performs **100% of skeletal skinning on the CPU in `f64`** followed by re-uploading the entire vertex/normal/tangent buffers via `queue.write_buffer` every frame, has **zero 3D rigid-body physics or collision detection** (only 2D `(x, z)` heightfield closures), has **a disconnected `BTreeMap<String, SceneNode>` scene graph**, **strips all hierarchy, skins, animations, and morphs from GLB files**, and **has several advanced WGSL shaders sitting in source files that are only parsed in unit tests and never executed in the render loop**.
   - Furthermore, the "QualiaDB-only development contract" ([`13-qualiadb-only-development-contract.md`](file:///C:/github/game-demo/docs/planning/13-qualiadb-only-development-contract.md#L5-L32)) enforced a policy prohibiting external game engines or libraries while forcing game repositories (`game-demo`, `ecopunk`) to build on a single-mesh-slot forward viewport and CPU-bound pipelines—masking engine architectural deficits as "unverified integration gates."

---

## 2. Forensic Evidence: Dead/Unwired Code, Stubs, and Architectural Bottlenecks in `qualiaDB`

Direct inspection of `C:\github\qualiaDB` revealed the following critical discrepancies between what module/file names suggest and what actually executes at runtime:

### 2.1 Unwired Shaders ("Paper Capabilities")
In [`crates/qualia-core-db/src/shaders/viewport/mod.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/shaders/viewport/mod.rs#L35-L49) and [`crates/qualia-core-db/src/render/deferred_resolve.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/deferred_resolve.rs#L110-L334):
- `DEFERRED_RESOLVE_WGSL`, `CLUSTER_LIGHT_WGSL`, `SGGX_MICROFLAKE_WGSL`, `FOLIAGE_LOD_WGSL`, `SWE_SOLVER_WGSL`, `PRECIPITATION_WGSL`, `ACOUSTIC_VOXEL_CONE_WGSL`, `SURFEL_INSTANCED_WGSL`, and `EMF_VOLUMETRIC_WGSL` exist as WGSL strings.
- **Actual Reality:** None of these shaders are wired into [`PortalGpu::render_internal`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/portal_gpu/render_frame.rs#L66-L523). For example, `DEFERRED_RESOLVE_WGSL` is only passed to `naga::front::wgsl::parse_str` inside a unit test ([`deferred_resolve.rs:326-334`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/deferred_resolve.rs#L326-L334)), while `evaluate_deferred_resolve_pixel` is a CPU reference function.
- Similarly, [`PGA_SCLERP_EVAL_WGSL`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/shaders/mod.rs#L9) exists as a compute shader string, but `PortalGpu` does not use GPU skinning at all.

### 2.2 Single-Mesh-Slot Renderer (`PortalGpu`)
In [`crates/qualia-core-db/src/render/gpu/mod.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/mod.rs#L484):
- `PortalGpu` stores `pub(super) mesh: Option<MeshGpu>`.
- Instead of managing multiple independent meshes, vertex slabs, or scene entities on the GPU, the entire 3D scene must be merged on the CPU into a single `MeshGpu` vertex/index buffer with submesh `draws: Vec<MeshDrawRange>` (plus up to `MAX_GPU_MESH_INSTANCES = 10_000` instances of that mesh, 1 water surface, 1 `Tensor10D` billboard cloud, and 1 ambient particle cloud).
- **WASM HDR & Bloom Disabled by Hardcoded Check:** In [`crates/qualia-core-db/src/render/gpu/bloom.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/bloom.rs#L21-L31), `probe_hdr_format` unconditionally returns `false` on `cfg(target_arch = "wasm32")`, disabling HDR and Kawase bloom in browser builds.

### 2.3 100% CPU-Bound `f64` Skeletal Skinning & Full-Buffer Re-Upload
In [`crates/qualia-core-db/src/render/skeletal/player.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/skeletal/player.rs#L347-L430), [`skin.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/skeletal/skin.rs#L14-L90), and [`animated_upload.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/animated_upload.rs#L7-L73):
- Every frame, `SkeletalPlayer::advance` transforms every vertex and normal on the **CPU in 64-bit floating point (`f64`)** (`skin_vertices_into`), recomputes tangents from UVs on the **CPU** (`tangents_from_uvs_into`), and calls `PortalGpu::write_mesh_animated_frames`, which issues **three full-buffer `queue.write_buffer` uploads** (positions, normals, tangents) to the GPU and invalidates the shadow map (`shadow_map_dirty = true`).
- Both `Bevy` ([`skinning.wesl`](file:///C:/github/bevy-main/crates/bevy_pbr/src/render/skinning.wesl)) and `Fyrox` ([`surface.rs`](file:///C:/github/Fyrox-master/fyrox-impl/src/scene/mesh/surface.rs#L1249-L1271)) perform hardware vertex-shader skinning by uploading only compact joint transform matrices.

### 2.4 Disconnected Toy `SceneGraph`
In [`crates/qualia-core-db/src/render/scene_graph/node.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/scene_graph/node.rs#L10-L341) and [`light.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/scene_graph/light.rs#L119-L131):
- `SceneGraph` is a `BTreeMap<String, SceneNode>` using heap-allocated `String` IDs and `BTreeMap<String, String>` metadata.
- Attaching a light to a scene node literally serializes `format!("{:?}", light.light_type)` into `node.metadata` strings (`light.rs:119-131`).
- It does not compute hierarchical world transform matrices and is not connected to `PortalGpu`.

### 2.5 Absence of 3D Rigid-Body Physics & Collision Detection
In [`crates/qualia-core-db/src/render/physics/`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/physics/mod.rs) and [`crates/qualia-sim/src/`](file:///C:/github/qualiaDB/crates/qualia-sim/src/lib.rs):
- There is **no broadphase, no narrowphase (no GJK/EPA/SAT), no trimesh/convex-hull/capsule collision, no impulse constraint solver, and no 3D raycasting against scene geometry**.
- `CharacterController` ([`qualia-sim/src/character.rs:154-332`](file:///C:/github/qualiaDB/crates/qualia-sim/src/character.rs#L154-L332)) and `RoverKinematics` ([`qualia-sim/src/kinematics.rs:127-275`](file:///C:/github/qualiaDB/crates/qualia-sim/src/kinematics.rs#L127-L275)) only evaluate a 2D heightfield closure `Fn(f32, f32) -> (f32, [f32; 3])`. Characters and vehicles pass straight through walls, trees, buildings, and bridges.

### 2.6 Incomplete GLB Importer & Approximated Texture Transcoding
- **GLB Importer ([`crates/qualia-core-db/src/render/assets/glb.rs:29-259`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/assets/glb.rs#L29-L259)):** Flattens all `meshes[].primitives[]` into one static `Mesh`, ignoring `nodes` transform hierarchies, `skins` (`JOINTS_0`/`WEIGHTS_0`), `animations`, and morph `targets`.
- **KTX2 / BC7 / ASTC / ETC2 Transcoder ([`crates/qualia-core-db/src/render/texture_ktx2.rs:39`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/texture_ktx2.rs#L39), [`texture_transcode.rs:163-284`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/texture_transcode.rs#L163-L284)):** Has no BasisLZ/UASTC decoder linked; `decode_bc7_block` only decodes Mode 6 and uses a crude byte-interpolation approximation for Modes 0–5 and 7 (`lines 216-230`); `Etc2Rgba` and `Astc4x4` (`lines 278-283`) literally call `decode_bc7_block` as a fake fallback!

---

## 3. Side-by-Side Architectural Comparison Matrix (`Bevy` vs. `Fyrox` vs. `QualiaDB`)

| Capability Domain | `Bevy` (`C:\github\bevy-main`) | `Fyrox` (`C:\github\Fyrox-master`) | `QualiaDB` (`C:\github\qualiaDB`) |
| :--- | :--- | :--- | :--- |
| **Core Data / Entity Architecture** | Hybrid **Archetype SOA `Table` + `ComponentSparseSet` ECS** (`bevy_ecs`), `MultiThreadedExecutor`, per-component `Added<T>`/`Changed<T>` ticks, lifecycle hooks (`on_add`/`on_remove`), push `Observer`s, `Relationship`s (`ChildOf`), `bsn!` scenes. | Generational **`Pool<T>` + `Handle<T>` Scene Graph** (`fyrox-core`, `fyrox-graph`), 26 node types, `InheritableVariable<T>` prefab delta inheritance (~90% size reduction), `MultiBorrowContext`, binary/ASCII `Visitor` serialization. | **48-byte `NQuin` graph store + `Tensor10D` arrays** (`qualia-core-db`). **No ECS.** `render::scene_graph::SceneGraph` is a disconnected `BTreeMap<String, SceneNode>`. |
| **Rendering Pipeline** | Pipelined `RenderApp` (`ExtractSchedule` + render thread), camera-driven `Core3d`/`Core2d` schedules, **Clustered Forward + Packed `Rgba32Uint` Deferred PBR**, bindless slabs, OIT. | Dual-backend `GraphicsServer` (**OpenGL/WebGL + wgpu**), **Full 5-RT G-Buffer Deferred + Forward renderer**, decal layer masks, hardware occlusion queries (`OcclusionTester`). | Forward-only [`PortalGpu`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/mod.rs#L403) with **1 `Option<MeshGpu>` slot**, 1 water pass, 1 `Tensor10D` sprite pass, 1 particle pass. `DEFERRED_RESOLVE_WGSL` is **unwired**. |
| **GPU-Driven Rendering & Culling** | Shared `MeshSlabs` (`MeshAllocator`), GPU compute frustum culling, **2-phase HZB GPU occlusion culling**, `multi_draw_indirect_count`, and **`MeshletPlugin` virtual geometry** (BVH DAG + software/hardware visibility buffer). | Per-surface frustum culling, static/dynamic draw batching (`RenderDataBundleStorage`), hardware `SamplesPassed` grid occlusion cache, QuadTree terrain chunk LODs. | CPU frustum & projected-radius culling (`instance_culling.rs`, max 10,000 instances of the single loaded mesh) uploaded via `queue.write_buffer`. CPU QEM LOD builder (`lod_chain.rs`) not auto-switched per instance on GPU. |
| **Global Illumination & Reflections** | **Real-time Raytraced GI (`bevy_solari`)** with BLAS/TLAS, **ReSTIR DI/GI**, World Irradiance Cache, DLSS-RR, progressive Pathtracer; plus **SSR**, **GTAO**, `IrradianceVolume`, `EnvironmentMapLight`, baked `Lightmap`. | Precomputed split-sum BRDF LUT, `ReflectionProbe` cubemaps with GPU `EnvironmentMapSpecularConvolution` & `IrradianceConvolution`, CPU/GPU baked `Lightmap`, half-res 32-sample **SSAO**, volumetric light shafts. | **No GI, no SSR, no cubemap IBL.** `EnvironmentLightingUniform` holds 4 sphere probes as 5-level scalar RGB uniforms (544 bytes). Half-res single-ring SSAO (`ao.rs`). |
| **Lighting & Shadows** | Clustered `PointLight`, `SpotLight`, LTC `RectLight`, `DirectionalLight` CSM + point cubemap + spot shadows, PCSS soft shadows, temporal blue-noise PCF, screen-space `ContactShadows`. | Deferred `PointLight` (6-face cubemap shadows), `SpotLight` (2D perspective shadows), `DirectionalLight` (3-cascade CSM), soft PCF, stencil volumetric cone/sphere scattering. | 1 directional sun light with 2-cascade shadow map (`SHADOW_CASCADE_COUNT = 2` in `shadows.rs`). `CLUSTER_LIGHT_WGSL` is unwired. |
| **Atmosphere & Post-Processing** | Hillaire 2020 4-LUT `Atmosphere`, 3D `VolumetricFog`, `AutoExposure` histogram, mip-chain `Bloom`, Bokeh/Gaussian `DepthOfField`, `MotionBlur`, `ChromaticAberration`, **TAA, DLSS-SR/RR, SMAA, FXAA, CAS**, 7 tonemappers + 3D LUTs. | `SkyBox`, `HighDynamicRangeRenderer` (64×64 luminance downsample/histogram + eye adaptation), Gaussian `Bloom`, `ColorGradingLut`, `FxaaRenderer`. | Procedural sky triangle, 4-pass Kawase bloom + ACES/AgX/Khronos Neutral tonemapping (**disabled on WASM** in `bloom.rs:21`), basic TAA resolve (`temporal_resolve_gpu.rs`). |
| **Skeletal Animation, Blending & IK** | **GPU vertex skinning** (1B joint allocator + motion vectors), **GPU morph targets** (`MAX_MORPH_WEIGHTS = 256`), `AnimationGraph` blend/additive DAG, 64-bit `AnimationMask`s, retargeting UUIDs, observer `AnimationEvent`s. | **GPU vertex skinning** + **3D texture `f16` Blend Shapes**, entity-agnostic `fyrox-animation`, **Root Motion**, **ABSM** (multi-layer bone masks, **2D Delaunay `BlendSpace`**, boolean logic transitions), `AnimationSignal`s. | **100% CPU `f64` skinning** (`skin_vertices_into`) + CPU tangent rebuild + **3 full `queue.write_buffer` uploads per frame** (`animated_upload.rs`). Max 128 joints, 1 cross-fade, no morph targets, unconstrained array-only `ccd_ik`. |
| **Physics & Collision** | External ecosystem plugins (`avian3d` / `bevy_rapier3d`) + built-in `bevy_picking` (`MeshRayCast` Möller–Trumbore ray-triangle + AABB). | **Built-in 3D & 2D Rapier (`PhysicsWorld`)** in scene graph: `RigidBody`, `Collider` (Trimesh, Heightfield, Convex, Capsule, etc.), `Joint`, `Ragdoll` (anim-to-dynamic), `KinematicCharacterController` (stairs/slopes/shapecast), raycasting. | **No 3D physics or collision engine.** Only 2D `(x, z)` heightfield closures in `qualia-sim` (`CharacterController`, `RoverKinematics`) and AABB bounds check (`render/physics/admission.rs`). |
| **Asset Pipeline & Formats** | Multithreaded `AssetServer`, `FileWatcher` + `EmbeddedWatcher` shader hot-reload, `AssetProcessor`, **full glTF 2.0 loader** (nodes, skins, morphs, anims, lights, 12 KHR extensions), **KTX2 + Basis Universal UASTC/ETC1S**, DDS, HDR, EXR. | Async `ResourceManager`, FBX + glTF importers with full node/skin/animation hierarchies, `.rgs`/`.rgsca` scenes with prefab delta inheritance, compressed textures, streaming audio. | Custom sealed **`.10d` v2** & **`.hmc`** page-aligned bundles with CRC32C/SHA256 + SHACL/VC gates. **GLB importer (`glb.rs`) drops nodes, skins, animations, and morphs.** KTX2 lacks BasisLZ; BC7/ETC2/ASTC transcoding is incomplete/faked. |
| **Audio Engine** | `bevy_audio` (`rodio`/`cpal`), `AudioPlayer`, `SpatialAudioSink`, `SpatialListener`. | **`fyrox-sound`**: Native backends (DirectSound/ALSA/CoreAudio/WebAudio/AAudio), **IRCAM `.hrir` FFT HRTF binaural 3D audio**, `AudioBusGraph`, 64-bit **Freeverb**, streaming OGG/WAV (`symphonia`). | **`qualia-audio`**: Rich zero-heap DSP library (HRTF FIR, Holosonic synthesis, STFT/CQT, biquad/compressor, PESQ/DNSMOS AQA, bioacoustics), but not auto-wired to a 3D scene graph with occlusion/reverb zones. |
| **UI & WYSIWYG Editor** | `bevy_ui` (`taffy` **Flexbox + CSS Grid + Block**), `bevy_ui_widgets`, `bevy_feathers` dark theme, `bevy_inspector`, JSON-RPC `bevy_remote` (BRP). | **`fyrox-ui`** (`#![forbid(unsafe_code)]`, **50+ widgets**, docking, curves, inspector) + **Full WYSIWYG Editor (`FyroxEd`)** with Undo/Redo, ABSM editor, terrain brush, navmesh editor, lightmapper, play-in-editor, **native `.dll` hot-reload**. | `ZeroHeapUiCanvas` (`webizen-render/src/ui.rs`: max 128 rects/buttons, **no text layout or font rasterizer**); relies on external Dioxus HTML/CSS DOM in `webizen-studio`. No 3D gizmo level editor. |
| **Semantic Governance & Domain Science** | None built-in (application layer). | None built-in (application layer). | **Unmatched:** 48-byte `NQuin` with ECC parity, Hohfeld jural & STIT deontic logic, SHACL opcode compiler, `validate_before_render` VC barrier, `vibe-0.1` budgeted scripting, `qualia-sim` ecological/biophysical models, Jakob-Hanika spectral math. |

---

## 4. Developer Instructions Manual: Working With `Bevy`, `Fyrox`, and `QualiaDB`

### 4.1 Evaluating & Running `C:\github\bevy-main` Examples
`bevy-main` provides a modern data-oriented ECS and GPU-driven rendering stack.

```powershell
# 1. Run standard 3D PBR & glTF skeletal animation examples
cd C:\github\bevy-main
cargo run --example animated_mesh
cargo run --example animation_graph
cargo run --example pbr

# 2. Run GPU-driven rendering (HZB occlusion culling & virtual geometry meshlets)
cargo run --example occlusion_culling
cargo run --example meshlet --features meshlet

# 3. Run real-time raytraced GI (Solari ReSTIR DI/GI)
cargo run --example solari --features bevy_solari

# 4. Run next-gen BSN scene inheritance & live ECS inspector
cargo run --example bsn
cargo run --example feathers
```

**When to use Bevy Architecture Patterns:**
- High-entity-count simulations (10,000–1,000,000+ independent entities) requiring multithreaded SOA archetype iteration (`Query<(&mut Transform, &Velocity), Changed<Velocity>>`).
- GPU-driven multi-draw indirect rendering with two-phase Hierarchical Z-Buffer (HZB) occlusion culling (`crates/bevy_pbr/src/render/mesh_preprocess.wesl`) and `MeshletMesh3d` cluster DAG streaming.
- Full glTF 2.0 ingestion (`crates/bevy_gltf/src/loader/mod.rs`) with GPU vertex skinning (`crates/bevy_pbr/src/render/skin.rs`) and GPU morph targets (`crates/bevy_render/src/mesh/morph.rs`).

### 4.2 Evaluating & Running `C:\github\Fyrox-master` & `FyroxEd`
`Fyrox-master` provides a complete, batteries-included 3D/2D game engine with integrated Rapier physics, HRTF binaural audio, ABSM animation state machines, and a full WYSIWYG editor (`FyroxEd`).

```powershell
# 1. Launch the standalone WYSIWYG Editor (FyroxEd)
cd C:\github\Fyrox-master
cargo run --package editor-standalone --release

# 2. Run 3D Navmesh + 3D Physics Raycasting example
cargo run --example navmesh

# 3. Run 2D Sprite + Dynamic 2D Lighting example
cargo run --example 2d

# 4. Run HRTF Binaural 3D Spatial Audio & Freeverb examples
cargo run --package fyrox-sound --example hrtf
cargo run --package fyrox-sound --example reverb
```

**When to use Fyrox Architecture Patterns:**
- Generational arena scene graphs (`Pool<Node>` + `Handle<Node>`) with `InheritableVariable<T>` prefab delta serialization (`fyrox-core/src/variable.rs`).
- Out-of-the-box 3D/2D physics (`PhysicsWorld` in `fyrox-impl/src/scene/graph/physics/mod.rs`) where `RigidBody`, `Collider` (trimesh, heightfield, convex hull), `Joint`, `Ragdoll`, and `KinematicCharacterController` are first-class scene nodes.
- Complex character animation blending using `AnimationBlendingStateMachine` (2D Delaunay `BlendSpace`, bone hierarchy `LayerMask`, and `RootMotion` extraction in `fyrox-animation/src/machine/`).

### 4.3 Developing & Auditing Against `C:\github\qualiaDB`
When using `qualiaDB` (`qualia-core-db`, `qualia-sim`, `webizen-render`, `vibe`), developers must understand its **actual** compiled boundaries rather than relying on aspirational documentation:

1. **Semantic, Governance, and Biophysical Simulation Layer (Use Directly):**
   - Use `NQuin` (`crates/qualia-core-db/src/lib.rs:1112`), `q42` volumes/journals (`q42/journal.rs`), deontic/jural modalities (`modalities/jural.rs`, `modalities/stit.rs`), `vibe-0.1` (`crates/vibe/`), and `qualia-sim` biophysical solvers (`metamer_fspm.rs`, `leaf_biophysics.rs`, `fire_ecology.rs`, `geomechanics.rs`).
2. **Asset Packaging (`.10d` & `.hmc`):**
   - Author or compile geometry into `.10d` v2 containers (`crates/qualia-core-db/src/container_10d/mod.rs`) and page-aligned `.hmc` bundles (`crates/qualia-core-db/src/bundle/mod.rs`).
   - Always pass assets through `validate_before_render` (`crates/qualia-core-db/src/render/barrier.rs:63-136`) to verify CRC-32C, provenance digests, SHACL shapes, and Ed25519 Verifiable Credentials.
3. **Current Renderer Constraints to Respect Until Remediated (See §5):**
   - Do **not** call `PortalGpu::write_mesh_animated_frames` per actor per tick on high-vertex merged scenes without first moving skinning to the GPU vertex shader (see §5.2).
   - Do **not** expect `import_glb_with_normals` (`render/assets/glb.rs`) to import skeletons, animations, or node trees—it currently flattens static geometry only (see §5.5).
   - Do **not** feed BC7 Modes 0–5/7, ASTC, or ETC2 KTX2 textures through `texture_transcode.rs` expecting accurate decompression—only BC1, BC3, and BC7 Mode 6 are properly implemented on CPU (see §5.6).

---

## 5. Mandatory Remediation Roadmap: Limitations That Must Be Updated in `C:\github\qualiaDB`

To bring `C:\github\qualiaDB` up to genuine parity with `Bevy` and `Fyrox` for interactive 3D simulation and game execution, the following **10 concrete engineering remediations** must be implemented in `C:\github\qualiaDB`:

### Remediation 1: Replace the Single-Mesh-Slot in `PortalGpu` with a Multi-Mesh Slab / Draw-Table Architecture
- **Current Limitation:** [`PortalGpu`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/mod.rs#L484) has `pub(super) mesh: Option<MeshGpu>`, forcing callers (`QualiaPortal` / `SceneMeshAccum`) to CPU-concatenate all scene geometry into one giant vertex/index buffer and recompile the whole scene when an asset changes.
- **Required Update:**
  1. Replace `mesh: Option<MeshGpu>` with a bounded `MeshArenaGpu` / slab allocator (modeled on Bevy's [`MeshAllocator`](file:///C:/github/bevy-main/crates/bevy_render/src/mesh/allocator.rs)) supporting multiple resident `MeshHandle`s (`MAX_RESIDENT_MESHES`, e.g., 1,024 meshes × 3 LODs).
  2. Allow each `GpuInstanceRecord` (`render/instance_culling.rs`) to reference `(mesh_id, lod_level, material_id, skin_offset)` and dispatch per-mesh/LOD instanced or `multi_draw_indirect` draws without CPU mesh concatenation.
  3. Wire the existing CPU QEM LOD chain (`crates/qualia-core-db/src/render/lod_chain.rs`) so `select_visible_mesh_instances` automatically selects `LOD0`/`LOD1`/`LOD2`/impostor buffers per instance based on projected screen radius.

### Remediation 2: Move Skeletal Skinning from CPU (`f64` + `write_buffer`) to GPU Vertex Shaders
- **Current Limitation:** [`SkeletalPlayer::advance`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/skeletal/player.rs#L347-L430) + [`skin_vertices_into`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/skeletal/skin.rs#L14-L90) deforms every vertex in `f64` on the CPU, recomputes tangents on the CPU, and calls [`PortalGpu::write_mesh_animated_frames`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/animated_upload.rs#L7-L73) to re-upload the entire mesh's vertex, normal, and tangent buffers every frame.
- **Required Update:**
  1. Add static `JOINTS_0: vec4<u16>` and `WEIGHTS_0: vec4<f32>` vertex buffer attributes to `MeshGpu` (`render/gpu/mesh_upload.rs`).
  2. Upload only the compact per-rig joint PGA `Motor` palette (`[f32; 8]` × `joint_count`, max 128 joints = 4 KB per rig) to a GPU storage/uniform buffer each frame (modeled on Bevy's [`SkinUniforms`](file:///C:/github/bevy-main/crates/bevy_pbr/src/render/skin.rs#L75) and Fyrox's [`MatrixStorageCache`](file:///C:/github/Fyrox-master/fyrox-impl/src/scene/mesh/surface.rs#L1249)).
  3. Perform dual-quaternion / PGA motor sandwich skinning (`sandwich_point`, `sandwich_unit`) directly in the mesh vertex shader (`shaders/viewport/mesh.wgsl`) and shadow vertex shader (`sun_shadow.wgsl`).

### Remediation 3: Implement a Generational Scene Graph / Bounded ECS with Transform Hierarchy Sync
- **Current Limitation:** [`render::scene_graph::SceneGraph`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/scene_graph/node.rs#L29-L33) uses heap-allocated `BTreeMap<String, SceneNode>` and stringified `BTreeMap<String, String>` metadata, does not propagate transforms, and is completely disconnected from `PortalGpu` and `qualia-sim`.
- **Required Update:**
  1. Replace `BTreeMap<String, SceneNode>` with a zero-heap generational arena `SceneNodePool` using 64-bit `NodeHandle { index: u32, generation: u32 }` and 60-bit `NQuin` entity tokens (modeled on Fyrox's [`Pool<T>`](file:///C:/github/Fyrox-master/fyrox-core/src/pool/mod.rs#L69-L76)).
  2. Store hierarchical parent/first-child/next-sibling indices, local PGA `Motor` + scale, dirty-bit change tracking (modeled on Bevy's [`Tick`](file:///C:/github/bevy-main/crates/bevy_ecs/src/change_detection/tick.rs) / Fyrox's `TrackedProperty`), and world `Motor` transforms updated in topological parent-to-child order.
  3. Synchronize dirty world transforms directly into `PortalGpu` instance buffers.

### Remediation 4: Build a Real 3D Collision & Rigid-Body Physics Subsystem
- **Current Limitation:** `qualiaDB` has **no 3D collision or rigid-body solver**—only AABB admission checks ([`render/physics/admission.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/physics/admission.rs)) and 2D `(x, z)` heightfield closures in [`qualia-sim/src/character.rs`](file:///C:/github/qualiaDB/crates/qualia-sim/src/character.rs#L154-L332) and [`kinematics.rs`](file:///C:/github/qualiaDB/crates/qualia-sim/src/kinematics.rs#L127-L275).
- **Required Update:**
  1. Wire the existing BVH in `specialized_libs/computational_geometry/bvh` into `qualia-sim` and `render/physics` as a dynamic broadphase + static scene query acceleration structure.
  2. Implement 3D raycasting (`cast_ray`) and shapecasting (`cast_capsule`, `cast_sphere`) against scene BVH + triangle meshes and analytical SDFs (replacing the 2D heightfield closure in `CharacterController` and `RoverKinematics` so characters and vehicles collide with 3D bridges, rocks, trees, buildings, and overhangs).
  3. Implement GJK/EPA narrowphase for convex polyhedra/capsules/spheres/boxes and a bounded zero-heap XPBD or Sequential Impulse constraint solver supporting `RigidBody` (`Dynamic`, `Static`, `Kinematic`) and physical joints with forces/torques (matching Fyrox's [`PhysicsWorld`](file:///C:/github/Fyrox-master/fyrox-impl/src/scene/graph/physics/mod.rs#L145)).

### Remediation 5: Upgrade the GLB Importer to Preserve Hierarchies, Skins, Animations, and Morph Targets
- **Current Limitation:** [`import_glb_with_normals`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/assets/glb.rs#L29-L259) flattens all primitives into a single static `Mesh` and discards `nodes`, `skins`, `animations`, and `targets`.
- **Required Update:**
  1. Parse glTF `nodes` (TRS / matrix hierarchy), `skins` (`inverseBindMatrices`, ` joints`, `JOINTS_0`, `WEIGHTS_0`), `animations` (samplers & channels: `translation`, `rotation`, `scale`, `weights` with `STEP`, `LINEAR`, and `CUBICSPLINE` interpolation), and morph `targets` (modeled on Bevy's [`GltfLoader`](file:///C:/github/bevy-main/crates/bevy_gltf/src/loader/mod.rs#L189)).
  2. Compile imported glTF skeletons and clips directly into `.10d` `SKL1` (`container_10d/skeletal_section.rs`) and multi-part `.hmc` scene hierarchies (`bundle/variant_matrix.rs`).

### Remediation 6: Fix Fake/Incomplete Texture Transcoding in `texture_transcode.rs` and `texture_ktx2.rs`
- **Current Limitation:** [`texture_transcode.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/texture_transcode.rs#L163-L284) only implements Mode 6 for `decode_bc7_block` (faking Modes 0–5 and 7 with byte interpolation) and routes `Etc2Rgba` and `Astc4x4` into `decode_bc7_block`. `texture_ktx2.rs:39` has no BasisLZ/UASTC decoder.
- **Required Update:**
  1. Either pass compressed `BC1`/`BC3`/`BC7`/`ASTC`/`ETC2` blocks directly to `wgpu::TextureFormat` when hardware-supported (querying `adapter.features()`) instead of CPU-decoding to RGBA8, OR
  2. Implement spec-compliant BC7 (all 8 modes, 0–7), ETC2 EAC, and ASTC 4×4 block decoders, plus UASTC/BasisLZ transcoding, and remove the silent approximation/fallback in `texture_transcode.rs:216-284`.

### Remediation 7: Wire Unconnected Viewport Shaders & Enable WebGPU HDR/Bloom on WASM
- **Current Limitation:**
  - `DEFERRED_RESOLVE_WGSL`, `CLUSTER_LIGHT_WGSL`, `SGGX_MICROFLAKE_WGSL`, `FOLIAGE_LOD_WGSL`, `SWE_SOLVER_WGSL`, `PRECIPITATION_WGSL`, `SURFEL_INSTANCED_WGSL`, and `EMF_VOLUMETRIC_WGSL` in [`shaders/viewport/mod.rs`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/shaders/viewport/mod.rs#L35-L49) are not wired into `PortalGpu`.
  - [`probe_hdr_format`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/bloom.rs#L21-L31) hardcodes `false` on `wasm32`, disabling HDR and Bloom in WebGPU browser builds.
- **Required Update:**
  1. Remove the blanket `#[cfg(target_arch = "wasm32")] return false;` in `bloom.rs:23` when running on a WebGPU backend that supports `Rgba16Float` render attachments and filtering.
  2. Wire `CLUSTER_LIGHT_WGSL` (clustered point/spot lights with shadow maps) and `DEFERRED_RESOLVE_WGSL` (spectral Jakob-Hanika + Wyman CIE 1931 XYZ resolve) into `PortalGpu` as selectable render passes, or explicitly mark unwired shaders as experimental/offline in code documentation so they are never cited as active renderer features.
  3. Add GPU cubemap IBL (pre-filtered GGX specular mipmap + irradiance cubemap) alongside the existing 4-probe scalar uniform fallback (`environment_lighting.rs`).

### Remediation 8: Upgrade Animation State Machine to Support 1D/2D Blend Spaces, Bone Masks, and Root Motion
- **Current Limitation:** Both `SkeletalPlayer` (`render/skeletal/player.rs`) and `AnimationStateMachine` (`qualia-sim/src/animation.rs`) only support 1 active clip + 1 linear cross-fade (`TransitionMode::CrossFade`).
- **Required Update:**
  1. Add multi-layer evaluation with per-bone `LayerMask` bitsets (128-bit mask for `MAX_JOINTS = 128`) so upper-body actions can blend over lower-body locomotion (modeled on Fyrox's [`MachineLayer`](file:///C:/github/Fyrox-master/fyrox-animation/src/machine/layer.rs) and Bevy's [`AnimationMask`](file:///C:/github/bevy-main/crates/bevy_animation/src/graph.rs#L64)).
  2. Add 1D/2D `BlendSpace` nodes using `qualia-core-db`'s existing Delaunay triangulation (`specialized_libs/computational_geometry/delaunay_2`) for barycentric pose interpolation.
  3. Add root-motion delta extraction (`RootMotionDelta { delta_motor }`) feeding into `CharacterController`.
  4. Replace the unconstrained array-only `ccd_ik` (`render/scene_graph/ik.rs`) with a two-bone analytical IK + joint-constrained FABRIK solver operating directly on `SkeletalPlayer` `Motor` joints (for foot-ground placement on uneven DEM terrain).

### Remediation 9: Add GPU Text/Font Atlas Rendering and Layout to `ZeroHeapUiCanvas`
- **Current Limitation:** [`ZeroHeapUiCanvas`](file:///C:/github/qualiaDB/crates/webizen-render/src/ui.rs#L113-L260) only computes 2D bounding boxes and bezier wire points for up to 128 widgets, with **no text shaping, no SDF/MSDF font atlas rendering, and no constraint/flex layout**.
- **Required Update:**
  1. Add a bounded zero-heap MSDF/glyph atlas text layout and GPU quad batch pass to `webizen-render/src/ui.rs` and `render/hud/mod.rs` so in-viewport HUDs, labels, and inspection overlays render natively inside `PortalGpu` without requiring external DOM/JS bridges.

### Remediation 10: Asynchronous / Range-Backed `.hmc` & `.q42` Browser Streaming and Worker Offloading
- **Current Limitation:** As documented in [`game-demo-remediation.md` F5 & F12](file:///C:/github/ecopunk/docs/planning/tech/game-demo-remediation.md#L62-L67), loading a 29 MB `.hmc` pack fetches the entire file into memory and verifies/decodes 905 `.10d` entries synchronously on the main browser thread (~12.17s main-thread stall). Furthermore, `GraphDatabase.volume_open` (`poet_host/invoke/graph/volume.rs`) rejects `wasm32`, and `LocalHost` (`vibe/src/catalog/local.rs`) returns an empty list for `GraphDatabase.sparql`.
- **Required Update:**
  1. Expose the slice-backed `.q42` + embedded `Q42LEX` reader (`q42/volume/opfs_source.rs`, `range_volume.rs`, `q42_lex.rs`) to the `wasm32` Vibe host so verified `.hmc` semantic slices can be queried via real engine SPARQL/SHACL in the browser without `LocalHost` empty-list stubs.
  2. Implement chunked/worker-offloaded `.hmc` entry verification and `.10d` mesh upload so main-thread frame presentation is never blocked during asset hydration.
