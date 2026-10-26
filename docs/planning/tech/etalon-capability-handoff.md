# Etalon — implemented capability, consumer integration and Ecopunk handoff

**Status:** source-inspection draft v0.1 · **Date:** 2026-10-24.
Timothy identifies **Etalon**, `C:/github/etalon`, as his developed wgpu fork,
open for continued development. Its local source and evidence take precedence
over incomplete online descriptions. Development remains part of Ecopunk's
purpose. This inspection neither certifies universal superiority nor assumes
upstream wgpu's limitations describe Etalon.

Companions: [production backlog](ecopunk-production-backlog.md),
[graphics audit](ecopunk-graphics-audit-2026-10-24.md),
[environment/framework](webizen-environment-framework.md),
[engine requirements](engine-requirements.md).

## 1. Inspected local evidence

Paths below are relative to `C:/github/`. No build, GPU execution, benchmark,
dependency resolution or new upstream comparison was performed in this review.

| Source | Evidence / practical implication |
|---|---|
| `etalon/AGENTS.md`, root `Cargo.toml` | GPU stack includes API, core validation, HAL, shared types/sync, Naga and remote interfaces. Preserve validation at the safe boundary; HAL is not a validating API. Workspace version is `0.0.1`; version alone does not identify feature parity or source provenance. |
| `etalon/etalon/src/backend/webgpu.rs` | Source includes null/undefined error-scope handling and supported-limit property checks. These are concrete fork changes to investigate/reuse, not reasons to prescribe a replacement browser shim. Actual target execution remains separate evidence. |
| `etalon/etalon-hal/src/vulkan/adapter.rs` | Cooperative-matrix memory-model capability handling is present. Device/backend support and enabled feature closure still require actual qualification. |
| `etalon/crates/etalon-agent/src/lib.rs` | Exports capability state, compute lanes, completion tokens, leases, slab/memory and interoperability surfaces. Source-present resource/compute layer beyond a bare rendering facade. |
| `etalon/crates/etalon-wgsl-forge/src/lib.rs` | Kernel IR/scheduling, certification, cache/autotune and WGSL/CUDA/HLSL/MSL emission/compilation interfaces exist. A compiler/exported function is not proof that a particular rendered frame uses its output. |
| `etalon/crates/etalon-nquin/src/lib.rs` | 48-byte NQuin layout, sieve and radix interfaces; GPU sorter/sieve exports are feature-gated. Trace selected features and actual graph workloads. |
| `etalon/crates/etalon-tensor/src/lib.rs` | Prepared tensor/forward arena, KV cache, attention, top-k, CUDA and DirectML-related surfaces exist. These do not author believable geometry or guarantee a rendering quality improvement. |
| `etalon/crates/etalon-render/src/lib.rs` | Exports scene POD contracts, volumetric pipeline, HUD layout and EMF pipeline. Inventory current active consumer routes before reallocating existing renderer work or assuming every planned mesh/film feature has migrated. |
| `etalon/crates/etalon-wasm/src/lib.rs` | SharedArrayBuffer ring, transferable transport and worker steward source exists. Browser scheduling, permissions, native-provider bridge and real application recovery remain distinct qualification cases. |
| `etalon/compat/wgpu/{Cargo.toml,src/lib.rs}` | Compatibility package declares `wgpu 30.0.0` over Etalon and forwards features. Seeing `wgpu` in a consumer does not establish use of unmodified upstream wgpu; package source and enabled features matter. |
| `ecopunk/Cargo.toml`, `qualiaDB/Cargo.toml` | Declare Etalon Git-branch dependencies and wgpu/Naga patches. Ecopunk's inspected lock contains Etalon Git source `ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6`. This is a lockfile observation, not identifiers proof for the current local Etalon tree or deployed WASM. |
| `game-demo/Cargo.toml` | wgpu/Naga patches point at local `../etalon/compat/wgpu` and `../etalon/naga`. Local paths and Ecopunk's Git dependencies can select different source states; compare resolved packages before diagnosing behaviour. |
| `rust-app-framework/Cargo.toml` | Aether declares registry `wgpu 30.0.1`; Etalon's inspected compatibility facade declares `30.0.0`. Audit semver/source/transitive resolution and interop, rather than assuming a patch selects all callers or identical public names imply compatible devices. |
| `qualiaDB/crates/qualia-core-db/Cargo.toml` | `gpu-runtime` includes Etalon, agent, forge, NQuin, tensor, render and WASM dependencies. `ecopunk-terrain` and `ecopunk-wasm` select `gpu-runtime`. This is feature intent, not proof that every subcapability executes. |
| `qualiaDB/.../src/render/hud/mod.rs`, `src/wgsl_forge/mod.rs` | Feature-gated Etalon HUD re-exports and forge bridge source exist. Reconcile their coverage before declaring the migration wholly unwired or creating duplicate adapters. |

