//! Multi-Scale Geomorphological Terrain DAG, Embedded Real Talbingo DEM Sampler,
//! CSIRO SLGA 6-Horizon Subsurface Cutaway, and Jointed Lithology / Point-Bar Builder.
//!
//! Synthesizes authentic Australian East Coast & Snowy Mountains geomorphology for
//! `ecopunk-terrain` `.10d` sectors by combining:
//! - Bilinearly sampled real geodetic DEM elevation data (`talbingo_128x128_elev_mm.bin`,
//!   EPSG:7844 GDA2020 / AHD millimeter grid) with multi-scale high-pass topographic
//!   spurs, side-gullies, and structural benches.
//! - `qualia_core_db::domains::geospatial::terrain` procedural landform primitives
//!   (`LandformKind`, `sample_landform`), Swiss-turbulence / ridged multifractal cascades
//!   (`evaluate_fractal_2d`), anisotropic Gabor strike/jointing (`gabor_anisotropic_2d`),
//!   and Voronoi cellular rock fracture fields (`voronoi_2d`).
//! - Engineered cut-and-fill road benching along the Hermite corridor centerline so
//!   dedicated road ribbon geometry rests cleanly without terrain-grid color aliasing.
//! - Clustered, imbricated riparian cobble point-bars, submerged riffle shelves, and
//!   jointed granite tors, Hawkesbury sandstone ledges, karst limestone pinnacles, and
//!   columnar basalt outcrops.

use crate::scene_builder_10d::SubmeshBuilder;
use qualia_core_db::domains::geospatial::terrain::{
    evaluate_fractal_2d, gabor_anisotropic_2d, opensimplex2s_2d, perlin_2d, sample_landform,
    voronoi_2d, FractalConfig, FractalMode, LandformKind, LandformParams,
};
use qualia_core_db::specialized_libs::computational_geometry::botanical_hulls::{
    asymmetric_hull_capacity, asymmetric_hull_into, AsymmetricHullParams,
};
use qualia_core_db::specialized_libs::computational_geometry::Point3;

/// Embedded 128×128 little-endian `i32` millimeter elevation raster for Talbingo Gorge (65,536 bytes).
const TALBINGO_DEM_128_MM_BYTES: &[u8] =
    include_bytes!("../../../data/geospatial/talbingo_128x128_elev_mm.bin");

const DEM_GRID_DIM: usize = 128;
const DEM_MIN_ELEV_MM: f32 = 385_000.0;
const DEM_MAX_ELEV_MM: f32 = 1_180_000.0;

#[inline]
fn hash21(x: f32, y: f32) -> f32 {
    let p3x = (x * 0.1031).fract();
    let p3y = (y * 0.1030).fract();
    let p3z = (x * 0.0973 + y * 0.1099).fract();
    let dot = p3x * (p3y + 33.33) + p3y * (p3z + 33.33) + p3z * (p3x + 33.33);
    ((p3x + p3y + dot) * (p3z + dot)).fract().abs()
}

#[inline]
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0).max(1e-6)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn quintic_smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0).max(1e-6)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Deterministic sector phase seed used across river, road, and terrain synthesis.
#[inline]
pub fn sector_seed_from_id(sector_id: &str) -> f32 {
    match sector_id {
        "talbingo" | "talbingo_pines" | "humes_crossing" => 1.1_f32,
        "yarrangobilly" | "yarrangobilly_cliff" | "jounama_karst" => 2.4_f32,
        "three_mile_dam" | "ohares_alpine_ash" => 3.7_f32,
        "sydney_sandstone" => 5.1_f32,
        "noosa_headland" => 6.5_f32,
        _ => 7.9_f32,
    }
}

/// Deterministic integer seed for `qualia_core_db::domains::geospatial::terrain` noise kernels.
#[inline]
fn sector_u64_seed(sector_id: &str) -> u64 {
    match sector_id {
        "talbingo" | "talbingo_pines" | "humes_crossing" => 0x5441_4C42_494E_474F,
        "yarrangobilly" | "yarrangobilly_cliff" => 0x5941_5252_414E_474F,
        "jounama_karst" => 0x4A4F_554E_414D_414B,
        "three_mile_dam" | "ohares_alpine_ash" => 0x5448_5245_454D_494C,
        "sydney_sandstone" => 0x4841_574B_4553_4255,
        "noosa_headland" => 0x4E4F_4F53_4148_444C,
        _ => 0x4543_4F50_554E_4B31,
    }
}

/// Read a single cell `[ix, iz]` from the embedded 128×128 Talbingo DEM in normalized `[0, 1]` range.
#[inline]
fn read_dem_cell_norm(ix: usize, iz: usize) -> f32 {
    let cx = ix.min(DEM_GRID_DIM - 1);
    let cz = iz.min(DEM_GRID_DIM - 1);
    let byte_offset = (cz * DEM_GRID_DIM + cx) * 4;
    if byte_offset + 4 <= TALBINGO_DEM_128_MM_BYTES.len() {
        let mm = i32::from_le_bytes([
            TALBINGO_DEM_128_MM_BYTES[byte_offset],
            TALBINGO_DEM_128_MM_BYTES[byte_offset + 1],
            TALBINGO_DEM_128_MM_BYTES[byte_offset + 2],
            TALBINGO_DEM_128_MM_BYTES[byte_offset + 3],
        ]) as f32;
        ((mm - DEM_MIN_ELEV_MM) / (DEM_MAX_ELEV_MM - DEM_MIN_ELEV_MM)).clamp(0.0, 1.0)
    } else {
        0.25
    }
}

/// Bilinearly sample the embedded 128×128 Talbingo DEM at normalized UV `[0, 1]²`.
#[inline]
fn sample_dem_bilinear_norm(u: f32, v: f32) -> f32 {
    let uc = u.clamp(0.0, 1.0) * (DEM_GRID_DIM - 1) as f32;
    let vc = v.clamp(0.0, 1.0) * (DEM_GRID_DIM - 1) as f32;
    let x0 = uc.floor() as usize;
    let z0 = vc.floor() as usize;
    let x1 = (x0 + 1).min(DEM_GRID_DIM - 1);
    let z1 = (z0 + 1).min(DEM_GRID_DIM - 1);
    let fx = uc - uc.floor();
    let fz = vc - vc.floor();
    let wx = fx * fx * (3.0 - 2.0 * fx);
    let wz = fz * fz * (3.0 - 2.0 * fz);

    let h00 = read_dem_cell_norm(x0, z0);
    let h10 = read_dem_cell_norm(x1, z0);
    let h01 = read_dem_cell_norm(x0, z1);
    let h11 = read_dem_cell_norm(x1, z1);

    let hx0 = h00 + wx * (h10 - h00);
    let hx1 = h01 + wx * (h11 - h01);
    hx0 + wz * (hx1 - hx0)
}

