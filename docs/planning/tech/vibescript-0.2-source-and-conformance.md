# VibeScript 0.2 — source audit, migration and ordered work

**Status:** read-only source inspection and proposed work, 2026-10-24.
No interpreter, REPL, fixture, build, WASM, UI or GPU run was performed. Existing
source tests and historical reported passes are not fresh execution evidence.
Source file digests and repository-metadata limitations are recorded in the
[baseline](vibescript-0.2-source-baseline.json); sibling HEAD/dirty state was not established.

## 1. What already exists

| Inspected source | Finding / practical implication |
|---|---|
| `C:/github/qualiaDB/crates/vibe/src/lib.rs` | `LANGUAGE_VERSION = vibe-0.1`, `HOST_VERSION = vibe-host-0.1`; `eval_cell`, `load_program`, `eval_function`, `dispatch_hook`, Engine/Env. Source evaluation already exists. Do not describe Vibe as requiring a Rust build per script. |
| `qualiaDB/docs/manuals/standards/vibescript-core.md`, §§0–17 and extensions | Substantial normative 0.1 definition: typed interpreted REPL/application destination, no JIT, mandatory AST evaluator, optional bytecode, graph/effect/budget rules and zero-recompilation UI loop. Older essays/progress lists are secondary evidence. |
| `qualiaDB/crates/vibe/grammar/vibe-0.1.ebnf`, `src/parse/` | Core grammar and broad parser exist, including functions/hooks/cells, field/material/law, graph/modal/quantity and presentation constructs. Standalone EBNF references undefined rules including ParamList, PresentProp and domain declarations; the grammar/checker/evaluator need one reconciled coverage map. |
| `qualiaDB/crates/vibe/src/parse/cond.rs`, `ast.rs`, `eval/expr.rs` | Value-producing `if … else` is present now, including bounded nesting and selected-branch evaluation. Do not resurrect older game-demo missing-if work as an unimplemented feature without revision reconciliation. |
| `C:/github/poet/native/src/main.rs`, `repl.rs` | Native `poet repl`, eval/format/lint command source and persistent LocalHost/Env loop exist. The loop reads one line and tries evaluation before deciding alternate input mode; richer session safety is not established. |
| `qualiaDB/crates/vibe/src/eval/mod.rs`, `eval/program.rs`, `eval/call.rs` | Env persists several value/declaration maps. The inspected REPL creates `Engine::new`; function dispatch uses a program context and eval_program ignores Function/Hook items. Cross-submission function lifetime requires qualification/repair; do not claim a measured failure. |
| `qualiaDB/crates/vibe/src/eval/expr.rs` | The inspected `Await` branch directly evaluates its operand. This path does not establish suspending futures, cancellation or a responsive UI; other native job mechanisms must be reconciled rather than assumed absent. |
| `qualiaDB/crates/vibe/src/types.rs`, `value.rs` | Broad typed/opaque/domain values exist. i32/u32/f32 currently resolve to wide types. `is_truthy` makes false/null/integer-zero/disclosure-denied false, most other values true; proposed 0.2 bool-only conditions are a deliberate migration. String/Option/Result/receipt runtime representations need semantic parity checks. |
| `qualiaDB/crates/vibe/src/effects.rs`, `bind/`, `capability_schema.rs` | Effect ordering, capability mapping, host environments, named dispatch and schema machinery exist. Static prefix/default classifications are not enough to prove every dynamically named operation has the correct effect/grant descriptor. |
| `qualiaDB/crates/vibe/src/budget.rs`, `bytecode/` and related modules | Step/workspace budgets and optional VM source exist. The core document describes VM subset exclusions. No need to replace working AST evaluation with a mandatory compiler. Existing compiler/Webizen VM work has a separate role. |
| `qualiaDB/crates/vibe/fixtures`, `tests` | Visible count at inspection: **166 `.vibe` fixtures**, **8 integration-test `.rs` files**. Counts are source inventory, not passes or comprehensive profile coverage. Prior Ecopunk “168” prose was stale. |
| `qualiaDB/crates/vibe/README.md` | Tooling/LSP path points to `C:/Projects/vibe-script`; that path and `C:/github/vibe-script` were absent at inspection. Locate/reconcile actual tooling before asserting availability or rebuilding it. Absence at these two paths is not an ecosystem-wide absence claim. |

