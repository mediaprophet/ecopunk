# Independent source review: Round 3 georaster candidate

Reviewed on 2026-10-25 by the separately assigned frames worker. The frozen
`geospatial/target-manifest.json` SHA-256 is
`5623c5f23b9424a785634153a0f958f9a10deec0e2b67be4975f1bba5e9032d8`.
All 12 candidate files matched their manifest hashes during this review.
No frozen candidate, live sibling source, asset or test result was changed.
This is source review only: no build, test, WASM execution or benchmark ran.

## Result

No blocking correctness finding was identified for the explicit developmental
v3 writer/checked-reader/recovery scope. The implementation and proposed contract
agree on the requested scalar, metadata, padding, refusal and compatibility
properties. Integration and actual execution remain the supervisor's work.

## Evidence examined

Paths below are relative to the frozen `geospatial/after/` snapshot.

| Review concern | Source assessment |
|---|---|
| Absolute elevation; no double offset | `crates/qualia-geospatial/src/raster_m64/writer.rs:95` assigns the original i32 sample directly to `surface_elevation_mm`. `raster_m64/mod.rs:84` recovers the reader sample directly. Neither path adds/subtracts `origin_elevation_mm`. The origin is retained independently in the metadata/header summary. |
| Exact frame and sample recovery | `crates/qualia-core-db/src/q42/m64_matter/georaster/wire.rs` writes original f64 LE latitude/longitude, i32 origin elevation and f32 epoch bits and decodes them with bit-preserving reads. The high-level recovery reconstructs all six `GeodeticFrame` fields. `checked_reader/mod.rs::raster_sample` returns the original i32 and distinguishes declared nodata from out-of-source extent. The reviewed lossless test compares f64/epoch bits and all source samples. |
| Checked usize/WASM32 arithmetic | `georaster/mod.rs:89` checks u64-to-usize dimensions; shape checks chunk-axis limits, chunk/record products and complete file-byte arithmetic. File size includes every 64-byte record, which bounds all later row-major address arithmetic. Generic reader preflight checks u64 offsets/counts before later immutable-reader `as usize` conversions. No truncating unchecked conversion was found on an attacker-controlled value that bypasses preflight. This reasoning does not replace a real WASM check. |
| Cancellation and output atomicity | Writer validates all shape/source-length/limits/capacity/metadata, builds metadata/header on stack, and checks cancellation immediately before its first output write (`writer.rs:63`). Its subsequent batch is infallible under the admitted shape, with no late `Err` or cancellation return. Recovery checks complete source count/capacity/cancellation before its output loop; its reader is an immutable previously admitted view. Both implement the documented non-interruptible final batch policy. |
| Ancillary zero-data behavior | Writer uses `M64StrataColumnRecord::zeroed`, populates only elevation and CRC. `georaster/validation.rs:50` checks all bytes 4..60 are zero; record CRC occupies 60..64 and is checked by generic admission. It does not inherit the physical defaults in `strata_record.rs::Default`. |
| Padding claims actually checked | The writer emits nearest in-range source samples. `georaster/validation.rs:54` calculates the clamped source record independently and compares the padding elevation bytes after generic record CRC validation. RejectPartial is enforced by metadata shape validation, making padding impossible under that policy. The test corrupts padding, recomputes the record CRC, and expects a Padding refusal. |
| Counts, budgets and association | Metadata validates exact records/chunks/payload byte declarations. Writer budgets include padded records and directory/header bytes. Reader checks header/file length, caller limits, directory ranges/alignment/overlap/CRC, codec/mode and record CRC before profile policy validation. Profile validation binds SRID, row-major Hilbert index, fixed record count, exact contiguous offsets/lengths, empty Bao prefix, canonical magic and narrowed header summaries to GRM3 metadata. |
| Backward compatibility | `M64_VERSION` and default header remain version 2; legacy geotiff writer is unchanged. Header validation retains v1/v2 sizes/CRC behavior while explicitly admitting the separate v3 enlarged header. New profile validation runs only for version 3; `georaster()` returns None for v1/v2. The authored test preserves legacy v1 output, e64/v2 admission and rejects relabelled legacy bytes as v3. |

## Qualification limits and follow-up execution

The 13 authored synthetic-byte tests are appropriate focused evidence to execute
after guarded installation, together with the existing M64 reader/writer cases.
Their presence and this review are not test passes. Native little-endian execution,
WASM build/interop, realistic scientific datum/precision assessment and performance
remain separate qualification work. The profile retains declarations and integer
samples; it does not perform or prove a datum transformation, geodesic/projection
accuracy, source authority, provenance adoption or complete HMC embodiment.

The final write/copy batches deliberately do not interrupt after admission. This
is stated in both API documentation and the proposed contract, so it is not a
hidden cancellation guarantee. Cancellation arriving then can complete with
success. Source padding remains distinguishable by dimensions, not a second
validity mask. Sentinel semantics intentionally classify occurrences of the
declared sentinel as nodata. No stronger interpretation was inferred.

No product gate, FND-02/FND-04 gate, model/package completion, human acceptance
or native Poet readiness claim follows. Asset work remains on hold.