/// Extract both macro DEM elevation and local high-pass micro-topographic spur/gully detail.
#[inline]
fn sample_dem_macro_and_detail(x: f32, z: f32, seed: f32) -> (f32, f32) {
    // Map world domain [-2.2, 2.2] into [0.06, 0.94] with a deterministic sector offset
    let u_shift = 0.07 * (seed * 1.3).sin();
    let v_shift = 0.07 * (seed * 0.9).cos();
    let u = ((x + 2.2) / 4.4 * 0.84 + 0.08 + u_shift).clamp(0.02, 0.98);
    let v = ((z + 2.2) / 4.4 * 0.84 + 0.08 + v_shift).clamp(0.02, 0.98);

    let center = sample_dem_bilinear_norm(u, v);
    let span = 0.045_f32;
    let cardinal_mean = 0.25
        * (sample_dem_bilinear_norm((u - span).clamp(0.0, 1.0), v)
            + sample_dem_bilinear_norm((u + span).clamp(0.0, 1.0), v)
            + sample_dem_bilinear_norm(u, (v - span).clamp(0.0, 1.0))
            + sample_dem_bilinear_norm(u, (v + span).clamp(0.0, 1.0)));
    let high_pass_detail = (center - cardinal_mean).clamp(-0.18, 0.18);
    (center, high_pass_detail)
}

/// Evaluate the river thalweg X centerline for a given `sector_id` and `z`.
#[inline]
pub fn sample_river_thalweg_x(sector_id: &str, z: f32) -> f32 {
    let seed = sector_seed_from_id(sector_id);
    let phase = 1.08 + 0.16 * (seed * 0.75).sin();
    0.26 * (z * 1.32 + phase).sin() + 0.09 * (z * 2.6 - phase * 0.5).cos()
}

/// Evaluate the smooth Hermite/meander X centerline of the road corridor for a given
/// `sector_id` and `z` in `[-1.95, 1.95]`.
///
/// Positioned around `meander_x + 0.54` so existing actor, rover, walker, and clearing
/// coordinates remain strictly aligned.
#[inline]
pub fn sample_road_corridor_center_x(sector_id: &str, z: f32) -> f32 {
    sample_river_thalweg_x(sector_id, z) + 0.54
}

