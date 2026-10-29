// Append inside the existing botanical_field_builder module. Uses the retained
// white-gum-specific profile including its connected petiole geometry.
pub(crate) fn add_white_gum_authored_spray(
    wood: &mut SubmeshBuilder, foliage: &mut SubmeshBuilder,
    attach: [f32;3], direction: [f32;3], length_m: f32,
    leaf_length_m: f32, leaf_width_m: f32, leaf_count: usize, seed: f32,
) {
    add_botanical_foliage_spray_profile(wood,foliage,attach,direction,length_m,
        leaf_length_m,leaf_width_m,leaf_count,
        [0.38,0.38,0.27,1.0],[0.15,0.30,0.18,1.0],[0.30,0.46,0.27,1.0],seed,true);
}
