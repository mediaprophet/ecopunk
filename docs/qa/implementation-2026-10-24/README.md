# First implementation batch — execution record

**Started:** 2026-10-24. **Status:** first bounded batch implemented and checked
to the scopes below; broad production packets and model/language-profile
acceptance remain open. This is the actual-execution
companion to the [dispatch](../../planning/implementation-dispatch.md), not a
claim that native cooperative authoring or exceptional models are delivered.

The [current execution ledger](../../planning/tech/ecopunk-production-claims.json)
retains all 120 packets and their remaining acceptance, with bounded milestones
for the work below. The reusable starter ledger stays blank/unassessed; execution
results are not silently baked into copies intended for new work.

## Routing and source boundaries

Nine [worker roles](../../planning/implementation-agent-roles.json) define
bounded scopes, model/effort selection, escalation and evidence. The
[120-packet queue](../../planning/implementation-queue.json) preserves dependencies
and acceptance clauses. Lower-cost gpt-6-luna handled inventory and the bounded
native audit tool; gpt-6.1-sol handled rendering/runtime changes and independent
review. No larger-model architectural escalation was needed for this batch.

Existing work was preserved. No asset generation, expected-hash rewrite, commits,
publication, paid providers or financial transfers were performed. The reviewed
Poet patch was applied through the explicit filesystem approval mechanism; its
pre-apply bytes and guards remain available. Other sibling sources were inspected
read-only. No Vibe dialect/version change was made.

## RUN-01 — declared and locked source inventory

[Matrix](stack-source-matrix.md) and [machine evidence](stack-source-matrix.json)
pin 20 files across Ecopunk, game-demo, QualiaDB, Etalon, Aether, Poet and Webizen
host/shell. Supervisor follow-up extracted 166 relevant package records from six
pinned Cargo lockfiles. JSON parsing/hash checks passed. These are actual
declaration/lock records, not target-specific feature closure or device execution.

Remaining: exact active target/feature/source and renderer/device ownership,
local fork revisions and required native/WASM integration. ECO-003/009/011 remain
open beyond this bounded inventory.

## RUN-02 — texture roles and explicit admission errors

The scene's four opaque colour PNGs now request the public portal's role 0
(sRGB/Color), matching the native path. The previous ordinal modulo mapping
incorrectly requested normal/data/alpha semantics for three inputs. Rust now
propagates upload errors and unavailable GPU admission with texture name/digest.
The existing two HTML bootstrap surfaces only relay the constructor error into
their status text/alert instead of leaving the user at “connecting”; they do not
own the resource decision. Existing browser application logic remains migration debt.

Source: `crates/ecopunk-wasm/src/texture_admission.rs`, generated-resource fixture
test, minimal `lib.rs` wiring and the two mirrored HTML entry points. Source review
confirmed the callers await WebGPU initialization and the public upload adopts
the pending GPU. No light/exposure guessing or engine/private codec changes.

Completed checks: four dependency-free Rust control-flow tests, formatting,
the actual native crate build, **all five focused Cargo tests**, including
the generated-resource fixture, and the offline/locked **WASM target check**.
The cold dependency build reported 37m 13s;
this is build duration, not rendering/interaction performance. Target details are retained in
[the build receipt](texture-build.receipt.json) and [partial compiler log](texture-build.log).
The retained compiler log begins mid-build; it is not a full build transcript.

Remaining: actual codec/GPU rejection and residency/material execution,
native/WASM visual comparison, required fallback/profile behavior and
human visual acceptance. WebGL2/no-GPU cannot claim successful required textured
admission. This bounded source correction cannot establish overall washed-out
image causes, high-grade content or ECO-051/052 completion.

## RUN-03 — native Poet REPL evaluates a selected input once

The original REPL reproduced **two** dispatches of a failing host operation.
The reviewed patch selects input mode by parsing before checking/evaluation,
never retries after a runtime/check failure, and preserves the full diagnostic.
Ten candidate regression tests passed against the real local Vibe crate offline;
the original targeted regression failed as expected (exit 101, two calls).

