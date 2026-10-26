# Bounded independent W2 integration source review

2026-10-26. Domain worker reviewed root-owned staged glue; no source changed, no Cargo/tests/build/GUI ran. Reviewed entire staged mcp/texture_effects.rs and surfaces/inspection.rs, and compared inspection.rs against canonical. The only three additional implementation bodies inspected were canonical mcp/material_motion.rs; selected mcp/mod.rs expected/host/receipt_authority/outcome bodies; selected cooperation/editor.rs authoring_view/authoring_edit/inspect_surface bodies. Also searched SurfaceInspection struct-literal locations without opening further bodies. No wider auth-stack review is implied.

## Finding requiring repair

[P2] Valid-shaped early refusal retains context. Staged `native/src/graphical/mcp/texture_effects.rs:69-72` invokes `backend.expected(&request.context)?` before clearing `backend.context`. `expected` can refuse stale source/execution, stale/revoked publication authority or a mismatched token while leaving the connection context present. This differs from frozen W2 contract: any valid-shaped mutation attempt consumes its connection context. Existing material_motion uses the same older ordering; it does not override the W2 requirement. This is a consumption/retry-contract mismatch, not evidence of an authorization bypass.

Reproducer: read a valid context, then submit structurally valid effects with a different context token. The first request returns stale_context; retry using the previously admitted token can still reach the target operation. A revoked/stale source variant similarly leaves the slot present, although native current checks still refuse source mutation.

Remedy after argument/profile preflight: capture `let expected = backend.expected(&request.context);`, set `backend.context = None;`, then unwrap with `let expected = expected?;`. This preserves malformed argument retention and consumes valid-shaped attempts even when expected refuses. Add/retain an integration assertion that a second call with the originally admitted token gets context_required after this refusal.

## Other review results and integration obligations

- Same operation and scope: edit calls existing AuthorizedEditorHost.authoring_edit with AuthoringEdit::TextureDetail and the expected SourceRef. The authorized wrapper delegates through self.write ExecuteSource; the MCP adapter creates no parallel state or weaker permission route. View and inspection use authorized ReadSource wrappers. Final outcome uses existing authority/source/execution receipt checks.
- Preservation: accepted AuthoringView supplies the PaintRecipe. Every part sharing target material must be Paint and equal; absent/procedural/conflicting cases refuse. Only version and effects are modified before existing TextureDetail submission. Base SHA, layers/strokes and other recipe fields are retained. Broader opaque source preservation remains the existing authoring source owner; its bodies were not re-audited here.
- Preflight: deny_unknown_fields on requests, nested strict effect schema, native max8/finite intensity checks precede context operations. Disabled parameters are checked by shared domain helper. Source/refusal behavior is not established by source inspection alone.
- Inspect output: source and authority are checked after both view/inspection calls. Output witness comes from resolved material.base_color_texture, after existing preview_input resource access/decoding succeeds. No fabricated preview/pixel result is introduced. The delta only adds optional resource digest/summary; it does not prove GPU rendering, human acceptance or production quality. The digest is an encoded resource witness, not permission or image-quality evidence.
- Compile integration: adding SurfaceInspection.texture_sha256 requires updating canonical jobs_tests.rs:41 struct literal. Search found that single non-constructor fixture site; root owns its mechanical fix. MCP registration and module wiring also remain root-owned. No other API mismatch was apparent from the bounded bodies; compiler execution is still required.

## Source pins

SHA-256 of inspected/sampled files:

| File | SHA-256 |
|---|---|
| staged mcp/texture_effects.rs | C37B034B40108ADBB28BFA5509210118F38DB2DF6065B7EA39EF779C1D0A65FA |
| staged surfaces/inspection.rs | 8B80D5B8BBD4BFC82AA4343DEBBDF4B3B0953AAA4608C94AB856A1523BE2F28D |
| canonical surfaces/inspection.rs | 8A14B6926F2AB2255F29B7592ECA14A8C1EE7DC80EDBA1119CBF5AB3E3A11BDB |
| canonical mcp/material_motion.rs | 1573F858739A22E23F609D1D22CD88B8677E232E88045CEA7EBF5E75CF84B3CB |
| canonical mcp/mod.rs | E669DDDF57310B9C54A28EBACA6E60B39D2FFA3776776B76D028E6B68110FE2E |
| canonical cooperation/editor.rs | B34E76F6A0288A1B12936904AF9796168AC6757628A6B68EACF6308DDF93798D |

Canonical mcp.rs probe returned path-not-found; immediately used actual mcp/mod.rs. Expected git diff --no-index exit 1 records a textual delta, not an execution failure. Source pins alone do not qualify runtime behavior. This review does not qualify models/packages, HMC fresh reopen, native current-window/GPU behavior or full IP-03 acceptance.
