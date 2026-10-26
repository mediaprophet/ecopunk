# Biosphere, Earth and anatomy capability programme

**Status:** proposed capability and acceptance contract v0.1, 2026-10-26.
This extends existing requirements; it is not an implemented engine, new binary
format release, model acceptance, or biomedical qualification.

## Purpose and scope

Ecopunk's primary subject is the living Earth: flora, fauna (including insects
and other invertebrates), funga, microorganisms, habitats and the geology, soil,
water and atmosphere on which they depend. Human-made systems and civics
experiences build upon that substrate. A rigid equipment trial is useful tooling
evidence, but cannot represent progress across this primary biological scope.

The intended capability is a coherent native **authoring, knowledge, simulation
and presentation** system, not just a collection of rendering effects. It must
support comparative biology and anatomy, eventually including reviewed
biomedical education, through the same QualiaDB/Vibe/Poet/Etalon ecosystem.
Organism-specific models, measurements and domain review remain essential even
where mathematical mechanisms are shared. No claim of supporting every form of
life is justified by one tree, one vertebrate, or one generic procedural recipe.

The [architecture](../architecture.md), [development modes](development-modes.md),
[species HMC contract](assets/species-embodiment-spec.md),
[spatial substrate](spatial-substrate.md), [high-grade contract](high-grade-production-and-civics-experience.md),
[format foundation](format-sdk-foundation.md) and
[model claims](assets/model-completion-claims.md) continue to govern.
Poet remains the unified interface; native authoring requires no JS/Node/V8.
WASM and native-assisted WASM retain explicit per-operation qualification.

## What is actually available

The following is a bounded working-source review, reconciled with saved results.
Source presence, binding, execution and qualification are separate statuses.

| Capability | Existing evidence | Remaining qualification |
|---|---|---|
| Editable branch/tube/polygon geometry | Qualia turtle kernel and Core host; bounded native Poet connection now saved. [Eleven software tests and three branch renders](../../qa/implementation-2026-10-26/biological-reference/execution.md). | Species architecture, growth continuity, organs, materials, native plant controls, wind and complete species HMC. The fixtures are not biological stages. |
| Branch graphs and organic surfaces | Core `space_colonize_3d`, `tree_skeleton_3d`, `pipe_model_radii`, differential growth, reconstruction/remeshing and deformation source. [Exact paths](../../qa/implementation-2026-10-26/biological-reference/native-capabilities.md). | Safe public/native operator integration and reviewed domain models. Graphs are not meshes; scalar SDF blending is not a complete sculpt/meshing workflow. |
| Animal rigging and animation | Existing skeletal codecs/player, skin-weight authoring, IK, morphs and animated upload APIs. Same audit identifies the current static Poet viewport boundary. | Editable connected anatomy, weight corrections, clips/contacts, animated native preview, recovery and species-specific behaviour. Rigid motors or generic growth factors do not establish tissue physiology. |
| Fungi, insects and microorganisms | Reusable branching, surface, skeletal, optics and field mechanisms exist. Bounded searches did not establish a complete native subject-specific production workflow. | Hyphae/fruiting-body and colony models; arthropod articulation/wings/larval transitions; cell/colony data and representations. Reconcile existing owners before commissioning more kernels. |
| Geospatial fields | [Round 3](../../qa/implementation-2026-10-25/batch-6/rounds/round3/progress.md) records executed GRM3/M64 codec and native HMC inspection/reopen checks. Canonical geospatial source also has geology classification and corridor/fluid mutation APIs. | Real source-to-geology/soil/water compilation and consumption with CRS/datum/units/domain review. The developmental raster writer explicitly writes elevation-only strata and does not fabricate other fields. |
| Anatomy metadata and images | Existing Core anatomy HMC/CBOR metadata, DICOM metadata/pixel-range parsing and native ingestion, HU windowing, resampling, segmentation and MIP ingredients. | Registered/calibrated volume reconstruction, supported compressed decoding, label editing, measured geometry and native Poet integration. Existing anatomy placement is explicitly approximate normalized body space. |
| Scientific payload and volume rendering | `.10d` typed fields, scientific sidecars and voxel resources; Etalon has procedural SDF raymarching. | A reviewed sampled-volume/label/time/transform contract and actual consumer path. FIEL's inspected raster metadata is 2D; procedural SDF raymarching is not proof of DICOM volume rendering. |

