# R3-POET typed M64 resource inspector — staged candidate

This is a **staged, uncompiled, unexecuted** Poet-only candidate. The supervisor
owns review, guarded installation, cached native compilation and verification.
No Core, Cargo/dependency manifest, asset, credential, publication or source
package was modified. No Cargo, tests, GUI/Computer Use or provider calls were
run by this worker. Current Core conformance command ownership stays with root.

[The contract](contract.md) defines a selected-resource request carrying filename,
package SHA, resource key/SHA and exact expected source. All 15 source targets
are pinned in [target-manifest.json](target-manifest.json), SHA-256
`06646bd4d4577ea6ea610120972a019521c54d87bbe3934cc34c29f81a136e6b`.
Twelve existing files have complete before/after snapshots; three new files have
null expected hashes and complete candidate snapshots. Every before hash matched
the current live source when frozen. No candidate edit followed this freeze.

The existing native chain is retained: `SessionEditorHost` →
`AuthorizedEditorHost` → `SharedEditorHost` → optional `ResourceImportHost` →
`CheckpointEditorHost` → the existing `CheckpointService`. The additive trait
method refuses unavailable profiles by default. The authorized wrapper holds
the same current `ReadSource` owner decision across the operation; no role label,
checkpoint assertion, JSON principal or result data grants authority. The native
service retains existing allowlisted filenames, package/source/domain binding,
immutable-release pin and host checkpoint limits, and checks exact package/key/
resource digests before decoding. It drops the first carrier before a second
canonical bounded load, refusing disk changes observed before returning details.

The canonical Core APIs already provide the necessary bounded inert bytes:
`poet_host::invoke::studio_store::checkpoint::{load,Checkpoint::resources}`;
`M64Reader::parse`, its typed metadata/sample/verification accessors and
`M64PayloadInterval` supply decoding. There is **no new Core owner or reader**.
The native worker passes a fixed 64-entry stack scratch buffer and explicit
limits: 64 chunks, 262,144 records and 16,781,504 bytes. The selected payload cap
is additionally clamped to the configured host checkpoint file cap. This is an
inspection profile, not a format or master-quality ceiling. Budget errors name
the required quantity and limit where available and leave content unassessed;
they do not label oversized content corrupt. An exact required record total is
not available after the canonical reader's early record-budget refusal, so the
diagnostic states it exceeds the configured limit rather than inventing a count.

`PackageControls` gains one per-resource **Inspect M64 metadata** button, with
presentation in a focused child module. Results show exact v3 source extent,
spacing, f64 first-cell geographic anchor, independent origin elevation, epoch,
declared SRID/vertical datum, row direction, nodata, edge/padding policy, first
source sample and structural/CRC verification scope. Legacy v1/v2 explicitly
reports missing georaster metadata and its narrowed header summaries; unknown
payloads remain unsupported/unassessed without fallback. No unrelated resource
or opaque CBOR metadata is decoded, rewritten or reinterpreted.

Worker publication additionally requires the final context of a read-only
inspection to equal its origin source/session/actor context. UI publication
reuses current authority/execution/source checks and requires the selected
filename/package/key/resource SHA to match the currently displayed inventory.
Logout, revocation, source replacement, session switching and retired execution
epochs cannot activate a stale result. Existing write commands retain their
separate successor-context behavior. Inspection performs no package edits.

Six new CPU tests are authored, not run:

- `graphical::checkpoints::inspection::tests::canonical_v3_and_legacy_inspection_are_distinct`
- `graphical::checkpoints::inspection::tests::corruption_unknown_payload_and_budgets_never_fallback`
- `graphical::checkpoints::inspection::tests::selected_hmc_digests_and_unknown_metadata_remain_exact`
- `graphical::checkpoints::inspection::tests::native_service_preserves_immutable_pin_and_path_refusals`
- `graphical::cooperation::tests::selected_resource_inspection_requires_current_read_authority_and_source`
- `graphical::jobs::tests::resource_inspection_late_source_and_session_completions_are_withheld`

Tests construct ephemeral in-memory serialization/HMC fixtures and explicitly
label delivery-only worker data. They use existing software-agent fixture
authority, never OS credentials or human proof. Selection matching, exact unknown
metadata/package preservation, stale package/resource/source refusal, late job
delivery and epoch-successor completion invalidation are included. There is no new review
system or default successful grant. All three new source files are below 500
lines (148 implementation, 260 tests, 37 presentation). Existing `jobs_tests.rs`
is 512 lines after formatted test additions; it remains one cohesive test module,
not new production implementation or an ownership/lifecycle consolidation.

Actual worker verification: `rustfmt --edition 2021 --config skip_children=true`
on all staged Rust targets exited 0. This is syntax/format acceptance only;
Rust/Dioxus name resolution, macro expansion, native CPU cases and the ordinary
native binary still require the supervisor's actual compilation/run. Unchanged
dependencies were inspected read-only: Poet `native/Cargo.toml`, native project
composition, current controller/checkpoint/authorization/job contracts, and Core
checkpoint resources/module plus canonical M64 metadata/reader APIs. Applicable
Poet/native/parent `AGENTS.md` candidates were checked and absent; Ecopunk's
staging instructions and previously read QualiaDB requirements were retained.

Limits: a started inspection is bounded and non-interruptible inside the existing
worker; closing cancels queued work and withholds results through existing
publication guards. The double read identifies the inspected snapshot, not
perpetual filesystem freshness, cross-store atomicity or external anti-rollback.
No geodetic transformation, identifier authority, source provenance adoption,
rights determination, scientific accuracy, whole-HMC/resource review or human
acceptance follows from metadata display. FND-05 and IP-03 C1–C8 remain open;
actual operator author/edit/save/reopen cooperation, complete models and
production acceptance are separate required evidence. This is only the bounded
typed inspection slice of that workflow.

Subsequent root compilation found E0624 in the new delivery test's direct call
to private `SessionEpoch::retire`. The [test-only repair](repair/README.md)
replaces it with supported worker `Action::Rebuild` and explicitly distinguishes
the probe's epoch replacement from actual lifetime retirement. The original
15-target snapshot remains frozen history; the one-file repair carries its own
current before/candidate hashes. Root will run the six new inspection cases and
the existing actual CanonicalEditorHost retirement case separately. The earlier
zero-matching-case binary attempt is not a passing inspection run; the failed
actual `--lib` log remains preserved. No repaired execution success is inferred.

The subsequent real six-case run was interrupted after three passes, two native
temporary-destination save failures and a hanging barrier-based delivery case.
The [second test-only repair](repair-2/README.md) replaces that new case's
unbounded barrier with deterministic source-incarnation replacement inside the
test-only inspector hook, after dispatch and before return. It preserves the
post-admission stale-result check without claiming a measured concurrent race;
existing lifecycle race tests remain unchanged. Root owns the caller-selected
TEMP/TMP rerun and all actual command outcomes. No passing suite is inferred
from either repair or from the interrupted run.
