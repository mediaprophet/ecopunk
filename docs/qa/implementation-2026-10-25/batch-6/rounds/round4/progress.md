# Round 4 — native producer/editing and package review

Later source changes are recorded in [Round 5](../round5/progress.md); this report
and its verification receipt describe the Round 4 snapshot at its verification time.

2026-10-25. **Saved and verified within the scope below. The stack is not ready
for reference production. Asset production remains on hold.** All FND-01–06 and
IP-03 C1–C8 gates remain open.

## What changed

Poet has native controls to create an explicit developmental M64 v3 elevation
grid inside the selected HMC, inspect it, edit spacing/origin/one sample, and save
through the existing authorized native owner. Canonical codecs produce the bytes;
source/package/resource digest guards prevent stale replacement. The bounded
constant-grid UI is a first operator, not a master-quality ceiling. Arbitrary
terrain sculpting, model geometry/rig tools and complete `.10d` semantic editing
are outside this implementation.

Source declarations persist as bounded **unverified** metadata with the original
payload digest. Synthetic edits retain that origin. Selected opaque, provided,
unknown or observational metadata refuses semantic editing until its derivation
contract exists. Unrelated resources/metadata survive; legacy/richer inputs are
not silently downgraded to this profile.

The saved-package panel records contributions (including paid/unpaid/unknown work
status), review requests and decisions for the exact saved HMC/source/producer.
Native steward controls grant/revoke separate package-review authority through
the existing protected signer route. Source-review authority confers no package
review grant. Known predecessor writers and source contributors remain excluded
from self-review across retained overwrite history and declared package aliases.
Historical contributions are separately labelled; old Accepted decisions cannot
appear as decisions on a successor package.

Publication checks invalidate results after a same-source resource-only native
save. Record commits retain the raw editor guard through durable native journal
append. External file corruption is refused on refresh/consume; continuous
monitoring of arbitrary external writes is not claimed.

## Saved implementation and review

Seventy distinct targets were saved with before/after hashes and retained
preimages: 29 canonical cooperative/native review targets, 39 integrated Poet
targets, one contract and one precise lock update. See the
[application receipt](applied-source.json) and [manifest](install-manifest.json).
Follow-on [UI-key](repairs/ui-keys/applied-source.json),
[test-fixture](repairs/history-fixture/applied-source.json) and
[UI-text](repairs/ui-text/applied-source.json) repairs have chained receipts and
modify existing targets. The lock adds local geospatial without changing prior
registry/Git entries. No commits or publication were made.

Capable workers owned Rust implementation and independent review; the lower-cost
worker owned inventory/evidence. Root reviewed/applied exact sibling targets and
ran Cargo serially. Reviews identified and repaired contributor-history loss,
unrelated decision display, lock lifetime, stale package publication and unsafe/
missing source metadata. The findings and rechecks remain under
[producer/review](producer/review/) and [review](review/).

## Executed evidence

| Saved-source check | Result | Receipt |
|---|---|---|
| Native library and test-target compiler check | Passed | [check-result.json](check-result.json) |
| Canonical package policy/native authority | 19 passed | [public-result.json](public-result.json) |
| Native producer/edit, refusal, preservation and fresh-child reopen | 9 passed | [producer-result.json](producer-result.json) |
| Actual HMC contributor → separate reviewer → durable reopen → successor | 1 passed | [workflow-result.json](workflow-result.json) |
| Existing source-review/revoke/reopen workflow | 1 passed | [source-regression-result.json](source-regression-result.json) |
| Existing native typed inspection/freshness | 6 passed | [inspect-regression-result.json](inspect-regression-result.json) |
| Regular native Poet binary | Passed | [build-result.json](build-result.json) |
| Actual Core/geospatial/Portal SDK WASM compilation | Passed; runtime untested | [wasm-result.json](wasm-result.json) |

**36 distinct selected native cases passed, zero failed/ignored in qualifying
runs:** 29 cases for this scope and seven retained regressions. Earlier Round 3's
63 are not added. Software actors and temporary synthetic HMCs establish executed
behavior, not human acceptance. Three private journal cases are authored but
unrun. This is not whole-suite or CI-enforced completion qualification.

