# Chora engine — separate source and host integration boundary

**Status:** specification draft v0.1 · **Inspected / clarified:** 2026-10-24.
Timothy identifies **Aether** as the Dioxus fork at `C:/github/rust-app-framework`
and **Chora** as the Servo fork at `C:/github/chora`. Chora must remain separate
from the other code bases because of licensing. This document records that
project instruction and inspected source declarations; it does not establish
license clearance for an unbuilt distribution or select a linking topology.

Companions: [environment/framework](webizen-environment-framework.md),
[architecture](../architecture.md), [Poet/Craft](poet-craft-consolidation.md),
[interfaces](interfaces.md).

## 1. Identifiers and responsibility

| Component | Identifiers / boundary |
|---|---|
| Aether | Controlled application framework derived from Dioxus; source home `C:/github/rust-app-framework`. Owns framework evolution and qualified native/WASM UI adapters. |
| Chora engine | Separate embeddable web/spatial document engine derived from Servo; source home `C:/github/chora`. Preserves its own workspace, source history, notices, dependency/feature inventory and identifiable build/release artifacts. |
| Webizen host integration | Host-owned adapters outside Chora connect its generic engine interfaces to authorized Webizen/Qualia capabilities. Application authority, graph/state policy and private domain types remain outside the engine. |
| Poet / Craft | Unified authoring application and extracted authoring libraries. Chora is not a Craft extraction candidate and is not to be absorbed into their workspace or Aether. |

Earlier planning also uses **Chora** for a commons/exploration canvas, including
`chora_canvas` source and WellFair integration. Those historical identifiers do
not prove that a path uses this engine. Use “Chora engine” and “historical Chora
canvas” when the distinction matters. Reconcile actual source/dependency and
product relationships before renaming anything; no new product name is assigned.

## 2. Inspected evidence and limits

| Source under `C:/github/` | Observation |
|---|---|
| `chora/LICENSE`, root `Cargo.toml` | Root license text is Mozilla Public License 2.0; workspace declares `MPL-2.0`. File and third-party notices must be inventoried for the actual selected closure. |
| `chora/AGENTS.md` | Explicit separate MPL file boundary; prohibits `qualia-*` / `webizen-*` dependencies inside Chora and copying QualiaDB/Webizen Desktop implementation into it. Names generic embedding traits and `ffi/capi` as integration seams. These are repository instructions, not executed dependency checks. |
| `chora/README.md` | Describes an embeddable engine and intended embedded feature profile. Its “license-clean” description is a source claim, not completed dependency/distribution qualification. |
| `chora/components/servo/lib.rs` | Exposes Chora-branded public aliases/re-exports, including `Chora`, `ChoraBuilder`, `ChoraDelegate`, `ChoraError` and `ChoraUrl`. Internal `servo` package/type identifiers remain. |
| `chora/ffi/capi/{Cargo.toml,lib.rs,README.md}` | C ABI source includes `chora_*` wrappers and older `servo_*` exports. Package/lib names and README retain older identifiers; C-API library metadata names `chora_capi`. Branded wrapper presence does not establish complete ABI coverage, stability or deployment parity. |
| `chora/components/servo/Cargo.toml` | `embedded_library` is source-present and enables `js_jit`; comments describe omitting `media-gstreamer` and selecting the FreeType license profile. Actual unified features/transitive dependencies and font notices remain to be qualified. |
| `chora/Cargo.toml`, `chora/ffi/capi/Cargo.toml` | Workspace declares `mozjs`; C-API dependency explicitly enables `js_jit`. Changing engine ancestry from Chromium/V8 does not establish a JavaScript-independent workflow. |
| `rust-app-framework/{README.md,AGENTS.md}` | Names Aether; records separate Qualia-specific terms and upstream Dioxus MIT/Apache terms. Preserve these origins separately from Chora's declarations; no unified project license is inferred. |

Inspection did not build, run, copy, move, relicense or modify either fork.
No claim is made that all vendored or transitive files carry the root license.

