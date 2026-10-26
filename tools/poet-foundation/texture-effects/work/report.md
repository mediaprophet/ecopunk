# W2 texture/effect reuse trace

**Scope:** bounded QualiaDB CPU image/effect trace for the W2 Poet texture/effect path. Read Ecopunk `AGENTS.md`, `docs/planning/poet-authoring-swarm-phases.md`, the capability adaptation guide, saved EffectCraft review (`docs/qa/implementation-2026-10-26/effectcraft-review/{README.md,engine-authoring.md}`), and saved round-7 texture/detail handoff (`docs/qa/implementation-2026-10-25/batch-6/rounds/round7/detail/{HANDOFF.md,INVENTORY.md}`). Live body inspection was limited to `hypermedia_authoring/{image.rs,image_source_over.rs,mod.rs}`, `render/{surface_authoring.rs,mod.rs}`, plus package/workspace Cargo metadata. Hashes below pin observed bytes, not a clean commit. Root separately inspected current Poet `surfaces/detail` route; this report does not re-audit it.

## Existing QualiaDB operations

Public module exposure is real: `qualia-core-db/src/lib.rs:382` exposes `hypermedia_authoring`; `hypermedia_authoring/mod.rs:8,15` exposes `image` and re-exports `BrushStroke`, `ImageDocument`, `ImageFilter`, `ImageLayer`, and `Selection`. The same API module documents host-side rendering/compositing via Vibe invoke IDs. `ImageDocument` owns dimensions/layers/active index/selections, appends/removes layers, and `composite()` folds visible layers with source-over. `BrushStroke::apply_to` applies coverage and source-over per covered pixel. These are CPU RGBA8 operations, not a native Poet authority/session integration by themselves.

`apply_filter(&mut ImageLayer, ImageFilter, intensity)` clamps intensity to 0–1 and mutates RGB bytes. Real implemented cases include Grayscale, Invert, Sepia, Brightness, Contrast and EdgeDetect. Sepia is a weighted RGB matrix blended with the original by intensity; alpha is unchanged. Brightness only adds `intensity * 100` (no negative exposure); Contrast only uses factor `1 + intensity` (1–2). GaussianBlur, Sharpen, HueRotate and Emboss are explicitly no-ops, so must not be offered as effective operations. The image operations use encoded byte samples and ordinary byte conversion; they do not establish linear-light, ICC-managed color, calibrated exposure, or a broad photographic grade. `BlendMode` enumerates modes, but `composite()` currently calls source-over regardless of `blend_mode`; do not claim multiply/screen/etc. execution.

The canonical source-over helper computes straight-alpha RGBA using source alpha × coverage and destination alpha, then rounds to bytes. Its source comment explicitly limits it to encoded-sample working space (not linear-light/ICC). This is the current useful transparency contract for composition.

Separate canonical procedural-texture owner: `render::surface_authoring::generate_surface_maps_into(SurfaceRecipe, side, colour, normal, orm)` validates side 8–1024, finite 0–1 factors, band limit 128 and output buffer lengths before writing. It generates seeded artistic colour/tangent-normal/ORM maps into caller buffers. `encode_surface_png_into(side, rgba, out)` emits standard RGBA8 PNG into caller buffer after dimension/capacity checks. This is not an image-effect stack or imported-height bake. `render/mod.rs:155` exposes `surface_authoring`.

## Smallest useful editable effect

Use one ordered, typed optional `Sepia { intensity: f32 }` operation in the existing Paint/detail recipe, not a new state owner. During preview/render, reconstruct a temporary image layer from the admitted immutable source plus authored paint layers, then replay the effect through the existing `apply_filter` operation before canonical PNG encoding. Keep the original encoded dependency and recipe as source; do not overwrite imported pixels or store only filtered pixels. Validate finite intensity and 0–1; preserve alpha and document encoded-sample behavior. The native control must edit the same recipe revision and existing preview/job path. This is a minimal example of an editable texture “effect stack” with actual existing math and avoids falsely promising an exposure control or invoking stub filters. Multiple operations/order, effect undo and actual output comparison still need implementation and qualified execution.

