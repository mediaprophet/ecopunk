# Mounted image component tests handoff

2026-10-26. Staged for coordinator review; not compiled or executed in this lane.

- Owned new file: `native/src/graphical/authoring/image_completed_component_tests.rs` (SHA-256 `316fd510497bc4ab96aabd53a191fe0331d79860862a5549408b2ab675c9e3ab`). No canonical Poet files were edited.
- Include as a child of existing `texture_effect_component_tests.rs` after the `ComponentHarness` definitions. The child uses `use super::*` and sends real mounted VDOM `change`, `input`, and `click` events through that harness.
- Six `image_completed_component_` tests cover four filter field/DTO emissions, ten blend choices, signed offsets, rectangle masks, red/alpha resource masks, invalid primary and secondary values, malformed SHA and extent, hidden layer and disabled effect refusal, v3 retention on legacy Add, and source/context change with explicit reload.
- The older harness fixture constructs `PaintLayer` with only legacy fields. When integrating the new DTO, add `..Default::default()` to that fixture in the coordinator-owned parent test file. Keep the three old mounted tests.
- Source pins inspected: image completion `CONTRACT.md`; staged controls `work/controls/native/src/graphical/authoring/detail_controls.rs` SHA-256 `5307a61a701b98d261dddd1d8f810b611772ae462e2cfbc85532521fac62bd6b` per controls handoff, including aria labels and Apply; staged DTO `work/poet-domain/native/src/graphical/surfaces/detail/recipe.rs`; prior mounted harness `tools/poet-foundation/texture-effects/work/gpu-qualification/component/native/src/graphical/authoring/texture_effect_component_tests.rs`.
- No Cargo, build, GUI, provider, asset, or native-window checks were run. These tests assert actual component events and emitted DTO only; they do not establish GPU, HMC, native-window, human, or complete model acceptance.
