//! Native public SDK consumers. Existing bytes are regression inputs, not new assets.
use qualia_core_db::{
    bundle::{BundleReader, BundleWriter},
    q42::m64_matter::{
        M64PayloadInterval, M64ReadError, M64ReadLimits, M64Reader, M64_MAGIC, M64_MODE_STRATA,
    },
    render::compile_10d::{decode_10d_mesh, decode_10d_mesh_frames},
};
use qualia_geospatial::{
    frame::GeodeticFrame,
    geotiff::{
        compile_raster_to_m64, compile_raster_to_m64_with_codec, CompressionCodec, ElevationRaster,
        IngestionError,
    },
};

const STORED_MATTER: &[u8] = include_bytes!("../../../assets/terrain/talbingo_catchment.m64");

fn read_matter(bytes: &[u8]) -> Result<M64Reader<'_>, M64ReadError> {
    let mut scratch = [M64PayloadInterval::default(); 8];
    M64Reader::parse(
        bytes,
        M64ReadLimits { max_file_bytes: 1_048_576, max_chunks: 8, max_records: 32_768 },
        &mut scratch,
    )
}

fn bundle(bytes: Vec<u8>) -> Vec<u8> {
    let mut writer = BundleWriter::new();
    writer.add_file("terrain/reference.m64", "application/x-qualia-m64", bytes, None).unwrap();
    writer.build().unwrap()
}

#[test]
fn stored_matter_fixture_passes_native_reader_without_claiming_geographic_truth() {
    assert_eq!(STORED_MATTER.len(), 262_272);
    let reader = read_matter(STORED_MATTER).unwrap();
    assert_eq!(reader.header().magic, M64_MAGIC);
    assert_eq!(reader.header().version, 1);
    assert_eq!(reader.verification().chunks, 1);
    assert_eq!(reader.verification().total_records, 4096);
    assert_eq!(reader.verification().record_crc_checked, 4096);
    assert_eq!(reader.verification().records_without_checksum, 0);
    assert_eq!(reader.chunk(0).unwrap().directory_entry().chunk_record_mode, M64_MODE_STRATA);
}

#[test]
fn hmc_preserves_intact_matter_and_native_reader_borrows_embedded_payload() {
    let bytes = bundle(STORED_MATTER.to_vec());
    let outer = BundleReader::parse(&bytes).unwrap();
    assert!(outer.verify_entry("terrain/reference.m64"));
    let embedded = outer.get("terrain/reference.m64").unwrap();
    assert_eq!(embedded, STORED_MATTER);
    let reader = read_matter(embedded).unwrap();
    let entry = reader.chunk(0).unwrap();
    let offset = entry.directory_entry().byte_offset as usize;
    assert_eq!(entry.payload().as_ptr(), embedded[offset..].as_ptr());
    assert_eq!(reader.verification().record_crc_checked, 4096);
}

#[test]
fn intact_hmc_does_not_certify_corrupt_inner_matter_records() {
    let original = read_matter(STORED_MATTER).unwrap();
    let offset = original.chunk(0).unwrap().directory_entry().byte_offset as usize;
    let mut corrupt = STORED_MATTER.to_vec();
    corrupt[offset + 8] ^= 1;
    let bytes = bundle(corrupt);
    let outer = BundleReader::parse(&bytes).unwrap();
    assert!(outer.verify_entry("terrain/reference.m64"));
    assert!(matches!(read_matter(outer.get("terrain/reference.m64").unwrap()),
        Err(M64ReadError::Record { chunk: 0, record: 0, .. })));
}

#[test]
fn raster_producer_emits_only_the_raw_codec_it_actually_implements() {
    let elevations = vec![500_000; 64 * 64];
    let raster = ElevationRaster { width: 64, height: 64, cell_size_mm: 30_000, elevations_mm: &elevations };
    let frame = GeodeticFrame::default();
    for codec in [CompressionCodec::Lz4, CompressionCodec::BaoTree,
        CompressionCodec::Htj2kWavelet, CompressionCodec::TerrainRgbPng,
        CompressionCodec::TerrariumPng, CompressionCodec::Av1IntraDrape] {
        let mut out = [0x5a; 1024];
        assert_eq!(compile_raster_to_m64_with_codec(&raster, &frame, 1, codec, &mut out),
            Err(IngestionError::UnsupportedCompression(codec)));
        assert_eq!(out, [0x5a; 1024]);
    }
    let mut out = vec![0; 262_272];
    let written = compile_raster_to_m64(&raster, &frame, 1, &mut out).unwrap();
    let reader = read_matter(&out[..written]).unwrap();
    assert_eq!(reader.header().version, 1);
    assert_eq!(reader.verification().record_crc_checked, 4096);
    assert_eq!(reader.chunk(0).unwrap().directory_entry().compression_flags, 0);
    assert_eq!(reader.chunk(0).unwrap().directory_entry().bao_root_hash_prefix, [0; 12]);
}

