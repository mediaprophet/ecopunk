# VibeScript 0.2 — source-evaluated application and REPL language

**Status:** proposed language definition v0.2, 2026-10-24; not a released or
implemented dialect. The inspected implementation still declares `vibe-0.1`
and `vibe-host-0.1`. Ratification, implementation and qualification are separate.

VibeScript replaces JavaScript as the authored application, interaction,
automation and REPL language of Webizen/QualiaDB. Poet is its human authoring
environment. Editing or running a script **must not require compiling Rust,
running Cargo, generating a WASM binary, Node, V8 or a JavaScript translation**.
The installed interpreter and native capabilities can themselves be built in
Rust. Adding a native engine capability can require a host build; changing the
script that uses an already installed capability does not.

This proposal consolidates, rather than discards, the substantial existing
0.1 definition and source. Read the [source audit and work order](vibescript-0.2-source-and-conformance.md),
[REPL/host contract](vibescript-0.2-repl-and-host-contract.md),
[candidate grammar](vibescript-0.2.ebnf) and [claim starter](vibescript-0.2-claims.template.json).
The [governing plan](../implementation-plan.md) remains the delivery order.

## 1. Authority, versions and scope

Within this **proposal**, MUST/MUST NOT express candidate acceptance requirements,
not claims about today's interpreter. Existing 0.1 packages retain their pinned
semantics. Source behavior that differs from this proposal is migration work,
not permission to silently change 0.1. The audit records known differences.

Language version, standard-library/catalog version, host ABI, canonical AST
schema, bytecode format and package format have independent identifiers/digests.
The enclosing document/package or REPL session MUST select the dialect explicitly.
An unversioned legacy source uses its documented 0.1 compatibility profile;
it MUST NOT silently acquire 0.2 semantics. No new `version` keyword is required.

| Profile | Required coverage |
|---|---|
| `vibe-core-0.2` | Source lexer/parser/checker/evaluator; scalar/structured values, functions, control flow, bounded execution, modules, diagnostics, pure REPL and capability negotiation. |
| `vibe-authoring-0.2` | Core plus persistent multi-submission sessions, reactive cells/presentation, graph transactions, typed host commands, async jobs, live edits, revision/review/recovery and native Poet controls. Required for production-readiness claims. |
| `vibe-wasm-local-0.2` | Same language semantics for declared locally supported capabilities, explicit resource limits and unavailable operations. Required product cases are not waived by a narrower profile. |
| `vibe-wasm-assisted-0.2` | Same language plus authorized running native services, version negotiation, jobs, disconnect and reconnection. No second application state/policy engine in JS. |
| Domain extensions | Versioned descriptors for geometry/materials, animation, fields/laws, quantities, semantic instruments, disclosure and other existing families. Syntax and semantics used by a package are declared and qualified. |

These are proposed profile names, not discovered release labels. A core-only pass
cannot certify the authoring profile or the whole ecosystem. A required operation
outside a selected profile remains an open dependency. Domain extensions register
operations/types through the shared catalog; app-specific language forks are prohibited.

## 2. Artifacts and evaluation model

1. `.vibe` source is UTF-8. Preserve original bytes for attribution and editing;
   canonical source uses no leading BOM and LF newlines for its separate digest.
   Record which bytes each digest covers. Normalize lexical identifiers to NFC;
   do not normalize string/IRI contents as an accidental source-hash operation.
2. The mandatory path is **source → parse → resolve/type/effect/budget check →
   checked AST → evaluate**. Parse/check is not a user build step. Implementations
   may cache checked ASTs, keyed by all relevant versions, dependencies and grants.
3. Bytecode is an optional optimization/interchange artifact. It must preserve
   semantics and diagnostics for its declared feature subset. Missing VM coverage
   selects the admitted AST path before execution or returns unsupported; it must
   not evaluate part of an effectful input and then retry it on another engine.
