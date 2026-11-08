//! `EcopunkVibeHost` — `impl vibe::Host` binding `vibe-0.1` capability invocations
//! directly to `ecopunk-terrain`'s zero-heap 26.5 km `RegionalCorridorWorld`,
//! `ContinuumExplorationSession`, 5-scale `botanical_anatomy` / `zoological_anatomy`
//! `.10d` Living Container generators, `botanical_ontology_q42` CURIE callout pins,
//! and `qualia-core-db`'s `.q42` `WasmSliceQ42Reader`!

use std::collections::BTreeMap;

use ecopunk_terrain::botanical_ontology_q42::CANONICAL_ANATOMICAL_PINS;
use ecopunk_terrain::world_continuum::RegionalCorridorWorld;
use ecopunk_terrain::LocalHostQ42Ext;
use vibe::{Diagnostic, Host, LocalHost, Span, Value};

pub struct EcopunkVibeHost {
    pub local: LocalHost,
    pub corridor: RegionalCorridorWorld,
}

impl Default for EcopunkVibeHost {
    fn default() -> Self {
        Self::new()
    }
}

impl EcopunkVibeHost {
    pub fn new() -> Self {
        Self {
            local: LocalHost::default(),
            corridor: RegionalCorridorWorld::new(),
        }
    }

    pub fn with_q42_pack(pack_bytes: &[u8]) -> Self {
        let local = LocalHost::with_q42_pack(pack_bytes);
        Self {
            local,
            corridor: RegionalCorridorWorld::new(),
        }
    }
}

fn f64_arg(args: &Value, key: &str, default: f64) -> f64 {
    match args {
        Value::Record(m) => match map_get_num(m, key) {
            Some(v) => v,
            None => default,
        },
        _ => default,
    }
}

fn map_get_num(m: &BTreeMap<String, Value>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(Value::F64(v)) => Some(*v),
        Some(Value::I64(v)) => Some(*v as f64),
        Some(Value::U64(v)) => Some(*v as f64),
        _ => None,
    }
}

impl Host for EcopunkVibeHost {
    fn graph_query(&mut self, args: &[Value], take: u64, span: Span) -> Result<Value, Diagnostic> {
        self.local.graph_query(args, take, span)
    }

    fn graph_stage(&mut self, term: &Value, span: Span) -> Result<Value, Diagnostic> {
        self.local.graph_stage(term, span)
    }

    fn graph_commit(&mut self, span: Span) -> Result<Value, Diagnostic> {
        self.local.graph_commit(span)
    }

    fn aura_validate(
        &mut self,
        node: &Value,
        shape: &Value,
        span: Span,
    ) -> Result<Value, Diagnostic> {
        self.local.aura_validate(node, shape, span)
    }

    fn pulse_publish(
        &mut self,
        topic: &str,
        payload: &Value,
        span: Span,
    ) -> Result<Value, Diagnostic> {
        self.local.pulse_publish(topic, payload, span)
    }

    fn capability_invoke(
        &mut self,
        id: &str,
        args: &Value,
        span: Span,
    ) -> Result<Value, Diagnostic> {
        match id {
            "WorldContinuum.sample_elevation_normal" => {
                let wx = f64_arg(args, "wx_m", 0.0) as f32;
                let wz = f64_arg(args, "wz_m", 0.0) as f32;
                let (elev, normal, mat) =
                    self.corridor.sample_world_elevation_and_normal_zero_heap(wx, wz);
                let water = self.corridor.sample_water_surface_zero_heap(wx, wz);
                let ceiling = self.corridor.sample_ceiling_overhang_zero_heap(wx, wz);
                let mut rec = BTreeMap::new();
                rec.insert("wx_m".into(), Value::F64(wx as f64));
                rec.insert("wz_m".into(), Value::F64(wz as f64));
                rec.insert("elevation_m".into(), Value::F64(elev as f64));
                rec.insert("normal_x".into(), Value::F64(normal[0] as f64));
                rec.insert("normal_y".into(), Value::F64(normal[1] as f64));
                rec.insert("normal_z".into(), Value::F64(normal[2] as f64));
                rec.insert("surface_material".into(), Value::String(format!("{mat:?}")));
                rec.insert(
                    "water_surface_m".into(),
                    Value::F64(water.unwrap_or(-1000.0) as f64),
                );
                rec.insert("cave_ceiling_m".into(), Value::F64(ceiling as f64));
                Ok(Value::Record(rec))
            }
            "WorldContinuum.locate_study_site" => {
                let wx = f64_arg(args, "wx_m", 0.0) as f32;
                let wz = f64_arg(args, "wz_m", 0.0) as f32;
                let (site_id, dist_m) = self.corridor.locate_nearest_study_site_zero_heap(wx, wz);
                let site_desc = &self.corridor.sites[site_id as usize];
                let mut rec = BTreeMap::new();
                rec.insert("site_index".into(), Value::I64(site_id as i64));
                rec.insert("site_id".into(), Value::String(format!("{site_id:?}")));
                rec.insert("site_name".into(), Value::String(site_desc.name.into()));
                rec.insert("distance_m".into(), Value::F64(dist_m as f64));
                rec.insert("corridor_km".into(), Value::F64(site_desc.corridor_km as f64));
                rec.insert("elevation_ahd_m".into(), Value::F64(site_desc.elevation_ahd_m as f64));
                rec.insert("loupe_available".into(), Value::Bool(dist_m <= 160.0));
                Ok(Value::Record(rec))
            }
            "BiologicalOntology.query_callout_pin" => {
                let scale = (f64_arg(args, "scale", 1.0) as u8).clamp(1, 5);
                let pin_idx = f64_arg(args, "pin_index", 0.0).max(0.0) as usize;
                let scale_pins: Vec<_> = CANONICAL_ANATOMICAL_PINS
                    .iter()
                    .filter(|p| p.scale == scale)
                    .collect();
                let pin = if scale_pins.is_empty() {
                    &CANONICAL_ANATOMICAL_PINS[0]
                } else {
                    scale_pins[pin_idx % scale_pins.len()]
                };
                let mut rec = BTreeMap::new();
                rec.insert("pin_id".into(), Value::String(pin.pin_id.into()));
                rec.insert("scale".into(), Value::I64(pin.scale as i64));
                rec.insert("ontology_curie".into(), Value::String(pin.ontology_curie.into()));
                rec.insert("scientific_term".into(), Value::String(pin.scientific_term.into()));
                rec.insert(
                    "biophysical_formula".into(),
                    Value::String(pin.biophysical_formula.into()),
                );
                rec.insert(
                    "educational_summary".into(),
                    Value::String(pin.educational_summary.into()),
                );
                rec.insert(
                    "indigenous_custodial_context".into(),
                    Value::String(pin.indigenous_custodial_context.into()),
                );
                rec.insert("anchor_x".into(), Value::F64(pin.anchor_local_xyz[0] as f64));
                rec.insert("anchor_y".into(), Value::F64(pin.anchor_local_xyz[1] as f64));
                rec.insert("anchor_z".into(), Value::F64(pin.anchor_local_xyz[2] as f64));
                Ok(Value::Record(rec))
            }
            _ => self.local.capability_invoke(id, args, span),
        }
    }
}
