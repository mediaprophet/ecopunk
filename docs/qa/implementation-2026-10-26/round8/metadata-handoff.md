# Round 8 source-entry metadata handoff

Status: **v2 frozen for independent review; staged, not applied or executed by
this worker**. The exact 20-file before/after images and SHA-256 guards are in
[the manifest](../../../../tools/poet-foundation/round8/metadata/manifest.json),
candidate `round8-source-carrier-v2`. Root owns sibling installation and actual
consumer verification. Assets remain on hold.

The initial metadata-rebinding proposal was rejected and is retained under
`tools/poet-foundation/round8/metadata/rejected/`. Opaque source metadata can sign
the original exact source payload, key, index or broader package context.
Copying its bytes onto changed source would misstate its referent, including
when only parent digest or JSON whitespace changes. Version 2 preserves the
whole original HMC; it never inherits its metadata onto the current source.

## Canonical contract and ownership

Core `studio_checkpoint/carrier.rs` owns the additive `VerifiedSourceCarrier`,
`decode_source_carrier`, and `Checkpoint::with_source_carrier` APIs. Native
`load_source_carrier` reuses existing bounded native file reads. A carrier
privately retains a canonically verified Checkpoint, original HMC bytes, and
its original SHA-256. Accessors expose `checkpoint`, `into_checkpoint`,
`resources`, `origin_digest` and `origin_key`; none grant authority or activate
source.

A merge requires the exact same artifact identifier and source domain and a
current capture without inherited source-entry metadata. It retains CURRENT
source record, journal and host-qualified parent. Shared ordinary resource keys
must agree exactly in bytes, kind and opaque metadata, including absence. When
the selected original has source metadata, the merge retains that complete
original unchanged at `authoring/origins/sha256/<bundleDigest>.hmc`, kind `hmc`,
with no outer metadata. Original metadata stays at its original source entry
inside that exact archive. Distinct bundle origins may coexist; equality of
names or digests does not establish semantic equivalence or authority.

The origin namespace is Core-owned. Canonical lowercase SHA-256 locator,
`hmc` kind, absent outer metadata and exact payload digest are checked by both
reader and writer without recursive archive interpretation. Generic resource
add/replace/remove refuses origin keys. Current source metadata is absent in
rewritten outputs; subsequent saves retain existing origins without wrapping
every new output recursively.

Reader compatibility includes existing metadata-bearing source files. Writer
behavior deliberately changes: `encode` refuses any inherited source metadata,
and legacy `with_resources_from` refuses metadata on either source or base.
The supported writer path captures current source and retains a verified
original archive. The old mixed-resource regression now checks exact original
archive preservation instead of metadata rebinding onto changed current source.
No metadata, signature, rights, licence, review or human acceptance is inferred.

## Native path

Poet adds `retained_surface_carrier(expected, limits)` to the existing native
EditorHost contract and forwards it through shared/session/package/diagnostic
and current authorized ReadSource owners. `TextureResources` uses Core's
canonical decoder with explicit package/copy budgets. Source-free texture
packages continue through the existing bounded ordinary resource path.

CheckpointService now keeps verified carriers for existing disk, retained
reopen/save and immutable-release bases. Fresh save/compile can retain a bound
source carrier. Existing destination/retained lineage remains authoritative
for the local parent link when present; a fresh bound-origin output uses the
verified input digest. Current source is captured and revalidated before the
atomic native write. Refusals leave the retained carrier and installed file
unchanged. Existing path, release and expected-digest guards remain in place.

Replacement compile routes inherited resources and protected origins through
the exact carrier merge, never generic origin ResourceEdits. Changed compiled
representations still require explicit fresh asset keys, preserving richer
existing sections and graph resources. Inert resource copying is available
while execution is held; the compile/current-publication and render invalidation
checks remain separate and unchanged.

## Verification and remaining work

This worker formatted staged sources and verified frozen before/after SHA-256.
The final rustfmt check found one formatting-only miss in Poet `package/mod.rs`:
the new carrier forwarding signature needs multiline wrapping. The frozen
candidate remains unchanged; root was notified so actual-source formatting can
be recorded without silently changing reviewed images. Per-target checks are
in `tools/poet-foundation/round8/metadata/verification.json`. No Cargo, builds, runtime tests,
credentials, provider calls, commits or publication were performed.

Eight Core regression cases are staged but **unrun**: reader/writer/legacy
compatibility, changed current record/journal/parent with exact noncanonical
original source bytes, metadata-free/idempotent union, ordinary conflicts and
protected origin edits, origin key/kind/metadata/digest validation, distinct
origins, foreign source/domain and budgets, and native temporary-directory
save/reopen/Save As/corruption/stale-digest refusal. Test scratch uses TempDir
RAII. Focused new implementation is below 500 lines; Core carrier is 258 lines,
Poet snapshot helper 46, and the largest touched existing implementation 498.

An independent worker owns the separate public Poet
`foundation_source_carrier` regression target and Cargo route. Root must review
and install both frozen packages, run actual-source consumer tests and relevant
checks, and record source-bound receipts before reporting any executed pass.
The [independent peer review](metadata-peer-review.md) found no remaining blocker
in the frozen 20-file implementation. Actual execution may supersede this staging status;
consult their receipts rather than interpreting source presence as a result.

This advances only bounded metadata/source retention within ECO-034/044 and
native cooperative authoring foundations. IP-03 C1–C8, FND gates, full graph and
rights derivation, required operators, native human contributor/reviewer
usability, all complete models and production acceptance remain open.