4. A cell body `= expression` is pure formula syntax. A program contains
   declarations/statements; a module load installs checked declarations without
   running hooks or external effects. Explicit program execution runs admitted
   top-level statements once. Hook dispatch is a separate host action.
5. No JIT, JS translation, ambient DOM, prototype chain, implicit browser globals
   or string `eval` is required or admitted as the application execution model.
   Authorized REPL submissions use the same checked source gateway as files.

## 3. Lexical and syntactic contract

The [EBNF](vibescript-0.2.ebnf) defines the candidate structural syntax. Lexical
terminals in that file are governed here. Whitespace separates tokens; comments
are `//` to newline and non-nested `/* … */`. Newlines do not terminate statements;
simple statements use `;`. Compound blocks do not require it. No automatic
semicolon insertion. Incomplete REPL input is a distinct parser result.

Lexical identifiers use Unicode XID_Start/XID_Continue plus `_`, NFC, at most
255 Unicode scalar values. Reject bidirectional controls; diagnose confusables
and apply a declared mixed-script policy consistently in source and tools.
English keywords always work; opt-in keyword locales round-trip through the AST.
Locale changes presentation/spelling, never numeric units, capabilities or authority.
Reserved words are the quoted word terminals in the grammar; contextual property
names remain usable after `.` and as record keys. Unregistered syntax is an error.

Strings use double quotes, escapes `\\`, `\"`, `\n`, `\r`, `\t`, `\u{…}` with valid
Unicode scalars. No backtick, regex or interpolated-string syntax is admitted by
this candidate core; existing interpolation/destructuring experiments must be
explicitly reconciled during VIBE-01/02, not silently deleted or called absent.
String indices/length use Unicode scalar values in 0.2; byte access is explicit.

Integers use decimal or `0x` hexadecimal digits with internal single `_`
separators and optional `i32/u32/i64/u64` suffixes. Decimal defaults to i64,
hexadecimal to u64. Floats have a fraction or exponent and optional `f32/f64`.
The sign is a unary operator; range-check the resulting signed literal, including
the minimum i64 value. No literal NaN/infinity. Colours are `#RRGGBB` or
`#RRGGBBAA`, interpreted as sRGB with straight alpha and converted explicitly
for a renderer; colour is not a unitless simulation vector.

IRIs use `<…>` with no raw whitespace/control characters. Prefix declarations
expand `prefix:local`; `_:` introduces a source-scoped blank node and `?name`
a query variable. Follow the existing whitespace disambiguation for relational
`<`, `>`, `<=`, `>=`: separate them from operands. Multi-character structural
tokens (`<<(`, `)>>`, `<<`, `>>`, `<->`, `|>`) take priority over IRI scanning.
IRIs are identifiers, not automatic network fetches. Quantity suffixes are a
registered unit token or unit IRI on a number; absent registration is an error.

Precedence, low to high: pipeline; tween; `||`; `&&`; equality; ordering;
addition/subtraction; multiplication/division/remainder; unary; postfix.
Binary operators are left-associative. `if c { a } else { b }` has one expression
per branch and mandatory final else; postfix use requires parentheses around
the entire conditional. Nesting and recursion have declared limits.

`value |> f(args)` evaluates value once, then calls f with that result as its
first positional argument followed by args; `value |> f` calls f(value). Other
right-hand forms are rejected. A tween's endpoints have the same interpolable
type; `over` is a positive typed Duration, with registered easing/spring
parameters and a selected clock. No hidden loop or wall-clock read is implied.

## 4. Values, types and operators

