# W2 exit gate reconciliation: native GPU and remaining acceptance

Date: 2026-10-26. Scope: reconcile the frozen `POET-TEXTURE-W2` contract and its exit criteria against executed evidence already recorded in `docs/qa/implementation-2026-10-26/poet-texture-effects/`. This is a bounded evidence reconciliation; no source was changed and no command was executed.

## Result

The selected software increment passed its recorded software cases: 268 passed, 0 failed, 9 explicitly ignored; the native executable built successfully. The signed native/MCP workflow, encoded CPU result, history, HMC resource closure, and fresh-process reopen are evidenced by `verification.json`, `check-attempt-2.log`, `native-build.log`, and the retained 8x8 synthetic PNGs plus `diagnostic/witness.json`.

W2 is **not fully exited** because actual native GPU preview/render qualification remains open. This is an evidence gap with a concrete missing case, not evidence that the current implementation is defective. No human acceptance or domain approval is evidenced. Asset production remains **HOLD**.

## Gate-by-gate

| W2 contract outcome | Recorded evidence | Status |
|---|---|---|
| Identified imported image/reference; ordered effect edits and result with source/result identifiers | `verification.json` records 15 selected effect/control/integration tests and synthetic source/output/package digests. | Passed for bounded CPU/software profile. |
| Existing native controls and admitted agent command edit the same document revision | `signed_texture_effect_native_mcp_pixels_history_hmc_and_fresh_process` and refusal case are listed as passed; source receipt pins installed Poet files. | Passed for fixture-backed software path. |
| Undo/reopen retains editable effects and unknown data | Integration test summary covers undo/redo, HMC metadata/resources, fresh-process reopen/rebind, and exact recipe/source persistence. Strict unknown-field/kind refusal is covered. | Passed within tested fixture/profile. |
| Result is actually visible through Poet's native GPU preview/render path | Nine GPU tests were ignored in the full test run. Specifically, `graphical::surfaces::detail_pixels::authored_atlas_paint_and_procedural_detail_change_actual_native_pixels` is an existing ignored native-GPU path, but its authored Paint recipe has `effects: vec![]`; it does not exercise W2 effects. The signed effect integration fixture explicitly states it produces no GPU frame. | **Open: missing effects-through-GPU oracle.** |
| Current-window usability and actual human review | `try-it.md` is only a route; it explicitly says no person used or accepted the current window. | Open; requires a human operation/review, not a code inference. |
| Separate human/domain approval | No such review/signature is in the evidence. | Open; must be supplied by an authorized reviewer/domain participant. |

## Dispatchable software evidence task

There is no demonstrated software failure in the selected W2 profile requiring an implementation fix. The narrow dispatchable addition is an **opt-in, serial native GPU qualification case** adapting the existing `surfaces/detail_pixels.rs` path (or a separate owned sibling test file) to render the same identified synthetic imported base with a non-empty enabled effect and its disabled/baseline counterpart through `TextureResources` → `resolution::resolve` → `assembly::render_assembly_with_resources` / actual native GPU readback. Assert a stable, nonzero expected pixel difference at the relevant material surface, alpha/resource behavior, and retain exact inputs/results and run metadata. Include effect order or a known single operation and verify that the resolved generated digest is the one uploaded. GPU absence must fail this explicit run, never silently skip. Keep this as synthetic engineering qualification, not visual quality or production acceptance.

Required evidence for that task: exact source preimage/current revision; exact test selector and command/session/exit; device/backend identity if reported; actual before/after GPU readback artifact digests; selected recipe/source/base/result IDs; actual resolved and uploaded resource identity; pixel oracle; failures/recovery; no produced production asset. The existing general surface GPU test cannot stand in for this case because it tests canonical maps/UV, not the W2 effect operation. Likewise, the existing detail GPU test uses no effect.

## Explicitly out of scope / still tracked

- The 15 Core no-op/ignored blend/mask/placement items remain in `work/deferred-functionality.json` and `work/deferred-functionality.md` as deferred future functionality. They are not all W2 blockers: W2 explicitly rejects full compositor/all-format parity as a prerequisite. Do not close or reclassify them by assertion.
- Full compositor, RAW, ICC/linear-light, animation, and broad format parity remain future work, not a fix required for this selected profile.
- A native offscreen GPU pass would not establish current-window usability, human acceptance, human/reviewer cooperation, biological/domain approval, or IP-03 C1–C8. These need their own evidence and authorized people.
- Asset production remains **HOLD**. The synthetic 8x8 calibration outputs must not be promoted as production assets.

## Evidence/source pins consulted

- Contract: `tools/poet-foundation/texture-effects/CONTRACT.md`.
- Wave/exits: `docs/planning/poet-authoring-swarm-phases.md` (W2 row; selected profile and no full-compositor prerequisite).
- Executed evidence: `docs/qa/implementation-2026-10-26/poet-texture-effects/README.md`, `verification.json`, `try-it.md`, `diagnostic/README.md`, `diagnostic/witness.json`, `check-attempt-2.log`.
- Concrete existing GPU path: `C:/github/poet/native/src/graphical/surfaces/detail_pixels.rs`, read for the relevant ignored test and recipe construction. The source currently constructs the paint recipe with an empty effect list, confirming the missing oracle only; it is not evidence of a runtime defect.
- Model/effort for this reconciliation: gpt-6-luna, medium. No usage/cost metric available. No tests, builds, GUI, provider calls, or assets were run or created.
