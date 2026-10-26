# VibeScript 0.2 — persistent REPL and native host contract

**Status:** proposed v0.2, 2026-10-24. Companion to the
[language definition](vibescript-0.2-specification.md); protocol fields below are
candidate contracts, not existing API names or implemented endpoints.

## 1. Session and submission

A session records a session reference, dialect/profile, project/document context,
actor identifier associations and role/authority reference, graph snapshot,
source/dependency revisions, actual grants, budgets, module/function table,
bindings/cell graph, installed hooks/views and outstanding job references.
Secrets and unrestricted host objects never become ordinary inspectable values.

The host provides three explicit source modes: expression, program and module.
The pure expression form accepts `= expression`; the REPL may infer this mode
for a bare expression **using parsing only**. Parsing produces Complete,
Incomplete or Invalid with spans. Incomplete input keeps a multiline buffer;
entering one newline cannot execute a half-entered block. The user can cancel
the buffer without changing the session. Delimiter counts alone are insufficient
because strings, comments and grammar context matter.

For each submission:

1. Preserve original source and assign submission/request references.
2. Choose mode/dialect and parse without executing either alternative.
3. Resolve imports/types/effects against the selected prior session revision;
   validate new declarations, grants, budgets and expected document revision.
4. Construct a candidate session revision and effect plan. Present necessary
   authority decisions through the canonical host. A declaration cannot grant itself.
5. Activate/evaluate the admitted candidate once. Record the value/diagnostic,
   state changes, jobs and verified receipts against that exact revision.
6. Reconcile pure dependents and presentation; append an inspectable history record.

A runtime failure must not cause expression/program/backend fallback. Preserve
the original error and committed-effect receipts. A request reference deduplicates
retransmission; it is not a universal promise of exactly-once external delivery.

## 2. Persistence and redefinition

Bindings, named functions, enums, imports, cells and hook definitions survive
later submissions within a session. A function's resolved module revision and
effect signature are retained; it cannot depend on a temporary evaluator object
that disappears after the declaration. Parameters/locals do not leak globally.
The following is a **required proposed REPL transcript**, not a recorded pass:

```text
vibe> let radius: f64 = 2.0;
vibe> pure fn area(r: f64) -> f64 { return 3.141592653589793 * r * r; }
vibe> area(radius)
12.566370614359172
```

The host distinguishes adding a declaration from explicitly replacing one from
an earlier submission. Replacement produces a revision with old/new source
references, dependency invalidation and a visible migration result. Do not
silently mutate a still-running continuation's function body. An incompatible
type/effect/interface replacement is rejected until dependent code/state has a
checked migration or explicit reset. Prior accepted source remains recoverable.

On syntax/type/effect/grant failure, retain the previous session unchanged. On
pure runtime failure, discard tentative local binding changes. On a transaction
failure, discard its staged changes. An external command already committed by
the host cannot be undone by discarding the REPL frame: retain the receipt and
show committed/partial/indeterminate status with the failure. Do not rerun it
while trying to recover a prettier diagnostic.

Proposed shell commands are `:help`, `:type`, `:env`, `:capabilities`, `:jobs`,
`:cancel`, `:history`, `:load`, `:save`, `:checkpoint`, `:reset` and `:quit`.
These are host UI commands, not language keywords or proof that today's CLI has
them. `:load` resolves through admitted storage; no ambient unrestricted read.
History recall previews source; it does not replay effects. Reset disposes owned
views/subscriptions/handles, reconciles pending jobs and asks only for genuinely
destructive loss. It does not delete shared project history or revoke others' work.

## 3. Capability descriptors and invocation

Every callable host operation has a descriptor containing:

| Descriptor field | Required contract |
|---|---|
| Operation reference, version, digest, owner | Canonical public family/method and implementing authority; aliases resolve explicitly. |
| Inputs/outputs/errors | Checked schemas including units, identifiers/revisions, opaque handle types and finite-number constraints. |
| Effect and determinism | Minimum effect, pure/snapshot behavior, seed/clock needs and replay classification. Unknown operations cannot default to Pure. |
| Authority | App/origin/session/role/resource scope, required capability, policy/consent, expiry and revocation. Requests do not equal grants. |
| Targets/providers | Native, WASM-local, assisted coverage; provider/ABI/source versions and availability. |
| Limits | Steps/workspace/results/deadline, GPU/resource use, stream/job limits and cancellation behavior. |
| State/receipts | Canonical state owner, expected revision, idempotency/duplicate behavior, actual commit/finality meaning and recovery procedure. |
| Resources | Handle provenance, size/format, ownership/lease/lifetime, transfer/disposal and loss/rebuild behavior. |

`Family.method(…)` and `capability.invoke("Family.method", {…})` resolve the same
descriptor and enforcement. Reflection/completion can reveal only admitted
metadata, not private data or secret capabilities. A broad catalog lease must
not bypass per-resource grants. Validate host results as rigorously as inputs.
There is no arbitrary native FFI, shell, filesystem, network or GPU entry point
merely because the interpreter runs on the desktop.

