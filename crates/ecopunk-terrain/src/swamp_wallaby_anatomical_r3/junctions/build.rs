use super::*;
use qualia_core_db::specialized_libs::computational_geometry::parametric_cad::{bezier_eval_into,loft_profiles};

pub(super) struct CurveControls {pub points:[Point3;6],pub len:usize}
/// Elevate the existing cubic exactly as a polynomial, then perturb only the
/// two interior controls. This adds 10*t²*(1-t)²*bulge; endpoint positions and
/// first derivatives are unchanged mathematically. Finite-grid geometry still
/// has to pass the actual boundary-angle, intersection and native-readback gates.
pub(super) fn curve_controls(p:[Point3;4],bulge:Option<[f64;3]>)->CurveControls{
    let mut points=[Point3::default();6];
    let Some(b)=bulge else {points[..4].copy_from_slice(&p);return CurveControls{points,len:4}};
    let v=p.map(|v|[v.x,v.y,v.z]);
    let weighted=|weights:[f64;4],offset:[f64;3]|{
        let q: [f64;3]=std::array::from_fn(|axis|weights.iter().zip(v).map(|(&w,p)|w*p[axis]).sum::<f64>()+offset[axis]);
        Point3::new(q[0],q[1],q[2])
    };
    points[0]=p[0];points[1]=weighted([0.4,0.6,0.0,0.0],[0.0;3]);
    points[2]=weighted([0.1,0.6,0.3,0.0],b);
    points[3]=weighted([0.0,0.3,0.6,0.1],b);
    points[4]=weighted([0.0,0.0,0.6,0.4],[0.0;3]);points[5]=p[3];
    CurveControls{points,len:6}
}

