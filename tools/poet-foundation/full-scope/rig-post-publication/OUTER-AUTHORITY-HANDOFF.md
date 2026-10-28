# Rig viewport: outer authorization publication boundary

Status: static source trace, 2026-10-27. No canonical source changed and no execution test performed for this boundary. The companion manifest stages a canonical-owner late-cancellation guard; it does not fix this outer boundary.

`NativeProject::open` composes canonical -> checkpoint -> optional import -> shared host, with a separately authorized host for each session (`native_project.rs:185-203`). `AuthorizedEditorHost::viewport_action` calls `with_authorized` (`cooperation/editor.rs:5-11`), whose callback dispatches to the shared host. `SharedEditorHost::viewport_action` takes its mutex only for the inner method (`shared_host.rs:59-63`). A successful rig preview changes the retained viewport during that method. After the shared guard is dropped, `with_authorized` still checks the source after the callback, the owner's return value, and duplicate callbacks (`cooperation/mod.rs:270-292`). The native owner itself also validates post-operation source state (`cooperation/native_owner.rs:source_transaction`). These code paths permit an error after an inner preview has mutated retained state. This is a source-level possibility, not a claim that an actual production failure was observed.

The shared guard is gone before the outer result is known. Another admitted frontend may acquire it in that interval; `viewport_provider.rs:ensure_scene` can retain and reuse the scene. A later unconditional invalidation after the outer error cannot guarantee isolation, because another action could already observe the candidate or install a later frame, which the invalidation would then discard. A generation check could avoid discarding a later frame but alone cannot stop interim observation. Native authority and shared-editor locking already have ordering rules; reversing them piecemeal risks deadlock or violating permit lifetime. Do not silently weaken authorization or present a no-op invalidator as a full remedy.

## Implementation seam and required validation

The coordinator should design a transaction at the existing shared-owner boundary: make a candidate rig frame private until all relevant authority and source checks complete, then atomically publish it under the shared viewport owner; on refusal, discard the candidate. Alternatively, explicitly retain a shared-owner transaction guard over authorization completion only after tracing every caller and establishing a single deadlock-safe lock order, nonblocking behavior and bounded critical section. Either option must preserve existing session-specific permits and avoid nesting an authorized host under the shared host. The exact design depends on the renderer and authority ownership; neither alternative is claimed implemented or proven here.

Add deterministic concurrent tests, not timing sleeps: an authority fixture pauses after the shared callback and then returns an error, while another authorized frontend attempts a redraw; verify no rejected candidate can be observed, no accepted newer frame is invalidated, and both permit and source checks remain effective. Include successful publication, duplicate callback, source retirement, owner error, cancellation, lock contention, and disconnect cases. Exercise the actual GPU/native-project path separately, and record the test identity and current source hashes. A fake owner callback alone is not evidence of a functioning human UI or GPU race freedom.

## Inspected source pins

- `C:/github/poet/native/src/graphical/native_project.rs`: `00bd63dc4f40b36a656b9ba0f80f3f035020aec1ffc618bb83c373c0b0efefa7`
- `C:/github/poet/native/src/graphical/cooperation/editor.rs`: `8671fe72e0ff9b7f3a3287e9114c6a05e2eb0d2f9da446f6c602e46673a74147`
- `C:/github/poet/native/src/graphical/cooperation/mod.rs`: `8e538ad7a7b1017a01fddcb86b9847f80000e6a270610d2f4d34c07c17166970`
- `C:/github/poet/native/src/graphical/cooperation/native_owner.rs`: `a846da15ad84bacb604116edaac18c2f33117c335ca273566f2c59b489f1728a`
- `C:/github/poet/native/src/graphical/shared_host.rs`: `9682c6aefd29c53ed2d8a6b04dfe64321ac036e113c86ba3279cc749c90f8867`
- `C:/github/poet/native/src/graphical/viewport_provider.rs`: `cb5656bf5e70aa70c9263f2eb3929e79a9c90ee5961d23e6bcad6321b073cf8a`
- `C:/github/poet/native/src/graphical/controller.rs`: `2772b614556604befeb21ae0c439f328f704801a325403b78b6804f451f24125`
