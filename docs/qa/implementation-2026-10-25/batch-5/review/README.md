# RUN-10 independent source review

Status: bounded independent source review complete. No unresolved blocking source
finding remains in the captured enrollment, source workflow, native entry,
session switching and UI/job scope after the corrections below. This is not a
complete production/product verdict.
This is a source review, not executed verification or human acceptance. This
reviewer has run no Cargo build/test, GPU/GUI session, external provider, commit
or publication and has not edited QualiaDB or Poet.

The [reviewed input manifest](reviewed-inputs.json) records 53 source/dependency
inputs. All 53 match the supervisor's frozen 166-input snapshot
`2e7ab1b867bfa3ac4e5ed49f8dd2d99e46e27113a77fdf7a0752cb9a3263f1bc`,
with zero mismatches. Changes after these digests require targeted renewed
review. [capture-inputs.ps1](capture-inputs.ps1) records the reproducible bounded
capture/comparison; it does not build or execute product code.
The [initial reviewed manifest](reviewed-inputs.initial.json) preserves the
earlier `e86737d8f3217ba46801461edf7edb316bf310cfe584caba6360a2ed7547ff0a`
review. Exactly three inspected inputs changed for the final targeted follow-up:
Poet `ui.rs`, `draft_buffer.rs`, and core enrollment `tests.rs`.
The [C5 reviewed manifest](reviewed-inputs.c5.json) preserves that intermediate
review. The subsequent Windows installation follow-up adds the actual native
checkpoint `native_io.rs` and `tests.rs` to the inspected inventory; all preceding
51 reviewed inputs remain unchanged.

## Findings and disposition

| ID | Priority | Finding | Disposition |
| --- | --- | --- | --- |
| B5-R01 | P1 for production signer qualification | The original `NativeSigner::from_vault` accepted any unlocked vault, including the deterministic `KeyVault::new` fixture. Selecting the strict OS opener only by caller convention left the public constructor unable to distinguish that fixture. | Corrected in saved source: a private, nonserialized `native_secure` provenance bit is set only by the concrete Windows OS-store opener; the signer requires it. The deterministic-vault rejection test is source evidence only. |
| B5-R02 | Integration | During concurrent initial saves, `DurableState` initializers did not yet include the new request/enrollment vectors. | Reported promptly to the supervisor and owning workers; source integration correction observed. Successful compilation remains supervisor-owned. |
| B5-R03 | P1 | OS credential creation originally used a read-NoEntry then set-password sequence without a cross-process lock. Two explicit create windows could both succeed with different keys and overwrite the first retained credential. | Corrected in saved source: absolute canonical profile binding and a held stable OS lock serialize read/create/readback; opened lock metadata and Windows reparse-point checks fail closed. Execution remains supervisor-owned. |
| B5-R04 | P1 | The existing job publication guard required the origin view to remain publication-current after a successful command. Actual source/epoch-changing submission or undo retires that origin, so the UI could reject its own current successor and remain stale. Mock tests did not retire the old source/epoch. | Corrected in saved source: validate current actor authority and exact origin placement, then the actual current successor and worker/closing state. A real canonical editor lifecycle regression now changes source and retires the original epoch. Execution remains supervisor-owned. |
| B5-R05 | P2 | Private draft retention initially relied only on a deferred UI effect. An actor switch unmounts that component, creating an avoidable final-input/effect ordering risk. | Corrected in saved source: synchronous buffering on text input, source replacement and material draft generation. No GUI failure or executed usability pass is claimed. |
| B5-R06 | P1 | Recreating an authorized frontend on each dynamic-session read gave each snapshot a different authority-owner Arc, defeating exact context equality even for one unchanged session. | Supervisor identified and corrected: cache the derived frontend per opaque session handle while continuing native authorization on every method. The initial view is read from the same host passed to its worker. |
| B5-R07 | Integration | Sealed signer helpers initially imported `sanctuary_audit` without its required feature; native check reported E0432. Initial backend helper visibility also needed correction for sibling native-entry modules. | Corrected in captured source: sealed APIs and their fixtures are gated by `sanctuary-crypto`, the actual native dependency enables that capability, and helpers have explicit module visibility. The OS-backed production constructor has no sealing or plaintext fallback. Successful supervisor checks are separate evidence. |
| B5-R08 | P2 | After another contributor's edit, a returning reviewer's retained draft could differ from accepted source, disabling review actions. Refresh preserved that text and offered no direct control to load current accepted Vibe text. | Corrected in captured source: an accepted-source preview and explicit replacement confirmation bind the current exact UI context; a current source/session check precedes synchronous private buffer/signal replacement. Shared source and review records are not changed. A real canonical-source helper regression preserves text without confirmation and rejects a stale source. |
| B5-R09 | Verification | The first executed full core suite found that the fresh-process fixture selected the public `poet_host` alias, while libtest names use the declared `vibe_host` module. The child ran zero tests; the marker assertion correctly made the parent fail. | Corrected in captured test source only: derive the selector from `module_path!()` without its crate prefix, retaining exit/marker checks and adding explicit one-test/one-pass output checks. The original failure remains recorded; corrected execution is supervisor-owned. |
| B5-R10 | P2 resilience / verification | The C5 full core rerun failed a checkpoint replacement with Windows OS error 5. Its cause is unestablished; repeated original-binary passes do not identify a cause or negate the failed full run. | Captured source adds bounded retry of the atomic installation primitive for Windows errors 5/32/33 only. The same staged file and writer lock survive attempts; each attempt repeats the expected-digest guard. Real deny-delete tests and the final native-boundary full suite pass on the captured source; no particular cause of the original OS5 is inferred. |

