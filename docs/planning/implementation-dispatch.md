# Implementation dispatch — authorized bounded batches

**Human principal / agent authority, 2026-10-26:** Authentication must preserve an admitted human principal and the scoped, expiring, revocable rights of separately authenticated agents. Apply the [hierarchy contract](tech/human-principal-agent-authority.md). Concurrent-session diagnostics do not prove a human delegation chain or independent review. AUTH-H1–H6 refine the existing IP-03 cooperation gates and remain open pending implementation/qualification.

**Started:** 2026-10-24. **Current direction, 2026-10-26:** bounded implementation has resumed by explicit user request. Use [active biosphere authoring round 1](biosphere-authoring-round1.md) and the [current job register](biosphere-work-packets.json). Preserve existing work and source boundaries. No commits, publication, paid providers, financial transfers or production assets. Historical RUN records below remain evidence, not instructions to repeat completed work.

The [governing plan](implementation-plan.md) retains all 120 packets, phase
dependencies and IP-03 C1–C8. This dispatch selects executable jobs within that
order. Independent diagnostic/integration fixes can proceed while subject/visual
acceptance decisions remain open; they do not certify an incomplete phase.

## Priority and assignment policy

Machine-readable contracts: [agent roles](implementation-agent-roles.json) and
[all 120 packets in dependency order](implementation-queue.json). Queue entries
retain every dependency and acceptance clause; they are not automatic permission
to dispatch an unbounded packet. Decompose a job into disjoint files/commands,
check dependencies and select the appropriate role before starting a worker.

1. **P0 — known correctness and reproducibility defects:** establish actual
   source/dependency/resource paths; fix incorrect texture admission and swallowed
   upload errors; verify manifests without rewriting reviewed content or claims.
2. **P0 — usable native authoring:** repair demonstrated REPL/session hazards,
   then qualify native Poet/host editing, jobs and source-to-HMC recovery. Reuse
   the 0.1 interpreter; a speculative full 0.2 rewrite is not the first task.
3. **P1 — missing authoring operators and exceptional masters:** finish the
   specific organic/CAD/material/rig operations exposed by the reference work,
   demonstrate cooperation, then produce and review editable high-grade sources.
4. **P1 — complete HMCs, deliberate derivatives and actual game-demo use:** retain
   all model claims and human acceptance; no screenshot-only completion.
5. **Later support:** unused backends/extensions and live payment/token adapters.

Use **gpt-6-luna** for bounded inventory, deterministic metadata checks and simple
documentation updates. Use **gpt-6.1-sol** for Rust/runtime/render/language changes,
integration diagnosis and independent review. Escalate a genuinely unresolved
architectural problem to **gpt-6-astra** only with a specific question/evidence;
do not assign every packet to the largest model. These are routing choices, not
a quoted price comparison. Match reasoning effort to uncertainty.

Every assignment specifies owned files, immutable inputs, dependencies, allowed
commands, acceptance and handoff evidence. Workers cannot widen scope or assign
themselves human acceptance. Parallel workers own disjoint files; the supervisor
reviews integration and runs only necessary checks. Reuse results until changes
or failures justify repeating them. External repository patches need their own
instructions and filesystem authority; stage exact patches if direct access is
unavailable rather than claiming them applied.

## Initial disjoint jobs

| Job | Allocation / agent | Owned files and deliverable | Acceptance / boundary |
|---|---|---|---|
| RUN-01 | ECO-003/009/011; gpt-6-luna inventory worker | `docs/qa/implementation-2026-10-24/stack-source-matrix.*` only. Record active native/WASM source declarations/locks and device/renderer/language seams; distinguish presence from executed resolution. | Machine-readable source evidence with paths/digests and concrete blockers. No dependency upgrades, sibling writes or build. |
| RUN-02 | ECO-051/052; ENG-02/06; gpt-6.1-sol graphics worker | Focused texture-admission module and wiring in `crates/ecopunk-wasm/src/`; meaningful focused tests. | Correct declared base-colour semantics, explicit upload failures, no silent neutral success or new JS logic. Preserve upstream public API and native/WASM distinction. No lighting guessing, model regeneration or full-world export. |
| RUN-03 | ECO-034/044; ENG-13; VIBE-04; gpt-6.1-sol runtime worker | Stage a reviewed, digest-guarded patch for native Poet REPL under `docs/qa/implementation-2026-10-24/poet-repl/`; source siblings read-only until an authorized apply. | Parse mode selected before evaluation, runtime errors preserved without retry, persistent program/function context and focused regression cases as feasible. Preserve 0.1 semantics; do not bump dialect or implement speculative language changes. |
| RUN-04 | ECO-021–023; integration supervisor | Native/script resource integrity checks and scoped implementation evidence/claims; integrate worker patches without overlapping their active files. | Verify real stored bytes versus manifest; reject unsafe paths and corruption; preserve original assets and review status. Record actual passed/failed/unrun checks and remaining work. |

