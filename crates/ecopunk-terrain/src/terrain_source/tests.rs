use super::*;
use qualia_core_db::q42::m64_matter::{M64RasterEdges, M64RasterRows};
use qualia_geospatial::{
    raster_m64::{compile_raster_to_m64_v3, RasterM64Limits, RasterM64Policy},
    ElevationRaster, GeodeticFrame,
};

fn source(north: bool, nodata: Option<i32>) -> Vec<u8> {
    let mut bytes = vec![0; 300_000];
    let n = compile_raster_to_m64_v3(
        &ElevationRaster {
            width: 2,
            height: 2,
            cell_size_mm: 25_000,
            elevations_mm: &[1_000_000, 1_000_000, 1_000_000, 1_004_000],
        },
        &GeodeticFrame {
            srid: qualia_geospatial::SRID_GDA2020,
            vertical_datum: qualia_geospatial::VERTICAL_DATUM_AHD,
            epoch_year: 2020.0,
            origin_lat_deg: -35.1,
            origin_lon_deg: 148.2,
            origin_elevation_mm: 1_000_000,
        },
        1,
        RasterM64Policy {
            rows: if north {
                M64RasterRows::North
            } else {
                M64RasterRows::South
            },
            edges: M64RasterEdges::RepeatNearest,
            nodata_sentinel_mm: nodata,
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
    bytes
}

#[test]
fn source_binding_and_actual_triangle_placement_survive_canonical_compile() {
    for north in [false, true] {
        let raw = source(north, None);
        let terrain = TerrainSource::admit(
            &raw,
            "TEST-ONLY declaration",
            TerrainSourceLimits::default(),
        )
        .unwrap();
        assert_eq!(terrain.source_m64(), raw);
        let scene = terrain.renderable().unwrap();
        let sign = if north { -1.0 } else { 1.0 };
        // Non-planar cell: bilinear would incorrectly report 1 metre at centre.
        let centre = scene.sample_local(12.5, sign * 12.5).unwrap();
        assert!(centre.position_m[1].abs() < 0.001);
        let far = scene.sample_local(18.75, sign * 18.75).unwrap();
        assert!((far.position_m[1] - 2.0).abs() < 0.001);
        assert!(far.geometric_normal[1] > 0.0);
        assert_eq!(far.surface_sha256, Sha256::digest(scene.bytes()).as_slice());
        assert_eq!(far.source_sha256, Sha256::digest(&raw).as_slice());
        let corner = scene.sample_local(25.0, sign * 25.0).unwrap();
        assert!((corner.position_m[1] - 4.0).abs() < 0.001);
        assert!(scene.sample_local(25.01, sign * 25.0).is_err());
        assert!(scene.sample_local(0.0, -sign).is_err());
        assert!(scene.sample_local(f64::NAN, 0.0).is_err());
    }
}

#[test]
fn public_compiler_refuses_mismatched_provenance_and_budgets() {
    let raw = source(false, None);
    let admitted = TerrainSource::admit(&raw, "TEST", TerrainSourceLimits::default()).unwrap();
    let mut frame = admitted.frame().clone();
    frame.origin_elevation_mm += 1000;
    assert!(compile_bound_surface(admitted.samples_mm(), &frame, &raw, "TEST").is_err());
    let mut samples = admitted.samples_mm().to_vec();
    samples[0] += 1;
    assert!(compile_bound_surface(&samples, admitted.frame(), &raw, "TEST").is_err());
    let mut limits = TerrainSourceLimits::default();
    limits.max_owned_bytes = raw.len();
    assert!(TerrainSource::admit(&raw, "TEST", limits).is_err());
    limits = TerrainSourceLimits::default();
    limits.max_cells = 3;
    assert!(TerrainSource::admit(&raw, "TEST", limits).is_err());
    assert!(TerrainSource::admit(
        &source(false, Some(1_004_000)),
        "TEST",
        TerrainSourceLimits::default()
    )
    .is_err());
    assert!(TerrainSource::admit(&raw, "  ", TerrainSourceLimits::default()).is_err());
    let mut corrupt = raw;
    corrupt.push(0);
    assert!(TerrainSource::admit(&corrupt, "TEST", TerrainSourceLimits::default()).is_err());
}
