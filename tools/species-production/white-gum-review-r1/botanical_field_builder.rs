//! Species-Authentic Botanical Stratum & Architectural Canopy Builder (`botanical_field_builder.rs`)
//!
//! Synthesizes multi-tiered Australian flora across emergent, canopy, mid-story, understorey,
//! riparian, and benthic macrophyte strata for `.10d` sector scenes:
//! - **Ghost Gum** (*Corymbia aparrerinja*) & **Snow Gum** (*Eucalyptus pauciflora*)
//! - **Mountain Ash** (*Eucalyptus regnans*) & **River Red Gum** (*Eucalyptus camaldulensis*)
//! - **Golden Wattle** (*Acacia pycnantha*) with golden-yellow globose raceme inflorescences
//! - **Old Man Banksia** (*Banksia serrata*) & **NSW Waratah** (*Telopea speciosissima*) with
//!   cylindrical amber/ochre floral cones and crimson waratah flower heads
//! - **Broad-Leaved Peppermint / Scribbly Gum** (*Eucalyptus dives* / *Eucalyptus rossii*)
//! - **Soft Tree Fern** (*Dicksonia antarctica*) with fibrous caudex, fiddlehead crosiers, and
//!   16+ arching bipinnate fronds with serrated pinnae
//! - **Austral Grass Tree** (*Xanthorrhoea australis*) with fire-charred caudex, 64+ drooping
//!   needle skirt, and velvety flowering scape spike
//! - **Ground Cover & Riparian/Benthic Macrophytes**: *Themeda triandra* (Kangaroo Grass),
//!   *Poa sieberiana* (Snow Grass), *Juncus usitatus* / *Carex* sedges, and deeply submerged
//!   dark-olive *Vallisneria australis* ribbon weed strictly below `water_y - 0.04`.

use std::f32::consts::{PI, TAU};

use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::ExtrusionStation;
use qualia_core_db::specialized_libs::computational_geometry::tapered_branch_field::{
    tapered_branch_field_into, TaperedFieldSegment,
};
use qualia_core_db::specialized_libs::computational_geometry::Point3;

use crate::scene_builder_10d::SubmeshBuilder;

/// Golden angle ($\phi \approx 137.507764^\circ$) in radians.
const GOLDEN_ANGLE: f32 = 2.399_963_1;

#[inline]
fn hash21(x: f32, y: f32) -> f32 {
    let h = (x * 127.1 + y * 311.7).sin() * 43_758.547;
    h - h.floor()
}

#[inline]
fn smooth_noise(x: f32) -> f32 {
    (x * 1.91).sin() * 0.50 + (x * 7.73 + 0.3).sin() * 0.28 + (x * 19.3).sin() * 0.12
}

#[inline]
fn add3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale3(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn normalize3(a: [f32; 3]) -> [f32; 3] {
    let len = dot3(a, a).sqrt();
    if len > 1e-6 {
        [a[0] / len, a[1] / len, a[2] / len]
    } else {
        [0.0, 1.0, 0.0]
    }
}

#[inline]
fn lerp_color(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    let u = t.clamp(0.0, 1.0);
    [
        a[0] * (1.0 - u) + b[0] * u,
        a[1] * (1.0 - u) + b[1] * u,
        a[2] * (1.0 - u) + b[2] * u,
        1.0,
    ]
}

/// Species architectural guild for canopy and mid-story trees.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeSpeciesGuild {
    /// Working E. dalrympleana subsp. dalrympleana concept; synthetic, not an identified specimen.
    MountainWhiteGum,
    /// *Corymbia aparrerinja* (Ghost Gum) / *Eucalyptus pauciflora* (Snow Gum)
    GhostOrSnowGum,
    /// *Acacia pycnantha* (Golden Wattle) with golden raceme flower clusters
    GoldenWattle,
    /// *Eucalyptus regnans* (Mountain Ash) / *Eucalyptus camaldulensis* (River Red Gum)
    TallAshOrRedGum,
    /// *Banksia serrata* (Old Man Banksia) / *Telopea speciosissima* (NSW Waratah)
    BanksiaOrWaratah,
    /// *Eucalyptus dives* (Broad-Leaved Peppermint) / *Eucalyptus rossii* (Scribbly Gum)
    PeppermintOrScribblyGum,
}

/// Evaluate smooth branch collar flare using `tapered_branch_field_into` from QualiaDB's
/// computational geometry library so fork junctions swell organically.
fn compute_collar_flare_multiplier(
    parent_a: [f32; 3],
    parent_b: [f32; 3],
    parent_ra: f32,
    parent_rb: f32,
    child_a: [f32; 3],
    child_b: [f32; 3],
    child_ra: f32,
    child_rb: f32,
) -> f32 {
    let seg_parent = TaperedFieldSegment {
        a: Point3::new(parent_a[0] as f64, parent_a[1] as f64, parent_a[2] as f64),
        b: Point3::new(parent_b[0] as f64, parent_b[1] as f64, parent_b[2] as f64),
        radius_a: (parent_ra.max(0.002)) as f64,
        radius_b: (parent_rb.max(0.002)) as f64,
    };
    let seg_child = TaperedFieldSegment {
        a: Point3::new(child_a[0] as f64, child_a[1] as f64, child_a[2] as f64),
        b: Point3::new(child_b[0] as f64, child_b[1] as f64, child_b[2] as f64),
        radius_a: (child_ra.max(0.0015)) as f64,
        radius_b: (child_rb.max(0.001)) as f64,
    };
    let sample = [Point3::new(
        (child_a[0] + child_ra * 0.85) as f64,
        child_a[1] as f64,
        child_a[2] as f64,
    )];
    let mut out = [0.0_f64; 1];
    let blend = (parent_ra.max(child_ra) * 0.65).max(0.003) as f64;
    if tapered_branch_field_into(
        &sample,
        &[seg_parent, seg_child],
        blend,
        1e-4,
        1e-2,
        1,
        2,
        2,
        &mut out,
    )
    .is_ok()
    {
        let swell = (-out[0] as f32 / child_ra.max(0.002)).clamp(0.0, 0.42);
        1.16 + swell * 0.45
    } else {
        1.24
    }
}

/// Emit a multi-segment curved falcate eucalypt leaf blade modeled on the authored
/// `eucalypt-spray-r1` study (`generate.rs` `Leaf::at` & `Leaf::color`), with raised
/// central midrib, lateral sickle curve, adaxial/abaxial shading, and ontogenetic copper-flush
/// to mature sclerophyll color gradient.
pub fn add_study_falcate_leaf(
    b_foliage: &mut SubmeshBuilder,
    base: [f32; 3],
    along: [f32; 3],
    across: [f32; 3],
    normal: [f32; 3],
    length: f32,
    width: f32,
    mature_col: [f32; 4],
    young_col: [f32; 4],
    age: f32,
) {
    let rows = 4usize;
    let cols = 2usize; // cols=2 -> left margin (-1), raised midrib (0), right margin (+1)
    let v_start = b_foliage.positions.len();
    let i_start = b_foliage.indices.len();

    let age_blend = (age * 1.65).clamp(0.0, 1.0);
    let leaf_base_col = lerp_color(young_col, mature_col, age_blend);

    for r in 0..=rows {
        let t = 0.01 + 0.98 * (r as f32 / rows as f32);
        let sin_pi_t = (PI * t).sin();
        let half_w = 0.5 * width * sin_pi_t.max(0.02).powf(0.82) * (1.0 - 0.18 * t);
        let sickle_shift = 0.17 * length * sin_pi_t * t;
        let droop_z = -0.08 * length * t * t + 0.018 * length * sin_pi_t;

        for c in 0..=cols {
            let s = 2.0 * (c as f32 / cols as f32) - 1.0; // -1.0, 0.0, +1.0
            let lateral = s * half_w + sickle_shift;
            // Midrib ridge at s == 0, slight lamina cup towards margins
            let midrib_relief = (1.0 - s.abs()) * width * 0.11 * sin_pi_t;
            let cup_z = 0.05 * width * s * s * sin_pi_t;
            let z_off = droop_z + midrib_relief + cup_z;

            let pos = add3(
                base,
                add3(
                    scale3(along, t * length),
                    add3(scale3(across, lateral), scale3(normal, z_off)),
                ),
            );

            // Subtle midrib highlight at s == 0, organic noise, and tip sun-warmth
            let v_var =
                0.94 + 0.08 * (1.0 - s.abs()) + 0.06 * t + 0.04 * smooth_noise(t * 32.0 + s * 11.0);
            let col = [
                (leaf_base_col[0] * v_var).clamp(0.0, 1.0),
                (leaf_base_col[1] * v_var).clamp(0.0, 1.0),
                (leaf_base_col[2] * v_var).clamp(0.0, 1.0),
                1.0,
            ];
            b_foliage.push_vertex(pos, normal, col, [0.5 * (s + 1.0), t]);
        }
    }

    let stride = (cols + 1) as u32;
    for r in 0..rows as u32 {
        for c in 0..cols as u32 {
            let a = v_start as u32 + r * stride + c;
            let b = a + 1;
            let d = a + stride;
            let e = d + 1;
            b_foliage.push_tri(a, b, d);
            b_foliage.push_tri(b, e, d);
        }
    }

    b_foliage.recompute_normals_since(v_start, i_start);
}