These findings establish useful foundations, not comprehensive production or
biomedical education readiness. No new build/test or medical accuracy assessment
was performed for this specification review.

**Expanded reuse review, 2026-10-26:** the
[graph, spectra and streaming contract](biosphere-graph-spectra-and-streaming.md)
and [source review index](../../qa/implementation-2026-10-26/biosphere-integration-review/README.md)
extend the earlier bounded survey across computational geometry, physics,
modalities/logic, solvers/forge, vision/audio/rendering and semantic instruments.
In particular, `qualia-sim` already exports metamer growth, leaf hydraulics,
botanical continuum/contact coupling and articulated physics. These source
owners must be considered before allocating replacement work; their species
calibration, native Poet integration and execution evidence remain per-operation
qualification. Q42-linked complete HMCs, physically typed cues and query-selected
residency are shared foundations, not late visual polish.

## Required domain coverage

| Domain | Authoring and presentation capabilities required as applicable |
|---|---|
| Vascular plants | Root/stem/branch architecture; developmental organs and foliage; reproductive structures; anatomical sections; physiology/environment interfaces; wind/contact and regeneration. |
| Other flora and aquatic photosynthetic life | Appropriate thallus/frond/cellular or colony forms, attachment and reproductive/developmental models; no forced tree grammar. Resolve taxon concepts rather than relying on informal group names. |
| Vertebrates | Coherent external/internal anatomy and systems; sex/stage/condition variation; skin/muscle/bone/organ correspondence; fur/feathers/scales; rigging, contact, motion and behaviour. |
| Insects and other invertebrates | Segmentation, exoskeleton/soft-body distinctions, appendage joints and constraints, fine structures, wings and optical surfaces where relevant; evidenced development/metamorphosis, locomotion and interaction. Insects do not represent all invertebrates. |
| Funga | Applicable hyphal/mycelial networks, fruiting bodies, spores, tissues, substrates and symbiotic/decomposition relations; topology and temporal change. Do not reduce all fungi to mushrooms or treat a branching algorithm as validated fungal growth. |
| Microbial and cellular systems | Acquired microscopy/volumes/channels, membranes/internal structures where evidenced, colonies/biofilms, lineage and dynamics; appropriate resolution and measurement uncertainty. |
| Habitats and ecological communities | Individually identified organisms, populations/communities, canopy/understorey/litter/deadwood, trophic and symbiotic relationships, environmental fields and interactions; distinguish population dynamics from organism development. |
| Geology, landforms and soils | Terrain, lithology/mineral/soil properties, stratigraphy, faults, cavities, erosion/deposition, weathering and roots/soil interfaces; sourced dimensional fields and features, not merely a textured heightfield. |
| Water and atmosphere | Surface/subsurface flows, wetlands, catchments, moisture, transport and weather/light forcing, with declared solver/approximation and boundary conditions; visual water is not evidence of validated hydrology. |
| Comparative anatomy and biomedical learning | Intact bodies, named structures/systems, registered sections/volumes, histology and supported cellular scales; verified spatial relationships, measurements, normal variation and explicitly sourced pathology when included. |

Coverage is extensible by taxon/system and declared use. Unknown biology remains
unknown. This table does not mandate unsupported traits for every organism or
claim universal biological simulation.

## Shared capability series and dependency order

These BSA IDs are local requirement/work-packet identifiers, not new ontology
terms or a second implementation queue. Keep the governing IP phases and existing
ECO/engine IDs; use these as expansions of their acceptance scope.

