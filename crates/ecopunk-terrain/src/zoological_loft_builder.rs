//! High-Detail Anatomical Loft & Countershaded SDF Fauna Builder (`zoological_loft_builder.rs`).
//!
//! Synthesizes anatomically proportioned, multi-shaded Australian vertebrate models for
//! sector scenes (`SubmeshBuilder`), replacing primitive monochrome cylinder/blob approximations
//! with:
//! - Higher-resolution C¹ smooth-union SDF bodies with analytical central-difference normals
//!   and biologically accurate **dorsal–flank–ventral countershading** (dark dorsal pelage/plumage,
//!   warm mid-flank coat, pale cream ventral throat/chest/underbelly, dark keratinous muzzle/claws).
//! - Multi-station elliptical & anatomical cross-section Bishop-frame loft sweeps (`AnatomicalLoftStation`)
//!   with smooth radial + longitudinal normal calculation and per-ring countershading.
//! - True 3D cambered airfoil wings with slotted primary wingtip fingers, secondary trailing feathers,
//!   coverts, and 3D rectrix tail fans.
//! - Cupped marsupial pinnae ears with concave pale inner-ear shading, glossy eyes with specular
//!   highlights, procumbent/hooked beaks, webbed monotreme paddles, and spiny/soft teleost fins.

use qualia_core_db::specialized_libs::computational_geometry::sdf_ops::{
    sdf_capsule, sdf_rounded_box, sdf_smooth_union, sdf_sphere,
};
use qualia_core_db::specialized_libs::computational_geometry::{marching_cubes, Point3};

use crate::scene_builder_10d::{
    sample_sector_elevation, SubmeshBuilder, SECTOR_METRIC_SCALE,
};

mod swamp_wallaby_review;
pub use swamp_wallaby_review::build_swamp_wallaby_review_specimen;

/// Single cross-section station along a 3D anatomical loft curve in local metric coordinates.
#[derive(Clone, Copy, Debug)]
pub struct AnatomicalLoftStation {
    /// Local 3D station center `[x, y, z]` in meters (+Z forward, +Y up, +X right).
    pub center: [f32; 3],
    /// Lateral half-width (meters).
    pub rx: f32,
    /// Dorsoventral half-height (meters).
    pub ry: f32,
    /// Dorsal color (top of cross-section).
    pub dorsal_color: [f32; 4],
    /// Mid-flank color (sides of cross-section).
    pub flank_color: [f32; 4],
    /// Ventral color (underside of cross-section).
    pub ventral_color: [f32; 4],
}

#[inline]
fn lerp_color(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    let u = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * u,
        a[1] + (b[1] - a[1]) * u,
        a[2] + (b[2] - a[2]) * u,
        a[3] + (b[3] - a[3]) * u,
    ]
}

#[inline]
fn countershade_color(
    dorsal: [f32; 4],
    flank: [f32; 4],
    ventral: [f32; 4],
    dorsoventral_ny: f32,
) -> [f32; 4] {
    if dorsoventral_ny >= 0.0 {
        lerp_color(flank, dorsal, dorsoventral_ny.clamp(0.0, 1.0).powf(0.75))
    } else {
        lerp_color(flank, ventral, (-dorsoventral_ny).clamp(0.0, 1.0).powf(0.70))
    }
}

#[inline]
fn transform_local_point(pos: [f32; 3], yaw: f32, scale: f32, lp: [f32; 3]) -> [f32; 3] {
    let cy = yaw.cos();
    let sy = yaw.sin();
    [
        pos[0] + (lp[0] * cy - lp[2] * sy) * scale,
        pos[1] + lp[1] * scale,
        pos[2] + (lp[0] * sy + lp[2] * cy) * scale,
    ]
}

#[inline]
fn transform_local_normal(yaw: f32, ln: [f32; 3]) -> [f32; 3] {
    let cy = yaw.cos();
    let sy = yaw.sin();
    let rx = ln[0] * cy - ln[2] * sy;
    let ry = ln[1];
    let rz = ln[0] * sy + ln[2] * cy;
    let len = (rx * rx + ry * ry + rz * rz).sqrt().max(1e-6);
    [rx / len, ry / len, rz / len]
}

