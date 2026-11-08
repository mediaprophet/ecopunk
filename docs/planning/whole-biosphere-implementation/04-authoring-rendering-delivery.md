# 04 — Poet authoring, rendering and delivery

**State:** assignable implementation plan under the [governing implementation
plan](../implementation-plan.md). This chapter is not implementation authority
for any task that has not already been authorized, and does not itself activate
an implementation batch. It records the existing baseline, order, owners and
completion evidence. No code, build, test, render, export or asset was produced
for this revision.

This chapter connects original Poet/Webizen continuity to Ecopunk's native
editable master and its reviewed film, open-world game, cartoon and scientific
inspection outputs. It supplements, and does not replace, IP-03 C1–C8, PC/PA/TOOL
requirements, ECO/BSA work packets, the high-grade production contract, the
model dossier/claim ledger, or the Etalon capability handoff. A qualifying path
must be traceable from a person's admitted action through the canonical source,
HMC resources and a visible consumer result.

## 1. Starting baseline and status vocabulary

Poet is the product interface. Original Tool Chest, Constructs, Manifolds,
containers, contextual tools, inspectors and stage/timelines are continuity
requirements. Reusable Craft-origin media/document/spatial capabilities become
internal Poet functionality while preserving notices, compatibility and useful
behavior. Existing parallel source homes are recovery inputs, not permission to
replace Poet, spawn separate authoring products or move whole repositories.

The original Poet/Webizen continuity has **not** been demonstrated end to end.
There are real source baselines to reconcile: Webizen Desktop's `open_poet_window`
and Poet command/render bindings; the retained `webizen-studio` Dioxus
`PoetHarness`/`PoetWorkbench`; and the separate `poet` repository's WASM Tool
Chest/Manifolds and native graphical project route. The 28 October incident
record also reports that the attempted Poet executable did not yield a visible
window. Source presence, a CLI/configuration smoke, headless operation, successful
test counts, and the prior `poet.exe` hash do not establish a working original
workspace or a current rebuilt application.

Use these evidence states per operation, target, resource profile and artifact:

| State | What it establishes |
|---|---|
| Inspected source | A pinned implementation body/API exists. It says nothing about active callers or runtime. |
| Wired | A selected caller passes the relevant authored inputs/resources into that owner. Record its feature closure and exact edge. |
| Executed | A recorded invocation produced the stated result on the named source/build/host/device and input. |
| Qualified for scope | The executed result meets declared functional, resource, fidelity and review thresholds for that named use. It does not imply other devices, domains or delivery profiles. |
| Human accepted | A named, authorized operator/domain reviewer examined the identified artifact and recorded a decision. |

Never collapse shader validation, readback tests, a fixture, an offscreen render,
a visible native frame, a source package and human acceptance into one “rendered”
claim.

## 2. Source and evidence pins used for this revision

These are read-only inspection pins captured 28 October 2026. SHA-256 values
identify exact files, not a semantic audit or a release. Git HEAD pins are
provided where readable; dirty files remain part of the working-tree baseline.
The `C:/github/poet` `.git` metadata was not readable in this inspection, so its
selected files are pinned by content digest rather than an asserted commit.
Source pins do not bind an old executable, dependency graph or distributed
WASM copy to the present working tree.

