//! Physically-Based Material Synthesis, Tangent-Frame Generation & Australian High Country IBL Probes.
//!
//! Provides high-detail `512×512` procedural sRGB base-colour textures with baked micro-relief
//! shading cues, deterministic UV-aligned tangent frames (`[tx, ty, tz, sign]`), and physically
//! energy-conserving `EnvironmentProbeSet` configurations for East Coast Australia study sites.

use crate::scene_builder_10d::ProceduralTextureAsset;
use image::ImageEncoder;
use qualia_core_db::render::lighting::environment_lighting::{
    EnvironmentProbe, EnvironmentProbeSet, ENVIRONMENT_SPECULAR_LEVELS,
};
use qualia_core_db::render::mesh_tangent_frames::{tangents_from_uvs_into, TangentFrameError};
use sha2::{Digest, Sha256};
use std::f32::consts::TAU;

/// Canonical resolution (`512×512`) for high-detail procedural terrain and botanical textures.
pub const HIGH_DETAIL_TEXTURE_DIM: u32 = 512;

/// Number of `f32` values per packed `EnvironmentProbe` record in `QualiaPortal::set_environment_probe_data`.
pub const PORTAL_PROBE_RECORD_FLOATS: usize = 3 + 1 + 3 + ENVIRONMENT_SPECULAR_LEVELS * 3 + 3;

#[inline]
fn hash21(x: f32, y: f32) -> f32 {
    let qx = (x * 127.1 + y * 311.7).sin() * 43758.5453;
    qx - qx.floor()
}

#[inline]
fn hash22(x: f32, y: f32) -> [f32; 2] {
    let a = (x * 127.1 + y * 311.7).sin() * 43758.5453;
    let b = (x * 269.5 + y * 183.3).sin() * 43758.5453;
    [a - a.floor(), b - b.floor()]
}

#[inline]
fn smooth_noise(x: f32, y: f32) -> f32 {
    let ix = x.floor();
    let iy = y.floor();
    let fx = x - ix;
    let fy = y - iy;
    let ux = fx * fx * (3.0 - 2.0 * fx);
    let uy = fy * fy * (3.0 - 2.0 * fy);
    let a = hash21(ix, iy);
    let b = hash21(ix + 1.0, iy);
    let c = hash21(ix, iy + 1.0);
    let d = hash21(ix + 1.0, iy + 1.0);
    a * (1.0 - ux) * (1.0 - uy)
        + b * ux * (1.0 - uy)
        + c * (1.0 - ux) * uy
        + d * ux * uy
}

#[inline]
fn fbm5(mut x: f32, mut y: f32) -> f32 {
    let mut v = 0.0_f32;
    let mut amp = 0.5_f32;
    for _ in 0..5 {
        v += amp * smooth_noise(x, y);
        let nx = x * 1.97 + y * 0.48 + 17.3;
        let ny = y * 1.97 - x * 0.48 + 31.7;
        x = nx;
        y = ny;
        amp *= 0.5;
    }
    v
}

#[inline]
fn toroidal_fbm(u: f32, v: f32, radius: f32, seed_x: f32, seed_y: f32) -> f32 {
    let ang_u = u * TAU;
    let ang_v = v * TAU;
    let x = ang_u.cos() * radius + ang_v.sin() * (radius * 0.45) + seed_x;
    let y = ang_v.cos() * radius - ang_u.sin() * (radius * 0.45) + seed_y;
    fbm5(x, y)
}

