# Per-model production dossier and to-do template

**Version:** 2026-10-24.2 · **Status:** reusable specification template.
Use this for every species, organism/part family, vehicle, structure, tool or
other modeled thing. All initial claims are unassessed. No model is qualified
by the existence of this checklist.

## Start a dossier

For biosphere, geology and anatomical/biomedical education scope, also apply
the [capability programme](../../biosphere-earth-and-anatomy-capabilities.md).
Expand applicable BSA and BSA-EDU requirements as subject-specific obligations
and cases without deleting or repinning this catalogue. Education is a declared
purpose/accuracy profile alongside the required film, open-world-game and cartoon
targets; acceptance requires actual evidence and domain/educator review.

1. Copy [claims.template.json](claims.template.json) into the model dossier as
   `claims.json`. Set `document_kind` to `claim_ledger`; fill entity/dossier,
   family, source/corpus revision and normative requirement baseline references.
2. Retain the pinned catalogue version and SHA-256. Keep the complete row set;
   add subject-specific obligations with stable new IDs. Do not delete rows.
3. Define **film**, **open_world_game** and **cartoon** profile criteria and actual
   representation resources. Cartoon also declares its film/realtime target.
4. Fill the dimension inventories. They are explicit applicability/coverage
   inventories, not assertions that every possible combination exists. Shared
   or derived views of an inventory must name a native rule and source revision.
5. Expand each repeated deliverable into all compatible expected cases. A case
   contains `dimensions` naming the inventory IDs and an `assessment` with the
   same claim/evidence fields as its parent. The parent cannot pass on an empty
   or partial set of required cases.
6. Complete every claim: applicability, result/status, claimant/time, exact
   artifact/source references, evidence, required reviews and remaining work.
   Inapplicable family rows still need a disposition and authorized rule/rationale.
7. Bind evidence and review records to the exact outputs and dependencies.
   Reopen affected claims after source/rig/material/behaviour/profile changes.
8. Report the native gate result, all three profile statuses and unresolved IDs.
   The validator is specified upstream; do not invent an execution/approval result.

JSON is an authoring/interchange record here, not a second runtime knowledge or
state engine. Native integration must map these claims to the shared qualified
Q42/provenance/review/journal contracts. Keep one authoritative record and trace
any projections. [claims.schema.json](claims.schema.json) describes structure;
it does not by itself verify coverage, authority, quality or execution.

The governing [completion contract](../model-completion-claims.md) defines evidence,
applicability, expansion, freshness and completion. [Production toolchain](../production-toolchain-spec.md)
identifies the native authoring enablement work. Existing [per-asset requirements](../digital-twin-asset-requirements.md)
and [package/species/made-world contracts](../asset-package-spec.md) continue to apply.

## Identifier model and version migration

A subject can have several identifiers. Populate `subject.identifiers` and
`identifier_mapping_refs` with scoped/source-qualified associations; `entity_id`
remains a compatibility/local record reference. Inventory records can likewise
carry identifier arrays. Expand ID-01/ID-02 using `subject_identifier` and
`identifier_mapping`; unknown inventories remain unassessed, not invented.

This starter uses `ecopunk-model-claims-v2` and catalogue `2026-10-24.2`.
[Version 1 source records](history/README.md) are retained with their original
bytes/pins. Existing dossiers retain their pinned versions until an explicit
migration preserves prior claims/evidence and assesses the new obligations;
never silently carry an old pass into the changed identifier cases.
## Dossier front sheet

- Subject / associated identifiers and scopes / local dossier record reference / real-or-synthetic referent:
- Family / taxon concept or artefact design revision:
- Whole/root and component inventory:
- Source/corpus revision / graph-completeness reconciliation:
- Required profiles and quality criteria / variants and compatibility:
- Native authoring tools, library versions and reproduction chain:
- Movement/clip/transition and behaviour/state/interaction inventories:
- Lifecycle/damage/repair/media and qualification scenario inventories:
- Editable source, HMC content references and external final-archive receipt:
- Claim ledger, evidence index, review authority and gate report:

## Checklist use

Each row below is a required **claim slot**. The `Repeat` column makes additional
case-level slots mandatory. Checking a box means the completion contract has
resolved that row; it is not permission to claim human approval or skip N/A.
Family supplements apply by subject and retain explicit dispositions for other
families. Human/avatar, microbial/scientific and environmental supplements cover
those subjects when modeled; add any missing specialist requirements.

Catalogue: **193 base deliverables**, plus all expanded cases and subject-specific additions.
Pinned catalogue SHA-256: `c0cb76151bf5d0776afaae4d59d7427efb65633bf359d3f4c2bda83c361c91cb`.

## ID — Identifiers, scope and applicability

Family: `all`; requirement trace: DTA-ID, REQ-NAT, REQ-HMC.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | ID-01 | Subject/referent identifiers and associations: Record all known identifiers associated with the topic, subject, entity, agent or actor, with scheme, assigning source/authority, scope, provenance, validity and association status. A local record reference is not the complete identifier set; support several without inventing unknown values. | subject_identifier | technical | unassessed; ledger ID-01 |
| [ ] | ID-02 | Referent, definition, instance, part and release identifier mappings: Distinguish subjects of identifier associations, including taxon concept, organism/design/instance, parts, versions, package releases and content digests. Preserve qualified alternate/historical mappings and disagreements; matching labels or strings do not establish universal equivalence. | identifier_mapping | technical | unassessed; ledger ID-02 |
| [ ] | ID-03 | Whole-object and component inventory: Enumerate the complete organism/thing, internal/external parts, attachments and independent review units. | component | domain | unassessed; ledger ID-03 |
| [ ] | ID-04 | Three mandatory target profiles: Film, open-world game and cartoon each have a declared style, use, target runtime/output, inspection distance and acceptance criteria. | profile | visual | unassessed; ledger ID-04 |
| [ ] | ID-05 | Variant and compatibility inventory: Enumerate evidenced stage, season, sex/phenotype/reproductive form or design/configuration variants; mark unknowns and valid combinations. | variant | domain | unassessed; ledger ID-05 |
| [ ] | ID-06 | Movement, behaviour, interaction and state inventories: Research and enumerate the full applicable known repertoire for this subject; unresolved coverage gaps remain open rather than hidden. | one base claim | domain | unassessed; ledger ID-06 |
| [ ] | ID-07 | Requirement applicability and trace matrix: Every normative requirement and catalogue row maps to delivery evidence or a reasoned, independently authorized inapplicability decision. | one base claim | technical | unassessed; ledger ID-07 |
| [ ] | ID-08 | Subject-specific additions and change control: Add missing subject-specific deliverables, freeze the baseline and retain additions/retirements; no scope reduction to reach done. | one base claim | domain | unassessed; ledger ID-08 |

