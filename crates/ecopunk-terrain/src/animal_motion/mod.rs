//! Explicit authored study motion on the existing named r3 rig and Core SKL1.
//! Source/rig/biological authority and visual acceptance remain separate.
mod binding;
mod compile;
mod export;
mod runtime;
mod validation;
pub use compile::{clip_id, compile, CompiledMotion};
pub use export::{export_frame_10d, PoseExportLimits};
pub use runtime::{ContactReceipt, MotionClipDescriptor, MotionError, MotionFrame, MotionPlayer};
use serde::{Deserialize, Serialize};
pub const SCHEMA: &str = "ecopunk.authored-animal-motion/v1";
pub const FRAME: &str = "authored-model-metres; +Y up; +Z front; rest-local wxyz joint deltas";
pub const STUDY_EDIT: &str = include_str!("study.vibe");
pub const MAX_CONTACTS: usize = 8;
#[derive(Clone, Copy, Debug)]
pub struct MotionLimits {
    pub max_clips: usize,
    pub max_tracks: usize,
    pub max_keys: usize,
    pub cold_bytes: usize,
    pub max_vertices: usize,
}
impl Default for MotionLimits {
    fn default() -> Self {
        Self {
            max_clips: 8,
            max_tracks: 512,
            max_keys: 4000,
            cold_bytes: 8 * 1024 * 1024,
            max_vertices: 262_144,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MotionSource {
    pub schema: String,
    pub status: String,
    pub frame: String,
    pub units: String,
    pub clips: Vec<ClipSource>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClipSource {
    pub id: String,
    pub duration_s: f64,
    pub looping: bool,
    pub tracks: Vec<TrackSource>,
    pub contacts: Vec<ContactSource>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrackSource {
    pub joint_id: String,
    pub max_delta_angle_rad: f64,
    pub keys: Vec<KeySource>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeySource {
    pub time_s: f64,
    pub rotation_wxyz: [f64; 4],
    pub translation_m: [f64; 3],
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContactSource {
    pub chain: [String; 3],
    pub start_s: f64,
    pub end_s: f64,
    pub attachment_local_m: [f64; 3],
    pub target_model_m: [f64; 3],
    pub pole_model_m: [f64; 3],
    pub min_knee_angle_rad: f64,
    pub max_knee_angle_rad: f64,
    pub tolerance_m: f64,
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod endpoint_tests;
