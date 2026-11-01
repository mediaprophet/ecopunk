//! Optional renderable local-grid derivative of an admitted M64 raster.
//! Reuses Core DEM triangulation, DDG normals and canonical .10d compilation.
//! Coordinates are X=east, Y=height relative to declared origin elevation,
//! Z=south, anchored at the first source cell. No resampling/reprojection occurs.
use super::DemFrameReceipt;
use qualia_core_db::{
    container_10d::{
        attach_material_section, provenance_section::ProvenanceSidecar, MaterialRecord,
        SubmeshRange,
    },
    domains::geospatial::dem::generate_terrain_mesh,
    render::{
        assets::Mesh,
        compile_10d::{compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras},
    },
    specialized_libs::computational_geometry::{ddg_operators::vertex_normals, Point3},
};
use sha2::{Digest, Sha256};
use std::io;

const MAX_SURFACE_CELLS: usize = 65_536;
const MAX_LOCAL_EXTENT_M: f64 = 10_000.0;

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

/// Cold optional derivative. Input must have passed the owning canonical M64
/// admission/no-data checks. This is not source authority or geographic survey
/// acceptance. The exact M64 source and caller licence declaration are carried
/// in canonical provenance. Large grids require separately reviewed tiling.
pub(super) fn compile_surface(
    samples: &[i32],
    frame: &DemFrameReceipt,
    source_m64: &[u8],
    source_licence: &str,
) -> io::Result<Vec<u8>> {
    let count = frame
        .width
        .checked_mul(frame.height)
        .ok_or_else(|| invalid("Terrain surface dimensions overflow"))?;
    let north = match frame.rows.as_str() {
        "North" => true,
        "South" => false,
        _ => {
            return Err(invalid(
                "Terrain surface requires explicit source row direction",
            ))
        }
    };
    if frame.width < 2
        || frame.height < 2
        || count != samples.len()
        || count > MAX_SURFACE_CELLS
        || frame.cell_size_mm == 0
        || source_m64.len() > 16 * 1024 * 1024
        || source_licence.trim().is_empty()
        || source_licence.len() > 1024
    {
        return Err(invalid(
            "Terrain surface shape/source/licence exceeds explicit derivative profile",
        ));
    }
    let cell_m = f64::from(frame.cell_size_mm) / 1000.0;
    if (frame.width - 1) as f64 * cell_m > MAX_LOCAL_EXTENT_M
        || (frame.height - 1) as f64 * cell_m > MAX_LOCAL_EXTENT_M
    {
        return Err(invalid(
            "Terrain surface exceeds 10 km local-grid extent; tiling/reprojection policy required",
        ));
    }
    let heights: Vec<f32> = samples
        .iter()
        .map(|v| ((i64::from(*v) - i64::from(frame.origin_elevation_mm)) as f64 / 1000.0) as f32)
        .collect();
    if heights
        .iter()
        .any(|v| !v.is_finite() || f64::from(v.abs()) > MAX_LOCAL_EXTENT_M)
    {
        return Err(invalid(
            "Terrain surface relative height exceeds 10 km local extent",
        ));
    }
    // Generate unit lattice coordinates first so Core's f32 centering does
    // not round large metre offsets before restoring the first-cell anchor.
    let grid = generate_terrain_mesh(&heights, frame.width, frame.height, 1.0);
    // Core DEM uses centered XY with height on Z. Its indices are reused;
    // translate to the source first-cell anchor and change to Y-up. Flip the
    // north-row winding so both source row conventions face positive world Y.
    let positions: Vec<[f32; 3]> = grid
        .vertices
        .iter()
        .map(|p| {
            [
                ((f64::from(p[0]) + frame.width as f64 / 2.0) * cell_m) as f32,
                p[2],
                ((f64::from(p[1]) + frame.height as f64 / 2.0)
                    * cell_m
                    * if north { -1.0 } else { 1.0 }) as f32,
            ]
        })
        .collect();
    let triangles: Vec<[u32; 3]> = grid
        .indices
        .chunks_exact(3)
        .map(|t| {
            if north {
                [t[0], t[2], t[1]]
            } else {
                [t[0], t[1], t[2]]
            }
        })
        .collect();
    drop(grid);
    drop(heights);
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(p[axis]);
            max[axis] = max[axis].max(p[axis]);
        }
    }
    let points: Vec<Point3> = positions
        .iter()
        .map(|p| Point3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2])))
        .collect();
    let mut normals = vec![Point3::new(0.0, 0.0, 0.0); count];
    vertex_normals(&points, &triangles, &mut normals)
        .map_err(|e| invalid(format!("Terrain normals refused: {e:?}")))?;
    drop(points);
    if normals
        .iter()
        .any(|p| !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite() || p.y <= 0.0)
    {
        return Err(invalid("Terrain surface has invalid/downward normals"));
    }
    let normals: Vec<[f32; 3]> = normals
        .iter()
        .map(|n| [n.x as f32, n.y as f32, n.z as f32])
        .collect();
    let mesh = Mesh {
        positions,
        triangles,
        min,
        max,
    };
    let provenance = ProvenanceSidecar::new(
        source_m64.to_vec(),
        "application/octet-stream",
        source_licence,
    )
    .with_metadata(Vec::new(), 0, Sha256::digest(source_m64).into());
    let bytes = compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        None,
        Some(&normals),
        None,
    )
    .map_err(|e| {
        invalid(format!(
            "Canonical terrain surface compilation refused: {e:?}"
        ))
    })?;
    // Neutral display material is an authored derivative choice, not measured
    // soil/lithology/vegetation. No external texture dependency is introduced.
    let material = MaterialRecord::legacy_default();
    let indices = u32::try_from(
        mesh.triangles
            .len()
            .checked_mul(3)
            .ok_or_else(|| invalid("Terrain indices overflow"))?,
    )
    .map_err(|_| invalid("Terrain index count exceeds u32"))?;
    attach_material_section(
        &bytes,
        &[material],
        &[SubmeshRange {
            first_index: 0,
            index_count: indices,
            material_id: material.id,
            semantic_id: 0,
        }],
        indices,
    )
    .map_err(|e| invalid(format!("Terrain display material refused: {e:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::{
        q42::m64_matter::{M64RasterEdges, M64RasterRows},
        render::compile_10d::{decode_10d_mesh, decode_10d_normals},
    };
    use qualia_geospatial::{
        raster_m64::{compile_raster_to_m64_v3, RasterM64Limits, RasterM64Policy},
        ElevationRaster, GeodeticFrame,
    };
    use std::sync::atomic::AtomicBool;

    fn source(north: bool) -> (Vec<i32>, DemFrameReceipt, Vec<u8>) {
        let samples = vec![1_000_000, 1_001_000, 1_002_000, 1_003_000];
        let frame = DemFrameReceipt {
            width: 2,
            height: 2,
            cell_size_mm: 25_000,
            srid: qualia_geospatial::SRID_GDA2020,
            vertical_datum: qualia_geospatial::VERTICAL_DATUM_AHD,
            epoch_year: 2020.0,
            origin_lat_deg: -35.1,
            origin_lon_deg: 148.2,
            origin_elevation_mm: 1_000_000,
            rows: if north { "North" } else { "South" }.into(),
            edges: "RepeatNearest".into(),
        };
        let geo = GeodeticFrame {
            srid: frame.srid,
            vertical_datum: frame.vertical_datum,
            epoch_year: frame.epoch_year,
            origin_lat_deg: frame.origin_lat_deg,
            origin_lon_deg: frame.origin_lon_deg,
            origin_elevation_mm: frame.origin_elevation_mm,
        };
        let mut bytes = vec![0; 300_000];
        let n = compile_raster_to_m64_v3(
            &ElevationRaster {
                width: 2,
                height: 2,
                cell_size_mm: 25_000,
                elevations_mm: &samples,
            },
            &geo,
            1,
            RasterM64Policy {
                rows: if north {
                    M64RasterRows::North
                } else {
                    M64RasterRows::South
                },
                edges: M64RasterEdges::RepeatNearest,
                nodata_sentinel_mm: None,
            },
            RasterM64Limits {
                max_output_bytes: 300_000,
                max_chunks: 1,
                max_records: 4096,
            },
            &AtomicBool::new(false),
            &mut bytes,
        )
        .unwrap();
        bytes.truncate(n);
        (samples, frame, bytes)
    }
    #[test]
    fn both_row_directions_preserve_first_cell_scale_elevation_and_upward_winding() {
        for north in [false, true] {
            let (samples, frame, source) = source(north);
            let bytes =
                compile_surface(&samples, &frame, &source, "TEST-ONLY declaration").unwrap();
            let mesh = decode_10d_mesh(&bytes).unwrap();
            assert_eq!((mesh.positions.len(), mesh.triangles.len()), (4, 2));
            let sign = if north { -1.0 } else { 1.0 };
            let expected = [
                [0.0, 0.0, 0.0],
                [25.0, 1.0, 0.0],
                [0.0, 2.0, sign * 25.0],
                [25.0, 3.0, sign * 25.0],
            ];
            for (p, want) in mesh.positions.iter().zip(expected) {
                for axis in 0..3 {
                    assert!((p[axis] - want[axis]).abs() < 0.001);
                }
            }
            for t in &mesh.triangles {
                let a = mesh.positions[t[0] as usize];
                let b = mesh.positions[t[1] as usize];
                let c = mesh.positions[t[2] as usize];
                let y = (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2]);
                assert!(y > 0.0);
            }
            assert!(decode_10d_normals(&bytes, 4)
                .unwrap()
                .unwrap()
                .iter()
                .all(|n| n[1] > 0.0));
            let header = qualia_core_db::container_10d::Container10dHeader::parse(&bytes).unwrap();
            let sections =
                qualia_core_db::container_10d::parse_section_table(&bytes, &header).unwrap();
            let p = sections
                .iter()
                .find(|p| {
                    p.typ() == Some(qualia_core_db::container_10d::SectionType::ProvenanceSidecar)
                })
                .unwrap();
            let view =
                qualia_core_db::container_10d::provenance_section::decode_provenance_section(
                    &bytes[p.byte_offset as usize..(p.byte_offset + p.byte_length) as usize],
                )
                .unwrap();
            assert_eq!(view.source_bytes(), source.as_slice());
        }
    }
    #[test]
    fn invalid_or_oversize_grid_is_refused_before_engine_mesh_generation() {
        let (samples, mut frame, source) = source(false);
        frame.width = 1;
        assert!(compile_surface(&samples, &frame, &source, "TEST").is_err());
        frame.width = 2;
        frame.rows = "Unknown".into();
        assert!(compile_surface(&samples, &frame, &source, "TEST").is_err());
        frame.rows = "South".into();
        frame.cell_size_mm = 20_000_000;
        assert!(compile_surface(&samples, &frame, &source, "TEST").is_err());
    }
}
