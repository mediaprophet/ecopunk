# RUN-07A independent native Poet integration review

**2026-10-25 — coherent native CPU/API render proof passed; GUI editing failed its first observed run.** This review owns
only this report. No source writes, builds, paid calls or publication were
performed by the reviewer. Workers own disjoint source; the supervisor owns the
coherent current-source build and native execution. Individual module checks
are distinguished from the coherent integration results below.

The final fifteen-target [integration map](native-poet/apply-manifest.json) is
frozen at SHA-256
`a572b6d83e36f1281df30daa796d1c842e62b3cc64a49815291b6e6f29b0b73b`.
Independent candidate, actual-original, archived-original and new-file absence
checks found **zero guard failures**. The concrete source findings NP-01–09 are
addressed in these frozen bytes. Exact guarded apply and the coherent actual
native test build have since completed. The actual API render proof and the
first GUI failures are distinguished below; GUI editing is not a passed result.

## Actual native CPU verification

The supervisor's [actual target test log](native-poet/actual-native-tests-python.log),
SHA-256 `47c5ada1dbb1bf61d9b44a5bae99a284f144150e8d660f960073826f69c27835`,
compiles local Aether, Qualia, Webizen render and `C:/github/poet/native`, then
runs the actual Poet unit-test binary: **36 passed, zero failed/ignored**, in
0.91 seconds after compilation. Fifteen existing format/lint/REPL cases also
pass. This is the coherent native-feature target, not the earlier cached
three-case host probe.

The twenty-one new cases were read alongside their assertions:

| Actual test suffix | Demonstrated result |
|---|---|
| `undo_then_explicit_rebuild_restores_editability_without_preview_replay` | Two accepted real geometry cells, source Undo, fresh-session rebuild to revision 1, old epoch retirement, no replay preview, and another accepted submission to revision 2. |
| `runtime_error_rebuild_keeps_source_and_retires_numeric_revision_epoch` | Invalid actual geometry call holds execution without changing accepted source; explicit recovery repeats the numeric revision under a new epoch and permits a later submission. |
| `replay_failure_preserves_prior_session_epoch_and_source` | Malformed accepted cell fails reconstruction while preserving the previous live session revision/epoch and externally accepted source stamp. |
| `identical_replacement_between_replay_and_promotion_is_rejected` | A deterministic identical-visible-content replacement changes incarnation between preparation and activation; prior session/epoch survive. |
| `recovery_aggregate_dispatch_and_instruction_limits_do_not_promote` | Two-cell replay with one aggregate kernel dispatch, then insufficient total instruction allocation, each fails without session/epoch promotion. |
| `gapped_or_noncanonical_accepted_cell_keys_cannot_open_or_rebuild` | Renaming cell 1 to key 3 prevents reopening and rebuilding, with no prior-epoch loss. |
| `actual_operation_grants_guard_dynamic_targets_at_runtime` | A computed swung-surface target with only extrusion authority receives E300 at runtime; session revision remains zero. |
| `granted_source_dispatches_real_native_geometry_kernel` | Granted canonical source invokes actual extrusion and returns nonempty native vertex/triangle streams at revision 1. |
| `actual_extrusion_session_result_uses_nested_mesh_rows` | Actual SourceSession/GeometryHost output marshals into a finite, nonempty preview mesh. |
| `malformed_result_rows_and_provider_frame_extent_fail_closed` | Ragged/mixed result streams and a `u32::MAX` provider-frame extent are refused. |
| `one_worker_one_queue_and_cancelled_queued_work_never_starts` | Blocking work runs on the owned worker; one queued slot admits, another refuses, cancelled queued work does not start. |
| `stale_session_or_replaced_incarnation_cannot_dispatch_or_install` | Wrong session context and identical-content replacement each prevent dispatch/publication. |
| `failed_host_attempt_delivers_reconciliation_snapshot_without_retry` | Failed attempt returns the held snapshot/message with no image or retry. |
| `completed_image_cannot_follow_a_new_source_or_session_snapshot` | Exact context admits the image; changed session/incarnation and unvalidated completion withhold it. |
| `publication_rechecks_actual_store_after_worker_completion` | Completion initially admissible to the UI becomes inadmissible after actual store replacement. |
| `rebuilt_epoch_rejects_old_queued_reply_and_frame_even_when_revision_repeats` | Successful rebuild installs its new epoch; old reply/frame/queued command cannot follow despite equal source/numeric revision. |
| `held_state_transition_rejects_prior_success_with_unchanged_epoch_revision_and_source` | Changing only the held flag invalidates earlier success while admitting the new held result. |
| `unsupported_profile_has_no_rebuild_dispatch` | Unsupported reconstruction returns an error without calling provider rebuild. |
| `preview_with_another_execution_epoch_is_withheld_before_encoding` | Wrong-epoch frame is refused before PNG encoding. |
| `closing_joins_started_rebuild_and_cancels_queued_command` | Close waits for started work and joins; queued work never starts. This is not a forced native-driver cancellation test. |
| `worker_panic_closes_reply_and_is_reported_when_owner_joins` | Worker panic closes the reply and the owner reports join failure. |