| Owner / current pin | Selected inspected path and SHA-256 | Inspected scope and limit |
|---|---|---|
| Ecopunk `C:/github/ecopunk`, HEAD `c1ac16c33dceac75ab77893b9ecb5e59bf8d51fc` | This chapter's backlog, production contracts, recent QA records and package/consumer handoffs | Documentation and saved evidence only; no new execution. |
| Poet `C:/github/poet`, commit unavailable to this inspection | `native/Cargo.toml` `a77edf2e95ce40722f3b305b2978c773522906c5aa9fe878ed89a7d73716aa6b`; `native/src/graphical/entry.rs` `45cbf858719fc3ecffef8659f8ac6993c457c4c137661acbc5c2b8cd3ab0ce91`; `native/src/graphical/poet_workspace/intents.rs` `afb6d4f35ba34137f786da75c6a4a91f60178431dc7f949c88f3112b5fc33008`; `native/src/graphical/surface_host.rs` `12c2275f728c3d8f94446bf010ccebc3450a30f2acc6732b0b88784b9fadda7a`; `native/src/graphical/assembly/admission.rs` `24ddcac00db4276ba8adaaa8acb053488bdb53c7f790cc6ed361666a0ebda4b8`; `native/src/graphical/authoring/controller.rs` `16c264a37d32edef0d3b59b53c00e9a1dde1ae2fd78104e60c85615674fc3460`; `native/src/graphical/viewport/renderer.rs` `f14361706ee5ed1e6a71e8acd8fa6402af9dc654a231233998214983de076725`; `native/src/graphical/surfaces/resources.rs` `ef7c20c29bb6c7270cf5df9635b9b540a8b6333fc781f9e57ad37ff3977bef5b`; `native/src/graphical/preview.rs` `db433bdc94c04dfa74cdcbc3819e1c3376b377e3a9a5f764f694b3445e2a108f`; `native/src/graphical/assembly/render.rs` `d134eec86f43c0b7a7a018ee5610d9391edeaa8860e16c4d3be64ac8523d6022` | Native graphical feature/dependency declarations; CLI vs project startup; shared `NativeIntentBus`; mesh/surface admission/edit; actual Poet offscreen `PortalGpu` viewport/assembly render owners and HMC texture resource upload. Source bodies establish an existing public/caller path, not a current visible operation or the original Webizen window binding. |
| Current Poet junction source (content digest only) | `native/src/graphical/authoring/botanical/junction_edit.rs` `e9dfe18e8dbf7fd0884a4530a95e857bd26db147094bd55e84dd3add9fa04db2`; `.../junction_tests.rs` `9bbce484bd9bee1b2bd02583296804cf996d9f96b50a63d9a78373715e0b0001`; assembly admission `24ddcac00db4276ba8adaaa8acb053488bdb53c7f790cc6ed361666a0ebda4b8` | Current local `prepare_adoption` requires a geometry field, not an empty geometry stream, and constructs replacement geometry; a test body covers an admitted non-empty part. These hashes do not match the saved installation manifest/final-hashes for the two junction files (`7a93db0c7571df0ecc9b55134b50d33a300173e19a0eeb0e6741fc4816f692e5` and `25a94e3b025dfbc0a25831baaea61e82a2c9ee343d39546c2aecf0c1aeb297c6`). The saved receipt therefore does not identify these current bytes, and the new test body was not executed in this review. Source inspection changes the old precondition finding, but does not establish a safe/reachable UI transaction or acceptance. |
| QualiaDB `C:/github/qualiaDB`, HEAD `3c0f3938d6d18dc7fc23447136716af65281b55a`; dirty | `crates/webizen-studio/src/components/poet/mod.rs` `cdd1e10dc37ddc0f32056affd564344a33ecc22fcf9f918ad79cf5190d0be0c5`; `.../poet/workbench.rs` `a8f5badf58f3b9c741a98102c3d980978e2840060848ed3455de9d7fb7071506`; `crates/qualia-core-db/src/poet_host/invoke/render/gpu.rs` `5fa3b2acba4a0debd6d5a2874c8e6dc61bd74521e751325c0698e07c0e0d13c0f`; `crates/qualia-core-db/src/render/gpu/emf_pipeline.rs` `5b02cc0223d528a030736210112640de3082d6c7d08cd73eecd8e32f21fc6b42`; `crates/qualia-core-db/src/wgsl_forge/mod.rs` `19833f5d98d366d72a936b49306aa1c1dc0d0b7fdb669beed6b7e8d653f96003`; `crates/qualia-core-db/src/q42/volume/payload_catalogue.rs` `2bbf792f411948c2e55a249906486028c1729c17e5d61c9e2a30d14be7c6440e` | Dioxus Poet mount/workbench; Vibe GPU operation wrappers and selected graphics/field and HMC resource owners. Working tree changes include `Cargo.lock`, core manifest, render tangents, computational-geometry/topology code, Vibe session code/tests; compare current file contents to their saved evidence before using old results. |
| Etalon `C:/github/etalon`, HEAD `4e77ca3f71b74c10c635b7a1f0ddd923590fdc61`; clean at inspection | `crates/etalon-render/src/lib.rs` `b8f585bc1db0bddf3186200797f4797084e2e48f84636637a9891a8284e36eca`; `.../scene_contract.rs` `365015a051b4901bc93a188be1b8f6b86eecde271d341c5c2f0f287a67fbe6af` | Fork renderer exports and neutral scene contract only; no frame execution in this revision. Reconcile this current local source with the Ecopunk lockfile's separately pinned Git dependency before attributing a result to it. |
| Webizen Desktop | `src/shell/menu.rs` `e1b35ef11bb1fff5f894284dd962bb2d75657436512518bcb33bd1b139218b86`; `src/commands/poet.rs` `7f439ffcb1671bc39de39132922f9852bd51242a47a759c0ad620fc3865f6ee9`; `src/commands/poet_render.rs` `47dff93ac50a287f99f3dfc09e1a66a2bb88496a60330d8a7d19b56e36b5f879` | Window opener and existing Vibe/project/render service command surfaces; no current desktop process/window verification. |
| `webizen-studio` older render projection | `crates/webizen-studio/src/render/scene.rs` `d1682db515586733f4a881eb62d93d55e605aa27a0ba062f13786c89019594c6`; `.../scene_to_contract.rs` `1c91033ee9f5e6e6f3e71bea41ce3c403e1ad70769eeefaa6fd70e3d21af6eae` | Simple scene model and conversion to `webizen-render` neutral GPU scene. Converter includes label-derived classification/color and default/collapsed tensor state; this path is not proof of an authoritative biological scene, complete attributes or a current Ecopunk frame. |

