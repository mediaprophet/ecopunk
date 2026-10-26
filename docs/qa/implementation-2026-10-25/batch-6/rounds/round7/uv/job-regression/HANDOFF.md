# Selected-surface worker regression stage

**Staged, not applied or executed.** [manifest.json](manifest.json) guards the
exact existing `native/src/graphical/jobs_tests.rs` preimage and the new focused
`jobs_surface_tests.rs` target. Rust 2021 rustfmt passed; no Cargo ran.

The existing ProbeHost now supports the new `EditorHost::inspect_surface` method
and injects post-admission source replacement, session revision or epoch identity
changes. A bounded synthetic data URI, summary and source-edge list are delivery
witnesses, not a rendered image/atlas claim. An optional successor snapshot under
a mutex lets the actual worker observe session/epoch changes without barriers,
timing races or unbounded waiting. Existing fixtures default to no injection.
The one existing literal Reply initializer now includes `surface:None`.

The new focused child test module defines two cases:

1. `surface_inspection_late_source_session_and_epoch_payloads_are_withheld`
   executes the actual new Action through Worker for three deterministic changes
   after admission. The read succeeds once but final `Reply.surface` must be None;
   a source-incarnation change also prevents publication of the completion.
2. `current_surface_inspection_publishes_once_and_old_completion_cannot_follow_rebuild`
   provides a positive current-context control, validates the delivered source,
   part and edge witness, then rejects delivery into a switched session and after
   the supported rebuild replaces the execution epoch.

The tests reuse the existing bounded receive helper and retained test-store
fixtures. No renderer, production asset, HMC, project credentials or live window
is used. Tests above 500 lines remain harness code in the existing 528-line file;
new cases live in their own focused child module. The supervisor owns
actual compilation/execution against the staged root trait, Reply and job guards.
