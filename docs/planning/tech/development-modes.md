# Development modes — VibeScript-first, and the two browser stacks

**Status:** draft v0.1 · the development-practice rule for everything ecopunk
builds on this stack.

## 1. The rule

**Qualia-native environments run VibeScript, not JavaScript.** `vibe-0.1` is
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

## 2. The two browser stacks (modes, not tiers)

| | **Native mode** | **Standard-web mode** |
|---|---|---|
| Stack | Qualia's own browser/Webizen stack — VibeScript-hosted UI, `.vibe` shader/asset sources (e.g. `portal/graphics_backend.vibe`), `.hmc` packs, WebGPU/native render | WASM profile ladder (`wasm-edge.md`) served to standard browsers |
| Scripting | VibeScript | JS shims bind the WASM surface — the *interop bridge*, not the authoring model |
| Role | primary | compatibility/projection — *supported, as a different kind of mode* |

The standard web is a **target the stack projects into** — not the thing the
stack is made of. Practically: author once in VibeScript; the standard-web
mode is a compiled/projected surface (WASM + minimal JS glue), not a parallel
hand-maintained codebase. Native-mode surfaces get the full stack (portal
renderer, `.vibe` sources, direct substrate); standard-web mode gets whatever
the declared profile supports — with the profile gap stated honestly per
feature (wasm-edge.md §6 tiers apply).

## 3. What this means for ecopunk work items

1. **Interface authoring** — encyclopedia/explorer/contribution UIs are
   VibeScript documents/QApps; Poet chain_actions invoke `poet_host/invoke`
   domains. No new JS business logic in ecopunk surfaces.
2. **Gap protocol** — a missing VibeScript capability is a documented
   `crates/vibe` or engine requirement for Timothy to commission upstream,
   not permission to implement a JS fallback. Record the public interface
   needed, inspected revision/profile, source evidence, and acceptance case.
   A browser-only interop exception requires the specific platform operation
   and why an existing Vibe/engine binding cannot perform it; keep authored
   rules, semantic state transitions, and evaluation logic out of that shim.
3. **Pack mode declarations** — packs should declare `native` / `std-web`
   surface coverage honestly; a pack may be native-only where the standard
   profile can't carry it (that's a stated boundary, not a defect).
4. **The 168 `vibe/fixtures`** are the language's existing coverage corpus —
   check there first before declaring a gap.
5. **WebizenVM continuity** — VibeScript compiles to the Webizen bytecode/
   modality machinery (`logic/` WebizenVM/compiler, `webizen_bytecode`);
   extension work inherits the 64-opcode/bytecode discipline and opcode
   ranges (AGENTS.md §1).
6. **Operational boundary for existing JS surfaces** — game-demo's current JS
   owns semantic state, walking, cameras, and scene lifecycle; migrating that
   to Vibe-owned scheduling over publicly supported bindings is remediation
   work R2/R4 (see
   [game-demo-remediation.md](game-demo-remediation.md),
   [webizen-execution.md](webizen-execution.md)). Browser **bootstrapping**
   JS (interop glue, loaders) is not prohibited — the rule is *no new
   business logic* in JS. Migration is not an automatic speedup: bounded
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
   implementation. Browser interop exceptions identify the necessary platform
   operation, checked Vibe/engine bindings, and the narrow shim responsibility.
   Rust adapters marshal inputs and outputs; they must not silently take
   ownership of authored semantic/game policy.
2. **Separate claims from evidence.** Record revision, build/profile, input
   artifact identity, actual invocation path, expected and observed result,
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