RUN-04's bounded native metadata tool is assigned to the lower-cost tools worker;
the supervisor and a separate stronger reviewer check its bounds, canonical
resource/ref outcomes and actual stored-data results. Review findings must be
fixed before treating its output as reliable evidence.

## Canonical language prerequisite — RUN-05 (P0, applied; integration checks recorded separately)

RUN-03's real-Vibe probes reproduced two canonical defects: a cell parser can
ignore tokens after a semicolon; the checker can admit a declared Pure function
that calls an External function. The probe code/log is under
`docs/qa/implementation-2026-10-24/poet-repl/`. These are observations of defects,
not passing language conformance cases.

The direct native REPL also reproduced a cross-submission mutable-binding
failure: a prior `let mut` value remains readable but its later assignment is
rejected. Baseline and patched observations confirm this predates the narrow
repair; the old fallback hid E701 behind E001, while the patch preserves E701.
See `poet-repl/session-mutability-handoff.md` in the execution evidence.

Assign a language-runtime worker to the canonical QualiaDB parser/checker with
explicit source/file authority, transitive call/alias/recursive-effect analysis
and meaningful negative cases before enabling persistent REPL functions or live
program contexts. Include retained typed/mutable/scoped declaration and
source/revision/import/grant context, immutable controls and failure-safe state
activation. Reuse existing checking/runtime APIs; no Poet-local checker,
language-version bump or broad engine rewrite. This refines VIBE-02/03/04,
VBC-004/011/019 and ECO-034/044 under ENG-13. Preserve approved 0.1 behavior
outside the exact malformed-input/authority correction. Its implementation is
not claimed by the narrower duplicate-dispatch repair.

The [second-batch record](../qa/implementation-2026-10-24/batch-2/README.md)
supersedes this section's original queued state. RUN-05A's two-target parser patch
and RUN-05B's independently reviewed 24-target canonical Vibe/native Poet package
have been applied with original/candidate digest guards. Private verification is
distinct from subsequent actual-source integration results. Version remains 0.1;
durable shared sessions, checked replacement, live jobs and complete 0.2
conformance remain open.

## Native authoring continuation — RUN-06/07 (P0)

| Job | Allocation / agent | Owned work | Required evidence |
| --- | --- | --- | --- |
| RUN-06A | ECO-035/036/044; gpt-6-luna inventory | Narrow CADCraft/Poet source inspection, staged evidence only. | Limit findings to inspected paths; do not infer a missing 3D engine from a 2D GUI. |
| RUN-06B | ECO-034/036/044; gpt-6.1-sol authoring worker, supervisor review | Four guarded canonical Studio store/domain/journal files; actual source undo/redo and atomic revision admission. | Ten focused passes; wider 16 passes and one original inline-CML failure disclosed; real cached core kernel regeneration scope explicit. Applied source is distinct from whole-core consumer qualification. |
| RUN-07A | ECO-033/034/035/036/044; gpt-6.1-sol native integration | Additive native Poet module, real Aether native launch/dependency contract and revision-bound Qualia/Etalon geometry/material preview. | Exact dependency closure, real build/window/render evidence; preserve Poet Tool-Chest/Manifold concepts. No webview/JS substitute, fake renderer, receipt-only production controls or source relocation. |

The lower-cost parser worker performs the small full-input correction and final
source-pin inventory. The stronger runtime worker owns canonical checking/session
semantics and target verification; a separate stronger reviewer checks authority,
failure activation, resource bounds and compatibility. The supervisor owns
guarded integration, consumer verification and broad claims. Keep concurrent
source ownership disjoint and retain ownership of checks through process exit.

Next acceptance depends on a usable native source-to-geometry/material workflow,
then real contributor/reviewer editing, revision conflicts, source recovery,
durable reopen and complete HMC retention. Fixed Studio/CAD/Scene receipt helpers
are explicitly unqualified until connected to actual operators and output.
High-grade masters and complete dossiers retain priority over payment rails.

RUN-07A's guarded optional-native package has subsequently been applied to the
actual Poet workspace (2026-10-25 local), including targeted browser binding/test
adapter alignment required by its current Etalon closure. The implementation
adds real geometry dispatch, source undo/rebuild and stale-result guards; its
actual native-feature tests, regular binary and window proof are tracked in the
second-batch record. Static PBR/SDR working preview is not material authoring,
HMC retention, durable cooperation or high-grade model acceptance.

## Native cooperative continuation — RUN-08 (P0)

The [batch-3 record](../qa/implementation-2026-10-25/batch-3/README.md) owns source
guards and executed results. Strong Rust workers implemented disjoint cooperative
policy/MCP, HMC persistence and material/rendering scopes; an independent worker
reviewed the AO consumers and a lower-cost worker audited hashes and open claims.
The supervisor owns composition, coherent builds and the final evidence ledger.

