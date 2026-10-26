//! Safe assembly mutations retain local TRS and all unrelated record fields.
use super::{AuthoringEdit, source::{field, identifier, integer, record, text}};
use std::collections::{BTreeMap, BTreeSet};
use vibe::Value;
type Record = BTreeMap<String, Value>;

fn quoted(s: &str) -> Result<String, String> { serde_json::to_string(s).map_err(|e| e.to_string()) }
fn part_key(id: &str) -> Result<(), String> {
    if id.is_empty() || id.len() > 128 || !id.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b,b'_'|b'-'|b'.'|b':'|b'/')) {
        return Err("Part key is outside the editable identifier profile".into());
    }
    Ok(())
}
/// Only copy fields representable by canonical member access. Never silently discard extensions.
fn copied(r: &Record, prefix: &str, updates: BTreeMap<&str, String>) -> Result<String, String> {
    let mut fields = Vec::new();
    for key in r.keys() {
        if !identifier(key) { return Err(format!("Unsupported source field {key}; edit refused to preserve authored data")); }
        fields.push(format!("{key}:{}", updates.get(key.as_str()).cloned().unwrap_or_else(|| format!("{prefix}.{key}"))));
    }
    for (key, value) in updates {
        if !r.contains_key(key) { fields.push(format!("{key}:{value}")); }
    }
    Ok(format!("{{{}}}", fields.join(",")))
}
pub(super) struct Inventory<'a> { parts: Vec<&'a Record>, ids: BTreeMap<String, usize> }
impl<'a> Inventory<'a> {
    pub(super) fn new(value: &'a Value) -> Result<Self, String> {
        let Value::List(values) = value else { return Err("Missing mutable poet_parts".into()); };
        if values.is_empty() || values.len() > 256 { return Err("Authoring part count outside native assembly bounds".into()); }
        let mut parts = Vec::new();
        let mut ids = BTreeMap::new();
        let mut semantics = BTreeSet::new();
        for (i, value) in values.iter().enumerate() {
            let part = record(value)?;
            let id = text(field(part,"id")?)?;
            part_key(id)?;
            if ids.insert(id.into(), i).is_some() { return Err("Duplicate assembly part identifier".into()); }
            if !semantics.insert(integer(field(part,"semantic_id")?)?) { return Err("Duplicate assembly semantic identifier".into()); }
            parts.push(part);
        }
        let result = Self { parts, ids };
        for part in &result.parts {
            let id = text(field(part,"id")?)?;
            let mut parent = result.parent(id)?;
            let mut seen = BTreeSet::from([id.to_owned()]);
            while let Some(next) = parent {
                if !seen.insert(next.to_owned()) { return Err("Assembly containment cycle refused".into()); }
                parent = result.parent(next)?;
            }
        }
        Ok(result)
    }
    fn selected(&self, id: &str) -> Result<&'a Record, String> {
        self.ids.get(id).map(|i| self.parts[*i]).ok_or_else(|| "Unknown selected part or parent".into())
    }
    fn parent(&self, id: &str) -> Result<Option<&'a str>, String> {
        match self.selected(id)?.get("parent") {
            None | Some(Value::Null) => Ok(None),
            Some(v) => { let id = text(v)?; self.selected(id)?; Ok(Some(id)) }
        }
    }
    fn mapped(&self, replacements: Vec<(&str, String)>) -> Result<String, String> {
        // Share identical schema expressions instead of nesting once per part.
        let mut groups: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for (id, replacement) in replacements { groups.entry(replacement).or_default().push(id); }
        let mut expression = "p".to_string();
        for (replacement, ids) in groups.into_iter().rev() {
            let predicate = if ids.len() == 1 { format!("p.id == {}",quoted(ids[0])?) } else {
                format!("[{}].contains(p.id)",ids.into_iter().map(quoted).collect::<Result<Vec<_>,_>>()?.join(","))
            };
            expression = format!("if {predicate} {{ {replacement} }} else {{ {expression} }}");
        }
        Ok(format!("poet_parts = poet_parts.map(|p| {expression});\n({{parts:poet_parts}});"))
    }
    pub(super) fn replace(&self, id: &str, updates: BTreeMap<&str, String>) -> Result<String, String> {
        self.mapped(vec![(id, copied(self.selected(id)?, "p", updates)?)])
    }
    pub(super) fn material(&self, id: u64, updates: BTreeMap<&str, String>) -> Result<String, String> {
        let mut replacements = Vec::new();
        for part in &self.parts {
            let material = record(field(part,"material")?)?;
            if integer(field(material,"id")?)? == id {
                let next = copied(material,"p.material",updates.clone())?;
                replacements.push((text(field(part,"id")?)?, copied(part,"p",BTreeMap::from([("material",next)]))?));
            }
        }
        if replacements.is_empty() { return Err("Unknown selected material".into()); }
        if replacements.iter().all(|(_,next)| next == &replacements[0].1) {
            // Canonical equality preserves numeric variants; the same admitted
            // identifier may be authored as signed or unsigned source values.
            let predicate = if id <= i64::MAX as u64 {
                format!("(p.material.id == {id} || p.material.id == {id}u64)")
            } else { format!("p.material.id == {id}u64") };
            return Ok(format!("poet_parts = poet_parts.map(|p| if {predicate} {{ {} }} else {{ p }});\n({{parts:poet_parts}});", replacements[0].1));
        }
        self.mapped(replacements)
    }
    pub(super) fn structural(&self, edit: &AuthoringEdit) -> Result<String, String> {
        match edit {
            AuthoringEdit::Reparent { id, parent } => {
                self.selected(id)?;
                let mut ancestor = parent.as_deref();
                while let Some(next) = ancestor {
                    if next == id { return Err("Assembly containment cycle refused".into()); }
                    ancestor = self.parent(next)?;
                }
                let value = parent.as_deref().map(quoted).transpose()?.unwrap_or_else(|| "null".into());
                self.replace(id,BTreeMap::from([("parent",value)]))
            }
            AuthoringEdit::RemoveLeaf { id } => {
                self.selected(id)?;
                if self.parts.len() == 1 { return Err("Removing the final assembly part is refused".into()); }
                for key in self.ids.keys() {
                    if self.parent(key)? == Some(id.as_str()) { return Err("Remove children or detach them before removing this part".into()); }
                }
                Ok(format!("poet_parts = poet_parts.filter(|p| p.id != {});\n({{parts:poet_parts}});", quoted(id)?))
            }
            AuthoringEdit::ReplaceGeometry { id, geometry } => {
                let (requirements, expression) = super::geometry_source::expression(geometry)?;
                Ok(requirements + &self.replace(id,BTreeMap::from([("geometry",expression)]))?)
            }
            AuthoringEdit::AddGeometry { id, semantic_id, material_from, geometry } => {
                part_key(id)?;
                if self.parts.len() >= 256 || self.ids.contains_key(id) { return Err("New part identifier is duplicate or assembly is full".into()); }
                // This bounded control profile uses positive signed-range identifiers.
                if *semantic_id == 0 || *semantic_id > i64::MAX as u64 || self.parts.iter().any(|p| integer(field(p,"semantic_id").expect("inventory field")).ok() == Some(*semantic_id)) {
                    return Err("New semantic identifier must be distinct and fit a positive Vibe 0.1 integer".into());
                }
                self.selected(material_from)?;
                let template = self.ids[material_from];
                let (requirements, expression) = super::geometry_source::expression(geometry)?;
                Ok(format!("{requirements}poet_parts = poet_parts.push({{id:{},semantic_id:{semantic_id},geometry:{expression},material:poet_parts[{template}].material,translation:[0.0,0.0,0.0],rotation:[0.0,0.0,0.0],scale:[1.0,1.0,1.0]}});\n({{parts:poet_parts}});",quoted(id)?))
            }
            AuthoringEdit::UniqueMaterial { id, material_id } => {
                let selected = self.selected(id)?;
                if *material_id == 0 || *material_id > i64::MAX as u64 { return Err("New material identifier must fit a positive Vibe 0.1 integer".into()); }
                for part in &self.parts {
                    if integer(field(record(field(part,"material")?)?,"id")?)? == *material_id { return Err("Material identifier is already used".into()); }
                }
                let material = copied(record(field(selected,"material")?)?,"p.material",BTreeMap::from([("id",material_id.to_string())]))?;
                self.replace(id,BTreeMap::from([("material",material)]))
            }
            _ => Err("Unsupported structural authoring edit".into()),
        }
    }
}
