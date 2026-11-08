//! DEM Elevation Admission & Strata Synthesis Module
//!
//! Enforces strict fail-closed scientific provenance:
//! Missing DEM data returns `io::ErrorKind::NotFound` unless `--allow-synthetic-fixture`
//! is explicitly provided. Never silently falls back to flat 385,000 mm terrain.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use qualia_core_db::container_10d::{HEADER_BYTE_SIZE, HEADER_VERSION, MAGIC_10D};
use qualia_core_db::q42::m64_matter::{M64ChunkDirectoryEntry, M64Header, M64_MAGIC};
use qualia_core_db::q_hash;
use qualia_geospatial::{
    compile_raster_to_m64, ElevationRaster, GeodeticFrame, CHUNK_CELL_COUNT,
};
use serde::{Deserialize, Serialize};

/// Provenance and acceptance receipt for admitted DEM elevation data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemProvenanceReceipt {
    pub source_path: String,
    pub cell_count: usize,
    pub is_synthetic_fixture: bool,
    pub acceptance_status: String,
    pub min_elevation_mm: i32,
    pub max_elevation_mm: i32,
    pub mean_elevation_mm: f64,
}

/// Result of compiling runtime terrain assets (.m64 and .10d).
pub struct CompiledTerrainResult {
    pub m64_buffer: Vec<u8>,
    pub m64_len: usize,
    pub m64_path: PathBuf,
    pub ten_d_path: PathBuf,
    pub receipt: DemProvenanceReceipt,
}

/// Loads DEM elevation integers from a binary file or admits a synthetic fixture if explicitly authorized.
pub fn load_or_admit_dem_elevations(
    elev_bin_path: &Path,
    expected_cells: usize,
    allow_synthetic_fixture: bool,
) -> io::Result<(Vec<i32>, DemProvenanceReceipt)> {
    let mut elev_mm = Vec::with_capacity(expected_cells);

    let loaded_from_disk = if elev_bin_path.exists() {
        let mut file = File::open(elev_bin_path)?;
        let mut buf = [0u8; 4];
        while file.read_exact(&mut buf).is_ok() {
            elev_mm.push(i32::from_le_bytes(buf));
        }
        elev_mm.len() == expected_cells
    } else {
        false
    };

    if loaded_from_disk {
        let min_el = *elev_mm.iter().min().unwrap_or(&385_000);
        let max_el = *elev_mm.iter().max().unwrap_or(&385_000);
        let sum_el: i64 = elev_mm.iter().map(|&x| x as i64).sum();
        let mean_el = (sum_el as f64) / (elev_mm.len() as f64);

        let receipt = DemProvenanceReceipt {
            source_path: elev_bin_path.display().to_string(),
            cell_count: elev_mm.len(),
            is_synthetic_fixture: false,
            acceptance_status: "SOURCE_ADMITTED".to_string(),
            min_elevation_mm: min_el,
            max_elevation_mm: max_el,
            mean_elevation_mm: mean_el,
        };
        Ok((elev_mm, receipt))
    } else if allow_synthetic_fixture {
        elev_mm.clear();
        elev_mm.resize(expected_cells, 385_000);

        let receipt = DemProvenanceReceipt {
            source_path: "SYNTHETIC_FIXTURE_GRID".to_string(),
            cell_count: expected_cells,
            is_synthetic_fixture: true,
            acceptance_status: "UNACCEPTED_SYNTHETIC_FIXTURE".to_string(),
            min_elevation_mm: 385_000,
            max_elevation_mm: 385_000,
            mean_elevation_mm: 385_000.0,
        };
        Ok((elev_mm, receipt))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "DEM elevation file missing or invalid at {}. \
                 Production requires an identified source snapshot; \
                 pass --allow-synthetic-fixture to admit unaccepted synthetic fixtures.",
                elev_bin_path.display()
            ),
        ))
    }
}

