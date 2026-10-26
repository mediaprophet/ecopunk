# Growing-station inventories and expected cases

This is an explicit finite **planning inventory**, not a finding that all listed parts, dimensions, materials, ports, functions, or profiles have been evaluated. `source-derived` means the editable Vibe file declares the item or intent; it does not prove that the native evaluator emitted it. IDs below are dossier-local inventory keys unless specifically identified as existing game replacement keys. Unknown values stay open.

## Source revision and preset pins

| Input | Size | SHA-256 | Scope |
|---|---:|---|---|
| `assets/reference/community-grounds-growing-station/master.vibe` | 15,690 bytes | `8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63` | Current editable synthetic master source, revision 0.2 |
| `assets/reference/community-grounds-growing-station/presets/film.vibe` | 442 bytes | `5317fb49df09ea71e477c8e3bb27bf30221f9e042d79167420c2ebed86ad0037` | Film-intent parameter preset only |
| `assets/reference/community-grounds-growing-station/presets/open_world_game.vibe` | 444 bytes | `c227ff809824d13d86ebbe09fad02799390e6ae8783ff1137d69b69eb9fc919f` | Open-world-game-intent parameter preset only |
| `assets/reference/community-grounds-growing-station/presets/cartoon.vibe` | 450 bytes | `f8b786e7212c109f07f7da802ba11e64bdd091e52655bcd2748cb89a33d70af5` | Cartoon-intent parameter preset only |
| `docs/planning/tech/assets/model-production-template/claims.template.json` | pinned separately | template catalogue SHA below | Source for complete claim row set |

The source intends 199 identified records and 36 openings/collars. In the bounded Batch 6 film-intent fixture run, native execution saved 199 identified parts, 9 material records, 199 material ranges, a 239,952-vertex/79,984-triangle `.10d`, and the linked HMC. Those measured counts apply to that run only. They do not establish engineering suitability, full semantic topology, 36 independently qualified ports/openings, or complete package acceptance. Exact artifact hashes and run checks are in [evidence-index.json](evidence-index.json).

The run's decoded mesh used a UV0 stream, but the spatial source may supply fallback UV values (including zero); no authored UV unwrap is asserted. Normal transport and material ranges were consumed by the renderer, but no material/appearance acceptance is claimed. Plants remain separate species embodiments, and water, service, safety, rights, and functional port claims remain unqualified.

## Assembly component inventory

| Inventory key | Expected/source-declared scope | Qualification needed |
|---|---|---|
| `station-root` | Root assembly, datum plate, local hierarchy | Actual transform hierarchy, units, emitted parts and selection behavior |
| `frame` | Rails/headers, posts, adjustable feet, braces, gussets, bolts | Material/section adequacy, dimensions, fasteners, tolerance, installation and domain review |
| `grow-channel-set` | Six formed channels, three levels per side | Actual wall/edge/cap topology, dimensions, cleanability and compatibility |
| `opening-and-collar-set` | Thirty-six source-intended opening locations and collars | Native emitted count, fit/access, botanical container compatibility; no plant/package implied |
| `channel-lids` | Edge pieces, removable bays/end details around openings | Actual opening clearance, safe access, drainage/retention behavior unknown |
| `feed-return-visual-set` | Feed/return pipe branches and trunks, valves, levers, couplings | Connection graph, dimensions, pressure/head/flow/leakage, materials and service behavior unknown |
| `sump-and-cabinet` | Enclosure panels/top/door, gasket, handle, hinges, vents, plaque | Assembly access, weathering, ingress, service clearances and functional state unknown |
| `pump-representation` | Internal pump-like visual geometry | Actual equipment/function not established; no capacity, electrical system, or operating claim |
| `plant-dependencies` | Potential separate living plant instances from qualified species HMCs | No species selected. Plants are not station manufactured parts; do not invent taxon, maturity, compatibility, or yield. |

The source has nine material record IDs: `9001` frame, `9002` grow channels, `9003` rubber, `9004` steel, `9005` cabinet, `9006` visual waterline, `9007` controls, `9008` label, and `9009` shadow. They are authored colour/factor inputs only. Physical composition, texture/normal/roughness maps, aging, safety, contact/water resistance, provenance rights, and appearance quality are unassessed.

## Mandatory profile inventory

