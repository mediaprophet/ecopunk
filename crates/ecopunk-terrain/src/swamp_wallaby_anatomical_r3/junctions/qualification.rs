use super::*;
use qualia_core_db::specialized_libs::computational_geometry::{
    surface_topology_qualification as topology, Aabb, EdgeSlot, HalfEdge, tri_tri_intersect_3,
};

fn selected(p:&Part,plans:&[ports::Plan],receipt:&JunctionReceipt)->bool{
    plans.iter().any(|plan|{
        let r=&receipt.records[plan.recipe].recipe;
        p.source_id==format!("{}:1",r.torso_loft)
            || p.source_id.starts_with(&format!("chain:{}:{}:",r.limb_chain,if r.mirrored{-1}else{1}))
            || p.source_id==format!("junction:{}",r.id)
    })
}
pub(super) fn check(a:&mut AnatomyMeshAccumulator,parts:&[Part],plans:&[ports::Plan],receipt:&mut JunctionReceipt,budget:&mut Budget)->Result<(),String>{
    let mut aliases=ports::aliases(plans,a.positions.len());
    for r in &receipt.records {for &[from,to] in &r.topology_aliases {aliases[from as usize]=aliases[to as usize];}}
    for (i,&other) in aliases.iter().enumerate(){if a.positions[i]!=a.positions[other as usize]{return Err("Junction declared alias positions disagree".into())}}
    let mut dense=vec![u32::MAX;a.positions.len()];let mut count=0u32;
    let mut triangles=Vec::new();let mut selected_faces=vec![false;a.indices.len()/3];
    for p in parts {if selected(p,plans,receipt){
        for face in p.indices.start/3..p.indices.end/3 {
            budget.charge(32)?;
            selected_faces[face]=true;
            let mut t=[0u32;3];
            for i in 0..3 {let v=aliases[a.indices[face*3+i]as usize]as usize;
                if dense[v]==u32::MAX {dense[v]=count;count+=1;}
                t[i]=dense[v];
            }
            triangles.push(t);
        }
    }}
    let bounds=topology::surface_qualification_storage_bounds(count,triangles.len()).map_err(|e|format!("Junction topology bounds: {e:?}"))?;
    topology::qualify_surface_topology_budgeted_into(count,&triangles,&mut budget.remaining,
        &mut vec![HalfEdge::default();bounds.half_edges],&mut vec![EdgeSlot::default();bounds.edge_slots],
        &mut vec![0;bounds.vertex_starts],&mut vec![false;bounds.visited_half_edges],
        &mut vec![0;bounds.face_labels],&mut vec![0;bounds.face_queue])
        .map_err(|e|format!("Junction selected connected surface refused: {e:?}"))?;
    receipt.component_faces=triangles.len();
    // Boundary comparisons use actual incident face geometry, not old shading
    // normals or endpoint derivative assertions on a continuous idealization.
    let mut boundary:BTreeMap<(u32,u32),(usize,usize,[[f64;3];2])>=BTreeMap::new();
    for p in plans {for ring in [&p.body_loop,&p.limb_loop] {for i in 0..ring.len(){
        let (x,y)=(aliases[ring[i]as usize],aliases[ring[(i+1)%ring.len()]as usize]);
        boundary.insert((x.min(y),x.max(y)),(p.recipe,0,[[0.0;3];2]));
    }}}
    let mut normals=vec![[0.0f64;3];a.positions.len()];
    let mut boxes=Vec::with_capacity(a.indices.len()/3);
    budget.charge(product(a.indices.len(),64)?)?;
    for (face,tri) in a.indices.chunks_exact(3).enumerate(){
        let p=tri.map_or_points(a)?;
        let normal=attributes::normal(p[0],p[1],p[2]);
        // Selected-source and all new collar faces must remain noncollapsed.
        if selected_faces[face] {
            if !attributes::dot64(normal,normal).is_finite() || attributes::dot64(normal,normal)<=1e-20 {
                return Err(format!("Junction selected face {face} collapsed"));
            }
            for &v in tri {let target=&mut normals[aliases[v as usize]as usize];for c in 0..3{target[c]+=normal[c];}}
            for e in 0..3 {
                let(x,y)=(aliases[tri[e]as usize],aliases[tri[(e+1)%3]as usize]);
                if let Some((_,n,values))=boundary.get_mut(&(x.min(y),x.max(y))) {
                    if *n>=2{return Err("Junction boundary nonmanifold".into())}
                    values[*n]=attributes::unit(normal)?;*n+=1;
                }
            }
        }
        boxes.push(Aabb::new(point(std::array::from_fn(|i|p[0][i].min(p[1][i]).min(p[2][i]))),
            point(std::array::from_fn(|i|p[0][i].max(p[1][i]).max(p[2][i])))));
    }
    for (_, (owner,n,pair)) in boundary {
        if n!=2{return Err("Junction boundary not stitched exactly twice".into())}
        let cosine=attributes::dot64(pair[0],pair[1]);
        let r=&mut receipt.records[owner];r.boundary_min_normal_dot=r.boundary_min_normal_dot.min(cosine);
        if cosine<r.recipe.min_boundary_normal_dot {return Err(format!("Junction {} boundary normal dot {cosine} below {}",r.recipe.id,r.recipe.min_boundary_normal_dot));}
    }
    for (i,&alias) in aliases.iter().enumerate(){if dense[alias as usize]!=u32::MAX {
        a.normals[i]=attributes::unit(normals[alias as usize])?.map(|x|x as f32);
    }}
    intersections(a,parts,&aliases,&boxes,receipt,budget)?;
    Ok(())
}

