# RUN-05B — staged canonical checker and source sessions

Status: the supervisor applied the reviewed package to canonical Vibe and Poet.
All24actual target hashes match the frozen candidates. Actual integration results
are pinned in [actual-verification.json](actual-verification.json). Source baseline and
exact target/candidate SHA-256 are in `source-guards.json` and `patch-map.json`.
No parser source is owned or mutated by this patch. No model completion claim.

The canonical checker now uses its existing walkers to compute monotone local
function summaries before validating declared effects. It resolves forward and
recursive calls using the actual interpreter's first-function-definition
precedence. Collection callbacks inspect known noncapturing lambda values;
unknown indirect callbacks/callees fail closed. No app-local checker or second
interpreter is introduced. Standalone `check_cell` remains context-free;
`SourceSession` uses the checked retained context for source submissions.

`SourceSession::new(actual_host_grants, limits)` provides in-memory persistence
across submissions. `prepare` only parses/checks and produces an opaque
session/revision-bound candidate. `execute` consumes it once; only new top-level
items run. It retains checked variable types/mutability, imports, requested
capabilities already covered by fixed host grants, functions, hooks and enums.
Definitions are additive: replacement is rejected rather than silently changing
old bodies. Functions keep 0.1 noncapturing semantics. Public legacy APIs and
the `vibe-0.1`/`vibe-host-0.1` constants remain.

Syntax/check/grant failures preserve active local state without executing.
Runtime failure preserves the original Diagnostic and active session revision,
discards tentative local values/definitions, and sets explicit host reconciliation
required. This does **not** roll back host effects, make a unit Receipt verified,
or promise exactly-once external delivery. Authoritative receipts remain the
host's responsibility. Failed execution is never retried in another mode.

Session limits bound submissions, retained source, revisions, declarations,
bindings, interpreter workspace and the measured retained value representation.
The canonical lexer admits at most16delimiter groups/64conservative path units;
evaluation shares a total expression-depth budget64 across a maximum8user calls.
These independent limits cannot multiply into unbounded evaluator stack use.
The last is counted through a bounded formatting sink without a serialization
allocation; it is an admission bound, not an allocator-level memory guarantee.
Host return allocation/schema enforcement remains an upstream host requirement.
Primitive/list/record declared types are retained and validated; this does not
claim a full 0.2 type lattice or general static type inference.

## Ownership review and decomposition

The 985-line checker is split into routing/items (<400 lines), expressions,
statements, helpers, context and tests, each below 500 lines. `check.rs` preserves
its compatibility exports. The existing oversized `eval/mod.rs` receives only
necessary ownership plumbing (Env Clone, Arc program context, crate-visible
literal conversion). Existing `eval/call.rs` receives only replacement of the
unsafe borrowed pointer read by an Arc share. No independent new lifecycle is
added there. Tracked remaining decomposition: move evaluator value operators
and pattern helpers out of `eval/mod.rs`, then separate call dispatch from
collection operators in `eval/call.rs` before substantial new behavior. This
candidate does not claim that decomposition is already implemented.

## Executed private verification and remaining integration

`actual-harness/` uses the **actual applied** canonical Vibe crate and native
Poet module paths. The earlier `harness/` and private verification snapshot use exact original
Vibe source plus staged overlays, exact referenced include resources, and an
explicit isolated manifest with cached bytemuck/unicode-xid. Its source/overlay
and include identifiers are in `verification-inputs.json`,
`verification-overlay.json` and `verification-includes.json`. It is ignored
scratch, not a vendored production fork. Tests cover chain/forward/recursive/alias/pipeline and
indirect effect denial, pure calls, typed mutability, immutable denial, retained
functions/imports, no historical effect replay, self-grant denial, invalid/stale
candidate preservation, no capture, bounded sessions, and runtime partial
effects with original diagnostic. Five isolated default-stack boundary processes
exercise accepted unary/postfix64path units,16delimiter groups, a valid8call
mixed case and fail-closed over-budget8call composition. Prior actual stack
overflows and intermediate test failures remain in their logs. No test stack
was raised. Final verification totals/status are recorded in the execution
receipt after commands exit; a private pass does not mean canonical files were
applied.

The native Poet candidate routes its interactive loop through SourceSession,
preserves the old eval_line compatibility API, and adds explicit repeatable
`poet repl --grant <capability>` host-level grants (defaultnone). This is a
trusted local profile, not external project/provider authority. Detailed
compatibility corrections and supported profile limits are in
[compatibility.md](compatibility.md).

Native Poet/Craft cooperation, durable shared
checkpoints, jobs and IP-03 C1–C8 remain open.

## Actual applied-source verification

The first unfiltered actual Vibe run passed972unit tests and failed only the
existing `deontic_interrupt::tests::vc5_prohibition_breach_completes_under_1_microsecond`
timing gate:1100ns against a strict1000ns limit. That result remains failed in
`actual-vibe-full-first.log`; no source was changed or test expectation weakened.
The labeled functional rerun passed1232tests with only that timing case filtered.

Separately, the actual session/Poet adapter harness passed45tests, the real Poet
CLI workspace passed15tests, and all5stack-boundary cases passed in isolated
default-stack processes (`RUST_MIN_STACK` unset). These are separate command
counts with overlapping coverage, not a summed completion score. Actual Poet
built and both default/no-grant and explicit-grant transcripts exited0, showing
persistent functions/mutable values, immutable/self-grant denial and recovery.
The actual Vibe library also passed `cargo check` for the installed
`wasm32-unknown-unknown` target; this is compile evidence, not browser/runtime
conformance or a native graphical authoring claim.

The actual receipt pins logs, input cases, source maps, all24current source hashes
and the native CLI binary. No production source changed during verification and
no build/test command remains owned or orphaned at handoff. The timing gate,
full proposed0.2 qualification, durable/shared authoring, native graphical
Poet/Craft cooperation and model/output acceptance remain open.
