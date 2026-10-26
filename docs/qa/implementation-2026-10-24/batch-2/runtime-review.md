# RUN-05 independent runtime/security review

**2026-10-24 — bounded review complete; approved for guarded apply.** The reviewed work is a staged canonical
Vibe 0.1 correction and additive in-memory session API. It has not been applied
to QualiaDB or integrated into the actual Poet checkout at this review freeze.
This review does not establish a
released 0.2 dialect, IP-03 C1–C8, human acceptance or model completion.

## Scope and evidence discipline

Reviewer owns only this file. Inputs are the Ecopunk dispatch, architecture,
development modes, proposed Vibe 0.2 contracts, and staged source under
`runtime/candidate/vibe/src`, compared with read-only canonical QualiaDB
parser, evaluator, host dispatch, capability catalogue and budget source.
No source writes, builds, paid calls, commits or publication were performed by
this reviewer. The supervisor owns compilation and execution evidence.

Review concentrates on evaluator/checker agreement, actual grant subsets,
checked-candidate session/revision binding, retained types and mutability,
exactly one execution attempt, no history replay, failed-state admission,
external-effect reconciliation and program ownership/bounds.

## Findings communicated for correction

These describe earlier candidates. RR-01–13 have been corrected and independently
reinspected in the frozen package below; no mandatory finding remains open for
this bounded change. The ledger preserves the actual issues and remedies rather
than treating earlier candidates as passed.

Callback examples in this table are checker-level shapes. Actual 0.1 lambda
creation rejects an unimported `pulse` namespace as a capture. Executable attack
and positive-control fixtures must begin with `import "pulse" as p;` and use
`p.publish` inside the lambda. This preserves the actual noncapturing rule;
checker rejection by itself is not evidence that a Host effect was executed.

| ID | Concrete issue and review case | Required disposition |
|---|---|---|
| RR-01 | `catalog::granted_covers` returns true for non-catalog paths. Calling it unconditionally for session request admission admits `requires [ capability("pulse.publish") ]; pulse.publish("escaped", 1);` with an empty actual grant set. Bare family requests have the same hazard. | Require exact actual legacy/family grants; use catalogue coverage only for a known catalogue operation. Verify no dispatch/revision change on denial. |
| RR-02 | Checker `walk_block` cloned/discarded callable facts while the 0.1 evaluator executes blocks in the same `Env`. An External lambda assigned to mutable `f` in an `if` could later execute through a Pure `map(f)` checked using the previous Pure lambda. | Conservative invalidation/merging must cover runtime writes through blocks, loops and matches. Preserve existing evaluator semantics explicitly. |
| RR-03 | `let g = f` stored checker expression `Ident(f)`, while runtime copies the current `Value`. Changing `f` from an External lambda to a Pure lambda made checker `g` appear Pure although runtime `g` remained External. | Snapshot the callable fact at binding or invalidate dependent aliases on writes, including chains. |
| RR-04 | Evaluator collection dispatch has a second branch that treats a variable named `math` or an import alias as a receiver even when that name is also a namespace. Checker skipped callback analysis for namespace receivers. `let math=[1]; let f=|x|pulse.publish("escaped",x); pure cell c := math.map(f);` could execute an External callback. | Match actual receiver precedence; analyze/invalidate namespace shadows and import aliases. |
| RR-05 | `Item::Material(_) => Pure` ignored executable properties although evaluator evaluates them. `material m: Material p: pulse.publish("escaped", 1);` could dispatch without a request or actual grant. | Check all property expressions and their normal capability obligations. |
| RR-06 | Session duplicate detection initially covered selected top-level declarations only. `{ let fixed: i64 = 9; }` could replace an accepted immutable `fixed: i64`; destructuring and other named item kinds also escaped the scan. | Cover all runtime-visible declaration paths or reject unsupported paths before execution. Retain explicit scope limitations. |
| RR-07 | Bounded-list inference preceded explicit `let` annotation, so `let xs: i64 = [1];` retained `List(Unknown)` and passed post-evaluation validation. | Preserve explicit annotation and independently reject incompatible literal/container values. |
| RR-08 | `bind` overwrites endpoint roots in `apply_bind`, without updating checker callable facts or validating immutable endpoints. `for` variables and `match` patterns similarly mutate runtime values. Session declaration checks alone cannot repair ordinary `check_program` effect inference. | Model or conservatively invalidate all writes; reject immutable session binding replacement through non-declaration paths. |
| RR-09 | Initial resolver treated any dotted unknown path as `binding_effect`, whose default is Pure. Also functions were resolved before `len`/constructors although evaluator builtins take precedence. | Fail closed for unknown targets and preserve trusted 0.1 binding/enum/builtin resolution. Unknown dispatch itself was inspected and returns E100 rather than reaching arbitrary Host code. |
| RR-10 | Function/hook budget values were executable but unchecked. `pure fn bad() budget(steps: pulse.publish("escaped", 1)) { return 1; }` dispatches when called despite Pure/no grant. `budget(steps: bad())` evaluates before the function dispatch increments call depth, so recursive budget evaluation bypasses the normal 64-call guard. `budget_steps` also discarded runtime diagnostics through `.ok()`. | Validate budget expressions/effects under the actual caller scope; preserve error diagnostics and establish a recursion guard or a restricted checked budget form. |
| RR-11 | A successful early top-level return skips later runtime assignments, while checking still visits and promotes their facts. After retaining External `f`, `return 1; f = |x| x;` could promote checker `f = Pure` while runtime `f` remained External, exposing the next submission's Pure `map(f)`. | Reject executable top-level return paths or reconcile actual control flow before promoting metadata; exclude function/hook-local returns from that restriction. |
| RR-12 | Several real Host operations were admitted by the new path registry but inherited `binding_effect`'s default Pure: `cue.post`, `dag.execute`, `hid.wait`, `hid.poll`, `crypto.generate_key` and `crypto.sign`. The session's extra grant guard applies only above Pure, so a pure function can deliver a cue or use a host-managed signing key without a matching session request/grant if the Host permits it. | Classify actual effectful Host seams and require their admitted authority; preserve explicit genuinely pure profiles separately. Test with an in-memory counting Host. |
| RR-13 | Full private-suite execution found `n7_time_in_pure_cell` lost its `suggested_fix`: the new generic call-effect diagnostic intercepted the older diagnostic-to-fix mapping. | Preserve the structured diagnostic/fix contract for known native violations; do not treat this diagnostic regression as an intentional source-authority migration. |

