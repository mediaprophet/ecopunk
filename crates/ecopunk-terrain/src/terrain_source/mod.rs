//! Explicit source terrain for authoring and consumer placement. Canonical M64
//! owns the declared frame and samples; `.10d` owns the rendered derivative.
//! No CRS transformation, source licence clearance or surveyed accuracy is implied.
mod placement;
mod surface;
#[cfg(test)]
mod tests;

pub use placement::{TerrainPlacement, TerrainPlacementError, TerrainSurfaceScene};
use qualia_core_db::q42::m64_matter::{M64PayloadInterval, M64ReadLimits, M64Reader};
use qualia_geospatial::raster_m64::decode_raster_from_m64_v3;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{io, sync::atomic::AtomicBool};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemFrameReceipt {
    pub width: usize,
    pub height: usize,
    pub cell_size_mm: u32,
    pub srid: u32,
    pub vertical_datum: u16,
    pub epoch_year: f32,
    pub origin_lat_deg: f64,
    pub origin_lon_deg: f64,
    pub origin_elevation_mm: i32,
    pub rows: String,
    pub edges: String,
}

/// Explicit cold-path profile. Owned bytes cover retained source, sample and
/// licence payloads only; surface compilation has its separately bounded profile.
#[derive(Debug, Clone, Copy)]
pub struct TerrainSourceLimits {
    pub max_source_bytes: usize,
    pub max_cells: usize,
    pub max_chunks: usize,
    pub max_owned_bytes: usize,
}
impl Default for TerrainSourceLimits {
    fn default() -> Self {
        Self {
            max_source_bytes: 16 * 1024 * 1024,
            max_cells: 65_536,
            max_chunks: 63,
            max_owned_bytes: 17 * 1024 * 1024,
        }
    }
}

pub struct TerrainSource {
    frame: DemFrameReceipt,
    samples_mm: Vec<i32>,
    source_m64: Vec<u8>,
    licence: String,
    sha256: [u8; 32],
}

pub(super) fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

impl TerrainSource {
    pub fn admit(bytes: &[u8], licence: &str, limits: TerrainSourceLimits) -> io::Result<Self> {
        if licence.trim().is_empty()
            || licence.len() > 1024
            || limits.max_chunks == 0
            || limits.max_chunks > 63
            || bytes.len() > limits.max_source_bytes
        {
            return Err(invalid("Terrain source/licence/chunk budget refused"));
        }
        let mut scratch = [M64PayloadInterval::default(); 63];
        let reader = M64Reader::parse(
            bytes,
            M64ReadLimits {
                max_file_bytes: limits.max_source_bytes,
                max_chunks: limits.max_chunks,
                max_records: (limits.max_chunks * 4096) as u64,
            },
            &mut scratch,
        )
        .map_err(|e| invalid(format!("Canonical terrain source: {e}")))?;
        let meta = reader
            .georaster()
            .ok_or_else(|| invalid("Terrain requires canonical v3 raster frame"))?;
        let shape = meta
            .shape()
            .map_err(|e| invalid(format!("Terrain source shape: {e:?}")))?;
        let count = shape
            .width
            .checked_mul(shape.height)
            .ok_or_else(|| invalid("Terrain cells overflow"))?;
        let owned = count
            .checked_mul(4)
            .and_then(|n| n.checked_add(bytes.len()))
            .and_then(|n| n.checked_add(licence.len()))
            .ok_or_else(|| invalid("Terrain owned bytes overflow"))?;
        if count == 0
            || count > limits.max_cells
            || owned > limits.max_owned_bytes
            || meta.srid == 0
            || meta.vertical_datum == 0
        {
            return Err(invalid(
                "Terrain source cell/frame/owned-payload budget refused",
            ));
        }
        let mut samples_mm = vec![0; count];
        let (_, geo) = decode_raster_from_m64_v3(&reader, &AtomicBool::new(false), &mut samples_mm)
            .map_err(|e| invalid(format!("Canonical terrain recovery: {e:?}")))?;
        if meta
            .nodata_sentinel_mm
            .is_some_and(|v| samples_mm.contains(&v))
        {
            return Err(invalid(
                "Terrain no-data requires an explicit mask policy; no height invented",
            ));
        }
        Ok(Self {
            frame: DemFrameReceipt {
                width: shape.width,
                height: shape.height,
                cell_size_mm: meta.cell_size_mm,
                srid: geo.srid,
                vertical_datum: geo.vertical_datum,
                epoch_year: geo.epoch_year,
                origin_lat_deg: geo.origin_lat_deg,
                origin_lon_deg: geo.origin_lon_deg,
                origin_elevation_mm: geo.origin_elevation_mm,
                rows: format!("{:?}", meta.rows),
                edges: format!("{:?}", meta.edges),
            },
            samples_mm,
            source_m64: bytes.to_vec(),
            licence: licence.to_owned(),
            sha256: Sha256::digest(bytes).into(),
        })
    }
    pub fn frame(&self) -> &DemFrameReceipt {
        &self.frame
    }
    pub fn samples_mm(&self) -> &[i32] {
        &self.samples_mm
    }
    pub fn source_m64(&self) -> &[u8] {
        &self.source_m64
    }
    pub fn source_sha256(&self) -> [u8; 32] {
        self.sha256
    }
    pub fn licence_declaration(&self) -> &str {
        &self.licence
    }
    pub fn renderable(self) -> io::Result<TerrainSurfaceScene> {
        let bytes = surface::compile_surface(
            &self.samples_mm,
            &self.frame,
            &self.source_m64,
            &self.licence,
        )?;
        TerrainSurfaceScene::from_compiled(self, bytes)
    }
}

/// Compatibility seam for the existing compiler. A mismatched sample/frame
/// projection cannot be packaged with another M64's provenance.
pub fn compile_bound_surface(
    samples: &[i32],
    frame: &DemFrameReceipt,
    source: &[u8],
    licence: &str,
) -> io::Result<Vec<u8>> {
    let admitted = TerrainSource::admit(source, licence, TerrainSourceLimits::default())?;
    if admitted.frame() != frame || admitted.samples_mm() != samples {
        return Err(invalid(
            "Terrain frame/samples differ from canonical source",
        ));
    }
    Ok(admitted.renderable()?.into_bytes())
}
