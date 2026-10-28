# Native contributor/reviewer control candidate

2026-10-27. Full-file candidates only. No canonical Poet/QualiaDB writes, Cargo,
rustc, test execution, native windows, production assets, external providers,
commits or publication were performed by this worker. Production remains HOLD;
IP-03 C1–C8 and AUTH-H2–H6 remain acceptance obligations.

`candidate-manifest.json` binds each target, staged candidate, original SHA256
(null requires target absence) and candidate SHA256. Compare every original
before saving. Preserve intervening work rather than overwriting a mismatch.
`source-pins.json` identifies inspected source bytes; hashes do not establish
semantic correctness or execution. Read-only sibling Git commands were refused
by the filesystem boundary; no current commit or clean-tree claim is made.

## Concrete changes

- Source review uses exact-target unanswered requests and displays their actor
  and evidence. The action callback rechecks selection in current inspected
  evidence. Invalid/answered/stale selections cannot enable a pending decision.
- Package requests select this actor's recorded contribution to the exact saved
  package. Package decisions select an unanswered exact-package request; manual
  copying of arbitrary contribution/request IDs is removed. Required new record,
  rationale and evidence fields must be populated to enable package decisions.
- Package records retain request actor/evidence, decision request ID, actual actor
  capacity, review grant, review admission and decision evidence. Historical
  records are retained and are not projected as pending work.
- Contributor authority displays the signed-in subject and project-admitted
  capacity, a readable direct-admission label, optional human principal and
  delegation digests. Empty ancestry is explained as direct project admission
  with no attached delegated-agent chain. No personhood service is implied as a
  prerequisite for authoring. A brief note keeps source review and domain
  acceptance separate.
- Workbench decision wording no longer interprets `Human` plus `Accepted` as
  proof of a human source decision. Decision, admitted actor capacity, adopted
  human authority and actual human/domain acceptance remain separate.

The pending task selector intentionally offers unanswered requests only. Existing
backend APIs can record further decisions against an answered request; this
candidate changes no backend review policy and retains all decision history.

## Required integration

Coordinator owns the one `graphical/mod.rs` registration:

```rust
mod cooperation_selection;
```

No Cargo/dependency/feature change. Keep `cooperation_selection_tests.rs` beside
`cooperation_selection.rs`; the latter registers its tests with `cfg(test)` and
an explicit path. Both are below 500 lines.

The source evidence panel depends on the authority worker's agreed additive
`WorkflowEvidence.human_authority: HumanAuthorityEvidence`, reexported from
cooperative-core `authoring`, with `acting_subject`, `acting_kind`, `status`,
`human_principal_subject: Option<String>` and `delegation_path: Vec<[u8; 32]>`.
Legacy direct admission yields `UnverifiedLegacyAdmission`, no human root and
an empty path. The coordinator must also update the pre-existing
`cooperative_workbench/projection.rs` test fixture's `WorkflowEvidence` literal;
that shared fixture is outside this candidate's file ownership.

## Selected operation traces

Source: `ui.rs` renders `CooperativeControls` → existing `jobs::run`
`Action::Workflow` → `SessionEditorHost::workflow` →
`WorkflowService::run` → `NativeAuthority` source evidence/contribution/request/
decision operations → cooperative-core authoring permit and durable journal.
The service retains selected-session locking through source inspection, exact
target checks, commit and final evidence. Existing `Context`, publication-current
and completion-current checks continue withholding stale/changed-session output.

Package: `package_controls.rs` renders `PackageReviewControls` → existing
`Action::PackageWorkflow` → `SessionEditorHost::package_workflow` →
`PackageWorkflowService::run` → Core package workflow → cooperative-core package
permit/records → native journal. `verify_package_guard` binds actual current
source scope/revision/digest and checkpoint inventory digest. The service retains
selection → authority → raw-editor locks across verification/commit. Source
review grants do not imply package review grants; exact-package native authority
and contributor exclusions remain backend-owned.

`native_dispatch.rs` explicitly retains package/cooperation operations on the
qualified same-owner jobs adapter until tool payload migration. These candidates
reuse that path, rather than claiming new IntentBus command coverage. No private
transport, authority system, semantic ledger or app-contract replacement is added.

Native enrolment: `native_entry/ui.rs` → `LaunchAction` → owned cold
`LaunchWorker` → `Runtime` → existing admission and protected credential owners.
Enrollment, local profile, login, role/reviewer grants and revoke controls already
exist. `session_proof::authenticate` cancels only its own failed proof challenge.
This candidate does not establish that enrolment is adopted human-root evidence,
that a production proof provider exists, or that a human has exercised the UI.

## Test-reference and compatibility audit

Read the existing control call sites (`ui.rs`, `package_controls.rs`), module
registrations, all references to the changed component names, and `cfg(test)`
projection fixture. No public component signature or existing helper is removed.
Existing source controls, saved package controls, work status, prior-contribution
lineage, draft handling, package identity field and source-only scope remain.

Selected source tests inspected: Core
`requested_review_is_revision_bound_distinct_and_durable`; native
`session_wrapper_worker_publishes_exact_successor_and_withholds_after_reviewer_handoff`,
`cooperative_operator_service_binds_edit_save_request_review_revoke_and_reopen`,
`package_workflow_real_producer_separate_reviewer_reopen_and_same_source_successor`;
cooperative-core package policy/record tests for exact grants, lineage, revocation,
self-review and foreign producer request refusal. These are source observations,
not new executed results.

Three new source tests cover full source target equality, wrong source digest and
author, absent/answered selection, full package equality including bytes/project/
predecessor, preserved request/evidence/history, and own-actor exact-package
contribution selection. Coordinator should compile/execute the
`cooperation_selection` filter, existing workbench projection fixture and relevant
native workflow/package suites after guarded integration. No UI trial is inferred.

Staged rustfmt completed successfully. It parses Rust syntax, not Dioxus macro
expansion, type checking, authority enforcement or native operator usability.
Earlier `post-update-qualification/README.md` records 1,769 selected passing
executions and a native GPU/fresh-process diagnostic before these candidates;
that evidence is retained at its original scope and is not a pass for this patch.

Owning QualiaDB areas for this selected operation are native authority,
cooperative-core authoring/projects and the studio checkpoint/journal boundary.
No new gap is claimed in domains, modalities, solvers, specialized_libs,
wgsl_forge, qualia-vision, qualia-audio, webizen-render, semantic_instruments or
qualia-sim; those unrelated capabilities were not semantically audited here.
