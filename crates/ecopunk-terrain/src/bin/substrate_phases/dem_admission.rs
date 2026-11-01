//! DEM Elevation Admission & Strata Synthesis Module
//!
//! Enforces strict fail-closed scientific provenance:
//! Missing DEM data returns `io::ErrorKind::NotFound` unless `--allow-synthetic-fixture`
//! is explicitly provided. Never silently falls back to flat 385,000 mm terrain.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use qualia_core_db::q42::m64_matter::{M64ChunkDirectoryEntry, M64Header, M64_MAGIC};
use qualia_geospatial::{
    compile_raster_to_m64, ElevationRaster, GeodeticFrame, CHUNK_CELL_COUNT,
};
use serde::{Deserialize, Serialize};

#[path = "terrain_heightfield.rs"]
mod terrain_heightfield;
pub use terrain_heightfield::{compile_terrain_from_m64, DemFrameReceipt, DemOutputReceipt};
#[path = "terrain_surface.rs"]
mod terrain_surface;

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
    /// Declaration retained from a canonical v3 M64 source; not survey acceptance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame: Option<DemFrameReceipt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<DemOutputReceipt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<DemSurfaceReceipt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemSurfaceReceipt {
    pub output_path: String,
    pub sha256: String,
    pub source_sha256: String,
    pub units: String,
    pub axes: String,
    pub local_origin: String,
    pub vertical_origin_elevation_mm: i32,
    pub scope: String,
}

pub struct CompiledTerrainSurface { pub bytes: Vec<u8>, pub path: PathBuf }

/// Result of compiling runtime terrain assets (.m64 and .10d).
pub struct CompiledTerrainResult {
    pub m64_buffer: Vec<u8>,
    pub m64_len: usize,
    pub m64_path: PathBuf,
    pub ten_d_path: PathBuf,
    pub ten_d_buffer: Vec<u8>,
    pub surface: Option<CompiledTerrainSurface>,
    pub receipt: DemProvenanceReceipt,
}

/// Loads DEM elevation integers from a binary file or admits a synthetic fixture if explicitly authorized.
pub fn load_or_admit_dem_elevations(
    elev_bin_path: &Path,
    expected_cells: usize,
    allow_synthetic_fixture: bool,
) -> io::Result<(Vec<i32>, DemProvenanceReceipt)> {
    if expected_cells == 0 || expected_cells > 262_144 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "DEM cell budget is 1..=262144"));
    }
    let expected_bytes = expected_cells.checked_mul(4)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "DEM size overflow"))?;
    let mut elev_mm = Vec::with_capacity(expected_cells);

    let loaded_from_disk = if elev_bin_path.exists() {
        let mut file = File::open(elev_bin_path)?;
        // A partial trailing sample or an I/O error is never silently accepted.
        if file.metadata()?.len() != expected_bytes as u64 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "DEM byte length does not match declared cells"));
        }
        let mut buf = [0u8; 4];
        for _ in 0..expected_cells {
            file.read_exact(&mut buf)?;
            elev_mm.push(i32::from_le_bytes(buf));
        }
        let mut extra = [0u8; 1];
        if file.read(&mut extra)? != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "DEM grew during admission"));
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
            frame: None,
            source_sha256: None,
            outputs: None,
            surface: None,
        };
        Ok((elev_mm, receipt))
    } else if allow_synthetic_fixture {
        synthetic_dem_fixture(expected_cells)
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

pub fn synthetic_dem_fixture(expected_cells: usize) -> io::Result<(Vec<i32>, DemProvenanceReceipt)> {
    if expected_cells == 0 || expected_cells > 262_144 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "DEM fixture cell budget exceeded"));
    }
    Ok((vec![385_000; expected_cells], DemProvenanceReceipt {
        source_path: "SYNTHETIC_FIXTURE_GRID".into(), cell_count: expected_cells,
        is_synthetic_fixture: true, acceptance_status: "UNACCEPTED_SYNTHETIC_FIXTURE".into(),
        min_elevation_mm: 385_000, max_elevation_mm: 385_000, mean_elevation_mm: 385_000.0,
        frame: None, source_sha256: None, outputs: None, surface: None,
    }))
}

/// Compiles .m64 strata chunk and .10d manifold heightfield.
pub fn compile_terrain_strata_and_10d(
    elev_mm: &[i32],
    assets_dir: &Path,
    mut receipt: DemProvenanceReceipt,
) -> io::Result<CompiledTerrainResult> {
    // Raw integers have no georeference. The historical frame is retained only
    // for an explicitly admitted synthetic fixture, never inferred for real data.
    if !receipt.is_synthetic_fixture || elev_mm.len() != CHUNK_CELL_COUNT {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,
            "Production terrain requires --dem-m64 with canonical frame metadata; raw legacy compilation is synthetic-fixture only"));
    }
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
    let ten_d_bytes = terrain_heightfield::encode_heightfield(elev_mm, 64, 64)?;
    receipt.outputs = Some(terrain_heightfield::output_receipt(&m64_out_path, &ten_d_path,
        &m64_buffer[..m64_len], &ten_d_bytes));
    terrain_heightfield::publish_pair_no_clobber(&m64_out_path, &ten_d_path,
        &m64_buffer[..m64_len], &ten_d_bytes)?;

    Ok(CompiledTerrainResult {
        m64_buffer,
        m64_len,
        m64_path: m64_out_path,
        ten_d_path,
        ten_d_buffer: ten_d_bytes,
        surface: None,
        receipt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_dem_refuses_partial_trailing_samples_and_fixture_substitution() {
        use std::io::Write;
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        struct OwnedFile(PathBuf);
        impl Drop for OwnedFile { fn drop(&mut self) { let _ = fs::remove_file(&self.0); } }
        let path = std::env::temp_dir().join(format!("ecopunk-dem-{}-{}.bin",
            std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&path).unwrap();
        let owned = OwnedFile(path);
        file.write_all(&123_i32.to_le_bytes()).unwrap();
        file.write_all(&[7]).unwrap();
        drop(file);
        // An explicit fixture fallback does not conceal an existing bad source.
        assert_eq!(load_or_admit_dem_elevations(&owned.0, 1, false).unwrap_err().kind(), io::ErrorKind::InvalidData);
        assert_eq!(load_or_admit_dem_elevations(&owned.0, 1, true).unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

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
