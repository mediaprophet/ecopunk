# Interactive digital-twin assets — per-asset requirements

**Status:** proposed specification v0.1, for authoring, engine planning and UAT.
**Owner of scope:** Timothy. This specification is not an implementation or
engine-conformance claim. QualiaDB changes are requirements for Timothy to
allocate upstream; this work does not authorise engine edits.

Companions: [style and scale](style-and-scale.md), [media types](media-types.md),
[development modes](../development-modes.md), [10D semantics](../tensor-10d.md),
and [engine requirements](../engine-requirements.md#gap-12--interactive-digital-twin-assets).

## 1. Purpose and conformance

An asset must be more than a renderable mesh. Its identity, ontology, physical
model, variants, lifecycle, interactions and observable behaviour must form
one inspectable, versioned construct. High-resolution authoring must support
lower-capability devices without silently changing the entity or its rules.
The game and asset-review app are evaluation surfaces for QualiaDB/Webizen,
not substitutes for those capabilities.

**MUST** denotes a release requirement within the declared scope. **SHOULD**
denotes a recommendation whose omission needs a recorded reason. A family
requirement becomes mandatory when that feature/family is declared applicable.
A plant need not walk; a stone need not eat; a static schematic need not run a
rig. However, every asset MUST declare applicability, not silently omit fields.

Each requirement carries one of:

- `implemented-and-qualified`: linked producer/runtime evidence at a versioned
  device/profile, plus human acceptance where required;
- `implemented-unqualified`: source/artifact exists but required proof is open;
- `required-unimplemented`: needed for the declared scope and not delivered;
- `not-applicable`: a documented reason tied to the asset family;
- `unknown`: not yet investigated; not equivalent to not applicable.

An unsupported runtime feature is distinct from an inapplicable asset feature.
A consumer MUST state the limitation and its effect on interaction/fidelity.
A placeholder MUST remain labelled; counts, compilation and hashes do not
establish appearance quality or scientific correctness.

### Asset kinds

| Kind | Meaning and claim boundary |
|---|---|
| Taxon/type template | Represents a class, species or broader taxon; not an individual observation. |
| Authored/synthetic instance | A particular simulation entity with explicit assumptions and seed; not a measured real object. |
| Captured specimen/object | A particular captured subject at a time/place, with capture provenance and rights. |
| Operational digital twin | A particular referent with a declared model, observations/calibration and synchronization contract. |
| Component | A leaf, branch, wheel, bone or other part with its own identity and parent relations. |
| Assembly/aggregate | A complete logical object made of components, potentially packaged in HMC. |

A visually realistic model is not automatically an operational digital twin.
A genus/family model MUST NOT be relabelled as a verified species model.
Style, physical scale, visual resolution, life stage and simulation fidelity
are separate axes, not interchangeable interpretations of "quality".

## 2. Identity and minimum descriptor

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-ID-01 | MUST distinguish stable entity identity, asset-definition/version identity, physical-instance identity, component identity, state revision and payload digest. | Edits, LOD switches and animation preserve entity continuity; hashes are not substituted for entity IDs. |
| DTA-ID-02 | MUST identify referent kind, family, root object and component/assembly role. | A branch preview is labelled as a branch, not a whole tree. |
| DTA-ID-03 | MUST declare versioned formats, required engine/language/profile versions and compatibility range. | Unsupported formats/profiles diagnose rather than partially load as success. |
| DTA-ID-04 | MUST retain display names, language, descriptions, source authors/custodians, creation/capture dates and provenance. | Inspector can distinguish labels from authoritative identifiers. |
| DTA-ID-05 | MUST declare canonical physical units, coordinate convention, handedness, axes, origin/pivot, orientation and authored bounds. | Round-trip preserves dimensions; viewport normalization does not rewrite physical size. |
| DTA-ID-06 | MUST expose a feature/applicability record covering all requirement families below. | Omitted, unknown, unsupported and not-applicable features remain distinguishable. |
| DTA-ID-07 | MUST retain lineage for derivatives, replacement, split/merge and component creation/retirement. | A damaged/split object can be traced back without rewriting its history. |

Minimum descriptor groups: identity/referent; ontology; provenance/rights;
geometry/materials; assembly; fidelity/device profiles; physics; rig/clips;
behaviour/affordances; lifecycle/damage; mutable state; observations; dependency
closure; validation/UAT evidence. These are conceptual groups, not a claim
that new wire fields or ontology IRIs have been ratified.

### 2.1 Canonical SI Metric Frame & Species-Trait Dimensional Envelopes (`DTA-ID-05`, `DTA-SEM-05`, `DTA-PHY-02`)

To prevent scale inversions (such as a marsupial rendered larger than a 4WD vehicle or a tree compressed into a dimensionless viewport cube) and to ensure every asset is physics-conformant across open-world Earth and alternative narrative environments:

1. **1:1 SI Metric Authoring & Runtime Invariant (`1.0 unit = 1.0 m`):**
   - All whole-organism, vehicle, building, and terrain assets MUST be authored and sealed into `.10d` in **1:1 SI metres** (`+X` East/Right, `+Y` Up, `-Z` North/Forward, pivot at basal ground contact `y = 0.0 m`).
   - Micro-scale dissection and histological study assets (Scale 2–4 organs/tissues) MUST declare their exact metric unit multiplier in `DTA-ID-05` (`qudt:Millimeter` or `qudt:Micrometer`) so the engine transforms them into the metric world frame without ambiguity.
   - Multi-asset world and sector scenes MUST execute with `QualiaPortal::set_preserve_authored_frame(true)` (`G70`); automatic orbit-box normalization (`[-0.85, +0.85]`) is restricted strictly to isolated single-component UAT inspection when explicitly disclosed (`DTA-UAT-05`).
2. **Unified Species Trait Binding (`AusTraits` / `AFD` / `AVONET` $\to$ `.q42` $\to$ `.10d` Bounds & Physics Mass):**
   - Living assets (`bio:DarwinianTaxon`) MUST derive their authored bounding box (`AABB`), allometric growth stages (`DTA-LIFE-01..03`), tissue density, and `qualia-sim` rigid/articulated body mass (`DTA-PHY-02`) from QUDT-typed `.q42` trait statements (e.g. `entities/taxa/<slug>/traits.json` compiled into `.q42` NQuins).
   - Manufactured vehicles and equipment (`made:Artefact`) MUST derive their wheelbase, track width, overall dimensions, and curb mass from cited engineering specifications.
   - **Canonical Reference Envelopes (Fail-Closed Admission Gate `G69`):**

| Entity / Taxon | Ontological Class | Canonical Height ($H$, Y-axis) | Canonical Length / Spread ($L$) | Canonical Mass & Tissue/Material Density |
|---|---|---|---|---|
| ***Corymbia aparrerinja*** (Ghost Gum, mature) | `bio:DarwinianTaxon` (`APC:3180424`) | $12.0\text{ m} \le H \le 20.0\text{ m}$ | Crown spread $8.0\text{ m} \le W \le 16.0\text{ m}$; $\text{DBH } 0.40\text{–}1.10\text{ m}$ | Wood density $\rho = 880\text{ kg/m}^3$ (`AusTraits`) |
| ***Eucalyptus pauciflora*** (Snow Gum, subalpine mature) | `bio:DarwinianTaxon` | $6.0\text{ m} \le H \le 15.0\text{ m}$ | Crown spread $6.0\text{ m} \le W \le 14.0\text{ m}$; $\text{DBH } 0.30\text{–}0.85\text{ m}$ | Wood density $\rho = 790\text{ kg/m}^3$ (`AusTraits`) |
| ***Macropus giganteus*** (Eastern Grey Kangaroo, adult) | `bio:DarwinianTaxon` (`AFD`) | Standing height $1.50\text{ m} \le H \le 1.85\text{ m}$ | Head-body $1.10\text{–}1.40\text{ m}$ + tail $0.85\text{–}1.05\text{ m}$ ($L_{\text{total}} \approx 2.20\text{ m}$) | Adult mass $35.0\text{–}66.0\text{ kg}$; Achilles tendon $k_t \approx 180\text{ kN/m}$ |
| ***Vombatus ursinus*** (Common Wombat, adult) | `bio:DarwinianTaxon` (`AFD`) | Shoulder height $0.32\text{ m} \le H \le 0.40\text{ m}$ | Body length $0.85\text{ m} \le L \le 1.15\text{ m}$ | Adult mass $22.0\text{–}35.0\text{ kg}$ |
| ***Phascolarctos cinereus*** (Koala, adult) | `bio:DarwinianTaxon` (`AFD`) | Seated height $0.60\text{ m} \le H \le 0.85\text{ m}$ | Body length $0.60\text{ m} \le L \le 0.85\text{ m}$ | Adult mass $6.5\text{–}14.0\text{ kg}$ |
| ***Ornithorhynchus anatinus*** (Platypus, adult) | `bio:DarwinianTaxon` (`AFD`) | Dorsal height $0.12\text{ m} \le H \le 0.16\text{ m}$ | Total length $0.43\text{ m} \le L \le 0.55\text{ m}$ | Adult mass $0.9\text{–}2.4\text{ kg}$ |
| ***Aquila audax*** (Wedge-Tailed Eagle, adult) | `bio:DarwinianTaxon` (`AFD`/`AVONET`) | Perched height $0.85\text{ m} \le H \le 1.05\text{ m}$ | Wingspan $1.85\text{ m} \le W \le 2.32\text{ m}$ | Adult mass $3.0\text{–}5.3\text{ kg}$ |
| **NPWS 4WD Field Utility / Survey Rover** | `made:Artefact` (`DOM-AX-1`) | Overall height $1.85\text{ m} \le H \le 2.05\text{ m}$ | Overall length $4.70\text{ m} \le L \le 5.10\text{ m}$; width $1.82\text{–}1.95\text{ m}$; wheelbase $2.85\text{ m}$ | Curb mass $2,100\text{–}2,450\text{ kg}$; wheel radius $0.41\text{ m}$ |

## 3. Semantic metadata, ontology and scientific claims

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-SEM-01 | MUST provide inspectable RDF/Quin statements, graph/context identity and embedded term resolution linked to the asset's stable IDs. | Queries return the intended entity and component, not a guessed name match. |
| DTA-SEM-02 | Living assets MUST declare authoritative taxon identifiers, source/version, rank, identification confidence and nomenclatural ambiguity. | Species, genus and family precision is visible; unknown taxa remain data-only. |
| DTA-SEM-03 | MUST distinguish taxon concepts, individual organisms, materials, functional roles and rendered representations. | Destruction of one tree does not destroy its taxon concept; bark is not mistaken for the whole organism. |
| DTA-SEM-04 | MUST link part/whole, depicts/represents, habitat/locality, material composition and other applicable relations. | Component selection resolves the parent and the applicable material/taxon facts. |
| DTA-SEM-05 | Physical/biological properties MUST carry quantity kind, unit, measurement/model method, uncertainty, source, validity conditions and time. | Dimensional checks reject incompatible units; estimated values are not displayed as measurements. |
| DTA-SEM-06 | MUST distinguish observed, inferred, modelled, synthetic, contested and unknown claims. | A hypothetical behaviour or generic parameter is visibly qualified. |
| DTA-SEM-07 | MUST bind the semantic slice to the admitted asset/package revision and validate it with versioned shapes/rules. | Tampered or wrong-version metadata cannot silently describe a different payload. |
| DTA-SEM-08 | MUST label authored source preview, manifest declaration, pack-decoded RDF and live/session graph results separately. | The review app never presents regenerated source terms as extracted Q42 evidence. |

Reuse approved vocabularies and identifier conventions, including existing
Darwin Core/GBIF, provenance, quantity/unit, temporal and spatial mappings where
applicable. Do not invent ontology URLs or claim unavailable source data.
Domain vocabularies and mappings requiring ratification remain explicit work.

## 4. Complete objects, components and HMC packaging

A simple component/model may be a standalone `.10d`. The distributable package
for a complete object or asset family with multiple payloads or model/behaviour
variants MUST be an **HMC container**. Its `.10d` entries are model/component
payloads, not replacements for the overall package. HMC carries the applicable
Q42 semantics, models, materials/textures, rigs/clips, sounds, Vibe behaviour
definitions, variant manifest and other dependencies as one versioned release.

For example, a species family may package male and female models and applicable
behaviour sets, juvenile/adult stages and their LODs, sharing compatible common
resources. The manifest declares supported combinations; a full Cartesian
product of every variant dimension is not presumed or fabricated. A captured
or operational twin still represents a particular individual, not all family
variants simultaneously.

A container may hold several logical roots or variants. Selecting one MUST
load its admitted dependency closure, not every alternative, the entire game
or both male and female models by default. The consumer must expose selection
and inspection of packaged variants through the qualified engine/Vibe path.

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-PKG-01 | MUST declare one or more selectable logical root objects and their component/resource membership. | The consumer can distinguish container, root object, instance and leaf payload. |
| DTA-PKG-02 | MUST provide stable child IDs, local transforms, attachment points and parent frame/unit conventions. | Full-object assembly preserves relative sizes and applies each transform exactly once. |
| DTA-PKG-03 | MUST declare exact dependency references, types, versions and integrity digests; resources may be shared without losing role/identity. | Only the selected root's dependency closure is required; missing resources identify the affected component. |
| DTA-PKG-04 | MUST declare variant groups and compatibility between sex/phenotype or reproductive form where applicable, LOD, material, rig, life stage, behaviour and damage state. | The loader cannot combine incompatible models/rigs/behaviour sets or mix different package revisions. |
| DTA-PKG-05 | MUST declare format/codec version and verified index/manifest identity; the `.hmc` suffix alone is not a format contract. | The engine reader rejects unsupported variants; no ad hoc JS container parser is introduced. |
| DTA-PKG-06 | MUST verify selected payloads before use, including `.10d` sections, referenced media and semantic data. | Entry digests plus appropriate format checks run before material/behaviour execution. |
| DTA-PKG-07 | MUST support bounded, deterministic traversal; detect cycles, duplicate/conflicting IDs, missing children and invalid ranges/transforms. | Malformed assemblies fail with a precise dependency path and do not recurse without bounds. |
| DTA-PKG-08 | MUST distinguish required/optional resources and admit or replace a complete root transactionally. | Failure retains the prior admitted object; omitted optional features are disclosed. |
| DTA-PKG-09 | MUST support bounded package acquisition/caching and a qualified partial/range path for large objects. | Trusted index/root and per-entry integrity protect partial reads; Q42 BIDX is not treated as the HMC entry index. |
| DTA-PKG-10 | MUST keep published resources immutable and mutable instance/session state separate. | Preview edits or simulation never rewrite the source HMC; journals bind to the correct base/package. |
| DTA-PKG-11 | Multi-model/behaviour asset families MUST distribute a versioned HMC package with explicit variant IDs, membership, shared dependencies and per-variant overrides. | Male/female or other applicable variants are discoverable within the same family release without ambiguous naming or duplicated authority. |
| DTA-PKG-12 | MUST declare supported, unavailable, unknown and inapplicable variant combinations and their qualification evidence. | Missing female/juvenile/LOD/behaviour combinations are disclosed, not synthesized or silently substituted. |

Current scene-index/asset-table mechanisms may be reused if their contracts
fit. The upstream owner must ratify root/member schema and APIs; this spec does
not assert that a general nested-object HMC descriptor already exists.

## 5. Geometry, materials and visual fidelity

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-VIS-01 | MUST retain editable high-fidelity source and references for proportions, morphology, silhouette and material appearance. | Human review can compare the model against its declared species/object/style, not merely count triangles. |
| DTA-VIS-02 | MUST support exterior, relevant interior/cutaway/anatomical detail and declared inspection distances. | Advertised inspect views contain meaningful detail; an outer shell does not claim working internals. |
| DTA-VIS-03 | MUST validate topology, winding, normals/tangents, UVs, seams, submeshes, finite coordinates and bounds. | Errors are linked to components and source operations; topology exceptions such as foliage cards are declared. |
| DTA-VIS-04 | MUST distinguish render materials from physical material definitions while linking both to the same applicable parts. | Paint roughness is not used as mechanical strength; a failed look does not substitute for a failed physical state. |
| DTA-VIS-05 | MUST declare texture/map roles, colour space, sampling/mip/compression conventions and exact dependencies. | The same source material is interpreted consistently on qualified targets; missing maps are not hidden as successful fidelity. |
| DTA-VIS-06 | MUST support deterministic variation with source/seed and biologically or mechanically appropriate bounds. | Variation changes instance properties only where declared, not authoritative species facts. |
| DTA-VIS-07 | SHOULD retain spectral/capture truth where available and label RGB/audio projections or approximations honestly. | Claimed spectral data is evidenced; a baked texture or frame is a projection, not the complete editable construct. |
| DTA-VIS-08 | Procedural and algorithmic generation MUST be preferred over brute-force static asset inflation where applicable. Micro-surface textures (bark fissures, scale lattices, cellular venation, masonry cracking) SHOULD be expressed via compact procedural seeds and Voronoi/Delaunay/SDF kernels rather than multi-gigabyte baked texture cascades. | Generative seeds and parameters reproduce identical deterministic surfaces; payload bloat is avoided without loss of resolution. |
| DTA-VIS-09 | Dimensional morphs, growth ontogeny, and sexual dimorphic traits (horns, plumage length, bill curvature) SHOULD be parameterized via allometric scaling, logarithmic spiral ODEs, and harmonic morph bases rather than duplicate static meshes. | Continuous parameter spaces generate variants on-demand in caller-buffered memory; Cartesian asset duplication is rejected. |

Use the existing style/scale matrix. A hero asset must pass its target's visual
review before its LODs and siblings are called final-quality assets. Low-detail
proxies and presentation forms remain useful, but must not masquerade as the
high-resolution source or as scientific validation.

## 6. Resolution, LOD and heterogeneous devices

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-LOD-01 | MUST declare a highest-detail source and a device-appropriate resolution ladder, with derivation lineage. | At least high-detail inspection and constrained-device representations are evidenced for a multi-device claim. |
| DTA-LOD-02 | MUST separate geometry, texture, rig/animation, collision, sensor and simulation fidelity levels. | A mesh reduction does not implicitly lower logical state accuracy or change taxonomy. |
| DTA-LOD-03 | MUST declare each level's detail/error envelope and intended viewing scale/distance. | Silhouette, critical features, material roles and semantic picks meet the declared level's acceptance criteria. |
| DTA-LOD-04 | MUST select using qualified capabilities and resource budgets, not device-name guesses. | Report selected level, reason, actual backend and unsupported features. |
| DTA-LOD-05 | MUST preserve identity, pose/state, attachment continuity and interaction mapping across swaps. | Picking and persistent references still address the same object/component after a level change. |
| DTA-LOD-06 | MUST provide stable transitions/hysteresis and an admitted fallback or explicit refusal. | Boundary oscillation does not produce flicker, missing parts or unbounded reloads. |
| DTA-LOD-07 | MUST support instancing/culling/batched updates where applicable without merging away per-instance identity. | Large populations retain distinct state and picking, with measured bounded residency. |
| DTA-LOD-08 | MUST record native, standard-web, mobile, desktop, AR/VR and offline/linear-output coverage actually claimed. | A profile declaration is not a deployment receipt; low-detail rendering does not silently suspend authoritative life/behaviour updates. |

No universal triangle count, FPS or texture size is prescribed here. Budgets
and error limits belong to the selected profile and require measured evidence.
Style differences and molecule-to-biome scale changes are not LOD switches.

## 7. Physical properties and environmental interaction

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-PHY-01 | MUST declare the active physical model and approximation: static, kinematic, rigid, articulated, deformable, field-based or other qualified model. | An unavailable solver or approximation is visible, not advertised as full physics. |
| DTA-PHY-02 | MUST provide applicable mass, volume, density, inertia/centre of mass and material composition with units and evidence. | Bounding-box volume is not silently treated as measured solid/biomass volume. |
| DTA-PHY-03 | MUST define collision/contact geometry, filtering, friction, restitution, clearance and joint constraints where interactive. | Contact tests demonstrate stable placement and explain permitted penetration/proxy error. |
| DTA-PHY-04 | MUST define mechanical response where claimed: stiffness, damping, elasticity/plasticity, yield/failure, wear/fatigue and stress limits. | Load-response fixtures use typed, cited parameters, not visual material guesses. |
| DTA-PHY-05 | MUST define applicable thermal, optical, acoustic, electrical and chemical response under stated conditions. | Changes in temperature, phase, moisture or stress update the relevant model/facets coherently. |
| DTA-PHY-06 | MUST declare environment coupling: gravity, wind, fluid/buoyancy, water/soil, light, heat, pressure and terrain as applicable. | Unknown environmental inputs have a stated policy; defaults are not hidden measurements. |
| DTA-PHY-07 | MUST declare conserved/accounted quantities and accepted numerical tolerances for the model. | Mass/energy/material transfer is tracked or its approximation explicitly disclosed. |
| DTA-PHY-08 | MUST define time integration, timestep, solver/version, boundary conditions and admissible parameter ranges. | Reproducible fixtures test stability and reject invalid/unit-incompatible inputs. |
| DTA-PHY-09 | MUST keep physical state and the corresponding rendered/sonic response linked by identity and revision. | A deformation or material failure updates contacts, bounds, sound and appearance where claimed; visual-only effects remain labelled. |

Preserve Qualia's existing material/facet and 10D field conventions. Typed
physical quantities are not permission to invent new tensor axes or repurpose
identity, time, provenance or spectral lanes.

## 8. Animation, articulation and action presentation

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-ANIM-01 | MUST declare applicable rigid, skeletal, motor/procedural, morph/deformation and lifecycle presentation mechanisms. | A rotating static mesh is not claimed as a rigged organism. |
| DTA-ANIM-02 | MUST provide versioned joint hierarchy, bind/rest poses, skin influence limits and attachment semantics where rigged. | Invalid hierarchies/influences are rejected; authoring scale and parent transforms remain consistent. |
| DTA-ANIM-03 | MUST define named clips/actions with stable IDs, duration, looping, event markers and state/rig compatibility. | The viewer can discover and select relevant actions without hard-coded application clip names. |
| DTA-ANIM-04 | MUST support admitted transitions, blending, interruption, seeking, pausing and reset. | Cancelling an action does not strand contacts, attachments or authoritative state. |
| DTA-ANIM-05 | MUST distinguish root motion, presentation motion and actual state displacement. | Collision/permission admission controls world movement; an animation does not silently move the entity through obstacles. |
| DTA-ANIM-06 | MUST maintain normals/tangents, deformation bounds, shadows and semantic picks during motion. | Deformed geometry and its interaction surface agree in native/browser fixtures. |
| DTA-ANIM-07 | MUST synchronize relevant contact/events: feet/ground, mouth/food, hand/tool, wheel/road, pump/flow and similar relationships. | An eating clip alone does not assert food consumption; event timing has a defined state owner. |
| DTA-ANIM-08 | MUST supply reduced-motion/static alternatives without concealing action/state information. | Accessible presentation reports the same accepted action and result. |
| DTA-ANIM-09 | SHOULD support qualified retargeting and stage/LOD-compatible rigs rather than duplicating ungoverned action rules. | Retargeting retains semantic attachment IDs and produces a reviewed compatibility receipt. |

## 9. Damage, destructibility, repair and disposal

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-DMG-01 | MUST declare whether the object is damageable/destructible and why; unknown is not indestructible. | Static/non-destructible profiles have a visible reason; relevant assets have tested thresholds/states. |
| DTA-DMG-02 | MUST define applicable impact, load, cut, tear, fatigue, corrosion, burning, dissolution or other mechanisms. | Cause, exposure, units and model assumptions are recorded with damage events. |
| DTA-DMG-03 | MUST distinguish cosmetic, functional, structural and terminal damage. | A scratched surface, failed pump and detached branch are not interchangeable health bars. |
| DTA-DMG-04 | MUST bind damage to component topology, collision, articulation, material response and affordances. | A detached part no longer participates in the original joint/contact/function graph. |
| DTA-DMG-05 | MUST identify detached/fractured results and preserve split lineage and accounted quantities. | New parts/fragments have IDs and provenance; arbitrary debris visuals do not claim physical conservation. |
| DTA-DMG-06 | MUST define reversible repair, replacement, regeneration, irreversible loss and their resource/time constraints. | Repair cannot silently resurrect an irreversibly lost object or omit required inputs. |
| DTA-DMG-07 | MUST define bounded debris/residue/decay/disposal handling and retirement rules. | Repeated destruction respects object/resource budgets and retains auditable state. |
| DTA-DMG-08 | MUST provide repeatable damage/repair UAT scenarios, including invalid or denied actions. | The sandbox resets/replays safely without modifying published asset definitions. |

Damage animation is a presentation of a state transition, not proof that
fracture, material loss, biological injury or repair is physically modelled.

## 10. Growth, maturity, life stages and phenology

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-LIFE-01 | Living/aging assets MUST declare supported stages and transition criteria with temporal units. | Seed/juvenile/adult/senescent/dead states are distinguished where applicable; not every organism uses the same ladder. |
| DTA-LIFE-02 | MUST declare growth/aging models, parameters, environmental drivers and uncertainty. | A genus-wide proxy is labelled; measured species/individual growth is not invented. |
| DTA-LIFE-03 | MUST link stage to geometry, proportions, material/facets, rig, physics, affordances and behaviour. | Growth is not just uniform mesh scaling or an unrelated texture swap. |
| DTA-LIFE-04 | MUST preserve individual continuity and record new/retired components and reproduction lineage. | A new leaf/offspring/seed is not confused with a new version of the taxon concept. |
| DTA-LIFE-05 | MUST support relevant seasonal/phenological state: dormancy, flowering, fruiting, shedding, moulting or equivalent. | Calendar/weather/context and stage prerequisites are explicit. |
| DTA-LIFE-06 | MUST account for relevant water, nutrients, energy/biomass and habitat constraints. | Growth/feeding fixtures expose deficits and declared approximations. |
| DTA-LIFE-07 | MUST distinguish simulation time, observed time and accelerated preview time. | Fast-forward does not claim years of measured real-world evidence or alter timestamps silently. |
| DTA-LIFE-08 | MUST model applicable disease/stress/injury, recovery/regrowth, death and decomposition with evidence boundaries. | Relevant failure and irreversibility cases are reproducible; population dynamics is not presented as individual development. |
| DTA-LIFE-09 | MUST declare slow accrual, maturity value and irreversible/time-horizon loss where relevant. | Removing an old hollow tree cannot be treated as immediately replaced by a seedling of equal value. |
| DTA-LIFE-10 | MUST declare applicable sexual dimorphism, reproductive forms/conditions and their life-stage compatibility, with source and uncertainty. | Male/female models are supported where the taxon requires them; shared morphology, other biological systems, unknown sex and not-applicable cases are represented honestly. |

Sex/phenotype is not inferred from a display name, model appearance or behaviour.
Sex and gender are not interchangeable metadata. Individual traits remain
separate from selectable family templates: an LOD/style change cannot change
an individual's sex or identity. Biological transitions occur only where a
qualified taxon-specific lifecycle model explicitly permits them. Comparing
family variants in UAT is not a mutation of a measured individual's state.

## 11. Behaviour, affordances and functional systems

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-BHV-01 | MUST expose discoverable actions/affordances with preconditions, required capabilities, participants, resources and expected state effects. | The same descriptor drives authoring, UAT and consumer interaction. |
| DTA-BHV-02 | Animals MUST declare applicable locomotion, resting, feeding/drinking, grooming, exploration, avoidance and reproductive behaviour. | Every claimed behaviour connects to relevant state/contact/resource changes, not only a clip. |
| DTA-BHV-03 | MUST declare social/group behaviours where relevant: following, flocking, herding, communication, cooperation, conflict or care. | Actor/recipient/group IDs and environmental/temporal conditions are explicit. |
| DTA-BHV-04 | Plants/fungi and other non-locomotive life MUST declare relevant growth, tropism, exchange, propagation and environmental response. | Lack of walking does not mean the organism has no dynamic behaviour. |
| DTA-BHV-05 | Mechanical/constructed objects MUST declare applicable operation, controls, energy/material flows, capacities, failure and maintenance. | A turning pump visual is not evidence of pumped volume or consumed energy. |
| DTA-BHV-06 | MUST distinguish observed behaviour, modelled policy, internal-state hypotheses and stochastic assumptions. | An inferred intention is not displayed as an observed fact. |
| DTA-BHV-07 | MUST define deterministic scheduling, seeded randomness, triggers, priorities and cancellation. | The same admitted event tape and seed reproduce the declared outcome. |
| DTA-BHV-08 | MUST separate authoritative Vibe/engine policy from animation and browser controls. | JS transports input/results only; missing Vibe functionality becomes an upstream requirement. |
| DTA-BHV-09 | MUST respect rights/capability/physical constraints and isolated proposal/what-if boundaries. | Denied actions have explanations and do not mutate state or coerce acceptance. |
| DTA-BHV-10 | MAY use learned/local-agent proposals with explicit model/version, budgets and review/admission. | A model is not the root of authority; ungrounded proposals do not silently become asset facts. |
| DTA-BHV-11 | MUST declare shared and variant-specific behaviour/animation/physiology where evidenced, including applicable sex, reproductive condition, life stage and context. | Differences are authored and sourced, not assumed stereotypes; incompatible mating, feeding, care or other action sets are refused or explicitly qualified. |
| DTA-BHV-12 | Every digital twin representation (flora, fauna, funga, or constructed object) MUST embed a self-describing **Personal Behavioral & Affordance Ontology** within its Q42 semantic slice. This ontology MUST declare discoverable affordances (`wot:ActionAffordance`, `schema:Action`), internal drives/states, and machine-readable preconditions/effects to enable direct **AI agent triggering, LLM tool-calling, and multi-agent scriptability** without client-side hardcoding. | External AI and player agents query the twin's affordances dynamically via SPARQL/Vibe; action intents validate against the twin's published SHACL shapes before execution. |
| DTA-BHV-13 | Autonomous behaviours MUST be governed by declarative semantic rule sets (N3/Datalog forward-chaining over the twin's personal ontology beliefs), triggering state transitions and emitting cryptographically verifiable 48-byte NQuin action intents to the session journal. | Entity autonomy executes within the 42MB Sentinel zero-heap evaluator; behaviour outcomes are deterministic and auditable. |

## 12. Mutable state, journal, replay and coordination

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-STATE-01 | MUST separate immutable definition/package data, instance state, authored rules and observations. | Simulation or review cannot overwrite the source asset or another session. |
| DTA-STATE-02 | MUST version state transitions and their causality, actors, time, rule versions and base identities. | Events resolve to the intended asset/instance and admitted dependency versions. |
| DTA-STATE-03 | MUST reuse existing Q42 mutable-session/journal machinery where applicable, rather than introduce an unrelated game-state implementation. | `.q42j` existence is acknowledged; the selected host/storage path is independently qualified. |
| DTA-STATE-04 | MUST support atomic transactions, checkpoint/export/reload and failure recovery within declared durability semantics. | Torn writes, corrupt commits and wrong-base/package journals do not silently recover as valid state. |
| DTA-STATE-05 | MUST support reset/seek/replay and explicit branch/what-if execution for UAT. | Sandbox branches cannot pollute the admitted real/session state. |
| DTA-STATE-06 | MUST declare synchronization/conflict rules, including a single authoritative writer or qualified distributed ownership. | Duplicate action application and stale snapshots cannot create contradictory identities/state. |
| DTA-STATE-07 | MUST report in-memory, exported, OPFS-persisted and restart-recovered status separately. | Exported bytes or a successful in-memory replay do not claim browser durability. |
| DTA-STATE-08 | MUST preserve state through permitted LOD/style/rig presentation changes. | A device downgrade or clip interruption cannot reset age, damage, hunger or maintenance history. |

## 13. Observations and operational digital-twin fidelity

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-TWIN-01 | Operational twins MUST identify the particular referent and limits of the representation. | A species template or synthetic vehicle is not falsely linked to a real individual. |
| DTA-TWIN-02 | MUST declare sensors/data sources, calibration, units, coordinate transforms, sample intervals, uncertainty and rights. | Each live value is traceable to an authorised source and time. |
| DTA-TWIN-03 | MUST distinguish observation, estimate, prediction and control command. | A forecast or simulation result cannot overwrite a measurement without a recorded derivation. |
| DTA-TWIN-04 | MUST define freshness, dropout, outlier, missing-data and clock-disagreement policies. | Stale data is visible; unknown values are not silently set to zero. |
| DTA-TWIN-05 | MUST specify reconciliation/data assimilation and validity/error envelopes for the model's intended use. | Model-observation divergence is inspectable, not concealed by changing the view. |
| DTA-TWIN-06 | MUST separate read-only observation from authorised actuation with explicit admission and safety bounds. | Preview controls cannot send real-world commands, payments or device actions. |
| DTA-TWIN-07 | MUST declare spatial/temporal scale, model applicability and limits of extrapolation. | An illustrative growth or failure curve is not represented as engineering/clinical certification. |
| DTA-TWIN-08 | MUST provide controlled replay of recorded observations and held-out validation where a predictive claim is made. | Quality/error claims link to the exact dataset, model and validation procedure. |

## 14. Media, accessibility, rights and provenance

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-MEDIA-01 | MUST declare applicable sound/call/operating-noise resources and their semantic/action association. | Sound events resolve to the correct emitter, state and timing. |
| DTA-MEDIA-02 | MUST declare source media, spectral/response data if claimed, spatialization and device projection. | A synthetic loop is not labelled a field recording or measured acoustic response. |
| DTA-MEDIA-03 | MUST provide textual descriptions, discoverable controls, keyboard access and reduced-motion/audio alternatives. | Essential state and affordances are not communicated only by colour, sound or motion. |
| DTA-MEDIA-04 | SHOULD support inspection annotations, labels, cutaways and accessible views across scale/LOD. | Annotations follow component identity rather than fragile mesh indices. |
| DTA-MEDIA-05 | MUST bound media decoding/residency and synchronize it with the selected object/profile. | Package switching releases old media and does not leave hidden emitters running. |
| DTA-RIGHTS-01 | MUST declare licence, attribution, derivative obligations, custody and sensitivity per resource and package. | Aggregate admission respects all applicable obligations, not just the root's label. |
| DTA-RIGHTS-02 | MUST retain captured/authored/procedural/AI-assisted provenance, source tools/versions, transformations and human review. | Generated or modified material is not passed off as captured evidence. |
| DTA-RIGHTS-03 | MUST enforce applicable consent, cultural/specimen/locality restrictions and private-data boundaries. | A public model does not imply public rights to precise locality or personal observations. |
| DTA-RIGHTS-04 | MUST validate declared executable capabilities and refuse unsupported/ungranted behaviour without a substitute execution tier. | Opening an HMC does not grant arbitrary scripts, networking or filesystem access. |
| DTA-RIGHTS-05 | MUST expose authenticity/integrity scope honestly. | A payload hash match is not misrepresented as publisher authentication or scientific validity. |
| DTA-RIGHTS-06 | MUST retain human acceptance decisions separately from automated format/function checks. | Passing tests does not manufacture art/species-fidelity approval. |

## 15. Authoring, compilation and resource management

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-AUTH-01 | MUST provide a Vibe-first authoring contract over public engine APIs, with retained editable sources. | Missing language/host functionality is advised upstream; no JS policy/physics/ontology replacement. |
| DTA-AUTH-02 | MUST support approved mesh/scan/CAD/rig/material/data imports with explicit conversion/loss reports. | Imported scale, topology, semantic IDs and rights survive or the loss is reported. |
| DTA-AUTH-03 | MUST derive and validate LOD, collision and compatible rig/material variants from the declared source. | Conversion does not invent missing physical/species parameters. |
| DTA-AUTH-04 | MUST author and inspect whole objects and independently editable components. | Per-component edits update affected dependency/version records and aggregate bounds. |
| DTA-AUTH-05 | MUST compile/export selected assets or HMC roots incrementally, without requiring a whole-game rebuild. | Unchanged geometry/rules/resources are not regenerated to review one model. |
| DTA-AUTH-06 | MUST be reproducible from declared toolchain, data, parameters and seeds; volatile metadata is separately identified. | Repeated equivalent authoring inputs produce matching content or a declared, evidenced reason for differences. |
| DTA-AUTH-07 | MUST produce linked semantic, dimensional, integrity, compatibility and authoring validation reports. | Each result names its scope/version and failed requirement IDs. |
| DTA-AUTH-08 | MUST support cancellation and owned bounded scratch/artifact retention. | Success, failure and cancellation do not fill disk or destroy unrelated user work. |
| DTA-RUN-01 | MUST declare CPU/GPU/decoded-media/transfer budgets and limits on joints, components, fragments, behaviours and instance counts. | Admission occurs before excessive allocation; aggregate budgets are distinct from per-object budgets. |
| DTA-RUN-02 | MUST conform to applicable Qualia Sentinel, 48-byte Quin, caller-buffered hot-path and deterministic execution contracts. | Real allocation/budget evidence qualifies the relevant kernels; no blanket construction exemption. |
| DTA-RUN-03 | MUST avoid per-frame graph-text parsing, unrelated recompilation and redundant deformation/upload. | Dirty-span and affected-component receipts identify actual work. |
| DTA-RUN-04 | MUST define cancellation, stale-load prevention, unload/disposal, cache expiry and device-loss recovery. | Failed replacements preserve the prior admitted view/state without hidden resource growth. |
| DTA-RUN-05 | MUST declare and qualify each native/browser/device feature profile and safe degradation path. | Backend/profile selection is actual evidence, not user-agent inference. |
| DTA-RUN-06 | MUST report measured cold/warm load, residency, action/animation cost and recovery for claimed targets. | Measurements retain workload/device/artifact identity; kernel timings are not extrapolated to full-game performance. |

## 16. Required asset-review app capabilities

The existing single-`.10d` viewer is an initial slice, not full conformance to
this section. HMC/complete-object preview and the following interactions are
requirements to implement/qualify through the engine, not permissions for
browser-side reimplementation.

| ID | Requirement | Acceptance/evidence |
|---|---|---|
| DTA-UAT-01 | MUST load a standalone `.10d` or an HMC package, list logical roots and distinguish complete-object/component modes. | The reviewer can preview a whole banksia/vehicle and drill into a branch/wheel without loading the game. |
| DTA-UAT-02 | MUST inspect verified package manifest/index, dependency closure, sizes/versions/digests and admission failures through engine APIs. | Corrupt/missing nested resources identify the exact failure; no custom JS HMC parser. |
| DTA-UAT-03 | MUST show actual pack-decoded RDF/lexicon, identity, physics, units, provenance and rights alongside authored-source comparisons. | Source-preview and extracted-data panels cannot be mistaken for each other. |
| DTA-UAT-04 | MUST expose LOD/style/device profile selection, actual selection reason and fidelity/resource limits. | Forced profile tests retain semantic/state identity and expose unsupported features. |
| DTA-UAT-05 | MUST support stable inspection cameras, physical scale references, component isolation and applicable cutaway/debug views. | Viewport normalization is disclosed; scale, pivots and attachment relationships remain inspectable. |
| DTA-UAT-06 | MUST discover/play/scrub/pause/reset applicable clips and inspect rig/contact/semantic mappings. | Animation tests demonstrate the declared object rather than a generic unrelated animation. |
| DTA-UAT-07 | MUST exercise applicable damage/destruction/repair transitions in an isolated, resettable session. | Event, fragment/component lineage, physics/appearance and replay results are visible. |
| DTA-UAT-08 | MUST preview growth/life-stage/phenology under controlled time/environment and inspect the underlying state. | Age/stage changes are not confused with zoom/LOD or replaced by decorative scaling. |
| DTA-UAT-09 | MUST exercise behaviour/affordances with controlled stimuli and participants, explaining denied actions. | Eating/moving/social actions show actual admitted effects and synchronized presentation. |
| DTA-UAT-10 | MUST record reproducible session evidence: object/component IDs, package/base digests, source/rule versions, engine build, device/backend, camera, inputs, seed, time and state/journal. | Another qualified run can replay the same declared scenario. |
| DTA-UAT-11 | MUST retain screenshots/video/engine receipts and human notes as distinct evidence. | A presented-frame receipt or passing assertion does not approve appearance automatically. |
| DTA-UAT-12 | MUST support negative qualification: corruption, unit mismatch, unsupported version/profile, denied capability, cycles, missing LOD/clip/resource, wrong journal base, exhaustion and stale loads. | Failures are explained, leave admitted state intact and never trigger "accept or revert" coercion. |
| DTA-UAT-13 | MUST discover and preview packaged model/behaviour variants, including male/female forms where applicable, and test compatible stage/rig/LOD/action combinations. | The same HMC family can be inspected variant-by-variant, with shared resources, exact selection provenance and rejected unsupported combinations; no other variant is loaded implicitly. |

## 17. Family applicability and representative qualification fixtures

All families must satisfy identity, semantic scope, provenance/rights,
package/dependency declaration, visual-profile declaration and inspection.
The table identifies additional applicable areas; it does not replace the
per-asset applicability record.

| Family | Additional requirements to consider | Representative fixture |
|---|---|---|
| Plant/tree | Root/branch/leaf/flower components; growth/phenology; water/nutrients/light; wind response; damage/regrowth/death; accrual. | One complete species-qualified tree HMC, leaf/branch inspection, seedling-to-mature states, wind and a recorded branch-loss event. |
| Animal | Morphology/rig; locomotion/contact; feeding/metabolism; social behaviour; life stages, injury/death; sensory/audio. | One identified animal with walk/rest/eat and a bounded two-actor social scenario, plus stage/LOD continuity. |
| Human/avatar | Articulation/contacts; accessible action presentation; consent/privacy; relevant social/activity affordances; clearly qualified identity/state. | Tool/contact task with an authored action, interruption and permission denial; no invented psychological facts. |
| Vehicle/machine | Hierarchical assemblies, joints, controls, energy/thermal/fluid flows, functional failure/repair and operating sounds. | One HMC vehicle/pump assembly with moving parts, typed consumption, a component failure and a resource-constrained repair. |
| Building/infrastructure | Assemblies/interiors; load/contact; doors/services; weathering; structural/functional damage and maintenance. | Complete workshop structure with operable door/service, component isolation and declared structural model. |
| Rock/material/terrain | Provenance/geology/material identity; physical bounds/contact; applicable fracture/erosion/water/thermal response. | A cited material/body or terrain tile with a qualified contact/damage/erosion fixture; life stages marked inapplicable. |
| Molecular/cellular/scientific | Typed quantities, scale/time conventions, scientific model validity, fields/reactions where qualified. | A declared species/reaction or cellular model with dimensions, uncertainty and model-vs-observation boundaries. |
| Symbolic/schematic | Semantic identity, explanatory content, accessibility and explicitly limited physical/animation claims. | A diagram/marker linked to the same entity; no implied full-physics fidelity. |

Fixtures must be selected and approved for their intended use. A complete
object preview must not be silently reduced to the first component that loads.
One successful species/family fixture does not qualify all assets.

## 18. Definition of done and upstream handoff

For an asset release:

1. Scope, family, referent and applicability are explicit; required features
   are implemented and qualified or the release remains a labelled prototype.
2. The editable source and component/variant graphs are retained, with rights
   and stable identity distinct from content hashes and manifold coordinates.
3. Formats/dependencies/semantics and dimensions pass their qualified engine
   gates at the declared versions. Whole-object HMC admission is demonstrated
   if that is the packaging form.
4. Device/LOD, animation, physics, life/damage/behaviour and replay cases pass
   for the applicable profile; limitations and failed cases remain recorded.
5. Human appearance/species-fidelity acceptance and the scientific validation
   required by any digital-twin claim are present. Automated checks cannot
   substitute for either.
6. A reviewer can inspect the selected object and reproduce its declared UAT
   scenarios without recompiling or loading the entire game.

Implementation allocation and capability evidence are in engine register
G52–G65. Existing LOD, skeletal, physics/material, HMC and Q42J code must be
reused or qualified first. Source inspection is not proof of shipped runtime
coverage. Necessary API/schema/codec extensions must be justified, versioned
and allocated upstream by Timothy, with existing assets kept compatible.
This specification does not itself assign engine-edit authority, ratify new
vocabulary namespaces or permit a private browser replacement implementation.
