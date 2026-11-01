//! Shared native/WASM Vibe projection of the admitted source terrain. This
//! delegates canonical derivation/placement and does not replace the graph owner.
use crate::vibe_host::EcopunkVibeHost;
use ecopunk_terrain::terrain_source::{TerrainSource, TerrainSourceLimits, TerrainSurfaceScene};
use std::collections::BTreeMap;
use vibe::{DiagCode, Diagnostic, Span, Value};

impl EcopunkVibeHost {
    /// Prepare without publishing: failed admission/derivation keeps current source.
    pub fn prepare_source_terrain(
        bytes: &[u8],
        licence: &str,
        limits: TerrainSourceLimits,
    ) -> std::io::Result<TerrainSurfaceScene> {
        TerrainSource::admit(bytes, licence, limits)?.renderable()
    }
    pub fn source_terrain(&self) -> Option<&TerrainSurfaceScene> {
        self.source_terrain.as_ref()
    }
    pub fn adopt_source_terrain(&mut self, scene: TerrainSurfaceScene) {
        self.source_terrain = Some(scene);
    }
    pub fn clear_source_terrain(&mut self) {
        self.source_terrain = None;
    }
}

pub(crate) fn sample(
    host: &EcopunkVibeHost,
    args: &Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    let fail = |message: &str| Diagnostic::new(DiagCode::E701, span, message.to_owned());
    let Value::Record(args) = args else {
        return Err(fail("TerrainSource.sample_local requires record arguments"));
    };
    let number = |key: &str| -> Result<f64, Diagnostic> {
        let value = match args.get(key) {
            Some(Value::F64(v)) => *v,
            Some(Value::I64(v)) => *v as f64,
            Some(Value::U64(v)) => *v as f64,
            _ => return Err(fail("TerrainSource coordinates must be explicit numbers")),
        };
        if !value.is_finite() {
            return Err(fail("TerrainSource coordinates must be finite"));
        }
        Ok(value)
    };
    let terrain = host
        .source_terrain()
        .ok_or_else(|| fail("No admitted source terrain"))?;
    let p = terrain
        .sample_local(number("east_m")?, number("south_m")?)
        .map_err(|e| fail(&e.to_string()))?;
    let mut out = BTreeMap::new();
    out.insert("east_m".into(), Value::F64(p.position_m[0]));
    out.insert(
        "height_relative_to_source_origin_m".into(),
        Value::F64(p.position_m[1]),
    );
    out.insert("south_m".into(), Value::F64(p.position_m[2]));
    for (key, v) in ["normal_x", "normal_y", "normal_z"]
        .into_iter()
        .zip(p.geometric_normal)
    {
        out.insert(key.into(), Value::F64(v));
    }
    out.insert("triangle".into(), Value::U64(p.triangle as u64));
    for (key, digest) in [
        ("source_sha256", p.source_sha256),
        ("surface_sha256", p.surface_sha256),
    ] {
        out.insert(
            key.into(),
            Value::String(digest.iter().map(|b| format!("{b:02x}")).collect()),
        );
    }
    out.insert(
        "method".into(),
        Value::String("canonical-decoded-triangle-plane/v1".into()),
    );
    Ok(Value::Record(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::q42::m64_matter::{M64RasterEdges, M64RasterRows};
    use qualia_geospatial::{
        raster_m64::{compile_raster_to_m64_v3, RasterM64Limits, RasterM64Policy},
        ElevationRaster, GeodeticFrame,
    };
    use std::sync::atomic::AtomicBool;
    use vibe::{eval_function, load_program, Env};

    #[test]
    fn actual_vibe_placement_uses_admitted_surface_and_failed_replacement_preserves_it() {
        let mut bytes = vec![0; 300_000];
        let length = compile_raster_to_m64_v3(
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
                rows: M64RasterRows::South,
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
        bytes.truncate(length);
        let program =
            load_program(include_str!("../../../vibe/ecopunk_source_terrain.vibe")).unwrap();
        let mut host = EcopunkVibeHost::new();
        let mut env = Env::default();
        let query = vec![Value::F64(12.5), Value::F64(12.5)];
        assert!(eval_function(
            &program,
            "sample_authored_placement",
            query.clone(),
            &mut host,
            &mut env
        )
        .is_err());
        let scene =
            EcopunkVibeHost::prepare_source_terrain(&bytes, "TEST", TerrainSourceLimits::default())
                .unwrap();
        let digest = scene.surface_sha256();
        host.adopt_source_terrain(scene);
        let result = eval_function(
            &program,
            "sample_authored_placement",
            query.clone(),
            &mut host,
            &mut env,
        )
        .unwrap();
        let Value::Record(result) = result else {
            panic!("Expected placement record");
        };
        let Some(Value::F64(height)) = result.get("height_relative_to_source_origin_m") else {
            panic!("Expected height");
        };
        assert!(height.abs() < 0.001);
        assert!(EcopunkVibeHost::prepare_source_terrain(
            &bytes[..12],
            "TEST",
            TerrainSourceLimits::default()
        )
        .is_err());
        assert_eq!(host.source_terrain().unwrap().surface_sha256(), digest);
        host.clear_source_terrain();
        assert!(eval_function(
            &program,
            "sample_authored_placement",
            query,
            &mut host,
            &mut env
        )
        .is_err());
    }
}