## KN — Knowledge, references and graph completeness

Family: `all`; requirement trace: DTA-SEM, REQ-SP, REQ-MADE.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | KN-01 | Source and corpus inventory: Identify acquired/held datasets, source revisions, references, acquisition times and coverage. | source | domain | unassessed; ledger KN-01 |
| [ ] | KN-02 | Identifiers/classification reconciliation: Preserve taxon concepts/names or artefact types/designs; conflicting and historical identifications remain attributable. | one base claim | domain | unassessed; ledger KN-02 |
| [ ] | KN-03 | Complete graph inclusion/reconciliation: Account for every available in-scope graph partition at the corpus revision; record omissions, restrictions and unresolved conflicts. | graph_partition | technical | unassessed; ledger KN-03 |
| [ ] | KN-04 | Lexicon, ontology and validation closure: Pack interpretation context, term mappings, required vocabulary versions and shapes for native offline queries. | one base claim | technical | unassessed; ledger KN-04 |
| [ ] | KN-05 | Typed dimensions and physical traits: Store sourced units, bounds/distributions, uncertainty and distinction between species/design values and individual observations. | trait | domain | unassessed; ledger KN-05 |
| [ ] | KN-06 | Anatomical/assembly semantics: Named organs, parts, tissues, systems and functions resolve to the same component IDs used in representations. | component | domain | unassessed; ledger KN-06 |
| [ ] | KN-07 | Time, environment and locality/context: Preserve observation/validity times, environmental context, spatial frames and sensitive-place conditions. | one base claim | domain | unassessed; ledger KN-07 |
| [ ] | KN-08 | Ecological/social/functional relations: Represent applicable partner entities and relation evidence without flattening them into appearance metadata. | relation_family | domain | unassessed; ledger KN-08 |
| [ ] | KN-09 | Life/reproduction or manufacture/service knowledge: Available lifecycle, behaviour or operational facts, conditions and limitations belong in the graph, including facts unused by graphics. | one base claim | domain | unassessed; ledger KN-09 |
| [ ] | KN-10 | Rights, consent, attribution and source licences: Each source/resource declares its rights and access conditions; a public derivative declares restricted exclusions honestly. | source | rights | unassessed; ledger KN-10 |
| [ ] | KN-11 | Observation/model/illustration distinction: Every material assertion is distinguishable as observed, sourced, simulated, inferred, generated or illustrative with uncertainty. | one base claim | domain | unassessed; ledger KN-11 |
| [ ] | KN-12 | Native graph query examples: Queries extract identifiers, parts, traits, relevant relations, available actions and representations from the actual packed graph offline. | query_case | technical | unassessed; ledger KN-12 |

## SRC — Editable sources and authoring chain

Family: `all`; requirement trace: DTA-AUTH, REQ-NAT.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | SRC-01 | Editable master source: Retain native editable geometry/procedural/CAD source, layers/history/parameters and semantic part bindings; compiled mesh alone is insufficient. | one base claim | technical | unassessed; ledger SRC-01 |
| [ ] | SRC-02 | Reference board and dimensional studies: Sourced orthographic/diagnostic references and proportional studies support the intended subject and style. | one base claim | domain | unassessed; ledger SRC-02 |
| [ ] | SRC-03 | Toolchain capability/revision manifest: Name actual native tools, library/API versions and supported features; separate source-present from runtime-qualified. | one base claim | technical | unassessed; ledger SRC-03 |
| [ ] | SRC-04 | Recorded native command/authoring chain: Source-to-output operations, inputs, seeds and receipts reproduce the artifact without required JS/Node/V8. | one base claim | technical | unassessed; ledger SRC-04 |
| [ ] | SRC-05 | Import/conversion and loss reports: Record every conversion, source digest, units, hierarchy/rig/material retention and any unsupported or lost data. | conversion | technical | unassessed; ledger SRC-05 |
| [ ] | SRC-06 | Source reopen/edit/undo/rebuild: A qualified native tool reopens and edits the source; undo/cancellation preserves work and the rebuilt part remains identifiable. | one base claim | technical | unassessed; ledger SRC-06 |
| [ ] | SRC-07 | Incremental compile/dependency invalidation: Changing one part, material or rule rebuilds affected outputs only and invalidates dependent evidence explicitly. | one base claim | technical | unassessed; ledger SRC-07 |
| [ ] | SRC-08 | Derivation lineage for every representation: Each profile/variant records its source/parameter basis and deliberate deviations; no untraceable independently generated lookalike. | representation | technical | unassessed; ledger SRC-08 |

## GEO — Geometry, anatomy and assembly form

Family: `all`; requirement trace: DTA-ID-05, DTA-VIS, REQ-MADE-003.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | GEO-01 | Metric dimensions, axes, pivot and frame: Author and runtime transforms preserve sourced physical dimensions; microscopic scale is explicit, viewport fitting does not change size. | representation | technical | unassessed; ledger GEO-01 |
| [ ] | GEO-02 | Diagnostic silhouette and proportion: Subject is recognizable without labels in multiple views at declared distances and matches relevant diagnostic structure. | profile × variant | visual_domain | unassessed; ledger GEO-02 |
| [ ] | GEO-03 | Complete exterior form: Trunk/branches/leaves or body/limbs/shell or chassis/envelope/parts are present with meaningful joins and shape hierarchy. | component × profile | visual | unassessed; ledger GEO-03 |
| [ ] | GEO-04 | Applicable interiors and sections: Claimed anatomy, internal structure, equipment/interiors and cut surfaces contain useful detail and correct membership. | section_view | visual_domain | unassessed; ledger GEO-04 |
| [ ] | GEO-05 | Topology, seams, intersections and integrity: Finite geometry, winding, degeneracy, manifold requirements and declared exceptions are checked; accidental gaps/intersections are resolved. | representation | technical | unassessed; ledger GEO-05 |
| [ ] | GEO-06 | Normals and tangent frames: Shading frames remain correct across mirrored UVs, hard edges, deformations and transforms. | representation | technical | unassessed; ledger GEO-06 |
| [ ] | GEO-07 | UV/parameterization and texel-density plan: Declare seams, density, tiling/atlas/procedural coordinates and sampling; no accidental stretching or unreported conversion. | representation | technical | unassessed; ledger GEO-07 |
| [ ] | GEO-08 | Attachments and component continuity: Named parts attach in a consistent frame with no visible disconnection or transform duplication; removable parts preserve identifiers. | attachment | technical | unassessed; ledger GEO-08 |
| [ ] | GEO-09 | Microstructure/relief/groom/foliage geometry: Required small detail uses an evidenced geometry/procedural/displacement/groom method that survives the intended inspection. | profile | visual | unassessed; ledger GEO-09 |
| [ ] | GEO-10 | Collision/contact and spatial envelopes: Qualified collision proxies, clearances, bounds and part selection agree with dimensions and motion, with declared error. | profile | technical | unassessed; ledger GEO-10 |

