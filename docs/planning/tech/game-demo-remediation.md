# Game-demo remediation — verified findings & work packages

**Status:** draft v0.1 · **Inspected:** 2026-10-12 · scope: the `game-demo`
(Maslows Challenge) slow/poor-graphics diagnosis, its upstream source-level
risks, and the dependency-ordered remediation plan.

Companion architecture docs: [webizen-execution.md](webizen-execution.md)
(governed worker orchestration) and [pack-delivery.md](pack-delivery.md)
(verified pack acquisition service). Sequencing is folded into
[implementation-strategy.md](../implementation-strategy.md); gaps land in
[engine-requirements.md](engine-requirements.md); the Vibe-first boundary is
[development-modes.md](development-modes.md).

---

## 1. Evidence scope — what was and was not executed

**Repositories inspected**

| Repo | Identity | Working tree |
|---|---|---|
| `game-demo` | HEAD `726573da1bacbe6653c253f9bd931f88643c521c` | user edits in `activity_presentation.rs`, `lib.rs`, `rig_workshop.rs`; untracked planning doc 40 (`40-qualia-advanced-systems-adoption-and-stub-elimination.md`) — all preserved |
| `qualiaDB` | inspected HEAD `0f2e6f5c372818bd0cf28d1f94ea73caac3f3918` with unrelated user edits; declared pin `014e400738acf20f1b7abf3fd44a6f88d11e0bc0` (`qualia.ref`) | renderer `git diff` between declared pin and inspected HEAD is **empty**; other source not verified identical |

**Executed checks (Node, not a rendered browser)**

- Pack verification run against actual exports fetched via a local server and
  the user-supplied published URL
  `https://mediaprophet.github.io/game-demo/game.html`:
  - published WASM build revision `014e400738acf20f1b7abf3fd44a6f88d11e0bc0`
    (matches declared pin); local WASM build `69bee9f9a7bb68806927b2683086a3b4134f2e5c`
    (drift — F7);
  - both builds report `ArrayBuffer` `shared:false` — no shared-memory CPU WASM
    threads; browser worker deployment **not measured**;
  - both manifests: `assets` 905, `textures` 39; `verify_game_hmc` returns
    **945** verified entries vs the page's expected **906** (F1);
  - local HMC bytes 28,989,894 vs remote 28,989,889.

**Browser capture (added 2026-10-12, inspection environment):** a
headless-Chrome session against the published build exists at game-demo
`docs/qa/graphics-overhaul/runtime-2026-10-12/` — CPU profiles, RAF
intervals, loader/visibility/adapter receipts, screenshots. Scope limits:
**HeadlessChrome 154 on Intel UHD 620 (inspection machine, not user
hardware)**, published WASM pin `204863ad`, window covers **pack
hydration** only — steady-state frame cost remains unmeasured.

Loading-screen and graphics complaints remain **user-reported**; the
capture covers the load path, not sustained play.

## 2. Findings

Source references are paths relative to the owning sibling repo
(`game-demo/` or `qualiaDB/`).

