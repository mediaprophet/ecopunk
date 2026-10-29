//! Runtime projection of an editable review source through existing Core geometry owners.
//! This is not a graph store, accepted Poet session, welded master, or mechanics solver.
//! The caller retains the exact authored source and returned source-to-render ranges.
use std::collections::{BTreeMap, BTreeSet};
use serde::Deserialize;
use qualia_core_db::specialized_libs::computational_geometry::{Point2, Point3, math_geometry::AffineFrame3, sweep_extrusion::ExtrusionStation};
use qualia_core_db::specialized_libs::computational_geometry::hard_cap_sweep::{hard_cap_sweep_counts, hard_cap_sweep_normals_into};
use crate::scene_builder_10d::SubmeshBuilder;

pub struct WhiteGumAuthoredReview {
    pub wood: SubmeshBuilder,
    pub foliage: SubmeshBuilder,
    pub metadata: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    source_version: String,
    botanical: RecipeProjection,
    radii_m: Vec<f32>,
    roles: Vec<String>,
    sprays: Vec<Spray>,
}
// Field-compatible projection of the existing Poet botanical v2 recipe. This
// browser adapter does not claim to run Poet's controller/validation pipeline.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RecipeProjection {
    schema: String, version: u32, part_id: String, organs: Vec<Organ>,
    attractors_m: Vec<[f32; 3]>, node_length_m: f32, kill_radius_m: f32,
    perception_radius_m: f32, cone_angle_rad: f32, max_iterations: usize,
    tip_radius_m: f32, pipe_exponent: f32, ring_slices: usize,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Organ { key: String, parent: Option<String>, position_m: [f32; 3] }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Spray {
    key: String, edge_end: String, edge_fraction: f32, direction: [f32; 3],
    length_m: f32, leaf_length_m: f32, leaf_width_m: f32, leaf_count: usize, seed: f32,
}
fn finite_point(p: [f32;3]) -> bool { p.iter().all(|v| v.is_finite() && v.abs() <= 20.0) }
fn key_valid(k: &str) -> bool { !k.is_empty() && k.len() <= 96 && k.bytes().all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b)) }
fn point(p: [f32;3]) -> Point3 { Point3::new(p[0] as f64,p[1] as f64,p[2] as f64) }
fn distance2(a:[f32;3],b:[f32;3])->f32 { (a[0]-b[0]).powi(2)+(a[1]-b[1]).powi(2)+(a[2]-b[2]).powi(2) }

// Reuse the existing Core hard-cap derivative so cap centers/rims have their
// own normals. Station arc length works for horizontal and descending roots.
// Returns local derivative-to-base-sweep vertex mapping; no fork weld is implied.
fn append_hard_cap_chain(mesh:&mut SubmeshBuilder, axis:&[Point3], stations:&[ExtrusionStation], sides:usize, base:[f32;4], tip:[f32;4]) -> Result<Vec<u32>,String> {
    let counts=hard_cap_sweep_counts(axis.len(),sides).map_err(|e|format!("Hard-cap counts: {e:?}"))?;
    let section=(0..sides).map(|j|{let a=std::f64::consts::TAU*j as f64/sides as f64;Point2::new(a.cos(),a.sin())}).collect::<Vec<_>>();
    let zero=Point3::new(0.0,0.0,0.0);
    let mut frames=vec![AffineFrame3{origin:zero,e0:zero,e1:zero,e2:zero};counts.frames];
    let mut vertices=vec![zero;counts.vertices]; let mut triangles=vec![[0;3];counts.triangles];
    let mut normals=vec![zero;counts.vertices]; let mut mapping=vec![0;counts.vertices];
    hard_cap_sweep_normals_into(axis,&section,Some(stations),&mut frames,&mut vertices,&mut triangles,&mut normals,&mut mapping)
        .map_err(|e|format!("Core hard-cap chain refused: {e:?}"))?;
    let mut arc=vec![0.0f64;axis.len()];
    for i in 1..axis.len() { let a=axis[i-1];let b=axis[i];arc[i]=arc[i-1]+((b.x-a.x).powi(2)+(b.y-a.y).powi(2)+(b.z-a.z).powi(2)).sqrt(); }
    let length=*arc.last().ok_or("Missing axis")?;
    if !length.is_finite() || length<=0.0 {return Err("Invalid chain arc length".into());}
    let base_rings=axis.len()*sides;
    let offset=u32::try_from(mesh.positions.len()).map_err(|_|"Chain vertex offset overflow")?;
    for i in 0..counts.vertices {
        let source=mapping[i] as usize;
        let station=if source==base_rings {0} else if source==base_rings+1 {axis.len()-1} else {source/sides};
        let t=(arc[station]/length) as f32;
        let color=[base[0]*(1.0-t)+tip[0]*t,base[1]*(1.0-t)+tip[1]*t,base[2]*(1.0-t)+tip[2]*t,1.0];
        let uv=if i>=base_rings {
            if source>=base_rings {[0.5,0.5]} else {let q=section[source%sides];[0.5+0.5*q.x as f32,0.5+0.5*q.y as f32]}
        } else {[(source%sides) as f32/sides as f32,t]};
        let p=vertices[i];let n=normals[i];
        mesh.push_vertex([p.x as f32,p.y as f32,p.z as f32],[n.x as f32,n.y as f32,n.z as f32],color,uv);
    }
    for t in triangles {mesh.push_tri(offset+t[0],offset+t[1],offset+t[2]);}
    Ok(mapping)
}

