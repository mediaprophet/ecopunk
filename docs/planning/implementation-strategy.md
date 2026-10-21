# Implementation strategy — RD&D frame & priorities

**Status:** v1.0 · updated 2026-10-20 following the forensic audit against `C:\github\bevy-main` and `C:\github\Fyrox-master` ([`docs/reports/game-engine-audit-and-qualiadb-manual.md`](../reports/game-engine-audit-and-qualiadb-manual.md)) and the resulting 10-Epic Engine Uplift Plan ([`docs/plans/qualiadb-superior-game-engine-master-plan.md`](../plans/qualiadb-superior-game-engine-master-plan.md)). v1.0 inserts both the game-demo remediation prerequisite (Phase 0A, per [tech/game-demo-remediation.md](tech/game-demo-remediation.md)) and **Phase 0B: QualiaDB Superior Game-Engine Architecture Uplift (Epics 1–10)**.

## 1. The RD&D driver: `game-demo`

The first implementation focus is **`C:\github\game-demo`** (Maslows
Challenge) — deliberately dual-purpose:

- a **useful product** in its own right (Australia-first browser/WASM game);
- the **development test-case** — its requirements force QualiaDB
  functionality to be *actually implemented and functioning*, not merely
  specified. Its written requirements (ecology contract, procedural pipeline,
  modality adoption in doc 40) are the demand curve ecopunk and the engine
  build against.

The dependency triangle to keep straight:

```
        game-demo (product + test-case)
          consumes ▲        ▲ forces
          packs    │        │ real impl
                   │        │
   ecopunk ── produces packs/corpus ──► qualiaDB (engine)
        ▲                                  ▲
        │  hero assets, authored media     │  capabilities
        └── poet + poet authoring studios ─┘  land upstream
```

**ecopunk's role in the loop:** the sourced, licensed, provenance-bearing
corpus — locality profiles, species selections, layer packs, style-tiered
assets — that game-demo's pipeline consumes. Where game-demo needs capability
X, that is *engine work* (the QualiaDB-only contract); where it needs *data*
Y, that is ecopunk work.

## 2. The authoring uplift (model quality)

Model quality uplift runs through the **Poet authoring studio suite** (per-medium
tools — see `tech/interfaces.md` §6) + agent-friendly authoring upstream.
ecopunk's stake: our style-tier quality gates (`assets/style-and-scale.md`)
define the *bar*; the *studio tools produce the assets that meet it.

## 3. Delivery targets (from style-and-scale §8)

Mobile/tablet · desktop/laptop · AR (elevation-aware anchors) · **film/TV
linear HD** — declared per artifact; linear output is a first-class product.

## 4. Proposed priority order (v0.3 — for confirmation)

Ordered so each phase proves the next thing game-demo needs:

### Phase 0 — Foundations in writing (now, near-done)
Planning docs complete enough to work against; gap register consolidated
(`engine-requirements.md`); namespace/PID decision **G16** resolved to
`https://webizen.org/` (`https://ns.webizen.org/`).

### Phase 0A — Game-demo stabilization & qualification (prerequisite)
The driver/test-case must be *reproducibly measurable and correct* before
pipeline breadth means anything: findings F1–F10 and work packages **R0–R4**
in [tech/game-demo-remediation.md](tech/game-demo-remediation.md) —
reproducible browser baseline + correctness fixes (R0), loader/scene
lifecycle (R1), hot-path and Vibe-boundary corrections (R2), verified pack
service (R3, [tech/pack-delivery.md](tech/pack-delivery.md)), and
Webizen-governed workers (R4, [tech/webizen-execution.md](tech/webizen-execution.md)).
Stabilization acceptance before breadth work resumes:

- compatible artifacts (same pin across builds) with a **verified textured
  pack load**;
- a **race-safe loader** with a real first-present receipt;
- **no non-state recompiles or text reparse in the frame path**, with the
  animation corrections qualified (F2–F4);
- **named-device browser performance evidence** against agreed budgets —
  captured pixels/frame data, not Node checks;
- the **first approved illustrated-realism hero scene** per R5 — this gate
  stands even though broader R5 content work continues afterwards.

R0–R4 deliver their services incrementally; the runtime-worker
OffscreenCanvas/shared-memory extensions and the optional R6 lane are **not**
gates on the earlier fixes.