/// Compiles .m64 strata chunk and .10d manifold heightfield.
pub fn compile_terrain_strata_and_10d(
    elev_mm: &[i32],
    assets_dir: &Path,
    receipt: DemProvenanceReceipt,
) -> io::Result<CompiledTerrainResult> {
    let raster = ElevationRaster {
        width: 64,
        height: 64,
        cell_size_mm: 30_000,
        elevations_mm: elev_mm,
    };

    let frame = GeodeticFrame {
        srid: qualia_geospatial::SRID_GDA2020,
        vertical_datum: qualia_geospatial::VERTICAL_DATUM_AHD,
        epoch_year: 2020.0,
        origin_lat_deg: -35.5854,
        origin_lon_deg: 148.3240,
        origin_elevation_mm: 385_000,
    };

    let mut m64_buffer = vec![0u8; 2 * 1024 * 1024];
    let m64_len = compile_raster_to_m64(&raster, &frame, 1, &mut m64_buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("m64 compilation failed: {e:?}")))?;

    let m64_out_path = assets_dir.join("talbingo_catchment.m64");
    fs::write(&m64_out_path, &m64_buffer[..m64_len])?;

    let header: &M64Header = bytemuck::from_bytes(&m64_buffer[..std::mem::size_of::<M64Header>()]);
    assert_eq!(header.magic, M64_MAGIC);
    assert_eq!(header.chunk_count, 1);
    assert!(header.validate().is_ok());

    let dir_offset = std::mem::size_of::<M64Header>();
    let dir_entry: &M64ChunkDirectoryEntry = bytemuck::from_bytes(
        &m64_buffer[dir_offset..dir_offset + std::mem::size_of::<M64ChunkDirectoryEntry>()],
    );
    assert_eq!(dir_entry.srid_geodetic_code, qualia_geospatial::SRID_GDA2020);
    assert_eq!(dir_entry.record_count, 4096);
    assert!(dir_entry.validate().is_ok());

    let ten_d_path = assets_dir.join("talbingo_heightfield.10d");
    let mut ten_d_bytes = Vec::new();

    let mut header_10d = [0u8; HEADER_BYTE_SIZE];
    header_10d[0..4].copy_from_slice(&MAGIC_10D);
    header_10d[4..6].copy_from_slice(&HEADER_VERSION.to_le_bytes());
    header_10d[6..8].copy_from_slice(&1u16.to_le_bytes());
    header_10d[8..16].copy_from_slice(&q_hash("geo:TalbingoCatchment").to_le_bytes());
    ten_d_bytes.extend_from_slice(&header_10d);

    let field_magic = *b"FIEL";
    let mut field_header = [0u8; 32];
    field_header[0..4].copy_from_slice(&field_magic);
    field_header[4..8].copy_from_slice(&20u32.to_le_bytes());
    field_header[8..12].copy_from_slice(&64u32.to_le_bytes());
    field_header[12..16].copy_from_slice(&64u32.to_le_bytes());
    field_header[16..20].copy_from_slice(&30_000u32.to_le_bytes());
    field_header[20..24].copy_from_slice(&385_000u32.to_le_bytes());
    ten_d_bytes.extend_from_slice(&field_header);

    for mm in elev_mm {
        let meters = (*mm as f32) / 1000.0;
        ten_d_bytes.extend_from_slice(&meters.to_le_bytes());
    }

    fs::write(&ten_d_path, &ten_d_bytes)?;

    Ok(CompiledTerrainResult {
        m64_buffer,
        m64_len,
        m64_path: m64_out_path,
        ten_d_path,
        receipt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_dem_fails_closed_by_default() {
        let non_existent = Path::new("target/test_missing_dem_file_does_not_exist.bin");
        let res = load_or_admit_dem_elevations(non_existent, CHUNK_CELL_COUNT, false);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_missing_dem_with_explicit_synthetic_flag_admits_fixture() {
        let non_existent = Path::new("target/test_missing_dem_fixture_allowed.bin");
        let (elevs, receipt) = load_or_admit_dem_elevations(non_existent, CHUNK_CELL_COUNT, true).unwrap();
        assert_eq!(elevs.len(), CHUNK_CELL_COUNT);
        assert!(receipt.is_synthetic_fixture);
        assert_eq!(receipt.acceptance_status, "UNACCEPTED_SYNTHETIC_FIXTURE");
        assert_eq!(receipt.min_elevation_mm, 385_000);
        assert_eq!(receipt.max_elevation_mm, 385_000);
    }

    #[test]
    fn test_real_dem_file_admission_sets_source_admitted() {
        let temp_dir = std::env::temp_dir().join("test_ecopunk_dem_admit");
        let _ = fs::create_dir_all(&temp_dir);
        let test_bin = temp_dir.join("test_4096_elev.bin");

        let mut data = Vec::with_capacity(CHUNK_CELL_COUNT * 4);
        for i in 0..CHUNK_CELL_COUNT {
            let el: i32 = 350_000 + (i as i32) * 10;
            data.extend_from_slice(&el.to_le_bytes());
        }
        fs::write(&test_bin, &data).unwrap();

        let (elevs, receipt) = load_or_admit_dem_elevations(&test_bin, CHUNK_CELL_COUNT, false).unwrap();
        assert_eq!(elevs.len(), CHUNK_CELL_COUNT);
        assert!(!receipt.is_synthetic_fixture);
        assert_eq!(receipt.acceptance_status, "SOURCE_ADMITTED");
        assert_eq!(receipt.min_elevation_mm, 350_000);
        assert_eq!(receipt.max_elevation_mm, 350_000 + 40950);

        let _ = fs::remove_file(&test_bin);
    }
}
