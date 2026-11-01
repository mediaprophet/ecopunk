//! Runtime-authored review derivatives and transparent source-preserving bundles.
use crate::scene_builder_10d::SubmeshBuilder;
use crate::species_review::{compile_review_parts, ReviewPart, SpeciesReviewAsset};
use crate::species_review_source::AuthoredSpeciesSource;
use crate::zoological_anatomy::AnatomyMeshAccumulator;
use qualia_core_db::bundle::BundleWriter;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Write;

const MAX_BUNDLE_INPUT: usize = 96 * 1024 * 1024;
#[derive(Serialize)]
struct RenderBinding { material: String, source_skin_vertices: Vec<u32> }
#[derive(Serialize)]
struct PartBinding<'a> { source_id: &'a str, material: usize, vertices: [usize;2], indices: [usize;2] }
#[derive(Serialize)]
struct AnatomyRecord<'a> {
    scope: &'static str,
    skin: AnatomyFields<'a>, osteology: AnatomyFields<'a>, myology: AnatomyFields<'a>,
    render_bindings: Vec<RenderBinding>, parts: Vec<PartBinding<'a>>,
    #[serde(skip_serializing_if="Option::is_none")]
    groom: Option<&'a crate::swamp_wallaby_anatomical_r3::GroomReceipt>,
    #[serde(skip_serializing_if="Option::is_none")]
    surface_junctions: Option<&'a crate::swamp_wallaby_anatomical_r3::JunctionReceipt>,
}
#[derive(Serialize)]
struct AnatomyFields<'a> {
    positions_m: &'a [[f32;3]], normals: &'a [[f32;3]], colors: &'a [[f32;4]], uv0: &'a [[f32;2]],
    indices: &'a [u32], layers: LayerNames<'a>, joint_indices: &'a [[u16;4]], joint_weights: &'a [[f32;4]],
}
struct LayerNames<'a>(&'a [crate::zoological_anatomy::ZoologicalAnatomicalLayer]);
impl Serialize for LayerNames<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer:S)->Result<S::Ok,S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq=serializer.serialize_seq(Some(self.0.len()))?;
        for layer in self.0 {seq.serialize_element(&format!("{layer:?}"))?;}
        seq.end()
    }
}
struct BoundedWriter { bytes:Vec<u8>, limit:usize }
impl Write for BoundedWriter {
    fn write(&mut self, bytes:&[u8])->std::io::Result<usize> {
        if bytes.len()>self.limit.saturating_sub(self.bytes.len()) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,"Review serialization budget exceeded"));
        }
        self.bytes.extend_from_slice(bytes);Ok(bytes.len())
    }
    fn flush(&mut self)->std::io::Result<()> {Ok(())}
}

pub struct AuthoredReviewAsset {
    pub render: SpeciesReviewAsset,
    pub bundle: Vec<u8>,
}

pub fn build(subject: u8, authored: &AuthoredSpeciesSource) -> Result<AuthoredReviewAsset, String> {
    build_with_branch_limits(subject,authored,
        qualia_core_db::specialized_libs::computational_geometry::branch_graph_surface::BranchGraphLimits::default())
}

/// Host-selected limits; authored species data cannot enlarge the budget.
pub fn build_with_branch_limits(subject:u8,authored:&AuthoredSpeciesSource,
    branch_limits:qualia_core_db::specialized_libs::computational_geometry::branch_graph_surface::BranchGraphLimits)
    ->Result<AuthoredReviewAsset,String> {
    build_with_limits(subject,authored,branch_limits,crate::wallaby_rig::RigLimits::default())
}

/// Independent caller budgets for selected graph construction and wallaby rig
/// projection/admission. Authored source cannot enlarge these limits.
pub fn build_with_limits(subject:u8,authored:&AuthoredSpeciesSource,
    branch_limits:qualia_core_db::specialized_libs::computational_geometry::branch_graph_surface::BranchGraphLimits,
    rig_limits:crate::wallaby_rig::RigLimits)->Result<AuthoredReviewAsset,String> {
    build_with_all_limits(subject,authored,branch_limits,rig_limits,crate::tree_junction::TreeJunctionLimits::default())
}