#[inline]
fn toroidal_voronoi(u: f32, v: f32, cells: i32, seed: f32) -> (f32, f32, f32, f32) {
    let cu = u * cells as f32;
    let cv = v * cells as f32;
    let ix = cu.floor() as i32;
    let iy = cv.floor() as i32;
    let fx = cu - ix as f32;
    let fy = cv - iy as f32;

    let mut d1 = 8.0_f32;
    let mut d2 = 8.0_f32;
    let mut best_id = 0.5_f32;
    let mut best_dx = 0.0_f32;
    let mut best_dy = 0.0_f32;

    for dy in -1..=1 {
        for dx in -1..=1 {
            let wx = (ix + dx).rem_euclid(cells) as f32;
            let wy = (iy + dy).rem_euclid(cells) as f32;
            let [jx, jy] = hash22(wx + seed * 13.7, wy + seed * 29.3);
            let px = dx as f32 + 0.14 + 0.72 * jx - fx;
            let py = dy as f32 + 0.14 + 0.72 * jy - fy;
            let dist = (px * px * 1.08 + py * py * 1.18).sqrt();
            if dist < d1 {
                d2 = d1;
                d1 = dist;
                best_id = hash21(wx * 19.1 + seed, wy * 37.7 + seed);
                best_dx = px;
                best_dy = py;
            } else if dist < d2 {
                d2 = dist;
            }
        }
    }
    let edge = (d2 - d1).clamp(0.0, 1.0);
    // Directional bevel cue from cell center (-best_dx, -best_dy)
    let bevel = ((-best_dx * 0.70 - best_dy * 0.70) / (d1 + 0.25)).clamp(-1.0, 1.0);
    (d1, edge, best_id, bevel)
}

fn sha256_pair(bytes: &[u8]) -> ([u8; 32], String) {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&digest);
    let mut hex = String::with_capacity(64);
    for b in arr {
        hex.push_str(&format!("{b:02x}"));
    }
    (arr, hex)
}

fn encode_rgba8_to_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let mut png = Vec::new();
    image::codecs::png::PngEncoder::new(&mut png)
        .write_image(rgba, width, height, image::ExtendedColorType::Rgba8)
        .expect("PNG encode");
    png
}

