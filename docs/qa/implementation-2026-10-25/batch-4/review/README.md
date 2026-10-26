# Batch 4 independent source review

Status: source review complete for the digest inventory below; execution remains
supervisor-owned. This review examines the actual saved QualiaDB and Poet
sources, not merely predecessor fixtures or staged candidates. No Cargo build,
test binary, GPU run, GUI session, commit or publication is performed by this
reviewer. Source inspection and test-source inspection do not establish runtime
execution, human acceptance, complete HMC embodiment delivery or IP-03 readiness.

The [reviewed input inventory](reviewed-inputs.json) binds the inspected files.
Changes after its digests require renewed review. No unresolved blocking source
defect remains in this bounded authentication/resource-preservation scope after
the corrections below. This is not a complete native authoring/product verdict.

## Findings and disposition

| ID | Priority | Finding | State |
| --- | --- | --- | --- |
| B4-R01 | P1 | Authority methods sampled the clock before waiting for the mutex, allowing a queued operation to authorize after expiration against stale time. | Source corrected: blocking methods now sample time after acquiring the lock. Execution belongs to supervisor verification. |
| B4-R02 | P1 | Independent `NativeAuthority` owners could open the same journal head without an OS ownership lock, then both run callbacks before one journal append collided. Per-instance mutexes do not serialize those owners. | Source corrected: stable held ownership sidecar/OS lock; second-owner and reopen regression test present. |
| B4-R03 | P2 | Journal retained a resolving root path without pinning/rechecking its canonical directory, and used metadata-length check followed by unbounded `fs::read`. | Source corrected: canonical directory pin/recheck, bounded read and Unix directory sync. Host-owned-filesystem race limit remains explicit. |
| B4-R04 | P1 for complete-package claims | Raw resource editing allowed native texture/geometry/graph dependencies to be removed or replaced without known closure checks. | Source corrected for known native references: all six material texture slots, content-addressed keys, generated geometry/graph pair preservation and explicit pair removal. Unknown semantic dependency completeness remains unassessed. |
| B4-R05 | P1 | First canonical-directory correction compared `read_dir(configured_root)` paths against canonical paths, rejecting retained Windows/relative-root journals. | Source corrected: enumeration uses canonical root. Ownership/drop/reopen regression test includes retained canonical path. |
| B4-R06 | P1 | Canonical contextual verifier is non-strict; only rejecting the all-zero public key allowed nonzero low-order controller or anchor keys. Identity-point signatures can bypass possession under a weak adopted key. | Source corrected: admission validates both pinned authority and each controller with Dalek `VerifyingKey` and `!is_weak()`. Explicit identity-key rejection regression test present. Existing shared verifier semantics were preserved. |
| B4-R07 | P2 | `NativeProject::open` prepared/read HMC before authenticating the supplied native session. | Source corrected: preparation runs inside current native authority transaction, with matching scope and `ReopenCheckpoint` authorization; activation separately reauthorizes. |
| B4-R08 | P2 | Resource library bytes were read before verifying expected package/resource digests and immutable-release policy. | Source corrected: explicit native preflight and package inventory/key/digest checks precede library access; reserved source-key rejection also precedes library read. |
| B4-R09 | P2 | Package audit knew an append was required, but a predictably exhausted journal entry budget could still allow the HMC write before append failure. | Source corrected: entry-capacity and conservative serialized-byte reserve preflight precede package callback; callback-not-run regression test present. Unexpected append failure truthfully reports source/package may have committed and latches reconciliation. |
| B4-R10 | P2 | The initial native dependency guard demanded changed graph bytes for any changed 10d payload, even when a presentation/material edit legitimately preserves authoritative graph facts. | Source corrected: the same plan must include an explicit old-SHA-guarded companion graph replacement; validated graph bytes may remain identical. Material-change/identical-graph regression case present. |
| B4-R11 | P2 | Initial batch-control help text contained literal braces inside interpolated RSX text. | Source corrected: plain help text avoids unintended interpolation. Compilation remains supervisor-owned. |

