# Round 3 bounded frame decoding candidate

FND-04 scoped staging only. No sibling source changes, build, test, asset work,
network access, Computer Use, credential actions, commit or publication occurred.
Rustfmt parsed/formatted the three new Rust files; this is not compilation.
Root owns review, guarded installation and serial execution.

`before/` preserves complete original target bytes; `after/` holds complete
candidate target bytes. `manifest.json` binds every original/candidate pair to
SHA-256, byte count, line count and the original sibling target. New files have
no original hash. Check all originals before installation; shared module export
changes must be merged deliberately with other Round 3 owners. `prepare.ps1`
records the once-applied staging transformation and is not an installer or a
replayable script against already transformed files.

The canonical zeroheap APIs are `inspect_mesh_frames` and
`decode_mesh_frames_into`, with independent caller-selected vertex, encoded-byte
and decoded-byte limits. The typed equivalents retain the canonical resolver.
All validation, including late tangent direction/handedness and both capacities,
precedes either output mutation. Empty active streams, NRM1/FRM1, legacy and MID2
selection semantics remain. Existing public signatures remain as cold bounded
compatibility wrappers. Full preflight also precedes cold allocation; reservations
are fallible. Decode reads LE bytes without aligned header casts.

Native scene mesh loading and Portal upload use the shared bounded convenience
reader with explicit limits, not a private decoder. Their entire existing mesh
loading/scene allocation is not claimed to be zeroheap or budgeted. The compiler
keeps its old error projection and selects the compatibility policy explicitly.

Exact API/wire/compatibility/error/decomposition delta is appended to the
candidate `docs/manuals/standards/10d-typed-field-payloads.md`. The wire version,
existing kind registrations, tangent quantization and accepted normal pairs do
not change. New error variants extend the public enum, which can affect external
exhaustive matches; no local exhaustive match needs repair. No model/package or
foundation gate completion is claimed. All model production stays on hold.

After review and integration, root should run existing `typed_field_payloads`
and the new `frame_decode_sdk` integration test binaries, plus existing
`normal_section` and `compile_10d` unit cases under the selected native profile.
The new binary has seven focused tests: optional/unaligned streams and old API
parity; both short capacities; late malformed tangent atomicity; independent
limits/header failures; empty streams/hard caps; FRM1 normal-only and extreme
legacy normal codes; typed/legacy selection and atomicity. Its allocator counts
raw alloc/alloc_zeroed/realloc calls on the measuring thread, including refused
cold preflight. Test results and zero-allocation observations remain unverified
until root executes and records actual results. Run the supported WASM library
check separately; shared source alone does not prove WASM execution/interop.

Decomposition: normal_section.rs shrinks below 500 lines; new frame decoder,
typed adapter and test binary remain below 500. Existing oversized compiler and
Portal files receive only thin delegation/caller-budget edits. Their broader
decomposition remains open, not silently waived.
