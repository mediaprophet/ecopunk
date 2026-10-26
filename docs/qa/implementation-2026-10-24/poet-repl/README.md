# RUN-03: guarded native Poet REPL candidate

**2026-10-24 — staged, not applied to Poet.** This bounded patch fixes repeated
evaluation during input-mode fallback. It does not qualify VIBE-04, a language
release, a persistent authoring session or native cooperative production.

The original native REPL dispatched a failing `quin.statement(...)` request
**twice** in the isolated baseline regression. The candidate dispatches once and
returns the original complete host `Diagnostic`. All 10 candidate regression
tests pass against the real local `vibe` crate, offline. Two separate upstream
source probes reproduce unresolved parser/effect-checker defects; those passing
probe assertions are evidence of defects, not language conformance passes.

## Candidate and compatibility

The exact replacement is [candidate/native/src/repl.rs](candidate/native/src/repl.rs).
[repl.patch](repl.patch) is the corresponding review/apply diff for
`C:/github/poet/native/src/repl.rs`. The public `eval_line` signature and existing
callers are retained. The two existing tests remain, with eight focused additions.
There are no dialect/version changes, new interpreter, host permissions, async
implementation or JavaScript dependencies.

Mode is selected before checking or evaluation:

1. Explicit `=` selects the existing Pure-cell parser/checker.
2. Otherwise a fully parsed program selects program checking/evaluation.
3. If program parsing fails, use the existing Pure-cell parser/checker.

Neither checker failure nor runtime failure retries another mode. Semicolon-
terminated expressions select program mode, preserving admission of valid
explicitly authorized effectful programs without trying a Pure evaluation first.
Bare expressions without a semicolon remain Pure cells. Empty input remains null.
Program-mode diagnostics now use the original program offsets rather than an
injected `= ` prefix. Multiple valid statements are evaluated as the authored
program instead of a successful cell parse silently ignoring the later statements.
These are explicit shell mode-selection changes; the `vibe-0.1` parser/checker/
evaluator themselves are unchanged.

The implementation deliberately retains `Engine::new`. `Engine::with_program`
would expose new callable retained functions while the canonical checker cannot
enforce their transitive effects. No Poet-local substitute effect checker was
introduced. Existing value/declaration maps in `Env` retain their current behavior;
this patch does not claim persistent functions/imports or transactional bindings.

## Executed evidence

All commands were run from `C:/github/ecopunk`, with writes confined to this owned
directory. The isolated [harness](harness/Cargo.toml) depends directly on the real
`C:/github/qualiaDB/crates/vibe`; it does not copy/vendor an interpreter or build
the Poet/QualiaDB workspace. Its lockfile pins the resolved cached support crates.
Build output is confined to ignored `harness/target/`.

| Check | Actual result / limit |
|---|---|
| `cargo test --manifest-path docs/qa/implementation-2026-10-24/poet-repl/harness/Cargo.toml --offline` | Exit 0: 10 REPL tests passed; 2 separate upstream-defect probes reproduced their observations. See [candidate-test.log](candidate-test.log). |
| Same harness, `--offline --features baseline --lib failing_host_call_runs_once_in_each_mode` | Expected exit 101 against original production code plus the same regression tests: dispatch count **2**, expected **1**. See [baseline-test.log](baseline-test.log). |
| `rustfmt --edition 2021 --check` on candidate, harness module selector and upstream probes | Exit 0. |
| `verify-or-apply.ps1` without `-Apply` | Source/candidate SHA256 guards matched; no application performed. |
| `git -C C:/github/poet apply --check …/repl.patch` | Could not run in the sibling directory: `fatal: cannot change to 'C:/github/poet': Permission denied` (exit 128). Direct file reads and all digest checks succeeded. Target checkout Git integration remains unverified. |
| `git -C …/poet-repl/harness/apply-check apply --check …/repl.patch` | Exit 0 against the exact staged original copied to `native/src/repl.rs` in this owned scratch directory. No source was patched. |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`; `cargo 1.97.1 (c980f4866 2026-06-30)`. |

The tests cover failing host calls in inferred cells, explicit cells and programs;
full returned diagnostics (including fix/evidential fields); authorized effects;
denied effect requests without dispatch; invalid program prefixes without effects;
preserved committed effects on later runtime failure; cross-submission value
declarations; arithmetic; and pinned 0.1 truthiness. The baseline feature compiles
`harness/baseline.rs`: original production code plus the candidate's test module.

The only final candidate-build warnings were incremental-cache hardlink failure
with copying fallback. No paid provider, network request, commit, publication,
whole-workspace test, actual native Poet CLI/UI, renderer or human review ran.

## Safe application boundary

[repl.original.rs](repl.original.rs) preserves the exact original bytes.
[source-guards.json](source-guards.json) records the target/original SHA256,
candidate SHA256, inspected Poet manifests/caller and every Rust source file in
the local Vibe dependency, plus its manifests (140 normalized source guards).
Candidate SHA256: `78F1B4C3640B2630B5760B03706078540B4D093010E26691CB44D3AE5C1AD2A3`.
Original SHA256: `2E95F923A5E4E89F0DE1AE47960A5EC136A6F396CF7A7CEF0CC9798284B6BD19`.
Run the guard script immediately
before review/application; any mismatch requires re-review/rebase rather than a
forced copy. Git line-ending conversion can change bytes and invalidate guards.

```powershell
# Read-only; already executed successfully.
& C:/github/ecopunk/docs/qa/implementation-2026-10-24/poet-repl/verify-or-apply.ps1

