//! Explicit closed subset study of retained authored axes. Never grafted into
//! the full-tree master, and never substitutes a coarse union for fine twigs.
use super::{read_source, point, WhiteGumAuthoredReview};
use serde::Deserialize;
use std::collections::{BTreeMap,BTreeSet};
use crate::scene_builder_10d::SubmeshBuilder;
use qualia_core_db::specialized_libs::computational_geometry::{self as cg, branch_graph_surface::*};
#[cfg(test)]
#[path="white_gum_connected_review_tests.rs"]
mod tests;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudySpec {
    pub schema:String, pub selected_organs:Vec<String>, pub origin_m:[f64;3],
    pub spacing_m:f64, pub dimensions:[usize;3], pub minimum_radius_samples:f64,
    pub min_axis_m:f64, pub projection_tolerance_m:f64, pub min_area_sq_m4:f64,
    pub max_incident_cosine:f64, pub uv_axes:[usize;2], pub metres_per_repeat:f32,
}

fn filled<T:Clone>(n:usize,value:T,bytes:&mut usize,cap:usize)->Result<Vec<T>,String>{
    let mut v=Vec::new();v.try_reserve_exact(n).map_err(|_|"Connected study allocation refused")?;
    *bytes=bytes.checked_add(v.capacity().checked_mul(std::mem::size_of::<T>()).ok_or("Connected capacity overflow")?)
        .ok_or("Connected capacity overflow")?;
    if *bytes>cap{return Err("Connected study actual buffer capacities exceed budget".into());}
    v.resize(n,value);Ok(v)
}
/// Host limits are separate from authored sampling controls. Returns unpublished
/// scratch; the existing SourceSession validator promotes only a successful build.
pub fn build(source:&str,limits:BranchGraphLimits,cancelled:&dyn Fn()->bool)->Result<WhiteGumAuthoredReview,String>{
    if cancelled(){return Err("Connected study cancelled".into());}
    let validated=read_source(source)?;let data=&validated.data;let graph=&data.botanical;
    let study=data.connected_study.as_ref().ok_or("Select white-gum-connected-study-v1 explicitly")?;
    if study.schema!="ecopunk.closed-branch-study/v1"||study.selected_organs.len()<2
        ||study.selected_organs.len()>limits.nodes||study.selected_organs.len()>256
        ||study.uv_axes[0]>=3||study.uv_axes[1]>=3||study.uv_axes[0]==study.uv_axes[1]
        ||!study.metres_per_repeat.is_finite()||!(0.000001..=1_000_000.0).contains(&study.metres_per_repeat){
        return Err("Invalid connected study source profile".into());
    }
    let mut selected=Vec::new();let mut keys=BTreeSet::new();
    for key in &study.selected_organs {
        if !keys.insert(key.as_str()){return Err("Duplicate selected organ".into());}
        selected.push(*validated.lookup.get(key).ok_or("Selected organ absent from retained source")?);
    }
    // Source order is authoritative and makes edge ordering deterministic.
    if selected.windows(2).any(|p|p[0]>=p[1]){return Err("Selected organs must retain original source order".into());}
    let local:BTreeMap<usize,u32>=selected.iter().enumerate().map(|(i,&s)|(s,i as u32)).collect();
    let mut parents=vec![cg::ROOT_PARENT;selected.len()];
    for (i,&source_index) in selected.iter().enumerate().skip(1){
        let parent=validated.parents[source_index].ok_or("Selected subset has an extra root")?;
        parents[i]=*local.get(&parent).ok_or("Selected subset omits an interior parent; no proximity welding permitted")?;
    }
    let positions=selected.iter().map(|&i|point(graph.organs[i].position_m)).collect::<Vec<_>>();
    let radii=selected.iter().map(|&i|f64::from(data.radii_m[i])).collect::<Vec<_>>();
    let spec=BranchGraphSpec{origin:cg::Point3::new(study.origin_m[0],study.origin_m[1],study.origin_m[2]),
        spacing:study.spacing_m,dimensions:study.dimensions,minimum_radius_samples:study.minimum_radius_samples,
        min_axis:study.min_axis_m,projection_tolerance:study.projection_tolerance_m,min_area_sq:study.min_area_sq_m4,
        max_incident_cosine:study.max_incident_cosine};
    // Reserve projection/metadata overhead as well as all Core buffers. This is
    // a logical envelope, not a claim to bound serde allocator metadata or RSS.
    let wrapper=limits.vertices.checked_mul(320).and_then(|v|limits.triangles.checked_mul(80).and_then(|t|v.checked_add(t)))
        .and_then(|v|v.checked_add(2*1024*1024)).ok_or("Connected projection reservation overflow")?;
    let mut core_limits=limits;core_limits.bytes=limits.bytes.checked_sub(wrapper).ok_or("Connected projection exceeds caller byte budget")?;
    let projection_work=limits.vertices.checked_mul(32).and_then(|v|limits.triangles.checked_mul(192).and_then(|t|v.checked_add(t)))
        .ok_or("Connected projection work overflow")?;
    core_limits.work=limits.work.checked_sub(projection_work).ok_or("Connected projection exceeds caller work budget")?;
    let b=branch_graph_surface_bounds(selected.len(),spec,core_limits).map_err(|e|format!("Connected study bounds: {e:?}"))?;
    let mut used=wrapper;let zero=cg::Point3::new(0.,0.,0.);
    let mut samples=filled(b.samples,zero,&mut used,limits.bytes)?;let mut values=filled(b.samples,0.,&mut used,limits.bytes)?;
    let mut segments=filled(b.segments,cg::tapered_branch_field::TaperedFieldSegment{a:zero,b:zero,radius_a:0.,radius_b:0.},&mut used,limits.bytes)?;
    let mut boxes=filled(b.segments,cg::Aabb::new(zero,zero),&mut used,limits.bytes)?;
    let mut vertices=filled(b.vertices,zero,&mut used,limits.bytes)?;let mut triangles=filled(b.triangles,[0;3],&mut used,limits.bytes)?;
    let mut normals=filled(b.vertices,zero,&mut used,limits.bytes)?;let mut faces=filled(b.triangles,zero,&mut used,limits.bytes)?;
    let mut edges=filled(b.vertices,[0usize;2],&mut used,limits.bytes)?;
    let mut extraction_slots=filled(b.extraction_slots,cg::indexed_tetra_surface::TetraEdgeSlot::default(),&mut used,limits.bytes)?;
    let q=b.topology;
    let mut half_edges=filled(q.half_edges,cg::HalfEdge::default(),&mut used,limits.bytes)?;
    let mut topology_slots=filled(q.edge_slots,cg::EdgeSlot::default(),&mut used,limits.bytes)?;
    let mut starts=filled(q.vertex_starts,0u32,&mut used,limits.bytes)?;let mut visited=filled(q.visited_half_edges,false,&mut used,limits.bytes)?;
    let mut labels=filled(q.face_labels,0u32,&mut used,limits.bytes)?;let mut queue=filled(q.face_queue,0u32,&mut used,limits.bytes)?;
    let result=branch_graph_surface_into(BranchGraphInput{positions:&positions,parents:&parents,radii:&radii},spec,core_limits,
        BranchGraphBuffers{samples:&mut samples,values:&mut values,segments:&mut segments,boxes:&mut boxes,
            vertices:&mut vertices,triangles:&mut triangles,normals:&mut normals,face_normals:&mut faces,sample_edges:&mut edges,
            extraction_slots:&mut extraction_slots,half_edges:&mut half_edges,topology_slots:&mut topology_slots,
            vertex_starts:&mut starts,visited:&mut visited,face_labels:&mut labels,face_queue:&mut queue},cancelled)
        .map_err(|e|format!("Connected branch study refused; previous source retained: {e:?}"))?;
    vertices.truncate(result.vertices);normals.truncate(result.vertices);edges.truncate(result.vertices);triangles.truncate(result.triangles);
    // Retain the exact extraction vertex order. No nearest-organ reassignment.
    let mut wood=SubmeshBuilder::new();
    wood.positions.try_reserve_exact(result.vertices).map_err(|_|"Connected projection allocation refused")?;
    wood.normals.try_reserve_exact(result.vertices).map_err(|_|"Connected normal allocation refused")?;
    wood.colors.try_reserve_exact(result.vertices).map_err(|_|"Connected color allocation refused")?;
    wood.uvs.try_reserve_exact(result.vertices).map_err(|_|"Connected UV allocation refused")?;
    wood.indices.try_reserve_exact(result.triangles.checked_mul(3).ok_or("Connected index overflow")?).map_err(|_|"Connected index allocation refused")?;
    for (p,n) in vertices.iter().zip(&normals){
        let pos=[p.x as f32,p.y as f32,p.z as f32];let normal=[n.x as f32,n.y as f32,n.z as f32];
        if pos.iter().chain(&normal).any(|v|!v.is_finite()){return Err("Connected projection exceeds native float range".into());}
        wood.push_vertex(pos,normal,[0.70,0.69,0.61,1.0],[0.,0.]);
    }
    qualia_core_db::render::surface_authoring::planar_uvs_into(&wood.positions,study.uv_axes,study.metres_per_repeat,&mut wood.uvs)
        .map_err(|e|format!("Connected planar UV refused: {e:?}"))?;
    if wood.uvs.iter().flatten().any(|v|!v.is_finite()||v.abs()>1e6){return Err("Connected planar UV exceeds native bounds".into());}
    for (i,t) in triangles.into_iter().enumerate() {
        let projected=face_vector(&wood.positions,t)?;let original=faces[i];
        check_face(projected,[original.x,original.y,original.z],study.min_area_sq_m4,"f32 projection",i)?;
        wood.push_tri(t[0],t[1],t[2]);
    }
    let excluded=graph.organs.iter().filter(|o|!keys.contains(o.key.as_str())).map(|o|o.key.as_str()).collect::<Vec<_>>();
    let cut_edges=graph.organs.iter().enumerate().skip(1).filter_map(|(i,o)|{
        let parent=validated.parents[i]?;let inside=local.contains_key(&i);let parent_inside=local.contains_key(&parent);
        (inside!=parent_inside).then(||serde_json::json!({"parent":graph.organs[parent].key,"child":o.key}))
    }).collect::<Vec<_>>();
    let metadata=serde_json::json!({"candidate":"white-gum-connected-study-v1","acceptance":"unreviewed",
        "authored_source":serde_json::from_str::<serde_json::Value>(source).map_err(|e|e.to_string())?,
        "projection":"explicit selected closed branch study; never grafted or silently substituted for the full tree",
        "selected_organs":study.selected_organs,"selected_source_indices":selected,"local_parent_indices":parents,
        "source_radii_m":radii,"excluded_organs":excluded,"excluded_sprays":data.sprays.iter().map(|s|s.key.as_str()).collect::<Vec<_>>(),
        "cut_source_edges":cut_edges,"cut_boundary":"endpoint-radius closed field caps; no matching open ring or graft contract",
        "vertex_grid_sample_edges":edges,"vertex_identity_scope":"grid-local extraction provenance; no asserted organ identity",
        "field":"hard-min tapered segments; C0 collars possible; area-weighted shading is not geometric smoothing",
        "clearance":"nonincident expanded AABB separation screen; false refusals possible; no general self-intersection certificate",
        "normal_uv_scope":"Core area-weighted normals; explicit planar UV; no longitudinal bark continuity or tangent quality acceptance",
        "sampling":{"spacing_m":study.spacing_m,"dimensions":study.dimensions,"samples":result.samples,
            "smallest_radius_over_spacing":result.smallest_radius_over_spacing,"required_radius_samples":study.minimum_radius_samples},
        "topology":{"vertices":result.vertices,"triangles":result.triangles,"components":result.topology.component_count,"closed_vertex_fans":true},
        "host_bounds":{"logical_buffer_bytes":b.bytes,"wrapper_reservation":wrapper,"structural_work_reserved":b.reserved_work,
            "projection_and_readback_work_reserved":projection_work},
        "targets":{"film":"incomplete","open_world_game":"incomplete","cartoon":"incomplete"}});
    if cancelled(){return Err("Connected study cancelled before delivery".into());}
    Ok(WhiteGumAuthoredReview{wood,foliage:SubmeshBuilder::new(),metadata})
}

