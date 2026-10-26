import json,pathlib,struct,hashlib
out=pathlib.Path(r'C:\github\ecopunk\docs\qa\implementation-2026-10-25\batch-6\rounds\round1\producer')
asset=pathlib.Path(r'C:\github\ecopunk\assets\reference\community-grounds-growing-station\outputs\round1-film-006')
core='C:/github/qualiaDB/crates/qualia-core-db/src/'
poet='C:/github/poet/native/'
blob=(asset/'growing-station.10d').read_bytes()
table,count=struct.unpack_from('<II',blob,56)
names={1:'QuantizedMesh',2:'Tensor10DNodes',7:'ProvenanceSidecar',8:'FieldSidecar',10:'Topology',11:'SpatialIndex',12:'SurfaceReading',13:'Materials',14:'TextureCoordinates',15:'ManifoldIdentityV2',16:'SkeletalAnimation',17:'EntityOntology',18:'ProceduralSynthesis',19:'ImpostorAtlas',20:'HierarchicalSurfels',21:'HyperspectralCube',22:'VoxelLattice'}
sections=[]
for i in range(count):
    kind,tier,res,offset,length,stride,elements,crc=struct.unpack_from('<BBHIIIII',blob,table+i*24)
    sections.append(dict(tag=kind,name=names.get(kind,'other'),offset=offset,bytes=length))
rows=[]
def row(id,area,presence,wiring,run,qualify,owner,evidence):
    rows.append(dict(id=id,area=area,source_presence=presence,wiring=wiring,executed_006=run,qualification_or_evolution_work=qualify,owner=owner,evidence=evidence))
row('FMT-01','Envelope, integrity and version evolution',
 '64-byte v1/v2 header; canonical ordered section table, alignment, per-section/whole-file CRC; unsupported/duplicate tags rejected.',
 'Public Container10dHeader::parse, encode_container, parse_section_table and integrity seal/verify functions; compile_mesh_to_10d uses them.',
 'Actual006 sealed .10d emitted and decoded; exact byte reconstruction from reopened HMC succeeded before render admission failed.',
 'Keep backward readers and explicit migrations; qualify richer optional/required section negotiation and unknown-data preservation before promising arbitrary forward compatibility. CRC is integrity evidence, not authenticity.',
 'Core container_10d', [core+'container_10d/header.rs:31',core+'container_10d/section.rs:552',core+'container_10d/section.rs:638',core+'container_10d/integrity.rs'])
row('FMT-02','Master precision versus mesh derivatives',
 'QuantizedMesh stores u16 positions inside f32 bbox; public Mesh holds f32 positions. This is an existing compact derivative representation.',
 'Poet compile_assembly calls compile_mesh_to_10d_with_frames; source comment explicitly separates editable master/Q42 graph from dense derived geometry.',
 '006 has239952vertices/79984triangles, u16-coordinate derivative;16endpoint vertices exposed one-f32-ULP decoder overshoot. Decoder correction saved; native verification pending at audit creation.',
 'Preserve editable source, analytic/procedural intent and declared tolerances. Qualify high-precision/large-coordinate masters and loss/error profiles; if unsupported, add an explicitly versioned representation rather than making u16 mesh the master ceiling. Encoder wide-finite-frame behavior is not qualified by decoder-specific tests.',
 'Poet authoring + Core precision codecs',[core+'container_10d/mesh_section.rs:1',core+'render/assets/mesh.rs',poet+'src/graphical/assembly/compile.rs:1',str(out/'decoded-bounds-probe.json')])
row('FMT-03','Editable parametric and non-destructive composition',
 'Vibe source parameters, identified parts, parent-relative TRS, material edits and rebuild source exist; procedural descriptors also exist separately.',
 'Poet NativeProject/SessionEditorHost/EditorHost authoring_edit persist canonical source cells; HMC resources keep master,preset,accepted source plus part graph.',
 '006 evaluated master+preset, applied parameter/material/transform edits, retained5sourcecells and199parts, then reopened and regenerated exact .10d.',
 'Qualify usable contributor/reviewer editing, dependency versions, undo/rebuild and non-destructive history across media/profiles. Do not infer a full CAD constraint/BRep/composition system from this recipe or transplant a private state engine into .10d.',
 'Poet canonical authoring, Core HMC/checkpoints',[poet+'examples/growing_reference/produce.rs',poet+'examples/growing_reference/package.rs',poet+'src/graphical/authoring/source.rs',poet+'src/graphical/authoring/tests.rs'])
