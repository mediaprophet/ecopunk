# Delegation documentation review

**Date:** 2026-10-26 · **Scope:** documentation only; no jobs dispatched, source changed, Cargo/build/test run, or implementation acceptance claimed.

## Evidence reviewed

- `AGENTS.md`, `docs/planning/implementation-plan.md`, `implementation-dispatch.md`, `implementation-agent-roles.json`, `implementation-queue.json`.
- `biosphere-implementation-dispatch.md`, `biosphere-work-packets.json`, `docs/qa/implementation-2026-10-26/biosphere-integration-review/README.md`, `docs/qa/implementation-2026-10-26/round9/progress.md` and `saved-evidence-verification.json`.
- `docs/planning/tech/high-grade-production-and-civics-experience.md`.

The plans correctly keep all 120 ECO packets and the governing phase order, route bounded work to lower-cost or stronger roles, preserve exact handoffs, and explicitly separate source, execution, domain/human acceptance. Biosphere work also has per-job staging/evidence paths, preimage hashes, status fields, a single shared-mutation/check coordinator, and a clear caveat that its lease helper only coordinates cooperating callers. Round-nine evidence is scoped and candid about unreviewed source and unqualified human workflow.

## Concrete gaps to fix

1. **One current status view:** implementation queue entries remain `queued` while biosphere entries are `prepared_not_started`; the main dispatch describes completed prior RUN work. A new-chat worker cannot tell which instruction reflects current authority. Add an as-of date, authoritative status source, and explicit mapping among proposed, ready, active, staged, applied, verified, blocked, superseded and closed. “Verified” must name what was verified.
2. **Ready now vs prepared:** biosphere's first wave is described as able to proceed, but every item is `prepared_not_started`; BS-10/20 depend on BS-01, whose semantic contract is supervisor-owned. Label read-only research/contract jobs that can start now separately from implementation jobs prepared but not activated. Give each a concrete gate that changes its readiness.
3. **Exact ownership:** broad packet acceptance and functional objectives do not identify canonical source files, symbols, or editable ranges. Staging paths are precise, canonical write ownership often is not. Add exact owned paths (or explicitly “research only; no canonical writes”), shared-file owner, and disjointness check. Require a narrow ownership amendment before scope expands.
4. **Commands and source authority:** “no Cargo” is useful but does not say which read-only tools, repositories/roots, generated artifacts, or network sources are allowed. Pin source revision/hash, approved root(s), immutable inputs, allowed and prohibited commands, and whether external repositories are read-only. Do not treat a sibling repository as writable authority.
5. **Acceptance levels:** acceptance prose mixes fixture/schema review, source edits, compilation, runtime behavior and human review. Give separate fields for source present, applied, executed (command/exit/artifact), rendered or runtime-observed, independent technical review, domain review, and human acceptance; each takes `not_applicable` only with rationale, otherwise an explicit pending/blocked/pass/fail state. Agent output cannot self-accept.
6. **Blockers and readiness:** add `blocked_by`, evidence, owner, required decision/contract, next action, and re-entry condition. Distinguish unresolved research questions (which may be answered autonomously from pinned sources) from authorization, source-root, semantic-contract or human-review gates.
7. **Process ownership:** current biosphere fields request PID/exit state, while general handoff fields omit process/session ownership and resume proof. Require command, process tree/session owner, working directory, start time, log/output, running/exit/unknown state, resumability evidence, and named continuing owner. Unknown is not success; do not start a duplicate build to recover context.
8. **Lease wording:** preserve the existing caveat verbatim in substance: the helper serializes cooperating callers only and does not control arbitrary Cargo. Do not describe it as a security or universal lock gate. One coordinator may own Cargo/checks; this does not imply workers' source edits are safe just because the lease is held.
9. **Inspectable artifacts:** state the exact user-reviewable output path and format (patch, report, fixture, log, image/runtime capture), plus a concise index linking evidence to requirement IDs and source/output hashes. Do not make private worker context or a generated receipt the only review surface.
10. **Cost and model routing:** retain the role matrix, but attach role/model and reasoning effort to each activated bounded assignment; use lower-cost models for deterministic research/docs/metadata and stronger models for semantic, runtime, integration or independent review. Escalate only with a specific evidence-backed question. No price or spend claim is implied.
11. **Status synchronization:** one human-readable dispatch and its machine packets/queue must identify which is authoritative and update together (or link a dated status ledger). Preserve ECO IDs, acceptance clauses and all 120 backlog entries; do not create a competing project/packet taxonomy.

## Small reusable assignment template

Use one short record per bounded job, linked to existing ECO/engine/spec IDs:

```text
job_id / ECO+requirement IDs / as_of / status / status_source
objective / deliverable and user-inspectable path+format
read_first / immutable source roots+revision or hashes / source authority
owned canonical files+symbols OR research-only / shared owner / overlap check
dependencies / ready-now condition / blocked_by+owner+re-entry condition
role+model+effort / supervisor / independent and human/domain reviewers
allowed commands+roots / prohibited actions / check owner
acceptance by level: source, applied, execution, runtime/render, technical,
  domain, human (each state, evidence, and explicit remaining IDs)
process: command, session/PID owner, cwd, log, running/exit/unknown,
  resumability proof, continuing owner
handoff: changed paths+before/after digests, assumptions, failures, next action
```

## Autonomous work that can start now

These are bounded read-only research/documentation jobs, not new backlog packets; root can assign them under existing ECO/BS IDs without waiting for implementation:

- Reconcile current queue/dispatch/biosphere statuses and produce a dated authoritative status crosswalk, preserving all existing packet IDs.
- For BS-01/10/20/30/40/50/60, inspect only cited source paths/contracts and propose exact disjoint source ownership, unresolved API decisions, and allowed serial check candidates. Deliver paths/hashes and unknowns; make no source edit.
- Map each biosphere acceptance clause to source-presence, execution, runtime, independent/domain review and human-acceptance evidence; identify missing rows.
- Audit user-reviewable artifact locations and process-resume records in the named QA directories; report missing links without re-running commands.

Keep BS-01's contract decision with its named supervisor. Implementation jobs remain **prepared, not activated** until root assigns exact canonical file ownership, source pins, command/check ownership and the relevant dependency gate. No revised docs or this review activate a job. Root owns the revisions and any dispatch. High-grade master, three target-package and IP-03 readiness claims stay open under the existing contracts.

## Coordinator disposition after revision

The revised central handoff/register addresses the listed status, ownership,
input/command, evidence, blocker, artifact and process-continuity gaps. A second
read-only review found no dependency cycle or conflicting output ownership. It
raised current-pass authority wording, evidence status vocabulary and parent
roll-up; the coordinator clarified all three in the central documents. This is
document review, not activation, engine execution or human product acceptance.