fn face_vector(positions:&[[f32;3]],triangle:[u32;3])->Result<[f64;3],String>{
    let a=positions.get(triangle[0] as usize).ok_or("Projection index outside vertices")?;
    let b=positions.get(triangle[1] as usize).ok_or("Projection index outside vertices")?;
    let c=positions.get(triangle[2] as usize).ok_or("Projection index outside vertices")?;
    let u=std::array::from_fn::<_,3,_>(|i|f64::from(b[i])-f64::from(a[i]));
    let v=std::array::from_fn::<_,3,_>(|i|f64::from(c[i])-f64::from(a[i]));
    Ok([u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]])
}
fn check_face(current:[f64;3],reference:[f64;3],min_area_sq:f64,stage:&str,index:usize)->Result<(),String>{
    let area=current.iter().map(|v|v*v).sum::<f64>();
    let orientation=current.iter().zip(reference).map(|(a,b)|a*b).sum::<f64>();
    if !area.is_finite()||area<=min_area_sq||!orientation.is_finite()||orientation<=0.0 {
        return Err(format!("Connected study {stage} collapses or reverses face {index}; adjust explicit sampling or retain previous source"));
    }Ok(())
}
/// Unpublished shape witness retained only across the canonical .10d compile.
/// It verifies projection loss, not a new geometry encoding or acceptance gate.
pub(crate) struct ProjectionWitness {vertices:usize,triangles:Vec<[u32;3]>,faces:Vec<[f64;3]>,min_area_sq:f64}
pub(crate) fn projection_witness(mesh:&SubmeshBuilder,min_area_sq:f64)->Result<ProjectionWitness,String>{
    if !min_area_sq.is_finite()||min_area_sq<0.0||mesh.indices.len()%3!=0{return Err("Invalid connected projection witness".into());}
    let count=mesh.indices.len()/3;let mut triangles=Vec::new();let mut faces=Vec::new();
    triangles.try_reserve_exact(count).map_err(|_|"Connected index witness allocation refused")?;
    faces.try_reserve_exact(count).map_err(|_|"Connected face witness allocation refused")?;
    for (i,t) in mesh.indices.chunks_exact(3).enumerate(){let triangle=[t[0],t[1],t[2]];
        let face=face_vector(&mesh.positions,triangle)?;check_face(face,face,min_area_sq,"f32 witness",i)?;
        triangles.push(triangle);faces.push(face);
    }
    Ok(ProjectionWitness{vertices:mesh.positions.len(),triangles,faces,min_area_sq})
}
pub(crate) fn qualify_compiled_projection(bytes:&[u8],witness:&ProjectionWitness)->Result<(),String>{
    use qualia_core_db::container_10d::{self as native,Container10dHeader,SectionType};
    let header=Container10dHeader::parse(bytes).map_err(|e|format!("Connected .10d header: {e:?}"))?;
    let sections=native::parse_section_table(bytes,&header).map_err(|e|format!("Connected .10d table: {e:?}"))?;
    let mut meshes=sections.iter().filter(|s|s.typ()==Some(SectionType::QuantizedMesh));
    let section=meshes.next().ok_or("Connected .10d mesh absent")?;
    if meshes.next().is_some(){return Err("Connected .10d has ambiguous mesh sections".into());}
    let start=usize::try_from(section.byte_offset).map_err(|_|"Connected .10d offset overflow")?;
    let length=usize::try_from(section.byte_length).map_err(|_|"Connected .10d length overflow")?;
    let end=start.checked_add(length).ok_or("Connected .10d range overflow")?;
    let payload=bytes.get(start..end).ok_or("Connected .10d mesh outside file")?;
    let (h,needed)=native::parse_mesh_header(payload).map_err(|e|format!("Connected mesh header: {e:?}"))?;
    if needed!=payload.len()||h.vertex_count as usize!=witness.vertices||h.triangle_count as usize!=witness.triangles.len(){
        return Err("Connected .10d geometry count changed during compile".into());}
    let decoded=native::decode_mesh_section(payload).map_err(|e|format!("Connected mesh decode: {e:?}"))?;
    if decoded.triangles!=witness.triangles||decoded.positions.iter().flatten().any(|v|!v.is_finite()){
        return Err("Connected .10d geometry correspondence changed during compile".into());}
    for (i,&t) in decoded.triangles.iter().enumerate(){check_face(face_vector(&decoded.positions,t)?,witness.faces[i],witness.min_area_sq,"native .10d quantization",i)?;}
    Ok(())
}