/// Synthesize the 4 canonical `512×512` high-detail procedural RGBA8 textures in exact order:
/// `0: "foliage_leaf_venation"`
/// `1: "bark_fibrous_mottled"`
/// `2: "riverbed_pebbles_lithology"`
/// `3: "slga_soil_strata_litter"`
///
/// Luminance is balanced in `[0.68, 1.0]` with crisp micro-contrast and normal-like directional
/// bevel cues so `multiply_vertex_color = true` materials gain sharp organic texture without mud.
pub fn build_high_detail_procedural_textures() -> Vec<ProceduralTextureAsset> {
    let w = HIGH_DETAIL_TEXTURE_DIM;
    let h = HIGH_DETAIL_TEXTURE_DIM;
    let inv_w = 1.0 / (w - 1) as f32;
    let inv_h = 1.0 / (h - 1) as f32;
    let mut out = Vec::with_capacity(4);

    // 0. "foliage_leaf_venation": crisp central midrib, pinnate 45° secondary veins branching
    // to an intramarginal vein, translucent essential-oil gland punctations, and waxy cuticle sheen.
    let mut leaf_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        let v = y as f32 * inv_h;
        let midrib_half_w = (0.028 * (1.0 - 0.55 * v)).max(0.008);
        for x in 0..w {
            let u = x as f32 * inv_w;
            let signed_du = u - 0.5;
            let midrib_dist = signed_du.abs();

            // Primary central midrib with directional ridge highlight
            let midrib_core = (1.0 - (midrib_dist / midrib_half_w).clamp(0.0, 1.0)).powf(1.65);
            let midrib_bevel = (-signed_du / (midrib_half_w * 2.5)).clamp(-1.0, 1.0) * midrib_core;

            // Pinnate secondary veins at 45° branching toward the intramarginal vein
            let pinnate_phase = (v * 28.0 - midrib_dist * 26.0) * TAU;
            let sec_ridge = (pinnate_phase.cos() * 0.5 + 0.5).powf(6.5);
            let sec_bevel = pinnate_phase.sin() * 0.04 * sec_ridge;
            let sec_envelope = (1.0 - (midrib_dist / 0.44).clamp(0.0, 1.0)).powf(0.55)
                * (midrib_dist / 0.025).clamp(0.0, 1.0);
            let secondary_vein = sec_ridge * sec_envelope;

            // Tertiary reticulate venation network + eucalyptus intramarginal vein near margin
            let tert_phase = ((u * 54.0 + v * 48.0).sin().abs() * (u * 46.0 - v * 58.0).cos().abs()).powf(2.4);
            let intramarginal = (1.0 - ((midrib_dist - 0.41).abs() / 0.018).clamp(0.0, 1.0)).powf(2.0);

            // Translucent eucalyptus oil-gland punctations in the mesophyll areoles
            let (_, _, gland_id, _) = toroidal_voronoi(u, v, 38, 3.1);
            let gland_dot = if gland_id > 0.74 {
                (gland_id - 0.74) * 3.8 * (1.0 - midrib_core)
            } else {
                0.0
            };

            // Waxy cuticle micro-sheen & stomatal cellular grain
            let cuticle_grain = toroidal_fbm(u, v, 6.5, 2.3, 7.9) * 0.10;
            let margin_shade = 1.0 - 0.10 * (midrib_dist * 2.0).powf(2.6);

            let vascular = (0.18 * midrib_core + 0.11 * secondary_vein + 0.07 * intramarginal + 0.04 * tert_phase)
                .clamp(0.0, 0.26);
            let bevel_cue = 0.05 * midrib_bevel + sec_bevel;

            let r = ((0.76 + vascular * 1.15 + 0.08 * gland_dot + cuticle_grain + bevel_cue) * margin_shade)
                .clamp(0.68, 1.0);
            let g = ((0.84 + vascular * 0.95 + 0.09 * gland_dot + cuticle_grain + bevel_cue) * margin_shade)
                .clamp(0.72, 1.0);
            let b = ((0.73 + vascular * 0.55 + 0.03 * gland_dot + cuticle_grain * 0.75 + bevel_cue * 0.6)
                * margin_shade)
                .clamp(0.68, 0.98);

            let idx = ((y * w + x) * 4) as usize;
            leaf_rgba[idx] = (r * 255.0).round() as u8;
            leaf_rgba[idx + 1] = (g * 255.0).round() as u8;
            leaf_rgba[idx + 2] = (b * 255.0).round() as u8;
            leaf_rgba[idx + 3] = 255;
        }
    }

    // 1. "bark_fibrous_mottled": longitudinal eucalypt ribbon/stringybark fibres interwoven
    // with smooth mottled Corymbia/Snow-Gum exfoliating bark plates and ochre/lichen streaks.
    let mut bark_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        let v = y as f32 * inv_h;
        for x in 0..w {
            let u = x as f32 * inv_w;

            // Smooth exfoliating Corymbia / Snow Gum bark plates (Voronoi + multi-octave FBM)
            let (_, plate_edge, plate_id, plate_bevel) = toroidal_voronoi(u, v, 9, 5.7);
            let plate_mottle = toroidal_fbm(u, v, 3.4, 4.2, 9.1);
            let scar_line = (1.0 - (plate_edge / 0.09).clamp(0.0, 1.0)).powf(1.8);

            // Longitudinal fibrous stringybark / ribbon grain warped by natural trunk sinuosity
            let warp = 0.045 * (v * TAU * 2.0 + plate_mottle * 3.2).sin();
            let fibre_u = (u + warp).rem_euclid(1.0);
            let fibre_wave = ((fibre_u * TAU * 36.0).sin() * 0.5 + 0.5)
                * (0.65 + 0.35 * ((v * TAU * 6.0).cos() * 0.5 + 0.5));
            let micro_fibre = toroidal_fbm(fibre_u * 2.0, v, 5.2, 11.4, 3.8);

            // Blend smooth gum plates (65%) with longitudinal fibrous ribbons (35%) + lichen/ochre nuances
            let ochre_streak = (plate_id - 0.55).max(0.0) * 0.32;
            let lichen_fleck = if micro_fibre > 0.66 && plate_mottle > 0.54 {
                0.06
            } else {
                0.0
            };

            let base_lum = 0.78
                + 0.10 * plate_mottle
                + 0.07 * fibre_wave
                + 0.05 * micro_fibre
                + 0.04 * plate_bevel
                - 0.09 * scar_line;
            let lum = base_lum.clamp(0.70, 0.99);

            let r = (lum * (0.98 + 0.06 * ochre_streak - 0.03 * lichen_fleck)).clamp(0.70, 1.0);
            let g = (lum * (0.96 + 0.02 * ochre_streak + 0.04 * lichen_fleck)).clamp(0.69, 0.99);
            let b = (lum * (0.93 - 0.05 * ochre_streak - 0.02 * lichen_fleck)).clamp(0.68, 0.97);

            let idx = ((y * w + x) * 4) as usize;
            bark_rgba[idx] = (r * 255.0).round() as u8;
            bark_rgba[idx + 1] = (g * 255.0).round() as u8;
            bark_rgba[idx + 2] = (b * 255.0).round() as u8;
            bark_rgba[idx + 3] = 255;
        }
    }

    // 2. "riverbed_pebbles_lithology": multi-scale Voronoi river cobbles & mineral grain texture
    // with quartz, jasper, and Kosciuszko granodiorite crystal speckle and interstitial crevices.
    let mut pebble_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        let v = y as f32 * inv_h;
        for x in 0..w {
            let u = x as f32 * inv_w;

            let (d_macro, edge_macro, id_macro, bevel_macro) = toroidal_voronoi(u, v, 14, 1.9);
            let (d_micro, edge_micro, id_micro, bevel_micro) = toroidal_voronoi(u, v, 32, 8.3);

            let cobble_dome = (1.0 - (d_macro / 0.62).clamp(0.0, 1.0)).powf(0.55);
            let gravel_dome = (1.0 - (d_micro / 0.58).clamp(0.0, 1.0)).powf(0.60);
            let crevice = (edge_macro / 0.11).clamp(0.0, 1.0) * (0.75 + 0.25 * (edge_micro / 0.12).clamp(0.0, 1.0));

            // Mineral petrology palette (quartz, warm ironstone/jasper, biotite granodiorite, Silurian karst)
            let (mr, mg, mb) = if id_macro > 0.76 {
                (0.92, 0.90, 0.86) // Milky quartz & orthoclase feldspar cobble
            } else if id_macro > 0.50 {
                (0.88, 0.80, 0.73) // Warm jasper / Hawkesbury ironstone-sandstone cobble
            } else if id_macro > 0.24 {
                (0.83, 0.82, 0.80) // Kosciuszko granodiorite cobble
            } else {
                (0.77, 0.79, 0.82) // Silurian karst limestone / basalt cobble
            };

            // High-frequency crystalline mica/feldspar speckle inside rock grains
            let crystal_hash = hash21((x as f32) * 0.73 + id_micro * 19.0, (y as f32) * 0.73);
            let crystal_sparkle = if crystal_hash > 0.84 {
                0.06
            } else if crystal_hash < 0.16 {
                -0.05
            } else {
                0.0
            };
            let rock_fbm = toroidal_fbm(u, v, 4.8, 6.1, 2.7) * 0.08;
            let relief_shade = (0.74
                + 0.16 * (0.70 * cobble_dome + 0.30 * gravel_dome)
                + 0.06 * bevel_macro
                + 0.03 * bevel_micro
                + rock_fbm
                + crystal_sparkle)
                * (0.82 + 0.18 * crevice);

            let r = (mr * relief_shade).clamp(0.68, 1.0);
            let g = (mg * relief_shade).clamp(0.68, 0.99);
            let b = (mb * relief_shade).clamp(0.68, 0.98);

            let idx = ((y * w + x) * 4) as usize;
            pebble_rgba[idx] = (r * 255.0).round() as u8;
            pebble_rgba[idx + 1] = (g * 255.0).round() as u8;
            pebble_rgba[idx + 2] = (b * 255.0).round() as u8;
            pebble_rgba[idx + 3] = 255;
        }
    }

    // 3. "slga_soil_strata_litter": seamless toroidal forest floor with overlapping fallen
    // eucalypt leaves, fine twigs, humus granules, and mossy bryophyte micro-patches.
    let mut soil_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        let v = y as f32 * inv_h;
        for x in 0..w {
            let u = x as f32 * inv_w;

            let macro_loam = toroidal_fbm(u, v, 3.2, 5.0, 5.0);
            let humus_grain = toroidal_fbm(u, v, 9.5, 14.2, 8.7);

            // Procedural overlapping fallen lance-shaped sclerophyll leaves & twigs on toroidal grid
            let (_, _, leaf_cell_id, leaf_bevel) = toroidal_voronoi(u, v, 18, 4.4);
            let angle = leaf_cell_id * TAU;
            let (sin_a, cos_a) = angle.sin_cos();
            let local_u = ((u * 18.0).fract() - 0.5) * cos_a - ((v * 18.0).fract() - 0.5) * sin_a;
            let local_v = ((u * 18.0).fract() - 0.5) * sin_a + ((v * 18.0).fract() - 0.5) * cos_a;
            let lance_shape = (1.0 - ((local_u * 3.6).powi(2) + (local_v * 1.35).powi(2))).clamp(0.0, 1.0);
            let fallen_leaf = lance_shape.powf(0.75) * if leaf_cell_id > 0.32 { 1.0 } else { 0.0 };

            // Mossy bryophyte micro-patches in damp loam hollows
            let moss_mask = ((macro_loam - 0.52) * 3.2).clamp(0.0, 1.0) * (1.0 - 0.6 * fallen_leaf);

            let lum = (0.76
                + 0.10 * macro_loam
                + 0.08 * humus_grain
                + 0.06 * fallen_leaf
                + 0.03 * leaf_bevel)
                .clamp(0.70, 0.98);

            let r = (lum * (0.97 + 0.05 * fallen_leaf - 0.04 * moss_mask)).clamp(0.69, 1.0);
            let g = (lum * (0.95 + 0.01 * fallen_leaf + 0.05 * moss_mask)).clamp(0.70, 0.99);
            let b = (lum * (0.90 - 0.04 * fallen_leaf - 0.03 * moss_mask)).clamp(0.68, 0.95);

            let idx = ((y * w + x) * 4) as usize;
            soil_rgba[idx] = (r * 255.0).round() as u8;
            soil_rgba[idx + 1] = (g * 255.0).round() as u8;
            soil_rgba[idx + 2] = (b * 255.0).round() as u8;
            soil_rgba[idx + 3] = 255;
        }
    }

    for (name, rgba8) in [
        ("foliage_leaf_venation", leaf_rgba),
        ("bark_fibrous_mottled", bark_rgba),
        ("riverbed_pebbles_lithology", pebble_rgba),
        ("slga_soil_strata_litter", soil_rgba),
    ] {
        let png_bytes = encode_rgba8_to_png(w, h, &rgba8);
        let (sha256_bytes, sha256_hex) = sha256_pair(&png_bytes);
        let hmc_key = format!("textures/sha256/{sha256_hex}");
        out.push(ProceduralTextureAsset {
            name,
            hmc_key,
            sha256_bytes,
            sha256_hex,
            width: w,
            height: h,
            rgba8,
            png_bytes,
        });
    }

    out
}

