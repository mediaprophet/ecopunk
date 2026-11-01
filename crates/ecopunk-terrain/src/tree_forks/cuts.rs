//! Exact source-triangle cuts of an existing hard-cap sweep wall.
//! A side quad is not assumed planar. Its diagonal remains an explicit ring vertex.
use crate::scene_builder_10d::SubmeshBuilder;

#[derive(Clone, Debug)]
pub(super) struct Sample {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
    pub source_vertices: [u32; 3],
    pub barycentric: [f64; 3],
}

impl Sample {
    pub fn original(mesh: &SubmeshBuilder, vertex: u32) -> Result<Self, String> {
        let i = vertex as usize;
        Ok(Self {
            position: *mesh
                .positions
                .get(i)
                .ok_or("Source cut position out of range")?,
            normal: *mesh
                .normals
                .get(i)
                .ok_or("Source cut normal out of range")?,
            color: *mesh.colors.get(i).ok_or("Source cut color out of range")?,
            uv: *mesh.uvs.get(i).ok_or("Source cut UV out of range")?,
            source_vertices: [vertex; 3],
            barycentric: [1.0, 0.0, 0.0],
        })
    }

    fn between(mesh: &SubmeshBuilder, a: u32, b: u32, t: f64) -> Result<Self, String> {
        if !t.is_finite() || !(0.0..=1.0).contains(&t) {
            return Err("Source-triangle cut fraction outside edge".into());
        }
        if t == 0.0 {
            return Self::original(mesh, a);
        }
        if t == 1.0 {
            return Self::original(mesh, b);
        }
        let a_sample = Self::original(mesh, a)?;
        let b_sample = Self::original(mesh, b)?;
        fn mix<const N: usize>(a: [f32; N], b: [f32; N], t: f64) -> [f32; N] {
            std::array::from_fn(|i| (f64::from(a[i]) * (1.0 - t) + f64::from(b[i]) * t) as f32)
        }
        let mut normal = mix(a_sample.normal, b_sample.normal, t);
        let length = normal
            .iter()
            .map(|&v| f64::from(v).powi(2))
            .sum::<f64>()
            .sqrt();
        if !length.is_finite() || length <= 1e-12 {
            return Err("Source-triangle cut has no valid interpolated normal".into());
        }
        for v in &mut normal {
            *v = (f64::from(*v) / length) as f32;
        }
        let out = Self {
            position: mix(a_sample.position, b_sample.position, t),
            normal,
            color: mix(a_sample.color, b_sample.color, t),
            uv: mix(a_sample.uv, b_sample.uv, t),
            source_vertices: [a, b, b],
            barycentric: [1.0 - t, t, 0.0],
        };
        if out
            .position
            .iter()
            .chain(out.normal.iter())
            .chain(out.color.iter())
            .chain(out.uv.iter())
            .any(|v| !v.is_finite())
        {
            return Err("Nonfinite source-triangle cut attributes".into());
        }
        Ok(out)
    }

    /// Stable construction identity, never a nearest-position weld.
    pub fn identity(&self) -> ([u32; 3], [u64; 3]) {
        (self.source_vertices, self.barycentric.map(f64::to_bits))
    }
}

/// The ordered ring is W0,D0,W1,D1,..., where D lies on the actual source
/// triangle diagonal. Endpoint duplicates deliberately retain the same identity.
pub(super) fn ring(
    mesh: &SubmeshBuilder,
    vertex_base: usize,
    station: usize,
    sides: usize,
    fraction: f64,
) -> Result<Vec<Sample>, String> {
    if !(5..=24).contains(&sides) || !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        return Err("Source-triangle ring profile refused".into());
    }
    let first = station
        .checked_mul(sides)
        .and_then(|v| v.checked_add(vertex_base))
        .ok_or("Source-triangle station offset overflow")?;
    let second = first
        .checked_add(sides)
        .ok_or("Source-triangle ring offset overflow")?;
    let mut out = Vec::with_capacity(sides * 2);
    for side in 0..sides {
        let next = (side + 1) % sides;
        let a = u32::try_from(first + side).map_err(|_| "Source cut index exceeds u32")?;
        let c = u32::try_from(second + next).map_err(|_| "Source cut index exceeds u32")?;
        let d = u32::try_from(second + side).map_err(|_| "Source cut index exceeds u32")?;
        out.push(Sample::between(mesh, a, d, fraction)?);
        out.push(Sample::between(mesh, a, c, fraction)?);
    }
    Ok(out)
}

/// Emit an exact scalar-station clipping of each original pair of triangles.
/// Caller maps samples through their construction identities; no position hashing.
pub(super) fn strip(
    lower: &[Sample],
    upper: &[Sample],
    mut triangle: impl FnMut([&Sample; 3]) -> Result<(), String>,
) -> Result<(), String> {
    if lower.len() != upper.len() || lower.len() % 2 != 0 || !(10..=48).contains(&lower.len()) {
        return Err("Mismatched source-triangle cut rings".into());
    }
    for side in 0..lower.len() / 2 {
        let w = side * 2;
        let d = w + 1;
        let next = (w + 2) % lower.len();
        for face in [
            [&lower[d], &lower[next], &upper[next]],
            [&lower[d], &upper[next], &upper[d]],
            [&lower[w], &lower[d], &upper[d]],
            [&lower[w], &upper[d], &upper[w]],
        ] {
            // Only the exact endpoint identities cause an intentionally empty
            // clipping triangle. Geometric collapse at distinct identities is
            // refused by the owning geometry/projection admission, never skipped.
            if face[0].identity() != face[1].identity()
                && face[1].identity() != face[2].identity()
                && face[2].identity() != face[0].identity()
            {
                triangle(face)?;
            }
        }
    }
    Ok(())
}
