# Ecopunk demo rebuild — 29 October 2026

## Authorized scope and active production path

Timothy requested rebuilding all assets in the Ecopunk demo and significantly improving the demo. The deployed site stages `index.html`, `viewer/pkg`, assets, data and Vibe resources. Its browser runtime calls Rust `EcopunkPortal` generators; it does not fetch the stored historical model `.10d`/HMC files. Updating stored files alone cannot update visible runtime assets.

This batch updates those live generators and the browser package and produces a new native resource pack through the same builders. All eight visible site selections (six distinct sector templates), five lab scales and five currently exposed animal choices are included in the export matrix. Slider inputs are continuous; exported neutral/midpoint/endpoint states are representative checks, not exhaustive enumeration of every independent slider combination. The catalogue names “305 flora” and “164 fauna” do not establish that hundreds of individual qualified species models are rendered by this demo.

Existing reference models, studies, user-modified substrate producer/phases and earlier receipts are preserved. The new resources go under `assets/demo-rebuild-2026-10-29/`. Existing `viewer/pkg` was backed up to `before-pkg/` here before rebuilding. No push or public deployment is part of the recorded commands so far.

## Changes and ownership

- Living-system worker: `scene_builder_10d.rs` and `botanical_anatomy.rs`. Removed solid canopy hulls from the live tree helper; smaller folded/falcate leaves and triangle-derived normals; retained wood/foliage material grouping; used existing pipe-model branch radii and refined the leaf lamina/midrib. These are authored presentation changes, not botanical validation.
- Zoological worker: `zoological_anatomy.rs`. Distinct authored silhouette ratios across existing archetypes, coarse fluke/patagial/flight-feather structures. Existing public identifiers and layer semantics retained. Grouped taxa and anatomical validity remain limitations.
- Interface worker: root and viewer HTML. Viewport-prioritized responsive panels, disclosure controls, focus and touch movement, clearer labels and data/provenance status, preserving existing control IDs and native state ownership.
- Coordinator: reusable per-frame rover/walker pose buffers in the WASM host; build/export integration; native resource inventory/Q42/HMC; serial build and verification. The build script now preserves caches in an isolated target instead of deleting the workspace target after each build.

Three managed workers owned disjoint files. Shared source integration, compile commands and Cargo ownership remain with the coordinator. External algorithms/state/runtimes were not substituted for QualiaDB. Etalon remains the resolved graphics dependency; native and browser backend evidence are separate.

## Reproduction and evidence

`scripts/build-demo-native.ps1` builds the native simulation/geospatial/render consumers against existing compiled Core libraries, then compiles the demo library and `rebuild_demo_assets` exporter. `tools/demo-build-support` supplies only a missing CRC32C build prerequisite; it is not an application runtime. Dependency paths and compiler logs are under `target/demo-native/`.

The native exporter calls the live site/lab builders, decodes every emitted `.10d`, writes resource/source inventories and Q42 metadata, renders site reference images, builds HMC, and checks every added bundle entry. [The completed native export](native-export-final.log) records all eight sites and 25 representative lab states. The initial 61-entry export was subsequently refreshed with the current selected sources and 41 full default claim dossiers: [144 entries verified](native-package-final-refresh.log), 84,130,393 bytes. `--refresh-sources` verifies prior entries, replaces source copies and rebuilds the dossier entries; repeated refreshes do not accumulate duplicate dossiers.

`scripts/build-wasm.ps1` builds `ecopunk-wasm` into `viewer/pkg` using preserved `target/demo-wasm`. [The initial browser build](wasm-build.log) completed after 39m55s of compilation. Subsequent application-only rebuilds reused that cache. Compile warnings remain; this is not a warning-free build. A sandboxed packaging attempt could not download the matching wasm-bindgen CLI; the [authorized network retry](wasm-environment-build-retry.log) succeeded. No private provider or paid generation call was made.

Local HTTP preview is `http://127.0.0.1:8765/`. The first attempt timed out before its server was ready. A later browser check also exposed an old cached WASM binary: normal reload alone retained it. `update-demo-version.ps1` now binds both JS and WASM URLs to their actual digests, and uses the current wasm-bindgen object-form initializer. New scene counts established that the rebuilt bundle, rather than the cached binary, was executing. Root and `/viewer/` HTML routes are kept byte-identical.

## Executed checks and corrections