| ID | Finding | Key evidence | Status |
|---|---|---|---|
| F1 | **HMC pack count mismatch.** `verify_game_hmc` returns *all* bundle entries (905 assets + 39 textures + 1 manifest = 945); `game.html` hydrate compared against `parsed.assets.length + 1` (906), rejecting valid packs and never activating the resident-HMC path. (Valid packs without texture entries would not mismatch this way.) Never weaken verification to fix. | `crates/rolling-commons-shell/src/lib.rs:342-357`; `web/game.html:3550-3573` | **fixed** — `feat/taxon-binding` `de96836`: texture count included (same formula as offline verifier + in-page self-test); browser re-verify pending |
| F11 | **Progress total lied under compression.** `readPackResponse` compared decoded stream bytes against compressed `content-length` → "24.0 of 7.0 MB". When `content-encoding` is present the total is unknowable from headers; loader now shows indeterminate bytes-received rather than a false denominator. | `web/game.html:3598+` | **fixed** — `feat/taxon-binding` `de96836`; confirmed by earlier browser capture |
| F2 | **Redundant scene recompilation.** `rebuild()` always calls `m.scene_build` before `tryLoadHmcScene`; `scene_build` recompiles recipes via `asset_compile.rs`; `refresh` invokes rebuild for UI/territory actions; startup calls it before hydration and the success path repeats. Compile only edited/authored assets; separate metadata vs asset vs UI refresh. | `web/game.html:1868-1932`, `:3017`, `:3565`, `:3627-3630`; `lib.rs:878-936`; `asset_compile.rs` | source-confirmed |
| F3 | **Hot-path animation risk — not measured dominant.** Per-asset player advance + upload each tick; per actor updates all material/shadow bounds then full-mesh AABB, invalidating shadows; scans all draws and entire fallback lineage per actor; CPU skin deformation + tangent rebuild each tick; 18-joint/eight-clip rig embedded for body parts, vehicle pieces also rigged. Direction: dirty spans/affected bounds, one finalization per frame, actor-level shared poses, rigid instance transforms where skinning unneeded, dirty fallback spans, animation/visibility LOD — preserving pose normals/tangents/semantic identity and zero-heap hot paths. GPU skinning only after profiling. | `render/portal/scene_animation.rs:54-90`; `render/gpu/animated_upload.rs:6-24`; `render/gpu/material_draws.rs:108-120`; `render/portal/cpu_mesh.rs:79-85`; `render/skeletal/player.rs:118-138` (qualiaDB); `resident_animation.rs:21-65` (game-demo) | source risk, unmeasured |
| F4 | **Text bridge in frame path.** Triples → `source.world().split` per values call; `state()` repeats for many properties; drive loop calls `state` each frame; HUD likewise. Direction: versioned projection updated on accepted state changes, compact typed ABI, no graph text parsing in frame path, Vibe controls authored behavior. | `web/game.html:858-906`, `:1999-2021`, `:2250+` | source-confirmed |
| F5 | **Whole-pack fetch.** Startup fetches the entire ~29 MB HMC into JS chunks plus a contiguous copy; `load_hmc_scene` accepts whole bytes. Q42 range support does not prove WASM/HMC integration: `serve.mjs` sends `200` whole-file (no `Range`); `HttpRangeSource` is `cfg !wasm32` native-only; BIDX (Q42 object→block index) differs from the HMC QBDL footer entry index — HMC entries are not "BIDX sections"; raw HTTP ranges are unauthenticated without a committed trusted index/digests or block proofs. | `web/game.html:3556`, `3576-3599`; `lib.rs:1359`; `scripts/serve.mjs:44-62`; `qualiaDB q42/volume/range.rs:148ff` | source-confirmed |
| F6 | **Quality default bug.** `Number(localStorage.getItem(...))` → `Number(null)=0` selects Economy instead of the intended High detail on first run. Quality affects effects, not full device LOD; quality-tier fallback labels do not prove actual backend. | `web/game.html:1373-1374` | source-confirmed |
| F7 | **Artifact drift.** Local exported WASM revision differs from declared pin (published matches pin). Pin match and hash-reproducible outputs required before comparisons; preserve user work. | §1 executed results | confirmed executable |
| F8 | **Partial Vibe adoption.** Real `.vibe` drive rules run via `GameDriveSimulation`, but JS owns walking, cameras, scene lifecycle, semantic state, and HUD. Standard-web JS is a limited host bridge; native Vibe only — extend missing Vibe capability upstream. Migration is not automatic speedup. Rust bounded kernels/GPU stay behind Vibe. | `lib.rs:1920-1988`; `web/game.html:2064-2077` | source-confirmed |
| F9 | **Scheduler ordering bug.** Descending priority / ascending deadline sort with `.pop()` dispatch reversed order. **Fixed upstream** — Track 4 `d7a0beb7`: `pending_tasks.remove(0)` + ordering regression tests (70 lines). | `inference/ambient_orchestration/scheduler.rs` (qualiaDB) | **fixed upstream** (Track 4) |
| F12 | **Hydration CPU is one hot wasm leaf.** Map-profile window 12.17 s during pack hydration (loader stage "Downloading the illustrated ground", `frames_presented` 29): `wasm-function[1580]` (code offset 3987064) = **88.5% exclusive sampled CPU**, `wasm-function[3647]` = 7.3%; JS text bridge `triples`/`values`/`state` (game.html:857-862, F4) ≈ 0.23% combined; GPU submission calls (`writeBuffer`/`beginRenderPass`/`fillText`) < 0.2%. RAF intervals 9.5–13.7 s, n≤5, `timedOut`, visibility `visible` — main thread monopolised during hydration, consistent with the pack-verify/mesh-decode path (~905-entry `.10d` verification on the main thread). Leaf is **not symbol-confirmed** — release wasm keeps only a `producers` custom section (no name section); attribution is behavioral. | `docs/qa/graphics-overhaul/runtime-2026-10-12/published-map-profile.json`, `published-*-raf.json`, `published-final.json` (game-demo) | **measured** — hydration window only, inspection hardware |
| F10 | **Loading screen + graphics, user-reported.** Not reproduced. Source risks: `setWorldLoading` resets `shownAt`, re-focuses root, clears `has-error` each call; delayed hide has no job-generation guard; `selectArtStyle` clears globals, rebuilds synchronously, starts fetch with no cancellation/version guard; `finishSceneLoading` polls `portal.tick` alongside normal RAF, catching silently; global error handler conflates errors with connection issues. Graphics: existing art plan (game doc 35) + art bible specify illustrated realism per `docs/reference/communitygrounds.jpg`; no approved baseline yet (QA README). Richer art needs one approved complete scene — not more primitives/palette/bloom; the fixed merged-scene path is not evidence of instancing/LOD adoption. | `web/game.html:493-524`, `1935-1949`, `2058-2098`, `3400-3407`; `docs/qa/graphics-overhaul/README.md` (game-demo) | user-reported; source risks |