[Worker handoff](poet-repl/README.md), [candidate log](poet-repl/candidate-test.log),
[baseline failure](poet-repl/baseline-test.log) and [guarded patch](poet-repl/repl.patch)
retain exact scope. The worker's staged status describes the earlier snapshot;
the supervisor subsequently applied the exact candidate to
`C:/github/poet/native/src/repl.rs` after all 140 guards matched. Candidate/target
SHA-256: `78f1b4c3640b2630b5760b03706078540b4d093010e26691cb44d3ae5c1ad2a3`.
The original source and pre-apply guards are preserved; reapplying blindly is
expected to fail the old-target guard.

The actual Poet-checkout native CLI suite subsequently passed **15/15 tests**
offline/locked, including the ten REPL regressions. The native binary was built
once, then executed directly as `poet.exe repl`: successive source inputs
returned **4**, then **8**, with no Cargo/Rust step between submissions. See the
[applied receipt](poet-repl/applied-receipt.json) and
[direct transcript](poet-repl/native-repl-session.log). This uses the CLI's local
host; it is not a shared-project or graphical authoring demonstration.

The direct session also exposed an existing mutable-binding limitation: a value
declared `let mut` stays readable but cannot be assigned in a later submission.
[Baseline/current observations](poet-repl/session-mutability-handoff.md) reproduce
the same rejection; the patch preserves canonical E701 instead of masking it as
E001. The [negative transcript](poet-repl/native-repl-mutable-bindings.log) is
retained separately. Persistent mutable/function/module contexts, multiline
entry, real async work, live native UI and IP-03 cooperation remain open.

Two separately identified upstream probes reproduced defects: trailing tokens
after a cell semicolon can be ignored; a Pure-labelled caller can reach an
External function. A probe “passing” means the defect reproduced, **not language
conformance**. RUN-05 is prioritized canonical parser/checker work before enabling
persistent function contexts. No Poet-local checker or authority workaround was added.

## RUN-04 — native stored-resource metadata audit

[Tool](../../../tools/asset-integrity/README.md) is an isolated native Rust CLI for
the existing model-set's component and graph file/hash/length declarations.
It does not decode HMC/Q42/10D, replace engine validation, change expected bytes
or establish package/model completeness. It has a separate tiny Cargo workspace
so engine GPU/crypto dependencies are not required for this metadata check.

[Independent review](integrity-review.md) caught actual-byte budget, canonical
reference propagation, quadratic report expansion and lexical Windows-path
defects in the initial six-test version. The worker corrected those issues and
split the implementation into focused modules. Supervisor follow-up added
regular-manifest/read consistency checks, per-resource read-error reporting,
post-read length checks and a digest of the consumed manifest bytes. **Nine
focused tests passed** offline/locked after these changes. Unix symlink cases
remain unexecuted on Windows; concurrent hostile filesystem mutation is outside
the tool's stated snapshot scope.

The actual [Corymbia run](corymbia-integrity-report.json) checked **126 references
to 122 canonical paths**, including the two graph resources. It correctly exited
**1** for integrity issues: **8 distinct resources with digest mismatches,
15 with length mismatches and 2 with conflicting declarations**. These categories
overlap; 15 distinct resources / 19 references were flagged overall. Repeated
references retain their own contexts and share the resource outcome. See the
[receipt and source pins](asset-audit.receipt.json). Manifest SHA-256 was unchanged
before/after and matched the digest embedded in the tool's report.

No expected hash, asset bytes or model status was rewritten. Reconcile those
resources against admitted editable sources/compiler lineage before repair;
matching newly observed bytes alone cannot certify intended content or quality.

## Canonical language and native authoring continuation

The [second-batch record](batch-2/README.md) tracks the subsequent applied canonical
parser/checker/session and native source-journal repairs, actual integration
checks and the native Poet/Aether authoring work. Earlier logs and limitations
above describe the first batch at its recorded source revisions; later fixes do
not rewrite those historical observations.

## Remaining mission gates

All 120 broad ECO packets remain open. No complete model, 0.2 language release,
native/WASM product profile, cooperative authoring gate or human acceptance is
claimed. The 193-base-deliverable model catalogue and expanded dossiers remain
unchanged. Next authoring work must finish canonical checking/session/job/UI
requirements and demonstrate IP-03 C1–C8; asset production still needs real
editable high-grade sources and reviewed derivatives actually used by game-demo.
