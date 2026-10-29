//! High-Resolution Engineered Road Corridor, Roadside Furniture, CAD 4WD Field Research Rover,
//! and Articulated 3D Field Ecologist Bushwalker Builder.
//!
//! Replaces coarse vertex-painted terrain road markings, monolithic cabin boxes, and axis-aligned
//! character boxes with true high-detail CAD/swept/lofted/revolved 3D geometry using QualiaDB's
//! native computational geometry kernels (`wheel_clearance_panel_into`, `loft_convex_sections_into`,
//! `revolve_profile`, `extrude_profile_along_path`, `sdf_ops`, and `marching_cubes`).

use crate::scene_builder_10d::SubmeshBuilder;
use qualia_core_db::specialized_libs::computational_geometry::authoring::loft_convex_sections_into;
use qualia_core_db::specialized_libs::computational_geometry::parametric_cad::revolve_profile;
use qualia_core_db::specialized_libs::computational_geometry::sdf_ops::{
    sdf_capsule, sdf_rounded_box, sdf_smooth_union, sdf_sphere,
};
use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::ExtrusionStation;
use qualia_core_db::specialized_libs::computational_geometry::wheel_panel::{
    wheel_clearance_panel_into, WheelPanel,
};
use qualia_core_db::specialized_libs::computational_geometry::{marching_cubes, Point3};

#[inline]
fn lerp_color(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
        a[3] + (b[3] - a[3]) * t,
    ]
}