/// Independent caller-selected limits for explicit full-tree junction edits.
pub fn build_with_all_limits(subject:u8,authored:&AuthoredSpeciesSource,
    branch_limits:qualia_core_db::specialized_libs::computational_geometry::branch_graph_surface::BranchGraphLimits,
    rig_limits:crate::wallaby_rig::RigLimits,junction_limits:crate::tree_junction::TreeJunctionLimits)->Result<AuthoredReviewAsset,String>{
    build_with_production_limits(subject,authored,branch_limits,rig_limits,junction_limits,
        crate::tree_forks::AllForkLimits::default(),&||false)
}

/// Aggregate full-tree budget and cancellation are selected by the host, not
/// enlarged by an authored fork profile. Publication follows complete readback.
pub fn build_with_production_limits(subject:u8,authored:&AuthoredSpeciesSource,
    branch_limits:qualia_core_db::specialized_libs::computational_geometry::branch_graph_surface::BranchGraphLimits,
    rig_limits:crate::wallaby_rig::RigLimits,junction_limits:crate::tree_junction::TreeJunctionLimits,
    all_fork_limits:crate::tree_forks::AllForkLimits,cancelled:&dyn Fn()->bool)->Result<AuthoredReviewAsset,String>{
    if cancelled() {return Err("Authored review build cancelled".into());}
    if authored.sources.is_empty() || authored.sources.len() > 128 || authored.revision != authored.sources.len() as u64
        || authored.sources.iter().any(|s| s.len() > 1024 * 1024) {
        return Err("Review source inventory exceeds checked session profile".into());
    }
    let source_bytes = authored.sources.iter().try_fold(0usize, |n, source| n.checked_add(source.len()))
        .filter(|&n| n <= 8 * 1024 * 1024).ok_or("Review retained source exceeds limit")?;
    let json = serde_json::to_string(&authored.data).map_err(|e| e.to_string())?;
    if json.len() > 16 * 1024 * 1024 { return Err("Evaluated review source exceeds limit".into()); }
    let mut extra_files = Vec::<(String, Vec<u8>)>::new();
    let mut connected_projection=None;
    let mut junction_projection=None;
    let mut animal_junction_projection=None;
    let mut wallaby_rig:Option<crate::wallaby_rig::PreparedRig>=None;
    let (id, title, taxon, dossier, parts, detail) = match subject {
        0 => {
            let study=authored.data.get("source_version").and_then(serde_json::Value::as_str)==Some("white-gum-connected-study-v1");
            let junction=authored.data.get("source_version").and_then(serde_json::Value::as_str)==Some("white-gum-chain-union-v1");
            let all_forks=authored.data.get("source_version").and_then(serde_json::Value::as_str)==Some("white-gum-all-forks-v1");
            let candidate = if study {crate::white_gum_connected_review::build(&json,branch_limits,&||false)?}
                else if all_forks {crate::white_gum_authored_review::build_with_all_fork_limits(&json,junction_limits,all_fork_limits,cancelled)?}
                else {crate::white_gum_authored_review::build_with_junction_limits(&json,junction_limits)?};
            if junction {junction_projection=Some(crate::tree_junction::projection_witness(&candidate.wood,&candidate.foliage,&candidate.metadata)?);}
            if all_forks {junction_projection=Some(crate::tree_forks::projection_witness(&candidate.wood,&candidate.foliage,&candidate.metadata)?);}
            if study {connected_projection=Some(crate::white_gum_connected_review::projection_witness(&candidate.wood,
                authored.data["connected_study"]["min_area_sq_m4"].as_f64().ok_or("Missing connected projection area threshold")?)?);}
            let mut parts=vec![ReviewPart{name:"wood".into(),mesh:candidate.wood,roughness:0.76,double_sided:false}];
            if !study {parts.push(ReviewPart{name:"foliage".into(),mesh:candidate.foliage,roughness:0.48,double_sided:true});}
            (if study {"white-gum-connected-study-v1"} else if all_forks {"white-gum-all-forks-v1"} else if junction {"white-gum-chain-union-v1"} else {"white-gum-authored-review-r2"},
                if study {"White gum — selected connected branch study"} else if all_forks {"White gum — complete fork collar candidate"} else if junction {"White gum — explicit two-chain union in retained tree"} else {"White gum — authored root and crown revision"},
                "Eucalyptus dalrympleana", "white-gum-review-r1",parts,candidate.metadata)
        }
        1 => {
            let candidate = crate::swamp_wallaby_anatomical_r3::build_swamp_wallaby_authored_review(&json)?;
            let total_vertices=candidate.skin.positions.len()+candidate.osteology.positions.len()+candidate.myology.positions.len();
            let total_indices=candidate.skin.indices.len()+candidate.osteology.indices.len()+candidate.myology.indices.len();
            // Conservative text/storage estimate before material copies or serialization.
            if total_vertices.saturating_mul(384).saturating_add(total_indices.saturating_mul(12))>MAX_BUNDLE_INPUT {
                return Err("Authored anatomy exceeds current review package working set".into());
            }
            let mut parts = Vec::new();
            let mut render_bindings = Vec::new();
            let mut render_skin_lineage=Vec::new();
            for (material, palette) in candidate.source.palettes.iter().enumerate() {
                let mut mesh = SubmeshBuilder::new();
                let mut remap = std::collections::BTreeMap::new();
                let mut source_vertices = Vec::new();
                for part in candidate.parts.iter().filter(|p| p.material == material) {
                    let indices = candidate.skin.indices.get(part.indices.clone()).ok_or("Invalid authored part range")?;
                    for &index in indices {
                        let original = index as usize;
                        if !part.vertices.contains(&original) || original >= candidate.skin.positions.len() {
                            return Err("Anatomy index escapes its declared part".into());
                        }
                        let next = mesh.positions.len() as u32;
                        let mapped = *remap.entry(index).or_insert_with(|| {
                            mesh.positions.push(candidate.skin.positions[original]);
                            mesh.normals.push(candidate.skin.normals[original]);
                            mesh.colors.push(candidate.skin.colors[original]);
                            mesh.uvs.push(candidate.skin.uvs[original]);
                            source_vertices.push(index);
                            next
                        });
                        mesh.indices.push(mapped);
                    }
                }
                if !mesh.indices.is_empty() {
                    render_skin_lineage.extend_from_slice(&source_vertices);
                    render_bindings.push(RenderBinding {material:palette.id.clone(),source_skin_vertices:source_vertices});
                    parts.push(ReviewPart {name:palette.id.clone(),mesh,roughness:palette.roughness,double_sided:true});
                }
            }
            animal_junction_projection=crate::swamp_wallaby_anatomical_r3::prepare_junction_projection(
                &candidate,&parts,&render_skin_lineage)?;
            let anatomical = AnatomyRecord {
                scope:"Authored anatomical study; external-skin rest rig projected; biological/deformation validation incomplete",
                skin:anatomy_fields("skin",&candidate.skin)?,osteology:anatomy_fields("osteology",&candidate.osteology)?,myology:anatomy_fields("myology",&candidate.myology)?,
                render_bindings,
                groom:candidate.groom.as_ref(),surface_junctions:candidate.surface_junctions.as_ref(),
                parts:candidate.parts.iter().map(|p|PartBinding {source_id:&p.source_id,material:p.material,
                    vertices:[p.vertices.start,p.vertices.end],indices:[p.indices.start,p.indices.end]}).collect(),
            };
            let mut writer=BoundedWriter {bytes:Vec::new(),limit:MAX_BUNDLE_INPUT};
            serde_json::to_writer(&mut writer,&anatomical).map_err(|e|e.to_string())?;
            extra_files.push(("authoring/anatomy.json".into(), writer.bytes));
            let rig=crate::wallaby_rig::prepare(&candidate,&render_skin_lineage,&authored.source_sha256,rig_limits)?;
            let detail = serde_json::json!({"joint_landmarks":candidate.source.joints.len(),
                "bone_spans":candidate.source.bones.len(),"muscles":candidate.source.muscles.len(),
                "anatomy_entry":"authoring/anatomy.json","rig":rig.metadata,
                "rig_admission":"canonical SKL1 and actual decoded-bind rest player checked before publication; execution/human acceptance not claimed",
                "coat":if candidate.groom.is_some(){"authored area-distributed curved fibres; physical hair scattering/simulation not asserted"}else{"directional short ribbon study; groom and simulation incomplete"},
                "groom":&candidate.groom,
                "surface_junctions":candidate.surface_junctions.as_ref().map(|r|serde_json::json!({
                    "records":r.records.len(),"work_used":r.work_used,"reserved_bytes":r.reserved_bytes,
                    "component_faces":r.component_faces,"receipt_entry":"authoring/anatomy.json"}))});
            wallaby_rig=Some(rig);
            ("swamp-wallaby-anatomical-review-r3", "Swamp wallaby — anatomical revision", "Wallabia bicolor", "swamp-wallaby-review-r1", parts, detail)
        }
        _ => return Err("Unsupported authored species subject".into()),
    };
    let metadata = serde_json::json!({
        "id":id,"title":title,"taxon":taxon,"status":"candidate_awaiting_visual_review",
        "referent":"synthetic authored candidate; not an observed specimen",
        "source_sha256":authored.source_sha256,"source_revision":authored.revision,
        "source_owner":"vibe::SourceSession; detached species data validation before promotion",
        "generator_source_sha256":generator_pins(),
        "dossier":format!("../assets/production-studies/{dossier}/dossier/claims.json"),
        "human_acceptance":"pending","profiles":{"film":"incomplete","open_world_game":"incomplete","cartoon":"incomplete"},
        "view_scope":"static review derivative; complete species, physics and human acceptance remain open",
        "package_scope":"limited authored review bundle; not a complete HMC embodiment or Studio checkpoint",
        "detail":detail
    });
    if cancelled() {return Err("Authored review build cancelled before compilation".into());}
    let mut render = compile_review_parts(parts, metadata)?;
    if let Some(witness)=connected_projection {crate::white_gum_connected_review::qualify_compiled_projection(&render.bytes,&witness)?;}
    if let Some(witness)=junction_projection {crate::tree_junction::qualify_compiled_projection(&render.bytes,&witness)?;}
    if let Some(witness)=animal_junction_projection {
        let report=crate::swamp_wallaby_anatomical_r3::qualify_junction_projection(&render.bytes,witness)?;
        render.metadata["detail"]["surface_junction_projection"]=serde_json::to_value(report).map_err(|e|e.to_string())?;
    }
    if let Some(rig)=wallaby_rig {render.bytes=crate::wallaby_rig::attach(&render.bytes,&rig,rig_limits)?;}
    let retained_bytes=extra_files.iter().map(|(_,b)|b.len()).sum::<usize>()+render.bytes.len()+json.len()
        +authored.sources.iter().map(String::len).sum::<usize>();
    if retained_bytes>MAX_BUNDLE_INPUT {return Err("Review bundle exceeds total input allocation".into());}
    let mut bundle = BundleWriter::new();
    let session = serde_json::json!({"schema":"ecopunk.species-review-source-session/1",
        "vibe_language":vibe::LANGUAGE_VERSION,"revision":authored.revision,
        "source_sha256":authored.source_sha256,"submissions":authored.sources});
    let entries = [
        ("authoring/source-session.json","json",serde_json::to_vec(&session).map_err(|e|e.to_string())?),
        ("authoring/evaluated-source.json","json",json.into_bytes()),
        ("review/manifest.json","manifest",serde_json::to_vec(&render.metadata).map_err(|e|e.to_string())?),
        ("review/model.10d","10d",render.bytes.clone()),
    ];
    if entries[0].2.len() > 16 * 1024 * 1024 { return Err("Serialized review source session exceeds limit".into()); }
    let exact_input=entries.iter().map(|(_,_,b)|b.len()).chain(extra_files.iter().map(|(_,b)|b.len()))
        .try_fold(source_bytes, |n, len| n.checked_add(len)).ok_or("Review serialized input size overflow")?;
    if exact_input>MAX_BUNDLE_INPUT {return Err("Review bundle exceeds serialized input allocation".into());}
    for (key,kind,bytes) in entries {bundle.add_file(key,kind,bytes,None).map_err(|e|format!("Review bundle: {e:?}"))?;}
    // Retain the exact native authored submissions as addressable resources.
    // The legacy JSON session is a compatibility projection, not a replacement
    // Vibe source format or a canonical Studio/graph ownership declaration.
    for (index, source) in authored.sources.iter().enumerate() {
        bundle.add_file(format!("authoring/submissions/{:04}.vibe", index + 1), "vibe",
            source.as_bytes().to_vec(), None).map_err(|e|format!("Review Vibe resource: {e:?}"))?;
    }
    for (key,bytes) in extra_files { bundle.add_file(key,"json",bytes,None).map_err(|e|format!("Review bundle: {e:?}"))?; }
    let bundle = bundle.build().map_err(|e|format!("Review bundle: {e:?}"))?;
    if cancelled() {return Err("Authored review build cancelled before publication".into());}
    Ok(AuthoredReviewAsset { render, bundle })
}

