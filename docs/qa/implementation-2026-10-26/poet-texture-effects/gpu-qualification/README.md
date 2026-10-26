# Wave 2 native GPU and component qualification

2026-10-26. User resumed remaining Wave 2 implementation/verification. Asset production **HOLD**. This extends the selected imported-texture/effect profile; it does not turn deferred full-compositor or Core image stubs into passed capability.

## Owned work and evidence

- [Qualification contract](../../../../../tools/poet-foundation/texture-effects/GPU-QUALIFICATION.md) and [gate reconciliation](../../../../../tools/poet-foundation/texture-effects/work/gpu-qualification/gates/report.md): exact scope and remaining human/domain distinctions.
- [Component handoff](../../../../../tools/poet-foundation/texture-effects/work/gpu-qualification/component/handoff.json): actual Aether/Dioxus DetailControls mount, mutation stream and input/change/click handlers; accepted/draft separation and callback recipe checks. A mutation sink locates rendered controls; it does not replace their form logic.
- [Retained GPU handoff](../../../../../tools/poet-foundation/texture-effects/work/gpu-qualification/session-gpu/handoff.json): signed project and existing InteractivePreviewProvider/PortalGpu; native dispatch, admitted MCP effects, actual source/session/epoch-bound frames, order/disable/remove, undo/rebuild, original/derived HMC resource closure and fresh-process GPU reopen/rebind.
- [Applied source](applied-source.json): four reviewed files (two new test files and two parent registrations), preserving exact preimages. No kernel, dependency or new state store was introduced by this installation.
- [Existing atlas/detail baseline](baseline-gpu.log): exit 0, one actual GPU test; paint and procedural changes each affected 2,304 pixels. [Retained baseline PNGs](baseline/) are synthetic calibration readbacks. This recipe had no effects, so it establishes only its stated baseline.

Luna medium performed the bounded reconciliation. Two gpt-6.1-sol high assignments encountered capacity errors; partial component work was preserved. gpt-6-sol high workers completed the GPU and component candidates. Root reviews, saves and runs all Cargo checks serially. Per-agent tokens/currency are unavailable; no price/savings claim is made.

## Runtime profile and current qualification state

The baseline used automatic native device selection. The effect-specific run will explicitly request `QUALIA_RENDER_WGPU_BACKEND=dx12`. Inspected Core `render/gpu/portal_gpu/constructors.rs::new_offscreen` bypasses the shared device for an explicit renderer pin, and `native_device.rs::request_offscreen_device` tries only that backend. An executed success under that pin qualifies the selected DX12 path; no adapter name/hardware class or other backend success is inferred. The current Poet root patches wgpu/Naga to Etalon Git `qualia-main`; its lock resolves Etalon revision `ed0b671d8903e8e0d3f7a7f38fba5a0f6b4e08a6`. This is the resolved dependency, not a claim that the current local Etalon worktree was built.

The following selected checks have completed:

| Check | Actual result |
|---|---|
| [Mounted component events](component-attempt-1.log) | Exit 0, three passed: real load/edit/order/disable/apply, invalid-text retention/refusal and changed-source/context explicit reload. |
| [Retained effect GPU workflow](retained-gpu-attempt-2.log) | Exit 0, one passed on strict DX12, including fresh child GPU/HMC recovery. Each of five effect/order comparisons changed 300 actual rendered pixels; disabled/removed/undo/rebuild and fresh-process outputs restored exact expected bytes. |
| [Material and retained viewport](native-gpu-regressions.log) | Exit 0, four passed on strict DX12: actual material edits, picking, device/upload retention and view-lighting changes. |
| [Map roles and UV](maps-uv-gpu.log) | Exit 0, one passed on strict DX12. Base colour, normal, metallic/roughness, occlusion, emissive, stylized ramp and UV transform each changed 2,304 pixels. |

The [first retained-GPU attempt](retained-gpu-attempt-1.log) failed at fresh-process journal opening after the effect/history assertions passed: the parent test still retained admission/session snapshots. The reviewed repair puts every parent authority owner inside a lifetime scope that returns plain JSON before child launch. Journal locking and every GPU oracle remain intact; the corrected run passed. [Initial source receipt](applied-source.initial.json) and failed readbacks remain available.

[Actual readback gallery](gallery.md) and [GPU witness](attempt-2/gpu-witness.json) provide inspectable outputs. Component events are programmatic software evidence, and the GPU readbacks are offscreen.

The [final library regression run](library-regressions.log) exited 0: **271 passed, zero failed, ten default-ignored GPU tests**. Seven of those GPU cases were explicitly executed and passed in the recorded commands; the biological branching and reference parent/child diagnostics were not run in this selected qualification. The [native executable rebuild](native-build.log) also exited 0. [Final verification](verification.json) pins sixteen current canonical files, actual logs, current executable, resolved dependency source and exact parent/child PNG equality. Prior CPU-only receipts remain historical evidence rather than claims about the newly qualified source set.

Current-window layout/accessibility and actual human/reviewer/domain acceptance remain distinct open outcomes. Programmatic component events plus offscreen GPU readback do not invent those approvals. Engineering inputs/readbacks are not production species/thing assets, and full IP-03 C1–C8 remains open.