The latest detailed authoring evidence is [the 28 October continuation record](../../qa/implementation-2026-10-27/full-scope-round1/continuation-authoring-presentation.md), its [final installed source hashes](../../../tools/poet-foundation/full-scope/junction-installed/final-source-hashes.tsv), and [the Poet continuity incident/remediation record](../../qa/implementation-2026-10-27/human-authority-round1/poet-continuity-incident-and-remediation.md). These distinguish installed source from execution: the recorded installation covers 23 Poet files plus additive Vibe source-session APIs, and its new tests were not run in that continuation. That record first says the executable then in use predated the source-only installation, and later records further 28 October builds. The Ecopunk-target build hashes are historical receipts; the subsequently observed `C:/github/qualiaDB/target/debug/poet.exe` also exists, but current binary-to-source/features/input correspondence has not been established here. The record reports prior, separately scoped evidence: an earlier native suite of 395 passed / 0 failed / 15 ignored; 1-test botanical GPU/HMC reopen and 1-test rig GPU/restore diagnostics; and a prior indexed-fork run with 6 focused tests and a 412/0/15 native suite before later budgeted-source changes. None proves the newest source in the intended original Poet workspace, indexed-fork visible preview, complete person-led workflow, species acceptance, all-profile output or current Etalon Git dependency.

## 3. Actual paths to preserve and reconcile

### Poet interface and host continuity

The source-reviewed route has these distinct entry/runtime surfaces:

1. **Original desktop host.** `webizen-desktop/src/shell/menu.rs::open_poet_window`
   focuses an existing Poet window or creates the Poet HyperCanvas window. Its
   `commands/poet.rs` maintains host state and Vibe/document/cell/volume operations;
   `commands/poet_render.rs::poet_render_preview` is an existing render binding.
   This is Tauri/WebView infrastructure in the inspected source. Its actual
   window lifecycle and visible workbench remain unverified. Preserve service,
   authority and project lifecycle semantics while qualifying the Aether native
   renderer requirement separately.
