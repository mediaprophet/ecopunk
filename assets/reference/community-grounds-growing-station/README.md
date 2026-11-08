# Community Grounds growing station — editable source

Original synthetic equipment concept, source revision 0.2. It depicts a proposed
modular growing station; it does not describe an installed system, verify water
flow or capacity, or include a qualified plant/species model. Film, open-world
game and cartoon delivery packages remain incomplete and require their separate
dossier claims and acceptance.

Current master pin: 15,690 bytes, SHA-256
`8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63`.

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

**Native parameter safety update, 2026-10-26:** native Parameter controls now
require an explicit `poet_parameter_parts` recipe baseline and refuse after
downstream part/material/UV/detail edits. This preserved reference master has
not been migrated while asset production is on hold. The direct source rebuild
above is an explicit replacement operation and can discard downstream edits;
retain the original and merge required edits into the recipe before using it.
Follow the [reviewed migration procedure](../../../docs/qa/implementation-2026-10-26/round8/edit-safety-handoff.md).

**Subsequent native source-layer implementation:** the native **Enable
non-destructive source layers** action now adopts this existing recipe through
a checked source revision, preserving material/TRS work. Later dimension edits
use those explicit source-owned overrides; unsupported topology-bound UV/detail
changes still refuse. The [actual 199-part qualification and retained HMC](../../../docs/qa/implementation-2026-10-26/round9/progress.md)
uses the host-selected equipment profile, since its 239,952 compiled preview
vertices exceed the legacy 200,000-vertex preview allowance. The master file
remains unchanged and asset production/visual acceptance remains on hold.

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

## Batch 6 native fixture result

The bounded film-intent producer run is recorded at
`outputs/round1-film-006/`. It consumed five source cells and saved an accepted
17,571-byte source, 199 identified parts, nine material records and 199 material
ranges. Its `.10d` output contains 239,952 vertices and 79,984 triangles. The
5,413,279-byte HMC and 5,288,192-byte `.10d` were reopened in a fresh process;
the actual native path produced four 1024×1024 inspection PNGs from that output.
The [render receipt](outputs/round1-film-006/render-receipt.json) and the dossier's
[evidence index](../../../docs/production/community-grounds-growing-station/evidence-index.json)
bind the artifact hashes and checks.

This is fixture execution evidence, not package or appearance acceptance. The
independent triage in `docs/qa/implementation-2026-10-25/batch-6/rounds/round1/review/visual-triage.md`
records cabinet framing, lid aperture gaps and curved-surface/material readability
as Round 2 work; it establishes no engine gap. UV0 and normal-related transport
data exist in the output path, but the current source may supply fallback UVs
(including zeros); this does not establish an authored UV unwrap. Root's image
inspection was technical and is not human visual acceptance. Rights, engineering,
plant/port/full-graph and game-demo qualifications remain open.