Read-only post-test verification found zero drift in all **46 current sources**,
seven reviewed map pins and 51 sequential transitions in
[integrated-sources.json](integrated-sources.json), SHA-256
`e50bc38d74d54122aeaba9086dd50ad49772d72d44b67dafc42b266f6a8c0f1b`.
All fifteen actual Poet targets and candidate bytes still match the frozen map.
The reviewer did not run a build or alter source/lock files.

The [first native test attempt](native-poet/actual-native-tests.log), SHA-256
`5be91c32b730a9498230883a5da0d9a468cbc216be6702c090c2ce8a3c4eec7e`,
failed in Stylo's Python code-generation build script because Python could not
be found. The supervisor reran with scoped `PYTHON3` pointing to bundled Python
3.12.14; source/lock pins are unchanged. This build-tool prerequisite does not
introduce a JavaScript runtime or per-script build.

This pass does not waive the separate pinned-0.1 baseline
`vc5_prohibition_breach_completes_under_1_microsecond` failure: the retained
[timing recheck](vibe-baseline-timing-recheck.log) measured 1500 ns against its
strict 1000 ns bound and exited 101. That performance requirement remains open.

## Intended bounded path

The staged surface uses the controlled Aether native renderer, explicitly
`vello-cpu-softbuffer`, for its window and controls. Vibe source is parsed,
checked and evaluated by canonical `SourceSession`; identified editable cells
are committed through the canonical Studio store and opaque incarnation/revision
stamps. Geometry is dispatched through the actual Qualia host/kernel. A separate
PortalGpu offscreen preview uploads the real mesh with a caller-selected
metallic/roughness material, renders, reads RGBA and supplies one native PNG
image to Aether after an explicit command.

This is a bounded native preview frame/window target. It is not a live GPU
viewport/device-sharing architecture or a complete recovered Poet tool chest.
The current material is static caller input, not authored material source. The
private HMC rendering helper is explicitly unmounted/unqualified and must not
be counted as texture, package or source-to-HMC authoring.

## Actual source/render proof and GUI follow-up

The [source smoke receipt](native-poet/source-smoke/verification.json), SHA-256
`3ef6c995d2144a8f2d839f09cccc32999d4dd4d3e3fb6da1626bc85fb3b0b875`,
binds the actual applied controller/geometry/preview modules, matching current
product dependency versions and a successful headless run. Independent review
checked all nineteen pinned artifacts and all 46 input source hashes with zero
drift. The harness substitutes its package root and removes unused dependencies;
its lock introduces no new dependency version. It calls the actual mounted
`MeshPreviewProvider`, rather than replacing the renderer.