/// Evaluates an analytical composite SDF on a local 3D grid, extracts a closed 2-manifold
/// triangle mesh via `marching_cubes`, welds shared vertices, computes smooth analytical
/// central-difference normals, and shades each vertex with an anatomical countershading closure.
#[allow(clippy::too_many_arguments)]
fn append_countershaded_sdf_body<F, C>(
    b: &mut SubmeshBuilder,
    bmin: [f32; 3],
    bmax: [f32; 3],
    res: (usize, usize, usize),
    pos: [f32; 3],
    yaw: f32,
    scale: f32,
    sdf: F,
    color_fn: C,
) where
    F: Fn(Point3) -> f64,
    C: Fn([f32; 3], [f32; 3]) -> [f32; 4],
{
    let (nx, ny, nz) = res;
    if nx < 3 || ny < 3 || nz < 3 {
        return;
    }
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

    // Current Core marching_cubes requires the six-tetrahedron cell bound.
    let max_verts = (nx - 1) * (ny - 1) * (nz - 1) * 30;
    let max_tris = (nx - 1) * (ny - 1) * (nz - 1) * 10;
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
        let quant = 2500.0_f64;
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
            let glen = (gx * gx + gy * gy + gz * gz).sqrt().max(1e-6);
            let ln = [(gx / glen) as f32, (gy / glen) as f32, (gz / glen) as f32];
            let lp = [uv.x as f32, uv.y as f32, uv.z as f32];

            let wp = transform_local_point(pos, yaw, scale, lp);
            let wn = transform_local_normal(yaw, ln);
            let col = color_fn(lp, ln);

            let u = ((lp[0] - bmin[0]) / (bmax[0] - bmin[0]).max(0.01)).clamp(0.0, 1.0);
            let v_coord = ((lp[2] - bmin[2]) / (bmax[2] - bmin[2]).max(0.01)).clamp(0.0, 1.0);
            b.push_vertex(wp, wn, col, [u, v_coord]);
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

/// Sweeps a multi-station anatomical loft tube with elliptical cross-sections, smooth normals,
/// end caps, and dorsal–flank–ventral countershading.
fn append_anatomical_loft_sweep(
    b: &mut SubmeshBuilder,
    stations: &[AnatomicalLoftStation],
    radial_segs: usize,
    pos: [f32; 3],
    yaw: f32,
    scale: f32,
    cap_ends: bool,
) {
    if stations.len() < 2 {
        return;
    }
    let m_sec = radial_segs.max(6);
    let n_st = stations.len();
    let ring_stride = (m_sec + 1) as u32;
    let base_idx = b.positions.len() as u32;

    for (si, st) in stations.iter().enumerate() {
        let prev_c = if si > 0 {
            stations[si - 1].center
        } else {
            stations[0].center
        };
        let next_c = if si + 1 < n_st {
            stations[si + 1].center
        } else {
            stations[n_st - 1].center
        };
        let mut tx = next_c[0] - prev_c[0];
        let mut ty = next_c[1] - prev_c[1];
        let mut tz = next_c[2] - prev_c[2];
        let tlen = (tx * tx + ty * ty + tz * tz).sqrt().max(1e-6);
        tx /= tlen;
        ty /= tlen;
        tz /= tlen;

        // Reference up vector to construct local right (e0) and dorsoventral up (e1)
        let ref_up = if ty.abs() < 0.92 {
            [0.0_f32, 1.0_f32, 0.0_f32]
        } else {
            [0.0_f32, 0.0_f32, 1.0_f32]
        };
        // e0 = ref_up x tangent
        let mut e0x = ref_up[1] * tz - ref_up[2] * ty;
        let mut e0y = ref_up[2] * tx - ref_up[0] * tz;
        let mut e0z = ref_up[0] * ty - ref_up[1] * tx;
        let e0len = (e0x * e0x + e0y * e0y + e0z * e0z).sqrt().max(1e-6);
        e0x /= e0len;
        e0y /= e0len;
        e0z /= e0len;

        // e1 = tangent x e0
        let e1x = ty * e0z - tz * e0y;
        let e1y = tz * e0x - tx * e0z;
        let e1z = tx * e0y - ty * e0x;

        let v_param = si as f32 / (n_st - 1) as f32;
        for j in 0..=m_sec {
            let u_param = j as f32 / m_sec as f32;
            let theta = u_param * core::f32::consts::TAU;
            let ct = theta.cos();
            let st_sin = theta.sin();

            let lx = st.center[0] + e0x * (st.rx * ct) + e1x * (st.ry * st_sin);
            let ly = st.center[1] + e0y * (st.rx * ct) + e1y * (st.ry * st_sin);
            let lz = st.center[2] + e0z * (st.rx * ct) + e1z * (st.ry * st_sin);

            let nx_local = e0x * (ct / st.rx.max(1e-4)) + e1x * (st_sin / st.ry.max(1e-4));
            let ny_local = e0y * (ct / st.rx.max(1e-4)) + e1y * (st_sin / st.ry.max(1e-4));
            let nz_local = e0z * (ct / st.rx.max(1e-4)) + e1z * (st_sin / st.ry.max(1e-4));

            let wp = transform_local_point(pos, yaw, scale, [lx, ly, lz]);
            let wn = transform_local_normal(yaw, [nx_local, ny_local, nz_local]);
            let col = countershade_color(st.dorsal_color, st.flank_color, st.ventral_color, st_sin);

            b.push_vertex(wp, wn, col, [u_param, v_param]);
        }
    }

    for si in 0..(n_st - 1) as u32 {
        for j in 0..m_sec as u32 {
            let i00 = base_idx + si * ring_stride + j;
            let i10 = i00 + ring_stride;
            let i01 = i00 + 1;
            let i11 = i10 + 1;
            b.push_tri(i00, i10, i11);
            b.push_tri(i00, i11, i01);
        }
    }

    if cap_ends {
        let first = &stations[0];
        let c0_wp = transform_local_point(pos, yaw, scale, first.center);
        let c0_wn = transform_local_normal(yaw, [0.0, 0.0, -1.0]);
        let c0_idx = b.push_vertex(c0_wp, c0_wn, first.flank_color, [0.5, 0.0]);
        for j in 0..m_sec as u32 {
            b.push_tri(c0_idx, base_idx + j + 1, base_idx + j);
        }

        let last = &stations[n_st - 1];
        let last_ring_base = base_idx + ((n_st - 1) as u32) * ring_stride;
        let c1_wp = transform_local_point(pos, yaw, scale, last.center);
        let c1_wn = transform_local_normal(yaw, [0.0, 0.0, 1.0]);
        let c1_idx = b.push_vertex(c1_wp, c1_wn, last.dorsal_color, [0.5, 1.0]);
        for j in 0..m_sec as u32 {
            b.push_tri(c1_idx, last_ring_base + j, last_ring_base + j + 1);
        }
    }
}

/// Appends a smooth local ellipsoid (used for glossy eyes, rhinarium pads, calcaneal hocks, claws).
#[allow(clippy::too_many_arguments)]
fn append_local_ellipsoid(
    b: &mut SubmeshBuilder,
    center: [f32; 3],
    radii: [f32; 3],
    u_segs: usize,
    v_segs: usize,
    col: [f32; 4],
    pos: [f32; 3],
    yaw: f32,
    scale: f32,
) {
    let u_n = u_segs.max(6);
    let v_n = v_segs.max(4);
    let base_idx = b.positions.len() as u32;

    for v in 0..=v_n {
        let phi = (v as f32 / v_n as f32) * core::f32::consts::PI;
        let sp = phi.sin();
        let cp = phi.cos();
        for u in 0..=u_n {
            let theta = (u as f32 / u_n as f32) * core::f32::consts::TAU;
            let ct = theta.cos();
            let st = theta.sin();

            let nx = sp * ct;
            let ny = cp;
            let nz = sp * st;

            let lp = [
                center[0] + radii[0] * nx,
                center[1] + radii[1] * ny,
                center[2] + radii[2] * nz,
            ];
            let ln = [
                nx / radii[0].max(1e-4),
                ny / radii[1].max(1e-4),
                nz / radii[2].max(1e-4),
            ];
            let wp = transform_local_point(pos, yaw, scale, lp);
            let wn = transform_local_normal(yaw, ln);
            b.push_vertex(
                wp,
                wn,
                col,
                [u as f32 / u_n as f32, v as f32 / v_n as f32],
            );
        }
    }

    let stride = (u_n + 1) as u32;
    for v in 0..v_n as u32 {
        for u in 0..u_n as u32 {
            let i00 = base_idx + v * stride + u;
            let i10 = i00 + stride;
            let i01 = i00 + 1;
            let i11 = i10 + 1;
            b.push_tri(i00, i10, i11);
            b.push_tri(i00, i11, i01);
        }
    }
}

/// Appends a 3D cupped mammalian ear pinna with furred outer shell and pale concave inner cavity.
#[allow(clippy::too_many_arguments)]
fn append_cupped_ear_pinna(
    b: &mut SubmeshBuilder,
    base_local: [f32; 3],
    tip_local: [f32; 3],
    width: f32,
    depth: f32,
    outer_col: [f32; 4],
    inner_col: [f32; 4],
    tip_col: [f32; 4],
    pos: [f32; 3],
    yaw: f32,
    scale: f32,
) {
    let dir = [
        tip_local[0] - base_local[0],
        tip_local[1] - base_local[1],
        tip_local[2] - base_local[2],
    ];
    let stations = [
        AnatomicalLoftStation {
            center: base_local,
            rx: width * 0.65,
            ry: depth * 0.65,
            dorsal_color: outer_col,
            flank_color: outer_col,
            ventral_color: inner_col,
        },
        AnatomicalLoftStation {
            center: [
                base_local[0] + dir[0] * 0.38,
                base_local[1] + dir[1] * 0.38,
                base_local[2] + dir[2] * 0.38 - depth * 0.25,
            ],
            rx: width,
            ry: depth * 0.55,
            dorsal_color: outer_col,
            flank_color: outer_col,
            ventral_color: inner_col,
        },
        AnatomicalLoftStation {
            center: [
                base_local[0] + dir[0] * 0.75,
                base_local[1] + dir[1] * 0.75,
                base_local[2] + dir[2] * 0.75 - depth * 0.12,
            ],
            rx: width * 0.68,
            ry: depth * 0.38,
            dorsal_color: lerp_color(outer_col, tip_col, 0.5),
            flank_color: outer_col,
            ventral_color: inner_col,
        },
        AnatomicalLoftStation {
            center: tip_local,
            rx: width * 0.18,
            ry: depth * 0.16,
            dorsal_color: tip_col,
            flank_color: tip_col,
            ventral_color: tip_col,
        },
    ];
    append_anatomical_loft_sweep(b, &stations, 10, pos, yaw, scale, true);
}

/// Appends a 3D cambered feather / fin blade with real thickness (both dorsal and ventral surfaces).
#[allow(clippy::too_many_arguments)]
fn append_cambered_feather_blade(
    b: &mut SubmeshBuilder,
    root: [f32; 3],
    tip: [f32; 3],
    chord_vec: [f32; 3],
    thickness: f32,
    dorsal_col: [f32; 4],
    ventral_col: [f32; 4],
    tip_col: [f32; 4],
    pos: [f32; 3],
    yaw: f32,
    scale: f32,
) {
    let span = [tip[0] - root[0], tip[1] - root[1], tip[2] - root[2]];
    let stations = [
        AnatomicalLoftStation {
            center: root,
            rx: (chord_vec[0].hypot(chord_vec[2])).max(0.008) * 0.55,
            ry: thickness * 0.65,
            dorsal_color: dorsal_col,
            flank_color: dorsal_col,
            ventral_color: ventral_col,
        },
        AnatomicalLoftStation {
            center: [
                root[0] + span[0] * 0.45 + chord_vec[0] * 0.15,
                root[1] + span[1] * 0.45 + thickness * 0.4,
                root[2] + span[2] * 0.45 + chord_vec[2] * 0.15,
            ],
            rx: (chord_vec[0].hypot(chord_vec[2])).max(0.008) * 0.65,
            ry: thickness * 0.50,
            dorsal_color: dorsal_col,
            flank_color: lerp_color(dorsal_col, tip_col, 0.35),
            ventral_color: ventral_col,
        },
        AnatomicalLoftStation {
            center: [
                root[0] + span[0] * 0.82 + chord_vec[0] * 0.08,
                root[1] + span[1] * 0.82 + thickness * 0.2,
                root[2] + span[2] * 0.82 + chord_vec[2] * 0.08,
            ],
            rx: (chord_vec[0].hypot(chord_vec[2])).max(0.008) * 0.42,
            ry: thickness * 0.32,
            dorsal_color: lerp_color(dorsal_col, tip_col, 0.70),
            flank_color: tip_col,
            ventral_color: lerp_color(ventral_col, tip_col, 0.60),
        },
        AnatomicalLoftStation {
            center: tip,
            rx: (chord_vec[0].hypot(chord_vec[2])).max(0.004) * 0.14,
            ry: thickness * 0.16,
            dorsal_color: tip_col,
            flank_color: tip_col,
            ventral_color: tip_col,
        },
    ];
    append_anatomical_loft_sweep(b, &stations, 8, pos, yaw, scale, true);
}

/// Builds a high-detail, anatomically proportioned, countershaded Australian fauna specimen
/// directly into `b_fauna` at world position `pos`, orientation `yaw`, and metric scale `scale`.
pub fn add_lofted_fauna_specimen(
    b_fauna: &mut SubmeshBuilder,
    pos: [f32; 3],
    yaw: f32,
    scale: f32,
    archetype: &str,
    coat_dark: [f32; 4],
    coat_lit: [f32; 4],
) {
    let ventral_cream = [
        (coat_lit[0] * 0.55 + 0.42).clamp(0.0, 0.96),
        (coat_lit[1] * 0.55 + 0.40).clamp(0.0, 0.94),
        (coat_lit[2] * 0.52 + 0.36).clamp(0.0, 0.90),
        1.0,
    ];
    let charcoal_tip = [0.14, 0.12, 0.12, 1.0];
    let obsidian_eye = [0.06, 0.06, 0.08, 1.0];
    let inner_ear_pink = [0.84, 0.72, 0.68, 1.0];

    let is_cockatoo = archetype == "cockatoo"
        || archetype == "cockatoo_parrot"
        || (archetype == "raptor" && coat_dark[0] > 0.75 && coat_lit[1] > 0.85);

    if archetype == "macropod" || archetype == "macropod_saltatorial" || archetype == "swamp_wallaby_review_base" {
        // ── 1. Eastern Grey Kangaroo (*Macropus giganteus*) Sculpted Torso, Haunches, Neck & Head ──
        let bmin = [-0.34_f32, 0.22_f32, -0.44_f32];
        let bmax = [0.34_f32, 1.48_f32, 0.48_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            if archetype == "swamp_wallaby_review_base" { (40, 64, 40) } else { (24, 28, 24) },
            pos,
            yaw,
            scale,
            |p| {
                // Pelvis & muscular saltatorial haunches
                let d_pelvis = sdf_sphere(p, Point3::new(0.0, 0.50, -0.10), 0.23);
                let d_thigh_l = sdf_capsule(
                    p,
                    Point3::new(-0.14, 0.52, -0.09),
                    Point3::new(-0.18, 0.37, 0.10),
                    0.125,
                );
                let d_thigh_r = sdf_capsule(
                    p,
                    Point3::new(0.14, 0.52, -0.09),
                    Point3::new(0.18, 0.37, 0.10),
                    0.125,
                );
                // Deep abdomen & tapered thorax
                let d_abdomen = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.52, -0.06),
                    Point3::new(0.0, 0.82, 0.07),
                    0.185,
                );
                let d_chest = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.80, 0.07),
                    Point3::new(0.0, 1.08, 0.16),
                    0.142,
                );
                // Muscular tapered neck
                let d_neck = sdf_capsule(
                    p,
                    Point3::new(0.0, 1.06, 0.16),
                    Point3::new(0.0, 1.29, 0.24),
                    0.090,
                );
                // Cranium, cheek masseters & tapered diprotodont muzzle
                let d_cranium = sdf_sphere(p, Point3::new(0.0, 1.31, 0.26), 0.082);
                let d_muzzle = sdf_capsule(
                    p,
                    Point3::new(0.0, 1.30, 0.27),
                    Point3::new(0.0, 1.27, 0.43),
                    0.052,
                );
                let d_jaw = sdf_capsule(
                    p,
                    Point3::new(0.0, 1.26, 0.25),
                    Point3::new(0.0, 1.24, 0.39),
                    0.040,
                );

                let d0 = sdf_smooth_union(d_pelvis, d_thigh_l, 0.08);
                let d1 = sdf_smooth_union(d0, d_thigh_r, 0.08);
                let d2 = sdf_smooth_union(d1, d_abdomen, 0.11);
                let d3 = sdf_smooth_union(d2, d_chest, 0.09);
                let d4 = sdf_smooth_union(d3, d_neck, 0.075);
                let d5 = sdf_smooth_union(d4, d_cranium, 0.055);
                let d6 = sdf_smooth_union(d5, d_muzzle, 0.045);
                sdf_smooth_union(d6, d_jaw, 0.035)
            },
            |lp, ln| {
                // Dark charcoal muzzle & rhinarium tip
                if lp[2] > 0.39 && lp[1] > 1.20 {
                    let muzzle_t = ((lp[2] - 0.39) / 0.05).clamp(0.0, 1.0);
                    return lerp_color(coat_dark, charcoal_tip, muzzle_t);
                }
                // Pale cream ventral chest, throat & pouch belly
                let is_ventral_front = lp[2] > 0.04 && lp[1] > 0.42 && lp[1] < 1.22 && ln[2] > 0.22;
                let dv = if is_ventral_front {
                    (ln[1] - 0.55 * ln[2]).clamp(-1.0, 1.0)
                } else {
                    ln[1]
                };
                countershade_color(coat_dark, coat_lit, ventral_cream, dv)
            },
        );

        // ── 2. Glossy Eyes & Dark Rhinarium Nose Pad ──
        append_local_ellipsoid(
            b_fauna,
            [0.0, 1.28, 0.44],
            [0.026, 0.022, 0.020],
            8,
            6,
            charcoal_tip,
            pos,
            yaw,
            scale,
        );
        for side in [-1.0_f32, 1.0_f32] {
            append_local_ellipsoid(
                b_fauna,
                [side * 0.056, 1.325, 0.32],
                [0.016, 0.018, 0.018],
                8,
                6,
                obsidian_eye,
                pos,
                yaw,
                scale,
            );
            // Cupped erect marsupial pinnae ears reaching 1.65 m standing height
            append_cupped_ear_pinna(
                b_fauna,
                [side * 0.052, 1.36, 0.24],
                [side * 0.115, 1.65, 0.20],
                0.042,
                0.028,
                coat_dark,
                inner_ear_pink,
                charcoal_tip,
                pos,
                yaw,
                scale,
            );
        }

        // ── 3. Multi-Station Pentapedal Counterbalance Tail resting on slope ──
        let tail_stations = [
            AnatomicalLoftStation {
                center: [0.0, 0.46, -0.18],
                rx: 0.115,
                ry: 0.110,
                dorsal_color: coat_dark,
                flank_color: coat_lit,
                ventral_color: ventral_cream,
            },
            AnatomicalLoftStation {
                center: [0.0, 0.28, -0.40],
                rx: 0.095,
                ry: 0.090,
                dorsal_color: coat_dark,
                flank_color: coat_lit,
                ventral_color: ventral_cream,
            },
            AnatomicalLoftStation {
                center: [0.0, 0.11, -0.62],
                rx: 0.072,
                ry: 0.068,
                dorsal_color: coat_dark,
                flank_color: coat_lit,
                ventral_color: ventral_cream,
            },
            AnatomicalLoftStation {
                center: [0.0, 0.045, -0.86],
                rx: 0.050,
                ry: 0.046,
                dorsal_color: coat_dark,
                flank_color: coat_dark,
                ventral_color: coat_lit,
            },
            AnatomicalLoftStation {
                center: [0.0, 0.028, -1.08],
                rx: 0.028,
                ry: 0.026,
                dorsal_color: charcoal_tip,
                flank_color: charcoal_tip,
                ventral_color: coat_dark,
            },
        ];
        append_anatomical_loft_sweep(b_fauna, &tail_stations, 14, pos, yaw, scale, true);

        // ── 4. Z-Folded Saltatorial Hindlimbs & Forelimbs with Digits/Claws ──
        for side in [-1.0_f32, 1.0_f32] {
            let hindlimb_stations = [
                // Distal thigh / knee
                AnatomicalLoftStation {
                    center: [side * 0.18, 0.38, 0.10],
                    rx: 0.078,
                    ry: 0.085,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: ventral_cream,
                },
                // Muscular gastrocnemius calf belly
                AnatomicalLoftStation {
                    center: [side * 0.175, 0.26, -0.03],
                    rx: 0.062,
                    ry: 0.070,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: ventral_cream,
                },
                // Slender tibia & Achilles tendon approaching calcaneal hock
                AnatomicalLoftStation {
                    center: [side * 0.17, 0.12, -0.15],
                    rx: 0.042,
                    ry: 0.048,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: coat_lit,
                },
                // Plantigrade tarsus / ankle
                AnatomicalLoftStation {
                    center: [side * 0.165, 0.04, -0.11],
                    rx: 0.040,
                    ry: 0.036,
                    dorsal_color: coat_dark,
                    flank_color: coat_dark,
                    ventral_color: charcoal_tip,
                },
                // Elongated 4th metatarsal pes
                AnatomicalLoftStation {
                    center: [side * 0.16, 0.03, 0.12],
                    rx: 0.036,
                    ry: 0.028,
                    dorsal_color: coat_dark,
                    flank_color: coat_dark,
                    ventral_color: charcoal_tip,
                },
                // Hopping digit IV claw tip
                AnatomicalLoftStation {
                    center: [side * 0.16, 0.02, 0.29],
                    rx: 0.022,
                    ry: 0.018,
                    dorsal_color: charcoal_tip,
                    flank_color: charcoal_tip,
                    ventral_color: charcoal_tip,
                },
            ];
            append_anatomical_loft_sweep(b_fauna, &hindlimb_stations, 12, pos, yaw, scale, true);

            // Forelimbs (shoulder -> elbow -> forearm -> dark-clawed paw)
            let forelimb_stations = [
                AnatomicalLoftStation {
                    center: [side * 0.12, 0.94, 0.14],
                    rx: 0.046,
                    ry: 0.050,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: ventral_cream,
                },
                AnatomicalLoftStation {
                    center: [side * 0.13, 0.76, 0.17],
                    rx: 0.034,
                    ry: 0.036,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: ventral_cream,
                },
                AnatomicalLoftStation {
                    center: [side * 0.10, 0.66, 0.27],
                    rx: 0.026,
                    ry: 0.024,
                    dorsal_color: coat_dark,
                    flank_color: coat_dark,
                    ventral_color: charcoal_tip,
                },
                AnatomicalLoftStation {
                    center: [side * 0.085, 0.61, 0.31],
                    rx: 0.020,
                    ry: 0.014,
                    dorsal_color: charcoal_tip,
                    flank_color: charcoal_tip,
                    ventral_color: charcoal_tip,
                },
            ];
            append_anatomical_loft_sweep(b_fauna, &forelimb_stations, 10, pos, yaw, scale, true);
        }
    } else if archetype == "wombat" || archetype == "vombatid_fossorial" {
        // ── Common Wombat (*Vombatus ursinus*) — Stocky Barrel Torso, Broad Head & Digging Claws ──
        let bmin = [-0.28_f32, 0.04_f32, -0.54_f32];
        let bmax = [0.28_f32, 0.42_f32, 0.54_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (22, 18, 24),
            pos,
            yaw,
            scale,
            |p| {
                let d_rump = sdf_sphere(p, Point3::new(0.0, 0.22, -0.22), 0.21);
                let d_barrel = sdf_rounded_box(
                    p,
                    Point3::new(0.0, 0.22, -0.02),
                    Point3::new(0.19, 0.14, 0.26),
                    0.075,
                );
                let d_shoulders = sdf_sphere(p, Point3::new(0.0, 0.23, 0.18), 0.195);
                let d_head = sdf_rounded_box(
                    p,
                    Point3::new(0.0, 0.21, 0.35),
                    Point3::new(0.135, 0.105, 0.125),
                    0.055,
                );
                let d_muzzle = sdf_sphere(p, Point3::new(0.0, 0.18, 0.44), 0.085);
                let d0 = sdf_smooth_union(d_rump, d_barrel, 0.08);
                let d1 = sdf_smooth_union(d0, d_shoulders, 0.08);
                let d2 = sdf_smooth_union(d1, d_head, 0.07);
                sdf_smooth_union(d2, d_muzzle, 0.05)
            },
            |lp, ln| {
                if lp[2] > 0.45 && lp[1] < 0.24 {
                    return charcoal_tip;
                }
                let mottled = 0.06 * ((lp[0] * 18.0).sin() * (lp[2] * 14.0).cos());
                let base_c = countershade_color(coat_dark, coat_lit, ventral_cream, ln[1]);
                [
                    (base_c[0] + mottled).clamp(0.05, 0.95),
                    (base_c[1] + mottled * 0.9).clamp(0.05, 0.95),
                    (base_c[2] + mottled * 0.8).clamp(0.05, 0.95),
                    1.0,
                ]
            },
        );

        // Broad leathery rhinarium nose pad & obsidian eyes
        append_local_ellipsoid(
            b_fauna,
            [0.0, 0.19, 0.49],
            [0.045, 0.032, 0.024],
            10,
            6,
            charcoal_tip,
            pos,
            yaw,
            scale,
        );
        for side in [-1.0_f32, 1.0_f32] {
            append_local_ellipsoid(
                b_fauna,
                [side * 0.095, 0.25, 0.38],
                [0.016, 0.016, 0.016],
                8,
                6,
                obsidian_eye,
                pos,
                yaw,
                scale,
            );
            // Small rounded wombat ears
            append_cupped_ear_pinna(
                b_fauna,
                [side * 0.11, 0.31, 0.30],
                [side * 0.14, 0.38, 0.28],
                0.032,
                0.024,
                coat_dark,
                inner_ear_pink,
                charcoal_tip,
                pos,
                yaw,
                scale,
            );

            // Muscular front & rear fossorial digging legs with plantigrade claws
            for (z_leg, is_front) in [(0.22_f32, true), (-0.22_f32, false)] {
                let leg_stations = [
                    AnatomicalLoftStation {
                        center: [side * 0.18, 0.20, z_leg],
                        rx: 0.072,
                        ry: 0.075,
                        dorsal_color: coat_dark,
                        flank_color: coat_lit,
                        ventral_color: coat_dark,
                    },
                    AnatomicalLoftStation {
                        center: [side * 0.19, 0.09, z_leg + if is_front { 0.02 } else { -0.01 }],
                        rx: 0.058,
                        ry: 0.058,
                        dorsal_color: coat_dark,
                        flank_color: coat_dark,
                        ventral_color: charcoal_tip,
                    },
                    AnatomicalLoftStation {
                        center: [side * 0.185, 0.02, z_leg + 0.05],
                        rx: 0.052,
                        ry: 0.028,
                        dorsal_color: charcoal_tip,
                        flank_color: charcoal_tip,
                        ventral_color: charcoal_tip,
                    },
                ];
                append_anatomical_loft_sweep(b_fauna, &leg_stations, 10, pos, yaw, scale, true);

                // Fossorial digging claws
                for c_idx in -1..=1 {
                    let cx = side * 0.185 + (c_idx as f32) * 0.018;
                    append_local_ellipsoid(
                        b_fauna,
                        [cx, 0.015, z_leg + 0.095],
                        [0.007, 0.008, 0.022],
                        6,
                        4,
                        [0.78, 0.74, 0.66, 1.0],
                        pos,
                        yaw,
                        scale,
                    );
                }
            }
        }
    } else if archetype == "platypus" || archetype == "aquatic" {
        // ── Platypus (*Ornithorhynchus anatinus*) — Hydrofoil Body, Leathery Duck-Bill & Beaver Tail ──
        let bill_slate = [0.22, 0.23, 0.26, 1.0];
        let bill_nostril = [0.10, 0.11, 0.13, 1.0];
        let bmin = [-0.14_f32, 0.01_f32, -0.30_f32];
        let bmax = [0.14_f32, 0.16_f32, 0.30_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (20, 16, 24),
            pos,
            yaw,
            scale,
            |p| {
                let d_torso = sdf_rounded_box(
                    p,
                    Point3::new(0.0, 0.075, 0.0),
                    Point3::new(0.082, 0.042, 0.145),
                    0.032,
                );
                let d_head = sdf_sphere(p, Point3::new(0.0, 0.082, 0.13), 0.052);
                let d_bill = sdf_rounded_box(
                    p,
                    Point3::new(0.0, 0.064, 0.21),
                    Point3::new(0.048, 0.011, 0.055),
                    0.009,
                );
                let d_tail = sdf_rounded_box(
                    p,
                    Point3::new(0.0, 0.066, -0.19),
                    Point3::new(0.068, 0.015, 0.075),
                    0.012,
                );
                let d0 = sdf_smooth_union(d_torso, d_head, 0.035);
                let d1 = sdf_smooth_union(d0, d_bill, 0.022);
                sdf_smooth_union(d1, d_tail, 0.030)
            },
            |lp, ln| {
                if lp[2] > 0.155 {
                    return bill_slate;
                }
                if lp[2] < -0.13 {
                    return lerp_color(coat_dark, charcoal_tip, 0.35);
                }
                countershade_color(coat_dark, coat_lit, [0.76, 0.62, 0.44, 1.0], ln[1])
            },
        );

        // Dorsal bill nostrils & bead eyes
        for side in [-1.0_f32, 1.0_f32] {
            append_local_ellipsoid(
                b_fauna,
                [side * 0.016, 0.076, 0.23],
                [0.006, 0.004, 0.009],
                6,
                4,
                bill_nostril,
                pos,
                yaw,
                scale,
            );
            append_local_ellipsoid(
                b_fauna,
                [side * 0.042, 0.095, 0.14],
                [0.009, 0.009, 0.010],
                6,
                4,
                obsidian_eye,
                pos,
                yaw,
                scale,
            );

            // Webbed front & hind swimming paddles
            for (z_pad, span_x) in [(0.08_f32, 0.15_f32), (-0.09_f32, 0.13_f32)] {
                let paddle_stations = [
                    AnatomicalLoftStation {
                        center: [side * 0.075, 0.055, z_pad],
                        rx: 0.025,
                        ry: 0.016,
                        dorsal_color: coat_dark,
                        flank_color: coat_lit,
                        ventral_color: bill_slate,
                    },
                    AnatomicalLoftStation {
                        center: [side * span_x, 0.025, z_pad + 0.025],
                        rx: 0.046,
                        ry: 0.008,
                        dorsal_color: bill_slate,
                        flank_color: bill_slate,
                        ventral_color: bill_slate,
                    },
                ];
                append_anatomical_loft_sweep(b_fauna, &paddle_stations, 10, pos, yaw, scale, true);
            }
        }
    } else if is_cockatoo {
        // ── Sulphur-Crested Cockatoo (*Cacatua galerita*) — White Plumage, Curved Black Bill & Yellow Crest ──
        let white_plumage = [0.96, 0.96, 0.94, 1.0];
        let underwing_sulphur = [0.95, 0.92, 0.65, 1.0];
        let crest_yellow = [0.98, 0.86, 0.10, 1.0];

        let bmin = [-0.14_f32, -0.06_f32, -0.26_f32];
        let bmax = [0.14_f32, 0.16_f32, 0.26_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (18, 16, 20),
            pos,
            yaw,
            scale,
            |p| {
                let d_body = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.02, -0.14),
                    Point3::new(0.0, 0.05, 0.10),
                    0.068,
                );
                let d_head = sdf_sphere(p, Point3::new(0.0, 0.075, 0.14), 0.048);
                let d_bill = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.065, 0.17),
                    Point3::new(0.0, 0.030, 0.205),
                    0.018,
                );
                let d0 = sdf_smooth_union(d_body, d_head, 0.035);
                sdf_smooth_union(d0, d_bill, 0.018)
            },
            |lp, ln| {
                if lp[2] > 0.175 {
                    return charcoal_tip;
                }
                countershade_color(white_plumage, white_plumage, underwing_sulphur, ln[1])
            },
        );

        // Erect forward-curving yellow sulphur crest plumes (5 fanned 3D blades)
        for c_idx in 0..5 {
            let frac = c_idx as f32 / 4.0;
            let z_root = 0.15 - frac * 0.04;
            let y_tip = 0.185 - frac * 0.015;
            let z_tip = 0.19 - frac * 0.06;
            let x_spread = (frac - 0.5) * 0.024;
            append_cambered_feather_blade(
                b_fauna,
                [x_spread * 0.3, 0.11, z_root],
                [x_spread, y_tip, z_tip],
                [0.0, 0.0, 0.022],
                0.008,
                crest_yellow,
                crest_yellow,
                crest_yellow,
                pos,
                yaw,
                scale,
            );
        }

        // Cambered 3D Cockatoo Wings & Rounded Tail Fan
        for side in [-1.0_f32, 1.0_f32] {
            let wing_stations = [
                AnatomicalLoftStation {
                    center: [side * 0.06, 0.045, 0.05],
                    rx: 0.065,
                    ry: 0.022,
                    dorsal_color: white_plumage,
                    flank_color: white_plumage,
                    ventral_color: underwing_sulphur,
                },
                AnatomicalLoftStation {
                    center: [side * 0.28, 0.085, 0.08],
                    rx: 0.082,
                    ry: 0.016,
                    dorsal_color: white_plumage,
                    flank_color: white_plumage,
                    ventral_color: underwing_sulphur,
                },
                AnatomicalLoftStation {
                    center: [side * 0.52, 0.11, 0.02],
                    rx: 0.055,
                    ry: 0.010,
                    dorsal_color: white_plumage,
                    flank_color: white_plumage,
                    ventral_color: underwing_sulphur,
                },
            ];
            append_anatomical_loft_sweep(b_fauna, &wing_stations, 10, pos, yaw, scale, true);

            for p_idx in 0..5 {
                let pf = p_idx as f32 / 4.0;
                append_cambered_feather_blade(
                    b_fauna,
                    [side * (0.34 + pf * 0.16), 0.095, 0.04 - pf * 0.04],
                    [side * (0.58 + pf * 0.12), 0.12 + pf * 0.015, -0.04 - pf * 0.09],
                    [0.0, 0.0, 0.038],
                    0.007,
                    white_plumage,
                    underwing_sulphur,
                    white_plumage,
                    pos,
                    yaw,
                    scale,
                );
            }
        }

        for t_idx in -2..=2 {
            let tf = t_idx as f32 / 2.0;
            append_cambered_feather_blade(
                b_fauna,
                [tf * 0.025, 0.02, -0.14],
                [tf * 0.11, -0.01, -0.34 + tf.abs() * 0.03],
                [0.035, 0.0, 0.0],
                0.007,
                white_plumage,
                underwing_sulphur,
                white_plumage,
                pos,
                yaw,
                scale,
            );
        }
    } else if archetype == "raptor" || archetype == "raptor_soaring" {
        // ── Wedge-Tailed Eagle (*Aquila audax*) — 2.10 m Cambered Airfoil Wings, Slotted Primaries & Diamond Tail ──
        let nape_rufous = [0.64, 0.42, 0.22, 1.0];
        let cere_gold = [0.92, 0.76, 0.18, 1.0];
        let slate_beak = [0.18, 0.20, 0.24, 1.0];

        let bmin = [-0.16_f32, -0.08_f32, -0.34_f32];
        let bmax = [0.16_f32, 0.16_f32, 0.34_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (20, 16, 22),
            pos,
            yaw,
            scale,
            |p| {
                let d_body = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.02, -0.19),
                    Point3::new(0.0, 0.045, 0.14),
                    0.080,
                );
                let d_head = sdf_sphere(p, Point3::new(0.0, 0.068, 0.19), 0.050);
                let d_cere = sdf_sphere(p, Point3::new(0.0, 0.062, 0.23), 0.026);
                let d_beak = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.056, 0.23),
                    Point3::new(0.0, 0.018, 0.285),
                    0.016,
                );
                let d0 = sdf_smooth_union(d_body, d_head, 0.04);
                let d1 = sdf_smooth_union(d0, d_cere, 0.02);
                sdf_smooth_union(d1, d_beak, 0.018)
            },
            |lp, ln| {
                if lp[2] > 0.255 {
                    return slate_beak;
                }
                if lp[2] > 0.22 && lp[1] > 0.04 {
                    return cere_gold;
                }
                if lp[2] > 0.08 && lp[2] < 0.21 && ln[1] > 0.25 {
                    return lerp_color(coat_lit, nape_rufous, 0.65);
                }
                countershade_color(coat_dark, coat_lit, [0.36, 0.28, 0.20, 1.0], ln[1])
            },
        );

        // Feathered tarsus legs & yellow talons tucked beneath pelvis
        for side in [-1.0_f32, 1.0_f32] {
            append_local_ellipsoid(
                b_fauna,
                [side * 0.032, 0.075, 0.20],
                [0.011, 0.011, 0.012],
                6,
                5,
                cere_gold,
                pos,
                yaw,
                scale,
            );
            let leg_st = [
                AnatomicalLoftStation {
                    center: [side * 0.048, -0.02, -0.06],
                    rx: 0.028,
                    ry: 0.028,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: coat_dark,
                },
                AnatomicalLoftStation {
                    center: [side * 0.045, -0.065, -0.14],
                    rx: 0.016,
                    ry: 0.016,
                    dorsal_color: cere_gold,
                    flank_color: cere_gold,
                    ventral_color: charcoal_tip,
                },
            ];
            append_anatomical_loft_sweep(b_fauna, &leg_st, 8, pos, yaw, scale, true);

            // 3D Cambered Wing Arm & Forearm Loft (2.10 m span)
            let wing_arm = [
                AnatomicalLoftStation {
                    center: [side * 0.07, 0.04, 0.06],
                    rx: 0.11,
                    ry: 0.028,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: coat_lit,
                },
                AnatomicalLoftStation {
                    center: [side * 0.34, 0.075, 0.09],
                    rx: 0.13,
                    ry: 0.024,
                    dorsal_color: coat_dark,
                    flank_color: nape_rufous,
                    ventral_color: coat_lit,
                },
                AnatomicalLoftStation {
                    center: [side * 0.62, 0.115, 0.11],
                    rx: 0.11,
                    ry: 0.018,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: coat_lit,
                },
                AnatomicalLoftStation {
                    center: [side * 0.84, 0.14, 0.05],
                    rx: 0.075,
                    ry: 0.012,
                    dorsal_color: charcoal_tip,
                    flank_color: coat_dark,
                    ventral_color: coat_dark,
                },
            ];
            append_anatomical_loft_sweep(b_fauna, &wing_arm, 12, pos, yaw, scale, true);

            // 6 Slotted Primary Wingtip Fingers (characteristic soaring eagle silhouette)
            for p_idx in 0..6 {
                let pf = p_idx as f32 / 5.0;
                let root_x = side * (0.64 + pf * 0.18);
                let root_y = 0.115 + pf * 0.022;
                let root_z = 0.08 - pf * 0.07;
                let tip_x = side * (0.92 + (1.0 - (pf - 0.35).abs()) * 0.16);
                let tip_y = 0.165 + (1.0 - pf) * 0.035;
                let tip_z = 0.10 - pf * 0.24;
                append_cambered_feather_blade(
                    b_fauna,
                    [root_x, root_y, root_z],
                    [tip_x, tip_y, tip_z],
                    [0.0, 0.0, 0.048],
                    0.009,
                    coat_dark,
                    coat_lit,
                    charcoal_tip,
                    pos,
                    yaw,
                    scale,
                );
            }

            // 7 Secondary Trailing-Edge Flight Feathers
            for s_idx in 0..7 {
                let sf = s_idx as f32 / 6.0;
                let rx = side * (0.12 + sf * 0.48);
                let ry = 0.045 + sf * 0.065;
                let rz = 0.04 + sf * 0.04;
                append_cambered_feather_blade(
                    b_fauna,
                    [rx, ry, rz],
                    [rx * 1.03, ry - 0.015, -0.18 - (1.0 - (sf - 0.5).abs()) * 0.05],
                    [0.042, 0.0, 0.0],
                    0.008,
                    coat_dark,
                    coat_lit,
                    charcoal_tip,
                    pos,
                    yaw,
                    scale,
                );
            }
        }

        // Iconic Diamond-Shaped Wedge Tail Fan (7 overlapping 3D cambered rectrices)
        for t_idx in -3..=3 {
            let tf = t_idx as f32 / 3.0;
            let wedge_ext = (1.0 - tf.abs()) * 0.16;
            append_cambered_feather_blade(
                b_fauna,
                [tf * 0.035, 0.015, -0.18],
                [tf * 0.22, -0.025, -0.48 - wedge_ext],
                [0.045, 0.0, 0.0],
                0.008,
                coat_dark,
                coat_lit,
                charcoal_tip,
                pos,
                yaw,
                scale,
            );
        }
    } else if archetype == "cod" || archetype == "fish" {
        // ── Murray Cod (*Maccullochella peelii*) — Deep Body, Mottled Reticulation, Spiny & Soft Fins ──
        let belly_cream = [0.86, 0.88, 0.78, 1.0];
        let fin_olive = lerp_color(coat_dark, coat_lit, 0.4);
        let fin_margin = [0.82, 0.88, 0.80, 1.0];

        let bmin = [-0.20_f32, 0.0_f32, -0.56_f32];
        let bmax = [0.20_f32, 0.30_f32, 0.56_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (20, 20, 26),
            pos,
            yaw,
            scale,
            |p| {
                let d_trunk = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.14, -0.22),
                    Point3::new(0.0, 0.15, 0.22),
                    0.135,
                );
                let d_peduncle = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.14, -0.22),
                    Point3::new(0.0, 0.13, -0.40),
                    0.075,
                );
                let d_head = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.15, 0.20),
                    Point3::new(0.0, 0.13, 0.48),
                    0.098,
                );
                let d0 = sdf_smooth_union(d_trunk, d_peduncle, 0.06);
                sdf_smooth_union(d0, d_head, 0.07)
            },
            |lp, ln| {
                let retic = 0.12 * ((lp[2] * 28.0).sin() * (lp[1] * 32.0).cos());
                let base = countershade_color(coat_dark, coat_lit, belly_cream, ln[1]);
                [
                    (base[0] + retic * 0.5).clamp(0.05, 0.95),
                    (base[1] + retic).clamp(0.05, 0.95),
                    (base[2] + retic * 0.4).clamp(0.05, 0.95),
                    1.0,
                ]
            },
        );

        // Eyes & Pectoral / Pelvic Fins
        for side in [-1.0_f32, 1.0_f32] {
            append_local_ellipsoid(
                b_fauna,
                [side * 0.078, 0.17, 0.36],
                [0.016, 0.018, 0.018],
                8,
                6,
                obsidian_eye,
                pos,
                yaw,
                scale,
            );
            append_cambered_feather_blade(
                b_fauna,
                [side * 0.11, 0.11, 0.22],
                [side * 0.21, 0.07, 0.10],
                [0.0, 0.045, 0.0],
                0.008,
                fin_olive,
                belly_cream,
                fin_margin,
                pos,
                yaw,
                scale,
            );
        }

        // Spiny Anterior Dorsal Fin + Rounded Soft Posterior Dorsal Fin
        for d_idx in 0..7 {
            let df = d_idx as f32 / 6.0;
            let z_pos = 0.14 - df * 0.34;
            let h_ext = if df < 0.5 { 0.065 } else { 0.095 };
            append_cambered_feather_blade(
                b_fauna,
                [0.0, 0.26, z_pos],
                [0.0, 0.26 + h_ext, z_pos - 0.04],
                [0.0, 0.0, 0.035],
                0.008,
                coat_dark,
                fin_olive,
                fin_margin,
                pos,
                yaw,
                scale,
            );
        }

        // Rounded Paddle Caudal Tail Fin & Anal Fin
        for c_idx in -2..=2 {
            let cf = c_idx as f32 / 2.0;
            append_cambered_feather_blade(
                b_fauna,
                [0.0, 0.13 + cf * 0.03, -0.38],
                [0.0, 0.13 + cf * 0.10, -0.55 + cf.abs() * 0.03],
                [0.0, 0.038, 0.0],
                0.008,
                fin_olive,
                fin_olive,
                fin_margin,
                pos,
                yaw,
                scale,
            );
        }
    } else if archetype == "emu" || archetype == "ratite" || archetype == "ratite_bipedal" {
        // ── Emu (*Dromaius novaehollandiae*) — Shaggy Drooping Plumage, Bare Bluish Neck & 3-Toed Cursorial Legs ──
        let bare_blue_neck = [0.28, 0.38, 0.52, 1.0];
        let bmin = [-0.30_f32, 0.76_f32, -0.42_f32];
        let bmax = [0.30_f32, 1.76_f32, 0.44_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (20, 24, 20),
            pos,
            yaw,
            scale,
            |p| {
                let d_body = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.96, -0.14),
                    Point3::new(0.0, 1.02, 0.08),
                    0.25,
                );
                let d_neck = sdf_capsule(
                    p,
                    Point3::new(0.0, 1.08, 0.12),
                    Point3::new(0.0, 1.60, 0.23),
                    0.062,
                );
                let d_head = sdf_sphere(p, Point3::new(0.0, 1.63, 0.25), 0.058);
                let d_beak = sdf_capsule(
                    p,
                    Point3::new(0.0, 1.62, 0.27),
                    Point3::new(0.0, 1.60, 0.39),
                    0.018,
                );
                let d0 = sdf_smooth_union(d_body, d_neck, 0.09);
                let d1 = sdf_smooth_union(d0, d_head, 0.045);
                sdf_smooth_union(d1, d_beak, 0.02)
            },
            |lp, ln| {
                if lp[2] > 0.31 && lp[1] > 1.54 {
                    return charcoal_tip;
                }
                if lp[1] > 1.18 && lp[1] < 1.58 {
                    return bare_blue_neck;
                }
                countershade_color(coat_dark, coat_lit, ventral_cream, ln[1])
            },
        );

        for side in [-1.0_f32, 1.0_f32] {
            let leg_st = [
                AnatomicalLoftStation {
                    center: [side * 0.13, 0.92, -0.02],
                    rx: 0.058,
                    ry: 0.062,
                    dorsal_color: coat_dark,
                    flank_color: coat_lit,
                    ventral_color: coat_dark,
                },
                AnatomicalLoftStation {
                    center: [side * 0.13, 0.48, -0.04],
                    rx: 0.028,
                    ry: 0.030,
                    dorsal_color: charcoal_tip,
                    flank_color: coat_dark,
                    ventral_color: charcoal_tip,
                },
                AnatomicalLoftStation {
                    center: [side * 0.125, 0.03, 0.04],
                    rx: 0.022,
                    ry: 0.022,
                    dorsal_color: charcoal_tip,
                    flank_color: charcoal_tip,
                    ventral_color: charcoal_tip,
                },
                AnatomicalLoftStation {
                    center: [side * 0.125, 0.015, 0.16],
                    rx: 0.032,
                    ry: 0.014,
                    dorsal_color: charcoal_tip,
                    flank_color: charcoal_tip,
                    ventral_color: charcoal_tip,
                },
            ];
            append_anatomical_loft_sweep(b_fauna, &leg_st, 10, pos, yaw, scale, true);
        }
    } else if archetype == "monitor" || archetype == "goanna_skink" || archetype == "squamate_sprawling" {
        // ── Lace Monitor (*Varanus varius*) — Banded Squamation, Sprawling Pentadactyl Limbs & Whip Tail ──
        let band_cream = [0.86, 0.80, 0.56, 1.0];
        let bmin = [-0.24_f32, 0.01_f32, -0.46_f32];
        let bmax = [0.24_f32, 0.24_f32, 0.66_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (20, 16, 24),
            pos,
            yaw,
            scale,
            |p| {
                let d_body = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.10, -0.28),
                    Point3::new(0.0, 0.11, 0.28),
                    0.088,
                );
                let d_neck = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.11, 0.26),
                    Point3::new(0.0, 0.12, 0.42),
                    0.064,
                );
                let d_head = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.12, 0.40),
                    Point3::new(0.0, 0.095, 0.60),
                    0.046,
                );
                let d0 = sdf_smooth_union(d_body, d_neck, 0.05);
                sdf_smooth_union(d0, d_head, 0.04)
            },
            |lp, ln| {
                let band = ((lp[2] * 36.0).sin() > 0.45) as i32 as f32;
                let dorsal_col = lerp_color(coat_dark, band_cream, band * 0.65);
                countershade_color(dorsal_col, coat_lit, band_cream, ln[1])
            },
        );

        let tail_st = [
            AnatomicalLoftStation {
                center: [0.0, 0.095, -0.28],
                rx: 0.065,
                ry: 0.062,
                dorsal_color: coat_dark,
                flank_color: band_cream,
                ventral_color: band_cream,
            },
            AnatomicalLoftStation {
                center: [0.07, 0.065, -0.64],
                rx: 0.044,
                ry: 0.042,
                dorsal_color: band_cream,
                flank_color: coat_dark,
                ventral_color: band_cream,
            },
            AnatomicalLoftStation {
                center: [-0.09, 0.040, -0.98],
                rx: 0.025,
                ry: 0.024,
                dorsal_color: coat_dark,
                flank_color: band_cream,
                ventral_color: band_cream,
            },
            AnatomicalLoftStation {
                center: [0.04, 0.022, -1.24],
                rx: 0.010,
                ry: 0.010,
                dorsal_color: charcoal_tip,
                flank_color: charcoal_tip,
                ventral_color: charcoal_tip,
            },
        ];
        append_anatomical_loft_sweep(b_fauna, &tail_st, 10, pos, yaw, scale, true);

        for side in [-1.0_f32, 1.0_f32] {
            for z_leg in [0.22_f32, -0.22_f32] {
                let leg_st = [
                    AnatomicalLoftStation {
                        center: [side * 0.08, 0.10, z_leg],
                        rx: 0.036,
                        ry: 0.034,
                        dorsal_color: coat_dark,
                        flank_color: band_cream,
                        ventral_color: band_cream,
                    },
                    AnatomicalLoftStation {
                        center: [side * 0.19, 0.075, z_leg + 0.03],
                        rx: 0.026,
                        ry: 0.026,
                        dorsal_color: coat_dark,
                        flank_color: coat_lit,
                        ventral_color: band_cream,
                    },
                    AnatomicalLoftStation {
                        center: [side * 0.23, 0.02, z_leg + 0.07],
                        rx: 0.022,
                        ry: 0.014,
                        dorsal_color: charcoal_tip,
                        flank_color: charcoal_tip,
                        ventral_color: charcoal_tip,
                    },
                ];
                append_anatomical_loft_sweep(b_fauna, &leg_st, 8, pos, yaw, scale, true);
            }
        }
    } else if archetype == "frog" {
        // ── Southern Corroboree Frog (*Pseudophryne corroboree*) — Vivid Aposematic Black & Yellow Stripes ──
        let corroboree_black = [0.07, 0.07, 0.08, 1.0];
        let corroboree_yellow = [0.96, 0.88, 0.06, 1.0];

        let bmin = [-0.11_f32, 0.01_f32, -0.16_f32];
        let bmax = [0.11_f32, 0.13_f32, 0.16_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (18, 16, 20),
            pos,
            yaw,
            scale,
            |p| {
                let d_body = sdf_sphere(p, Point3::new(0.0, 0.055, -0.02), 0.072);
                let d_head = sdf_sphere(p, Point3::new(0.0, 0.062, 0.068), 0.050);
                sdf_smooth_union(d_body, d_head, 0.04)
            },
            |lp, _ln| {
                // Vivid longitudinal yellow and black dorsal bands
                let stripe = (lp[0] * 68.0 + (lp[2] * 18.0).sin() * 0.8).cos();
                if stripe > 0.15 {
                    corroboree_yellow
                } else {
                    corroboree_black
                }
            },
        );

        for side in [-1.0_f32, 1.0_f32] {
            append_local_ellipsoid(
                b_fauna,
                [side * 0.034, 0.095, 0.085],
                [0.014, 0.014, 0.014],
                8,
                6,
                obsidian_eye,
                pos,
                yaw,
                scale,
            );
            let hind_st = [
                AnatomicalLoftStation {
                    center: [side * 0.055, 0.045, -0.06],
                    rx: 0.024,
                    ry: 0.022,
                    dorsal_color: corroboree_black,
                    flank_color: corroboree_yellow,
                    ventral_color: corroboree_black,
                },
                AnatomicalLoftStation {
                    center: [side * 0.095, 0.035, -0.01],
                    rx: 0.018,
                    ry: 0.016,
                    dorsal_color: corroboree_yellow,
                    flank_color: corroboree_black,
                    ventral_color: corroboree_yellow,
                },
                AnatomicalLoftStation {
                    center: [side * 0.075, 0.015, -0.07],
                    rx: 0.016,
                    ry: 0.010,
                    dorsal_color: corroboree_black,
                    flank_color: corroboree_yellow,
                    ventral_color: corroboree_black,
                },
            ];
            append_anatomical_loft_sweep(b_fauna, &hind_st, 8, pos, yaw, scale, true);
        }
    } else {
        // Fallback countershaded quadruped / marsupial body
        let bmin = [-0.20_f32, 0.02_f32, -0.42_f32];
        let bmax = [0.20_f32, 0.36_f32, 0.42_f32];
        append_countershaded_sdf_body(
            b_fauna,
            bmin,
            bmax,
            (18, 18, 20),
            pos,
            yaw,
            scale,
            |p| {
                let d_body = sdf_capsule(
                    p,
                    Point3::new(0.0, 0.16, -0.22),
                    Point3::new(0.0, 0.18, 0.22),
                    0.13,
                );
                let d_head = sdf_sphere(p, Point3::new(0.0, 0.22, 0.30), 0.095);
                sdf_smooth_union(d_body, d_head, 0.06)
            },
            |_lp, ln| countershade_color(coat_dark, coat_lit, ventral_cream, ln[1]),
        );
    }
}

