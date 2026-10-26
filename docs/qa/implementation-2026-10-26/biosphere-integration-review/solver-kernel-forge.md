# Solver, scientific kernel and Forge reuse audit

2026-10-26. **Selected function-body audit, not comprehensive Core coverage.**
This pass covers selected files under `solvers`, `specialized_libs` and
`wgsl_forge`, plus narrowly traced export/host/CLI consumers. No builds, tests,
Cargo, Git, sibling changes or GPU execution were performed. Current working
source is not a clean revision, compiled feature set or deployed capability.
The [coverage manifest](solver-kernel-forge.coverage.json) records actual file
reads/ranges, reviewed symbols and body-read limitations; search hits are not
counted as audited bodies. Other root directories remain outside this pass.

The three module roots were inventoried before selecting bodies. `solvers/mod.rs`
exports calculus, linear algebra, optimization, transforms and other families;
its own comment corrects the blanket no_std claim. `specialized_libs/mod.rs`
exports computational geometry and computer vision, with target/feature gates
around physics, engineering, chemistry and other libraries. `wgsl_forge/mod.rs`
exports emission, schedules, oracle certification and actual compute dispatch.
Core `lib.rs:275` gates Forge to native plus `wgsl-forge`; `:552` exports
specialized libraries and `:1978` exports solvers. Module existence is not proof
that every operation is fixed-memory, calibrated, host-bound or native-UI usable.

## Sixteen concrete anchors

All paths in the table are under `C:/github/qualiaDB/crates/` and are exact
source anchors. E = exported implementation; H = traced Core invoke/CLI caller;
R = source reaches real GPU commands; Q = newly executed evidence (none here).