The EBNF/prose/parser/value/checker/VM are separate coverage surfaces. The source
also contains interpolation, richer patterns, reflection, reactive/projectional,
semantic/deontic, clocks and physics work beyond the compact grammar. Preserve
these inputs and determine admitted profiles; neither a compact spec nor a
file count justifies deleting them or calling them fully delivered.

## 2. Explicit candidate migration decisions

| Topic | Candidate 0.2 contract / migration obligation |
|---|---|
| Source-first execution | Retain existing AST interpreter. Remove mandatory-compilation language from consuming plans; optional bytecode remains separately versioned and parity-qualified. |
| Versions/imports | Select source dialect in session/package. Existing import alias code recognizes `vibe:0.1/…`; implement explicit compatible 0.2 resolution/catalog pinning without rewriting old modules. |
| Numeric spellings | Keep current narrow spellings as documented wide aliases. True narrow arithmetic is not implied. Check mixed conversion/range/NaN behavior against the new exact rules. |
| Conditions | Proposed 0.2 requires bool; 0.1 truthiness preserved for pinned legacy scripts. Diagnose/migrate affected code explicitly, including disclosure-denied branches. |
| Strings | Proposed scalar-based length/index differs from observed byte-length code. Retain original source text and define span-to-editor mappings. |
| Option/Result | Define Some/None independently from null, payload/error types and `?` propagation; source enum/value/constructor/checker/VM coverage must agree. Do not claim syntax alone supplies this. |
| Module/session state | Persist functions and resolved program context between submissions; load/activate/run and declaration replacement are explicit. Prevent accidental effect execution while probing parse alternatives. |
| Effects/receipts | Resolve all dynamic host calls through descriptors; fail closed. Replace any convenience receipt markers only through versioned host migration, retaining historical evidence. |
| Async | Add/qualify real suspension, continuations, jobs, cancellation and streams; reconcile existing host jobs. Synchronous await syntax is not completion. |
| Syntax extensions | Candidate EBNF normalizes async prefixes, fills previously undefined rules and includes structural patterns. Reconcile interpolation, separators, enum unit patterns, graph forms and quantity lexing with existing parser/fixtures. Unsupported optional syntax must be diagnosed, never silently accepted then ignored. |
| Native UI | Preserve existing presentation/reactive/QApp work; project one typed model into native Aether/Poet and required WASM adapters. A DOM/CSS implementation does not certify the native path. |
| Domain contracts | Preserve units/frames/clock/semantic/disclosure rules and multi-identifier subjects. Bind source and host descriptors; do not collapse broad native capabilities into untyped record guesses. |

These are candidate design choices for ratification, not orders to change sibling
source under Ecopunk planning authority. Preserve valid 0.1 outputs and expose
incompatible source before activation; no silent downgrade to JS or hidden reset.

## 3. Dependency-ordered implementation packets

All packets below are **open**, with proposed owners. Reuse the inspected source
and existing checks; add only missing behavior and meaningful conformance cases.
They extend existing ECO packets rather than introduce a competing phase order.