## Enrollment and key ownership

Enrollment requests prove controller-key possession with a contextual signature
binding a fresh OS nonce, project, current signed-admission digest and sequence,
candidate capacity/qualified identifiers/membership and validity. They cannot
request steward or review authority. Approval needs a current native steward
session, explicit decision evidence and the actual signer matching the pinned
project authority. A Human capacity additionally requires an explicit operator
verification reference; that reference does not prove legal identity. Review
grants are a separate signed act. Admission changes invalidate retained sessions,
pending challenges and permissions. A fresh process must prove possession again.

The production vault constructs the concrete Windows credential store, bypassing
replaceable global/mock defaults, requires local persistence, and fails closed on
read/write/format errors. It has no plaintext/memory-provider fallback. The signer
has no Debug/Serialize implementation, keeps its seed zeroizing, and purpose
separates derivation and proof contexts. Separately sealed software fixtures use
authenticated wrapping and reject corruption, wrong wrapping material, wrong
length and plaintext; these fixtures are not OS-store or human enrollment proof.
Their existing native crypto capability is explicitly selected; a profile
without it does not acquire an improvised wrapping algorithm.

The authority's retained roster is trusted native host metadata available before
login to bootstrap project binding. It is not an authenticated partition-rights
gate and must not be exposed as a transport authorization mechanism.

## Cooperative source workflow

The inspected service uses the same opaque session slot, authority owner and raw
shared canonical editor. It inspects actual current source/stamp under native
admission; the core commit repeats current-session, exact-source target and
permission checks under the authority mutex. Admitted native edits use that same
mutex. Arbitrary direct Studio-store mutations are outside this coordinated owner
boundary; they do not acquire admission by changing source bytes.

Review requests are limited to the exact admitted source's author. Decisions need
a current explicit document review grant and a distinct reviewer subject. Request
decision identifiers are retained atomically with the canonical source decision.
Source/history projections and the undo predecessor contain actual retained
source; no historical revision number is invented for that predecessor. Displayed
evidence is guarded by source stamp, execution/session context and authority.

Effort and actual asserted work status remain separate from authorship, role,
authority, legal agreement and payment. Source decisions explicitly exclude whole
HMC/resources, rights, complete model and high-grade master acceptance. Package
write evidence binds its own SHA-256 and actor; it is not acceptance.

## Native entry and actor switching