/// Build a complete botanical eucalypt/myrtaceous branchlet spray with a curved woody shoot,
/// petiole stalks, and a dense phyllotactic cluster of pendulous falcate leaves.
pub fn add_botanical_foliage_spray(
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
    attach_pos: [f32; 3],
    shoot_dir: [f32; 3],
    spray_length: f32,
    leaf_length: f32,
    leaf_width: f32,
    leaf_count: usize,
    twig_col: [f32; 4],
    leaf_dark: [f32; 4],
    leaf_lit: [f32; 4],
    seed: f32,
) {
    add_botanical_foliage_spray_profile(b_trunks,b_foliage,attach_pos,shoot_dir,
        spray_length,leaf_length,leaf_width,leaf_count,twig_col,leaf_dark,leaf_lit,seed,false);
}
fn add_botanical_foliage_spray_profile(
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
    attach_pos: [f32; 3],
    shoot_dir: [f32; 3],
    spray_length: f32,
    leaf_length: f32,
    leaf_width: f32,
    leaf_count: usize,
    twig_col: [f32; 4],
    leaf_dark: [f32; 4],
    leaf_lit: [f32; 4],
    seed: f32,
    white_gum: bool,
) {
    let dir = normalize3(shoot_dir);
    let up_ref = if dir[1].abs() < 0.85 {
        [0.0, 1.0, 0.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let side_vec = normalize3(cross3(dir, up_ref));
    let up_vec = normalize3(cross3(side_vec, dir));

    // Curved 4-station shoot axis that arches outward and droops at the tip
    let curve_pt = |t: f32| -> [f32; 3] {
        let sway = (PI * t).sin() * spray_length * 0.12 * (seed * 1.7).sin();
        let droop = -t * t * spray_length * 0.28;
        add3(
            attach_pos,
            add3(
                scale3(dir, t * spray_length),
                add3(scale3(side_vec, sway), [0.0, droop, 0.0]),
            ),
        )
    };

    let p0 = curve_pt(0.0);
    let p1 = curve_pt(0.35);
    let p2 = curve_pt(0.70);
    let p3 = curve_pt(1.0);
    let shoot_axis = [
        Point3::new(p0[0] as f64, p0[1] as f64, p0[2] as f64),
        Point3::new(p1[0] as f64, p1[1] as f64, p1[2] as f64),
        Point3::new(p2[0] as f64, p2[1] as f64, p2[2] as f64),
        Point3::new(p3[0] as f64, p3[1] as f64, p3[2] as f64),
    ];
    let r_base = (spray_length * if white_gum {0.006} else {0.032}).max(0.0022) as f64;
    let shoot_stations = [
        ExtrusionStation {
            scale_x: r_base,
            scale_y: r_base,
            twist_rad: 0.0,
        },
        ExtrusionStation {
            scale_x: r_base * 0.76,
            scale_y: r_base * 0.76,
            twist_rad: 0.25,
        },
        ExtrusionStation {
            scale_x: r_base * 0.54,
            scale_y: r_base * 0.54,
            twist_rad: 0.50,
        },
        ExtrusionStation {
            scale_x: r_base * 0.34,
            scale_y: r_base * 0.34,
            twist_rad: 0.75,
        },
    ];
    b_trunks.add_bishop_swept_tube(&shoot_axis, &shoot_stations, 5, twig_col, twig_col);

    // Young copper-bronze flush color for apical leaves
    let young_flush = if white_gum {leaf_lit} else {[
        (leaf_lit[0] * 0.85 + 0.24).min(1.0),
        leaf_lit[1] * 0.72 + 0.06,
        leaf_lit[2] * 0.65,
        1.0,
    ]};

    let total = leaf_count.max(6);
    for idx in 0..total {
        let frac = (idx as f32 + 0.35) / (total as f32);
        let at_t = 0.14 + 0.84 * frac;
        let node_pos = curve_pt(at_t);
        let side_sign = if idx % 2 == 0 { 1.0_f32 } else { -1.0_f32 };
        let phi = idx as f32 * GOLDEN_ANGLE + seed;

        // Sclerophyllous leaves hang pendulously (-Y bias) while fanning outward
        let along = normalize3([
            dir[0] * 0.42 + side_vec[0] * side_sign * 0.58 + 0.22 * phi.cos(),
            -0.78 - 0.18 * (idx as f32 * 1.3).sin().abs(),
            dir[2] * 0.42 + side_vec[2] * side_sign * 0.58 + 0.22 * phi.sin(),
        ]);
        let across0 = normalize3(cross3(along, up_vec));
        let norm0 = normalize3(cross3(across0, along));
        let roll = 0.45 * (idx as f32 * 1.9 + seed).sin();
        let across = normalize3(add3(scale3(across0, roll.cos()), scale3(norm0, roll.sin())));
        let normal = normalize3(cross3(along, across));

        // Short petiole offset from shoot node to lamina base
        let petiole_len = leaf_length * 0.14;
        let leaf_base = add3(node_pos, scale3(along, petiole_len));
        if white_gum {
            let axis=[Point3::new(node_pos[0]as f64,node_pos[1]as f64,node_pos[2]as f64),
                Point3::new(leaf_base[0]as f64,leaf_base[1]as f64,leaf_base[2]as f64)];
            let stations=[ExtrusionStation{scale_x:0.0009,scale_y:0.0009,twist_rad:0.0},
                ExtrusionStation{scale_x:0.0005,scale_y:0.0005,twist_rad:0.0}];
            b_trunks.add_bishop_swept_tube(&axis,&stations,6,twig_col,twig_col);
        }

        let age = (1.0 - 0.65 * frac).clamp(0.15, 1.0);
        let sun_mix = (0.35 + 0.65 * frac).clamp(0.0, 1.0);
        let mature_col = lerp_color(leaf_dark, leaf_lit, sun_mix);
        let cur_len = leaf_length * (0.82 + 0.32 * hash21(idx as f32, seed));
        let cur_w = leaf_width * (0.85 + 0.28 * hash21(seed, idx as f32));

        add_study_falcate_leaf(
            b_foliage,
            leaf_base,
            along,
            across,
            normal,
            cur_len,
            cur_w,
            mature_col,
            young_flush,
            age,
        );
    }
}

/// Add a porous inner crown sub-canopy of intersecting curved botanical leaf-fan clusters
/// so the tree crown reads as a lush, full, sunlit 3D volume from afar while remaining
/// completely open and botanical up close.
fn add_inner_crown_leaf_fans(
    b_foliage: &mut SubmeshBuilder,
    center: [f32; 3],
    rx: f32,
    ry: f32,
    rz: f32,
    leaf_dark: [f32; 4],
    leaf_lit: [f32; 4],
    fan_count: usize,
    seed: f32,
) {
    let count = fan_count.max(8);
    for i in 0..count {
        let fi = i as f32;
        let frac = (fi + 0.5) / (count as f32);
        let phi = fi * GOLDEN_ANGLE + seed * 1.3;
        let y_norm = 1.0 - 1.85 * frac;
        let r_ring = (1.0 - y_norm * y_norm).max(0.08).sqrt();
        let radial_shell = 0.38 + 0.56 * hash21(fi * 2.1, seed);

        let pos = [
            center[0] + phi.cos() * rx * r_ring * radial_shell,
            center[1] + y_norm * ry * 0.85,
            center[2] + phi.sin() * rz * r_ring * radial_shell,
        ];

        let sun_t = (0.52 + 0.48 * y_norm).clamp(0.08, 1.0);
        let col_top = lerp_color(leaf_dark, leaf_lit, sun_t);
        let col_mid = lerp_color(leaf_dark, leaf_lit, (sun_t - 0.18).max(0.0));

        // Emit a 3-lobed arched botanical leaf fan at `pos`
        let fan_r = rx * (0.28 + 0.12 * hash21(seed, fi * 3.7));
        for lobe in 0..3 {
            let la = phi + (lobe as f32 - 1.0) * 0.95;
            let ca = la.cos();
            let sa = la.sin();
            let px = -sa;
            let pz = ca;

            let l_len = fan_r * (0.85 + 0.20 * (lobe as f32));
            let l_w = l_len * 0.34;
            let p_base = pos;
            let p_mid_l = [
                pos[0] + ca * l_len * 0.46 - px * l_w,
                pos[1] - l_len * 0.22 + l_w * 0.18,
                pos[2] + sa * l_len * 0.46 - pz * l_w,
            ];
            let p_mid_c = [
                pos[0] + ca * l_len * 0.52,
                pos[1] - l_len * 0.12 + l_w * 0.36,
                pos[2] + sa * l_len * 0.52,
            ];
            let p_mid_r = [
                pos[0] + ca * l_len * 0.46 + px * l_w,
                pos[1] - l_len * 0.22 + l_w * 0.18,
                pos[2] + sa * l_len * 0.46 + pz * l_w,
            ];
            let p_tip = [
                pos[0] + ca * l_len * 0.92 + px * l_w * 0.28,
                pos[1] - l_len * 0.68,
                pos[2] + sa * l_len * 0.92 + pz * l_w * 0.28,
            ];

            let v_start = b_foliage.positions.len();
            let i_start = b_foliage.indices.len();
            let n_placeholder = [ca * 0.3, 0.9, sa * 0.3];
            let v0 = b_foliage.push_vertex(p_base, n_placeholder, col_mid, [0.5, 0.0]);
            let v1 = b_foliage.push_vertex(p_mid_l, n_placeholder, col_top, [0.0, 0.5]);
            let v2 = b_foliage.push_vertex(p_mid_c, n_placeholder, col_top, [0.5, 0.5]);
            let v3 = b_foliage.push_vertex(p_mid_r, n_placeholder, col_top, [1.0, 0.5]);
            let v4 = b_foliage.push_vertex(p_tip, n_placeholder, col_mid, [0.5, 1.0]);

            b_foliage.push_tri(v0, v1, v2);
            b_foliage.push_tri(v0, v2, v3);
            b_foliage.push_tri(v1, v4, v2);
            b_foliage.push_tri(v2, v4, v3);
            b_foliage.recompute_normals_since(v_start, i_start);
        }
    }
}

/// Add a golden-yellow *Acacia pycnantha* (Golden Wattle) raceme flower cluster.
fn add_wattle_raceme_cluster(
    b_foliage: &mut SubmeshBuilder,
    center: [f32; 3],
    scale: f32,
    seed: f32,
) {
    let puff_count = 5usize;
    let col_gold_deep = [0.95, 0.72, 0.08, 1.0];
    let col_gold_bright = [1.00, 0.90, 0.22, 1.0];

    for p in 0..puff_count {
        let fp = p as f32;
        let a = fp * GOLDEN_ANGLE + seed;
        let r = scale * (0.35 + 0.45 * (fp / puff_count as f32));
        let puff_c = [
            center[0] + a.cos() * r,
            center[1] - fp * scale * 0.16 + 0.01 * a.sin(),
            center[2] + a.sin() * r,
        ];
        let pr = scale * (0.28 - 0.025 * fp).max(0.008);

        // Octahedral-stellate golden blossom puff
        let top = b_foliage.push_vertex(
            [puff_c[0], puff_c[1] + pr, puff_c[2]],
            [0.0, 1.0, 0.0],
            col_gold_bright,
            [0.5, 1.0],
        );
        let bot = b_foliage.push_vertex(
            [puff_c[0], puff_c[1] - pr, puff_c[2]],
            [0.0, -1.0, 0.0],
            col_gold_deep,
            [0.5, 0.0],
        );
        let ring_n = 5usize;
        let mut ring_idx = [0u32; 5];
        for k in 0..ring_n {
            let ka = (k as f32 / ring_n as f32) * TAU + fp * 0.4;
            let nx = ka.cos();
            let nz = ka.sin();
            ring_idx[k] = b_foliage.push_vertex(
                [puff_c[0] + nx * pr, puff_c[1], puff_c[2] + nz * pr],
                [nx, 0.2, nz],
                col_gold_bright,
                [k as f32 / ring_n as f32, 0.5],
            );
        }
        for k in 0..ring_n {
            let kn = (k + 1) % ring_n;
            b_foliage.push_tri(top, ring_idx[k], ring_idx[kn]);
            b_foliage.push_tri(bot, ring_idx[kn], ring_idx[k]);
        }
    }
}

/// Add an upright cylindrical *Banksia serrata* amber/ochre floral spike or crimson
/// *Telopea speciosissima* (NSW Waratah) inflorescence head.
fn add_banksia_or_waratah_inflorescence(
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
    pos: [f32; 3],
    scale: f32,
    is_waratah: bool,
) {
    if is_waratah {
        // Crimson Waratah floral dome + basal ring of scarlet involucral bracts
        let col_crimson = [0.86, 0.10, 0.16, 1.0];
        let col_scarlet = [0.96, 0.22, 0.24, 1.0];
        let r_dome = (scale * 0.55).max(0.012);
        let h_dome = r_dome * 1.15;

        let apex = b_foliage.push_vertex(
            [pos[0], pos[1] + h_dome, pos[2]],
            [0.0, 1.0, 0.0],
            col_scarlet,
            [0.5, 1.0],
        );
        let bract_count = 8usize;
        let mut ring = [0u32; 8];
        for k in 0..bract_count {
            let a = (k as f32 / bract_count as f32) * TAU;
            let ca = a.cos();
            let sa = a.sin();
            ring[k] = b_foliage.push_vertex(
                [pos[0] + ca * r_dome, pos[1] + h_dome * 0.25, pos[2] + sa * r_dome],
                [ca, 0.35, sa],
                col_crimson,
                [k as f32 / bract_count as f32, 0.5],
            );
            // Radiating pointed scarlet bract
            let bract_tip = b_foliage.push_vertex(
                [
                    pos[0] + ca * r_dome * 1.65,
                    pos[1] + h_dome * 0.05,
                    pos[2] + sa * r_dome * 1.65,
                ],
                [ca * 0.5, 0.85, sa * 0.5],
                col_scarlet,
                [0.5, 0.0],
            );
            let a_next = ((k + 1) as f32 / bract_count as f32) * TAU;
            let bract_side = b_foliage.push_vertex(
                [
                    pos[0] + a_next.cos() * r_dome * 0.95,
                    pos[1] + h_dome * 0.15,
                    pos[2] + a_next.sin() * r_dome * 0.95,
                ],
                [0.0, 1.0, 0.0],
                col_crimson,
                [1.0, 0.2],
            );
            b_foliage.push_tri(ring[k], bract_tip, bract_side);
        }
        for k in 0..bract_count {
            let kn = (k + 1) % bract_count;
            b_foliage.push_tri(apex, ring[k], ring[kn]);
        }
    } else {
        // Cylindrical upright *Banksia serrata* floral cone in warm amber-ochre & gold
        let h_cone = (scale * 1.25).max(0.022);
        let r_cone = (h_cone * 0.36).max(0.008) as f64;
        let axis = [
            Point3::new(pos[0] as f64, pos[1] as f64, pos[2] as f64),
            Point3::new(pos[0] as f64, (pos[1] + h_cone * 0.55) as f64, pos[2] as f64),
            Point3::new(pos[0] as f64, (pos[1] + h_cone) as f64, pos[2] as f64),
        ];
        let st = [
            ExtrusionStation {
                scale_x: r_cone * 0.92,
                scale_y: r_cone * 0.92,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: r_cone * 1.04,
                scale_y: r_cone * 1.04,
                twist_rad: 0.35,
            },
            ExtrusionStation {
                scale_x: r_cone * 0.72,
                scale_y: r_cone * 0.72,
                twist_rad: 0.70,
            },
        ];
        b_trunks.add_bishop_swept_tube(
            &axis,
            &st,
            7,
            [0.64, 0.36, 0.14, 1.0],
            [0.92, 0.68, 0.22, 1.0],
        );
    }
}

/// Add a woody eucalypt gumnut capsule and creamy stamen blossom cluster at branchlet nodes.
fn add_eucalypt_gumnut_and_blossom_cluster(
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
    pos: [f32; 3],
    scale: f32,
    flower_col: [f32; 4],
    seed: f32,
) {
    let nut_len = (scale * 0.42).max(0.008);
    let nut_r = (nut_len * 0.45).max(0.004) as f64;
    for k in 0..3 {
        let a = (k as f32 / 3.0) * TAU + seed;
        let ca = a.cos();
        let sa = a.sin();
        let p_end = [
            pos[0] + ca * nut_len,
            pos[1] - nut_len * 0.65,
            pos[2] + sa * nut_len,
        ];
        let axis = [
            Point3::new(pos[0] as f64, pos[1] as f64, pos[2] as f64),
            Point3::new(p_end[0] as f64, p_end[1] as f64, p_end[2] as f64),
        ];
        let st = [
            ExtrusionStation {
                scale_x: nut_r * 0.45,
                scale_y: nut_r * 0.45,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: nut_r,
                scale_y: nut_r,
                twist_rad: 0.2,
            },
        ];
        b_trunks.add_bishop_swept_tube(
            &axis,
            &st,
            5,
            [0.38, 0.28, 0.20, 1.0],
            [0.52, 0.40, 0.28, 1.0],
        );
    }

    // Creamy-white / crimson stamen burst on flowering shoots
    let burst_r = (scale * 0.36).max(0.007);
    let c_top = b_foliage.push_vertex(
        [pos[0], pos[1] + burst_r, pos[2]],
        [0.0, 1.0, 0.0],
        flower_col,
        [0.5, 1.0],
    );
    for k in 0..4 {
        let a0 = (k as f32 / 4.0) * TAU + seed;
        let a1 = ((k + 1) as f32 / 4.0) * TAU + seed;
        let v0 = b_foliage.push_vertex(
            [pos[0] + a0.cos() * burst_r, pos[1], pos[2] + a0.sin() * burst_r],
            [a0.cos(), 0.3, a0.sin()],
            flower_col,
            [0.0, 0.5],
        );
        let v1 = b_foliage.push_vertex(
            [pos[0] + a1.cos() * burst_r, pos[1], pos[2] + a1.sin() * burst_r],
            [a1.cos(), 0.3, a1.sin()],
            flower_col,
            [1.0, 0.5],
        );
        b_foliage.push_tri(c_top, v0, v1);
    }
}

/// Build an architectural multi-tiered Australian canopy or mid-story tree with:
/// - Root-collar buttress flares + basal stocking
/// - Smoothly curved bole with peeling bark ribbons
/// - Primary scaffold boughs, secondary branches, and tertiary twigs with `tapered_branch_field`
///   collar flare calculation
/// - Porous inner crown leaf-fan clusters + dense outer pendulous falcate leaf sprays
/// - Species-authentic wattle racemes, banksia cones, waratah heads, or eucalypt gumnuts.
pub fn add_architectural_canopy_tree(
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
    root: [f32; 3],
    height: f32,
    guild: TreeSpeciesGuild,
    sector_id: &str,
    seed: f32,
) {
    let (
        stocking_col,
        bark_base,
        bark_top,
        twig_col,
        leaf_dark,
        leaf_lit,
        flower_col,
        contorted,
    ) = match guild {
        TreeSpeciesGuild::MountainWhiteGum => (
            [0.40, 0.37, 0.32, 1.0],
            [0.80, 0.79, 0.71, 1.0],
            [0.92, 0.91, 0.83, 1.0],
            [0.43, 0.43, 0.30, 1.0],
            [0.15, 0.31, 0.19, 1.0],
            [0.32, 0.49, 0.29, 1.0],
            [0.97, 0.96, 0.89, 1.0],
            false,
        ),
        TreeSpeciesGuild::GhostOrSnowGum => {
            let is_alpine =
                sector_id.contains("three_mile") || sector_id.contains("yarrangobilly");
            if is_alpine {
                // Snow Gum (*Eucalyptus pauciflora*): twisted silver/olive/ochre-cream bark
                (
                    [0.36, 0.34, 0.30, 1.0],
                    [0.76, 0.78, 0.72, 1.0],
                    [0.93, 0.91, 0.85, 1.0],
                    [0.54, 0.42, 0.30, 1.0],
                    [0.18, 0.38, 0.24, 1.0],
                    [0.38, 0.58, 0.36, 1.0],
                    [0.97, 0.96, 0.86, 1.0],
                    true,
                )
            } else {
                // Ghost Gum (*Corymbia aparrerinja*): chalk-white / cream-ochre smooth bark
                (
                    [0.38, 0.30, 0.24, 1.0],
                    [0.88, 0.85, 0.80, 1.0],
                    [0.96, 0.95, 0.92, 1.0],
                    [0.62, 0.54, 0.44, 1.0],
                    [0.19, 0.40, 0.22, 1.0],
                    [0.38, 0.62, 0.34, 1.0],
                    [0.98, 0.96, 0.88, 1.0],
                    true,
                )
            }
        }
        TreeSpeciesGuild::GoldenWattle => (
            // Golden Wattle (*Acacia pycnantha*): dark olive-brown bark + lush phyllodes + gold racemes
            [0.24, 0.19, 0.14, 1.0],
            [0.30, 0.25, 0.18, 1.0],
            [0.42, 0.36, 0.24, 1.0],
            [0.36, 0.34, 0.20, 1.0],
            [0.20, 0.42, 0.16, 1.0],
            [0.42, 0.64, 0.24, 1.0],
            [1.00, 0.86, 0.14, 1.0],
            false,
        ),
        TreeSpeciesGuild::TallAshOrRedGum => (
            // Mountain Ash (*Eucalyptus regnans*) / River Red Gum: rough fibrous stocking + pale upper trunk
            [0.30, 0.22, 0.16, 1.0],
            [0.54, 0.48, 0.42, 1.0],
            [0.80, 0.77, 0.70, 1.0],
            [0.46, 0.36, 0.26, 1.0],
            [0.14, 0.35, 0.18, 1.0],
            [0.32, 0.56, 0.28, 1.0],
            [0.96, 0.95, 0.84, 1.0],
            false,
        ),
        TreeSpeciesGuild::BanksiaOrWaratah => (
            // Old Man Banksia (*Banksia serrata*) / NSW Waratah (*Telopea speciosissima*)
            [0.24, 0.18, 0.15, 1.0],
            [0.34, 0.26, 0.20, 1.0],
            [0.46, 0.38, 0.30, 1.0],
            [0.38, 0.30, 0.22, 1.0],
            [0.18, 0.38, 0.16, 1.0],
            [0.38, 0.58, 0.26, 1.0],
            [0.90, 0.56, 0.16, 1.0],
            true,
        ),
        TreeSpeciesGuild::PeppermintOrScribblyGum => (
            // Peppermint / Scribbly Gum (*Eucalyptus dives* / *Eucalyptus rossii*)
            [0.34, 0.28, 0.22, 1.0],
            [0.58, 0.52, 0.44, 1.0],
            [0.82, 0.78, 0.70, 1.0],
            [0.48, 0.40, 0.30, 1.0],
            [0.17, 0.38, 0.24, 1.0],
            [0.35, 0.58, 0.36, 1.0],
            [0.95, 0.94, 0.82, 1.0],
            true,
        ),
    };

    let white_gum = guild == TreeSpeciesGuild::MountainWhiteGum;
    let r_bole = (height * if white_gum { 0.025 } else { 0.068 }).max(0.012);
    let bend_x = if contorted {
        height * 0.16 * (seed * 1.1).sin()
    } else {
        height * 0.05 * (seed * 1.1).sin()
    };
    let bend_z = if contorted {
        height * 0.15 * (seed * 1.3).cos()
    } else {
        height * 0.05 * (seed * 1.3).cos()
    };

    // 1. Flared Root Buttresses (4 radiating buttress spurs at ground collar)
    let buttress_count = 4usize;
    for br in 0..buttress_count {
        let ba = (br as f32 / buttress_count as f32) * TAU + seed * 0.7;
        let ca = ba.cos();
        let sa = ba.sin();
        let spur_reach = r_bole * (2.3 + 0.4 * (br as f32).sin().abs());
        let spur_h = height * if white_gum { 0.035 } else { 0.13 };
        let axis = [
            Point3::new(
                (root[0] + ca * spur_reach) as f64,
                (root[1] - 0.018) as f64,
                (root[2] + sa * spur_reach) as f64,
            ),
            Point3::new(
                (root[0] + ca * r_bole * 1.15) as f64,
                (root[1] + spur_h * 0.45) as f64,
                (root[2] + sa * r_bole * 1.15) as f64,
            ),
            Point3::new(
                (root[0] + ca * r_bole * 0.45) as f64,
                (root[1] + spur_h) as f64,
                (root[2] + sa * r_bole * 0.45) as f64,
            ),
        ];
        let st = [
            ExtrusionStation {
                scale_x: (r_bole * 0.48) as f64,
                scale_y: (r_bole * 0.34) as f64,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: (r_bole * 0.62) as f64,
                scale_y: (r_bole * 0.45) as f64,
                twist_rad: 0.15,
            },
            ExtrusionStation {
                scale_x: (r_bole * 0.38) as f64,
                scale_y: (r_bole * 0.32) as f64,
                twist_rad: 0.30,
            },
        ];
        b_trunks.add_bishop_swept_tube(&axis, &st, 5, stocking_col, bark_base);
    }

    // 2. Curved Main Bole (Basal Fibrous Stocking -> Smooth / Mottled Upper Bole)
    let fork_y = height * if contorted { 0.52 } else { 0.60 };
    let bole_p0 = [root[0], root[1] - 0.015, root[2]];
    let bole_p1 = [
        root[0] + bend_x * 0.22,
        root[1] + fork_y * 0.32,
        root[2] + bend_z * 0.22,
    ];
    let bole_p2 = [
        root[0] + bend_x * 0.64,
        root[1] + fork_y * 0.68,
        root[2] + bend_z * 0.64,
    ];
    let bole_p3 = [root[0] + bend_x, root[1] + fork_y, root[2] + bend_z];

    // Lower stocking section
    let lower_axis = [
        Point3::new(bole_p0[0] as f64, bole_p0[1] as f64, bole_p0[2] as f64),
        Point3::new(bole_p1[0] as f64, bole_p1[1] as f64, bole_p1[2] as f64),
    ];
    let lower_st = [
        ExtrusionStation {
            scale_x: (r_bole * 1.42) as f64,
            scale_y: (r_bole * 1.28) as f64,
            twist_rad: 0.0,
        },
        ExtrusionStation {
            scale_x: (r_bole * 0.96) as f64,
            scale_y: (r_bole * 0.94) as f64,
            twist_rad: 0.32,
        },
    ];
    b_trunks.add_bishop_swept_tube(&lower_axis, &lower_st, 8, stocking_col, bark_base);

    // Upper smooth/mottled trunk section
    let upper_axis = [
        Point3::new(bole_p1[0] as f64, bole_p1[1] as f64, bole_p1[2] as f64),
        Point3::new(bole_p2[0] as f64, bole_p2[1] as f64, bole_p2[2] as f64),
        Point3::new(bole_p3[0] as f64, bole_p3[1] as f64, bole_p3[2] as f64),
    ];
    let upper_st = [
        ExtrusionStation {
            scale_x: (r_bole * 0.95) as f64,
            scale_y: (r_bole * 0.93) as f64,
            twist_rad: 0.32,
        },
        ExtrusionStation {
            scale_x: (r_bole * 0.76) as f64,
            scale_y: (r_bole * 0.74) as f64,
            twist_rad: 0.68,
        },
        ExtrusionStation {
            scale_x: (r_bole * 0.58) as f64,
            scale_y: (r_bole * 0.58) as f64,
            twist_rad: 1.02,
        },
    ];
    b_trunks.add_bishop_swept_tube(&upper_axis, &upper_st, 8, bark_base, bark_top);

    // 3. Primary Scaffold Boughs -> Secondary Sub-Boughs -> Tertiary Botanical Sprays
    let bough_count = 5usize;
    let leaf_len = if white_gum { 0.145 } else { (height * 0.095).clamp(0.024, 0.062) };
    let leaf_w = if white_gum { 0.022 } else { leaf_len * 0.28 };

    // Inner crown volume at the main fork dome so the canopy center is lush and sunlit
    if white_gum {
        add_white_gum_crown_sprays(b_trunks,b_foliage,bole_p3,
            [bole_p3[0],root[1]+height*0.78,bole_p3[2]],height*0.32,height*0.22,height*0.32,
            twig_col,leaf_dark,leaf_lit,40,seed+3.1);
    } else {
    add_inner_crown_leaf_fans(
        b_foliage,
        [
            bole_p3[0],
            root[1] + height * 0.78,
            bole_p3[2],
        ],
        height * 0.32,
        height * 0.22,
        height * 0.32,
        leaf_dark,
        leaf_lit,
        14,
        seed + 3.1,
    );

    }
    for b in 0..bough_count {
        let fb = b as f32;
        let ang = (fb / bough_count as f32) * TAU + seed * 1.13 + 0.35;
        let ca = ang.cos();
        let sa = ang.sin();
        let reach = height * (0.38 + 0.08 * (fb * 1.7 + seed).sin());
        let attach_y = fork_y * (0.78 + 0.22 * (fb / bough_count as f32));
        let b_start = [
            root[0] + bend_x * (attach_y / fork_y),
            root[1] + attach_y,
            root[2] + bend_z * (attach_y / fork_y),
        ];
        let b_mid = [
            b_start[0] + ca * reach * 0.52,
            b_start[1] + height * (0.16 + 0.04 * (fb * 2.1).cos()),
            b_start[2] + sa * reach * 0.52,
        ];
        let b_tip = [
            b_start[0] + ca * reach,
            root[1] + height * (0.82 + 0.11 * (fb * 1.4 + seed).cos()),
            b_start[2] + sa * reach,
        ];

        let r_bough_base = r_bole * 0.44;
        let r_bough_tip = r_bole * 0.18;
        let collar_mult = compute_collar_flare_multiplier(
            bole_p1,
            bole_p3,
            r_bole * 0.92,
            r_bole * 0.58,
            b_start,
            b_mid,
            r_bough_base,
            r_bough_tip,
        );

        let bough_axis = [
            Point3::new(b_start[0] as f64, b_start[1] as f64, b_start[2] as f64),
            Point3::new(b_mid[0] as f64, b_mid[1] as f64, b_mid[2] as f64),
            Point3::new(b_tip[0] as f64, b_tip[1] as f64, b_tip[2] as f64),
        ];
        let bough_st = [
            ExtrusionStation {
                scale_x: (r_bough_base * collar_mult) as f64,
                scale_y: (r_bough_base * collar_mult) as f64,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: (r_bough_base * 0.65) as f64,
                scale_y: (r_bough_base * 0.65) as f64,
                twist_rad: 0.4,
            },
            ExtrusionStation {
                scale_x: r_bough_tip as f64,
                scale_y: r_bough_tip as f64,
                twist_rad: 0.8,
            },
        ];
        b_trunks.add_bishop_swept_tube(&bough_axis, &bough_st, 6, bark_base, bark_top);

        // Porous leaf-fan cluster around each primary bough tip
        if white_gum {
            add_white_gum_crown_sprays(b_trunks,b_foliage,b_tip,b_tip,height*0.22,height*0.15,height*0.22,
                twig_col,leaf_dark,leaf_lit,24,seed+fb*5.3);
        } else {
        add_inner_crown_leaf_fans(
            b_foliage,
            b_tip,
            height * 0.22,
            height * 0.15,
            height * 0.22,
            leaf_dark,
            leaf_lit,
            8,
            seed + fb * 5.3,
        );

        }
        // 2 Secondary Sub-Boughs per Primary Bough
        for s in 0..2 {
            let side_s = if s == 0 { -1.0_f32 } else { 1.0_f32 };
            let sub_ang = ang + side_s * (0.58 + 0.12 * hash21(fb, s as f32));
            let sca = sub_ang.cos();
            let ssa = sub_ang.sin();
            let sub_len = reach * 0.56;
            let s_start = if s == 0 { b_mid } else { b_tip };
            let s_mid = [
                s_start[0] + sca * sub_len * 0.55,
                s_start[1] + sub_len * 0.18,
                s_start[2] + ssa * sub_len * 0.55,
            ];
            let s_tip = [
                s_start[0] + sca * sub_len,
                s_start[1] + sub_len * 0.08,
                s_start[2] + ssa * sub_len,
            ];
            let sub_axis = [
                Point3::new(s_start[0] as f64, s_start[1] as f64, s_start[2] as f64),
                Point3::new(s_mid[0] as f64, s_mid[1] as f64, s_mid[2] as f64),
                Point3::new(s_tip[0] as f64, s_tip[1] as f64, s_tip[2] as f64),
            ];
            let sub_st = [
                ExtrusionStation {
                    scale_x: (r_bough_tip * 0.95) as f64,
                    scale_y: (r_bough_tip * 0.95) as f64,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: (r_bough_tip * 0.58) as f64,
                    scale_y: (r_bough_tip * 0.58) as f64,
                    twist_rad: 0.3,
                },
                ExtrusionStation {
                    scale_x: (r_bough_tip * 0.32) as f64,
                    scale_y: (r_bough_tip * 0.32) as f64,
                    twist_rad: 0.6,
                },
            ];
            b_trunks.add_bishop_swept_tube(&sub_axis, &sub_st, 5, bark_top, twig_col);

            // Attach detailed botanical foliage spray at secondary branch tip
            add_botanical_foliage_spray_profile(
                b_trunks,
                b_foliage,
                s_tip,
                [sca, -0.18, ssa],
                height * 0.18,
                leaf_len,
                leaf_w,
                7,
                twig_col,
                leaf_dark,
                leaf_lit,
                seed + fb * 3.1 + s as f32 * 1.9,
                white_gum,
            );

            // Species-specific floral / fruiting organs on secondary shoots
            match guild {
                // Reproductive state is unobserved; do not add oversized generic gumnuts.
                TreeSpeciesGuild::MountainWhiteGum => {},
                TreeSpeciesGuild::GoldenWattle => {
                    add_wattle_raceme_cluster(
                        b_foliage,
                        s_tip,
                        height * 0.075,
                        seed + fb * 2.3 + s as f32,
                    );
                }
                TreeSpeciesGuild::BanksiaOrWaratah => {
                    let is_waratah =
                        sector_id.contains("sydney") && ((b + s) % 2 == 1);
                    add_banksia_or_waratah_inflorescence(
                        b_trunks,
                        b_foliage,
                        s_mid,
                        height * 0.065,
                        is_waratah,
                    );
                }
                _ => {
                    if (b + s) % 2 == 0 {
                        add_eucalypt_gumnut_and_blossom_cluster(
                            b_trunks,
                            b_foliage,
                            s_mid,
                            height * 0.048,
                            flower_col,
                            seed + fb + s as f32,
                        );
                    }
                }
            }
        }

        // Terminal spray on primary bough tip
        add_botanical_foliage_spray_profile(
            b_trunks,
            b_foliage,
            b_tip,
            [ca, -0.12, sa],
            height * 0.20,
            leaf_len * 1.08,
            leaf_w * 1.08,
            8,
            twig_col,
            leaf_dark,
            leaf_lit,
            seed + fb * 7.1,
            white_gum,
        );
    }
}

/// Build a 3D *Dicksonia antarctica* (Soft Tree Fern) with a thick fibrous reddish-brown
/// caudex, coiled central fiddlehead crosiers, and 16 arching bipinnate fern fronds with
/// serrated pinnae lobes.
pub fn add_dicksonia_tree_fern(
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
    root: [f32; 3],
    height: f32,
    seed: f32,
) {
    let r_base = (height * 0.15).max(0.020) as f64;
    let trunk_axis = [
        Point3::new(root[0] as f64, (root[1] - 0.012) as f64, root[2] as f64),
        Point3::new(
            (root[0] + 0.018 * seed.sin()) as f64,
            (root[1] + height * 0.48) as f64,
            (root[2] + 0.018 * seed.cos()) as f64,
        ),
        Point3::new(root[0] as f64, (root[1] + height) as f64, root[2] as f64),
    ];
    let st = [
        ExtrusionStation {
            scale_x: r_base * 1.28,
            scale_y: r_base * 1.28,
            twist_rad: 0.0,
        },
        ExtrusionStation {
            scale_x: r_base * 0.96,
            scale_y: r_base * 0.96,
            twist_rad: 0.3,
        },
        ExtrusionStation {
            scale_x: r_base * 0.86,
            scale_y: r_base * 0.86,
            twist_rad: 0.6,
        },
    ];
    b_trunks.add_bishop_swept_tube(
        &trunk_axis,
        &st,
        7,
        [0.26, 0.16, 0.10, 1.0],
        [0.38, 0.24, 0.15, 1.0],
    );

    let crown = [root[0], root[1] + height, root[2]];

    // 4 coiled fiddlehead crosiers at the crown apex
    for c in 0..4 {
        let ca = (c as f32 / 4.0) * TAU + seed;
        let cr = height * 0.12;
        let c_axis = [
            Point3::new(crown[0] as f64, crown[1] as f64, crown[2] as f64),
            Point3::new(
                (crown[0] + ca.cos() * cr * 0.55) as f64,
                (crown[1] + height * 0.14) as f64,
                (crown[2] + ca.sin() * cr * 0.55) as f64,
            ),
            Point3::new(
                (crown[0] + ca.cos() * cr * 0.25) as f64,
                (crown[1] + height * 0.19) as f64,
                (crown[2] + ca.sin() * cr * 0.25) as f64,
            ),
        ];
        let c_st = [
            ExtrusionStation {
                scale_x: r_base * 0.22,
                scale_y: r_base * 0.22,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: r_base * 0.15,
                scale_y: r_base * 0.15,
                twist_rad: 0.8,
            },
            ExtrusionStation {
                scale_x: r_base * 0.18,
                scale_y: r_base * 0.18,
                twist_rad: 1.6,
            },
        ];
        b_trunks.add_bishop_swept_tube(
            &c_axis,
            &c_st,
            5,
            [0.34, 0.26, 0.14, 1.0],
            [0.42, 0.52, 0.20, 1.0],
        );
    }

    // 16 arching bipinnate fern fronds in upper + lower rosette tiers
    let frond_count = 16usize;
    for f in 0..frond_count {
        let tier = if f % 2 == 0 { 1.0_f32 } else { 0.72_f32 };
        let ang = (f as f32 / frond_count as f32) * TAU + seed * 0.41;
        let ca = ang.cos();
        let sa = ang.sin();
        let px = -sa;
        let pz = ca;
        let frond_len = height * (1.08 + 0.16 * (f as f32 * 1.7).sin());
        let segs = 8usize;

        let v_start = b_foliage.positions.len();
        let i_start = b_foliage.indices.len();

        for seg in 0..segs {
            let t0 = seg as f32 / segs as f32;
            let t1 = (seg + 1) as f32 / segs as f32;
            let r0 = frond_len * t0;
            let r1 = frond_len * t1;
            let y0 = crown[1] + (t0 * PI * 0.80).sin() * height * 0.36 * tier
                - t0 * t0 * height * 0.28;
            let y1 = crown[1] + (t1 * PI * 0.80).sin() * height * 0.36 * tier
                - t1 * t1 * height * 0.28;

            // Bipinnate pinnae width envelope with serrated pinna flare
            let serrate0 = 1.0 + 0.22 * ((seg as f32) * 2.5).sin();
            let serrate1 = 1.0 + 0.22 * (((seg + 1) as f32) * 2.5).sin();
            let w0 = frond_len * 0.16 * (t0 * PI).sin().max(0.06) * serrate0;
            let w1 = frond_len * 0.16 * (t1 * PI).sin().max(0.03) * serrate1;

            let c0 = [crown[0] + ca * r0, y0, crown[2] + sa * r0];
            let c1 = [crown[0] + ca * r1, y1, crown[2] + sa * r1];
            let l0 = [c0[0] - px * w0, y0 - w0 * 0.18, c0[2] - pz * w0];
            let r0_p = [c0[0] + px * w0, y0 - w0 * 0.18, c0[2] + pz * w0];
            let l1 = [c1[0] - px * w1, y1 - w1 * 0.18, c1[2] - pz * w1];
            let r1_p = [c1[0] + px * w1, y1 - w1 * 0.18, c1[2] + pz * w1];

            let n_up = [ca * 0.22, 0.95, sa * 0.22];
            let col0 = lerp_color(
                [0.14, 0.36, 0.14, 1.0],
                [0.34, 0.62, 0.22, 1.0],
                t0 * 0.85 + 0.15 * tier,
            );
            let col1 = lerp_color(
                [0.14, 0.36, 0.14, 1.0],
                [0.34, 0.62, 0.22, 1.0],
                t1 * 0.85 + 0.15 * tier,
            );

            let vc0 = b_foliage.push_vertex(c0, n_up, col0, [0.5, t0]);
            let vl0 = b_foliage.push_vertex(l0, n_up, col0, [0.0, t0]);
            let vr0 = b_foliage.push_vertex(r0_p, n_up, col0, [1.0, t0]);
            let vc1 = b_foliage.push_vertex(c1, n_up, col1, [0.5, t1]);
            let vl1 = b_foliage.push_vertex(l1, n_up, col1, [0.0, t1]);
            let vr1 = b_foliage.push_vertex(r1_p, n_up, col1, [1.0, t1]);

            b_foliage.push_tri(vc0, vl0, vl1);
            b_foliage.push_tri(vc0, vl1, vc1);
            b_foliage.push_tri(vc0, vc1, vr1);
            b_foliage.push_tri(vc0, vr1, vr0);
        }
        b_foliage.recompute_normals_since(v_start, i_start);
    }
}

/// Build a 3D *Xanthorrhoea australis* (Austral Grass Tree) with a fire-charred caudex,
/// dense spherical skirt of 64+ fine drooping needles (green live crown + straw dead skirt),
/// and a tall velvety flowering scape spike.
pub fn add_xanthorrhoea_grass_tree(
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
    root: [f32; 3],
    height: f32,
    seed: f32,
) {
    let r_base = (height * 0.15).max(0.018) as f64;
    let trunk_axis = [
        Point3::new(root[0] as f64, (root[1] - 0.012) as f64, root[2] as f64),
        Point3::new(
            (root[0] + 0.014 * seed.cos()) as f64,
            (root[1] + height * 0.52) as f64,
            (root[2] + 0.014 * seed.sin()) as f64,
        ),
        Point3::new(root[0] as f64, (root[1] + height) as f64, root[2] as f64),
    ];
    let st = [
        ExtrusionStation {
            scale_x: r_base * 1.18,
            scale_y: r_base * 1.18,
            twist_rad: 0.0,
        },
        ExtrusionStation {
            scale_x: r_base * 0.98,
            scale_y: r_base * 0.98,
            twist_rad: 0.25,
        },
        ExtrusionStation {
            scale_x: r_base * 0.86,
            scale_y: r_base * 0.86,
            twist_rad: 0.50,
        },
    ];
    b_trunks.add_bishop_swept_tube(
        &trunk_axis,
        &st,
        7,
        [0.12, 0.10, 0.09, 1.0],
        [0.18, 0.15, 0.13, 1.0],
    );

    let crown = [root[0], root[1] + height, root[2]];
    let needle_count = 64usize;
    let v_start = b_foliage.positions.len();
    let i_start = b_foliage.indices.len();

    for n in 0..needle_count {
        let fn_i = n as f32;
        let frac = fn_i / needle_count as f32;
        let phi = fn_i * GOLDEN_ANGLE + seed;
        let ca = phi.cos();
        let sa = phi.sin();
        let px = -sa;
        let pz = ca;

        // Lower frac = upper live green needles; higher frac = lower drooping skirt
        let is_skirt = frac > 0.72;
        let needle_len = height * (0.78 + 0.32 * hash21(fn_i, seed));
        let w = (height * 0.022).max(0.0035);

        let arch_up = if is_skirt {
            -needle_len * 0.15
        } else {
            needle_len * (0.48 - 0.55 * frac)
        };
        let droop_y = if is_skirt {
            -needle_len * 0.72
        } else {
            arch_up - needle_len * (0.28 + 0.52 * frac)
        };
        let reach = needle_len * (0.55 + 0.38 * (PI * frac).sin());

        let (col_base, col_tip) = if is_skirt {
            ([0.46, 0.36, 0.20, 1.0], [0.64, 0.52, 0.28, 1.0])
        } else {
            ([0.18, 0.40, 0.16, 1.0], [0.42, 0.64, 0.24, 1.0])
        };

        let p0_l = [crown[0] - px * w, crown[1], crown[2] - pz * w];
        let p0_r = [crown[0] + px * w, crown[1], crown[2] + pz * w];
        let p1_l = [
            crown[0] + ca * reach * 0.52 - px * w * 0.65,
            crown[1] + arch_up,
            crown[2] + sa * reach * 0.52 - pz * w * 0.65,
        ];
        let p1_r = [
            crown[0] + ca * reach * 0.52 + px * w * 0.65,
            crown[1] + arch_up,
            crown[2] + sa * reach * 0.52 + pz * w * 0.65,
        ];
        let p2_tip = [
            crown[0] + ca * reach,
            crown[1] + droop_y,
            crown[2] + sa * reach,
        ];

        let norm = [ca * 0.35, 0.88, sa * 0.35];
        let col_mid = lerp_color(col_base, col_tip, 0.55);
        let v0 = b_foliage.push_vertex(p0_l, norm, col_base, [0.0, 0.0]);
        let v1 = b_foliage.push_vertex(p0_r, norm, col_base, [1.0, 0.0]);
        let v2 = b_foliage.push_vertex(p1_l, norm, col_mid, [0.2, 0.55]);
        let v3 = b_foliage.push_vertex(p1_r, norm, col_mid, [0.8, 0.55]);
        let v4 = b_foliage.push_vertex(p2_tip, norm, col_tip, [0.5, 1.0]);
        b_foliage.push_tri(v0, v1, v3);
        b_foliage.push_tri(v0, v3, v2);
        b_foliage.push_tri(v2, v3, v4);
    }
    b_foliage.recompute_normals_since(v_start, i_start);

    // Tall velvety flowering scape & nectarous spike
    let scape_h = height * 1.42;
    let scape_axis = [
        Point3::new(crown[0] as f64, crown[1] as f64, crown[2] as f64),
        Point3::new(
            (crown[0] + 0.01 * seed.sin()) as f64,
            (crown[1] + scape_h * 0.45) as f64,
            (crown[2] + 0.01 * seed.cos()) as f64,
        ),
        Point3::new(crown[0] as f64, (crown[1] + scape_h * 0.92) as f64, crown[2] as f64),
        Point3::new(crown[0] as f64, (crown[1] + scape_h) as f64, crown[2] as f64),
    ];
    let scape_st = [
        ExtrusionStation {
            scale_x: r_base * 0.24,
            scale_y: r_base * 0.24,
            twist_rad: 0.0,
        },
        ExtrusionStation {
            scale_x: r_base * 0.22,
            scale_y: r_base * 0.22,
            twist_rad: 0.1,
        },
        ExtrusionStation {
            scale_x: r_base * 0.30,
            scale_y: r_base * 0.30,
            twist_rad: 0.2,
        },
        ExtrusionStation {
            scale_x: r_base * 0.12,
            scale_y: r_base * 0.12,
            twist_rad: 0.3,
        },
    ];
    b_trunks.add_bishop_swept_tube(
        &scape_axis,
        &scape_st,
        6,
        [0.42, 0.32, 0.18, 1.0],
        [0.86, 0.80, 0.54, 1.0],
    );
}

/// Add a 3D Cosserat grass tussock (*Themeda triandra* Kangaroo Grass, *Poa sieberiana* Snow Grass),
/// riparian sedge (*Juncus* / *Carex*), or deeply submerged *Vallisneria australis* ribbon weed.
pub fn add_botanical_ground_tussock(
    b_foliage: &mut SubmeshBuilder,
    root: [f32; 3],
    blade_count: usize,
    height: f32,
    spread: f32,
    col_base: [f32; 4],
    col_tip: [f32; 4],
    stream_bias_z: f32,
    seed: f32,
) {
    let v_start = b_foliage.positions.len();
    let i_start = b_foliage.indices.len();

    for i in 0..blade_count {
        let fi = i as f32;
        let angle = (fi / blade_count.max(1) as f32) * TAU + hash21(seed, fi) * 0.48;
        let ca = angle.cos();
        let sa = angle.sin();
        let px = -sa;
        let pz = ca;

        let h = height * (0.72 + 0.36 * hash21(fi * 3.1, seed));
        let droop = spread * (0.68 + 0.48 * hash21(seed * 1.9, fi));
        let w_base = (height * 0.065).clamp(0.003, 0.011);
        let w_mid = w_base * 0.62;

        let p0_l = [root[0] - px * w_base, root[1], root[2] - pz * w_base];
        let p0_r = [root[0] + px * w_base, root[1], root[2] + pz * w_base];
        let p1_l = [
            root[0] + ca * droop * 0.44 - px * w_mid,
            root[1] + h * 0.62,
            root[2] + sa * droop * 0.44 + stream_bias_z * 0.45 - pz * w_mid,
        ];
        let p1_r = [
            root[0] + ca * droop * 0.44 + px * w_mid,
            root[1] + h * 0.62,
            root[2] + sa * droop * 0.44 + stream_bias_z * 0.45 + pz * w_mid,
        ];
        let p2_tip = [
            root[0] + ca * droop,
            root[1] + h * 0.88,
            root[2] + sa * droop + stream_bias_z,
        ];

        let norm = [ca * 0.35, 0.88, sa * 0.35];
        let col_mid = lerp_color(col_base, col_tip, 0.52);
        let v0 = b_foliage.push_vertex(p0_l, norm, col_base, [0.0, 0.0]);
        let v1 = b_foliage.push_vertex(p0_r, norm, col_base, [1.0, 0.0]);
        let v2 = b_foliage.push_vertex(p1_l, norm, col_mid, [0.2, 0.55]);
        let v3 = b_foliage.push_vertex(p1_r, norm, col_mid, [0.8, 0.55]);
        let v4 = b_foliage.push_vertex(p2_tip, norm, col_tip, [0.5, 1.0]);
        b_foliage.push_tri(v0, v1, v3);
        b_foliage.push_tri(v0, v3, v2);
        b_foliage.push_tri(v2, v3, v4);
    }

    b_foliage.recompute_normals_since(v_start, i_start);
}

/// Populate the complete multi-stratum botanical geometry for a `.10d` focal sector scene.
pub fn build_sector_botanical_stratum(
    sector_id: &str,
    water_y: f32,
    veg_roots_and_heights: &[([f32; 3], f32)],
    sample_elev: &dyn Fn(f32, f32) -> f32,
    road_center_x: &dyn Fn(f32) -> f32,
    b_trunks: &mut SubmeshBuilder,
    b_foliage: &mut SubmeshBuilder,
) {
    // 1. Emit the Multi-Species Canopy & Mid-Story Trees + Soft Tree Ferns + Austral Grass Trees
    for (i, &(root, h)) in veg_roots_and_heights.iter().enumerate() {
        let fi = i as f32;
        if i % 7 == 0 {
            // *Dicksonia antarctica* Soft Tree Fern in moist gullies
            add_dicksonia_tree_fern(b_trunks, b_foliage, root, h * 0.54, fi);
        } else if i % 6 == 0 {
            // *Xanthorrhoea australis* Austral Grass Tree with flowering scape
            add_xanthorrhoea_grass_tree(b_trunks, b_foliage, root, h * 0.48, fi);
        } else {
            let guild = match i % 5 {
                0 => TreeSpeciesGuild::GhostOrSnowGum,
                1 => TreeSpeciesGuild::GoldenWattle,
                2 => TreeSpeciesGuild::TallAshOrRedGum,
                3 => TreeSpeciesGuild::BanksiaOrWaratah,
                _ => TreeSpeciesGuild::PeppermintOrScribblyGum,
            };
            add_architectural_canopy_tree(
                b_trunks,
                b_foliage,
                root,
                h,
                guild,
                sector_id,
                fi + 0.37,
            );
        }
    }

    // 2. Dense Multi-Stratum Ground Cover:
    // - Dry slopes: *Themeda triandra* (Kangaroo Grass) & *Poa sieberiana* (Snow Grass)
    // - Water margin: *Juncus usitatus* & *Carex appressa* olive-russet sedges
    // - Deep riverbed: subtle dark-olive *Vallisneria australis* ribbon weed strictly below `water_y - 0.04`
    for i in 0..240 {
        let fi = i as f32;
        let x = -1.78 + 3.56 * hash21(fi * 4.3, 7.1);
        let z = -1.78 + 3.56 * hash21(fi * 8.9, 2.7);
        let road_x = road_center_x(z);
        if (x - road_x).abs() < 0.21 {
            continue;
        }
        // Keep tall grass tussocks out of the foreground camera lens cone (`z > 0.30` near the road)
        if z > 0.30 && (x - road_x).abs() < 0.46 {
            continue;
        }
        let y = sample_elev(x, z);

        if y < water_y - 0.08 {
            // Subtle, deeply submerged dark-olive benthic *Vallisneria australis* ribbon weed:
            // strictly capped below `water_y - 0.04` so NO spikes ever poke through the water!
            let max_submerged_h = (water_y - 0.045 - y).clamp(0.015, 0.095);
            if max_submerged_h > 0.016 {
                add_botanical_ground_tussock(
                    b_foliage,
                    [x, y, z],
                    5,
                    max_submerged_h,
                    0.038,
                    [0.12, 0.20, 0.08, 1.0],
                    [0.20, 0.30, 0.12, 1.0],
                    0.032,
                    fi,
                );
            }
        } else if y >= water_y + 0.015 && y < water_y + 0.11 {
            // Riparian *Juncus usitatus* rushes & *Carex* sedges along the sub-aerial bank margin
            add_botanical_ground_tussock(
                b_foliage,
                [x, y, z],
                9,
                0.095,
                0.058,
                [0.20, 0.34, 0.14, 1.0],
                [0.42, 0.52, 0.22, 1.0],
                0.0,
                fi,
            );
        } else if y >= water_y + 0.11 {
            // *Themeda triandra* (Kangaroo Grass) & *Poa sieberiana* (Snow Grass) tussocks
            let is_themeda = i % 2 == 0;
            let (col_b, col_t) = if is_themeda {
                ([0.36, 0.32, 0.15, 1.0], [0.66, 0.52, 0.24, 1.0])
            } else {
                ([0.24, 0.36, 0.18, 1.0], [0.46, 0.60, 0.30, 1.0])
            };
            add_botanical_ground_tussock(
                b_foliage,
                [x, y, z],
                8,
                0.082,
                0.064,
                col_b,
                col_t,
                0.0,
                fi,
            );
        }
    }
}

// White-gum review uses attached leaf-bearing shoots rather than metre-wide fan proxies.
// This specializes the retained botanical spray owner; other sector guilds are unchanged.
fn add_white_gum_crown_sprays(
    trunks: &mut SubmeshBuilder, foliage: &mut SubmeshBuilder,
    attach: [f32;3], center: [f32;3], rx:f32,ry:f32,rz:f32,
    twig:[f32;4], dark:[f32;4], lit:[f32;4], count:usize, seed:f32,
) {
    for i in 0..count {
        let f=(i as f32+0.5)/count as f32;
        let a=i as f32*GOLDEN_ANGLE+seed;
        let y=1.0-1.8*f;
        let ring=(1.0-y*y).max(0.10).sqrt();
        let radius=0.62+0.35*hash21(i as f32,seed);
        let target=[center[0]+a.cos()*rx*ring*radius,
            center[1]+y*ry,center[2]+a.sin()*rz*ring*radius];
        let vector=[target[0]-attach[0],target[1]-attach[1],target[2]-attach[2]];
        let length=dot3(vector,vector).sqrt().max(0.18);
        let direction=normalize3([vector[0],vector[1]+length*0.22,vector[2]]);
        add_botanical_foliage_spray_profile(trunks,foliage,attach,direction,length,
            0.145,0.022,18,twig,dark,lit,seed+i as f32*1.31,true);
    }
}
/// Browser review candidate for the working mountain-white-gum taxon concept.
/// Reuses the current sector's architectural builder and Core tapered branch fields.
/// Metres, origin at ground collar, +Y up. Authored 6 m stature, not age/measurement.
/// This surface candidate is unaccepted and does not identify a local individual.
pub fn build_white_gum_review_specimen(
    trunks: &mut SubmeshBuilder,
    foliage: &mut SubmeshBuilder,
) {
    add_architectural_canopy_tree(
        trunks,
        foliage,
        [0.0, 0.0, 0.0],
        6.0,
        TreeSpeciesGuild::MountainWhiteGum,
        "white-gum-review-synthetic",
        2.73,
    );
}

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
