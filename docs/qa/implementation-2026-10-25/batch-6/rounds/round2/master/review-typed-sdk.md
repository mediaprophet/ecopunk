# Independent source review: frozen typed `.10d` SDK candidate

2026-10-25. Reviewer: round2_master, independent of the typed-SDK implementation
worker. Result: **no blocking source findings within the declared foundation
scope**. The candidate is ready for explicit installation approval and serial
build/test qualification. It is not installed, compiled, runtime-qualified or
accepted as production content by this review.

Freeze binding: `surfaces/sdk-save-manifest.json` SHA256
`7886467cd423a0645530fd2b0577a56b87c855845adf0ab22d2c517d161c0b58`;
`surfaces/foundation-sdk.patch` SHA256
`ee3316d40a42b0beeea7180edc8317f94428e4e213226dc9cad25383676df90b`.
All eleven `after/` source files were independently hashed and matched the
manifest's exact `after_sha256` values before this review was written. The review
read the SDK README, saved contract, staged public modules/call sites/tests,
canonical section-table child, and actual shared codec/container owners.

The public route is verified in the candidate source, rather than inferred from
private file presence: `container_10d/mod.rs:76` declares `typed_payloads` and
`:197` reexports `TypedPayloadResolver`, `TypedPayload`, `TypedPayloadError`,
`read_container_mesh_frames` and `read_container_morph_targets_into`.
`typed_payloads/mod.rs:4` also exports both readers. In these references the
candidate prefix is
`C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-6/rounds/round2/surfaces/after/src/`.

`container_10d/typed_payloads/resolver.rs:47` delegates actual envelope/manifest
admission to the canonical owners. The existing
`C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/manifold_identity_container.rs:169`
checks every declared section-relative offset/length before copying scratch into
caller output at line 170. Fields retain manifest identifiers and units.
`resolver.rs:87` selects exact bytes by field identifier; `:96` and `:102` select
unique registered frame/morph kinds, with ambiguity and FieldSidecar/unit binding
errors. Generic vector/tensor/scientific records are not guessed to be frames or
morphology. The fixed convenience workspace retains the established 256-field
manifest ceiling, and the generic resolver exposes caller-owned record buffers.

`typed_payloads/readers.rs:12` checks the actual shared 16-byte NRM1/FRM1 header
and declared vertex count before invoking the existing bounded allocating decoder.
The second count check binds decoded streams to the expected mesh count.
`:66` selects explicit morph ranges, checks mesh vertex count and delegates MRP1
validation. The staged `morph_section.rs:330` validates all delta indices before
the final copy loop publishes any output. This addresses malformed later records
without changing MRP1 bytes or introducing a competing morphology format.
Morph mixed-unit semantics are stated explicitly; scientific units remain distinct.

Legacy no-MID2 whole-sidecar NRM1/FRM1 and recognized MRP1 selection is retained.
MID2 selects only declared registered kinds and exact ranges. The compile helper
at `render/compile_10d.rs:637` retains compatibility error projection while
delegating to the public reader; the portal at `render/portal/mod.rs:1335` invokes
the same public reader after obtaining the mesh count. Registry additions keep
MID2's layout/version unchanged and explicitly disclose older-reader admission
requirements. No existing default writer is changed by this candidate.

The canonical `container_10d/section.rs:685` now uses fallible aligned table
borrowing rather than panic-prone casting. Its existing checked range/bounds
validation precedes `section/table_validation.rs:8` metadata exclusion. The
padding helper traverses occupied header/table/payload regions without slicing
backwards across a later table. Nonempty payloads cannot overlap metadata;
empty-section shared offsets and valid payloads before a later table remain
supported. The resolver contains no private replacement envelope parser.

The nine public integration tests are source-linked to public exports. They cover
actual frame/morph/scientific values in alternate payload orders; legacy byte
preservation and no guessing; ambiguity/unsupported kind/codec errors; expected
counts/bindings/small output; malformed later MRP1 records; invalid ranges and
transactional outputs; malformed metadata overlap and unaligned input; a later
table; and legacy MRP1. This review did not execute them or measure allocations.

Unchanged shared source pins used to assess the real delegated contracts:

| Source under `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/` | SHA256 |
| --- | --- |
| `manifold_identity_container.rs` | `764e4370d2e6ed1d125a4a9cdbfd20185e54d203e4880b69d612ad5e3b835b2b` |
| `normal_section.rs` | `996ca418ba5693e6651bcdef1a9d914be353615f9ebbef964faae9392847db3d` |
| `header.rs` | `465fb36fd7278f203b14341531e8f0b312c1bc2f500cdfcddb3f8e6b0bdae2b9` |

Qualification remains explicit: normal stream decoding retains its existing
allocating API and codec cap, without a new caller byte budget. The section
reader's existing bounded pairwise scan and the oversized compiler/portal
decomposition debt remain disclosed. Whole-file sealing is the enclosing
admission owner's policy, not silently strengthened here. Native/WASM builds,
actual zero-allocation measurements, authored cooperative UX, complete HMC and
model packages, science/physics and human acceptance remain unqualified.

No source was edited and no Cargo, rustc, product runtime or live sibling write
was performed for this review. The existing automatic-approval rejection still
applies; this source review does not replace user approval for the concrete
cross-repository installation.
