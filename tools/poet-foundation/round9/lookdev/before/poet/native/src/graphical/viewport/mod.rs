//! Retained native Portal viewport. Derived state only; SourceSession owns edits.
mod camera;
mod highlight;
mod picking;
mod renderer;
pub use camera::{apply_gesture, camera_state, frame_bounds, ViewportGesture};
pub use picking::PartSelection;
pub use renderer::{RetainedViewport, ViewportBudget, ViewportCounters};

/// Coordinates refer to the square native RGBA attachment, after UI letterbox mapping.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ViewportCommand {
    Redraw,
    FrameAll,
    FramePart { key: String },
    Orbit { dx: f32, dy: f32 },
    Pan { dx: f32, dy: f32 },
    Zoom { steps: f32 },
    Pick { frame_id: u64, x: f32, y: f32 },
    ClearSelection,
}

#[cfg(test)]
mod tests;
