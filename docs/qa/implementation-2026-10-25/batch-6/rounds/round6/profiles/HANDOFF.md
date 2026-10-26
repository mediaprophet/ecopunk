# Round 6 staged metric profiles

The combined profile/CAD source targets are pinned by [target-manifest.json](target-manifest.json).
The existing `geometry_source.rs` has a complete before snapshot; the other
three targets are new (`before_sha256: null`). No sibling source was written.
No Cargo command, GUI, provider call, production asset or human review was run.
The later bounded surface-of-revolution extension has separate exact pins and
limits in [CAD-HANDOFF.md](CAD-HANDOFF.md); the original profile pins below remain.

## Root integration pins

In native `graphical/authoring/mod.rs`:

```rust
mod profile_recipe;
mod profile_controls;
#[cfg(test)]
mod profile_tests;
pub use profile_recipe::ProfileRecipe;
// Inside GeometryRecipe:
ProfileSweep { profile: ProfileRecipe, length: f64, caps: bool },
```

Mount `super::profile_controls::ProfileControls` beside `StructureControls`, with
the same `selected`, `inspected`, `fresh`, and `onedit` properties. Existing
`AddGeometry` / `ReplaceGeometry` native command routing carries the new recipe;
there is no independent command or authority path.

`ProfileRecipe` is a strict serde record with a snake_case `kind` tag:
`rounded_rectangle`, `open_channel`, `polyline`. Dimensions and XY coordinates
are metres. Polyline closing edges are implicit. Caps are a typed Boolean on
`GeometryRecipe::ProfileSweep`. JSON units, code and unrelated parameters refuse.
The typed operator bounds are a control profile, not a master-quality ceiling.

Geometry source serializes only validated numeric values and known field names
into a pinned Vibe 0.1 program. Its sole capability invocation remains outside
lambdas. `Profile::points` and Core `polygon_validation::validate_simple_polygon`
perform bounded preflight. The existing native host performs the only mesh
generation against its actual target grant, remaining dispatch/work, profile
point and scratch limits. No profile triangulator or mesh builder was added.

## Required preservation integration

Root owns `assembly_source.rs`. The common replacement path must refuse old
geometry with UV/normal/skin/morph/unknown fields rather than silently deleting
them when the profile kernel returns vertices/triangles. Its material and every
non-geometry part field remain exact. The staged test
`metric_profile_replacement_refuses_loss_of_topology_bound_geometry_fields`
requires this root-owned guard. This candidate is not independently installable
without the enum, control mount and preservation integration.

## Authored checks, not yet executed

All five checks are in `profile_tests.rs`:

| Test suffix after `metric_profile` | Intended evidence |
|---|---|
| `s_execute_once_with_caps_length_and_preserved_records` | Three actual native profile edits, capped/uncapped counts and length, field retention, exactly one exhausted dispatch, no failed source promotion |
| `_controls_validate_fields_units_bounds_and_simple_sections` | Form mapping, canonical shape rejection, positive length/range, strict tags/Boolean, explicit units syntax refusal |
| `_grants_work_and_point_budgets_refuse_without_source_promotion` | Actual E300 target grant and E400 work/point denial, unchanged source state |
| `_replacement_refuses_loss_of_topology_bound_geometry_fields` | Common refusal for UV, normal, skin, morph and unknown geometry fields |
| `_add_stale_undo_and_source_to_native_hmc` | Native editor add, stale guard, retained single invocation source, actual `.10d`/Q42 checkpoint encode/decode, undo/rebuild |

Root owns serial compile/test execution and saved-source evidence. Static source
inspection is not evidence that these checks pass. The HMC test does not render,
restore an actual human's session, certify graph completeness, or accept the
package. Arbitrary sweep paths, capped/pole/partial-angle revolution, CAD constraints/B-rep, profile rehydration
from existing mesh, UV regeneration, rigs, manufacturing certification and native
GUI usability remain outside this bounded candidate. IP-03/FND gates remain open;
assets remain on hold and film/game/cartoon packages remain incomplete.