fn anatomy_fields<'a>(name:&str,mesh: &'a AnatomyMeshAccumulator) -> Result<AnatomyFields<'a>, String> {
    let n = mesh.positions.len();
    if n > 750_000 || mesh.normals.len()!=n || mesh.colors.len()!=n || mesh.uvs.len()!=n
        || mesh.layer_tags.len()!=n || mesh.joint_indices.len()!=n || mesh.joint_weights.len()!=n
        || mesh.indices.len()%3!=0 || mesh.indices.iter().any(|&i|i as usize>=n)
        || mesh.positions.iter().flatten().chain(mesh.normals.iter().flatten())
            .chain(mesh.colors.iter().flatten()).chain(mesh.uvs.iter().flatten())
            .chain(mesh.joint_weights.iter().flatten()).any(|v|!v.is_finite()) {
        return Err(format!("Invalid authored anatomy streams ({name}): positions={n}, normals={}, colors={}, uv={}, layers={}, joints={}, weights={}, indices={}, out_of_range_indices={}, nonfinite_values={}",
            mesh.normals.len(),mesh.colors.len(),mesh.uvs.len(),mesh.layer_tags.len(),mesh.joint_indices.len(),mesh.joint_weights.len(),mesh.indices.len(),
            mesh.indices.iter().filter(|&&i|i as usize>=n).count(),
            mesh.positions.iter().flatten().chain(mesh.normals.iter().flatten()).chain(mesh.colors.iter().flatten()).chain(mesh.uvs.iter().flatten()).chain(mesh.joint_weights.iter().flatten()).filter(|v|!v.is_finite()).count()));
    }
    Ok(AnatomyFields {positions_m:&mesh.positions,normals:&mesh.normals,colors:&mesh.colors,uv0:&mesh.uvs,
        indices:&mesh.indices,layers:LayerNames(&mesh.layer_tags),joint_indices:&mesh.joint_indices,joint_weights:&mesh.joint_weights})
}

