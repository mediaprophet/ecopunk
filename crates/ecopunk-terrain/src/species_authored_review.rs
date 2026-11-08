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
    let json = serde_json::to_string(&authored.data).map_err(|e| e.to_string())?;
    let mut extra_files = Vec::<(String, Vec<u8>)>::new();
    let (id, title, taxon, dossier, parts, detail) = match subject {
        0 => {
            let candidate = crate::white_gum_authored_review::build_white_gum_authored_review(&json)?;
            ("white-gum-authored-review-r2", "White gum — authored root and crown revision", "Eucalyptus dalrympleana", "white-gum-review-r1",
                vec![ReviewPart { name:"wood".into(),mesh:candidate.wood,roughness:0.76,double_sided:false },
                     ReviewPart { name:"foliage".into(),mesh:candidate.foliage,roughness:0.48,double_sided:true }], candidate.metadata)
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
                    render_bindings.push(RenderBinding {material:palette.id.clone(),source_skin_vertices:source_vertices});
                    parts.push(ReviewPart {name:palette.id.clone(),mesh,roughness:palette.roughness,double_sided:true});
                }
            }
            let anatomical = AnatomyRecord {
                scope:"Authored anatomical study; skeleton binding and biological validation incomplete",
                skin:anatomy_fields(&candidate.skin)?,osteology:anatomy_fields(&candidate.osteology)?,myology:anatomy_fields(&candidate.myology)?,
                render_bindings,
                parts:candidate.parts.iter().map(|p|PartBinding {source_id:&p.source_id,material:p.material,
                    vertices:[p.vertices.start,p.vertices.end],indices:[p.indices.start,p.indices.end]}).collect(),
            };
            let mut writer=BoundedWriter {bytes:Vec::new(),limit:MAX_BUNDLE_INPUT};
            serde_json::to_writer(&mut writer,&anatomical).map_err(|e|e.to_string())?;
            extra_files.push(("authoring/anatomy.json".into(), writer.bytes));
            let detail = serde_json::json!({"joint_landmarks":candidate.source.joints.len(),
                "bone_spans":candidate.source.bones.len(),"muscles":candidate.source.muscles.len(),
                "anatomy_entry":"authoring/anatomy.json","rig_admission":"not yet qualified",
                "coat":"directional short ribbon study; groom and simulation incomplete"});
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
    let render = compile_review_parts(parts, metadata)?;
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
    let exact_input=entries.iter().map(|(_,_,b)|b.len()).sum::<usize>()+extra_files.iter().map(|(_,b)|b.len()).sum::<usize>();
    if exact_input>MAX_BUNDLE_INPUT {return Err("Review bundle exceeds serialized input allocation".into());}
    for (key,kind,bytes) in entries {bundle.add_file(key,kind,bytes,None).map_err(|e|format!("Review bundle: {e:?}"))?;}
    for (key,bytes) in extra_files { bundle.add_file(key,"json",bytes,None).map_err(|e|format!("Review bundle: {e:?}"))?; }
    let bundle = bundle.build().map_err(|e|format!("Review bundle: {e:?}"))?;
    Ok(AuthoredReviewAsset { render, bundle })
}

fn anatomy_fields(mesh: &AnatomyMeshAccumulator) -> Result<AnatomyFields<'_>, String> {
    let n = mesh.positions.len();
    if n > 750_000 || mesh.normals.len()!=n || mesh.colors.len()!=n || mesh.uvs.len()!=n
        || mesh.layer_tags.len()!=n || mesh.joint_indices.len()!=n || mesh.joint_weights.len()!=n
        || mesh.indices.len()%3!=0 || mesh.indices.iter().any(|&i|i as usize>=n)
        || mesh.positions.iter().flatten().chain(mesh.normals.iter().flatten())
            .chain(mesh.colors.iter().flatten()).chain(mesh.uvs.iter().flatten())
            .chain(mesh.joint_weights.iter().flatten()).any(|v|!v.is_finite()) {
        return Err("Invalid authored anatomy streams".into());
    }
    Ok(AnatomyFields {positions_m:&mesh.positions,normals:&mesh.normals,colors:&mesh.colors,uv0:&mesh.uvs,
        indices:&mesh.indices,layers:LayerNames(&mesh.layer_tags),joint_indices:&mesh.joint_indices,joint_weights:&mesh.joint_weights})
}

fn generator_pins()->serde_json::Value {
    let mut pins=serde_json::Map::new();
    for (name,bytes) in [
        ("white_gum_authored_review.rs",include_bytes!("white_gum_authored_review.rs").as_slice()),
        ("swamp_wallaby_anatomical_r3.rs",include_bytes!("swamp_wallaby_anatomical_r3.rs").as_slice()),
        ("botanical_field_builder.rs",include_bytes!("botanical_field_builder.rs").as_slice()),
        ("zoological_anatomy.rs",include_bytes!("zoological_anatomy.rs").as_slice()),
        ("scene_builder_10d.rs",include_bytes!("scene_builder_10d.rs").as_slice()),
        ("species_authored_review.rs",include_bytes!("species_authored_review.rs").as_slice()),
        ("species_review_source.rs",include_bytes!("species_review_source.rs").as_slice()),
        ("species_review.rs",include_bytes!("species_review.rs").as_slice()),
        ("Cargo.lock",include_bytes!("../../../Cargo.lock").as_slice()),
    ] {pins.insert(name.into(),format!("{:x}",Sha256::digest(bytes)).into());}
    serde_json::Value::Object(pins)
}
