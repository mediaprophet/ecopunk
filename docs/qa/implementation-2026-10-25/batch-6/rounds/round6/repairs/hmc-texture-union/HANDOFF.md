# Canonical Studio HMC texture union repair

Staged source only. No live sibling edits, Cargo builds/tests, GPU, GUI, provider,
production assets, credentials, commits or publication were performed here.
Root retains the original failed native workflow and owns installation and actual
qualification. The four Core targets and one Poet Cargo registration are pinned
with exact before/after SHA-256 in `manifest.json` / `install-manifest.json`.
All Rust files were parsed/formatted with rustfmt; this is not type correctness.

## Cause and repair

Core `poet_host/invoke/studio_checkpoint/codec.rs::encode` inserted all retained
resources, then inserted a separately built canonical compiled-asset HMC. A
texture already retained under `textures/sha256/<digest>` was therefore inserted
twice. Its old preflight also counted/charged repeated image declarations rather
than the actual unique output. Source presence establishes this cause; root's
failed actual workflow is the runtime evidence.

The private focused `texture_union` module reserves source/geometry/graph keys,
then validates participating canonical texture dependencies and plans the exact
union. Same digest/bytes/exact MIME declarations reuse a retained entry or one new
image entry. All retained opaque metadata/provenance stays byte-exact because
the original ResourceData record is written unchanged. TextureDependency has no
opaque metadata field; conflicting duplicate MIME declarations refuse. Geometry
and Q42 entries never gain the texture deduplication rule. Cross-kind collisions,
source/reserved texture namespace keys, canonical SHA mismatch and participating
case aliases refuse before a candidate is returned. No retained resource is
removed or rewritten to avoid the collision.

Unrelated retained resources remain inert, including future kinds and unusual
keys within the texture-like namespace; source-only encoding preserves their old
opaque behavior. Validation is applied when an incoming dependency participates
in a union. Borrowed key/graph-key and per-entry payload/MIME length charges run
before key clones or dependency SHA work; aggregate unique charges still run
before canonical packaging. No replacement image codec or renderer is added.

## Public native consumer tests — authored, unexecuted

`crates/qualia-core-db/tests/studio_hmc_texture_union.rs` contains **six** tests,
also registered in Poet as `foundation_hmc_texture_union` using its existing
native dependency closure. The path is
`../../qualiaDB/crates/qualia-core-db/tests/studio_hmc_texture_union.rs`, gated by
`native-graphical`. No new dependencies are required. Its memory-only GLB fixture
uses the existing public native importer/compiler with a synthetic 2 × 2 RGBA8
KTX2 image, real MAT4 reference, source/compiled witnesses and Q42 graph.

Cases cover retained exact metadata/source/parent lineage and unique entry
budget; shared new dependencies; MIME/digest/alias/cross-kind/reserved conflicts
without input mutation; source-only unknown namespace preservation; entry-length
preflight before digest work; and real native save/reopen plus exact unchanged
destination/directory state after a conflicting encode/save pipeline. Native IO
uses RAII TempDir. The parent native contributor/reviewer HMC workflow remains
strict and separately proves the application consumer's saved texture closure.

Suggested root command selects the existing Poet native configuration and
`--test foundation_hmc_texture_union -- --test-threads=1` with six expected
passes. Existing material/resource/checkpoint tests and the original failed full
workflow must be rerun as root decides. Source-only unknown resources are not
semantic or rights admission, and these tests are not model/human acceptance.
Assets remain HOLD; IP-03/FND and complete film/game/cartoon models remain open.