Corrective source references in the captured revision:

| Finding | Exact saved source |
| --- | --- |
| R01 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/mod.rs:306` (lock), `:307` (clock), with corresponding ordering in `session.rs` and `records.rs`. |
| R02 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/journal.rs:96` (held OS lock). |
| R03 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/journal.rs:140` (bounded read), and `append` canonical-root checks. |
| R04 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_checkpoint/dependencies.rs:26` (known closure), `:46` (geometry/graph), `:59` (content-addressed texture key). |
| R05 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/journal.rs:99` (canonical enumeration). |
| R06 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/admission.rs:121`, `:139`, `:171` (strong-key gates/helper). |
| R07 | `C:/github/poet/native/src/graphical/native_project.rs:67`, `:82` (current authenticated preparation). |
| R08 | `C:/github/poet/native/src/graphical/package/mod.rs:129`, `:141`, `:152` (reserved key/preflight before library read); `checkpoints/resources.rs:40` (release policy preflight). |
| R09 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/journal.rs:40` and `mod.rs:314` (capacity preflight/callsite). |
| R10 | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_checkpoint/resources.rs:240` (actual validated replacement keys), `dependencies.rs:51` (explicit companion guard). |
| Batch | `C:/github/poet/native/src/graphical/package/plan.rs:5`, `:36`, `:86` (whole-plan guards/one apply/aggregate read); `package/intents.rs:65` (strict text command); `mcp/resources.rs:83` (same native batch dispatch). |

These are references to corrections, not claims that the original defects remain
in the captured source. `reviewed-inputs.json` supplies every file's exact SHA-256.

## Threat model and qualification limits to resolve

The trust anchor and signed admission must be supplied by trusted native host
configuration. Source/HMC/MCP fields and matching identifiers cannot grant that
authority. A real adopted signature and controller possession proof are required;
test seed material is an explicit fixture and cannot enroll a human.

Hash-chaining detects changed/interrupted retained entries and interior sequence
gaps. It does not independently establish an externally protected latest head.
Deleting a contribution/review-only tail leaves the same document/admission and
can pass current reopen checks. A writer able to rewrite the host-owned journal
can also rehash altered unsigned evidence records. Any final contract must state
the trusted-storage assumption and must not claim hostile-storage rollback or
forgery resistance from a plain hash chain. Source/admission rollback is refused
only when the native caller retains and supplies the correct current values.

Authority journal append syncs file bytes. Directory-entry durability and Windows
power-loss recovery require qualification; retained file synchronization alone is
not a universal durability claim. The HMC IO module explicitly discloses the
cooperating-writer/external-writer compare-and-rename limit and Windows directory
durability limit.

Review/contribution records bind editable source `DocumentRevision`. They are not
proof of acceptance of arbitrary opaque package resources. Resource-only changes
must not inherit a source review as model/package acceptance. The added native
package-write audit separately binds actor, source, prior/result package SHA and
time, under the same authority lock; it is write evidence, not model acceptance.
Whole-HMC review decisions, quality evidence and human acceptance remain separate
requirements. Only one native principal session is active per authority owner;
switching subjects retires the old session. Simultaneous multi-principal authoring
has not been demonstrated.

The project-level native authoring gate is implemented here. Copying opaque rights
or sensitivity metadata byte-exactly does not implement per-partition rights or
sensitivity admission. No such policy qualification is inferred from preservation.

The UI inventory is a digest-bound observed snapshot. Source/session/authority
changes hide it, and every later mutation checks the expected package SHA. Another
cooperating or external writer can change disk after inspection; no continuous
filesystem-current inventory guarantee is inferred from a displayed snapshot.