One invocation envelope carries request reference, operation/version/schema
digest, source/submission/call-site references, project/document/subject/part
references, expected revision, actor/authority references, grants, arguments,
input handles, budgets/deadline, clock/seed and dependency versions. This is an
abstract contract to reconcile with existing `vibe::Host`/Poet APIs, not a
new parallel transport. Native calls and assisted transport share the meaning.

An outcome distinguishes completed value, accepted pending job, rejected,
conflict, cancelled, unavailable, unsupported and indeterminate. It carries
validated result/diagnostic, actual source/result revisions, receipts and
resource/job references where applicable. Transport success is not commit;
accepted queue placement is not completed work; `Receipt` cannot be constructed
by a script literal or stand in for human acceptance.

## 4. Jobs, futures, streams and external effects

Jobs expose submitted → admitted/queued → running → completed, failed or
cancelled, with separate rejected and indeterminate outcomes where needed.
The provider declares which state transitions are durable. Cancellation is a
request until acknowledged; completion racing with cancellation is reconciled
by recorded state, never silently labelled undone.

`await` suspends the script continuation while native UI remains responsive.
On resume, validate result schema, authority, owner/provider and expected source
revision; revoked grants or stale revisions cannot auto-apply output. Keep a
recoverable un-applied result when policy permits. Progress is informative;
100% is not a receipt or visual acceptance. Streams have bounded queues,
backpressure/overflow policy, sequence/checkpoint references and disposal.

Disconnect distinguishes lost transport from failed operation. Reconnect first
queries request/job/receipt state; it cannot blindly resubmit effectful work.
If the provider cannot establish whether an external effect occurred, show
indeterminate and require reconciliation through the owning service. Do not
claim distributed exactly-once behavior or rollback without that service's proof.

Geometry, rendering, rig solving, textures, media and simulation reuse the native
owners and Etalon device/resource path. Large buffers stay in typed handles;
bounded streams/chunks/residency manage transfer. One script parameter edit must
not automatically rebuild the whole Ecopunk world or duplicate device ownership.

## 5. Live edits, UI and state migration

Parse/check a candidate off the active event path. Determine affected definitions,
views, resources and jobs. Pause at a declared safe activation boundary, reconcile
pending effects, migrate compatible persistent values and atomically activate
the new revision. Retain old revision/results for diagnosis and recovery.
Invalid code leaves the last accepted scene/editor usable with visible errors.

An edit cannot silently reset selected subject, artist source, camera, project
role or pending review. Incompatible state requires an explicit mapped migration
or a visible scoped reset decision. New/removed hooks dispose or install once;
updates cannot multiply subscriptions, leak resources or replay old commands.
Run continuations under their pinned definitions or explicitly cancel/migrate
them; never splice incompatible locals into new code.

Poet projects typed view state into native Aether controls/viewport and supported
WASM surfaces. User events become typed, revision-aware commands handled by Vibe
and the canonical native owner. Author scripts can change behavior/formulas/UI
without rebuilding Rust. Kernel/ABI changes remain explicit upstream builds.
Graphical authoring, accessible input, selection, timeline and undo/reopen are
required; a functioning text REPL does not close IP-03 C1–C8.

## 6. Cooperation, checkpoints and recovery

A shared checkpoint contains versioned source, dependency/resource references,
committed document revision, contribution/review records and recoverable job
references under source/policy authority. Ephemeral secrets and nonserializable
handles are excluded; reacquire handles through their admitted owners. Pure
state may replay deterministically from recorded inputs. External history is
reconciled from receipts, never replayed as if it were pure initialization.

Concurrent submissions use expected revisions. Conflicts preserve both source
contributions and produce an inspectable resolution; local latest-wins bindings
cannot overwrite another contributor. An agent cannot approve its own human
review claim or promote an unverified job into accepted model work. Several
identifiers may refer to a contributor/subject; role and authority are separate.

Recovery demonstrates application restart, provider disconnect, lost device,
revoked grant, cancelled job, malformed import and stale source/output. Preserve
accepted work, show which operations are held and let independent admitted work
continue. Chora stays separate; its adapter must not introduce JS/V8 into the
required native path. Live payment adapters remain a later support track.

## 7. Required operational demonstration

Use actual native Poet to load a reference asset, edit a source function and
material/behaviour parameter through the REPL/UI, receive an incremental native
preview, checkpoint, have another authorized contributor/reviewer revise/review,
reopen and reproduce the exact HMC/derivative. Include a deliberately invalid
edit, conflicting edit, late cancelled geometry job and restart/rejoin. Show no
per-script Rust/Cargo/JS build step and preserve the accepted master/source.
Record actual identifiers, revisions, outputs, latency/work counters, errors,
effects and reviews. This is a required future acceptance run, not a run claimed
by this document; an attractive screenshot alone cannot satisfy it.