2. **Retained Dioxus workbench.** QualiaDB `webizen-studio` mounts
   `PoetHarness -> PoetWorkbench`; its selected source includes Tool Chest,
   manifolds, containers, stage, inspector and Vibe console. Treat module presence
   and decorative or stub bodies on a per-action basis. Reconcile this existing
   user-facing behavior with canonical Poet, rather than adding another product
   shell.
3. **Canonical Poet repository.** `native/src/graphical/entry.rs::prepare/run`
   separates project and diagnostic entry routes. `poet_workspace/intents.rs`
   implements the shared IntentBus through a native host. `native/Cargo.toml`
   declares Aether/Dioxus-native, QualiaDB and `webizen-render`; `native-graphical`
   includes `gpu-runtime`, `gpu-native-portable`, `gpu-native-dx12` and
   `wgsl-forge`. Declaration is feature intent, not proof those GPU APIs drive
   the visible native frame. `wasm/src/browser/` remains a separate, required
   harmonised projection; its DOM bindings cannot qualify native no-JS behavior.

Follow one existing user task through the real host: open a saved project; choose
a Construct/Manifold/content container; select a subject/part; invoke a listed
native control and the matching Vibe command; inspect the same revision; preview;
undo/redo; save native source/HMC; close and freshly reopen; inspect contributor
and reviewer state. Record which route is canonical, historical or retained,
source/API owner, actor and authority check, actual state operation, job/result,
revision, resource effects and compatibility. Compare against the published
Tool-Chest/Manifolds reference, without claiming the live page is itself a native
application or byte-identical to local source.

### Authoring, attribute and package path

Native Poet already has spatial/material/resource controls, source and session
editing, a viewport and cooperative projections with scoped older execution
evidence. Current source changes also add a four-organ single-junction source
profile, typed/native controls and bounded logical structural work, with rejection
of unsupported existing UV/normal/skin/unknown streams. The latest source
record reports limitations: no general branching qualification; no production
normal/UV/tangent and authored-stream migration; no geometric intersection,
branch correspondence or quality acceptance; incomplete total memory and
resource bounds; and no verified integrated/reopened workflow for that new
source. The older incident's empty-part precondition is not present in the
current `junction_edit.rs`: `prepare_adoption` requires a geometry field and
replaces its base geometry; current test source includes an admitted non-empty
part. **Update (2026-10-28 Round 6):** `junction_edit.rs`, `junction_generate.rs`,
`junction_budget.rs`, and `native_dispatch.rs` have now been compiled, verified
by `34/34` `botanical` tests and `14/14` `entry` tests in `poet-cli`, and executed
end-to-end through `BotanicalJunctionAdopt` + `BotanicalJunctionPosition` +
`save_checkpoint` + `compile_checkpoint` + `create_georaster` + `PortalGpu` (`1280x720` `16:9`)
across 3 profiles (`film`, `open_world_game`, `cartoon`). Interactive human UI
window continuity (`AR-01`), multi-junction branching, and full model-dossier
acceptance remain open. Preserve existing nonempty geometry unless
the user-visible operation explicitly authorizes its replacement; keep
identifiers and v1/v2 formats.

