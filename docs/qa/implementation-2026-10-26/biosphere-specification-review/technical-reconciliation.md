# Biosphere specification: format, coordinate and hardware reconciliation

**Date:** 2026-10-26. **Status:** bounded source/document review, not an ABI change,
implementation, execution or acceptance. Original file is preserved. Line numbers
refer to SHA256 `883fe0ecf692d6a9b6d55870ef28c729df125fc550e5f506e20343176d5c5afc` (547 physical lines).
Exact source excerpts, discovery limits and external references are in the
[coverage record](technical-reconciliation.coverage.json).

## Valuable design considerations to retain

Keep complete graph-bearing HMC embodiments, authored and procedural sources,
multiscale representations, typed environmental/organism cues, selected-resource
residency, explicit hot/cold budgets and deterministic derivation. Canonical
portable bytes and hardware-specific runtime layouts are usefully distinct.
Configurable physical boundary conditions and optional frontier laboratories can
extend the ecosystem without making every research model a first-output gate.
The existing owners and compatibility contracts must determine their admission.

## Corrections before an agent treats this draft as a format contract

| ID / original lines | Source-backed finding | Required disposition |
|---|---|---|
| TECH-01 / 12, 244–269 | Opening eliminates M64 as redundant; §2.7 retains it. Existing HMC explicitly embeds intact files byte-for-byte, and M64 has its own checked-reader/producer contract. | Retain standalone M64 and existing consumers. Describe any decomposed representation as an optional versioned derivative with loss/overhead/consumer measurements, not silent deprecation or mandatory repackaging. |
| TECH-02 / 15, 125–145, 254, 515 | Actual `.10d` tags include 14=TextureCoordinates and 22=VoxelLattice. Draft assigns 14=VOX0 and 22=STR0. Tag5 remains SpecReservedTemporalIndex;18 is ProceduralSynthesis with `PROC`, not the proposed `MGEN` record. | Do not reuse occupied IDs or activate reserved tags by documentation. Any new profile needs a versioned tag/payload/compatibility proposal, known/unknown preservation and actual consumer cases. |
| TECH-03 / 15, 125–143 | Section13 uses MAT4 (464-byte material records, legacy readers MAT1–3), not the 16-KiB constituent LUT. Section17 EONT contains entity/affordance data, not the proposed complete developmental-keyframe store. Existing three-bank 256-entry LUT is source-present under M64. | Reference/reuse actual schemas; keep material optics, constituent recipes, clips, developmental state and affordances distinct. A shared 64-byte size or similar name does not establish byte compatibility. |
| TECH-04 / 125–141, 262 | Proposed Holo deformation bits, phase-space record and strata fields differ from current M64 encodings. Declaring the same layout/name does not migrate semantics, units or bit assignments. | Specify independent version/profile, units, valid ranges, lineage and round-trip/loss rules. Never reinterpret existing flags, material indices or stored bytes under an unversioned new meaning. |
| TECH-05 / 18, 26, 169–180, 255 | Inspected QBDL is version1, 64-byte header, 64-byte entry alignment and CBOR entry index with SHA256. `.10d` section descriptor is 24 bytes; actual alignment tiers encode 1/4/16/64 bytes. No proposed universal `TenDSectionHeader.substructure_encoding` appears in this selected descriptor. | Label v2/256-byte/16-KiB/encoding-flag designs as proposals. Do not relabel existing release bytes or reuse reserved fields. Exact legacy `CacheLine`/`Page` enum names are retained, without confusing them with actual OS page/cache-line sizes. |
| TECH-06 / 18, 184, 238–242, 268 | Current range verifier checks supplied whole-entry SHA256 or section CRC; these are not Bao branches. CRC detects corruption, not authenticity. An embedded hash prefix or actor key does not prove authority or physical truth. | Reuse actual range planning/read owners. Define trusted root/manifest, proof scope, chunk offsets/lengths, metadata and source authorization before verified partial streaming claims. Point/holo M64 records do not gain CRCs merely by requesting chunk reads. |
| TECH-07 / 18, 26, 241 | Variant resolver source scans a Vec, collects matching candidates and scans fallback entries. It is not a demonstrated O(1), allocation-free lookup. Quantum result caches were not established by this focused audit. | State current lookup complexity and selection dimensions; index/cook only if a measured case needs it. Cache-hit lookup cost excludes fetch/decode/verification and does not make an interpolated scientific result exact. |
| TECH-08 / 32–48 | Tensor10D uses f32 lanes. `mu` cannot hold every u64 provenance digest exactly; `sigma` is not an inline arbitrary spectrum. The second ten is explicitly inference-internal in selected source, not exposed to Vibe. | Preserve declared axis roles and typed referenced resources. Add physical spectrum/biomass/flux/chirality/pressure state with explicit units, scales and mappings. Do not reinterpret inference density/curvature/weight as physical tissue properties or conflate coordinate IDs with quantum state. |
| TECH-09 / 32, 54, 151–152, 227 | Compact coordinates and quantized records do not establish precision across 42 orders of magnitude. Zero allocation applies to a chosen operation/profile, not all named solvers or a whole renderer. One arena/default budget is not universal feasibility. | Declare frames/origins, units, precision/error, allocation tiers, per-operation budgets and actual admission outcomes. Keep source/simulation/runtime representations separate and disclose conversions. |
| TECH-10 / 160–197 | Alignment, AoSoA, local tiling and hot/cold separation are useful design hypotheses. Fixed 8× speed, single-cycle loads, zero-copy and 25% bandwidth savings are unmeasured here. 48 hot+8 cold bytes leave8 bytes of a64-byte record unexplained. | Retain existing wire layout; prototype target-specific cooked variants/transcoding only with exact field accounting, round trips, hashes/proofs and relevant workload/device measurements. No universal performance or lossless claim from a diagram. |
| TECH-11 / 201–242 | Detailed NPU/iGPU/QPU/platform routing claims exceed this inspected source/consumer evidence. Correct local paths include `etalon/crates/etalon-agent`, `etalon/etalon-hal`, `etalon/crates/etalon-wgsl-forge`. Absence of a searched token is not an engine-wide gap. | Reuse current Etalon capability handoff and inspect selected actual owner/backend/consumer before implementation. Record supported/enabled hardware separately, bounded CPU fallback and synchronized transfer. Do not commission a duplicate HAL, quantum dispatcher or cache from this draft alone. |
| TECH-12 / 16, 24, 492–499 | Bytecode/runtime totals and host counts are assertions requiring profile evidence. Current governing Vibe language remains source-evaluated, with 0.1 compatibility and proposed 0.2 distinct; optional bytecode is not mandatory per-edit compilation. | Keep source evaluation and existing host effects/sessions. Grammar-constrained model output is a proposal; normal validation/authority/budget/review applies. No provider/QPU invocation or source execution from viewing a package. |
| TECH-13 / 232–242 | Semantic stripping, randomized transforms, hashes, physical residuals and commons caching do not automatically prove privacy, reversible operator mappings, correctness, rights or universal reuse. | Treat as a proposed research-service contract. Qualify threat model, metadata leakage, permitted transforms/observables, solver/basis/conditions/tolerances, exact versus surrogate results, source rights and independent verification. No live external dispatch is enabled here. |