EffectCraft is a useful source workflow precedent, not a kernel dependency mandate. Saved review traces composition/layer/effect/keyframe/history/revision/render/save routes and reports the actual `effect.apply` path, but did not qualify every kernel or integrate it into Poet. Reuse its editable effect controls/history semantics as adapted in `poet-capability-adaptation-guide.md`; map its operation to this real QualiaDB CPU filter and Poet's existing detail recipe/host rather than its shell, Boa expressions, standalone IDs, transport, project serializer or GPU owner. Preserve its license notices if any selected source is later copied; no EffectCraft body/license files were reread here. QualiaDB `qualia-core-db` declares `license-file = "LICENSE"`; retain that package's required notice when adapting/copying source. Package/workspace source identity is not independently confirmed as upstream provenance.

## Integration seam, evidence, and unknowns

Root-reported current Poet route already has bounded `DetailRecipe` Paint/Procedural, source expression, immutable base digest, authorized resource decode, standard PNG encode, and canonical procedural map rendering. That is the smallest current integration seam: optionally carry a strict, versioned effect list in the same recipe and invoke the public Qualia image operation inside the existing render function. Do not add a second pixel/effect store, JS/Boa path, GPU context, exporter, or bespoke color math. Exact Poet body pin/range is owned by root's separate trace.

This review read implementation bodies only. Qualia's embedded source tests and saved Poet domain test descriptions are source evidence; they were not run during this assignment. The prior round-7 handoff explicitly says its staged tests were unrun and that actual GPU/HMC/review/reopen qualification remains outstanding. Do not infer execution, preview correctness, fresh reopen, human acceptance, or end-to-end Poet reachability from public module exposure. The current selected effect's alpha behavior on transparent RGB, behavior of source colorspace metadata, document persistence/undo, and profile-level reachability require a focused later qualification. No build/test was run.

## Source pins and inspected ranges

Current live paths are under `C:/github/qualiaDB` (not immutable revision identifiers). SHA-256:

| Path | Inspected range/symbols | SHA-256 |
|---|---|---|
| `crates/qualia-core-db/src/hypermedia_authoring/image_source_over.rs` | full file, `rgba` | `E04DC49F5E08216057BA56927EF1DCD98046179F658C17F46DD1F7E4E66CB84E` |
| `crates/qualia-core-db/src/hypermedia_authoring/image.rs` | `ImageLayer`, `BrushStroke::apply_to`, `ImageFilter`, `apply_filter`, `ImageDocument::{new,add_layer,remove_layer,active_layer_mut,composite}` (approx. 18–383) | `6D492B8DB1E1A78E2F43BD95BC0A9FD2B7DA9DCC054FCA7EAEAE8E789F0AAB49` |
| `crates/qualia-core-db/src/hypermedia_authoring/mod.rs` | full public module/re-export declarations | `A4019E11157ECC4DCFE9694E2E2487336D979C514A278892CD1C5D32AF5238AC` |
| `crates/qualia-core-db/src/render/surface_authoring.rs` | `SurfaceRecipe`, validation/error, map generator, PNG encoder (approx. 1–179); embedded tests read as source only | `148D3CA2F1586EC2C3F4C9B7815316B09A0AF6EE0D761484C903F41ED7BA88D0` |
| `crates/qualia-core-db/src/render/mod.rs` | `pub mod surface_authoring` around 155 | not separately hashed; file was read only for module exposure |
| `crates/qualia-core-db/Cargo.toml` | package name/version/license-file metadata | `0800E9E90AC14FE2C4A076B87E4F236BA09E8D0E2F67480D6594E36478F0F375` |
| `Cargo.toml` | workspace package membership context only | `BCD94075A5C27307B9C9B74B651AB1AD3EAB308E7902D4B3A4998AE20D08B9A8` |

Hashes indicate observed file content only, not semantic review of unlisted functions or tests. No commands beyond read-only inspection/hash and creation of these two assigned outputs were performed.
