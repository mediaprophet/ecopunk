# RUN-01 stack source matrix

**Inspected:** 2026-10-24. **Scope:** declaration and lockfile source inventory only; no build, Cargo metadata/resolve, runtime, browser, GPU or device execution. Hashes are SHA-256 of the exact files listed in the JSON companion. Sibling repositories were read-only.

| Stack slice | Declared source / features | Lock evidence | Boundary and status |
|---|---|---|---|
| Ecopunk native/WASM crates | Root workspace paths into `../qualiaDB`; Etalon Git branch `qualia-main`; native target activates `gpu-runtime`, `gpu-native-dx12`, forge/privacy features; WASM target activates `wasm-full` and wasm-bindgen/web-sys bindings. Terrain selects Etalon native backend features and smaller WASM set. | Ecopunk `Cargo.lock` exists and its SHA is recorded; prior Etalon handoff reports the locked Git revision `ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6`. This was not independently parsed/resolved in this bounded pass. | Strong declared target split, not proof of feature closure, execution or no-JS native operation. Lock and local Etalon source revisions need reconciliation. |
| game-demo | Root manifest/lock exist. Existing Etalon handoff records local `[patch]` paths for `../etalon/compat/wgpu` and `../etalon/naga`; do not equate with Ecopunk's Git dependencies. | Lock hash pinned; exact active package/source selection not independently established here. | Source/patch presence only. Need resolve per target/profile and identify active call path. |
| QualiaDB | Root manifest/lock exist; prior Etalon handoff records Etalon Git branch dependencies and `wgpu`/`naga` patches. Ecopunk consumes local Qualia crate paths. | Lock hash pinned; exact effective patch/source selection not freshly parsed. | Upstream source is read-only. Need a source-qualified resolution record for relevant features. |
| Etalon | Local fork workspace; compatibility `wgpu` package is a facade over Etalon, per existing handoff. | No dependency resolution run. | API/source presence does not prove device interoperability or an Ecopunk frame using Etalon output. |
| Aether (`rust-app-framework`) | Root workspace has native, desktop and web members. Existing environment handoff records native renderer candidate (Blitz/AnyRender/Vello/winit), desktop WebView/interpreter path, web WASM/browser-binding path; root declares registry `wgpu 30.0.1`. | No root lockfile was found in the targeted root path inventory. Member lock/config/feature resolution was not collected. | `desktop`/WebView is not the native renderer; browser bindings belong to WASM evidence, not native no-JS proof. Controlled fork provenance and full consumer closure unresolved. |
| Poet | Root Cargo manifest and lock exist; hashes recorded. | No selected target/profile dependency extraction in this pass. | Exact framework source, renderer, and native Vibe host resolution remain unproven. |
| Webizen Desktop | Root Cargo manifest and lock exist; hashes recorded. Existing environment handoff describes Tauri/WebView2 host with Qualia/Vibe/runtime/render dependencies. | No selected target/profile dependency extraction in this pass. | Host services and controlled native renderer integration are not demonstrated by declaration. |
| Webizen OS-shell | Root Cargo manifest and lock exist. Existing environment handoff records registry Dioxus `=0.8.0-alpha.1`, desktop/web targets, optional embedded engine, and WebView2 on Windows. | Hashes recorded; no active target/profile resolution. | Registry Dioxus declaration does not prove Aether fork source; shell stage routing is not proof Poet editor is mounted. |

## Concrete next source-resolution blockers

1. Parse each lockfile into package blocks and compare effective `wgpu`/`naga`/Etalon/Dioxus/Vibe/Qualia source IDs against the manifest patches, including target-specific dependency closure. Current hashes pin inspected bytes, not those conclusions.
2. Reconcile Ecopunk's locked Etalon Git revision with local Etalon HEAD/dirty state and game-demo's path-patched compatibility facade. Do not update either lock as part of inventory.
3. Capture Aether member-level lock/config/patch state and establish which exact framework source the native, web, and WebView consumers resolve.
4. Establish per-profile resolved feature graphs for Ecopunk native and WASM, Poet, Webizen Desktop and OS-shell; declaration alone does not establish active features.
5. Trace the actual Poet-to-host-to-Qualia rendering/device path, including device/resource ownership across Aether and Etalon. No execution evidence was collected; native no-JS/V8 qualification and WASM-local/native-assisted qualification remain open.

This matrix is a bounded source inventory, not an implementation or product readiness claim. It does not establish that any dependency resolved, compiled, ran, rendered, or passed human review.

## Supervisor follow-up: actual locked package records

The JSON now also includes locked_package_records: relevant Etalon/wgpu/Naga,
Dioxus/Aether, Vibe and selected Qualia package name/version/source fields
extracted from every pinned lockfile. This supersedes the initial table's
“not independently parsed” limitation for those exact fields. Per-target
feature/patch closure, local source revisions and runtime/device ownership
remain unqualified. Absence of a lock source URI means local/workspace selection;
the actual path still comes from manifest/config resolution.
| Lockfile | Recorded family packages | Etalon API/compiler entries |
|---|---|---|
| C:/github/ecopunk/Cargo.lock | 23 | etalon 0.0.1: git git+https://github.com/mediaprophet/etalon.git?branch=qualia-main#ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6; naga 0.0.1: git git+https://github.com/mediaprophet/etalon.git?branch=qualia-main#ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6; wgpu 30.0.0: git git+https://github.com/mediaprophet/etalon.git?branch=qualia-main#ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6 |
| C:/github/game-demo/Cargo.lock | 22 | etalon 0.0.1: local/workspace; no source URI recorded by lockfile ; naga 0.0.1: local/workspace; no source URI recorded by lockfile ; wgpu 30.0.0: local/workspace; no source URI recorded by lockfile  |
| C:/github/qualiaDB/Cargo.lock | 51 | etalon 0.0.1: git git+https://github.com/mediaprophet/etalon.git?branch=qualia-main#ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6; naga 0.0.1: git git+https://github.com/mediaprophet/etalon.git?branch=qualia-main#ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6; wgpu 30.0.0: git git+https://github.com/mediaprophet/etalon.git?branch=qualia-main#ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6 |
| C:/github/poet/Cargo.lock | 1 |  |
| C:/github/webizen-desktop/Cargo.lock | 40 | naga 30.0.1: registry registry+https://github.com/rust-lang/crates.io-index; wgpu 30.0.0: registry registry+https://github.com/rust-lang/crates.io-index |
| C:/github/webizen-os-shell/Cargo.lock | 29 |  |
