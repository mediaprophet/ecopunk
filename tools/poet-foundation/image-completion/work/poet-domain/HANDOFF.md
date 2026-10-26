# Poet image domain integration handoff

2026-10-26; staged only. Implementing model: GPT-6 Sol, high reasoning. The six canonical Poet preimages matched `work/integration/preimages.json` SHA-256 values before editing. This lane has not changed canonical Poet or shared Core files and has not run Cargo, tests, GPU, providers, or asset production.

Copy the complete files under `native/` in this directory into the matching `C:/github/poet/native/` paths after preimage review. `detail/completed_tests.rs` is new. All seven files passed `rustfmt --edition 2021 --config skip_children=true` syntax/format parsing. Full rustfmt module traversal was unavailable because this intentionally partial stage lacks the existing `detail/tests.rs` and `surfaces/source_carrier.rs` children.

| Relative staged file under `native/src/graphical/surfaces/` | SHA-256 |
|---|---|
| `detail/recipe.rs` | `d02c35fc1e4cc31c18cf01ce221d038fd84a20a5b4fb8f004e87a5fcfefa443b` |
| `detail/mod.rs` | `23f7b1c681591724198739d9d02a87cc462aee1cbf8468fc157e23d27a135c9b` |
| `detail/render.rs` | `d92cc26a8268f334e1b7ae4d436b71067ff2d4b480e161a82dc40731945953e0` |
| `detail/completed_tests.rs` | `4158dc98a0fbc68523f85e9524786f311e6989ad87065b7e0b24678ec68f721b` |
| `resolution.rs` | `8b19ef4dca7667a20c9eb3fd65b9717559450b283130595a3f9da6e2a1be310b` |
| `texture_inputs.rs` | `6c51810eef00e14abb4138a19c32ba9524863488cee53f3f449a8b8f84ea0833` |
| `resources.rs` | `ef7c20c29bb6c7270cf5df9635b9b540a8b6333fc781f9e57ad37ff3977bef5b` |

## Behaviour staged

- `recipe.rs` and `detail/mod.rs`: exact frozen DTO names; four typed filters; strict layer blend, signed offset, rectangle/resource mask; v3 admission; default-skipped new layer fields preserve v1/v2 serialized source expressions and digests; `required_paint_version` exported. New filters and nondefault layer features require v3. Existing v1/v2 cost formulas remain unchanged. v3 adds Core filter-plan work/byte charges and dense mask charges.
- `render.rs`: `render_detail_with_masks` with immutable `DetailMaskBase`, while old `render_detail` delegates with no masks. Preflight checks references, extents, decoded bytes, and budget before document allocation. Core owns all blending, offset, coverage math and the four typed operations. The old three steps retain `apply_filter`. `ImageDocument::try_composite` replaces unchecked `composite`.
- `resolution.rs`, `resources.rs`, `texture_inputs.rs`: resource-mask hashes join source dependency collection; exact-side HMC mask decode accepts explicit linear or sRGB transfer because selected red/alpha are raw bytes; imported base remains strictly sRGB/opaque. Mask original dependencies and retained source resources survive alongside generated output. Aggregate admission includes decoded mask storage.
- `completed_tests.rs`: source replay, rectangle and signed offset output pixels, raw red/alpha resource masks, missing resource refusal, typed filter identity/change, Multiply pixel vector, extreme offset clipping and strict v3 admission. Execution is pending root's serial native closure checks.

## Integration requirements

- Root installs its staged Core `image.rs`, `image_filters.rs`, and `image_composite.rs` first. This lane imports public `filter_plan`, `ImageFilterOp`, `ImageLimits`, `try_apply_image_filter`, `BlendMode`, and `ImageDocument::try_composite`.
- Root's mechanical legacy fixture edits must add `..PaintLayer::default()` to older struct literals. Existing `detail/effect_tests.rs` has a formerly unsupported `version=3` assertion that must instead reject version 4. Other canonical callers with complete `PaintLayer` literals require the same compatible default update.
- Controls and MCP should use frozen DTO names and `p.version.max(required_paint_version(p))`; no version downgrade.
- Root owns serial Cargo, GPU, HMC reopen and review evidence. No executed test or product completion claim is made here.