## MAT — Surface, material and look development

Family: `all`; requirement trace: DTA-VIS, DTA-PHY.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | MAT-01 | Surface/material inventory: Distinguish applicable bark/leaf/wood/fur/skin/eye, paint/metal/rubber/glass/cloth and cut tissues by part and appearance. | surface | visual_domain | unassessed; ledger MAT-01 |
| [ ] | MAT-02 | Editable/procedural material sources: Keep layered maps, nodes or recipes/parameters, source captures and deterministic seeds needed to recreate each surface. | surface | technical | unassessed; ledger MAT-02 |
| [ ] | MAT-03 | Map roles and interpretation: Declare colour/data space, units, channels, normal convention, UV set, sampler, mip/compression and integrity for all maps. | material_map | technical | unassessed; ledger MAT-03 |
| [ ] | MAT-04 | Physical versus render material linkage: Physical composition/response and render shading parameters are separately sourced and linked to matching parts. | surface | domain | unassessed; ledger MAT-04 |
| [ ] | MAT-05 | Neutral and varied-light material review: Neutral, directional/backlit and habitat/production light reveal convincing distinct surfaces without clipped or featureless shading. | profile | visual | unassessed; ledger MAT-05 |
| [ ] | MAT-06 | Transparency, thin surfaces and transmission: Applicable leaf/glass/membrane/hair/feather coverage, sidedness and shadow response work at intended scales. | profile | visual_technical | unassessed; ledger MAT-06 |
| [ ] | MAT-07 | State/age/season/wear surface changes: Applicable wetness, age, seasonal, injury, weathering and repair states follow the same admitted state and part identifiers. | surface_state | visual_domain | unassessed; ledger MAT-07 |
| [ ] | MAT-08 | Colour/exposure/display management: Source, working and output spaces, exposure and display transforms are declared and tested without accidental double conversion. | profile | technical | unassessed; ledger MAT-08 |
| [ ] | MAT-09 | Sampling and repetition review: No dominant tiling, stretched detail, seams, mask halos or unacceptable shimmer at target distances and motion. | profile | visual | unassessed; ledger MAT-09 |
| [ ] | MAT-10 | Material dependency admission: Native inspection proves actual required bindings/residency; missing/wrong-role maps are explicit failures, not silent fidelity success. | profile | technical | unassessed; ledger MAT-10 |

## FILM — Film delivery profile

Family: `all`; requirement trace: DTA-VIS, DTA-LOD, DTA-MEDIA.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | FILM-01 | Film shot/quality specification: Declare intended closeups, camera/lens, shot distance, output size/rate, colour/dynamic range and temporal/quality tolerances. | one base claim | visual | unassessed; ledger FILM-01 |
| [ ] | FILM-02 | Film source/detail representation: High-detail source, surfaces and silhouette remain convincing at the specified closest shot; geometry/detail method and limits are evidenced. | variant | visual | unassessed; ledger FILM-02 |
| [ ] | FILM-03 | Film deformation and secondary simulation: Required joints, face, muscle, skin, fur/feather/cloth/foliage dynamics and caches support film poses without artifacts. | film_simulation | visual_technical | unassessed; ledger FILM-03 |
| [ ] | FILM-04 | Film rendering capability and passes: Qualify the needed lighting, shadows, transmission, displacement/groom, compositing passes and effects; unimplemented capabilities stay open. | render_feature | technical | unassessed; ledger FILM-04 |
| [ ] | FILM-05 | Temporal rendering quality: Test movement, shutter/motion blur, sampling/noise, denoise and temporal stability where used, at the declared shot settings. | film_shot | visual_technical | unassessed; ledger FILM-05 |
| [ ] | FILM-06 | Reproducible film render/capture: Retain scene/light/camera/time/seed/dependency setup, frame outputs, costs and colour metadata tied to package/source digests. | film_shot | technical | unassessed; ledger FILM-06 |
| [ ] | FILM-07 | Editable film handoff: FilmCraft or qualified native sequence workflow reopens rendered shots/passes and sound with correct rational time and colour interpretation. | one base claim | technical | unassessed; ledger FILM-07 |
| [ ] | FILM-08 | Film motion/behaviour coverage: Every inventoried applicable clip/action/state has film-compatible rendering/deformation coverage or an unresolved delivery gap. | film_action | visual_domain | unassessed; ledger FILM-08 |
| [ ] | FILM-09 | Film acceptance reel: Review neutral turntable, diagnostic closeups and continuous action shots at final settings; human visual acceptance names the exact output. | one base claim | human_visual | unassessed; ledger FILM-09 |

## GAME — Open-world game delivery profile

Family: `all`; requirement trace: DTA-LOD, DTA-RUN, DTA-ANIM.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | GAME-01 | Game device/workload budget specification: Declare target hardware/native backend, view distances, instance density and measured frame/memory/transfer budgets; no universal guessed budget. | device_profile | technical | unassessed; ledger GAME-01 |
| [ ] | GAME-02 | Game geometry/texture/rig LOD ladder: Derived levels have explicit error/detail bounds and compatible topology/material/rig/collision variants; preserve diagnostic features. | lod | visual_technical | unassessed; ledger GAME-02 |
| [ ] | GAME-03 | Stable LOD transitions and impostors: Distance/hysteresis transitions preserve volume, coverage, part/state identifiers and picking; impostor/proxy behavior is declared. | lod_transition | visual_technical | unassessed; ledger GAME-03 |
| [ ] | GAME-04 | Streaming, culling and instancing: Native bounded acquisition/admission, culling and instancing retain individual identifiers; exercise actual populated-world conditions. | device_profile | technical | unassessed; ledger GAME-04 |
| [ ] | GAME-05 | Game collision/navigation/interaction: Collision, joints, traversal/navigation, reach/seat/grab/use and obstacle/contact behavior work at physical scale as applicable. | game_interaction | technical | unassessed; ledger GAME-05 |
| [ ] | GAME-06 | Runtime animation/behaviour fidelity levels: Reduced rig/update/simulation work preserves authoritative state and required events; distant visibility does not silently stop essential life/operation. | lod | technical | unassessed; ledger GAME-06 |
| [ ] | GAME-07 | Large-world precision and environmental integration: Position, terrain/shore contact, camera/depth, lighting and origin changes remain stable at declared world extent. | world_case | visual_technical | unassessed; ledger GAME-07 |
| [ ] | GAME-08 | Game stress/recovery evidence: Measure cold/warm load, CPU/GPU frame distributions, residency, uploads and recovery; test device loss, exhaustion and stale replacement. | device_profile | technical | unassessed; ledger GAME-08 |
| [ ] | GAME-09 | Persistent instance/replay continuity: Unload/reload, checkpoint and permitted representation swaps preserve admitted instance state and compatible journals. | one base claim | technical | unassessed; ledger GAME-09 |
| [ ] | GAME-10 | Open-world acceptance sequence: Human reviews walking/drive/inspect distances, a populated habitat and continuous interactions on declared native hardware. | one base claim | human_visual | unassessed; ledger GAME-10 |

