use super::*;
use qualia_core_db::specialized_libs::computational_geometry::{
    self as cg, smooth_junction as kernel, Point3,
};
use std::collections::{BTreeMap, BTreeSet};
fn add(a: usize, b: usize) -> Result<usize, String> {
    a.checked_add(b)
        .ok_or_else(|| "Collar size overflow".into())
}
fn mul(a: usize, b: usize) -> Result<usize, String> {
    a.checked_mul(b)
        .ok_or_else(|| "Collar size overflow".into())
}
fn port_maps(
    input: &CollarInput<'_>,
    seed: &seed::Seed,
    limits: CollarLimits,
    work: &mut usize,
) -> Result<(Vec<PortMapping>, BTreeSet<(u32, u32)>), String> {
    seed::charge(work, mul(seed.positions.len(), 32)?)?;
    let mut lookup = BTreeMap::new();
    for (i, &p) in seed.positions.iter().enumerate() {
        if lookup.insert(seed::bits(p), i as u32).is_some() {
            return Err("Ambiguous exact seed boundary position".into());
        }
    }
    let mut ports = Vec::new();
    let mut removed = BTreeSet::new();
    let mut ring_ids = BTreeSet::new();
    for port in input.ports {
        let piece = input.pieces.get(port.piece).ok_or("Port piece absent")?;
        let n = port.cut_vertices.len();
        if n < 3
            || n > limits.ring_vertices
            || port.guard_vertices.len() != n
            || port.cut_cap_faces.len() != n
            || port.edge_child_key != piece.edge_child_key
            || !input.joint_ids.contains(&port.joint_key)
            || !port.cut_fraction.is_finite()
            || port.cut_fraction <= 0.
            || port.cut_fraction >= 1.
            || !port.transition_length.is_finite()
            || port.transition_length <= 0.
            || port
                .outward_axis
                .iter()
                .chain(&port.guard_center)
                .any(|x| !x.is_finite())
            || (port.outward_axis.iter().map(|x| x * x).sum::<f64>() - 1.).abs() > 1e-6
        {
            return Err("Invalid declared collar port".into());
        }
        seed::charge(work, mul(n, 256)?)?;
        let mut rings = [Vec::new(), Vec::new()];
        for (which, ids) in [&port.cut_vertices, &port.guard_vertices]
            .into_iter()
            .enumerate()
        {
            let mut local = BTreeSet::new();
            for &id in ids {
                let j = id as usize;
                let p = piece.mesh.positions.get(j).ok_or("Port vertex absent")?;
                if piece.vertex_sources[j].original_wood_vertices.is_none()
                    || piece.fade_coordinates[j] > 0.
                {
                    return Err("Port is not an exact source-bound fixed-band vertex".into());
                }
                if which == 1 && piece.fade_coordinates[j] != 0. {
                    return Err("Guard coordinates must be exactly zero".into());
                }
                let mapped = *lookup
                    .get(&seed::bits(seed::point(*p)))
                    .ok_or("Seed union changed a prescribed ring vertex")?;
                if !local.insert(mapped) || !ring_ids.insert((which, mapped)) {
                    return Err("Overlapping/duplicate prescribed port rings; define a larger connected cluster".into());
                }
                rings[which].push(mapped);
            }
        }
        let mut cap_center = None;
        let cut_aliases = port
            .cut_vertices
            .iter()
            .map(|&v| piece.topology_aliases[v as usize])
            .collect::<Vec<_>>();
        let mut cap_edges = BTreeSet::new();
        for &face in &port.cut_cap_faces {
            if !removed.insert((port.piece as u32, face)) {
                return Err("Repeated cut-cap face".into());
            }
            let start = mul(face as usize, 3)?;
            let ids = piece
                .mesh
                .indices
                .get(start..add(start, 3)?)
                .ok_or("Cut cap face absent")?;
            let aliases = ids
                .iter()
                .map(|&i| piece.topology_aliases[i as usize])
                .collect::<Vec<_>>();
            let off = aliases
                .iter()
                .enumerate()
                .filter(|(_, id)| !cut_aliases.contains(id))
                .collect::<Vec<_>>();
            if off.len() != 1 {
                return Err(
                    "Cut cap must be a nondegenerate centroid fan over the exact ring".into(),
                );
            }
            let center = *off[0].1;
            if cap_center.is_some_and(|old| old != center) {
                return Err("Cut-cap fan centers differ".into());
            }
            cap_center = Some(center);
            let edge = aliases
                .iter()
                .copied()
                .filter(|id| *id != center)
                .collect::<Vec<_>>();
            cap_edges.insert((edge[0].min(edge[1]), edge[0].max(edge[1])));
            seed::charge(work, seed.triangles.len())?;
            let mut found = 0;
            for (t, s) in seed.triangles.iter().zip(&seed.sources) {
                if s.piece == port.piece as u32 && s.face == face {
                    seed::charge(work, 1)?;
                    let actual = t.map(|i| seed::bits(seed.positions[i as usize]));
                    let expected = [ids[0], ids[1], ids[2]]
                        .map(|i| seed::bits(seed::point(piece.mesh.positions[i as usize])));
                    if !(0..3).any(|shift| (0..3).all(|i| actual[i] == expected[(i + shift) % 3])) {
                        return Err("Union cut or moved an external cap; enlarge/repartition explicit cluster".into());
                    }
                    found += 1;
                }
            }
            if found != 1 {
                return Err("External cut cap is missing or split by another arm".into());
            }
        }
        for i in 0..n {
            let (a, b) = (cut_aliases[i], cut_aliases[(i + 1) % n]);
            if !cap_edges.remove(&(a.min(b), a.max(b))) {
                return Err("Cut cap does not match declared cyclic ring".into());
            }
        }
        if !cap_edges.is_empty() {
            return Err("Extra cap boundary edges".into());
        }
        ports.push(PortMapping {
            cut_topology: std::mem::take(&mut rings[0]),
            guard_topology: std::mem::take(&mut rings[1]),
        });
    }
    Ok((ports, removed))
}
/// All outputs are detached. Every cluster is explicitly authored; no nearest
/// branch discovery, no implicit fallback and no per-fork hidden budget reset.
pub fn build(
    input: CollarInput<'_>,
    controls: CollarControls,
    limits: CollarLimits,
    cancelled: &dyn Fn() -> bool,
) -> Result<CollarMesh, String> {
    validate_controls(controls)?;
    if input.pieces.len() < 2
        || input.pieces.len() > limits.pieces
        || limits.pieces > 256
        || input.ports.is_empty()
        || input.ports.len() > limits.ports
        || limits.ports > 128
        || limits.ring_vertices > 64
        || limits.fragments_per_face == 0
        || limits.fragments_per_face > 65536
        || input.field_segments.len() < 2
        || input.field_segments.len() > 256
        || input.joint_ids.is_empty()
        || input.joint_ids.len() > 256
    {
        return Err("Smooth collar inventory limits refused".into());
    }
    if cancelled() {
        return Err("Smooth collar cancelled".into());
    }
    let count = input
        .pieces
        .iter()
        .try_fold(0usize, |n, p| add(n, p.mesh.indices.len() / 3))?;
    if count > limits.input_faces {
        return Err("Smooth collar input face cap exceeded".into());
    }
    let kl = kernel::Limits {
        vertices: limits.vertices,
        faces: limits.faces,
        bytes: limits.bytes,
        work: limits.work,
    };
    let core_bytes =
        kernel::storage_bytes(kl).map_err(|e| format!("Smooth kernel bounds: {e:?}"))?;
    let union_bytes = cg::boolean_3::conforming::union_buffer_bytes(
        limits.faces,
        limits.input_faces,
        limits.fragments_per_face,
        limits.vertices,
        limits.faces,
    )
    .map_err(|e| format!("Smooth seed bounds: {e:?}"))?;
    let base = add(
        add(core_bytes, union_bytes)?,
        add(mul(limits.vertices, 256)?, mul(limits.faces, 128)?)?,
    )?;
    if base > limits.bytes {
        return Err("Smooth collar fixed scratch exceeds remaining caller bytes".into());
    }
    let mut work = limits.work;
    let seed = seed::build(&input, limits, &mut work, cancelled)?;
    let (mut ports, removed) = port_maps(&input, &seed, limits, &mut work)?;
    let fade = attributes::fade(&seed, &input, &mut work)?;
    let zero = Point3::new(0., 0., 0.);
    let mut positions = seed::allocate(limits.vertices, zero)?;
    let mut references = seed::allocate(limits.vertices, zero)?;
    let mut normals = seed::allocate(limits.vertices, zero)?;
    let mut weights = seed::allocate(limits.vertices, 0.)?;
    let mut triangles = seed::allocate(limits.faces, [0; 3])?;
    let mut sources = seed::allocate(limits.faces, kernel::FaceSource::default())?;
    let receipt = kernel::smooth_into(
        kernel::Input {
            positions: &seed.positions,
            triangles: &seed.triangles,
            sources: &seed.sources,
            segments: input.field_segments,
            fade_coordinates: &fade,
        },
        core_controls(controls),
        kernel::Limits {
            bytes: core_bytes,
            work,
            ..kl
        },
        kernel::Output {
            positions: &mut positions,
            references: &mut references,
            triangles: &mut triangles,
            sources: &mut sources,
            normals: &mut normals,
            weights: &mut weights,
        },
        cancelled,
    )
    .map_err(|e| format!("Geometric smooth collar refused: {e:?}"))?;
    seed::charge(&mut work, receipt.work_used)?;
    positions.truncate(receipt.vertices);
    references.truncate(receipt.vertices);
    normals.truncate(receipt.vertices);
    weights.truncate(receipt.vertices);
    triangles.truncate(receipt.faces);
    sources.truncate(receipt.faces);
    // Allocation admission uses actual produced face count, never the source
    // text length. Includes Value array lineage/binding tables and all mesh,
    // strict frame and compact-topology copies, including geometric Vec growth.
    let projection = mul(receipt.faces, 8192)?;
    if add(base, projection)? > limits.bytes {
        return Err(
            "Smooth collar produced attribute/lineage envelope exceeds remaining bytes".into(),
        );
    }
    let mut map = seed::allocate(positions.len(), u32::MAX)?;
    let mut topology_positions = Vec::new();
    let mut topology_triangles = Vec::new();
    seed::charge(&mut work, mul(receipt.faces, 128)?)?;
    for (t, s) in triangles.iter().zip(&sources) {
        if removed.contains(&(s.piece, s.face)) {
            continue;
        }
        let mut ids = [0; 3];
        for (k, &i) in t.iter().enumerate() {
            let j = i as usize;
            if map[j] == u32::MAX {
                map[j] = topology_positions.len() as u32;
                let p = positions[j];
                topology_positions.push([p.x, p.y, p.z]);
            }
            ids[k] = map[j];
        }
        topology_triangles.push(ids);
    }
    for port in &mut ports {
        for id in port.cut_topology.iter_mut().chain(&mut port.guard_topology) {
            let old = *id as usize;
            if positions[old] != seed.positions[old] || weights[old] != 0. || map[old] == u32::MAX {
                return Err("Smooth collar changed/lost an exact prescribed boundary".into());
            }
            *id = map[old];
        }
    }
    // Closed qualified seed minus unchanged disjoint disk caps: enumerate every
    // open edge and require exactly the caller's cut-ring edges, no extra holes.
    let mut edge_counts = BTreeMap::<(u32, u32), (usize, i32)>::new();
    for t in &topology_triangles {
        for i in 0..3 {
            let (a, b) = (t[i], t[(i + 1) % 3]);
            let value = edge_counts.entry((a.min(b), a.max(b))).or_default();
            value.0 += 1;
            value.1 += if a < b { 1 } else { -1 };
        }
    }
    let mut expected = BTreeSet::new();
    for p in &ports {
        for i in 0..p.cut_topology.len() {
            let (a, b) = (
                p.cut_topology[i],
                p.cut_topology[(i + 1) % p.cut_topology.len()],
            );
            if !expected.insert((a.min(b), a.max(b))) {
                return Err("Collar cut rings share a boundary edge".into());
            }
        }
    }
    for (edge, (count, orientation)) in edge_counts {
        match count {
            1 => {
                if !expected.remove(&edge) {
                    return Err("Undeclared collar hole".into());
                }
            }
            2 => {
                if orientation != 0 {
                    return Err("Collar winding disagreement".into());
                }
            }
            _ => return Err("Nonmanifold collar edge".into()),
        }
    }
    if !expected.is_empty() {
        return Err("Collar ring boundary incomplete".into());
    }
    let projected = attributes::project(
        &input,
        &positions,
        &references,
        &normals,
        &weights,
        &triangles,
        &sources,
        &map,
        &removed,
        controls.minimum_triangle_area_sq_m4,
        &mut work,
    )?;
    let mut retained = add(
        mul(projected.mesh.positions.capacity(), 12)?,
        mul(projected.mesh.normals.capacity(), 12)?,
    )?;
    for bytes in [
        mul(projected.mesh.uvs.capacity(), 8)?,
        mul(projected.mesh.colors.capacity(), 16)?,
        mul(projected.mesh.indices.capacity(), 4)?,
        mul(projected.aliases.capacity(), 4)?,
        mul(topology_positions.capacity(), 24)?,
        mul(topology_triangles.capacity(), 12)?,
        mul(
            projected.lineage.capacity() + projected.bindings.capacity(),
            std::mem::size_of::<serde_json::Value>(),
        )?,
    ] {
        retained = add(retained, bytes)?
    }
    for value in projected.lineage.iter().chain(&projected.bindings) {
        retained = add(retained, attributes::value_bytes(value)?)?
    }
    for p in &ports {
        retained = add(
            retained,
            mul(p.cut_topology.capacity() + p.guard_topology.capacity(), 4)?,
        )?
    }
    let record = serde_json::json!({"schema":"ecopunk.smooth-collar-receipt/v1","joint_ids":input.joint_ids,"pieces":input.pieces.len(),"ports":ports.len(),
        "positive_blend_m":controls.blend_m,"refinement_rounds":controls.refinement_rounds,"moved_vertices":receipt.moved_vertices,
        "max_full_support_field_residual_m":receipt.max_field_residual,"max_full_support_dihedral_radians":receipt.max_dihedral,
        "min_full_support_normal_alignment":receipt.min_normal_alignment,"controls":controls,"fixed_ring_positions_exact":true,
        "face_bindings_schema":"[piece,local_face,local_vertex_ids,[[original_wood_ids,source_weights];3]]",
        "vertex_lineage_schema":"[face_binding_index,face_barycentric,geometric_displacement_m]",
        "surface":"geometrically projected positive smooth-union field inside explicit incident/connected cluster; cubic displacement fade with exact source fixed band; finite mesh, not analytic C1 claim",
        "charts":"source face/branch charts retain UV and physical normal correspondence; chart seams explicit; tangent generation strictly admitted",
        "acceptance":"unreviewed candidate; no executed or biological acceptance"});
    retained = add(retained, attributes::value_bytes(&record)?)?;
    if add(base, retained)? > limits.bytes {
        return Err("Smooth collar actual retained capacities exceed reservation".into());
    }
    if cancelled() {
        return Err("Smooth collar cancelled before delivery".into());
    }
    Ok(CollarMesh {
        mesh: projected.mesh,
        topology_positions,
        topology_triangles,
        render_topology_vertices: projected.aliases,
        ports,
        vertex_lineage: projected.lineage,
        face_bindings: projected.bindings,
        receipt: record,
        work_used: limits.work - work,
        retained_bytes: retained,
    })
}
