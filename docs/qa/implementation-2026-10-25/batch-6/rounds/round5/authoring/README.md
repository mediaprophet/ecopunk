# Round 5 staged native assembly authoring

Status: saved staged source, **uncompiled and unexecuted**. No live sibling,
asset, credential, Cargo or production-source changes were made by this agent.
Assets remain HOLD. Root must review exact manifest hashes, install authorized
targets and run the relevant compiler/tests. The manifest pins four existing
before/after files and four new files. Every focused module is under 500 lines.

## Source-backed contract

The existing native path is retained: `authoring/controls.rs::send` calls
`native_dispatch::run` with `Action::AuthoringEdit`; the existing worker,
canonical intent owner and `authoring/controller.rs::edit_assembly` retain source
freshness, mutable binding and reconciliation checks. Root owns the shared
dispatch/intent decoder updates required by the new enum variants. No new
cooperation, jobs, session or provider route is defined here.

`source::edit_source` constructs one bounded pinned Vibe 0.1 cell. The existing
controller submits it once. Functional map/filter/push and record member access
are existing canonical evaluator operations, inspected in
`qualiaDB/crates/vibe/src/eval/call.rs` and `eval/expr.rs`; this patch does not
introduce a language dialect or per-script build.

| Intent | Explicit behavior and refusal |
|---|---|
| Reparent/detach | Changes structural `parent` only, preserving local translation/rotation/scale byte-for-byte as values. World placement can change. All current parents must exist and containment must be acyclic; self/descendant/missing parents fail before source execution. |
| Remove leaf | Removes only a selected leaf. Children and the final part cause refusal. No cascade or inferred physical removal event is created. |
| Replace geometry | Replaces the selected part's geometry record using one typed canonical operator invocation. All other part fields/material remain. |
| Add geometry | Creates one unattached identity-TRS part with distinct part and semantic identifiers, copying the selected existing part's complete material record. It does not infer a design/instance relationship or rights. |
| Unique material | Copies the complete selected material record under an unused positive identifier and changes only that part's material. Reselecting refreshes the factor inspector. |
| Existing transform/shared factors | Reconstructs each affected record from its own exact field set. Different part/material extension schemas cannot overwrite each other. Unrepresentable field names refuse the edit. |
| Existing parameter rebuild | Explicitly regenerates recipe parts and replaces overrides, as the existing UI states. Untouched values retain integer type/precision or direct references for other values. Float source uses shortest round-trip formatting, replacing the former nine-decimal rounding. |

Part identifiers use the existing editable ASCII profile, 1–128 bytes,
alphanumeric plus `_ - . : /`. New semantic/material identifiers use an explicit
positive signed-range control profile (`1..=i64::MAX`); this is a bounded UI/API
choice, not a Vibe parser restriction. Duplicate part/semantic identifiers and
unknown current parent schemas fail closed. Assembly size is bounded to the
actual admission limit, 256. Generated source remains limited to 8192 bytes;
shared edits group identical record expressions. One shared schema uses a
single material-identifier predicate, keeping source size/depth independent of
the number of affected parts; mixed schemas group their selected identifiers.
Unusually large mixed-schema edits can be refused without dropping fields.

Supported typed recipes are sphere, torus and straight circular profile sweep
along local Z. Dimensions must be finite and in `[1e-6,1e6]` metres; segment
counts are whole integers in `[3,256]`; torus minor radius must be smaller than
major radius. Sweep caps are explicit. These are control working bounds, not a
master-quality ceiling. `GeometryRecipe` has strict internally tagged serde
decoding and a `pub(crate) validate()` method for root-owned ingress.

The recipes invoke existing `ComputationalGeometry.uv_sphere`, `.torus` and
`.profile_sweep`, inspected in `graphical/spatial_host.rs` and
`crates/spatial-authoring/src/geometry.rs`. `GeometryHost` owns actual grants,
remaining dispatch/work, vertex, triangle, profile and scratch budgets; authored
`requires` cannot grant authority. A failure after invocation is subject to the
existing reconciliation behavior, not a claim of effect rollback. Transform,
material and containment edits invoke no geometry operator.

## Expected exact tests (not run by this agent)

All new names are prefixed `graphical::authoring::structure_tests::`:

- `reparent_detach_and_remove_leaf_preserve_local_trs_and_unrelated_values`
- `assembly_mutations_refuse_cycles_missing_parents_and_nonleaf_removal_without_execution`
- `unique_material_preserves_complete_material_and_only_selected_assignment`
- `field_preserving_edits_keep_distinct_extension_schemas_and_refuse_unsafe_names`
- `bounded_geometry_recipes_add_and_replace_through_actual_grants_and_one_dispatch`
- `geometry_edits_refuse_denied_effect_and_exhausted_work_without_advancing_source`
- `authoring_inventory_and_recipes_refuse_duplicates_injection_and_nonfinite_arguments`
- `structural_source_revisions_keep_stale_guard_undo_and_rebuild_lineage`
- `parameter_rebuild_retains_unsigned_precision_and_unknown_untouched_values`
- `shared_material_edit_groups_256_parts_without_source_or_expression_depth_growth`
- `shared_material_delta_matches_unsigned_and_mixed_identifiers_without_losing_extensions`

Relevant existing regressions in `graphical::authoring::tests::`:

- `every_generated_edit_is_a_canonical_program_with_a_record_result`
- `part_material_parameter_edits_are_real_source_revisions_with_stale_safety`
- `compiled_hmc_replacement_preserves_opaque_resources_and_native_companion_graph`

The one-dispatch geometry test admits all three supported kernels through the
actual native host, then proves another invocation is denied without a reset.
The low-work and denied-grant tests require unchanged session revision/parts.
The extension tests deliberately use source-session-only records: they prove
field preservation, not extended preview or package admission. Stale source,
undo and accepted-source rebuild use the existing canonical editor host and
temporary synthetic fixture. No GPU output or human review is fabricated.

## Remaining qualification and scope

Native `materials::material_from_value` currently rejects texture/physical
extensions and admits only the explicit factor material profile. This patch
preserves fields, and does not claim those richer schemas are rendered or
admitted. The Q42 compile path produces source/mesh digests and geometry
knowledge; full stable part/design/instance graph and derivation qualification
remains open. World-preserving reparent, arbitrary profiles/axis editing, joints,
rigs, multiple material slots and production recipes remain later work.

No native UI interaction, native/WASM interoperability, complete contributor
and reviewer workflow or C1–C8 readiness is established here. FND-01–06 remain
open within their declared scopes, and required rig/composition/material/graph
work remains. No model/package is declared complete: all 193 catalogue and
eight subject-specific claims remain unassessed; film, open-world game and
cartoon packages remain incomplete. No production asset was made or changed.