#[inline]
fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-6 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        [0.0, 1.0, 0.0]
    }
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
fn sub3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn add3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale3(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// Emit a single flat-shaded or smoothly-oriented quad `(p0, p1, p2, p3)` in counter-clockwise order.
fn push_oriented_quad(
    b: &mut SubmeshBuilder,
    p0: [f32; 3],
    p1: [f32; 3],
    p2: [f32; 3],
    p3: [f32; 3],
    col_bottom: [f32; 4],
    col_top: [f32; 4],
) {
    let n = normalize3(cross3(sub3(p1, p0), sub3(p2, p0)));
    let v0 = b.push_vertex(p0, n, col_bottom, [0.0, 0.0]);
    let v1 = b.push_vertex(p1, n, col_bottom, [1.0, 0.0]);
    let v2 = b.push_vertex(p2, n, col_top, [1.0, 1.0]);
    let v3 = b.push_vertex(p3, n, col_top, [0.0, 1.0]);
    b.push_quad(v0, v1, v2, v3);
}

/// Add an arbitrary 3D hexahedron (strut/pillar/beam) connecting a bottom 4-point quad `bot` to a top
/// 4-point quad `top` (ordered counter-clockwise when viewed from +Y), with crisp per-face normals.
fn add_hexahedron(
    b: &mut SubmeshBuilder,
    bot: [[f32; 3]; 4],
    top: [[f32; 3]; 4],
    col_bottom: [f32; 4],
    col_top: [f32; 4],
) {
    // 4 side faces
    for i in 0..4 {
        let ni = (i + 1) % 4;
        push_oriented_quad(b, bot[i], bot[ni], top[ni], top[i], col_bottom, col_top);
    }
    // Top cap
    push_oriented_quad(b, top[0], top[1], top[2], top[3], col_top, col_top);
    // Bottom cap
    push_oriented_quad(
        b,
        bot[0],
        bot[3],
        bot[2],
        bot[1],
        col_bottom,
        col_bottom,
    );
}

/// Add an oriented 3D rectangular beam between `p_start` and `p_end` with cross-sectional half-width
/// `hw` (lateral) and half-height `hh` (vertical-ish).
fn add_oriented_beam(
    b: &mut SubmeshBuilder,
    p_start: [f32; 3],
    p_end: [f32; 3],
    up_hint: [f32; 3],
    hw: f32,
    hh: f32,
    col_bottom: [f32; 4],
    col_top: [f32; 4],
) {
    let dir = sub3(p_end, p_start);
    let len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
    if len < 1e-6 {
        return;
    }
    let fwd = [dir[0] / len, dir[1] / len, dir[2] / len];
    let mut right = cross3(fwd, up_hint);
    let rlen = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
    if rlen < 1e-5 {
        right = cross3(fwd, [1.0, 0.0, 0.0]);
    }
    right = normalize3(right);
    let up = normalize3(cross3(right, fwd));

    let r = scale3(right, hw);
    let u = scale3(up, hh);

    let bot = [
        sub3(sub3(p_start, r), u),
        sub3(add3(p_start, r), u),
        add3(add3(p_start, r), u),
        add3(sub3(p_start, r), u),
    ];
    let top = [
        sub3(sub3(p_end, r), u),
        sub3(add3(p_end, r), u),
        add3(add3(p_end, r), u),
        add3(sub3(p_end, r), u),
    ];
    add_hexahedron(b, bot, top, col_bottom, col_top);
}

/// Loft convex XY sections along vehicle X (mapped `(z_lat, y, x_long) -> [x_long, y, z_lat]`)
/// and recompute exact face-averaged normals.
fn add_vehicle_loft_along_x(
    b: &mut SubmeshBuilder,
    sections_zy_at_x: &[Vec<[f32; 3]>],
    col_bottom: [f32; 4],
    col_top: [f32; 4],
) {
    if sections_zy_at_x.len() < 2 {
        return;
    }
    let ring_len = sections_zy_at_x[0].len();
    let req_v = (sections_zy_at_x.len() - 1) * (ring_len + 1) * 2 + (ring_len + 1) * 2 + 16;
    let req_t = sections_zy_at_x.len() * ring_len * 2 + 16;
    let mut out_v = vec![[0.0f32; 3]; req_v];
    let mut out_t = vec![[0u32; 3]; req_t];

    if let Ok(receipt) = loft_convex_sections_into(sections_zy_at_x, &mut out_v, &mut out_t) {
        let v_start = b.positions.len();
        let i_start = b.indices.len();
        let min_y = receipt.min[1];
        let span_y = (receipt.max[1] - min_y).max(1e-4);
        let min_x = receipt.min[2];
        let span_x = (receipt.max[2] - min_x).max(1e-4);

        for p in &out_v[..receipt.vertex_count] {
            let vx = p[2];
            let vy = p[1];
            let vz = p[0];
            let ty = ((vy - min_y) / span_y).clamp(0.0, 1.0);
            let tx = ((vx - min_x) / span_x).clamp(0.0, 1.0);
            let col = lerp_color(col_bottom, col_top, ty);
            b.push_vertex([vx, vy, vz], [0.0, 1.0, 0.0], col, [tx, ty]);
        }
        for tri in &out_t[..receipt.triangle_count] {
            // Swapping X and Z in coordinates reverses handedness; flip winding (0, 2, 1) for outward normals
            b.push_tri(
                v_start as u32 + tri[0],
                v_start as u32 + tri[2],
                v_start as u32 + tri[1],
            );
        }
        b.recompute_normals_since(v_start, i_start);
    }
}

/// Loft convex XZ sections along vertical Y (authored as `(x, z, y) -> [x, y, z]`)
/// and recompute exact face-averaged normals.
fn add_vertical_loft_along_y(
    b: &mut SubmeshBuilder,
    sections_xz_at_y: &[Vec<[f32; 3]>],
    col_bottom: [f32; 4],
    col_top: [f32; 4],
) {
    if sections_xz_at_y.len() < 2 {
        return;
    }
    let ring_len = sections_xz_at_y[0].len();
    let req_v = (sections_xz_at_y.len() - 1) * (ring_len + 1) * 2 + (ring_len + 1) * 2 + 16;
    let req_t = sections_xz_at_y.len() * ring_len * 2 + 16;
    let mut out_v = vec![[0.0f32; 3]; req_v];
    let mut out_t = vec![[0u32; 3]; req_t];

    if let Ok(receipt) = loft_convex_sections_into(sections_xz_at_y, &mut out_v, &mut out_t) {
        let v_start = b.positions.len();
        let i_start = b.indices.len();
        let min_y = receipt.min[2];
        let span_y = (receipt.max[2] - min_y).max(1e-4);

        for p in &out_v[..receipt.vertex_count] {
            let vx = p[0];
            let vy = p[2];
            let vz = p[1];
            let ty = ((vy - min_y) / span_y).clamp(0.0, 1.0);
            let col = lerp_color(col_bottom, col_top, ty);
            b.push_vertex([vx, vy, vz], [0.0, 1.0, 0.0], col, [0.5, ty]);
        }
        for tri in &out_t[..receipt.triangle_count] {
            // Swapping Y and Z reverses handedness; flip winding (0, 2, 1) for outward normals
            b.push_tri(
                v_start as u32 + tri[0],
                v_start as u32 + tri[2],
                v_start as u32 + tri[1],
            );
        }
        b.recompute_normals_since(v_start, i_start);
    }
}

/// Extract a smooth C¹ SDF surface in local coordinates `(bmin .. bmax)` scaled by `scale`
/// and append welded vertices + analytical gradient normals to `b`.
fn append_local_sdf_surface<F>(
    b: &mut SubmeshBuilder,
    bmin: [f32; 3],
    bmax: [f32; 3],
    res: (usize, usize, usize),
    scale: f32,
    col_bottom: [f32; 4],
    col_top: [f32; 4],
    sdf: F,
) where
    F: Fn(Point3) -> f64,
{
    let (nx, ny, nz) = (res.0.max(6), res.1.max(6), res.2.max(6));
    let dx = (bmax[0] - bmin[0]) as f64 / (nx - 1) as f64;
    let dy = (bmax[1] - bmin[1]) as f64 / (ny - 1) as f64;
    let dz = (bmax[2] - bmin[2]) as f64 / (nz - 1) as f64;

    let total_cells = nx * ny * nz;
    let mut grid = Vec::with_capacity(total_cells);
    for zk in 0..nz {
        let z = bmin[2] as f64 + zk as f64 * dz;
        for yj in 0..ny {
            let y = bmin[1] as f64 + yj as f64 * dy;
            for xi in 0..nx {
                let x = bmin[0] as f64 + xi as f64 * dx;
                grid.push(sdf(Point3::new(x, y, z)));
            }
        }
    }

    let max_verts = (nx - 1) * (ny - 1) * (nz - 1) * 15;
    let max_tris = (nx - 1) * (ny - 1) * (nz - 1) * 5;
    let mut out_v = vec![Point3::default(); max_verts];
    let mut out_t = vec![[0u32; 3]; max_tris];

    if let Ok((vert_count, tri_count)) = marching_cubes(
        &grid,
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
        bmin[0] as f64,
        bmin[1] as f64,
        bmin[2] as f64,
        0.0,
        &mut out_v,
        &mut out_t,
    ) {
        if vert_count == 0 || tri_count == 0 {
            return;
        }
        let mut unique_verts = Vec::with_capacity(vert_count / 3);
        let mut welded_indices = Vec::with_capacity(vert_count);
        let mut map = std::collections::HashMap::<(i32, i32, i32), u32>::new();
        let quant = 2000.0_f64;
        for v in out_v.iter().take(vert_count) {
            let key = (
                (v.x * quant).round() as i32,
                (v.y * quant).round() as i32,
                (v.z * quant).round() as i32,
            );
            let idx = *map.entry(key).or_insert_with(|| {
                let new_idx = unique_verts.len() as u32;
                unique_verts.push(*v);
                new_idx
            });
            welded_indices.push(idx);
        }

        let eps = 0.0025_f64;
        let span_y = (bmax[1] - bmin[1]).max(0.01);
        let base_idx = b.positions.len() as u32;

        for uv in &unique_verts {
            let gx = (sdf(Point3::new(uv.x + eps, uv.y, uv.z))
                - sdf(Point3::new(uv.x - eps, uv.y, uv.z)))
                / (2.0 * eps);
            let gy = (sdf(Point3::new(uv.x, uv.y + eps, uv.z))
                - sdf(Point3::new(uv.x, uv.y - eps, uv.z)))
                / (2.0 * eps);
            let gz = (sdf(Point3::new(uv.x, uv.y, uv.z + eps))
                - sdf(Point3::new(uv.x, uv.y, uv.z - eps)))
                / (2.0 * eps);
            let norm = normalize3([gx as f32, gy as f32, gz as f32]);
            let lx = uv.x as f32;
            let ly = uv.y as f32;
            let lz = uv.z as f32;
            let t = ((ly - bmin[1]) / span_y).clamp(0.0, 1.0);
            let col = lerp_color(col_bottom, col_top, t);
            b.push_vertex(
                [lx * scale, ly * scale, lz * scale],
                norm,
                col,
                [0.5, t],
            );
        }

        for t in 0..tri_count {
            let i0 = welded_indices[t * 3];
            let i1 = welded_indices[t * 3 + 1];
            let i2 = welded_indices[t * 3 + 2];
            if i0 != i1 && i1 != i2 && i0 != i2 {
                b.push_tri(base_idx + i0, base_idx + i1, base_idx + i2);
            }
        }
    }
}

/// Build a dedicated, high-resolution engineered road corridor mesh (`240` longitudinal stations ×
/// `12` cross-sectional profile points), W-beam steel safety guardrails, upslope stone/timber retaining
/// cribs, cross-drainage culverts, Australian roadside reflector guideposts, and a riparian timber
/// viewing deck & ecological telemetry station.
pub fn build_engineered_road_and_furniture(
    sector_id: &str,
    water_y: f32,
    z_max: f32,
    sample_elev: &dyn Fn(f32, f32) -> f32,
    road_center_x: &dyn Fn(f32) -> f32,
    b_terrain: &mut SubmeshBuilder,
    b_lithology: &mut SubmeshBuilder,
    b_trunks: &mut SubmeshBuilder,
) {
    let z_start = -1.92_f32;
    let z_end = (z_max - 0.015).max(-1.75);
    let n_stations = 240usize;

    // 12 lateral offsets across the road bench (in normalized sector units; 1 m = 0.045):
    // [-0.112: river-side drainage swale lip]
    // [-0.092: river-side red-ochre gravel shoulder outer edge]
    // [-0.068: left bitumen pavement edge]
    // [-0.063: left white edge line outer]
    // [-0.057: left white edge line inner]
    // [-0.0045: dashed yellow centerline left]
    // [+0.0045: dashed yellow centerline right]
    // [+0.057: right white edge line inner]
    // [+0.063: right white edge line outer]
    // [+0.068: right bitumen pavement edge]
    // [+0.092: upslope red-ochre gravel shoulder outer edge]
    // [+0.112: upslope drainage swale outer lip]
    let offsets: [f32; 12] = [
        -0.112, -0.092, -0.068, -0.063, -0.057, -0.0045, 0.0045, 0.057, 0.063, 0.068, 0.092, 0.112,
    ];
    // Cambered road crown elevation above the terrain bench so there is zero z-fighting
    let crown_lift: [f32; 12] = [
        0.0008, 0.0032, 0.0052, 0.0056, 0.0058, 0.0068, 0.0068, 0.0058, 0.0056, 0.0052, 0.0032,
        0.0008,
    ];

    let mut station_pts = vec![[[0.0f32; 3]; 12]; n_stations + 1];
    let mut station_norms = vec![[[0.0f32; 3]; 12]; n_stations + 1];
    let mut station_centers = vec![[0.0f32; 3]; n_stations + 1];
    let mut station_lat_vec = vec![[1.0f32, 0.0]; n_stations + 1];

    for i in 0..=n_stations {
        let t = i as f32 / n_stations as f32;
        let z = z_start + t * (z_end - z_start);
        let cx = road_center_x(z);
        let dz_eps = 0.015_f32;
        let dcx = road_center_x(z + dz_eps) - road_center_x(z - dz_eps);
        let tang_len = (dcx * dcx + (2.0 * dz_eps).powi(2)).sqrt().max(1e-5);
        let tx = dcx / tang_len;
        let tz = (2.0 * dz_eps) / tang_len;
        // Unit normal in XZ plane pointing to the right (+X side of road)
        let nx = tz;
        let nz = -tx;
        station_lat_vec[i] = [nx, nz];
        let cy = sample_elev(cx, z);
        station_centers[i] = [cx, cy, z];

        for j in 0..12 {
            let s_off = offsets[j];
            let px = cx + nx * s_off;
            let pz = z + nz * s_off;
            let base_y = sample_elev(px, pz);
            // Blend road bench height smoothly across the carriageway so the surface is planar & cambered
            let carriage_blend = (1.0 - (s_off.abs() / 0.105).powi(2)).clamp(0.0, 1.0);
            let py = base_y * (1.0 - 0.65 * carriage_blend)
                + cy * (0.65 * carriage_blend)
                + crown_lift[j];
            station_pts[i][j] = [px, py, pz];
            let camber_nx = -s_off * 1.4;
            station_norms[i][j] = normalize3([camber_nx, 1.0, 0.0]);
        }
    }

    // Emit crisp, unblurred quad strips per longitudinal interval
    for i in 0..n_stations {
        let v0_coord = (i as f32 / n_stations as f32) * 16.0;
        let v1_coord = ((i + 1) as f32 / n_stations as f32) * 16.0;
        let z_mid = 0.5 * (station_centers[i][2] + station_centers[i + 1][2]);
        let micro_var = 0.94 + 0.06 * ((z_mid * 19.3).sin() * (z_mid * 7.1).cos());

        // Dashed yellow centerline pattern: 3 stations solid yellow, 3 stations dark asphalt gap
        let is_yellow_dash = (i % 6) < 3;

        for strip in 0..11 {
            let col: [f32; 4] = match strip {
                0 | 10 => [
                    0.48 * micro_var,
                    0.35 * micro_var,
                    0.22 * micro_var,
                    1.0,
                ], // Graded drainage swale verge
                1 | 9 => [
                    0.66 * micro_var,
                    0.44 * micro_var,
                    0.27 * micro_var,
                    1.0,
                ], // Compacted red-ochre gravel shoulder
                2 | 8 => [
                    0.24 * micro_var,
                    0.25 * micro_var,
                    0.27 * micro_var,
                    1.0,
                ], // Bitumen outer margin
                3 | 7 => [0.95, 0.96, 0.94, 1.0], // Crisp continuous white highway edge line
                4 | 6 => [
                    0.22 * micro_var,
                    0.23 * micro_var,
                    0.25 * micro_var,
                    1.0,
                ], // Compacted dark basalt-bitumen travel lane
                5 => {
                    if is_yellow_dash {
                        [0.98, 0.82, 0.14, 1.0] // Sharp golden-yellow highway centerline dash
                    } else {
                        [
                            0.22 * micro_var,
                            0.23 * micro_var,
                            0.25 * micro_var,
                            1.0,
                        ]
                    }
                }
                _ => [0.25, 0.26, 0.28, 1.0],
            };

            let p00 = station_pts[i][strip];
            let p10 = station_pts[i][strip + 1];
            let p11 = station_pts[i + 1][strip + 1];
            let p01 = station_pts[i + 1][strip];

            let n00 = station_norms[i][strip];
            let n10 = station_norms[i][strip + 1];
            let n11 = station_norms[i + 1][strip + 1];
            let n01 = station_norms[i + 1][strip];

            let u0 = strip as f32 / 11.0;
            let u1 = (strip + 1) as f32 / 11.0;

            let idx00 = b_terrain.push_vertex(p00, n00, col, [u0, v0_coord]);
            let idx10 = b_terrain.push_vertex(p10, n10, col, [u1, v0_coord]);
            let idx11 = b_terrain.push_vertex(p11, n11, col, [u1, v1_coord]);
            let idx01 = b_terrain.push_vertex(p01, n01, col, [u0, v1_coord]);
            // Note: +Z is forward along stations, +X is right across strips -> (idx00, idx01, idx11, idx10) has +Y normal
            b_terrain.push_quad(idx00, idx01, idx11, idx10);
        }
    }

    // 2. W-Beam Galvanized Steel Safety Guardrail along the River-Drop Shoulder
    // Build 3 guardrail segments along the curved river-facing side (`s_off = -0.082`)
    let guardrail_ranges = [(18usize, 78usize), (96usize, 152usize), (168usize, 222usize)];
    for &(s0, s1) in &guardrail_ranges {
        let end_idx = s1.min(n_stations);
        if s0 + 4 >= end_idx {
            continue;
        }
        let mut upper_rail_axis = Vec::with_capacity((end_idx - s0) / 2 + 2);
        let mut lower_rail_axis = Vec::with_capacity((end_idx - s0) / 2 + 2);
        let mut rail_stations = Vec::with_capacity((end_idx - s0) / 2 + 2);

        for idx in (s0..=end_idx).step_by(2) {
            let c = station_centers[idx];
            let lat = station_lat_vec[idx];
            let gx = c[0] - lat[0] * 0.082;
            let gz = c[2] - lat[1] * 0.082;
            let gy = sample_elev(gx, gz) + 0.003;

            upper_rail_axis.push(Point3::new(gx as f64, (gy + 0.032) as f64, gz as f64));
            lower_rail_axis.push(Point3::new(gx as f64, (gy + 0.022) as f64, gz as f64));
            rail_stations.push(ExtrusionStation {
                scale_x: 0.0028,
                scale_y: 0.0042,
                twist_rad: 0.0,
            });

            // Vertical galvanized steel I-beam support post every 6 stations
            if (idx - s0) % 6 == 0 {
                b_lithology.add_local_box(
                    [gx - 0.0032, gy - 0.004, gz - 0.0032],
                    [gx + 0.0032, gy + 0.036, gz + 0.0032],
                    [0.52, 0.55, 0.58, 1.0],
                    [0.76, 0.79, 0.83, 1.0],
                );
                // Retroreflective red safety tab on every guardrail post
                b_lithology.add_local_box(
                    [gx - 0.0038, gy + 0.029, gz - 0.0038],
                    [gx + 0.0038, gy + 0.035, gz + 0.0038],
                    [0.92, 0.16, 0.12, 1.0],
                    [0.98, 0.28, 0.20, 1.0],
                );
            }
        }
        b_lithology.add_bishop_swept_tube(
            &upper_rail_axis,
            &rail_stations,
            6,
            [0.64, 0.67, 0.72, 1.0],
            [0.86, 0.89, 0.93, 1.0],
        );
        b_lithology.add_bishop_swept_tube(
            &lower_rail_axis,
            &rail_stations,
            6,
            [0.58, 0.61, 0.66, 1.0],
            [0.80, 0.83, 0.88, 1.0],
        );
    }

    // 3. Australian Highway Reflector Guideposts along both sides of the corridor
    let post_step = 10usize;
    for idx in (8..n_stations.saturating_sub(6)).step_by(post_step) {
        let c = station_centers[idx];
        let lat = station_lat_vec[idx];
        for &side in &[-1.0_f32, 1.0_f32] {
            let px = c[0] + lat[0] * (side * 0.088);
            let pz = c[2] + lat[1] * (side * 0.088);
            let py = sample_elev(px, pz) + 0.002;
            // White flexible guidepost shaft (1.05 m metric = 0.047 world units)
            b_lithology.add_local_box(
                [px - 0.0038, py, pz - 0.0022],
                [px + 0.0038, py + 0.046, pz + 0.0022],
                [0.88, 0.90, 0.88, 1.0],
                [0.98, 0.98, 0.97, 1.0],
            );
            // Black base collar
            b_lithology.add_local_box(
                [px - 0.0042, py, pz - 0.0026],
                [px + 0.0042, py + 0.009, pz + 0.0026],
                [0.14, 0.15, 0.16, 1.0],
                [0.22, 0.23, 0.25, 1.0],
            );
            // Retroreflective top band (Red on left side, Amber/White on right side)
            let ref_col = if side < 0.0 {
                [0.96, 0.14, 0.10, 1.0]
            } else {
                [0.98, 0.84, 0.16, 1.0]
            };
            b_lithology.add_local_box(
                [px - 0.0042, py + 0.036, pz - 0.0028],
                [px + 0.0042, gy_clamp(py + 0.044), pz + 0.0028],
                ref_col,
                ref_col,
            );
        }
    }

    // 4. Upslope Cut-Bank Dressed Stone & Timber Retaining Crib Wall + Cross-Drainage Culverts
    let crib_ranges = [(36usize, 84usize), (132usize, 180usize)];
    let stone_col_b = match sector_id {
        "sydney_sandstone" => [0.58, 0.42, 0.26, 1.0],
        "yarrangobilly" | "jounama_karst" => [0.50, 0.52, 0.56, 1.0],
        _ => [0.46, 0.44, 0.41, 1.0],
    };
    let stone_col_t = match sector_id {
        "sydney_sandstone" => [0.78, 0.60, 0.40, 1.0],
        "yarrangobilly" | "jounama_karst" => [0.72, 0.74, 0.78, 1.0],
        _ => [0.68, 0.65, 0.60, 1.0],
    };

    for &(c0, c1) in &crib_ranges {
        let end_idx = c1.min(n_stations.saturating_sub(2));
        for idx in (c0..end_idx).step_by(4) {
            let p_a = station_centers[idx];
            let p_b = station_centers[idx + 4];
            let lat_a = station_lat_vec[idx];
            let lat_b = station_lat_vec[idx + 4];

            let wx0 = p_a[0] + lat_a[0] * 0.106;
            let wz0 = p_a[2] + lat_a[1] * 0.106;
            let wy0 = sample_elev(wx0, wz0);

            let wx1 = p_b[0] + lat_b[0] * 0.106;
            let wz1 = p_b[2] + lat_b[1] * 0.106;
            let wy1 = sample_elev(wx1, wz1);

            // 3 tiers of stone retaining blocks + timber crib stretcher beams
            for tier in 0..3 {
                let ty = tier as f32 * 0.016;
                let setback = tier as f32 * 0.005;
                add_oriented_beam(
                    b_lithology,
                    [wx0 + lat_a[0] * setback, wy0 + ty, wz0 + lat_a[1] * setback],
                    [wx1 + lat_b[0] * setback, wy1 + ty, wz1 + lat_b[1] * setback],
                    [0.0, 1.0, 0.0],
                    0.009,
                    0.007,
                    stone_col_b,
                    stone_col_t,
                );
                add_oriented_beam(
                    b_trunks,
                    [
                        wx0 + lat_a[0] * (setback - 0.004),
                        wy0 + ty + 0.008,
                        wz0 + lat_a[1] * (setback - 0.004),
                    ],
                    [
                        wx1 + lat_b[0] * (setback - 0.004),
                        wy1 + ty + 0.008,
                        wz1 + lat_b[1] * (setback - 0.004),
                    ],
                    [0.0, 1.0, 0.0],
                    0.004,
                    0.0035,
                    [0.34, 0.24, 0.16, 1.0],
                    [0.48, 0.35, 0.24, 1.0],
                );
            }
        }
    }

    // 3 Cross-Drainage Concrete / Corrugated Culvert Pipes & Headwalls under the road bench
    for &culvert_idx in &[52usize, 118usize, 186usize] {
        if culvert_idx >= n_stations {
            continue;
        }
        let c = station_centers[culvert_idx];
        let lat = station_lat_vec[culvert_idx];
        let p_in = [
            c[0] + lat[0] * 0.105,
            sample_elev(c[0] + lat[0] * 0.105, c[2] + lat[1] * 0.105) - 0.006,
            c[2] + lat[1] * 0.105,
        ];
        let p_out = [
            c[0] - lat[0] * 0.105,
            sample_elev(c[0] - lat[0] * 0.105, c[2] - lat[1] * 0.105) - 0.010,
            c[2] - lat[1] * 0.105,
        ];
        let culvert_axis = [
            Point3::new(p_in[0] as f64, p_in[1] as f64, p_in[2] as f64),
            Point3::new(p_out[0] as f64, p_out[1] as f64, p_out[2] as f64),
        ];
        let culvert_st = [
            ExtrusionStation {
                scale_x: 0.012,
                scale_y: 0.012,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.012,
                scale_y: 0.012,
                twist_rad: 0.0,
            },
        ];
        b_lithology.add_bishop_swept_tube(
            &culvert_axis,
            &culvert_st,
            10,
            [0.42, 0.44, 0.46, 1.0],
            [0.68, 0.70, 0.72, 1.0],
        );
        // Concrete headwalls at inlet and outlet
        for ep in [p_in, p_out] {
            b_lithology.add_local_box(
                [ep[0] - 0.008, ep[1] - 0.012, ep[2] - 0.022],
                [ep[0] + 0.008, ep[1] + 0.018, ep[2] + 0.022],
                stone_col_b,
                stone_col_t,
            );
        }
    }

    // 5. Riparian Timber Viewing Deck & Solar Ecological Telemetry Station near `z = -0.46`
    let z_deck = (-0.46_f32).min(z_end - 0.18);
    let x_road_deck = road_center_x(z_deck);
    let deck_cx = x_road_deck - 0.185;
    let deck_cz = z_deck;
    let deck_ground_y = sample_elev(deck_cx, deck_cz).max(water_y + 0.02);
    let deck_top_y = (sample_elev(x_road_deck - 0.09, deck_cz) + 0.012).max(deck_ground_y + 0.038);
    let deck_hw_x = 0.075_f32;
    let deck_hw_z = 0.110_f32;

    let timber_dark = [0.36, 0.24, 0.15, 1.0];
    let timber_lit = [0.56, 0.39, 0.25, 1.0];

    // 6 treated hardwood support piles & perimeter balustrade posts
    for &dx in &[-deck_hw_x, deck_hw_x] {
        for &dz in &[-deck_hw_z, 0.0_f32, deck_hw_z] {
            let px = deck_cx + dx;
            let pz = deck_cz + dz;
            let py_base = sample_elev(px, pz).min(deck_ground_y) - 0.02;
            // Keep road-side entry open at (dx > 0, dz == 0)
            let post_top = if dx > 0.0 && dz.abs() < 0.01 {
                deck_top_y
            } else {
                deck_top_y + 0.046
            };
            b_trunks.add_local_box(
                [px - 0.0045, py_base, pz - 0.0045],
                [px + 0.0045, post_top, pz + 0.0045],
                timber_dark,
                timber_lit,
            );
        }
    }

    // Sub-frame bearers
    for &dz in &[-deck_hw_z, 0.0_f32, deck_hw_z] {
        b_trunks.add_local_box(
            [deck_cx - deck_hw_x - 0.004, deck_top_y - 0.009, deck_cz + dz - 0.004],
            [deck_cx + deck_hw_x + 0.004, deck_top_y - 0.002, deck_cz + dz + 0.004],
            timber_dark,
            timber_lit,
        );
    }

    // 14 individual spaced hardwood decking planks with shadow gaps between planks
    let n_planks = 14usize;
    for p_idx in 0..n_planks {
        let u0 = p_idx as f32 / n_planks as f32;
        let u1 = (p_idx as f32 + 0.82) / n_planks as f32;
        let z0 = deck_cz - deck_hw_z + u0 * (2.0 * deck_hw_z);
        let z1 = deck_cz - deck_hw_z + u1 * (2.0 * deck_hw_z);
        let plank_var = 0.92 + 0.14 * ((p_idx as f32 * 3.7).sin().abs());
        b_trunks.add_local_box(
            [deck_cx - deck_hw_x - 0.003, deck_top_y - 0.002, z0],
            [deck_cx + deck_hw_x + 0.003, deck_top_y + 0.0025, z1],
            scale_color(timber_dark, plank_var),
            scale_color(timber_lit, plank_var),
        );
    }

    // Top timber handrail & mid-height stainless cable rails on river-facing (-X) and N/S ends
    let rail_y = deck_top_y + 0.044;
    let mid_y = deck_top_y + 0.024;
    // River-side (-X) top rail
    b_trunks.add_local_box(
        [
            deck_cx - deck_hw_x - 0.0055,
            rail_y,
            deck_cz - deck_hw_z - 0.005,
        ],
        [
            deck_cx - deck_hw_x + 0.0055,
            rail_y + 0.0045,
            deck_cz + deck_hw_z + 0.005,
        ],
        timber_dark,
        timber_lit,
    );
    b_lithology.add_local_box(
        [
            deck_cx - deck_hw_x - 0.0015,
            mid_y,
            deck_cz - deck_hw_z,
        ],
        [
            deck_cx - deck_hw_x + 0.0015,
            mid_y + 0.002,
            deck_cz + deck_hw_z,
        ],
        [0.72, 0.75, 0.80, 1.0],
        [0.90, 0.92, 0.96, 1.0],
    );
    // North & South end top rails
    for &end_sign in &[-1.0_f32, 1.0_f32] {
        let rz = deck_cz + end_sign * deck_hw_z;
        b_trunks.add_local_box(
            [deck_cx - deck_hw_x - 0.005, rail_y, rz - 0.005],
            [deck_cx + deck_hw_x + 0.005, rail_y + 0.0045, rz + 0.005],
            timber_dark,
            timber_lit,
        );
    }

    // Solar Micro-Climate & Hydrological Telemetry Station Mast at corner of deck
    let mast_x = deck_cx - deck_hw_x + 0.012;
    let mast_z = deck_cz + deck_hw_z - 0.012;
    b_lithology.add_local_box(
        [mast_x - 0.003, deck_top_y, mast_z - 0.003],
        [mast_x + 0.003, deck_top_y + 0.135, mast_z + 0.003],
        [0.68, 0.72, 0.76, 1.0],
        [0.88, 0.90, 0.94, 1.0],
    );
    // NEMA weatherproof telemetry enclosure box
    b_lithology.add_local_box(
        [mast_x - 0.010, deck_top_y + 0.052, mast_z - 0.008],
        [mast_x + 0.010, deck_top_y + 0.082, mast_z + 0.008],
        [0.84, 0.86, 0.88, 1.0],
        [0.94, 0.95, 0.96, 1.0],
    );
    // Angled solar panel & ultrasonic anemometer crossbar at mast top
    add_oriented_beam(
        b_lithology,
        [mast_x - 0.018, deck_top_y + 0.105, mast_z - 0.014],
        [mast_x + 0.018, deck_top_y + 0.118, mast_z + 0.014],
        [0.0, 1.0, 0.0],
        0.016,
        0.002,
        [0.10, 0.16, 0.32, 1.0],
        [0.18, 0.28, 0.52, 1.0],
    );
    b_lithology.add_local_box(
        [mast_x - 0.002, deck_top_y + 0.132, mast_z - 0.022],
        [mast_x + 0.002, deck_top_y + 0.136, mast_z + 0.022],
        [0.75, 0.78, 0.82, 1.0],
        [0.92, 0.94, 0.98, 1.0],
    );
}

#[inline]
fn gy_clamp(v: f32) -> f32 {
    v
}

#[inline]
fn scale_color(c: [f32; 4], f: f32) -> [f32; 4] {
    [
        (c[0] * f).clamp(0.0, 1.0),
        (c[1] * f).clamp(0.0, 1.0),
        (c[2] * f).clamp(0.0, 1.0),
        c[3],
    ]
}

/// Build a true-scale (`4.85 m × 1.88 m × 1.92 m` at `scale = 1.0`) high-detail CAD 4WD Ecological
/// Research Utility Vehicle in local template coordinates (`+X` forward, `Y` up with tyres touching
/// `y = 0.0`, `Z` lateral).
///
/// Features:
/// - Sculpted front bonnet with raised power-dome hood scoop, chamfered fenders, and `recompute_normals_since`
/// - `wheel_clearance_panel_into` side panels + 3D flared trapezoidal off-road wheel-arch lips & rock sliders
/// - Distinct **body-colored A-pillars, B-pillars, C-pillars, cant-rails, roof cap, and lower doors**
///   framing **recessed dark-tinted specular windshield, front/rear side windows, and rear cab glass**
/// - Wing mirrors on stalks, door handles, radiator grille, LED projector headlamps, and amber indicators
/// - Modular field-lab rear canopy with recessed charcoal gullwing instrument hatches, hi-vis orange stripes,
///   rear tailgate-mounted spare mud-terrain wheel (`revolve_profile`), and recovery tow hitch
/// - Multi-hoop Australian steel bullbar (`add_bishop_swept_tube`), winch fairlead, twin spotlights,
///   right A-pillar safari snorkel, tubular roof rack, monocrystalline solar array, 3D LiDAR dome, and VHF whip antenna
/// - 4 revolved mud-terrain off-road tyres (`revolve_profile`) with shoulder lug treads, 6-spoke alloy rims,
///   brake discs, live axles, differential pumpkins, and coil-over suspension struts.
pub fn build_cad_field_rover_4wd_local(b: &mut SubmeshBuilder, scale: f32) {
    let s = scale;

    // Palette
    let body_bot = [0.76, 0.71, 0.57, 1.0]; // Australian desert limestone / sand expedition finish (lower)
    let body_top = [0.92, 0.88, 0.76, 1.0]; // Limestone / sand upper highlight
    let roof_white = [0.94, 0.95, 0.96, 1.0]; // Heat-reflective expedition white roof
    let glass_dark = [0.07, 0.12, 0.18, 1.0]; // Deep tinted specular automotive glass
    let glass_sky = [0.15, 0.26, 0.38, 1.0]; // Upper sky-reflecting windshield tint
    let trim_black = [0.13, 0.14, 0.16, 1.0]; // Matte black polymer flares / seals / bumpers
    let trim_lit = [0.22, 0.24, 0.27, 1.0];
    let steel_dark = [0.20, 0.22, 0.25, 1.0]; // Heavy powder-coated steel chassis / bullbar
    let steel_lit = [0.38, 0.41, 0.45, 1.0];

    // 1. Twin Ladder-Frame Chassis Rails, Underbody Skid Plate, Live Axles & Coil Suspension
    for &rail_z in &[-0.42_f32 * s, 0.42_f32 * s] {
        b.add_local_box(
            [-2.38 * s, 0.28 * s, rail_z - 0.05 * s],
            [2.28 * s, 0.40 * s, rail_z + 0.05 * s],
            [0.12, 0.13, 0.15, 1.0],
            [0.20, 0.22, 0.24, 1.0],
        );
    }
    // Underbody transfer-case & sump protection skid plate
    b.add_local_box(
        [-1.85 * s, 0.30 * s, -0.54 * s],
        [1.95 * s, 0.38 * s, 0.54 * s],
        [0.14, 0.15, 0.17, 1.0],
        [0.22, 0.24, 0.26, 1.0],
    );
    // Front angled alloy bash plate under bullbar
    add_oriented_beam(
        b,
        [1.92 * s, 0.30 * s, 0.0],
        [2.36 * s, 0.46 * s, 0.0],
        [0.0, 0.0, 1.0],
        0.018 * s,
        0.48 * s,
        [0.52, 0.55, 0.58, 1.0],
        [0.74, 0.77, 0.80, 1.0],
    );

    // Front and rear live axles, differential housings & coil springs at x = ±1.425 * s
    let wheel_axles = [-1.425 * s, 1.425 * s];
    for &ax in &wheel_axles {
        let axle_axis = [
            Point3::new(ax as f64, (0.39 * s) as f64, (-0.78 * s) as f64),
            Point3::new(ax as f64, (0.39 * s) as f64, (0.78 * s) as f64),
        ];
        let r_ax = (0.042 * s) as f64;
        let axle_st = [
            ExtrusionStation {
                scale_x: r_ax,
                scale_y: r_ax,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: r_ax,
                scale_y: r_ax,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(
            &axle_axis,
            &axle_st,
            8,
            [0.12, 0.13, 0.15, 1.0],
            [0.22, 0.24, 0.27, 1.0],
        );
        // Differential pumpkin housing (offset slightly to right)
        b.add_local_box(
            [ax - 0.11 * s, 0.30 * s, 0.08 * s],
            [ax + 0.11 * s, 0.48 * s, 0.28 * s],
            [0.14, 0.15, 0.17, 1.0],
            [0.24, 0.26, 0.28, 1.0],
        );
        // Yellow Old-Man-Emu heavy-duty suspension coil struts
        for &sz in &[-0.56_f32 * s, 0.56_f32 * s] {
            let strut_axis = [
                Point3::new(ax as f64, (0.40 * s) as f64, sz as f64),
                Point3::new(ax as f64, (0.72 * s) as f64, sz as f64),
            ];
            let r_str = (0.045 * s) as f64;
            let strut_st = [
                ExtrusionStation {
                    scale_x: r_str,
                    scale_y: r_str,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: r_str,
                    scale_y: r_str,
                    twist_rad: 0.0,
                },
            ];
            b.add_bishop_swept_tube(
                &strut_axis,
                &strut_st,
                8,
                [0.82, 0.66, 0.08, 1.0],
                [0.96, 0.80, 0.14, 1.0],
            );
        }
    }

    // 2. Lower Body Side Panels with Circular Wheel-Clearance Cutouts (`wheel_clearance_panel_into`)
    // and Recomputed True Geometric Normals!
    for &side_z in &[-0.85_f32, 0.85_f32] {
        let recipe = WheelPanel {
            center_x: side_z * s,
            thickness: 0.065 * s,
            z: [-2.36 * s, 2.34 * s],
            bottom: 0.38 * s,
            top: [1.14 * s, 1.14 * s],
            wheel_y: 0.41 * s,
            clearance_radius: 0.49 * s,
            axles: &wheel_axles,
            segments: 32,
        };
        let sections = recipe.segments as usize + 1;
        let mut panel_v = vec![[0.0f32; 3]; sections * 10];
        let mut panel_t = vec![[0u32; 3]; sections * 8];
        if let Ok(receipt) = wheel_clearance_panel_into(recipe, &mut panel_v, &mut panel_t) {
            let v_start = b.positions.len();
            let i_start = b.indices.len();
            for p in &panel_v[..receipt.vertex_count] {
                let vx = p[2];
                let vy = p[1];
                let vz = p[0];
                let t = ((vy - 0.38 * s) / (0.76 * s).max(1e-4)).clamp(0.0, 1.0);
                let col = lerp_color(body_bot, body_top, t);
                let u = ((vx + 2.36 * s) / (4.70 * s).max(1e-4)).clamp(0.0, 1.0);
                b.push_vertex([vx, vy, vz], [0.0, 1.0, 0.0], col, [u, t]);
            }
            for tri in &panel_t[..receipt.triangle_count] {
                b.push_tri(
                    v_start as u32 + tri[0],
                    v_start as u32 + tri[2],
                    v_start as u32 + tri[1],
                );
            }
            b.recompute_normals_since(v_start, i_start);
        }

        // 3D Sculpted Matte-Black Trapezoidal Off-Road Wheel-Arch Flares around both axles
        let z_flare = (side_z + side_z.signum() * 0.048) * s;
        for &ax in &wheel_axles {
            let mut flare_axis = Vec::with_capacity(9);
            let mut flare_st = Vec::with_capacity(9);
            for k in 0..9 {
                let frac = k as f64 / 8.0;
                // Sweep angle from 0.05*pi (front lip) to 0.95*pi (rear lip)
                let ang = std::f64::consts::PI * (0.04 + 0.92 * frac);
                let r_arch = (0.495 * s) as f64;
                let fx = ax as f64 + r_arch * ang.cos();
                let fy = (0.40 * s) as f64 + r_arch * ang.sin() * 0.94;
                flare_axis.push(Point3::new(fx, fy, z_flare as f64));
                flare_st.push(ExtrusionStation {
                    scale_x: (0.038 * s) as f64,
                    scale_y: (0.046 * s) as f64,
                    twist_rad: 0.0,
                });
            }
            b.add_bishop_swept_tube(&flare_axis, &flare_st, 6, trim_black, trim_lit);
        }

        // Heavy-duty tubular steel rock-slider side steps between front and rear wheels
        let step_z = (side_z + side_z.signum() * 0.065) * s;
        let step_axis = [
            Point3::new((-0.86 * s) as f64, (0.37 * s) as f64, (side_z * s) as f64),
            Point3::new((-0.78 * s) as f64, (0.37 * s) as f64, step_z as f64),
            Point3::new((0.78 * s) as f64, (0.37 * s) as f64, step_z as f64),
            Point3::new((0.86 * s) as f64, (0.37 * s) as f64, (side_z * s) as f64),
        ];
        let step_st = vec![
            ExtrusionStation {
                scale_x: (0.032 * s) as f64,
                scale_y: (0.024 * s) as f64,
                twist_rad: 0.0,
            };
            step_axis.len()
        ];
        b.add_bishop_swept_tube(&step_axis, &step_st, 6, steel_dark, steel_lit);
    }

    // 3. Sculpted Chamfered Front Bonnet with Raised Power-Dome via `add_vehicle_loft_along_x`
    {
        // 6-vertex convex counterclockwise section in (z_lat, y):
        // bottom-left -> bottom-right -> upper-right fender -> power-dome right -> power-dome left -> upper-left fender
        let bonnet_stations: [(f32, f32, f32, f32, f32); 5] = [
            // (x_station, half_w_bot, half_w_top, y_fender, y_dome)
            (0.68 * s, 0.86 * s, 0.82 * s, 1.15 * s, 1.21 * s),
            (1.12 * s, 0.86 * s, 0.81 * s, 1.14 * s, 1.20 * s),
            (1.62 * s, 0.84 * s, 0.79 * s, 1.12 * s, 1.18 * s),
            (2.08 * s, 0.81 * s, 0.75 * s, 1.09 * s, 1.14 * s),
            (2.33 * s, 0.77 * s, 0.71 * s, 1.05 * s, 1.08 * s),
        ];
        let mut sections = Vec::with_capacity(bonnet_stations.len());
        for &(xs, hw_b, hw_t, yf, yd) in &bonnet_stations {
            let y_bot = 0.76 * s;
            let dome_w = hw_t * 0.48;
            sections.push(vec![
                [-hw_b, y_bot, xs],
                [hw_b, y_bot, xs],
                [hw_t, yf, xs],
                [dome_w, yd, xs],
                [-dome_w, yd, xs],
                [-hw_t, yf, xs],
            ]);
        }
        add_vehicle_loft_along_x(b, &sections, body_bot, body_top);

        // Intercooler hood scoop intake slot on top of bonnet power-dome
        b.add_local_box(
            [1.18 * s, 1.18 * s, -0.22 * s],
            [1.56 * s, 1.23 * s, 0.22 * s],
            trim_black,
            body_top,
        );
    }

    // Front Radiator Grille, Slats, LED Projector Headlights & Amber Turn Signals
    b.add_local_box(
        [2.29 * s, 0.76 * s, -0.70 * s],
        [2.35 * s, 1.06 * s, 0.70 * s],
        [0.10, 0.11, 0.13, 1.0],
        [0.18, 0.20, 0.22, 1.0],
    );
    // Horizontal chrome/silver radiator grille bars
    for bar_y in [0.83_f32, 0.91_f32, 0.99_f32] {
        b.add_local_box(
            [2.33 * s, (bar_y - 0.012) * s, -0.44 * s],
            [2.36 * s, (bar_y + 0.012) * s, 0.44 * s],
            [0.65, 0.68, 0.72, 1.0],
            [0.88, 0.90, 0.94, 1.0],
        );
    }
    for &side in &[-1.0_f32, 1.0_f32] {
        // LED Headlamp housing + bright projector lens
        b.add_local_box(
            [2.31 * s, 0.84 * s, (side * 0.56 - 0.11) * s],
            [2.36 * s, 1.03 * s, (side * 0.56 + 0.11) * s],
            [0.92, 0.95, 0.98, 1.0],
            [1.00, 1.00, 0.96, 1.0],
        );
        // Outer amber indicator lamp
        b.add_local_box(
            [2.30 * s, 0.85 * s, (side * 0.71 - 0.035) * s],
            [2.35 * s, 1.01 * s, (side * 0.71 + 0.035) * s],
            [0.95, 0.52, 0.04, 1.0],
            [1.00, 0.74, 0.12, 1.0],
        );
    }

    // 4. Dual-Cab Greenhouse with Recessed Specular Glass Core AND Explicit Body-Colored
    // A-Pillars, B-Pillars, C-Pillars, Lower Door Panels, Wing Mirrors & Roof Cap!
    {
        // 4a. Recessed dark-tinted specular glass greenhouse core (`x: -0.92*s .. +0.70*s`, `y: 1.14*s .. 1.80*s`)
        // Inset by `0.025*s` inside the pillars so the pillars & door frames stand out in 3D relief!
        let glass_stations: [(f32, f32, f32, f32); 5] = [
            (-0.91 * s, 0.77 * s, 1.14 * s, 1.80 * s),
            (-0.34 * s, 0.77 * s, 1.14 * s, 1.80 * s),
            (0.18 * s, 0.76 * s, 1.14 * s, 1.79 * s),
            (0.58 * s, 0.78 * s, 1.14 * s, 1.34 * s),
            (0.68 * s, 0.79 * s, 1.14 * s, 1.18 * s),
        ];
        let mut g_sec = Vec::with_capacity(glass_stations.len());
        for &(xs, hw, yb, yt) in &glass_stations {
            g_sec.push(vec![
                [-hw, yb, xs],
                [hw, yb, xs],
                [hw * 0.92, yt, xs],
                [-hw * 0.92, yt, xs],
            ]);
        }
        add_vehicle_loft_along_x(b, &g_sec, glass_dark, glass_sky);

        // 4b. Sculpted Heat-Reflective White Cab Roof Panel (`y: 1.79*s .. 1.86*s`, overhanging glass)
        let roof_stations: [(f32, f32, f32, f32); 4] = [
            (-0.95 * s, 0.78 * s, 1.79 * s, 1.86 * s),
            (-0.40 * s, 0.78 * s, 1.79 * s, 1.86 * s),
            (0.14 * s, 0.76 * s, 1.78 * s, 1.85 * s),
            (0.26 * s, 0.73 * s, 1.77 * s, 1.82 * s),
        ];
        let mut r_sec = Vec::with_capacity(roof_stations.len());
        for &(xs, hw, yb, yt) in &roof_stations {
            r_sec.push(vec![
                [-hw, yb, xs],
                [hw, yb, xs],
                [hw * 0.94, yt, xs],
                [-hw * 0.94, yt, xs],
            ]);
        }
        add_vehicle_loft_along_x(b, &r_sec, body_top, roof_white);

        // 4c. Explicit 3D Body-Colored A-Pillars, B-Pillars, C-Pillars, Window Sills & Door Handles
        for &side in &[-1.0_f32, 1.0_f32] {
            let z_sill = side * 0.82 * s;
            let z_roof = side * 0.73 * s;

            // Sloped A-Pillar (bonnet cowl `x=0.70*s, y=1.14*s` -> front roof corner `x=0.20*s, y=1.82*s`)
            add_oriented_beam(
                b,
                [0.70 * s, 1.14 * s, z_sill],
                [0.20 * s, 1.82 * s, z_roof],
                [0.0, 0.0, side],
                0.032 * s,
                0.028 * s,
                body_bot,
                body_top,
            );

            // Center B-Pillar between front and rear crew-cab doors (`x = -0.32*s`)
            add_oriented_beam(
                b,
                [-0.32 * s, 1.12 * s, z_sill],
                [-0.32 * s, 1.82 * s, z_roof],
                [1.0, 0.0, 0.0],
                0.024 * s,
                0.042 * s,
                body_bot,
                body_top,
            );

            // Rear C-Pillar at back of dual cab (`x = -0.90*s`)
            add_oriented_beam(
                b,
                [-0.90 * s, 1.12 * s, z_sill],
                [-0.90 * s, 1.82 * s, z_roof],
                [1.0, 0.0, 0.0],
                0.026 * s,
                0.055 * s,
                body_bot,
                body_top,
            );

            // Horizontal beltline door window sill (`x: -0.94*s .. +0.72*s`, `y: 1.10*s .. 1.17*s`)
            add_oriented_beam(
                b,
                [-0.94 * s, 1.14 * s, z_sill],
                [0.72 * s, 1.14 * s, z_sill],
                [0.0, 1.0, 0.0],
                0.026 * s,
                0.032 * s,
                body_bot,
                body_top,
            );

            // Black exterior door handles (front door at `x = -0.18*s`, rear door at `x = -0.74*s`)
            for &hx in &[-0.18_f32 * s, -0.74_f32 * s] {
                b.add_local_box(
                    [hx - 0.06 * s, 1.02 * s, (side * 0.885 - 0.015) * s],
                    [hx + 0.06 * s, 1.07 * s, (side * 0.885 + 0.015) * s],
                    trim_black,
                    trim_lit,
                );
            }

            // Folding Towing Wing Mirrors on Twin Arms at front door corner (`x = 0.44*s, y = 1.24*s`)
            let mir_x = 0.44 * s;
            let mir_y = 1.25 * s;
            let mir_z_in = side * 0.80 * s;
            let mir_z_out = side * 0.96 * s;
            add_oriented_beam(
                b,
                [mir_x, mir_y, mir_z_in],
                [mir_x - 0.04 * s, mir_y + 0.02 * s, mir_z_out],
                [0.0, 1.0, 0.0],
                0.016 * s,
                0.018 * s,
                trim_black,
                trim_lit,
            );
            // Wing mirror aerodynamic housing
            b.add_local_box(
                [mir_x - 0.08 * s, mir_y - 0.07 * s, mir_z_out - 0.03 * s],
                [mir_x + 0.02 * s, mir_y + 0.11 * s, mir_z_out + 0.03 * s],
                trim_black,
                trim_lit,
            );
        }
    }

    // 5. Rear Modular Field-Lab Expedition Canopy (`x: -2.35*s .. -0.95*s`, `y: 1.12*s .. 1.84*s`)
    {
        let canopy_stations: [(f32, f32, f32, f32); 4] = [
            (-2.35 * s, 0.84 * s, 1.12 * s, 1.83 * s),
            (-1.88 * s, 0.85 * s, 1.12 * s, 1.84 * s),
            (-1.40 * s, 0.85 * s, 1.12 * s, 1.84 * s),
            (-0.96 * s, 0.84 * s, 1.12 * s, 1.84 * s),
        ];
        let mut c_sec = Vec::with_capacity(canopy_stations.len());
        for &(xs, hw, yb, yt) in &canopy_stations {
            c_sec.push(vec![
                [-hw, yb, xs],
                [hw, yb, xs],
                [hw * 0.92, yt, xs],
                [-hw * 0.92, yt, xs],
            ]);
        }
        add_vehicle_loft_along_x(b, &c_sec, body_bot, body_top);

        // Recessed Dark-Charcoal Gullwing Field-Instrument Hatches, Hi-Vis Survey Stripes & Taillights
        for &side in &[-1.0_f32, 1.0_f32] {
            // Gullwing equipment locker door panel
            b.add_local_box(
                [-2.20 * s, 1.22 * s, (side * 0.835 - 0.018) * s],
                [-1.10 * s, 1.72 * s, (side * 0.835 + 0.018) * s],
                [0.18, 0.20, 0.23, 1.0],
                [0.28, 0.31, 0.35, 1.0],
            );
            // High-visibility ecological survey orange/gold safety stripe along canopy
            b.add_local_box(
                [-2.32 * s, 1.15 * s, (side * 0.852 - 0.015) * s],
                [-0.98 * s, 1.24 * s, (side * 0.852 + 0.015) * s],
                [0.95, 0.46, 0.08, 1.0],
                [1.00, 0.64, 0.14, 1.0],
            );
            // Vertical LED combination rear taillights (Red brake + Amber turn + White reverse)
            b.add_local_box(
                [-2.38 * s, 0.86 * s, (side * 0.72 - 0.08) * s],
                [-2.33 * s, 1.10 * s, (side * 0.72 + 0.08) * s],
                [0.86, 0.10, 0.08, 1.0],
                [0.98, 0.22, 0.16, 1.0],
            );
        }

        // Rear Bumper Bar & Recovery Tow Hitch at `x = -2.40 * s`
        b.add_local_box(
            [-2.42 * s, 0.40 * s, -0.84 * s],
            [-2.32 * s, 0.54 * s, 0.84 * s],
            steel_dark,
            steel_lit,
        );
        b.add_local_box(
            [-2.46 * s, 0.36 * s, -0.08 * s],
            [-2.34 * s, 0.46 * s, 0.08 * s],
            [0.85, 0.16, 0.12, 1.0],
            [0.96, 0.24, 0.18, 1.0],
        );
    }

    // 6. Australian Heavy-Duty Multi-Hoop Steel Bullbar, Winch & Twin Driving Spotlights
    {
        // Main lower bumper beam of bullbar
        b.add_local_box(
            [2.32 * s, 0.42 * s, -0.84 * s],
            [2.42 * s, 0.58 * s, 0.84 * s],
            steel_dark,
            steel_lit,
        );
        // Electric winch fairlead & red recovery hooks in center of bullbar
        b.add_local_box(
            [2.36 * s, 0.45 * s, -0.22 * s],
            [2.44 * s, 0.56 * s, 0.22 * s],
            [0.10, 0.11, 0.12, 1.0],
            [0.25, 0.27, 0.30, 1.0],
        );
        for &hk_z in &[-0.32_f32 * s, 0.32_f32 * s] {
            b.add_local_box(
                [2.38 * s, 0.40 * s, hk_z - 0.025 * s],
                [2.45 * s, 0.48 * s, hk_z + 0.025 * s],
                [0.90, 0.14, 0.10, 1.0],
                [0.98, 0.24, 0.16, 1.0],
            );
        }

        // Swept main tubular bullbar wing bar
        let r_tube = (0.036 * s) as f64;
        let bar_axis = [
            Point3::new((2.28 * s) as f64, (0.58 * s) as f64, (-0.84 * s) as f64),
            Point3::new((2.40 * s) as f64, (0.60 * s) as f64, (-0.64 * s) as f64),
            Point3::new((2.425 * s) as f64, (0.60 * s) as f64, 0.0),
            Point3::new((2.40 * s) as f64, (0.60 * s) as f64, (0.64 * s) as f64),
            Point3::new((2.28 * s) as f64, (0.58 * s) as f64, (0.84 * s) as f64),
        ];
        let bar_st = vec![
            ExtrusionStation {
                scale_x: r_tube,
                scale_y: r_tube,
                twist_rad: 0.0,
            };
            bar_axis.len()
        ];
        b.add_bishop_swept_tube(&bar_axis, &bar_st, 8, steel_dark, steel_lit);

        // Center A-frame radiator hoop + headlight protection wings
        let hoop_axis = [
            Point3::new((2.34 * s) as f64, (0.88 * s) as f64, (-0.80 * s) as f64),
            Point3::new((2.40 * s) as f64, (0.96 * s) as f64, (-0.46 * s) as f64),
            Point3::new((2.42 * s) as f64, (1.06 * s) as f64, (-0.32 * s) as f64),
            Point3::new((2.42 * s) as f64, (1.06 * s) as f64, (0.32 * s) as f64),
            Point3::new((2.40 * s) as f64, (0.96 * s) as f64, (0.46 * s) as f64),
            Point3::new((2.34 * s) as f64, (0.88 * s) as f64, (0.80 * s) as f64),
        ];
        let hoop_st = vec![
            ExtrusionStation {
                scale_x: r_tube * 0.82,
                scale_y: r_tube * 0.82,
                twist_rad: 0.0,
            };
            hoop_axis.len()
        ];
        b.add_bishop_swept_tube(&hoop_axis, &hoop_st, 8, steel_dark, steel_lit);

        // Twin round 9-inch LED driving spotlights mounted inside bullbar hoop
        for &sp_z in &[-0.18_f32 * s, 0.18_f32 * s] {
            b.add_local_box(
                [2.37 * s, 0.64 * s, sp_z - 0.09 * s],
                [2.43 * s, 0.82 * s, sp_z + 0.09 * s],
                [0.88, 0.92, 0.98, 1.0],
                [1.00, 1.00, 0.96, 1.0],
            );
        }

        // Tall UHF/VHF whip antenna mounted on left bullbar wing (`z = -0.74 * s`)
        let ant_axis = [
            Point3::new((2.38 * s) as f64, (0.92 * s) as f64, (-0.74 * s) as f64),
            Point3::new((2.36 * s) as f64, (1.55 * s) as f64, (-0.74 * s) as f64),
            Point3::new((2.32 * s) as f64, (2.05 * s) as f64, (-0.74 * s) as f64),
        ];
        let ant_st = [
            ExtrusionStation {
                scale_x: (0.016 * s) as f64,
                scale_y: (0.016 * s) as f64,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: (0.009 * s) as f64,
                scale_y: (0.009 * s) as f64,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: (0.005 * s) as f64,
                scale_y: (0.005 * s) as f64,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(&ant_axis, &ant_st, 6, trim_black, roof_white);
    }

    // 7. Right A-Pillar Raised Safari Snorkel & Ram-Air Intake Head
    {
        let snorkel_axis = [
            Point3::new((1.22 * s) as f64, (1.02 * s) as f64, (0.86 * s) as f64),
            Point3::new((0.74 * s) as f64, (1.16 * s) as f64, (0.86 * s) as f64),
            Point3::new((0.24 * s) as f64, (1.76 * s) as f64, (0.79 * s) as f64),
            Point3::new((0.20 * s) as f64, (1.88 * s) as f64, (0.78 * s) as f64),
        ];
        let r_snork = (0.036 * s) as f64;
        let snork_st = vec![
            ExtrusionStation {
                scale_x: r_snork,
                scale_y: r_snork,
                twist_rad: 0.0,
            };
            snorkel_axis.len()
        ];
        b.add_bishop_swept_tube(&snorkel_axis, &snork_st, 8, trim_black, trim_lit);
        b.add_local_box(
            [0.12 * s, 1.84 * s, 0.72 * s],
            [0.29 * s, 1.94 * s, 0.84 * s],
            trim_black,
            trim_lit,
        );
    }

    // 8. Tubular Expedition Roof Rack, Solar Telemetry Array, LiDAR Dome & Amber Beacon
    {
        let rack_loop = [
            Point3::new((-2.22 * s) as f64, (1.89 * s) as f64, (-0.74 * s) as f64),
            Point3::new((0.22 * s) as f64, (1.89 * s) as f64, (-0.74 * s) as f64),
            Point3::new((0.22 * s) as f64, (1.89 * s) as f64, (0.74 * s) as f64),
            Point3::new((-2.22 * s) as f64, (1.89 * s) as f64, (0.74 * s) as f64),
            Point3::new((-2.22 * s) as f64, (1.89 * s) as f64, (-0.74 * s) as f64),
        ];
        let r_rack = (0.022 * s) as f64;
        let rack_st = vec![
            ExtrusionStation {
                scale_x: r_rack,
                scale_y: r_rack,
                twist_rad: 0.0,
            };
            rack_loop.len()
        ];
        b.add_bishop_swept_tube(&rack_loop, &rack_st, 6, steel_dark, steel_lit);

        // Front roof-mounted LED lightbar
        b.add_local_box(
            [0.16 * s, 1.86 * s, -0.62 * s],
            [0.25 * s, 1.92 * s, 0.62 * s],
            [0.88, 0.94, 1.00, 1.0],
            [1.00, 1.00, 0.98, 1.0],
        );
        // Monocrystalline solar array over rear canopy
        b.add_local_box(
            [-2.08 * s, 1.87 * s, -0.66 * s],
            [-0.58 * s, 1.91 * s, 0.66 * s],
            [0.08, 0.14, 0.28, 1.0],
            [0.16, 0.26, 0.48, 1.0],
        );
        // Rotating 360° Ecological LiDAR / Multispectral Sensor Puck + Amber Beacon on cab roof
        b.add_local_box(
            [-0.28 * s, 1.86 * s, -0.14 * s],
            [-0.02 * s, 1.95 * s, 0.14 * s],
            [0.20, 0.22, 0.26, 1.0],
            [0.82, 0.86, 0.90, 1.0],
        );
        b.add_local_box(
            [-0.22 * s, 1.94 * s, -0.09 * s],
            [-0.08 * s, 2.00 * s, 0.09 * s],
            [0.95, 0.58, 0.05, 1.0],
            [1.00, 0.84, 0.18, 1.0],
        );
    }

    // 9. 4 Revolved Off-Road Mud-Terrain Tyres (`revolve_profile`), Lug Treads, 6-Spoke Alloy Rims
    // + 5th Tailgate-Mounted Spare Wheel!
    {
        let r_outer = (0.41 * s) as f64;
        let r_inner = (0.22 * s) as f64;
        let r_shoulder = (0.385 * s) as f64;
        let hw = (0.135 * s) as f64;

        let tyre_profile = [
            Point3::new(r_inner, -hw * 0.68, 0.0),
            Point3::new(r_inner + (0.055 * s) as f64, -hw, 0.0),
            Point3::new(r_shoulder, -hw * 0.96, 0.0),
            Point3::new(r_outer, -hw * 0.45, 0.0),
            Point3::new(r_outer * 1.01, 0.0, 0.0),
            Point3::new(r_outer, hw * 0.45, 0.0),
            Point3::new(r_shoulder, hw * 0.96, 0.0),
            Point3::new(r_inner + (0.055 * s) as f64, hw, 0.0),
            Point3::new(r_inner, hw * 0.68, 0.0),
        ];

        let segments = 28usize;
        let req_v = tyre_profile.len() * segments;
        let req_t = (tyre_profile.len() - 1) * segments * 2;
        let mut out_v = vec![Point3::default(); req_v];
        let mut out_t = vec![[0u32; 3]; req_t];

        if let Ok((vc, tc)) = revolve_profile(&tyre_profile, segments, &mut out_v, &mut out_t) {
            for &wx in &wheel_axles {
                for &wz in &[-0.88_f32 * s, 0.88_f32 * s] {
                    let v_start = b.positions.len();
                    let i_start = b.indices.len();
                    let wy = 0.41_f32 * s;
                    let sign_z = if wz >= 0.0 { 1.0_f32 } else { -1.0_f32 };

                    for (vi, p) in out_v.iter().take(vc).enumerate() {
                        let ring_idx = vi % tyre_profile.len();
                        let rx = p.x as f32;
                        let ry = p.z as f32;
                        let rz = (p.y as f32) * sign_z;
                        let vx = wx + rx;
                        let vy = wy + ry;
                        let vz = wz + rz;
                        let is_tread = (3..=5).contains(&ring_idx);
                        let col = if is_tread {
                            [0.10, 0.11, 0.12, 1.0]
                        } else {
                            [0.15, 0.16, 0.18, 1.0]
                        };
                        b.push_vertex([vx, vy, vz], [0.0, 1.0, 0.0], col, [0.5, 0.5]);
                    }
                    for tri in out_t.iter().take(tc) {
                        if sign_z > 0.0 {
                            b.push_tri(
                                v_start as u32 + tri[0],
                                v_start as u32 + tri[1],
                                v_start as u32 + tri[2],
                            );
                        } else {
                            b.push_tri(
                                v_start as u32 + tri[0],
                                v_start as u32 + tri[2],
                                v_start as u32 + tri[1],
                            );
                        }
                    }
                    b.recompute_normals_since(v_start, i_start);

                    // 6-Spoke Silver Metallic Alloy Wheel Rim, Hub Cap & Brake Disc
                    let hub_z = wz + sign_z * (hw as f32 * 0.72);
                    // Inner brake rotor disc
                    b.add_local_box(
                        [wx - 0.17 * s, wy - 0.17 * s, wz - 0.02 * s],
                        [wx + 0.17 * s, wy + 0.17 * s, wz + 0.02 * s],
                        [0.35, 0.37, 0.40, 1.0],
                        [0.52, 0.55, 0.58, 1.0],
                    );
                    // Center locking hub
                    b.add_local_box(
                        [wx - 0.08 * s, wy - 0.08 * s, hub_z - 0.025 * s],
                        [wx + 0.08 * s, wy + 0.08 * s, hub_z + 0.025 * s],
                        [0.22, 0.24, 0.27, 1.0],
                        [0.82, 0.85, 0.90, 1.0],
                    );
                    // 6 oriented radial alloy spokes
                    for sp in 0..6 {
                        let ang = sp as f32 * (std::f32::consts::TAU / 6.0);
                        let (ca, sa) = (ang.cos(), ang.sin());
                        add_oriented_beam(
                            b,
                            [wx + ca * 0.06 * s, wy + sa * 0.06 * s, hub_z],
                            [wx + ca * 0.22 * s, wy + sa * 0.22 * s, hub_z],
                            [0.0, 0.0, 1.0],
                            0.015 * s,
                            0.028 * s,
                            [0.70, 0.73, 0.78, 1.0],
                            [0.92, 0.94, 0.98, 1.0],
                        );
                    }
                }
            }

            // 5th Full-Size Spare Mud-Terrain Tyre Mounted on Rear Tailgate (`x = -2.44 * s, y = 1.24 * s, z = 0.0`)
            let sp_v_start = b.positions.len();
            let sp_i_start = b.indices.len();
            let sp_x = -2.44_f32 * s;
            let sp_y = 1.24_f32 * s;
            for p in out_v.iter().take(vc) {
                // Tyre axis along X (tailgate mount): p.y is axial thickness -> mapped to X
                let vx = sp_x + (p.y as f32) * 0.88;
                let vy = sp_y + (p.z as f32) * 0.88;
                let vz = (p.x as f32) * 0.88;
                b.push_vertex(
                    [vx, vy, vz],
                    [-1.0, 0.0, 0.0],
                    [0.12, 0.13, 0.15, 1.0],
                    [0.5, 0.5],
                );
            }
            for tri in out_t.iter().take(tc) {
                b.push_tri(
                    sp_v_start as u32 + tri[0],
                    sp_v_start as u32 + tri[1],
                    sp_v_start as u32 + tri[2],
                );
            }
            b.recompute_normals_since(sp_v_start, sp_i_start);
            // Spare wheel alloy hub center
            b.add_local_box(
                [sp_x - 0.10 * s, sp_y - 0.15 * s, -0.15 * s],
                [sp_x + 0.04 * s, sp_y + 0.15 * s, 0.15 * s],
                [0.72, 0.75, 0.80, 1.0],
                [0.90, 0.92, 0.96, 1.0],
            );
        }
    }
}

/// Build an anatomically proportioned, articulated 3D Field Ecologist Bushwalker Avatar in 1:1 SI
/// metric dimensions (`1.80 m` standing height at `scale = 1.0`, feet at `y = 0.0`, `+Z` forward).
///
/// Replaces the legacy 9 axis-aligned boxes with smooth swept/lofted/SDF/revolved geometry:
/// - Laced leather hiking boots with treaded soles, rounded toe boxes & canvas snake gaiters
/// - Articulated khaki field trousers (ankle, calf, knee, thigh, pelvis) via `add_bishop_swept_tube`
/// - Sculpted anatomical torso (`add_vertical_loft_along_y`) in eucalypt-green field shirt & hi-vis
///   ochre survey vest with retroreflective silver safety strips, utility belt & collar
/// - Articulated shoulders, upper arms, bent elbows, forearms & hands holding a ruggedized GIS
///   field tablet with an illuminated terrain map screen
/// - Sculpted neck, smooth SDF cranium/jaw head, polarized field sunglasses & a curved wide-brimmed
///   Australian Akubra felt bush hat (`revolve_profile` + pinched crown + leather hatband)
/// - Contoured expedition backpack with rolled canvas swag, shoulder harness straps & RTK-GNSS survey pole.
pub fn build_cad_ecologist_walker_local(b: &mut SubmeshBuilder, scale: f32) {
    let s = scale;
    let sf = scale as f64;

    // Palette
    let boot_dark = [0.22, 0.15, 0.10, 1.0];
    let boot_leather = [0.38, 0.26, 0.17, 1.0];
    let gaiter_col = [0.30, 0.28, 0.22, 1.0];
    let trouser_dark = [0.46, 0.40, 0.28, 1.0];
    let trouser_lit = [0.62, 0.55, 0.40, 1.0];
    let shirt_green_d = [0.18, 0.36, 0.22, 1.0];
    let shirt_green_l = [0.28, 0.50, 0.32, 1.0];
    let vest_ochre_d = [0.86, 0.48, 0.10, 1.0];
    let vest_ochre_l = [0.98, 0.66, 0.16, 1.0];
    let refl_silver = [0.90, 0.92, 0.95, 1.0];
    let skin_dark = [0.72, 0.54, 0.42, 1.0];
    let skin_lit = [0.88, 0.68, 0.54, 1.0];
    let hat_felt_d = [0.32, 0.23, 0.15, 1.0];
    let hat_felt_l = [0.48, 0.36, 0.24, 1.0];
    let pack_olive_d = [0.26, 0.28, 0.20, 1.0];
    let pack_olive_l = [0.40, 0.42, 0.30, 1.0];

    // 1. Laced Leather Hiking Boots, Canvas Snake Gaiters & Articulated Khaki Trousers (Left & Right Legs)
    for &side in &[-1.0_f32, 1.0_f32] {
        // Natural slight contrapposto / walking stride offset along Z
        let stride_z = side * 0.035;
        let hip_x = side * 0.105;
        let knee_x = side * 0.115;
        let ankle_x = side * 0.115;

        // Sculpted boot sole & forefoot (`add_bishop_swept_tube` along foot axis from heel to rounded toe)
        let foot_axis = [
            Point3::new(
                (ankle_x * s) as f64,
                (0.038 * s) as f64,
                ((stride_z - 0.06) * s) as f64,
            ),
            Point3::new(
                (ankle_x * s) as f64,
                (0.042 * s) as f64,
                ((stride_z + 0.04) * s) as f64,
            ),
            Point3::new(
                (ankle_x * s) as f64,
                (0.032 * s) as f64,
                ((stride_z + 0.15) * s) as f64,
            ),
        ];
        let foot_st = [
            ExtrusionStation {
                scale_x: 0.052 * sf,
                scale_y: 0.038 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.056 * sf,
                scale_y: 0.042 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.042 * sf,
                scale_y: 0.028 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(&foot_axis, &foot_st, 10, boot_dark, boot_leather);

        // Lower leg: ankle cuff & tough canvas snake gaiter (`y: 0.04 .. 0.34`)
        let gaiter_axis = [
            Point3::new(
                (ankle_x * s) as f64,
                (0.04 * s) as f64,
                (stride_z * s) as f64,
            ),
            Point3::new(
                (ankle_x * s) as f64,
                (0.20 * s) as f64,
                ((stride_z + 0.01) * s) as f64,
            ),
            Point3::new(
                (knee_x * s) as f64,
                (0.36 * s) as f64,
                ((stride_z + 0.025) * s) as f64,
            ),
        ];
        let gaiter_st = [
            ExtrusionStation {
                scale_x: 0.054 * sf,
                scale_y: 0.058 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.062 * sf,
                scale_y: 0.066 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.066 * sf,
                scale_y: 0.068 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(&gaiter_axis, &gaiter_st, 12, boot_leather, gaiter_col);

        // Upper leg: knee -> thigh -> hip pelvis (`y: 0.34 .. 0.86`)
        let thigh_axis = [
            Point3::new(
                (knee_x * s) as f64,
                (0.35 * s) as f64,
                ((stride_z + 0.025) * s) as f64,
            ),
            Point3::new(
                (knee_x * s) as f64,
                (0.48 * s) as f64,
                ((stride_z + 0.03) * s) as f64,
            ),
            Point3::new(
                (hip_x * s) as f64,
                (0.68 * s) as f64,
                ((stride_z * 0.5) * s) as f64,
            ),
            Point3::new((hip_x * 0.92 * s) as f64, (0.86 * s) as f64, 0.0),
        ];
        let thigh_st = [
            ExtrusionStation {
                scale_x: 0.066 * sf,
                scale_y: 0.068 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.074 * sf,
                scale_y: 0.078 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.088 * sf,
                scale_y: 0.092 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.095 * sf,
                scale_y: 0.098 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(&thigh_axis, &thigh_st, 12, trouser_dark, trouser_lit);
    }

    // 2. Sculpted Pelvis, Waist Belt & Anatomical Torso via Octagonal Convex Loft (`add_vertical_loft_along_y`)
    {
        let oct_ring = |rx: f32, rz: f32, z_center: f32, y_level: f32| -> Vec<[f32; 3]> {
            let mut ring = Vec::with_capacity(12);
            for k in 0..12 {
                // Counterclockwise in (x, z) plane so area > 0
                let ang = std::f32::consts::TAU * (k as f32) / 12.0;
                ring.push([
                    rx * ang.cos(),
                    z_center + rz * ang.sin(),
                    y_level,
                ]);
            }
            ring
        };

        // Lower pelvis & khaki hip block (`y: 0.78*s .. 0.94*s`)
        let pelvis_sections = vec![
            oct_ring(0.185 * s, 0.115 * s, 0.0, 0.78 * s),
            oct_ring(0.195 * s, 0.122 * s, 0.0, 0.86 * s),
            oct_ring(0.178 * s, 0.112 * s, 0.0, 0.94 * s),
        ];
        add_vertical_loft_along_y(b, &pelvis_sections, trouser_dark, trouser_lit);

        // Leather field utility belt & GPS pouch at `y: 0.92*s .. 0.97*s`
        let belt_sections = vec![
            oct_ring(0.186 * s, 0.120 * s, 0.0, 0.92 * s),
            oct_ring(0.186 * s, 0.120 * s, 0.0, 0.97 * s),
        ];
        add_vertical_loft_along_y(b, &belt_sections, boot_dark, boot_leather);

        // Upper Torso & Hi-Vis Ochre Ecological Survey Vest (`y: 0.96*s .. 1.38*s`)
        let torso_sections = vec![
            oct_ring(0.182 * s, 0.116 * s, 0.0, 0.96 * s),
            oct_ring(0.205 * s, 0.132 * s, 0.01 * s, 1.12 * s),
            oct_ring(0.228 * s, 0.142 * s, 0.015 * s, 1.28 * s),
            oct_ring(0.215 * s, 0.125 * s, 0.005 * s, 1.37 * s),
            oct_ring(0.120 * s, 0.095 * s, 0.0, 1.42 * s),
        ];
        add_vertical_loft_along_y(b, &torso_sections, vest_ochre_d, vest_ochre_l);

        // Retroreflective silver safety bands around waist and over shoulders
        let refl_waist = vec![
            oct_ring(0.198 * s, 0.128 * s, 0.006 * s, 1.04 * s),
            oct_ring(0.204 * s, 0.132 * s, 0.008 * s, 1.08 * s),
        ];
        add_vertical_loft_along_y(b, &refl_waist, refl_silver, refl_silver);

        // Eucalypt-green bush shirt collar at base of neck (`y: 1.38*s .. 1.45*s`)
        let collar_sections = vec![
            oct_ring(0.095 * s, 0.088 * s, 0.005 * s, 1.38 * s),
            oct_ring(0.082 * s, 0.078 * s, 0.008 * s, 1.45 * s),
        ];
        add_vertical_loft_along_y(b, &collar_sections, shirt_green_d, shirt_green_l);
    }

    // 3. Articulated Shoulders, Upper Arms, Bent Elbows, Forearms & Hands Holding Ruggedized GIS Field Tablet
    for &side in &[-1.0_f32, 1.0_f32] {
        // Eucalypt-green sleeved upper arm from deltoid shoulder `(±0.22, 1.33, 0.0)` to bent elbow `(±0.25, 1.08, 0.05)`
        let upper_arm = [
            Point3::new((side * 0.21 * s) as f64, (1.34 * s) as f64, 0.0),
            Point3::new(
                (side * 0.245 * s) as f64,
                (1.21 * s) as f64,
                (0.02 * s) as f64,
            ),
            Point3::new(
                (side * 0.25 * s) as f64,
                (1.08 * s) as f64,
                (0.05 * s) as f64,
            ),
        ];
        let upper_arm_st = [
            ExtrusionStation {
                scale_x: 0.062 * sf,
                scale_y: 0.062 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.054 * sf,
                scale_y: 0.054 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.048 * sf,
                scale_y: 0.048 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(&upper_arm, &upper_arm_st, 10, shirt_green_d, shirt_green_l);

        // Forearm & hand reaching forward to hold the field tablet at `(±0.12, 1.13, 0.24)`
        let forearm = [
            Point3::new(
                (side * 0.25 * s) as f64,
                (1.08 * s) as f64,
                (0.05 * s) as f64,
            ),
            Point3::new(
                (side * 0.19 * s) as f64,
                (1.10 * s) as f64,
                (0.16 * s) as f64,
            ),
            Point3::new(
                (side * 0.125 * s) as f64,
                (1.13 * s) as f64,
                (0.24 * s) as f64,
            ),
        ];
        let forearm_st = [
            ExtrusionStation {
                scale_x: 0.045 * sf,
                scale_y: 0.045 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.038 * sf,
                scale_y: 0.038 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.034 * sf,
                scale_y: 0.026 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(&forearm, &forearm_st, 10, skin_dark, skin_lit);
    }

    // Ruggedized Ecological GIS Field Tablet held in hands at `z = +0.25 * s, y = 1.14 * s`
    add_oriented_beam(
        b,
        [-0.12 * s, 1.13 * s, 0.25 * s],
        [0.12 * s, 1.13 * s, 0.25 * s],
        [0.0, 0.75, -0.66],
        0.085 * s,
        0.012 * s,
        [0.14, 0.16, 0.18, 1.0],
        [0.18, 0.82, 0.72, 1.0], // Illuminated cyan/emerald GIS display screen on top face
    );

    // 4. Sculpted Neck, Smooth SDF Cranium/Jaw Head, Field Sunglasses & Curved Australian Akubra Bush Hat
    {
        // Smooth SDF head + neck + nose bridge (`y: 1.40 .. 1.69`)
        append_local_sdf_surface(
            b,
            [-0.12, 1.38, -0.13],
            [0.12, 1.70, 0.15],
            (14, 16, 14),
            s,
            skin_dark,
            skin_lit,
            |p| {
                let d_neck = sdf_capsule(
                    p,
                    Point3::new(0.0, 1.39, 0.0),
                    Point3::new(0.0, 1.49, 0.01),
                    0.054,
                );
                let d_cranium = sdf_sphere(p, Point3::new(0.0, 1.58, 0.0), 0.088);
                let d_jaw = sdf_rounded_box(
                    p,
                    Point3::new(0.0, 1.52, 0.025),
                    Point3::new(0.056, 0.048, 0.062),
                    0.028,
                );
                let d_nose = sdf_sphere(p, Point3::new(0.0, 1.56, 0.095), 0.022);
                let h0 = sdf_smooth_union(d_neck, d_cranium, 0.035);
                let h1 = sdf_smooth_union(h0, d_jaw, 0.035);
                sdf_smooth_union(h1, d_nose, 0.018)
            },
        );

        // Wrap-around polarized dark field sunglasses across eyes (`y = 1.58 * s, z = +0.082 * s`)
        let glasses_axis = [
            Point3::new((-0.072 * s) as f64, (1.585 * s) as f64, (0.045 * s) as f64),
            Point3::new((-0.042 * s) as f64, (1.585 * s) as f64, (0.088 * s) as f64),
            Point3::new(0.0, (1.588 * s) as f64, (0.096 * s) as f64),
            Point3::new((0.042 * s) as f64, (1.585 * s) as f64, (0.088 * s) as f64),
            Point3::new((0.072 * s) as f64, (1.585 * s) as f64, (0.045 * s) as f64),
        ];
        let glasses_st = vec![
            ExtrusionStation {
                scale_x: 0.016 * sf,
                scale_y: 0.012 * sf,
                twist_rad: 0.0,
            };
            glasses_axis.len()
        ];
        b.add_bishop_swept_tube(
            &glasses_axis,
            &glasses_st,
            6,
            [0.08, 0.10, 0.14, 1.0],
            [0.18, 0.26, 0.36, 1.0],
        );

        // Revolved Wide-Brimmed Australian Akubra Felt Bush Hat (`revolve_profile` around vertical Y axis)
        // Profile in (radius, height_y, 0.0) reaching 1.80 * s total standing height
        let hat_profile = [
            Point3::new(0.255 * sf, 1.635 * sf, 0.0), // Outer upturned brim edge
            Point3::new(0.185 * sf, 1.622 * sf, 0.0), // Mid brim dip
            Point3::new(0.112 * sf, 1.632 * sf, 0.0), // Crown base / hatband
            Point3::new(0.106 * sf, 1.658 * sf, 0.0), // Hatband top
            Point3::new(0.096 * sf, 1.755 * sf, 0.0), // Tapered cattleman crown rim
            Point3::new(0.052 * sf, 1.795 * sf, 0.0), // Crown pinch
            Point3::new(0.002 * sf, 1.800 * sf, 0.0), // Crown center apex (1.80 m height)
        ];
        let hat_segs = 24usize;
        let req_v = hat_profile.len() * hat_segs;
        let req_t = (hat_profile.len() - 1) * hat_segs * 2;
        let mut out_v = vec![Point3::default(); req_v];
        let mut out_t = vec![[0u32; 3]; req_t];
        if let Ok((vc, tc)) = revolve_profile(&hat_profile, hat_segs, &mut out_v, &mut out_t) {
            let v_start = b.positions.len();
            let i_start = b.indices.len();
            for (vi, p) in out_v.iter().take(vc).enumerate() {
                let ring_idx = vi % hat_profile.len();
                // Slight front-to-back oval elongation (1.12 along Z, 0.96 along X) + Aussie side-brim curl
                let vx = (p.x as f32) * 0.96;
                let vz = (p.z as f32) * 1.12;
                let side_curl = if ring_idx <= 1 {
                    ((vx / (0.25 * s).max(1e-4)).powi(2)) * 0.022 * s
                } else {
                    0.0
                };
                let vy = (p.y as f32) + side_curl;
                let col = if ring_idx == 2 || ring_idx == 3 {
                    // Braided kangaroo-leather hatband
                    [0.24, 0.14, 0.08, 1.0]
                } else {
                    lerp_color(
                        hat_felt_d,
                        hat_felt_l,
                        (ring_idx as f32 / 6.0).clamp(0.0, 1.0),
                    )
                };
                b.push_vertex([vx, vy, vz], [0.0, 1.0, 0.0], col, [0.5, 0.5]);
            }
            for tri in out_t.iter().take(tc) {
                b.push_tri(
                    v_start as u32 + tri[0],
                    v_start as u32 + tri[2],
                    v_start as u32 + tri[1],
                );
            }
            b.recompute_normals_since(v_start, i_start);
        }
    }

    // 5. Contoured Botanical Expedition Backpack, Rolled Swag & RTK-GNSS Survey Rover Antenna Mast
    {
        // Ergonomic rounded backpack body (`add_vertical_loft_along_y`, `z: -0.29*s .. -0.11*s`, `y: 0.86*s .. 1.34*s`)
        let pack_ring = |rx: f32, rz: f32, zc: f32, yl: f32| -> Vec<[f32; 3]> {
            let mut ring = Vec::with_capacity(8);
            for k in 0..8 {
                let ang = std::f32::consts::TAU * (k as f32) / 8.0;
                ring.push([rx * ang.cos(), zc + rz * ang.sin(), yl]);
            }
            ring
        };
        let pack_sections = vec![
            pack_ring(0.16 * s, 0.085 * s, -0.19 * s, 0.86 * s),
            pack_ring(0.18 * s, 0.095 * s, -0.20 * s, 1.04 * s),
            pack_ring(0.17 * s, 0.092 * s, -0.19 * s, 1.24 * s),
            pack_ring(0.14 * s, 0.075 * s, -0.17 * s, 1.34 * s),
        ];
        add_vertical_loft_along_y(b, &pack_sections, pack_olive_d, pack_olive_l);

        // Rolled canvas bedroll/swag strapped across top of backpack (`y = 1.36 * s, z = -0.18 * s`)
        let swag_axis = [
            Point3::new((-0.17 * s) as f64, (1.36 * s) as f64, (-0.18 * s) as f64),
            Point3::new((0.17 * s) as f64, (1.36 * s) as f64, (-0.18 * s) as f64),
        ];
        let swag_st = [
            ExtrusionStation {
                scale_x: 0.048 * sf,
                scale_y: 0.048 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.048 * sf,
                scale_y: 0.048 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(
            &swag_axis,
            &swag_st,
            10,
            [0.54, 0.44, 0.30, 1.0],
            [0.72, 0.60, 0.42, 1.0],
        );

        // Vertical RTK-GNSS survey pole + yellow/white high-precision GNSS dome rising from right side of backpack
        let gnss_axis = [
            Point3::new((0.12 * s) as f64, (1.10 * s) as f64, (-0.22 * s) as f64),
            Point3::new((0.12 * s) as f64, (1.84 * s) as f64, (-0.22 * s) as f64),
        ];
        let gnss_st = [
            ExtrusionStation {
                scale_x: 0.012 * sf,
                scale_y: 0.012 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.010 * sf,
                scale_y: 0.010 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(
            &gnss_axis,
            &gnss_st,
            6,
            [0.18, 0.20, 0.22, 1.0],
            [0.92, 0.42, 0.10, 1.0],
        );
        // GNSS receiver saucer dome at `y = 1.85 * s`
        let dome_axis = [
            Point3::new((0.12 * s) as f64, (1.83 * s) as f64, (-0.22 * s) as f64),
            Point3::new((0.12 * s) as f64, (1.86 * s) as f64, (-0.22 * s) as f64),
            Point3::new((0.12 * s) as f64, (1.885 * s) as f64, (-0.22 * s) as f64),
        ];
        let dome_st = [
            ExtrusionStation {
                scale_x: 0.045 * sf,
                scale_y: 0.045 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.056 * sf,
                scale_y: 0.056 * sf,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: 0.022 * sf,
                scale_y: 0.022 * sf,
                twist_rad: 0.0,
            },
        ];
        b.add_bishop_swept_tube(
            &dome_axis,
            &dome_st,
            10,
            [0.92, 0.78, 0.14, 1.0],
            [0.98, 0.98, 0.96, 1.0],
        );
    }
}
