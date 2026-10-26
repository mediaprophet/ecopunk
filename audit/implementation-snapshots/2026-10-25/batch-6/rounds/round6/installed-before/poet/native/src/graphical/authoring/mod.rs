//! Native authoring intents over the canonical source session, never a second document.
use crate::graphical::preview::SourceRef;
pub(crate) mod controls;
pub(crate) mod package;
pub(crate) mod source;
mod assembly_source;
mod geometry_source;
mod structure_controls;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod structure_tests;

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
    /// Changes containment while retaining the exact parent-relative TRS values.
    Reparent { id: String, parent: Option<String> },
    RemoveLeaf { id: String },
    ReplaceGeometry { id: String, geometry: GeometryRecipe },
    /// Creates an unattached identity-TRS part, copying only the template material.
    AddGeometry { id: String, semantic_id: u64, material_from: String, geometry: GeometryRecipe },
    /// Duplicates this part's complete material record under an unused identifier.
    UniqueMaterial { id: String, material_id: u64 },
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum GeometryRecipe {
    Sphere { radius: f64, lat_segments: u32, lon_segments: u32 },
    Torus { major_radius: f64, minor_radius: f64, major_segments: u32, minor_segments: u32 },
    CircularSweep { radius: f64, length: f64, segments: u32, caps: bool },
}
impl GeometryRecipe {
    pub(crate) fn validate(&self) -> Result<(), String> {
        geometry_source::expression(self).map(|_| ())
    }
}