| # | Source / reviewed operation | Actual behavior and qualification boundary |
|---|---|---|
| 1 | `qualia-core-db/src/solvers/calculus/ode_adaptive.rs:54` `integrate_dopri5` | E: caller state plus checked 8×dimension workspace, finite/config guards, adaptive error norm, step bounds and max_steps. Accepted steps mutate state before a later error; transactional host use requires a candidate state. No explicit cancellation token or species law is supplied. |
| 2 | `qualia-core-db/src/specialized_libs/physics_simulation/ode.rs:8` `integrate_ode_samples` | The real higher-level wrapper reuses DOPRI5 per requested sample interval, allocates workspace and clones every snapshot. Default max_steps applies per interval, not one cumulative job budget. It is not a zero-heap streaming solver merely because the underlying kernel uses caller buffers. |
| 3 | `qualia-core-db/src/specialized_libs/physics_simulation/population.rs:7` `run_logistic_growth` | E+H: actual derivative `r*N*(1-N/K)` calls the shared ODE wrapper. It returns sampled population plus accepted/rejected counts. This is a declared logistic model, not ecological calibration or a developmental tree/wallaby model. Source rates, capacity, time units and limits remain application evidence. |
| 4 | Same `population.rs:80` `step_flocking_simulation` | E: no-heap pairwise separation/alignment/cohesion, speed clamp and position integration. O(n²); updates in place, so later agents observe earlier agents' updated state. No synchronous-snapshot guarantee, spatial index, habitat/contact law or species calibration is established. Not a wallaby locomotion solver. |
| 5 | `qualia-core-db/src/specialized_libs/computational_geometry/botanical_hulls.rs:79` `asymmetric_hull_into` | E+H: actual six-point asymmetric crown mesh, caller vertices/triangles, capacity refusal and deterministic ring/fan topology. It is a crown envelope, not branching anatomy/foliage. Capacity arithmetic is unchecked; host calculates and allocates capacities before kernel parameter rejection. Require finite values, safe integer/product/index limits and work admission. |
| 6 | Same `botanical_hulls.rs:207` `fit_asymmetric_hull` | E: a one-pass extrema-derived envelope with fixed shape exponents 2.0. The first point initializes extrema while finite checks start at `.skip(1)`; preflight all points. This fit is not an optimized species reconstruction or a measurement-quality certificate. |
| 7 | `qualia-core-db/src/specialized_libs/computational_geometry/canopy_radiation.rs:221` `rasterize_orthographic_id_buffer` | E: CPU directional orthographic triangle ID/depth rasterization with index checks and caller buffers, not a placeholder renderer. Cost depends on triangle projected coverage×pixels; explicit cumulative budget/cancellation and finite-coordinate admission are not parameters. A late invalid triangle can follow scratch mutations. |
| 8 | Same `canopy_radiation.rs:371` `integrate_canopy_radiation` | E+H: multiple source directions, visible organ silhouette areas, weighted PAR/STAR, gap fraction, supplied ground-area LAI and a fixed `-ln(gap)/0.5` effective-LAI approximation. Reuse for declared canopy analysis; do not infer full spectral scattering, photosynthesis, ecology or measured plant response. Ground area <=0 produces NaN LAI; source/unit/organ-ID assumptions require admission. |
| 9 | `qualia-core-db/src/solvers/transforms/fourier.rs:23` `dft`, `:115` `idft` | E+H: actual allocating f64 CPU O(n²) forward/inverse reference. “f64-exact” in source means the retained f64 implementation contract, not mathematical exact arithmetic. Existing Core host intentionally calls this path; finite sample, size/work and cancellation contracts are still needed for large scientific inputs. |
| 10 | Same `fourier.rs:81` `dft_accelerated` | E: explicit f64→f32 opt-in for native Forge with eligible power-of-two N=2..1024 and available adapter; otherwise/error uses CPU f64 reference. Return type does not identify chosen device/precision. Preserve exact reference API and expose execution/precision diagnostics in a scientific consumer rather than silently changing it. |
| 11 | `qualia-core-db/src/wgsl_forge/dispatch.rs:1110` `fft_f32`, `:1136` `fft_f32_gpu` | E+R: shared locked ForgeRuntime is initialized with 64 MiB capacity, actual runtime FFT attempted, failures silently return to CPU DFT. An eligible input or successful result alone does not prove GPU execution. Per-job admission, contention/cancellation and backend/fallback receipts require consumer integration. |
| 12 | `qualia-core-db/src/wgsl_forge/runtime.rs:710` `ForgeRuntime::fft` | E+R: validates shape/workgroup limits, allocates/writes device views, emits/compiles kernel, dispatches, reads back f32 and clears transient allocations on success. Allocating Vec/readback exists; it is not persistent GPU field residency or Q42/HMC selected-range acquisition. Error cleanup behavior needs separate qualification. |
| 13 | `qualia-core-db/src/wgsl_forge/emit/mod.rs:81` `emit_shader`; `runtime.rs:98` `compile_kernel_pipeline` | E+R: emitter selects WGSL/MSL/HLSL/PTX/CUDA-C/SPIR-V text/binary emission; runtime supports concrete WGSL/SPIR-V and feature-gated HLSL→DXC paths. Emission for a target is not execution on that target. Runtime falls to WGSL for other backend branches; macOS MSL branch calls an error-returning `compile_msl` (`execute/wgpu.rs:1116`) and does not locally catch that error. |
| 14 | `qualia-core-db/src/wgsl_forge/oracle/evaluate.rs:406` `certify_builtin`, `:95` `evaluate_affine` | E+R: actual oracle path emits/validates, allocates, dispatches, reads/computes comparison, rejects mismatches/zero timing and builds adapter/tolerance/source/schedule evidence. Certification is scoped to fixture vectors and selected shape; no source-commit provenance is plumbed. No manifest was generated or accepted in this audit. |
| 15 | `qualia-core-db/src/wgsl_forge/mod.rs:236` `generate_and_certify_via_etalon` | E: the gpu-runtime-gated bridge calls Etalon emission and `certify_kernel` for WGSL, returning generated shader. This wrapper has no dispatch/readback/oracle call; its “certify” name must not be confused with Core's executed `certify_builtin` pipeline. Etalon implementation itself was not inspected in this bounded pass. |
| 16 | `qualia-core-db/src/wgsl_forge/execute/wgpu.rs:774` `compile_pipeline`, `:900` `submit_graph`, `:1232` `QualiaCompute::dispatch` | E+R: creates real wgpu compute pipeline, records commands/ordered GPU handoff copies, submits queue and polls/maps timing. `submit_graph` waits indefinitely; no cancellation parameter. Source demonstrates an execution implementation, not a run on today's adapter, bounded native magnifier latency or species scientific validity. |