## TOON — Cartoon representation profile

Family: `all`; requirement trace: DTA-VIS, DTA-ANIM, REQ-NAT-006.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | TOON-01 | Cartoon design bible and delivery target: Define coherent shape language, proportions, palette, shading/line treatment and film/realtime use; cartoon is not merely a low LOD. | one base claim | visual | unassessed; ledger TOON-01 |
| [ ] | TOON-02 | Cartoon model and surface set: Purpose-designed geometry/materials read clearly at declared distances and retain recognizable subject features. | variant | visual_domain | unassessed; ledger TOON-02 |
| [ ] | TOON-03 | Stylization-to-authoritative mapping: Declare exaggerated/omitted detail and physical/interaction implications; stylization does not rewrite species/design facts or state. | one base claim | domain | unassessed; ledger TOON-03 |
| [ ] | TOON-04 | Cartoon rig, shapes and pose language: Required squash/stretch, bend, facial/expression or mechanical stylization has controlled semantic/collision mappings and stable extreme poses. | toon_rig_feature | visual_technical | unassessed; ledger TOON-04 |
| [ ] | TOON-05 | Cartoon movement/behaviour presentation: All applicable inventoried movements/actions have deliberate timing, contacts, poses, anticipation/follow-through and transition coverage. | toon_action | visual_domain | unassessed; ledger TOON-05 |
| [ ] | TOON-06 | Cartoon rendering/line temporal quality: Qualified native shading/outlines/quantization retain readable form without temporal line/shadow flicker or inconsistent cuts. | toon_render_case | visual_technical | unassessed; ledger TOON-06 |
| [ ] | TOON-07 | Cartoon technical delivery and compatibility: Declare budgets/output settings and test rigs/materials/LOD for its film or game target; share valid resources without mixing incompatible profiles. | toon_target | technical | unassessed; ledger TOON-07 |
| [ ] | TOON-08 | Cartoon acceptance reel: Review turntable, expression/pose sheet and continuous action sequence within the intended coherent scene style. | one base claim | human_visual | unassessed; ledger TOON-08 |

## RIG — Articulation, deformation and authoring

Family: `all`; requirement trace: DTA-ANIM.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | RIG-01 | Articulation/deformation applicability inventory: Enumerate skeletal, rigid-joint, morph, procedural, growth and secondary mechanisms; a static prop requires an explicit reviewed reason. | one base claim | domain | unassessed; ledger RIG-01 |
| [ ] | RIG-02 | Joint/constraint hierarchy and rest/bind poses: Stable joints/parts, frames, limits and attachments match anatomy/design; retain editable controls and authored bind/rest state. | rig | technical | unassessed; ledger RIG-02 |
| [ ] | RIG-03 | Skin weights and influence validation: Weights, joint mapping, influence limits and deformation topology meet the chosen profile without leaking/collapsing parts. | rig × profile | technical | unassessed; ledger RIG-03 |
| [ ] | RIG-04 | IK/FK and contact authoring: Required feet/hands/tail/wheel/tool constraints and pose controls solve with stable contacts and intentional mode transitions. | rig_feature | visual_technical | unassessed; ledger RIG-04 |
| [ ] | RIG-05 | Correctives, facial and fine-motion shapes: Applicable joint correctives, eyelids/mouth/digits, muscle or mechanical seals are authored and tied to named parts/states. | corrective | visual_domain | unassessed; ledger RIG-05 |
| [ ] | RIG-06 | Extreme-pose and deformation review: Exercise representative and worst allowed poses for every rig/profile, including normals/tangents and visible volume preservation. | rig × profile | visual_technical | unassessed; ledger RIG-06 |
| [ ] | RIG-07 | Variant/LOD retarget compatibility: Declare and qualify joint/clip mappings, scale changes and excluded combinations without silently retargeting incompatible anatomy. | rig_compatibility | technical | unassessed; ledger RIG-07 |
| [ ] | RIG-08 | Deformation bounds/shadow/pick consistency: Animated geometry, bounds, shadows, collision proxies and semantic picks agree throughout allowed motion. | rig × profile | technical | unassessed; ledger RIG-08 |
| [ ] | RIG-09 | Secondary dynamics and baked/procedural cache policy: Applicable foliage/cloth/fur/feather/antenna/cable/suspension dynamics have controlled parameters, budgets, collision and deterministic inputs. | secondary_system × profile | visual_technical | unassessed; ledger RIG-09 |
| [ ] | RIG-10 | Native rig edit/play/scrub/reset: Native tools discover and manipulate the actual rig and clips; pause, seek, edit, interruption and reset preserve coherent state. | rig | technical | unassessed; ledger RIG-10 |

## MOV — Full movement and transition library

Family: `all`; requirement trace: DTA-ANIM, DTA-BHV.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | MOV-01 | Movement research and applicability coverage: Catalogue all known applicable movement modes and contexts for the subject, including stationary and rare/recovery movement; source or label assumptions. | movement | domain | unassessed; ledger MOV-01 |
| [ ] | MOV-02 | Idle, rest and secondary living/operating motion: Applicable breathing, gaze, ear/eye motion, settling, leaf/wind, vibration or running machinery has intentional state-linked presentation. | movement × profile | visual_domain | unassessed; ledger MOV-02 |
| [ ] | MOV-03 | Locomotion or functional motion clips: For each inventoried mode author required start/cruise/stop/reverse/turn/speed/direction forms, or expose the precise missing coverage. | clip × profile × compatible_variant | visual_domain | unassessed; ledger MOV-03 |
| [ ] | MOV-04 | Special traversal and environment movement: Applicable jump/hop/climb/swim/fly/burrow/slither/crawl, door/wheel/suspension/actuator or plant tropic/wind motion is represented and qualified. | movement × profile | visual_domain | unassessed; ledger MOV-04 |
| [ ] | MOV-05 | Event and contact markers: Clips declare duration/rate/looping, contact phases, action events and exact timing; events drive the correct named subject parts. | clip | technical | unassessed; ledger MOV-05 |
| [ ] | MOV-06 | Root motion and kinematic/state displacement: Distinguish presentation from admitted movement; speed, turning, slope, traction and collision agree without foot/wheel/tail sliding. | clip × profile | visual_technical | unassessed; ledger MOV-06 |
| [ ] | MOV-07 | Complete transition and blend coverage: Every allowed state/clip edge is implemented/tested, including entry/exit, interruption, reverse/seek and denied transitions. | transition × profile | visual_technical | unassessed; ledger MOV-07 |
| [ ] | MOV-08 | Interaction and multi-actor timing: Compatible actors/objects align contact, reach, feed/grasp/use/board/social events and separate each participant's state authority. | interaction × profile | visual_technical | unassessed; ledger MOV-08 |
| [ ] | MOV-09 | Injury, fault, recovery and lifecycle movement: Applicable impairment, stumble/stall, shutdown/death, recovery/restart and stage changes are represented without fake physiological/operational claims. | movement_state × profile | visual_domain | unassessed; ledger MOV-09 |
| [ ] | MOV-10 | Variant and target movement matrix: Every compatible variant/profile/LOD action is listed as delivered, shared with evidence, blocked or unavailable; no claim from one tested clip to the whole library. | one base claim | technical | unassessed; ledger MOV-10 |
| [ ] | MOV-11 | Continuous movement review and replay: Continuous sequences show real transitions/contacts from the admitted runtime; keep input/time/seed and source/clip identifiers for reproduction. | movement_scenario × profile | human_visual | unassessed; ledger MOV-11 |
| [ ] | MOV-12 | Reduced-motion/static movement alternatives: Alternative presentation preserves important action/state information and timing without claiming absent motion. | profile | accessibility | unassessed; ledger MOV-12 |

