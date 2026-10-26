//! Additive native Poet Tool-Chest/Manifold source authoring surface.
pub mod controller;
pub mod entry;
pub mod geometry_host;
pub mod preview;
mod jobs;
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
    dioxus_native::launch_cfg_with_props(ui::NativeStudio, ui::NativeStudioProps {
        host, initial, initial_draft,
    }, Vec::new(), vec![Box::new(attributes)]);
    worker.finish()
}
