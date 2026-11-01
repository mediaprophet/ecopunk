use super::Chain;
use crate::scene_builder_10d::SubmeshBuilder;
use qualia_core_db::specialized_libs::computational_geometry::{
    boolean_3::bounded::SourceFace, Point3,
};
#[derive(serde::Serialize)]
pub(super) struct VertexLineage {
    topology_vertex: u32,
    input_chain: u8,
    input_face: u32,
    original_wood_vertices: [u32; 3],
    barycentric: [f64; 3],
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}
pub(super) fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
pub(super) fn face(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> [f64; 3] {
    cross(sub(b, a), sub(c, a))
}
pub(super) fn check(current: [f64; 3], reference: [f64; 3], min: f64) -> Result<(), String> {
    let area = dot(current, current);
    let facing = dot(current, reference);
    if !area.is_finite() || area <= min || !facing.is_finite() || facing <= 0.0 {
        return Err("Tree union projected face collapsed or reversed".into());
    }
    Ok(())
}
fn barycentric(p: [f64; 3], t: [[f64; 3]; 3]) -> Result<[f64; 3], String> {
    let u = sub(t[1], t[0]);
    let v = sub(t[2], t[0]);
    let q = sub(p, t[0]);
    let uu = dot(u, u);
    let uv = dot(u, v);
    let vv = dot(v, v);
    let d = uu * vv - uv * uv;
    if !d.is_finite() || d <= 0.0 {
        return Err("Degenerate source attribute face".into());
    }
    let b = (dot(q, u) * vv - dot(q, v) * uv) / d;
    let c = (dot(q, v) * uu - dot(q, u) * uv) / d;
    let w = [1. - b - c, b, c];
    // Assembly tolerance is 1e-9 m; refuse source-face drift rather than
    // clamping/reassigning to a nearby face. These are numerical witnesses,
    // not exact rational weights or a new biological identity.
    let reconstructed = std::array::from_fn(|i| w[0] * t[0][i] + w[1] * t[1][i] + w[2] * t[2][i]);
    if w.iter()
        .any(|x| !x.is_finite() || *x < -1e-7 || *x > 1. + 1e-7)
        || dot(sub(reconstructed, p), sub(reconstructed, p)) > 4e-18
    {
        return Err("Core union fragment no longer lies on its exact source face; attribute transfer refused".into());
    }
    Ok(w)
}
pub(super) fn project(
    chains: [&Chain; 2],
    vertices: &[Point3],
    triangles: &[[u32; 3]],
    sources: &[SourceFace],
    min: f64,
) -> Result<(SubmeshBuilder, Vec<VertexLineage>), String> {
    let mut mesh = SubmeshBuilder::new();
    let count = triangles
        .len()
        .checked_mul(3)
        .ok_or("Tree attributed vertex count overflow")?;
    mesh.positions
        .try_reserve_exact(count)
        .map_err(|_| "Tree position allocation refused")?;
    mesh.normals
        .try_reserve_exact(count)
        .map_err(|_| "Tree normal allocation refused")?;
    mesh.colors
        .try_reserve_exact(count)
        .map_err(|_| "Tree color allocation refused")?;
    mesh.uvs
        .try_reserve_exact(count)
        .map_err(|_| "Tree UV allocation refused")?;
    mesh.indices
        .try_reserve_exact(count)
        .map_err(|_| "Tree index allocation refused")?;
    let mut lineage = Vec::new();
    lineage
        .try_reserve_exact(count)
        .map_err(|_| "Tree lineage allocation refused")?;
    // No geometric growth while emitting faces. Validate real reported Vec
    // capacities against the named 512-byte-per-corner typed projection slice
    // of the caller's larger 8192-byte-per-face logical envelope.
    let actual = mesh
        .positions
        .capacity()
        .checked_mul(12)
        .and_then(|n| {
            mesh.normals
                .capacity()
                .checked_mul(12)
                .and_then(|x| n.checked_add(x))
        })
        .and_then(|n| {
            mesh.colors
                .capacity()
                .checked_mul(16)
                .and_then(|x| n.checked_add(x))
        })
        .and_then(|n| {
            mesh.uvs
                .capacity()
                .checked_mul(8)
                .and_then(|x| n.checked_add(x))
        })
        .and_then(|n| {
            mesh.indices
                .capacity()
                .checked_mul(4)
                .and_then(|x| n.checked_add(x))
        })
        .and_then(|n| {
            lineage
                .capacity()
                .checked_mul(std::mem::size_of::<VertexLineage>())
                .and_then(|x| n.checked_add(x))
        })
        .ok_or("Tree attribute capacity overflow")?;
    if actual
        > count
            .checked_mul(512)
            .ok_or("Tree attribute reservation overflow")?
    {
        return Err("Tree actual attribute capacities exceed reserved envelope".into());
    }
    for (t, s) in triangles.iter().zip(sources) {
        let chain = *chains
            .get(s.mesh as usize)
            .ok_or("Invalid union source mesh")?;
        let offset = (s.face as usize)
            .checked_mul(3)
            .ok_or("Source face overflow")?;
        let ids = chain
            .mesh
            .indices
            .get(offset..offset + 3)
            .ok_or("Invalid union source face")?;
        let source = std::array::from_fn::<_, 3, _>(|j| {
            chain.mesh.positions[ids[j] as usize].map(f64::from)
        });
        let reference = face(source[0], source[1], source[2]);
        let start = mesh.positions.len() as u32;
        for &id in t {
            let p = vertices
                .get(id as usize)
                .ok_or("Union vertex outside topology")?;
            let p = [p.x, p.y, p.z];
            let w = barycentric(p, source)?;
            let interpolate = |get: &dyn Fn(usize, usize) -> f32, k: usize| -> f32 {
                (0..3)
                    .map(|j| w[j] * f64::from(get(ids[j] as usize, k)))
                    .sum::<f64>() as f32
            };
            let mut normal = std::array::from_fn::<_, 3, _>(|k| {
                interpolate(&|i, k| chain.mesh.normals[i][k], k)
            });
            let norm = normal.iter().map(|v| v * v).sum::<f32>().sqrt();
            if !norm.is_finite() || norm < 1e-6 {
                return Err("Union source normal interpolation cancels".into());
            }
            for n in &mut normal {
                *n /= norm;
            }
            let uv = std::array::from_fn(|k| interpolate(&|i, k| chain.mesh.uvs[i][k], k));
            let color = std::array::from_fn(|k| interpolate(&|i, k| chain.mesh.colors[i][k], k));
            mesh.push_vertex(p.map(|x| x as f32), normal, color, uv);
            lineage.push(VertexLineage {
                topology_vertex: id,
                input_chain: s.mesh,
                input_face: s.face,
                original_wood_vertices: [
                    chain.original_vertices[ids[0] as usize],
                    chain.original_vertices[ids[1] as usize],
                    chain.original_vertices[ids[2] as usize],
                ],
                barycentric: w,
            });
        }
        let j = start as usize;
        check(
            face(
                mesh.positions[j].map(f64::from),
                mesh.positions[j + 1].map(f64::from),
                mesh.positions[j + 2].map(f64::from),
            ),
            reference,
            min,
        )?;
        mesh.push_tri(start, start + 1, start + 2);
    }
    Ok((mesh, lineage))
}

pub(super) fn strict_frames(mesh: &SubmeshBuilder) -> Result<Vec<[f32; 4]>, String> {
    let n = mesh.positions.len();
    let mut out = vec![[0.; 4]; n];
    let mut temporary = vec![[0.; 4]; n];
    let mut scratch = vec![[0.; 6]; n];
    let work = (n as u64)
        .checked_add((mesh.indices.len() / 3) as u64)
        .and_then(|n| n.checked_mul(100))
        .ok_or("Tree frame work overflow")?;
    qualia_core_db::render::mesh_tangent_frames::tangents_from_uvs_strict_into(
        &mesh.positions,
        &mesh.uvs,
        &mesh.normals,
        &mesh.indices,
        work,
        &mut out,
        &mut temporary,
        &mut scratch,
    )
    .map_err(|e| format!("Tree union changed-face UV/frame admission refused: {e:?}"))?;
    Ok(out)
}
