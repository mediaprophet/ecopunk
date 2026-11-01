//! Adoption of retained r3 wallaby landmarks/weights into canonical SKL1.
//! Rest-only authoring derivative; neither anatomical axes nor biological motion
//! are inferred. Native player admission is distinct from visual acceptance.
mod admission;
mod projection;
pub use admission::admit_model;
pub(crate) use projection::{attach, prepare, PreparedRig};
use qualia_core_db::render::skeletal::{MAX_JOINTS, MAX_VERTICES};

/// Explicit policy for the existing player's regenerated runtime tangents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TangentPolicy {
    RefuseStoredTangents,
    /// Retains resource bytes but permits runtime reconstruction from bind/pose
    /// positions, normals and UV0. Does not preserve authored tangent values.
    RegenerateFromRetainedUv,
}

#[derive(Clone, Copy, Debug)]
pub struct RigLimits {
    pub max_joints: usize,
    pub max_vertices: usize,
    pub max_indices: usize,
    pub max_container_bytes: usize,
    /// Additional cold rig projection/readback working set, not whole-scene RSS.
    pub cold_bytes: usize,
    /// Core's resident player also retains its existing hard admission ceiling.
    pub player_bytes: usize,
}
impl Default for RigLimits {
    fn default() -> Self {
        Self {
            max_joints: MAX_JOINTS,
            max_vertices: MAX_VERTICES,
            max_indices: MAX_VERTICES * 6,
            max_container_bytes: 42 * 1024 * 1024,
            cold_bytes: 256 * 1024 * 1024,
            player_bytes: 42 * 1024 * 1024,
        }
    }
}
fn add(a: usize, b: usize) -> Result<usize, String> {
    a.checked_add(b)
        .ok_or_else(|| "Wallaby rig size overflow".into())
}
fn mul(a: usize, b: usize) -> Result<usize, String> {
    a.checked_mul(b)
        .ok_or_else(|| "Wallaby rig size overflow".into())
}
fn filled<T: Clone>(count: usize, value: T) -> Result<Vec<T>, String> {
    let mut out = Vec::new();
    out.try_reserve_exact(count)
        .map_err(|_| "Wallaby rig allocation refused")?;
    out.resize(count, value);
    Ok(out)
}
#[cfg(test)]
mod tests;
