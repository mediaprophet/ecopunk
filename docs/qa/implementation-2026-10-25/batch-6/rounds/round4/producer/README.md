# Round 4 native Poet raster producer and edit candidate

Staged source only. No sibling source, existing terrain/model asset, native
credential, GUI, network, commit or publication was touched by this worker.
No Cargo build or test ran. Root owns review, merged installation, lockfile
reconciliation and canonical native library checks. Asset production stays on hold.

The candidate adds native HMC controls and typed worker actions for explicit
developmental M64 v3 creation and editing. It uses installed public
`qualia_geospatial::raster_m64::{compile_raster_to_m64_v3,
decode_raster_from_m64_v3}` and Core `M64Reader`; it adds no format codec.
Canonical source remains in `C:/github/poet` and `C:/github/qualiaDB`.

## State, authority and persistence

`SessionEditorHost -> AuthorizedEditorHost -> SharedEditorHost ->
ResourceImportHost -> CheckpointEditorHost -> CheckpointService` remains one
canonical editor/session/resource owner. Both new mutations call the existing
`AuthorizedEditorHost::write_package` with expected filename/package SHA and
source reference. The production native owner therefore retains its current
grant and package-write contribution/audit lifecycle. Fixture owners are explicit
software-agent authority and provide no durable native-authority or human claim.
No package review adapters or review authority are implemented here.

Creation takes full explicit `M64GeoRasterMetadata`, LUT epoch and integer source
samples, expected existing HMC SHA and a new resource key. The canonical producer
and reader run before `ResourceEdit::Add`; a colliding key, stale package/source,
bad metadata/extent/sample count, exhausted budget or cancellation refuses the
operation. The native form exposes every frame/policy value and an explicitly
synthetic constant sample; reference codes and epoch zero are visible unqualified
declarations. This form is not acquisition of measured terrain. It does not
persist an external observation/provenance attestation. The explicit sample-array
API supports supplied source values without inventing legacy metadata. A required
typed source declaration now records Synthetic, Provided or Unknown status,
bounded description/reference and an optional explicit observation claim. The
UI always declares Synthetic. An observation claim requires Provided status and
an explicit reference; every saved declaration remains Unverified. Standard
serde/ciborium encoding stores this annotation in the existing HMC resource
metadata. It is not a qualified scientific graph, observation attestation,
permission or rights record; FND-01 qualification remains open.
The annotation also binds the SHA256 of the original canonical M64 payload.
Its declaration concerns those origin bytes; later synthetic edits have a
different current payload SHA and retain separate canonical package-write lineage.

Editing takes the inspected filename/package SHA/resource key/resource SHA,
new positive integer cell spacing, independent origin elevation and one selected
source-coordinate absolute sample. It recovers the entire original source array,
regenerates its original bytes with the canonical writer, and requires exact byte
equality before changing anything. This refuses richer/noncanonical layouts
whose fields the elevation-only producer cannot retain; unsupported content stays
unassessed, never converted by guesswork. The canonical reader itself rejects
v3 non-raster fields. Legacy v1/v2 missing metadata expressly refuses automatic
upgrade/edit. Exact origin f64 bits, epoch, SRID/datum declarations, rows, nodata,
edge policy, source extent and LUT epoch remain when other fields are edited.
Samples remain absolute mm; changing origin does not shift sample values.

The selected replacement uses canonical `ResourceEdit::ReplaceBytes`, retaining
its kind/opaque CBOR metadata. Both mutations use canonical bounded checkpoint
capture/save with the expected source stamp and HMC SHA, keeping editable source,
every unrelated resource, opaque CBOR and parent digest. Service preflight and
final save repeat source/package/resource guards. No extra/private store exists.
Byte preservation alone does not establish semantic truth after a payload edit.
Selected metadata is therefore admitted only when absent (no source claims) or
when it is the complete bounded known Unverified/Synthetic/non-observational
origin annotation with a nonzero original SHA. Opaque, Provided, Unknown and
observational selected metadata holds editing pending a qualified derivation
contract. Unrelated opaque resources and their metadata remain byte-exact.

The worker validates request bounds before queueing, serializes operations,
checks current source/session/authority before dispatch and publication, and
passes its closing cancellation token through compilation to the precommit
check. Codecs and the admitted canonical save are bounded noninterruptible
batches: cancellation arriving after the final precommit check can complete a
save. There is no claim of atomic cancellation against the OS rename. A changed
context withholds completion; started writes can require reconciliation. UI
success clears the old inventory/inspection and requires a fresh inventory.