fn generator_pins()->serde_json::Value {
    let mut pins=serde_json::Map::new();
    for (name,bytes) in [
        ("white_gum_authored_review.rs",include_bytes!("white_gum_authored_review.rs").as_slice()),
        ("white_gum_connected_review.rs",include_bytes!("white_gum_connected_review.rs").as_slice()),
        ("tree_junction/mod.rs",include_bytes!("tree_junction/mod.rs").as_slice()),
        ("tree_junction/union.rs",include_bytes!("tree_junction/union.rs").as_slice()),
        ("tree_junction/attributes.rs",include_bytes!("tree_junction/attributes.rs").as_slice()),
        ("tree_junction/projection.rs",include_bytes!("tree_junction/projection.rs").as_slice()),
        ("wallaby_rig/mod.rs",include_bytes!("wallaby_rig/mod.rs").as_slice()),
        ("wallaby_rig/projection.rs",include_bytes!("wallaby_rig/projection.rs").as_slice()),
        ("wallaby_rig/admission.rs",include_bytes!("wallaby_rig/admission.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3.rs",include_bytes!("swamp_wallaby_anatomical_r3.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/skin_profile.rs",include_bytes!("swamp_wallaby_anatomical_r3/skin_profile.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/loft_chain/mod.rs",include_bytes!("swamp_wallaby_anatomical_r3/loft_chain/mod.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/loft_chain/build.rs",include_bytes!("swamp_wallaby_anatomical_r3/loft_chain/build.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/loft_chain/qualification.rs",include_bytes!("swamp_wallaby_anatomical_r3/loft_chain/qualification.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/groom.rs",include_bytes!("swamp_wallaby_anatomical_r3/groom.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/junctions/mod.rs",include_bytes!("swamp_wallaby_anatomical_r3/junctions/mod.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/junctions/ports.rs",include_bytes!("swamp_wallaby_anatomical_r3/junctions/ports.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/junctions/build.rs",include_bytes!("swamp_wallaby_anatomical_r3/junctions/build.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/junctions/attributes.rs",include_bytes!("swamp_wallaby_anatomical_r3/junctions/attributes.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/junctions/qualification.rs",include_bytes!("swamp_wallaby_anatomical_r3/junctions/qualification.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3/junctions/projection.rs",include_bytes!("swamp_wallaby_anatomical_r3/junctions/projection.rs").as_slice()),
        ("species_knowledge/graph.rs",include_bytes!("species_knowledge/graph.rs").as_slice()),
        ("species_knowledge/authored.rs",include_bytes!("species_knowledge/authored.rs").as_slice()),
        ("species_knowledge/mod.rs",include_bytes!("species_knowledge/mod.rs").as_slice()),
        ("species_review_resources/corpus.rs",include_bytes!("species_review_resources/corpus.rs").as_slice()),
        ("species_hmc_review/inspect.rs",include_bytes!("species_hmc_review/inspect.rs").as_slice()),
        ("species_knowledge/wallaby.rs",include_bytes!("species_knowledge/wallaby.rs").as_slice()),
        ("species_knowledge/wallaby_recipes.rs",include_bytes!("species_knowledge/wallaby_recipes.rs").as_slice()),
        ("species_knowledge/ontology.ttl",include_bytes!("species_knowledge/ontology.ttl").as_slice()),
        ("qualia-core-db/specialized_libs/computational_geometry/parametric_cad.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/parametric_cad.rs").as_slice()),
        ("qualia-core-db/query/resolver.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/query/resolver.rs").as_slice()),
        ("qualia-core-db/query/typed_double.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/query/typed_double.rs").as_slice()),
        ("qualia-core-db/query/lexicon.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/query/lexicon.rs").as_slice()),
        ("qualia-core-db/query/ingest_formats.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/query/ingest_formats.rs").as_slice()),
        ("qualia-core-db/sparql_library/sparql_ast.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/sparql_library/sparql_ast.rs").as_slice()),
        ("qualia-core-db/sparql_library/parsers/cbor_parser.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/sparql_library/parsers/cbor_parser.rs").as_slice()),
        ("botanical_field_builder.rs",include_bytes!("botanical_field_builder.rs").as_slice()),
        ("zoological_anatomy.rs",include_bytes!("zoological_anatomy.rs").as_slice()),
        ("zoological_anatomy/numeric_profile.rs",include_bytes!("zoological_anatomy/numeric_profile.rs").as_slice()),
        ("ecopunk-terrain/Cargo.toml",include_bytes!("../Cargo.toml").as_slice()),
        ("scene_builder_10d.rs",include_bytes!("scene_builder_10d.rs").as_slice()),
        ("scene_mesh_buffer.rs",include_bytes!("scene_mesh_buffer.rs").as_slice()),
        ("species_authored_review.rs",include_bytes!("species_authored_review.rs").as_slice()),
        ("species_review_source.rs",include_bytes!("species_review_source.rs").as_slice()),
        ("species_review.rs",include_bytes!("species_review.rs").as_slice()),
        ("Cargo.lock",include_bytes!("../../../Cargo.lock").as_slice()),
    ] {pins.insert(name.into(),format!("{:x}",Sha256::digest(bytes)).into());}
    for (name,bytes) in [
        ("animal_motion_review.rs",include_bytes!("animal_motion_review.rs").as_slice()),
        ("animal_motion_review_reopen.rs",include_bytes!("animal_motion_review_reopen.rs").as_slice()),
        ("species_review_resources/session.rs",include_bytes!("species_review_resources/session.rs").as_slice()),
        ("animal_motion/binding.rs",include_bytes!("animal_motion/binding.rs").as_slice()),
        ("animal_motion/compile.rs",include_bytes!("animal_motion/compile.rs").as_slice()),
        ("animal_motion/export.rs",include_bytes!("animal_motion/export.rs").as_slice()),
        ("animal_motion/mod.rs",include_bytes!("animal_motion/mod.rs").as_slice()),
        ("animal_motion/runtime.rs",include_bytes!("animal_motion/runtime.rs").as_slice()),
        ("animal_motion/validation.rs",include_bytes!("animal_motion/validation.rs").as_slice()),
        ("smooth_collars/attributes.rs",include_bytes!("smooth_collars/attributes.rs").as_slice()),
        ("smooth_collars/build.rs",include_bytes!("smooth_collars/build.rs").as_slice()),
        ("smooth_collars/mod.rs",include_bytes!("smooth_collars/mod.rs").as_slice()),
        ("smooth_collars/seed.rs",include_bytes!("smooth_collars/seed.rs").as_slice()),
        ("tree_forks/adoption.rs",include_bytes!("tree_forks/adoption.rs").as_slice()),
        ("tree_forks/cuts.rs",include_bytes!("tree_forks/cuts.rs").as_slice()),
        ("tree_forks/mod.rs",include_bytes!("tree_forks/mod.rs").as_slice()),
        ("tree_forks/pieces.rs",include_bytes!("tree_forks/pieces.rs").as_slice()),
        ("tree_forks/plan.rs",include_bytes!("tree_forks/plan.rs").as_slice()),
        ("tree_forks/topology.rs",include_bytes!("tree_forks/topology.rs").as_slice()),
        ("qualia-core-db/boolean_3.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/boolean_3.rs").as_slice()),
        ("qualia-core-db/boolean_3/bounded.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/boolean_3/bounded.rs").as_slice()),
        ("qualia-core-db/boolean_3/conforming.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/boolean_3/conforming.rs").as_slice()),
        ("qualia-core-db/expansion.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/expansion.rs").as_slice()),
        ("qualia-core-db/boolean_3/exact_registry.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/boolean_3/exact_registry.rs").as_slice()),
        ("qualia-core-db/boolean_3/facet_constraints.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/boolean_3/facet_constraints.rs").as_slice()),
        ("qualia-core-db/topology.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/topology.rs").as_slice()),
        ("qualia-core-db/connectivity.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/connectivity.rs").as_slice()),
        ("qualia-core-db/surface_topology_qualification.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/surface_topology_qualification.rs").as_slice()),
        ("qualia-core-db/indexed_tetra_surface.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/indexed_tetra_surface.rs").as_slice()),
        ("qualia-core-db/tapered_branch_field.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/tapered_branch_field.rs").as_slice()),
        ("qualia-core-db/hard_cap_sweep.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/hard_cap_sweep.rs").as_slice()),
        ("qualia-core-db/smooth_junction/mod.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/smooth_junction/mod.rs").as_slice()),
        ("qualia-core-db/smooth_junction/project.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/smooth_junction/project.rs").as_slice()),
        ("qualia-core-db/smooth_junction/refine.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/smooth_junction/refine.rs").as_slice()),
        ("qualia-core-db/render/mesh_tangent_frames.rs",include_bytes!("../../../../qualiaDB/crates/qualia-core-db/src/render/mesh_tangent_frames.rs").as_slice()),
    ] {pins.insert(name.into(),format!("{:x}",Sha256::digest(bytes)).into());}
    serde_json::Value::Object(pins)
}
