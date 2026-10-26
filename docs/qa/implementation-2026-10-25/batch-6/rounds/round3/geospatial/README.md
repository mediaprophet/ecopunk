# Round 3 geospatial FND-02 staged handoff

This bounded candidate is **staged, not applied or runtime-qualified**. Asset
production is on hold. No assets/fixtures were exported or rewritten; no sibling
repository write, build/test, network, credentials, Computer Use, commit or
publication was performed by this worker. The integration supervisor owns
review, guarded installation and serial verification.

The approved [developmental contract](georaster-v3-contract.md) adds opt-in
M64 georaster v3 with a checked native GRM3 block. Core owns the fixed wire
contract and whole-container admission, and geospatial owns source conversion
and recovery. Existing version 1/2 reading, `e64` alias, `M64_VERSION=2`, default
header and legacy raster-writing bytes retain their prior contracts. V3 requires
canonical `m64` and an explicit 192-byte extended header; old readers reject it.

The inspected fixed v1/v2 header and directory have no safe extensible field for
the missing georaster data. The existing SpatialAssetManifest retains some frame
declarations but lacks exact origin, resolution, epoch, source dimensions and
nodata/output association. The `.10d` field frame contract belongs to that
container. This candidate therefore extends the native M64 format under a
numbered developmental contract rather than inventing a parallel JSON sidecar.

Saved targets and exact expected/candidate SHA-256 values are in
[target-manifest.json](target-manifest.json). Five existing files have full
`before/` and `after/` snapshots; seven new files have null expected hashes and
full `after/` snapshots. Every existing expected hash matched its live source
when frozen. Context snapshots preserve inspected frame, old writer, manifest,
M64 layout/directory/strata and overlap helper source; context hashes identify
those inputs separately.

The candidate retains original f64 geographic anchor, original integer base
elevation, epoch bits, source width/height, resolution, SRID/datum declarations,
explicit local-grid row direction, first-cell-center anchor/basis, edge policy
and optional nodata sentinel. Samples retain their original absolute mm values:
no base offset is applied. Ancillary strata fields are zero, without inventing
soil/physical properties. Emitted padding is checked against the nearest source
edge. The source accessor returns `None` outside source extent and distinguishes
nodata from an elevation of the same raw type.

Core checks metadata CRC/reserved fields, header-summary agreement, source/count
overflow, 16-bit Hilbert bounds, directory SRID/mode/codec, exact row-major
association, record CRCs, extent/budget constraints and edge padding. No datum
transform, reference authority, source provenance adoption, signature or human
acceptance follows. Native numerical placement/physical accuracy, arbitrary
nodata masks, compressed codecs, unknown v3 schema preservation, streaming,
WASM interoperability and performance benchmarks remain outside this candidate.

Writer and recovery use caller buffers and explicit admission bounds. All errors
and cancellation checks precede output mutation; the admitted final batch is
non-interruptible. A cancellation arriving after its final check may finish
successfully. Reader scratch may change on refusal, matching the existing reader
contract. Source borrowing and immutable reader views prevent source edits during
the admitted operation.

Verification authored: 13 integration tests at
`crates/qualia-geospatial/tests/m64_georaster_v3.rs`, covering lossless recovery,
nodata/padding distinction, determinism/unaligned reads, retained legacy behavior,
metadata/header/CRC/reserved/count/policy corruption, directory association,
padding/physical-field refusals, limits/cancellation/failure-before-write and
thread-local allocation measurement over preallocated writer/reader/recovery
buffers. The zero-allocation case is unexecuted and makes no measured claim.

Actual worker verification: `rustfmt --edition 2021 --config skip_children=true`
on all 11 staged Rust files completed with exit 0. This checks parser/formatting
acceptance only, not name resolution, compilation or runtime behavior. No tests
or builds were run. Proposed supervisor check after guarded installation:
`cargo test -p qualia-geospatial --test m64_georaster_v3 --offline`, followed by
the previously established M64 SDK/consumer regression targets. The supervisor
must pin actual command source/features and record command results separately.

Independent review of the frozen 12-target candidate reported no blocking
correctness finding: [frames review](../frames/review-geospatial.md), SHA-256
`1e02b67013a48ceeab4298bb45fa403f6441bc00ba46e8307c1958612fd8de3b`.
The reviewed target-manifest SHA-256 is
`5623c5f23b9424a785634153a0f958f9a10deec0e2b67be4975f1bba5e9032d8`.
Read-only compatibility search checked the existing canonical
`tests/m64_container_sdk.rs` and `q42/m64_matter/tests.rs`: the unsupported-version
fixture uses `0x1234_0001`, not version 3; no invalid-version-3 expectation was
found. The retained header-size-128 refusal remains valid. No existing test
source needs a version migration for this candidate. The 13 new tests already
cover relabelled v3 bytes with an invalid legacy-size header and malformed GRM3
metadata. No post-review candidate source edits were made.

Remaining gates: FND-01 identifier/interop qualification; FND-02 full formats and
geospatial scientific semantics; FND-04 measured precision/budget workloads;
FND-05 actual native Poet contributor/reviewer work; FND-06 WASM/interop and
benchmarks. IP-03 C1–C8, all production packets and high-grade/model acceptance
remain open. This is a format/SDK candidate, not an embodiment completion claim.
