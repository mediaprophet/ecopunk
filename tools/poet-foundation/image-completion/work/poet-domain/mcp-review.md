# Staged MCP texture edit review

2026-10-26. Scope: read-only review of `work/integration/poet/native/src/graphical/mcp/texture_effects.rs` lines 1–122 and `mod.rs` lines 1–420 against the frozen Poet domain DTO and existing native edit pattern. No code, Cargo, or test execution in this review.

## Result

No integration blocker found in the staged MCP glue.

- The advertised effect schema covers all seven `TextureEffect` kinds with the four new typed fields and limits. The layer schema covers all ten `LayerBlendMode` values, signed i32 offsets, nullable rectangle/resource masks, and the existing layer/stroke fields. Rust argument decoding uses strict DTO structs and the recipe validator; unknown fields and unsupported kinds refuse.
- A layer edit constructs a version-3, side-1024 probe with unbounded work/byte counters but the normal layer/stroke/point limits. This checks shape and universal numeric ranges before looking up authority; actual target side and normal budgets are checked at line 117 after current target lookup. Valid-shaped edit attempts consume the connection context at lines 98–100, including later target/admission refusal.
- Target lookup at lines 103–113 requires every part sharing `material_id` to have the same existing imported Paint recipe. Effects edits change only `effects`; layers edits change only `layers`. Base SHA, other recipe fields and the opposite editable list remain in the cloned Paint value. Line 115 uses `max(existing_version, required_paint_version)` and never downgrades v3.
- Both edit tools call the existing `AuthoringEdit::TextureDetail` host operation with `expected.source`, then its authority checked outcome. Resource masks enter only as immutable SHA references in the strict recipe; no MCP supplied filesystem path, payload or authority is introduced. The domain resolver's referenced-HMC input admission must be exercised by root's native closure tests.
- `mod.rs` registers `poet_texture_layers_edit` and dispatches it together with effects inspect/edit. The inspect path still checks current source and authority before returning detail/output SHA.

## Minor schema precision

The JSON Schema permits zero rectangle width/height and an all-zero 64-character mask hash at `texture_effects.rs:21–22`; the strict recipe probe rejects these before consuming context. This does not admit an invalid source or bypass native authorization, but clients may receive `invalid_arguments` for a value that appears schema-valid. If exact advertised bounds are required, set rectangle width/height minimum to 1 and exclude the all-zero digest in the pattern. Actual-side rectangle bounds remain target dependent and correctly follow authority.

Execution of target/authority/HMC behaviour remains unverified by this read-only review.
