use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use vibe::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DetailRecipe {
    Paint(PaintRecipe),
    Procedural(ProceduralRecipe),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaintRecipe {
    pub version: u32,
    pub side: u32,
    pub base_sha256: String,
    pub layers: Vec<PaintLayer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<TextureEffectStep>,
}
/// Authored post-composite step; disabled parameters remain validated and retained.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextureEffectStep {
    pub enabled: bool,
    pub effect: TextureEffect,
}
/// Bounded real Core filters in encoded RGBA8 space. Grayscale/Invert are full strength.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TextureEffect {
    Grayscale,
    Invert,
    Sepia { intensity: f32 },
}
/// Profile admission shared by source, native controls, and pre-context MCP checks.
pub fn validate_effects(effects: &[TextureEffectStep]) -> Result<(), String> {
    if effects.len() > 8 {
        return Err("Texture effect stack admits at most 8 steps".into());
    }
    for step in effects {
        if let TextureEffect::Sepia { intensity } = step.effect {
            if !unit(intensity) {
                return Err("Texture effect Sepia intensity must be finite 0..1".into());
            }
        }
    }
    Ok(())
}
/// Ordered normal source-over layer in canonical encoded RGBA8 working space.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaintLayer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub strokes: Vec<PaintStroke>,
}
/// Discrete canonical brush dabs in image-pixel coordinates, +Y down.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaintStroke {
    pub size: f32,
    pub colour: [u8; 4],
    pub opacity: f32,
    pub hardness: f32,
    pub points: Vec<[f32; 2]>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProceduralRecipe {
    pub version: u32,
    pub side: u32,
    pub seed: u32,
    pub tint: [f32; 3],
    pub contrast: f32,
    pub relief: f32,
    pub bands: [u32; 2],
    pub band_strength: f32,
    pub roughness: f32,
    pub metallic: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct DetailLimits {
    pub working_bytes: usize,
    pub output_bytes: usize,
    pub pixel_visits: usize,
    pub layers: usize,
    pub strokes: usize,
    pub points: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DetailCost {
    pub working_bytes: usize,
    pub output_bytes: usize,
    pub pixel_visits: usize,
}
impl Default for DetailLimits {
    fn default() -> Self {
        Self {
            working_bytes: 128 * 1024 * 1024,
            output_bytes: 16 * 1024 * 1024,
            pixel_visits: 128 * 1024 * 1024,
            layers: 8,
            strokes: 128,
            points: 4096,
        }
    }
}
fn unit(v: f32) -> bool {
    v.is_finite() && (0.0..=1.0).contains(&v)
}
fn charge(total: &mut usize, n: usize, bound: usize) -> Result<(), String> {
    *total = total
        .checked_add(n)
        .filter(|v| *v <= bound)
        .ok_or("Texture detail work budget exceeded")?;
    Ok(())
}
impl DetailRecipe {
    pub fn side(&self) -> u32 {
        match self {
            Self::Paint(p) => p.side,
            Self::Procedural(p) => p.side,
        }
    }
    pub fn from_value(v: &Value) -> Result<Self, String> {
        fn json(v: &Value, depth: usize, nodes: &mut usize) -> Result<serde_json::Value, String> {
            if depth > 8 || *nodes >= 32768 {
                return Err("Texture recipe source bound exceeded".into());
            }
            *nodes += 1;
            Ok(match v {
                Value::Bool(v) => serde_json::Value::Bool(*v),
                Value::I64(v) => (*v).into(),
                Value::U64(v) => (*v).into(),
                Value::F64(v) => serde_json::Number::from_f64(*v)
                    .ok_or("Nonfinite recipe number")?
                    .into(),
                Value::String(v) if v.len() <= 256 => v.clone().into(),
                Value::List(v) if v.len() <= 4096 => serde_json::Value::Array(
                    v.iter()
                        .map(|v| json(v, depth + 1, nodes))
                        .collect::<Result<_, _>>()?,
                ),
                Value::Record(v) if v.len() <= 32 => serde_json::Value::Object(
                    v.iter()
                        .map(|(k, v)| {
                            if k.len() > 64 {
                                return Err("Texture recipe field bound exceeded".into());
                            }
                            Ok((k.clone(), json(v, depth + 1, nodes)?))
                        })
                        .collect::<Result<_, String>>()?,
                ),
                _ => return Err("Unsupported texture recipe value".into()),
            })
        }
        let result: Self = serde_json::from_value(json(v, 0, &mut 0)?)
            .map_err(|e| format!("Texture detail recipe: {e}"))?;
        result.validate(DetailLimits::default())?;
        Ok(result)
    }
    /// Marshal only this strict schema into pinned Vibe record/list expressions.
    pub fn source_expression(&self) -> Result<String, String> {
        self.validate(DetailLimits::default())?;
        fn expression(v: serde_json::Value) -> Result<String, String> {
            Ok(match v {
                serde_json::Value::Object(r) => format!(
                    "{{{}}}",
                    r.into_iter()
                        .map(|(k, v)| {
                            if !k.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
                                return Err("Invalid texture recipe field".into());
                            }
                            Ok(format!("{k}:{}", expression(v)?))
                        })
                        .collect::<Result<Vec<_>, String>>()?
                        .join(",")
                ),
                serde_json::Value::Array(v) => format!(
                    "[{}]",
                    v.into_iter()
                        .map(expression)
                        .collect::<Result<Vec<_>, _>>()?
                        .join(",")
                ),
                v => serde_json::to_string(&v).map_err(|e| e.to_string())?,
            })
        }
        let source = expression(serde_json::to_value(self).map_err(|e| e.to_string())?)?;
        if source.len() > 48 * 1024 {
            return Err("Texture recipe expression exceeds 48 KiB".into());
        }
        Ok(source)
    }
    pub fn digest(&self) -> Result<[u8; 32], String> {
        Ok(Sha256::digest(self.source_expression()?.as_bytes()).into())
    }
    pub fn validate(&self, l: DetailLimits) -> Result<(), String> {
        self.budget_plan(l).map(|_| ())
    }
    /// Same admission estimate for individual validation and root aggregation.
    pub fn budget_plan(&self, l: DetailLimits) -> Result<DetailCost, String> {
        let side = self.side();
        if !(8..=1024).contains(&side) {
            return Err("Texture detail side must be 8..1024".into());
        }
        let pixels = side as usize * side as usize;
        let mut visits = 0;
        let output_count = match self {
            Self::Paint(p) => {
                if !matches!(p.version, 1 | 2)
                    || (p.version == 1 && !p.effects.is_empty())
                    || p.layers.len() > l.layers
                    || p.base_sha256.len() != 64
                    || !p
                        .base_sha256
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                    || p.base_sha256.bytes().all(|b| b == b'0')
                {
                    return Err("Invalid texture paint version/base/layer count".into());
                }
                validate_effects(&p.effects)?;
                let mut strokes = 0;
                let mut points = 0;
                // These passes also run with zero authored paint layers.
                charge(
                    &mut visits,
                    pixels.checked_mul(4).ok_or("Detail work overflow")?,
                    l.pixel_visits,
                )?;
                for layer in &p.layers {
                    if layer.name.is_empty()
                        || layer.name.len() > 80
                        || layer.name.chars().any(char::is_control)
                        || !unit(layer.opacity)
                    {
                        return Err("Invalid texture paint layer".into());
                    }
                    charge(&mut strokes, layer.strokes.len(), l.strokes)?;
                    // Initialization/composite passes including hidden layers.
                    charge(
                        &mut visits,
                        pixels.checked_mul(3).ok_or("Detail work overflow")?,
                        l.pixel_visits,
                    )?;
                    for s in &layer.strokes {
                        if !s.size.is_finite()
                            || !(1.0..=side as f32 * 2.0).contains(&s.size)
                            || !unit(s.opacity)
                            || !unit(s.hardness)
                            || s.colour[3] != 255
                            || s.points.is_empty()
                            || s.points
                                .iter()
                                .flatten()
                                .any(|v| !v.is_finite() || !(0.0..side as f32).contains(v))
                        {
                            return Err("Invalid opaque discrete paint stroke".into());
                        }
                        charge(&mut points, s.points.len(), l.points)?;
                        charge(
                            &mut visits,
                            pixels
                                .checked_mul(s.points.len())
                                .ok_or("Detail work overflow")?,
                            l.pixel_visits,
                        )?;
                    }
                }
                if !p.effects.is_empty() {
                    // Initialization/copy plus every authored step, including disabled ones.
                    charge(
                        &mut visits,
                        pixels.checked_mul(p.effects.len() + 2).ok_or("Detail work overflow")?,
                        l.pixel_visits,
                    )?;
                }
                1usize
            }
            Self::Procedural(p) => {
                if p.version != 1
                    || !p
                        .tint
                        .iter()
                        .copied()
                        .chain([
                            p.contrast,
                            p.relief,
                            p.band_strength,
                            p.roughness,
                            p.metallic,
                        ])
                        .all(unit)
                    || p.bands.iter().any(|v| *v > 128)
                {
                    return Err("Invalid procedural texture detail recipe".into());
                }
                charge(
                    &mut visits,
                    pixels.checked_mul(32).ok_or("Detail work overflow")?,
                    l.pixel_visits,
                )?;
                3
            }
        };
        let rgba = pixels * 4;
        let png = rgba + side as usize + 4096;
        // Includes opaque snapshots, canonical composite temporaries, decode copy,
        // encoded outputs, and conservative 8 MiB canonical PNG encoder scratch.
        let working = match self {
            Self::Paint(p) => {
                // One post-composite RGBA layer in addition to legacy temporaries.
                rgba.checked_mul(p.layers.len() + 6 + usize::from(!p.effects.is_empty()))
            }
            Self::Procedural(_) => rgba.checked_mul(4),
        }
        .and_then(|n| n.checked_add(png.checked_mul(output_count)?))
        .and_then(|n| n.checked_add(8 * 1024 * 1024))
        .ok_or("Detail memory overflow")?;
        if working > l.working_bytes || png * output_count > l.output_bytes {
            return Err("Texture detail byte budget exceeded".into());
        }
        Ok(DetailCost {
            working_bytes: working,
            output_bytes: png * output_count,
            pixel_visits: visits,
        })
    }
}