| Slice | Required completion evidence | Remaining production dependency |
| --- | --- | --- |
| RUN-08-AUTH | Real canonical project types; denial/role/revision/review cases; native publication freshness | Existing native proof, identifier association and membership-admission provider; durable contribution/review commit |
| RUN-08-MCP | Strict lifecycle/framing and actual protocol → policy → shared canonical editor | Real host admission and complete required operators; no receipt-only success or JSON principal |
| RUN-08-HMC | Native source/material edit → save → two fresh-process reopen/rebuild/edit; exact raw-source witnesses and measured derivative loss | Full embodiment/graph/resource retention and resource-preserving package editing |
| RUN-08-MATERIAL | Explicit Vibe factors → canonical records → actual native pixels; invalid output cannot promote | Texture/physical/HDR authoring, calibrated lighting and human material/model acceptance |
| RUN-08-RENDER | Reproduced metallic/AO faults; all affected consumers updated; real GPU and geometry oracles | Complete rendering/target qualification; unresolved broad-core failures remain explicit |

Preserve the operator-facing Poet route: configured local diagnostic Save and
fresh reopen must be usable without writing an embedding. Keep it visibly
diagnostic; no human/project authority is inferred. Production UI and MCP must
share the canonical native owner and execution session, with separate current
permission checks. No Computer Use is requested for this continuation.

## Handoff and completion rule

Keep ownership of a running verification command until its exit/result is
captured, or verify that the supervisor can actually resume it before handing
it off. Tool sessions can be worker-scoped; an untested session ID is not a
working handoff. Model-capacity failures must preserve sources, logs and command
ownership; recover the existing process before considering another invocation.
Never start duplicate costly builds because a response or model was unavailable.

Each worker reports changed files, source/API assumptions, actual commands/results,
expected/observed behavior, unresolved cases and remaining IDs. The supervisor
checks code, dependencies and integration before reporting a bounded job complete.
All broad ECO, model and language-release claims remain open until their full
acceptance is evidenced. A staged upstream patch is explicitly **not applied**;
a source fix without a successful target run is explicitly **not qualified**.

Implementation results are recorded in
`docs/qa/implementation-2026-10-24/README.md` as they become available.

## Native enrollment and cooperative Poet continuation — RUN-10 (P0)

The [batch-5 record](../qa/implementation-2026-10-25/batch-5/README.md) owns saved source evidence and the final receipt refresh contract. RUN-10 adds a QualiaDB-owned native enrollment signer/request/decision lifecycle; Poet's explicit `--native-project` create/adopt/profile/login entry; shared session-backed source contribution/review controls; and actor-private drafts/current-publication guards. Ownership and source receipts are disjoint across the `enrollment/`, `native-entry/`, `workflow/`, and `review/` subrecords. The repository-local inventory preserves pre-edit Poet entry/UI/job snapshots and identifies the source APIs and boundaries.

The first Poet native check exposed that the selected encrypted-key feature was missing from its dependency lock; an offline lock reconciliation added four already-resolved packages while preserving the existing 850 lock packages and their sources/checksums. A follow-up `--locked` attempt correctly failed on that not-yet-reconciled lock. The saved enrollment APIs/tests now gate encrypted file support on explicit `sanctuary-crypto`, and both the selected native profile and QualiaDB core qualification enable that feature. Core test construction disables OS credential API use. These setup attempts are not final check results. Consult the final source-bound receipts before reporting verification.

The current source snapshot pin is `c09fdeeb0297b2b19ef09e0d30b70fa673e9826e62a412e96f8d2a59a0d592ee` over 166 inputs; any later final command must identify its own exact current pin and report no source drift. Required final evidence is QualiaDB core tests, Poet native tests, native library build and WASM check. GPU and direct `--native-project DIR --check-project` CLI proof are optional; CLI evidence is a public-config-only check and cannot read project source/HMC, access OS credentials, perform adoption or launch UI. Refresh saved planning records only after the supervisor announces completed command results. The refresh script enforces exact zero exit, `source_unchanged: true` and current snapshot pin for every receipt counted as passed; missing/failed/stale receipts cannot be reported as pass.

A key possession proof authenticates control of the key currently bound by signed project admission; it does not verify personhood or legal identity. Project adoption, qualified identifier association, membership, current role/review grant and operation authorization remain separate decisions. Tests using software agents or fixtures do not establish a human enrollment or human review. The graphical path and production operator experience remain unqualified until actual accepted contributor/reviewer work is demonstrated.

All 120 packets remain open. IP-03 C1–C8, complete VibeScript 0.2 release, accepted high-grade editable masters, complete model dossiers and required film/open-world-game/cartoon embodiments remain open; no payment, publication, human acceptance or mission-completion claim is supported by RUN-10 source evidence.
