# Independent source review of Round 4 package authority

Historical first review. The subsequent repairs and current disposition are in
[the frozen-candidate recheck](independent-review-recheck.md). Original findings
below are retained as review history, not current unresolved blockers.

Reviewer: producer worker, 2026-10-25. This is read-only source review of the
separate `round4/review` candidate. No candidate, sibling, asset, build, test,
GUI, credential or live project write was performed. The source hashes in the
companion JSON pin the candidate observed at completion; candidate fixes made
by its owner while this review was running are not earlier execution evidence.

**Disposition: one blocking contributor-continuity finding; integration lock
lifetime remains a required root check.** This does not establish human/package
acceptance, compiled correctness or a passed FND/IP-03 gate.

## P1 — Known contributors disappear from exclusions after retained derivation

Affected native source: `package_review/mod.rs:58–73` and
`enrollment/package_review_admin.rs:31–38`. `PackageRevision` equality includes
package identifier, digest, predecessor, producer, source revision and locator.
`with_package` collects package contributors only where `r.target == *target`
and source contributors only where `r.target == target.source`. The steward
grant path repeats those exact-target checks. `validate_ledger` checks only
exact-target package contributions when checking decision attribution.

Consequently, a known native contributor becomes eligible after another actor
saves a successor that retains their work. For example, A produces and records
package P; B changes one raster sample, saving P' with predecessor P and all
other resources retained; the source author is C. A is neither P' last producer
nor current source author, and A's contribution to P fails the exact-target
filter. A can receive and exercise an exact P' review grant despite retained
known contribution. The analogous source case is A contributing to source v1,
B becoming the author of v2, and C packaging v2: A's known source contribution
is excluded from the list because its revision differs. The current tests
provide the known-contributor slice directly to pure policy, but do not prove
that the native owner preserves this list across either successor.

This is a continuity gap, not a digest collision or a claim that matching
identifiers establish equivalence. The native save predecessor/source succession
is explicit evidence that must be reconciled. The statement that other unknown
authorship/rights remain unassessed cannot turn already recorded contributors
into unknown contributors.

Remedy: derive one bounded conservative contributor exclusion set from the
canonical native source/package lineage, and reuse it for grants, decisions
and retained-record validation. Traverse only qualified explicit predecessor
links with bounded cycle/missing-link handling; do not infer relationships from
matching strings. Include relevant recorded prior source contributors and
native saved producers, or refuse review as unresolved when retained origins
cannot be qualified. Add a three-actor public-native test covering retained
predecessor resources and a source-successor case, with grant and decision
refusals and unchanged journal on denial.

## Required root integration check — retain the raw owner guard through commit

The native workflow holds its authority mutex through validation, the
`verify_current` callback and journal append. This correctly prevents native
authority/source-document metadata changes during admission. However, the
callback returns `()` before the append; the native API cannot itself keep a
closure-local raw checkpoint/editor guard alive. The Poet adapter must retain
the selected raw SharedEditorHost/source/checkpoint guard until the native
method returns, while following the documented authority-to-raw-owner lock
order. A guard dropped at callback return allows another shared UI/MCP operation
between file/source verification and record commit. Root owns this glue and
must inspect its actual guard lifetime and prove a bounded contention/staleness
case. A captured outer guard slot can satisfy the existing callback API; this
review does not assert that an uninspected adapter already violates the rule.

## Source-backed strengths and limits

- Pure package policy never consults source `review_grants`. Decisions require
  current canonical ReadSource admission plus a distinct exact target/actor/
  project/window package grant. Observer contribution/request is denied;
  observer review is possible only through an explicit package grant, preserving
  role versus authority distinctions. Principal comes from the current host,
  not from record/request text.
- `saved_matches` binds filename, result SHA/bytes, expected predecessor,
  exact source and actual native actor. `saved_package_target` takes the producer
  from the latest saved audit, so it need not equal the source author. Declared
  package identifiers remain project-scoped declarations; no universal identifier
  or referent mapping is inferred.
- Steward grant/revocation requires an opaque current EnrollmentPermission and
  the pinned NativeSigner, with a separately signed admission successor. The
  latest observed `commit_enrollment` preflights successor policy, signature
  digest and generation before append. Success invalidates pending/session/grant
  state and changes generation. Unsafe grant removal/rebinding/resurrection is
  refused. Revocation does not silently revoke independent source grants.
- New signed admission and journal fields use default-empty plus skip-empty
  serialization. The staged literal pre-extension admission test checks exact
  bytes/digest/signature compatibility; it was not executed here. The journal
  reads historical bytes and hashes their actual representation without rewriting
  old entries.
- New record vectors and decision-ID lists are capped before append. Journal
  encoding uses the existing bounded writer before creating a file. In-memory
  durable state is admitted only after append/sync succeeds; append failure
  latches reconciliation. Reopen creates no live session. Existing local
  anti-rollback and Windows directory power-loss limitations are explicitly
  retained; no stronger durability is inferred.
- Native staged tests explicitly identify their synthetic saved-receipt and
  no-op current-file callbacks as fixtures, and explicitly authenticate the
  selected test actor before actions. Their source-review isolation test uses
  source grants without package grants and expects refusal. They do not prove
  actual HMC file verification or guard lifetime; those belong to the separate
  real Poet integration test. No new test reviewed here silently grants review
  solely from the source-reviewer role or a client-supplied actor name.

All statements above are source observations. The contributor-continuity finding
requires owner repair/re-review and a real executed negative fixture before
qualification. FND-01–06, FND-05 native cooperation, IP-03 C1–C8, whole-HMC
rights/content admission, accepted masters and complete film/open-world-game/
cartoon packages remain open. Asset production stays on hold.