| Family | Defined meaning / boundary |
|---|---|
| i64, u64, f64, bool, string, bytes | Fixed 64-bit integers, binary64 floats, Boolean, Unicode text and explicit byte sequences. No truthiness or string-to-number coercion in 0.2. |
| i32/u32/f32 spellings | Compatibility aliases for i64/u64/f64 in this candidate, matching inspected type resolution. They do **not** promise narrow storage, precision or range. True narrow types require a later explicit design/migration. |
| List<T>, Record | Finite ordered homogeneous lists; finite string-keyed records with inferred field types. No prototypes, getters or hidden host state. Duplicate keys are rejected. |
| Option<T>, Result<T,E> | Distinct `Some(value)` / `None`, `Ok(value)` / `Err(error)`; null is not None. `?` propagates Err or None only in an enclosing compatible function, never a grant or silent default. |
| Named enums | Namespaced variants, optional typed payloads and exhaustive matching; no numeric-enum coercion. Constructor notation `Enum.Variant(…)`. |
| Iri, BlankNode, Did, Hash, Var, Literal, TripleTerm, Reifier | Distinct validated graph values; lexical strings do not acquire these types implicitly. A DID is one possible identifier, not the whole subject or its authority. |
| Quin, QuinRef, Continuant, SpeciesRef, AssetRef, TensorRef, GeometryRef, FieldRef, MaterialRef | Host-validated immutable values or opaque handles with explicit type, owner, lifetime and source/version references. No raw pointers or author-written binary overlays. |
| Instant, Duration, Quantity, Frame, Pose, Transform, WorldLine, Mixture | Typed domain values with explicit clock, unit/dimension, frame/convention or composition metadata. Registered semantic operations govern conversions. |
| Future<T>, Stream<T>, Receipt | Host-coordinated jobs, bounded streams and verified operation outcomes, respectively. Receipt is not merely a success marker and cannot be forged in source. |
| Null, function values | `null` is its own sentinel type; noncapturing lambdas have checked argument/result/effect types. Capturing closures are not mandatory core scope. |

`Unknown` is an analysis state, not an unrestricted runtime escape hatch. A
checker must resolve it or report an explicit dynamic-host schema boundary before
execution; all host inputs/results are validated. Pure failures cannot fabricate
default geometry, Null, approval, disclosure or settlement success.

Arithmetic requires the same resolved numeric type; use explicit checked
conversion for mixed types. Integer overflow, divide/remainder by zero and
out-of-range casts fail; signed division truncates toward zero and remainder
has the dividend's sign. Integer/float conversion cannot silently lose precision.
Binary64 uses IEEE arithmetic, except division/remainder by zero is diagnosed.
NaN equality is false, inequality true, and ordering comparisons false. Schemas
requiring finite numbers reject non-finite inputs/results. Quantity addition and
comparison require compatible dimensions and explicit common units; products/
quotients derive dimensions. Hosts may not treat minutes and currency as one unit.

Equality is structural for immutable data of compatible types and defined by
the descriptor for opaque references. String comparison is scalar lexicographic;
locale sorting is an explicit library call. Records have canonical key order
for serialization, not observable mutation order. Conditions, `!`, `&&`, `||`
require bool; logical operators short-circuit and return bool. This deliberately
replaces 0.1's observed truthiness only in the proposed 0.2 dialect.

Indexing is zero-based; bad type, absent field/key or out-of-bounds index is a
diagnostic/explicit Result API, not implicit null. Values are immutable snapshots;
`let mut` permits rebinding and checked updates through mutable local paths,
not alias-based changes to another binding or an underlying host resource.

## 5. Scope, functions, modules and control flow

Blocks and functions have lexical scope. `let` and parameters are immutable;
`let mut` permits assignment. Typed uninitialized bindings require definite
assignment before reading. `const` requires a pure initializer. Duplicate names
in one source scope are errors; explicit nested shadowing is allowed. REPL
replacement of a prior submission is a versioned operation, not duplicate syntax.

Function parameters are typed; return annotations are explicit or inferred to
one checked type. Calls have arity/type checks, left-to-right argument evaluation
and no implicit receiver/global object. Named arguments map to unique declared
parameters; no duplicate or positional argument after a named argument. Return
without value produces null; every reachable path respects the result type.
Recursion is allowed only within declared stack/step limits, never unbounded hot
work. Lambdas do not capture external locals or acquire external authority.

