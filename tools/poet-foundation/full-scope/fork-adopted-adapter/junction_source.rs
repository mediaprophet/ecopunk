//! Staged source-value codec for an opt-in single junction. This module does
//! not submit source or replace geometry: host transaction accounting and
//! stream remapping must be integrated before publication can be enabled.
use super::recipe::{read_recipe, BotanicalLimits, BotanicalRecipe};
use std::collections::BTreeMap;
use vibe::Value;

const SCHEMA: &str = "poet.botanical.single-junction/v1";
const FIELDS: [&str; 10] = ["schema", "graph", "origin_m", "spacing_m", "dimensions", "blend_m", "min_axis_m", "projection_tolerance_m", "max_same_direction_cosine", "min_area_sq_m4"];

#[derive(Clone, Debug)]
pub struct JunctionSource {
    /// Preserve the entire existing Vibe graph value; no alternate project format.
    graph: Value,
    pub origin_m: [f64; 3],
    pub spacing_m: f64,
    pub dimensions: [usize; 3],
    pub blend_m: f64,
    pub min_axis_m: f64,
    pub projection_tolerance_m: f64,
    pub max_same_direction_cosine: f64,
    pub min_area_sq_m4: f64,
}

fn number(v: &Value) -> Result<f64, String> {
    let n = match v { Value::F64(x) => *x, Value::I64(x) if x.unsigned_abs() <= 1u64 << 53 => *x as f64,
        Value::U64(x) if *x <= 1u64 << 53 => *x as f64, _ => return Err("Expected exact finite number".into()) };
    if !n.is_finite() { return Err("Nonfinite number".into()); }
    Ok(n)
}
fn triple(v: &Value) -> Result<[f64; 3], String> {
    let Value::List(x) = v else { return Err("Expected triple".into()); };
    if x.len() != 3 { return Err("Expected triple".into()); }
    Ok([number(&x[0])?, number(&x[1])?, number(&x[2])?])
}
fn dimensions(v: &Value) -> Result<[usize; 3], String> {
    let Value::List(x) = v else { return Err("Expected three dimensions".into()); };
    if x.len() != 3 { return Err("Expected three dimensions".into()); }
    let dim = |v: &Value| match v { Value::U64(n) => usize::try_from(*n).map_err(|_| "Dimension overflow"),
        Value::I64(n) if *n >= 0 => usize::try_from(*n).map_err(|_| "Dimension overflow"), _ => Err("Expected integer dimension") };
    Ok([dim(&x[0])?, dim(&x[1])?, dim(&x[2])?])
}
fn graph_for_junction(value: &Value, limits: BotanicalLimits) -> Result<BotanicalRecipe, String> {
    // Bound collection sizes before existing read_recipe allocates owned vectors.
    // The host must also bound the incoming Value/source bytes before this call.
    let Value::Record(fields) = value else { return Err("Invalid graph value".into()); };
    match fields.get("organs") {
        Some(Value::List(organs)) if organs.len() == 4 && organs.len() <= limits.max_organs => {
            for organ in organs {
                let Value::Record(o) = organ else { return Err("Invalid organ".into()); };
                if !matches!(o.get("key"), Some(Value::String(s)) if !s.is_empty() && s.len() <= 128)
                    || !(matches!(o.get("parent"), Some(Value::Null)) || matches!(o.get("parent"), Some(Value::String(s)) if !s.is_empty() && s.len() <= 128)) {
                    return Err("Organ identity outside bounds".into());
                }
            }
        },
        _ => return Err("Junction needs exactly four bounded adopted organs".into()),
    }
    if !matches!(fields.get("part_id"), Some(Value::String(s)) if !s.is_empty() && s.len() <= 128) { return Err("Part identity outside bounds".into()); }
    match fields.get("attractors_m") {
        Some(Value::List(points)) if !points.is_empty() && points.len() <= limits.max_attractors => {},
        _ => return Err("Attractor list exceeds host limit".into()),
    }
    let graph = read_recipe(Some(value), limits)?.ok_or("Missing graph")?;
    // Current host's v2 evaluator is a chain-only sweep. Do not reinterpret it.
    if graph.version != 1 { return Err("Junction graph currently requires v1 source".into()); }
    if graph.organs.len() != 4 || graph.organs[0].parent.is_some()
        || graph.organs[1].parent.as_deref() != Some(graph.organs[0].key.as_str())
        || graph.organs[2].parent.as_deref() != Some(graph.organs[1].key.as_str())
        || graph.organs[3].parent.as_deref() != Some(graph.organs[1].key.as_str()) {
        return Err("Expected exactly one adopted three-edge junction".into());
    }
    Ok(graph)
}
impl JunctionSource {
    /// The caller provides independent host-selected sample and scalar limits.
    pub fn read(value: &Value, limits: BotanicalLimits, max_samples: usize) -> Result<Self, String> {
        let Value::Record(r) = value else { return Err("Junction source must be a record".into()); };
        if r.len() != FIELDS.len() || FIELDS.iter().any(|f| !r.contains_key(*f)) {
            return Err("Unknown or missing junction source field".into());
        }
        if !matches!(r.get("schema"), Some(Value::String(s)) if s == SCHEMA) { return Err("Unsupported junction schema".into()); }
        let graph = r.get("graph").ok_or("Missing graph")?;
        graph_for_junction(graph, limits)?;
        let get = |k: &str| r.get(k).ok_or_else(|| format!("Missing {k}"));
        let source = Self { graph: graph.clone(), origin_m: triple(get("origin_m")?)?,
            spacing_m: number(get("spacing_m")?)?, dimensions: dimensions(get("dimensions")?)?,
            blend_m: number(get("blend_m")?)?, min_axis_m: number(get("min_axis_m")?)?,
            projection_tolerance_m: number(get("projection_tolerance_m")?)?,
            max_same_direction_cosine: number(get("max_same_direction_cosine")?)?, min_area_sq_m4: number(get("min_area_sq_m4")?)? };
        source.validate(limits, max_samples)?;
        Ok(source)
    }
    pub fn validate(&self, limits: BotanicalLimits, max_samples: usize) -> Result<(), String> {
        graph_for_junction(&self.graph, limits)?;
        let samples = self.dimensions.iter().try_fold(1usize, |n, d| n.checked_mul(*d)).ok_or("Sample count overflow")?;
        if self.dimensions.iter().any(|d| *d < 2) || samples > max_samples
            || self.origin_m.iter().any(|n| !n.is_finite() || n.abs() > 1e6)
            || !self.spacing_m.is_finite() || self.spacing_m <= 0.0 || self.spacing_m > 1e6
            || !self.blend_m.is_finite() || self.blend_m < 0.0 || self.blend_m > 1e6
            || !self.min_axis_m.is_finite() || self.min_axis_m <= 0.0 || self.min_axis_m > 1e6
            || !self.projection_tolerance_m.is_finite() || self.projection_tolerance_m <= 0.0 || self.projection_tolerance_m > 1e6
            || !self.min_area_sq_m4.is_finite() || self.min_area_sq_m4 < 0.0 || self.min_area_sq_m4 > 1e24
            || !self.max_same_direction_cosine.is_finite() || !(0.0..1.0).contains(&self.max_same_direction_cosine) {
            return Err("Junction parameters outside host bounds".into());
        }
        for i in 0..3 {
            let extent = self.origin_m[i] + self.spacing_m * (self.dimensions[i] - 1) as f64;
            if !extent.is_finite() || extent.abs() > 1e6 { return Err("Grid extent outside metre bounds".into()); }
        }
        Ok(())
    }
    /// Edits a copy of the exact graph value. Does not mutate caller's source.
    pub fn with_position(&self, key: &str, position_m: [f64; 3], limits: BotanicalLimits, max_samples: usize) -> Result<Self, String> {
        let mut next = self.clone();
        let Value::Record(graph) = &mut next.graph else { return Err("Invalid graph".into()); };
        let Some(Value::List(organs)) = graph.get_mut("organs") else { return Err("Invalid organs".into()); };
        let mut matched = 0;
        for organ in organs {
            let Value::Record(o) = organ else { return Err("Invalid organ".into()); };
            if matches!(o.get("key"), Some(Value::String(s)) if s == key) {
                o.insert("position_m".into(), Value::List(position_m.iter().copied().map(Value::F64).collect()));
                matched += 1;
            }
        }
        if matched != 1 { return Err("Unknown or duplicate organ key".into()); }
        next.validate(limits, max_samples)?;
        Ok(next)
    }
    pub(crate) fn graph(&self, limits: BotanicalLimits) -> Result<BotanicalRecipe, String> {
        graph_for_junction(&self.graph, limits)
    }
    pub fn to_value(&self, limits: BotanicalLimits, max_samples: usize) -> Result<Value, String> {
        self.validate(limits, max_samples)?;
        let mut r = BTreeMap::new();
        r.insert("schema".into(), Value::String(SCHEMA.into()));
        r.insert("graph".into(), self.graph.clone());
        r.insert("origin_m".into(), Value::List(self.origin_m.iter().copied().map(Value::F64).collect()));
        r.insert("spacing_m".into(), Value::F64(self.spacing_m));
        let dims = self.dimensions.iter().map(|n| u64::try_from(*n).map(Value::U64).map_err(|_| "Dimension overflow")).collect::<Result<Vec<_>, _>>()?;
        r.insert("dimensions".into(), Value::List(dims));
        r.insert("blend_m".into(), Value::F64(self.blend_m));
        r.insert("min_axis_m".into(), Value::F64(self.min_axis_m));
        r.insert("projection_tolerance_m".into(), Value::F64(self.projection_tolerance_m));
        r.insert("max_same_direction_cosine".into(), Value::F64(self.max_same_direction_cosine));
        r.insert("min_area_sq_m4".into(), Value::F64(self.min_area_sq_m4));
        Ok(Value::Record(r))
    }
}