#[inline]
fn fallback_orthonormal_tangent(normal: [f32; 3]) -> [f32; 4] {
    let n_len_sq = normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2];
    let n = if n_len_sq > 1e-12 && n_len_sq.is_finite() {
        let inv = n_len_sq.sqrt().recip();
        [normal[0] * inv, normal[1] * inv, normal[2] * inv]
    } else {
        [0.0, 1.0, 0.0]
    };
    let axis = if n[0].abs() < 0.8 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let cx = axis[1] * n[2] - axis[2] * n[1];
    let cy = axis[2] * n[0] - axis[0] * n[2];
    let cz = axis[0] * n[1] - axis[1] * n[0];
    let c_len = (cx * cx + cy * cy + cz * cz).max(1e-20).sqrt();
    [cx / c_len, cy / c_len, cz / c_len, 1.0]
}

/// Caller-buffered wrapper around `qualia_core_db::render::mesh_tangent_frames::tangents_from_uvs_into`.
pub fn compute_mikktspace_tangents_into(
    positions: &[[f32; 3]],
    uvs: &[[f32; 2]],
    normals: &[[f32; 3]],
    indices: &[u32],
    out: &mut [[f32; 4]],
    scratch: &mut [[f32; 6]],
) -> Result<usize, TangentFrameError> {
    tangents_from_uvs_into(positions, uvs, normals, indices, out, scratch)
}