## Ownership and external outcomes

The initial candidate has the right admission shape: checked submissions have
private owner/revision fields, execution consumes the candidate, and foreign or
stale candidates fail before evaluation. Local `Env`, checked metadata,
definitions, source history and revision promote together only after execution
and validation succeed. Prior executable statements are not re-run to reconstruct
state. Host effects operate outside the tentative frame; a failed execution is
marked as attempted and requiring authoritative host reconciliation. That flag
does not assert rollback or a verified receipt.

Replacing the engine's raw program pointer with an owned `Arc<Program>` addresses
the program lifetime hazard. Recursive dispatch clones the Arc rather than the
AST. Function/hook locals preserve noncapturing 0.1 behavior, rather than silently
introducing closures. Public low-level engine/evaluator APIs still require their
documented checked-input discipline; the session API does not transform every
existing host call into a capability-enforcing sandbox.
The new actual-grant API admits a fixed set of capability identifier strings;
resource/topic/actor/expiry restrictions and dynamic operation descriptors still
require the authoritative Host contract. Identifier admission is not proof of
those wider scoped-authority checks.

## Bounds and compatibility qualifications

Source bytes, retained source/revisions/declarations/bindings and retained value
representation limits are useful admission bounds. The retained value limit is
explicitly a bounded Debug-size measurement, **not an allocator hard ceiling**.
The evaluator still has preexisting operations such as string replacement/join
that allocate output before a final retained-size check. A finite step count
alone does not establish a peak memory bound for these operations.

The canonical parser alone has no general recursion budget. The supervisor's
new `session/source_limits.rs` adds a preparse **session profile** using the
canonical lexer: sixteen delimiter frames and sixty-four path units, with child
maximum paths carried into parents. Flat data/statement separators reset only
their current frame; source-declared locale keywords receive canonical lexer
classification. Interpolated strings are held because their embedded parser
needs a shared nesting contract. The hook runs after byte/revision checks and
before either parser entry. This is conservative syntax admission, not a second
parser or a global allocator guarantee. Five separate default-stack boundary
processes passed, including admitted unary/postfix paths, sixteen groups, a valid
mixed call/expression case and an over-budget composition returning E400.

The final session profile rejects nested global declarations and top-level
return paths before execution. The underlying 0.1 evaluator still shares its
frame across blocks; it has not silently acquired different lexical execution
semantics. Functions retain supported local scopes and ordinary return behavior.
Session calls have a separate clamped range of one through eight frames; legacy
stateless engines retain their existing sixty-four-call setting.
Expression evaluation also has a shared one-through-sixty-four limit across
function calls. The public wrapper increments before expression evaluation and
decrements on its Result path; recursive expression evaluation routes through
that wrapper. This closes the actual composed-limit overflow reproduced during
review. Legacy engines retain their previous unbounded expression-depth profile.

