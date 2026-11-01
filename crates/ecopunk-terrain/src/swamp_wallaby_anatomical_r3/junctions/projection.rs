//! Qualify graft faces and attachment aliases in the actual canonical .10d.
use super::*;
use crate::species_review::ReviewPart;
use qualia_core_db::container_10d::{self as native,Container10dHeader,SectionType};

pub struct JunctionProjection {
    vertices:usize,triangles:usize,faces:Vec<(usize,[u32;3],[f64;3])>,
    aliases:Vec<[u32;2]>,boundaries:Vec<([usize;2],f64)>,
    remaining:usize,initial:usize,source_work:usize,reserved:usize,changed:usize,
}
#[derive(Serialize)]
pub struct JunctionProjectionReport {
    pub scope:&'static str,
    pub source_work_used:usize,
    pub projection_work_used:usize,
    pub reserved_bytes:usize,
    pub changed_faces:usize,
    pub alias_pairs:usize,
}
fn root(ids:&[u32],mut at:u32)->Result<u32,String>{
    for _ in 0..4 {let next=*ids.get(at as usize).ok_or("Junction projection alias out of range")?;
        if next==at{return Ok(at)}at=next;
    }
    Err("Junction projection alias chain invalid".into())
}
pub fn prepare_junction_projection(c:&Candidate,parts:&[ReviewPart],lineage:&[u32])->Result<Option<JunctionProjection>,String>{
    let Some(receipt)=c.surface_junctions.as_ref()else{return Ok(None)};
    let set=c.source.surface_junctions.as_ref().ok_or("Junction receipt lacks authored policy")?;
    let vertices=parts.iter().try_fold(0,|n,p|sum(n,p.mesh.positions.len()))?;
    let index_count=parts.iter().try_fold(0,|n,p|sum(n,p.mesh.indices.len()))?;
    if vertices!=lineage.len()||vertices>750_000||index_count%3!=0||index_count>9_000_000 {
        return Err("Junction render projection exceeds correspondence/count profile".into());
    }
    let triangles=index_count/3;
    let reserved=sum(sum(product(c.skin.positions.len(),160)?,product(vertices,256)?)?,sum(product(triangles,96)?,58*1024*1024)?)?;
    if reserved>set.cold_bytes{return Err("Junction compiled readback cold reservation exceeds authored budget".into())}
    let initial=set.work_units.checked_sub(receipt.work_used).ok_or("Junction source exhausted work")?;
    let mut budget=Budget{remaining:initial,initial};
    budget.charge(sum(product(c.skin.positions.len(),4)?,sum(product(vertices,128)?,product(triangles,160)?)?)?)?;
    let mut ids:Vec<_>=(0..c.skin.positions.len()as u32).collect();
    for &[from,to] in &receipt.source_aliases {ids[from as usize]=to;}
    let mut changed_vertices=BTreeSet::new();
    for r in &receipt.records {for v in &r.vertices{changed_vertices.insert(v.output_vertex);}
        for &[from,to] in &r.topology_aliases{ids[from as usize]=to;}
    }
    let mut first=vec![u32::MAX;ids.len()];let mut aliases=Vec::new();
    for (render,&source) in lineage.iter().enumerate(){let id=root(&ids,source)? as usize;
        if first[id]==u32::MAX{first[id]=render as u32}else{aliases.push([first[id],render as u32]);}
    }
    let mut boundary:BTreeMap<(u32,u32),(f64,Vec<usize>)>=BTreeMap::new();
    for r in &receipt.records {
        let n=2*(r.recipe.body_rows[1]-r.recipe.body_rows[0]+r.recipe.body_column_count);let stride=n+1;
        for row in [0,r.recipe.subdivisions] {for col in 0..n {
            let a=root(&ids,r.vertices[row*stride+col].output_vertex)?;
            let b=root(&ids,r.vertices[row*stride+col+1].output_vertex)?;
            boundary.insert((a.min(b),a.max(b)),(r.recipe.min_boundary_normal_dot,Vec::with_capacity(2)));
        }}
    }
    let names:BTreeSet<_>=receipt.records.iter().map(|r|format!("junction:{}",r.recipe.id)).collect();
    let mut expected=BTreeMap::new();
    for part in &c.parts {if names.contains(&part.source_id) {for t in c.skin.indices[part.indices.clone()].chunks_exact(3){
        let face=[t[0],t[1],t[2]];
        if expected.insert(face,attributes::normal(c.skin.positions[t[0]as usize],c.skin.positions[t[1]as usize],c.skin.positions[t[2]as usize])).is_some(){return Err("Duplicate source junction face".into())}
    }}}
    let changed=expected.len();let mut faces=BTreeMap::new();let(mut vertex_base,mut face_base)=(0usize,0usize);
    for part in parts {
        if part.mesh.indices.len()%3!=0{return Err("Junction render triangle layout invalid".into())}
        for (i,t) in part.mesh.indices.chunks_exact(3).enumerate(){
            let mut global=[0u32;3];let mut source=[0u32;3];
            for k in 0..3 {if t[k]as usize>=part.mesh.positions.len(){return Err("Junction render index invalid".into())}
                global[k]=(vertex_base+t[k]as usize)as u32;source[k]=lineage[global[k]as usize];}
            let face_index=face_base+i;
            if let Some(area)=expected.remove(&source){faces.insert(face_index,(global,area));}
            else if source.iter().any(|i|changed_vertices.contains(i)){return Err("Junction material projection changed source face correspondence".into())}
            let canonical=[root(&ids,source[0])?,root(&ids,source[1])?,root(&ids,source[2])?];
            for edge in 0..3 {let(a,b)=(canonical[edge],canonical[(edge+1)%3]);
                if let Some((_,incident))=boundary.get_mut(&(a.min(b),a.max(b))) {
                    if incident.len()==2{return Err("Junction projected boundary nonmanifold".into())}
                    incident.push(face_index);
                    faces.entry(face_index).or_insert_with(||(global,attributes::normal(part.mesh.positions[t[0]as usize],part.mesh.positions[t[1]as usize],part.mesh.positions[t[2]as usize])));
                }
            }
        }
        vertex_base+=part.mesh.positions.len();face_base+=part.mesh.indices.len()/3;
    }
    if !expected.is_empty()||changed==0{return Err("Junction projection omitted source collar faces".into())}
    let mut boundaries=Vec::with_capacity(boundary.len());
    for (_,(threshold,incident)) in boundary {if incident.len()!=2{return Err("Junction projected attachment not paired".into())}
        boundaries.push(([incident[0],incident[1]],threshold));}
    Ok(Some(JunctionProjection{vertices,triangles,faces:faces.into_iter().map(|(i,(t,n))|(i,t,n)).collect(),aliases,boundaries,
        remaining:budget.remaining,initial,source_work:receipt.work_used,reserved,changed}))
}