## 3. Work packages (dependency order)

Owners are proposed: `game` = game-demo repo, `qualia` = upstream engine,
`eco` = ecopunk spec/contract work.

### R0 — Reproducible browser baseline + immediate correctness (game + qualia + eco)

Gate for feature/content **expansion** — the baseline and the immediate
corrective regressions iterate in parallel; writing an obvious-defect
regression does not wait for the full browser evidence set. Exact
source/artifact identity recorded; console/GPU receipts and actual
pixels/frame evidence collected. Fix F1 (count contract, not weaker
verification) and F6 (quality default). Validate true asset dependency
counts. Same pin across all artifacts.

- Acceptance: regression cases — pack with no textures; textured entry set;
  missing/corrupt dependency entries; malformed count; missing + invalid saved
  quality values; identical pin across artifacts.
- **Cannot be marked done from Node verification — requires rendered-browser
  receipts.**

### R1 — Loader & scene lifecycle (qualia + game)

The generic load-generation/first-frame contract, receipts, and any Vibe-host
changes are **upstream** work; the game composes its UI and fixtures on top of
it. Generation-guarded loading with one frame owner; retain last admitted
scene on style switch until the replacement is admitted and presented;
separate metadata/UI refresh from asset compilation (F2, F10).

- Cases: startup; rapid style switch A→B with stale A arriving last; stale
  completion/error; required vs optional failure; first-present timeout;
  resize/device loss; preserve camera/save/replay and input focus.
- R0 precedes any feature/content expansion.

### R2 — Hot-path correction & Vibe boundary (qualia + game)

- Cases: repeated non-state UI/territory focus produces **zero** scene compiles
  when content unchanged; accepted action changes only affected assets; one
  admitted world projection reused until state version changes; zero-alloc
  dirty animation span fixture — poses/normals/tangents/picks identical, static
  span untouched, bounds match full oracle; priority A>B dispatches A and
  earlier deadline before later (F9 regression). Profiling still required to
  rank F3. No copied custom renderer/logic (F4/F8 boundary holds).

