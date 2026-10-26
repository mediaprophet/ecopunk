# Image controls form-state review handoff

2026-10-26. Staged only. No integration or canonical files edited. No Cargo, mounted test execution, GPU, provider, or asset production ran. `rustfmt --edition 2021 --config skip_children=true` parsed/formatted the two staged Rust files.

| Relative file under `native/src/graphical/authoring/` | Staged SHA-256 | Integration preimage SHA-256 |
|---|---|---|
| `detail_controls.rs` | `330b3c37f7948e955558b572cd4c75ba2a3e2e72507c72a5b79416ced8ce41db` | `5307a61a701b98d261dddd1d8f810b611772ae462e2cfbc85532521fac62bd6b` |
| `image_completed_component_tests.rs` | `43e5fd4c0f09e8687261c26d6875199030fb07bdbbec7b52d1963c0cb2e90376` | `316fd510497bc4ab96aabd53a191fe0331d79860862a5549408b2ab675c9e3ab` |

`detail_controls.rs` keeps pending stroke text and selected stroke index per layer in the existing local form state. Layer add/select/reorder/removal loads the matching form, so “Replace loaded stroke” cannot use another layer's fields. Invalid raw stroke text survives a switch away and back, and Apply still refuses it. Move/remove stroke refreshes the form from the new selected stored stroke, with a visible message saying that pending unstored stroke text was discarded. Add stroke updates the selected index. The existing authority/source edit path is unchanged.

`image_completed_component_tests.rs` adds three mounted Aether event tests: cross-layer Replace plus invalid raw retention and layer reorder, layer removal, and stroke reorder/removal followed by Replace. The tests depend on the retained `texture_effect_component_tests.rs` mounted harness and are unexecuted. Root owns review, install and serial checks.
