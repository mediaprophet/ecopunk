# Image completion signed GPU workflow handoff

2026-10-26. Staged only; implementing model GPT-6 Sol, high reasoning. No canonical Poet or QualiaDB files changed by this lane. No Cargo, GPU, tests, providers or asset production ran here. `rustfmt --edition 2021 --config skip_children=true` parsed and formatted the staged module.

| Item | SHA-256 |
|---|---|
| New `native/src/graphical/native_project_image_completed_tests.rs` under this directory | `dfaf2469bc337a63167bbd25d4b7cf87bc0a5552bd4f174fc5a880e9a2c7359b` |
| Canonical parent `C:/github/poet/native/src/graphical/native_project_texture_effect_gpu_tests.rs` inspected | `608485792b914af038933a55ebafebef354816fbde43802a134b50026d0ca076` |
| Frozen staged Poet DTO `work/poet-domain/native/src/graphical/surfaces/detail/recipe.rs` inspected | `d02c35fc1e4cc31c18cf01ce221d038fd84a20a5b4fb8f004e87a5fcfefa443b` |
| Root staged MCP `work/integration/poet/native/src/graphical/mcp/texture_effects.rs` inspected | `2c4710e56d1e84f77838142ce26faa7d386b266ac87c460e1b04c3f54cc3c3f3` |

The new module is a child of the existing GPU test module, so register it **inside** `native_project_texture_effect_gpu_tests.rs` with `#[path = "native_project_image_completed_tests.rs"] mod image_completed_tests;`. Its `use super::*` deliberately reuses `gpu_provider`, `gpu_frame`, `gpu_apply`, `gpu_evidence`, `gpu_child_reopen`, the signed fixture, texture seed, source digest/recipe, and HMC helpers. Root owns registration and canonical install after source review. Its canonical preimage is absent, since the module is new.

The first ignored exact test edits all four typed filters through native TextureDetail and admitted MCP effects, and all nine non-Normal modes through native TextureDetail and admitted MCP layers. Each must change actual GPU frame pixels and output digest against the same full painted Normal recipe. It checks signed offset plus rectangle mask; creates a synthetic 8×8 HMC PNG mask with distinct raw red/alpha coverage and bound metadata; edits both mask channels natively and by MCP; checks inspector detail/base/output; undoes/redoes and rebuilds; compiles and saves original, mask, derived resource closure; then launches its own `--ignored --exact` child. The child reuses the ancestor's authoritative reopen/rebind/real GPU byte comparison and additionally checks original mask bytes, metadata and child-resaved closure. It writes a unique marker so zero-test child exit fails. Optional witness output uses `POET_IMAGE_COMPLETION_EVIDENCE_DIR`, pre-created by caller, and `create_new`; ancestor GPU PNGs continue using `POET_TEXTURE_EFFECT_GPU_EVIDENCE_DIR`.

The second ignored exact test checks missing mask resource, invalid filter parameter, version 2 feature refusal, stale MCP context and observer-role denial against exact saved Studio artifact/history, while requiring a real native GPU frame for the accepted source. Both tests remain **unexecuted**. Root should run the actual serial native closure/GPU tests, repair any failures, and report precise commands/results. The generated 8×8 content is engineering-only and does not fulfill the production asset dossier.

Integration assumptions to review: root's mechanical `PaintLayer { ..PaintLayer::default() }` compatibility edits are installed; canonical domain and MCP staged modules are installed; existing GPU parent helper and `saved-gpu.hmc` witness shape remain as inspected. These tests do not establish native-window/human/product acceptance.