pub fn qualify_junction_projection(bytes:&[u8],w:JunctionProjection)->Result<JunctionProjectionReport,String>{
    if bytes.len()>42*1024*1024{return Err("Junction compiled container exceeds readback profile".into())}
    let mut budget=Budget{remaining:w.remaining,initial:w.initial};
    budget.charge(sum(product(w.vertices,64)?,product(w.triangles,64)?)?)?;
    let header=Container10dHeader::parse(bytes).map_err(|e|format!("Junction native header: {e:?}"))?;
    let sections=native::parse_section_table(bytes,&header).map_err(|e|format!("Junction native table: {e:?}"))?;
    let mut meshes=sections.iter().filter(|s|s.typ()==Some(SectionType::QuantizedMesh));
    let section=meshes.next().ok_or("Junction compiled mesh absent")?;
    if meshes.next().is_some(){return Err("Junction compiled mesh ambiguous".into())}
    let start=usize::try_from(section.byte_offset).map_err(|_|"Junction mesh offset overflow")?;
    let end=sum(start,usize::try_from(section.byte_length).map_err(|_|"Junction mesh length overflow")?)?;
    let payload=bytes.get(start..end).ok_or("Junction mesh outside container")?;
    let (h,size)=native::parse_mesh_header(payload).map_err(|e|format!("Junction canonical mesh header: {e:?}"))?;
    if size!=payload.len()||h.vertex_count as usize!=w.vertices||h.triangle_count as usize!=w.triangles {
        return Err("Junction compiled mesh counts changed".into());
    }
    let decoded=native::decode_mesh_section(payload).map_err(|e|format!("Junction canonical mesh decode: {e:?}"))?;
    let frame_limits=native::MeshFrameDecodeLimits{max_vertices:w.vertices,max_payload_bytes:bytes.len(),max_decoded_bytes:product(w.vertices,28)?};
    let layout=native::inspect_container_mesh_frames(bytes,w.vertices,frame_limits).map_err(|e|format!("Junction frame layout: {e:?}"))?
        .ok_or("Junction compiled normals absent")?;
    if !layout.has_normals{return Err("Junction compiled normal stream absent".into())}
    let mut normals=vec![[0.0;3];w.vertices];let mut tangents=vec![[0.0;4];if layout.has_tangents{w.vertices}else{0}];
    native::read_container_mesh_frames_into(bytes,w.vertices,frame_limits,&mut normals,&mut tangents).map_err(|e|format!("Junction typed frame decode: {e:?}"))?;
    for &[a,b] in &w.aliases {
        if decoded.positions[a as usize].map(f32::to_bits)!=decoded.positions[b as usize].map(f32::to_bits)
            ||normals[a as usize].map(f32::to_bits)!=normals[b as usize].map(f32::to_bits) {
            return Err("Junction native projection separates prescribed position/normal aliases".into());
        }
    }
    for &(index,expected,reference) in &w.faces {
        let actual=decoded.triangles.get(index).ok_or("Junction native face absent")?;
        if *actual!=expected{return Err("Junction native triangle correspondence changed".into())}
        let p=actual.map(|v|decoded.positions[v as usize]);let area=attributes::normal(p[0],p[1],p[2]);
        if attributes::dot64(area,area)<=1e-20||!attributes::dot64(area,area).is_finite()||attributes::dot64(area,reference)<=0.0 {
            return Err("Junction native quantization collapsed/reversed a graft or attachment face".into());
        }
    }
    for &(pair,threshold) in &w.boundaries {
        let mut n=[[0.0;3];2];
        for k in 0..2 {let p=decoded.triangles[pair[k]].map(|v|decoded.positions[v as usize]);n[k]=attributes::unit(attributes::normal(p[0],p[1],p[2]))?;}
        if attributes::dot64(n[0],n[1])<threshold{return Err("Junction native quantization exceeds authored boundary angle".into())}
    }
    Ok(JunctionProjectionReport{scope:"Actual canonical quantized graft/attachment faces and exact position/normal aliases; tangent validity decoded, tangent correspondence/posed collisions not asserted",
        source_work_used:w.source_work,projection_work_used:budget.initial-budget.remaining,reserved_bytes:w.reserved,changed_faces:w.changed,alias_pairs:w.aliases.len()})
}

#[cfg(test)]
#[path="projection_tests.rs"]
mod tests;
