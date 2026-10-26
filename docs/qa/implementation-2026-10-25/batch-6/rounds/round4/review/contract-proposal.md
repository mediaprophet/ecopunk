# Digest-bound native package review — proposed staged contract

2026-10-25. Approved with refinements by the supervising agent; the
[staged implementation handoff](handoff.md) records the final contract and limits.
This proposal is retained as design history. Sibling repositories
remain read-only; assets, builds, tests, GUI, credentials and live project writes
are on hold. This work does not establish human review or complete HMC acceptance.

## Ownership and compatibility

Canonical policy and records belong in `qualia-cooperative-core::authoring`.
Native authentication, signed admission, revocation and persistence remain in
`qualia_core_db::poet_host::native_authority`. Poet projects those APIs through
the existing native session and source/package owner. No Poet-private ledger.

An exact package target contains its explicit qualified package identifier,
filename locator, SHA-256, byte length, predecessor SHA-256, exact
`DocumentRevision` and actual saved package producer subject. Package identifier,
referent identifier, source identifier, filename and digest remain distinct.
The producer subject comes from the native package-write audit; it is not
inferred from the source author's subject or a name supplied by a client.

`AdmissionSnapshot.package_reviews` is additive and uses both `serde(default)`
and `skip_serializing_if = "Vec::is_empty"`. Empty additions must preserve old
admission serialization and signatures. Nonempty grants deliberately change
the signed payload. Existing source-review grants grant no package review.
No enrollment request receives grants automatically.

## Current authorization and retained evidence

The canonical package policy reuses current source read/membership admission.
A contribution or request requires a non-observer member who actually produced
the current saved package. A decision requires a distinct explicit package grant
binding the full target, reviewer subject, project, validity and admission
evidence. Review by the producer or source author fails closed. Expired,
revoked, duplicate, cross-project, wrong-source/revision, wrong-lineage and
wrong-package/digest grants fail closed. Current sessions and authority
generation are rechecked for each commit.

Native methods verify the target against the latest saved audit for the filename
and current exact source. An owner callback under the native authority lock must
also verify the actual bounded HMC file bytes immediately before recording.
Journal knowledge cannot detect external file mutation by itself. The adapter
must maintain its selected-source/package owner lock through this callback and
commit; filesystem protection outside this lock retains its existing limits.

Separate canonical package contribution, request and decision records retain
exact target, authenticated actor capacity, qualified identifier association,
membership/admission references, authority generation and time. Requests refer
to an existing exact package contribution; decisions refer to a current request.
Work status remains independent of role and permission; effort is not payment.

The existing native journal gains bounded default-empty evidence vectors.
Validate counts before append and encode through its bounded writer before
creating an entry. Admit in-memory state only after durable append succeeds.
Partial append latches reconciliation. Reopening evidence creates no session.
Journal lineage is local evidence, not an external anti-rollback oracle; Windows
directory survival across power loss remains unqualified.

Grant administration uses current explicit steward `EnrollmentPermission` and
the pinned native signer. Grant/revocation updates are explicit signed admission
successors; session and pending proofs are invalidated after atomic admission.
No OS credential access or real human adoption is part of staged qualification.

## Required unexecuted qualification

Add focused canonical CPU cases for explicit grant isolation, exact targets,
self-review, duplicate/revoked/expired grants and record bounds. Add native cases
for signed compatibility, current file verification refusal, contribution to
request to decision lineage, source/package staleness, revocation, reopen,
duplicate IDs, count/byte budgets and failed atomic admission. Test actors are
software fixtures, never evidence of human participation.

Human acceptance still requires actual adopted project authority, enrolled
contributors/reviewers, usable native editing/save/reopen/review, relevant package
domain/rights/content checks and explicit human decisions. Package workflow
decisions do not pass model catalogue gates or accept film, open-world game or
cartoon packages. FND-01–06 and IP-03 C1–C8 remain open.
