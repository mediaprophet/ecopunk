//! Additive native Poet Tool-Chest/Manifold source authoring surface.
pub mod checkpoints;
pub mod controller;
pub mod cooperation;
mod cooperative_controls;
pub mod cooperative_workflow;
pub mod draft_buffer;
pub mod entry;
pub mod geometry_host;
mod jobs;
mod material_controls;
mod material_edit;
pub mod materials;
pub mod mcp;
pub mod native_entry;
pub mod native_project;
pub mod package;
mod package_batch_controls;
mod package_controls;
pub mod preview;
pub mod session_host;
pub mod shared_host;
mod ui;

use controller::EditorHost;

/// Main-thread Aether/Blitz native entry. Host grants are supplied by the caller.
/// Closing the window stops queued admission, then joins the one native worker.
pub fn launch(host: Box<dyn EditorHost>, initial_draft: String) -> Result<(), String> {
    let initial = host.snapshot()?;
    let (worker, host) = jobs::Worker::start(host)?;
    let attributes = dioxus_native::WindowAttributes::default()
        .with_title("Poet · Studio")
        .with_surface_size(dioxus_native::LogicalSize::new(1440, 900));
    dioxus_native::launch_cfg_with_props(
        ui::NativeStudio,
        ui::NativeStudioProps {
            host,
            initial,
            initial_draft,
            draft_buffer: None,
            shared_busy: None,
        },
        Vec::new(),
        vec![Box::new(attributes)],
    );
    worker.finish()
}