| Packet | Depends on | Proposed owner / concrete deliverable | Governing allocation |
|---|---|---|---|
| VIBE-01 | none | Language/Qualia/Poet owners: baseline exact source, catalog/tool paths, grammar/type/effect/runtime coverage and accepted compatibility decisions; ratify version/profile boundary. REQ-001–006/022/023. | IP-00; ECO-001/003/009; ENG-13 |
| VIBE-02 | VIBE-01 | Parser/checker owners: canonical complete grammar/lexical policy, AST/schema/source mappings, type/control semantics, diagnostics and parser Complete/Incomplete/Invalid. REQ-003–006/019/023. | IP-01; ECO-011/018; ENG-13 |
| VIBE-03 | VIBE-01 | Host/catalog owners: typed descriptors, versions/effects/grants, resource lifetime and budgets; map actual native/WASM bindings before adding missing ones. REQ-002/007–010/017/020/022. | IP-01; ECO-009/012/018/034; ENG-02/07/13 |
| VIBE-04 | VIBE-02, VIBE-03 | Evaluator/Poet owners: persistent multiline sessions, functions/modules, safe mode selection, one execution attempt, revision history and structured errors. REQ-005/006/014/019. | IP-03; ECO-034/044/045; ENG-08/13 |
| VIBE-05 | VIBE-03, VIBE-04 | Runtime/job owners: real futures/continuations, cancellation/progress/streams, rechecked authority, stale results and external receipt recovery. REQ-008/009/015–017. | IP-03; ECO-034/044/050; ENG-07/09/13 |
| VIBE-06 | VIBE-04, VIBE-05 | Reactive/UI/framework owners: cell graph, native presentation/input, explicit effect activation, clock/tween behavior, transactional live edits and state migration. REQ-011–015/019. | IP-03; ECO-044/045/050; ENG-08/10/13 |
| VIBE-07 | VIBE-03, VIBE-06 | Cooperative/Poet owners: authoritative shared source, revisions/roles/review/undo, checkpoints/restart/rejoin; preserve contributions/terms. REQ-009/010/018/022. | IP-03; ECO-034/044/050; ENG-09/13 |
| VIBE-08 | VIBE-02, VIBE-03, VIBE-06 | WASM/host owners: qualify shared local semantics and assisted provider version/grant/job/disconnect behavior. REQ-012/020. Required cases remain open until delivered. | IP-07; ECO-037/038/040/116; ENG-08/13 |
| VIBE-09 | VIBE-02, VIBE-03 | Format/runtime owners: source/AST/HMC manifests, cache invalidation, optional VM differential tests on supported subset and safe AST selection. REQ-002/006/021/023. | IP-02/03 source/package cases; ECO-021/028/044; ENG-02/11/13 |
| VIBE-10 | VIBE-06, VIBE-07, VIBE-09 | Production/QA/domain reviewers: real no-build native Poet authoring loop on the defined reference source with conflict/invalid edit/late job/restart and exact HMC reproduction. REQ-001/024 plus used domain cases. | IP-03 C1–C8; ECO-045/050; ENG-03–09/13 |
| VIBE-11 | VIBE-07, VIBE-08, VIBE-09, VIBE-10 | Language/independent QA: native/WASM profile conformance and claim coverage, source/API migration and release notes; only then label a qualified 0.2 release. REQ-001–024. | IP-07/09; ECO-040/112/116/120; ENG-12/13 |

“REQ-001” in this table abbreviates REQ-VIBE-001 only. VIBE-01–10 may qualify
required authoring cases incrementally; optional VM/backends and unused domain
extensions must not delay the first exceptional masters. A qualified pinned 0.1
path can satisfy a production case if it actually meets that case's semantics;
the 0.2 label itself is not a prerequisite. Conversely an actual blocking REPL,
host or cooperative defect cannot be deferred merely to render a nicer image.

## 4. Required conformance cases

These are **case definitions, not executed tests**. Every case includes positive
and negative inputs where meaningful, exact profile/source/catalog/target refs,
expected/actual results and effects, diagnostic spans and reviewer evidence.
The [claim starter](vibescript-0.2-claims.template.json) retains all 32 case rows.

