//! Typed-field SDK over existing MID2 ranges and payload codecs.
mod readers;
mod resolver;
pub use readers::{read_container_mesh_frames, read_container_morph_targets_into};
pub use resolver::{TypedPayload, TypedPayloadError, TypedPayloadResolver};
