# Public native carrier fresh-process recovery

Staged, unrun, 2026-10-26. No sibling writes or Cargo/build/test command ran in
this worker. `manifest.json` pins the current saved public consumer file before
adding a two-line module route, plus its new 149-line companion. The primary
file stays below 500 lines. Both pass rustfmt check. The companion lives in
`native/tests/foundation_source_carrier/restart.rs`, avoiding Cargo treating a
top-level helper `.rs` as a separate integration test crate.

The parent uses the public native EditorHost to bind a metadata-bearing original
HMC, apply transform/material edits and compile `restart.hmc`. It validates the
current source/journal/parent and exact original archive, then dispatches its
current test executable with the exact child test and an ephemeral root/source
identifier. No user paths, credentials, GPU or principal are inferred.

The child asserts its Studio store is empty, calls public
`CheckpointService::prepare_reopen` and `activate` on the produced HMC, retains
that same service, opens a held canonical native host, confirms inspection is
unavailable before explicit rebuild, rebuilds, and checks actual part transform
and material plus exact accepted cells. Source activation/rebuild must not change
the accepted artifact. Native SaveAs must retain the exact original HMC archive
and current full source/journal with the selected parent digest, without wrapping
another origin. The parent checks the child's saved parent and original disk
bytes after successful exit.

Substantive parent selection:
`source_carrier_restart::metadata_carrier_native_output_survives_fresh_process_activation_and_save_as`.
It dispatches exact child
`source_carrier_restart::source_carrier_restart_child`; invoking that child without
the parent environment is an inert no-op and is not recovery evidence.

The supervisor owns independent review, guarded installation after the active
consumer command exits, and execution. Synthetic CPU recovery is distinct from
human authority, actual UI usability, graphics and complete model acceptance.
