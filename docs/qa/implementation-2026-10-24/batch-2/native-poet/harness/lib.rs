//! Offline native renderer dependency probe. This is not a launched application.
pub fn native_launch_entry(app: fn() -> dioxus::prelude::Element) {
    dioxus_native::launch(app);
}
