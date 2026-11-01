use super::{attributes, Chain, TreeJunctionLimits};
use crate::scene_builder_10d::SubmeshBuilder;
use cg::boolean_3::bounded::*;
use cg::surface_topology_qualification::{
    qualify_surface_topology_budgeted_into, surface_qualification_storage_bounds,
};
use qualia_core_db::specialized_libs::computational_geometry::{self as cg, Point3};

fn filled<T: Clone>(n: usize, value: T, used: &mut usize, cap: usize) -> Result<Vec<T>, String> {
    let bytes = n
        .checked_mul(std::mem::size_of::<T>())
        .and_then(|n| used.checked_add(n))
        .ok_or("Tree union allocation overflow")?;
    if bytes > cap {
        return Err("Tree union scratch exceeds caller bytes".into());
    }
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| "Tree union allocation refused")?;
    *used = used
        .checked_add(
            v.capacity()
                .checked_mul(std::mem::size_of::<T>())
                .ok_or("Tree capacity overflow")?,
        )
        .ok_or("Tree capacity overflow")?;
    if *used > cap {
        return Err("Tree union actual capacities exceed caller bytes".into());
    }
    v.resize(n, value);
    Ok(v)
}
fn topology(c: &Chain) -> Result<(Vec<Point3>, Vec<[u32; 3]>), String> {
    let count = c
        .topology_vertices
        .iter()
        .copied()
        .max()
        .ok_or("Empty chain")? as usize
        + 1;
    if count > c.mesh.positions.len() {
        return Err("Sweep topology mapping outside vertices".into());
    }
    let mut positions = vec![Point3::new(0., 0., 0.); count];
    let mut seen = vec![false; count];
    for (i, &id) in c.topology_vertices.iter().enumerate() {
        let p = c.mesh.positions[i];
        let p = Point3::new(p[0] as f64, p[1] as f64, p[2] as f64);
        let j = id as usize;
        if seen[j] && [positions[j].x, positions[j].y, positions[j].z] != [p.x, p.y, p.z] {
            return Err("Sweep seam positions disagree with exact source mapping".into());
        }
        positions[j] = p;
        seen[j] = true;
    }
    if seen.contains(&false) {
        return Err("Sweep topology has missing vertices".into());
    }
    let triangles = c
        .mesh
        .indices
        .chunks_exact(3)
        .map(|t| {
            [
                c.topology_vertices[t[0] as usize],
                c.topology_vertices[t[1] as usize],
                c.topology_vertices[t[2] as usize],
            ]
        })
        .collect();
    Ok((positions, triangles))
}
fn qualify(
    n: usize,
    t: &[[u32; 3]],
    work: &mut usize,
    used: usize,
    cap: usize,
) -> Result<(), String> {
    let q = surface_qualification_storage_bounds(
        u32::try_from(n).map_err(|_| "Topology vertex overflow")?,
        t.len(),
    )
    .map_err(|e| format!("Tree topology bounds: {e:?}"))?;
    let mut bytes = used;
    let mut h = filled(q.half_edges, cg::HalfEdge::default(), &mut bytes, cap)?;
    let mut slots = filled(q.edge_slots, cg::EdgeSlot::default(), &mut bytes, cap)?;
    let mut starts = filled(q.vertex_starts, 0u32, &mut bytes, cap)?;
    let mut visited = filled(q.visited_half_edges, false, &mut bytes, cap)?;
    let mut labels = filled(q.face_labels, 0u32, &mut bytes, cap)?;
    let mut queue = filled(q.face_queue, 0u32, &mut bytes, cap)?;
    qualify_surface_topology_budgeted_into(
        n as u32,
        t,
        work,
        &mut h,
        &mut slots,
        &mut starts,
        &mut visited,
        &mut labels,
        &mut queue,
    )
    .map_err(|e| {
        format!("Tree union refused: selected topology is not one closed manifold: {e:?}")
    })?;
    Ok(())
}
pub(super) fn build(
    a: &Chain,
    b: &Chain,
    min_area: f64,
    limits: TreeJunctionLimits,
) -> Result<
    (
        SubmeshBuilder,
        Vec<attributes::VertexLineage>,
        serde_json::Value,
    ),
    String,
