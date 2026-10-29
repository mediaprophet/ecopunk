//! Preserve smooth source normals across the explicit UV seams produced by
//! `uv_atlas::unwrap_atlas_into`. This is a render derivative, not a master.
//!
//! Normals are area-weighted by the existing DDG kernel on the *unsplit*
//! surface, then copied through the atlas source-vertex map. This prevents a
//! shading discontinuity caused solely by splitting a smooth source vertex.
//! It does not preserve hard edges, prove geometric quality, generate seams or
//! certify UV distortion. Caller must retain its source attributes separately.
use super::{ddg_operators::vertex_normals, Point3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasSourceNormalsError {
    InvalidInput,
    Budget,
    OutputTooSmall,
    NormalKernel,
    DegenerateNormal,
}

/// `scratch` may change on any error after validation; `out` is unchanged on
/// all errors. `split_triangles` must preserve original triangle order and
/// winding and `source_vertices` must be the corresponding atlas map.
/// `min_cross_norm_sq_m4` is the minimum squared magnitude of the face cross
/// product (four times squared triangle area) in metres to the fourth power.
pub fn atlas_source_normals_into(
    positions: &[Point3],
    triangles: &[[u32; 3]],
    source_vertices: &[u32],
    split_triangles: &[[u32; 3]],
    work_units: u64,
    min_cross_norm_sq_m4: f64,
    scratch: &mut [Point3],
    out: &mut [Point3],
) -> Result<(), AtlasSourceNormalsError> {
    use AtlasSourceNormalsError::*;
    if positions.is_empty() || triangles.is_empty() || source_vertices.is_empty()
        || triangles.len() != split_triangles.len() { return Err(InvalidInput); }
    if !min_cross_norm_sq_m4.is_finite() || min_cross_norm_sq_m4 < 0.0 { return Err(InvalidInput); }
    let work = u64::try_from(positions.len()).ok()
        .and_then(|n| u64::try_from(triangles.len()).ok().and_then(|f| f.checked_mul(12).and_then(|f| n.checked_mul(3)?.checked_add(f))))
        .and_then(|x| u64::try_from(source_vertices.len()).ok().and_then(|n| n.checked_mul(3).and_then(|n| x.checked_add(n))))
        .ok_or(Budget)?;
    if work > work_units { return Err(Budget); }
    if scratch.len() < positions.len() || out.len() < source_vertices.len() { return Err(OutputTooSmall); }
    if positions.iter().any(|p| !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite())
        || source_vertices.iter().any(|&v| v as usize >= positions.len()) { return Err(InvalidInput); }
    for (original, split) in triangles.iter().zip(split_triangles) {
        if original[0] == original[1] || original[1] == original[2] || original[2] == original[0] { return Err(InvalidInput); }
        for corner in 0..3 {
            let s = split[corner] as usize;
            if s >= source_vertices.len() || original[corner] as usize >= positions.len()
                || source_vertices[s] != original[corner] { return Err(InvalidInput); }
        }
        let a = positions[original[0] as usize];
        let b = positions[original[1] as usize];
        let c = positions[original[2] as usize];
        let u = [b.x-a.x,b.y-a.y,b.z-a.z];
        let v = [c.x-a.x,c.y-a.y,c.z-a.z];
        let cross = [u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0]];
        let area_sq = cross[0]*cross[0]+cross[1]*cross[1]+cross[2]*cross[2];
        if !area_sq.is_finite() || area_sq <= min_cross_norm_sq_m4 { return Err(DegenerateNormal); }
    }
    vertex_normals(positions, triangles, scratch).map_err(|_| NormalKernel)?;
    for &index in source_vertices {
        let n = scratch[index as usize];
        let l2 = n.x * n.x + n.y * n.y + n.z * n.z;
        if !l2.is_finite() || l2 <= 0.0 { return Err(DegenerateNormal); }
    }
    for (dst, &index) in out.iter_mut().zip(source_vertices) { *dst = scratch[index as usize]; }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_uv_vertex_keeps_identical_source_normal() {
        let p = [Point3::new(0.,0.,0.), Point3::new(1.,0.,0.), Point3::new(0.,1.,0.), Point3::new(0.,0.,1.)];
        let t = [[0,1,2],[0,3,1]];
        let map = [0,1,2,0,3,1];
        let split = [[0,1,2],[3,4,5]];
        let mut scratch = [Point3::new(0.,0.,0.);4];
        let mut out = [Point3::new(0.,0.,0.);6];
        atlas_source_normals_into(&p,&t,&map,&split,100,0.0,&mut scratch,&mut out).unwrap();
        assert_eq!((out[0].x,out[0].y,out[0].z),(out[3].x,out[3].y,out[3].z));
        assert!(out[0].y > 0.0 && out[0].z > 0.0);
    }
    #[test]
    fn bad_mapping_or_budget_does_not_publish() {
        let p = [Point3::new(0.,0.,0.),Point3::new(1.,0.,0.),Point3::new(0.,1.,0.)];
        let t = [[0,1,2]];
        let mut scratch = [Point3::new(0.,0.,0.);3];
        let sentinel = Point3::new(7.,8.,9.);
        let mut out = [sentinel;3];
        assert_eq!(atlas_source_normals_into(&p,&t,&[0,1,2],&[[0,2,1]],100,0.0,&mut scratch,&mut out),Err(AtlasSourceNormalsError::InvalidInput));
        assert_eq!(atlas_source_normals_into(&p,&t,&[0,1,2],&t,0,0.0,&mut scratch,&mut out),Err(AtlasSourceNormalsError::Budget));
        assert!(out.iter().all(|n| n.x == 7. && n.y == 8. && n.z == 9.));
    }
    #[test]
    fn zero_area_does_not_publish() {
        let p = [Point3::new(0.,0.,0.),Point3::new(1.,0.,0.),Point3::new(2.,0.,0.)];
        let mut scratch = [Point3::new(0.,0.,0.);3];
        let mut out = [Point3::new(7.,8.,9.);3];
        assert!(atlas_source_normals_into(&p,&[[0,1,2]],&[0,1,2],&[[0,1,2]],100,0.0,&mut scratch,&mut out).is_err());
        assert!(out.iter().all(|n| n.x == 7.));
    }
    #[test]
    fn refuses_degenerate_face_even_when_adjacent_face_has_normal() {
        let p = [Point3::new(0.,0.,0.),Point3::new(1.,0.,0.),Point3::new(2.,0.,0.),Point3::new(0.,1.,0.)];
        let t = [[0,1,3],[0,1,2]];
        let mut scratch = [Point3::new(0.,0.,0.);4];
        let mut out = [Point3::new(7.,8.,9.);4];
        assert_eq!(atlas_source_normals_into(&p,&t,&[0,1,2,3],&t,100,0.0,&mut scratch,&mut out),Err(AtlasSourceNormalsError::DegenerateNormal));
        assert!(out.iter().all(|n| n.x == 7.));
    }
    #[test]
    fn refuses_nonfinite_area_and_caller_threshold_without_publication() {
        let t = [[0,1,2]];
        let mut scratch = [Point3::new(0.,0.,0.);3];
        let mut out = [Point3::new(7.,8.,9.);3];
        let huge = [Point3::new(0.,0.,0.),Point3::new(1e200,0.,0.),Point3::new(0.,1e200,0.)];
        assert_eq!(atlas_source_normals_into(&huge,&t,&[0,1,2],&t,100,0.0,&mut scratch,&mut out),Err(AtlasSourceNormalsError::DegenerateNormal));
        let small = [Point3::new(0.,0.,0.),Point3::new(1.,0.,0.),Point3::new(0.,1.,0.)];
        assert_eq!(atlas_source_normals_into(&small,&t,&[0,1,2],&t,100,1.0,&mut scratch,&mut out),Err(AtlasSourceNormalsError::DegenerateNormal));
        assert!(out.iter().all(|n| n.x == 7.));
    }
}
