# M64 SDK staged compatibility and qualification

Six scoped files are saved under `sdk-staging/`, mapped with exact before/after
SHA256 values and diffs in `source-manifest.json`. Existing-source snapshots are
under `before/`; new-file snapshots use the documented empty baseline.

The checked reader adds a public API without changing existing layout types,
magics, version constants or validators. It admits native little-endian M64 v1/v2
and legacy E64 magic. Unknown versions preserve the complete u32 value in the new
reader error; the existing legacy validator API remains unchanged. Only raw codec
0 and container flags 0 are admitted. Serialized offsets must be multiples of 64;
the caller's input pointer may be unaligned. Empty containers are admitted;
nonempty chunk entries require a positive record count. Unreferenced padding is
allowed and is not authenticated. Caller limits cap file bytes, chunks and total
records; interval scratch requires one element per chunk. Iterative heapsort
checks overlap independently of directory/payload ordering.

Only payload byte ranges are borrowed without copying. Headers, directory rows
and records are stack-copied using alignment-safe POD reads. Strata and phase
record CRCs reuse their actual validators. Point and holo layouts have no embedded
record CRC and are explicitly counted as such. No Bao, authenticity, authority,
domain/scientific correctness, geodetic accuracy, model quality or native
authoring acceptance follows from structural verification.

The geospatial entrypoints and codec enum remain callable with existing argument
shapes. Unsupported nonraw codec selections now return
`IngestionError::UnsupportedCompression(codec)` before output mutation. Existing
successful raw v1 output construction is retained. Overflow preflight and an
explicit little-endian admission error are added; misleading Hilbert-sort prose
is corrected to describe the actual existing row-major output with Hilbert keys.
No original terrain artifact is rewritten. Added public tests cover mixed modes,
versions/aliases, offset-one input, corruption/bounds/budgets, caller scratch,
permuted interval order, checksum coverage and measured zero allocations. Three
producer tests cover unsupported codec output preservation, raw SDK readability
and overflow-shaped dimensions. These tests are saved but have not been run.

Automatic approval review rejected the guarded sibling installation script. Its
stated reason was that current trusted repository authority excludes sibling
edits, and general ecosystem authorization did not authorize the exact live
QualiaDB mutations. No sibling source/test file was written. Installation and
serial Cargo checks require the parent to resolve explicit user authorization
for the six destinations; this rejection must not be bypassed. The staged
implementation is reviewable but neither installed nor build-qualified.
