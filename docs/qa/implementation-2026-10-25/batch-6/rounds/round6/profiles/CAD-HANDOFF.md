# Bounded native surface of revolution

Staged only. No Cargo build/test, GUI, provider, credential, production asset or
live sibling write was performed. Asset production remains on hold.

## Exact integration pins

Root adds these authoring modules:

```rust
mod cad_recipe;
mod cad_controls;
#[cfg(test)]
mod cad_tests;
// GeometryRecipe variant, under the existing strict serde kind tag:
RevolveProfile { points: Vec<[f64; 2]>, segments: u32 },
```

Mount `super::cad_controls::CadControls` with the same `selected`, `inspected`,
`fresh`, and `onedit` props as `StructureControls`. Existing AddGeometry and
ReplaceGeometry intent serialization/authority applies. Root owns the trusted
native project creation-grant list and entry allowlist; old configurations must
not acquire this grant automatically.

The staged `geometry_host.rs` mounts a sibling helper through
`#[path = "revolve_host.rs"] pub(crate) mod revolve_host;`. Its pure validation
function is also reused by `authoring/cad_recipe.rs`. No additional root module
or re-export is needed. `spatial_host.rs` provides strict parameter admission,
then calls that bounded native adapter. GeometryHost retains its actual target
grant, remaining dispatch/work, profile-point and scratch budget checks.

The exact new catalogue ID is `ComputationalGeometry.revolve_profile`. It is
registered in Vibe's existing static IDs registry; there is no dialect version
change, Core dispatcher emulation or LocalHost fallback. The existing six Poet
catalogue checks now also cover this operation. The large registry is existing
static catalogue data, within the AGENTS registry exception.

## Inspected Core contract and winding

Actual implementation:
`C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/parametric_cad.rs:323`.
Public export: the existing `computational_geometry::revolve_profile` function.

```rust
pub fn revolve_profile(
    profile: &[Point3],
    segments: usize,
    out_vertices: &mut [Point3],
    out_triangles: &mut [[u32; 3]],
) -> Result<(usize, usize), CadError>
```

The kernel uses Point3.x as radius and Point3.y as height, ignores z, rotates
one full turn about +Y, closes the angular seam, and leaves profile endpoints
open. It emits `stations * segments` vertices and
`(stations - 1) * segments * 2` triangles. It does not offer caps, poles,
partial turns, shell thickness or solid CAD topology. This adapter does not
claim those outcomes or engineering certification.

Source inspection shows triangle order `[a,b,c], [a,c,d]` with b/c on the next
angular station. For descending heights its cross product has a positive radial
component; ascending heights face inward. Consequently this declared operator
requires top-to-bottom strictly decreasing heights and positive radii. It does
not reverse source input or rewrite Core triangle winding. Height values must
remain distinct after native f32 output conversion to prevent collapsed rings.
The authored output test independently computes triangle cross products and
requires `normal dot radial > 0` for every triangle, plus two open boundary
rings. That test has not yet executed; source reasoning is not a runtime pass.

## Bounds, preservation and cancellation

This control profile permits 2–256 radius-height stations, 3–256 radial
segments, radii in [0.000001,1000000] metres and finite heights within
[-1000000,1000000] metres. It admits monotonic radius-as-a-function-of-height
surfaces only. These are operator limits, not a ceiling on master fidelity.
The host checks all bounds and byte/count budgets before output allocation.
The canonical caller-buffered kernel constructs vertices and triangles. The
adapter only converts admitted pairs/output, checks returned counts and indices,
and derives Mesh bounds. No private mesh kernel, caps, loft or boolean was added.

Unknown parameters including units, caps and angle refuse. The source serializer
emits one host invocation outside any lambda in canonical pinned Vibe 0.1.
Authoring edits use the existing expected-source guards and retention/undo path.
Root's common topology-attribute guard is required to avoid losing old UV,
normal, skin, morph or unknown geometry fields. Other part/material fields remain
exact under replacement. No new resource/package write path was introduced.

The existing kernel has no cooperative cancellation hook. A running bounded
revolution cannot be interrupted mid-kernel. Surrounding job cancellation and
late-result rejection retain their existing owners; no new cancellation result
is claimed here. This limitation is also stated in the control.

## Authored checks

Four new `cad_revolution*` checks, not yet executed:

- Actual SourceSession/GeometryHost edit: exact counts, finite vertex/index
  bounds, outward radial normals, open ends, closed seam, preserved part/material
  fields and exactly one dispatch.
- Actual E300 missing target grant and E400 work/scratch/profile-point refusal,
  with no source promotion.
- Actual host negatives for wrong station direction, collapsed heights,
  nonpositive radius, segment bounds, caps/units and wrong coordinate arity;
  a later valid edit proves rejection consumed no dispatch. Form and strict
  source recipe checks are included.
- Canonical add/stale/source-retention/undo/rebuild and native `.10d`/Q42 HMC
  encode/decode on synthetic software-agent source.

The six catalogue checks cover source requests, host grants, family grants,
missing invocation grants, unknown siblings and pure-effect rejection for the
new registered ID as well as the previous operators. Root owns serial execution
and evidence. There is no rendered, human usability, complete embodiment or
manufacturing acceptance claim. IP-03/FND remain open and film, open-world game
and cartoon packages remain incomplete.
