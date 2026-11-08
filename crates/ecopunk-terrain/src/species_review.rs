//! Review compositions of the existing botanical and zoological model owners.
//! Produces a bounded candidate, not a completed species embodiment.
use crate::scene_builder_10d::SubmeshBuilder;
use qualia_core_db::container_10d::{MaterialRecord, ShadingModel, SubmeshRange};
use qualia_core_db::q_hash;
use sha2::{Digest, Sha256};

pub struct SpeciesReviewAsset {
    pub bytes: Vec<u8>,
    pub metadata: serde_json::Value,
}

pub struct ReviewPart {
    pub name: String,
    pub mesh: SubmeshBuilder,
    pub roughness: f32,
    pub double_sided: bool,
}

pub fn build_species_review_10d(subject: u8) -> Result<SpeciesReviewAsset, String> {
    let mut wood = SubmeshBuilder::new();
    let mut leaves = SubmeshBuilder::new();
    let mut animal = SubmeshBuilder::new();
    let (id, title, taxon, source, dossier) = match subject {
        0 => {
            crate::botanical_field_builder::build_white_gum_review_specimen(&mut wood, &mut leaves);
            ("white-gum-review-r1", "White gum — whole-tree candidate", "Eucalyptus dalrympleana",
             include_bytes!("botanical_field_builder.rs").as_slice(), "white-gum-review-r1")
        }
        1 => {
            crate::zoological_loft_builder::build_swamp_wallaby_review_specimen(&mut animal);
            ("swamp-wallaby-review-r1", "Swamp wallaby — standing candidate", "Wallabia bicolor",
             include_bytes!("zoological_loft_builder/swamp_wallaby_review.rs").as_slice(), "swamp-wallaby-review-r1")
        }
        2 => {
            crate::botanical_field_builder::build_white_gum_review_foliage_detail(&mut wood, &mut leaves);
            ("white-gum-foliage-review-r1", "White gum — adult foliage candidate", "Eucalyptus dalrympleana",
             include_bytes!("botanical_field_builder.rs").as_slice(), "white-gum-review-r1")
        }
        _ => return Err("Unknown species review subject".into()),
    };
    let source_sha256 = format!("{:x}", Sha256::digest(source));
    let metadata = serde_json::json!({
        "id":id, "title":title, "taxon":taxon, "status":"rejected_r1_revision_required",
        "referent":"synthetic authored candidate; not an observed specimen",
        "source_sha256":source_sha256,
        "dossier":format!("../assets/production-studies/{dossier}/dossier/claims.json"),
        "profiles":{"film":"incomplete","open_world_game":"incomplete","cartoon":"incomplete"},
        "human_acceptance":"revision_required", "view_scope":"static external form; movement and complete species package remain open"
    });
    let parts = [("wood", wood, 0.76, false), ("foliage", leaves, 0.48, true),
        ("body", animal, 0.72, false)].into_iter().map(|(name,mesh,roughness,double_sided)|
            ReviewPart { name:name.into(),mesh,roughness,double_sided }).collect();
    compile_review_parts(parts, metadata)
}