| Profile ID | Source preset | Target obligation | Status |
|---|---|---|---|
| `film` | `presets/film.vibe` | Defined film style, runtime/output, camera/shot-scale, deformation/secondary motion if used, temporal/render and reproducibility criteria | unassessed |
| `open_world_game` | `presets/open_world_game.vibe` | Game LOD/streaming/material/runtime budget and actual game-demo HMC load, interaction, state, save/replay acceptance | unassessed |
| `cartoon` | `presets/cartoon.vibe` | Coherent cartoon shape/material language and explicit film/realtime target, compatible parts/actions/render criteria | unassessed |

Preset source presence is not a representation resource or accepted profile. A film-intent technical run now exists, but the film package, open-world-game package, and cartoon package statuses all remain unassessed/incomplete. Profile criteria, all compatible cases, technical outputs and required human visual/domain reviews still need qualified evidence.

## Expected operational and review scenarios

These are obligations for investigation, not asserted current capabilities:

1. Exact-source part inventory and editable-source reconstruction, with native emitted part count, source/HMC hashes, material bindings and transform hierarchy.
2. Film/game/cartoon preset comparison using the same source revision and declared target-specific settings.
3. Static views at defined assembly, access, detail, and target-distance scales; inspect legibility, actual geometry, grounding, seams, openings/collars, pipe/cabinet and material appearance.
4. Inspect channels, lids, collars, ports, feed/return path, sump and pump representation as separately selectable units.
5. Domain-reviewed flow/maintenance/failure scenarios only after a sourced operational model is defined. Until then, explicitly report all flow/capacity/safety fields as unknown/unqualified.
6. Game-demo pack route: bind source-derived output to the exact approved replacement-key mapping, export `.10d`/textures into `assets/generated/`, include them in `web/assets/maslows-challenge-scenes.hmc`, run its existing validators and inspect the actual game-loaded source revision/scene. This has not occurred.
7. Rights/provenance review for authored design, source illustration, supporting libraries, materials, derivatives, source-to-output lineage and publication scope.

## Existing consumer key candidates (not adopted equivalences)

`C:/github/game-demo/docs/art/asset-replacement-register.csv`, rows 337–342, contains `rc:asset/hydroponic-channel-frame`, `rc:asset/hydroponic-channel-liner`, `rc:asset/hydroponic-greens`, `rc:asset/hydroponic-water-centre`, `rc:asset/hydroponic-water-east`, and `rc:asset/hydroponic-water-west`. Their current register labels are distributed across Kestrel Flats garden and prop/terrain/water families. A future contributor/reviewer must inspect the exact current records, scene placements and usages before mapping this assembly; the keys are not proof of matching topology or semantics.

The game README identifies generated `.10d` assets at `assets/generated/` and its packaged HMC at `web/assets/maslows-challenge-scenes.hmc`; `crates/rolling-commons-shell/src/pack_validation.rs` validates HMC entries, `.10d`, texture references, impostors and Q42 resources. The product build is `scripts/build-game.ps1`. No game-demo source or artifact has been changed by this Ecopunk dossier.

## Round 1 output and visual follow-up

The saved output is `assets/reference/community-grounds-growing-station/outputs/round1-film-006/`. It contains accepted source, `.10d`, HMC, render receipt and four 1024×1024 PNGs. Root inspected all four for technical render behavior. Independent software-agent triage of three captures found cabinet framing that obscures service detail, exposed rectangular corners around circular lid collars, and segmented curved highlights/material readability. These are Round 2 source/camera/material investigations; the triage established no renderer or engine gap and is not human appearance acceptance. See [visual-triage.md](../../qa/implementation-2026-10-25/batch-6/rounds/round1/review/visual-triage.md).

The producer also verified fresh-process HMC source reopen and exact regenerated `.10d` bytes before rendering. Native execution and selected check results are recorded in [evidence-index.json](evidence-index.json); all 201 ledger rows remain open. This narrow run does not qualify plants, actual hydraulic/utility ports, the complete graph/resources, source rights, domain/engineering safety, accessible authoring UX, human acceptance, or a game-demo derivative.

## Pinned catalogue and completeness rule

The ledger preserves the template's **193 base deliverables**, catalogue version `2026-10-24.2`, SHA-256 `c0cb76151bf5d0776afaae4d59d7427efb65633bf359d3f4c2bda83c361c91cb`, and eight added `CGGS-*` obligations. Every base row currently has applicability `unassessed` and claim status `unassessed`; no N/A disposition, satisfied row, expanded case, formal model review decision, native gate, human acceptance, or profile pass is asserted. Round 1 software-agent technical/visual triage is recorded separately in the evidence index and does not change ledger dispositions. Inventories contain expected dimensions, but compatible expanded case matrices still need to be derived and assessed. All three target profiles and the package remain incomplete.
