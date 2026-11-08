# VibeScript — comprehensive language expansion recommendations

**Prepared:** 2026-10-27. **Status:** review and proposed development agenda, not an adopted language specification or an implementation-completion report.

**Expanded at Timothy's request:** material/“world of man”, legal and economic domains, with extensibility for future domains and language specificities as a governing requirement. LX-45–68 extend the original LX-01–44; no original proposal is removed. These domain profiles are examples of an open extension model, not the final list of built-in domains.

**Later testing update:** Timothy confirmed the updates were ready and authorised [post-update qualification](../../qa/implementation-2026-10-27/post-update-qualification/README.md). That report supersedes earlier capture-time runtime findings where current source repairs them, and records executed negative criteria separately from static review. The historical source findings below are not a claim that subsequently updated code still has each issue.

The recommendation is to develop VibeScript into a coherent language for interactive applications, cooperative authoring, scientific/domain computation and governed automation. Its strongest distinction can be the combination of readable source, live authoring, typed domain data, explicit effects and authority, inspectable execution and native high-performance services. Each part needs working semantics and usable tools. A large catalogue of names alone cannot provide this.

This document deliberately covers more than the immediate Ecopunk authoring requirements. **It does not make every expansion a prerequisite for producing the first exceptional asset.** The [governing implementation plan](../implementation-plan.md) retains the delivery order, IP-03 C1–C8 retains the native cooperative-authoring gate, and asset production remains on hold under the current instructions. Subsequent saved repairs and executed software checks are recorded in the linked post-update qualification; this broad review does not itself authorise competing source edits or builds.

The most valuable broad expansions are typed lexical closures and reusable generic data/functions; compositional effects with real task/resource lifetimes; durable, revisioned live authoring; a truthful typed native capability boundary; and integrated graph/scientific/media/UI libraries over the existing stack. Language services, debugging and conformance make those capabilities usable by people and agents. Optional syntax and compiler research should follow demonstrated needs. The immediate integrity/runtime corrections and the broader language agenda have separate scope and acceptance.

**Extensibility is foundational:** new terminology, human-language conventions, domain types, constraints, operators and interpretations must be definable over time through versioned, independently authored extension packages. Ordinary additions expressed as data, schemas and checked Vibe modules should not require a core-language rebuild. A genuinely new native kernel may require an admitted provider installation/build; it must not make every subsequent script or vocabulary edit require Cargo. Core safety, source evaluation and compatibility remain shared across extensions.

## 1. How to use this feedback

Read this alongside the existing [proposed 0.2 definition](vibescript-0.2-specification.md), [source/migration audit](vibescript-0.2-source-and-conformance.md), [REPL and host contract](vibescript-0.2-repl-and-host-contract.md), and [targeted testing review](vibescript-testing-review-2026-10-27.md). Preserve their REQ-VIBE, VBC and VIBE work IDs. The LX IDs below are recommendation references, not a replacement backlog or additional completion gate.

The review separates four kinds of work:

| Kind | Meaning | Example |
|---|---|---|
| Language semantics | How authored programs behave, independently of a particular application | Lexical capture, matching, numeric overflow and evaluation order. |
| Runtime/host contract | How checked programs interact with jobs, resources and privileged services | Cancellation acknowledgement, resource disposal and delegated authority. |
| Standard/domain library | Operations built on the language and existing native owners | Collection algorithms, geometry queries and spectral material evaluation. |
| Tooling/product integration | How people and agents author, inspect, review and recover work | Poet editing, debugging, completion and contributor/reviewer workflows. |

Prefer a library operation or typed descriptor over new grammar when that gives clear authoring. Add syntax when it materially improves recurring composition or correctness and has a complete parser/checker/evaluator/tooling design.

Priority labels are recommendations:

- **R0 — qualify or repair:** existing advertised behavior and production paths must be truthful. Only cases actually needed by the selected production profile block that profile; misleading success must still be corrected or explicitly excluded wherever exposed.
- **R1 — broad application language:** high-value expansion after the current semantic baseline is settled. These can be staged rather than bundled into an indefinitely growing 0.2 release.
- **R2 — advanced authoring and domain expansion:** implement against real product cases and existing native engines.
- **R3 — later research/optional optimisation:** valuable possibilities requiring evidence before becoming mandatory architecture.

Candidate surface spellings and data shapes in this review are **design suggestions**, not claims that the current parser or host accepts them. Preserve pinned 0.1 behavior; reconcile each incompatible proposal with the current testing branch before assigning implementation. Do not infer a new released dialect from a directory name or version constant.

## 2. What the current code already provides

Three scoped readers inspected language, runtime and authoring aspects; the coordinator checked critical bodies and selected cross-references. No Rust tests, interpreter programs, GPU checks, native windows or asset exports ran for this document. No clean-tree or frozen sibling revision was established. The filesystem changed during inspection, so the source receipt records capture-time hashes rather than a reproducible Git checkout. A hash does not turn a discovery-level read into a semantic audit.

| Existing source or contract | What can be reused | Evidence limit / follow-up |
|---|---|---|
| `crates/vibe/src/ast.rs`, `types.rs`, `parse/`, `check/`, `eval/` | Typed declarations, expressions, enums/patterns, checked source evaluation and diagnostics. | Trace each feature across all stages; syntax/type names alone do not establish behavior. |
| `value.rs`, `eval/expr.rs`, `eval/call.rs` | Function/lambda and structured-value foundations. | Inspected lambdas reject capture. `None` evaluates to `Null` on the inspected path; reconcile Option representation with the proposed distinction. |
| `session/mod.rs`, `session/prepare.rs` | Bounded, checked, additive in-memory sessions, candidates and revision validation. | Its header explicitly excludes durable recovery, live definition replacement and a job API. Do not repeat the stale claim that there are no persistent in-memory sessions. |
| Newly added `task_scheduler.rs`, interpreter/bytecode task dispatch | Registry-backed task references, cancellation/status handling and await routing now exist. | Added during this review. Selected await execution remains inline; owner/context, deadlines and terminal-state handling require qualification as described below. |
| `module_system.rs`, `cbor_ast.rs` | Module metadata/catalog/resolver helpers and semantic AST/value encodings. | Verify actual source loading, export checking, byte verification and activation policies. |
| `capability_schema.rs`, `catalog/`, `bind/dispatch.rs`, `effects.rs` | Capability schemas/catalogue, bindings and effect classification. | A unified trusted source of metadata and complete dispatch/checker parity still require qualification. |
| `reactive_cell.rs`, `presentation.rs`, `projectional.rs`, `ast_merge.rs`, `pretty.rs`, `trivia.rs` | Reactive dependency data, presentation, structural editing/merge and source tools. | Source presence is not evidence that every native Poet interaction and conflict case works. |
| `stdlib_v2/num_and_autodiff/` | Decimal routines, checked/saturating math, seeded PRNG and autodiff helpers. | Audit numerical contracts; decimal arithmetic bodies include saturating operations. Do not describe these as arbitrary precision or universally exact overflows. |
| `stdlib_v2/text_and_collections/` | Grapheme utilities, regex and collection families. | String indexing currently uses grapheme splitting while sampled `len`/`.length` use UTF-8 bytes; the 0.2 proposal specifies scalar indexing. This needs an explicit versioned decision. Full Unicode conformance is not established. |
| `quantity.rs`, `frame_morphism.rs`, `replay_clock.rs`, `animation/`, `physics/` | Units, frames, clocks, animation and domain descriptors. | Bind and qualify real native owners; do not recreate these families as parallel engines. |
| `law_package.rs`, `deontic_interrupt.rs`, `observer.rs`, `reflection.rs` | Governance, bounded agent phases and inspection concepts. | Their existence does not prove human-principal delegation enforcement, adopted agreements or safe reflective execution. |

### Added domain-reuse findings

The material/legal/economic amendment used three bounded readers plus coordinator checks. The following are selected source seams, not full-domain audits or fresh execution evidence. Paths in this table are relative to `C:/github/qualiaDB` unless stated otherwise.

| Source seam | Existing functionality to reuse | Boundary / qualification work |
|---|---|---|
| `crates/vibe/src/vocab/{mod,parse,shake}.rs`, `locale.rs`, `lexicon_pin.rs`, `module_system.rs` | Bounded vocabulary chunks/reference selection, keyword locale tables, pinned lexicon metadata and exported module metadata. | A bounded chunk parser is not a full ontology reasoner. Complete runtime-defined extension semantics, composition and admission need their own contract; keyword translation alone is not language-specific meaning. |
| `crates/qualia-core-db/src/specialized_libs/engineering_analysis/{mechanical,structural}.rs` | `ComponentDesign`, `MaterialSelection`, `AssemblyDesign`, `MaterialProperties`, selected model validation and analysis methods. | Selected engineering models do not establish the whole BOM/as-built/port/installation/lifecycle journal; trace the actual product owner before allocating a gap. |
| `crates/qualia-core-db/src/container_10d/material_section.rs` | `MaterialRecord`, texture-resource digests and material-section validation/encoding. | Representation/resource packaging is distinct from physical batch provenance and a material passport. |
| `C:/github/license/RIGHTS-ONTOLOGY-AND-INSTRUMENTS.md`, `ontology/rights-instrument-model.ttl`, profile/generator sources | Draft jural/authority/instrument/remedy framework, jurisdiction/time/evidence fields and draft/acceptance-template generation. | The inspected specifications are drafts, not adopted project terms; generated validation does not establish assent or legal effect. |
| `crates/qualia-core-db/src/modalities/logic/deontic.rs`, `semantic_instruments/` | `compile_n3_rule_to_norm`, `evaluate_deontic_contract`, norm lifecycle/conflict helpers and version-bound execution/attestation records. | Selected formal-rule and supplied-fact evaluation is not automatic jurisdiction selection or legal adjudication. An attestation/review category is not agreement adoption. |
| `crates/qualia-cooperative-core/src/{projects,finance,agency_delegation}.rs` | Contribution/predecessor and unique-event ledger mechanisms, derived obligations, scoped delegated-agency data. | Distinguish score/unit/valuation, actual backing, accepted obligations and adopted authority. Existing host paths may still use historical `wellfare_core`; full consolidation is not established. |
| `crates/qualia-core-db/src/specialized_libs/computational_economics/accounting.rs` | Actual double-entry `Posting`/`JournalEntry`, per-entry validation, account balances and trial-balance routines. | Accounting is already implemented source, not a new engine requirement. Qualification must include actual Vibe wrappers, checked output conversion and entry validation. |
| `crates/qualia-core-db/src/net/qdnf/economics/{obligation,reserve,settle,recover,adapter,consent,router}.rs` | Bounded obligation target/discharge/holds, reserve/finalise/reverse/reconcile, aggregation and separate credit/consent/resource mechanisms. | `MAX_OPS=16` requires honest capacity/checkpoint handling. `is_funded` means unused recoverable target in that helper, not verified external backing. Selected live rail functions return `Unsupported`; a full funded-allocation/calendar-close workflow was not established here. |
| `C:/github/humanitarian-equity-framework/ontology/cost-model.ttl`, `C:/github/semantic-tokens-xec/README.md` and historical app | Existing contribution/valuation intent and earlier RDF/SHACL/token authoring/protocol reference work. | Preserve draft namespace/history and source notices; historical UI/protocol source is not native integration, adopted terms or verified settlement. |

**Tracked static wrapper issue, LX-64-A:** the inspected `poet_host/invoke/econ/computational_economics.rs::econ_accounting_balance` passes an empty output vector to `account_balances_into`, whose first check rejects an output slice shorter than the accounts slice. Nonempty accounts therefore take `OutputBufferTooSmall` on that captured path. Reuse the accounting kernel, size the output correctly, and qualify the public Vibe call with meaningful nonempty inputs. This issue is recorded for the relevant integration owner; no runtime failure was executed here.

**Tracked contract checks, LX-64-B:** `poet_host/invoke/econ/finance_ext.rs::ledger_balance` allocates output but does not call per-entry journal validation in the inspected body and narrows `i128` totals with `as i64`. A balance projection may deliberately accept arbitrary postings, but must not be advertised as validation of a balanced journal; if that guarantee is claimed, call the existing validator. Preserve wide totals or return a checked conversion error. Include offsetting unbalanced entries and out-of-range totals in qualification. These findings do not imply the native accounting kernel or every ingress is broken.

