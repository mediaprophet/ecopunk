//! Cold source-replayed texture editing. Canonical Core owns pixels and PNG.
mod recipe;
mod render;
pub use recipe::{
    DetailCost, DetailLimits, DetailRecipe, PaintLayer, PaintRecipe, PaintStroke, ProceduralRecipe,
};
pub use render::{render_detail, DetailBase, DetailOutput};
#[cfg(test)]
mod tests;