> {
    let na = a.mesh.indices.len() / 3;
    let nb = b.mesh.indices.len() / 3;
    if na.checked_add(nb).ok_or("Tree face count overflow")? > limits.input_faces
        || limits.faces == 0
        || limits.vertices == 0
        || limits.fragments_per_face == 0
    {
        return Err("Tree union input/output face capacity refused".into());
    }
    let logical = union_buffer_bytes(
        na,
        nb,
        limits.fragments_per_face,
        limits.vertices,
        limits.faces,
    )
    .map_err(|e| format!("Tree union bounds: {e:?}"))?;
    if logical > limits.bytes {
        return Err("Tree union Core buffers exceed caller bytes".into());
    }
    let (va, ta) = topology(a)?;
    let (vb, tb) = topology(b)?;
    let retained = (va.len() + vb.len())
        .checked_mul(24)
        .and_then(|n| {
            (ta.len() + tb.len())
                .checked_mul(12)
                .and_then(|t| n.checked_add(t))
        })
        .ok_or("Tree topology allocation overflow")?;
    let initialization = na
        .checked_add(nb)
        .and_then(|n| {
            limits
                .fragments_per_face
                .checked_mul(2)
                .and_then(|f| n.checked_add(f))
        })
        .and_then(|n| n.checked_add(limits.vertices))
        .and_then(|n| limits.faces.checked_mul(2).and_then(|f| n.checked_add(f)))
        .ok_or("Tree initialization work overflow")?;
    let mut work = limits
        .work
        .checked_sub(initialization)
        .ok_or("Tree initialization exceeds caller work")?;
    qualify(va.len(), &ta, &mut work, retained, limits.bytes)?;
    qualify(vb.len(), &tb, &mut work, retained, limits.bytes)?;
    let zero = Point3::new(0., 0., 0.);
    let mut used = retained;
    let mut ca = filled(na, [zero; 3], &mut used, limits.bytes)?;
    let mut cb = filled(nb, [zero; 3], &mut used, limits.bytes)?;
    let mut current = filled(
        limits.fragments_per_face,
        [zero; 3],
        &mut used,
        limits.bytes,
    )?;
    let mut next = filled(
        limits.fragments_per_face,
        [zero; 3],
        &mut used,
        limits.bytes,
    )?;
    let mut v = filled(limits.vertices, zero, &mut used, limits.bytes)?;
    let mut t = filled(limits.faces, [0; 3], &mut used, limits.bytes)?;
    let mut sources = filled(limits.faces, SourceFace::default(), &mut used, limits.bytes)?;
    let receipt = union_exact_bounded_into(
        &va,
        &ta,
        &vb,
        &tb,
        UnionBudget {
            bytes: limits
                .bytes
                .checked_sub(retained)
                .ok_or("Tree union byte exhaustion")?,
            work,
        },
        UnionBuffers {
            corners_a: &mut ca,
            corners_b: &mut cb,
            current: &mut current,
            next: &mut next,
            vertices: &mut v,
            triangles: &mut t,
            source_faces: &mut sources,
        },
        &|| false,
    )
    .map_err(|e| format!("Bounded Core chain union refused: {e:?}"))?;
    work = work
        .checked_sub(receipt.work)
        .ok_or("Union work accounting mismatch")?;
    v.truncate(receipt.vertices);
    t.truncate(receipt.triangles);
    sources.truncate(receipt.triangles);
    qualify(v.len(), &t, &mut work, used, limits.bytes)?;
    // Attribute projection is linear in output faces (source-face provenance
    // comes directly from Core, never nearest-organ or nearest-triangle search).
    let projection_work = t
        .len()
        .checked_mul(2048)
        .ok_or("Tree projection work overflow")?;
    work = work
        .checked_sub(projection_work)
        .ok_or("Tree attribute projection exceeds caller work")?;
    let (mesh, lineage) = attributes::project([a, b], &v, &t, &sources, min_area)?;
    attributes::strict_frames(&mesh)?;
    Ok((
        mesh,
        lineage,
        serde_json::json!({"vertices":v.len(),"faces":t.len(),"component_count":1,
        "core_logical_bytes":receipt.logical_bytes,"core_structural_work":receipt.work,"total_work_consumed":limits.work-work,
        "remaining_work":work,"cold_bytes_limit":limits.bytes,"output_order":"source mesh A faces then B faces; exact provenance retained"}),
    ))
}