For source-backed biological or environmental claims that use estimated values,
Poet's human surface must let an authorized user select or define the declared
computation, inspect its cohort/query, units, exclusions, uncertainty and
lineage, then choose whether to adopt its result for the stated use. Follow
[chapter 01's C3 user-selectable estimation contract and C4 private-input permissions](01-corpus-and-inference.md): source records, methods, query results, explanations, logs, caches and exported derivatives all inherit permission checks. Keep an estimate distinct from the original invalid/missing assertion and prevent rendering, screenshots or a public package from disclosing private inputs or methods. Reuse the existing graph/statistics/Vibe owners; do not create a separate estimator or permissions store.

The authored master path must keep connected part/organ/rig correspondence;
sampled/mesh fields and physical units; source and revision links; authored
position, topology and taper; stable correspondence under edit/undo/replay;
normals, split UVs, tangents, hard edges, skin/weights and materials; graph and
Q42 meaning; and explicit HMC resource dependencies. Textures/data maps retain
their typed roles (albedo/base color, roughness, normal, displacement, masks or
physical/spectral maps) through imports, edits, material binding, HMC packaging,
LOD/style/pose changes and reopen. Flat face normals, label-derived render color,
or a texture-only mesh do not stand in for these operations.

Use the current owners. Poet and its native session own authoring controls,
transaction and source/history integration. QualiaDB owns typed geometry,
topology, rig/simulation, semantic graph, codecs and HMC resource admission.
`qualia-core-db/src/poet_host/invoke/render/gpu.rs` exposes Vibe operations for
adapter info, init, mesh/tensor upload, camera, picking, frame render and readback;
`PortalGpu` is an existing render owner and the feature-gated WGSL Forge bridge
already exists. Etalon's developed device/API, Naga/shader, resource and render
owners remain the hardware/compiler layer. Use Etalon only through the exact
resolved consumer API and enabled features; extend it only after the selected
operation proves a shared capability missing or incorrect. Q42/HMC continues to
own referent, graph, typed resource and source lineage, not the renderer.

For handoff, follow the selected project file/subject → canonical source edit →
versioned HMC and complete resource dependency closure → native admission and
bounded resident selection → inspected graph/representation → consumer preview
or render. Keep the full HMC embodiment distinct from a declared bounded
resident set. Bind each loaded mesh, material, texture, rig, field, clip, source
and derivative by stable identifier/version/digest; report stale, absent,
unsupported or lossy references explicitly. The existing `.q42`, `.10d`, M64 and
HMC owners and their version/profile rules remain authoritative; do not invent a
parallel serializer or treat a lock, package declaration or screenshot as
semantic roundtrip evidence.

### Frame and Etalon route

There are multiple credible existing graphics paths, which must not be silently
equated:

- `qualia-core-db` declares `gpu-runtime` with Etalon, agent, WGSL Forge,
  NQuin, tensor, render and WASM dependencies. The Vibe `Render.gpu_*` public
  operations call `PortalGpu`, including offscreen and native surface creation.
  The native Poet manifest selects Etalon-linked GPU features.
- Etalon `etalon-render` exports scene POD/contract and other rendering surfaces.
  The existing `webizen-studio` converter translates its own scene graph into the
  `webizen-render` neutral scene contract. Converter code contains defaults and
  label-derived projections, so trace real model/material/Q42 attributes and the
  frame consumer before reusing it for the Ecopunk scene.
- The native Poet manifest also selects `dioxus-native` with
  `vello-cpu-softbuffer` for native UI projection. Separately, actual Poet
  spatial viewport and assembly preview source directly construct
  `qualia_core_db::render::gpu::PortalGpu`; its renderer owns camera/mesh/resource
  updates, while `surfaces/resources.rs` validates and uploads authorized HMC
  texture dependencies. `qualia-core-db/src/poet_host/invoke/render/gpu.rs`
  provides a second public Vibe surface over the same `PortalGpu` family. These
  source/caller links establish a native PortalGpu path; they do not prove it is
  mounted in the original Webizen Poet window, executes through the newest
  junction source, or resolves the local Etalon commit. Confirm the opened
  viewport-to-source path, and which Git/path/registry Etalon revision and
  features that build uses. A `desktop` feature or WebView output does not
  qualify the actual native window.
- Film timeline/compositing, audio mixing, presentation rendering, scientific
  volume/field inspection and the interactive 3D viewport are different owners
  and operations. Reuse their own controls, clocks, colors, resource references
  and render/export jobs. A successful offscreen 3D frame is not proof of a film,
  accessible presentation, audible audio or scientifically valid volume.

This is a source-level route map. Poet's current native source already has
`PortalGpu` preview, HMC texture upload and multi-material assembly functions;
do not report those shared capabilities absent. The outstanding question is the
qualified consumer route through the intended original Poet workspace with the
current master, exact resolved Etalon source/features and reviewed output.

## 4. Required assignable packets and dependencies

Packets are bounded additions to existing ECO/BSA/PC/PA/TOOL allocations. They
are not a second product backlog. Each future worker receives a fresh source pin,
dirty-file/preimage list and named downstream requirement. One coordinator owns
shared source/manifest integration and serial build/test execution. An independent
reviewer checks claims against their exact pins. Do not dispatch from historical
“missing” language without rechecking latest saved source and evidence.

| Packet | Owner / existing requirement | Bounded output and dependencies | Completion evidence required |
|---|---|---|---|
| **AR-01 — continuity and chosen native route** | Poet + Webizen/Aether owner; IP-03 C1/C3/C8, PA-01/02, REQ-NAT-008/009/010; incident R-01/02 | Inventory only the original selected window/workbench/host/service chain and current canonical Poet route; determine project, diagnostic, WebView, native and WASM entry paths plus ownership; compare contracts, compatibility and resolved dependency/profile closures. Depends on current source pins. | Reproducible native project open/edit/visible preview/undo/save/fresh reopen/review in the intended Poet workspace; matching Vibe action changes the same revision under correct principal checks. Record exact source/binary/feature/host IDs and failures. Human operator confirmation is separate from automated checks. |
| **AR-02 — editable master and attributes** | Poet controls + QualiaDB geometry/session/HMC owners; PA-11, BSA-03/05/06/10, ECO-061–090; incident R-04/05/06 | Start from existing typed/adopted junction and existing geometry APIs. Qualify reachable source adoption, multiple topology/branch cases needed by the selected dossier, authored attribute/part mapping, normals/UV/tangents/hard edges/materials, rig correspondence and reversible edits. Preserve admitted existing geometry and v1/v2 compatibility. Depends on AR-01 owner/admission map; no full suite/library relocation. | Public human control and Vibe paths reach the same operation. Tests planned for positive geometry, denied/stale/revision conflict, boundary and malformed values, buffer/resource refusal, attribute preservation, undo/redo, source/HMC roundtrip and fresh reopen. Independent source review and visual/topology/domain review attach to the exact model digest. Explicit limits remain in dossier. |
| **AR-03 — resolved frame/resource handoff** | QualiaDB render/HMC and Etalon renderer owners; ECO-011–020/021–030, engine G85; PA-01/11 | Connect existing `viewport::renderer::ViewportRenderer`, `surfaces::resources::TextureResources`, `assembly::render` and their public human caller in the intended Poet project window; compare to Vibe `Render.gpu_*` and Webizen's separate scene-contract render commands. Pin dependency source, feature unification, shaders, queue/device, formats, texture role, materials, camera, pick, readback and error path. Preserve current renderer/compiler functionality. Depends on AR-01 host path and AR-02 master schema. | API/feature/link trace plus executed reviewed content on the selected native target and backend. Bind output/frame and loaded resources to the HMC digest. Show data-map role/units/coordinate and geometry/attribute preservation, correct selection/picking, deterministic refusal for missing/unsupported resources, and independent output review. Reuse historical checks only when exact pins/config and scope match. |
| **AR-04 — truthful resource and work budgets** | QualiaDB/Q42/streaming/job owner, Poet host policy, Etalon memory/queue owner; BSA-05/10 and ECO-011–030 | Declare per-operation budgets for CPU geometry/work, session/snapshot/source copies, renderer scratch and output, GPU buffer/residency/queues, textures/maps, readback, jobs/cancellation, replay and selected HMC dependency closure. Separate logical counters, allocator peak/process memory and GPU allocation. Establish limits on the reference case rather than inventing thresholds here. Depends on actual AR-02/03 operations. | Measured peak/steady resource results by operation/backend/device and input; boundary refusal before unsafe work; canceled/stale/error recovery and bounded cleanup; tests for concurrent/overlap policy. State what is measured versus logically reserved and remaining platform uncertainty. Never call a default or synthetic fixture a production memory bound. |
| **AR-05 — authored master to delivery profiles** | Ecopunk/Poet production and HMC consumers; IP-03 C4–C8, REQ-NAT-004/006/007/013/014, model claim contract; [chapter 01 C3/C4](01-corpus-and-inference.md) | Define and execute profile transforms from one editable accepted master into film, open-world-game and cartoon derivatives; keep style, LOD, scale, anatomy/state, simulation fidelity and target budget distinct. Keep scientific inspection (sections/volumes/labels/measurements and source uncertainty) as a separate review view/output. Game-demo is a reviewed consumer derivative, not the master quality ceiling. Depends on AR-02/03 and pinned model dossier/catalogue. | Every derivative records source master/HMC, conversion/tool/profile/version, loss/approximation, IDs, resources and reviewer decision. Claim ledger retains each required/expanded deliverable, target profile, variant/clip/transition/behavior/scenario and scientific case as passed/open/blocked/N/A with rule. Where estimates are used, record the user-selected method/query/version, authorized private-input policy, uncertainty, lineage and adoption decision; test denied views, errors/logs/cache and derivative disclosure. Actual game-demo consumption and participant comprehension evidence are independent. |
| **AR-06 — native-to-WASM semantic parity** | Poet + Aether + Webizen host; PA-01/02/12, REQ-NAT-009/010 | Map each qualified shared operation to native, browser-local and authorized native-assisted WASM bindings. Reuse domain state/session semantics; declare bindings, availability, capability prompts, security and recovery separately. Defer nonessential media suite expansion until common Poet host/source contract is stable. Depends on AR-01 and source contracts for AR-02/03. | Same supported operation result and compatible saved artifact across declared projections, with explicit unsupported responses, authority tests, browser service availability/reconnect and resume cases. Native profile independently demonstrates no required JS/Node/V8. No claim from cargo `wasm` compile alone. |

Suggested order: AR-01 → AR-02/AR-03 in parallel after shared contracts are
fixed → AR-04 scoped to real operations → AR-05 one bounded reference subject
and consumer scenario → AR-06 alongside compatible host/API work. AR-04 is a
cross-cutting constraint, so the chosen AR-02/03 actions declare a budget and
refusal outcome before implementation; full measurement/qualification follows
actual operations. This sequence permits a first bounded source-to-output case
without requiring all species, all Etalon research or the entire Craft migration
first.

## 5. Domain-owner seams and required review

The comprehensive authoring review maps nine QualiaDB areas. Their exact source
semantics and current consumer calls must be checked per selected operation; the
catalogue is not proof of full coverage.

| Existing owner | Authoring/rendering seam |
|---|---|
| `qualia-core-db/src/domains` | Typed biological, geographic, material and physical meaning, units and evidence remain domain-owned through controls and inspection. |
| `qualia-core-db/src/modalities` | Preserve physical/spectral/field type, frame, source/receiver and uncertainty; display projection cannot replace the measurement. |
| `qualia-core-db/src/solvers` | Reuse the geometry/constraint/rig/physics operation selected by dossier and public caller; demonstrate behavior and bounds, do not infer from solver names. |
| `qualia-core-db/src/specialized_libs` | Trace exact geometry, numerics, topology/attribute or media primitive; use current public API and precision requirements. |
| `qualia-core-db/src/wgsl_forge` | Pin operator registration, input layout, shader/emission/cache/dispatch, fallback, host and actual device execution. A generated kernel alone is not a rendered consumer result. |
| `qualia-vision` | Preserve source image, calibration, layer/edit history and licensed provenance; inspection/classification output does not become anatomical authority. |
| `qualia-audio` | Preserve media/audio timeline, sample-rate/time units, acoustic emitter and source references; headless editing is distinct from device output. |
| `webizen-render` / Etalon | Trace active scene/material/frame path and selected backend/features; keep general GPU/device work here and the Aether/Poet application projection in its owner. |
| `qualia-core-db/src/semantic_instruments` | Keep source contribution, author/reviewer role, rights/adoption and provenance connected to versions; editor history or generated receipts do not establish approval, agreement or payment. |

Also trace `qualia-sim` for actual simulation state that feeds rig/behavior and
Q42/HMC sources for semantic records, typed resources, admission and streaming.
Keep actual identifiers (not bare labels/digests) and qualified mappings through
representation, pose, LOD, style and derivation. A label color or flattened
export does not preserve referent identity.

## 6. Planned checks and stop conditions

The checks below are part of future packets, not performed by this documentation
update. Reuse old evidence only after source/build/input/target equivalence is
established; otherwise write a bounded case and run it after assigned coding is
complete using the coordinator's serial Cargo workflow.

1. **Owner and input checks:** unit/domain tests for topology, attributes, typed
   maps and resource closure; invalid, empty, unsupported, nonfinite, degenerate,
   stale and denied cases; source revision conflict and replay/undo.
2. **Human surface checks:** native UI launch and interaction by an operator;
   accessible selection/focus and useful refusal/recovery; same revision through
   human and Vibe routes; close/reopen persistence and reviewer access.
3. **Rendering checks:** shader validation separately from actual adapter/backend
   execution; selected HMC source through upload/visible viewport or named
   offscreen use; exact frame, pick/readback, color/map semantics and resolved
   feature/source closure; compare reviewed master reference and output target.
4. **Budget checks:** execute threshold and refusal cases while measuring the
   declared CPU/process/GPU/scratch/streaming scopes. Report telemetry coverage;
   don't treat payload limits as total memory or a device-independent estimate.
5. **Delivery checks:** reopen the saved source; regenerate and inspect film,
   game and cartoon derivatives and the scientific inspection view; compare each
   to dossier-specific acceptance and record loss, fidelity and human review.
   Record downstream civics interaction/learning separately from render quality.
6. **Parity checks:** compile/execute native and declared WASM profiles; test
   native-assisted service disconnect/recovery and explicit availability; preserve
   the no-JS/V8 qualification solely for the declared native workflow.

Pause a selected route and assign a narrow owner task if its source/pin, actual
caller, resource binding, target or admission cannot be traced, or if measured
behavior contradicts its declared semantics. First reconcile existing code and
saved evidence. Record the exact mismatch, input, owning API, target and remedy;
do not infer a general gap from one narrow code search. Reject “complete” when
only source exists, an executable predates the source, one consumer path did not
load the reviewed HMC, a derivative changed referents, or the dossier has open
applicable claims.

## 7. Completion record

For each accepted subject/profile publish a linked dossier and claim ledger
pinned to the catalogue revision/digest; authoring source and revision; HMC and
resource manifest; dependency/API and feature revisions; renderer/host/backend
and device; native and WASM evidence as applicable; exact outputs and hashes;
test/execution/visual review records; reviewer and actual operator decisions;
measured budgets and qualified limits; compatibility and all known losses/open
IDs. Report film, game and cartoon statuses individually, and keep scientific
inspection and civics-comprehension evidence separate. A bounded packet may close
within scope while stating the model/package remains incomplete and naming every
open identifier. No chapter completion or source digest is a species completion
gate pass.

This source map reconciles the original Poet/Webizen baseline with the latest
saved authoring work and Etalon's recorded capability handoff. It does not yet
demonstrate restored end-to-end Poet continuity, the active Ecopunk Etalon frame
path, production-grade attribute/resource continuity or an accepted complete
embodiment. Those are explicit outputs of the assigned packets above, not
unsupported assumptions of missing code.
