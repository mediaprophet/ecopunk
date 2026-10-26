# Internal AO depth precision proposal

**Staged, not applied, no post-fix GPU qualification.** ENG-04/06 high-grade
surface/foliage precision blocker. Parent owns review/application and one coherent
core/Poet rebuild. No Computer Use, sibling edits or whole-core rebuild was used
by this worker. Exceptional flora, masters, film/game/cartoon and model/HMC
acceptance remain incomplete; no production packet or full-suite gate is waived.

## Reproduced fault and evidence

The exact unchanged canonical core test binary was run once with
`QUALIA_REQUIRE_GPU_TESTS=1`, exact test selection and one test thread. It failed
again with hole=18 / solid=15 (exit 101), matching the full-suite failure.
[Isolated log](../isolated-test.log), [binary/dates/command](../isolated-verification.json).
This is not attributed to ordering/environment or declared preexisting.

The native camera far plane is 15000. The old AO BA16 depth packer divides
linear view depth across 65534 intervals, producing a 0.22888882 world-unit
quantum. The AO radius accepts 0.05..2.0 and its horizon test uses a 0.04 ramp;
near-object detail/bias can therefore be lost before AO reconstruction and the
mesh bilateral upsampler. The old test's >100-code separation corresponds to
22.89 world units even though its front/rear planes differ by 0.7.

An independent CPU ray/plane calculation using the actual native CameraState
predicts rear/front view depths 3.874034 / 3.149179 and old codes 18 / 15,
exactly matching the GPU observations. This supports both the alpha-mask
visibility diagnosis and the precision defect; it does not qualify all alpha/AO
behavior. The existing >100-code GPU assertion is **unchanged**. The proposed
encoding gives CPU-oracle codes 10796 / 9698 and adds a direct GPU-code vs
analytic ray/plane oracle with a <0.001 near-depth error requirement.

## Versioned format and complete consumer audit

`AODEPTH_LOG1P_BA16 v1` retains octahedral normals in RG8 and packs
`log2(1 + view_depth)` into BA16. Code zero remains background; 1..65535 cover
zero through the native CAMERA_FAR_PLANE. AoUniform.depth.x remains that camera
far value; depth.y now carries `log2(1 + far) / 65534`. The formerly reserved
camera_forward.w lane identifies version 1. All three shaders check that kind.
Unsupported internal kinds discard the prepass or return neutral AO rather than
decode a mismatched packed value; the native uniform factory always selects v1.

All discovered packed-depth producers/consumers are updated:

| Owner | Role |
|---|---|
| `ao_prepass.wgsl` | Sole producer; alpha mask and normal encoding preserved; log1p depth packing. |
| `ao_generate.wgsl` | Decodes centre/neighbour depth for world reconstruction and occlusion. |
| `mesh.wgsl::screen_space_ao_visibility` | Third consumer: decodes depth for bilateral AO upsampling/rejection. |
| `ao_math.rs` / new `ao_depth.rs` | Native camera-derived uniform scale/version and CPU precision/readback oracle. |
| `material_pixel_tests.rs` | Test-only raw BA16 readback; original assertion preserved and analytic oracle added. |

Boundary inspection: gpu::ao is a private module; AoUniform/AoTargets are
pub(super). The packed normal_view is consumed only by portal_gpu init and
lighting-pass bind construction. Raw normal texture readback is cfg(test) only.
No public method exporting packed BA16 bytes or persisted HMC depth resource was
found in the inspected core tree. Main camera/deferred depth and other depth
formats are not changed. This is an explicit **internal semantic format change**,
not a claim that unchanged byte size means unchanged ABI interpretation. Transient
AO targets/pipelines are rebuilt from the same binary; no persisted migration
or compatibility with mixed old/new shaders is asserted.

The internal uniform layout, texture extent/formats and budget remain the same;
no extra attachments or per-pixel CPU work are added. The shader encoder uses
log2 and its consumers exp2. Their actual performance and near/far rendering
quality still require native qualification; passing equations is not that result.

## Executed checks and required next checks

Five focused CPU tests passed against actual cached native camera constants:
sentinel/endpoints/invalid inputs, submillimetre near-object roundtrip,
monotonic bounded relative error through 15 km, 5 mm..5 cm bias/detail separation,
and the actual camera/plane oracle. [tests.log](harness/tests.log).

All three candidate shaders parsed and validated through the actual cached
Etalon Naga (one test passed). [shader-tests.log](harness/shader-tests.log).
The first focused shader link omitted windows.0.48.5 library search path; the
corrected link succeeded, with the original error retained separately. There was
no Cargo/dependency rebuild. [verification.json](verification.json) pins inputs.

Parent must apply all seven guarded targets together, rebuild the actual core
and Poet coherently, run the unchanged-and-strengthened native AO mask test and
the relevant AO/material GPU family, and record results. The original 9712/43/3
full-core result remains failed. No CPU-only or isolated result waives it.
