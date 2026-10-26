# Community Grounds growing station — editable source

Original synthetic equipment concept, source revision 0.2. It depicts a proposed
modular growing station; it does not describe an installed system, verify water
flow or capacity, or include a qualified plant/species model. Film, open-world
game and cartoon delivery packages remain incomplete and require their separate
dossier claims and acceptance.

The 0.2 source revision replaces the unreleased external authoring namespace
with `ComputationalGeometry.profile_sweep` hosted by Poet. Part keys, semantic
identifiers, dimensions, shapes and preset parameters are unchanged. MIT
library provenance remains in the integrated library origin records.

`master.vibe` is the editable native Vibe source. Its numeric `poet_parameters`
record controls dimensions and geometry detail. `poet_rebuild_parts(p)` returns
the identified part list; `poet_parts` is mutable for the native Poet controls.
After a parameter change, explicitly evaluate
`poet_parts=poet_rebuild_parts(poet_parameters); ({parts:poet_parts});`.
Do not replace a nested record field through an unsupported assignment.

The engineering frame uses metres and local parent-relative transforms, with
rotation radians in the native T·Rx·Ry·Rz·S convention. The chassis root carries
the datum plate and its identified children. The nominal frame is 2.60 m wide,
1.65 m deep and 1.72 m high. Six replaceable formed channels sit at three levels
on each side. Their removable lid pieces leave 36 actual geometric openings,
with separate netpot collars. There are no generic leaf stand-ins.

Rounded sections, U-section walls, swept service bends, toroidal collars and
chamfered gusset profiles are Poet native geometry capabilities over QualiaDB kernels. The cabinet retains
separate panels, hatch, gasket, handle, hinges, vents and pump representation.
Pipe exterior geometry and visual connections are concept representations;
internal hydraulic continuity, pressure, drainage, electrical safety and
maintenance certification are unverified.

Load a preset cell only after loading the master:

| Preset | Deliberate change |
| --- | --- |
| `presets/film.vibe` | Highest currently authored section/ring tessellation and physical-looking palette. This is a target intent, not accepted film quality. |
| `presets/open_world_game.vibe` | Reduced section/ring tessellation, preserving the part structure, dimensions and collar positions. |
| `presets/cartoon.vibe` | Reduced tessellation and an illustrated matte palette; palette changes are representation styling, not a revised physical specification. |

At initial handoff the source's bounded loops intend 199 identified records.
That is a static inventory prediction; native admission, measured mesh counts,
rendered views and source/HMC round-trip results belong in the production dossier.
No build, credential operation or GUI run was performed by the source author.
Native source evaluation and actual offscreen review must precede any quality
acceptance. Preserve the raw source and presets alongside compiled derivatives.
