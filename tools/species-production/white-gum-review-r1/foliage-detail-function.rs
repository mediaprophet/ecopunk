
/// Isolated adult-foliage view of the same white-gum candidate's authored spray.
/// Metres, +Y up, branch base at origin; authored0.60m shoot and145x22mm nominal blades.
/// Native profile owns the same blade geometry, thin shoot and connected petioles.
/// Not a photographed/identified specimen; biological/visual acceptance remains pending.
pub fn build_white_gum_review_foliage_detail(
    trunks: &mut SubmeshBuilder,
    foliage: &mut SubmeshBuilder,
) {
    add_botanical_foliage_spray_profile(
        trunks,
        foliage,
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        0.60,
        0.145,
        0.022,
        12,
        [0.43, 0.43, 0.30, 1.0],
        [0.15, 0.31, 0.19, 1.0],
        [0.32, 0.49, 0.29, 1.0],
        2.73,
        true,
    );
}