## Allocation and scope

No heap is introduced in per-sample codec kernels. This cold native adapter uses
fallible bounded sample/output allocation. Its caller selects M64 byte/chunk/
record and combined sample-plus-output byte limits. The UI profile uses the
existing 64-chunk/262144-record inspection profile and 32 MiB combined raster
working budget; the constant-grid form separately limits authored source cells
to 4096. These are operator profiles, not a format/master-quality ceiling.
HMC loads/capture/encode and retained carriers remain separately bounded by
existing `CheckpointLimits`; the 32 MiB value is not a whole-process memory cap.
The outer queue additionally caps supplied sample arrays at 262144 integers.

New implementation modules are below 500 lines; test helpers and tests are split
by fixture construction versus execution. A new optional native
`qualia-geospatial` path dependency is enabled only by `native-graphical`.
Existing lockfile versions are not modified by this worker.
Source annotations have a fixed 16 KiB CBOR writer cap plus bounded 4096-byte
description/reference fields and use the existing native HMC metadata mechanism.
These small cold annotation allocations are additional to the documented raster
sample/output budget and remain covered by canonical aggregate HMC byte limits.

## Root verification targets, all unexecuted here

Run the native **library** targets against reviewed installed source with explicit
workspace TEMP/TMP, debug=0 and incremental=false as dispatched. Filtering by
`georaster_` should select the following nine cases, zero skipped/ignored:

- `graphical::checkpoints::georaster_edit::tests::georaster_edit_canonical_samples_metadata_and_padding_roundtrip`
- `graphical::checkpoints::georaster_edit::tests::georaster_edit_cancellation_budgets_and_coordinates_fail_closed`
- `graphical::checkpoints::georaster_edit::tests::georaster_edit_legacy_corrupt_and_richer_payloads_refuse_loss`
- `graphical::cooperation::tests::georaster_edit_tests::georaster_edit_authorized_save_and_fresh_process_reopen`
- `graphical::cooperation::tests::georaster_edit_tests::georaster_edit_authority_stale_digests_source_and_cancel_leave_hmc_exact`
- `graphical::cooperation::tests::georaster_edit_tests::georaster_create_explicit_source_guarded_add_and_refusals_preserve_hmc`
- `graphical::cooperation::tests::georaster_edit_tests::georaster_jobs_dispatch_real_authorized_create_edit_and_withhold_revoked_results`
- `graphical::checkpoints::georaster_edit::source::tests::georaster_source_declarations_remain_unverified_bounded_and_distinct`
- `graphical::cooperation::tests::georaster_edit_tests::metadata_tests::georaster_edit_refuses_selected_opaque_provided_unknown_and_observation_metadata`

Fixtures generate only ephemeral synthetic grids through the canonical writer.
Test-only negative mutations renew canonical CRCs to exercise unsupported fields,
legacy version compatibility and noncanonical metadata retention refusals; no
production implementation hand-encodes M64. Fresh reopen invokes the exact same
library case in a fresh child process, with a bounded 30-second deadline, absent
Studio source and explicit fixture ReopenCheckpoint authority. The child proves
both newly created and edited resources plus source cells after canonical
activation.
The fresh child also verifies the synthetic source declaration/reference remains
Unverified after creation, a later `ReplaceBytes` sample edit and process restart;
the unrelated unknown resource/CBOR remains exact. Direct-array creation verifies
its explicit Provided/observation-claim annotation remains Unverified. Unknown
status and invalid/oversized source declarations have focused unit expectations.
The worker case exercises real native queue/actions/authorized owner
create/edit dispatch and revoked completion withholding. Polls are bounded at
10 seconds and contain no barriers. Source/session, revocation, observation-role,
package/resource digest, cancellation, capacity and retained bytes have explicit
negative expectations.

`rustfmt --edition 2021 --config skip_children=true` parsed/formatted only the new
focused staged Rust files successfully. This is syntax/formatting evidence,
not compiled or executed correctness. Root must review common trait/job/UI
snapshots together with the separate package-review worker's changes.

## Remaining requirements

FND-05 and IP-03 C1–C8 remain open: no usable native GUI operation, actual people
enrollment, contributor/reviewer acceptance, package review, accepted master,
scientific CRS/datum validation or complete source-to-output production case was
executed here. All FND-01–06 readiness gates remain open. Film, open-world game and
cartoon packages remain incomplete; this work produces no model package and does
not close any catalogue or subject-specific completion IDs. Full source rights,
provenance authority, review scope and production evidence remain independent
requirements. Existing terrain/model assets remain untouched.