## BHV — Behaviour, autonomy, lifecycle and interactions

Family: `all`; requirement trace: DTA-BHV, DTA-LIFE, DTA-STATE.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | BHV-01 | Complete behaviour/affordance ontology inventory: Discover all known applicable actions, states, needs/controls and interactions from the packed graph; distinguish authored policy, observations and unknowns. | behaviour | domain | unassessed; ledger BHV-01 |
| [ ] | BHV-02 | Preconditions, effects and applicability: Each behaviour declares participants, trigger/stimulus, allowed states/environment/variants, units, outputs, refusals and consequences. | behaviour | domain | unassessed; ledger BHV-02 |
| [ ] | BHV-03 | Native Vibe/rule/engine implementation binding: Actual named rules/capabilities implement each action; UI/animation is not an independent state engine. | behaviour | technical | unassessed; ledger BHV-03 |
| [ ] | BHV-04 | Feeding/metabolism or resource/energy operation: Applicable food/water/nutrients/energy/fuel/material intake, storage, consumption and waste/output are accounted for under sourced assumptions. | resource_process | domain_technical | unassessed; ledger BHV-04 |
| [ ] | BHV-05 | Sensing, stimuli and environmental response: Applicable light, sound, temperature, moisture, terrain, threat or operator inputs change state under declared models; do not invent inner psychology. | stimulus_response | domain | unassessed; ledger BHV-05 |
| [ ] | BHV-06 | Social, reproductive and cooperative interactions: Applicable communication, mating/care, group response, pollination or operator/multi-device interactions have compatible participant/state contracts. | interaction | domain | unassessed; ledger BHV-06 |
| [ ] | BHV-07 | Growth, maturity, season and ageing: Applicable stage/phenology/age changes drive coherent anatomy/rig/material/physics/behaviour; uniform scaling alone is not development. | lifecycle_transition | domain_technical | unassessed; ledger BHV-07 |
| [ ] | BHV-08 | Damage, failure, repair/regrowth and death/retirement: Causal thresholds and resulting part/state/resource/appearance changes preserve loss/replacement lineage and irreversibility where modeled. | damage_transition | domain_technical | unassessed; ledger BHV-08 |
| [ ] | BHV-09 | Deterministic scheduling, randomness and priorities: Recorded time, seed, event ordering and policy versions reproduce outcomes within declared numerical tolerances. | one base claim | technical | unassessed; ledger BHV-09 |
| [ ] | BHV-10 | Authoritative state, journals and causality: Single/qualified writer, versioned transactions, base binding, checkpoint/restore and causal events preserve immutable HMC definitions. | one base claim | technical | unassessed; ledger BHV-10 |
| [ ] | BHV-11 | Action-to-animation/audio/physics synchronization: Admitted effects drive clips, poses, contacts, emitters and physical changes; decorative motion never claims resource transfer. | behaviour × profile | technical | unassessed; ledger BHV-11 |
| [ ] | BHV-12 | Positive and negative behaviour scenarios: Exercise expected effect, missing inputs, wrong state, denied authority, cancellation, interruption and stale requests for every applicable behaviour. | behaviour | technical | unassessed; ledger BHV-12 |
| [ ] | BHV-13 | Full behaviour-transition/context coverage: Enumerate allowed/denied transitions and materially distinct contexts; each has evidence or an explicit open gap, including profile/variant differences. | behaviour_case | domain_technical | unassessed; ledger BHV-13 |
| [ ] | BHV-14 | Isolation, reset and replay review: What-if/review actions cannot mutate source or another session; replay/reset restores the documented state and releases old effects. | one base claim | technical | unassessed; ledger BHV-14 |

## FLORA — Plant/fungus applicability supplement

Family: `flora_funga`; requirement trace: DTA-LIFE, REQ-SP.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | FLORA-01 | Diagnostic habit and branching/fruiting architecture: Represent the actual applicable organism habit, trunk/stem, roots/mycelium, branches and crown/fruiting-body structure; qualify taxon-specific assumptions. | variant | visual_domain | unassessed; ledger FLORA-01 |
| [ ] | FLORA-02 | Organ and tissue inspection set: Applicable leaf/bark/root/flower/bud/fruit/seed/spore/gill or other diagnostic organs and tissues have named, meaningful inspectable detail. | organ | domain | unassessed; ledger FLORA-02 |
| [ ] | FLORA-03 | Development, propagation and senescence inventory: Applicable germination/emergence, juvenile/mature/reproductive/senescent/dead and propagation states have evidence and compatible representations. | life_stage | domain | unassessed; ledger FLORA-03 |
| [ ] | FLORA-04 | Phenology and reproductive motion: Applicable budding, flowering/anthesis, fruiting, spore/seed release, leaf flush/drop and season changes are modeled/presented with sourced timing. | phenology | visual_domain | unassessed; ledger FLORA-04 |
| [ ] | FLORA-05 | Wind, touch, tropism and environmental motion: Applicable trunk/branch/stem/leaf motion and response preserve attachments, scale, physical assumptions and profile compatibility. | plant_motion × profile | visual_domain | unassessed; ledger FLORA-05 |
| [ ] | FLORA-06 | Water, nutrient, light and substrate processes: Declare and qualify applicable uptake/growth/resource/environment coupling without confusing a simplified model with measured physiology. | plant_process | domain_technical | unassessed; ledger FLORA-06 |
| [ ] | FLORA-07 | Damage, stress, fire/disease and recovery: Applicable breakage, drought/wilting, disease/fire response, regrowth and death preserve causal/state/component consistency. | plant_damage | domain | unassessed; ledger FLORA-07 |
| [ ] | FLORA-08 | Ecological partner and habitat relations: Available host/symbiosis/pollination/food/habitat relations and placement conditions are represented with sourced applicability, not random taxon colour cycling. | plant_relation | domain | unassessed; ledger FLORA-08 |

