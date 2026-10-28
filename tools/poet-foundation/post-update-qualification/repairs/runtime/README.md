# Runtime repair candidate — VQ-01–04

These are staged candidates for coordinator review/promotion, not saved sibling changes.
`manifest.json` records canonical originals and candidate SHA-256 values. The worker
ran staged-file Rust formatting only; no Cargo build or tests ran in this track.

The existing task registry owner remains in Vibe. Runtime `Env::task_scope` owns an
opaque scope independent of authored record fields and human-principal authority.
Default environments isolate scopes. Host-created Env clones intentionally share
their task scope, including effects; a source snapshot is not an effect rollback.
Final scope disposal removes retained entries. Numeric status lookup is refused;
copied foreign-scope task records cannot read, cancel or attach a deadline.

AST syntax await and explicit task.await use the caller's Host, environment,
remaining budget and phase gate. One registry executor claim prevents concurrent
or recursive duplicate execution. Source mode is selected from complete parser
results before checking/evaluation, handling comments, requirements and arbitrary
declaration headers; bare-expression compatibility fallback is parse-only. Checking/evaluation
errors remain failures. Checked program requirements must fit caller grants; task
program imports/program/grant context are restored after evaluation. Expression
payloads use the checked Pure-cell contract. Native calls remain non-preemptible:
cooperative expression checkpoints and a pre-commit deadline check withhold a late
result without claiming rollback of an attempted external effect.

ID/family matching is not full attenuation or human authority. A child requirement
is admitted only when its current parent Program exposes a matching unconstrained
declaration and the current Engine grant IDs also cover it. Constrained child
declarations, constrained-only parents and unavailable parent constraint metadata
refuse with a precise owner-evaluation diagnostic. The local task profile does not
invent context/resource entailment; adopted permission remains enforced by the
inherited Host and existing native owners.

Supported task behavior is bounded local deferred synchronous execution. Other
provider nodes, standalone task calls and bytecode task execution fail explicitly
without constructing a new LocalHost or budget. Standalone await of an ordinary
non-task value retains its identity behavior. Pending tasks can be cancelled
immediately; running cancellation is requested and acknowledged at a checkpoint.
Observers and timeout setters keep a blocked Running callback Running with a
request and no acknowledgement; only its executor checkpoint, return or unwind
acknowledges stopped local execution. Nursery aggregates count actual acknowledged
new sibling cancellation separately from accepted cancellation requests.
Terminal outcomes are immutable. Runtime deadlines can tighten, never extend;
caller elapsed/status/result flags are not authoritative.

Compatibility changes are explicit: registry-changing operations require the
appropriate effect declaration, dynamic await/task.await/nursery conservatively
require External until typed child-effect propagation exists, numeric IDs are no
longer handles, remote launch refuses, and detached records cannot certify jobs.
Checked local fixtures use the existing `effect async fn` spelling with
`load_program` and `eval_function`; no new admission constructor is introduced.
Literal `await 42` stays Pure. Legacy task metadata neither grants scope nor supplies
elapsed time. Nursery validates all handles before running and processes a
sequential group, cancelling remaining work after a real registry failure.

Limits are 256 entries/scope, 4096/process, 65536 source bytes, eight nested tasks,
1 MiB/result Debug representation, 32 retained-value/AST nesting levels, 16384
traversed nodes/elements, 8 MiB/scope and 64 MiB/process retained source
plus result representation. These are accounting limits, not an exact heap-size or
zero-allocation claim. Program diagnostics and registry overhead are additional
bounded metadata. Caller evaluator budgets still apply.

Twenty-two new regression sources cover checker admission and a checked positive
program, runtime isolation, both AST await routes, terminal immutability, timeout
and shared budget, single-executor claim, scope disposal/capacity, actual concurrent
join refusal using a blocked synthetic host callback, post-native-call timeout
without retry and inherited phase refusal. They include complete commented child
program execution/grant admission, bytecode compile/admission refusal before an
earlier host effect, literal await compatibility and static host calls. They
require coordinator execution.

Bytecode compilation declines scoped task operations (including imported aliases)
and dynamic await/callees before returning a Chunk. VM entry points independently
scan an entire decoded/manual Chunk before any instruction can cause a host effect;
scoped task paths, dynamic host calls and non-instruction-boundary branch/function
targets refuse. There is no execution fallback. Literal await compiles to identity.
Static namespace calls retain dispatch through a real named path rather than the
old dynamic-callee fallback that returned Null. Op::CallUser operand metadata is
corrected to its existing wire layout (u16 function + u8 argument count); no wire
format change is introduced.

New focused implementation files remain below 500 lines. Existing eval/mod.rs is
oversized, predominantly existing helpers/tests; this change adds only runtime
state/module wiring there and extracts new context behavior to eval/task_runtime.rs.
Existing eval/call.rs and expr.rs receive bounded routing/checkpoint changes;
their broader decomposition remains a separate owner review. capability_schema.rs
is an existing static registry/test owner; the new contracts live in a focused
runtime_entries.rs. This candidate does not establish full dialect conformance,
native human-authority admission, asynchronous suspension or remote execution.