/// Compute per-vertex UV-aligned `[tx, ty, tz, sign]` tangent frames for `.10d` compilation
/// and `PortalGpu` normal-mapped PBR shading.
pub fn compute_scene_tangent_frames(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    indices: &[u32],
) -> Vec<[f32; 4]> {
    let n = positions.len();
    if n == 0 {
        return Vec::new();
    }
    let mut out = vec![[1.0_f32, 0.0, 0.0, 1.0]; n];
    let mut scratch = vec![[0.0_f32; 6]; n];

    let kernel_ok = if normals.len() == n && uvs.len() == n && indices.len() >= 3 && indices.len() % 3 == 0 {
        compute_mikktspace_tangents_into(positions, uvs, normals, indices, &mut out, &mut scratch)
            .is_ok()
    } else {
        false
    };

    for i in 0..n {
        let t = out[i];
        let len_sq = t[0] * t[0] + t[1] * t[1] + t[2] * t[2];
        let valid_sign = (t[3].abs() - 1.0).abs() <= 1e-3;
        if !kernel_ok || !len_sq.is_finite() || len_sq <= 1e-8 || !valid_sign {
            let nrm = normals.get(i).copied().unwrap_or([0.0, 1.0, 0.0]);
            out[i] = fallback_orthonormal_tangent(nrm);
        } else {
            let inv = len_sq.sqrt().recip();
            let sign = if t[3] < 0.0 { -1.0 } else { 1.0 };
            out[i] = [t[0] * inv, t[1] * inv, t[2] * inv, sign];
        }
    }
    out
}

