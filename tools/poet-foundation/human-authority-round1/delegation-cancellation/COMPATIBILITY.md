# Delegation cancellation: compatibility and minimum consumer

Status: staged only, not compiled, executed, installed or accepted. The journal schema currently uses serde deny_unknown_fields; older executables cannot read a record carrying delegation_cancellations. The new reader accepts old records through serde default. No delegation is adopted by this change.

## Safe migration and rollback

1. Build and verify an upgraded reader and its compatibility tests against representative copies of the actual retained journal. Include empty old history, newly cancelled edge, ancestor cancellation, denial, restart, append failure, and journal bounds. Do not mutate the user journal for qualification.
2. Before enabling writes, record executable identity and backup a complete, closed, consistent authority journal as a recovery snapshot. Preserve the exclusive owner and source/admission binding. The first actual cancellation is the format transition; require the upgraded reader on future opens.
3. The current staged change has no enforced version gate preventing a user from invoking an older executable. Implement and qualify a host-owned minimum-reader/version mechanism and operator-facing error before enabling this format in ordinary use; do not rely on an agent note as enforcement. That mechanism must preserve existing journal replay and not permit older binaries to silently ignore revocations.
4. After a cancellation, simply restoring the pre-transition backup or opening with an old implementation would discard or fail to enforce later cancellations. It is not a safe operational rollback. The safe response is to retain the full updated journal, stop delegated execution, and use a repaired compatible reader or an explicitly designed lossless migration that retains all records and cancellation effects. Any emergency restore must make the resulting loss of later state explicit and be separately authorized; no automatic rollback or overwrite.
5. An alternative storage/version design is acceptable only after tracing the existing owner and proving it keeps old data, source/admission relationships and revocations intact; do not invent a parallel policy store merely to avoid a schema migration.

## Minimum actual consumer

The already existing NativeAuthority::inspect_controller_delegation consumes cancellation state after the staged change. It is currently only a candidate-inspection function, not a delegated session or permit; source search found selected tests but no mounted Poet caller. A useful next boundary needs an authenticated root control to inspect and explicitly cancel a verified signed chain through the real native owner, then show exact target and cancellation outcome. It must not interpret candidate inspection as adoption or permission. Before delegated edits or review, the owner must persist adopted chain, authenticate the leaf, recheck the full ancestor state on actual operation/commit/publication, preserve authorship and enforce applicable budgets and review-independence policy.

## Remaining risks

The staged cancellation API accepts currently valid signatures and admissions only; expiry or prior admission changes complicate recovery and need an explicit UX. Journal records are bounded and monotonic in the new reader, but do not claim tamper-proof storage. UI, authenticated operator behavior, old binary refusal, migration handling and all tests remain unexecuted. The signed-chain, principal and human labels do not prove personhood or human/domain acceptance.