The same process generated three actual PortalGpu frames at 256×256. The initial
and edited real meshes each contain 22 vertices and 40 triangles; changing an
axis point changes geometry coordinates and **6,927 pixels**. Invalid syntax
preserves accepted source/session/epoch; bad runtime geometry arguments return
E100 and hold execution. Explicit reconstruction retires the prior epoch without
replaying previews. Undo, explicit reconstruction and another source edit produce
the third real frame. The reviewer inspected the edited PNG: it shows a brown
faceted diagnostic surface on a dark background. Static factor material, this
single frame/profile and pixel change do not establish master quality, physical
appearance, textured HMC authoring or model acceptance. Selected adapter identity
was not independently instrumented.

The supervisor's first actual native-window observations are **failures of GUI
usability**, not a successful GUI source-editing demonstration:

- Clipboard paste after selecting the draft removed the selection but inserted
  nothing; an ordinary typed key worked.
- Accessibility inspection exposed the window title bar without application
  descendants.
- The initial draft was visually blank until the first Check repaint.
- Minimum-content layout caused overlap at the observed window size.

Source inspection confirms that the current Poet native profile selects
`vello-cpu-softbuffer`, fonts and data URI support while disabling defaults and
omitting Aether's explicit `clipboard` and `accessibility` features. The supervisor
authorized a separate UI5 follow-up for those native features, responsive layout
and an inspected first-paint remedy. The frozen proposal is reviewed below;
its application and rerun are still pending. A successful API render does not waive the observed
GUI failures; initial source visibility, paste, accessible controls, edit/submit,
actual preview and close must be demonstrated after the focused correction.

Independent source inspection confirms the initial-value cause: Blitz DOM
`layout/construct.rs::create_text_editor` seeds a multiline editor from
`node.text_content()`. Aether native DOM forwards Dioxus's controlled `value`
attribute before that editor exists; Blitz's `mutator.rs::set_attribute` only
copies `value` into an already-existing `TextInputData`. A later controlled
update can therefore populate the editor after layout. Furthermore,
`TextInputData::clone` creates an empty editor, so static template cloning needs
the same controlled-value hydration. A focused Aether adapter correction must
preserve uncontrolled textarea child-content semantics, explicit empty controlled
values and post-layout controlled updates; an app-specific first repaint is not
the remedy.

The frozen [UI5 map](native-poet/ui5-followup/apply-manifest.json) has SHA-256
`963f3e7466d84b870984f9080a40b3e9d981f623ab22cdcd846d73d1d2a6c6b3`;
its [staging receipt](native-poet/ui5-followup/staging-receipt.json) has SHA-256
`ab453355d736469fd4d01bb0d03309aa2bc47651b9f0e1ecdb46e9e2b58bf2e8`.
Independent review verified five actual/candidate guards, fourteen evidence pins
and four root-cause source inputs with zero failures. It is **approved for exact
guarded apply and focused compilation**, not GUI usability acceptance.

The five targets add explicit clipboard/accessibility features to the existing
native CPU/font/data-URI profile, update the lock, adjust flex/box sizing and
textarea height/accessibility name, and add the focused Aether controlled-value
helper with ten wiring lines. No controller/session/geometry/job behavior or
registry source is changed. The lock
`004e0f683cdc4af2da231801c622febc90392d24fa6be8cf13aaf804985ba2c8`
adds sixteen feature-closure packages and disambiguates existing Objective-C
dependency versions; the full diff does not update an existing package version.
The initial offline metadata attempt failed on uncached AccessKit. The supervisor
then fetched exact locked Windows dependencies successfully; a subsequent
offline locked selected-native tree passed. Those are resolver/cache results,
not compile or clipboard/accessibility execution results.

`textarea_value.rs`, SHA-256
`16287c4e793b33c5481159ef82946ffbbdd6197a45f74ba697175ae5fa0857ba`,
seeds only controlled textarea editors before initialization and rehydrates only
their explicit values on native deep clone. Existing initialized-editor updates
continue through the normal Blitz setter. Three staged actual-Aether tests cover
dynamic multiline/Unicode values before/after first layout plus update/empty,
explicit-empty versus uncontrolled child fallback/ordinary input, and static and
dynamic deep clones. Their execution remains required alongside the actual GUI
rerun; no test pass is inferred from source inspection or rustfmt.

