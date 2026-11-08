# Format and SDK foundation qualification

**Status:** contract and work gates are open. This contract selects foundation work for the 2026-10-25 implementation priority; it does not claim format or SDK completion, native Poet readiness, model acceptance, or delivery readiness. It supplements the governing phase order in [the implementation plan](../implementation-plan.md), including IP-03 C1–C8.

## Format relationships established from source

**Reconciled 2026-10-26:** the table includes later saved checked-reader and
developmental GRM3 work; the evidence sections below retain its execution scope.
Initial findings are not instructions to rebuild an already implemented reader,
resolver, producer or HMC retention path. Open whole gates retain their remaining
domain, consumer, fidelity and human-acceptance requirements.

These names identify distinct contracts and must not be conflated:

| Name | Source-backed relationship | Qualification boundary |
|---|---|---|
| `.m64` / `M64` | Matter-64 geospatial/material container. Core retains `m64\0` and default version 2; current header validation accepts 1/2 and explicit developmental georaster 3, with its larger header. | A canonical bounded `checked_reader` is already implemented and has scoped execution evidence below. Reuse it; do not substitute header validation for complete structure checks or treat structural validity as authentication/domain accuracy. |
| `.e64` / `E64` | `e64\0` is explicitly a backward-compatible magic alias for early `.e64` prototypes; `E64Header` aliases `M64Header`. | It is not a second unrelated format family. Preserve the exact legacy magic when reading; canonical new output uses `m64\0` unless the accepted compatibility contract says otherwise. |
| Geospatial raster-to-M64 output | The legacy `compile_raster_to_m64[_with_codec]` path retains version 1; the separately opt-in caller-buffered `compile_raster_to_m64_v3` writes developmental GRM3 metadata and raw elevation-only strata. | Preserve legacy compatibility and declared profile. Reuse the existing native producer/editing path and qualify domain/CRS/consumer cases; do not fabricate soil horizons or claim every listed codec. |
| `.10d` | QualiaDB's versioned outer spatial container has typed section descriptors and payloads. It is not a generic synonym for M64, a mesh, or a JSON export. | Keep the established v1 reader behavior and use v2 sections only under explicit compatibility rules. Reject malformed inputs safely and preserve unsupported data where the selected operation permits. |
| `MID2` | A v2 `.10d` manifold-identity manifest section, with stable entity identifier (`StableEntityId`), separate profile/domain/coordinate-convention identifiers, ordered Tensor10D coordinates, and typed field references. Optional external digest/index hint is not the entity identifier. | Field kinds and payloads are typed references, not standalone formats. Keep the declared identifiers, units, referents and scopes distinct. |
| `FIEL`, `NRM1`/`FRM1`, `MRP1` | Existing `.10d` source has FIEL v0 descriptor and raster codecs, including public `encode_field_section_with_rasters` and `decode_field_raster_samples` APIs; normal/frame streams have NRM1/FRM1 payloads; morph targets use MRP1. The current MorphTargets logical type aliases `FieldSidecar`. | The older FIEL Rustdoc statement that no samples are encoded is stale relative to those source APIs and is not capability proof. Qualify codec behavior, round trips, and coexistence through the outer section-table API without dropping or confusing scientific fields, normals, tangents, morphs or unknown payloads. |
| HMC and Q42 | A complete embodiment is the identified HMC release with the available graph corpus and associated resources; Q42 is the native graph/knowledge layer. See [REQ-NAT-004](../architecture.md) and the species/embodiment contracts. | A mesh, labels, JSON Studio export or list of section names is not a complete HMC/Q42 embodiment or binary-format qualification. Preserve knowledge, resource identifiers, provenance and revision lineage through supported edits. |

Source references for these observations: `C:/github/qualiaDB/crates/qualia-core-db/src/q42/m64_matter/layout.rs`, `chunk_directory.rs` and `mod.rs`; `C:/github/qualiaDB/crates/qualia-geospatial/src/geotiff.rs`; `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/{section.rs,field_section.rs,normal_section.rs,morph_section.rs,manifold_identity_v2.rs}`; and Ecopunk's [architecture contract](../architecture.md). Ecopunk also retains `assets/terrain/talbingo_catchment.m64`, listed in `assets/terrain/compilation_receipt.json`, as a read-only geospatial artifact for inspection; do not rewrite it in place. Round 2 source capture is assigned to [format-discovery.json](../../qa/implementation-2026-10-25/batch-6/rounds/round2/master/format-discovery.json) and [field-sidecar-coexistence.md](../../qa/implementation-2026-10-25/batch-6/rounds/round2/surfaces/field-sidecar-coexistence.md); those evidence records should state their inspected revisions and exact observations.