# Only a supervisor with real Poet filesystem authority may opt into this.
# NOT executed by this worker.
& C:/github/ecopunk/docs/qa/implementation-2026-10-24/poet-repl/verify-or-apply.ps1 -Apply
```

The script checks all recorded source guards and the candidate bytes before a
single-file write, then verifies the applied bytes. The separate diff can be used
for review, but applying it alone does not establish the recorded byte guards or
target integration. After an authorized application, run focused `poet-cli`
tests and actual native Poet session checks under that repository's authority.

## Remaining requirements and concrete upstream remedies

| Open scope | Source/observed evidence | Required remedy and acceptance |
|---|---|---|
| VIBE-02 / VBC-004/019: complete input consumption and incomplete/invalid distinction | `vibe/src/parse/mod.rs::parse_cell_body` accepts a semicolon without consuming the remaining source. The executed probe accepts `= 1; not valid @` and evaluates 1. This defect remains in both baseline and candidate. | Canonical parser must validate complete cell input and expose structured Complete/Incomplete/Invalid results. Reject trailing garbage before dispatch; prove strings/comments/grouped expressions and incomplete blocks. No delimiter-count heuristic in Poet. |
| VIBE-03/04 / VBC-011/020: transitive function effects | `check_cell` has no prior-program context; `check_function`/`walk_expr` classify unknown user calls through `binding_effect`, which defaults to Pure. The executed probe passes `load_program` and `check_cell` for a Pure caller which then invokes an External `pulse.publish` function through `Engine::with_program`. | Canonical checker needs resolved callable signatures, transitive effect/capability analysis (including cycles, aliases and dynamic calls), with an explicit public program/session-context checking API. Pure cells calling retained External functions, directly or transitively, must fail before host dispatch; authorized program invocation must still work. |
| VIBE-04 / VBC-008/019/021: persistent program/function/import context and scoping | `Env` has no function table; `eval_program` ignores Function/Hook items. Function dispatch reads the attached Program through a raw pointer. Function-local `Env` copies selected declaration maps and parameters, not global value bindings. The REPL still creates an unattached Engine for each submission. These are inspected source findings, not completed persistence regressions. | After canonical effect checking, add a session-owned checked Program/revision with a safe lifetime, explicit declaration replacement, prior-source recovery and no replay of old statements. Prove nested calls, parameter/local isolation, imports, constants/global lookup policy, malformed replacement, failed activation, recursion limits and mutable binding behavior against pinned 0.1 decisions. |
| VIBE-04/05/06/07 and VBC-019–025 | No multiline buffer, revision history, transactional activation, futures, jobs, live native UI edits, cooperative review, checkpoints or restart qualification in this patch. | Implement through canonical parser/runtime/host owners and demonstrate their defined positive/negative/recovery cases. |
| IP-03 C1–C8 / VBC-031 | No contributor/reviewer native source-to-preview-to-HMC run or human acceptance. | Perform actual usable native Poet cooperative authoring and artifact reproduction. A passing isolated Rust harness cannot close this gate. |

ECO-034/044, ENG-13 and VIBE-04 remain open beyond this narrow staged correctness
fix. VBC-020 is supported only for the local mode-fallback regression, not
reconnect/job delivery guarantees. No model package, film/game/cartoon profile,
production master or broader language completion claim is made.
