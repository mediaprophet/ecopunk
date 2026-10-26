# Image composition worker handoff — 2026-10-26

## Staged output

- `crates/qualia-core-db/src/hypermedia_authoring/image_composite.rs` — new file, canonical preimage absent; SHA-256 `EE2170DB389066B9435257A1928E28F17732A4FC2C6DF26ACEDE842A1076D5B2`.
- `crates/qualia-core-db/tests/image_composite_completed.rs` — new file, canonical preimage absent; SHA-256 `918ED3B1EA0C99DEA3104B69387FE3AFD944B1CC57BB41360B8848F908361A98`.

These paths are relative to this handoff's `work/composite` directory. No canonical Core, shared module, manifest, or Poet file was edited.

## Integration seam

Register `image_composite.rs` as a child of the existing `image.rs` module and publicly re-export `composite_into` and `validate_document` from `image.rs`. The module refers to root-owned `ImageError` and `ImageLimits` exactly as declared in `CONTRACT.md`, and calls the existing sibling `image_source_over.rs` through `super::source_over::rgba` for every Normal pixel. The test imports the functions through `qualia_core_db::hypermedia_authoring::image`.

`validate_document` checks document and every layer, including hidden ones, before output mutation: nonzero/bounded dimensions, exact pixel/mask lengths, finite opacity in [0,1], aggregate work and byte admission. The bytes budget includes output, every layer pixel buffer and every dense mask. `composite_into` requires exact caller output length, clears it only after validation, clips signed offsets in `i64`, and samples masks in layer-local coordinates. Nine blend modes use W3C separable equations with backdrop-alpha interpolation and straight-alpha source-over. The hot loop does not allocate.

Root review caught and corrected an Overlay/HardLight argument reversal in the staged source before canonical installation. The fixed independent byte vectors were unchanged. Aggregate budget exceedance now returns within the validation loop, still before output mutation.

## Source and evidence

Inspected `C:/github/qualiaDB/crates/qualia-core-db/src/hypermedia_authoring/image.rs`: public `BlendMode`, `ImageLayer`, `ImageDocument`, current `composite()` and layer masks/offsets. Inspected `image_source_over.rs` exact legacy Normal operation. Searched `C:/github/craft` for selected blend operator bodies; no applicable Rust body surfaced in that bounded search. Checked the pinned [W3C Compositing Level 1 draft](https://www.w3.org/TR/2024/CRD-compositing-1-20240321/) for blend equations and backdrop-alpha interpolation. Test vectors are fixed expected bytes for all nine modes, partial alpha and transparent backdrop, endpoint handling, mask+opacity, clipping extremes, invalid hidden layers, budgets and output atomic refusal.

No Cargo/build/test was run by this worker. Root owns serial checks after shared registration and canonical installation; these staged source tests are not executed evidence.