## FAUNA — Animal applicability supplement

Family: `fauna`; requirement trace: DTA-LIFE, DTA-BHV, REQ-SP.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | FAUNA-01 | Body plan and diagnostic anatomy: Skeletal/soft-tissue landmarks, limb/digit/head/tail/wing and coat/covering features identify the species and support plausible poses. | variant | visual_domain | unassessed; ledger FAUNA-01 |
| [ ] | FAUNA-02 | Applicable locomotor repertoire: Inventory subject-appropriate gaits/modes, turning, slopes, terrain/water/air transitions, speed ranges and contacts, including unusual species-specific modes. | locomotor_mode | domain | unassessed; ledger FAUNA-02 |
| [ ] | FAUNA-03 | Feeding, drinking and elimination behaviours: Applicable foraging/capture/chewing/swallow/drink/digest/output actions have coherent contacts, resources and presentation. | animal_process | domain | unassessed; ledger FAUNA-03 |
| [ ] | FAUNA-04 | Rest, grooming, sensing and communication: Applicable sleeping/resting/grooming, gaze/sensory response, calls and body signals are covered and evidence-labelled. | animal_behaviour | domain | unassessed; ledger FAUNA-04 |
| [ ] | FAUNA-05 | Social, threat and reproductive behaviour: Applicable group/social/threat/escape/defence/mating/nesting/parental actions have source/context, compatible participants and noninvented effects. | animal_interaction | domain | unassessed; ledger FAUNA-05 |
| [ ] | FAUNA-06 | Sex/phenotype/stage/reproductive form: Available dimorphism, juvenile/mature/ageing and reproductive-condition morphology/rig/behaviour compatibility is explicit. | variant | domain | unassessed; ledger FAUNA-06 |
| [ ] | FAUNA-07 | Injury, illness, death and recovery forms: Applicable impairment/stress/recovery and death states preserve anatomy, physical contacts, behaviour limitations and evidence boundaries. | animal_state | domain | unassessed; ledger FAUNA-07 |
| [ ] | FAUNA-08 | Habitat, resources and multi-species relationships: Available home/habitat/resource/food-web/host interactions drive only qualified behaviour and contextual placement. | animal_relation | domain | unassessed; ledger FAUNA-08 |

## MADE — Vehicle, structure, tool and system supplement

Family: `made`; requirement trace: REQ-MADE, DTA-BHV-05.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | MADE-01 | Design, type and as-built/instance distinction: Design revision, manufactured/synthetic instance and actual configuration/observations have separate identifiers and provenance. | one base claim | domain | unassessed; ledger MADE-01 |
| [ ] | MADE-02 | BOM and fitted assembly/part graph: Specified/actual quantities, material batches, replaceable parts, structural parents, attachment frames and configuration history are retained. | component | domain_technical | unassessed; ledger MADE-02 |
| [ ] | MADE-03 | Mechanical articulation and access: Applicable wheels, steering, suspension, hinges/doors, actuators, mechanisms, service spaces and interiors have dimensions/limits/contact. | mechanism | visual_technical | unassessed; ledger MADE-03 |
| [ ] | MADE-04 | Functional port and connection graph: Mechanical/electrical/fluid/thermal/data interfaces declare medium, units, compatibility, ratings and actual state; containment and functional loops are distinct. | port | technical | unassessed; ledger MADE-04 |
| [ ] | MADE-05 | Operating/control modes and indicators: Start/stop/idle/operate/reverse/configure or other applicable modes expose controls, preconditions, feedback and exact state changes. | operating_mode | domain_technical | unassessed; ledger MADE-05 |
| [ ] | MADE-06 | Energy, fluid, material and thermal accounting: Inputs/stores/outputs/losses, capacities, consumption and temperature models are sourced and budgeted; rated performance is not observed flow. | made_process | domain_technical | unassessed; ledger MADE-06 |
| [ ] | MADE-07 | Failure, degradation and damage modes: Applicable wear, fatigue, corrosion, impact, overheating, leaks or faults produce coherent operation/appearance/part consequences. | failure_mode | domain | unassessed; ledger MADE-07 |
| [ ] | MADE-08 | Maintenance, repair, replacement and dismantling: Declared work uses inputs/tools/parts, removes/adds actual component identifiers, records condition/configuration and preserves reuse/disposal lineage. | work_procedure | domain_technical | unassessed; ledger MADE-08 |
| [ ] | MADE-09 | Installation/site connection and movement: Location, anchors, service connection/disconnection and relocation remain distinct from manufacturing identifiers and compatible state. | installation_case | technical | unassessed; ledger MADE-09 |
| [ ] | MADE-10 | Authority, occupancy and ownership distinctions: Use/control/maintenance/custody/ownership claims are explicit; occupants/operators are agents, never manufactured parts or owned contents. | one base claim | rights | unassessed; ledger MADE-10 |
| [ ] | MADE-11 | CAD/diagram/interior/illustrated representation links: Required design/section/exploded/system and film/game/cartoon depictions resolve to matching assembly/part state and record simplifications. | representation | visual_technical | unassessed; ledger MADE-11 |
| [ ] | MADE-12 | Operational and refusal scenarios: Exercise normal operation, wrong units/connections, insufficient supply, stale configuration, denied control and a failure/repair/restart sequence. | made_scenario | technical | unassessed; ledger MADE-12 |

## MICRO — Microbial, molecular and cellular supplement

Family: `micro_scientific`; requirement trace: DTA-SEM, DTA-PHY, DTA-LIFE.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | MICRO-01 | Scientific entity and scale interpretation: Distinguish organism, colony, cell, organelle, molecule and explanatory model; include physical units, time scale and evidence limits. | scientific_scale | domain | unassessed; ledger MICRO-01 |
| [ ] | MICRO-02 | Diagnostic structure and interior models: Applicable membranes, walls, appendages, organelles, molecular/colony structures and sections have sourced identifiers and meaningful inspection. | micro_structure | visual_domain | unassessed; ledger MICRO-02 |
| [ ] | MICRO-03 | Movement and resource/reaction repertoire: Applicable motility, transport, metabolism/reaction and response modes have declared models and presentation at meaningful temporal scales. | micro_process | domain_technical | unassessed; ledger MICRO-03 |
| [ ] | MICRO-04 | Development, division and colony/lifecycle state: Available growth, division, differentiation, reproduction and death/decay states preserve lineage and distinguish observed from modeled dynamics. | micro_lifecycle | domain | unassessed; ledger MICRO-04 |
| [ ] | MICRO-05 | Scientific representation and model validation: Scientific, film/game and cartoon abstractions retain entity identifiers; simulations/fields have sourced parameters, valid regimes and required verification. | scientific_model | domain_technical | unassessed; ledger MICRO-05 |
| [ ] | MICRO-06 | Cross-scale graph and resource navigation: Native inspection traverses the correct part/scale relations and resources without treating magnification as physical growth or graph facts as mesh detail. | scientific_scale | technical | unassessed; ledger MICRO-06 |

