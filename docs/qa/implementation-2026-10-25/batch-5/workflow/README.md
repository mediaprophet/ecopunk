# RUN10 native cooperative source workflow — saved source handoff

The canonical native authority now retains bounded source-review requests and
their exact `DocumentRevision` targets. New contribution and requested-review
entry points compare the caller's complete source target inside the current
native authority lock. Review decisions require a distinct currently admitted
subject with a current explicit document review grant. Request decision IDs
reference the canonical source decision records. Existing source-review APIs
remain compatible. Journal schema additions use defaults for earlier journals.

Poet's concrete `WorkflowService` reads the native shell's one opaque session
slot and the existing shared canonical editor. It verifies current authored
source against the durable native revision, then exposes current capacity,
actual editable source, retained source history, contributions, requests, source
decisions and separately labelled package-write evidence. It creates no local
principal roster, permission cache or second source/session owner. All admitted
native source edits serialize through that same authority owner; arbitrary raw
Studio writers are outside this qualified cooperation path.

The native `CooperativeControls` submits explicit jobs through the existing
authoring worker. It provides actual effort/status fields, an explicit current
source review request, full source inspection, an actual undo predecessor
comparison and three source decision controls. Dirty drafts disable record
actions and remain preserved. Source/session/authority changes invalidate
inspection and late completions. Root-owned session UI remounting isolates
actor drafts and form state during handoff; wiring and its executed status are
recorded in the parent batch evidence.

The predecessor helper reads the existing bounded undo journal under the exact
current `StoreStamp`. Its original revision number is not stored; the UI labels
it an undo predecessor rather than inventing a historical revision. Comparison
is linear, trims common prefix/suffix lines, and caps display at 128 KiB.

Whole-HMC, rights, complete-model and high-grade-master acceptance are explicitly
unavailable through source decision controls. A package-write digest is shown as
write audit only. Whole-package review still requires a separate canonical
qualified scope and exact package SHA-256. Work-status evidence is retained as
an assertion; no funds, valuation, payment verification, waiver or agreement
adoption occurs.

## Saved evidence and checks

- [Initial core save](core-save-receipt.json), [workflow/store/control save](workflow-save-receipt.json),
  [native integration test save](native-test-save-receipt.json) and
  [test owner-lifetime correction](native-test-lifetime-fix.json), plus the
  [native operator-wrapper regression extension](operator-test-save-receipt.json), record exact
  before/after SHA-256 transitions and source-only status.
- [Saved inputs](saved-inputs.json) pins owned source inputs after saving. Parent
  wiring files have their own ownership and evidence.
- `rustfmt --edition 2021 --config skip_children=true` parsed and formatted the
  staged new implementation/test modules. Existing Studio files retain their
  prior formatting apart from the narrow helper/reexport additions.
- New canonical tests cover self-review denial, stale targets, actual software
  reviewer capacity, disconnect denial and journal evidence retention. The
  predecessor test checks real undo contents and stale-stamp denial.
- New native integration test exercises actual Vibe geometry submission,
  resource-preserving HMC save, source contribution/request, separately admitted
  software reviewer handoff through the shared session slot, actual source
  inspection/diff/history, stale/self-review denial, source decision, disconnect
  denial and native journal-owner reopen retaining all evidence and package
  digest. Editable-master and opaque resource bytes/metadata are checked through
  the real HMC codec. Existing process-local Studio state is retained in this
  regression: it does not establish a fresh-process source recovery case.
- The native regression also calls the real `SessionEditorHost` used by the
  native UI: source check/submission/read/save delegate through the shared
  project and current session slot; workflow inspection changes principal after
  actual reviewer authentication and old author publication becomes stale.
  Separate author/reviewer `DraftBuffer`s retain private text without putting it
  in admitted source; a clone retains the original author's text. This is a
  headless buffer/wrapper case, not GUI or profile-map acceptance.
- [The worker publication regression save](publication-test-save-receipt.json)
  adds one real `SessionEditorHost`/`jobs::run` submission case. It checks stable
  snapshot context, publishes the exact successful source successor while the
  origin stamp is retired, then rejects the old completion after actual signed
  reviewer authentication. It polls the public future with `Waker::noop` for at
  most 200 five-millisecond waits and joins the real worker. Execution remains
  pending parent verification.

Tests were saved but not executed by this worker; root coordinates compilation
and focused/full verification. No GUI/Computer Use, GPU launch, real human
enrollment/review, adoption, commit, publication, provider call or money movement
was performed. Native graphical usability and IP-03 C1–C8 remain unqualified;
all 120 production packets remain open.
