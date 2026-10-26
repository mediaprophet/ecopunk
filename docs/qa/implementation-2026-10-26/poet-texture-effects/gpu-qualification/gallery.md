# Native DX12 texture-effect readbacks

Actual 64×64 GPU frames from the signed synthetic texture test. The imported image, editable paint layer and geometry are engineering calibration inputs. These frames verify the rendering path and recovery; they are not production assets or artistic-quality references. No image-generation provider or image enhancement was used.

| Accepted paint baseline | Grayscale | Invert |
|---|---|---|
| ![GPU baseline](attempt-2/parent-baseline-gpu.png) | ![GPU grayscale](attempt-2/parent-grayscale-gpu.png) | ![GPU invert](attempt-2/parent-invert-gpu.png) |

| Sepia | Ordered final stack | Fresh-process HMC reopen |
|---|---|---|
| ![GPU sepia](attempt-2/parent-sepia-gpu.png) | ![GPU final](attempt-2/parent-final-gpu.png) | ![GPU reopened](attempt-2/child-reopened-gpu.png) |

The final stack is Sepia followed by Invert. The test checks non-commuting order, disabled/removed steps, exact undo/rebuild restoration, source/session/epoch currency, admitted native/MCP edits and retained original/derived resource bytes/metadata. Final and reopened PNG hashes must match exactly. [Machine-readable witness](attempt-2/gpu-witness.json) records actual source, input/output/package and frame digests. Asset production remains **HOLD**.