/// Shared 10D derivative projection for the retained r1 and authored revisions.
pub fn compile_review_parts(parts: Vec<ReviewPart>, mut metadata: serde_json::Value) -> Result<SpeciesReviewAsset, String> {
    let id = metadata["id"].as_str().ok_or("Missing review identifier")?.to_owned();
    let vertices: usize = parts.iter().map(|p| p.mesh.positions.len()).sum();
    let index_count: usize = parts.iter().map(|p| p.mesh.indices.len()).sum();
    if vertices == 0 || vertices > 1_000_000 || index_count == 0 || index_count > 6_000_000 {
        return Err(format!("Species review geometry outside budget: {vertices} vertices, {index_count} indices"));
    }
    let mut combined = SubmeshBuilder::new();
    let mut materials = Vec::new();
    let mut ranges = Vec::new();
    let mut names = std::collections::BTreeSet::new();
    for ReviewPart { name:part, mesh, roughness, double_sided } in parts {
        if mesh.indices.is_empty() { continue; }
        if !names.insert(part.clone()) || !roughness.is_finite() || !(0.0..=1.0).contains(&roughness) {
            return Err("Duplicate part name or invalid material roughness".into());
        }
        let n = mesh.positions.len();
        if mesh.normals.len() != n || mesh.colors.len() != n || mesh.uvs.len() != n
            || mesh.indices.len() % 3 != 0 || mesh.indices.iter().any(|&i| i as usize >= n)
            || mesh.positions.iter().flatten().chain(mesh.normals.iter().flatten())
                .chain(mesh.colors.iter().flatten()).chain(mesh.uvs.iter().flatten()).any(|v| !v.is_finite()) {
            return Err(format!("Invalid geometry streams for {part}"));
        }
        let base = combined.positions.len() as u32;
        let first_index = combined.indices.len() as u32;
        let count = mesh.indices.len() as u32;
        combined.positions.extend(mesh.positions);
        combined.normals.extend(mesh.normals);
        combined.colors.extend(mesh.colors);
        combined.uvs.extend(mesh.uvs);
        combined.indices.extend(mesh.indices.into_iter().map(|i| base + i));
        let material_id = q_hash(&format!("urn:ecopunk:review:{id}:material:{part}"));
        let mut material = MaterialRecord::legacy_default();
        material.id = material_id;
        material.shading_model = ShadingModel::MetallicRoughness;
        material.base_color = [1.0; 4];
        material.multiply_vertex_color = true;
        material.double_sided = double_sided;
        material.casts_shadow = true;
        material.receives_shadow = true;
        material.pickable = true;
        material.roughness = roughness;
        material.metallic = 0.0;
        material.wind_animated = false;
        material.base_color_texture = [0; 32];
        materials.push(material);
        ranges.push(SubmeshRange { first_index, index_count: count, material_id,
            semantic_id: q_hash(&format!("urn:ecopunk:review:{id}:{part}")) });
    }
    materials.sort_by_key(|m| m.id);
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &combined.positions { for axis in 0..3 { min[axis] = min[axis].min(p[axis]); max[axis] = max[axis].max(p[axis]); } }
    metadata["vertex_count"] = combined.positions.len().into();
    metadata["triangle_count"] = (combined.indices.len()/3).into();
    metadata["bounds_min_m"] = serde_json::json!(min);
    metadata["bounds_max_m"] = serde_json::json!(max);
    let mesh = qualia_core_db::render::assets::Mesh {
        positions: combined.positions,
        triangles: combined.indices.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect(),
        min, max,
    };
    let tangents = crate::pbr_material_synth::compute_scene_tangent_frames(
        &mesh.positions, &combined.normals, &combined.uvs, &combined.indices);
    let provenance = qualia_core_db::container_10d::provenance_section::ProvenanceSidecar::new(
        serde_json::to_vec(&metadata).map_err(|e| e.to_string())?,
        "application/vnd.ecopunk.species-review+json;version=1", "Apache-2.0");
    let bytes = qualia_core_db::render::compile_10d::compile_mesh_to_10d_with_extras_and_frames(
        &mesh, &[], Some(&provenance), Default::default(), Some(&combined.colors),
        Some(&combined.normals), Some(&tangents)).map_err(|e| e.to_string())?;
    let bytes = qualia_core_db::container_10d::attach_material_section(
        &bytes, &materials, &ranges, combined.indices.len() as u32).map_err(|e| format!("Materials: {e:?}"))?;
    let mut bytes = qualia_core_db::container_10d::attach_texture_coordinate_section(
        &bytes, &combined.uvs).map_err(|e| format!("UV: {e:?}"))?;
    qualia_core_db::container_10d::integrity::seal_whole_file_crc32c(&mut bytes);
    Ok(SpeciesReviewAsset { bytes, metadata })
}
