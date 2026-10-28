# Botanical normal derivative: native/Vibe host trace

Status: inspected source, no canonical changes or tests. The optional QualiaDB hard-cap sweep creates an index-split render derivative with an output-to-base-vertex mapping. It is not an index-watertight source mesh. The latest existing native regression and public-caller results are recorded in `docs/qa/implementation-2026-10-27/full-scope-round1/continuation-authoring-presentation.md`; they do not demonstrate this derivative in the viewport, HMC or Vibe.

## Existing owners and the exact gap

`C:/github/poet/native/src/graphical/botanical_graph_host.rs::evaluate` builds the accepted v1 per-edge or v2 single-child-chain geometry using the QualiaDB sweep and separately computes radii. `authoring/botanical/source_edit.rs::validate_projection` checks the accepted generator mesh and refuses normals, skin and unknown geometry fields; indexed atlas regeneration is refused. `authoring/botanical_session.rs::validate` binds the accepted recipe/parts and existing GeometryHost capability/work checks. The v2 source profile currently refuses authored UV0 retention. Consequently overwriting `poet_parts.geometry` with the hard-cap output would replace the existing topological master and violate the preservation contract.

The native `controller_viewport.rs::viewport_native` chooses canonical `poet_parts` (or single-geometry fallback), and `viewport_provider.rs::viewport_cached`/`ensure_scene` key retained derived scene state by the source and current execution epoch. `ViewportCommand::RestoreSourceGeometry` invalidates and reloads the retained scene. A second representation under the same source/epoch therefore needs an explicit derived-mode identity and transition: simply substituting a value once would allow later Redraw, Pick, FramePart or restoration to use a cached derivative while appearing to address canonical source geometry. `authoring/controller.rs::compiled_assets` independently compiles canonical `poet_parts`; a preview value is not a saved HMC derivative. `assembly/admission.rs` accepts authored normals and UV0, applies UV atlas and projection downstream, and `assembly/compile.rs` writes normal/UV sections but passes no tangent stream. Existing APIs offer substantial reuse but none of these observations proves authored normal/UV continuity.

## Bounded integration contract to implement and test

1. Start from an admitted v2 recipe/current selected part under existing actor/session, source revision, epoch, cancellation and capability checks. Revalidate the recipe-to-master geometry correspondence. Refuse authored normals, skin, unknown fields and an indexed atlas until qualified mapping exists; retain every accepted input and offer source restoration. Initially permit explicit planar projection only if it can be applied to the derivative within existing assembly limits. No Vibe/Poet source mutation or new JSON representation is required for preview.
2. Decompose the exact same v2 chains and use `hard_cap_sweep_counts` and `hard_cap_sweep_normals_into` with caller-owned buffers. Before allocating, reserve aggregate original-plus-derivative vertices, triangles, frame buffers, source-index maps, per-part assembly and scratch/work bounds; account for the additional regeneration, comparisons and normal processing. Validate output mapping by reconstructing base vertices and triangles against the accepted master. Do not attribute quality or lineage to raw array indices as durable identifiers. Test core failure leaves the canonical source unchanged; kernel scratch is explicitly not transactional.
3. Introduce an explicit authorized view-only request and retained scene mode keyed by source, session/epoch, selected part and derivative profile. On a mode change, invalidate scene/frame/pick bindings and rebuild; on restoration, return to canonical scene. Recheck revocation/cancellation/context before publication. Source changes or altered selection must retire the derivative. Expose the mode visibly in native controls; prevent stale pick/frame work from being mistaken for a canonical source revision. Reuse the existing jobs/intent and authority path rather than a second state owner or rendering loop.
4. Expose the same bounded domain operation via the existing checked Vibe capability catalog/host grants if general Vibe use is required; register an actual dispatcher and native host route with truthful unsupported responses for other hosts. An API name, receipt or REPL echo is not execution. Resource budgets and partial host availability must be explicit.
5. Preview alone must not be presented as saved output. If a compiled derivative is later requested, use the existing `.10d`/HMC compile and resource closure with an explicit derivative identity and master/source lineage; do not replace the editable master. Require UV/normal/source-vertex mapping validation and an explicit tangent policy before claiming that representation is complete.

Required evidence includes straight, taper and bent selected branches; non-black shape-specific GPU difference; mode transitions, selection and stale-frame/cancellation/revocation cases; unaffected unrelated parts; atlas/authored-stream refusals; zero/over-budget refusal before promotion; original source/HMC digest unchanged; compiled derivative fresh-process reopen if implemented; and signed contributor/reviewer actions tied to actual revisions. Native human usability and species/visual acceptance remain separate evidence.

## Selected source SHA-256 pins

| Source | SHA-256 |
|---|---|
| QualiaDB `hard_cap_sweep.rs` | `02352ea05952a4c6059534017c5a6e2bce369e333ee280bc7503e01737b806f4` |
| Poet `botanical_graph_host.rs` | `b360de5701e2620540419a79483d2efb0081175249c229683bd01e7cef8cad85` |
| `authoring/botanical/source_edit.rs` | `fcd18a1c2a280fbc4d1beadc00c3ce988f65d8f22fae4eaa271dbba5f77a2afc` |
| `authoring/botanical_session.rs` | `b4c6de89980bfbc9a70a4449e6b596422406769a9a64ac9674eb01bdfc8ee1dc` |
| `controller_viewport.rs` | `2c2daf6b9fa87e913f079765859e1fd98f90b9eb061f17f9713a73e503d8cf7d` |
| `viewport_provider.rs` | `b8900509b1bf26924314262c5f105bcff6199d01d9f59a69124a17eafdc744e6` |
| `authoring/controller.rs` | `7137bdd7ee875736471e7f7fc5c72bb2683a3b55e435a44aaee4d36a1f46e669` |
| `assembly/admission.rs` | `24ddcac00db4276ba8adaaa8acb053488bdb53c7f790cc6ed361666a0ebda4b8` |
| `assembly/compile.rs` | `904ffc4f793ac5a546b4a55192c6d9e526b0b031a0ea6cc1e7a9baa2c8753b27` |
| `viewport/mod.rs` | `0eea226f30275d20ac1e83f1ae7786caae288bf4b32d5677a87093e7524b813f` |
| `viewport_controls.rs` | `f591d51808f8768163d51399562a0decc53939b7c9330f1289a2935e3eb207c3` |

These are pins for the named source only, not a whole-stack semantic audit, executed verification, or a new implementation claim.
