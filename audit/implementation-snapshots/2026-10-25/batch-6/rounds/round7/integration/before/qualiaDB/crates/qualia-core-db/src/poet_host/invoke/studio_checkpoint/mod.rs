//! Cold, bounded native HMC source checkpoints for the existing Studio store.
//!
//! This is not an embodiment-completeness claim, rights store, or interpreter
//! snapshot. Callers authorize capture, write and activation through the native
//! owner. Opening bytes never evaluates Vibe or imports grants/principals.
mod codec;
mod dependencies;
mod graph;
#[cfg(not(target_arch = "wasm32"))]
mod native_io;
mod resources;
mod snapshot;
mod texture_union;
pub use codec::{decode, encode};
#[cfg(not(target_arch = "wasm32"))]
pub use native_io::{load, save, ExpectedFile, SaveReceipt};
pub use resources::{ResourceData, ResourceEdit, ResourceSummary};
pub use snapshot::{activate, capture, Checkpoint, CheckpointLimits};

#[cfg(test)]
mod resource_tests;
#[cfg(test)]
mod tests;
