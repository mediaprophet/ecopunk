# Fifteen image functions: implementation and verification

**All 15 selected cases are implemented and engineering verified within the declared native image profile. Asset production remains HOLD.**

[Per-ID and acceptance-clause ledger](completion-claims.json) | [Machine-checked evidence](verification.json) | [Saved source hashes](saved-source-integrity.json) | [Native Poet executable](C:/github/ecopunk/target/debug/poet.exe).

QualiaDB owns the real filter/composite operations. Poet exposes them through strict source recipes, native controls and admitted MCP operations, reusing its signed authorisation, source history, undo/rebuild, resource and HMC owners. There is no new separate image application or required JavaScript/V8 host. Existing v1/v2 default-field source expressions remain covered; new features require Paint v3 and never silently downgrade. Package sources remain 0.0.40 working-tree builds; no release, commit or publication was made.

The original inventory now has zero of these fifteen cases deferred. Its original IDs, clauses and baseline findings are preserved. This report does not declare the complete authoring environment or any model package ready.

## Every required case

All rows have passed engineering claims. Human/native-window/domain acceptance remains unassessed. Images are small synthetic calibration readbacks, not production-quality examples.

| Required ID | Retained native GPU proof | Result |
|---|---|---|
| `PA-03-FILTER-GAUSSIAN-BLUR` | [image-gaussian_blur-gpu](gpu-attempt-1/image-gaussian_blur-gpu.png) | PASS (engineering) |
| `PA-03-FILTER-SHARPEN` | [image-sharpen-gpu](gpu-attempt-1/image-sharpen-gpu.png) | PASS (engineering) |
| `PA-03-FILTER-HUE-ROTATE` | [image-hue_rotate-gpu](gpu-attempt-1/image-hue_rotate-gpu.png) | PASS (engineering) |
| `PA-03-FILTER-EMBOSS` | [image-emboss-gpu](gpu-attempt-1/image-emboss-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-MULTIPLY` | [image-multiply-gpu](gpu-attempt-1/image-multiply-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-SCREEN` | [image-screen-gpu](gpu-attempt-1/image-screen-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-OVERLAY` | [image-overlay-gpu](gpu-attempt-1/image-overlay-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-SOFTLIGHT` | [image-soft_light-gpu](gpu-attempt-1/image-soft_light-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-HARDLIGHT` | [image-hard_light-gpu](gpu-attempt-1/image-hard_light-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-COLOR-DODGE` | [image-color_dodge-gpu](gpu-attempt-1/image-color_dodge-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-COLOR-BURN` | [image-color_burn-gpu](gpu-attempt-1/image-color_burn-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-DIFFERENCE` | [image-difference-gpu](gpu-attempt-1/image-difference-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-EXCLUSION` | [image-exclusion-gpu](gpu-attempt-1/image-exclusion-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-MASK` | [image-mask-red-gpu](gpu-attempt-1/image-mask-red-gpu.png), [image-mask-alpha-gpu](gpu-attempt-1/image-mask-alpha-gpu.png) | PASS (engineering) |
| `PA-03-COMPOSITE-OFFSET` | [image-offset-positive-gpu](gpu-offset-only-final/image-offset-positive-gpu.png), [image-offset-negative-gpu](gpu-offset-only-final/image-offset-negative-gpu.png), [image-offset-original-only-gpu](gpu-offset-only-final/image-offset-original-only-gpu.png) | PASS (engineering) |

## Executed verification

- **20 focused Core tests passed:** independent opaque/partial-alpha vectors, transparent and endpoint/neutral interactions, mask identity, vertical/diagonal/different-extent clipping, noncommuting filters, exact identities, invalid/budget refusal, legacy entry points and zero-heap caller-buffer kernels. [Final log](core-acceptance-final.log).
- **289 Poet library tests passed, 0 failed:** includes nine mounted component event cases, five new control-validation cases and four new domain cases. Twelve tests were default-ignored at that snapshot; the later isolated-offset test makes thirteen. The three new GPU cases were explicitly executed separately. [Library log](poet-library-attempt-2.log).
- **Three new native GPU workflows passed:** filters/blends/masks plus native/MCP parity, undo/rebuild and fresh-process HMC; missing-resource hold and authorised recovery plus parameter/stale/role refusals; isolated unmasked offsets and extreme clipping/reset. [Positive/HMC log](gpu-attempt-1.log), [corrected refusal](gpu-refusal-final.log), [strict DX12 offsets](gpu-offset-only-final.log).
- **Existing grayscale/invert/sepia, ordering and HMC GPU regression passed.** [Log](legacy-gpu.log).
- **Full Core library: 9,869 passed, 0 failed, 4 ignored.** [Final suite](core-owner-attempt-5.log).
- **Current native Poet executable built successfully.** [Build](native-final-build-attempt-2.log), [binary SHA-256](verification.json). Subsequent additions changed test files only.
- **All 31 saved canonical files and retained artifact hashes verified.** [Source integrity](saved-source-integrity.json), [evidence verifier](verification.json). Recheck using the bundled Python and `tools/poet-foundation/image-completion/verify_results.py gpu-attempt-1`; it performs no builds.

CUDA hardware is unqualified. The original broad run could not load its driver; the corrected optional-backend path returns `GpuUnavailable`, and existing optional tests can return early. A passing optional test does not establish CUDA execution. The image GPU checks explicitly use DX12.

Peer delivery timed out in an earlier full run, passed in isolation and passed in the final full run. Its cause remains an open reliability observation. [Original full run](core-owner-attempt-3.log), [isolated run](mesh-isolated.log), [bounded diagnosis](../../../../tools/poet-foundation/image-completion/work/mesh-review.md).

## Inspectable HMC and source outputs

[Compiled HMC](gpu-attempt-1/image-completed.hmc), [saved HMC](gpu-attempt-1/image-saved-gpu.hmc), [fresh-child resaved HMC](gpu-attempt-1/image-child-resaved-gpu.hmc), [source/pixel witness](gpu-attempt-1/image-source-witness.json), [child recovery](gpu-attempt-1/image-child-recovered.json), and [output hashes](gpu-attempt-1/image-completed-witness.json). These use synthetic fixture identifiers/test authority and retain original, mask, derived image data and metadata. They are engineering fixtures, not complete species or thing embodiments.

## Saved changes and review history

Source owners: `C:/github/qualiaDB/crates/qualia-core-db/src/hypermedia_authoring/image{,_filters,_composite}.rs`; Poet's `native/src/graphical/surfaces/detail`, `authoring/detail_controls.rs`, resource resolution and `mcp/texture_effects.rs`. Exact real file paths/hashes and test files are in the source receipt. Every install uses reviewed preimages and retains backups.

[Core save](applied-core.json), [Poet save](applied-poet.json), [expanded vectors](applied-core-vectors.json), [identity/extent vectors](applied-core-identity-vectors.json), [native fixture correction](applied-poet-repair-1.json), [retained evidence](applied-poet-evidence.json), [refusal recovery](applied-poet-refusal.json), [isolated offsets](applied-poet-offset-evidence.json), [offset readbacks](applied-poet-offset-retention.json).

Broader verification also repaired the existing [FIEL test-module path](applied-core-owner-repair.json), [carrier fixture borrow](applied-core-carrier-repair.json), [optional CUDA absence handling](applied-cuda-guard.json), and shared-tree [q42 CRC import](applied-q42-crc-repair.json)/[HMAC constructor qualification](applied-hmac-trait-repair.json). The last two preserve source introduced during verification; no dependency or format change, cryptographic algorithm or key/input handling was altered. The q42 catalogue's broader wiring/bounds were not assessed by this image job.

Failed attempts remain. The first Poet compile caught a missing fixture comma. The first GPU batch passed its positive workflow but failed a refusal fixture that inspected held execution; the repaired test explicitly proves native/MCP refusal, unchanged source/history, authorised rebuild and recovered GPU output before testing context reuse. The positive fixture is byte-pinned unchanged and its passing evidence reused. No required assertion/case was removed. [Independent review](review-notes.md) records the corrections and adjacent API debt. Candidate claims under `work/` are historical drafts; this report and ledger supersede their status and pins.

## Authoring behavior and qualified limits

In an admitted native project with an imported Paint resource, load the accepted detail recipe. The existing Poet panel exposes seven filter kinds, ten blend choices, signed X/Y offsets, rectangle masks and immutable HMC masks with explicit raw red/alpha channel selection. Add/Replace stores stroke forms; Apply submits through the existing authority/source owner. Invalid raw numbers remain visible and refuse submission. Each layer keeps its pending stroke form; explicit stroke reorder/removal loads the corresponding stored form.

Agents use `poet_texture_effects_edit` for effects and `poet_texture_layers_edit` for layers with current connection-local context; each preserves the other half of the recipe. Valid-shaped mutation attempts consume context on subsequent denial. Parameter-shape refusal preserves it. Failed host/resource evaluation can hold execution with accepted source/history intact; rebuilding is explicit and authorised.

Operations are artistic encoded RGBA8. Core kernels accept checked budgets and caller buffers; admitted cold authoring may allocate bounded storage. Poet's current imported profile is opaque square sRGB base colour, side 8–1024, with raw red/alpha masks. This does not set the eventual Ecopunk quality ceiling. Larger/transparent/HDR master authoring, colour management and physical/biomedical accuracy require their own qualification. RGBA8 cannot encode exact 0.5; fixed 127/128 neutral vectors retain HardLight's one-LSB equation bias.

## Delegation and remaining foundation gates

Disjoint staged Sol lanes handled filters, composition, domain, controls, component and signed workflow work; Luna handled reuse/metadata. The controls lane recovered from Sol capacity failure using Sol 6.1. Supervisor review corrected operator-branch, fixture, editor-selection and evidence gaps. Separate Sol-medium lanes reviewed peer reliability and fixed optional CUDA availability. Actual model/effort records are in `docs/planning/biosphere-work-packets.json`; token/billing amounts are unknown. Root reviewed canonical saves and kept Cargo serial.

The 15-case engineering ledger is complete. Native-window/human usability and domain acceptance, larger production profiles, adjacent legacy allocation-API hardening (`PA-03-API-ALLOCATION-HARDENING`) and the peer observation remain explicit. Original IP-03 C1–C8, the 120 packets and every model-production obligation remain governed by the consolidated plan. The repository instructions are not a claim that the complete per-model runtime acceptance gate is installed.

**Film, open-world game and cartoon assets: HOLD / not produced. No whole-stack readiness, complete model/package, human review or universal superiority claim.**