### R3 — Verified pack service (qualia + eco)

Range-backed browser/native loader + cache/install per
[pack-delivery.md](pack-delivery.md) — test cases live there. Shared service
for nodes/Chora/Poet where supported, **not** a game-only JS downloader (F5).

### R4 — Webizen-governed workers (qualia + eco)

Capability-scoped supervisor, portable native/browser contract, Vibe
interfaces per [webizen-execution.md](webizen-execution.md).

- Cases: low-core/no-shared-memory; cancellation; worker crash; backpressure;
  stale versions; aggregate admission refusal; no privilege expansion.
  Renderer-worker OffscreenCanvas adaptation is conditional, not a launch
  assumption.

### R5 — Coherent visual slice (eco + game)

Workshop/resident/vehicle/gum/garden at illustrated-realism reference across
map/survey/walk; stable IDs preserved; textures, materials, normals, lighting,
contact, animation, LOD, readable UI. The engine source already contains
generic instancing/culling — the game's merged-scene path is not sufficient
batching; integrate or qualify the existing machinery before duplicating it.
Reuse Poet-suite authoring. Human-approved captures per device/profile — not
an automatic rubric pass, no invented numeric visual scores.

### R6 — Optional LLM lane (qualia + eco, after foundations)

NPC/NL proposals only; batch-limited prefill/decode/token quanta; fallback
interaction without model; renderer/simulation never blocked; stale/cancelled
replies discarded; no eviction of essential reserves; no unauthorized
disclosure.

## 4. Evidence checklist (for R0 onward)

Collect raw, no authored thresholds:

- Raw frame intervals/distributions; CPU vs GPU timing distinguished.
- Stage timing: fetch / verification / decode / compile / upload /
  first-present.
- Compile counts, draws, uploads, residency, memory, queue counts.
- Cold vs cached; identical cameras at default/high/balanced and
  map/walk/drive.
- Exact viewport, DPR, browser, GPU, backend, quality tier, commit, hash.

No universal budgets or thresholds are fixed before a hardware baseline exists;
new budget values require approval plus empirical justification. Outstanding
absences — actual console output, FPS, and loader reproduction — stay explicit
until captured.

## 5. Non-goals & boundaries

- **No verification weakening.** The F1 fix corrects the count contract the
  page applies; it does not reduce what the pack verifier checks.
- **No claim of measured root cause.** F3 is a source-level risk register;
  only profiling under R0/R2 can rank its contribution.
- **No inferred user hardware.** All recorded evidence is from this inspection
  environment; headed/headless mode and GPU backend must be recorded so
  results are never misreported as the user's machine.
- **No scope creep into new features.** R0–R2 stabilize what exists; content
  breadth waits for R5, and R6 is strictly optional after foundations.
- **Ephemeral tooling stays ephemeral.** The ad-hoc local server/preview used
  during diagnosis is not persisted as a standard deployment configuration.

## 6. Open questions

1. Which default scene/camera should anchor the R0 baseline before Walk/Drive
   captures?
2. True per-entry dependency counts for textured assets (F1 acceptance) —
   authoritative from manifest or from pack index?
3. Whether `graphics-view` fixed cameras or live input is the right baseline
   harness for identical-camera comparisons.
4. Owner split between game-side shell work and upstream engine work per
   package — proposed above, confirm before scheduling.
5. Partially answered by F12: hydration cost is a single wasm leaf (~88.5%,
   not symbol-confirmed — release wasm has no name section). Next capture
   needs a name section or in-page phase marks around verify/decode to pin
   it, plus a steady-state window. The loading-screen glitch itself remains
   unreproduced under a browser.
6. Which representative device budgets are agreed for qualification captures?
7. Upstream interfaces still needed: an authenticated browser HMC range
   reader, and OffscreenCanvas renderer-worker bindings.
8. LLM profile/residency budgets for the optional R6 lane.