The existing [biosphere integration audit](../../qa/implementation-2026-10-26/biosphere-integration-review/README.md) remains the detailed cross-crate reuse input for domains, modalities, solvers, specialized libraries, WGSL Forge, vision, audio, rendering and semantic instruments. Those entire owners were **not** semantically re-audited for this language review. Use that audit, then trace the selected current public operation before declaring a new gap. Include `qualia-sim` where it owns the operation.

### Current runtime-honesty findings to carry into testing

The inspected `stdlib_v2/topology_and_manifold/effects_and_systems.rs` is reachable from `bind/dispatch.rs` through `call_stdlib_v2`. Its bodies include:

- `code_hash_ast`: FNV-1a 64-bit output labelled `sha256:vibe:...`; `code_resolve`: nonempty-string success. `code_spawn_at` now registers a task through the new scheduler; this supersedes the earlier record-only finding for that function.
- `task.*` now contacts a scheduler registry on selected paths. `spawn_scheduled_task` inserts an entry labelled Running, while `await_task_record` computes an unresolved payload inline. This demonstrates registry wiring, not continuation suspension or verified remote-node execution. `supervisor.start_link` and `effect.*` still transform records in the inspected bodies.
- `value.attach_provenance` / `value.verify_provenance`: FNV-derived checks with fields named `signature_u64` and `authentic`. These bodies do not verify an issuer signature.
- `crypto.prove_zk_predicate` / `crypto.verify_zk_predicate`: FNV-based commitments and a supplied `verified` Boolean. These bodies do not establish a cryptographic zero-knowledge proof or prove the named predicate.
- `runtime.with_budget`, `memory.with_arena`, continuation and channel helpers: descriptive/checking/record operations, with returned claims such as `within_budget`, `zero_heap_escape`, `rolled_back` or `lock_free`. They are not by themselves evidence of evaluator-budget changes, allocator ownership, execution rollback or a concurrent channel.

**Source-change reconciliation:** the initial interpreter read directly evaluated the `await` operand; the later read routes it to `task_scheduler::await_scheduled_value`, and the bytecode path now routes through `task.await`. The initial finding is historical, not a current assertion. Other native job owners can exist without being correctly connected to these particular paths. Likewise, actual native budget enforcement can exist separately from the descriptive helper.

Selected new-scheduler concerns, confirmed by the coordinator's later body reads:

- Task references are ordinary records with numeric IDs. The inspected registry lookup/cancel/await functions do not bind them to a session owner or adopted delegation. Owner-validated opaque references and explicit permitted sharing are required; this is not an executed claim of a public NativeAuthority bypass.
- `execute_scheduled_payload` creates a fresh `LocalHost`, environment and default-budget engine, attempts cell evaluation and then program evaluation, and can return fallback arguments on failure. Preserve the admitted checked host/context, remaining budgets and source revisions; errors must remain errors, and effectful evaluation must not retry under a second mode.
- `await_task_record` can accept a caller-record `result` if the registered result is absent. Caller data must not certify job completion or supply a trusted result.
- Cancellation overwrites terminal state and clears results in the inspected body. Timeout handling trusts supplied elapsed data and can mark a pending/running entry Completed without computing its payload. Use immutable committed outcomes, monotonic runtime deadlines and an explicit cancellation/reconciliation state machine.

These are current capture-time static findings, not executed tests. The author of the in-progress scheduler should reconcile them against subsequent fixes before using this document as a work order.

**Recommendation:** implement the required semantics through the existing owners, or rename/classify descriptor utilities honestly and make unavailable execution fail explicitly. Do not use these returned fields as authority, security evidence or completion claims. Recheck the active testing revision before reporting any finding as still unresolved. This is static evidence, not a fresh exploit test or a claim that all native host security is bypassed.

## 3. Language design and core semantics

### LX-01 — Versioned semantic profiles and a compatibility charter

**Priority/kind:** R0, language/release contract. **Builds on:** existing 0.1 implementation, 0.2 proposal and VIBE-01/11.

Maintain independently versioned language semantics, library catalogue, host ABI, AST encoding, package format and optional VM. Select a dialect/profile in the package/session rather than guessing from syntax. Publish a compatibility matrix for supported, intentionally different, host-dependent and deferred JavaScript tasks. Include behavior, not just API names: scope, equality, truthiness, coercion, errors, text indexing, modules, callbacks, scheduling and collections. A replacement for JavaScript need not reproduce its prototype/ambient-global/coercion behavior, but must explain how ordinary application tasks are expressed.

**Acceptance:** a pinned old program retains its old behavior; a migration reports changed meaning before activation; unsupported profile/dependency combinations refuse clearly; native source execution requires no per-script build. Choose subsequent release numbers only after reconciling the current work, rather than loading every proposal into 0.2.

### LX-02 — One coherent evaluation and mutation model

**Priority/kind:** R0, language. **Depends on:** LX-01.

Specify evaluation order, lexical scope, shadowing, definite assignment, equality, aliasing, mutable local updates, collection ordering and control-flow completion. Keep immutable values/snapshots as the ordinary model; local rebinding and host-resource mutation must remain visibly different. Specify whether field/key absence is an error or an explicit optional lookup, and reject accidental false-success defaults. Callback predicates need checked Boolean results rather than hidden truthiness under the strict profile.

**Acceptance:** nested blocks do not leak declarations, mutations cannot change an unrelated snapshot through an accidental alias, function arguments evaluate once in a defined order, and missing-field/type failures have stable diagnostics. Current 0.1 behavior changes only through the selected migration contract.

### LX-03 — First-class typed functions and lexical closures

**Priority/kind:** R1, language. **Depends on:** LX-02, LX-13, LX-18.

Add complete function-value types covering parameters, results, effects and bounded capture. The inspected implementation deliberately supports noncapturing lambdas; lexical closures would therefore be an explicit expansion. Start with immutable value capture, a defined capture-time revision and bounded environment size. A captured resource reference remains subject to its live lease/authority; capture must not preserve expired permission. Decide how recursion, named/default arguments, partial application and asynchronous callbacks interact with these types. Mutable shared capture can remain later work until ownership and race rules are clear.

**Acceptance:** a returned closure uses the intended captured value after its creating call returns; shadowing does not change that capture; callback effects propagate to the caller; expired handles are refused; closure serialization rejects secrets/leases and unsupported environments. Retaining noncapturing lambdas in the current release remains legitimate if honestly documented.

### LX-04 — Useful generics and constrained polymorphism

**Priority/kind:** R1, language/checker. **Depends on:** LX-02/03.

Move beyond generic spellings for selected built-ins to a defined system for type parameters on reusable functions and data. Specify substitution, inference boundaries, constraint solving and diagnostics. Prefer a small explicit interface/constraint mechanism over unconstrained ad hoc operator overloading. Type-check recursion and bound inference work so an editor cannot hang on an adversarial declaration. Avoid requiring per-script native compilation: the interpreter can use checked generic representations, with optional specialisation as a cache.

**Acceptance:** one typed map/filter/result-composition routine works across multiple admitted types; mismatched callbacks fail before effects; incomplete/ambiguous inference explains the needed annotation. Higher-kinded types and elaborate type-level programming are optional later research, not prerequisites.

### LX-05 — Records, named data types and explicit interfaces

**Priority/kind:** R1, language/schema boundary. **Depends on:** LX-02/04, LX-14.

Define structural records, nominal domain types, optional fields, tuples and interface conformance with clear evolution rules. Use nominal wrappers where confusing two equal-looking values is dangerous: a source revision, resource reference, anatomical part and delegation reference are different roles. Specify extension/unknown-field handling, field visibility, constructor validation and immutable update. Bind native records through typed descriptors; do not let an unrestricted record impersonate a host handle. General classes/inheritance are optional; reusable composition and modules should meet normal application needs first.

**Acceptance:** record update preserves unmodified data; a changed host schema either migrates explicitly or rejects; a user-created lookalike cannot become a resource or grant. Dynamic decoded records are validated before crossing into checked operations.

### LX-06 — Complete algebraic data types, patterns and exhaustiveness

**Priority/kind:** R0/R1, language. **Depends on:** LX-02/05.

Complete the existing enum/pattern foundation across checker, evaluator and encoding. Keep null, absence, failure and disclosure denial distinct. Resolve the current `None`/`Null` representation against the proposed `Option` contract before expanding pattern syntax. Support nested patterns, typed payloads, exhaustive matches and clear unreachable-arm diagnostics. Pattern guards and destructuring conveniences are useful later additions if evaluation and binding rules are explicit. An unknown future enum variant needs an intentional compatibility policy.

**Acceptance:** `Some(value)` differs from `None`; decide explicitly whether `Option<Null>` is admissible, and if so ensure `Some(null)` also remains distinct from absence. Wrong variant arity/types fail; incomplete enum matches are diagnosed; decoding/encoding and optional VM preserve the distinction. Do not silently retrofit these semantics into pinned legacy programs.

### LX-07 — An error model that separates failure, traps and uncertainty

**Priority/kind:** R0, language/runtime. **Depends on:** LX-06, LX-16/20.

Unify typed recoverable errors, source/runtime diagnostics, cancellation, budget traps, denial, conflicts and indeterminate external outcomes without collapsing their meaning. Make propagation and any future catch/finally semantics explicit. Cleanup can run on failure but cannot grant fresh authority or turn an unbounded loop into repeated retries. Preserve causal chains and safe source locations. A partial external action needs a reconciliation result, not a fabricated local rollback.

**Acceptance:** expected domain errors can be matched/propagated; budget exhaustion cannot be swallowed to reset the budget; denied information is not returned as an empty collection; failure after an external write retains the uncertain/committed outcome. Error presentation redacts secrets without destroying useful diagnosis.

### LX-08 — Numeric precision, rounding and storage semantics

**Priority/kind:** R0/R1, core plus library. **Depends on:** LX-01/02.

Qualify checked fixed integers/floats first, including signed minima, overflow, non-finites, conversion loss and equality. Existing `i32/u32/f32` aliases must not imply narrow storage/range. Introduce real narrow types only with a migration and ABI design. Reuse decimal helpers, but define overflow, scale, rounding modes and bounds; a saturating result must be explicit, not advertised as exact arithmetic. Add bounded big integers/rationals only for identified symbolic, scientific or identifier-handling cases. Money remains an amount-plus-unit/domain concept, not an unqualified decimal.

**Acceptance:** edge-case vectors distinguish checked/saturating/wrapping behavior; exact operations either return exact results or an explicit overflow; serialisation preserves large integer values; CPU/GPU precision conversions are declared and measured against case-specific tolerances.

### LX-09 — Unicode text, bytes, localisation and source positions

**Priority/kind:** R0/R1, core/library/tooling. **Depends on:** LX-01.

Settle scalar-versus-grapheme indexing explicitly. Recommendation: retain the proposed scalar contract for the selected 0.2 core unless ratification changes it, while exposing named byte/scalar/grapheme operations for intent. Editor cursor/display behavior can use graphemes without changing source byte spans. Pin the Unicode data version, use conformance data for segmentation, and cover combining marks, Indic text, emoji sequences and bidirectional presentation. Source normalization must not rewrite arbitrary strings, IRIs or evidence bytes. Localised keyword presentation must round-trip through the same AST and authority semantics.