## Traced consumer paths

Existing Core invoke arms at `poet_host/invoke/mod.rs:220–221`, `:506`, `:515`
and `:688` route respectively to DFT, asymmetric hull, canopy and logistic growth.
Read wrapper bodies: `invoke/math/transforms.rs:35/46`,
`invoke/geometry/wave27_host.rs:112/463`, `invoke/science/physics.rs:363`.
Geometry/science/math reexports retain native or `wasm-scientific` guards.
These are executable Rust/Vibe host seams, not proof of native Poet grants/UI use.

The hull host casts slices/stacks and allocates output from unchecked capacity;
the canopy host casts floating triangle indices to u32, derives dense organ-array
size from max ID and allocates width×height buffers. The inspected wrappers do
not themselves provide cumulative checked input/work/memory admission. Reuse the
kernel through the existing native admission owner with stronger borrowed
preflight, not by copying the algorithm into Ecopunk. Preserve actual ID mappings:
a dense organ slot is a solver index, not the organism/organ's sole identifier.

`qualia-cli/src/shader.rs:415–469` separates dry-run generation/validation from
non-dry-run context construction→`certify_builtin`→optional manifest/cache writes.
The actual non-dry-run branch exists; neither branch was invoked here. Core
`ForgeRuntime::fft` reaches `WgpuPipeline::dispatch`; the existing Core DFT host
instead reaches the f64 CPU reference. Do not infer automatic GPU or certification
from a tool name, host binding, successful result or generated WGSL alone.

## What to reuse next, and what stays open

1. Tree authoring should reuse existing branch/organ work plus crown envelope and
   canopy kernels only for explicitly selected operations. Crown fitting is a
   representation/analysis tool; it cannot replace sourced metamer/leaf state,
   editable branch identity or complete species HMC knowledge. Bind declared
   units/frames, measurement sources and uncertainty to graph records and resources.
2. Wallaby work keeps the existing rig/contact/deformation plan. Generic ODE and
   flocking are useful kernels, but contain no validated macropod gait, tendon,
   physiological or ecological model by themselves. Calibrated domain equations
   and actual rig/source/review evidence remain separate tasks.
3. Scientific streaming reuses [Q42/HMC selection](graph-streaming-reuse.md):
   select graph-bound source/resources, acquire a bounded verified working set,
   then invoke the admitted kernel. These solver APIs accept slices/vectors;
   none of the inspected bodies itself queries an HMC dependency closure. Do not
   fetch the whole biosphere to populate an FFT or canopy input.
4. Owner-level wrappers need finite/integer/schema checks before allocation,
   explicit selected precision/backend/fallback, cumulative CPU/GPU/scratch and
   source×triangle×pixel or n² work accounting, cancellation checkpoints or short
   atomic calls, and candidate-state publication guarded by release/source/graph
   revision. A max_steps constant alone is not an end-to-end job budget.
5. Future qualification compares analytic fixtures and independent domain inputs,
   records actual device path/errors/precision/timing and native UI acceptance,
   then proves graph/organ/rights/source continuity through edit/save/reopen.
   Spectral/physical model validity requires declared domain calibration and
   reviewed tolerances; shader oracle success does not establish that validity.

The [prior compute review](compute-reuse.md) is reused for geometry workspace,
isosurface/remeshing/PBD and sim growth/leaf/contact/rig bodies; they were not
re-audited here. Existing Q42/HMC review remains intact. Geometry algorithms,
engineering/FEM/thermal/fluid libraries, symbolic math, other solvers, CUDA,
Etalon internals and other Forge kernel bodies are **not comprehensively audited**.
An inventory/search result is not an absence finding in any of those areas.

This review qualifies no whole foundation/readiness gate and no new runtime.
Tree/wallaby species packages, film/open-world-game/cartoon targets, applicable
catalogue claims, scientific calibration and native contributor/reviewer
acceptance remain incomplete.