/// Evaluate multi-scale East Coast & Snowy Mountains terrain elevation in PortalGpu world
/// coordinates `[-2.2, 2.2]` using the real embedded Talbingo DEM raster and
/// `qualia_core_db::domains::geospatial::terrain` landform/fractal primitives.
pub fn sample_sector_elevation_dag(sector_id: &str, x: f32, z: f32, water_y: f32) -> f32 {
    let seed = sector_seed_from_id(sector_id);
    let u64_seed = sector_u64_seed(sector_id);

    // 1. Meandering river thalweg and signed lateral offset
    let meander_x = sample_river_thalweg_x(sector_id, z);
    let signed_river_dx = x - meander_x;
    let dist_river = signed_river_dx.abs();

    // Curvature proxy of meander (-d²x/dz²) to create asymmetric cut-bank vs point-bar profile
    let meander_curv = -0.28 * 1.35 * 1.35 * (z * 1.35 + seed).sin()
        - 0.11 * 2.7 * 2.7 * (z * 2.7 - seed * 0.6).cos();
    let outer_bend_bias = (signed_river_dx * meander_curv * 0.35).clamp(-0.08, 0.08);

    // 2. Real embedded 128×128 Talbingo DEM macro relief and high-pass spur/gully detail
    let (dem_macro, dem_detail) = sample_dem_macro_and_detail(x, z, seed);

    // 3. Multi-fractal SwissTurbulence & Ridged cascade from `qualia_core_db::domains::geospatial::terrain`
    let swiss_cfg = FractalConfig {
        mode: FractalMode::SwissTurbulence,
        octaves: 4,
        frequency: 0.85,
        lacunarity: 2.05,
        persistence: 0.48,
        ridge_offset: 0.95,
        warp_slope: 0.14,
        seed: u64_seed,
    };
    let swiss_val = evaluate_fractal_2d(x * 0.95, z * 0.95, &swiss_cfg);
    let micro_simplex = opensimplex2s_2d(x * 3.8, z * 3.8, u64_seed ^ 0xA5A5_5A5A);

    // Valley-wall mask so DEM/fractal relief is suppressed inside the active river bed
    // and smoothly ramps up across valley flanks
    let valley_wall_mask = smoothstep(0.24, 0.72, dist_river);

    // Side-gully dendritic drainage incisions descending into the main river gorge
    let gully_wave = (z * 4.2 + seed * 1.7 + 0.35 * perlin_2d(x * 2.0, z * 2.0, u64_seed)).sin();
    let gully_incision = (1.0 - gully_wave.abs()).powi(3)
        * smoothstep(0.32, 0.95, dist_river)
        * (1.0 - smoothstep(1.35, 1.95, dist_river))
        * 0.075;

    // 4. Deep incised river channel thalweg + inner point-bar shingle terrace
    // Ensures river thalweg is cleanly below `water_y` (`water_y - 0.16` to `water_y - 0.26`)
    // including along the standard fauna swim track `0.28 * (z * 1.35 + 1.1).sin()`.
    let fauna_swim_x = 0.28 * (z * 1.35 + 1.1).sin();
    let dist_fauna_swim = (x - fauna_swim_x).abs();
    let primary_thalweg = (-((dist_river / 0.36).powi(2))).exp();
    let swim_thalweg = (-((dist_fauna_swim / 0.28).powi(2))).exp();
    let combined_channel = primary_thalweg.max(0.82 * swim_thalweg);

    // Point-bar shallow shelf on the inner bend (`dist_river` in `0.22..0.44`)
    let point_bar_shelf = (-(((dist_river - 0.31) / 0.11).powi(2))).exp()
        * (0.5 - outer_bend_bias * 4.0).clamp(0.15, 0.95)
        * 0.032;

    let channel_depth = (0.31 + 0.03 * perlin_2d(x * 3.1, z * 3.1, u64_seed)) * combined_channel
        - point_bar_shelf;

    // 5. Sector-specific geomorphological valley walls and `LandformParams` primitives
    let valley_rise = match sector_id {
        "sydney_sandstone" => {
            // Stepped Hawkesbury Sandstone & Narrabeen Group cliff benches + slot gorge
            let canyon_params = LandformParams {
                kind: LandformKind::Canyon,
                center_x: meander_x,
                center_y: 0.0,
                radius: 2.35,
                height: 0.38,
                azimuth_rad: 1.48,
                sharpness: 1.65,
                roughness: 0.22,
                secondary_ratio: 0.5,
                seed: u64_seed,
            };
            let canyon_lift = sample_landform(x, z, &canyon_params);
            let raw = (dist_river - 0.30).max(0.0) * 0.72;
            // Multi-tier structural sandstone cliff steps (Wentworth Falls claystone breaks)
            let tier_phase = raw * 5.2;
            let tier_idx = tier_phase.floor();
            let tier_frac = tier_phase - tier_idx;
            let stepped = (tier_idx + smoothstep(0.22, 0.82, tier_frac)) / 5.2;
            let pagoda_fluting =
                0.025 * gabor_anisotropic_2d(x * 3.5, z * 3.5, 0.42, 1.8, 0.65, u64_seed);
            stepped * 0.82 + canyon_lift * 0.35 + pagoda_fluting * valley_wall_mask
        }
        "noosa_headland" => {
            // Coastal Oligocene basalt-diorite headland on +X with wave-cut platform,
            // shelving into sandy Laguna Bay / estuary on -X
            if x < meander_x {
                let bay_dist = (meander_x - x).min(1.65);
                let sandbar_ripple = 0.018 * (x * 9.5 + z * 3.2).sin() * smoothstep(0.15, 0.8, bay_dist);
                -0.15 * bay_dist + sandbar_ripple
            } else {
                let head_dist = x - meander_x;
                let escarp_params = LandformParams {
                    kind: LandformKind::Escarpment,
                    center_x: meander_x + 0.95,
                    center_y: 0.15,
                    radius: 1.95,
                    height: 0.36,
                    azimuth_rad: 0.25,
                    sharpness: 1.45,
                    roughness: 0.20,
                    secondary_ratio: 0.4,
                    seed: u64_seed,
                };
                let headland_dome = sample_landform(x, z, &escarp_params);
                // Wave-cut rocky platform bench near water level (`head_dist` in `0.24..0.42`)
                let platform_bench = smoothstep(0.20, 0.32, head_dist) * 0.05
                    + smoothstep(0.38, 1.55, head_dist).powf(1.22) * 0.62;
                platform_bench + headland_dome * 0.42
            }
        }
        "yarrangobilly" | "yarrangobilly_cliff" | "jounama_karst" => {
            // Steep Late Silurian reef limestone karst gorge, rillenkarren solution fluting,
            // and travertine thermal pool terraces
            let karst_params = LandformParams {
                kind: LandformKind::KarstTower,
                center_x: if signed_river_dx >= 0.0 { 1.15 } else { -1.10 },
                center_y: 0.10,
                radius: 1.85,
                height: 0.34,
                azimuth_rad: 0.35,
                sharpness: 1.8,
                roughness: 0.25,
                secondary_ratio: 0.6,
                seed: u64_seed,
            };
            let karst_towers = sample_landform(x, z, &karst_params);
            let gorge_wall = (dist_river - 0.26).max(0.0).powf(1.32) * 0.78;
            // Travertine tufa terrace steps along lower bank (`dist_river` in `0.26..0.52`)
            let tufa_zone = (1.0 - smoothstep(0.28, 0.56, dist_river)) * smoothstep(0.20, 0.30, dist_river);
            let tufa_terrace = (dist_river * 18.0).sin().abs() * 0.024 * tufa_zone;
            let rillen = 0.028 * gabor_anisotropic_2d(x * 5.2, z * 5.2, 1.52, 2.2, 0.75, u64_seed);
            gorge_wall + karst_towers * 0.38 + tufa_terrace + rillen * valley_wall_mask
        }
        "three_mile_dam" | "ohares_alpine_ash" => {
            // Kiandra High Plains subalpine frost-hollow basin, periglacial moraine hummocks,
            // and Sphagnum peat fen channels
            let cirque_params = LandformParams {
                kind: LandformKind::GlacialCirque,
                center_x: -0.35,
                center_y: 0.10,
                radius: 2.20,
                height: 0.28,
                azimuth_rad: 0.40,
                sharpness: 1.15,
                roughness: 0.18,
                secondary_ratio: 0.45,
                seed: u64_seed,
            };
            let cirque_relief = sample_landform(x, z, &cirque_params);
            let basin_flank = (dist_river - 0.36).max(0.0).powf(1.14) * 0.44;
            // Periglacial boulder-moraine hummocks and peat bog micro-terraces
            let hummock = 0.032 * perlin_2d(x * 4.5, z * 4.5, u64_seed ^ 0x99) * valley_wall_mask;
            basin_flank + cirque_relief * 0.35 + hummock
        }
        _ => {
            // Talbingo Gorge / Humes Crossing: incised Tumut River gorge with Silurian
            // granodiorite & Devonian columnar basalt valley walls and DEM-derived spurs
            let ridge_params = LandformParams {
                kind: LandformKind::RidgeLine,
                center_x: if signed_river_dx >= 0.0 { 1.25 } else { -1.25 },
                center_y: 0.0,
                radius: 2.15,
                height: 0.32,
                azimuth_rad: 1.50,
                sharpness: 1.35,
                roughness: 0.22,
                secondary_ratio: 0.5,
                seed: u64_seed,
            };
            let spur_lift = sample_landform(x, z, &ridge_params);
            let gorge_wall = (dist_river - 0.29).max(0.0).powf(1.24) * 0.66;
            gorge_wall + spur_lift * 0.32
        }
    };

    // Combine real Talbingo DEM relief, SwissTurbulence fractals, and side-gully incisions
    let dem_contribution =
        ((dem_macro - 0.32) * 0.22 + dem_detail * 0.55) * valley_wall_mask;
    let fractal_contribution =
        (0.095 * swiss_val + 0.022 * micro_simplex) * (0.28 + 0.72 * valley_wall_mask);

    let natural_y = water_y + 0.055 - channel_depth + valley_rise + dem_contribution
        + fractal_contribution
        - gully_incision;

    // 6. Engineered Cut-and-Fill Road Corridor Bench along `road_center_x(sector_id, z)`
    // Flat roadbed half-width = 0.075 (total width 0.150), with smooth cut/fill batter slopes
    // extending out to 0.195 so the road ribbon mesh sits on a true civil-engineered formation.
    let road_x = sample_road_corridor_center_x(sector_id, z);
    let road_dx = x - road_x;
    let track_dist = road_dx.abs();

    // Gentle longitudinal grade along Z (`water_y + 0.175 .. water_y + 0.215`)
    let road_target_y = water_y + 0.185 + 0.018 * (z * 0.85 + seed * 0.4).sin();

    let flat_half_w = 0.075_f32;
    let batter_outer_w = 0.205_f32;

    let base_elev = if track_dist <= flat_half_w {
        // Crowned roadbed formation sub-base (slight 1.5% crossfall for drainage)
        let crown = 0.0025 * (1.0 - (track_dist / flat_half_w).powi(2));
        road_target_y + crown
    } else if track_dist < batter_outer_w {
        let blend = quintic_smoothstep(flat_half_w, batter_outer_w, track_dist);
        // Upslope (+X) cut-batter ditch toes slightly below road edge before rising into cut rock wall
        let ditch_dip = if road_dx > 0.0 {
            let ditch_t = ((track_dist - flat_half_w) / 0.045).clamp(0.0, 1.0);
            -0.012 * (ditch_t * std::f32::consts::PI).sin()
        } else {
            0.0
        };
        road_target_y * (1.0 - blend) + natural_y * blend + ditch_dip * (1.0 - blend)
    } else {
        natural_y
    };

    // 7. Gently graded grassy woodland clearing bench on the upslope side (`road_dx in 0.14..0.52`, `z in -0.36..0.40`)
    // so the foreground kangaroo mob stands cleanly on a natural terrace without hillside clipping
    if road_dx > 0.12 && road_dx < 0.56 && z > -0.42 && z < 0.46 {
        let wx = (1.0 - ((road_dx - 0.32) / 0.22).powi(2)).clamp(0.0, 1.0);
        let wz = (1.0 - ((z - 0.02) / 0.42).powi(2)).clamp(0.0, 1.0);
        let clearing_weight = (wx * wz).powi(2) * 0.78;
        let clearing_target_y = road_target_y + 0.045 + (road_dx - 0.18).max(0.0) * 0.14;
        base_elev * (1.0 - clearing_weight) + clearing_target_y * clearing_weight
    } else {
        base_elev
    }
}