/// Build a physically plausible, energy-conserving 2-probe `EnvironmentProbeSet` tuned per sector:
/// - Probe 0: Cool Zenith Sky Hemisphere fill (`dominant_direction = [0.0, 1.0, 0.0]`) with
///   roughness-attenuated specular levels (`0..5`) so water, rover glass/paint, and waxy leaves
///   catch crisp sky reflections while rough soil and bark receive zero grazing Fresnel haze.
/// - Probe 1: Warm Australian Solar-Canopy / Lithology Ground Bounce (`dominant_direction` aligned
///   with the lateral sun/valley bounce) so shaded relief retains natural 3D depth without any
///   additive gray veil (`direct_fallback = [0.0, 0.0, 0.0]`).
pub fn build_australian_high_country_probe_set(sector_id: &str) -> EnvironmentProbeSet {
    let (zenith_irr, zenith_spec0, bounce_irr, bounce_dir) = match sector_id {
        "talbingo" => (
            [0.34_f32, 0.42, 0.52],
            [0.16_f32, 0.21, 0.27],
            [0.32_f32, 0.28, 0.20],
            [0.48_f32, 0.36, 0.44],
        ),
        "yarrangobilly" => (
            [0.30_f32, 0.44, 0.52],
            [0.15_f32, 0.22, 0.27],
            [0.28_f32, 0.30, 0.26],
            [-0.22_f32, 0.38, 0.56],
        ),
        "three_mile_dam" => (
            [0.36_f32, 0.46, 0.58],
            [0.18_f32, 0.23, 0.30],
            [0.30_f32, 0.29, 0.22],
            [0.48_f32, 0.35, 0.48],
        ),
        "sydney_sandstone" => (
            [0.36_f32, 0.43, 0.50],
            [0.17_f32, 0.20, 0.25],
            [0.38_f32, 0.29, 0.19],
            [0.40_f32, 0.38, 0.44],
        ),
        "noosa_headland" => (
            [0.32_f32, 0.48, 0.58],
            [0.17_f32, 0.24, 0.30],
            [0.34_f32, 0.31, 0.24],
            [0.32_f32, 0.40, 0.44],
        ),
        "jounama_karst" => (
            [0.33_f32, 0.41, 0.52],
            [0.16_f32, 0.20, 0.26],
            [0.31_f32, 0.27, 0.22],
            [0.44_f32, 0.36, 0.46],
        ),
        _ => (
            // Botanical / Zoological Dissection Lab studio hemisphere probe pair
            [0.36_f32, 0.42, 0.50],
            [0.17_f32, 0.20, 0.25],
            [0.32_f32, 0.28, 0.22],
            [0.42_f32, 0.45, 0.45],
        ),
    };

    // Roughness-attenuated specular pyramid (0 = mirror, 4 = rough diffuse) prevents
    // unattenuated grazing Schlick Fresnel from washing out rough terrain or bark.
    let make_specular_pyramid = |s0: [f32; 3]| -> [[f32; 3]; ENVIRONMENT_SPECULAR_LEVELS] {
        [
            s0,
            [s0[0] * 0.65, s0[1] * 0.65, s0[2] * 0.65],
            [s0[0] * 0.36, s0[1] * 0.36, s0[2] * 0.36],
            [s0[0] * 0.16, s0[1] * 0.16, s0[2] * 0.16],
            [s0[0] * 0.06, s0[1] * 0.06, s0[2] * 0.06],
        ]
    };

    let mut set = EnvironmentProbeSet::new();
    // Probe 0: Zenith sky hemisphere probe covering the normalized orbit frame & metric sector
    let _ = set.push(EnvironmentProbe {
        position: [0.0, 0.65, 0.0],
        radius: 14.0,
        irradiance: zenith_irr,
        specular_levels: make_specular_pyramid(zenith_spec0),
        dominant_direction: [0.0, 1.0, 0.0],
        valid: true,
    });
    // Probe 1: Warm Australian sunlight-canopy-ground bounce hemisphere probe
    let bounce_spec0 = [
        zenith_spec0[0] * 0.72,
        zenith_spec0[1] * 0.68,
        zenith_spec0[2] * 0.58,
    ];
    let _ = set.push(EnvironmentProbe {
        position: [0.0, -0.25, 0.0],
        radius: 14.0,
        irradiance: bounce_irr,
        specular_levels: make_specular_pyramid(bounce_spec0),
        dominant_direction: bounce_dir,
        valid: true,
    });
    set
}

