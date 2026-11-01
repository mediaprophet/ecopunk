//! Complete structural assembly qualification through the existing Core owner.
use crate::scene_builder_10d::SubmeshBuilder;
use qualia_core_db::specialized_libs::computational_geometry::{
    self as cg,
    surface_topology_qualification::{
        qualify_surface_topology_budgeted_into, surface_qualification_storage_bounds,
    },
};
use std::collections::BTreeMap;

fn filled<T: Clone>(n: usize, value: T, used: &mut usize, limit: usize) -> Result<Vec<T>, String> {
    let requested = n
        .checked_mul(std::mem::size_of::<T>())
        .and_then(|n| used.checked_add(n))
        .ok_or("All-fork topology allocation overflow")?;
    if requested > limit {
        return Err("All-fork topology scratch exceeds remaining aggregate bytes".into());
    }
    let mut out = Vec::new();
    out.try_reserve_exact(n)
        .map_err(|_| "All-fork topology allocation refused")?;
    *used = used
        .checked_add(
            out.capacity()
                .checked_mul(std::mem::size_of::<T>())
                .ok_or("Topology capacity overflow")?,
        )
        .ok_or("Topology capacity overflow")?;
    if *used > limit {
        return Err("All-fork topology actual capacities exceed aggregate bytes".into());
    }
    out.resize(n, value);
    Ok(out)
}

pub(super) fn qualify(
    mesh: &SubmeshBuilder,
    aliases: &[u32],
    indices: &[u32],
    bytes: usize,
    max_work: usize,
    cancelled: &dyn Fn() -> bool,
) -> Result<(serde_json::Value, usize), String> {
    if aliases.len() != mesh.positions.len() || indices.is_empty() || indices.len() % 3 != 0 {
        return Err("Incomplete all-fork structural topology inventory".into());
    }
    // Densification is by explicit source/collar alias only. Equal positions
    // never authorize a weld. This envelope covers maps and output arrays.
    let map_bytes = indices
        .len()
        .checked_mul(64)
        .ok_or("Topology map storage overflow")?;
    if map_bytes > bytes {
        return Err("All-fork topology map exceeds remaining aggregate bytes".into());
    }
    let map_work = indices
        .len()
        .checked_mul(64)
        .ok_or("Topology map work overflow")?;
    let mut remaining = max_work
        .checked_sub(map_work)
        .ok_or("All-fork aggregate topology work exhausted")?;
    let initial_remaining = remaining;
    let mut dense = BTreeMap::<u32, (u32, [f32; 3])>::new();
    let mut triangles = Vec::with_capacity(indices.len() / 3);
    for face in indices.chunks_exact(3) {
        if cancelled() {
            return Err("All-fork complete structural qualification cancelled".into());
        }
        let mut t = [0u32; 3];
        for corner in 0..3 {
            let vertex = face[corner] as usize;
            let alias = *aliases
                .get(vertex)
                .ok_or("Structural index escapes alias inventory")?;
            let point = *mesh
                .positions
                .get(vertex)
                .ok_or("Structural index escapes mesh")?;
            if point.iter().any(|v| !v.is_finite()) {
                return Err("Nonfinite all-fork structural position".into());
            }
            let next =
                u32::try_from(dense.len()).map_err(|_| "All-fork dense topology exceeds u32")?;
            let (id, old) = *dense.entry(alias).or_insert((next, point));
            if old != point {
                return Err(
                    "Source/collar attribute seams have separated geometric positions".into(),
                );
            }
            t[corner] = id;
        }
        triangles.push(t);
    }
    let vertex_count =
        u32::try_from(dense.len()).map_err(|_| "All-fork structural vertex count overflow")?;
    let bounds = surface_qualification_storage_bounds(vertex_count, triangles.len())
        .map_err(|e| format!("All-fork Core topology bounds: {e:?}"))?;
    let mut used = map_bytes;
    let mut edges = filled(bounds.half_edges, cg::HalfEdge::default(), &mut used, bytes)?;
    let mut slots = filled(bounds.edge_slots, cg::EdgeSlot::default(), &mut used, bytes)?;
    let mut starts = filled(bounds.vertex_starts, 0u32, &mut used, bytes)?;
    let mut visited = filled(bounds.visited_half_edges, false, &mut used, bytes)?;
    let mut labels = filled(bounds.face_labels, 0u32, &mut used, bytes)?;
    let mut queue = filled(bounds.face_queue, 0u32, &mut used, bytes)?;
    let receipt = qualify_surface_topology_budgeted_into(
        vertex_count,
        &triangles,
        &mut remaining,
        &mut edges,
        &mut slots,
        &mut starts,
        &mut visited,
        &mut labels,
        &mut queue,
    )
    .map_err(|e| {
        format!("Complete source-enumerated structural tree is not one closed manifold: {e:?}")
    })?;
    if cancelled() {
        return Err("All-fork complete structural qualification cancelled".into());
    }
    let work = map_work
        .checked_add(initial_remaining - remaining)
        .ok_or("All-fork topology work accounting overflow")?;
    Ok((
        serde_json::json!({"vertices":receipt.vertex_count,"faces":receipt.face_count,
        "components":receipt.component_count,"half_edges":receipt.half_edge_count,
        "scope":"all adopted source-chain structural faces; independently retained sprays excluded",
        "logical_scratch_bytes":used,"structural_work":work,
        "not_proven":["geometric nonintersection","botanical correctness","visual acceptance"]}),
        work,
    ))
}