/// Accepts the JSON-compatible `species_asset` binding from the Vibe source session.
/// Model edits change runtime source; this function contains no baked model data.
/// Returns scratch geometry only on success, with authored graph and range metadata.
pub fn build_white_gum_authored_review(source: &str) -> Result<WhiteGumAuthoredReview,String> {
    if source.len() > 512 * 1024 { return Err("White-gum source exceeds 512 KiB".into()); }
    let data: Source = serde_json::from_str(source).map_err(|e|format!("White-gum source: {e}"))?;
    let r=&data.botanical;
    let count=r.organs.len();
    if data.source_version != "white-gum-authored-review-r2" || r.schema != "poet.botanical/v2" || r.version != 2 {
        return Err("Unsupported authored review / botanical profile".into());
    }
    if !(2..=256).contains(&count) || data.radii_m.len()!=count || data.roles.len()!=count
        || data.sprays.len()>256 || !(5..=24).contains(&r.ring_slices) || !key_valid(&r.part_id) {
        return Err("Authored organ/profile working set refused".into());
    }
    if !r.attractors_m.is_empty() || r.max_iterations>64 {
        return Err("Review adapter accepts adopted organs only, not unadopted growth proposals".into());
    }
    let controls=[r.node_length_m,r.kill_radius_m,r.perception_radius_m,r.cone_angle_rad,r.tip_radius_m,r.pipe_exponent];
    if controls.iter().any(|v|!v.is_finite() || *v<=0.0 || *v>1.0e6)
        || r.perception_radius_m<r.kill_radius_m || r.cone_angle_rad>std::f32::consts::TAU
        || !(1.0..=8.0).contains(&r.pipe_exponent) {
        return Err("Invalid botanical controls".into());
    }
    let allowed_roles=["root_collar","trunk_metamer","scaffold_branch","secondary_branchlet","lateral_root","secondary_root","primary_root_hypothesis"];
    let mut lookup=BTreeMap::<String,usize>::new();
    let mut parents=vec![None;count];
    let mut children=vec![Vec::<usize>::new();count];
    for (i,o) in r.organs.iter().enumerate() {
        if !key_valid(&o.key) || lookup.contains_key(&o.key) || !finite_point(o.position_m)
            || !data.radii_m[i].is_finite() || !(0.0001..=0.6).contains(&data.radii_m[i])
            || !allowed_roles.contains(&data.roles[i].as_str()) {
            return Err(format!("Invalid organ/profile at index {i}"));
        }
        match &o.parent {
            None if i==0 && data.roles[i]=="root_collar" => {},
            Some(k) => {
                let p=*lookup.get(k).ok_or_else(||format!("Parent must precede organ {}",o.key))?;
                if distance2(o.position_m,r.organs[p].position_m)<1e-8 { return Err("Zero-length authored edge".into()); }
                parents[i]=Some(p); children[p].push(i);
            },
            _ => return Err("Exactly one first root-collar organ is required".into()),
        }
        lookup.insert(o.key.clone(),i);
    }
    let mut shoot_keys=BTreeSet::new();
    let mut total_leaves=0usize;
    for s in &data.sprays {
        let Some(&edge)=lookup.get(&s.edge_end) else { return Err("Unknown shoot parent edge".into()); };
        if parents[edge].is_none() || !key_valid(&s.key) || lookup.contains_key(&s.key) || !shoot_keys.insert(s.key.clone())
            || !s.edge_fraction.is_finite() || !(0.0..=1.0).contains(&s.edge_fraction)
            || !finite_point(s.direction) || distance2(s.direction,[0.0;3])<1e-8
            || !s.length_m.is_finite() || !(0.12..=0.85).contains(&s.length_m)
            || !s.leaf_length_m.is_finite() || !(0.08..=0.22).contains(&s.leaf_length_m)
            || !s.leaf_width_m.is_finite() || !(0.01..=0.04).contains(&s.leaf_width_m)
            || !(6..=32).contains(&s.leaf_count) || !s.seed.is_finite() || s.seed.abs()>1e4 {
            return Err(format!("Invalid attached shoot {}",s.key));
        }
        if data.roles[edge].contains("root") { return Err("Adult foliage attached below root collar".into()); }
        total_leaves+=s.leaf_count;
    }
    if total_leaves>6144 { return Err("Leaf working set refused".into()); }

    let mut wood=SubmeshBuilder::new();
    let mut foliage=SubmeshBuilder::new();
    let mut bindings=Vec::new();
    // Same chain partition as Poet botanical v2: start at root/fork and continue
    // through single-child nodes. Fork caps overlap; they are NOT a welded union.
    for child in 1..count {
        let parent=parents[child].ok_or("Missing validated parent")?;
        if parent!=0 && children[parent].len()==1 { continue; }
        let mut chain=vec![parent,child];
        while children[*chain.last().ok_or("Empty chain")?].len()==1 {
            let last=*chain.last().ok_or("Empty chain")?;
            chain.push(children[last][0]);
        }
        let axis=chain.iter().map(|&i|point(r.organs[i].position_m)).collect::<Vec<_>>();
        let root=data.roles[child].contains("root");
        let branch=data.roles[child]!=data.roles[parent];
        let stations=chain.iter().enumerate().map(|(j,&i)|{
            // Source-authored station radius. A departing side-axis base is
            // bounded by its own first radius rather than inheriting bole width.
            let radius=if j==0 && branch { data.radii_m[i].min(data.radii_m[child]*1.45) } else {data.radii_m[i]};
            ExtrusionStation{scale_x:radius as f64,scale_y:radius as f64,twist_rad:0.0}
        }).collect::<Vec<_>>();
        let (base,tip)=if root {([0.34,0.29,0.22,1.0],[0.27,0.23,0.18,1.0])}
            else {([0.70,0.69,0.61,1.0],[0.80,0.79,0.72,1.0])};
        let start=wood.positions.len(); let index_start=wood.indices.len();
        let source_vertices=append_hard_cap_chain(&mut wood,&axis,&stations,r.ring_slices,base,tip)?;
        if wood.positions.len()==start || wood.indices.len()==index_start { return Err("Core chain sweep refused authored axis".into()); }
        bindings.push(serde_json::json!({"kind":"chain","organ_keys":chain.iter().map(|&i|&r.organs[i].key).collect::<Vec<_>>(),"source_radii_m":chain.iter().map(|&i|data.radii_m[i]).collect::<Vec<_>>(),"effective_station_radii_m":stations.iter().map(|station|station.scale_x).collect::<Vec<_>>(),"radius_projection":"At a role-changing departure only, base radius is min(source parent radius, first child radius * 1.45); all remaining stations use source radius.","derivative_to_base_sweep_vertices":source_vertices,"normal_uv_scope":"Core hard-cap split normals; wall V and color follow station arc length; caps planar UV; longitudinal texture seam not yet split","wood_vertices":[start,wood.positions.len()],"wood_indices":[index_start,wood.indices.len()],"fork_surface":"separate_capped_chain"}));
    }
    for s in &data.sprays {
        let end=lookup[&s.edge_end]; let parent=parents[end].ok_or("Invalid shoot edge")?;
        let a=r.organs[parent].position_m; let b=r.organs[end].position_m;
        let attach=[a[0]+(b[0]-a[0])*s.edge_fraction,a[1]+(b[1]-a[1])*s.edge_fraction,a[2]+(b[2]-a[2])*s.edge_fraction];
        let w=wood.positions.len();let wi=wood.indices.len();let f=foliage.positions.len();let fi=foliage.indices.len();
        crate::botanical_field_builder::add_white_gum_authored_spray(&mut wood,&mut foliage,attach,s.direction,s.length_m,s.leaf_length_m,s.leaf_width_m,s.leaf_count,s.seed);
        if wood.positions.len()==w || foliage.positions.len()==f { return Err("Attached foliage generation refused".into()); }
        bindings.push(serde_json::json!({"kind":"leaf_bearing_shoot","key":s.key,"parent_edge_end":s.edge_end,"edge_fraction":s.edge_fraction,"leaf_identifier_pattern":format!("{}/leaf/{{index}}",s.key),"leaf_count":s.leaf_count,"wood_vertices":[w,wood.positions.len()],"wood_indices":[wi,wood.indices.len()],"foliage_vertices":[f,foliage.positions.len()],"foliage_indices":[fi,foliage.indices.len()],"render_binding_granularity":"shoot; per-leaf range mapping pending"}));
    }
    for mesh in [&wood,&foliage] {
        if mesh.positions.len()>400_000 || mesh.indices.len()>1_800_000
            || mesh.positions.iter().flatten().chain(mesh.normals.iter().flatten()).any(|v|!v.is_finite())
            || mesh.normals.iter().any(|n|distance2(*n,[0.0;3])<0.5)
            || mesh.indices.iter().any(|&i|i as usize>=mesh.positions.len()) {
            return Err("Generated authored projection failed finite/index/working-set bounds".into());
        }
    }
    let metadata=serde_json::json!({
        "candidate":"white-gum-authored-review-r2","acceptance":"pending human and coordinator review",
        "authored_source":serde_json::from_str::<serde_json::Value>(source).map_err(|e|e.to_string())?,
        "source_binding":"runtime authored JSON / Vibe value; not an accepted Poet SourceSession",
        "structural_organs":count,"leaf_bearing_shoots":data.sprays.len(),"leaves":total_leaves,
        "units":"metres; +Y up","collar_position_m":r.organs[0].position_m,"bindings":bindings,
        "retained_inactive_recipe_controls":["node_length_m","kill_radius_m","perception_radius_m","cone_angle_rad","max_iterations","tip_radius_m","pipe_exponent"],
        "control_scope":"Growth/proposal/pipe-model controls are retained for source compatibility but do not affect this adopted-axis explicit-radius derivative. UI must label these inactive; do not imply edits change this mesh.",
        "roots":"authored hypothetical primary/lateral axes; not measured specimen anatomy",
        "junctions":"separate capped chain surfaces; no welded master claim",
        "physics":{"root_soil_coupled":false,"wind_load_coupled":false,"metamer_graph_bound":false},
        "targets":{"film":"incomplete","open_world_game":"incomplete","cartoon":"incomplete"}
    });
    Ok(WhiteGumAuthoredReview{wood,foliage,metadata})
}
