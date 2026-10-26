//! Version-1 Vibe projection onto the shared Craft assembly domain engine.
//! TRS are local to parent (roots are world), metres/radians, T·Rx·Ry·Rz·S.
//! `id` is a local part key; semantic hashes do not exhaust qualified identifiers.
mod admission;
mod compile;
mod render;
pub use admission::{admit_assembly, validate_compiled};
pub use compile::compile_assembly;
pub use craft_authoring::{CompiledAssembly, CompiledPart};
pub use render::{render_assembly, render_assembly_with_settings, AssemblyRenderSettings};
pub const ASSEMBLY_SCHEMA_VERSION: u32 = 1;

pub fn is_assembly(value: &vibe::Value) -> bool {
    matches!(value, vibe::Value::Record(record) if record.contains_key("parts"))
}

#[cfg(test)]
mod tests;
