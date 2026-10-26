# RUN-05B pinned 0.1 compatibility and bounded scope

This is an implementation candidate for ENG-13 / ECO-034/044 / VIBE-02/03/04 /
VBC-004/011/019. It does not release VibeScript 0.2. `LANGUAGE_VERSION` and
`HOST_VERSION` stay `vibe-0.1` and `vibe-host-0.1`.

## Explicit authority corrections

Local function calls no longer inherit the old `binding_effect` fallback Pure.
Forward/recursive effects reach callers, including tick hooks, pipelines and
admitted collection callback aliases. Unknown callable forms/host paths fail
closed; unsupported dynamic record/index callbacks remain unproven External.
Known noncapturing Pure lambdas and builtins remain usable. Bind/For/Match/block
writes invalidate callable facts; known aliases copy facts by value, never live
references to a mutable binding. Builtin/enum/function dispatch precedence follows
the actual evaluator.

The existing dispatch catalogue omitted effect classes for host clock/field/law,
cue delivery, DAG execution, HID access, signing and key generation. Those
effectful seams now require External context; the new session additionally
requires a requested capability already covered by actual host-admitted grants.
Legacy stateless requires/using authorization semantics remain their documented
0.1 contract; a SourceSession source cannot grant itself authority.

Eight existing evaluator unit dispatch/type fixtures now use `effect fn main`;
their expected Host E702/type E100 errors remain unchanged. The crypto fixture
`crypto1_hash_sign.vibe` marks only `sign_data` and `new_ed25519_key` External.
The original `g1_tick_hook.vibe`, `h1_tick_counter.vibe` and
`h3_tick_time_publish.vibe` bytes remain unsafe-negative fixtures: their Hot tick
hooks call External helpers. Their conformance tests now expect E200 before
dispatch rather than treating the authority bypass as a positive behavior.
`n4_unknown_math_fn.vibe` remains unchanged; its unknown operation is rejected
during checking (E200) instead of being admitted Pure until runtime E100.

Budget expressions are checked Pure. They may use operators/builtins, but cannot
enter user functions in the bounded budget profile; both checker and evaluator
enforce this with E400. This closes recursive budget evaluation before function
body admission. Budget evaluation errors propagate with their original
Diagnostic instead of `.ok()` swallowing them. The first64-depth attempt failed
with a real Windows test-thread stack overflow; that failure is retained in
`verification-fourth.log`, not hidden by raising the stack.

## New SourceSession profile limits

This additive in-memory profile preserves types/mutability/imports/functions/
hooks/enums across submissions and runs only new items. It rejects definition
replacement, nested global declarations, and top-level return paths; these
would require explicit scope/migration/exit-flow contracts before safe metadata
promotion. Function locals keep actual noncapturing 0.1 behavior. Bind endpoints
require declared mutable roots. Primitive/List/Record type preservation is
validated, without claiming complete static type inference/domain type coverage.

The new profile uses an explicitly bounded call stack (default/clamped maximum8)
and a total expression-evaluation depth budget64 shared across function calls,
and the canonical-lexer ingress guard (16delimiter groups /64conservative path
units). Complex valid syntax may need smaller submissions; interpolated source
is held until embedded parser bounds share the canonical guard. Flat wide data
and ordinary statements remain supported. Legacy stateless Engine constructors
retain their existing64-call setting; those APIs are not qualified untrusted
source ingress by this work.

Source/revision/declaration/binding/interpreter workspace and retained-value
representation ceilings are admission controls, not allocator/process isolation.
The guard is a bounded source-session profile, never an Ecopunk master quality
ceiling. Durable shared checkpoints, source replacement/live jobs, cooperative
project authority and native graphical Poet/Craft authoring remain open.

On runtime failure the last accepted local session survives; external host state
may already have changed. The original Diagnostic (including fixes/evidential
fields) is returned, no retry occurs, and host reconciliation is explicitly
required. Unit/default Receipt values are not verified authoritative receipts.

An isolated default-stack probe demonstrated that8calls×60unary expressions
overflowed despite each independent source/call ceiling. The shared expression
budget now returns E400 for that composition. Separate child processes then
passed64path-unit unary and postfix cases,16groups, the fail-closed composition,
and8calls with14nested groups/4unary operators per body. No global Rust test
stack increase was used. These are qualified Windows/native profile cases,
not a proof of allocator isolation or arbitrary native Host safety.