| Case | Required proof |
|---|---|
| VBC-001 | Evaluate changed arithmetic/function/UI source on an installed native host with no per-script Cargo/Rust/JS/WASM build. |
| VBC-002 | Explicit 0.1/0.2/profile/import negotiation, unknown version rejection and legacy semantic preservation. |
| VBC-003 | Unicode/BOM/newline/comment/escape/locale/token boundary/security and source-span round-trips. |
| VBC-004 | Complete grammar examples, rejection of malformed/unknown syntax and all referenced production coverage. |
| VBC-005 | Integer extrema/overflow/division/remainder, aliases, precision conversions and NaN/finite-schema behavior. |
| VBC-006 | Bool-only 0.2 conditions and short circuit; legacy truthiness stays explicitly pinned. |
| VBC-007 | Unicode string length/index, list/record bounds, duplicate keys, immutable aliases and typed updates. |
| VBC-008 | Functions/locals/returns/patterns/exhaustiveness, recursive limits and noncapturing lambda restrictions. |
| VBC-009 | Some/None versus null, Ok/Err payloads and compatible/incompatible `?`. |
| VBC-010 | Offline pinned imports, export scope/cycle errors and load without running effects/hooks. |
| VBC-011 | Pure/Hot/Cold/Async/External transitive and dynamic-call effects; unknown binding cannot acquire authority. |
| VBC-012 | Grant absence/scope/expiry/revocation, denied disclosure and malicious schema/handle rejection. |
| VBC-013 | Parser/loop/stack/workspace/job/stream budgets, host cold/hot limits and safe exhaustion. |
| VBC-014 | One graph snapshot, stage/validate/commit, stale revisions/conflicts and unforgeable receipt meaning. |
| VBC-015 | Several subject identifiers and disputed/historical mappings survive Vibe → host → HMC → query/picking. |
| VBC-016 | Reactive propagation/order/cycles; repeated view rendering does not rerun effect cells. |
| VBC-017 | Native controls/viewport/input/accessibility and WASM projection share typed state/commands. |
| VBC-018 | Clock/seed/tween/hooks, pause/resume and replay with external observations recorded. |
| VBC-019 | Multiline, incomplete/invalid distinction and variable/function/import persistence over submissions. |
| VBC-020 | Parse mode selected before execution; one effectful request is not repeated on failure/fallback/reconnect. |
| VBC-021 | Compatible/incompatible live edits, migration, safe activation, no duplicate hooks/resource leaks and previous source recovery. |
| VBC-022 | Real await suspension, responsive UI, typed result and resumed authority/revision checks. |
| VBC-023 | Job cancellation/completion race, late result, progress, stream backpressure/disposal and indeterminate recovery. |
| VBC-024 | Descriptor/call parity for shorthand/named invoke, typed resources, bounds/lifetime/device-loss recovery. |
| VBC-025 | Two contributor/reviewer contexts, shared revisions/conflict/undo, durable checkpoint/restart/rejoin. |
| VBC-026 | Formatter/type/effect/completion/diagnostic tooling preserves source meaning and links results to exact source. |
| VBC-027 | Native/WASM-local equivalent core results and explicit unsupported/held required operations. |
| VBC-028 | Assisted provider negotiation/grants/disconnect/reconnect without duplicate effects or lost source. |
| VBC-029 | Source/AST/HMC provenance and cache invalidation; optional VM subset differential results and pre-execution path selection. |
| VBC-030 | Domain units/frames/laws/modal/disclosure/instrument results preserve evidence and authority distinctions. |
| VBC-031 | Actual native Poet reference-asset authoring/live edit/cooperative review → reproducible HMC and accepted derivative, IP-03 C1–C8. |
| VBC-032 | Claim gate rejects missing/stale/duplicate/fabricated scope/results/human review; every applicable requirement/case has an evidenced disposition. |

## 5. Completion boundary

Do not call the language, authoring profile or Ecopunk package done on the basis
of these documents. Every applicable REQ-VIBE/VBC row needs an explicit claim;
every VIBE packet needs source, executed evidence and review for its allocated
scope. Unused optional profiles remain unqualified, not falsely passed.
The model catalogue's 193 base obligations and expanded cases still govern
assets; language conformance does not prove species/anatomy/film/game/cartoon
quality. Human visual/domain acceptance and actual game-demo use remain required.
