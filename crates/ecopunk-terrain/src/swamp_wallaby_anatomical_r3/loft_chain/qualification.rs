use super::*;
use qualia_core_db::specialized_libs::computational_geometry::{surface_topology_qualification as topology,EdgeSlot,HalfEdge};

pub(super) fn reserve(rows:usize,radial:usize)->Result<(),String> {
    let vertices=rows.checked_mul(radial).and_then(|n|n.checked_add(2)).ok_or("Chain topology overflow")?;
    let faces=rows.checked_mul(radial).and_then(|n|n.checked_mul(2)).ok_or("Chain faces overflow")?;
    let b=topology::surface_qualification_storage_bounds(u32::try_from(vertices).map_err(|_|"Chain vertex overflow")?,faces)
        .map_err(|e|format!("Chain topology reservation: {e:?}"))?;
    // Include geometric growth of all retained parallel streams, temporary
    // rings/frames/weights, triangles, output metadata and topology scratch.
    let bytes=vertices.checked_mul(512).and_then(|n|faces.checked_mul(96).and_then(|f|n.checked_add(f)))
        .and_then(|n|b.half_edges.checked_mul(std::mem::size_of::<HalfEdge>()).and_then(|v|n.checked_add(v)))
        .and_then(|n|b.edge_slots.checked_mul(std::mem::size_of::<EdgeSlot>()).and_then(|v|n.checked_add(v)))
        .and_then(|n|n.checked_add(1024*1024)).ok_or("Chain cold reservation overflow")?;
    if bytes>MAX_COLD_BYTES {return Err("Chain exceeds 64 MiB logical cold reservation".into());}
    Ok(())
}

pub(super) fn check(a:&AnatomyMeshAccumulator,base:usize,start:usize,rows:usize,radial:usize,work:&mut usize)->Result<(),String> {
    reserve(rows,radial)?;
    let stride=radial+1;let ring_vertices=rows*stride;
    let count=rows*radial+2;
    let alias=|index:u32|->u32 {
        let local=index as usize-base;
        if local<ring_vertices {((local/stride)*radial+(local%stride)%radial)as u32}
        else {(rows*radial+local-ring_vertices)as u32}
    };
    let mut triangles=Vec::with_capacity(a.indices[start..].len()/3);
    for tri in a.indices[start..].chunks_exact(3) {
        let p=a.positions[tri[0]as usize];let q=a.positions[tri[1]as usize];let r=a.positions[tri[2]as usize];
        let n=cross(sub(q,p),sub(r,p));
        if !finite(&n)||dot(n,n)<=1e-20 {return Err("Chain contains a collapsed sampled face".into());}
        triangles.push([alias(tri[0]),alias(tri[1]),alias(tri[2])]);
    }
    let b=topology::surface_qualification_storage_bounds(count as u32,triangles.len()).map_err(|e|format!("Chain topology bounds: {e:?}"))?;
    topology::qualify_surface_topology_budgeted_into(count as u32,&triangles,work,
        &mut vec![HalfEdge::default();b.half_edges],&mut vec![EdgeSlot::default();b.edge_slots],
        &mut vec![0;b.vertex_starts],&mut vec![false;b.visited_half_edges],
        &mut vec![0;b.face_labels],&mut vec![0;b.face_queue])
        .map_err(|e|format!("Continuous limb topology refused: {e:?}"))?;
    Ok(())
}