/// Populates a sector scene with an ecologically authentic, multi-strata community of
/// high-detail Australian fauna specimens (`b_fauna`).
pub fn build_sector_fauna_population(
    sector_id: &str,
    water_y: f32,
    sample_elev: &dyn Fn(f32, f32) -> f32,
    road_center_x: &dyn Fn(f32) -> f32,
    b_fauna: &mut SubmeshBuilder,
) {
    // 1. Eastern Grey Kangaroo (*Macropus giganteus*) mob in the grassy woodland clearing
    for (kx_off, kz, kyaw, is_road_side, scale_mul) in [
        (0.24_f32, 0.14_f32, -0.45_f32, true, 1.05_f32),
        (0.32_f32, -0.10_f32, -0.25_f32, true, 0.96_f32),
        (0.38_f32, 0.04_f32, -0.65_f32, true, 0.74_f32),
        (-0.68_f32, 0.35_f32, 0.85_f32, false, 1.00_f32),
    ] {
        let kx = if is_road_side {
            road_center_x(kz) + kx_off
        } else {
            kx_off
        };
        let ky = sample_elev(kx, kz).max(water_y + 0.08) + 0.008;
        add_lofted_fauna_specimen(
            b_fauna,
            [kx, ky, kz],
            kyaw,
            SECTOR_METRIC_SCALE * scale_mul,
            "macropod",
            [0.42, 0.35, 0.28, 1.0],
            [0.68, 0.58, 0.46, 1.0],
        );
    }

    // 2. Common Wombats (*Vombatus ursinus*) foraging near tussock grass
    for (wx, wz, wyaw) in [
        (-0.55_f32, -0.32_f32, 0.45_f32),
        (0.38_f32, 0.65_f32, -0.85_f32),
    ] {
        let wy = sample_elev(wx, wz).max(water_y + 0.05);
        add_lofted_fauna_specimen(
            b_fauna,
            [wx, wy, wz],
            wyaw,
            SECTOR_METRIC_SCALE,
            "wombat",
            [0.28, 0.22, 0.18, 1.0],
            [0.48, 0.40, 0.32, 1.0],
        );
    }

    // 3. Submerged Platypus (*Ornithorhynchus anatinus*) & Murray Cod (*Maccullochella peelii*) in the river channel
    for (uz, uyaw, is_cod) in [
        (-0.45_f32, 0.20_f32, false),
        (0.35_f32, -0.30_f32, true),
        (-0.08_f32, 0.15_f32, false),
    ] {
        let ux = 0.28 * (uz * 1.35 + 1.1).sin();
        let bed_y = sample_sector_elevation(sector_id, ux, uz, water_y).min(sample_elev(ux, uz));
        let swim_y = (bed_y + 0.08).min(water_y - 0.04);
        let (cd, cl, arch) = if is_cod {
            (
                [0.16, 0.34, 0.20, 1.0],
                [0.38, 0.60, 0.34, 1.0],
                "cod",
            )
        } else {
            (
                [0.26, 0.18, 0.12, 1.0],
                [0.46, 0.34, 0.24, 1.0],
                "platypus",
            )
        };
        add_lofted_fauna_specimen(
            b_fauna,
            [ux, swim_y, uz],
            uyaw,
            SECTOR_METRIC_SCALE,
            arch,
            cd,
            cl,
        );
    }

    // 4. Aerial Stratum: Soaring Wedge-Tailed Eagle (*Aquila audax*) & Sulphur-Crested Cockatoo (*Cacatua galerita*)
    add_lofted_fauna_specimen(
        b_fauna,
        [-0.25, 0.58, -0.15],
        0.55,
        SECTOR_METRIC_SCALE,
        "raptor",
        [0.22, 0.16, 0.12, 1.0],
        [0.46, 0.34, 0.22, 1.0],
    );
    add_lofted_fauna_specimen(
        b_fauna,
        [0.32, 0.48, 0.25],
        -0.75,
        SECTOR_METRIC_SCALE,
        "cockatoo",
        [0.92, 0.92, 0.88, 1.0],
        [0.98, 0.96, 0.64, 1.0],
    );

    // 5. Sector-Specific Keystone Species (Corroboree Frogs, Lace Monitors, Emus)
    match sector_id {
        "three_mile_dam" => {
            for (fx, fz, fyaw) in [
                (-0.34_f32, 0.18_f32, 0.60_f32),
                (-0.28_f32, -0.22_f32, -0.45_f32),
            ] {
                let fy = sample_elev(fx, fz).max(water_y + 0.02);
                add_lofted_fauna_specimen(
                    b_fauna,
                    [fx, fy, fz],
                    fyaw,
                    SECTOR_METRIC_SCALE * 1.35,
                    "frog",
                    [0.08, 0.08, 0.09, 1.0],
                    [0.96, 0.88, 0.06, 1.0],
                );
            }
            let ex = -0.82_f32;
            let ez = -0.58_f32;
            let ey = sample_elev(ex, ez).max(water_y + 0.08);
            add_lofted_fauna_specimen(
                b_fauna,
                [ex, ey, ez],
                0.35,
                SECTOR_METRIC_SCALE,
                "emu",
                [0.24, 0.20, 0.18, 1.0],
                [0.44, 0.38, 0.32, 1.0],
            );
        }
        "sydney_sandstone" | "noosa_headland" => {
            let mx = 0.78_f32;
            let mz = -0.38_f32;
            let my = sample_elev(mx, mz).max(water_y + 0.08);
            add_lofted_fauna_specimen(
                b_fauna,
                [mx, my, mz],
                -1.20,
                SECTOR_METRIC_SCALE,
                "monitor",
                [0.20, 0.18, 0.15, 1.0],
                [0.74, 0.66, 0.42, 1.0],
            );
            let ex = -0.76_f32;
            let ez = -0.48_f32;
            let ey = sample_elev(ex, ez).max(water_y + 0.08);
            add_lofted_fauna_specimen(
                b_fauna,
                [ex, ey, ez],
                0.65,
                SECTOR_METRIC_SCALE,
                "emu",
                [0.24, 0.20, 0.18, 1.0],
                [0.44, 0.38, 0.32, 1.0],
            );
        }
        _ => {
            let ex = -0.78_f32;
            let ez = -0.52_f32;
            let ey = sample_elev(ex, ez).max(water_y + 0.08);
            add_lofted_fauna_specimen(
                b_fauna,
                [ex, ey, ez],
                0.40,
                SECTOR_METRIC_SCALE,
                "emu",
                [0.24, 0.20, 0.18, 1.0],
                [0.44, 0.38, 0.32, 1.0],
            );
            let mx = 0.82_f32;
            let mz = -0.44_f32;
            let my = sample_elev(mx, mz).max(water_y + 0.08);
            add_lofted_fauna_specimen(
                b_fauna,
                [mx, my, mz],
                -1.10,
                SECTOR_METRIC_SCALE,
                "monitor",
                [0.20, 0.18, 0.15, 1.0],
                [0.74, 0.66, 0.42, 1.0],
            );
        }
    }
}