The first actual Aether workspace compile then stopped before running tests:
[actual-aether-workspace-tests.log](native-poet/ui5-followup/actual-aether-workspace-tests.log)
reports two ambiguous `Atom::as_ref()` comparisons in the new helper and one
test return of `SplitString` where `String` is declared, plus an unused import.
The supervisor assigned a bounded one-file compiler correction using typed
`local_name!("textarea")` atom comparison and explicit test `.to_string()`.
The separate [compiler-fix guard](native-poet/ui5-followup/compiler-fix/guard-map.json)
is frozen at SHA-256
`871a8da692246185634b812fd74c12aefd888f8c0cd3a5bda564fc4695a42bfd`.
Independent review confirms the actual helper and archived original both match
`16287c4e793b33c5481159ef82946ffbbdd6197a45f74ba697175ae5fa0857ba`,
and the corrected candidate matches
`5ff72ba5250ee0a0cd2dc94343b00b471f41b34bcfbf14b4fe82ec01b6b95f69`.
Its exact diff contains only those two typed comparisons, explicit test string
conversion and unused-import removal. The one-file correction is **approved for
exact guarded apply**. Focused test execution remains required; the initial UI5
proposal is not claimed to have compiled or passed its tests.

## Findings and required disposition

| ID | Actual issue | Status at this report revision |
|---|---|---|
| NP-01 | Jobs validated a frame against `outcome.snapshot`, then discarded that binding and later read a different host snapshot. A new snapshot could be paired with the old image. | UI3 carries `BoundImage` context through completion and compares it with the final snapshot. Independently re-read; adversarial CPU cases passed in the coherent actual target. |
| NP-02 | Worker-only current checks plus local view/origin equality permitted a replacement between completion and UI installation. | New cheap `artifact_stamp_is_current` uses the same store's `try_lock`, pointer/revision match, no source clone or waiting. UI3 checks it immediately before publication; busy/poison/absent fail closed with non-causal “could not be revalidated” wording. Separate guarded publication map reviewed below. |
| NP-03 | A Check reply could retain an unbound old image while installing a new snapshot. Refresh also unconditionally overwrote an unsubmitted draft. | UI3 clears images on every action and error/stale completion. It preserves dirty drafts on Refresh/Undo/Redo and reports preservation. Independently re-read. |
| NP-04 | `GeometryHost` admitted both builders independently of actual CLI operation grants. A computed `capability.invoke` target bypassed the checker’s literal-target check and could invoke ungranted `swung_surface`. | Draft checks the actual target against immutable host grants before native dispatch. The canonical checker separately checks invocation-style authority. Denied dynamic-target and real positive CPU cases passed. Source requests cannot supply actual grants. |
| NP-05 | Default source calls real `extrude_profile`, whose wave27 host returns nested vertex/triangle triples. Preview marshaling accepted only the flat streams returned by wave29 `swung_surface`; the default preview would always be unavailable. | Draft explicitly validates/marshals both real result shapes, rejecting mixed/ragged triples. Actual GeometryHost→mesh positive CPU case passed. No placeholder mesh or replacement codec; GPU frame pending. |
| NP-06 | Extrusion preflight omitted the actual host’s two cap-vertex allocation allowance. PNG admission computed an unchecked extent product before validating public frame size. | Draft checked `+2` preflight is present. PNG validates side 8..1024 before its extent product, making that multiplication bounded. Coherent CPU admission regression passed. |
| NP-07 | Runtime errors and source Undo/Redo deliberately held execution, but no in-window reconciliation command existed. Numeric session revisions can also repeat after a fresh session. | Supervisor authorized a strictly GeometryHost-specific, explicit bounded rebuild with a fresh session and opaque session epoch. Contract below implemented; recovery/epoch CPU cases passed, in-window use pending. No generic Host replay or arbitrary canonical Clone. |
| NP-08 | Prefix-only stored-cell enumeration admitted gapped/noncanonical identifiers. Replaying keys 3/4 as two cells then appending revision 3 overwrote accepted history. | Draft now requires exact contiguous twenty-digit identifiers 1..N before opening/rebuilding. Malformed history is rejected without promotion; Coherent CPU regression passed. |
| NP-09 | `activate_rebuild` retired the prior epoch/promoted the candidate before a fallible fresh `snapshot()` read. A concurrent malformed replacement could return failure after activation. | Draft retains the captured artifact, constructs the complete outcome and validates source before retirement/promotion, then returns without fallible operations. Independently re-read. Subsequent external changes are handled by derived-held state/final publication checks. Stale-candidate CPU regression passed. |

