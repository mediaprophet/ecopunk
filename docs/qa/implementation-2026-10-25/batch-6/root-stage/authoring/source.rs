//! Bounded source construction; the canonical parser/checker/evaluator executes edits once.
use super::{AuthoringEdit, AuthoringView, PartView};
use crate::graphical::preview::SourceRef;
use std::collections::BTreeMap;
use vibe::Value;

pub(crate) fn number(v: &Value) -> Result<f64, String> {
    let n = match v { Value::F64(n) => *n, Value::I64(n) => *n as f64, Value::U64(n) => *n as f64,
        _ => return Err("Expected numeric authored value".into()) };
    if !n.is_finite() { return Err("Authored value must be finite".into()); }
    Ok(n)
}
fn record(v: &Value) -> Result<&BTreeMap<String, Value>, String> {
    match v { Value::Record(r) => Ok(r), _ => Err("Expected authored record".into()) }
}
fn field<'a>(r: &'a BTreeMap<String, Value>, k: &str) -> Result<&'a Value, String> {
    r.get(k).ok_or_else(|| format!("Authored record lacks {k}"))
}
fn integer(v: &Value) -> Result<u64, String> {
    match v { Value::U64(n) if *n > 0 => Ok(*n), Value::I64(n) if *n > 0 => Ok(*n as u64),
        _ => Err("Expected positive authored integer".into()) }
}
fn text(v: &Value) -> Result<&str, String> {
    match v { Value::String(s) => Ok(s), _ => Err("Expected authored string".into()) }
}
fn vector<const N: usize>(v: &Value) -> Result<[f64; N], String> {
    let Value::List(xs) = v else { return Err("Expected authored vector".into()); };
    if xs.len() != N { return Err("Wrong authored vector dimension".into()); }
    let mut result = [0.0; N];
    for (slot, value) in result.iter_mut().zip(xs) { *slot = number(value)?; }
    Ok(result)
}
pub(crate) fn view(source: SourceRef, value: &Value, parameters: Option<&Value>) -> Result<AuthoringView, String> {
    let Value::List(parts) = value else { return Err("Authoring controls require mutable poet_parts".into()); };
    if parts.is_empty() || parts.len() > 2048 { return Err("Authoring part count outside host bounds".into()); }
    let mut result = Vec::with_capacity(parts.len());
    for part in parts {
        let p = record(part)?;
        let m = record(field(p, "material")?)?;
        let g = record(field(p, "geometry")?)?;
        let count = |k: &str| -> usize { match g.get(k) { Some(Value::List(xs)) if matches!(xs.first(),Some(Value::List(_))) => xs.len(), Some(Value::List(xs))=>xs.len()/3, _ => 0 } };
        result.push(PartView {
            id: text(field(p, "id")?)?.into(),
            parent: match p.get("parent") { None | Some(Value::Null) => None, Some(v) => Some(text(v)?.into()) },
            semantic_id: integer(field(p, "semantic_id")?)?,
            translation: vector(field(p, "translation")?)?, rotation: vector(field(p, "rotation")?)?,
            scale: vector(field(p, "scale")?)?, material_id: integer(field(m, "id")?)?,
            base_color: vector(field(m, "base_color")?)?, roughness: number(field(m, "roughness")?)?,
            metallic: number(field(m, "metallic")?)?, vertices: count("vertices"), triangles: count("triangles"),
        });
    }
    let mut params = Vec::new();
    if let Some(value) = parameters {
        let p = record(value)?;
        if p.len() > 64 { return Err("Authoring parameter count outside host bounds".into()); }
        for (key, value) in p { if identifier(key) { if let Ok(n) = number(value) { params.push((key.clone(), n)); } } }
    }
    Ok(AuthoringView { source, parts: result, parameters: params })
}
fn identifier(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.bytes().enumerate().all(|(i,b)| b.is_ascii_alphabetic() || b == b'_' || (i > 0 && b.is_ascii_digit()))
}
fn literal(n: f64) -> Result<String, String> {
    if !n.is_finite() || n.abs() > 1.0e9 { return Err("Edit exceeds finite authored numeric range".into()); }
    Ok(format!("{n:.9}"))
}
fn array(xs: &[f64]) -> Result<String, String> {
    Ok(format!("[{}]", xs.iter().map(|n| literal(*n)).collect::<Result<Vec<_>,_>>()?.join(",")))
}
/// Functional list replacement avoids unsupported nested assignment in pinned Vibe 0.1.
/// Only schema-admitted existing fields are copied; no kernel/effect is repeated by transform/material edits.
pub(crate) fn edit_source(parts: &Value, parameters: Option<&Value>, edit: &AuthoringEdit, revision: u64) -> Result<String, String> {
    let Value::List(parts) = parts else { return Err("Missing mutable poet_parts".into()); };
    let function = format!("poet_edit_{revision}");
    let (predicate, updates): (String, BTreeMap<&str, String>) = match edit {
        AuthoringEdit::Parameter { key, value } => {
            if !identifier(key) { return Err("Invalid authoring parameter name".into()); }
            let params = record(parameters.ok_or("This source has no poet_parameters binding")?)?;
            if !params.contains_key(key) { return Err("Unknown authored parameter".into()); }
            let mut fields = Vec::new();
            for (name, current) in params {
                if !identifier(name) { return Err("Unsupported parameter identifier".into()); }
                let next = if name == key { *value } else { number(current)? };
                let encoded = match current {
                    Value::I64(_) | Value::U64(_) => { if next.fract()!=0.0 {return Err("Integer parameter requires a whole number".into());} literal(next)?;format!("{next:.0}") },
                    _ => literal(next)?,
                };
                fields.push(format!("{name}:{encoded}"));
            }
            return Ok(format!("poet_parameters = {{{}}};\npoet_parts = poet_rebuild_parts(poet_parameters);\n{{parts:poet_parts}};", fields.join(",")));
        }
        AuthoringEdit::Transform { id, translation, rotation, scale } => {
            if id.is_empty() || id.len()>128 || !id.bytes().all(|b|b.is_ascii_alphanumeric()||matches!(b,b'_'|b'-'|b'.'|b':'|b'/')) {return Err("Part key is outside the editable identifier profile".into());}
            if scale.iter().any(|v| *v <= 0.0) { return Err("Scale must be positive".into()); }
            if !parts.iter().any(|p| record(p).ok().and_then(|p| p.get("id")).is_some_and(|v| matches!(v,Value::String(s) if s==id))) { return Err("Unknown selected part".into()); }
            (format!("p.id == {}", serde_json::to_string(id).map_err(|e|e.to_string())?),
                BTreeMap::from([("translation", array(translation)?),("rotation",array(rotation)?),("scale",array(scale)?)]))
        }
        AuthoringEdit::Material { material_id, base_color, roughness, metallic } => {
            if *material_id == 0 || base_color.iter().chain([roughness,metallic]).any(|v| !v.is_finite() || !(0.0..=1.0).contains(v)) { return Err("Material factors must be in [0,1]".into()); }
            let selected = parts.iter().find_map(|p| {let p=record(p).ok()?;let m=record(p.get("material")?).ok()?;(integer(m.get("id")?).ok()?==*material_id).then_some(m)}).ok_or("Unknown selected material")?;
            let mut fields = Vec::new();
            for key in selected.keys() {
                if !identifier(key) { return Err("Invalid material source field".into()); }
                let next = match key.as_str() { "base_color"=>array(base_color)?, "roughness"=>literal(*roughness)?, "metallic"=>literal(*metallic)?, _=>format!("p.material.{key}") };
                fields.push(format!("{key}:{next}"));
            }
            (format!("p.material.id == {material_id}"), BTreeMap::from([("material",format!("{{{}}}",fields.join(",")))]))
        }
    };
    // Optional parent must have the same shape in every part copied by the helper.
    let has_parent = parts.iter().any(|p| record(p).is_ok_and(|r| r.contains_key("parent")));
    let mut fields = vec!["id", "semantic_id", "geometry", "material", "translation", "rotation", "scale"];
    if has_parent { fields.push("parent"); }
    let mut copied = Vec::new();
    for key in fields { let expr = updates.get(key).cloned().unwrap_or_else(||if key=="parent"{format!("poet_parent_{revision}(p)")}else{format!("p.{key}")});copied.push(format!("{key}:{expr}")); }
    let parent = if has_parent { format!("fn poet_parent_{revision}(p) {{ if p.contains(\"parent\") {{ return p.parent; }} return null; }}\n") } else {String::new()};
    let source = format!("{parent}fn {function}(p) {{ if {predicate} {{ return {{{}}}; }} return p; }}\npoet_parts = poet_parts.map({function});\n{{parts:poet_parts}};",copied.join(","));
    if source.len() > 8192 { return Err("Authoring edit source exceeds host bound".into()); }
    Ok(source)
}
