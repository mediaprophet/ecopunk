use super::*;
use qualia_core_db::specialized_libs::computational_geometry::{
    smooth_junction::FaceSource, Point3,
};
use std::collections::BTreeMap;
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}
pub(super) fn weights(
    p: Point3,
    piece: &CollarPiece,
    face: u32,
) -> Result<([u32; 3], [f64; 3]), String> {
    let at = (face as usize)
        .checked_mul(3)
        .ok_or("Collar source face overflow")?;
    let ids = piece
        .mesh
        .indices
        .get(at..at + 3)
        .ok_or("Collar source face absent")?;
    let ids = [ids[0], ids[1], ids[2]];
    let t = ids.map(|i| piece.mesh.positions[i as usize].map(f64::from));
    let p = [p.x, p.y, p.z];
    for i in 0..3 {
        if p == t[i] {
            let mut w = [0.; 3];
            w[i] = 1.;
            return Ok((ids, w));
        }
    }
    let u = sub(t[1], t[0]);
    let v = sub(t[2], t[0]);
    let q = sub(p, t[0]);
    let uu = dot(u, u);
    let uv = dot(u, v);
    let vv = dot(v, v);
    let d = uu * vv - uv * uv;
    if !d.is_finite() || d <= 0. {
        return Err("Degenerate collar source attribute triangle".into());
    }
    let b = (dot(q, u) * vv - dot(q, v) * uv) / d;
    let c = (dot(q, v) * uu - dot(q, u) * uv) / d;
    let w = [1. - b - c, b, c];
    let rebuilt = std::array::from_fn(|i| w[0] * t[0][i] + w[1] * t[1][i] + w[2] * t[2][i]);
    if w.iter()
        .any(|v| !v.is_finite() || *v < -1e-7 || *v > 1. + 1e-7)
        || dot(sub(rebuilt, p), sub(rebuilt, p)) > 4e-18
    {
        return Err("Collar reference drifted off its exact source face".into());
    }
    Ok((ids, w))
}
pub(super) fn fade(
    seed: &seed::Seed,
    input: &CollarInput<'_>,
    work: &mut usize,
) -> Result<Vec<f64>, String> {
    seed::charge(
        work,
        seed.triangles
            .len()
            .checked_mul(128)
            .ok_or("Collar fade work overflow")?,
    )?;
    let mut fade = seed::allocate(seed.positions.len(), f64::INFINITY)?;
    for (t, s) in seed.triangles.iter().zip(&seed.sources) {
        let piece = &input.pieces[s.piece as usize];
        for &v in t {
            let (ids, w) = weights(seed.positions[v as usize], piece, s.face)?;
            let values = ids.map(|i| piece.fade_coordinates[i as usize]);
            let d = if values[0] == values[1] && values[1] == values[2] {
                values[0]
            } else {
                values.into_iter().zip(w).map(|(v, w)| v * w).sum()
            };
            fade[v as usize] = fade[v as usize].min(d);
        }
    }
    Ok(fade)
}
pub(super) struct Projected {
    pub mesh: SubmeshBuilder,
    pub aliases: Vec<u32>,
    pub lineage: Vec<serde_json::Value>,
    pub bindings: Vec<serde_json::Value>,
}
pub(super) fn project(
    input: &CollarInput<'_>,
    positions: &[Point3],
    references: &[Point3],
    normals: &[Point3],
    weights_out: &[f64],
    triangles: &[[u32; 3]],
    sources: &[FaceSource],
    old_to_compact: &[u32],
    removed: &std::collections::BTreeSet<(u32, u32)>,
    minimum_area: f64,
    work: &mut usize,
) -> Result<Projected, String> {
    seed::charge(
        work,
        triangles
            .len()
            .checked_mul(512)
            .ok_or("Collar attribute work overflow")?,
    )?;
    let mut mesh = SubmeshBuilder::new();
    let mut aliases = Vec::new();
    let mut lineage = Vec::new();
    let mut bindings = Vec::new();
    let mut binding_index = BTreeMap::new();
    for (t, s) in triangles.iter().zip(sources) {
        if removed.contains(&(s.piece, s.face)) {
            continue;
        }
        let piece = &input.pieces[s.piece as usize];
        let start = mesh.positions.len() as u32;
        for &id in t {
            let index = id as usize;
            let (ids, w) = self::weights(references[index], piece, s.face)?;
            if ids.iter().any(|&i| {
                piece.vertex_sources[i as usize]
                    .original_wood_vertices
                    .is_none()
            }) {
                return Err(
                    "Surviving collar face depends on artificial cut-cap construction point".into(),
                );
            }
            let source_binding = *binding_index.entry((s.piece, s.face)).or_insert_with(|| {
                let n = bindings.len() as u32;
                // Array schema: [piece, local_face, local_vertex_ids,
                // [[original_wood_ids, source_weights];3]]. No invented IDs.
                bindings.push(serde_json::json!([
                    s.piece,
                    s.face,
                    ids,
                    ids.map(|i| {
                        let v = piece.vertex_sources[i as usize];
                        serde_json::json!([v.original_wood_vertices, v.barycentric])
                    })
                ]));
                n
            });
            let interpolate = |get: &dyn Fn(usize, usize) -> f32, k: usize| -> f32 {
                if let Some(j) = w.iter().position(|&x| x == 1.) {
                    if w.iter().filter(|&&x| x == 0.).count() == 2 {
                        return get(ids[j] as usize, k);
                    }
                }
                (0..3)
                    .map(|j| w[j] * f64::from(get(ids[j] as usize, k)))
                    .sum::<f64>() as f32
            };
            let p = positions[index];
            let normal = normals[index];
            let mut n = [normal.x as f32, normal.y as f32, normal.z as f32];
            if weights_out[index] == 0. {
                n = std::array::from_fn(|k| interpolate(&|i, k| piece.mesh.normals[i][k], k));
            }
            let length = n.iter().map(|v| v * v).sum::<f32>().sqrt();
            if !length.is_finite() || length < 1e-6 {
                return Err("Collar physical/prescribed normal undefined".into());
            }
            let exact_fixed = weights_out[index] == 0.
                && w.iter().any(|&x| x == 1.)
                && w.iter().filter(|&&x| x == 0.).count() == 2;
            if exact_fixed {
                if (length - 1.).abs() > 1e-4 {
                    return Err("Prescribed exact boundary normal is not unit length".into());
                }
            } else {
                for x in &mut n {
                    *x /= length
                }
            }
            let uv = std::array::from_fn(|k| interpolate(&|i, k| piece.mesh.uvs[i][k], k));
            let color = std::array::from_fn(|k| interpolate(&|i, k| piece.mesh.colors[i][k], k));
            // These streams have already been validated/normalized above.
            // push_vertex normalizes again and changes prescribed boundary bits.
            mesh.positions.push([p.x as f32, p.y as f32, p.z as f32]);
            mesh.normals.push(n);
            mesh.colors.push(color);
            mesh.uvs.push(uv);
            aliases.push(old_to_compact[index]);
            let r = references[index];
            lineage.push(serde_json::json!([
                source_binding,
                w,
                [p.x - r.x, p.y - r.y, p.z - r.z]
            ]));
        }
        let vector = |a: [f64; 3], b: [f64; 3], c: [f64; 3]| {
            let u = sub(b, a);
            let v = sub(c, a);
            [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ]
        };
        let actual = vector(
            mesh.positions[start as usize].map(f64::from),
            mesh.positions[start as usize + 1].map(f64::from),
            mesh.positions[start as usize + 2].map(f64::from),
        );
        let p = t.map(|i| {
            let p = positions[i as usize];
            [p.x, p.y, p.z]
        });
        let reference = vector(p[0], p[1], p[2]);
        if dot(actual, actual) <= minimum_area || dot(actual, reference) <= 0. {
            return Err("Smooth collar f32 projection collapses or reverses a face".into());
        }
        mesh.push_tri(start, start + 1, start + 2);
    }
    let n = mesh.positions.len();
    let frame_work = (n as u64)
        .checked_add((mesh.indices.len() / 3) as u64)
        .and_then(|n| n.checked_mul(100))
        .ok_or("Collar tangent work overflow")?;
    seed::charge(
        work,
        usize::try_from(frame_work).map_err(|_| "Collar frame work overflow")?,
    )?;
    let mut frames = seed::allocate(n, [0.; 4])?;
    let mut tmp = seed::allocate(n, [0.; 4])?;
    let mut scratch = seed::allocate(n, [0.; 6])?;
    qualia_core_db::render::mesh_tangent_frames::tangents_from_uvs_strict_into(
        &mesh.positions,
        &mesh.uvs,
        &mesh.normals,
        &mesh.indices,
        frame_work,
        &mut frames,
        &mut tmp,
        &mut scratch,
    )
    .map_err(|e| format!("Smooth collar UV chart/frame admission: {e:?}"))?;
    Ok(Projected {
        mesh,
        aliases,
        lineage,
        bindings,
    })
}
/// Count actual Value vector/string capacities; object nodes are charged at
/// 512 bytes/entry (includes key/value/tree-node storage, not just JSON text).
pub(super) fn value_bytes(v: &serde_json::Value) -> Result<usize, String> {
    use serde_json::Value;
    let base = std::mem::size_of::<Value>();
    let extra = match v {
        Value::Array(a) => {
            let mut n = a
                .capacity()
                .checked_mul(base)
                .ok_or("Metadata capacity overflow")?;
            for v in a {
                n = n
                    .checked_add(value_bytes(v)?.saturating_sub(base))
                    .ok_or("Metadata capacity overflow")?
            }
            n
        }
        Value::Object(m) => {
            let mut n = m
                .len()
                .checked_mul(512)
                .ok_or("Metadata capacity overflow")?;
            for (k, v) in m {
                n = n
                    .checked_add(k.capacity())
                    .and_then(|n| n.checked_add(value_bytes(v).ok()?))
                    .ok_or("Metadata capacity overflow")?
            }
            n
        }
        Value::String(s) => s.capacity(),
        _ => 0,
    };
    base.checked_add(extra)
        .ok_or_else(|| "Metadata capacity overflow".into())
}