The draft descriptions in the table refer to the implementation now pinned in
the integration map above; source freeze and coherent CPU regressions are complete.
GPU/window and broader native outcome qualification remain pending.

## Jobs, authority and state ownership

The inspected UI3 owns one native worker thread and one queued request via
`sync_channel(1)`/`try_send`; textarea input does not evaluate source. Host work,
source snapshots, geometry, GPU/readback and PNG encoding stay on that worker.
The launch owner joins the worker after the native event loop exits, including
unwinding. Closing/cancelled receivers abandon work that has not started;
started calls are not falsely cancelled, rolled back or automatically retried.
Panic/disconnection outcomes require reconciliation rather than success.

Worker admission checks the requested source/session context and actual store
stamp before dispatch, and rejects an action target different from its origin.
The final image must agree with the returned source stamp/session context.
UI publication additionally checks that the requesting view remains current and
revalidates the actual store using the cheap nonblocking API. Frozen UI4 extends
these checks with a live resulting epoch and a latest-context cache. The cache
is invalidated while each job runs, then replaced before sending its completion.
An earlier reply cannot publish while a queued rebuild advances the host, even
if the old local view/reply agree and numeric revisions repeat. The successful
rebuild may legitimately replace its retired origin epoch; the resulting epoch
must remain live. The cache also includes the held/reconciliation flag.

The UI4 staging receipt has SHA-256
`10050ae233bf4849565c704ddfbb5aa8e3ad95b0d89b24a6b0f8ec8348be7057`.
Its four pinned rows are jobs `2cabc295cda3947ce78b7ea7fc58814d53d602ffab5f23e390b2a043f41e0886`,
jobs tests `6135ddc755dafe9a920982ff6aa3d461d51b768547c860ec2f211bb336782302`,
UI `b2bb9d0e2f070f99bdbffec642661c9aa58aba229a6fb9565e1ed1a05adfc81c`,
and module `577ad23770a7ee49da94025bc7b203b093db367b6a66db811c31309f56777765`.
Eleven jobs CPU cases passed in the actual target, including delayed completions, repeated numeric
revisions across epochs, held-flag changes, wrong frame epochs, unsupported
rebuild, panic and close/join. They ran against the coherent actual native target;
the whole frozen map additionally includes six real-host controller recovery
cases, two preview admission/marshaling cases and two host-authority cases
(twenty-one CPU cases total). The earlier cached isolated three-case host probe
remains narrower historical evidence.

The canonical controller validates stored cell ordering when opened, creates a
fresh session and holds existing source until explicit geometry-only recovery.
It never automatically executes old source to reopen a general Host. A submission
can advance its execution session before its stamped
source commit loses a race; that outcome is held, preserves the draft and does
not pretend external rollback or retry. The source store is authoritative for
editable cells, while the live session is explicitly an execution projection.
The window is not a second project/rights database.

`GeometryHost` denies graph reads/writes, pulse publication and invocation IDs
outside the two selected builders. Its final runtime operation-grant check must
also enforce the caller’s admitted subset, not merely this maximum profile.
Default/local receipt values and generic capability-resolution records do not
establish authority, actual exports or human acceptance.

Started GPU/driver work has no implemented forced cancellation/deadline contract.
Joining an owned worker is better lifecycle accounting, but it does not prove
that a hung native driver cannot delay close. This remains an explicit C8/job
limitation, not an implemented bounded-deadline claim.

## Explicit narrow reconciliation contract