| ID | Dependencies | Deliverable and acceptance | Existing owner / trace |
|---|---|---|---|
| BSA-01 | Existing domain/format contracts | **Multiscale scientific resource contract.** Describe sampled data, meshes, implicit surfaces, networks, materials, labels and temporal state with units, axes, physical extents, transforms, channels, uncertainty, referents and provenance. Preserve unknown/opaque data; declare schema/version compatibility and conversion loss. | Qualia graph/container/geospatial owners; ECO-021–030, G52–58, FND-01/02. |
| BSA-02 | BSA-01 | **Native acquisition and reconstruction.** Reuse existing scan, DICOM and geospatial ingestion. Declare each supported image/mesh/point/volume profile; preserve acquisition metadata and originals, bounded decoding and calibration. Prove supported and refused inputs before claiming format compatibility. | Existing native codec/ingestion owners; ECO-023–026/067, G74–75. |
| BSA-03 | BSA-01 | **Connected topology and organ networks.** Native curves/surfaces/solids/volumes, branching graphs, remeshing, subdivision and fine structure; preserve semantic components, UV/detail, weights and vertex/organ lineage through edits. Distinguish shared mathematics from validated organ models. | Qualia geometry + Poet; ECO-046–049/062/065, G53/G76. |
| BSA-04 | BSA-01/03 | **Development, variation and biological state.** Author evidenced stage transitions and local/environmental drivers, organ appearance/loss and compatible anatomy/rig/material/behaviour states. Preserve individual continuity; uniform scaling, unrelated meshes or population curves cannot stand in for development. | Vibe policy, native domain kernels, Poet timeline; ECO-064/081–086, G58–59. |
| BSA-05 | BSA-01/03 | **Biological and geological appearance.** Editable surface/volume optical properties, thin transmission/subsurface scattering, fur/groom/feathers, cuticle/wing effects, bark/leaf/rock/soil detail and wetness where relevant. Validate actual native light/material outputs and scale dependence against references. | Existing material/render/Etalon + Poet; ECO-051–060/063/066, G55/G85. |
| BSA-06 | BSA-01/03/04 | **Deformation and interaction.** Reuse rig/skin/morph/IK/contact and appropriate physics owners; add missing bindings. Author joint constraints, soft-body/tissue or fluid interactions only with declared material laws, units, parameters and limits. Qualify observed contacts and transitions, not just clip playback. | Native physics/skeletal/render + Poet; ECO-048/081–087, G55–59/G76. |
| BSA-07 | BSA-01/02 | **Scientific volume and section presentation.** Registered orthogonal/oblique slices, sampled-volume rendering, clipping, isosurfaces and label overlays; correct sample interpolation and optical compositing. MIP, cut planes, mesh cuts and true acquired sections are different operations. | Existing image/volume/renderer owners; ECO-017/067–068/096–097, G74–76/G85. |
| BSA-08 | BSA-01/03/07 | **Semantic inspection and measurement.** Pick the actual organ/tissue/stratum, query its graph, show units/scale bars and confidence, measure in the correct frame, section/explode/reassemble without altering original anatomy. Preserve labels and selection through LOD, pose and scale changes. | Qualia graph + Poet/Webizen; ECO-027–034/068/095–099, G52/G56/G88. |
| BSA-09 | BSA-01/02 | **Earth and habitat fields.** Connect real geology/soil/hydrology/atmospheric sources to semantic features, M64/FIEL/voxel resources and native consumer operations. Preserve source coverage, datum and uncertainty; refuse silent flat terrain or fabricated horizon substitution in qualified output. | qualia-geospatial + existing field/physics owners; ECO-031–040/091–094, spatial-substrate. |
| BSA-10 | Relevant BSA-03–09 | **Usable cooperative Poet authoring.** Integrated controls, viewport, organ/label/network editing, rig/timeline, materials, annotations and source REPL on one canonical owner. Prove permission, conflict, undo/rebuild, cancellation and HMC fresh-process save/reopen with contributor/reviewer work. | Poet/Aether/native host; ECO-044–050, IP-03 C1–C8. |
| BSA-11 | BSA-01/10 | **Complete embodiments and explicit derivatives.** Whole available graph/resource corpus and rights in HMC; editable masters, source references, measured conversion loss, preserved identifiers and offline interpretation. Film, game and cartoon remain mandatory package profiles; education is a separately declared use/accuracy profile. | Existing HMC/.10d/M64/Q42 owners; ECO-021–030/101–108/115, FND gates. |
| BSA-12 | Relevant BSA-01–11 | **Scientific, visual and educational acceptance.** Reference datasets and analytic phantoms, domain-reviewed morphology/anatomy/geology, known tolerances and uncertainty, actual native outputs and learning tasks. Technical pass, attractive image, human/domain review and educational acceptance are separate claims. | Existing dossier/review owners and domain educators; ECO-067/070/097/109–120. |

