# Development modes — controlled framework, native launcher and WASM

**Status:** draft v0.4 · updated 2026-10-24 from native/framework/WASM and source-REPL clarifications.
The [architecture](../architecture.md) governs authoring, packaging and runtime.
The [environment/framework contract](webizen-environment-framework.md) defines
**Aether**, the controlled Dioxus fork, Webizen Desktop launcher/host and native-assisted WASM.

The [Chora engine boundary](chora-engine-boundary.md) keeps the Servo fork at
`C:/github/chora` separate, with generic interfaces and host adapters outside
its source/workspace. Existing references to a Chora exploration canvas below
are historical application terminology pending source/dependency reconciliation.
The inspected Chora embedded profile enables JS JIT; its use requires explicit
qualification against the native workflow below.

## 1. The rule

**The native workflow requires the Qualia-native stack without JavaScript,
Node.js or V8**, including authoring, package construction, UI, graph query,
simulation and rendering. Harmonised WASM versions are also required; they
use the same authored rules/contracts and running native capabilities where
declared necessary. `vibe-0.1` is
the JS-replacement language (`crates/vibe` — its own lib doc: *"Vibe is the
scripting language (the JS replacement). Poet is the human creative
environment and CLI that hosts Vibe"*). Poet hosts it; Chora consumes the
packs it authors.

**When required functionality is not supported in VibeScript, advise Timothy
of the exact upstream requirement so he can arrange its implementation; do
not inject a JavaScript substitute.** Check existing language functionality,
fixtures, and exposed host bindings first. Distinguish a language capability
gap from a missing host binding and a browser-only operation. Document the
requirement, source evidence, and acceptance case; do not implement it in
QualiaDB under game-demo/ecopunk authority.

Rationale: a JS fallback quietly re-introduces exactly what the stack exists
to remove — an ungoverned execution tier beside the governed one. VibeScript
carries what JS lacks by construction: `capability_schema` (declared
capabilities), `law_package` + `deontic_interrupt` (governance *in* the
language), `budget`/`degrade` (designed boundedness + degradation), receipts/
provenance paths, and the two-tens visibility rule (pose lanes visible,
epistemic internals not).

## 2. Supported execution modes

| Mode | Application execution | Qualification |
|---|---|---|
| Native | Controlled framework's qualified native renderer/host; Vibe-authored interaction and native Rust/custom engine capabilities. Webizen Desktop supplies launcher and native environment services. | Actual fork/backend/dependency provenance, no required JS/V8 workflow, real app/service operation and data recovery. |
| WASM local | Shared authored application compiled for the supported WASM profile; locally available capabilities execute there. | Actual profile coverage, local/offline behaviour and explicit unsupported/held features; no promise that every native operation runs in-browser. |
| WASM native-assisted | Same application projection using an authorized compatible running native provider for declared capabilities. | Versioned discovery/pairing, scoped authority, one state owner, jobs/cancellation and disconnect/reconnect/recovery. |

WASM is a supported product target, not merely a legacy surface. Existing
hand-maintained JS editor/state/control logic remains legacy debt. Browser
platform/DOM binding infrastructure must be declared and qualified through
the fork; it is not a substitute for Vibe/engine functionality and cannot
establish native no-JS/V8 conformance. Required authored logic remains shared.
The inspected fork's `desktop` path uses a webview/JS interpreter; its `native`
path is a distinct renderer candidate. Do not equate those feature names.

## 3. What this means for ecopunk work items

1. **Interface authoring** — encyclopedia/explorer/contribution UIs are
   VibeScript documents/QApps; Poet chain_actions invoke `poet_host/invoke`
   domains. No new JS business logic in ecopunk surfaces.
2. **Gap protocol** — a missing VibeScript capability is a documented
   `crates/vibe` or engine requirement for Timothy to commission upstream,
   not permission to implement a JS fallback. Record the public interface
   needed, inspected revision/profile, source evidence, and acceptance case.
   Any browser platform-binding need must be explicit in the controlled fork's
   projection contract; it cannot introduce new JS business/state/physics logic
   or become a dependency of the JS/V8-independent native workflow.
3. **Pack and capability modes** — declare native, WASM-local and WASM-assisted
   coverage and required native providers per operation. An unsupported browser
   operation stays held; required WASM product coverage remains open until
   delivered, rather than silently relabelling the entire app native-only.
4. **Existing coverage corpus** — read the current language fixtures before
   declaring a gap. The 2026-10-24 source inventory found 166 `.vibe` fixtures
   and eight integration-test source files; these counts are not executed passes.
5. **Source-evaluated REPL** — `.vibe` source is parsed, checked and evaluated
   by the installed interpreter. Script edits do not require Rust/Cargo builds,
   JS translation or generated WASM binaries. Bytecode is optional, separately
   versioned and parity-qualified for its supported subset. Existing Webizen VM
   compiler/opcode work retains its own contracts; it is not a prerequisite for
   every interactive script. Follow the [proposed 0.2 definition](vibescript-0.2-specification.md)
   and [source audit](vibescript-0.2-source-and-conformance.md), preserving pinned
   0.1 semantics until explicit migration. Required REPL/host/live-edit gaps are
   ENG-13/G89 work in the governing implementation plan.
6. **Operational boundary for existing JS surfaces** — game-demo's current JS
   owns semantic state, walking, cameras, and scene lifecycle; migrating that
   to Vibe-owned scheduling over publicly supported bindings is remediation
   work R2/R4 (see
   [game-demo-remediation.md](game-demo-remediation.md),
   [webizen-execution.md](webizen-execution.md)). Existing bootstrapping/binding
   machinery requires explicit target-specific framework qualification and is
   not an allowed dependency of the JS/V8-independent native workflow.
   Migration is not an automatic speedup: bounded
   Rust kernels/GPU work stay behind Vibe, and speed comes from the
   corrections in the remediation plan, not the language move itself.

## 4. Open questions

1. Which `vibe` subsystems are stable vs in-flight (`decompiler`, `metamodel`,
   `projectional`, `conditioning`…) — a maturity map before committing
   ecopunk UI chains.
2. `vibe-host-0.1` ABI: Poet hosts it — does Chora/WellFair host Vibe too, or
   consume compiled artifacts only?
3. Standard-web projection toolchain — is there a `vibe`→WASM-mode path
   (compile to the wasm surface) or is std-web mode written against
   `wasm_bridge` directly?
4. Where does `vibe` governance (`law_package`, `deontic_interrupt`) intersect
   the pack/SHACL validation gates — same rule source or two?
5. Exact controlled fork pin/features, native-renderer integration, shared
   launcher/Poet UI contract and per-capability WASM/native provider requirements.

## 5. Supervised development and evaluation integrity

**Authority clarification (Timothy, this supervision review):** continued
work is on game-demo and ecopunk. QualiaDB is a read-only investigation
source: advise Timothy of required engine/language changes, interfaces, and
acceptance cases so he can arrange upstream implementation. Do not edit its
source, dependencies, configuration, or artifacts. Continued game-demo and
ecopunk work does not authorise upstream implementation. Earlier handoff
implementation wording does not override this clarified boundary. Existing
engine edits remain unreconciled; do not silently revert them or treat them
as an approved baseline.

The game is also an evaluation/testing environment for QualiaDB and Webizen,
not merely a game whose visible output can be reproduced by another stack.
A plausible screen or passing game-owned substitute is not engine evidence.

1. **Trace purpose to execution.** Each work packet identifies the user
   requirement, authored VibeScript entry point, public engine API, allowed
   files, and acceptance cases. Missing capability becomes an upstream
   requirements entry reported to Timothy, not a JS or game-owned replacement
   implementation. Historical compatibility interop mapping is not authorization
   to add JS/V8 dependencies to the native workflow clarified on 2026-10-24.
   Rust adapters marshal inputs and outputs; they must not silently take
   ownership of authored semantic/game policy.
2. **Separate claims from evidence.** Record revision, build/profile, input
   artifact identifiers, actual invocation path, expected and observed result,
   and evidence location. Distinguish source inspection, compiled code,
   executed fixture, and rendered-browser integration. Unexecuted checks
   stay unverified; do not manufacture receipts or mark them passed.
3. **Preserve successful baselines.** Keep earlier working HMC/Q42 receipts
   and their scope. A failure in one browser binding or new integration is
   not evidence that the filesystem or format is broken. Compare the same
   identified artifacts before diagnosing a regression. Proposed patches
   and modified engine trees are not proof of the baseline's behaviour.
4. **Protect evaluation validity.** Evaluation paths must exercise the
   declared engine capability. Fixtures and independent expected-result
   oracles may be game-owned, but cannot replace that capability. Record
   fallback/mock activation explicitly and exclude it from engine-success
   claims. Include negative cases for denied capability, corrupt input,
   budget exhaustion, and stale results where relevant to the requirement.
5. **No coercive recovery.** Failure or denied permission leaves existing
   work intact. Never condition repair on accepting scope reduction,
   provider attribution, broader authority, or rollback. Report the exact
   blocker and continue independent authorised work. Reversal, deletion,
   history rewriting, publication, and privilege changes need specific
   authority; no unilateral cleanup of the user's changes.
6. **Respect execution and attribution limits.** No unrequested builds,
   tests, exports, commits, pushes, or provider bylines. Describe the narrow
   verification needed and its status without making permission to run it
   a condition for unrelated authorised progress.
7. **Review before integration.** Supervision owns the requirements and
   reviews the complete patch against them, including the actual execution
   path and evidence. Delegation does not expand scope or permissions;
   where its controls cannot be verified, do not delegate authority-bearing
   actions. Surface conflicts rather than silently choosing another agenda.
8. **Evolve guards from observed failures.** Record the failure mechanism,
   affected requirement, proposed preventive control, enforcement owner,
   and acceptance case. Preserve prior findings; corrections are explicit.
   Distinguish proposed, configured, and independently tested controls.

**Observed harness failure and corrective gate:** the first single-asset UAT
candidate placed review control in an 800-line JS controller and a JS evidence
module despite this boundary. That was a supervision failure, not proof that
Vibe lacked the functionality. The correction authors catalog/selection,
admission rules, RDF projection, camera bounds and human-note schema in
`game-demo/web/fixtures/asset-review.vibe`. Before authorising future work,
identify each decision's Vibe entry point and each remaining browser-only API
(fetch/stream, DOM/canvas, Web Crypto, download). Review that mapping against
the actual patch; a promise of "minimal glue" is not evidence of compliance.
The corrected source still requires runtime UAT; no test or render pass is
inferred from moving code between languages.

These are working requirements, not a claim of technical containment. Strong
controls belong outside agent-writable scope: narrow filesystem authority,
protected approval/policy records, restricted command/network execution,
resource limits, and externally held audit evidence. Permission refusal must
not itself trigger rollback or data loss. Installed CLI documentation reports
OS-level sandboxing unsupported on Windows; prompts, hooks, and repository
rules alone must not be represented as an OS isolation guarantee.
