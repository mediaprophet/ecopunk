//! Shared cold authoring mesh buffers; geometry operations remain QualiaDB-owned.
use qualia_core_db::specialized_libs::computational_geometry::math_geometry::AffineFrame3;
use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::{
    extrude_profile_along_path, ExtrusionStation,
};
use qualia_core_db::specialized_libs::computational_geometry::{Point2, Point3};
/// Multi-submesh geometry accumulator that tracks submesh index ranges per material.
#[derive(Clone, Default)]
pub struct SubmeshBuilder {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl SubmeshBuilder {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn push_vertex(
        &mut self,
        pos: [f32; 3],
        norm: [f32; 3],
        col: [f32; 4],
        uv: [f32; 2],
    ) -> u32 {
        let idx = self.positions.len() as u32;
        let len = (norm[0] * norm[0] + norm[1] * norm[1] + norm[2] * norm[2]).sqrt();
        let unit_norm = if len > 1e-6 {
            [norm[0] / len, norm[1] / len, norm[2] / len]
        } else {
            [0.0, 1.0, 0.0]
        };
        self.positions.push(pos);
        self.normals.push(unit_norm);
        self.colors.push(col);
        self.uvs.push(uv);
        idx
    }

    pub fn push_tri(&mut self, a: u32, b: u32, c: u32) {
        self.indices.extend_from_slice(&[a, b, c]);
    }

    pub fn push_quad(&mut self, a: u32, b: u32, c: u32, d: u32) {
        self.indices.extend_from_slice(&[a, b, c, a, c, d]);
    }

    pub fn recompute_normals_since(&mut self, vertex_start: usize, index_start: usize) {
        if vertex_start >= self.positions.len() || index_start >= self.indices.len() {
            return;
        }
        for n in &mut self.normals[vertex_start..] {
            *n = [0.0, 0.0, 0.0];
        }
        for tri in self.indices[index_start..].chunks_exact(3) {
            let (ia, ib, ic) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
            if ia < vertex_start || ib < vertex_start || ic < vertex_start {
                continue;
            }
            let a = self.positions[ia];
            let b = self.positions[ib];
            let c = self.positions[ic];
            let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let fnrm = [
                ab[1] * ac[2] - ab[2] * ac[1],
                ab[2] * ac[0] - ab[0] * ac[2],
                ab[0] * ac[1] - ab[1] * ac[0],
            ];
            for idx in [ia, ib, ic] {
                self.normals[idx][0] += fnrm[0];
                self.normals[idx][1] += fnrm[1];
                self.normals[idx][2] += fnrm[2];
            }
        }
        for n in &mut self.normals[vertex_start..] {
            let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if l > 1e-6 {
                n[0] /= l;
                n[1] /= l;
                n[2] /= l;
            } else {
                *n = [0.0, 1.0, 0.0];
            }
        }
    }

    /// Extrude a 3D tube/stem/limb along `axis` using `qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::extrude_profile_along_path`.
    pub fn add_bishop_swept_tube(
        &mut self,
        axis: &[Point3],
        stations: &[ExtrusionStation],
        radial_sides: usize,
        base_color: [f32; 4],
        top_color: [f32; 4],
    ) {
        if axis.len() < 2 || stations.len() != axis.len() {
            return;
        }
        let m_sec = radial_sides.max(5);
        let mut cross_section = Vec::with_capacity(m_sec);
        for j in 0..m_sec {
            let theta = std::f64::consts::TAU * (j as f64) / (m_sec as f64);
            cross_section.push(Point2::new(theta.cos(), theta.sin()));
        }
        let n_axis = axis.len();
        let req_v = n_axis * m_sec + 2;
        let req_t = (n_axis - 1) * m_sec * 2 + 2 * m_sec;
        let default_frame = AffineFrame3 {
            origin: Point3::new(0.0, 0.0, 0.0),
            e0: Point3::new(1.0, 0.0, 0.0),
            e1: Point3::new(0.0, 1.0, 0.0),
            e2: Point3::new(0.0, 0.0, 1.0),
        };
        let mut frames = vec![default_frame; n_axis];
        let mut out_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
        let mut out_t = vec![[0u32; 3]; req_t];

        if let Ok((vc, tc)) = extrude_profile_along_path(
            axis,
            &cross_section,
            Some(stations),
            true,
            true,
            &mut frames,
            &mut out_v,
            &mut out_t,
        ) {
            let base_idx = self.positions.len() as u32;
            let mut arc = vec![0.0f64; n_axis];
            for i in 1..n_axis {
                let a = axis[i - 1];
                let b = axis[i];
                arc[i] = arc[i - 1]
                    + ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt();
            }
            let length = arc[n_axis - 1].max(1e-10);

            for (vi, p) in out_v.iter().take(vc).enumerate() {
                let cap = vi >= n_axis * m_sec;
                let ring = if vi == n_axis * m_sec {
                    0
                } else {
                    (vi / m_sec).min(n_axis - 1)
                };
                let origin = frames[ring].origin;
                let sign = if ring == 0 { -1.0 } else { 1.0 };
                let nx = if cap {
                    (frames[ring].e2.x * sign) as f32
                } else {
                    (p.x - origin.x) as f32
                };
                let ny = if cap {
                    (frames[ring].e2.y * sign) as f32
                } else {
                    (p.y - origin.y) as f32
                };
                let nz = if cap {
                    (frames[ring].e2.z * sign) as f32
                } else {
                    (p.z - origin.z) as f32
                };
                let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-5);
                let t = (arc[ring] / length) as f32;
                let col = [
                    base_color[0] * (1.0 - t) + top_color[0] * t,
                    base_color[1] * (1.0 - t) + top_color[1] * t,
                    base_color[2] * (1.0 - t) + top_color[2] * t,
                    1.0,
                ];
                let u = (vi % m_sec) as f32 / m_sec as f32;
                self.push_vertex(
                    [p.x as f32, p.y as f32, p.z as f32],
                    [nx / nlen, ny / nlen, nz / nlen],
                    col,
                    [u, t],
                );
            }
            for tri in out_t.iter().take(tc) {
                self.push_tri(base_idx + tri[0], base_idx + tri[1], base_idx + tri[2]);
            }
        }
    }
}
