//! Detached source-preserving collar projection. Interface frozen with tree_forks.
//! Implementation modules are staged separately; no successful delivery claim.
mod attributes;
mod build;
mod seed;
#[cfg(test)]
mod tests;
use crate::scene_builder_10d::SubmeshBuilder;
pub use build::build;
use qualia_core_db::specialized_libs::computational_geometry::tapered_branch_field::TaperedFieldSegment;

#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct VertexSource {
    /// None is an explicit artificial cut-cap construction point; its weights
    /// must be zero and no surviving collar face may depend on it.
    pub original_wood_vertices: Option<[u32; 3]>,
    pub barycentric: [f64; 3],
}
pub struct CollarPiece {
    pub mesh: SubmeshBuilder,
    /// Dense geometric IDs, including cap-rim attribute duplicates if present.
    pub topology_aliases: Vec<u32>,
    pub vertex_sources: Vec<VertexSource>,
    /// Source-strip coordinate: <=0 fixed guard/cut band; >=1 full smooth
    /// support. Derived by exact triangle interpolation, not a guessed plane.
    pub fade_coordinates: Vec<f64>,
    pub edge_child_key: String,
    /// Both authored endpoint keys for an interior edge; one may lie outside
    /// the coupled region for a terminal stub. These are source keys, not IDs
    /// manufactured by geometric proximity.
    pub endpoint_keys: [String; 2],
}
pub struct CollarPort {
    pub piece: usize,
    pub edge_child_key: String,
    pub joint_key: String,
    pub cut_fraction: f64,
    /// Local render vertex indices, cyclic and same-count corresponding order.
    /// Preserve source-triangle diagonal intersections as well as wall edges.
    pub cut_vertices: Vec<u32>,
    pub guard_vertices: Vec<u32>,
    /// Local triangle indices of ONLY the artificial outer CUT cap. All must
    /// survive seed union unchanged and are removed from the delivered collar.
    /// Original joint caps are ordinary seed faces, not listed here.
    pub cut_cap_faces: Vec<u32>,
    /// Unit vector from region interior toward retained external branch.
    pub outward_axis: [f64; 3],
    pub guard_center: [f64; 3],
    /// Positive inward transition width. At/beyond guard plane displacement
    /// is identically zero; at this distance inward the weight reaches one.
    pub transition_length: f64,
}
pub struct CollarInput<'a> {
    pub pieces: &'a [CollarPiece],
    pub ports: &'a [CollarPort],
    pub field_segments: &'a [TaperedFieldSegment],
    /// Explicit connected authored junction/cluster membership. This is the
    /// allowed blend region; no outside/unrelated part participates.
    pub joint_ids: &'a [String],
}
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CollarControls {
    pub blend_m: f64,
    pub projection_tolerance_m: f64,
    pub gradient_step_m: f64,
    pub max_projection_step_m: f64,
    pub max_displacement_m: f64,
    pub projection_iterations: usize,
    pub refinement_rounds: usize,
    /// Discrete tessellation criterion in the full-support interior, not a
    /// claim that a finite triangle mesh is an analytic C1 surface.
    pub max_dihedral_radians: f64,
    pub min_field_normal_alignment: f64,
    pub minimum_triangle_area_sq_m4: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct CollarLimits {
    pub pieces: usize,
    pub ports: usize,
    pub ring_vertices: usize,
    pub input_faces: usize,
    pub vertices: usize,
    pub faces: usize,
    pub fragments_per_face: usize,
    pub bytes: usize,
    pub work: usize,
}
impl Default for CollarLimits {
    fn default() -> Self {
        Self {
            pieces: 256,
            ports: 128,
            ring_vertices: 64,
            input_faces: 8192,
            vertices: 32768,
            faces: 32768,
            fragments_per_face: 256,
            bytes: 256 * 1024 * 1024,
            work: 256_000_000,
        }
    }
}
#[derive(serde::Serialize)]
pub struct PortMapping {
    /// Same input port order, then identical input cyclic ring order.
    pub cut_topology: Vec<u32>,
    pub guard_topology: Vec<u32>,
}
pub struct CollarMesh {
    pub mesh: SubmeshBuilder,
    pub topology_positions: Vec<[f64; 3]>,
    pub topology_triangles: Vec<[u32; 3]>,
    pub render_topology_vertices: Vec<u32>,
    pub ports: Vec<PortMapping>,
    pub vertex_lineage: Vec<serde_json::Value>,
    pub receipt: serde_json::Value,
    /// Compact source-face bindings referenced by vertex_lineage records.
    pub face_bindings: Vec<serde_json::Value>,
    pub work_used: usize,
    pub retained_bytes: usize,
}
fn core_controls(
    c: CollarControls,
) -> qualia_core_db::specialized_libs::computational_geometry::smooth_junction::Controls {
    qualia_core_db::specialized_libs::computational_geometry::smooth_junction::Controls {
        blend: c.blend_m,
        tolerance: c.projection_tolerance_m,
        gradient_step: c.gradient_step_m,
        max_step: c.max_projection_step_m,
        max_displacement: c.max_displacement_m,
        iterations: c.projection_iterations,
        refinements: c.refinement_rounds,
        max_dihedral: c.max_dihedral_radians,
        min_normal_alignment: c.min_field_normal_alignment,
        min_area_sq: c.minimum_triangle_area_sq_m4,
    }
}
pub fn validate_controls(c: CollarControls) -> Result<(), String> {
    qualia_core_db::specialized_libs::computational_geometry::smooth_junction::validate_controls(
        core_controls(c),
    )
    .map_err(|e| format!("Smooth collar controls: {e:?}"))
}
