# Poet foundation: remaining work and reviewed dispatch

2026-10-26. Governing order remains [IP-01–IP-06](../../../planning/implementation-plan.md), with [FND-01–06](../../../planning/tech/format-sdk-foundation.md) and IP-03 C1–C8 open. Asset production stays **HOLD**. This is an equipment-authoring readiness forecast, not a promise that the entire ecosystem will be complete in the quoted time.

## What “usable” means

The first interactive tryout must let an operator navigate and select real equipment geometry, change an editable source operation, inspect the resulting geometry/materials, undo, and save/reopen it without losing work. The production threshold additionally requires the equipment-specific operations, complete source/resource/graph/rights continuity, distinct authorized contributor/reviewer sessions, conflict/recovery cases and human acceptance of C1–C8. Software actors and PNG readbacks alone do not meet that threshold.

Native authoring, admission, contribution/review, HMC editing and many geometry/material operators are already implemented and have selected software-test evidence from earlier rounds. They need integration and qualification, not wholesale replacement. The [Round 7 record](../../implementation-2026-10-25/batch-6/rounds/round7/progress.md) includes actual GPU atlas/paint/procedural readbacks; those synthetic images do not establish equipment quality. The [hygiene and component inventory](hygiene-report.md) separates existing operators from remaining operator/UI questions.

## Current assignments and acceptance

| Job | Assigned role and owned scope | Required handoff and supervisor review |
|---|---|---|
| R8-A: preserve authored work | gpt-6.1-sol; parameter preflight/generator and focused tests, staged in `tools/poet-foundation/round8/edit-safety` | Explicit source-owned baseline; reject regeneration after downstream changes before source submission. Verify repeated parameters, material/atlas/detail/structure refusal, unchanged history, undo/restart. Full modifier stack remains separate. Root reviews and executes tests. |
| R8-B: source-entry continuity | gpt-6.1-sol; Core verified source carrier and native Poet save/compile forwarding | Retain the exact original HMC where opaque source metadata exists, rather than attaching that metadata to rewritten JSON. Preserve current source/journal, exact resource union, limits/conflicts and digest-qualified archive integrity. Independent worker reviews before root installs and verifies. |
| R8-C: private draft preservation | Root; native completion handlers and DraftBuffer | Only successful current completions may replace unchanged local text. Revision and buffer association protect typing during pending work. Independent review plus tests and native compile. |
| R8-D: repository hygiene | gpt-6-luna; inventory/reference scan and guarded relocation proposal | Hash/path map, dependency scan, exact archive copy verification and navigation repairs. Historical receipts remain unchanged. Root reviews before any move; no automatic replay of archived scripts. |

Workers stage inside Ecopunk, with immutable before/after hashes. Root owns integration, applies reviewed changes to canonical sibling source with preimage checks, retains failed attempts, and runs serial relevant checks. Workers cannot award human acceptance. There is no commit, publication, payment or asset-production authority in these jobs.

## Next jobs in dependency order

| Order | Scoped job | Dependency and completion evidence |
|---|---|---|
| 1 | Close R8 correctness failures and inspect save/reopen/compile integration | Saved source hashes, successful selected regression commands, independent review dispositions. Do not leave known engine failures merely documented. |
| 2 | Interactive native viewport | Inspect and reuse Aether input/window/widget hooks and Etalon/Portal geometry rendering. Add orbit/pan/zoom, framing and part selection against actual triangles with stable part mappings; retain renderer/cache and source/session/epoch guards. Existing graph-node picking is not assumed to be mesh-part picking. Prove pointer intent → native camera/pick → visible result; a numeric camera form or one isolated image is insufficient. Audit supported native embedding before choosing an upstream change. |
| 3 | Source-linked manipulation and equipment operator closure | Depends on viewport and source safety. Map every included frame/channel/opening/pipe/cabinet component to its actual required operator; implement missing routes, not unrelated suite parity. Qualified source layers/modifiers must preserve or explicitly remap downstream UV/detail/material work across regeneration. Conservative R8 refusal is a safety stop, not this editing capability. |
| 4 | Cooperative project workbench and complete HMC continuity | Reuse existing auth/admission/review/MCP owners. Expose shared brief/tasks/roles, contribution/review revisions, conflicts and recovery through Poet, with current per-action authority. Prove graph/resource/identifier/rights/derivation retention, not just mesh or source JSON. Can proceed alongside viewport work with disjoint ownership. |
| 5 | First operator tryout, then repair | Requires a brief-relevant interactive edit and safe HMC recovery. Provide repeatable launch/setup instructions and an inspection checklist; retain actual operator results. Computer Use remains excluded; operator evidence must be obtained separately. |
| 6 | Production qualification for growing-area equipment | Complete required operations, representative memory/latency/precision workloads, declared native/WASM runtime interoperability and C1–C8 acceptance. Only then release the production hold for the approved reference scope. Build assets and reviewed game-demo derivatives in the governing later phases. |

Use gpt-6-luna for bounded inventory/link/receipt checks; gpt-6.1-sol for Rust, rendering, runtime integration and independent review. A genuinely unresolved architecture question may justify a separately bounded stronger-model job. Each dispatch retains file ownership, allowed commands, dependencies, acceptance and a review handoff; no unbounded “finish everything” worker.

## Provisional estimate

From the start of this dispatch, allow **3–5 further reviewed rounds / 12–25 active engineering hours** for the first interactive equipment-authoring tryout. Allow **8–12 rounds / 30–60 active engineering hours** for production-ready growing-area-equipment tooling, **plus operator/reviewer acceptance time**. A round is one bounded implementation/review/integration cycle, not one agent or one message. The estimates include the current round; they are not a calendar promise or measured throughput commitment.

Assumptions: reuse the implemented stack, stable equipment brief, no major Aether/Etalon redesign, disjoint concurrent work and available reviewers. Discovery of a required upstream redesign or new production operators changes the estimate and must be reported with evidence. Broader flora/fauna, rigs, all three style/delivery embodiments, full ecosystem completion and asset production are **not covered** by these equipment-tooling estimates and remain unestimated pending their gap assessment. No paid-provider cost is estimated or authorized.