/// Add a 3D faceted boulder, granite tor block, columnar basalt prism, or flattened river cobble.
fn add_faceted_rock(
    b: &mut SubmeshBuilder,
    center: [f32; 3],
    radii: [f32; 3],
    yaw: f32,
    pitch: f32,
    col_base: [f32; 4],
    col_top: [f32; 4],
    bottom_shape: f64,
    top_shape: f64,
    seed: f32,
) {
    let slices = 11usize;
    let stacks = 5usize;
    let (req_v, req_t) = asymmetric_hull_capacity(slices, stacks);
    let mut hull_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
    let mut hull_t = vec![[0u32; 3]; req_t];
    let params = AsymmetricHullParams {
        neg_x_radius: (radii[0] * 0.94) as f64,
        pos_x_radius: (radii[0] * 1.06) as f64,
        neg_y_radius: (radii[2] * 0.91) as f64,
        pos_y_radius: (radii[2] * 1.09) as f64,
        neg_x_height: (radii[1] * 0.38) as f64,
        pos_x_height: (radii[1] * 0.44) as f64,
        neg_y_height: (radii[1] * 0.40) as f64,
        pos_y_height: (radii[1] * 0.42) as f64,
        bottom: Point3::new(0.0, 0.0, (-radii[1] * 0.65) as f64),
        top: Point3::new(0.0, 0.0, (radii[1] * 0.85) as f64),
        bottom_shape,
        top_shape,
        slices,
        stacks,
    };
    if let Ok((vc, tc)) = asymmetric_hull_into(&params, &mut hull_v, &mut hull_t) {
        let base_idx = b.positions.len() as u32;
        let index_start = b.indices.len();
        let (cy, sy) = (yaw.cos(), yaw.sin());
        let (cp, sp) = (pitch.cos(), pitch.sin());
        for (vi, p) in hull_v.iter().take(vc).enumerate() {
            let lx = p.x as f32;
            let ly = p.z as f32;
            let lz = p.y as f32;
            // Deterministic polyhedral joint-face faceting
            let facet = 0.91 + 0.17 * hash21(vi as f32 * 1.3 + seed, seed * 2.3);
            let px = lx * facet;
            let py = (ly * cp - lz * sp) * facet;
            let pz = (ly * sp + lz * cp) * facet;
            let rx = px * cy - pz * sy;
            let ry = py;
            let rz = px * sy + pz * cy;
            let nlen = (rx * rx + ry * ry + rz * rz).sqrt().max(1e-5);
            let t = (0.5_f32 + 0.5_f32 * (ry / nlen)).clamp(0.0_f32, 1.0_f32);
            let col = [
                col_base[0] * (1.0 - t) + col_top[0] * t,
                col_base[1] * (1.0 - t) + col_top[1] * t,
                col_base[2] * (1.0 - t) + col_top[2] * t,
                1.0,
            ];
            let u = (vi % slices) as f32 / slices as f32;
            b.push_vertex(
                [center[0] + rx, center[1] + ry, center[2] + rz],
                [rx / nlen, ry / nlen, rz / nlen],
                col,
                [u, t],
            );
        }
        for tri in hull_t.iter().take(tc) {
            b.push_tri(base_idx + tri[0], base_idx + tri[1], base_idx + tri[2]);
        }
        b.recompute_normals_since(base_idx as usize, index_start);
    }
}

/// Add a 3D stepped sandstone cliff bench or jointed rock slab aligned to local slope yaw.
fn add_jointed_rock_slab(
    b: &mut SubmeshBuilder,
    center: [f32; 3],
    half_extents: [f32; 3],
    yaw: f32,
    tilt_x: f32,
    col_bottom: [f32; 4],
    col_top: [f32; 4],
    seed: f32,
) {
    let v_start = b.positions.len();
    let i_start = b.indices.len();
    let (cy, sy) = (yaw.cos(), yaw.sin());
    let (ct, st) = (tilt_x.cos(), tilt_x.sin());
    let [hx, hy, hz] = half_extents;

    let corners: [[f32; 3]; 8] = [
        [-hx, -hy, -hz],
        [hx, -hy, -hz],
        [hx, hy, -hz],
        [-hx, hy, -hz],
        [-hx, -hy, hz],
        [hx, -hy, hz],
        [hx, hy, hz],
        [-hx, hy, hz],
    ];

    let mut world_pts = [[0.0_f32; 3]; 8];
    for (i, c) in corners.iter().enumerate() {
        // Taper top slightly and add natural fracture perturbation
        let top_taper = if c[1] > 0.0 { 0.86 } else { 1.04 };
        let jitter = 0.92 + 0.16 * hash21(seed + i as f32 * 3.7, seed * 1.9);
        let lx = c[0] * top_taper * jitter;
        let ly = c[1] * jitter;
        let lz = c[2] * top_taper * jitter;

        // Tilt and yaw rotation
        let tx = lx * ct - ly * st;
        let ty = lx * st + ly * ct;
        let tz = lz;

        let rx = tx * cy - tz * sy;
        let ry = ty;
        let rz = tx * sy + tz * cy;
        world_pts[i] = [center[0] + rx, center[1] + ry, center[2] + rz];
    }

    let faces: [([usize; 4], [f32; 3]); 6] = [
        ([0, 1, 2, 3], [0.0, 0.0, -1.0]),
        ([5, 4, 7, 6], [0.0, 0.0, 1.0]),
        ([4, 0, 3, 7], [-1.0, 0.0, 0.0]),
        ([1, 5, 6, 2], [1.0, 0.0, 0.0]),
        ([3, 2, 6, 7], [0.0, 1.0, 0.0]),
        ([4, 5, 1, 0], [0.0, -1.0, 0.0]),
    ];

    for (quad, n) in faces {
        let mut idx = [0u32; 4];
        for (k, &ci) in quad.iter().enumerate() {
            let is_top = corners[ci][1] > 0.0;
            let col = if is_top { col_top } else { col_bottom };
            let uv = match k {
                0 => [0.0, 0.0],
                1 => [1.0, 0.0],
                2 => [1.0, 1.0],
                _ => [0.0, 1.0],
            };
            idx[k] = b.push_vertex(world_pts[ci], n, col, uv);
        }
        b.push_quad(idx[0], idx[1], idx[2], idx[3]);
    }
    b.recompute_normals_since(v_start, i_start);
}

