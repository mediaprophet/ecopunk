# Implementation of all 15 deferred image cases

2026-10-26. User explicitly requests full implementation of the fifteen source-pinned deferred cases, using scoped agents. This authorizes their owning QualiaDB image modules and their Poet authoring integration; older investigation-only sibling scope does not prohibit this concrete new request. Root alone applies reviewed preimage-pinned changes with sandbox approval and owns shared files and serial Cargo. No commits/publication/payment or production assets. Preserve existing work and all old receipts. Asset production remains HOLD.

## Ownership and rounds

Existing IP-03, PA-03/06/11/12, PC-06, TOOL-06 govern. Core owns the real operations, Poet owns strict source recipes and native controls/MCP projections. Do not install another image engine/app or JS host. Reuse existing Core/CV operators where semantics match; investigate only the selected unresolved seams.

- Sol high, filters: staged NEW Core `crates/qualia-core-db/src/hypermedia_authoring/image_filters.rs` and `crates/qualia-core-db/tests/image_filters_completed.rs` only. No shared image.rs edits. Root registers/re-exports module under image.rs; standalone integration tests import the public image API.
- Sol medium, composition: staged NEW Core `crates/qualia-core-db/src/hypermedia_authoring/image_composite.rs` and `crates/qualia-core-db/tests/image_composite_completed.rs` only. No shared image.rs edits. Root registers/re-exports module; integration tests use public APIs. Root may include these tests from a Poet native test wrapper to qualify the actual dependency closure without unrelated workspace builds.
- Luna medium: bounded reuse/notice/API trace, separate metadata files; then release slot for a Sol-high Poet controls worker after the DTO contract is frozen.
- Root: shared Core image.rs registration/error/limits/wrappers; Poet recipe/render/resource/MCP integration; mechanical compatibility fixtures; review and canonical install; all Cargo/GPU/recovery checks. At most root plus three workers. Staged outputs are disjoint; canonical build inputs freeze during checks.

Round-two delegation, after the composition lane finishes: a Sol-high Poet domain worker owns staged `native/src/graphical/surfaces/detail/{recipe.rs,mod.rs,render.rs}` plus optional NEW `recipe_features.rs` / `completed_tests.rs`, and `surfaces/{resolution.rs,texture_inputs.rs,resources.rs}`. Root relinquishes those staged candidates to this lane and retains shared Core, MCP, native controls' registration, compatibility fixture edits and integration/check ownership. Its staging directory is `work/poet-domain/`; no worker edits root's integration staging or canonical source. The controls worker continues owning only its separate authoring files.

After those implementation lanes finish, a Sol-high workflow engineer owns only NEW staged `native/src/graphical/native_project_image_completed_tests.rs` under `work/workflow/`; root registers it as a child of existing `native_project_texture_effect_gpu_tests.rs` to reuse real signed GPU/input helpers. A Sol-medium component engineer owns only NEW staged `native/src/graphical/authoring/image_completed_component_tests.rs` under `work/component/`; root registers it below existing `texture_effect_component_tests.rs` to reuse the actual mounted-event harness. No overlapping files; workers may edit isolated staging while root checks already-frozen canonical inputs, but never canonical check inputs or shared root staging.

## Core shared API (root integrates)

