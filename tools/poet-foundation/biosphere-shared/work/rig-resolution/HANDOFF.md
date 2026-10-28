# Rig workflow: staged candidate and integration boundary

Assigned by the coordinator for OP-VIEW-01 / BS-20 and BS-50. The staged files are a proposed safety component, not an applied patch or a completed native workflow. No asset, build, test, renderer, window, or human review was run by this worker. Poet was read only.

`rig_candidate.rs` reconstructs a disposable checked `BoundRig`, applies the existing control, and returns a non-forgeable staged descriptor and JSON literal. It keeps a retained preview unchanged until an authorized source owner accepts the candidate. `rig_source.rs` proposes an existing-only mutable `poet_rig` Vibe string binding: it reads the versioned JSON and produces a source cell only if the original binding equals the staged, resource-admitted candidate. It refuses missing, immutable, foreign and changed bindings; initial binding adoption remains unimplemented. Neither an inspected descriptor nor the literal proves resource admission, authority or durable commit. The two `.inc.rs` files contain proposed tests to include inside `rig_binding::tests` alongside its existing synthetic fixture. Nothing is registered, compiled, or executed. Before use, review cold admission cost and require rechecking source and authority at commit.

## Actual integration still needed

1. `CheckpointEditorHost` must supply the selected HMC resource bytes through current read authority, selected resource key, package association and bounded read; typed inspection alone does not supply those bytes. Recheck source and authority before publishing.
2. `CanonicalEditorHost` must project and validate a source-owned rig binding, and commit checked candidate controls through its existing expected-source, authority, history and checkpoint path. Its current `inspect_authoring` and `edit_assembly` support parts, parameters, material motion and botany but no rig descriptor or edit variant. The JSON output above cannot simply be appended to Vibe as if it were a source program.
3. `InteractivePreviewProvider` and `RetainedViewport` own the retained scene and GPU. A current selection exposes a part key and index range, not a verified vertex span or local bind mesh. Assembly transforms and flat-normal/atlas expansion may change correspondence. Establish a verified exact correspondence with the selected resource or refuse; do not infer a range from arbitrary index extrema. Then provide actual source/selection/authority retirement and frame upload on that owner.
4. Mount controls only after a real selected descriptor and authorized edit dispatch exist. Connect UI and MCP to the same source operation. Qualify source edit, save and fresh reopen, selected resource and span, visible frame, stale/denied/cancelled preservation and observer denial. Direct admitted contributor use does not require a new personhood provider.

Coordinator owns the shared registration, host, authority, UI, viewport and serial Cargo checks. Candidate files are not a substitute for those changes. Current assembly geometry includes world transforms, normals and UV streams; exact local bind correspondence must be established, not assumed. Original asset HOLD and full model/package obligations remain.

## Source pins inspected (SHA-256)

Poet source was read at the report's stated HEAD `3d167fdff42eb7fd913d998d786a0a39639b5d6e`; its `.git` worktree status was not independently re-established. Hashes pin bytes actually inspected:

| Poet path | SHA-256 |
|---|---|
| `native/src/graphical/rig_binding/mod.rs` | `28d3659061cd9f220a8b6a575a6f95a4cd2939b434b3120a1fef0a854c6f7f08` |
| `native/src/graphical/rig_binding/controls.rs` | `71668671a00f04be1b70c312754d7f45a6fee9b2f3c1c097b0fbace7b2c43613` |
| `native/src/graphical/rig_binding/tests.rs` | `3f113e3d63f3866a55ab9185323076ea8ed665e8735e6b9ddc6ef66904c628c2` |
| `native/src/graphical/ui.rs` | `873d764813a85101e5ad9dff90afe47dfbb36d34830eeb7f4802e97131739d21` |
| `native/src/graphical/animated_rig_preview.rs` | `62036a8855f192d89853997e629920be430d3c694a524edf577f9c51a7db92a7` |
| `native/src/graphical/authoring/controller.rs` | `2753496fc6233a7c04d02989e8e099a4f7fb15d6436c4434d89e4d7c4bfe16cf` |
| `native/src/graphical/viewport_provider.rs` | `187b42d683694d3298aa9ed7da0ddd0e941fc30dbcad6009d02e058587aec5d6` |
| `native/src/graphical/viewport/renderer.rs` | `2f97ea57e6f23b7fa551c563d6dc359fb154d0a9ef16a09adaa9e914c40e0ded` |
| `native/src/graphical/checkpoints/editor.rs` | `74b07c4015f17c9f6a50b806f33959557c44030350078d1c50341f4352989d9b` |

The report and prior Round 1 receipts distinguish source tests from executed evidence; they do not prove this candidate. Source searches were limited to the graphical host, selected checkpoints, renderer, assembly and rig modules. No complete ecosystem audit is claimed.

## Re-entry and focused check

Only after review and approved application into Poet: copy the candidate to `native/src/graphical/rig_binding/rig_candidate.rs` and register `pub(crate) mod rig_candidate;` in `rig_binding/mod.rs`; copy `rig_source.rs` to `native/src/graphical/authoring/rig_source.rs` and register `pub(crate) mod rig_source;` in its `mod.rs`; include the two test bodies in `rig_binding/tests.rs`, and verify current source pins first. The first proposed test expects a retained preview unchanged and a fresh candidate replay, with wrong selected key refusal. The second evaluates the emitted Vibe binding cell and checks missing/immutable/changed/malformed source refusal. Neither exercises actual source-owner authorization, resource resolution, UI or checkpoint. The coordinator can run from `C:/github/poet` using the existing serialized lease:

```powershell
python C:/github/ecopunk/tools/poet-foundation/biosphere-shared/serial_check.py --owner OP-VIEW-01 -- cargo test --locked --offline --target-dir C:/github/ecopunk/target -p poet-cli --features native-graphical --lib staged_rig_control_keeps_existing_preview_and_refuses_wrong_resource
```

Use the same command with test filter `rig_source_candidate_evaluates_and_refuses_unsupported_contexts` for the second test. Neither is the end-to-end qualification. Wiring this into `AuthoringEdit` needs the controller to receive authorized resource bytes and verified viewport data before creating its candidate, plus intent dispatch and cooperative authorization. Those shared interfaces do not exist in the selected path; a serialized candidate supplied by a UI or agent would circumvent this safety boundary and must not be substituted.

Capture actual source hashes, process and exit status; do not count a zero-test result. Add integration tests after connecting the source owner and retained viewport. This worker left no intended running command, process lease, or retained build session.