Do not make finishing all twelve domains a new serial prerequisite for the
white-gum or wallaby. Implement shared foundations once, qualify the operations
needed for each reference, and extend parallel domain cases. Biomedical learning
must not acquire an unsupported readiness claim from those early references.

## Biomedical education acceptance

"Biomedical education grade" is a declared, reviewed use profile, not a polygon
count, material preset or universal certification. Specify audience, learning
objectives, anatomical scope, representation accuracy, inspection/measurement
limits and required reviewers. Add all following applicable cases to the normal
pinned model dossier; retain the three existing production targets.

- **BSA-EDU-01, reference truth:** attributable anatomical sources, normal
  variation and evidential status; acquired, reconstructed, inferred and
  illustrative content remain distinguishable. A donor/specimen does not define
  every person's anatomy. Human source rights/authority remain distinct from
  the asset's distribution permission.
- **BSA-EDU-02, spatial fidelity:** physical sample spacing, slice ordering,
  axes/handedness/laterality, frame/registration and intensity calibration.
  Preserve original medical/microscopy coordinates with explicit transforms;
  do not force an organ into a ground-contact pivot or silently normalize it.
- **BSA-EDU-03, segmentation and reconstruction:** source-linked structures and
  masks, appropriate interpolation, correction history, uncertainty and measured
  boundary/topology error. Smooth or plausible surfaces are not proof of truth.
- **BSA-EDU-04, coherent anatomy:** named parts and connected systems align in
  intact, sectioned, exploded and moving views. Do not invent missing interior
  anatomy behind a decorative exterior; expose unavailable structures clearly.
- **BSA-EDU-05, measurement/display:** known-length phantoms, anisotropic voxels,
  flipped/order-shuffled slices, affine transforms, thin structures, overlapping
  labels and multiresolution sampling test physical measurements and visible
  boundaries. Declare error tolerances before acceptance; do not invent values.
- **BSA-EDU-06, process validity:** biomechanical/physiological models require
  their own reference parameters, boundary conditions and verification. A rigged
  animation, HU colour mapping or approximate allometry does not validate them.
- **BSA-EDU-07, learning usability:** anatomy/biomedical educator reviews actual
  lessons and interactions, structure identification, spatial understanding,
  accessible labels/controls and colour interpretation. Verify learning goals
  using a stated evaluation method; review signatures are actual human records.
- **BSA-EDU-08, recovery and lineage:** source/labels/measurements/annotations,
  corrections, graph references and rights survive native editing, HMC reopen,
  collaborative revision and derivative selection. Changed evidence invalidates
  affected claims. Education acceptance does not imply clinical-use validation.

## Interoperability and existing-source anchors

Use published data profiles as interoperability requirements, not a mandate to
replace the native containers or to import another runtime. Pin selected editions
and unsupported features in each adapter; a permissive parser is not conformance.