The first public run had 11 passes and one failed assertion; seven canonical
cases were not reached. Its test accidentally constructed a duplicate grant,
causing rejection before the intended predecessor check. The
[test-only repair](review/fixture-repair/README.md) retains signed old grants,
adds a structural-admission positive control and keeps the strict predecessor
assertion. All 19 then passed. The [failed receipt](public-before-fixture-repair-result.json)
and log remain. Ineffective UI keys were repaired before qualifying tests;
subsequent label cleanup changes text only and is covered by the final build.

Commands are locked/offline, debug=0, incremental=false, with uniquely owned
workspace TEMP/TMP and serial host cases. Receipts pin commands, logs, sources and
actual selected names. No GUI/Computer Use, user credentials, real admission or
grant, adopted agreement or payments were used. WASM compilation is not execution.

## Remaining foundation gates

| Gate | Concrete progress this round | Required next work/evidence |
|---|---|---|
| FND-01 | Package/source/producer bindings and known history persist through review | Qualified identifier mappings, adopted version/extension contracts and complete graph/resource/SaveAs derivation |
| FND-02 | Canonical GRM3 has an authorized native producer/edit/reopen consumer | Domain/CRS/datum and observation qualification, richer source/edit profiles and compatibility acceptance |
| FND-03 | Package controls preserve unrelated typed/unknown resources | Required `.10d` typed operations in Poet, broader coexistence/extension negotiation and source-to-output cases |
| FND-04 | Selected budgets, cancellation, stale guards and preservation execute | Whole-scene/resource limits, fidelity/precision reporting and representative memory/latency measurements |
| FND-05 | Native controls and actual service-level separate-author/reviewer HMC workflow execute | Required geometry/material/assembly/rig/composition operators, usable native C1–C8 demonstration, recovery and human acceptance |
| FND-06 | Actual target compilation is checked separately | WASM runtime interoperability with declared host services; representative size/performance/precision benchmarks |

| IP-03 case (all open) | Partial evidence and remaining gap |
|---|---|
| C1 Definition | Shared brief/tasks/criteria/knowledge projection remains unqualified. |
| C2 Cooperation | Separate software contributor/reviewer sessions pass; human cooperation remains. |
| C3 Real authoring | Raster operators implemented; required model operators/previews unqualified. |
| C4 Revisions | Scoped source/package contribution/review/reopen pass; full authoring undo remains. |
| C5 Conflict safety | Selected stale/cancel/revoke/successor cases pass; conflict resolution remains. |
| C6 Production | Synthetic HMC reproduction/corruption refusal pass; full source-to-preview remains. |
| C7 Recovery | Durable authority/package reopen passes; disconnect/pending-job/rejoin remains. |
| C8 Usability/evidence | Actual control usability, native captures and human acceptance remain open. |

## How to inspect the implementation

The [built native executable](../../../../../../target/debug/poet.exe) was
not launched by this pass. Its existing entry is
`poet studio --native-project <admitted-project-directory>`; use an existing
authorized project and credential provider, not the ephemeral test actors.
Within the package controls, **Inspect resources** exposes the create operator;
**Inspect M64 metadata** exposes supported raster edits. **Saved package
contribution and review** binds records to the displayed saved HMC digest and a
declared project-scoped release identifier. **Steward: exact saved-package review
grants** is the separate explicit signing control. Signing changes admission and
locks old sessions. Compilation/source inspection establishes these controls are
wired; interaction and usability remain unverified.

Next qualify the required native `.10d`/geometry/material/assembly operations and
source/graph derivation through these review controls. Finish version/extension
negotiation and fidelity/budget reporting, then representative native/WASM runtime
evidence. Payment rails and asset polish remain later work.

[Final reconciliation](saved-evidence-verification.json) checks saved hashes,
actual test selection and unchanged terrain/`.10d`/HMC/master fixtures. No new
production asset was created. All 193 catalogue plus eight subject-specific
claims remain unassessed; film, open-world game and cartoon packages are
incomplete. All 120 production packets remain open. High-grade production and
reviewed game-demo handoff resume only after applicable readiness evidence passes.
