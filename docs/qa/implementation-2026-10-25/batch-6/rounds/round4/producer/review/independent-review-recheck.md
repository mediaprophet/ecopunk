# Frozen candidate source recheck

2026-10-25. Producer worker independently re-read the repaired canonical/native
review candidate and eight native Poet adapter/UI files. **No open blocker in
the inspected current source.** This supersedes the first report's open-finding
disposition while preserving that report as history. No builds, tests, GUI,
assets, credentials, candidate edits or live project writes were performed.

The canonical owner reports a 28-target frozen candidate, aggregate patch SHA256
`52f6895302f856a930f3312c3f0284ac2ddab792102f5aa638ddbb1fd10f69ff`.
The companion `recheck-source.json` independently pins the actual manifest and
candidate bytes plus the eight inspected integration files. A later source
change requires a fresh review; no execution result follows from these hashes.

The original P1 contributor-continuity finding is resolved at source for the
declared known-native-record scope. Native `history.rs` collects every recorded
source contributor within the current project/document/source owner scope,
including contributions before any HMC save at that revision. It also traverses
the retained native destination-predecessor edges, collecting saved write actors,
saved source authors and recorded package contributors independently of package
identifier/filename aliases. Subject, traversal and frontier bounds fail closed.
The same history feeds runtime decisions, explicit steward grants and new grants
in signed replacement admissions. The pure policy receives the host-owned list
and denies known contributors despite an exact explicit grant.

New public-native fixtures specifically cover A producing P, B saving a renamed
successor and A being denied grant/decision; and A's recorded source-v1 work,
B's v2 source edit without a v1 HMC save, C's first HMC save and A's subsequent
grant/decision denial. These cases are unexecuted in this review. They authenticate
the actual selected fixture actor and do not infer package authority from source
review grants. The latter fixture starts with an explicit signed future target
grant and proves known-source history still denies its exercise.

The P2 selected-target display finding is also resolved at source. Native
`package_workflow_evidence` now filters contribution/request/review output by the
entire exact selected `PackageRevision`; its complete internal ledger remains
available to the contributor-exclusion calculation. The public target-isolation
fixture creates an Accepted decision for one target, then checks unrelated and
identifier-alias targets receive no unrelated decisions or requests. The UI
therefore no longer receives an unrelated Accepted record under the current
package heading. This source fixture remains unexecuted here.

The lock-lifetime integration condition is resolved at source in
`package_workflow/service.rs` and `shared_host.rs`: Core first admits the current
authority/session; its callback acquires the raw shared editor guard; an outer
`Option<MutexGuard>` retains it after callback return until the native record
method, including journal append, returns. The guard is then dropped before the
next inspection. No authorized editor or authority method is recursively called
inside the raw verification callback. The raw guard uses a bounded try-lock, so
contention reports refusal rather than waiting on a reversed lock order.

`verify_package_guard` checks current SourceRef/stamp, raw snapshot, host-bound
project/document/source/domain, source revision/digest and actual bounded
canonical HMC inventory SHA. It repeats source freshness after loading. The
native authority independently requires the latest exact saved write audit,
source, size, predecessor and actor, and holds authority through the commit.
The worker repeats source/session/authority publication checks before exposing
the result. The native steward UI submits an explicit filename/release ID/SHA,
admitted reviewer subject and evidence; runtime prechecks the actual package,
then Core rechecks current steward permission, exact audit target and signer.
Grant/revoke signs an admission successor and detaches the session.

Important bounded limits remain explicit. `expected_digest` and the history
closure describe prior **destination** bytes, not complete SaveAs derivation.
Missing, ambiguous or cyclic retained history carries a false completeness
flag. SaveAs ancestry, authorship outside the canonical recorded scope and
resource/rights/domain completeness stay unassessed. These scoped decisions do
not establish model completion or human acceptance. Direct external filesystem
writers remain outside the shared editor mutex; verification attests a package
snapshot and does not promise protection against later external changes.

The eight integration files reviewed were package_workflow `mod.rs` and
`service.rs`, `shared_host.rs`, `package_review_controls.rs`, native_entry
`package_admin.rs` and `package_grant_controls.rs`, `native_project.rs` and
`jobs.rs`. Other routing files and the root's real-HMC NativeProject integration
test are outside this eight-file review. Root still owns compilation, actual
file-mismatch/stale-save/actor/revocation execution and graphical qualification.
FND-01–06 and IP-03 C1–C8 remain open; assets stay on hold and complete film,
open-world-game and cartoon packages remain incomplete.
