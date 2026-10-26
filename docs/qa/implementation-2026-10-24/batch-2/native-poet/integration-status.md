# RUN-07A frozen native Poet integration

Source frozen in `apply-manifest.json` (15 exact targets); `verify-or-apply.ps1` validates every existing original, absent new target, and candidate digest before writes. This adds optional `native-graphical` to the actual Poet workspace and a `poet studio` route. Existing CLI format/lint/eval/repl remain; browser dependency pins are reconciled explicitly to the existing controlled Etalon family. Old manifests/lock are preserved in originals/. No sibling source was written by this worker.

The native path is Aether local dioxus/dioxus-native 0.8.0-alpha.3, CPU Vello/softbuffer, system fonts and native data-uri image decoding. Its actual launch_cfg_with_props path creates a Blitz document/window and enters winit. It is not the desktop/webview feature. Root's controlled canonical Aether check passed. The selected actual CLI normal/build graph has 604 packages, only Etalon wgpu30/Naga at ed0b671d; registry wgpu29 in raw workspace metadata is not active here. See native-dependency-closure.json. These are dependency/source checks, not a launched-window result.

`CanonicalEditorHost` owns the canonical SourceSession and concrete GeometryHost. Source is accepted text cells in the existing StudioArtifact, with checked incarnation/revision mutations and real undo/redo. Draft text is UI-only, not another authoritative document store. Runtime source requests never create grants. GeometryHost independently rechecks exact target grants, including dynamic IDs, before the real PoetSnapshot dispatch. Only ComputationalGeometry.extrude_profile and swung_surface are admitted; graph/network/aura/pulse are refused. Source cells must be the exact contiguous identified sequence 1..N; malformed/gapped histories cannot reopen/rebuild and overwrite a cell.

Undo, redo and attempted runtime errors hold execution. Explicit Rebuild execution re-evaluates the captured accepted cells into a fresh canonical session under unchanged actual grants, SessionLimits and the same two read-only builders. It bounds total source bytes/cells/instructions/estimated geometry work/kernel dispatches, publishes no frame, rechecks the opaque store stamp and prepares the entire outcome before promotion. Failed/stale replay preserves the previous session/epoch/source. Only successful promotion retires the old opaque epoch. This is a narrow read-only reconstruction, not generic external-effect replay or a durable checkpoint.

One owned bounded worker performs explicit Check/Evaluate/Undo/Redo/Rebuild; no per-keystroke kernel/GPU work. Request origin, source incarnation/revision, session revision, opaque active epoch and held state bind completions. Shared latest context is cleared during work. Frames retain their exact context; UI installation rechecks the live epoch/latest context and cheap try_lock store stamp. A busy/poisoned store fails closed. Worker tests cover stale frames, queued replies, held state and epoch reuse. Dirty drafts survive source navigation.

The first mounted preview is actual geometry output with static caller-supplied PBR factors. Wave27 nested triples and wave29 flat streams are validated into canonical Mesh. PortalGpu/Etalon actually uploads/renders/readbacks; PNG/RGBA8 is only the bounded initial native viewport (512 default), not HDR/master-colour qualification or a ceiling on editable master fidelity. Semantic/spectral/material sources are not replaced by preview pixels. Source-linked material editing and eventual interactive GPU/master-review presentation remain open. The HMC helper is explicitly unmounted/unqualified: existing loader colour-role/flags, refused uploads and resource integrity need separate repair/review. Legacy Studio receipt-shaped export handlers are not wired as executed geometry/HMC exports.

Entry admission is explicitly diagnostic and atomic create-if-absent. Existing project/source reopen requires native host admission integration. Contributor/reviewer roles, principal identifiers and authorization must come from the existing native project/rights owners. Native wellfair projects/work-items remain the project membership, contribution and review owners. This in-memory store is not the complete embodiment graph, persistence or full cooperative C1-C8 outcome. No organism/assembly master or complete film/game/cartoon package is claimed.

## Verification and commands

Final narrow Host probe passes 3/3 with real cached, mutually matching core/Vibe artifacts: denied ungranted dynamic target; real nested extrusion to canonical mesh; real flat swung mesh and exact cap capacity refusal. It compiles the production GeometryHost and verbatim production mesh parser functions against actual core types. It does not establish current SourceSession/UI linkage. The separate canonical SourceSession prepare probe passed explicit dispatcher+target grants and denied empty grants; prepare is not dispatch.

The frozen product stages 21 CPU tests: 6 controller recovery, 2 geometry authority, 2 preview admission/marshalling, 11 worker/completion tests. They require the coherent current core/Vibe build and have not run yet. Run serially because busy try_lock admission intentionally fails closed; independent test documents share one store mutex.

After reviewed guarded apply, root owns the one coherent actual product build:

```powershell
$env:CARGO_PROFILE_DEV_DEBUG='0'
$env:CARGO_PROFILE_DEV_INCREMENTAL='false'
$env:PYTHON3='C:/Users/tim_h/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe'
$env:PYTHONDONTWRITEBYTECODE='1'
cargo build --offline --locked --manifest-path C:/github/poet/Cargo.toml -p poet-cli --features native-graphical --target-dir C:/github/ecopunk/target
cargo test --offline --locked --manifest-path C:/github/poet/Cargo.toml -p poet-cli --features native-graphical --bin poet --target-dir C:/github/ecopunk/target graphical:: -- --test-threads=1
C:/github/ecopunk/target/debug/poet.exe studio --diagnostic-new-source --grant capability.invoke --grant ComputationalGeometry.extrude_profile
```

Current lock SHA256: F1C6D2DDFD33EC813815B3CEBDEA120B90AFC97CBBDC764742D64EC0D6C1B627. No cached old ABI substitution is allowed in this product build. Actual window, source edit, real rendered output, undo/rebuild and reviewer observations remain required evidence. A source freeze is not completion.

Supervisor has now applied the frozen 15-target package; independent applied-verification.json confirms all 15 current target hashes match the reviewed candidates. The map remains a historical pre-apply receipt. Root owns the running actual Poet test build; no result is asserted here.

The first native-feature test invocation stopped in Stylo code generation because
the sandbox PATH exposed only the WindowsApps Python launcher. Its failure log is
retained as `actual-native-tests.log`. The bundled Python executable was verified
as 3.12.14; the subsequent invocation supplies it through `PYTHON3` and suppresses
bytecode-cache writes. This is a build-toolchain prerequisite, not a Python
runtime dependency of the native product. Source and lock bytes were unchanged.

The exact newly resolved lock also changes preexisting resolver-selected versions of cc, libc, minicov, serde_json, syn, unicode-ident, uuid and zerocopy, and adds additional versions for native dependencies. This is disclosed in lock-delta.json, including original/resolved lock hashes; it is not represented as only a browser binding delta. Controlled source pins and lock bytes remain frozen. Browser/WASM compilation and parity require their separate check after the native build.