row('FMT-04','Materials and texture dependencies',
 'Versioned MaterialRecord/SubmeshRange, PBR/stylized shading, texture samplers/transforms and validation APIs exist.',
 'attach_material_section/read_material_section_into/validate_material_bindings are used by Poet assembly compile/decode; broader texture rendering requires separate consumer qualification.',
 '006 MAT section emitted/read with stable per-part semantic/material ranges; source carries9materials. Pixel output not yet qualified.',
 'Exercise actual texture resources, colour management, tangent-space dependencies, alpha/transmission/stylization and master-to-film/game/cartoon fidelity. Preserve explicit dependency identities and rejection of missing required companions.',
 'Core material/render + Poet controls',[core+'container_10d/material_section.rs',core+'container_10d/material_container.rs',poet+'src/graphical/assembly/compile.rs',poet+'examples/growing_reference/render.rs'])
row('FMT-05','Normals, tangents and UV streams',
 'NRM1/FRM1 normals/tangents use octahedral encoding; UV01 is vertex-aligned f32 data with bounded validation.',
 'encode_mesh_frames/decode_mesh_frames and attach_texture_coordinate_section/read_texture_coordinate_section_into are public; Poet compile uses normals+UV.',
 '006 contains FieldSidecar normal stream and UV section;239952entries each decoded; independent numeric probe found finite nonzero normals and validUV. No tangent stream or pixel-quality pass claimed.',
 'Qualify normal/tangent/UV coherence, seams and precision under skin/morph/deformation and each production profile; stream presence alone is not visual acceptance.',
 'Core geometry/frame codecs and rendering',[core+'container_10d/normal_section.rs:1',core+'container_10d/texture_coordinate_section.rs:1',poet+'examples/growing_reference/render.rs:26'])
row('FMT-06','Rigging, clips and morph targets',
 'SKL1 typed joint/skin/clip codecs and MRP1 sparse/dense quantized morph codec/apply kernel exist.',
 'read_skeletal_section/attach_skeletal_section_into and SkeletalSection/encode_skeletal_section_into are public; render/portal/scene_mesh.rs consumes SKL1. MorphTargets currently aliases FieldSidecar; section table forbids duplicate types.',
 'Not included/exercised by static equipment006; this absence is not a capability absence.',
 'Qualify actual animated authored assets, stable clip/joint identifiers and transitions. Reconcile simultaneous normal+MRP1+scientific payload composition in the shared FieldSidecar envelope before claiming complete coexistence; test exact packaging/consumer compatibility.',
 'Core animation/envelope + Poet timelines',[core+'container_10d/skeletal_section/mod.rs:1',core+'container_10d/skeletal_container.rs:39',core+'container_10d/morph_section.rs:38',core+'render/portal/scene_mesh.rs:152'])
row('FMT-07','Identifiers, manifold identity and graph continuity',
 'MID2 StableEntityId/IntegrityIndexDigest are distinct types; profile/domain/convention plus typed field references are explicit. Tensor10D AoS/SoA node codecs exist.',
 'encode/decode_container_manifold_identity and compile_mesh_to_10d_with_identity/attach_manifold_identity_v2_into connect manifest and envelope; full knowledge belongs in HMC graph/resources.',
 '006 uses199semantic part identifiers + preserved source/HMC graph lineage; MID2/Tensor10D node sections absent in inspected derivative.',
 'Qualify source-authority-qualified identifier associations across edits, derivation, hosts and replay. Opaque stable handle/digest/hash equality cannot establish authority or universal equivalence. Keep complete entity/species graphs inside HMC.',
 'Core identifiers/graph/HMC + Poet mappings',[core+'container_10d/manifold_identity_v2.rs:1',core+'container_10d/manifold_identity_container.rs',core+'container_10d/node_section.rs',core+'render/compile_10d_manifest.rs:27'])
row('FMT-08','Provenance, source rights and adoption',
 'PRV sidecar retains source bytes,media type,licence and optional VC; encode/decode/validate_provenance exist; integrity anchor includesCRC.',
 'compile_mesh_to_10d_with_provenance and combined nodes/provenance APIs seal sidecar. Native authority/checkpoint records supply distinct project/source workflow evidence.',
 '006 retains editable masters/lineage in HMC and explicit SoftwareAgent author/review request evidence; derivative PRV sidecar absent. No human acceptance, rights adoption, adopted project authority or payment proof.',
 'Qualify cryptographic attestations, rights/license scope, trusted issuer/reviewer authority and downstream retention; presence of PRV/VC bytes or token is not verified permission. Use existing native rights/project records; payment rails remain later work.',
 'Core provenance/native authority/HMC',[core+'container_10d/provenance_section.rs:89',core+'container_10d/provenance_section.rs:483',core+'render/compile_10d.rs:143',poet+'examples/growing_reference/authority.rs'])
