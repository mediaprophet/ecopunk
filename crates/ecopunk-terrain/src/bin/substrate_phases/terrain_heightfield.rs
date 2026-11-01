//! Canonical M64 source admission and .10d elevation-field derivation.
//! The M64 remains the exact frame/sample source. FIEL stores metre-valued
//! samples, not a second georeferencing schema or a claim of survey accuracy.
use super::{
    CompiledTerrainResult, CompiledTerrainSurface, DemProvenanceReceipt, DemSurfaceReceipt,
};
use qualia_core_db::container_10d::field_section::{
    encode_field_section_with_rasters, ContinuousRasterField,
};
use qualia_core_db::container_10d::{
    encode_container, seal_whole_file_crc32c, AlignmentTier, Container10dHeader, SectionInput,
    SectionType,
};
use qualia_core_db::q42::m64_matter::{M64PayloadInterval, M64ReadLimits, M64Reader};
use qualia_core_db::q_hash;
use qualia_geospatial::raster_m64::decode_raster_from_m64_v3;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::atomic::AtomicBool;

const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;
const MAX_CELLS: usize = 262_144;
const MAX_CHUNKS: usize = 63;

pub use ecopunk_terrain::terrain_source::DemFrameReceipt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemOutputReceipt {
    pub m64_path: String,
    pub ten_d_path: String,
    pub m64_sha256: String,
    pub ten_d_sha256: String,
}

pub(super) fn output_receipt(
    m64_path: &Path,
    ten_d_path: &Path,
    m64: &[u8],
    ten_d: &[u8],
) -> DemOutputReceipt {
    fn sha(bytes: &[u8]) -> String {
        Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }
    DemOutputReceipt {
        m64_path: m64_path.display().to_string(),
        ten_d_path: ten_d_path.display().to_string(),
        m64_sha256: sha(m64),
        ten_d_sha256: sha(ten_d),
    }
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

pub(super) fn encode_heightfield(
    samples_mm: &[i32],
    width: usize,
    height: usize,
) -> io::Result<Vec<u8>> {
    encode_heightfield_bound(samples_mm, width, height, None)
}

/// Publish only to two absent destinations in a trusted output directory.
/// Existing files are never overwritten. On ordinary I/O failure remove only
/// the files this call created. This is not crash-atomic multi-file publication;
/// consumers still require the package/hash binding before adopting outputs.
pub(super) fn publish_pair_no_clobber(
    m64_path: &Path,
    ten_d_path: &Path,
    m64: &[u8],
    ten_d: &[u8],
) -> io::Result<()> {
    publish_outputs_no_clobber(&[(m64_path, m64), (ten_d_path, ten_d)])
}

fn publish_outputs_no_clobber(outputs: &[(&Path, &[u8])]) -> io::Result<()> {
    if outputs.is_empty() || outputs.len() > 3 {
        return Err(invalid("Terrain output inventory exceeds profile"));
    }
    let mut opened = Vec::with_capacity(outputs.len());
    let result = (|| {
        for (path, _) in outputs {
            opened.push(
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path)?,
            );
        }
        for (file, (_, bytes)) in opened.iter_mut().zip(outputs) {
            file.write_all(bytes)?;
            file.sync_all()?;
        }
        Ok::<_, io::Error>(())
    })();
    let created = opened.len();
    drop(opened);
    if let Err(error) = result {
        let mut failures = Vec::new();
        for (path, _) in &outputs[..created] {
            if let Err(cleanup) = fs::remove_file(path) {
                failures.push(format!("{}: {cleanup}", path.display()));
            }
        }
        if !failures.is_empty() {
            return Err(io::Error::new(
                error.kind(),
                format!(
                    "{error}; newly-created terrain cleanup failed: {}",
                    failures.join("; ")
                ),
            ));
        }
        return Err(error);
    }
    Ok(())
}