## ENV — Natural body, terrain and habitat supplement

Family: `environment`; requirement trace: DTA-SEM, DTA-PHY, DTA-VIS.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | ENV-01 | Natural referent, frame and source coverage: Distinguish measured/place-specific from illustrative landform/material/habitat; declare geospatial frame, resolution, time, provenance and uncertainty. | one base claim | domain | unassessed; ledger ENV-01 |
| [ ] | ENV-02 | Surface, interior and stratigraphic structure: Applicable terrain, rocks, soil/substrate, water and internal layers have meaningful geometry/material/field representations and source-derived boundaries. | environment_layer | visual_domain | unassessed; ledger ENV-02 |
| [ ] | ENV-03 | Habitat population and placement rules: Species/artefact instances retain their own embodiments; composition respects ecological/contextual constraints, physical contact and declared density. | placement_rule | domain | unassessed; ledger ENV-03 |
| [ ] | ENV-04 | Natural dynamics and coupled processes: Applicable erosion/fracture/flow/wave/soil/weather or other declared dynamics link parameters, mass/energy accounting and stable visual response. | environment_process | domain_technical | unassessed; ledger ENV-04 |
| [ ] | ENV-05 | Multi-scale/tile/LOD continuity: Declared world-to-closeup transitions retain coordinates, topology/coverage, contacts and picking without tile seams or physical-scale compression. | environment_transition | visual_technical | unassessed; ledger ENV-05 |
| [ ] | ENV-06 | Habitat and environmental native scenarios: Review representative walk/inspect/film/cartoon sequences and applicable environment interactions using identified graph and model inputs. | environment_scenario | human_visual | unassessed; ledger ENV-06 |

## HUM — Human/avatar applicability supplement

Family: `human_avatar`; requirement trace: DTA-RIGHTS, DTA-ANIM, DTA-BHV.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | HUM-01 | Human/synthetic referent and depiction consent: Distinguish fictional avatar from a real human; preserve applicable identifiers, capture/biometric consent and privacy, without invented authority. | one base claim | rights | unassessed; ledger HUM-01 |
| [ ] | HUM-02 | Human body, clothing and equipment: Applicable proportions, face/hands/feet, clothing layers and equipment are credible, deformable and linked to appropriate entity/part/attachment identifiers. | avatar_variant | visual_domain | unassessed; ledger HUM-02 |
| [ ] | HUM-03 | Human locomotion, gestures and task contacts: Inventoried movement, grasp/use/carry/sit/board and social/task interactions have stable contacts and authored expression/timing for required targets. | avatar_action × profile | visual | unassessed; ledger HUM-03 |
| [ ] | HUM-04 | Speech/expression and accessible alternatives: Applicable face/lip/voice/gesture synchronization uses licensed sources and explicit fictional/real scope, with text and reduced-motion alternatives. | avatar_expression | rights_accessibility_audio | unassessed; ledger HUM-04 |
| [ ] | HUM-05 | Authored human behaviour claim boundary: Actions, role/permission and scenario policy are declared; animation or inference never claims observed intent, consent or psychological truth. | avatar_behaviour | domain | unassessed; ledger HUM-05 |
| [ ] | HUM-06 | Person/artefact/social relation distinction: People remain agents with applicable standing; clothing/tools/vehicles and occupancy do not turn the person into an owned assembly component. | one base claim | rights | unassessed; ledger HUM-06 |

## PHY — Physical response and environment coupling

Family: `all`; requirement trace: DTA-PHY, DTA-DMG, DTA-TWIN.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | PHY-01 | Physical model applicability and assumptions: Declare static/kinematic/rigid/articulated/deformable/field or other model, version, bounds and approximation; unknown does not mean indestructible. | physical_system | domain_technical | unassessed; ledger PHY-01 |
| [ ] | PHY-02 | Mass, composition, volume and inertia: Source and validate applicable typed physical quantities, centre of mass and density without treating AABB volume as measured matter. | physical_component | domain | unassessed; ledger PHY-02 |
| [ ] | PHY-03 | Contact, collision, joints and friction: Qualified proxies and coefficients support intended interactions with stable contacts, material units and constrained motion. | contact_case | technical | unassessed; ledger PHY-03 |
| [ ] | PHY-04 | Gravity, terrain, wind, water and buoyancy: Applicable environment coupling has explicit inputs, scales, boundary conditions and stable response on the actual subject. | environment_case | technical | unassessed; ledger PHY-04 |
| [ ] | PHY-05 | Mechanical/thermal/optical/acoustic/electrical/chemical facets: Applicable response facets have sourced parameters and units; unsupported responses remain declared limitations. | physical_facet | domain | unassessed; ledger PHY-05 |
| [ ] | PHY-06 | Damage thresholds and fragment/part lineage: Applicable break/bend/burn/tear/dissolve/fail transitions tie cause and units to resulting parts, conserved/accounted resources and state. | physical_damage_case | domain_technical | unassessed; ledger PHY-06 |
| [ ] | PHY-07 | Numerical model and conservation evidence: Declare solver, timestep, tolerances, accounted quantities and stability limits; replay and stress cases use the actual selected solver. | physical_system | technical | unassessed; ledger PHY-07 |
| [ ] | PHY-08 | Twin calibration and observation contract: If operational twin is claimed, identify real referent, observations, calibration/synchronization/staleness and held-out validation; illustration cannot qualify it. | twin_model | domain_technical | unassessed; ledger PHY-08 |

## AUD — Audio, media and accessible presentation