`poet studio --native-project EXISTING_DIRECTORY` now routes from the actual
native entry to the Aether project shell. The argument locates bounded public
configuration only; it does not adopt a key or grant a role. Creation and
fingerprint adoption are explicit native gestures. Login opens only the strict
OS profile, checks the actual public controller key, proves a fresh challenge,
and then reopens HMC through authenticated native admission. Initial project
creation saves an actual empty editable working HMC with its own write evidence;
it does not generate or accept a master model.

The runtime retains one canonical shared editor/execution service. Each session
gets a new worker/Studio lease, and the old worker is joined before another actor
starts. The shared busy flag coordinates shell and editor gestures. Private
unsubmitted text is retained in an actor-specific in-memory buffer; the keyed
Studio remount discards old actor workflow form state and displayed evidence.
Returning reviewers can explicitly preview and load the accepted source into
their private draft. Confirmation is tied to the displayed source/session
context and resets after replacement; a changed context requires confirmation
again. Refresh alone continues preserving unsubmitted local text.
The cached dynamic frontend holds no independent principal or grants. Every
operation still enters current native authority before shared editor access.
Steward revocation is held for a separate recovery/rotation procedure.

Test-source inspection includes the real canonical editor source/undo/rebuild
lifecycle and an actual signed software-actor `SessionEditorHost` worker case.
The latter checks stable repeated-snapshot context, exact successor publication
and withholding after a distinct reviewer handoff. Enrollment fixtures cover
signed forged/stale/replayed requests, explicit grant separation, revocation,
sealed-key reopen and a fresh child-process possession challenge. None of these
test sources creates a human review or substitutes for executed results.

Core unit-test compilation now explicitly refuses OS credential APIs; downstream
Poet tests link a normal core library, so that guard is not a universal
integration-test isolation guarantee. The inspected downstream tests use public
metadata and software authority fixtures, without native profile creation or
OS unlock. This report makes no retrospective assertion about earlier suites'
credential effects and no credential was purged by this reviewer.

## Retained limits

The protected host-directory assumption from batch 4 remains: hash chaining does
not prevent a privileged writer from truncating a tail or rehashing unsigned
evidence. No externally protected latest head is claimed. File sync does not
qualify Windows directory-entry survival after machine power loss. Symlink and
canonical-directory checks are bounded protections within this trusted-storage
boundary, not hostile concurrent-filesystem isolation.

Native graphical contributor/reviewer usability, OS credential execution,
fresh-process HMC recovery and source-to-output production remain distinct
supervisor evidence. Film, open-world game and cartoon packages, exhaustive model
claims, accepted high-grade masters and IP-03 C1–C8 remain open.

## Targeted final follow-up evidence

Only the three changed inputs above were reinspected; unchanged source review
was reused. The supervisor's [native draft-control check receipt](../native-check-draft-replacement.json)
records `cargo check --tests` exit 0 with unchanged source at snapshot `c20db801...`.
This compiles the new control/helper test source and does not execute it.

The [initial full-core receipt](../initial-results/core-final.json) records exit
101 against the earlier `e86737d8...` inputs. The supervisor reported 9,851 passed,
one failed and three ignored; the failing fresh-process selection was an actual
failed verification, not a silently passing empty child. The exact [before](../enrollment/fresh-process-selector-before.json)
and [after](../enrollment/fresh-process-selector-after.json) test-source receipts
bind that narrow correction. The current review makes no corrected test-pass
claim from that receipt alone. The later [corrected exact fresh-process receipt](../core-fresh-process-corrected.json)
records an actual exit-0 exact test against the final unchanged `2e7ab1b8...`
inputs. Its selector, one-test/one-pass and fresh-child marker checks are retained.

## Bounded Windows installation follow-up

Only native checkpoint `native_io.rs` and `tests.rs` were added to this targeted
review. The [C5 failed full-core receipt](../checkpoint-failure/core-final.json)
and its log remain preserved: 9,851 passed, one failed and three ignored, with
the checkpoint atomic installation returning OS error 5. No antivirus,
particular external process or own-handle leak is asserted as the cause.