## 3. Required integration boundary

| Requirement | Contract |
|---|---|
| REQ-CHORA-001 | Chora MUST remain a distinct repository/workspace and source/provenance boundary. Do not copy its implementation into Aether, QualiaDB, Webizen, Poet or Craft, or import their implementation into Chora. Keep patches to the engine in its owning source home. |
| REQ-CHORA-002 | Chora MUST NOT acquire `qualia-*` or `webizen-*` dependencies or application-specific graph/state/authority policy. Host integration uses generic engine-owned interfaces; adapters and domain policy live outside Chora. Audit the actual transitive closure as well as direct manifest declarations. |
| REQ-CHORA-003 | Interface definitions MUST have identified owners, versions and provenance. Qualify generic embedding traits/C ABI for needed behaviour; a Rust package alias, raw pointer or shared object name is not an integration contract. Do not duplicate private Rust structs across the boundary. |
| REQ-CHORA-004 | The selected source/feature/build/distribution closure MUST retain applicable file and third-party notices, modification provenance and required license/source materials. Record actual evidence; do not infer compliance from directory separation, a feature label or the README. |
| REQ-CHORA-005 | Integration MUST define lifecycle, event loop/thread ownership, input/accessibility, render/device/resource ownership, cancellation and teardown; where data/services cross the boundary, enforce origin/session/authority, bounds and revocation. Reuse Webizen's capability contracts. |
| REQ-CHORA-006 | Completion MUST identify actual Chora revision, artifact identifiers, enabled features, ABI/protocol version, host adapter and execution mode, with positive/failure/recovery cases and unresolved branding/profile gaps. Engine presence alone does not qualify launcher, Poet, WASM or native no-JS/V8 operation. |

Keep the **source/license boundary** explicit while deciding runtime topology.
The repository documents Rust embedding and C ABI integration. In-process
embedding versus a separate process/IPC, and the exact packaging of either,
remain decisions to qualify. A separate process is not imposed merely by the
root license declaration, and a C ABI is not treated as automatic clearance.

Chora is not assumed to replace Aether's native renderer or to be available
inside WASM. Its role in web/spatial content hosting and any native service
exposed to WASM must be identified per capability. Chora's inspected JS/JIT
profile cannot establish the requested native JS/V8-independent authoring
workflow. If that workflow requires Chora, the relevant dependency/profile gap
must be resolved and evidenced explicitly; moving application logic into its
browser scripting engine would not satisfy the shared Vibe/native contract.

## 4. Work packets and completion evidence

All packets remain open specification work for implementation allocation.

| Packet | Deliverable / evidence needed |
|---|---|
| CHORA-01 | Source, license/notice, feature and dependency inventory for engine and selected third parties; identify actual artifacts and source/notice obligations for the intended distribution. Preserve separate provenance from Aether and host code. |
| CHORA-02 | Inspect actual host consumers and integration needs; map historical Chora canvas references to engine/app roles without assuming identifiers or replacing preserved functionality. |
| CHORA-03 | Ratify generic interface/ABI and host-adapter ownership, versioning and runtime topology. Record missing generic operations for engine-side work and application operations for host-side work. |
| CHORA-04 | Qualify embedding, rendering/input/event loop, resource lifecycle, service authority and failure/recovery on the selected host/backend. Record actual results, not documentation claims. |
| CHORA-05 | Verify the selected dependency/feature closure and packaged notices/source materials, required native no-JS/V8 profile and all declared native/WASM-assisted use cases. Report open branding/API/profile cases and controlled-update compatibility. |

## 5. Open decisions

Exact role and required capabilities in Webizen/Poet; host-adapter source home;
mapping of historical canvas terminology; runtime linking/process topology;
versioned ABI coverage; selected dependency/features and distribution obligations;
native no-JS/V8 profile feasibility; actual source/artifact provenance and update
compatibility. Aether's identifiers, Chora's identifiers and the requirement to keep
Chora's code base separate are established.