In `image.rs` define/re-export:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageError { Dimensions, Storage, Parameters, Budget, Output, Scratch }
#[derive(Clone, Copy, Debug)]
pub struct ImageLimits { pub max_pixels: usize, pub max_work: usize, pub max_bytes: usize }
// Default: 4*1024*1024 pixels, 1024*1024*1024 work visits, 256*1024*1024 bytes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageFilterOp {
 GaussianBlur { sigma: f32 }, Sharpen { sigma: f32, amount: f32 },
 HueRotate { degrees: f32 }, Emboss { azimuth_degrees: f32, strength: f32 }
}
pub struct FilterPlan { pub scratch_pixels: usize, pub work: usize, pub bytes: usize }
```

Filter module exports `filter_plan(width:u32,height:u32,op:ImageFilterOp,limits:ImageLimits)->Result<FilterPlan,ImageError>` and `filter_rgba8_into(src:&[[u8;4]],width:u32,height:u32,op:ImageFilterOp,limits:ImageLimits,out:&mut[[u8;4]],scratch:&mut[[f32;4]])->Result<(),ImageError>`. Dimensions/product, storage, finite parameters, output, scratch and admitted work/bytes validate BEFORE any output mutation or allocation. Loops allocate nothing; caller owns buffers. Cold wrappers may allocate only after checked plan. Gaussian/Sharpen scratch is at most two image-sized float RGBA planes; other operations need no scratch.

Sigma is standard deviation in image pixels, finite 0..16, radius ceil(3*sigma), normalized separable Gaussian, clamped edge coordinates. Sigma 0 is exact identity. Gaussian filters premultiplied encoded RGB and alpha, unpremultiplies once and rounds/clamps bytes; zero alpha yields zero RGB (except exact identity returns original bytes). Sharpen is unsharp mask, amount finite 0..4; preserve original alpha, use alpha-aware blurred RGB, clamp/round; zero amount or sigma is exact identity. Hue uses SVG hue-rotation color-matrix convention, degrees finite -360..360, preserve alpha; complete turns exact identity, achromatic input invariant to byte rounding. Emboss uses central luminance differences (weights .2126/.7152/.0722), +X right/+Y down, clamped edge, azimuth degrees finite -360..360; grayscale relief `128 + cos(angle)*(Lright-Lleft)/2 + sin(angle)*(Ldown-Lup)/2`, clipped to bytes and blended with original by strength finite 0..1. Preserve alpha; zero strength exact identity. These are artistic encoded-sample operations, not ICC/linear-light or physical surface measurements.

Compositor exports `validate_document(doc:&ImageDocument,limits:ImageLimits)->Result<usize,ImageError>` (returns output pixels) and `composite_into(doc:&ImageDocument,out:&mut[[u8;4]],limits:ImageLimits)->Result<(),ImageError>`. Validate all layers, including hidden, finite opacity 0..1, exact layer pixel count/mask length, nonzero bounded dimensions, checked aggregate visits and byte budgets before output changes. Clip signed offsets using wide arithmetic; layer mask is in layer-local coordinates. Mask coverage/255 multiplies source alpha and layer opacity exactly once. No blend-mode branch may silently act as Normal.

All nine non-Normal modes follow W3C Compositing Level 1 separable blend definitions, including its piecewise SoftLight and denominator endpoint rules for dodge/burn. Combine blend color with straight-alpha source-over using the backdrop-alpha interpolation from that specification; transparent backdrop must not contaminate source RGB. Normal with no mask/zero offset must call the existing source_over::rgba operation to preserve legacy byte rounding. Per-pixel routines allocate nothing. Source masks/pixels never mutate. Reuse current authoring owner; do not flatten editable layers into replacement source.

Root keeps existing valid legacy API calls source-compatible. Add checked cold wrappers `try_apply_image_filter(layer:&mut ImageLayer,op:ImageFilterOp,limits:ImageLimits)->Result<(),ImageError>`, `ImageDocument::try_composite(limits)->Result<Vec<[u8;4]>,ImageError>` and caller-output method. Existing legacy `apply_filter` maps previously no-op variants explicitly: blur sigma=4*intensity, sharpen sigma=1/amount=intensity, hue degrees=360*intensity, emboss azimuth=135/strength=intensity. Existing intensity clamping remains, with invalid input rejected by checked APIs; the compatibility wrapper documents finite valid input and bounded allocation preconditions. The old `composite()` delegates to checked composition for valid documents; malformed callers use the new fallible API rather than receiving invented transparent output. Public typed checked operations carry the qualified refusal contract. Record the intentional legacy no-op-to-real-output behavior change.

## Poet version 3 integration contract

Version 1/2 recipes keep their existing byte/source expressions when new fields are absent/default; unsupported versions/unknown fields refuse. Version 3 enables new filters and layer features. No silent version downgrade: legacy controls add steps using max(current version, required version).

New effects match Core typed fields exactly. Add optional/default-skipped `blend_mode` (all ten names in snake_case), `offset:[i32;2]` and `mask` to existing PaintLayer. Mask supports a bounded rectangle (origin, size, coverage byte) and an immutable HMC texture reference (SHA-256 plus channel red/alpha), in layer-local coordinates. Dense arbitrary masks remain supported by Core; source mask images are resources, not million-element inline source arrays. Resource mask bytes and metadata must survive source/HMC compile/reopen alongside original base and derived output. Preserve all existing layers/strokes and opaque part/material fields.

Native controls and admitted MCP must edit the same versioned recipe through existing TextureDetail/SourceSession/authority/history; add no state store. UI fields expose all four filters with explicit units, all blend modes, signed offsets, rectangle/resource masks, order/enable/remove. Retain invalid raw input and stale drafts; refuse applying old numeric values. Strict typed MCP edits expose layers/features as well as the effect list, guard current source/authority, and preserve immutable input resources. Effects-only edits retain layer data and choose required version, never downgrade v3.

Frozen DTO names for the separate controls worker: `TextureEffect` adds `GaussianBlur {sigma:f32}`, `Sharpen {sigma:f32,amount:f32}`, `HueRotate {degrees:f32}`, `Emboss {azimuth_degrees:f32,strength:f32}`. `PaintLayer` adds default-skipped `blend_mode:LayerBlendMode`, `offset:[i32;2]`, `mask:Option<LayerMask>`. `LayerBlendMode` has Normal plus the nine existing Core names, derives serde snake_case and Default=Normal. `LayerMask` is strict internally tagged snake_case: `Rectangle {x:u32,y:u32,width:u32,height:u32,coverage:u8}` or `Resource {sha256:String,channel:MaskChannel}`; `MaskChannel::{Red,Alpha}` is snake_case. `PaintLayer::default()` is provided for compatibility struct updates. Root provides `required_paint_version(&PaintRecipe)->u32`; use `p.version.max(required_paint_version(p))` after draft changes. UI should use required version 3 for these new features, retain existing v1/v2 behavior otherwise. Rectangle bounds are validated against side, mask resource must match side/extent and explicit channel; hashes confer no authority.

## Completion evidence

Encoded RGBA8 cannot represent an exact 0.5 channel. The blend-neutral checks use fixed vectors for both neighbouring byte values, 127 and 128: Overlay/SoftLight round to the selected base bytes, while HardLight has the equation's documented one-LSB bias. Do not claim that byte 128 is an exact mathematical neutral for every mode. Transparent-source, opaque endpoint, black/white interaction and mask identities are checked separately.

Each of the fifteen inventory IDs receives implementation path, real API/caller, independent known-vector/alpha/boundary/identity/refusal tests and actual command/exit. Core zero-allocation kernels use caller buffers; cold allocation is bounded/admitted. Include malicious sizes, signed offset extremes, invalid/hidden masks/parameters, transparent pixels, tiny images, noncommuting filters and partial-alpha blend equations. No implementation-shaped test alone establishes correctness.

Poet tests must prove actual component edits and matching admitted agent operations, undo/rebuild, all fifteen cases retained in source, source-to-pixels, original/mask/derived HMC closure and metadata, fresh-process reopen/rebind. Qualify actual native GPU effect/layer changes and recovery; GPU absence fails. Retain synthetic evidence only. Run Cargo serially through the existing root lease and the actual native Poet dependency closure; use a direct integration test wrapper if Core's wide workspace is unrelated. All failures get concrete repair; do not mark inventory complete before executed verification. Native-window/human/domain and whole-product acceptance remain distinct.

Primary convention references (read 2026-10-26): https://www.w3.org/TR/2024/CRD-compositing-1-20240321/ and https://www.w3.org/TR/2018/WD-filter-effects-1-20181218/. These are equation conventions, not a claim of full browser/filter-spec conformance or a browser dependency.

## Bounded repairs discovered by mandatory broader verification

Core's AGENTS requires its full library test suite. Testing it as an external Poet dependency is refused by Cargo, so the owner workspace is checked serially with its frozen lock. This exposed an existing FIEL test-module path error and a mutable-borrow error in the source-carrier integrity fixture. Root reviewed/saved their minimal test-only corrections; runtime parsing and corruption oracles remain unchanged.

The resulting 9,871-test run reported 9,855 passed, 12 failed, 4 ignored: eleven optional-CUDA cases panic loading a missing driver, and peer delivery timed out once but passed in isolation. A bounded Sol-medium worker may stage a non-panicking existing CUDA-constructor availability guard and an admission/refusal test. It cannot install drivers, change GPU oracles, skip tests or claim CUDA hardware qualification. Root alone saves guarded preimages and qualifies the change. The peer failure is investigated read-only first; an isolated pass does not close its cause. Preserve failure evidence and report actual broader status separately from the fifteen image functions.