The core resource API, native batch controls and MCP `poet_hmc_edit_resources`
now accept one vector of 1–64 edits. The complete plan's filenames, duplicate and
reserved keys, expected package SHA, per-resource old SHA and immutable policy
are checked before library reads. One host-configured byte budget bounds all
imported payloads together. The opened file's size and bounded read are checked,
then source/package/policy checks repeat before one core save. Coupled geometry/
graph replacements and removals can use this path; the earlier one-entry product
limitation is resolved in source. Actual graphical usability remains unqualified.

## Positive source observations

- Native session/challenge constructors have private fields and no Deserialize;
  signed project admission binds controller keys, subject, project, validity and
  review grants. Session challenge binds owner incarnation, nonce, descriptor
  digest, key, expiry and length-delimited project/subject.
- Every callback passes current canonical `ReadSource` admission, with operation
  permissions independently checked by the cooperative adapter. Replacing signed
  admission invalidates active/pending sessions and cannot regress its sequence.
- Authority mutex poisoning denies access; source-successor or post-source
  persistence failures latch reconciliation rather than asserting rollback.
- Checkpoint carrier retains all non-reserved payloads, kind strings and opaque
  metadata. Unknown resource kinds remain inert. The source metadata is retained
  separately. Whole-file expected SHA plus per-edit payload SHA guard replacement.
- Resource candidate edits are detached; core encoding rejects duplicate keys.
  Existing published input can be pinned by native host policy and copied to a
  distinct working file with parent digest lineage.
- Atomic install is the commit point. Post-install Unix directory-sync trouble
  returns a committed receipt with a durability warning rather than a generic
  failure that would leave the retained carrier at the old package revision.
- The source clearly labels checkpoint packaging as incomplete embodiment
  authoring. Fixture tests do not manufacture master-quality or human acceptance.

## Integration source review

Inspected root-owned native project configuration, controller/cooperation/shared
host/jobs/package UI/MCP integration. The actual composition is separate frontend
`AuthorizedEditorHost` instances outside one `SharedEditorHost`, then optional
resource library import and checkpoint decorators over one canonical editor. A
callback holds current native authority before acquiring the shared editor; no
policy wrapper is placed inside the shared lock. Resource actions require
`SaveCheckpoint`, inventory requires `ReadSource`, and reopened execution stays
held. Incoming source/HMC/MCP fields do not construct native sessions or adoption.

The shared editor contract has been extracted from the controller. Resource
ingress, persistence, controls, authority, evidence and tests remain in focused
modules. New authority and cooperation implementations have likewise been split.

Supervisor-owned builds/tests and actual usable graphical contributor/reviewer
acceptance remain separate evidence. Film, open-world game and cartoon packages,
their exhaustive deliverable/expanded-case ledgers, high-grade master quality,
rights/semantic admission and human acceptance are not completed by this work.

## Final bounded batch follow-up

The supervisor froze its 127-file production input snapshot as
`130ad0c42d5e10e607f649ede387c1450b8d696495debaeb1d6f5ef51983ec9f`.
The refreshed reviewer manifest binds the inspected subset; the initial manifest
is retained as `reviewed-inputs.initial.json`. No test-source edits or Cargo
commands were made by this reviewer.

Inspected the new `package/{plan,intents,library}.rs` path, strict shared intent
schema, editor/auth/shared forwarding, `Action::ResourcePlan`, actual UI mounting
and MCP dispatch. Batch requests remain within the same current native owner and
package-write audit transaction; they do not create grants or broaden paths.
The core derives its companion-graph replacement set only after validating each
actual old digest. An explicit byte-identical graph replacement therefore records
the intended pairing without inventing changed scientific facts. This is an
intent/integrity guard, not proof of semantic agreement between graph and mesh.

Test-source review covers actual canonical geometry/Q42 paired replacement and
removal, stale intent, immutable preflight, duplicate keys, aggregate budget
failure, strict/no-path command parsing and material-only change with an identical
explicitly replaced graph. Execution results must be obtained from the supervisor's
saved-source checks; no passing test result is inferred here.
