# VibeScript testing review — 2026-10-27

Read-only source review for the agent testing the current updates. This is a bounded static inspection, not a complete language audit or an executed failure report. Sources are changing: recheck each finding against the tested revision before implementation. No VibeScript/Poet source changes, Cargo checks or asset production were performed for this review.

**Later same-day source update:** a new `task_scheduler.rs` and interpreter/bytecode routing were added during the subsequent expansion review. The direct-operand `await` and record-only task findings below describe the earlier captured source, not the later scheduler. The [expansion review's source-change reconciliation](vibescript-language-expansion-recommendations.md#current-runtime-honesty-findings-to-carry-into-testing) records the new wiring and remaining selected-path ownership/context/deadline concerns. Preserve this original receipt as historical evidence; recheck the current testing revision before assigning repairs.

Use the existing [0.2 specification](vibescript-0.2-specification.md), [REPL/host contract](vibescript-0.2-repl-and-host-contract.md) and [human-principal authority contract](human-principal-agent-authority.md). These recommendations refine those contracts; they do not create another language or state owner. Source pins and inspected scopes are in [the inspection receipt](../../qa/implementation-2026-10-27/vibescript-testing-review/source-inspection.json).

## Priority 0: qualify runtime promises used by production

### VT-01 — truthful code addressing and resolution

Observed in `crates/vibe/src/stdlib_v2/topology_and_manifold/effects_and_systems.rs`: `code_hash_ast` encodes the AST and computes `fnv1a_64`, then labels it `sha256:vibe:...`. `code_resolve` returns `resolved=true` for any nonempty string; `code_spawn_at` returns `spawned=true` without invoking a host in the inspected body.

Reuse the existing cryptography, package resolver and authorised native job owners. Use an accurate algorithm identifier and canonicalisation/version contract; an optional fast index must remain distinct from the cryptographic digest. Resolution must retrieve and verify bytes and compatible dependencies. Dispatch must return an actual admitted job reference or an explicit unsupported/error result. Descriptor construction must be named and labelled as such, never reported as an executed operation.

Acceptance: known digest vectors; changed bytes fail verification; unavailable content does not resolve; unavailable/unauthorised provider does not spawn; returned job references correlate with native provider evidence. These checks are requested, not run here.

### VT-02 — real asynchronous jobs, cancellation and bounded streams

Observed: the interpreter's `ExprKind::Await` evaluates its operand directly; the bytecode compiler explicitly leaves scheduling to the host. The inspected `task.cancel`, `task.with_timeout`, `task.nursery` and `supervisor.start_link` functions manipulate supplied records. Their bodies do not contact live jobs, wait on timers, cancel siblings or restart workers. The three `effect.*` helpers similarly construct or alter records rather than dispatching resumable continuations. This finding does not establish that every other native host path lacks scheduling.

Connect required operations to existing native job/effect owners, with checked continuation suspension/resumption, deadlines, cancellation acknowledgement, bounded queues/backpressure, disposal and provider-loss reconciliation. Recheck source revisions and authority on resume. A cancellation request must remain distinct from acknowledged cancellation and already-committed effects. Do not introduce automatic retries for an uncertain external write.

Acceptance: a real delayed job suspends; concurrent work progresses; cancel reaches the provider; resource use stops or the unresolved outcome is reported; deadline expiry is measured by the runtime; a slow consumer cannot grow an unlimited queue; effect handlers execute the intended continuation exactly within their documented policy. Unsupported operations must refuse rather than simulate success.

### VT-03 — human principal and delegated agent enforcement

Reuse native authentication, delegation and project/resource rights. Authentication, graph membership, a script-created record or a `Receipt` value must not grant authority. The current `Value::Receipt` variant alone is a marker, not independently verifiable execution evidence.

Acceptance: the same operation through Poet controls, MCP and VibeScript enforces the same adopted human-principal → agent chain, permitted operation/resource scope, budget, expiry, revocation and subdelegation limits. Recheck before publication and after suspended work resumes. Include forged receipts, changed resources, revoked grants and reviewer agents controlled by the same human. Preserve the existing AUTH-H work IDs rather than starting another authority implementation.

## Priority 1: complete the authoring and large-data path

### VT-04 — durable live editing and recovery

`SourceSession` already provides bounded checked in-memory submissions and host-supplied grants. Its module header explicitly excludes durable recovery, live definition replacement and a job API. Existing reactive cells, module metadata and replay-clock facilities should be reused.

Complete the required host integration: revision-pinned live replacement, explicit state migration, invalidation of dependent cells/jobs, source-mapped diagnostics, and durable project save/reopen through the native project/HMC owners. Recovery must not restore stale grants or replay external effects. Preserve scene selection, camera, private drafts and one subscription per hook when editing repeatedly.

Acceptance: edit → checked preview → undo/rebuild → save → fresh-process reopen; invalid edits preserve the last accepted source; incompatible migration is explicit; outstanding work cannot silently publish against a superseded source revision. Reuse C1–C8 trial preparation and existing evidence, not a second ledger.

### VT-05 — typed large-resource access and deterministic cleanup

Typed geometry/tensor/material/asset handles already exist. Extend and qualify their bindings to existing HMC/Q42/.10d/M64, geometry, media and Etalon owners where required. Avoid materialising whole embodiments or copying large meshes through ordinary lists for each frame. Query-dependent chunk access, cancellation and cache eviction must preserve identifiers, graph provenance and derivation lineage.

Acceptance: bounded progressive loading of a synthetic large package; deep inspection requests the required detail; backing resources stay valid while borrowed; disposal and device/provider loss produce defined results; an agent cannot forge an authorised resource handle. State reproducibility bounds for seeded algorithms, clocks, floating-point/device variation and dependency revisions rather than promising universal bitwise determinism.

### VT-06 — one truthful capability catalogue and useful authoring feedback

`capability_schema.rs` already defines schemas and honesty/effect labels. Build on that catalogue. Expose actual provider/profile availability, effects, limits, units, resource lifetimes, failures and authority requirements to checking, completion and agent discovery. Ensure dispatch/schema conformance covers these meanings, not only matching operation names. A supported descriptor utility and a supported executable operation require distinct declarations.

Acceptance: every production-profile catalogue entry either executes its documented behavior through its owner or refuses with a precise reason. Include negative cases for missing providers, exceeded budgets, unsupported native/WASM capability, stale handles and missing rights. Source locations and actionable diagnostics should lead a human or agent back to the offending authored expression.

### VT-07 — domain semantics without duplicate domain engines

Quantities, frames, replay clocks, semantic AST encoding and domain reference values are already present. Bind these to audited QualiaDB geometry, solvers and modalities. Preserve dimensional units, spatial/reference frames, uncertainty, biological state and graph provenance. Keep electromagnetic radiation distinct from acoustic/mechanical waves while supporting both within the wider sensory-cue model; do not silently treat sound as electromagnetic frequency.

Acceptance: a dimensionally invalid biological/material/physics operation is rejected; coordinate conversions are explicit; sampling and observations retain units and source records; unavailable domain operators cannot claim execution through descriptive records alone.

## Scope and dispatch

1. Testing agent: recheck VT-01/02 against its frozen source, including reachability through public dispatch. If repaired already, attach the actual regression evidence. If descriptor-only by design, classify it explicitly and stop advertising live execution.
2. Native integration owner: qualify VT-03/04/05 against existing project/authority/resource owners after the user announces VibeScript/Poet readiness. Keep Cargo checks serial. Do not edit shared manifests or dependency locks concurrently.
3. Catalogue/review owner: reconcile VT-06/07 with actual callable providers and record remaining gaps. A bounded review does not establish full runtime coverage.

No extra JavaScript compatibility quirks or broad new syntax are prerequisites for the first asset unless a required authoring case needs them. Maintain an explicit compatibility matrix: supported, deliberately different, host-dependent and deferred. In particular, the inspected interpreter checks lambdas as non-capturing; document that restriction or implement and test lexical capture if required, rather than claiming general JavaScript closure parity.

These are testing and integration recommendations. Asset production remains on hold; no native cooperative authoring, human acceptance or model-completion gate is closed by this document.