### Phase 0B — QualiaDB Superior Game-Engine Architecture Uplift (Epics 1–10)
Full technical specification: [`docs/plans/qualiadb-superior-game-engine-master-plan.md`](../plans/qualiadb-superior-game-engine-master-plan.md).
Eliminates the core 3D engine bottlenecks identified when auditing `qualiaDB` against `Bevy` and `Fyrox`, while preserving `qualiaDB`'s Two-Tier Zero-Heap Contract (`AGENTS.md §0-A`), 48-byte `NQuin` governance, PGA `Motor` algebra, and `qualia-sim` continuum physics:

1. **Epic 1 (Zero-Heap Hybrid Scene Graph & SOA ECS):** Replace `BTreeMap<String, SceneNode>` (`render/scene_graph/node.rs`) with a generational `NodePool` + prefab `InheritableField<T>` delta inheritance + fixed-arena SOA archetype tables with `Tick(u32)` change detection and topological PGA `Motor` propagation.
2. **Epic 2 (Multi-Mesh Slab Allocator & GPU Indirect Culling/LOD):** Replace `PortalGpu`'s single `mesh: Option<MeshGpu>` slot (`render/gpu/mod.rs:484`) with `MeshSlabArenaGpu` (4,096 meshes × 4 LODs), GPU compute frustum + 2-phase HZB occlusion culling, `multi_draw_indexed_indirect`, and automatic `LOD0`→`LOD2`→`SGGX`/`Surfel` switching.
3. **Epic 3 (GPU Vertex-Shader PGA Motor Skinning & Morphs):** Replace CPU `f64` `skin_vertices_into` + per-frame `queue.write_buffer` uploads (`animated_upload.rs`) with static `JOINTS_0`/`WEIGHTS_0` attributes, double-buffered `RigMotorPaletteBuffer`, and GPU vertex-shader PGA sandwich skinning + morph target deltas.
4. **Epic 4 (Multi-Layer ABSM, 2D Delaunay Blend Spaces, Root Motion & Constrained IK):** Upgrade `SkeletalPlayer` and `qualia-sim/src/animation.rs` with 128-bit bone masks, 2D Delaunay `BlendSpace`, root motion extraction, and analytical 2-bone + constrained FABRIK Motor IK.
5. **Epic 5 (Active GPU Spectral Deferred, Clustered Lighting, Cubemap IBL & Refractive Water):** Wire `CLUSTER_LIGHT_WGSL` (256 local lights + 4-cascade CSM + point/spot PCSS), `DEFERRED_RESOLVE_WGSL` (Jakob–Hanika + Wyman CIE 1931 XYZ spectral resolve), `HYDROLOGIC_OPTICS_WGSL`, `FOLIAGE_LOD_WGSL`, `SGGX_MICROFLAKE_WGSL`, GPU Cubemap IBL, GTAO, SSR, and enable WebGPU HDR/Bloom on `wasm32` (`bloom.rs:21-31`).
6. **Epic 6 (Zero-Heap 3D Rigid-Body Physics & Continuum Coupling):** Build a 3D dynamic/static BVH (`cast_ray`, `cast_capsule`, `cast_sphere`), zero-heap GJK/EPA narrowphase, 64 Hz XPBD rigid-body & 6-DOF Motor joint solver, `Ragdoll`, 3D `CharacterController`/`RoverKinematics`, and 2-way force coupling with `geomechanics.rs`, `hydro.rs`, and `metamer_fspm.rs`.
7. **Epic 7 (Full glTF 2.0 Hierarchy/Skin/Anim/Morph Compiler & Native KTX2/BC7/ASTC):** Upgrade `render/assets/glb.rs` to import full node hierarchies, `SKL1` skins, PGA motor clips, and `MRP1` morphs; upload hardware-compressed BCn/ASTC/ETC2 textures directly to `wgpu` and implement spec-compliant BC7 (all 8 modes) + BasisLZ/UASTC transcoding (`texture_transcode.rs`).
8. **Epic 8 (3D Spatial Audio Graph & BVH Occlusion):** Synchronize `AudioEmitter`/`AudioListener` scene nodes with `qualia-audio` HRTF/Holosonic synthesis, Freeverb buses, and 3D BVH raycast + `ACOUSTIC_VOXEL_CONE_WGSL` acoustic occlusion.
9. **Epic 9 (Zero-Heap Flexbox/Grid UI, GPU MSDF Text & Studio 3D Gizmos):** Add Flexbox/Grid layout and MSDF glyph atlas rendering to `ZeroHeapUiCanvas` (`webizen-render/src/ui.rs`), plus 3D PGA Motor gizmos and Command-Stack Undo/Redo in `webizen-studio`.
10. **Epic 10 (WASM Byte-Backed `.q42`/`Q42LEX` Seam & Range-Streamed `.hmc` Workers):** Expose slice-backed `.q42` + `Q42LEX` to the `wasm32` Vibe host (retiring the `LocalHost` empty-list stub) and stream/verify `.hmc` entries on background Webizen workers.

