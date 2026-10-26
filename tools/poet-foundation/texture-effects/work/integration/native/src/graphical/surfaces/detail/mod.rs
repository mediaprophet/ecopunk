//! Cold source-replayed texture editing. Canonical Core owns pixels and PNG.
mod recipe;
mod render;
pub use recipe::{
    validate_effects, DetailCost, DetailLimits, DetailRecipe, PaintLayer, PaintRecipe, PaintStroke,
    ProceduralRecipe, TextureEffect, TextureEffectStep,
};
pub use render::{render_detail, DetailBase, DetailOutput};
#[cfg(test)]
mod tests;

#[cfg(test)]
mod effect_tests;