**Acceptance:** length/slice/index agree within each named unit; UTF-8/UTF-16/editor-coordinate conversion is tested; formatter preserves literals and comments; mixed-script warnings are actionable. Grapheme segmentation should be checked against the chosen version of [Unicode UAX #29](https://www.unicode.org/reports/tr29/), rather than inferred from a few emoji examples.

### LX-10 — Collections, iterators and predictable algorithm costs

**Priority/kind:** R1, library/language integration. **Depends on:** LX-03/04/08/09.

Reuse existing list/set/deque/queue helpers. Add coherent maps, ordered maps/sets, ranges, iterators, lazy transformations, stable sorting, folds and comprehensions where they simplify real work. Define equality/hashing, iteration order, empty-container inference, comparator consistency and callback effects. Lazy composition must remain bounded; materialisation, infinite sources and streams cannot masquerade as finite lists. Provide efficient bulk operations over typed native buffers rather than forcing list-per-element processing.

**Acceptance:** eager/lazy forms agree on finite admitted inputs, sorting is reproducible under its policy, predicate type errors are diagnosed, and pathological comparisons/allocations respect budgets. Large data crosses a host boundary as bounded chunks or handles.

### LX-11 — Time, calendars and clocks as distinct concepts

**Priority/kind:** R1, library/domain types. **Depends on:** LX-08, LX-22.

Build on `Instant`, `Duration` and replay clocks. Add civil dates/times, intervals, timezone/calendar conversion, parsing/formatting and recurrence as libraries with versioned data. Keep simulation time, monotonic deadlines, sample/frame clocks and observed wall time separate. Define leap/discontinuity/ambiguous-local-time handling and arithmetic overflow. Calendar durations need not equal fixed elapsed seconds. Timezone data should be pinned where it affects reproducibility.

**Acceptance:** a replay never secretly reads the current clock; deadline checks use a monotonic owner; ambiguous civil times require an explicit resolution; animation and audio/video timelines retain their declared scale and sampling policy.

### LX-12 — Ergonomic syntax with a complete semantics cost

**Priority/kind:** R1/R2, language/tooling. **Depends on:** LX-02–10.

Evaluate destructuring, pattern guards, string interpolation, optional chaining, null/option coalescing, ranges, spread/update and pipeline conveniences. Add them selectively with one documented evaluation model. Interpolation must be ordinary data formatting, not automatic HTML/SQL/query/code construction. Optional chaining should apply to a defined optional type and must not hide denial, stale resources or failed effects. Decide parser ambiguity and formatting/source-map consequences before adoption. Existing experimental syntax must be inventoried, not discarded because the earlier candidate grammar omitted it.

**Acceptance:** each construct desugars to a documented checked operation, evaluates operands once, preserves error/effect behavior and round-trips through tools. Avoid adding dozens of synonyms for the same operation.

## 4. Runtime, authority, resources and distribution

### LX-13 — Orthogonal effects, execution constraints and actual grants

**Priority/kind:** R0/R1, checker/runtime. **Depends on:** LX-01/03/14.

The current ordered effect classes and `max` join are useful inspected foundations. For broader expansion, model separately what a function can do (read/write/network/device/etc.), whether it can suspend, and its execution constraints (frame-safe, bounded cold work, allocation/blocking rules). A total order alone cannot express every useful combination. Define sound transitive inference through callbacks, dynamic dispatch, modules and handlers. Map existing annotations explicitly; do not silently replace their semantics. Effect declarations describe behavior, while the native authority owner grants actual permission.

**Acceptance:** a pure higher-order function cannot conceal an effectful callback; a frame-safe hook rejects blocking/heavy work; an unknown operation descriptor is not trusted as pure merely because a lookup falls through. Audit the current `binding_effect` fallback and catalogue classification without assuming an exploit from classification alone.

### LX-14 — A single trustworthy typed capability catalogue

**Priority/kind:** R0, host/library/tooling. **Depends on:** LX-01/05/13.

Make operation metadata authoritative for checking, dispatch, agent discovery and Poet controls. Specify types, effects, provider/version, native/WASM availability, units/frames, limits, lifetime, concurrency, revisions, failure modes, authority and receipt semantics. Generate views or enforce bidirectional conformance so manual tables cannot drift. Distinguish descriptor-building operations from execution. Dynamic discovery can narrow the admitted set but cannot grant rights or redefine a pinned operator silently.

**Acceptance:** every advertised production operation either executes the declared semantics or refuses precisely; schema/dispatch/effect parity is checked; a provider upgrade invalidates incompatible cached checks. Discovery works offline for packaged contracts without initiating network access from an IRI.

### LX-15 — Human principals, agents and delegated authority

**Priority/kind:** R0, native authority integration. **Depends on:** LX-14/24; reuse AUTH-H1–H6.

Enforce the [existing hierarchy contract](human-principal-agent-authority.md): admitted human principal, adopted scoped delegation, authenticated acting agent, constrained optional subdelegation and operation-time verification. Keep several qualified identifiers per subject/actor; matching names/digests do not establish authority. Review independence must consider common human control. Script values are requests/references, not grant constructors. Support inspection of why an action is allowed or denied without exposing secrets.

**Acceptance:** UI/MCP/Vibe use the same owner and policy; expiry/revocation/source changes invalidate pending publication; resuming work rechecks authority; agents cannot widen scope/budgets or appoint themselves reviewers. Reuse native agency/task delegation, QDNF and project rights rather than adding a private Vibe permission database.

### LX-16 — Structured concurrency and meaningful algebraic effects

**Priority/kind:** R0/R1, runtime/host. **Depends on:** LX-07/13–15/18.

Implement typed futures, task groups, suspension, parent-child lifetimes, cancellation propagation and reconciliation. Define when sibling failures cancel work, what joining waits for, and which effects may already have committed. Supervisors need real worker ownership and bounded restart policies, not lists labelled restarted. If resumable algebraic effects are retained as a language goal, specify handler scope, continuation lifetime, one-shot versus multi-shot resumption, cleanup and budget accounting. Immutable descriptors are useful but must be described as descriptors.

Preserve the checked parent host, source/module pin, grants and remaining budgets in child execution. Select parsing mode before executing; failure cannot trigger a second interpretation or return input arguments as a successful result. Registering a target-node string is not evidence of remote execution.

**Acceptance:** a delayed native job suspends while the editor stays responsive; cancellation is requested then acknowledged; leaked/orphan jobs are accounted for; another session cannot control a guessed task ID or inject a result; a handler actually resumes the checked continuation; duplicate resumption is rejected unless its profile explicitly supports it. An error after an admitted effect does not execute it a second time. [ECMAScript control-abstraction semantics](https://tc39.es/ecma262/multipage/control-abstraction-objects.html) provide a comparison for async/generator behavior, not a requirement to reproduce JavaScript's full runtime model.

### LX-17 — Streams, subscriptions and backpressure

**Priority/kind:** R0/R1, runtime/library. **Depends on:** LX-16/18/19.

Define bounded asynchronous streams, event subscriptions, generators and channel behavior. Specify queue capacity, ordering, dropped/coalesced events, flow control, cancellation, checkpoint/reconnect and subscription ownership. Reliable command streams and lossy visual/audio preview streams require different policies. An in-memory list capped to a length is not evidence of a lock-free concurrent queue. Keep secrets and authority out of serialisable subscription state.

**Acceptance:** a slow consumer cannot grow memory without bound; loss is reported according to policy; removing/reloading a view disposes subscriptions once; network/provider reconnection does not replay unacknowledged commands blindly; no “lock-free” claim is made without the actual concurrent implementation and applicable verification.

### LX-18 — Resource lifetimes and deterministic disposal

**Priority/kind:** R0/R1, runtime/host/type boundary. **Depends on:** LX-05/14.

Provide opaque typed handles for buffers, assets, jobs, files, devices and decoders, with owner/generation, revision, permissions, lease, disposal and borrowing rules. A scoped cleanup construct is useful, including asynchronous disposal where required. Start with a manageable scoped/affine resource model rather than imposing Rust's entire borrow-checking language on script authors. Device loss and use-after-disposal need typed recoverable results. Capturing a handle must not extend its underlying authority.

**Acceptance:** forged/stale/wrong-owner handles fail, normal/error/cancel paths release resources, duplicate close is defined, and saved projects retain reconstructible references rather than live pointers or secrets. Ordinary garbage collection cannot be the only contract for timely GPU/file/job cleanup.

### LX-19 — Real budgets and truthful performance contracts

**Priority/kind:** R0, runtime/native host. **Depends on:** LX-13/14/18.

Charge parsing, inference, evaluation, callbacks, allocations, graph scans, decoding, native compute and output against explicit limits with units. Parent/child budgets must not multiply backing resources accidentally. Provide deadline, cancellation and quota semantics for native jobs and GPU allocations. Distinguish script working memory from large native resources; an interpreter sentinel is not a universal master-asset ceiling. Remove claims such as enforced arena/zero-heap escape unless the allocator/runtime actually enforces them.

**Acceptance:** adversarial source and expanding native results terminate safely; supplied caller records cannot reset counters; outstanding allocations remain accounted for; quality is never silently downgraded to fit a budget. Profile representative scenes/scripts and report measurements instead of assuming the new language is automatically faster.

### LX-20 — Transactions, commands, receipts and reconciliation

**Priority/kind:** R0, native state/host. **Depends on:** LX-07/14/15/24.

Define snapshot reads, expected-revision writes, validation, commit/conflict and durable native receipts using existing graph/project owners. Keep pure planning, preview, staging and commit separate. External multi-service work needs operation identifiers, deduplication and a reconciliation state machine; local transactions do not imply distributed rollback or exactly-once execution. Undo is a checked revision/compensating operation, not erasure of history. A script-created `Receipt` marker is not proof of a commit.

**Acceptance:** concurrent changes produce a visible conflict or valid merge; repeated delivery cannot duplicate a supported deduplicated action; crash after uncertain transport queries native state before retry; persistence errors are surfaced. Reject transaction support claims when the selected provider only implements no-op defaults.

### LX-21 — Durable source sessions, hot replacement and state migration

**Priority/kind:** R0/R1, runtime/Poet integration. **Depends on:** LX-16/20/23/34/35.

Extend the existing in-memory SourceSession into named durable workspaces through the canonical project owner. Specify candidate preparation, activation boundaries, function/module replacement, compatible value migration and dependency invalidation. Pin suspended jobs to the source that started them or cancel/migrate explicitly. Separate session-local exploration/private drafts from adopted project source. Reopen must not restore expired grants, run old hooks automatically or replay external effects.

**Acceptance:** successive REPL submissions share admitted definitions; invalid replacement leaves the last accepted scene usable; save/reopen in a fresh process recovers authored source/history and necessary references; repeated live edits do not duplicate hooks or reset selection/camera/review state. Editing a script using an installed capability never requires a Rust build.

### LX-22 — Deterministic replay with honest boundaries

**Priority/kind:** R1, runtime/tooling. **Depends on:** LX-11/16/20/23.

Reuse replay clocks and seeded algorithms, recording source/dependency versions, snapshots, seeds, event ordering and admitted external observations. Define distinct guarantees for exact pure replay, numerical tolerance replay and evidence-only reconstruction of external actions. Parallel scheduling and floating-point/device differences require a documented policy. Private data in traces follows disclosure/retention rules. Checkpoints are owned state snapshots, not arbitrary records labelled continuations.

**Acceptance:** replaying a pure authoring case reproduces its admitted result; recorded external effects are shown without being reissued; cross-device comparisons use stated tolerances; missing dependencies/observations make the replay incomplete explicitly. Do not promise universal bitwise reproducibility for every GPU/solver operation.

### LX-23 — Modules, packages and dependency evolution

**Priority/kind:** R0/R1, language/package host. **Depends on:** LX-01/04/14/24.

Extend module metadata into fully checked offline-capable loading with pinned content, export signatures, explicit dependency closure and load/run/activate separation. Define cycles, lazy initialisation if any, incompatible upgrades, source availability and namespace conflicts. Package capabilities are requests; dependencies do not inherit ambient caller authority. Use signed/admitted manifests and existing HMC package ownership; source caches and optional bytecode are disposable derivatives.

**Acceptance:** changing dependency bytes invalidates the pin/cache; import alone does not install hooks or execute external effects; a packaged program runs offline with its declared closure; dependency migration can be reviewed and rolled back without discarding attribution. A package registry service is optional, not a requirement for local development.

### LX-24 — Cryptography, content addressing and provenance that mean what they say

**Priority/kind:** R0, library/native security owner. **Depends on:** LX-01/14/20/23.

Correct the inspected hash/signature/proof claims. Reuse actual cryptographic providers and adopted key/credential mechanisms. Separate fast indexes/checksums from cryptographic content digests, issuer signatures, proof verification and semantic truth. Canonicalisation must identify exactly which bytes/schema/algorithm were hashed. Zero-knowledge APIs need a defined proof system, statement, public inputs, verification parameters and genuine verifier; unsupported proof systems refuse. Attaching an attributed provenance statement is distinct from verifying its issuer and independently accepting the statement.

**Acceptance:** known vectors and malformed/forged cases behave correctly; changed bytes/public inputs fail verification; script-controlled Boolean fields cannot establish authenticity; digests do not become permissions. Provenance exchange can map entities, activities, agents and derivation to [W3C PROV-DM](https://www.w3.org/TR/prov-dm/) while retaining the ecosystem's richer qualified records.

### LX-25 — Safe native, file, network and process interoperability

**Priority/kind:** R1, host/library. **Depends on:** LX-14/15/18–20.

Expose typed admitted native functions and streams without raw pointer access, unsafe binary overlays or unrestricted dynamic library loading. Filesystem handles need scoped roots, atomic-save semantics, explicit encodings and symlink/path policy; network operations need bounded responses, cancellation, declared endpoints and transport/security owners. Process execution, if supported, is an explicitly privileged capability with structured arguments, not shell-string interpolation. Foreign data is validated and copied/borrowed under a stated lifetime contract.

**Acceptance:** path escapes, oversized results, malformed foreign values and absent services fail safely; cancellation reaches the provider; APIs do not smuggle authority through arbitrary records. Ordinary scripts using installed adapters need no compile step. Broader Rust/C/Python interoperability can be optional adapters with distinct trust profiles, not mandatory runtime dependencies.

### LX-26 — Native, WASM-local and native-assisted parity

**Priority/kind:** R0/R1, framework/host. **Depends on:** LX-14–25.

Use one checked language semantics and authoritative domain command model across supported targets. Advertise locally executable versus native-assisted operations explicitly, with pairing, negotiation, authority, disconnect, resumability and privacy rules. Keep Aether, Etalon and native service ownership intact; Chora remains a separate engine/source boundary. WASM is a supported target, while browser binding dependencies must be disclosed separately from native no-JS/V8 qualification.

**Acceptance:** shared local cases agree in results/effects/diagnostic categories; absent providers cannot silently change algorithms; reconnect reconciles outstanding operations; no JS state/policy mirror becomes authoritative. [WIT](https://component-model.bytecodealliance.org/design/wit.html) is a useful reference for typed interface/resource contracts where a component boundary is adopted; it specifies interfaces, not the behavior of a complete runtime.

## 5. Scientific, creative and cooperative domain expansion

### LX-27 — Graph queries, identifiers and knowledge selection

**Priority/kind:** R0/R2, native graph adapters. **Depends on:** LX-05/14/20/23.

Expose typed snapshot queries, pagination/streams, selected inference profiles and provenance-aware results through QualiaDB/Q42 owners. Preserve multiple identifiers, qualified mappings, observation/valid times, taxon-versus-individual distinctions and restricted disclosure. Query plans and result coverage should explain what was actually searched. Expand graph syntax only with a versioned query contract; no IRI should trigger an ambient fetch. An empty visible answer is not proof that the fact is absent.

**Acceptance:** the same selected species/part is navigable through graph, authoring and representations; contradictory sources remain attributable; pagination has stable semantics under its snapshot; protected partitions do not leak through counts/errors/caches. Reuse the full HMC embodiment rather than building a renderer-specific species database.

### LX-28 — Units, frames, uncertainty and measurement types

**Priority/kind:** R0/R2, domain libraries/checking. **Depends on:** LX-05/08/11/14.

Extend existing quantity/frame families with documented dimensional algebra, reference-frame conversions, affine units, coordinate conventions, tolerances, intervals/distributions and measurement uncertainty. Distinguish measured values, estimates, model parameters and assumptions. Support typed spectral axes, wavelengths/frequencies, pressure, humidity, temperature and sample rates. Electromagnetic radiation and acoustic/mechanical waves belong to a broader sensory model but are physically distinct; sharing a frequency unit does not make sound electromagnetic.

**Acceptance:** adding a duration to a length fails; temperature offsets are handled explicitly; transforms retain handedness/units/frame/time; uncertainty propagation declares its assumptions. Unit/frame mismatches are caught before native kernels consume misleading numbers.

### LX-29 — Geometry, CAD, fields and scientific compute composition

**Priority/kind:** R2, domain/native integration. **Depends on:** LX-14/16/18/19/27/28.

Provide ergonomic typed selections and composition for existing geometry, topology, constraints, meshing, fields, tensors, solvers and WGSL Forge owners. Keep robust predicates, tolerances, topology validation, convergence and performance in their native libraries. Expose diagnostics and residuals rather than “solved=true” from a descriptor. Preserve editable parametric/source history and stable part references. Large work remains bounded native/GPU jobs; a Vibe loop is not the default per-vertex engine.

**Acceptance:** a profile/constraint edit produces a versioned candidate with validity/convergence evidence; invalid topology remains rejected/inspectable; stale solves cannot overwrite new source; CPU/GPU fallback is explicit and numerically qualified. The nine-area audit and actual public owner trace precede a new algorithm assignment.

### LX-30 — Materials, textures, spectral appearance and rendering plans

**Priority/kind:** R2, domain/native integration. **Depends on:** LX-28/29/33/34.

Expose layered materials, UV/atlas/detail workflows, procedural fields, texture graphs, colour management and spectral/visible projections through Poet and the actual renderer/Etalon path. Separate physical material parameters, artistic overrides, illumination, sensor response and display transforms. High-grade master values should survive derivatives; rendering style and LOD are independent selections. Shader/node composition needs typed inputs, bounded resource use and readable errors mapped to source.

**Acceptance:** editing a material/texture parameter updates only affected work; native GPU execution matches its declared operator; spectral-to-colour conversion retains its illuminant/observer policy; texture resolution/LOD changes preserve lineage. A language feature cannot by itself certify film-grade appearance—actual outputs require visual and domain review.

### LX-31 — Animation, rigging, behavior and simulation composition

**Priority/kind:** R2, domain/native integration. **Depends on:** LX-11/16/22/28/29.

Reuse animation, clocks, rig/player and simulation owners for timelines, clips, state machines, transitions, constraints, contacts and event-driven behaviors. Define interpolation versus simulation, update order, seek/restart and deterministic sampling. Keep biological state, style, LOD and simulation fidelity separate. Editable curves, source motions and their derivations remain in the package; do not bake away all semantics into a clip.

**Acceptance:** a transition/seek is recoverable after save/reopen under the declared profile; contacts/events have correct timing; stale rig/topology references fail; changing cartoon style does not alter a wallaby's biological stage. Behavior claims need actual scenario execution and domain review.

### LX-32 — Biology, anatomy, ecology and multiscale embodiment

**Priority/kind:** R2, domain modelling/bindings. **Depends on:** LX-27–31/33.

Expose developmental stages, organs/tissues, morphology, physiological state, environmental cues, interactions and observation-linked annotations through existing QualiaDB domains and simulation. Support whole-organism → organ → tissue/detail inspection by selecting available resources/knowledge with explicit scale and resolution. Keep taxon templates, observed specimens, simulated individuals and manufactured assemblies distinct. Geometry must not invent unsupported anatomy or infer biomedical truth from attractive pixels.

**Acceptance:** a tree stage and a wallaby part retain supporting source/model assumptions; zooming requests the right admitted resources rather than fabricating finer facts; absent evidence is visibly unknown. Biomedical-education suitability needs anatomical/scientific review and task-specific validation; “biomedical grade” is not a language/runtime certification.

### LX-33 — Audio, video, images and sensor/signal pipelines

**Priority/kind:** R2, media/domain libraries. **Depends on:** LX-11/17–19/28.

Compose the existing vision/audio/media owners with typed sample/frame clocks, channel layouts, colour spaces, buffers, calibration and transforms. Include streaming decode, analysis, filtering, spatial acoustics and synchronised annotations where actually needed. Keep feature extraction distinct from accepted interpretation. Native operators must declare latency, approximation, supported formats and loss/error behavior; a listed filter must either execute or remain explicitly deferred/unsupported.

**Acceptance:** cancellation stops expensive processing, A/V clocks stay synchronised under their policy, edge cases have reference results, and operation provenance survives packaging. Reuse existing completed image cases; do not reopen them without changed source or evidence of a problem.

### LX-34 — Declarative native UI, reactivity and accessible input

**Priority/kind:** R0/R2, presentation/Poet/Aether. **Depends on:** LX-03/13/17/21.

Develop shared typed presentation/state/command semantics using existing reactive cells and presentation modules. Support forms, selection, inspectors, timelines, spatial manipulation, layout/virtualisation, validation, event routing and undo. View calculation remains pure; effectful commands activate explicitly. Focus, keyboard navigation, IME, pointer capture, gestures, screen-reader semantics, scaling, contrast and reduced motion need real native behavior, with supported WASM projections. A text `aria_label` field alone does not prove accessibility.

**Acceptance:** keyboard/IME and agent commands can complete the same scoped material/source task; repeated render/reload causes no hidden write or duplicate subscription; invalid edits remain visible and recoverable. Poet remains the unified interface, including useful non-CAD authoring functionality from MIT-licensed source references.

### LX-35 — Cooperative source, semantic merging and review

**Priority/kind:** R0/R2, native project/Poet. **Depends on:** LX-15/20/21/23/34.

Build on AST merge and native source/project owners for private drafts, proposals, semantic diffs, conflict resolution, review and adoption. Merge structure where semantics permit it; preserve unresolved incompatible constraints, concurrent material edits and changed meaning. CRDT convergence is not proof of a semantically valid or authorised merge. Source/result/package versions and contribution records remain linked across offline edits and reconnect. Selection/editor state is local unless intentionally shared.

**Acceptance:** two contributors can propose independently; a reviewer sees exact source/output differences and permissions; conflicting edits cannot erase attribution; acceptance binds a specific version and qualified reviewer. No agent can self-award human acceptance or use a second agent under common control to fake independence.

### LX-36 — Logic, rules, disclosure and semantic instruments

**Priority/kind:** R2, domain/native integration. **Depends on:** LX-15/24/27/28/35.

Expose existing logic/solver/semantic-instrument families through declared evaluation profiles. Distinguish asserted, observed, inferred, believed, permitted and adopted facts; preserve open-world/closed-world assumptions, temporal horizons, uncertainty and evidence. Rules proposing actions are separately authorised commands, not authority created by a proposition. Keep source rights, contribution records, commons obligations and no-charge uses distinct from financial settlement. Reconcile the existing license/humanitarian-equity/semantic-token work before extending financial instruments.

**Acceptance:** inference reports rules/inputs/profile, contradictory or unknown facts do not silently become false, restricted knowledge respects disclosure, and a drafted agreement is never labelled adopted by evaluating a script. Payment rails, prepaid allocations and periodic reconciliation remain later support work under the existing contracts, not prerequisites for the first asset.

## 6. Tooling, qualification and controlled extensibility

### LX-37 — One language service for Poet, editors and agents

**Priority/kind:** R0/R1, tooling. **Depends on:** LX-01–14/23.

Expose parsing with Complete/Incomplete/Invalid states, incremental checking, completion, hover, signature/effect/authority information, rename, references, formatting and safe code actions from the same language service. Preserve comments/trivia and source maps for structural edits, generated fragments and optional VM instructions. A formatter is not permission to rewrite meaning or drop unknown source. Keep internal byte spans explicit while adapting external editor encodings. LSP can be an adapter rather than a second checker.

**Acceptance:** an incomplete multiline submission never executes, rename distinguishes a lexical symbol from graph identifiers/labels, structural/text editing round-trips, and tools show actual profile availability. Use the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/) for interoperable editor transport where useful; its implementation need not require a JavaScript server. [ECMA-426](https://tc39.es/ecma426/) is a reference for source-map interchange where applicable, not a reason to translate Vibe to JavaScript.

### LX-38 — Debugging, profiling and explainable execution

**Priority/kind:** R1/R2, runtime/tooling. **Depends on:** LX-13/16/20–22/37.

Provide source breakpoints/stepping for the supported interpreter subset, stack/value views, bounded watch expressions, job/effect traces, dependency invalidation inspection and cost/resource profiling. Watches must not mutate state, disclose hidden facts or allocate unlimited work. Explain why an operation is allowed/denied and which source/dependency revision produced an output. Time-travel applies only within a qualified replay/owned-state model; external history is evidence to inspect, not something to undo by pretending.

**Acceptance:** trace a mesh/material result to its source call, native job, source snapshot, parameters and rights; a watch cannot publish; recorded external effects are not rerun during stepping/recovery. Exportable diagnostics omit private payloads under the applicable disclosure policy.

### LX-39 — Agent-facing authoring without a second privileged language

**Priority/kind:** R1, tooling/MCP/native host. **Depends on:** LX-14/15/20/35/37.

Generate machine-readable schemas, examples, diagnostics and structured edit proposals from the canonical language/catalogue. Agents should request typed operations, preview semantic diffs, inspect limits/effects and submit revision-checked changes through the same owners as people. Support dry-run plans and bounded batching where the owner can actually guarantee them. Treat imported prose, comments and reference documents as data rather than authority to widen permissions. Keep instruction provenance outside ordinary data strings.

**Acceptance:** UI and MCP/Vibe equivalent commands produce equivalent admitted operations; the agent can report each deliverable against source/evidence; malformed/stale proposals fail without losing work. No helper success Boolean becomes a review, signature or completed model claim.

### LX-40 — Conformance, property tests and adversarial qualification

**Priority/kind:** R0/R1, independent qualification. **Depends on:** all features actually included in the selected profile.

Maintain a feature matrix from grammar/AST through resolution/checking/evaluation, encoding, optional VM, REPL, host and UI. Add positive/negative examples, property/metamorphic tests, parser/checker fuzzing, resource-exhaustion cases and relevant differential tests. Differential testing is against the adopted Vibe semantics/reference implementation, not automatic imitation of JavaScript. Test evidence includes pinned source/dependencies, target/provider, inputs, actual results and remaining coverage. Do not count names, fixtures or ignored tests as passes.

**Acceptance:** every release claim links to executed evidence or remains explicit open scope; native/WASM shared cases agree; incompatible migrations are demonstrated; human/domain acceptance stays separately recorded. Reuse existing REQ-VIBE/VBC and C1–C8 ledgers rather than creating a parallel definition of completion.

### LX-41 — Documentation, packages and a sustainable library ecosystem

**Priority/kind:** R1/R2, documentation/package tooling. **Depends on:** LX-01/14/23/37/40.

Provide a language reference, small application cookbook, API reference generated from true descriptors, runnable offline examples and migration guides. Include everyday forms/network/files/data-processing examples alongside geometry/knowledge examples so the language is usable beyond one demo. Packages retain source, notices, contribution/provenance and pinned interfaces. Registry search, vulnerability/revocation metadata and dependency analysis can expand later without requiring a central commercial service.

**Acceptance:** a new contributor can edit, run, debug, review and reopen a native Poet example from documentation; examples are executed against their named profile; deprecated/unavailable functions are visibly labelled. Documentation must distinguish proposal, implemented source, verified target and accepted product use.

### LX-42 — Optional bytecode, specialisation and parallel performance

**Priority/kind:** R3, runtime optimisation. **Depends on:** LX-13/18/19/22/40.

Optimise measured bottlenecks with checked-AST caching, optional bytecode, generic specialisation, efficient immutable structures and native bulk kernels. Parallel pure work requires declared ordering/resource/reproducibility policies; SIMD/GPU facilities need actual implementations behind their claims. An optional shader/compute subset can target qualified native compilation through WGSL Forge/Etalon while preserving source diagnostics and an admitted execution path. General script source evaluation remains mandatory.

**Acceptance:** interpreter/VM supported-subset results, failures, effects and budgets agree; caches invalidate on all semantic dependencies; cold starts and representative editor/frame workloads show measured benefits. No partial effectful VM run may fall back and execute the program a second time. A JIT or ahead-of-time script build is not required for ordinary authoring.

### LX-43 — Controlled metaprogramming and domain extension

**Priority/kind:** R3, language/tooling. **Depends on:** LX-01/13/14/24/37/40.

If repeated domain composition justifies it, design typed quotations or hygienic structural macros with explicit expansion phases, bounded work, source mapping and no ambient I/O. Generated ASTs go through the same checker and authority boundary. Prefer registered schema/operation/type extensions first; unrestricted parser plugins or string evaluation can fragment the language and make agent/tool behavior unpredictable. Reflection should expose admitted structure, not private handles/grants.

**Acceptance:** expansion is reproducible from pinned inputs, cannot capture unintended lexical names or smuggle effects, and points diagnostics to both generated and authored source. Macro execution cannot install grants or bypass module admission. Do not require this feature to author the reference tree/wallaby.

### LX-44 — Query-aware progressive resources and performance planning

**Priority/kind:** R2/R3, graph/resource host. **Depends on:** LX-17–19/23/27–33.

Allow scripts to express inspection intent, selected subject/part, spatial/temporal bounds, desired fidelity and tolerances, then let existing native resource planners select a bounded dependency closure. Preserve semantic resource contracts rather than turning every request into a renderer-only LOD decision. Include cache/residency/eviction and access-right revalidation. A magnifying inspection may request finer scales, anatomy, lichen or insects only where the embodiment actually contains admitted evidence/resources; it does not manufacture those facts.

**Acceptance:** loading a large synthetic HMC keeps a bounded resident set, cancellation stops obsolete detail work, missing chunks are visible, and source/knowledge remain resolvable independently of the current visual derivative. Measure latency, transfer and memory on declared fixtures before claiming superiority to other formats or engines.

## 7. Extensible domains and language specificities

The core should define how extensions are described, composed, checked, admitted, executed and evolved. It should not contain a closed enumeration of all future sciences, professions, human languages or institutional vocabularies. New terminology is not necessarily new semantics; new semantics is not necessarily new grammar; a new operator is not automatically a new native kernel.

The existing seams include `vocab::{parse_chunk, tree_shake, project_referenced_iris}`, `lexicon_pin::{LexiconPin, PackManifest, LexiconFraming}`, `LocaleRegistry`, module exports, capability schemas and semantic-instrument packages. The inspected vocabulary module preserves canonical IRIs and bounded referenced chunks; lexicon metadata already distinguishes living/artifact framing. Reuse these seams. Their presence is not qualification of arbitrary domain loading, complete multilingual semantics or safe executable extensions. Current keyword tables also use static string representations; open runtime-defined lexicons need an explicit loading/admission contract.

### LX-45 — Independently versioned extension packages

**Priority/kind:** R1 foundation; required subset R0 where current authoring uses extensions. **Depends on:** LX-01/14/23/24.

Define a composable extension manifest carrying qualified package identifiers, namespace, version/content digest, dialect/schema compatibility, dependencies, authorship/notices, review/adoption status and exported types/operators/vocabularies/tools. Separate vocabulary-only, schema/constraint, checked Vibe-module, native-provider and optional syntax-extension roles. A legal profile, a manufacturing profile and a language-specific lexicon can depend on shared packages without becoming core built-ins. Local/offline authorship must work without a mandatory central registry, payment or company-issued permission. Executing privileged operations still requires the appropriate native authority.

**Acceptance:** an independently authored, previously unknown domain pack loads on an installed compatible interpreter, adds useful checked data/operations/tooling and packages its dependency closure without modifying the core binary. Missing required dependencies produce a precise held/unsupported outcome. Pure vocabulary/type aliases must not be advertised as new executable capabilities.

### LX-46 — Extensible types, operators, constraints and semantic contracts

**Priority/kind:** R1, type/library/host extension boundary. **Depends on:** LX-04–06/13/14/28/45.

Specify how a pack declares nominal/structural types, units/frames, validated constructors, relations, shape constraints, pure functions and effectful operator bindings. Every executable operator needs an actual checked implementation or admitted native provider, with result/error/effect/resource contracts; an ontology label does not implement it. Define equality, comparison, hashing, encoding and migration per exported type where applicable. Unknown operators fail checking/admission rather than receiving generic purity or success. Keep static type checking, graph validation, inference, application policy and external-world truth distinct.

**Acceptance:** a new material batch or legal instrument type cannot impersonate money, authority or a live handle; generic containers/functions can use it through declared constraints; its codecs round-trip; invalid values fail at the boundary. [SHACL](https://www.w3.org/TR/shacl/) is a useful existing graph-validation contract where adopted, not proof of legal validity, physical realizability or operational implementation.

### LX-47 — Human-language, dialect and specialist terminology packs

**Priority/kind:** R1/R2, language service/lexicon semantics. **Depends on:** LX-09/12/37/45/46.

Support separately versioned display labels, keyword aliases, terminology/senses, domain phrases, grammatical/formatting conventions and interpretation notes. Distinguish programming syntax locale, natural-language content, jurisdictional usage and community/domain vocabulary; the same label can denote different concepts in different contexts. Preserve original wording, language/script/dialect tags and attributed translations. Translation/alignment is a qualified mapping, not automatic semantic equivalence. Controlled phrases can resolve to checked AST/operations; unrestricted natural language remains interpreted source with ambiguity, provenance and review status.

Locale-specific decimal/date input is explicitly parsed into canonical typed values; it cannot silently change code-token meanings or a signed agreement. RTL text, IME, inflection, plural forms and terminology need tools beyond a keyword replacement table. Use [BCP 47 language tags](https://www.rfc-editor.org/rfc/rfc5646.html) where suitable and retain richer community/lexicon identifiers rather than forcing every distinction into a language tag.

**Acceptance:** two lexicons can use the same word differently through explicit imports/qualification; opening one does not rewrite another package's meaning. A translation round-trips to the original clause/source reference; unresolved ambiguity is surfaced. Living subjects cannot be silently reclassified as manufactured things by an alias upgrade.

### LX-48 — Composition, conflicts, migration and open-ended data

**Priority/kind:** R1, extension/module contract. **Depends on:** LX-01/05/23/45–47.

Compose packages by explicit imports and qualified exports, with declared mappings for overlapping concepts, units, laws, type interfaces and local terminology. No load-order winner, matching string or “latest” version should silently determine meaning. Separate asserted equivalence from directional/lossy/contextual mapping. Retain unknown noncritical annotations/extension payloads for lossless round-trip under declared size/privacy limits; unknown critical executable/schema requirements hold the affected operation instead of being ignored. Record dependency versions, semantic migrations, deprecation and revocation separately.

**Acceptance:** conflicting meanings require qualification or an adopted mapping; changing one pack invalidates affected checks/caches but does not rewrite signed history; old packages retain their pinned interpretation; unsupported critical extensions prevent a false-success run while permitted inspection remains available. A data migration reports information loss and preserves the original where policy requires it.

### LX-49 — Bounded extension admission, execution and lifecycle

**Priority/kind:** R0/R1, native host/runtime. **Depends on:** LX-13–19/24–26/45/48.

Define prepare → validate → admit → explicitly activate → retire/unload states. Loading a vocabulary or inspecting a manifest cannot run hooks, start payments, fetch arbitrary URLs or install grants. Checked Vibe implementations use the ordinary evaluator; native-provider extensions use qualified interfaces and installation policies. Any future parser/grammar extension is opt-in, namespaced, pinned, bounded and tool-aware; preserve unambiguous parsing and diagnostics, with an admitted lowering into checked semantics. This is compatible with later grammar extensibility without making unrestricted parser injection the everyday domain mechanism.

**Acceptance:** a malicious/oversized/cyclic pack is rejected or bounded, not a host compromise; removing/upgrading a pack reconciles dependents, jobs and subscriptions; a signature verifies declared bytes/issuer only and does not self-authorise operation. Native, WASM-local and assisted availability remain explicit.

### LX-50 — Extension authoring, discovery and conformance

**Priority/kind:** R1, Poet/language service/qualification. **Depends on:** LX-37/39–41/45–49.

Provide a documented extension SDK/schema and native Poet workflow for defining terms/types/operators, reviewing mappings and previewing typed editors. Generate completion, documentation, inspectors and agent schemas from the same admitted contracts. Extensions carry positive/negative examples, compatibility tests, actual provider evidence and stated unknowns. New domains must be testable out of tree, not only by editing a central list in Vibe's source. Independently authored packs can be curated through cooperative review without requiring all uses to share one institution's interpretation.

**Acceptance:** a new test domain and a new specialist lexicon can be authored, checked, loaded, used, saved in HMC, reopened and upgraded by another contributor using the published extension contract. Human-readable and agent-facing views agree. Domain review is separately identified; a passing schema test does not self-certify legal, engineering or economic suitability.

### Proposed extension contract groups

These are conceptual requirements, not new ratified wire fields or parser syntax:

| Group | Required distinctions |
|---|---|
| Package and dependencies | Qualified identifiers, immutable release/digest, namespace, compatible language/host/schema versions, dependency closure and source/notices. |
| Meaning and language | Canonical concepts, language/script/community/domain senses, source wording, mappings, uncertainty, review/adoption and interpretation versions. |
| Types and values | Constructors, invariants, units/frames, encoding, equality/hash semantics, extension/unknown-field policy and migration. |
| Operators and evaluation | Checked implementation/provider, inputs/results/errors/effects, snapshot/time semantics, resource limits and unsupported cases. |
| Authority and lifecycle | Requested grants, actual admission owner, activation/disposal, validity/revocation, data-disclosure scope and separate legal/physical authority. |
| Tooling and evidence | Source maps, editors/completion/diagnostics, examples/conformance, target coverage and actual reviewer/execution evidence. |

## 8. Material world and manufactured systems

“Material” here includes the user's “world of man”: artefacts, designs, structures, equipment, infrastructure and industrial processes. It is broader than renderer materials. Natural geology, living organisms, harvested feedstock, processed batches, manufactured items and their depictions retain separate referents. Follow the [made-world contract](../made-world.md) and existing REQ-MADE-001–014 rather than inventing a second asset taxonomy.

### LX-51 — Design, configuration, assemblies and as-built state

**Priority/kind:** R2; required authoring subset follows IP-03. **Depends on:** LX-20/27–29/35/45–50.

Define extensible artefact/type, design revision, manufactured instance, fitted component, assembly, installation and representation profiles. Specified BOM, as-built BOM, current fitted configuration and a visual assembly are different records. Preserve stable part references, quantities, spatial/joint constraints, substitution rules, tolerances and discrepancy evidence. Physical containment must remain acyclic where required; functional connections and kinematic/service loops have their own valid graph semantics. Ownership/custody/use rights are separate social/legal relations, not assembly membership.

**Acceptance:** replacing a pump/battery records removal and installation without rewriting the original build; diagrams, CAD and cartoon/game/film representations still select the same parts/configuration. A generic model licence does not grant control of the real machine. Refer to REQ-MADE-001–004/010–012/014.

### LX-52 — Material specifications, batches and property evidence

**Priority/kind:** R2, material/domain packs. **Depends on:** LX-08/28/30/46/51.

Separate material specification, composition/formulation, sourced batch/lot, processing history, samples/tests, fitted material and visual appearance. Properties need units, temperature/humidity/pressure/age conditions, test/model method, uncertainty, anisotropy and frame/direction where relevant. Declared hazard, conductivity, strength and durability are not inferred from a shader. Transformation records connect natural source → harvested/recovered feedstock → processed batch → component; mixtures and material balances retain their bases and tolerances.

**Acceptance:** changing paint/roughness cannot silently change structural mass or fire characteristics; substituting a batch updates qualified evidence/constraints; a blend reports composition basis and unknown constituents. No extension can present an unreviewed simulation as certified material testing. Reuse existing material/physics/quantity owners rather than a Vibe-specific property database.

### LX-53 — Ports, circuits, flows and coupled-system operation

**Priority/kind:** R2, systems/domain adapters. **Depends on:** LX-16/19/20/28/29/51/52.

Represent typed mechanical, electrical, fluid, thermal, data and service interfaces with endpoint frames, medium, compatibility, direction, operating limits and state. Keep rated capacity, requested operation, simulated flow and observed output separate. Operations declare supplies/sinks, resource reservation, conservation assumptions, solver coupling and failure modes. Conservation applies to declared physical quantities and modeled boundaries; missing energy/material terms must not be hidden as visual motion.

**Acceptance:** an incompatible connector or supply is rejected; a pump cannot deliver absent water/energy; a solar/battery model distinguishes power from energy and actual generation from a rating; failed/stale operations do not consume hidden resources. Bind actual QualiaDB physics/solver owners under REQ-MADE-006–008/013.

### LX-54 — Manufacturing, construction and process plans

**Priority/kind:** R2, process/constraint profiles. **Depends on:** LX-29/46/51–53.

Support extensible process steps, work instructions, tooling/machine capabilities, dimensioned features, tolerances, fit/assembly conditions, stock/material inputs, yields, scrap/rework, inspections and as-built deviations. A design/process plan is not proof it was executed or manufacturable on a particular machine. Installation/access/service-clearance constraints belong to the qualified spatial model. Procurement listings, prices and supplier assertions remain attributed inputs; economic projections do not redefine material evidence.

**Acceptance:** an impossible tolerance/tool/stock combination is rejected or flagged unresolved; each actual build/inspection event cites its evidence; rework produces a new configuration/material lineage. Export/import to external CAD/manufacturing formats uses explicit mappings/loss reports, not a claim that every format's semantics are interchangeable.

### LX-55 — Digital twins, maintenance and circular lifecycle

**Priority/kind:** R2, telemetry/lifecycle packs. **Depends on:** LX-11/17/22/27/31/51–54.

Define operational twins only where an identified referent, observation source, clock/calibration, synchronization policy and model-validity envelope exist. Extend lifecycle events for condition, fault, wear, inspection, maintenance, repair, modification, dismantling, reuse and recycling. Record component/material passports and environmental inventories with system boundaries, measurement methods, time/place and uncertainty. Embodied/operating impacts are modelled indicators, not automatic monetary prices or verified offsets.

**Acceptance:** stale/offline sensors are visible, measured/predicted/illustrative states stay distinct, replaced parts retain provenance, and repair/reuse does not reset source-rights or contribution history. A realistic static mesh cannot be advertised as an operational twin solely because it has a telemetry field. Maintenance/circularity extensions can evolve independently of the language release.

### LX-56 — Simulation, training and real-world actuation boundaries

**Priority/kind:** R0 where physical control exists; R2 for domain integration. **Depends on:** LX-15/18–20/25/26/34/49/51–55.

Distinguish illustration, sandbox simulation, training scenario, diagnostic advice and commands to an actual device. A simulated switch or animated mechanism must not acquire physical control by swapping a provider implicitly. Real actuation requires an explicitly admitted device/operation scope, preconditions, bounded execution, interlocks, state acknowledgement and safe failure behavior through the device owner. Engineering/compliance evidence, model limitations and qualified human responsibilities remain explicit; the language does not certify safety from a successful call.

**Acceptance:** a scenario script runs against a labelled simulation provider without touching hardware; wrong-device/stale-state/expired-authority commands fail; loss of connection produces a defined owner-controlled safe state or clearly unresolved state. Human and agent interfaces expose the same real/simulated boundary and applicable operational controls.

## 9. Legal domains, instruments and interpretation

Use the [existing agreements/instruments handoff](semantic-agreements-and-compensation.md), [human-principal authority contract](human-principal-agent-authority.md) and source rights framework in `C:/github/license`. The inspected rights specification is a draft and separates normative authority, jural positions, legal instruments, technical instruments and remedies. These extensions model and operationalise explicitly adopted policies; they do not determine current law, create consent, adjudicate liability or adopt repository drafts merely by evaluating them.

### LX-57 — Jurisdictional, temporal and normative applicability profiles

**Priority/kind:** R2; required source-rights subset early. **Depends on:** LX-11/15/27/36/45–50.

Define extensible profiles for jurisdiction, source of authority, competent forum/institution, parties/capacities, territory, act/purpose, valid/effective dates and review/as-of dates. Keep law, contract, organisational policy, custom and an individual's instructions distinct, with explicit precedence/interpretation rules and disputed/unknown applicability. Preserve source citations and historical versions. No global rule should assume a single country, language or legal tradition; adoption and changes require attributed decisions under the relevant framework.

**Acceptance:** the same act evaluated under two declared profiles can produce different reasoned results without changing the underlying facts; expired/superseded/missing sources are surfaced; a jurisdiction label alone cannot establish applicable law. Business-day/calendar/deadline conventions are profile data, not hidden runtime assumptions.

### LX-58 — Clauses, instruments, translations and adopted releases

**Priority/kind:** R2, agreement/source tooling. **Depends on:** LX-23/24/35/47/57.

Link exact natural-language terms, clause spans, schedules and source releases to their formal interpretation, with rationale, ambiguity, reviewer/adopter and operative-language status. Distinguish proposed wording, offered terms, reviewed draft, adopted release, acceptance evidence, effective conditions and amendments. Grant boundary, condition, covenant, warranty, notice and procedure must not all become an undifferentiated Boolean duty. Translation retains original wording and a qualified interpretation; it cannot silently replace signed bytes or presume legal equivalence.

**Acceptance:** an amended clause produces an explicit version/impact comparison; acceptance binds the actual terms and acting capacity; a generated agreement or acceptance-receipt template remains a draft/template until real evidence exists. Unmapped or ambiguous text remains unresolved rather than being automatically converted into executable authority.

### LX-59 — Rights positions, legal capacity and technical authority

**Priority/kind:** R0 for authority; R2 for richer domain profiles. **Depends on:** LX-15/24/36/46/57/58.

Represent claim/duty, privilege/no-right, power/liability and immunity/disability with their parties, object/action, purpose, authority, time and conditions. Distinguish ownership, authorship, licence, custody, possession, maintenance responsibility and technical access. A human principal's adopted delegation can provide scoped technical authority where the principal holds and may delegate it; delegation alone does not establish legal capacity, source rights or permission for a real-world act. It must not invent a legal power, bypass mandatory protections or equate token possession with consent. Paid access and payment history do not enlarge rights. Preserve multiple qualified identifiers without collapsing referents or roles.

**Acceptance:** a custodian cannot grant rights it does not hold, a model licence does not authorise real-device actuation, and a signature verifies its declared claim without proving that the signer had the necessary legal capacity. Native policy enforcement uses the existing authority owner; legal relationships are not a second hidden permission store.

### LX-60 — Defeasible reasoning, evidence and explanations

**Priority/kind:** R2, logic/instrument profiles. **Depends on:** LX-07/27/36/46/57–59.

Expose rule priorities/exceptions, contrary assertions, temporal conditions, competing interpretations, burden/evidence annotations and unresolved conflicts through declared reasoning profiles. Evaluation returns a scoped decision with source rules/facts, unknowns, defeated rules, assumptions and trace; it is not a universal legal verdict. Reuse existing deontic/logic/instrument owners. Preserve the difference between evidence supplied, cryptographically authenticated, independently assessed and accepted for a particular proceeding/policy.

**Acceptance:** insufficient or contradictory facts can produce indeterminate/review-required outcomes rather than invented certainty; a changed rule/fact produces an inspectable reason difference; the execution receipt names the exact instrument release. [LegalRuleML](https://docs.oasis-open.org/legalruleml/legalruleml-core-spec/v1.0/os/legalruleml-core-spec-v1.0-os.html) provides a reference for formal legal-rule interchange; adopting its encoding does not adjudicate law or validate a local interpretation.

### LX-61 — Obligations, notices, disputes and remedies

**Priority/kind:** R2, legal workflow/native project integration. **Depends on:** LX-11/15/20/35/57–60.

Model triggers, conditions precedent, performance, notice, cure periods, expiry, amendment, alleged breach, contest, suspension, correction, appeal and disposition as evidenced versioned events. Distinguish alleged liability, accepted obligation and adjudicated outcome. Notices need addressed parties, content/version, transport/delivery evidence and a profile-specific receipt rule; sending is not automatically receipt or assent. Legal contest and remedies must preserve evidence, independent review and authorised correction/restoration paths. No automatic fine, debt, retaliation or asset seizure follows merely from a classifier result.

**Acceptance:** disputed facts preserve both positions and route through the adopted review process; routine authorised policy actions remain possible without fabricated adjudication; cancellation/amendment preserves history and surviving obligations under the adopted profile. A remedy record cannot retrospectively turn prohibited conduct into a permitted priced service.

### LX-62 — Source-rights, derivative licensing and commons continuity

**Priority/kind:** R0 required rights records; R2 advanced legal integration. **Depends on:** LX-23/27/35/36/51/58–61/65.

Preserve per-source terms, authorship/contributions, permitted-use scope, lineage and applicable obligations through HMC, high-grade masters and reviewed derivatives. Keep upstream MIT and other existing grants/notices intact; a new ecosystem policy cannot retroactively impose fees or relicense rights already granted by others. Model the actual terms governing new work separately. Commercial recovery under the adopted permissive-commons model remains bounded to the relevant obligation pool; qualifying human-rights no-charge use does not waive contribution claims or falsely discharge that pool. People-led civics and government civic roles retain actual paid/unpaid status separately.

**Acceptance:** deriving/repackaging a work preserves originating contributions and operative terms; a missing/ambiguous right stays open; a no-charge decision records its basis without inventing payment. [ODRL](https://www.w3.org/TR/odrl-model/) can express permissions, prohibitions, duties and constraints where mapped, while the ecosystem retains authority, adoption, jural and remedy distinctions that a generic policy document alone does not settle.

## 10. Economic domains, accounting and cooperative value

Economic extensions include production/resources, contribution and valuation, authorised allocations, obligations, accounting and scenario analysis. They are broader than payments, markets or a single monetary theory. Follow [semantic agreements and compensation](semantic-agreements-and-compensation.md) and [pre-funded credit/period reconciliation](credit-allocations-and-periodic-reconciliation.md), including AGR-01–11, FUND-01–06 and REQ-CREDIT-001–012. Reuse native project/finance/QDNF ownership; payment rails remain IP-08 support work, not a first-master prerequisite.

### LX-63 — Amounts, valuation, prices and heterogeneous resources

**Priority/kind:** R0 for truthful values; R2 for richer economic profiles. **Depends on:** LX-08/11/28/45–48.

Use unit-safe types for money/currency/minor units, micro-units, commodity/resource quantities, labour effort, capital, entitlements, quoted prices and valuation estimates. Bind conversions to an identified rate/method, observation/valid time, rounding, fees, source and authority. A contribution score is not monetary debt; minutes and cents cannot be added without an explicit accepted valuation. Market price, replacement cost, social value, ecosystem impact and an agreed compensation basis are different measures. Keep alternative models and uncertainty contestable rather than hardcoding one universal theory.

**Acceptance:** incompatible units/precisions are rejected; conversion does not invent an exchange rate; rounding/residuals reconcile; an estimate is not labelled an actual charge or payment. Preserve original quantities and the valuation policy/version used to derive a displayed amount.

### LX-64 — Event accounting, reservations and reconciliation invariants

**Priority/kind:** R0 for used financial paths; R2 integration. **Depends on:** LX-19/20/24/63.

Build on immutable unique-ID native records and derived balances. Define the selected accounting model, account/party/purpose roles, per-unit reconciliation and backing. Where double-entry is adopted, balanced postings and reversal/correction rules must be enforced by that owner; do not merely add a “double-entry” label to existing single events. Keep quotes, holds, consumed service, invoiced/accepted obligations, payment and settlement distinct. Deduplicate by durable event/operation references and qualify bounded-table/checkpoint behavior across restart and reordered events.

**Acceptance:** reserve/finalise/cancel/reverse conserve the declared amounts, concurrent requests cannot spend the same backing twice, duplicate events do not change balances, and corrections preserve the audit trail. No accounting invariant proves a liability was validly incurred or that a rail transfer happened; those require their own evidence.

### LX-65 — Contributions, compensation and bounded commons obligations

**Priority/kind:** R0 contribution/rights records; R2 full compensation integration. **Depends on:** LX-15/27/35/58/63/64.

Preserve contributed source/useful work, contributors' roles and actual paid/unpaid status, attributable costs, accepted valuation and adopted recovery rules. Distinguish proposed valuation, disputed claim, accepted target, reserved recovery, actual settled compensation and authorised non-cash discharge. Non-cash discharge requires the adopted obligation/pool rules to accept the particular instrument and its evidenced performance; a no-charge classification alone cannot discharge it. Bind payments to the correct bounded pool and contributor ancestry; derivation, institutional adoption or change of host cannot erase unpaid origins or reset a discharged collective pool into fresh per-user debt. Zero-cost, gift and waiver states need explicit policy/evidence.

**Acceptance:** recovery stops at the applicable target; duplicated/reversed/disputed settlement reconciles correctly; source attribution persists after discharge. The QDNF target/settled/discharge/hold invariant and existing cooperative contribution chain are reused rather than reimplemented in Vibe. Eligible no-charge human-rights use is not a contributor payment or automatic waiver.

### LX-66 — Pre-funded allocations, periods and provider-directed residuals

**Priority/kind:** R2, FUND integration; live rails later. **Depends on:** LX-11/15/20/58/63–65.

Represent provider-backed allocations with purpose/beneficiary, permitted uses, caps, cadence, renewal/cancellation and fee policy. Keep allocation backing, available amounts, holds, consumed amounts and residual disposition separate from contributor obligations and loans. Period statements specify cutoff, late events, holds/disputes and reopening/correction rules; separately accepted obligations may be reconciled without inventing new debt, fees or interest. Netting needs an adopted scope and the necessary parties' authority. Residual return/carry/delegation follows the provider's instructions and retains provider/purpose/unit tranches.

**Acceptance:** drawdown cannot exceed funded permission, repeated period close cannot double-pay or double-return, unresolved holds are not disposable residual, and absent instructions do not cause donation/renewal/overdraft. Reuse REQ-CREDIT-001–012 instead of inventing subscription/billing policy inside language syntax.

### LX-67 — Production economics, access, cooperation and civics scenarios

**Priority/kind:** R2, economic/domain instruments. **Depends on:** LX-22/27/28/36/51–55/57/63–66.

Expose real resources and constraints for capacity, inventories, labour/time, operating/maintenance costs, shared services, procurement, cooperative decision-making and allocation scenarios. Support multiple objectives and distributional outcomes rather than equating success with revenue. Record beneficiaries, unpaid work, access barriers, human-rights purposes and environmental/resource indicators under declared methods. Compare scenarios with pinned assumptions, counterfactuals, uncertainty and sensitivity. Simulation balances remain labelled simulated; an attractive scenario does not prove economic or causal truth.

**Acceptance:** a Community Grounds equipment/repair comparison traces quantities and costs to sources, shows who benefits/contributes/pays, and distinguishes forecasts from observations. Sensitivity analysis changes explicit assumptions without rewriting original evidence. Existing financial economics/solver/instrument owners are traced before implementing a new model; no real transaction is required for this educational scenario.

### LX-68 — Economic instruments, interoperability and actual settlement status

**Priority/kind:** R2; live payment/token qualification remains later support. **Depends on:** LX-14–20/24–26/58/63–67.

Use typed quotes/orders/usage statements/invoices/receipts where needed, preserving party capacity, unit/precision, exact agreement/source references and actual status. A document exchange format does not prove the represented event happened; [UBL](https://docs.oasis-open.org/ubl/os-UBL-2.4/UBL-2.4.html) is a reference for business-document mappings, not a required new financial engine. Optional tokens can represent evidence, entitlement, allocation, obligation or transport, but each role needs explicit issuer/status/backing semantics. Compare rails against actual availability, fees, finality and privacy rather than assuming inexpensive issuance or final payment.

**Acceptance:** unsupported/queued/broadcast/in-process/externally settled outcomes stay distinct; reconnect/reversal/duplicate delivery does not invent compensation; a token transfer is not settlement unless the operative agreement accepts that specific evidenced discharge. Keep private financial/contract data scoped while public contribution/provenance can remain inspectable. No token issuance, live transfer or agreement adoption is implied by loading the domain pack.

## 11. Cross-cutting architectural decisions

### Keep one authoritative owner per concern

Vibe orchestrates native capabilities; it does not become a replacement graph database, solver suite, renderer, payment engine or project history. QualiaDB retains graph/rights/domain ownership, Poet the unified authoring interface, Webizen Desktop native lifecycle/services, Aether the controlled framework, and Etalon its actual graphics/compute responsibilities. Reused MIT-licensed authoring functionality becomes Poet functionality, retaining required notices. Keep Chora separate under its existing engine boundary.

The boundary should be visible in diagnostics and receipts: authored call → checked descriptor → admitted native operation → versioned result → proposed/adopted project change. A pure formula cannot secretly mutate the project because a UI projection rendered it.

### Distinguish encodings, runtime values and authority

Define source bytes, canonical AST bytes, runtime data encodings, digest algorithms and package manifests independently. CBOR is a useful existing basis; specify the exact deterministic-encoding profile rather than assuming every valid CBOR encoder produces identical bytes. Bound nesting, lengths, duplicate keys, tags, decompression and input validation. Unknown tags/versions require an explicit compatibility policy. The [CBOR specification, RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html), is the reference for encoding choices; it does not grant authority or define the application's semantic canonicalisation.

Serialized references can identify resources but cannot recreate a live capability lease. Ordinary records/strings must not mint grants, receipts, cryptographic proofs or accepted review. Offline/HMC reconstruction re-admits the relevant native resources and authority separately.

### Put guarantees in enforceable places

Types/checking prevent invalid composition; native owners enforce resource limits, rights and durable state; services enforce their own external effects. Document which layer enforces each guarantee and what remains a checked assertion or descriptive plan. Tests need to observe the promised state transition, not merely compare a returned `true` field. Keep actual measured evidence and review separate from proposed acceptance criteria.

## 12. Staged delivery without an expanding first-output gate

This is a dependency map for the language agenda, subordinate to the governing IP phases. It is **not** a promise of a fixed number of turns/days. Existing work may already satisfy parts; recheck the current testing branch before estimating or implementing them.

| Stage | Work and dependency focus | Exit evidence | Effect on first asset |
|---|---|---|---|
| S0: reconcile the tested baseline | LX-01/02/06–09/13/14/24; current VT findings; identify exact required profile. | Explicit compatibility decisions; honest catalogue; pinned source and actual positive/negative checks for used paths. | Repair or exclude misleading exposed behaviors; no demand to build every optional feature. |
| S1: qualify required authoring | Required subsets of LX-15–23/26/34/35/37/39/40 using native owners. | Source edit → preview → contributor/reviewer adoption → durable package → fresh reopen, with conflict/invalid-edit/stale-job/authority cases. | This closes actual C1–C8 blockers when demonstrated; documentation alone does not. |
| S2: broaden application ergonomics | LX-03–12/25/38/41 as ratified and useful. | Real forms/data-processing/module/callback programs; typed errors and useful diagnostics. | Can proceed alongside authorised production after the required profile passes; not a new blanket gate. |
| S3: advanced domain authoring | LX-27–36/44 and LX-45–68 against audited owners and actual asset/education/civics cases. Specify the reusable extension contract first; implement only selected needed packs. | Reviewed editable outputs, independently authored extension examples and traceable package/runtime evidence. | Add required cases to existing dossiers; do not lower master quality or wait for all legal/economic features. Full funding/live rails stay on IP-08. |
| S4: optional language research and optimisation | LX-42/43 and measured advanced query/planning work. | Demonstrated correctness and workload benefit; migration/maintenance cost justified. | Independent R&D; does not postpone accepted production paths. |

Some cycles in the conceptual dependency descriptions above are intentional design coordination, not instructions to implement a cyclic build graph: for example, live editing and native UI contracts must be designed together. Before dispatch, split an agreed interface contract from independently implementable providers/consumers, then produce an acyclic task graph. No worker should wait indefinitely for another worker to invent its API.

Resolve the minimal reusable extension contracts with the existing VIBE-01/02/03/09 owners during S0/S1 design where needed, without requiring every future extension to exist. Prove out-of-tree extension loading/tooling as a bounded SDK case. Treat material operations already needed for the first reference as selected required authoring cases; legal/source-rights/contribution records begin early, while advanced instruments, financial products and actual settlement remain later support scope.

### Recommended release policy

Freeze a finite current release scope with its known restrictions and selected profiles. Put unratified additions in a proposal queue. A feature can ship as an explicitly experimental, non-production extension with honest support labels; it cannot be called a completed production guarantee. Do not keep renaming an unfinished rolling branch as a stable release. Maintain security/integrity corrections separately from optional syntax expansion so they are not indefinitely deferred behind language ambition.

## 13. Independently assignable work tracks

These are **prepared assignments**, not dispatched implementation. The user requested this Markdown feedback, and existing source updates remain under test. Resume source work only under the governing coordination instructions and writable-repository permissions.

| Track | Recommended capability/cost level | Scope / LX references | Owned output and boundary |
|---|---|---|---|
| A: baseline and compatibility | Lower-cost capable reader; strong reviewer for disputed semantics. | LX-01/02/06–12/23; reconcile source/tests/spec. | Feature/status matrix, exact pointers, compatibility decisions. Read-only first; no manifests or runtime edits. |
| B: type/function design | Strong language/checker implementer. | LX-03–07/12/13. | Ratified function/capture/generic/ADT/effect contract, then bounded AST/check/eval changes. One worker owns shared type/AST changes. |
| C: runtime and native authority | Strong runtime/security implementer with independent review. | LX-14–24. | Typed host/job/resource/state contract and required native adapters. Reuse AUTH-H/VIBE jobs; do not invent parallel stores or proof systems. |
| D: library and domain bindings | Lower-cost implementer for mechanical wrappers; strong domain implementer/reviewer for numerical/scientific semantics. | LX-08–11/25/27–33/36/44. | Disjoint owner-specific adapters and meaningful reference cases. Numerical and biological acceptance need suitable experts, not only general coding agents. |
| E: Poet and language service | Mid-cost UI/tooling implementer; strong reviewer for cooperative state semantics. | LX-21/34/35/37–39. | Native human/agent authoring surfaces over shared services; preserve existing Poet behavior. |
| F: independent conformance | Lower-cost runner for fixed checks after implementation; strong reviewer for runtime/security/migration evidence. | LX-26/40/41 and selected profile claims. | Evidence ledger and gap report; never infer execution from declarations. No source edits during the qualification lease. |
| G: measured optimisation/research | Strong specialist, only after a concrete bottleneck/proposal is approved. | LX-42/43. | Bounded experiment and benchmark/correctness report, not mandatory new architecture. |
| H: extension contract and lexicons | Strong type/runtime designer for shared contracts; lower-cost readers for vocabulary inventories and mappings, with fluent/domain review. | LX-45–50, existing vocab/locale/module/schema owners. | Open extension SDK/manifest/semantic composition, independent test pack and language-service support. One shared-interface owner; no global parser edits by competing workers. |
| I: made-world profiles | Domain-capable engineering implementer/reviewer; lower-cost workers for mechanical schemas/fixtures. | LX-51–56 and existing G73/REQ-MADE work. | Selected configuration/material/port/lifecycle adapters and negative cases against existing native engineering/state owners. Physical controls require their own qualified owner. |
| J: legal/economic profiles | Strong formal-logic/accounting/authority implementers; qualified human reviewers for adopted terms, legal interpretation and valuation policy. | LX-57–68; existing AGR/FUND/AUTH-H owners. | Versioned legal/economic packs, wrapper repairs and explicit source/evidence/status mappings. Payments later; no private replacement agreement, accounting or billing store. |

With a four-agent concurrency limit, use the coordinator plus three workers. For read-only discovery, A/B, C and D/E can research independently. For implementation, schedule disjoint source owners only after the coordinator freezes shared interfaces. One coordinator owns `Cargo.toml`, `Cargo.lock`, dependency feature changes, public shared types and serial Cargo checks. A lower-cost worker is appropriate for inventories, documentation, fixture preparation and mechanical adapters with a clear oracle. Use a stronger worker for semantic design, async/effect ownership, cryptography/authority, transactions and concurrency; savings from an unsuitable worker can disappear in review/rework. Current review used two bounded lower-cost readers and one stronger runtime reader, with coordinator review.

### Required handoff fields

Every implementation assignment should specify:

1. Existing governing work IDs plus LX references; exact product case and finite in/out scope.
2. Current source/dependency/catalogue pins and prior valid evidence to preserve.
3. Reused native owner and traced public call path; unknown scope must remain labelled unknown.
4. Owned files/directories and shared interfaces, with a named coordinator for contested edits.
5. Semantics, failure modes, authority/resource rules and compatibility/migration impact.
6. Expected positive/negative/integration cases, commands, targets/providers and serial-check lease.
7. Concrete artifacts and evidence paths; distinction between source saved, executed, independently reviewed and human accepted.
8. Remaining gaps, no-op/descriptor-only cases and rollback/recovery instructions.

Use the existing [handoff template](../agent-handoff.template.json) and [job register](../biosphere-work-packets.json); add adopted detail there rather than maintaining a conflicting implementation queue here.

## 14. Representative acceptance programs and workflows

These are scenario definitions, not fabricated executable syntax or executed results. Map each adopted case into existing VBC/C1–C8 records and extend only where coverage is genuinely new.

| Case | What the program/workflow does | Observable acceptance |
|---|---|---|
| APP-01 | A local form filters Unicode text, validates typed fields and saves through an admitted file/project owner. | Correct byte/scalar/grapheme handling; no write during rendering; invalid input preserves source; keyboard/IME works. |
| APP-02 | A generic transform uses a closure over immutable configuration and returns typed failures. | Correct lexical capture, inference, errors and transitive effects; no ambient grant capture. |
| APP-03 | An application imports pinned modules offline, then explicitly activates selected hooks. | Changed bytes fail the pin; import does not run effects; each hook is installed once. |
| APP-04 | A cancellable native solve produces progress while its source is edited. | UI remains responsive; cancel acknowledgement is distinguished from request; late output cannot replace the new source. |
| APP-05 | A large synthetic embodiment is inspected at whole/part/detail scales. | Bounded memory/transfer; dependency closure and provenance retained; missing data is explicit. No real asset production is required to test streaming mechanics. |
| APP-06 | A material/UV edit is previewed, proposed by a contributor, reviewed and saved/reopened. | Same typed command via user/agent, actual native output, revision-bound review and recoverable editable source. |
| APP-07 | A delegated agent attempts a changed-scope action after revocation. | Native denial on every UI/MCP/Vibe route; uncommitted output remains private/inspectable only as permitted. |
| APP-08 | A time/seed/snapshot-pinned simulation drives a developmental or motion parameter. | Reproducible admitted behavior within declared bounds; clocks/units/biological state stay distinct. |
| APP-09 | An external write loses transport before an acknowledgement. | Recovery reconciles the native job/receipt; no blind duplicate write or invented rollback. |
| APP-10 | A source formatter, structural edit and optional VM process a supported sample. | Preserved meaning/comments/source links; supported VM agrees; effectful execution is not repeated on fallback. |
| APP-11 | Two contributors edit overlapping source while a reviewer examines a candidate. | Semantic conflicts retained, attribution preserved and adoption pinned to the reviewed source/output version. |
| APP-12 | A query returns uncertain/conflicting/restricted biological information. | Attributed evidence and unknown/denied distinctions preserved; visualization cannot convert missing facts into invented anatomy. |
| APP-13 | An independent contributor defines a previously unknown domain pack using the published extension SDK. | Installed compatible interpreter loads checked schemas/modules without a core/Cargo rebuild; tooling and HMC round-trip preserve the pack; unavailable native operators stay explicit. |
| APP-14 | Two specialist or human-language packs share a spelling but define different senses; a new version changes a mapping. | Qualification resolves ambiguity; original wording and pinned meaning remain; migration is reviewed; unknown critical semantics hold execution. |
| APP-15 | A manufactured pump receives a replacement part from a recorded material batch. | Design/as-built/current configuration, removal/install events, quantities and source/rights lineage remain distinct through CAD and visual representations. |
| APP-16 | A simulated water/energy system requests operation with incompatible ports or missing supply. | Actual native simulation owner rejects/flags the request without hidden consumption; no real device is controlled; rated, predicted and observed states remain separate. |
| APP-17 | An agreement draft is translated, mapped to formal rules, reviewed and offered under a versioned profile. | Original/translated wording, interpretation and adoption/acceptance evidence stay separate; missing authority or ambiguous applicable rules remain unresolved. No adoption is fabricated for the test. |
| APP-18 | Nonempty accounting data and individually unbalanced entries are sent to the public Vibe adapters. | Correct output sizing, explicit projection-versus-validation contract, checked wide totals and meaningful errors; no false “real/balanced” claim from an empty-result fixture. |
| APP-19 | A provider-backed allocation spans periods with holds, a reversal and residual instructions. | Funding, contributor obligations, consumed service and settlement remain distinct; no duplicated return/delegation, silent donation, renewal or debt. Use labelled fixtures until actual rails are qualified. |
| APP-20 | An authorised commercial/no-charge derivative is considered for a made-world asset with contributor history. | Actual operative source terms and bounded recovery apply; MIT/upstream grants remain intact; no-charge use is not payment/waiver/discharge; a government role does not erase people-led unpaid contributions. |

### Cross-domain composition example

A Community Grounds pump can reference a design and current fitted parts, sourced material batches, a simulated or observed operating state, scoped maintenance authority, model/source licence terms, contribution history, and an optional provider allocation. These are linked records with separate owners, not one universal “asset status”. A repair proposal checks compatibility and quantities, adopted permission and source revision, then any authorised reservation. The selected owner records actual execution, configuration changes and financial outcome separately. A simulation-only repair can illustrate alternatives without creating a real obligation, payment or physical action. New domains can join through explicit versioned mappings without redefining what the pump or its contributors are.

## 15. Deliberate deferrals and decision criteria

Do not make prototype inheritance, implicit coercion, ambient browser globals, npm compatibility, arbitrary string evaluation, mandatory JIT/AOT, unrestricted native plugins, general distributed execution or universal proof-system support requirements merely because another language offers them. If a real application needs an analogous task, design the explicit typed/host-bound alternative and document it.

Likewise, dependent types, higher-kinded abstractions, general continuations, multi-shot effects, user-extensible parser grammars, mutable shared closures and automatic cross-device parallelisation deserve focused proposals before implementation. They can expand expressiveness while imposing substantial checker, runtime, tool and maintenance costs.

For each addition ask: which authored task becomes materially better; which existing code already serves it; is syntax necessary; what semantics/authority/resource guarantees change; how will users debug it; how is compatibility preserved; what executable evidence closes the claim? Prefer a small coherent feature with complete behavior over several impressive names with descriptive implementations.

## 16. Evidence and review limits

The initial review's [source receipt](../../qa/implementation-2026-10-27/vibescript-language-expansion/source-inspection.json) records its capture-time hashes, targeted inspection scopes and original document checks. It does not certify a frozen checkout, complete source audit or executed language conformance. Current tests elsewhere may already repair findings; reconcile their exact revision/evidence first. The earlier [testing receipt](../../qa/implementation-2026-10-27/vibescript-testing-review/source-inspection.json) remains a separate historical capture.

The [domain-extension amendment receipt](../../qa/implementation-2026-10-27/vibescript-language-expansion/domain-extension-review.json) separately pins the additional selected extension/material/legal/economic sources and records this revision's document checks. Initial runtime findings above remain observations of their original captures, not a statement that the subsequently changing testing branch still has every issue. Legal/financial standards are consulted for modelling/interchange; no current-law opinion, valuation, adopted agreement, live balance or transaction is determined by this document.

External references above are primary specifications consulted for particular design comparisons. They are not dependencies automatically introduced into the product, nor proof that Vibe implements those standards. No paid provider calls, native launches, builds, tests, exports, commits or publication were performed for this feedback. No model/package or production-readiness gate is declared complete.
