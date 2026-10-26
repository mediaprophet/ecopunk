# Round 4 native package-review stage

**Staged only, 2026-10-25.** No sibling source was changed and no build or test
was run. Assets remain on hold. `rustfmt --edition 2021 --config
skip_children=true` parsed/formatted staged Rust; the read-only finalizer checked
all original source guards, new-target absence and new-file line bounds.
The [target manifest](target-manifest.json) pins full before/after sources and
the [patch](review.patch). [Case inventory](case-inventory.json) lists 19 public
integration cases and three private-only journal cases, all **not run**.

## Canonical ownership and exact binding

`qualia-cooperative-core::authoring::package_review` owns the package scope,
authorization and contribution/request/decision records. Native authority owns
signed admission, current native sessions, revocation and journal commit.
`PackageWriteEvidence` remains write provenance and conveys no review authority.
Existing source-review grants are never consulted as package grants.

`PackageRevision` binds project-scoped declared release `package_id`, filename
locator, exact digest and bytes, producer subject, exact `DocumentRevision` and
`predecessor_digest`. That predecessor is the previous **destination** digest
from the overwrite guard. It does not prove Save As ancestry or complete resource
derivation. Package ID, source/referent identifiers, filename and digest remain
distinct. Matching any of them grants no universal referent authority.

Source author and package producer stay separate. A bounded iterative history
walk follows all retained prior-destination digest edges within the same
project/document/source scope. It excludes every known matching write actor,
source author and recorded source/package contributor, independently of package
ID or filename aliases. Conservatively, all recorded source contributors in the
same qualified project/document/source scope are excluded across revisions,
including source work before any HMC save. Finer source ancestry remains
unresolved; this restriction does not infer ancestry from names or digests.
New signed/steward grants and decision exercise reject
these contributors. Missing, ambiguous or cyclic edges set
`known_destination_lineage_complete=false`; this flag concerns only retained
destination edges. Request/decision records retain the flag and explicit unknown
broader authorship/rights/completeness scope. This workflow has no full-model or
human-acceptance operation.

`AdmissionSnapshot.package_reviews` defaults empty and skips empty serialization.
The literal legacy-body/signature integration case verifies pre-extension bytes
through the public open API. Nonempty grants intentionally enter the signed body.
Grant identifiers cannot be removed, rebound or resurrected by replacement
admissions; revocation is monotonic. There is no automatic grant or role upgrade.

## Native integration

All package types are re-exported from
`qualia_core_db::poet_host::native_authority::package_review`.

1. `saved_package_target(handle, filename, package_id)` returns metadata for the
   latest native save, bound to current source. It does not read the HMC file.
2. `package_workflow_evidence(handle, target, verify_current)` returns current
   actor/generation, explicit can-contribute/can-review flags, exact-target records,
   destination-history completeness and unassessed rights scope.
   Separate `prior_contributions` retains other recorded package credits in the
   same qualified project/document/source scope with their original full target;
   ancestry/applicability remains unqualified. Prior decisions are never shown
   as current and no payment or obligation discharge is inferred.
3. `record_package_contribution(handle, expected_generation, target, id,
   description, minutes, work_status, verify_current)` records the actual producer.
4. `request_package_review(handle, expected_generation, target, contribution_id,
   id, evidence, verify_current)` requires that producer's exact contribution.
5. `record_requested_package_review(handle, expected_generation, target,
   request_id, id, decision, rationale, evidence, verify_current)` requires an
   exact current request and independent explicitly granted reviewer.

Callbacks have type `FnOnce(&PackageRevision) -> Result<(), String>`. Lock order
is **native authority → raw shared editor/checkpoint owner → direct bounded file
verification**. The callback must never call `AuthorizedEditorHost` or native
authority recursively. The root adapter retains the acquired raw owner guard
until the authority operation returns, including durable append. Its direct
checkpoint guard checks actual source, package digest and byte length. The result
describes a moment-in-time snapshot; an uncooperative external writer can change
bytes later. Recheck source/session/package freshness on publication and consume.
The nonblocking `package_publication_is_current(handle, generation, target)`
checks one current native lock, session/generation and latest exact native save
audit without filesystem IO. Poet binds it into an opaque publication stamp.
Same-source native/MCP package successors invalidate old package views; external
disk changes still require bounded checkpoint verification on refresh/consume.

Explicit grant administration is
`grant_package_review(&EnrollmentPermission, PackageReviewGrant, &NativeSigner)`
and `revoke_package_review(&EnrollmentPermission, grant_id,
decision_evidence_ref, &NativeSigner)`, each returning `SignedAdmission`.
The permission must come from the existing current admitted-steward route and
the signer must match the adopted project authority key. The existing native
entry credential provider supplies that signer; source-review/enrollment actions
never manufacture package authority. Both operations invalidate pending/live
sessions after signed admission and local audit commit. No credential provider
was accessed while staging; encrypted-file tests are software fixtures.

Journal count checks and bounded JSON encoding precede file creation. In-memory
state is admitted only after append. Admission digest/generation validation is
preflighted before durable updates. Partial append latches reconciliation. The
local journal is not an external anti-rollback oracle; Windows directory survival
across power loss remains unqualified. Count/byte exhaustion preserves prior
work and requires explicit retention/migration work, never silent pruning.

## Verification and remaining acceptance

Public targets are `qualia-cooperative-core/tests/package_review.rs` and
`qualia-core-db/tests/native_package_review.rs` with its fixture/admin/history
children. The native integration target uses only public APIs. It can be included
by the existing native conformance runner with `serde_json`, `sha2`, `tempfile`
and the selected `sanctuary-crypto` capability. No dependency changes are staged.
Save callbacks in these Core tests are explicitly signed audit fixtures; actual
HMC bytes/owner locking must be exercised by root Poet integration tests.

The root owns Poet review adapter, UI/action forwarding and explicit native
steward grant/revoke entry. Add `package_reviews: Vec::new()` to Poet snapshot
constructors (create, native_project tests, poet_workspace tests and growing
reference example). Do not rerun `prepare.py` after final history refinements:
the full reviewed after-images are authoritative; `finalize.py` is read-only
against sibling sources and safely refreshes hashes only before installation.

All actual build/test results remain pending the root's single frozen-source
qualification. Adopted human authority, contributor/reviewer operator work,
native GUI acceptance, complete HMC semantic/rights/content review and complete
model acceptance remain open. Film, open-world-game and cartoon embodiments
remain incomplete; FND-01–06, IP-03 C1–C8 and catalogue/subject claims are not
passed by this bounded implementation. No payment, publication or mission
completion claim is supported.
