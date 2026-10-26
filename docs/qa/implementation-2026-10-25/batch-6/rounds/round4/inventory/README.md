# Round 4 inventory — native Poet producer and package review

Captured 2026-10-25. Bounded read-only inventory; no source changes, builds, tests, GUI/WASM execution, credentials, or asset production. The current baseline pins 44 Round 3 installed targets; all 44 match their latest Round 3 receipts, including the two test-only fixture overrides. See `source-baseline.json`. This is a freshness check, not a new qualification pass.

## Governing status

Round 3 records 63 distinct historical checks (56 SDK + 6 Poet inspection + 1 lifecycle), not new Round 4 passes. Native Poet binary build existed, GUI review was not run; WASM results were compile-only. All FND-01–06 remain open and assets remain ON HOLD. Round 3 explicitly says typed M64 inspection is read-only and grants no package/resource review acceptance. Round 2 `preserved_artifacts` contains four evidence/fixture files (one log and three conformance-tool files), not asset-master pins. Round 3 final verification counts three preserved regression artifacts in its own reconciliation scope. These are different scopes; no mismatch or corruption is implied. The exact Round 2 paths/hashes are in `asset-pins-round2.json`. Asset production remains prohibited by current scope.

## Relevant public seams and owning layers

| Concern | Existing API/owner | What it establishes | Missing for requested outcome |
|---|---|---|---|
| Source operation authorization | Poet `cooperation::{AuthorityOwner, Scope, with_authorized}`; `AuthoringAction::{ReadSource, ExecuteSource, MaterialEdit, Undo, Redo, Rebuild, SaveCheckpoint, ReopenCheckpoint, ContributionRecord, ReviewDecision}` from `qualia-cooperative-core::authoring` | Current owner transaction, source digest/revision binding, action permit; callback is single-use. `UnboundAuthorityOwner` denies. | Not itself a native grant-administration route. Source review decisions need distinct scope and durable evidence. |
| Native admission owner | Poet `cooperation::NativeAuthorityOwner::{new, with_current, with_current_package}` over QualiaDB `NativeAuthority` and opaque `SessionHandle` | Session is supplied by actual host proof/project admission; source transaction compares admitted/current source and requests reconciliation on drift. Package writes use QualiaDB `transact_with_package`. | No CLI/MCP/request can construct this authority, as intended. Do not bypass it for grant installation. |
| Package write audit | QualiaDB `PackageWriteIntent`, `PackageWriteEvidence`, `NativeAuthority::package_write_evidence`; Poet `AuthorityOwner::with_current_package` | Durable actor/source/package digest evidence for package writes. The core API documentation explicitly says retained audit is not package review authority and not acceptance of the complete HMC. | No package-review decision/grant type or accept/reject evidence surfaced here. |
| Resource/package mutation | Poet `CheckpointService::{preflight_resource_edit, inventory, save, save_with_resource_edits}`; `PackageWritePolicy::{WorkingCopy, ImmutableRelease}`; `PackageInventory` | Expected digest checks, bounded host-owned `.hmc` path, retained resource carrier; source and resource edits installed as one HMC rename. | This is persistence/editing, not an independent review workflow or model/package acceptance. |
| HMC reopen | Poet `CheckpointService::{prepare_reopen, activate}` and `checkpoints::authorised_reopen::reopen`; native owner guard required | Prepare is read/validate only; activation rechecks bytes, source binding, release policy. Saved source does not restore grants or replay effects. | Does not establish acceptance or grant management. |
| Typed inspection | `ResourceInspectionRequest`, `CheckpointService::inspect_resource`, `ResourceInspection::matches_inventory`; native Poet package controls | Read-only M64 metadata inspection bound to package/resource/source context. | No mutation/review acceptance. |

### Review distinctions

- **Source review** applies to editable source state: who authored which identified source/revision/digest and which reviewer made a decision under `ReviewDecision` authority. Existing source transaction and review-record APIs operate in this domain.
- **Package review** applies to a saved HMC package/release snapshot: exact package digest, complete resource inventory and typed payload evidence, provenance/lineage, and a separate reviewer decision. Existing `PackageWriteEvidence` proves a write audit only; the upstream API comment explicitly disclaims package review authority and complete-HMC acceptance. Resource inspection is read-only metadata, also not a decision.
- **Model/package acceptance** is a higher-level production claim: the dossier/claim ledger, all applicable catalogue and subject-specific obligations, target statuses and human acceptance. Neither source review nor package review alone can close it.

## Grant-management route finding

Correction after examining the native entry/action route: Poet already exposes `LaunchAction::GrantReview { subject, evidence }` and `LaunchAction::Revoke { subject, evidence }` in `native_entry::types`; `native_entry::actions::grant_review` obtains `owner.enrollment_permission` and `authority_signer`, verifies the subject is admitted and unrevoked, constructs a `ReviewAdmission` scoped to the configured document, and calls `owner.grant_enrollment_review`. Its validity ends at the earlier reviewer or admission-snapshot expiry. The adjacent `Revoke` action calls `revoke_enrollment` (revokes enrollment, not the review grant specifically); UI text also labels this “Revoke this enrollment.” Thus an existing native scoped source-review grant path exists. The initial absence claim was based on a limited file set and is superseded. This route uses the established enrolled authority/provider flow; it does not require inventing a separate source-review admin feature.

The next distinct gap is a **canonical package-review workflow**, bound to an exact HMC/package digest and complete resource inventory, with independent reviewer authorization and durable package decision evidence. Reuse the existing scoped source-review grant route where applicable; do not treat source review as package review. The existing `Revoke` control currently revokes enrollment rather than only the review grant, so any request for grant-only revocation should be explicitly scoped as a follow-up to that native authority owner. Package review remains the separate assigned implementation stream.

## Scope guard and next action

Assets remain ON HOLD. Implement only the scoped native grant-management route needed for distinct reviewers, while separate workers handle source producer/editing and canonical package review. Root owns source application, checks and reporting. No FND gate is closed by this inventory.