## 2. Existing completion claims must be reconciled, not discarded

`etalon/dev-docs/implementation/EXECUTION-TRACKER.md` records **299/299** TODO
items resolved across six waves and names commits for several waves.
`05-IMPLEMENTATION-AUDIT-AND-REMAINING-GAP-REPORT.md` describes nine earlier gaps
and later records **all nine resolved** on 20 October. The earlier defect text
must not be copied into a new current-bug list without reading those resolutions.

These are historical completion/evidence assertions. Preserve them and link
their exact revision, invocation, target and results to the capability ledger.
Source review can establish presence and wiring; execution evidence establishes
only its exercised scope. A previous check need not be repeated without a changed
dependency, missing evidence, applicable new target or unresolved discrepancy.
Equally, “299 resolved” does not establish complete Ecopunk model production,
film output, all-device conformance or superiority in every workload.

The local playbooks, migration inventories and consultant reviews are prior
work inputs. Their scope includes much more than the immediate Ecopunk graphics
defects. They are not a reason to postpone a truthful native reference scene
until every research objective or alternative-engine comparison is finished.

## 3. Required capability record

Create one row per operation/backend/profile, not one “engine complete” flag:

```text
Capability ID / intended Ecopunk outcome / owning source and public API
Local source revision and dirty state / resolved crate source, features and lock
Documented claim / inspected implementation / selected active consumer call path
Input HMC/entity/representation IDs / shader and pipeline identifiers
Requested, supported and enabled adapter features and limits
Execution evidence: target/backend/device/driver, invocation, result, artifact
Correctness/visual/performance scope and agreed threshold; failures/limitations
Historical receipt reuse / stale or missing evidence / new verification needed
Status: documented | source-present | wired | executed | qualified-for-this-scope
Disposition: reuse | integrate | repair | extend | not-needed-for-this-outcome
Open task IDs / authority and acceptance reviewer
```

Statuses describe evidence, not a promise that every intermediate state is a
fresh task. Native and WASM evidence, declared versus measured allocation bounds,
shader validation versus GPU results, and device-wide versus per-queue completion
must remain distinguishable. “Superior” requires a named property, workload,
comparison version, device and result; it is not a substitute completion status.

## 4. Allocation and immediate order

The [master backlog](ecopunk-production-backlog.md) ECO-011–020 allocates source/
version/capability reconciliation, safe rendering/resource interop and qualified
consumer use. Engine register G85 records this upstream handoff.

1. Identify the actual dependencies and active frame path used by the chosen
   native reference build; do not conflate local, Git and deployed artifacts.
2. Reuse existing Etalon/Qualia functionality for texture roles, material maps,
   lighting, graph/resources and rendering. Fix Ecopunk's demonstrated input and
   handoff errors in their owning layer.
3. Qualify the small set of required native operations with reviewed content.
   Extend Etalon only for a demonstrated missing/incorrect shared capability.
4. Expand backend/performance/representation coverage and continue the independent
   R&D programme with explicit outcome/measurement criteria.

General GPU/device/shader/memory mechanisms belong in Etalon; semantic graph,
domain authority, format admission and authored policy remain with their owning
Qualia/Vibe libraries. Aether owns application framework concerns; Webizen owns
host lifecycle; Poet/Craft own authoring integration; artists/domain specialists
own content quality. Existing allocation/migration agreements must be reconciled
before moving modules. Chora retains its separate source/license boundary.

Open: actual current revisions/dirty trees, complete resolution across consumers,
scope/freshness of recorded completion evidence, active versus dormant pipelines,
required backend/quality thresholds, device interop and remaining outcome-specific
gaps. Continued development is available; no sibling edits are allocated here.