trait TrianglePoints {fn map_or_points(&self,a:&AnatomyMeshAccumulator)->Result<[[f32;3];3],String>;}
impl TrianglePoints for [u32] {
    fn map_or_points(&self,a:&AnatomyMeshAccumulator)->Result<[[f32;3];3],String>{
        let mut result=[[0.0;3];3];
        for i in 0..3 {result[i]=*a.positions.get(self[i]as usize).ok_or("Junction face index escapes positions")?;
            if result[i].iter().any(|v|!v.is_finite()){return Err("Junction input face not finite".into())}
        }
        Ok(result)
    }
}

fn intersections(a:&AnatomyMeshAccumulator,parts:&[Part],aliases:&[u32],boxes:&[Aabb],receipt:&JunctionReceipt,budget:&mut Budget)->Result<(),String>{
    for record in &receipt.records {
        let part=parts.iter().find(|p|p.source_id==format!("junction:{}",record.recipe.id)).ok_or("Junction output Part missing")?;
        let faces=part.indices.start/3..part.indices.end/3;
        let mut region=boxes[faces.start];for face in faces.clone(){region=region.union(&boxes[face]);}
        let mut candidates=Vec::new();
        budget.charge(product(boxes.len(),16)?)?;
        for (face,b) in boxes.iter().enumerate(){if region.overlaps(b){candidates.push(face);}}
        for face in faces.clone(){let t=&a.indices[face*3..face*3+3];
            for &other in &candidates {
                budget.charge(32)?;
                if face==other || (faces.contains(&other)&&other<face) || !boxes[face].overlaps(&boxes[other]){continue}
                let q=&a.indices[other*3..other*3+3];
                // Match the existing Core self-intersection owner's shared
                // vertex exclusion, using prescribed topology aliases.
                if t.iter().any(|x|q.iter().any(|y|aliases[*x as usize]==aliases[*y as usize])){continue}
                budget.charge(4096)?;
                let [p0,p1,p2]=t.map_or_points(a)?.map(point);
                let [q0,q1,q2]=q.map_or_points(a)?.map(point);
                if tri_tri_intersect_3(p0,p1,p2,q0,q1,q2).0 {
                    return Err(format!("Junction {} intersects nonadjacent retained/generated face {other} at collar face {face}",record.recipe.id));
                }
            }
        }
    }
    Ok(())
}