/// Build the high-resolution (`160x160`) sector terrain continuum mesh, optional 6-layer
/// CSIRO SLGA geological cross-section cutaway wall, and 3D lithology (clustered riparian
/// cobble point-bars, submerged riffle shelves, jointed granite tors, sandstone cliff
/// ledges, and columnar basalt outcrops).
pub fn build_sector_terrain_and_lithology(
    sector_id: &str,
    water_y: f32,
    include_cutaway_wall: bool,
    b_terrain: &mut SubmeshBuilder,
    b_lithology: &mut SubmeshBuilder,
) {
    let u64_seed = sector_u64_seed(sector_id);
    let grid_n = 160usize;
    let extent = 1.95_f32;
    let z_max = if include_cutaway_wall { 0.55_f32 } else { extent };
    let mut grid_idx = vec![0u32; (grid_n + 1) * (grid_n + 1)];

    // Sector-calibrated CSIRO SLGA & Sentinel-2 rock/soil palettes from `sectors_terrain.json`
    let (bedrock_dark, bedrock_light, soil_loam_dry, soil_loam_moist, sand_shingle) = match sector_id
    {
        "sydney_sandstone" => (
            [0.64_f32, 0.46, 0.30], // Hawkesbury ironstone-banded sandstone
            [0.82_f32, 0.65, 0.44], // Cross-bedded quartzose cliff
            [0.44_f32, 0.38, 0.25], // Dry sandy sclerophyll woodland litter
            [0.24_f32, 0.38, 0.20], // Moist gully Coachwood/fern humus
            [0.84_f32, 0.75, 0.58], // Golden alluvial quartz sand
        ),
        "yarrangobilly" | "yarrangobilly_cliff" | "jounama_karst" => (
            [0.54_f32, 0.55, 0.56], // Weathered Silurian marble / karst limestone
            [0.78_f32, 0.77, 0.74], // Pale stromatoporoid reef cliff
            [0.42_f32, 0.32, 0.22], // Calcium-rich terra rossa loam
            [0.22_f32, 0.42, 0.21], // Bryophyte-rich karst spring bank
            [0.70_f32, 0.66, 0.58], // Limestone shingle & travertine sand
        ),
        "three_mile_dam" | "ohares_alpine_ash" => (
            [0.44_f32, 0.43, 0.42], // Periglacial Kosciuszko granodiorite & basalt
            [0.64_f32, 0.62, 0.59], // Lichen-mottled grey granite tor
            [0.34_f32, 0.36, 0.20], // Subalpine Poa tussock & snow-gum loam
            [0.18_f32, 0.35, 0.16], // Dark Sphagnum cristatum peat fen
            [0.58_f32, 0.53, 0.44], // Coarse granitic grus & shingle
        ),
        "noosa_headland" => (
            [0.24_f32, 0.23, 0.26], // Oligocene coastal basalt-diorite headland
            [0.38_f32, 0.35, 0.34], // Salt-weathered headland rock platform
            [0.46_f32, 0.44, 0.28], // Coastal Wallum & hoop-pine gallery loam
            [0.20_f32, 0.42, 0.22], // Subtropical vine-forest bank
            [0.88_f32, 0.83, 0.68], // Cooloola Pleistocene silica beach sand
        ),
        _ => (
            [0.36_f32, 0.34, 0.32], // Blowering dacitic tuff & Talbingo basalt
            [0.56_f32, 0.53, 0.49], // Jointed Silurian granodiorite gorge wall
            [0.35_f32, 0.39, 0.22], // Montane sclerophyll biomantle & eucalypt litter
            [0.21_f32, 0.43, 0.19], // Moist Tumut riparian moss & fern bank
            [0.68_f32, 0.60, 0.46], // Alluvial river shingle & quartz-jasper sand
        ),
    };

    // 1. Build High-Resolution 160×160 Terrain Continuum Grid
    for gz in 0..=grid_n {
        for gx in 0..=grid_n {
            let u = gx as f32 / grid_n as f32;
            let v = gz as f32 / grid_n as f32;
            let x = (u * 2.0 - 1.0) * extent;
            let z = -extent + v * (z_max + extent);
            let y = sample_sector_elevation_dag(sector_id, x, z, water_y);

            // Central-difference surface normal and slope
            let eps = 0.016_f32;
            let y_dx = sample_sector_elevation_dag(sector_id, x + eps, z, water_y)
                - sample_sector_elevation_dag(sector_id, x - eps, z, water_y);
            let y_dz = sample_sector_elevation_dag(sector_id, x, z + eps, water_y)
                - sample_sector_elevation_dag(sector_id, x, z - eps, water_y);
            let nx = -y_dx;
            let ny = 2.0 * eps;
            let nz = -y_dz;
            let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-5);
            let norm = [nx / nlen, ny / nlen, nz / nlen];
            let slope = 1.0 - norm[1];

            // Multi-scale organic variation via `qualia_core_db::domains::geospatial::terrain`
            let macro_noise = perlin_2d(x * 2.4, z * 2.4, u64_seed);
            let micro_noise = opensimplex2s_2d(x * 9.5, z * 9.5, u64_seed ^ 0x1357);
            let vor = voronoi_2d(x * 5.5, z * 5.5, 0.85, u64_seed ^ 0x2468);
            let organic_var = (0.92 + 0.08 * macro_noise + 0.05 * micro_noise).clamp(0.78, 1.14);

            let dist_water = y - water_y;
            let road_x = sample_road_corridor_center_x(sector_id, z);
            let track_dist = (x - road_x).abs();

            // NOTE: Leave terrain under the road as graded crushed-basalt/gravel sub-base!
            // Swarm S4 (`cad_actors_corridor.rs`) places a dedicated high-resolution engineered
            // road ribbon mesh with crisp continuous markings, shoulders, and guardrails on top.
            let col = if track_dist < 0.082 {
                // Graded crushed-basalt / compacted aggregate roadbed sub-base
                let grain = 0.96 + 0.04 * micro_noise;
                [0.32 * grain, 0.30 * grain, 0.28 * grain, 1.0]
            } else if track_dist < 0.145 {
                // Engineered cut/fill gravel batter shoulder blending into natural slope
                let shoulder_t = smoothstep(0.082, 0.145, track_dist);
                let sub_r = 0.48 * organic_var;
                let sub_g = 0.37 * organic_var;
                let sub_b = 0.26 * organic_var;
                let nat_r = soil_loam_dry[0] * organic_var;
                let nat_g = soil_loam_dry[1] * organic_var;
                let nat_b = soil_loam_dry[2] * organic_var;
                [
                    sub_r * (1.0 - shoulder_t) + nat_r * shoulder_t,
                    sub_g * (1.0 - shoulder_t) + nat_g * shoulder_t,
                    sub_b * (1.0 - shoulder_t) + nat_b * shoulder_t,
                    1.0,
                ]
            } else if dist_water < -0.04 {
                // Deep riverbed quartz/basalt alluvium & dark water-worn cobble armor
                let depth_atten = (1.0 - ((-dist_water - 0.04) * 1.45).clamp(0.0, 0.48))
                    * (0.88 + 0.12 * vor.crackle);
                [
                    0.38 * depth_atten,
                    0.36 * depth_atten,
                    0.30 * depth_atten,
                    1.0,
                ]
            } else if dist_water < 0.045 {
                // Wet riparian shingle & damp alluvial sand bar (`-0.04 .. 0.045`)
                let wet_t = smoothstep(-0.04, 0.045, dist_water);
                let damp_darken = 0.72 + 0.28 * wet_t;
                let pebble_var = 0.90 + 0.10 * vor.f1.clamp(0.0, 1.0);
                [
                    sand_shingle[0] * damp_darken * pebble_var,
                    sand_shingle[1] * damp_darken * pebble_var,
                    sand_shingle[2] * damp_darken * pebble_var,
                    1.0,
                ]
            } else if slope > 0.26 {
                // Exposed jointed bedrock on steep slopes (`slope > 0.26`), stratified by elevation
                let rock_blend = smoothstep(0.26, 0.42, slope);
                let strata_band = 0.5 + 0.5 * (y * 28.0 + 1.8 * macro_noise).sin();
                let joint_dark = 0.86 + 0.14 * vor.crackle;
                let rk_r = (bedrock_dark[0] * (1.0 - strata_band) + bedrock_light[0] * strata_band)
                    * joint_dark;
                let rk_g = (bedrock_dark[1] * (1.0 - strata_band) + bedrock_light[1] * strata_band)
                    * joint_dark;
                let rk_b = (bedrock_dark[2] * (1.0 - strata_band) + bedrock_light[2] * strata_band)
                    * joint_dark;
                let sl_r = soil_loam_dry[0] * organic_var;
                let sl_g = soil_loam_dry[1] * organic_var;
                let sl_b = soil_loam_dry[2] * organic_var;
                [
                    sl_r * (1.0 - rock_blend) + rk_r * rock_blend,
                    sl_g * (1.0 - rock_blend) + rk_g * rock_blend,
                    sl_b * (1.0 - rock_blend) + rk_b * rock_blend,
                    1.0,
                ]
            } else if dist_water < 0.14 {
                // Mossy riparian bank & fern loam (`0.045 .. 0.14`)
                let bank_t = smoothstep(0.045, 0.14, dist_water);
                let r = (sand_shingle[0] * 0.78 * (1.0 - bank_t) + soil_loam_moist[0] * bank_t)
                    * organic_var;
                let g = (sand_shingle[1] * 0.78 * (1.0 - bank_t) + soil_loam_moist[1] * bank_t)
                    * organic_var;
                let b = (sand_shingle[2] * 0.78 * (1.0 - bank_t) + soil_loam_moist[2] * bank_t)
                    * organic_var;
                [r, g, b, 1.0]
            } else {
                // Sclerophyll forest biomantle loam with warm dry eucalypt leaf-litter patches
                // and aspect-dependent sun/shade variation
                let moisture = (-((dist_water - 0.14) / 0.34).powi(2)).exp();
                let litter_patch = (0.5 + 0.5 * perlin_2d(x * 4.8 + 3.1, z * 4.8 - 1.7, u64_seed))
                    .powf(1.6)
                    * (1.0 - moisture * 0.65);
                // Sun-facing aspect warmth (+X/+Z vs shaded south-west gully aspect)
                let aspect_sun = (0.5 + 0.35 * norm[0] + 0.25 * norm[2]).clamp(0.35, 0.85);
                let base_r =
                    soil_loam_dry[0] * (1.0 - moisture) + soil_loam_moist[0] * moisture;
                let base_g =
                    soil_loam_dry[1] * (1.0 - moisture) + soil_loam_moist[1] * moisture;
                let base_b =
                    soil_loam_dry[2] * (1.0 - moisture) + soil_loam_moist[2] * moisture;

                // Warm ochre/russet dry eucalypt bark & leaf-litter mottling
                let lit_r = 0.48_f32;
                let lit_g = 0.34_f32;
                let lit_b = 0.20_f32;
                let r = (base_r * (1.0 - 0.38 * litter_patch) + lit_r * (0.38 * litter_patch))
                    * organic_var
                    * (0.92 + 0.14 * aspect_sun);
                let g = (base_g * (1.0 - 0.38 * litter_patch) + lit_g * (0.38 * litter_patch))
                    * organic_var
                    * (0.94 + 0.10 * aspect_sun);
                let b = (base_b * (1.0 - 0.38 * litter_patch) + lit_b * (0.38 * litter_patch))
                    * organic_var;
                [r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), 1.0]
            };

            let vi = b_terrain.push_vertex([x, y, z], norm, col, [u * 4.0, v * 4.0]);
            grid_idx[gz * (grid_n + 1) + gx] = vi;
        }
    }

    for gz in 0..grid_n {
        for gx in 0..grid_n {
            let i00 = grid_idx[gz * (grid_n + 1) + gx];
            let i10 = grid_idx[gz * (grid_n + 1) + gx + 1];
            let i01 = grid_idx[(gz + 1) * (grid_n + 1) + gx];
            let i11 = grid_idx[(gz + 1) * (grid_n + 1) + gx + 1];
            b_terrain.push_tri(i00, i01, i11);
            b_terrain.push_tri(i00, i11, i10);
        }
    }

    // 2. Optional 6-Layer CSIRO SLGA Subsurface Geological Cutaway Cross-Section (`include_cutaway_wall`)
    if include_cutaway_wall {
        let strata_colors: [[f32; 4]; 6] = [
            [0.19, 0.15, 0.11, 1.0], // L0 O-Horizon (0-5cm): Dark Alpine Humus & Litter Peat
            [0.36, 0.26, 0.17, 1.0], // L1 A1-Horizon (5-15cm): Organic Biomantle Loam
            [0.76, 0.68, 0.52, 1.0], // L2 A2-Horizon (15-30cm): Bleached Eluvial Quartz Sand
            [0.78, 0.33, 0.16, 1.0], // L3 B2-Horizon (30-60cm): Red-Ochre Illuvial Ferrosol Clay
            [0.62, 0.54, 0.38, 1.0], // L4 C-Horizon (60-100cm): Weathering Granitic Saprolite Grus
            [
                bedrock_dark[0],
                bedrock_dark[1],
                bedrock_dark[2],
                1.0,
            ], // L5 R-Horizon (100-200cm+): Jointed Country Bedrock
        ];
        // Realistic non-uniform cumulative horizon depth fractions matching CSIRO SLGA depths
        let horizon_fracs: [f32; 7] = [0.0, 0.06, 0.16, 0.30, 0.54, 0.78, 1.0];
        let base_y = -0.98_f32;

        for gx in 0..grid_n {
            let u0 = gx as f32 / grid_n as f32;
            let u1 = (gx + 1) as f32 / grid_n as f32;
            let x0 = (u0 * 2.0 - 1.0) * extent;
            let x1 = (u1 * 2.0 - 1.0) * extent;
            let top0 = sample_sector_elevation_dag(sector_id, x0, z_max, water_y);
            let top1 = sample_sector_elevation_dag(sector_id, x1, z_max, water_y);

            for layer in 0..6 {
                let f0 = horizon_fracs[layer];
                let f1 = horizon_fracs[layer + 1];
                let wave0 = 0.014 * (x0 * 4.8 + layer as f32 * 1.4).sin()
                    + 0.006 * perlin_2d(x0 * 6.0, layer as f32 * 2.0, u64_seed);
                let wave1 = 0.014 * (x1 * 4.8 + layer as f32 * 1.4).sin()
                    + 0.006 * perlin_2d(x1 * 6.0, layer as f32 * 2.0, u64_seed);
                let y0_a = top0 * (1.0 - f0) + base_y * f0 + wave0 * (1.0 - f0) * (f0 * 2.5).min(1.0);
                let y0_b = top0 * (1.0 - f1) + base_y * f1 + wave0 * (1.0 - f1) * (f1 * 2.5).min(1.0);
                let y1_a = top1 * (1.0 - f0) + base_y * f0 + wave1 * (1.0 - f0) * (f0 * 2.5).min(1.0);
                let y1_b = top1 * (1.0 - f1) + base_y * f1 + wave1 * (1.0 - f1) * (f1 * 2.5).min(1.0);
                let z_step_a = z_max + (layer as f32) * 0.011;
                let z_step_b = z_max + ((layer + 1) as f32) * 0.011;
                let col = strata_colors[layer];
                let norm = [0.0, 0.20, 0.98];
                let v0 = b_terrain.push_vertex([x0, y0_a, z_step_a], norm, col, [u0 * 4.0, f0]);
                let v1 = b_terrain.push_vertex([x1, y1_a, z_step_a], norm, col, [u1 * 4.0, f0]);
                let v2 = b_terrain.push_vertex([x1, y1_b, z_step_b], norm, col, [u1 * 4.0, f1]);
                let v3 = b_terrain.push_vertex([x0, y0_b, z_step_b], norm, col, [u0 * 4.0, f1]);
                b_terrain.push_tri(v0, v3, v2);
                b_terrain.push_tri(v0, v2, v1);
            }
        }

        // Embedded saprolite corestones & bedrock inclusions exposed in the cutaway face
        for c_idx in 0..18 {
            let fc = c_idx as f32;
            let cx = -1.65 + (fc / 17.0) * 3.30;
            let top_y = sample_sector_elevation_dag(sector_id, cx, z_max, water_y);
            let depth_frac = 0.52 + 0.34 * hash21(fc * 4.1, 8.3);
            let cy = top_y * (1.0 - depth_frac) + base_y * depth_frac;
            let cz = z_max + depth_frac * 0.066 + 0.008;
            let rx = 0.032 + 0.038 * hash21(fc * 7.7, 2.9);
            let ry = rx * (0.48 + 0.24 * hash21(fc * 3.3, 5.1));
            add_faceted_rock(
                b_lithology,
                [cx, cy, cz],
                [rx, ry, rx * 0.45],
                fc * 0.7,
                0.12 * (fc * 1.9).sin(),
                [bedrock_dark[0], bedrock_dark[1], bedrock_dark[2], 1.0],
                [bedrock_light[0], bedrock_light[1], bedrock_light[2], 1.0],
                1.6,
                1.5,
                fc + 500.0,
            );
        }
    }

    // 3. Clustered Riparian Cobble Point-Bars & Rocky Riffle Shelves (`b_lithology`)
    // Replaces uniform mid-river spheres with natural imbricated point-bar shingle clusters
    // along the shallow river margins and bank toes, with flattened river-stone proportions
    // (`[r * 1.6, r * 0.38, r * 1.1]`).
    let cluster_count = 18usize;
    for c in 0..cluster_count {
        let fc = c as f32;
        let z_cluster = -1.72 + (fc / (cluster_count - 1) as f32) * 3.44;
        if z_cluster > z_max - 0.10 {
            continue;
        }
        let meander_x = sample_river_thalweg_x(sector_id, z_cluster);
        // Alternate between left and right shallow river margins / point-bars (`0.18 .. 0.38` from thalweg)
        let bank_side = if c % 2 == 0 { -1.0_f32 } else { 1.0_f32 };
        let bar_offset = bank_side * (0.20 + 0.15 * hash21(fc * 5.3, 2.7));
        let cx = meander_x + bar_offset;

        // Keep well clear of the road corridor
        let road_x = sample_road_corridor_center_x(sector_id, z_cluster);
        if (cx - road_x).abs() < 0.22 {
            continue;
        }

        // Anchor each point-bar cluster with 1–2 larger flat riffle shelf stones and 7–11 imbricated cobbles
        let stones_in_cluster = 8 + (c % 5);
        for s in 0..stones_in_cluster {
            let fs = (c * 16 + s) as f32;
            let ang = fs * 2.399_963;
            let rad_spread = 0.025 + 0.11 * hash21(fs * 3.1, fc * 7.9).sqrt();
            let sx = cx + ang.cos() * rad_spread * 0.85;
            let sz = (z_cluster + ang.sin() * rad_spread * 1.35).clamp(-1.88, z_max - 0.06);

            let sy = sample_sector_elevation_dag(sector_id, sx, sz, water_y);
            // Place stones only along shallow margin/shingle zone (`water_y - 0.09 .. water_y + 0.08`)
            if sy < water_y - 0.11 || sy > water_y + 0.10 {
                continue;
            }

            let r = if s == 0 {
                // Anchor riffle ledge boulder
                0.026 + 0.018 * hash21(fs * 4.7, 1.9)
            } else {
                // Flattened imbricated river cobble / shingle stone
                0.010 + 0.016 * hash21(fs * 7.3, 2.1)
            };

            let tint_sel = hash21(fs * 11.1, 5.7);
            let (col_b, col_t) = if tint_sel > 0.72 {
                // Water-worn quartz / aplite river cobble
                ([0.58, 0.54, 0.46, 1.0], [0.82, 0.78, 0.68, 1.0])
            } else if tint_sel > 0.44 {
                // Porphyritic dacite / ironstone river cobble
                ([0.46, 0.33, 0.24, 1.0], [0.68, 0.48, 0.35, 1.0])
            } else {
                // Dark Silurian basalt / granodiorite river cobble
                ([0.26, 0.28, 0.31, 1.0], [0.45, 0.47, 0.51, 1.0])
            };

            // Flattened river-stone proportions (`[r * 1.6, r * 0.38, r * 1.1]`) with downstream imbrication pitch
            add_faceted_rock(
                b_lithology,
                [sx, sy + r * 0.16, sz],
                [r * 1.60, r * 0.38, r * 1.10],
                fs * 1.45,
                0.14,
                col_b,
                col_t,
                1.85,
                1.75,
                fs,
            );
        }
    }

    // 4. Multi-Block Jointed Granite Tors, Sandstone Cliff Ledges, Karst Pinnacles & Columnar Basalt Outcrops
    // Placed on the valley flanks while keeping the road corridor and kangaroo clearing completely clear.
    let outcrop_count = 32usize;
    for i in 0..outcrop_count {
        let fi = i as f32;
        let side = if i % 2 == 0 { 1.0_f32 } else { -1.0_f32 };
        let z = -1.68 + (fi / (outcrop_count - 1) as f32) * 3.36;
        if z > z_max - 0.14 {
            continue;
        }
        let meander_x = sample_river_thalweg_x(sector_id, z);
        let road_x = sample_road_corridor_center_x(sector_id, z);

        let x = if side > 0.0 {
            // Upslope of road corridor (+X valley wall)
            (road_x + 0.34 + 0.92 * hash21(fi * 5.3, 1.7)).clamp(-1.82, 1.82)
        } else {
            // Opposite river valley wall (-X escarpment)
            (meander_x - 0.46 - 1.15 * hash21(fi * 5.3, 1.7)).clamp(-1.82, 1.82)
        };

        // Keep outcrops strictly off the road corridor (`< 0.30`) and out of the kangaroo clearing
        if (x - road_x).abs() < 0.30 || (x > 0.85 && x < 1.44 && z > -0.40 && z < 0.48) {
            continue;
        }
        // In Noosa Headland, -X is open sandy bay/ocean, so restrict rocky outcrops to +X headland
        if sector_id == "noosa_headland" && x < meander_x + 0.15 {
            continue;
        }

        let y = sample_sector_elevation_dag(sector_id, x, z, water_y);
        if y < water_y + 0.05 {
            continue;
        }

        let col_b = [bedrock_dark[0], bedrock_dark[1], bedrock_dark[2], 1.0];
        let col_t = [bedrock_light[0], bedrock_light[1], bedrock_light[2], 1.0];

        match sector_id {
            "sydney_sandstone" => {
                // Stepped Hawkesbury Sandstone tabular cliff ledges & ironstone pagoda slabs
                let w = 0.085 + 0.095 * hash21(fi * 2.9, 8.4);
                let h = 0.032 + 0.042 * hash21(fi * 4.1, 3.3);
                let d = 0.075 + 0.085 * hash21(fi * 6.7, 1.2);
                let yaw = 0.25 * (fi * 1.7).sin();
                add_jointed_rock_slab(
                    b_lithology,
                    [x, y + h * 0.45, z],
                    [w, h, d],
                    yaw,
                    -0.06 * side,
                    col_b,
                    col_t,
                    fi + 200.0,
                );
                // Upper stepped pagoda bench
                if i % 2 == 0 {
                    add_jointed_rock_slab(
                        b_lithology,
                        [x + side * w * 0.22, y + h * 1.45, z + 0.02],
                        [w * 0.72, h * 0.78, d * 0.76],
                        yaw + 0.12,
                        -0.04 * side,
                        col_b,
                        col_t,
                        fi + 250.0,
                    );
                }
            }
            "yarrangobilly" | "yarrangobilly_cliff" | "jounama_karst" => {
                // Steep fluted Silurian limestone karst pinnacles & solution-jointed blocks
                let rx = 0.042 + 0.055 * hash21(fi * 2.9, 8.4);
                let ry = rx * (1.15 + 0.65 * hash21(fi * 4.1, 3.3));
                add_faceted_rock(
                    b_lithology,
                    [x, y + ry * 0.42, z],
                    [rx * 0.85, ry, rx * 0.78],
                    fi * 0.85,
                    0.08 * (fi * 1.3).cos(),
                    col_b,
                    col_t,
                    1.4,
                    1.3,
                    fi + 300.0,
                );
                // Adjacent solution-weathered satellite pinnacle
                add_faceted_rock(
                    b_lithology,
                    [x + 0.045 * side, y + ry * 0.24, z + 0.04],
                    [rx * 0.62, ry * 0.68, rx * 0.58],
                    fi * 1.25 + 0.5,
                    -0.06,
                    col_b,
                    col_t,
                    1.4,
                    1.3,
                    fi + 340.0,
                );
            }
            "noosa_headland" => {
                // Columnar coastal basalt-diorite headland prisms & wave-cut ledges
                let r_col = 0.036 + 0.032 * hash21(fi * 3.7, 6.1);
                let h_col = 0.055 + 0.065 * hash21(fi * 5.9, 2.4);
                for p in 0..3 {
                    let fp = p as f32;
                    let ox = (fp - 1.0) * r_col * 1.15;
                    let oz = ((p % 2) as f32 - 0.5) * r_col * 0.95;
                    let ph = h_col * (0.75 + 0.25 * hash21(fi + fp * 7.1, 4.3));
                    add_jointed_rock_slab(
                        b_lithology,
                        [x + ox, y + ph * 0.45, z + oz],
                        [r_col * 0.65, ph, r_col * 0.65],
                        fi * 0.6 + fp * 0.52,
                        0.08,
                        col_b,
                        col_t,
                        fi * 10.0 + fp + 400.0,
                    );
                }
            }
            _ => {
                // Multi-block jointed Silurian granodiorite tors & Snowy Mountains periglacial corestones
                let rx = 0.052 + 0.065 * hash21(fi * 2.9, 8.4);
                let ry = rx * (0.58 + 0.38 * hash21(fi * 4.1, 3.3));
                let rz = rx * (0.78 + 0.28 * hash21(fi * 6.3, 1.9));
                // Basal joint block
                add_faceted_rock(
                    b_lithology,
                    [x, y + ry * 0.35, z],
                    [rx, ry, rz],
                    fi * 0.85,
                    0.06 * (fi * 1.7).sin(),
                    col_b,
                    col_t,
                    1.75,
                    1.65,
                    fi + 100.0,
                );
                // Perched upper corestone / woolpack tor block on every second outcrop
                if i % 2 == 0 {
                    add_faceted_rock(
                        b_lithology,
                        [
                            x + 0.012 * (fi * 2.1).cos(),
                            y + ry * 1.05,
                            z + 0.012 * (fi * 2.1).sin(),
                        ],
                        [rx * 0.72, ry * 0.68, rz * 0.70],
                        fi * 0.85 + 0.35,
                        -0.05,
                        col_b,
                        col_t,
                        1.65,
                        1.55,
                        fi + 150.0,
                    );
                }
            }
        }
    }
}