`for` iterates a finite List in order; stream iteration uses registered async
operations. `while` checks a bool each iteration. All paths charge budget.
`match` selects the first compatible arm; guards are not part of this candidate
syntax. Enums/Option/Result require exhaustiveness or `_`; incompatible payload
patterns are errors. `yield` is valid only in an admitted stream-producing async
context. Cancellation/budget traps cannot be swallowed to restart unbounded work.

Imports resolve only through the host's pinned package/module resolver, offline
where resources are present. No implicit filesystem/network access, mutable
remote latest version or inherited grants. Module exports are top-level functions,
constants, enums and declared cells addressed through the import alias; local
variables are private. Dependency cycles fail with a path diagnostic in this
candidate; initialization order is dependency order then source order. Module
load does not run top-level external effects or install hooks into a live session
until its checked revision is explicitly activated.

## 6. Effects, grants and bounded execution

The effect ordering is Pure < Hot < Cold < Async < External (`effect` in source).
Check transitive calls and dynamic catalog targets. An effect ceiling is a
restriction, never a grant. `using`/`requires` declare requested capabilities;
the host independently decides actual scope, app/session/actor authority,
resource access and expiry. Unknown operation descriptors fail closed.

Pure formulas read immutable admitted inputs/snapshots and cannot mutate graph,
UI state, files, processes or devices. Hot functions perform bounded frame work
over admitted buffers; they cannot hide heap allocation, blocking I/O, remote
requests or arbitrary cold geometry work. Cold functions can run bounded heavy
work; Async permits suspension; External permits authorized observable effects.
Async External functions use `effect async fn`; `async fn` has Async effect.

The host advertises step, stack, workspace, result, graph-scan, job, stream and
deadline budgets with units and ceilings. Budget clauses can lower limits, not
raise authority. Charge parser/checker work as well as evaluation; a host call
has independently bounded work, memory, GPU use and output. Exhaustion preserves
accepted source/state and emits a typed outcome, never silently lower quality.

The inspected evaluator defaults to 100,000 steps and 42 MiB workspace. These
are **observed defaults**, not a universal production limit. Preserve the native
hot/cold budget distinction: high-detail geometry, textures, rigs, media and
rendering execute as bounded native jobs with handles/residency controls. Vibe
orchestrates them; it need not walk every vertex in a frame script. Actual cold
budgets follow the owning engine and caller policy, with explicit failures.

## 7. Graph, identifiers, transactions and semantic rules

`<<(s p o)>>` constructs a triple term; `<<s p o ~reifier>>` expresses a reified
assertion. Host sealing/validation creates stored Quins; source cannot change
binary parity, fabricate receipts or bypass graph policy. `graph? { … }` and
`graph? ask { … }` query one admitted snapshot, with explicit projection/order,
disclosure, pagination/scan budget and Result/stream schema supplied by the graph
profile. Lack of visible facts is not proof of absence or permission.

The candidate base graph-query body is a conjunction of triple patterns with
query variables, validated against the admitted RDF profile. ASK returns bool;
the finite query form returns List<Record> keyed by projected variable names,
with typed graph values. Unbound variables, unsupported filters or inferred
default ordering cannot silently alter results. Extended projection, ordering,
pagination, entailment and streaming use versioned graph-profile operations;
their complete descriptors accompany any package using them.

Every hook/evaluation records its graph snapshot. A `transaction` stages
validated changes, checks expected revisions, rights and schema, then commits
atomically through the canonical graph owner or returns a conflict/denial. No
partial silent commit. External effects spanning systems use the job/receipt
contract; local graph atomicity cannot promise universal external rollback.