The production retry wraps only the existing persist operation, retaining its
staged file and the calling save's OS writer lock. It never reevaluates authored
source, repeats contribution/audit effects, deletes a destination or truncates
existing bytes. Only Windows 5/32/33 errors can retry; all other failures return
immediately. Six bounded pauses and a 315 ms admission/wait deadline bound the
retry policy. Synchronous filesystem IO is explicitly not timed out by that
deadline. Expected-file validation repeats before every attempt; a changed
digest refuses replacement. Final errors retain first/last OS errors and
destination/stage context, and RAII cleans failed staging.

The three new tests operate on actual Windows deny-delete handles at the shared
installation primitive. The private local observer releases or mutates the
destination only after a real failed persist; production passes a no-op. Test
source covers release followed by successful retry, persistent refusal with old
bytes retained, and an external byte change that stops retry. The final native
full-core run below passes these tests on the same captured source.

## Final supervisor execution follow-up

The reviewer inspected the completed receipts and result logs; the supervisor
owned all execution. Each current receipt below binds unchanged final snapshot
`2e7ab1b867bfa3ac4e5ed49f8dd2d99e46e27113a77fdf7a0752cb9a3263f1bc`.
The final read-only hash check found zero mismatches across both the 53 reviewed
inputs and the full 166-source snapshot; the snapshot file's own SHA-256 also
matches that identifier.

| Evidence | Observed result and qualification |
| --- | --- |
| [Full core receipt](../core-final.json), [log](../core-final.log) | Native execution boundary: 9,855 passed, zero failed, three ignored; 497.24 seconds. The three ignored tests are not claimed passed. |
| [Fresh-process enrollment receipt](../core-fresh-process-corrected.json) | One exact parent test passed with its checked one-test child and possession marker; software fixtures, not OS-store or human enrollment. |
| [Native Poet receipt](../native-final.json), [log](../native-final.log) | Library: 77 passed, zero failed, one ignored; CLI tests: 15 passed, zero failed. This is headless test execution, not contributor/reviewer GUI acceptance. |
| [Native library build](../library-final.json) | Exit 0, actual native-feature library build. |
| [WASM library check](../wasm-final.json) | Exit 0 for `wasm32-unknown-unknown` compilation check; no WASM runtime/assisted-operation acceptance inferred. |
| [Direct CLI public entry](../direct-cli/native-project-check.json) | Exit 0 for the actual public-configuration branch, with unchanged sources and an empty-directory witness. No adoption, OS credential access, source/HMC activation or GUI launch claimed. |
| [GPU receipt](../gpu-final.json), [log](../gpu-final.log) | One exact native material-pixel test passed in 10.77 seconds. At 64×64, observed changed-pixel counts are base colour 295, roughness 211, metallic 295 and emissive 295. This qualifies that factor/SDR working-preview case only. |

The earlier [network-restricted full-core failure](../network-restricted-results/core-final.json)
and its log remain preserved. They include explicit socket OS error 10013 in a
two-process WSS child. The [sandbox exact-test proof](../network-sandbox-proof.json)
and [native exact-test proof](../network-native-proof.json) bind the same binary
SHA-256, arguments, final source snapshot and separate supervisor-selected
execution boundaries. Their retained log hashes match the actual proof logs.
The exact application datagram test fails in the restricted boundary and passes
in the native boundary. The subsequent unchanged-source native full run passes
all executed tests, with the 21 earlier network-restricted failures absent. This supports
qualification in the recorded native boundary; it does not establish the
individual cause of every earlier network failure or the original checkpoint
OS5 failure. No source changes, test removal or failure suppression were used
for that comparison.

Closeout script review also corrected empty-GPU-test acceptance and missing
current/live-source checks in direct CLI and record refresh. These are evidence
guards in the saved QA scripts, not an installed production acceptance gate.
The runtime, human acceptance, complete-HMC, master-quality and IP-03 limits
above remain open after these successful bounded checks.
