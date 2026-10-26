# Producer source-declaration repair

This historical repair is followed by the stricter selected-metadata safety
policy in [semantic-repair](../semantic-repair/README.md). Use its precise
20-to-21-target transition with this repair's 19-to-20 transition.

This repairs the source-review P2 finding that the native constant-grid label was
discarded when a new resource was saved with `meta: None`. Eight target changes
are staged only in this producer candidate; sibling and root-integration sources
were not modified by this worker. `before/` contains the exact seven previously
frozen producer files copied into root integration, and `source.rs` is new.
`target-manifest.json` binds those baseline hashes to the updated candidate.

`GeoRasterCreateRequest` now requires `source: SourceDeclaration`. Its typed status
is Synthetic, Provided or Unknown; description and optional evidence reference
each require 1–4096 bytes when present. `observation_claim` must be false unless
status is Provided and an evidence reference is explicitly supplied. Such a claim
is still unverified. There is no auto-generated provenance or observation claim.

New resources retain `SourceAnnotation { source_declaration, qualification:
Unverified }` in their existing HMC `ResourceData.meta` using serde/ciborium CBOR,
with a fixed 16 KiB writer cap. This is an inert app annotation inside the
existing metadata mechanism, not a new package format, authority store, source
rights graph, scientific observation or adopted agreement. FND-01 provenance/
scientific qualification remains open. The existing workspace ciborium package,
already a dev dependency, becomes optional normal native dependency; lockfile
versions are unchanged by this worker.

The constant-grid form always declares Synthetic, displays that status and the
Unverified qualification, and retains the entered description and explicit source
reference. Direct-array callers must explicitly choose Provided or Unknown (or
Synthetic when accurate). No missing legacy source status is guessed. Existing
`ReplaceBytes` edits retain every old metadata byte, including opaque unknown
CBOR and the new annotation.

The fresh-process fixture now creates a synthetic resource, changes its selected
sample through the canonical edit, compares the source metadata digest before/
after the edit, then reopens in an absent-source child process and checks source
declaration, reference and Unverified qualification plus unrelated resource/CBOR
bytes. Direct-array creation checks its explicit Provided observation claim
remains Unverified. New unit case:
`graphical::checkpoints::georaster_edit::source::tests::georaster_source_declarations_remain_unverified_bounded_and_distinct`.
The total selected `georaster_` library case count is now eight.

Formatting/syntax parsing only ran with rustfmt on the changed focused staged
Rust files. No Cargo, tests, builds, GUI, credentials, live package write or asset
production ran. Root must merge the eight precise changes into its integration
candidate, add the new required source field to its own fixture constructors,
then compile/execute the reviewed library cases. FND-01–06 and IP-03 C1–C8 remain
open; complete model packages and human acceptance remain unestablished.