fn encode_heightfield_bound(
    samples_mm: &[i32],
    width: usize,
    height: usize,
    source: Option<(&[u8], &str)>,
) -> io::Result<Vec<u8>> {
    let count = width
        .checked_mul(height)
        .ok_or_else(|| invalid("DEM dimensions overflow"))?;
    if count == 0 || count > MAX_CELLS || count != samples_mm.len() {
        return Err(invalid("DEM dimensions/sample budget mismatch"));
    }
    let samples: Vec<f32> = samples_mm.iter().map(|mm| *mm as f32 / 1000.0).collect();
    let field = ContinuousRasterField {
        name_hash: q_hash("geo:elevation"),
        unit_hash: q_hash("http://qudt.org/vocab/unit/M"),
        width: width as u32,
        height: height as u32,
        samples: &samples,
    };
    let mut payload = vec![0; count * 4 + 64];
    let n = encode_field_section_with_rasters(&[field], &mut payload)
        .map_err(|e| invalid(format!("Canonical elevation field refused: {e:?}")))?;
    payload.truncate(n);
    let input = SectionInput {
        section_type: SectionType::FieldSidecar,
        alignment_tier: AlignmentTier::Word,
        stride: 0,
        element_count: 0,
        payload: &payload,
    };
    let provenance = if let Some((bytes, licence)) = source {
        use qualia_core_db::container_10d::provenance_section::{
            encode_provenance_section, encoded_len, ProvenanceSidecar,
        };
        if bytes.len() > MAX_SOURCE_BYTES || licence.trim().is_empty() || licence.len() > 1024 {
            return Err(invalid(
                "M64 derivation requires a bounded explicit source licence statement",
            ));
        }
        // The source M64 carries its own frame. Do not invent a second frame
        // schema, acquisition timestamp, source-rights credential or licence.
        let sidecar = ProvenanceSidecar::new(bytes.to_vec(), "application/octet-stream", licence)
            .with_metadata(Vec::new(), 0, Sha256::digest(bytes).into());
        let mut encoded = vec![0; encoded_len(&sidecar)];
        encode_provenance_section(&sidecar, &mut encoded)
            .map_err(|e| invalid(format!("Canonical M64 provenance refused: {e:?}")))?;
        Some(encoded)
    } else {
        None
    };
    let mut inputs = vec![input];
    if let Some(bytes) = provenance.as_ref() {
        inputs.push(SectionInput {
            section_type: SectionType::ProvenanceSidecar,
            alignment_tier: AlignmentTier::Word,
            stride: 0,
            element_count: 0,
            payload: bytes,
        });
    }
    let mut output = vec![0; payload.len() + provenance.as_ref().map_or(0, Vec::len) + 4096];
    let n = encode_container(&Container10dHeader::proposed(), &inputs, &mut output)
        .map_err(|e| invalid(format!("Canonical .10d envelope refused: {e:?}")))?;
    output.truncate(n);
    seal_whole_file_crc32c(&mut output);
    Ok(output)
}

