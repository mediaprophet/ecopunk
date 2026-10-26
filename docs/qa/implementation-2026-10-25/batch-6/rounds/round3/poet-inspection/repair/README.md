# Test-only epoch visibility repair

Root's actual `--lib` compilation failed with E0624 because the newly authored
inspector delivery test called `SessionEpoch::retire()`, which is private to the
controller/execution parent boundary. This was a test defect. The earlier `--bin`
filtered invocation executed zero matching cases and is not a passing inspector
test result. The original failed `poet-tests.log` and zero-case attempt remain
root-owned execution history; this repair does not alter those records.

Only `native/src/graphical/jobs_tests.rs` is staged. Full current before/candidate
source and guarded hashes are in [target-manifest.json](target-manifest.json).
Current before SHA-256 is
`b4fa7f0b7957e54ef8fa50f5491e34455bb0bba211d31cdff5e65171c4628fa7`;
candidate SHA-256 is
`b545bbd6ee47210e534c2503889d32c63ec404e0ab2e60be1ea0dfc36aacfe51`.

The repair invokes the existing supported `Action::Rebuild` through the same
native test worker. Its existing `ProbeHost::rebuild_execution` replaces the
epoch; the test asserts this successor differs and confirms the old resource
inspection reply can no longer install. No production API/privacy, Core source,
authority grant or inspector implementation changes are made.

This preserves the inspector's epoch-successor completion-invalidation case.
It does **not** pretend the probe retires the original lifetime flag: the probe
only replaces its epoch. Exact production retirement is already exercised by
the same file's existing
`graphical::jobs::tests::actual_source_successor_and_retired_rebuild_epoch_publish_into_the_requesting_view`,
which uses actual `CanonicalEditorHost` undo/rebuild and checks the retired
epoch. Root should include that test if claiming retirement execution evidence.
The repaired new case is
`graphical::jobs::tests::resource_inspection_late_source_and_session_completions_are_withheld`.

No Cargo, builds, tests, GUI, credentials, assets, live writes or Core changes
were performed by this worker. This repair is staged and unqualified until
root reviews, applies and records actual matching `--lib` case results.