## Open qualification gates

All gates below are **open**. Individual source or execution facts can be recorded as observed evidence without passing the gate.

| Gate | Required contract and evidence |
|---|---|
| **FND-01 — format contract and identifier qualification** | Pin format family, revisions, magic/version behavior, endian/layout, compatible and unsupported versions, identifiers/units/datum semantics, and error/preservation rules. Establish source-to-output lineage and interoperability cases. |
| **FND-02 — checked M64 and truthful geospatial codecs** | Provide a canonical bounded reader for accepted v1/v2 M64 and the legacy E64 magic alias; check all header, directory, record, extent, CRC and budget constraints before exposing content. Make raster writer version, codec, datums, precision, nodata and edge behavior truthful and explicit. Preserve unsupported/unknown data or fail without corrupting the prior admitted object. |
| **FND-03 — coexistent typed `.10d` payloads** | Resolve and validate MID2 plus normal/frame, morph and scientific field payloads in one `.10d` without collisions, silent overwrite or reinterpretation. Cover legacy v1 and v2 behavior, deterministic output, malformed/unknown sidecars, fresh reopen, and round-trip preservation. This is a typed-section/resolver contract, not authorization to invent a parallel JSON format. |
| **FND-04 — precision, budgets and unknown preservation** | Declare coordinate/datum/unit conventions, quantization and lossy boundaries, caller/resource limits, overflow and cancellation behavior. Measure representative workload and memory/latency only with recorded commands, data and results. Keep unknown fields and provenance visible and safe. |
| **FND-05 — native Poet operator and contributor workflow** | Through Poet's usable native controls, author/edit identified source, use the required M64 and `.10d` operations, save/reopen HMC, inspect errors/results, and show distinct authorized contributor/reviewer contexts with source-to-output lineage. This is necessary for IP-03 C1–C8 and cannot be substituted by a CLI, isolated render, JSON export or placeholder UI. |
| **FND-06 — native/WASM interoperability and benchmarks** | State native versus WASM bindings and host services separately; test declared format interoperability and supported round trips on each claimed target. Add reproducible performance/size/precision benchmarks with workload and resource budgets. A desktop feature or browser binding does not qualify native no-JS/V8 execution; no universal-best or superiority claim is made without relevant tests. |

## Ownership and product boundary

### Current Round 8 evidence, 2026-10-26

The [saved safety and continuity implementation](../../qa/implementation-2026-10-26/round8/progress.md)
advances selected FND-04/05 preservation, failure/recovery and native authoring
subcases. Exact original metadata-bearing HMCs are retained without rebinding
opaque source metadata to changed JSON; native compile/reopen/SaveAs and
fresh-process source activation have public consumer evidence. These bounded
results do not pass a whole gate. All six gates remain open and asset production
remains on hold; the linked dispatch identifies interactive native production,
full graph/rights continuity and actual operator acceptance as remaining work.

### Subsequent Round 3 evidence, 2026-10-25

The installed [Round 3 changes](../../qa/implementation-2026-10-25/batch-6/rounds/round3/progress.md)
advance specific subcases without passing the six larger gates. The native
conformance run passed **56 cases, zero failed/ignored**, with no drift in its
recorded local source set. It retains the prior 22-case scope and adds 34 cases.
The direct Core/geospatial/Portal SDK consumer compiled for
`wasm32-unknown-unknown`; the separate Poet frontend WASM check also passed.
Neither compilation is a runtime interoperability result.

- **FND-02:** explicit opt-in developmental M64 georaster version 3 retains f64
  origin, integer elevation, spacing, source extent, epoch/datum declarations,
  nodata and edge policy. Checked metadata/directory/padding and source recovery
  cases pass, including in-memory HMC reopen. Legacy defaults/writer bytes remain
  unchanged. Profile adoption, native producer use and domain/CRS qualification
  remain separate from preserved declarations and structural validity.
- **FND-03/04:** canonical caller-buffered frame decoding and bounded cold wrappers
  pass selected unaligned, capacity, late-invalid-data and allocation-counter
  cases. Native scene and Portal consumers select explicit frame limits.
  Whole-scene budgets, fidelity/error reporting and workload benchmarks remain open.
