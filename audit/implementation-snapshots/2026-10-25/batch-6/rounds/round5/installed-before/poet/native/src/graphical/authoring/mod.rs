//! Native authoring intents over the canonical source session, never a second document.
use crate::graphical::preview::SourceRef;
pub(crate) mod controls;
pub(crate) mod package;
pub(crate) mod source;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct PartView {
    pub id: String,
    pub parent: Option<String>,
    pub semantic_id: u64,
    pub translation: [f64; 3],
    pub rotation: [f64; 3],
    pub scale: [f64; 3],
    pub material_id: u64,
    pub base_color: [f64; 4],
    pub roughness: f64,
    pub metallic: f64,
    pub vertices: usize,
    pub triangles: usize,
}

#[derive(Clone, Debug)]
pub struct AuthoringView {
    pub source: SourceRef,
    pub parts: Vec<PartView>,
    pub parameters: Vec<(String, f64)>,
}

#[derive(Clone, Debug)]
pub enum AuthoringEdit {
    Transform {
        id: String,
        translation: [f64; 3],
        rotation: [f64; 3],
        scale: [f64; 3],
    },
    Material {
        material_id: u64,
        base_color: [f64; 4],
        roughness: f64,
        metallic: f64,
    },
    Parameter {
        key: String,
        value: f64,
    },
}
