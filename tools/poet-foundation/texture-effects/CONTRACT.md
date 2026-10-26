# POET-TEXTURE-W2 bounded implementation contract

2026-10-26. User explicitly requested Wave 2 implementation and serial verification. Existing IP-03 / PA-03, PA-06, PA-11, PA-12, PC-06 and TOOL-06 govern; this is not a replacement phase order. Asset production stays HOLD. Sibling writes are applied only by root after exact preimage review and sandbox approval. No QualiaDB/Aether/Etalon source changes, Cargo manifests/locks, new renderer, scripting host, state owner, app shell, assets, commits or publication.

## Existing operation and selected outcome

Existing Poet `AuthoringEdit::TextureDetail` -> `source::edit_source_with_layers` -> canonical source session handles native authoring. `surfaces/detail/{recipe,render}.rs` preserves immutable imported base SHA, ordered paint layers and source; `surfaces/resolution.rs` resolves authorized HMC bytes, generated texture digests and retained input dependencies. `detail_controls.rs` already edits paint/procedural recipes. Keep those controls and their accepted/draft separation.

Public existing Qualia operation: `qualia_core_db::hypermedia_authoring::image::{ImageDocument,ImageLayer,ImageFilter,apply_filter}`. Reuse actual Core filters, source-over and PNG owner. Photo/Effect references provide editable property/order/enable/history workflow semantics; no source shell, Boa, egui, math or project store is copied. See `work/report.md` and saved adaptation/media/EffectCraft reviews. This is a bounded post-composite effect stack, not adjustment-layer masks, RAW development, ICC/linear-light grading or complete compositor parity.

## Frozen shared API

Extend existing `PaintRecipe` (same detail slot) with:

```rust
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub effects: Vec<TextureEffectStep>,

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextureEffectStep { pub enabled: bool, pub effect: TextureEffect }

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag="kind", rename_all="snake_case", deny_unknown_fields)]
pub enum TextureEffect { Grayscale, Invert, Sepia { intensity: f32 } }
```

The public effect enum uses an explicit `Deserialize` adapter with strict empty-struct wire variants for Grayscale/Invert. Derived deserialization of internally tagged unit variants accepted extra fields in the first actual test run; the repaired implementation and unchanged refusal cases qualified the strict wire contract. Do not restore that failed decoder while following this API sketch.

Re-export both new types and `validate_effects(&[TextureEffectStep]) -> Result<(), String>` from `surfaces/detail/mod.rs`; the latter owns shared pre-context step admission. At most 8 effect steps; Sepia intensity finite 0..1, including disabled steps. Grayscale and Invert are full-strength operations because Core ignores intensity for them; no fake strength slider. Legacy Paint version 1 accepts only empty/absent effects and retains its previous serialized expression when empty. Version 2 admits the effect list (including empty); UI/agent addition promotes explicitly to version 2. Procedural version 1 unchanged. Unknown kind/fields/version refuses before mutation. No silent migration or lost layer/stroke/opaque part/material data.

Replay existing paint layers, composite once, then apply enabled effects in authored order to one temporary `ImageLayer` using Core `apply_filter`; keep alpha, original bytes/digest and recipe. Generate derived PNG using existing Core encoder and digest logic. Count all steps conservatively in pixel/work admission before allocation; account extra temporary RGBA layer/copy. Rendering must not mutate the recipe or authorized imported resource. Disabled steps retain their state and parameters. Existing HMC dependency closure must retain original input and derived output.

Root adds MCP `poet_texture_effects_inspect(context,part_id)` and `poet_texture_effects_edit(context,material_id,effects)`. The edit reads the existing accepted Paint recipe, changes only its effects/version, and submits `AuthoringEdit::TextureDetail` through the same native host. Require an existing Paint recipe; refuse conflicting shared recipes/procedural/absent targets, malformed steps, unknown fields and stale/denied/revoked context. Preflight args before consuming context; any valid-shaped mutation attempt consumes its connection context. Inspect returns actual recipe, source revision, immutable base SHA and actual resolved output digest; no inferred authority from IDs/digests. Root extends existing selected inspection with optional resolved base-colour digest for this output witness. No image polling transport.

## Disjoint ownership

All worker output goes in `tools/poet-foundation/texture-effects/work/<lane>/` with original canonical relative paths, preimage hashes and handoff.json. Workers read siblings but do not write canonical sources or shared plans. Root owns all integration, registration, mechanical legacy struct-literal fixes, MCP adapters, SurfaceInspection output witness, source receipts and serial Cargo lease.

| Lane | Model/effort | Exact owned canonical candidates, staged only |
|---|---|---|
| domain | gpt-6.1-sol medium: fixed API and isolated Core adapter | `native/src/graphical/surfaces/detail/{recipe.rs,render.rs,mod.rs}`; new `effect_tests.rs` under that directory |
| controls | gpt-6.1-sol high: native reactive draft/state interactions | `native/src/graphical/authoring/detail_controls.rs` and optional new sibling `texture_effect_controls.rs` only (root registers module if needed) |
| integration-tests | gpt-6.1-sol high: authority/resources/fresh process | new `native/src/graphical/native_project_texture_effect_tests.rs` only; root adds parent module declaration |

Luna medium completed the focused public-API/source metadata trace in an earlier slot. Root plus at most 3 workers active. Workers do not run Cargo, build/export, launch GUI or call providers. No canonical edits while root checks are running.

## Required checks and acceptance limits

- Domain cases: legacy source-expression preservation; strict version/field/parameter/size refusal; direct expected pixels for actual filters; ordered non-commuting operations, disabled/removed effects, layer retention, immutable base; budget before allocation; PNG digest determinism and canonical decode.
- Control cases: add/change/enable/reorder/remove effects; invalid edits retain draft; accepted source/context changes retain local form with explicit reload, unsubmitted source draft remains guarded; existing layer/procedural controls preserved.
- Integration: signed fixture native dispatch and matching admitted MCP edit share actual source; inspect recipe/base/output SHA; undo/redo/rebuild; actual encoded pixel difference; original/derived HMC resources and metadata survive save and fresh-process reopen/rebind; stale/malformed/observer/revoked attempts preserve source/history. Follow current single-active-session contract; no simultaneous-authority claim.
- Root reviews all patches, applies preimage-pinned files with backups, runs focused tests and relevant existing texture/motion regressions serially, then builds native Poet. Record every exit, failure/repair, hashes, remaining IDs and actual output. No mock output may count as GPU/visual success.
- Actual human review, current-window/GPU rendering, biological quality, complete models and full IP-03 C1-C8 remain unassessed unless separately performed. Fixtures are engineering inputs only. No production species/thing package is made in this wave.
