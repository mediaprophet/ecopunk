use super::*;
use qualia_core_db::specialized_libs::computational_geometry::{
    self as cg, boolean_3::bounded::*, smooth_junction::FaceSource, Point3,
};
pub(super) struct Seed {
    pub positions: Vec<Point3>,
    pub triangles: Vec<[u32; 3]>,
    pub sources: Vec<FaceSource>,
}
pub(super) fn point(p: [f32; 3]) -> Point3 {
    Point3::new(p[0] as f64, p[1] as f64, p[2] as f64)
}
pub(super) fn bits(p: Point3) -> [u64; 3] {
    [p.x, p.y, p.z].map(|x| if x == 0. { 0 } else { x.to_bits() })
}
pub(super) fn charge(work: &mut usize, n: usize) -> Result<(), String> {
    *work = work
        .checked_sub(n)
        .ok_or("Smooth collar structural work exhausted")?;
    Ok(())
}
pub(super) fn allocate<T: Clone>(n: usize, value: T) -> Result<Vec<T>, String> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| "Smooth collar allocation refused")?;
    if v.capacity() > n {
        return Err("Smooth collar allocator capacity exceeds reserved count".into());
    }
    v.resize(n, value);
    Ok(v)
}
fn qualify(s: &Seed, work: &mut usize) -> Result<(), String> {
    use cg::surface_topology_qualification::*;
    let q = surface_qualification_storage_bounds(s.positions.len() as u32, s.triangles.len())
        .map_err(|e| format!("Collar topology bounds: {e:?}"))?;
    charge(
        work,
        q.half_edges
            .checked_add(q.edge_slots)
            .and_then(|n| n.checked_add(q.vertex_starts))
            .and_then(|n| q.face_labels.checked_mul(2).and_then(|v| n.checked_add(v)))
            .ok_or("Collar topology init overflow")?,
    )?;
    let mut edges = allocate(q.half_edges, cg::HalfEdge::default())?;
    let mut slots = allocate(q.edge_slots, cg::EdgeSlot::default())?;
    let mut starts = allocate(q.vertex_starts, 0u32)?;
    let mut visited = allocate(q.visited_half_edges, false)?;
    let mut labels = allocate(q.face_labels, 0u32)?;
    let mut queue = allocate(q.face_queue, 0u32)?;
    qualify_surface_topology_budgeted_into(
        s.positions.len() as u32,
        &s.triangles,
        work,
        &mut edges,
        &mut slots,
        &mut starts,
        &mut visited,
        &mut labels,
        &mut queue,
    )
    .map_err(|e| format!("Closed collar seed refused: {e:?}"))?;
    Ok(())
}
fn piece(
    p: &CollarPiece,
    id: usize,
    limits: CollarLimits,
    work: &mut usize,
) -> Result<Seed, String> {
    let n = p.mesh.positions.len();
    let f = p.mesh.indices.len() / 3;
    if n < 4
        || n > limits.vertices
        || f < 4
        || f > limits.input_faces
        || p.mesh.indices.len() % 3 != 0
        || p.topology_aliases.len() != n
        || p.vertex_sources.len() != n
        || p.fade_coordinates.len() != n
        || p.mesh.normals.len() != n
        || p.mesh.colors.len() != n
        || p.mesh.uvs.len() != n
    {
        return Err("Invalid collar piece stream inventory".into());
    }
    charge(
        work,
        n.checked_mul(32)
            .and_then(|n| f.checked_mul(8).and_then(|f| n.checked_add(f)))
            .ok_or("Collar input work overflow")?,
    )?;
    if p.mesh
        .positions
        .iter()
        .flatten()
        .chain(p.mesh.normals.iter().flatten())
        .chain(p.mesh.colors.iter().flatten())
        .chain(p.mesh.uvs.iter().flatten())
        .any(|v| !v.is_finite())
        || p.mesh.indices.iter().any(|&i| i as usize >= n)
        || p.fade_coordinates
            .iter()
            .any(|d| !d.is_finite() || d.abs() > 1e6)
    {
        return Err("Invalid collar piece values".into());
    }
    for v in &p.vertex_sources {
        if v.barycentric.iter().any(|x| !x.is_finite())
            || match v.original_wood_vertices {
                Some(_) => {
                    v.barycentric.iter().any(|x| *x < -1e-9 || *x > 1. + 1e-9)
                        || (v.barycentric.iter().sum::<f64>() - 1.).abs() > 1e-8
                }
                None => v.barycentric != [0.; 3],
            }
        {
            return Err("Invalid exact/construction collar source binding".into());
        }
    }
    let maximum = p
        .topology_aliases
        .iter()
        .copied()
        .max()
        .ok_or("Empty aliases")? as usize;
    let count = maximum
        .checked_add(1)
        .ok_or("Collar alias count overflow")?;
    if count > n {
        return Err("Collar topology aliases are not dense".into());
    }
    let zero = Point3::new(0., 0., 0.);
    let mut positions = allocate(count, zero)?;
    let mut seen = allocate(count, false)?;
    for (i, &id) in p.topology_aliases.iter().enumerate() {
        let q = point(p.mesh.positions[i]);
        if seen[id as usize] && positions[id as usize] != q {
            return Err("Authored collar alias positions differ".into());
        }
        seen[id as usize] = true;
        positions[id as usize] = q;
    }
    if seen.contains(&false) {
        return Err("Unused collar topology alias".into());
    }
    let triangles = p
        .mesh
        .indices
        .chunks_exact(3)
        .map(|t| {
            [
                p.topology_aliases[t[0] as usize],
                p.topology_aliases[t[1] as usize],
                p.topology_aliases[t[2] as usize],
            ]
        })
        .collect();
    let s = Seed {
        positions,
        triangles,
        sources: allocate(
            f,
            FaceSource {
                piece: id as u32,
                face: 0,
            },
        )?
        .into_iter()
        .enumerate()
        .map(|(face, mut s)| {
            s.face = face as u32;
            s
        })
        .collect(),
    };
    qualify(&s, work)
        .map_err(|e| format!("Input collar piece {id} ({}): {e}", p.edge_child_key))?;
    Ok(s)
}
pub(super) fn build(
    input: &CollarInput<'_>,
    limits: CollarLimits,
    work: &mut usize,
    cancelled: &dyn Fn() -> bool,
) -> Result<Seed, String> {
    let mut acc = piece(&input.pieces[0], 0, limits, work)?;
    let zero = Point3::new(0., 0., 0.);
    for (id, p) in input.pieces.iter().enumerate().skip(1) {
        if cancelled() {
            return Err("Smooth collar cancelled before seed union".into());
        }
        let right = piece(p, id, limits, work)?;
        charge(
            work,
            limits
                .vertices
                .checked_add(limits.faces.checked_mul(2).ok_or("Union init overflow")?)
                .and_then(|n| {
                    limits
                        .fragments_per_face
                        .checked_mul(2)
                        .and_then(|v| n.checked_add(v))
                })
                .and_then(|n| {
                    acc.triangles
                        .len()
                        .checked_add(right.triangles.len())
                        .and_then(|v| n.checked_add(v))
                })
                .ok_or("Union init overflow")?,
        )?;
        let mut ca = allocate(acc.triangles.len(), [zero; 3])?;
        let mut cb = allocate(right.triangles.len(), [zero; 3])?;
        let mut current = allocate(limits.fragments_per_face, [zero; 3])?;
        let mut next = allocate(limits.fragments_per_face, [zero; 3])?;
        let mut vertices = allocate(limits.vertices, zero)?;
        let mut triangles = allocate(limits.faces, [0; 3])?;
        let mut origins = allocate(limits.faces, SourceFace::default())?;
        let receipt = cg::boolean_3::conforming::union_conforming_bounded_into(
            &acc.positions,
            &acc.triangles,
            &right.positions,
            &right.triangles,
            UnionBudget {
                bytes: limits.bytes,
                work: *work,
            },
            UnionBuffers {
                corners_a: &mut ca,
                corners_b: &mut cb,
                current: &mut current,
                next: &mut next,
                vertices: &mut vertices,
                triangles: &mut triangles,
                source_faces: &mut origins,
            },
            cancelled,
        )
        .map_err(|e| {
            format!(
                "Bounded incident seed union {id} ({}) refused: {e:?}",
                p.edge_child_key
            )
        })?;
        charge(work, receipt.work)?;
        vertices.truncate(receipt.vertices);
        triangles.truncate(receipt.triangles);
        origins.truncate(receipt.triangles);
        let sources = origins
            .iter()
            .map(|s| {
                if s.mesh == 0 {
                    acc.sources.get(s.face as usize).copied()
                } else {
                    right.sources.get(s.face as usize).copied()
                }
            })
            .collect::<Option<Vec<_>>>()
            .ok_or("Seed union source-face correspondence lost")?;
        acc = Seed {
            positions: vertices,
            triangles,
            sources,
        };
        qualify(&acc, work).map_err(|e| {
            format!(
                "After incident union {id} ({}), {} vertices/{} faces: {e}",
                p.edge_child_key,
                acc.positions.len(),
                acc.triangles.len()
            )
        })?;
    }
    Ok(acc)
}