row('FMT-09','Simulation, fields and entity affordances',
 'FieldDescriptor declarations AND later raster sample encode/decode APIs exist despite older module header saying ontology-only. MID2 typed units/payload references and EONT energy/damage/hydration/wear/affordance tables exist.',
 'encode_field_section_with_rasters/decode_field_raster_samples and encode/decode_entity_ontology_section_into are public; specific solver/renderer wiring must be traced per field/case.',
 'No field/EONT scientific-simulation execution in006; no verified flow/capacity or biological behavior claimed.',
 'Qualify units, spatial/temporal support, validity, schema versions, dynamics, solver state and exact source-to-field consumers; reconcile stale documentation with existing raster code before calling sample storage missing.',
 'Core field schemas/solvers + authored HMC behavior',[core+'container_10d/field_section.rs:402',core+'container_10d/field_section.rs:426',core+'container_10d/field_section.rs:494',core+'container_10d/entity_ontology_section.rs:27'])
row('FMT-10','Procedural representations and deterministic evaluation',
 'PROC header/codecs plus Voronoi,SDF smooth minimum,allometry/logspiral,PGA oscillation and cleavage evaluation functions exist.',
 'encode/decode_procedural_synthesis_section and evaluate_* are public. Module claims of GPU/SIMD recipe execution are not independent proof of consumer scheduling/cancellation/render integration.',
 '006 authored geometry calls existing bounded profile_sweep/torus primitives; no PROC section or procedural GPU replay exercised.',
 'Qualify which descriptors drive real kernels through Poet/Vibe/native jobs, deterministic bounded evaluation and editable parameter changes. Extend existing authoring/engine contracts where required instead of treating every module comment as production delivery.',
 'Core procedural engines + Poet native host',[core+'container_10d/procedural_synthesis.rs:31',core+'container_10d/mod.rs:179',poet+'src/graphical/geometry_host.rs:165'])
row('FMT-11','Alternative geometry and spatial query representations',
 'VOX0 octree/DAG, hierarchical/gaussian surfel, impostor, topology and BVH/kd-tree codecs exist with public parsers/encoders.',
 'parse_voxel_section/decode_surfel_section/decode_impostor_section/decode_topology_section/decode_spatial_index_section exist; query_frontend consumes spatial index and bundle entity_verifier consumes surfels.',
 '006 compact derivative includes none of these optional sections; triangle source/decode only exercised.',
 'Qualify native/WASM consumers, geometry/appearance/state equivalence, transitions, picking/query accuracy, LOD and memory/latency budgets on actual content. Preserve masters rather than accepting a low-device budget as quality ceiling.',
 'Core representations/query/render + Etalon consumers',[core+'container_10d/voxel_section.rs:179',core+'container_10d/surfel_section.rs:104',core+'container_10d/impostor_section.rs',core+'container_10d/spatial_index_section.rs:230',core+'specialized_libs/computational_geometry/query_frontend.rs:93'])
row('FMT-12','Streaming, interchange and comparative qualification',
 'Section offsets/alignment/CRCs and borrowed views support addressable payloads; parse_section_table currently takes a full byte slice and verifies section payloads. Existing imports/compilers are separate from a proven complete interchange product.',
 'Public section/header readers, caller-buffered decoders and glTF/rest/skin import-related render pipeline source exist; no bounded audit proof of partial-fetch/cache/delta update/editor export roundtrip across competing formats.',
 '006 whole-file5.288MB .10d and5.4MBHMC source-to-byte reconstruction executed; incremental loading/editing and cross-format fidelity were not exercised. Render admission exposed concrete codec defect, with repair verification pending.',
 'Define versioned capability profiles and reproducible corpora/metrics: loss/precision, edit retention, semantics, portability, size, startup/stream latency, memory and verified native rendering. Qualify partial range verification, chunk/dependency invalidation, migrations and import/export loss reports. Superiority must be per measured workload/profile; no evidence supports universal superiority over all formats.',
 'Core codecs/storage/import-export, Poet, Etalon; Ecopunk acceptance',[core+'container_10d/section.rs:638',core+'container_10d/header.rs:108',core+'render/compile_10d.rs:742',core+'render/gltf_import.rs',str(out/'codec-repair.json')])
assert len(rows)==12
report=dict(schema='ecopunk-bounded-format-evolution-audit/1',status='read-only source inspection; not normative specification or release claim',scope='12 bounded capability/qualification rows; existing capabilities retained, no wholesale format rewrite proposed',artifact_006=dict(root=str(asset),sha256=hashlib.sha256(blob).hexdigest(),bytes=len(blob),container_version=struct.unpack_from('<H',blob,4)[0],sections=sections,qualification='source/edit/HMC persistence and exact derivative reconstruction executed; rendering remains unqualified at audit creation; all film/open_world_game/cartoon whole-model claims remain open'),boundary='.10d is a typed embodiment/resource representation within complete HMC graph/resources, not a private replacement for native graph/project/rights mechanisms.',rows=rows)
(out/'format-evolution-audit.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
print(json.dumps(dict(rows=len(rows),sections=sections,path=str(out/'format-evolution-audit.json')),indent=2))