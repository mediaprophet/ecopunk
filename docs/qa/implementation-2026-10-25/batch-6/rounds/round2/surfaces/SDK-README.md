# Staged `.10d` foundation SDK patch

2026-10-25. Eleven candidate Core Rust files are prepared under `after/`; six
existing-file preimages remain under `before/`. No candidate is installed in
QualiaDB. Root reports automatic approval review rejected the companion M64
cross-repository installation; this worker made no live Core write attempt.
Root owns independent review, specific installation approval, and one frozen
Cargo run. No Cargo, rustc, product runtime, or human review was run here.

`sdk-save-manifest.json` pins every destination, exact original/absent state,
candidate SHA256, and line count. `foundation-sdk.patch` is the review diff.
`static-checks.json` pins the candidate hashes parsed successfully by rustfmt;
this is syntax/format evidence, not Rust typechecking or test execution.
`foundation-sdk-contract.md` records the contract saved before implementation.

The patch registers MID2 SurfaceFrames `0x0200` and MorphTargets `0x0201`,
reusing NRM1/FRM1 and MRP1 codec bytes in exact typed FieldSidecar ranges.
Generic vectors remain distinct. Older MID2 readers reject unsupported kinds
unless explicitly updated or admitting those registered kinds. No outer tag,
header/codec version, persistence format, writer default, or old payload changes.
Legacy one-payload frames remain readable when MID2 is absent; MID2 always uses
declared ranges and never guesses a normal stream from generic vector data.
Recognized whole MRP1 sidecars also retain the legacy no-MID2 path. Morph records
have mixed units: position deltas inherit mesh coordinates, normal/tangent deltas
are unitless, and the enclosing declaration uses unit_id=0. Scientific field units
remain independent and unchanged.

`container_10d::TypedPayloadResolver` borrows payload bytes and caller-buffered
records, validates manifest kinds/ranges before publishing records, selects by
field identifier, and rejects ambiguous surface/morph kinds. Its success path
allocates no heap. The normal convenience reader checks declared vertex count
before calling the existing allocating stream decoder, whose codec count cap
still applies; it does not expose a new caller byte budget. Fixed selection
workspace is two arrays of at most 256 records. The morph reader retains
caller-buffered output; shared MRP1 validation completes before any output copy.

The legacy compile helper and WASM portal delegate to the same SDK. New SDK
behavior lives in focused child modules. `compile_10d.rs` (previously 1,241
lines) and `portal/mod.rs` (2,989) remain decomposition debt; this patch removes
their duplicated frame-selection lifecycle and adds only compatibility glue.
The touched 514-line MID2 codec receives only registry and identifier rustdoc
changes. Wider decomposition and identifiers API migration remain open.
Canonical section.rs (previously 1,213 lines) delegates table borrowing and padding
validation to a focused child. Unaligned views return errors; nonempty sections
cannot overlap header/table bytes. Valid payloads before a later table and empty
section offsets remain supported. The resolver has no private envelope parser.

Nine staged integration tests cover actual frames/morph/scientific raster
coexistence in alternate byte orders, exact independent values, the high-level
frame consumer, legacy byte preservation, no vector guessing, ambiguity,
unsupported kinds, invalid codec/range/binding/count, insufficient output,
malformed later morph records, and malformed/unaligned table preconditions.
Tests are source only until root executes them. Native/WASM compiled parity,
allocation measurements, actual author/reviewer UX, HMC/model completeness,
and content acceptance remain unqualified. No new model/asset was created.