#[test]
fn overflowing_raster_dimensions_leave_caller_output_untouched() {
    let raster = ElevationRaster { width: usize::MAX, height: 2, cell_size_mm: 1, elevations_mm: &[] };
    let mut out = [0x5a; 64];
    assert_eq!(compile_raster_to_m64(&raster, &GeodeticFrame::default(), 1, &mut out),
        Err(IngestionError::DimensionMismatch));
    assert_eq!(out, [0x5a; 64]);
}

#[test]
fn existing_10d_and_m64_share_hmc_without_reframing_or_rebuilding_assets() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../assets/reference/community-grounds-growing-station/outputs/round1-film-006/growing-station.10d",
    );
    let geometry = std::fs::read(path).unwrap();
    let mut writer = BundleWriter::new();
    writer.add_file("terrain/reference.m64", "application/x-qualia-m64", STORED_MATTER.to_vec(), None).unwrap();
    writer.add_file("geometry/reference.10d", "application/x-qualia-10d", geometry.clone(), None).unwrap();
    let bytes = writer.build().unwrap();
    let outer = BundleReader::parse(&bytes).unwrap();
    assert_eq!(outer.get("geometry/reference.10d").unwrap(), geometry);
    let matter = read_matter(outer.get("terrain/reference.m64").unwrap()).unwrap();
    assert_eq!(matter.verification().record_crc_checked, 4096);
    let embedded = outer.get("geometry/reference.10d").unwrap();
    let mesh = decode_10d_mesh(embedded).unwrap();
    assert_eq!(mesh.positions.len(), 239_952);
    assert_eq!(mesh.triangles.len(), 79_984);
    assert!(mesh.positions.iter().all(|p| (0..3).all(|a| p[a] >= mesh.min[a] && p[a] <= mesh.max[a])));
    let frames = decode_10d_mesh_frames(embedded, mesh.positions.len()).unwrap().unwrap();
    assert_eq!(frames.normals.unwrap().len(), mesh.positions.len());
    // This is container/SDK compatibility, not a complete HMC embodiment claim.
}

#[test]
fn hmc_reopen_retains_developmental_georaster_metadata_and_exact_source() {
    use qualia_core_db::q42::m64_matter::{M64RasterEdges, M64RasterRows, M64RasterSample};
    use qualia_geospatial::raster_m64::{
        compile_raster_to_m64_v3, decode_raster_from_m64_v3, RasterM64Limits, RasterM64Policy,
    };
    use std::sync::atomic::AtomicBool;
    let frame = GeodeticFrame {
        origin_lat_deg: -35.78123456789012,
        origin_lon_deg: 148.29123456789012,
        origin_elevation_mm: 350_123,
        ..GeodeticFrame::default()
    };
    let samples = [500_123, i32::MIN, -123];
    let raster = ElevationRaster {
        width: 3, height: 1, cell_size_mm: 12_345, elevations_mm: &samples,
    };
    let policy = RasterM64Policy {
        rows: M64RasterRows::South, edges: M64RasterEdges::RepeatNearest,
        nodata_sentinel_mm: Some(i32::MIN),
    };
    let cancel = AtomicBool::new(false);
    let mut encoded = vec![0; 262_400];
    let n = compile_raster_to_m64_v3(&raster, &frame, 1, policy, RasterM64Limits {
        max_output_bytes: encoded.len(), max_chunks: 1, max_records: 4096,
    }, &cancel, &mut encoded).unwrap();
    encoded.truncate(n);
    let packaged = bundle(encoded.clone());
    let reopened = BundleReader::parse(&packaged).unwrap();
    assert!(reopened.verify_entry("terrain/reference.m64"));
    let preserved = reopened.get("terrain/reference.m64").unwrap();
    assert_eq!(preserved, encoded);
    let reader = read_matter(preserved).unwrap();
    let meta = reader.georaster().unwrap();
    assert_eq!(meta.origin_lat_deg.to_bits(), frame.origin_lat_deg.to_bits());
    assert_eq!(meta.origin_lon_deg.to_bits(), frame.origin_lon_deg.to_bits());
    assert_eq!(meta.cell_size_mm, 12_345);
    assert_eq!(meta.vertical_datum, frame.vertical_datum);
    assert_eq!(meta.rows, M64RasterRows::South);
    assert_eq!(reader.raster_sample(1, 0), Some(M64RasterSample::Nodata(i32::MIN)));
    assert_eq!(reader.raster_sample(3, 0), None);
    let mut recovered = [42; 4];
    let (count, restored) = decode_raster_from_m64_v3(&reader, &cancel, &mut recovered).unwrap();
    assert_eq!(count, samples.len());
    assert_eq!(recovered, [500_123, i32::MIN, -123, 42]);
    assert_eq!(restored, frame);
    // Serialization vectors only: no asset, geographic-truth or review claim.
}