- **FND-04:** explicit structural opaque inspection/exact copy passes retention,
  CRC and refusal cases. Changed-byte replacement refuses unresolved dependencies,
  flags, metadata or opaque tail. This does not provide required/optional
  extension negotiation or arbitrary safe semantic rewriting of unknown data.
- **FND-05:** the separately reviewed Poet-only selected-resource inspector is
  installed through the existing native authorized owner and HMC controls.
  Current native test/build outcomes are recorded in the linked Round 3 report;
  no GUI verification or contributor/reviewer C1–C8 acceptance is implied.

The canonical developmental profile is documented in
`C:/github/qualiaDB/docs/manuals/standards/m64-georaster-v3.md`; frame limits and
opaque retention have separate standards companions. These are developmental
contracts with scoped execution evidence, not a released universal-best format.
All six gates remain open and **asset production remains on hold**.

### Subsequent Round 4 evidence, 2026-10-25

The saved [native producer/edit and package-review work](../../qa/implementation-2026-10-25/batch-6/rounds/round4/progress.md)
advances FND-02/04/05 with 36 distinct selected native cases: 19 package-policy/
authority, nine producer/edit/preservation/reopen, one actual author/reviewer HMC
workflow and seven existing source-workflow/inspection regressions. The initial
public fixture failure and strict test-only repair remain in the evidence.

Native controls use canonical GRM3 codecs and the existing authorized HMC owner.
Unverified source declarations persist with origin digest; opaque/observational
selected metadata refuses unsupported rewriting. Exact saved-package grants,
contributions, requests, decisions and known contributor exclusions are durable;
same-source resource successors invalidate previous review views. Retained
destination history does not establish full resource/SaveAs/rights derivation.
This is software-actor workflow evidence, not usable human cooperation or model
acceptance. Required model operators, graph/identifier mappings, adopted format/
extension contracts, fidelity/budgets, native usability and WASM runtime/benchmark
evidence remain open. **All FND-01–06 gates remain open; assets stay on hold.**

### Subsequent Round 5 evidence, 2026-10-25

The [native spatial authoring pass](../../qa/implementation-2026-10-25/batch-6/rounds/round5/progress.md)
adds canonical checked-FIEL inspection, MAT4 layout-preserving visual derivatives
and native geometry/assembly/material source controls. Selected native cases
cover all MID2 binding declarations before optional-sidecar handling, malformed
science, exact unaffected byte preservation, unsigned material identifiers,
bounded 256-part edits and native source-to-HMC/review/reopen. Actual results,
corrected failures and source pins are in that report; no gate is passed here.

Direct mutation of a graph-bearing `.10d` package refuses unsupported graph
rewriting. Authored production uses source edits and coherent `.10d`/Q42 compiler
output; changed existing targets require a new explicit asset key to retain old
sections and graph facts. Raw-source CRC witnesses now include every source byte;
these checksums are distinct from native authored-state SHA256 and identifier
authority. Complete part/graph derivation and safe semantic same-key merge,
required production operators, fidelity/whole-workload budgets, native usability
and runtime native/WASM interoperability remain open. **Assets remain on hold.**

### Subsequent Round 6 evidence, 2026-10-25

The [profile/CAD and surface authoring record](../../qa/implementation-2026-10-25/batch-6/rounds/round6/progress.md)
tracks original Poet native intent-based profiles/revolution, part-local planar
UV/projection controls, canonical visual MAT4/map authoring, bounded immutable
HMC texture binding and actual native GPU map/UV readback. FND-03/04/05 subcases
advance; exact saved-source, failure, repair, test and build status is recorded
there. A real textured checkpoint composition failure required a canonical Core
texture-union repair preserving retained opaque metadata and refusing conflicts.
Source-only unknown resources remain inert and retained. These are bounded
implementation facts, not a complete HMC embodiment or accepted native workflow.
General unwrap/atlas/detail editing, required CAD/organic/rig operations, full
graph/rights derivation, version/extension adoption, whole-workload measurements,
native usability and runtime WASM interoperability remain open. **All six gates
remain open; assets stay on hold.**

### Subsequent Round 7 evidence, 2026-10-25–26