## Hardware/API corrections with primary references

The WebGPU CTS records default 256-byte buffer-offset alignments; the Vulkan
specification exposes implementation-dependent limits. Query the selected device
and distinguish a GPU buffer binding offset from a file's interior offset.
A GPU allocation can hold/transcode an entry independently of disk placement;
file alignment alone neither proves zero-copy nor forces a copy in every path.
[WebGPU CTS](https://gpuweb.github.io/cts/docs/tsdoc/functions/webgpu_capability_info.getDefaultLimits.html),
[Vulkan limits](https://docs.vulkan.org/spec/latest/chapters/limits.html).

Windows file-view offsets follow system allocation granularity, not simply the
hardware page size. A larger aligned mapped view can contain interior file data.
Use the actual platform APIs, not the draft's universal 4/16-KiB inference.
[Microsoft MapViewOfFile](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile).

NVIDIA's guide supports the particular strided-load overfetch example. It does
not establish an 8× application speedup, universal warp width, single-cycle
execution or identical Apple/Intel/AMD behaviour. Keep useful layout experiments
and measure total work, including conversion, padding, synchronization and cache.
[NVIDIA CUDA guide](https://docs.nvidia.com/cuda/cuda-programming-guide/pdf/cuda-programming-guide.pdf).

## Proposed replacement architecture paragraph

> HMC remains the complete embodiment package, retaining versioned Q42 knowledge,
> editable sources, typed spatial/animation/material resources and relevant M64
> physical data. Reuse existing readers, producers, graph/physics/geometry services
> and native Poet authoring. Additional morphogenesis, phase-space, strata or
> hardware layouts are candidate versioned profiles, admitted only through their
> actual supported codecs and consumers. Preserve old bytes and unknown data;
> qualify precision, units, provenance, capabilities, selected-resource access,
> recovery and output for each declared production or research case.

The current task does not adopt this paragraph as an upstream standard. The
coordinator can use it in the revision proposal without modifying existing
formats or treating all research goals as new prerequisites. Existing R-04/05/06,
PA-R01/11/12 and BS-30/40/50 are the relevant allocations. No source bytes,
builds/tests, software/runtime gates, human acceptance or species package claims
were changed by this review.