- The rebuilt WebGPU browser route exercised all eight site selections: sites 0/1 126,770 vertices / 170,606 triangles; site 2 126,700 / 170,564; sites 3/4 126,550 / 170,474; site 5 127,749 / 172,475; site 6 126,845 / 170,651; site 7 126,540 / 170,468. The earlier Talbingo binary reported 165,746 / 245,870; the rebuilt open canopy removes the opaque hull rather than adding more triangles to it.
- All five anatomy scales loaded: scale 1 3,675 / 3,820; scale 2 2,701 / 3,872; scale 3 4,033 / 4,964; scale 4 4,696 / 4,234; scale 5 default 3,329 / 4,724 and eagle 3,581 / 5,012. All five exposed vertebrate options were selected. Independent slider Home/End interactions exercised explosion/peel and cutaway/anthesis controls; counts are not evidence of anatomical correctness.
- Corrected actual material range bindings: fauna no longer receive the bark texture and people/vehicles no longer receive the pebble texture. Re-exported resources and native previews after this correction. The preceding resource pack is preserved in [before-materials](before-materials/), outside the public asset staging directory.
- Browser visual checking found an additive gray veil. Disabling tensor sprites/ambient particles alone did not remove it. An independent source trace confirmed that the no-probe environment fallback was added directly to surface colour. The Ecopunk host now explicitly supplies zero fallback through QualiaDB's existing probe API, retaining directional/ambient scene lighting. Before/after browser inspection confirmed improved material contrast; this is not parity with the native reference's authored probe lighting.
- Camera checks found that directly applying the native authored camera to the follow target could bury the eye in a steep bank. The host converts camera distance to its normalized frame and uses QualiaDB's orbit-eye calculation plus the same ground sampler as the actors for a bounded clearance adjustment. The corrected initial corridor view visibly shows the rover/river/bank. This is ground clearance, not a comprehensive camera occlusion solver or a test against sourced DEM terrain.
- Scene selection now resets the inspector to that scene's actual metadata, retains the selected site when switching experiences and hides corridor telemetry during anatomy inspection. It no longer leaves unrelated anatomy callouts attached to a landscape.
- Phone-sized 390×844 WebGPU startup rendered the rebuilt scene with collapsed panels and no initial horizontal overflow observed. Compact layouts allow one expanded panel at a time. The interface provides 44-pixel touch controls, zoom buttons, keyboard focus, movement hold controls, cancellation/blur release, and repeated-key protection for E/F mode switches. Physical phone hardware and assistive-technology qualification remain separate.

## Mobile compatibility work

The prior constructor required WebGPU texture admission even after selecting WebGL2. The host now accepts an explicit compatibility choice, retaining strict texture admission for the full-material route. The compatibility route uses QualiaDB's existing vertex-colour renderer and does not claim texture admission. A graphics selector exposes this choice; `?graphics=webgl2` selects it reproducibly. Actual backend/presentation receipts, rather than the numeric portal tier, drive its status badge.

The first real compatibility startup failed with `webgl2_fog_falloff_uniform_missing`. Inspection found that the required `u_fog_falloff` was declared but unused in the GLSL, so the driver optimized it away. The two-line repair in QualiaDB `render/anatomy/webgl2.rs` uses the uniform in the existing fog expression and sets it to the previous literal 0.35, preserving the intended expression. [The rebuilt browser log](wasm-webgl-uniform-fix-build.log) and subsequent live checks qualify this fix only when successful; a successful Rust compilation alone cannot prove WebGL shader initialization.

[ELVIS source findings and concrete terrain seam](elevation-source.md) record Timothy's elevation suggestion. The current demo still uses procedural relief. The separate, concurrently updated native substrate compiler was preserved and inspected; this batch does not claim its `.m64` is already consumed by the browser.

## Acceptance limits

This is a demo asset rebuild, not acceptance of complete species packages. Film/game/cartoon, expanded organ/state/clip/behaviour/scenario claims and human/domain acceptance remain governed by the existing pinned model catalogue and dossiers. Current scenes are illustrative: terrain/site metadata, anatomical formulas, cultural text, species identities and authored proportions are not validated merely because they appear in the demo. Unreviewed cultural interpretation must not be presented as adopted knowledge-holder evidence.

Technical criteria: builds complete for the pinned source/profile; emitted meshes are nonempty and decode; counts/indices and required streams are valid for the chosen path; all HMC entries verify; browser starts and exercises site/scale/archetype/sliders without errors; accessible panels preserve the viewport and controls; exact changed source and output digests are recorded. Visual review, scientific correctness and independent human acceptance remain separate.

Saved-byte checks are recorded in [saved-output-verification.log](saved-output-verification.log) and [artifact-digests.tsv](artifact-digests.tsv). They check all 41 inventory/dossier pairs, preserve the complete default catalogue claims, compare all six packaged source copies with current source, compare the two HTML routes, and verify that the cache revision derives from the built JS/WASM. These checks complement native decoding/bundle verification; they do not implement another graph store or acceptance authority.

**Remaining final verification at this revision:** finish the QualiaDB shader rebuild, retest WebGL2 and final responsive interactions, save final screenshots and regenerate byte digests. No public deployment, Git commit or push has been performed. No Poet rebuild or native cooperative-authoring acceptance is claimed.
