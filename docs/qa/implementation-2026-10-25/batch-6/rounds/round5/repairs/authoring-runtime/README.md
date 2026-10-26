# Authoring runtime repair — staged, unexecuted

Two full-file replacements are pinned in `target-manifest.json`, with before
copies from the live files used by the first qualifying run. Live files and the
original Round 5 stage/receipts remain untouched. No Cargo/build/test execution
was performed by this repair agent.

The saved first `authoring.log` reports 12 passed and two failed cases. The
first failure is a real pinned-language incompatibility: ReplaceGeometry placed
`capability.invoke` inside a map lambda; Vibe 0.1's evaluator rejects capturing
the `capability` root. The repair reconstructs a flat list from indexed existing
part references, with a field-preserving selected record and exactly one inline
kernel invocation. It creates no temporary binding and changes no language,
host grant, resource budget or reconciliation rule. All geometry, parent, TRS,
material and other field assertions remain in the operator tests.

The second failure is a malformed synthetic 256-part test program. It declared
`let material` then used the keyword `material` as a primary expression.
`parse/expr.rs` permits a Keyword primary only for `capability`, while the
session's failed-program expression fallback obscured the original location.
The fixture now uses ordinary `mat` and explicitly requires `parse_program`
success before execution. It still authors and checks all 256 parts, the one
shared-material branch, generated source under 2048 bytes, actual factor deltas
and every unrelated field.

Existing exact cases requiring supervisor rerun:

- `graphical::authoring::structure_tests::bounded_geometry_recipes_add_and_replace_through_actual_grants_and_one_dispatch`
- `graphical::authoring::structure_tests::geometry_edits_refuse_denied_effect_and_exhausted_work_without_advancing_source`
- `graphical::authoring::structure_tests::shared_material_edit_groups_256_parts_without_source_or_expression_depth_growth`

The geometry case now additionally checks one authored invocation and absence
of a capture-prone map. Grant/work refusal assertions now require actual E300
and E400 respectively, preventing the former lambda E100 from satisfying those
negative cases. The existing native source→geometry→HMC/Q42 workflow must also
be rerun against this generator. No model, package or native UI acceptance is
claimed; assets remain HOLD and all three delivery targets remain incomplete.