/// Derive an elevation .10d from an explicitly framed canonical M64 v3 source.
/// At most 16 MiB source bytes and 262144 source cells are admitted; cold
/// working allocations are bounded to source + recovered i32 + f32 + envelopes.
/// This validates declarations, not dataset rights, source identity or accuracy.
/// Byte-identical M64 is copied alongside the .10d; no datum is inferred.
pub fn compile_terrain_from_m64(
    source: &Path,
    assets_dir: &Path,
    source_licence: &str,
    with_surface: bool,
) -> io::Result<CompiledTerrainResult> {
    if source_licence.trim().is_empty() || source_licence.len() > 1024 {
        return Err(invalid(
            "Supply a source licence statement (1..=1024 bytes); no licence is inferred from M64",
        ));
    }
    let mut input = File::open(source)?;
    if !input.metadata()?.is_file() || input.metadata()?.len() > MAX_SOURCE_BYTES as u64 {
        return Err(invalid("M64 source exceeds the 16 MiB input budget"));
    }
    let mut bytes = Vec::new();
    Read::by_ref(&mut input)
        .take(MAX_SOURCE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_SOURCE_BYTES {
        return Err(invalid("M64 source grew beyond input budget"));
    }
    let (samples, mut receipt) = admit_m64(&bytes, source)?;
    let frame = receipt
        .frame
        .as_ref()
        .ok_or_else(|| invalid("M64 frame absent"))?;
    let ten_d = encode_heightfield_bound(
        &samples,
        frame.width,
        frame.height,
        Some((&bytes, source_licence)),
    )?;
    let surface = if with_surface {
        let data =
            super::terrain_surface::compile_surface(&samples, frame, &bytes, source_licence)?;
        let path = assets_dir.join("terrain_surface.10d");
        let sha256 = Sha256::digest(&data)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        receipt.surface = Some(DemSurfaceReceipt {output_path:path.display().to_string(),sha256,
            source_sha256:receipt.source_sha256.clone().ok_or_else(|| invalid("Surface source digest absent"))?,
            units:"metres".into(),axes:"X east; Y up; Z south".into(),local_origin:"first source cell centre".into(),
            vertical_origin_elevation_mm:frame.origin_elevation_mm,
            scope:"Sample-faithful local tangent-grid mesh with neutral display material; source/rights/geographic accuracy unaccepted; .10d quantization applies".into()});
        Some(CompiledTerrainSurface { bytes: data, path })
    } else {
        None
    };
    // All source/format admission and derivation succeeds before any output write.
    let m64_path = assets_dir.join("talbingo_catchment.m64");
    let ten_d_path = assets_dir.join("talbingo_heightfield.10d");
    receipt.outputs = Some(output_receipt(&m64_path, &ten_d_path, &bytes, &ten_d));
    let mut outputs = vec![
        (m64_path.as_path(), bytes.as_slice()),
        (ten_d_path.as_path(), ten_d.as_slice()),
    ];
    if let Some(surface) = surface.as_ref() {
        outputs.push((surface.path.as_path(), surface.bytes.as_slice()));
    }
    publish_outputs_no_clobber(&outputs)?;
    drop(outputs);
    let m64_len = bytes.len();
    Ok(CompiledTerrainResult {
        m64_buffer: bytes,
        m64_len,
        m64_path,
        ten_d_path,
        ten_d_buffer: ten_d,
        surface,
        receipt,
    })
}

fn admit_m64(bytes: &[u8], source: &Path) -> io::Result<(Vec<i32>, DemProvenanceReceipt)> {
    let mut scratch = [M64PayloadInterval::default(); MAX_CHUNKS];
    let reader = M64Reader::parse(
        bytes,
        M64ReadLimits {
            max_file_bytes: MAX_SOURCE_BYTES,
            max_chunks: MAX_CHUNKS,
            max_records: (MAX_CHUNKS * 4096) as u64,
        },
        &mut scratch,
    )
    .map_err(|e| invalid(format!("Canonical M64 admission refused: {e}")))?;
    let meta = reader
        .georaster()
        .ok_or_else(|| invalid("M64 source lacks v3 raster/frame metadata"))?;
    let shape = meta
        .shape()
        .map_err(|e| invalid(format!("M64 raster shape refused: {e:?}")))?;
    let count = shape
        .width
        .checked_mul(shape.height)
        .ok_or_else(|| invalid("M64 cell count overflow"))?;
    if count == 0 || count > MAX_CELLS || meta.srid == 0 || meta.vertical_datum == 0 {
        return Err(invalid(
            "M64 source requires nonzero explicit frame identifiers and bounded dimensions",
        ));
    }
    let mut samples = vec![0; count];
    let (_, frame) = decode_raster_from_m64_v3(&reader, &AtomicBool::new(false), &mut samples)
        .map_err(|e| invalid(format!("Canonical M64 sample recovery refused: {e:?}")))?;
    if meta
        .nodata_sentinel_mm
        .is_some_and(|nodata| samples.contains(&nodata))
    {
        return Err(invalid("Elevation .10d derivation requires an explicit no-data mask policy; no-data cannot become a height"));
    }
    let minimum = *samples.iter().min().ok_or_else(|| invalid("Empty DEM"))?;
    let maximum = *samples.iter().max().ok_or_else(|| invalid("Empty DEM"))?;
    let sum: i64 = samples.iter().map(|v| i64::from(*v)).sum();
    let source_sha256 = Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    let receipt = DemProvenanceReceipt {
        source_path: source.display().to_string(),
        cell_count: count,
        is_synthetic_fixture: false,
        acceptance_status: "FRAME_DECLARATIONS_RETAINED_NOT_SCIENTIFICALLY_ACCEPTED".into(),
        min_elevation_mm: minimum,
        max_elevation_mm: maximum,
        mean_elevation_mm: sum as f64 / count as f64,
        source_sha256: Some(source_sha256),
        outputs: None,
        surface: None,
        frame: Some(DemFrameReceipt {
            width: shape.width,
            height: shape.height,
            cell_size_mm: meta.cell_size_mm,
            srid: frame.srid,
            vertical_datum: frame.vertical_datum,
            epoch_year: frame.epoch_year,
            origin_lat_deg: frame.origin_lat_deg,
            origin_lon_deg: frame.origin_lon_deg,
            origin_elevation_mm: frame.origin_elevation_mm,
            rows: format!("{:?}", meta.rows),
            edges: format!("{:?}", meta.edges),
        }),
    };
    Ok((samples, receipt))
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::container_10d::{
        field_section::{decode_field_raster_samples, decode_field_section},
        parse_section_table,
    };
    use qualia_core_db::q42::m64_matter::{M64RasterEdges, M64RasterRows};
    use qualia_geospatial::{
        raster_m64::{compile_raster_to_m64_v3, RasterM64Limits, RasterM64Policy},
        ElevationRaster, GeodeticFrame,
    };

    fn source(samples: &[i32], srid: u32, nodata: Option<i32>) -> Vec<u8> {
        let raster = ElevationRaster {
            width: 2,
            height: 2,
            cell_size_mm: 25_000,
            elevations_mm: samples,
        };
        let frame = GeodeticFrame {
            srid,
            vertical_datum: qualia_geospatial::VERTICAL_DATUM_AHD,
            epoch_year: 2020.0,
            origin_lat_deg: -35.1,
            origin_lon_deg: 148.2,
            origin_elevation_mm: 123_000,
        };
        let policy = RasterM64Policy {
            rows: M64RasterRows::South,
            edges: M64RasterEdges::RepeatNearest,
            nodata_sentinel_mm: nodata,
        };
        let mut bytes = vec![0; 300_000];
        let n = compile_raster_to_m64_v3(
            &raster,
            &frame,
            1,
            policy,
            RasterM64Limits {
                max_output_bytes: bytes.len(),
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
    fn pair_publication_preserves_existing_output_and_cleans_only_new_file() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let stem = format!(
            "ecopunk-heightfield-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        );
        let first = std::env::temp_dir().join(format!("{stem}.m64"));
        let second = std::env::temp_dir().join(format!("{stem}.10d"));
        struct Owned(std::path::PathBuf);
        impl Drop for Owned {
            fn drop(&mut self) {
                let _ = fs::remove_file(&self.0);
            }
        }
        let mut existing = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&second)
            .unwrap();
        let _owned = Owned(second.clone());
        existing.write_all(b"preserve previous asset").unwrap();
        drop(existing);
        assert!(publish_pair_no_clobber(&first, &second, b"new m64", b"new 10d").is_err());
        assert!(!first.exists());
        assert_eq!(fs::read(&second).unwrap(), b"preserve previous asset");
    }

    #[test]
    fn optional_surface_conflict_cleans_new_pair_without_overwriting_surface() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let stem = format!(
            "ecopunk-heightfield-surface-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        );
        let first = std::env::temp_dir().join(format!("{stem}.m64"));
        let second = std::env::temp_dir().join(format!("{stem}.10d"));
        let third = std::env::temp_dir().join(format!("{stem}-surface.10d"));
        struct Owned(std::path::PathBuf);
        impl Drop for Owned {
            fn drop(&mut self) {
                let _ = fs::remove_file(&self.0);
            }
        }
        let mut old = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&third)
            .unwrap();
        let _owned = Owned(third.clone());
        old.write_all(b"existing surface").unwrap();
        drop(old);
        assert!(publish_outputs_no_clobber(&[
            (&first, b"new source"),
            (&second, b"new field"),
            (&third, b"new surface")
        ])
        .is_err());
        assert!(!first.exists());
        assert!(!second.exists());
        assert_eq!(fs::read(&third).unwrap(), b"existing surface");
    }

    #[test]
    fn canonical_m64_frame_and_sample_order_survive_heightfield_derivation() {
        let bytes = source(
            &[-12_000, 0, 123_000, 456_000],
            qualia_geospatial::SRID_GDA2020,
            None,
        );
        let (samples, receipt) = admit_m64(&bytes, Path::new("declared.m64")).unwrap();
        let frame = receipt.frame.unwrap();
        assert_eq!(
            (frame.width, frame.height, frame.cell_size_mm),
            (2, 2, 25_000)
        );
        assert_eq!(frame.origin_elevation_mm, 123_000);
        assert_eq!(frame.rows, "South");
        assert!(receipt.source_sha256.is_some());
        let encoded = encode_heightfield(&samples, 2, 2).unwrap();
        qualia_core_db::container_10d::integrity::check_whole_file_crc32c(&encoded).unwrap();
        let header = Container10dHeader::parse(&encoded).unwrap();
        let sections = parse_section_table(&encoded, &header).unwrap();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].typ(), Some(SectionType::FieldSidecar));
        let section = &sections[0];
        let payload = &encoded
            [section.byte_offset as usize..(section.byte_offset + section.byte_length) as usize];
        let (_, fields) = decode_field_section(payload).unwrap();
        let (grid, heights) = decode_field_raster_samples(payload, &fields[0]).unwrap();
        assert_eq!((grid.width, grid.height), (2, 2));
        assert_eq!(heights, &[-12.0, 0.0, 123.0, 456.0]);
        assert_eq!(fields[0].unit_hash, q_hash("http://qudt.org/vocab/unit/M"));
    }

    #[test]
    fn missing_frame_and_nodata_never_become_ordinary_heights() {
        assert!(admit_m64(&[0; 32], Path::new("unframed.bin")).is_err());
        let missing_crs = source(&[1, 2, 3, 4], 0, None);
        assert!(admit_m64(&missing_crs, Path::new("unknown-crs.m64")).is_err());
        let nodata = source(
            &[1, -9999, 3, 4],
            qualia_geospatial::SRID_GDA2020,
            Some(-9999),
        );
        assert!(admit_m64(&nodata, Path::new("nodata.m64")).is_err());
        assert!(encode_heightfield(&[1, 2, 3], 2, 2).is_err());
    }

    #[test]
    fn canonical_provenance_physically_binds_exact_m64_frame_and_declared_licence() {
        use qualia_core_db::container_10d::provenance_section::{
            decode_provenance_section, validate_provenance,
        };
        let source = source(
            &[-12_000, 0, 123_000, 456_000],
            qualia_geospatial::SRID_GDA2020,
            None,
        );
        let encoded = encode_heightfield_bound(
            &[-12_000, 0, 123_000, 456_000],
            2,
            2,
            Some((&source, "TEST-ONLY source licence declaration")),
        )
        .unwrap();
        let header = Container10dHeader::parse(&encoded).unwrap();
        let sections = parse_section_table(&encoded, &header).unwrap();
        let provenance = sections
            .iter()
            .find(|s| s.typ() == Some(SectionType::ProvenanceSidecar))
            .unwrap();
        let payload = &encoded[provenance.byte_offset as usize
            ..(provenance.byte_offset + provenance.byte_length) as usize];
        let view = decode_provenance_section(payload).unwrap();
        validate_provenance(&view).unwrap();
        assert_eq!(view.source_bytes(), &source);
        assert_eq!(
            view.version_hash(),
            &<[u8; 32]>::from(Sha256::digest(&source))
        );
        let (recovered, _) = admit_m64(view.source_bytes(), Path::new("embedded.m64")).unwrap();
        assert_eq!(recovered, [-12_000, 0, 123_000, 456_000]);
        assert!(encode_heightfield_bound(&recovered, 2, 2, Some((&source, "  "))).is_err());
    }
}