[DOM-AX-7](../domain-model.md#dom-ax-7-multiple-identifiers-can-refer-to-one-subject)
governs subject identifiers: several scheme/source/authority/scope/time/status-
qualified identifiers may refer to one subject. A variable's lexical identifier,
subject identifier, record key, part reference and content digest have distinct
roles. Matching strings do not establish equivalence, authority or agreement.
HMC queries retain the full admitted knowledge, beyond visible mesh resources.

`obligate`, `permit`, `forbid`, `knows`, `believes`, `always`, `eventually`
construct/check semantic propositions through registered logic profiles. A
proposition is not an executed permission, signed agreement, payment or fact.
Its ontology, evidence, temporal horizon and evaluator version accompany results.
Law guards are pure; consequences are separately authorized revision-aware
commands, cycle-bounded and not autonomously granted by declaring a law.

## 8. Reactive cells, presentation, time and interaction

Cells form a dependency graph over named source revisions and admitted snapshots.
Recompute pure dependents in deterministic topological order; cycles are errors
unless an explicitly qualified delayed-state operator breaks the cycle. A
`when` guard is bool. A named effect cell executes only on explicit activation
or an admitted event, not incidental view rendering or pure recalculation.

`present` describes typed UI/view properties from values. The host projects that
model into native Aether/Poet controls and supported WASM views. Native UI must
not require DOM/CSS/JS behavior; existing CSS projection is a target adapter.
Selection, tool state, inspectors, timelines, input/accessibility and errors
share canonical commands/state. No UI property can secretly submit a mutation.

`bind a <-> b` requires writable endpoints with compatible units/types. Optional
registered clamps and `resolve latest|left|right` declare local synchronization;
`latest` uses a host sequence within one authority domain, not untrusted wall
clocks. It cannot resolve concurrent contributor revisions or erase a conflict.
Bindings suppress echo loops and dispose with their owning revision/session.

Tweens produce a clocked value plan (`from ~ to over duration ease name` or
registered spring parameters), not blocking loops. Explicit clocks separate
simulation time, monotonic durations and externally observed wall time; seeds,
inputs and external time observations are recorded for replay. Hook order and
frame sampling are deterministic under that record. Host tick handlers default
to Hot; other hooks declare/profile-check their permitted effect and grants.

## 9. REPL, jobs, reload and cooperative authoring

The [REPL/host contract](vibescript-0.2-repl-and-host-contract.md) is normative
within this proposal. It defines multiline submissions, persistent functions/
variables/modules, checked revision activation, exactly one execution attempt,
diagnostics, explicit state migration and durable recovery. Editing a behaviour,
material parameter, UI formula or interaction must update the running admitted
scene without rebuilding the native host or unrelated world assets.

`await` suspends a continuation on Future<T>, releases the UI thread and resumes
with a typed result under rechecked authority/revision. It is not a synonym for
evaluating its operand synchronously. Jobs expose progress, cancellation,
backpressure and receipt reconciliation. Late results cannot overwrite a newer
asset revision. Streams are bounded and require defined disposal/reconnect rules.

Cooperative sessions bind commands to project/document/subject references,
expected revision, actor identifiers/role, provenance and review state. Shared
source has one authoritative owner; local REPL state is not an unreviewed second
project database. Undo creates checked compensating/revision commands; it cannot
erase attribution or pretend to undo an irrevocable external action.

Semantic agreements, contribution/commons records and provider allocations use
the existing native contracts. Financial rails are supportable later work; no
language feature depends on real payments for the first asset-authoring outcome.
No-charge use, contributor obligation, pre-funded credit, assent and payment
remain distinct typed facts, never Boolean success returned by a script stub.

## 10. Diagnostics, tools, portability and packaging

Diagnostics carry stable code/category, source reference/digest/revision, UTF-8
byte span, related locations, safe message and optional non-authorizing fix.
Separate incomplete source, syntax/type/effect/grant/budget/policy/runtime errors,
conflict, unsupported profile, cancelled, unavailable and indeterminate external
outcome. Preserve existing E001/E100–E800 mappings; new subcodes need a registry.
Disclosure denial retains its meaning rather than masquerading as missing data.

Poet provides source editing, formatting, completion, hover/type/effect displays,
structured diagnostics, inspectable values/jobs, source-to-result navigation,
history and checkpoint/reopen. Native and WASM adapters consume the same checked
language service. An unresolved old tooling repository path is not proof that a
working LSP or editor integration exists. Formatting preserves literals/comments
and meaning; canonical AST/source round-trips preserve locale and spans/mappings.

Packages/HMC retain source, dialect/profile, AST/schema, pinned imports/catalogs,
capability requests, resource handles, provenance/terms and actual evidence.
Checked AST/bytecode caches are disposable, versioned derivatives. Signatures
authenticate declared bytes/authority, not visual quality or semantic truth.
No JS mirror may become the authoritative implementation of authored rules.

Native/WASM runs agree on language results, effects and error categories for
the same admitted inputs. Device-dependent GPU/image results use declared
tolerances and resource descriptors; they do not alter graph/permission semantics.
Assisted WASM exposes local/provider coverage and degraded/offline states. A
missing provider must not silently run a different algorithm or discard work.

## 11. Required claims and ratification

The [claims starter](vibescript-0.2-claims.template.json) includes each requirement
and conformance case, all unassessed. Every release claim needs profile, source/
dependency hashes, inputs, expected/actual outputs, diagnostic/effect traces,
target/backend, reviewer and remaining cases. Presence, wiring, execution and
accepted scope are distinct. A feature count or a new version constant is not
0.2 completion. Agent task closure must enumerate the applicable rows.

| Requirement | Acceptance subject |
|---|---|
| REQ-VIBE-001 | JavaScript-replacement role and source execution without per-script build. |
| REQ-VIBE-002 | Independent version/profile negotiation and pinned legacy dialect. |
| REQ-VIBE-003 | Complete lexical/grammar and locale/security contract. |
| REQ-VIBE-004 | Typed values, numeric rules, strings, containers and conversions. |
| REQ-VIBE-005 | Lexical scope, functions, patterns, control flow and bounded recursion. |
| REQ-VIBE-006 | Offline pinned modules and explicit load/run/activation. |
| REQ-VIBE-007 | Transitive effect checks and independently enforced scoped grants. |
| REQ-VIBE-008 | Parser/evaluator/host budgets and hot/cold separation. |
| REQ-VIBE-009 | Snapshot graph queries, atomic revisions and typed receipts. |
| REQ-VIBE-010 | Multiple subject identifiers and qualified mappings through HMC. |
| REQ-VIBE-011 | Reactive dependency/cycle/effect activation rules. |
| REQ-VIBE-012 | Shared native/WASM presentation, commands and input/accessibility. |
| REQ-VIBE-013 | Explicit clocks, seeds, animation and bounded simulation hooks. |
| REQ-VIBE-014 | Persistent multiline REPL and parse-before-execute diagnostics. |
| REQ-VIBE-015 | Checked live edits, state migration, stale-result protection/recovery. |
| REQ-VIBE-016 | Real async suspension, jobs/cancellation and bounded streams. |
| REQ-VIBE-017 | Versioned typed capability descriptors and opaque resource lifetime. |
| REQ-VIBE-018 | Cooperative source/roles/review/conflicts and reproducible jobs. |
| REQ-VIBE-019 | Structured diagnostics, language services and inspectable evidence. |
| REQ-VIBE-020 | Native, WASM-local and assisted portability/disconnect qualification. |
| REQ-VIBE-021 | Source/HMC provenance, manifests and optional cache/VM parity. |
| REQ-VIBE-022 | Registered domain semantics, disclosure and agreement distinctions. |
| REQ-VIBE-023 | Explicit 0.1 migration, source/tool preservation and rollback. |
| REQ-VIBE-024 | Exact claims coverage and real Poet/Ecopunk authoring demonstration. |

Ratification must settle and execute the explicit compatibility cases in the
audit: strict bool conditions, Unicode indexing, numeric conversions, Option
representation, import versions, grammar experiments, effects and receipt shape.
Until then this remains a comprehensive **candidate contract**, not an adopted
replacement for QualiaDB's normative 0.1 document. Preserve existing copyright,
licensing, source history and supported application behavior in any upstream work.