### Phase 1 — First pipeline end-to-end (thin slice)
**Goal:** one reviewed pack consumed by game-demo — proof of the whole loop.

- **P-data**: *one locality-profile pack* for game-demo's sample corridor —
  licence-clear (P0/P1) sources only: NSL names backbone (subset), IBRA7/NVIS
  context layers, a bounded ALA occurrence slice, TERN vegetation.
- **P-pipeline**: LOD vocab registry extension (Darwin Core, TERN, ENVO,
  QUDT) → occurrence→quins mapper → `.q42` compile → `.hmc` pack → pack
  manifest + review-gate pass.
- **P-quality**: **one hero species family** (the bottlebrush/banksia/
  grass-tree/saltbush/kangaroo-paw group game-demo already uses
  illustratively — replace illustrative with sourced+styled) in T3,
  desktop+mobile delivery classes.
- **Depends on:** G1 (only if raster layers needed — can defer), G16
  (namespace), review-workflow fast lane.

### Phase 2 — Contribution loop live
- **P-interfaces**: observation+identification contribution kind through
  Poet (`vision` invoke) → proposals → review fast-lane → pack deltas —
  the smallest complete contribution loop.
- **P-pipeline**: media capture → fragment quins (ma-ont) — needs G6
  temporal-encoding unify.
- **P-economics**: the contribution loop's obligation accounting runs on the
  fair-compensation model ([tech/compensation.md](tech/compensation.md),
  G43) — labour classes + PPP/living-wage benchmarks + agent-kind modulation
  feeding the verified `royalty()`/`commons_cost` machinery.
- **P-network**: seed+cache node (pack availability) — testnet first (G13).

### Phase 3 — Layer breadth + quality uplift
- More layer families (terrain `.10d` exists; historical borders; Country
  sovereignty layer with custodial process — sovereign lane).
- **Socioeconomic statistics layer** ([tech/statistics.md](tech/statistics.md)):
  ABS Data API SDMX ingest (G38), ASGS editioned geographies, one SEEA/land-
  account bridge pack (G41), statistical claims stored under STAT-AX epistemic
  rules.
- **Reconciliation hooks** ([tech/reconciliation.md](tech/reconciliation.md)):
  staleness metadata on holdings + one event-triggered reconciliation path
  (FIRMS/DEA disturbance → affected-locality re-check flags) as the corpus
  freshness proof (G46).
- Hero-first expansion to more families; AR delivery class + anchored layers
  (G22); bioacoustic packs (G2+G25).

### Phase 4 — Products & network
- Chora `biosphere_world` bound to reviewed packs (G21); encyclopedia
  surface (webcivics); node roles expand (resolve, edge); micropayment rail
  when G11 lands; film/TV linear deliverables as quality demonstrator.

## 5. Constraints that shape sequencing

- Stubs are being eliminated upstream by another agent — sequence *around*
  confirmed-implemented capability, not against the stub list.
- Licence review gates are per-source; the first packs should pick
  *already-clear* licence lanes (P0/P1) to prove the pipeline end-to-end.
- The webcivics participation tier defines the minimum-spec device envelope;
  mobile delivery class is the harder constraint.
- **Range-addressable packs change the consumption pattern — with honest
  scoping.** `Q42ByteRange` + `BIDX` + HTTP `Range`/`206` + trustless IPFS
  partial-CAR (`q42/volume/`, `webtorrent_routes.rs`) are verified substrate —
  but BIDX is the `.q42` object→block index, **not** the `.hmc` QBDL entry
  index, and the range source is native (`cfg !wasm32`); browser integration
  is open. Consumption is **both** explicit install *and* streaming through
  one verified service ([tech/pack-delivery.md](tech/pack-delivery.md)) — the
  current game-demo still does an unintegrated whole-pack fetch (remediation
  F5). Phase-1 acceptance should include a range-fetched layer read where the
  service supports it, not just a whole-pack load — it also makes
  partial-volume cache nodes useful from day one (network-nodes §1).