pub(super) fn apply(s:&Source,a:&mut AnatomyMeshAccumulator,parts:&[Part],plans:&[ports::Plan],bytes:usize,budget:&mut Budget)
    ->Result<(Vec<Part>,JunctionReceipt),String>{
    let set=s.surface_junctions.as_ref().unwrap();
    let aliases=ports::aliases(plans,a.positions.len());
    let mut source_aliases=Vec::new();
    for (i,&alias) in aliases.iter().enumerate(){if i!=alias as usize {
        // An authored parametric U seam is one vertex by construction. This is
        // explicit identity, never a position-nearness merge.
        if a.joint_indices[i]!=a.joint_indices[alias as usize] || a.joint_weights[i]!=a.joint_weights[alias as usize] {
            return Err("Junction authored source seam weights disagree".into());
        }
        a.positions[i]=a.positions[alias as usize];
        source_aliases.push([i as u32,alias]);
    }}
    let removed:BTreeSet<_>=plans.iter().flat_map(|p|p.removed.iter().copied()).collect();
    let mut next_indices=Vec::with_capacity(a.indices.capacity());
    let mut next_parts=Vec::with_capacity(parts.len()+plans.len());
    // Validate old ownership once: exactly one Part per face, no missing face.
    let mut ownership=vec![false;a.indices.len()/3];
    for part in parts {
        if part.indices.start%3!=0||part.indices.end%3!=0||part.indices.end>a.indices.len(){return Err("Junction invalid old Part range".into())}
        let start=next_indices.len();
        for face in part.indices.start/3..part.indices.end/3 {
            budget.charge(4)?;
            if ownership[face]{return Err("Junction source Part face ownership overlaps".into())}
            ownership[face]=true;
            if removed.contains(&(face as u32)){continue}
            let tri=&a.indices[face*3..face*3+3];
            if tri.iter().any(|v|!part.vertices.contains(&(*v as usize))){return Err("Junction index escapes old Part".into())}
            next_indices.extend_from_slice(tri);
        }
        if next_indices.len()!=start {next_parts.push(Part{source_id:part.source_id.clone(),material:part.material,
            vertices:part.vertices.clone(),indices:start..next_indices.len()});}
    }
    if ownership.iter().any(|v|!*v){return Err("Junction source face has no Part owner".into())}
    a.indices=next_indices;
    let mut receipt=JunctionReceipt{schema:"ecopunk.anatomical-surface-junction/v1",
        scope:"Explicit discrete torso/limb graft; selected component topology and changed nonadjacent face intersections checked; no analytic C1, posed collision or whole-body closure claim",
        uv_policy:"New rectangular collar charts; explicit U seam and both attachment chart seams; exact geometric/skin aliases, independent UV coordinates",
        normal_policy:"Physical area-weighted normals on shared prescribed topology aliases after surgery; tangents remain the canonical material compiler's responsibility",
        source_aliases,records:Vec::with_capacity(plans.len()),work_used:0,reserved_bytes:bytes,component_faces:0};
    for p in plans {
        let r=&set.recipes[p.recipe];let n=p.body_loop.len();let stride=n+1;
        let mut controls=Vec::with_capacity(n);
        for col in 0..n {
            budget.charge(if r.interior_bulge_m.is_some(){256}else{128})?;
            let p0=a.positions[p.body_loop[col]as usize];
            let inside=a.positions[p.body_inside[col]as usize];
            let p3=a.positions[p.limb_loop[col]as usize];
            let next=a.positions[p.limb_next[col]as usize];
            let normal=attributes::unit(a.normals[p.body_loop[col]as usize].map(f64::from))?;
            let raw=std::array::from_fn(|i|f64::from(inside[i])-f64::from(p0[i]));
            let radial=attributes::dot64(raw,normal);
            let d0: [f64;3]=std::array::from_fn(|i|(raw[i]-normal[i]*radial)*r.body_tangent_scale);
            let d1: [f64;3]=std::array::from_fn(|i|(f64::from(next[i])-f64::from(p3[i]))*r.limb_tangent_scale);
            attributes::unit(d0)?;attributes::unit(d1)?;
            controls.push(curve_controls([point(p0),Point3::new(f64::from(p0[0])+d0[0],f64::from(p0[1])+d0[1],f64::from(p0[2])+d0[2]),
                Point3::new(f64::from(p3[0])-d1[0],f64::from(p3[1])-d1[1],f64::from(p3[2])-d1[2]),point(p3)],r.interior_bulge_m));
        }
        let base=a.positions.len();let face_start=a.indices.len();
        let mut record=JunctionRecord{recipe:r.clone(),removed_source_faces:p.removed.clone(),vertices:Vec::with_capacity((r.subdivisions+1)*stride),
            topology_aliases:Vec::with_capacity(stride*2+r.subdivisions),boundary_min_normal_dot:1.0,maximum_discarded_weight:0.0};
        let mut curve_scratch=[Point3::default();6];
        for row in 0..=r.subdivisions {let t=row as f64/r.subdivisions as f64;
            for col in 0..=n {let at=col%n;budget.charge(if controls[at].len==6{512}else{256})?;
                let position=if row==0{a.positions[p.body_loop[at]as usize]}else if row==r.subdivisions{a.positions[p.limb_loop[at]as usize]}
                    else{let c=&controls[at];let v=bezier_eval_into(&c.points[..c.len],t,&mut curve_scratch).map_err(|e|format!("Junction curve refused: {e:?}"))?;[v.x as f32,v.y as f32,v.z as f32]};
                let (vertex,loss)=attributes::append(a,position,[col as f32/n as f32,t as f32],p.body_loop[at],p.limb_loop[at],t,r.max_influence_loss)?;
                record.maximum_discarded_weight=record.maximum_discarded_weight.max(loss);
                if row==0 {record.topology_aliases.push([vertex,p.body_loop[at]])}
                else if row==r.subdivisions {record.topology_aliases.push([vertex,p.limb_loop[at]])}
                else if col==n {record.topology_aliases.push([vertex,(base+row*stride)as u32])}
                record.vertices.push(CollarVertexLineage{output_vertex:vertex,body_vertex:p.body_loop[at],body_interior_vertex:p.body_inside[at],
                    limb_vertex:p.limb_loop[at],limb_next_vertex:p.limb_next[at],parameter:t as f32});
            }
        }
        // Existing Core strip ordering gives +first-ring and -last-ring edge
        // directions; the port owner checked their opposite retained neighbours.
        let mut out=[Point3::default();130];let mut faces=[[0u32;3];128];
        let mut first=[Point3::default();65];let mut last=[Point3::default();65];
        for row in 0..r.subdivisions {
            budget.charge(product(stride,64)?)?;
            for col in 0..stride {first[col]=point(a.positions[base+row*stride+col]);last[col]=point(a.positions[base+(row+1)*stride+col]);}
            let (nv,nt)=loft_profiles(&first[..stride],&last[..stride],&mut out,&mut faces).map_err(|e|format!("Junction strip refused: {e:?}"))?;
            if nv!=stride*2||nt!=n*2{return Err("Junction strip layout changed".into())}
            for face in &faces[..nt] {for &index in face {a.indices.push((base+row*stride+index as usize)as u32)}}
        }
        next_parts.push(Part{source_id:format!("junction:{}",r.id),material:r.material,vertices:base..a.positions.len(),indices:face_start..a.indices.len()});
        receipt.records.push(record);
    }
    Ok((next_parts,receipt))
}