- [DICOM Segmentation](https://dicom.nema.org/medical/dicom/current/output/chtml/part03/sect_A.51.html),
  inspected PS3.3 2026d, distinguishes sampled classifications and their reference
  images/frames. [Surface Segmentation](https://dicom.nema.org/medical/dicom/current/output/chtml/part03/sect_A.57.html)
  describes surfaces separately. Preserve source acquisition dependencies.
- [OME-Zarr 0.5](https://ngff.openmicroscopy.org/0.5/) describes multiscale bioimages,
  axes/units, coordinate transforms, channels and integer label images. A future
  native adapter must retain these meanings and disclose unsupported codecs or
  transformations; no OME adapter is claimed installed here.
- [NLM Visible Human](https://www.nlm.nih.gov/research/visible/visible_human.html)
  is an example source corpus with anatomical cryosections/CT/MRI, not an adopted
  project dataset or evidence that source variation, ethics/rights or model
  acceptance have been settled. No dataset was downloaded by this review.

Inspected canonical working-source anchors (not new executed qualification):

- `C:/github/qualiaDB/crates/qualia-core-db/src/medical/dicom.rs:617`,
  `split_dicom_payload`: Part 10 parsing/pixel ranges. Recognition of compressed
  transfer syntax is not proof of decompression; inspected metadata does not
  establish calibrated, spatially registered volume ingestion.
- `C:/github/qualiaDB/crates/qualia-core-db/src/medical/dicom_ingest.rs:386`,
  `submit_dicom_ingest`: existing native worker/blob/WAL ingestion owner.
- `C:/github/qualiaDB/crates/qualia-core-db/src/render/anatomy_pack.rs:19`,
  `AnatomyOrganMeta`: existing organ metadata; approximate placement explicitly
  requires further registration work.
- `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/clinical/imaging.rs:10`,
  `hu_window`: bounded window mapping of caller-supplied HU samples, not acquisition
  calibration or a complete medical application.
- `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/field_section.rs:377`,
  `RasterGridMetadata`, and
  `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/manifold_identity_v2.rs:120`,
  `TypedFieldRecord`:
  reuse existing typed scientific resources; inspect dimensional limitations
  before proposing a registered 3D/time/channel payload extension.
- `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/voxel_section.rs:1`:
  existing VOX0 strata/cavity DAG data has spatial bounds, materials and moisture
  fields. It is not automatically a calibrated medical intensity/label volume;
  sampling precision, channels and interpretation require their own contract.
- `C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computer_vision/bio/medical/isotropic_resample_nn.rs:52`
  and `C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computer_vision/bio/morphology/watershed_markers.rs:16`:
  existing 3D nearest-neighbour
  resampling and **2D** marker watershed, respectively. Output-size rounding and
  dimensional scope need explicit acceptance; these are not a reviewed organ atlas.
- `C:/github/qualiaDB/crates/qualia-geospatial/src/raster_m64/writer.rs:8`
  and `src/corridor.rs:75`: canonical developmental elevation writer and corridor
  mutation. Reuse tested format work without inventing unpopulated soil data.
- `C:/github/etalon/crates/etalon-render/src/volumetric.rs:9`:
  procedural semantic SDF raymarching; distinguish it from sampled-volume rendering.

## Review and implementation boundary

Each scoped task records source/API, actual consumer, allowed files, units and
data contract, expected positive/negative cases, outputs and remaining IDs.
Missing shared mechanisms go to their current native owner; missing Poet
controls do not justify a duplicate engine. Do not copy Chora code across its
separate boundary. Reconcile existing implementation and evidence first.

Expand the existing model catalogue through subject-specific obligations before
producing or declaring complete the relevant embodiment. This document does not
silently revise the pinned catalogue, existing claims, `.10d` section schema or
HMC/M64 wire versions. Cross-family and educational acceptance remains open.
