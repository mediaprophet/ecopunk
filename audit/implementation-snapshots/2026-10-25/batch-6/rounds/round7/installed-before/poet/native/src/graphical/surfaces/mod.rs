//! Native Poet surface projection. MAT4, codecs, HMC keys and GPU remain Core-owned.
pub mod binding;
pub mod draft;
pub mod resources;
pub mod uv;
pub mod wire;
pub use binding::{TextureBindingDraft, TextureRole};
pub use draft::SurfaceDraft;
pub use resources::{SurfaceTextureLimits, TextureResources};
pub use uv::UvTransformDraft;

#[cfg(test)]
mod pixels;
#[cfg(test)]
mod tests;