/// Pack an `EnvironmentProbeSet` into the flat `25 × N` `f32` slice consumed by
/// `QualiaPortal::set_environment_probe_data(&packed, &[0.0, 0.0, 0.0])`.
pub fn pack_environment_probe_set_for_portal(probes: &EnvironmentProbeSet) -> Vec<f32> {
    let slice = probes.as_slice();
    let mut out = Vec::with_capacity(slice.len() * PORTAL_PROBE_RECORD_FLOATS);
    for probe in slice {
        if !probe.valid || probe.radius <= 0.0 {
            continue;
        }
        out.extend_from_slice(&probe.position);
        out.push(probe.radius);
        out.extend_from_slice(&probe.irradiance);
        for level in &probe.specular_levels {
            out.extend_from_slice(level);
        }
        out.extend_from_slice(&probe.dominant_direction);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_detail_procedural_textures_match_canonical_names_and_512_resolution() {
        let assets = build_high_detail_procedural_textures();
        assert_eq!(assets.len(), 4);
        assert_eq!(
            assets.iter().map(|a| a.name).collect::<Vec<_>>(),
            [
                "foliage_leaf_venation",
                "bark_fibrous_mottled",
                "riverbed_pebbles_lithology",
                "slga_soil_strata_litter",
            ]
        );
        for asset in &assets {
            assert_eq!(asset.width, 512);
            assert_eq!(asset.height, 512);
            assert_eq!(asset.rgba8.len(), 512 * 512 * 4);
            assert!(asset.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
            assert!(asset.rgba8.chunks_exact(4).all(|px| px[3] == 255));
        }
    }

    #[test]
    fn tangent_frames_are_unit_length_with_valid_handedness() {
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let normals = [[0.0, 0.0, 1.0]; 3];
        let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
        let indices = [0, 1, 2];
        let tangents = compute_scene_tangent_frames(&positions, &normals, &uvs, &indices);
        assert_eq!(tangents.len(), 3);
        for t in tangents {
            let len = (t[0] * t[0] + t[1] * t[1] + t[2] * t[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-4);
            assert!((t[3].abs() - 1.0).abs() < 1e-4);
        }
    }

    #[test]
    fn high_country_probe_set_packs_to_portal_contract() {
        let probes = build_australian_high_country_probe_set("talbingo");
        assert_eq!(probes.len(), 2);
        let packed = pack_environment_probe_set_for_portal(&probes);
        assert_eq!(packed.len(), 2 * PORTAL_PROBE_RECORD_FLOATS);
        assert!(packed.iter().all(|v| v.is_finite()));
    }
}