Budget metadata is deliberately restricted: pure operators/builtins are allowed;
user-function calls are rejected by checker and runtime budget context. This
closes the reproduced stack overflow without increasing the test thread stack.
Budget failures preserve their diagnostics. Durable recovery, live replacement,
jobs, expiry/revocation and native cooperative authoring remain outside this
additive session correction.

The parser/checker has input/declaration caps but no separately charged checker
operation/deadline budget. The fixed-point function analysis can require many
passes on a long unannotated call chain. No timing claim is made; general
checker-work/UI responsiveness qualification remains open. Native Host operation
allocation, result validation and scheduling likewise require their own bounds.

## Frozen review and execution disposition

Frozen candidate: **RUN-05B-2026-10-24-final**. The reviewer independently checked
all candidate hashes, unchanged original target/archive hashes, absence of new
targets and all artifact hashes in the frozen receipt. Results: 24 targets,
zero candidate mismatches, zero original/absence guard failures and zero frozen
artifact mismatches. No source apply or build was performed by this reviewer.

| Pinned artifact | SHA-256 |
|---|---|
| [Canonical map, 22 targets](runtime/patch-map.json) | `741c51314b8aff6899247690e1c5fc02a49d0a40cfc1fb57c97b49cc896adb0d` |
| [Poet map, 2 targets](runtime/poet-patch-map.json) | `61e1f173654fde85acc6b8d26b42334f200829ca70e3cf3618dbd81ca3d70e4e` |
| [Frozen package receipt](runtime/frozen-package.json) | `5f4edbe1d967f074bbce1138920bf767b13de93a0bf53a25f86b1c8367dd9b53` |

**Disposition: proceed with exact guarded apply and target verification.** The
source corrections address RR-01–13 for the declared profile. Canonical-checkout
and actual Poet dependency/consumer verification remain supervisor work; a private
snapshot pass is not an applied-source run or broad language/security release.

The author's [final full private execution](runtime/verification-full-final.log)
and [receipt](runtime/verification-receipt.json) record exit 0, **1,278 passes**,
including 973 library tests, 184 conformance cases, 32 session/authority
regressions and 13 Poet module/harness cases. Five ignored boundary cases in that
aggregate were executed separately in default-stack child processes; all five
passed and their exact logs are pinned in the frozen receipt. No stack size was
increased. Negative authority cases have actual granted callback/cue dispatch
positive controls; failed/stale state, no replay and original runtime diagnostics
are checked. RR-13's original diagnostic/fix regression now passes.

The five earlier conformance failures were reconciled explicitly, as recorded in
[compatibility.md](runtime/compatibility.md): unsafe Hot tick fixtures `g1`, `h1`
and `h3` remain unchanged negative sources expecting E200; only signing/key
generation functions in `crypto1` gain External annotations; unknown math `n4`
is rejected by the checker. Eight old evaluator host-dispatch unit fixtures use
External entry functions and retain their expected Host/type outcomes.

The private [native Poet build](runtime/native-poet-build.log) and
[default](runtime/native-default-session.log)/[explicit-grant](runtime/native-granted-session.log)
transcripts exited 0. They compile the actual staged main/REPL source with a
private manifest and exercise retained mutation/functions, immutable/self-grant
denial, Pure-to-External denial and recovery. The interactive entry routes to the
canonical session; repeatable `--grant` options supply a trusted local host
profile before source evaluation. The old `eval_line` API remains a legacy
compatibility path. This is native CLI evidence, not graphical Poet/Craft
cooperation, shared project authority or IP-03 C1–C8 acceptance.

The fourth focused run's recursive budget case aborted with Windows
`STATUS_STACK_OVERFLOW`; its earlier 64-depth adjustment was insufficient. The
subsequent checked/runtime user-function ban in budget expressions is the
reviewed remedy. The failing log is retained as evidence, not a passing bound.

The separate parser prerequisite **is applied**, inspected again after the
supervisor's change. Canonical `C:/github/QualiaDB/crates/vibe/src/parse/mod.rs`
has SHA-256 `3a38527eaa49f0c1ed40eda19f8523dcf5c64b2f58bfec2c5671798a218b1f40`.
`parse_cell_body` consumes one optional semicolon and then requires EOF. The
supervisor-owned [parser execution log](parser/applied.log) records five passing
regressions, exit 0: optional semicolon, whitespace/comments, original lexer
errors, trailing-token rejection and nested record/conditional parsing. This
result does not establish a general parser nesting budget.