Family: `all`; requirement trace: DTA-MEDIA, DTA-RIGHTS.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | AUD-01 | Subject audio/media inventory: Catalogue applicable calls, movement/contact, operation/fault, environmental response, images/video/plates and their provenance. | media_resource | domain | unassessed; ledger AUD-01 |
| [ ] | AUD-02 | Editable sound source/stems and processing: Retain native source sessions, samples/synthesis parameters, edits, stems and output settings; generated sound is labelled as such. | sound | technical | unassessed; ledger AUD-02 |
| [ ] | AUD-03 | Emitter, event and state association: Sounds/media bind to actual parts/actions/state with timing, loop/variation and environment applicability. | sound_event | domain_technical | unassessed; ledger AUD-03 |
| [ ] | AUD-04 | Spatial/temporal audio quality: Qualified sample rate, levels, channels, spatial response/occlusion and sync work on declared native playback; no clicks, drift or orphan emitters. | profile | human_audio | unassessed; ledger AUD-04 |
| [ ] | AUD-05 | Accessible descriptions and controls: Provide text/labels/captions and keyboard/discoverable controls; essential identifiers/state is not only colour, sound or motion. | profile | accessibility | unassessed; ledger AUD-05 |
| [ ] | AUD-06 | Media rights, bounded residency and unload: Encoded media is licensed, verified, budgeted and released on unload; required unavailable media is not hidden as a complete package. | media_resource | technical | unassessed; ledger AUD-06 |

## PKG — Complete HMC and native discovery

Family: `all`; requirement trace: REQ-HMC, REQ-SP, DTA-PKG.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | PKG-01 | Versioned complete HMC embodiment: Package the whole declared species or made-world embodiment, with entity/release identifiers, codec/schema/profile versions and source lineage. | one base claim | technical | unassessed; ledger PKG-01 |
| [ ] | PKG-02 | Packed complete graph and interpretation: Actual Q42 entries match reconciled available corpus, lexicon/vocabulary/shapes/provenance and access partition declarations. | one base claim | technical | unassessed; ledger PKG-02 |
| [ ] | PKG-03 | Root, part, profile and variant discovery: Native engine enumerates complete logical roots, components, representations and compatible combinations from the package itself. | one base claim | technical | unassessed; ledger PKG-03 |
| [ ] | PKG-04 | Exact dependency/resource inventory: Entries, roles, digests, sizes, shared resources, sources, rule/media/rig/material dependencies and required/optional distinctions reconcile. | one base claim | technical | unassessed; ledger PKG-04 |
| [ ] | PKG-05 | Artifact format/integrity validation: Run qualified HMC and interior-format checks over actual final bytes; hashes alone do not establish semantics, authorship or visual validity. | payload_type | technical | unassessed; ledger PKG-05 |
| [ ] | PKG-06 | Selective native admission and graph use: Query graph, plan resource closure and admit a selected whole root without loading all alternatives or the game; state/pick identifiers survives swaps. | profile | technical | unassessed; ledger PKG-06 |
| [ ] | PKG-07 | Offline self-contained interpretation and use: Required graph/resources/terms and declared operations work without external DNS/network dependencies; explicit extensions bind exact bases. | profile | technical | unassessed; ledger PKG-07 |
| [ ] | PKG-08 | Definition/instance/journal separation: Source HMC remains immutable; observations, simulation, review and user state have explicit ownership/base compatibility and durability. | one base claim | technical | unassessed; ledger PKG-08 |
| [ ] | PKG-09 | Transactional failure and cancellation: Corrupt/missing/cyclic/incompatible resources, exhausted budgets, stale results and interrupted loads retain the last admitted object and user work. | admission_negative_case | technical | unassessed; ledger PKG-09 |
| [ ] | PKG-10 | Rights/attribution/review and claims inventory: HMC exposes the per-resource rights and truthful per-deliverable claim/evidence status; no fabricated signatures or human approval. | one base claim | rights_technical | unassessed; ledger PKG-10 |
| [ ] | PKG-11 | Rebuild, version migration and rollback compatibility: Native rebuild and permitted upgrades preserve IDs/state and record schema/resource migration or explicit refusal without rewriting old releases. | one base claim | technical | unassessed; ledger PKG-11 |
| [ ] | PKG-12 | Package-level native dossier: Reviewer inspects actual decoded facts, parts, operation and all three representations using identified package bytes and public engine APIs. | one base claim | technical | unassessed; ledger PKG-12 |

## QA — Per-deliverable evidence and final disposition

Family: `all`; requirement trace: DTA-UAT, DTA-RUN, REQ-NAT.

| Resolved | ID | Deliverable and acceptance | Repeat | Required review | Claim |
|---|---|---|---|---|---|
| [ ] | QA-01 | Qualified native execution manifest: Record source/tool/library/binary/package identifiers, host/backend/device, settings, budgets, input/seed/time and actual invocation path. | qualification_run | technical | unassessed; ledger QA-01 |
| [ ] | QA-02 | Full expanded deliverable claim ledger: Disposition every catalogue row and every inventoried part/clip/behaviour/profile case; no missing/duplicate/vacuous/stale claims. | one base claim | technical | unassessed; ledger QA-02 |
| [ ] | QA-03 | Technical results and negative-case evidence: Evidence distinguishes source inspection, compile, executed check, runtime result and scope; failed/unexecuted checks remain visible. | qualification_case | technical | unassessed; ledger QA-03 |
| [ ] | QA-04 | Human visual acceptance of three profiles: Authorized reviewer accepts exact film, game and cartoon artifacts against declared criteria, including required movement and scene use. | profile | human_visual | unassessed; ledger QA-04 |
| [ ] | QA-05 | Scientific/design subject review: Required expert/domain review accepts claim scope, diagnostics/proportions/anatomy or mechanical/operational correspondence, and remaining uncertainty. | one base claim | domain | unassessed; ledger QA-05 |
| [ ] | QA-06 | Rights/accessibility/media review: Record applicable independent rights, accessibility and listening/media review results with actual evidence and authority. | review_domain | rights_accessibility_audio | unassessed; ledger QA-06 |
| [ ] | QA-07 | Evidence currency and derivation validity: Evidence binds exact output/input/dependency/settings digests; changes reopen affected claims and stale acceptance is never inherited silently. | one base claim | technical | unassessed; ledger QA-07 |
| [ ] | QA-08 | Declared limitations and authorized applicability decisions: Every unknown, blocked, partial, failed or proposed exclusion is reported; no agent self-waiver, target removal or invented completion percentage. | one base claim | technical | unassessed; ledger QA-08 |
| [ ] | QA-09 | Final completion computation and report: Independent gate checks all applicable rows/expansions/evidence/reviews and three target profiles; final report states complete only if no required work remains. | one base claim | technical | unassessed; ledger QA-09 |
| [ ] | QA-10 | Release dossier and reproducibility handoff: Retain editable source, HMC, inventories, claims, evidence/review references and native reproduction instructions; no publication or signing is inferred. | one base claim | technical | unassessed; ledger QA-10 |

## Completion statement

Link the full claim ledger and case inventories, exact sources/HMC receipt, technical evidence and required human decisions. State film/game/cartoon status, remaining open IDs, authorized N/A references and the actual native gate result. Until all applicable deliverables and cases are evidenced and accepted, the package remains incomplete. A bounded task can be delivered with its exact scope stated; it cannot silently become model completion.
