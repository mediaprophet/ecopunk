# Staged quality repairs

These files are review candidates only. Their canonical QualiaDB and Poet sources were read but not modified, and no Cargo command was run. `source-candidate-hashes.json` pins each canonical source, its preserved copy, and its proposed candidate by SHA-256.

## Vibe deontic interrupt

The candidate retains the existing 32-sample strict `< 1000 ns` trigger assertion. Cold construction reserves bounded history capacity (64), and a custom `Clone` preserves that reservation. The hot trigger flips the interrupt latch and appends retained events without allocation; all lease queries honor the latch. At saturation, revocation still occurs and `interrupts_overflowed()` counts omitted events, saturating at `u64::MAX`; the omitted incoming event's owned strings are dropped on return. Resume re-enters the active phase. This changes formerly unbounded interrupt-history retention into an explicit bounded contract. The change does not claim staged-delta rollback; that belongs to the transaction owner. No timing result is claimed until the coordinator runs and reviews the qualified test.

## Vibe predicate crypto

`crypto.prove_zk_predicate` and `crypto.verify_zk_predicate` now return an explicit unsupported diagnostic because the selected implementation has no native ZK proof provider. New `crypto.commit_predicate` and `crypto.verify_commitment` APIs report deterministic local SHA-256 binding integrity only. They explicitly report predicate truth and issuer authenticity as unverified. A commitment is not hiding, a proof, an adopted agreement, or authority. The VQ05 case `"1 == 0"` demonstrates that commitment integrity cannot be mistaken for predicate truth. Exact legacy spellings remain registered to make migration/refusal visible.

The same candidate keeps original execution source separately from its canonical AST normalization. Lookup re-normalizes and re-encodes the source, then matches those bytes and their digest to the immutable internal artifact before returning source for execution. `code.spawn_at` accepts only `did:q42:local_scheduler`; unknown/remote scheduler names fail explicitly. A focused local cell-hash → spawn → await regression is staged. Its `effect async fn` uses the existing checked program path and a synthetic `LocalHost`, not native authority or human admission.

## Warning cleanup candidates

Candidates cover the warning inventory in `docs/qa/implementation-2026-10-27/post-update-qualification/poet-native-tests.log`: unused imports/variables/mutability, test-only source helpers, the obsolete 64 MiB snapshot wrapper, the explicitly unqualified unused preview helper, and the unused zero-grant REPL wrapper. `dense_device_layout` remains crate-private and is gated to the same native CUDA feature as its sole consumer. No warning suppression was added. The source log remains historical evidence; warning clearance must be established by the coordinator's serial qualification.

## Qualification status

Candidate source review only; none of these candidates has been compiled, tested, installed, or accepted. The coordinator owns any canonical merge and serial Cargo qualification. The Vibe stdlib test candidate updates selected effectful scenarios to explicit `effect async fn`, uses the existing low-level admitted test gateway for setup, and keeps a pure-cell refusal assertion.
