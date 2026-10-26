# RUN-06B independent source-journal review

**2026-10-24 — applied journal and frozen store addendum reviewed.**
The bounded source journal has no newly identified blocker within its declared
revision-checked API. Native UI integration also needs replacement-safe source
stamps: a revision alone does not fence the preserved legacy replacement paths.
The authorized addendum implements that contract and is approved for exact
guarded apply. Native UI wiring and current whole-core qualification remain
separate. No source writes or builds were performed by this reviewer.

## Inspected source and evidence

Read [the worker's scope/evidence](native-3d/README.md),
[source inventory](native-3d/source-evidence.json) and
[verification receipt](native-3d/verification.json), then the actual canonical
four files. The staging receipt's “not applied” wording predates the supervisor's
apply. Independent actual-file SHA-256 checks match every candidate:

| Actual file under `qualia-core-db/src/poet_host/invoke/` | SHA-256 |
|---|---|
| `studio_store.rs` | `6d1f8e945b352550a9a810330f54dc2758127346b2abdec797f1461ecfb49664` |
| `studio_domains.rs` | `f629a7c4885e814a723b33bc8711ef417bdd00cc64fecb4332570c1b765f2caf` |
| `studio_journal.rs` | `af24a575f039c7b87e6cb88693b264aed92141dda3455f30ec4aa19aeb192da8` |
| `studio_journal_tests.rs` | `dc8f48fa09add4b2d84a6b104eba1d99706f6c2460ce645bfa97b2391185c8c8` |

The recorded isolated harness has ten focused journal passes. Its full mounted
module run has sixteen passes and one preexisting CML importer failure, also
reproduced against the original module. These are exact candidate modules linked
to pinned cached full-core/Vibe artifacts; they do not establish freshness of
the current whole-core consumer. The supervisor/session worker owns canonical
checks and the current core build; no duplicate build was started here.

## Journal correctness assessment

`studio_store.rs:325`, `:374` and `:383` take the existing store mutex before
edit, restore or limit configuration. `journal::admit` checks the supplied
revision before candidate work; stale callback execution is therefore avoided.
`None` deliberately retains the legacy no-conflict-protection API. Native checked
operations advance the positive 29-bit revision and refuse exhaustion rather
than wrap it. Undo/redo restore source and advance the clock instead of rewinding
it.

The detached candidate in `studio_journal.rs:125` permits only title, nodes and
generic properties to change. Before activation it compares `id`, `did_q42`,
domain, routing lane, sensitivity, context, clock, history and public redo markers
against the stored artifact. `SourceState` snapshots contain only editable source;
undo does not restore an old typed authority envelope or erase audit history.
Properties named `source_author` or `subject_identifier` remain editable source
strings, not authoritative contribution or identifier-association records.

Admission checks precede stack/source/history activation. Edit budgets its final
undo stack before clearing redo; restore replaces one retained snapshot with the
current source and budgets the resulting combined retained bytes. Snapshot count
is unchanged by undo/redo. Configuration checks all current/retained sources,
combined stack count/bytes and its own prospective audit entry before changing
limits. It never silently discards state to meet a smaller ceiling. The audit
boundary regression checks refusal preserves a pending redo.

The charge includes UTF-8 source bytes and a declared per-field allowance. It is
not allocator accounting or process isolation; current source, history, journal
and trusted callback transient allocations must not be presented as one measured
hard memory ceiling. Callbacks execute under the store mutex and remain trusted
native code, rather than isolated jobs with external rollback.

The script handlers parse supplied expected revisions through canonical argument
helpers, reject invalid/out-of-range supplied values and call the checked store
paths. An integral F64 is accepted by the existing helper; fractional/non-finite,
negative, zero and out-of-range revisions do not become valid positive 29-bit
revisions. Omitted revisions preserve compatibility and do not prove stale
command protection.

## NJ-01 — replacement-safe native integration contract

**Concrete integration blocker if only the numeric revision is used:**
`studio_store.rs:303` retains unchecked `upsert_artifact`; `Studio.create`, import
and legacy receipt helpers call it. A client can observe revision 1, another edit
advance it, and replacement reset it to 1 before the first client's expected-1
edit. The checked edit then accepts a command from before the replacement. Even
full visible artifact equality cannot detect an identical replacement that
clears private journal state. This is a preserved legacy boundary, not evidence
that checked source edits protect every writer in the store.

The supervisor authorized, and the frozen addendum implements, this minimal
additive contract before freezing native UI integration:

- Read the artifact and an opaque process-local incarnation/revision stamp under
  the same store mutex.
- Give every insertion/replacement a fresh private incarnation. A held `Arc<()>`
  token is sufficient: stamps clone it and comparison uses `Arc::ptr_eq`, never
  derived `Arc<()>` value equality. Holding the old token prevents address-reuse
  ABA and avoids a numeric generation exhaustion/panic path.
- Compare both incarnation and revision under that mutex before edit/undo/redo
  candidate work. Return the new stamped snapshot after accepted changes.
- Create native documents atomically only when absent; existing documents must
  not be initialized by unchecked replacement. Native UI edits use stamped APIs.
- Existing mutation callbacks return unit. An additive fallible callback runs
  against the detached candidate and propagates `Err` before any source,
  history, revision or undo/redo activation. This promises local admission only.

`StoreStamp` fields are private, its only public getter exposes the revision,
and equality explicitly uses pointer identity plus revision. Stamped reads,
comparisons, candidate work and returned snapshots share the original mutex;
there is no compare-then-reacquire race. Every store insertion/replacement
constructs a fresh incarnation, while ordinary checked operations retain it.
The public `StudioArtifact` shape remains unchanged.

The new creation path checks existence before insertion and validates source,
audit and positive 29-bit revision admission. Review also identified supplied
public redo markers with no corresponding private snapshots; the final candidate
rejects nonempty initial `redo_stack` before insertion. That case is included in
the initial-admission test. The fallible path returns the original callback
error before typed metadata comparison or any source/journal activation; the
existing unit callback wraps this path with `Ok(())`.

## Frozen addendum disposition

**Approved for exact guarded apply after the supervisor's current core build
exits.** Independent checks found four candidate targets, zero candidate hash
mismatches, zero changed original targets and zero unexpectedly existing new
targets. The initial applied journal hashes above are the addendum's expected
baseline, not a claim that the addendum was already applied at review time.

| Pinned addendum artifact | SHA-256 |
|---|---|
| [Apply manifest](native-poet/store-addendum/apply-manifest.json) | `8af167658446f3235b7c6875ea43189e1e905a00669c170c9c22920a1531d2bc` |
| [Verification receipt](native-poet/store-addendum/verification.json) | `1c9568243a665779dfd48c484539934ebecdf359621cb64547170089ee54c602` |
| `studio_store.rs` | `2912f4962efbc9e750c895610d14072db0cd7aa7683c3d8751a153a0a2b396a1` |
| `studio_journal.rs` | `4e9a08e599ec30a418204876d7fb63dbe871c4381d263c6be34aa6799abadc98` |
| `studio_stamped.rs` | `d61a3a24b531598127f4d5b0097ece8282870e5cf456f14dc3a3e7afbc6fa78d` |
| `studio_stamped_tests.rs` | `125948436c758d423e96c756b80b586434d1dd26bf375f026c139348c62aecbc` |

The worker's exact-source cached-dependency compile exited 0. Read execution
logs record [five stamped tests passed](native-poet/store-addendum/stamped-tests.log)
and [ten prior journal tests passed](native-poet/store-addendum/journal-regressions.log).
They cover identical-visible replacement invalidation, foreign/stale stamps,
non-destructive create conflicts, one winning concurrent callback, initial
admission and fallible rejection preserving both undo and redo paths. No new
build or test was run by this reviewer.

NJ-01 is addressed **for the new stamped paths**. Legacy numeric expected-revision
and unchecked replacement APIs remain outside that guarantee. Native UI source
must actually use stamped reads/commands and create-if-absent; adding the APIs
alone does not establish that wiring. The stamp is process-local concurrency
state, not a durable document identifier, project permission or verified receipt.

## Product and evidence boundary

The original undo defect and inline CML failure have distinct baseline evidence.
Placeholder vector/CAD/document receipts, parity flags and JSON “10d” export
envelopes remain preexisting unqualified behavior; they do not demonstrate
geometry execution, binary 10D/HMC export, signatures, rights or human acceptance.
The test-only loft/material regeneration is useful kernel/source evidence, not
an installed organic editor or native GPU preview.

Durable checkpoint/reopen, source-to-HMC editable-source retention, job/late-result
admission, contributor/reviewer authority, graphical Poet/Craft cooperation and
IP-03 C1–C8 / ECO-034/044 / TOOL-04–07/11–12 remain open. No model/package was
created or declared complete by this review.
