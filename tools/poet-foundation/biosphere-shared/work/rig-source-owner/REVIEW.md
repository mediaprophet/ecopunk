# Independent review of staged rig source-owner candidate

Status: read-only review of candidate manifest and selected diffs; no compilation, execution, canonical edits or human/native trial. Manifest pins exact preimages and candidate hashes. Findings apply to those bytes; re-review modified candidates.

## Blocking issue: real worker cancellation is not propagated

- `viewport/renderer.rs.after:118` creates a new always-false AtomicBool, then passes it into admission at line 139. The source edit ultimately commits via `controller.rs.after:162`. The current native job dispatch `C:/github/poet/native/src/graphical/job_actions.rs:121` calls authoring_edit without forwarding the worker cancellation token; `checkpoints/editor.rs.after:162-163` can perform full resource IO and then invoke source editing without that token. A cancellation after dispatch cannot be detected by this path before commit.
- Required remedy: propagate the actual worker token through the rig-specific operation and resource/viewport staging, check it before source submit, and verify cancellation after dispatch but before commit preserves the last accepted source. Clarify the commit point and late-cancellation result; an always-false token is not cancellation handling.

## Significant limits and validation needed

- `checkpoints/inspection.rs.after:103` loads the whole HMC before enforcing the selected-resource cap; `viewport/renderer.rs.after:127,139` decodes mesh before the separate 42 MiB rig admission budget. Neither bound represents aggregate peak memory. Qualify the actual load, decode, source and GPU working sets; do not call this streaming or a 16 MiB peak limit.
- Existing test candidates exercise a synthetic `CanonicalEditorHost` with a no-GPU fixture preview (`rig_binding/tests.rs.after:476`), and isolated topology selection (`viewport/tests.rs.after:338`). They do not establish the composed signed native owner → selected HMC → real retained viewport → source edit → save/reopen route. Add positive and stale/denied/wrong-resource/cancel/recovery cases on the composed path, then a real human-window trial for usability.
- Existing source-bound control depends on an already present mutable `poet_rig`; `controller.rs.after:158` refuses initial adoption, and `authoring/controls.rs.after:310` hides controls without a descriptor. The staged UI explicitly says preview animation is disconnected at line 313. An edit of a manually prepared fixture is a bounded increment, not an accessible start-to-finish rig authoring or animated viewport workflow.
- `checkpoints/inspection.rs.after` uses current project read authorization, exact source association, package/resource digest and release policy. These are not proof of third-party per-resource rights; synthetic fixture scope is supportable under existing authoring tests. Source/media rights must be resolved when applicable.

## Reviewed owner path

`cooperation/editor.rs.after` checks ExecuteSource plus ReadSource under the authority callback; `checkpoints/editor.rs.after` resolves the selected HMC resource; `controller.rs.after` validates current mutable source, asks the selected viewport to stage a candidate and submits the generated source through the existing owner. No demonstrated authority bypass was found in this limited static trace; this is not a security audit. Exact correspondence, unshared geometry and source stamp checks are present, but success, integration and reuse compatibility remain unexecuted.

## Next action

Keep candidate staged until cancellation design is corrected and reviewed; then compile and run the focused tests and composed native scenarios under the coordinator lease. Preserve prior source/manifest and report real command exit, disk use and source pins. Do not claim rig, cooperation, or species/package completion from source or unit fixtures.

## Follow-up review: cancellation-aware staged revision

The earlier always-false-token finding above is superseded for the updated staged files: `job_actions.rs.after:121-124` passes its token to `rig_authoring_edit`; the staged session/shared/package/checkpoint/authorized decorators forward it; `checkpoints/inspection.rs.after:110` checks it around HMC reading; `viewport/renderer.rs.after:116-118` passes it into viewport admission; and `controller.rs.after:163-165` checks before source submit. A source fixture at `rig_binding/tests.rs.after:481-504` injects cancellation during staging, but it is unexecuted.

This is the worker closing token, not proof of per-request cancellation: `C:/github/poet/native/src/graphical/jobs.rs:163,188-200` checks the reply receiver cancellation only before starting, while the flag forwarded during work is worker_closing. A requester abandoning an already-started operation is not connected to this token. Cancellation concurrent with or after the final check also has no atomic guarantee against commit; the commit outcome must be reported honestly. Qualify cancellation by timing and source-preservation evidence rather than calling all mid-flight cancels handled.

No composed native signed-session/HMC/viewport cancellation test or measured peak memory check was found in this staged revision. Whole-HMC loading, decoded mesh and rig working-set caveats in the earlier review remain. No compilation or execution was performed during this follow-up.