The [native UV/texture/detail record](../../qa/implementation-2026-10-25/batch-6/rounds/round7/progress.md)
advances FND-03/04/05 subcases with canonical explicit-seam disk-chart atlas
construction, native source editing, actual UV01 serialization comparison,
layered paint/procedural maps and fresh HMC/review/reopen preserving original
image bytes and opaque metadata. It fixes canonical normal alpha compositing and
refuses incompatible UV proposals before source evaluation. Actual GPU pixels
change through the native atlas/material/map path; these are calibration checks,
not exceptional content acceptance. The linked report retains exact failures,
repairs, tests, builds and final-source verification status.

No container wire revision or released Vibe dialect is introduced. Source-entry
opaque metadata refuses snapshot-only compilation until source-carrier continuity
is qualified. General/automatic seams, optimal packing, edge dilation/mip baking,
full texture/rig/organic/CAD operations, whole-workload measurements, complete
graph/rights derivation, usable human cooperation and runtime native/WASM
interoperability remain open. **All FND-01–06 and IP-03 C1–C8 remain open; assets
stay on hold.**

QualiaDB Core owns canonical M64 and `.10d` format contracts/codecs; the geospatial crate owns raster conversion at its public boundary; Poet owns the unified authoring and operator experience; HMC/Q42 services own complete embodiment knowledge/resources and graph continuity. Reuse existing APIs where their contract meets these cases. Assign only demonstrated gaps to the owning dependency; do not claim all SDK capabilities are missing or imply all 120 production packets are complete.

Poet remains the product interface and its original UX baseline is preserved. `poet-geometry` and `poet-spatial-authoring` are internal libraries refactored into Poet, not a separate Craft product or product brand. Historical Craft workspace applications are migration/reference material, not the product target. Payment rails remain later work.

## Round 2 execution and completion boundary

Round 2 first qualifies the M64 reader/writer and the `.10d` MID2 typed-sidecar resolver. The 199-part growing-station artifact remains a regression fixture only; do not resume reference asset polish or production claims until the applicable foundation gate is qualified. Capture exact source revisions, commands, tests, artifact hashes and human review status separately. Re-estimate later round/pass counts after these findings; do not invent a fixed estimate. No broad readiness claim follows from any single gate or source-presence check.

**Current execution status:** an earlier agent request to install six M64 files in sibling QualiaDB was rejected by automatic review; retain that as historical review state. Later, a hash-pinned 17-file Core/geospatial install (six M64 reader/writer files plus eleven `.10d` typed-SDK files) to the exact reviewed targets was approved under the trusted earlier direct user QualiaDB save authorization and current SDK request, then applied. The [application receipt](../../qa/implementation-2026-10-25/batch-6/rounds/round2/applied-source.json) is the historical tested source snapshot; its installer source snapshot SHA-256 is `3931f35738e1ac71eb4c02690498a98c755b0ca3fa35530bf911221afeaec4bc`. After the run, root applied documentation-only corrections to `container_10d/mod.rs` and `field_section.rs`. The first current file hash therefore differs from its receipt hash, and the second is an additional post-test source-comment correction; the [correction manifest](../../qa/implementation-2026-10-25/batch-6/rounds/round2/evidence/doc-corrections/manifest.json) records the current hashes and unchanged implementation-body hashes. The receipt remains intact as the tested snapshot. This history does not erase the earlier denial, and there is no current permission block. Both SDK families are applied and implementation source is frozen. The native runner completed on Win64 little-endian with exit 0: 6 existing-consumer tests, 7 M64 SDK tests and 9 typed-field SDK tests passed (22 total; 0 failed/ignored). Build time was 19m59s; test binaries reported 2.11s, 0.13s and 0.01s. See [the recorded log](../../qa/implementation-2026-10-25/batch-6/rounds/round2/native-conformance.log), [Round 2 status](../../qa/implementation-2026-10-25/batch-6/rounds/round2/status-final.json) and [evidence index](../../qa/implementation-2026-10-25/batch-6/rounds/round2/evidence-index.json). Final canonical M64 and `.10d` standards documentation are saved in sibling QualiaDB and hash-bound to local copies by the [spec-final manifest](../../qa/implementation-2026-10-25/batch-6/rounds/round2/evidence/spec-final-manifest.json). The M64 allocation-counter case recorded zero allocations for its success fixture; this is not a no-heap claim for all SDK calls. The run is scoped selected conformance, not a passed foundation gate: identifier authority/mappings, full geospatial datum/resolution/nodata semantics, precision/budget coverage, Poet contributor C1–C8 and WASM/interop/benchmarks remain open. Ecopunk assets are unchanged and no human acceptance is claimed.