Source inspection establishes the selected builders are read-only computations:
`PoetSnapshot::capability_invoke` dispatches directly; the selected branches call
wave27/wave29 geometry functions without mutating the snapshot. Those handlers
allocate local buffers and call `sweep_extrusion` math/output-buffer kernels.
They do not write the Studio store, graph, pulse, files or external jobs. This
evidence permits a narrowly qualified explicit rebuild, not general effect replay.

Required contract before declaring the source editor usable:

- The user explicitly requests reconciliation; it is not an automatic retry or
  resubmission after error/undo.
- Capture the exact accepted source history and current opaque store stamp.
  Keep actual grants, session limits, profile and installed language/kernel
  version unchanged.
- Construct a fresh canonical SourceSession through its public constructor.
  Re-evaluate only those texts under the deny-all-except-two read-only host.
  Do not render/publish intermediate frames or mutate accepted source while
  rebuilding.
- Bound accepted cell count, aggregate source bytes, per-cell ticks and the
  total native kernel dispatch quota before invocation. A new per-cell Vibe
  budget alone does not meter expensive kernel work.
- Recheck the source incarnation/revision after rebuilding. Promote the fresh
  session and a fresh opaque epoch only after full success; on failure or stale
  source keep the previous session/epoch and remain held.
- Carry epoch through request, result, frame and final UI publication checks.
  Queued/completed work from a prior epoch must fail even when numeric revisions
  and source stamp happen to match.

Draft recovery caps are 64 accepted cells, 1 MiB aggregate source text, one
million total instruction steps partitioned among cells, two million aggregate
estimated geometry vertices and 256 native dispatches. Aggregate geometry
counters live on the fresh profile host across all cells; per-submission reset
is not called during rebuild. All selected native invocations decrement/check
these counters before entering the kernel. Profile grants/limits are private
factory values and rebuilding uses the same installed code; no intermediate
preview producer is called.

CPU tests of successful recovery, failure preservation, stale source and epoch
rejection passed in the coherent actual native build. Durable/shared
canonical checkpoints require a separate contract and are not supplied by this
profile-specific rebuild.

## Publication prerequisite and dependency pins

The frozen [store-publication map](native-poet/store-publication/apply-manifest.json)
has SHA-256 `9ee9320f9c310b9ba44cd21e3676627ea7445b008426ea776e2f4c9ac4419cfe`.
Independent source/candidate checks found two targets and zero guard failures.
The implementation is a read-only, nonblocking stamp comparison plus re-export;
it was applied and compiled in the coherent actual native test target. Current
source-byte verification includes both publication targets.

The proposed actual Poet workspace aligns browser binding pins to versions
already selected by Ecopunk: wasm-bindgen `0.2.129`, wasm-bindgen-futures
`0.4.79`, js-sys/web-sys `0.3.106`. The previous Poet exact `0.2.125` family
conflicts with the native Etalon dependency closure. This is a targeted resolver
prerequisite, not new JavaScript business logic or evidence of a working WASM
application. The frozen lock has SHA-256
`f1c6d2ddfd33ec813815b3cebdea120b90afc97cbbdc764742d64ec0d6c1b627`;
the manifest diff adds the optional native surface and Etalon patch, leaves
legacy CLI routes intact and pins the matching WASM test family. The resolved
closure also includes ordinary transitive version changes; it is not presented
as a lock containing only four browser-binding changes. Native-specific tree
evidence selects controlled local Aether and Etalon commit `ed0b671d`, with
canonical local Vibe/Qualia/Webizen render dependencies. Resolution/tree evidence
is declarations-only; the actual native CPU target compilation/execution is
recorded separately above. Browser target
execution and native no-JS/V8 dependency qualification remain distinct claims.

## Remaining product boundary

Material source authoring, verified texture role/colour-space admission,
source-to-binary-10D/HMC retention, durable reopen, project/contributor/reviewer
authority, graphics/device coverage, high-grade master acceptance and full
IP-03 C1–C8 remain open. No species/thing model or film/game/cartoon package is
created or declared complete by this diagnostic integration review.
