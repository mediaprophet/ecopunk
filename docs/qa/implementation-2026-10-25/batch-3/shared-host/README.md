# One native editor/session shared by UI and MCP

Staged adapter only; no sibling edits. Parent owns module routing and guarded
application. `SharedEditorHost::new(Box<dyn EditorHost>) -> Self` captures one
raw canonical editor, optionally decorated by the native checkpoint editor.
Cloning it shares that exact boxed object through Arc/Mutex. It creates no new
SourceSession, source store, journal, project database or cached authority.

Mount in this order:

```text
CanonicalEditorHost -> optional CheckpointEditorHost -> SharedEditorHost
    clone -> AuthorizedEditorHost(UI owner/scope)  -> native UI
    clone -> AuthorizedEditorHost(MCP owner/scope) -> NativeAuthoringBackend
```

Never put AuthorizedEditorHost inside SharedEditorHost: nested owner-lock
acquisition could deadlock or confuse caller authority. This is a trusted Rust
composition contract, not a runtime type inspection or authentication provider.
Canonical snapshots have no authorisation stamp; the outer wrapper admits and
stamps each frontend response using its current native owner.

All source/session/check/material/undo/redo/rebuild/checkpoint operations try_lock
the same object exactly once. Busy denies without blocking, dispatch or retry.
Poisoning requires explicit native reconciliation; it never silently recovers the
inner object. Checkpoint service availability is captured once at composition,
independently of busy state or user authority, so tool catalog discovery is stable.

`shared_host_tests.rs` contains actual canonical store/session and headless native
geometry/material cases: one clone submits the diagnostic source, another edits
the literal material, both observe session/source advancement, stale edits fail,
undo holds execution and explicit rebuild through the other clone restores it;
redo/rebuild also propagates. Separate cases verify busy denial/no source or
session mutation and poison handling. These tests produce no GPU frame, GUI result,
production principal or acceptance claim.

Tests are staged for the parent's coherent native checks and were **not executed
by this worker**. No Cargo/core build was launched. rustfmt completed. The guard
map pins these two new files and requires target absence. Product/master/model
completion and IP-03 C1–C8 remain open.
