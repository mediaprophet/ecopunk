//! Placement on the actual decoded canonical terrain derivative. Bilinear
//! source interpolation would disagree with Core DEM's two planar triangles.
use super::{invalid, TerrainSource};
use qualia_core_db::render::{assets::Mesh, compile_10d::decode_10d_mesh};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainPlacementError {
    NonfiniteQuery,
    OutsideSurface,
    DegenerateFace,
    NonfiniteHeight,
}
impl std::fmt::Display for TerrainPlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Terrain placement: {self:?}")
    }
}
impl std::error::Error for TerrainPlacementError {}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TerrainPlacement {
    pub position_m: [f64; 3],
    pub geometric_normal: [f64; 3],
    pub triangle: usize,
    pub source_sha256: [u8; 32],
    pub surface_sha256: [u8; 32],
}

pub struct TerrainSurfaceScene {
    source: TerrainSource,
    bytes: Vec<u8>,
    mesh: Mesh,
    sha256: [u8; 32],
}
impl TerrainSurfaceScene {
    pub(super) fn from_compiled(source: TerrainSource, bytes: Vec<u8>) -> io::Result<Self> {
        let mesh =
            decode_10d_mesh(&bytes).map_err(|e| invalid(format!("Terrain readback: {e:?}")))?;
        let f = source.frame();
        let count = f
            .width
            .checked_mul(f.height)
            .ok_or_else(|| invalid("Terrain grid overflow"))?;
        let triangles = (f.width - 1)
            .checked_mul(f.height - 1)
            .and_then(|n| n.checked_mul(2))
            .ok_or_else(|| invalid("Terrain triangle count overflow"))?;
        if mesh.positions.len() != count
            || mesh.triangles.len() != triangles
            || mesh.positions.iter().flatten().any(|v| !v.is_finite())
        {
            return Err(invalid("Terrain decoded geometry correspondence failed"));
        }
        let sign = if f.rows == "North" { -1.0 } else { 1.0 };
        for row in 0..f.height {
            for col in 0..f.width {
                let p = mesh.positions[row * f.width + col];
                if p[0] != mesh.positions[col][0]
                    || p[2] != mesh.positions[row * f.width][2]
                    || (col > 0 && p[0] <= mesh.positions[row * f.width + col - 1][0])
                    || (row > 0
                        && sign * p[2] <= sign * mesh.positions[(row - 1) * f.width + col][2])
                {
                    return Err(invalid(
                        "Terrain quantization collapsed or changed grid axes",
                    ));
                }
            }
        }
        for row in 0..f.height - 1 {
            for col in 0..f.width - 1 {
                let a = (row * f.width + col) as u32;
                let b = a + 1;
                let c = a + f.width as u32;
                let d = c + 1;
                let expected = if sign < 0.0 {
                    [[a, b, c], [b, d, c]]
                } else {
                    [[a, c, b], [b, c, d]]
                };
                let start = 2 * (row * (f.width - 1) + col);
                if mesh.triangles[start..start + 2] != expected {
                    return Err(invalid(
                        "Terrain triangle lineage changed in canonical compile",
                    ));
                }
            }
        }
        let sha256 = Sha256::digest(&bytes).into();
        Ok(Self {
            source,
            bytes,
            mesh,
            sha256,
        })
    }
    pub fn source(&self) -> &TerrainSource {
        &self.source
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
    pub fn surface_sha256(&self) -> [u8; 32] {
        self.sha256
    }
    pub fn bounds(&self) -> ([f32; 3], [f32; 3]) {
        (self.mesh.min, self.mesh.max)
    }

    /// Allocation-free geometric grounding. Local X=east, Y=up, Z=south, in
    /// metres relative to source first-cell centre / declared elevation origin.
    /// Outside-domain and nonfinite queries refuse; no clamping or default plane.
    /// This is the rendered quantized mesh, not integer-mm source accuracy.
    pub fn sample_local(&self, east_m: f64, south_m: f64) -> Result<TerrainPlacement, TerrainPlacementError> {
        if !east_m.is_finite() || !south_m.is_finite() {
            return Err(TerrainPlacementError::NonfiniteQuery);
        }
        let f = self.source.frame();
        let sign = if f.rows == "North" { -1.0 } else { 1.0 };
        let row_distance = sign * south_m;
        let col_at = |c: usize| f64::from(self.mesh.positions[c][0]);
        let row_at = |r: usize| sign * f64::from(self.mesh.positions[r * f.width][2]);
        if east_m < col_at(0)
            || east_m > col_at(f.width - 1)
            || row_distance < row_at(0)
            || row_distance > row_at(f.height - 1)
        {
            return Err(TerrainPlacementError::OutsideSurface);
        }
        // Binary search the actual quantized axes, not assumed ideal cell sizes.
        fn cell(mut hi: usize, value: f64, at: impl Fn(usize) -> f64) -> usize {
            let mut lo = 0;
            while lo + 1 < hi {
                let mid = lo + (hi - lo) / 2;
                if at(mid) <= value {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            lo
        }
        let col = cell(f.width - 1, east_m, col_at);
        let row = cell(f.height - 1, row_distance, row_at);
        let u = (east_m - col_at(col)) / (col_at(col + 1) - col_at(col));
        let v = (row_distance - row_at(row)) / (row_at(row + 1) - row_at(row));
        let triangle = 2 * (row * (f.width - 1) + col) + usize::from(u + v > 1.0);
        let t = self.mesh.triangles[triangle];
        let p = t.map(|i| self.mesh.positions[i as usize].map(f64::from));
        let ab = [p[1][0] - p[0][0], p[1][1] - p[0][1], p[1][2] - p[0][2]];
        let ac = [p[2][0] - p[0][0], p[2][1] - p[0][1], p[2][2] - p[0][2]];
        let normal = [
            ab[1] * ac[2] - ab[2] * ac[1],
            ab[2] * ac[0] - ab[0] * ac[2],
            ab[0] * ac[1] - ab[1] * ac[0],
        ];
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if !length.is_finite() || length == 0.0 || normal[1] <= 0.0 {
            return Err(TerrainPlacementError::DegenerateFace);
        }
        let height = p[0][1]
            - (normal[0] * (east_m - p[0][0]) + normal[2] * (south_m - p[0][2])) / normal[1];
        if !height.is_finite() {
            return Err(TerrainPlacementError::NonfiniteHeight);
        }
        Ok(TerrainPlacement {
            position_m: [east_m, height, south_m],
            geometric_normal: normal.map(|v| v / length),
            triangle,
            source_sha256: self.source.source_sha256(),
            surface_sha256: self.sha256,
        })
    }
}
