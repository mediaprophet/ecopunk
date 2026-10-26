# Existing QualiaDB functionality: audit coverage and revised allocation

2026-10-26. **A targeted source audit has now covered each of Timothy's nine
listed locations. It is not a comprehensive audit of every file, algorithm,
feature combination, correctness property or deployed capability.** The earlier
work inspected selected production paths and should not be described as an
exhaustive QualiaDB audit. This record makes the actual coverage reviewable.

The [inventory](requested-source-inventory.json) records 1,314 discovered
Rust/WGSL/manifest files and their hashes. Discovery/hashing is not semantic
inspection. The [coverage ledger](requested-source-coverage.json) consolidates
exact read ranges, symbols, evidence levels and current hashes from the three
new source reviews. It lists 101 unique files including consumers outside the
nine roots; 65 include implementation/consumer body excerpts, **51 within the
nine requested roots**. Excerpts are not a claim to have reviewed the full file.
API/types/call-site-only records and unreviewed files remain explicit.

| Requested location | Discovered files | Files with body excerpts | API/types/call-site only | No semantic review recorded |
|---|---:|---:|---:|---:|
| Core `domains` | 49 | 3 | 4 | 42 |
| Core `modalities` | 94 | 7 | 1 | 86 |
| Core `solvers` | 190 | 3 | 2 | 185 |
| Core `specialized_libs` | 485 | 10 | 4 | 471 |
| Core `wgsl_forge` | 73 | 6 | 3 | 64 |
| `qualia-vision/src` | 110 | 3 | 6 | 101 |
| `qualia-audio` | 254 | 3 | 2 | 249 |
| `webizen-render` | 33 | 4 | 1 | 28 |
| Core `semantic_instruments` | 26 | 12 | 4 | 10 |

This table counts the explicit new coverage manifests, not every source file
read in earlier work. The earlier compute/spectral/Q42 reports remain separate
evidence and are not inflated into new body-read or test counts.

## Actual findings, including limits that change the plan

| Review | Concrete implemented functionality inspected | Important limit for the next work |
|---|---|---|
| [Domains, modalities, instruments](domain-logic-instruments.md) / [ranges and hashes](domain-logic-instruments.coverage.json) | Affine-gap sequence alignment and host consumer; environmental raster sampling/compilation; thermodynamic sampling; botanical/hydrological state encoding/decoding; graph analysis; epistemic/guard/contract mechanisms; instrument package/resolve/receipt/attestation paths. | State decoders are not simulators. Blackboard/DAG writes and hard-constraint handling are not fail-closed promotion. Instrument execution is explicitly a labelled identity-transform demo; empty catalogue locks do not prove transitive dependency resolution. Signatures/labels do not prove truth, adopted terms or human acceptance. |
| [Solvers, kernels, Forge](solver-kernel-forge.md) / [ranges and hashes](solver-kernel-forge.coverage.json) | Adaptive DOPRI5 and logistic wrapper; flocking; asymmetric crown hulls; actual organ-ID canopy radiation; f64 DFT and opt-in f32 GPU route; real kernel emission, GPU dispatch/readback and oracle certification source. | Population growth is not organism development. In-place flocking is order-dependent. Crown envelopes do not supply tree anatomy. Runtime fallback and static certification differ from measured GPU execution. Per-call kernel parameters do not ensure cumulative job budgets/cancellation. |
| [Vision, audio, renderer](vision-audio-render.md) / [read scope](vision-audio-render.coverage.json) | Histology stain decomposition, spectral NNLS, segmentation, volume projection; public image/mesh handoffs; audio DSP/capture/spatial paths; actual native render-provider and HMC texture residency plan/apply source. | Image-to-3D is explicitly a luminance heightfield reference; some views/pipelines are illustrative or synthetic. Spatial/calibration/species validity and actual native Poet integration remain separate. GPU texture residency does not establish selective HMC network acquisition or bounded whole-package CPU residency. |

All conclusions refer to inspected working source. They are neither clean Git
revision claims nor fresh execution evidence. The supervisor checked selected
critical bodies, source/hash consistency, ownership and the merged allocation;
the three workers' reports remain inspectable rather than being treated as
unreviewable completion declarations. Unselected domain, solver, modality,
vision/audio and instrument functionality remains to review when relevant.

## Additional cross-crate reuse already identified

- [Compute reuse](compute-reuse.md) follows actual `qualia-sim` metamer growth,
  leaf hydraulics, botanical continuum coupling, contact/XPBD/ragdoll/event APIs,
  Core geometry, skeletal upload and N3/LTL. Adapters preserve sim-to-Core
  dependency direction and need explicit scientific parameters and state.
- [Q42 and streaming](graph-streaming-reuse.md) follows real native range readers,
  BIDX/field-index query pages, HMC planners, MID2 typed payloads, rights/job and
  residency owners. It identifies missing selected closure/lazy query work,
  integrity-granularity and CPU lifetime qualification.
- [EMF/cues](emf-source-inventory.md) records actual 380–780 nm spectra, Gaussian
  reconstruction, field visualizers, DSP and thermal/material quantities. It
  distinguishes physical fields, sonification and species response evidence.

The preserved [botanical implementation evidence](../biological-reference/execution.md)
contains ten native host tests, one GPU/editor test and the native build. Those
are prior narrow results, not tests of the functions audited here. Earlier
format/native authoring evidence is linked by the source reviews; no counts are
summed into a whole-engine pass.

## Prepared next work, and what ran this turn

The [integration contract](../../../planning/tech/biosphere-graph-spectra-and-streaming.md)
and [dispatch](../../../planning/biosphere-implementation-dispatch.md) require
reuse and explicit qualification before missing-capability/replacement work is
allocated. Domain adapters must preserve existing botany/hydrology quins,
physical units, graph relations, input/output limits and accepted-state promotion.
Do not use the current demo instrument runner or post-write constraint counter
as proof of production execution/enforcement. Source rights/contribution records
start now; payment rails remain later.

Prepared jobs have disjoint staging directories and persistent handoffs. One
coordinator owns shared manifests/module registration and serial Cargo/native/GPU
checks. [The lease check](coordination-check-reviewed.json) exercised a second
**separate process**, which was refused while the lease was held, followed by
release/reacquisition. This is a coordination smoke test only; it invoked no
Cargo or engine and controls cooperating future callers only. Historical scripts
do not automatically use the guard.

**No new engine build, GPU run, scientific test, species model, semantic instrument
execution, adoption or payment was performed for this review.** Engine source
was not changed. Both requested species and all film/game/cartoon packages remain
incomplete; foundation, scientific/educational and actual human workflow gates
remain open. No statement here labels all remaining 1,236 inventory files as
missing, incorrect or production ready.
