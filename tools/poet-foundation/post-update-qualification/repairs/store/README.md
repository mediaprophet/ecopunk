# Staged source-publication repair

This is a review package only. It does not modify `C:/github/qualiaDB` or `C:/github/poet`.

## Source-publication stamps

`qualia-core-db` owns one canonical process-wide artifact map. Each `StoredArtifact` owns its atomic publication state in that same entry; there is no second source registry. `artifact_stamp_is_current` reads only that stamp's retired/busy/revision atomics and the map mutex poison bit, so unrelated artifact activity no longer causes false-stale results and the check never waits for the store mutex.

Same-artifact writes publish a busy state before changing source and publish the resulting revision before clearing busy. A normally rejected detached candidate restores its previous revision. A panicking mutation retires its token and leaves it busy through mutex poisoning, preventing the unwind gap from publishing potentially partial source. `StoreStamp` retains `Arc` identity, so replaced entries cannot revalidate through allocator reuse. `try_upsert_artifact` validates a candidate before taking the map lock and retires the previous entry only after admission succeeds; it returns an error on rejection, preserving the accepted artifact and token. The legacy infallible `upsert_artifact` keeps its previous unvalidated compatibility semantics and is documented accordingly. `import_into_store` now uses the fallible API, so it cannot report success after a failed admission. Legacy and checked mutate, undo, redo, and journal configuration all bracket their mutations. `StoredArtifact::drop` retires any remaining token when a canonical entry is actually removed or replaced.

The inspected store has no direct public removal/clear API. Tests cover replacement and a local `StoredArtifact` drop, without needing to mutate or clear the shared global store.

Added tests cover independent-artifact activity, same-entry busy denial, replacement retirement/ABA identity, poison fail-closed, failed fallible replacement preservation, legacy unvalidated upsert compatibility, panic unwind using isolated local stamp state, and dropping a stored artifact with a retained stamp. Existing stamped mutation tests cover stale stamps, concurrent competing commits, rollback, undo and redo.

## Preview failure disposition

No `NativeIntentBus` change remains in this candidate. The isolated native surface workflow passed unchanged, along with the isolated source-layer case. Current evidence points to the same shared-store nonblocking stamp contention: `jobs::run` can reject publication via `reply.current` while the authored outcome itself remains successful. The existing typed current-context / operation-success checks should remain intact; no warning-text promotion is warranted.

## Verification and use

`rustfmt --edition 2021 --config skip_children=true` succeeds on candidate Rust files. No Cargo command, build or test was run. Review source originals and candidates against `source-hashes.json`; tests must be run by the owning coordinator after the patch is explicitly accepted and installed in the canonical sibling worktree.
