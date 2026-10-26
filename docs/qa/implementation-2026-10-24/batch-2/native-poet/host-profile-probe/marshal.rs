use qualia_core_db::render::assets::Mesh;
use vibe::Value;
#[derive(Clone, Copy)]
pub struct PreviewLimits {
    pub side: u32,
    pub vertices: usize,
    pub indices: usize,
}

impl Default for PreviewLimits {
    fn default() -> Self {
        Self {
            side: 512,
            vertices: 200_000,
            indices: 1_200_000,
        }
    }
}

/// Existing wave27 extrusion returns nested triples; wave29 swung surfaces
/// return flat streams. Admit both actual public result shapes without a codec.
fn stream_values<'a>(values: &'a [Value], maximum: usize) -> Result<Vec<&'a Value>, String> {
    if matches!(values.first(), Some(Value::List(_))) {
        let count = values
            .len()
            .checked_mul(3)
            .ok_or("preview result size overflow")?;
        if count > maximum {
            return Err("geometry stream exceeds preview budget".into());
        }
        let mut flat = Vec::with_capacity(count);
        for row in values {
            let Value::List(row) = row else {
                return Err("mixed geometry stream shapes".into());
            };
            if row.len() != 3 {
                return Err("geometry rows must contain exactly three values".into());
            }
            flat.extend(row.iter());
        }
        Ok(flat)
    } else {
        if values.len() > maximum || values.len() % 3 != 0 {
            return Err("invalid or oversized flat geometry stream".into());
        }
        Ok(values.iter().collect())
    }
}

/// Marshals actual existing ComputationalGeometry invoke output only.
pub fn mesh_from_invoke(value: &Value, limits: PreviewLimits) -> Result<Mesh, String> {
    let Value::Record(record) = value else {
        return Err("geometry result must be a record".into());
    };
    let Some(Value::List(vertices)) = record.get("vertices") else {
        return Err("geometry result lacks vertices".into());
    };
    let Some(Value::List(triangles)) = record.get("triangles") else {
        return Err("geometry result lacks triangles".into());
    };
    let max_vertex_scalars = limits
        .vertices
        .checked_mul(3)
        .ok_or("preview vertex budget overflow")?;
    let vertices = stream_values(vertices, max_vertex_scalars)?;
    let triangles = stream_values(triangles, limits.indices)?;
    if vertices.is_empty()
        || vertices.len() % 3 != 0
        || vertices.len() / 3 > limits.vertices
        || triangles.is_empty()
        || triangles.len() % 3 != 0
        || triangles.len() > limits.indices
    {
        return Err(
            "geometry result exceeds declared preview bounds or has invalid topology".into(),
        );
    }
    let numbers: Result<Vec<f32>, String> = vertices
        .into_iter()
        .map(|value| {
            let value = match value {
                Value::F64(v) => *v,
                Value::I64(v) => *v as f64,
                Value::U64(v) => *v as f64,
                _ => return Err("vertex is not numeric".into()),
            };
            let result = value as f32;
            if !result.is_finite() {
                return Err("vertex is not finite f32".into());
            }
            Ok(result)
        })
        .collect();
    let positions: Vec<[f32; 3]> = numbers?
        .chunks_exact(3)
        .map(|v| [v[0], v[1], v[2]])
        .collect();
    let indices: Result<Vec<u32>, String> = triangles
        .into_iter()
        .map(|value| {
            let value = match value {
                Value::U64(v) => *v,
                Value::I64(v) if *v >= 0 => *v as u64,
                _ => return Err("index is not an unsigned integer".into()),
            };
            if value >= positions.len() as u64 {
                return Err("geometry index is out of bounds".into());
            }
            Ok(value as u32)
        })
        .collect();
    let triangles = indices?
        .chunks_exact(3)
        .map(|v| [v[0], v[1], v[2]])
        .collect();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for position in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(position[axis]);
            max[axis] = max[axis].max(position[axis]);
        }
    }
    Ok(Mesh {
        positions,
        triangles,
        min,
        max,
    })
}

