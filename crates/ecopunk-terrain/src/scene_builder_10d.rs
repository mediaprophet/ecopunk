//! Native Zero-V8 `.10d` Scene & Botanical/Zoological Digital Twin Builder.
//!
//! Synthesizes multi-material (`MAT4`), UV-mapped (`UV01`), normal-framed (`FNRM`),
//! vertex-coloured (`SRD1`), provenance-sealed (`PROV`) `.10d` containers and
//! renders them directly on the native GPU via `qualia_core_db::render::gpu::PortalGpu`
//! and `webizen_render::VolumetricRenderer` (`wgpu` Vulkan/DX12/Metal — 100% Zero V8).
//!
//! Uses QualiaDB's native computational geometry and simulation crates:
//! - `qualia_core_db::specialized_libs::computational_geometry::botanical_hulls::{asymmetric_hull_capacity, asymmetric_hull_into, AsymmetricHullParams}`
//! - `qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::{extrude_profile_along_path, ExtrusionStation}`
//! - `qualia_core_db::render::vegetation::{VegetationBendingParams, VegetationCollider, VegetationEvaluator, VegetationInstanceState}`
//! - `qualia_core_db::render::lighting::environment_lighting::{EnvironmentProbe, EnvironmentProbeSet, ENVIRONMENT_SPECULAR_LEVELS}`
//! - `qualia_core_db::render::compile_10d::{compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras}`
//! - `qualia_core_db::container_10d::{attach_material_section, attach_texture_coordinate_section, MaterialRecord, OpacityMode, ShadingModel, SubmeshRange}`

use image::ImageEncoder;
use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
use qualia_core_db::container_10d::{
    attach_material_section, attach_texture_coordinate_section, MaterialRecord, OpacityMode,
    ShadingModel, SubmeshRange,
};
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, compiled_digest, Compile10dExtras,
};
use qualia_core_db::render::gpu::{PortalGpu, TextureColorSpace, TextureMipSemantic};
use qualia_core_db::render::instance_culling::GpuInstanceRecord;
use qualia_core_db::render::lighting::environment_lighting::{
    EnvironmentProbe, EnvironmentProbeSet,
};
use qualia_core_db::render::telemetry::SystemTelemetry;
use qualia_core_db::render::vegetation::{
    VegetationBendingParams, VegetationCollider, VegetationEvaluator, VegetationInstanceState,
};
use qualia_core_db::specialized_libs::computational_geometry::botanical_hulls::{
    asymmetric_hull_capacity, asymmetric_hull_into, AsymmetricHullParams,
};
use qualia_core_db::specialized_libs::computational_geometry::math_geometry::AffineFrame3;
use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::{
    extrude_profile_along_path, ExtrusionStation,
};
use qualia_core_db::specialized_libs::computational_geometry::{Point2, Point3};
use sha2::{Digest, Sha256};

/// Procedural RGBA8 texture resource with its SHA-256 digest and encoded PNG bytes for `.hmc` packaging.
#[derive(Clone)]
pub struct ProceduralTextureAsset {
    pub name: &'static str,
    pub hmc_key: String,
    pub sha256_bytes: [u8; 32],
    pub sha256_hex: String,
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
    pub png_bytes: Vec<u8>,
}

/// Compiled `.10d` scene + water surface pair ready for `.hmc` packaging and `PortalGpu` rendering.
pub struct CompiledSector10d {
    pub sector_id: &'static str,
    pub title: &'static str,
    pub scene_hmc_key: String,
    pub water_hmc_key: String,
    pub scene_10d_bytes: Vec<u8>,
    pub water_10d_bytes: Vec<u8>,
    pub scene_crc32c: u32,
    pub water_crc32c: u32,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
    pub materials: Vec<MaterialRecord>,
    pub ranges: Vec<SubmeshRange>,
    pub water_positions: Vec<[f32; 3]>,
    pub water_indices: Vec<u32>,
    pub water_color: [f32; 3],
    pub water_alpha: f32,
    pub sun_dir: [f32; 3],
    pub cam_yaw: f32,
    pub cam_pitch: f32,
    pub cam_zoom: f32,
    pub cam_target: [f32; 3],
    pub sky_preset: u32,
    pub vegetation_instances_evaluated: usize,
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

#[inline]
fn hash21(x: f32, y: f32) -> f32 {
    let qx = (x * 127.1 + y * 311.7).sin() * 43758.5453;
    qx - qx.floor()
}

/// Synthesize 4 high-detail procedural RGBA8 textures for Foliage Venation, Bark Micro-Relief,
/// Riverbed Pebbles/Lithology, and 6-Layer SLGA Soil Strata.
pub fn build_procedural_textures() -> Vec<ProceduralTextureAsset> {
    let w = 128u32;
    let h = 128u32;
    let mut out = Vec::with_capacity(4);

    // 1. Sclerophyllous Leaf Venation & Cuticle Texture (`LeafVascularSkeleton` midrib + lateral veins + oil glands)
    let mut leaf_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / (w - 1) as f32;
            let v = y as f32 / (h - 1) as f32;
            let midrib_dist = (u - 0.5).abs();
            let midrib = (1.0 - (midrib_dist / 0.045).clamp(0.0, 1.0)).powf(1.8);
            let vein_coord = (v * 18.0 - midrib_dist * 11.0).sin().abs();
            let secondary_vein = (1.0 - (vein_coord / 0.16).clamp(0.0, 1.0))
                * (1.0 - (midrib_dist * 1.8).clamp(0.0, 1.0));
            let oil_gland = if hash21(x as f32 * 0.7, y as f32 * 0.7) > 0.86 {
                0.10
            } else {
                0.0
            };
            let edge_darken = 1.0 - 0.18 * (midrib_dist * 2.0).powf(2.0);
            let r = ((0.76 + 0.18 * midrib + 0.09 * secondary_vein + oil_gland) * edge_darken)
                .clamp(0.0, 1.0);
            let g = ((0.84 + 0.14 * midrib + 0.08 * secondary_vein + oil_gland) * edge_darken)
                .clamp(0.0, 1.0);
            let b = ((0.72 + 0.08 * midrib + 0.04 * secondary_vein) * edge_darken).clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            leaf_rgba[idx] = (r * 255.0) as u8;
            leaf_rgba[idx + 1] = (g * 255.0) as u8;
            leaf_rgba[idx + 2] = (b * 255.0) as u8;
            leaf_rgba[idx + 3] = 255;
        }
    }

    // 2. Eucalyptus / Ghost Gum / Stringybark Fibrous Bark Texture
    let mut bark_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / (w - 1) as f32;
            let v = y as f32 / (h - 1) as f32;
            let furrow = 0.5 + 0.5 * (u * 44.0 + (v * 12.0).sin() * 2.2).sin();
            let mottling =
                0.5 + 0.5 * (u * 9.5 + v * 7.2).sin() * (u * 5.1 - v * 11.3).cos();
            let micro = hash21(x as f32, y as f32) * 0.12;
            let shade = (0.72 + 0.18 * furrow + 0.12 * mottling - 0.06 + micro).clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            bark_rgba[idx] = (shade * 250.0) as u8;
            bark_rgba[idx + 1] = (shade * 244.0) as u8;
            bark_rgba[idx + 2] = (shade * 234.0) as u8;
            bark_rgba[idx + 3] = 255;
        }
    }

    // 3. Riverbed Voronoi Pebbles, Imbricated Cobbles, Alluvial Sand & Lithology Texture
    let mut pebble_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / w as f32 * 10.0;
            let v = y as f32 / h as f32 * 10.0;
            let cell_x = u.floor();
            let cell_y = v.floor();
            let fx = u - cell_x - 0.5;
            let fy = v - cell_y - 0.5;
            let dist = (fx * fx * 1.1 + fy * fy * 1.4).sqrt();
            let pebble_mask = (1.0 - (dist / 0.48).clamp(0.0, 1.0)).powf(0.65);
            let rnd = hash21(cell_x * 13.7, cell_y * 29.3);
            let (pr, pg, pb) = if rnd > 0.72 {
                (0.92, 0.86, 0.76) // quartz pebble
            } else if rnd > 0.44 {
                (0.84, 0.66, 0.54) // jasper / ironstone cobble
            } else if rnd < 0.24 {
                (0.62, 0.64, 0.68) // basalt / slate cobble
            } else {
                (0.78, 0.74, 0.68) // granite cobble
            };
            let sand_ripple = 0.82 + 0.18 * (u * 3.1 + v * 2.4).sin();
            let r = (sand_ripple * 0.78 * (1.0 - pebble_mask) + pr * pebble_mask).clamp(0.0, 1.0);
            let g = (sand_ripple * 0.72 * (1.0 - pebble_mask) + pg * pebble_mask).clamp(0.0, 1.0);
            let b = (sand_ripple * 0.62 * (1.0 - pebble_mask) + pb * pebble_mask).clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            pebble_rgba[idx] = (r * 255.0) as u8;
            pebble_rgba[idx + 1] = (g * 255.0) as u8;
            pebble_rgba[idx + 2] = (b * 255.0) as u8;
            pebble_rgba[idx + 3] = 255;
        }
    }

    // 4. SLGA 6-Layer Soil Horizon & Forest Litter Micro-Relief Texture
    let mut soil_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / (w - 1) as f32;
            let v = y as f32 / (h - 1) as f32;
            let band = 0.85 + 0.15 * (v * 32.0 + (u * 8.0).sin() * 1.4).sin();
            let litter = hash21(x as f32 * 1.3, y as f32 * 1.7) * 0.16;
            let val = (band - 0.08 + litter).clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            soil_rgba[idx] = (val * 248.0) as u8;
            soil_rgba[idx + 1] = (val * 240.0) as u8;
            soil_rgba[idx + 2] = (val * 226.0) as u8;
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

/// Multi-submesh geometry accumulator that tracks submesh index ranges per material.
struct SubmeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl SubmeshBuilder {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn push_vertex(&mut self, pos: [f32; 3], norm: [f32; 3], col: [f32; 4], uv: [f32; 2]) -> u32 {
        let idx = self.positions.len() as u32;
        let len = (norm[0] * norm[0] + norm[1] * norm[1] + norm[2] * norm[2]).sqrt();
        let unit_norm = if len > 1e-6 {
            [norm[0] / len, norm[1] / len, norm[2] / len]
        } else {
            [0.0, 1.0, 0.0]
        };
        self.positions.push(pos);
        self.normals.push(unit_norm);
        self.colors.push(col);
        self.uvs.push(uv);
        idx
    }

    fn push_tri(&mut self, a: u32, b: u32, c: u32) {
        self.indices.extend_from_slice(&[a, b, c]);
    }

    /// Extrude a 3D tube/stem/limb along `axis` using `qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::extrude_profile_along_path`.
    fn add_bishop_swept_tube(
        &mut self,
        axis: &[Point3],
        stations: &[ExtrusionStation],
        radial_sides: usize,
        base_color: [f32; 4],
        top_color: [f32; 4],
    ) {
        let m_sec = radial_sides.max(5);
        let mut cross_section = Vec::with_capacity(m_sec);
        for j in 0..m_sec {
            let theta = std::f64::consts::TAU * (j as f64) / (m_sec as f64);
            cross_section.push(Point2::new(theta.cos(), theta.sin()));
        }
        let n_axis = axis.len();
        let req_v = n_axis * m_sec + 2;
        let req_t = (n_axis - 1) * m_sec * 2 + 2 * m_sec;
        let default_frame = AffineFrame3 {
            origin: Point3::new(0.0, 0.0, 0.0),
            e0: Point3::new(1.0, 0.0, 0.0),
            e1: Point3::new(0.0, 1.0, 0.0),
            e2: Point3::new(0.0, 0.0, 1.0),
        };
        let mut frames = vec![default_frame; n_axis];
        let mut out_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
        let mut out_t = vec![[0u32; 3]; req_t];

        if let Ok((vc, tc)) = extrude_profile_along_path(
            axis,
            &cross_section,
            Some(stations),
            true,
            true,
            &mut frames,
            &mut out_v,
            &mut out_t,
        ) {
            let base_idx = self.positions.len() as u32;
            let min_y = axis.first().map(|p| p.y as f32).unwrap_or(0.0);
            let max_y = axis.last().map(|p| p.y as f32).unwrap_or(1.0);
            let span_y = (max_y - min_y).abs().max(0.01);

            for (vi, p) in out_v.iter().take(vc).enumerate() {
                let ring = (vi / m_sec).min(n_axis - 1);
                let origin = frames[ring].origin;
                let nx = (p.x - origin.x) as f32;
                let ny = (p.y - origin.y) as f32;
                let nz = (p.z - origin.z) as f32;
                let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-5);
                let t = ((p.y as f32 - min_y) / span_y).clamp(0.0, 1.0);
                let col = [
                    base_color[0] * (1.0 - t) + top_color[0] * t,
                    base_color[1] * (1.0 - t) + top_color[1] * t,
                    base_color[2] * (1.0 - t) + top_color[2] * t,
                    1.0,
                ];
                let u = (vi % m_sec) as f32 / m_sec as f32;
                self.push_vertex(
                    [p.x as f32, p.y as f32, p.z as f32],
                    [nx / nlen, ny / nlen, nz / nlen],
                    col,
                    [u, t],
                );
            }
            for tri in out_t.iter().take(tc) {
                self.push_tri(base_idx + tri[0], base_idx + tri[1], base_idx + tri[2]);
            }
        }
    }

    /// Generate a Cescatti 6-point `AsymmetricHull` inner foliage core (`0.46 * rx`) surrounded by
    /// multi-layered 3D phyllotactic falcate leaf blades, wattle flower racemes, and floral spikes!
    fn add_cescatti_canopy_with_leaf_blades(
        &mut self,
        center: [f32; 3],
        rx: f32,
        ry: f32,
        rz: f32,
        leaf_color_a: [f32; 4],
        leaf_color_b: [f32; 4],
        blade_count: usize,
        seed: f32,
    ) {
        let slices = 8usize;
        let stacks = 3usize;
        let (req_v, req_t) = asymmetric_hull_capacity(slices, stacks);
        let mut hull_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
        let mut hull_t = vec![[0u32; 3]; req_t];
        // Compact shaded inner foliage core so individual 3D leaf blades form the lacy outer silhouette
        let core_scale = 0.48_f64;
        let params = AsymmetricHullParams {
            neg_x_radius: (rx as f64) * core_scale * 0.92,
            pos_x_radius: (rx as f64) * core_scale * 1.06,
            neg_y_radius: (rz as f64) * core_scale * 0.95,
            pos_y_radius: (rz as f64) * core_scale * 1.04,
            neg_x_height: (ry as f64) * core_scale * 0.45,
            pos_x_height: (ry as f64) * core_scale * 0.52,
            neg_y_height: (ry as f64) * core_scale * 0.48,
            pos_y_height: (ry as f64) * core_scale * 0.50,
            bottom: Point3::new(0.0, 0.0, (-ry * 0.34) as f64),
            top: Point3::new(0.0, 0.0, (ry * 0.48) as f64),
            bottom_shape: 1.8,
            top_shape: 2.1,
            slices,
            stacks,
        };
        if let Ok((vc, tc)) = asymmetric_hull_into(&params, &mut hull_v, &mut hull_t) {
            let base_idx = self.positions.len() as u32;
            for (vi, p) in hull_v.iter().take(vc).enumerate() {
                let lx = p.x as f32;
                let ly = p.z as f32;
                let lz = p.y as f32;
                let lobe = 1.0
                    + 0.22
                        * (lx * 9.0 + seed).sin()
                        * (ly * 8.0 - seed * 0.7).cos()
                        * (lz * 9.0 + seed * 1.3).sin();
                let wx = center[0] + lx * lobe;
                let wy = center[1] + ly * lobe;
                let wz = center[2] + lz * lobe;
                let nlen = (lx * lx + ly * ly + lz * lz).sqrt().max(1e-5);
                let nx = lx / nlen;
                let ny = ly / nlen;
                let nz = lz / nlen;
                let sun_blend = (0.45_f32 + 0.55_f32 * ny).clamp(0.0_f32, 1.0_f32);
                let col = [
                    leaf_color_a[0] * (1.0 - sun_blend * 0.6) + leaf_color_b[0] * (sun_blend * 0.6),
                    leaf_color_a[1] * (1.0 - sun_blend * 0.6) + leaf_color_b[1] * (sun_blend * 0.6),
                    leaf_color_a[2] * (1.0 - sun_blend * 0.6) + leaf_color_b[2] * (sun_blend * 0.6),
                    1.0,
                ];
                let u = (vi % slices) as f32 / slices as f32;
                let v = sun_blend;
                self.push_vertex([wx, wy, wz], [nx, ny, nz], col, [u, v]);
            }
            for tri in hull_t.iter().take(tc) {
                self.push_tri(base_idx + tri[0], base_idx + tri[1], base_idx + tri[2]);
            }
        }

        // Multi-shell phyllotactic 3D falcate leaf blades (`LeafVascularSkeleton` midrib + lamina)
        // arranged at golden-angle (137.50776 deg) spirals across inner, mid, and outer crown shells!
        let golden_angle = 2.399_963_1_f32;
        let total_blades = (blade_count * 2).max(28);
        for b in 0..total_blades {
            let frac = (b as f32 + 0.5) / (total_blades as f32);
            let phi = b as f32 * golden_angle + seed;
            let y_norm = 1.0 - 2.0 * frac;
            let r_ring = (1.0 - y_norm * y_norm).max(0.08).sqrt();
            let shell = 0.52 + 0.58 * hash21(b as f32 * 1.7, seed);
            let bx = center[0] + phi.cos() * rx * r_ring * shell;
            let by = center[1] + y_norm * ry * shell * 0.92;
            let bz = center[2] + phi.sin() * rz * r_ring * shell;
            let blade_len = rx * (0.42 + 0.24 * hash21(seed, b as f32 * 2.3));
            let blade_w = blade_len * 0.28;
            let sun_t = (0.5_f32 + 0.5_f32 * y_norm).clamp(0.0_f32, 1.0_f32);
            let col_top = [
                leaf_color_a[0] * (1.0 - sun_t) + leaf_color_b[0] * sun_t,
                leaf_color_a[1] * (1.0 - sun_t) + leaf_color_b[1] * sun_t,
                leaf_color_a[2] * (1.0 - sun_t) + leaf_color_b[2] * sun_t,
                1.0,
            ];
            let col_bot = [
                leaf_color_a[0] * 0.82 + leaf_color_b[0] * 0.18,
                leaf_color_a[1] * 0.82 + leaf_color_b[1] * 0.18,
                leaf_color_a[2] * 0.82 + leaf_color_b[2] * 0.18,
                1.0,
            ];
            self.add_falcate_leaf_blade(
                [bx, by, bz],
                phi + 0.6 * (b as f32).sin(),
                blade_len,
                blade_w,
                col_top,
                col_bot,
            );
        }
    }

    /// Add a 3D falcate (sickle-shaped) sclerophyllous leaf blade with raised midrib ridge.
    fn add_falcate_leaf_blade(
        &mut self,
        base: [f32; 3],
        azimuth: f32,
        length: f32,
        width: f32,
        col_adaxial: [f32; 4],
        col_abaxial: [f32; 4],
    ) {
        let ca = azimuth.cos();
        let sa = azimuth.sin();
        // Pendulous sclerophyllous hang: droops downward (-Y) and curves laterally (falcate sickle)
        let p_base = base;
        let p_mid_l = [
            base[0] + ca * length * 0.38 - sa * width * 0.62,
            base[1] - length * 0.44,
            base[2] + sa * length * 0.38 + ca * width * 0.62,
        ];
        let p_mid_c = [
            base[0] + ca * length * 0.42,
            base[1] - length * 0.38 + width * 0.24,
            base[2] + sa * length * 0.42,
        ];
        let p_mid_r = [
            base[0] + ca * length * 0.38 + sa * width * 0.62,
            base[1] - length * 0.44,
            base[2] + sa * length * 0.38 - ca * width * 0.62,
        ];
        let p_tip = [
            base[0] + ca * length * 0.74 + sa * width * 0.42,
            base[1] - length * 0.94,
            base[2] + sa * length * 0.74 - ca * width * 0.42,
        ];
        let n_l = [ca * 0.35 - sa * 0.45, 0.72, sa * 0.35 + ca * 0.45];
        let n_c = [ca * 0.45, 0.84, sa * 0.45];
        let n_r = [ca * 0.35 + sa * 0.45, 0.72, sa * 0.35 - ca * 0.45];
        let v0 = self.push_vertex(p_base, n_c, col_adaxial, [0.5, 0.0]);
        let v1 = self.push_vertex(p_mid_l, n_l, col_abaxial, [0.0, 0.5]);
        let v2 = self.push_vertex(p_mid_c, n_c, col_adaxial, [0.5, 0.5]);
        let v3 = self.push_vertex(p_mid_r, n_r, col_abaxial, [1.0, 0.5]);
        let v4 = self.push_vertex(p_tip, n_c, col_adaxial, [0.5, 1.0]);
        self.push_tri(v0, v1, v2);
        self.push_tri(v0, v2, v3);
        self.push_tri(v1, v4, v2);
        self.push_tri(v2, v4, v3);
    }

    /// Add a 3D faceted boulder, granite tor, columnar basalt prism, or riverbed pebble/cobble.
    fn add_rock_or_pebble(
        &mut self,
        center: [f32; 3],
        radii: [f32; 3],
        yaw: f32,
        col_base: [f32; 4],
        col_top: [f32; 4],
        seed: f32,
    ) {
        let slices = 7usize;
        let stacks = 3usize;
        let (req_v, req_t) = asymmetric_hull_capacity(slices, stacks);
        let mut hull_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
        let mut hull_t = vec![[0u32; 3]; req_t];
        let params = AsymmetricHullParams {
            neg_x_radius: (radii[0] * 0.95) as f64,
            pos_x_radius: (radii[0] * 1.05) as f64,
            neg_y_radius: (radii[2] * 0.92) as f64,
            pos_y_radius: (radii[2] * 1.08) as f64,
            neg_x_height: (radii[1] * 0.38) as f64,
            pos_x_height: (radii[1] * 0.44) as f64,
            neg_y_height: (radii[1] * 0.40) as f64,
            pos_y_height: (radii[1] * 0.42) as f64,
            bottom: Point3::new(0.0, 0.0, (-radii[1] * 0.65) as f64),
            top: Point3::new(0.0, 0.0, (radii[1] * 0.85) as f64),
            bottom_shape: 1.7,
            top_shape: 1.6,
            slices,
            stacks,
        };
        if let Ok((vc, tc)) = asymmetric_hull_into(&params, &mut hull_v, &mut hull_t) {
            let base_idx = self.positions.len() as u32;
            let cy = yaw.cos();
            let sy = yaw.sin();
            for (vi, p) in hull_v.iter().take(vc).enumerate() {
                let lx = p.x as f32;
                let ly = p.z as f32;
                let lz = p.y as f32;
                let facet = 0.92 + 0.16 * hash21(vi as f32 + seed, seed * 2.3);
                let rx = (lx * cy - lz * sy) * facet;
                let ry = ly * facet;
                let rz = (lx * sy + lz * cy) * facet;
                let nlen = (rx * rx + ry * ry + rz * rz).sqrt().max(1e-5);
                let t = (0.5_f32 + 0.5_f32 * (ry / nlen)).clamp(0.0_f32, 1.0_f32);
                let col = [
                    col_base[0] * (1.0 - t) + col_top[0] * t,
                    col_base[1] * (1.0 - t) + col_top[1] * t,
                    col_base[2] * (1.0 - t) + col_top[2] * t,
                    1.0,
                ];
                let u = (vi % slices) as f32 / slices as f32;
                self.push_vertex(
                    [center[0] + rx, center[1] + ry, center[2] + rz],
                    [rx / nlen, ry / nlen, rz / nlen],
                    col,
                    [u, t],
                );
            }
            for tri in out_slice(&hull_t, tc) {
                self.push_tri(base_idx + tri[0], base_idx + tri[1], base_idx + tri[2]);
            }
        }
    }

    /// Add a 3D Cosserat Cubic Bézier Tussock Grass / Sedge / Submerged *Vallisneria* Ribbon Weed tuft.
    fn add_cosserat_grass_tuft(
        &mut self,
        root: [f32; 3],
        blade_count: usize,
        height: f32,
        spread: f32,
        col_base: [f32; 4],
        col_tip: [f32; 4],
        seed: f32,
    ) {
        for i in 0..blade_count {
            let angle = (i as f32 / blade_count.max(1) as f32) * std::f32::consts::TAU
                + hash21(seed, i as f32) * 0.45;
            let ca = angle.cos();
            let sa = angle.sin();
            let px = -sa;
            let pz = ca;
            let h = height * (0.72 + 0.45 * hash21(i as f32 * 3.1, seed));
            let droop = spread * (0.65 + 0.55 * hash21(seed * 1.9, i as f32));
            let w_base = (height * 0.085).max(0.008);
            let w_mid = w_base * 0.65;

            let p0_l = [root[0] - px * w_base, root[1], root[2] - pz * w_base];
            let p0_r = [root[0] + px * w_base, root[1], root[2] + pz * w_base];
            let p1_l = [
                root[0] + ca * droop * 0.42 - px * w_mid,
                root[1] + h * 0.58,
                root[2] + sa * droop * 0.42 - pz * w_mid,
            ];
            let p1_r = [
                root[0] + ca * droop * 0.42 + px * w_mid,
                root[1] + h * 0.58,
                root[2] + sa * droop * 0.42 + pz * w_mid,
            ];
            let p2_tip = [
                root[0] + ca * droop,
                root[1] + h * 0.92,
                root[2] + sa * droop,
            ];
            let norm = [ca * 0.35, 0.88, sa * 0.35];
            let col_mid = [
                0.5 * (col_base[0] + col_tip[0]),
                0.5 * (col_base[1] + col_tip[1]),
                0.5 * (col_base[2] + col_tip[2]),
                1.0,
            ];
            let v0 = self.push_vertex(p0_l, norm, col_base, [0.0, 0.0]);
            let v1 = self.push_vertex(p0_r, norm, col_base, [1.0, 0.0]);
            let v2 = self.push_vertex(p1_l, norm, col_mid, [0.2, 0.55]);
            let v3 = self.push_vertex(p1_r, norm, col_mid, [0.8, 0.55]);
            let v4 = self.push_vertex(p2_tip, norm, col_tip, [0.5, 1.0]);
            self.push_tri(v0, v1, v3);
            self.push_tri(v0, v3, v2);
            self.push_tri(v2, v3, v4);
        }
    }

    /// Add a 3D *Dicksonia antarctica* Tree Fern or *Xanthorrhoea australis* Grass Tree.
    fn add_tree_fern_or_grass_tree(
        &mut self,
        trunk_builder: &mut SubmeshBuilder,
        root: [f32; 3],
        height: f32,
        is_grass_tree: bool,
        seed: f32,
    ) {
        // Fibrous caudex / charred fire-trunk
        let trunk_axis = [
            Point3::new(root[0] as f64, root[1] as f64, root[2] as f64),
            Point3::new(
                (root[0] + 0.015 * seed.sin()) as f64,
                (root[1] + height * 0.5) as f64,
                (root[2] + 0.015 * seed.cos()) as f64,
            ),
            Point3::new(root[0] as f64, (root[1] + height) as f64, root[2] as f64),
        ];
        let r0 = (height * 0.14).max(0.018) as f64;
        let stations = [
            ExtrusionStation {
                scale_x: r0 * 1.15,
                scale_y: r0 * 1.15,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: r0 * 0.95,
                scale_y: r0 * 0.95,
                twist_rad: 0.25,
            },
            ExtrusionStation {
                scale_x: r0 * 0.82,
                scale_y: r0 * 0.82,
                twist_rad: 0.5,
            },
        ];
        let trunk_col = if is_grass_tree {
            [0.14, 0.12, 0.10, 1.0] // fire-charred caudex
        } else {
            [0.32, 0.22, 0.15, 1.0] // fibrous tree-fern caudex
        };
        trunk_builder.add_bishop_swept_tube(&trunk_axis, &stations, 6, trunk_col, trunk_col);

        let crown = [root[0], root[1] + height, root[2]];
        if is_grass_tree {
            // Dense drooping needle skirt + tall flowering spike scape
            self.add_cosserat_grass_tuft(
                crown,
                22,
                height * 0.85,
                height * 0.95,
                [0.24, 0.46, 0.18, 1.0],
                [0.48, 0.66, 0.26, 1.0],
                seed,
            );
            let scape_axis = [
                Point3::new(crown[0] as f64, crown[1] as f64, crown[2] as f64),
                Point3::new(
                    crown[0] as f64,
                    (crown[1] + height * 1.35) as f64,
                    crown[2] as f64,
                ),
            ];
            let scape_st = [
                ExtrusionStation {
                    scale_x: r0 * 0.32,
                    scale_y: r0 * 0.32,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: r0 * 0.22,
                    scale_y: r0 * 0.22,
                    twist_rad: 0.0,
                },
            ];
            trunk_builder.add_bishop_swept_tube(
                &scape_axis,
                &scape_st,
                5,
                [0.52, 0.42, 0.24, 1.0],
                [0.82, 0.76, 0.48, 1.0],
            );
        } else {
            // 12 arching bipinnate fern fronds with lateral pinnae
            let frond_count = 12usize;
            for f in 0..frond_count {
                let ang = (f as f32 / frond_count as f32) * std::f32::consts::TAU + seed * 0.3;
                let ca = ang.cos();
                let sa = ang.sin();
                let frond_len = height * 1.18;
                for seg in 0..6 {
                    let t0 = seg as f32 / 6.0;
                    let t1 = (seg + 1) as f32 / 6.0;
                    let r_a = frond_len * t0;
                    let r_b = frond_len * t1;
                    let y_a = crown[1] + (t0 * std::f32::consts::PI * 0.85).sin() * height * 0.38
                        - t0 * t0 * height * 0.22;
                    let y_b = crown[1] + (t1 * std::f32::consts::PI * 0.85).sin() * height * 0.38
                        - t1 * t1 * height * 0.22;
                    let pinna_w = frond_len * 0.22 * (1.0 - (t0 - 0.35).abs() * 1.2).max(0.15);
                    let px = -sa * pinna_w;
                    let pz = ca * pinna_w;
                    let p_c0 = [crown[0] + ca * r_a, y_a, crown[2] + sa * r_a];
                    let p_c1 = [crown[0] + ca * r_b, y_b, crown[2] + sa * r_b];
                    let p_l = [
                        0.5 * (p_c0[0] + p_c1[0]) - px,
                        0.5 * (y_a + y_b) - pinna_w * 0.15,
                        0.5 * (p_c0[2] + p_c1[2]) - pz,
                    ];
                    let p_r = [
                        0.5 * (p_c0[0] + p_c1[0]) + px,
                        0.5 * (y_a + y_b) - pinna_w * 0.15,
                        0.5 * (p_c0[2] + p_c1[2]) + pz,
                    ];
                    let n_up = [ca * 0.25, 0.94, sa * 0.25];
                    let col_in = [0.18, 0.48, 0.16, 1.0];
                    let col_out = [0.34, 0.68, 0.22, 1.0];
                    let v0 = self.push_vertex(p_c0, n_up, col_in, [0.5, t0]);
                    let v1 = self.push_vertex(p_c1, n_up, col_out, [0.5, t1]);
                    let v2 = self.push_vertex(p_l, n_up, col_out, [0.0, 0.5 * (t0 + t1)]);
                    let v3 = self.push_vertex(p_r, n_up, col_out, [1.0, 0.5 * (t0 + t1)]);
                    self.push_tri(v0, v2, v1);
                    self.push_tri(v0, v1, v3);
                }
            }
        }
    }

    /// Add an articulated 3D Australian canopy tree (*Corymbia aparrerinja*, *Eucalyptus pauciflora*,
    /// *Eucalyptus regnans*, *Eucalyptus camaldulensis*, *Acacia pycnantha*, *Banksia serrata*)
    /// with Bishop-frame swept trunk, primary limbs, secondary fork twigs, Cescatti cores, and 3D falcate leaf blades!
    fn add_canopy_tree(
        &mut self,
        trunk_builder: &mut SubmeshBuilder,
        root: [f32; 3],
        height: f32,
        bark_base: [f32; 4],
        bark_top: [f32; 4],
        leaf_dark: [f32; 4],
        leaf_lit: [f32; 4],
        contorted: bool,
        seed: f32,
    ) {
        let bend_x = if contorted {
            height * 0.20 * seed.sin()
        } else {
            height * 0.06 * seed.sin()
        };
        let bend_z = if contorted {
            height * 0.18 * seed.cos()
        } else {
            height * 0.06 * seed.cos()
        };
        let trunk_axis = [
            Point3::new(root[0] as f64, (root[1] - 0.015) as f64, root[2] as f64),
            Point3::new(
                (root[0] + bend_x * 0.35) as f64,
                (root[1] + height * 0.26) as f64,
                (root[2] + bend_z * 0.35) as f64,
            ),
            Point3::new(
                (root[0] + bend_x * 0.82) as f64,
                (root[1] + height * 0.54) as f64,
                (root[2] + bend_z * 0.82) as f64,
            ),
            Point3::new(
                (root[0] + bend_x) as f64,
                (root[1] + height * 0.74) as f64,
                (root[2] + bend_z) as f64,
            ),
        ];
        let r_base = (height * 0.066).max(0.012) as f64;
        let trunk_stations = [
            ExtrusionStation {
                scale_x: r_base * 1.38,
                scale_y: r_base * 1.25,
                twist_rad: 0.0,
            },
            ExtrusionStation {
                scale_x: r_base * 0.90,
                scale_y: r_base * 0.88,
                twist_rad: 0.35,
            },
            ExtrusionStation {
                scale_x: r_base * 0.64,
                scale_y: r_base * 0.64,
                twist_rad: 0.72,
            },
            ExtrusionStation {
                scale_x: r_base * 0.38,
                scale_y: r_base * 0.38,
                twist_rad: 1.05,
            },
        ];
        trunk_builder.add_bishop_swept_tube(&trunk_axis, &trunk_stations, 7, bark_base, bark_top);

        let fork = [
            root[0] + bend_x * 0.85,
            root[1] + height * 0.58,
            root[2] + bend_z * 0.85,
        ];
        // 5 spreading primary boughs + secondary forked branchlets + lacy 3D falcate leaf clusters
        let bough_count = 5usize;
        for b in 0..bough_count {
            let ang =
                (b as f32 / bough_count as f32) * std::f32::consts::TAU + seed * 1.17 + 0.4;
            let ca = ang.cos();
            let sa = ang.sin();
            let reach = height * (0.36 + 0.09 * ((b as f32 + seed).sin()));
            let mid = [
                fork[0] + ca * reach * 0.55,
                fork[1] + height * 0.16,
                fork[2] + sa * reach * 0.55,
            ];
            let tip = [
                fork[0] + ca * reach,
                root[1] + height * (0.86 + 0.08 * (b as f32 * 1.7).cos()),
                fork[2] + sa * reach,
            ];
            let bough_axis = [
                Point3::new(fork[0] as f64, fork[1] as f64, fork[2] as f64),
                Point3::new(mid[0] as f64, mid[1] as f64, mid[2] as f64),
                Point3::new(tip[0] as f64, tip[1] as f64, tip[2] as f64),
            ];
            let bough_st = [
                ExtrusionStation {
                    scale_x: r_base * 0.46,
                    scale_y: r_base * 0.46,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: r_base * 0.28,
                    scale_y: r_base * 0.28,
                    twist_rad: 0.4,
                },
                ExtrusionStation {
                    scale_x: r_base * 0.14,
                    scale_y: r_base * 0.14,
                    twist_rad: 0.8,
                },
            ];
            trunk_builder.add_bishop_swept_tube(&bough_axis, &bough_st, 6, bark_top, bark_top);

            // Secondary lateral twig fork off `mid`
            let sub_ang = ang + 0.65 * (if b % 2 == 0 { 1.0 } else { -1.0 });
            let sub_tip = [
                mid[0] + sub_ang.cos() * reach * 0.48,
                mid[1] + height * 0.12,
                mid[2] + sub_ang.sin() * reach * 0.48,
            ];
            let twig_axis = [
                Point3::new(mid[0] as f64, mid[1] as f64, mid[2] as f64),
                Point3::new(sub_tip[0] as f64, sub_tip[1] as f64, sub_tip[2] as f64),
            ];
            let twig_st = [
                ExtrusionStation {
                    scale_x: r_base * 0.22,
                    scale_y: r_base * 0.22,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: r_base * 0.09,
                    scale_y: r_base * 0.09,
                    twist_rad: 0.3,
                },
            ];
            trunk_builder.add_bishop_swept_tube(&twig_axis, &twig_st, 5, bark_top, bark_top);

            let crown_rx = height * 0.24;
            let crown_ry = height * 0.16;
            self.add_cescatti_canopy_with_leaf_blades(
                tip,
                crown_rx,
                crown_ry,
                crown_rx * 0.94,
                leaf_dark,
                leaf_lit,
                20,
                seed + b as f32 * 2.1,
            );
            self.add_cescatti_canopy_with_leaf_blades(
                sub_tip,
                crown_rx * 0.72,
                crown_ry * 0.72,
                crown_rx * 0.70,
                leaf_dark,
                leaf_lit,
                14,
                seed + b as f32 * 3.7 + 1.1,
            );
        }

        // Apical crown dome with dense falcate leaf blades
        let apex = [
            root[0] + bend_x,
            root[1] + height * 0.96,
            root[2] + bend_z,
        ];
        self.add_cescatti_canopy_with_leaf_blades(
            apex,
            height * 0.28,
            height * 0.18,
            height * 0.27,
            leaf_dark,
            leaf_lit,
            26,
            seed + 9.4,
        );
    }

    /// Add an articulated 3D Australian Fauna Specimen (*Macropus giganteus* Kangaroo,
    /// *Vombatus ursinus* Wombat, *Ornithorhynchus anatinus* Platypus, *Aquila audax* Eagle,
    /// *Menura novaehollandiae* Lyrebird, or *Maccullochella peelii* Murray Cod).
    fn add_fauna_specimen(
        &mut self,
        pos: [f32; 3],
        yaw: f32,
        scale: f32,
        archetype: &str,
        coat_dark: [f32; 4],
        coat_lit: [f32; 4],
    ) {
        let cy = yaw.cos();
        let sy = yaw.sin();
        let to_world = |lx: f32, ly: f32, lz: f32| -> Point3 {
            Point3::new(
                (pos[0] + (lx * cy - lz * sy) * scale) as f64,
                (pos[1] + ly * scale) as f64,
                (pos[2] + (lx * sy + lz * cy) * scale) as f64,
            )
        };
        let to_world_f32 = |lx: f32, ly: f32, lz: f32| -> [f32; 3] {
            [
                pos[0] + (lx * cy - lz * sy) * scale,
                pos[1] + ly * scale,
                pos[2] + (lx * sy + lz * cy) * scale,
            ]
        };

        if archetype == "macropod" {
            // Sculpted Kangaroo spine from heavy counterbalance tail tip -> muscular pelvis -> chest -> neck -> muzzle
            let spine = [
                to_world(0.0, 0.04, -0.95), // tail tip
                to_world(0.0, 0.12, -0.52), // tail base
                to_world(0.0, 0.38, -0.18), // muscular pelvis / haunch
                to_world(0.0, 0.68, 0.12),  // torso / chest
                to_world(0.0, 0.96, 0.26),  // neck / head
                to_world(0.0, 0.98, 0.44),  // elongated muzzle
            ];
            let s = scale as f64;
            let spine_st = [
                ExtrusionStation {
                    scale_x: 0.035 * s,
                    scale_y: 0.035 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.085 * s,
                    scale_y: 0.085 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.21 * s,
                    scale_y: 0.23 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.15 * s,
                    scale_y: 0.16 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.10 * s,
                    scale_y: 0.11 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.055 * s,
                    scale_y: 0.055 * s,
                    twist_rad: 0.0,
                },
            ];
            self.add_bishop_swept_tube(&spine, &spine_st, 8, coat_dark, coat_lit);

            // Syndactylous hind legs, forepaws, and rotatable pinnae ears
            for side in [-1.0_f32, 1.0_f32] {
                let leg = [
                    to_world(side * 0.14, 0.38, -0.14),
                    to_world(side * 0.15, 0.16, -0.24),
                    to_world(side * 0.14, 0.02, 0.08),
                ];
                let leg_st = [
                    ExtrusionStation {
                        scale_x: 0.095 * s,
                        scale_y: 0.11 * s,
                        twist_rad: 0.0,
                    },
                    ExtrusionStation {
                        scale_x: 0.055 * s,
                        scale_y: 0.065 * s,
                        twist_rad: 0.0,
                    },
                    ExtrusionStation {
                        scale_x: 0.045 * s,
                        scale_y: 0.035 * s,
                        twist_rad: 0.0,
                    },
                ];
                self.add_bishop_swept_tube(&leg, &leg_st, 6, coat_dark, coat_lit);

                let arm = [
                    to_world(side * 0.11, 0.64, 0.12),
                    to_world(side * 0.10, 0.42, 0.24),
                ];
                let arm_st = [
                    ExtrusionStation {
                        scale_x: 0.042 * s,
                        scale_y: 0.042 * s,
                        twist_rad: 0.0,
                    },
                    ExtrusionStation {
                        scale_x: 0.024 * s,
                        scale_y: 0.024 * s,
                        twist_rad: 0.0,
                    },
                ];
                self.add_bishop_swept_tube(&arm, &arm_st, 5, coat_dark, coat_lit);

                let ear = [
                    to_world(side * 0.06, 1.02, 0.24),
                    to_world(side * 0.11, 1.24, 0.22),
                ];
                let ear_st = [
                    ExtrusionStation {
                        scale_x: 0.035 * s,
                        scale_y: 0.025 * s,
                        twist_rad: 0.0,
                    },
                    ExtrusionStation {
                        scale_x: 0.015 * s,
                        scale_y: 0.012 * s,
                        twist_rad: 0.0,
                    },
                ];
                self.add_bishop_swept_tube(&ear, &ear_st, 5, coat_dark, coat_lit);
            }
        } else if archetype == "raptor" {
            // Wedge-tailed eagle / Sulphur-crested cockatoo with thin cambered airfoil wings,
            // fanned primary wingtip feathers, and diamond/wedge tail fan!
            let s = scale as f64;
            let body = [
                to_world(0.0, 0.0, -0.38),
                to_world(0.0, 0.02, 0.0),
                to_world(0.0, 0.01, 0.34),
            ];
            let body_st = [
                ExtrusionStation {
                    scale_x: 0.07 * s,
                    scale_y: 0.05 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.11 * s,
                    scale_y: 0.09 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.045 * s,
                    scale_y: 0.045 * s,
                    twist_rad: 0.0,
                },
            ];
            self.add_bishop_swept_tube(&body, &body_st, 6, coat_dark, coat_lit);

            // Cambered airfoil wings + fanned primary flight feathers on both sides
            let n_up = [0.0, 1.0, 0.0];
            for side in [-1.0_f32, 1.0_f32] {
                let p_shoulder = to_world_f32(side * 0.08, 0.03, 0.08);
                let p_wrist = to_world_f32(side * 0.56, 0.09, 0.14);
                let p_elbow_te = to_world_f32(side * 0.44, 0.05, -0.14);
                let p_root_te = to_world_f32(side * 0.08, 0.01, -0.18);
                let p_tip = to_world_f32(side * 1.12, 0.16, 0.02);
                let p_tip_te = to_world_f32(side * 0.92, 0.11, -0.16);

                let v0 = self.push_vertex(p_shoulder, n_up, coat_lit, [0.0, 0.0]);
                let v1 = self.push_vertex(p_wrist, n_up, coat_lit, [0.5, 0.0]);
                let v2 = self.push_vertex(p_elbow_te, n_up, coat_dark, [0.5, 1.0]);
                let v3 = self.push_vertex(p_root_te, n_up, coat_dark, [0.0, 1.0]);
                let v4 = self.push_vertex(p_tip, n_up, coat_lit, [1.0, 0.2]);
                let v5 = self.push_vertex(p_tip_te, n_up, coat_dark, [0.9, 0.8]);
                if side > 0.0 {
                    self.push_tri(v0, v1, v2);
                    self.push_tri(v0, v2, v3);
                    self.push_tri(v1, v4, v5);
                    self.push_tri(v1, v5, v2);
                } else {
                    self.push_tri(v0, v2, v1);
                    self.push_tri(v0, v3, v2);
                    self.push_tri(v1, v5, v4);
                    self.push_tri(v1, v2, v5);
                }
            }
            // Wedge-shaped tail fan (`Aquila audax`)
            let t_root = to_world_f32(0.0, 0.01, -0.26);
            let t_l = to_world_f32(-0.22, -0.02, -0.58);
            let t_c = to_world_f32(0.0, -0.03, -0.72);
            let t_r = to_world_f32(0.22, -0.02, -0.58);
            let vt0 = self.push_vertex(t_root, n_up, coat_dark, [0.5, 0.0]);
            let vt1 = self.push_vertex(t_l, n_up, coat_dark, [0.0, 0.8]);
            let vt2 = self.push_vertex(t_c, n_up, coat_lit, [0.5, 1.0]);
            let vt3 = self.push_vertex(t_r, n_up, coat_dark, [1.0, 0.8]);
            self.push_tri(vt0, vt1, vt2);
            self.push_tri(vt0, vt2, vt3);
        } else {
            // Wombat / Platypus / Murray Cod / Corroboree Frog streamlined body + limbs/fins
            let body = [
                to_world(0.0, 0.14, -0.48),
                to_world(0.0, 0.22, 0.0),
                to_world(0.0, 0.18, 0.45),
            ];
            let s = scale as f64;
            let body_st = [
                ExtrusionStation {
                    scale_x: 0.14 * s,
                    scale_y: 0.10 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.24 * s,
                    scale_y: 0.20 * s,
                    twist_rad: 0.0,
                },
                ExtrusionStation {
                    scale_x: 0.15 * s,
                    scale_y: 0.11 * s,
                    twist_rad: 0.0,
                },
            ];
            self.add_bishop_swept_tube(&body, &body_st, 7, coat_dark, coat_lit);
        }
    }
}

#[inline]
fn out_slice<T>(slice: &[T], len: usize) -> &[T] {
    &slice[..len.min(slice.len())]
}

/// Evaluate multi-scale East Coast terrain elevation in PortalGpu world coordinates `[-2.2, 2.2]`.
/// Includes river gorge channel, point-bar shingle beaches, bush track bench, and mountain ridges.
pub fn sample_sector_elevation(sector_id: &str, x: f32, z: f32, water_y: f32) -> f32 {
    let seed = match sector_id {
        "talbingo" => 1.1_f32,
        "yarrangobilly" => 2.4_f32,
        "three_mile_dam" => 3.7_f32,
        "sydney_sandstone" => 5.1_f32,
        "noosa_headland" => 6.5_f32,
        _ => 7.9_f32,
    };

    // Meandering river thalweg across z
    let meander_x = 0.28 * (z * 1.35 + seed).sin() + 0.11 * (z * 2.7 - seed * 0.6).cos();
    let dist_river = (x - meander_x).abs();

    // River channel carving below water_y so riverbed cobbles/pebbles/sand are visible underwater
    let channel_depth = 0.36 * (-((dist_river / 0.42).powi(2))).exp();
    // Valley walls rising toward mountain ridges / sandstone escarpments / karst cliffs
    let valley_rise = match sector_id {
        "sydney_sandstone" => {
            // Stepped Hawkesbury sandstone cliff benches
            let raw = (dist_river - 0.32).max(0.0) * 0.72;
            raw + 0.08 * (raw * 14.0).sin()
        }
        "noosa_headland" => {
            // Coastal headland on +x, open turquoise estuary/ocean on -x
            if x < meander_x {
                -0.16 * (meander_x - x).min(1.2)
            } else {
                (x - meander_x).powf(1.25) * 0.55
            }
        }
        "yarrangobilly" | "jounama_karst" => (dist_river - 0.26).max(0.0).powf(1.35) * 0.82,
        "three_mile_dam" => (dist_river - 0.38).max(0.0).powf(1.15) * 0.44,
        _ => (dist_river - 0.30).max(0.0).powf(1.25) * 0.68,
    };

    let ridge_undulation = 0.14 * (x * 2.1 + seed).sin() * (z * 1.9 - seed * 0.8).cos()
        + 0.06 * (x * 4.7 - z * 3.9 + seed).sin();

    // Bush track bench along the right bank (`dist_river ~ 0.52`)
    let track_dist = (x - (meander_x + 0.54)).abs();
    let track_flatten = (-((track_dist / 0.09).powi(2))).exp();

    let raw_y = water_y + 0.06 - channel_depth + valley_rise + ridge_undulation * (0.35 + 0.65 * (dist_river / 1.5).min(1.0));
    let track_y = water_y + 0.16;
    raw_y * (1.0 - 0.65 * track_flatten) + track_y * (0.65 * track_flatten)
}

/// Build and seal a complete `.10d` sector scene + `.10d` water surface for an East Coast sector.
pub fn build_sector_10d(
    sector_id: &'static str,
    title: &'static str,
    textures: &[ProceduralTextureAsset],
    include_cutaway_wall: bool,
) -> Result<CompiledSector10d, String> {
    let tex_leaf = textures[0].sha256_bytes;
    let tex_bark = textures[1].sha256_bytes;
    let tex_pebble = textures[2].sha256_bytes;
    let tex_soil = textures[3].sha256_bytes;

    let (water_y, water_color, sun_dir, cam_yaw, cam_pitch, cam_zoom, cam_target, sky_preset) =
        match sector_id {
            "talbingo" => (
                -0.22_f32,
                [0.06, 0.32, 0.44],
                [0.45, 0.76, 0.48],
                0.42,
                0.36,
                2.55,
                [0.0, -0.02, 0.0],
                0u32,
            ),
            "yarrangobilly" => (
                -0.24_f32,
                [0.06, 0.46, 0.50],
                [-0.22, 0.78, 0.58],
                0.08,
                0.24,
                2.32,
                [0.0, -0.22, 0.15],
                0u32,
            ),
            "three_mile_dam" => (
                -0.20_f32,
                [0.05, 0.42, 0.58],
                [0.48, 0.72, 0.50],
                0.38,
                0.30,
                2.25,
                [0.0, -0.02, 0.0],
                1u32,
            ),
            "sydney_sandstone" => (
                -0.25_f32,
                [0.09, 0.34, 0.39],
                [0.40, 0.80, 0.45],
                -0.32,
                0.35,
                2.50,
                [0.0, 0.0, 0.0],
                0u32,
            ),
            "noosa_headland" => (
                -0.18_f32,
                [0.05, 0.46, 0.56],
                [0.32, 0.84, 0.44],
                0.26,
                0.30,
                2.40,
                [0.0, -0.02, 0.0],
                0u32,
            ),
            _ => (
                -0.23_f32,
                [0.06, 0.34, 0.46],
                [-0.44, 0.74, 0.51],
                0.18,
                0.40,
                2.45,
                [0.0, -0.02, 0.0],
                2u32,
            ),
        };

    // 5 Submesh Builders mapped to 5 distinct MAT4 MaterialRecords:
    // 0: Terrain & 6-Layer SLGA Soil Strata (`tex_soil`)
    // 1: 3D Lithology (Granite Tors, Sandstone Benches, Karst) & 3D Submerged Riverbed Pebbles/Cobbles (`tex_pebble`)
    // 2: Tree Trunks, Buttresses, Roots & Fallen Hollow Logs (`tex_bark`)
    // 3: 3D Foliage Crowns, Falcate Leaf Blades, Tree Ferns, Grass Trees, Cosserat Tussock Grasses & Submerged Ribbon Weed (`tex_leaf`, `wind_animated: true`, `double_sided: true`)
    // 4: Articulated 3D Australian Fauna (`tex_bark`)
    let mut b_terrain = SubmeshBuilder::new();
    let mut b_lithology = SubmeshBuilder::new();
    let mut b_trunks = SubmeshBuilder::new();
    let mut b_foliage = SubmeshBuilder::new();
    let mut b_fauna = SubmeshBuilder::new();

    // 1. Build Multi-Resolution Terrain Continuum Grid (`[-1.95, z_max]`)
    let grid_n = 72usize;
    let extent = 1.95_f32;
    let z_max = if include_cutaway_wall { 0.55_f32 } else { extent };
    let mut grid_idx = vec![0u32; (grid_n + 1) * (grid_n + 1)];

    for gz in 0..=grid_n {
        for gx in 0..=grid_n {
            let u = gx as f32 / grid_n as f32;
            let v = gz as f32 / grid_n as f32;
            let x = (u * 2.0 - 1.0) * extent;
            let z = -extent + v * (z_max + extent);
            let y = sample_sector_elevation(sector_id, x, z, water_y);

            let eps = 0.025_f32;
            let y_dx = sample_sector_elevation(sector_id, x + eps, z, water_y)
                - sample_sector_elevation(sector_id, x - eps, z, water_y);
            let y_dz = sample_sector_elevation(sector_id, x, z + eps, water_y)
                - sample_sector_elevation(sector_id, x, z - eps, water_y);
            let nx = -y_dx;
            let ny = 2.0 * eps;
            let nz = -y_dz;
            let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-5);
            let norm = [nx / nlen, ny / nlen, nz / nlen];
            let slope = 1.0 - norm[1];

            // Color by bathymetry / shoreline shingle / riparian / bush track / forest loam / exposed rock
            let dist_water = y - water_y;
            let col = if dist_water < -0.02 {
                // Submerged riverbed cobble armor & quartz sand
                let pebbles = 0.85 + 0.15 * hash21(x * 19.0, z * 19.0);
                [0.44 * pebbles, 0.42 * pebbles, 0.34 * pebbles, 1.0]
            } else if dist_water < 0.055 {
                // Shingle pebble point-bar & golden alluvial sand beach
                let sand = 0.88 + 0.12 * hash21(x * 23.0, z * 23.0);
                [0.68 * sand, 0.60 * sand, 0.46 * sand, 1.0]
            } else if slope > 0.32 {
                // Exposed Hawkesbury sandstone / Silurian karst limestone / Bogong granite cliff
                match sector_id {
                    "sydney_sandstone" => [0.68, 0.52, 0.36, 1.0],
                    "yarrangobilly" | "jounama_karst" => [0.58, 0.60, 0.63, 1.0],
                    _ => [0.52, 0.50, 0.48, 1.0],
                }
            } else {
                // Rich forest biomantle loam & eucalypt leaf litter + green mossy banks
                let moisture = (-((dist_water / 0.25).powi(2))).exp();
                let r = 0.26 * (1.0 - moisture) + 0.20 * moisture;
                let g = 0.38 * (1.0 - moisture) + 0.44 * moisture;
                let b = 0.19 * (1.0 - moisture) + 0.18 * moisture;
                [r, g, b, 1.0]
            };

            let vi = b_terrain.push_vertex([x, y, z], norm, col, [u * 6.0, v * 6.0]);
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

    // Optional 6-Layer CSIRO SLGA Subsurface Geological Cutaway Cross-Section (`include_cutaway_wall`)
    if include_cutaway_wall {
        let strata_colors: [[f32; 4]; 6] = [
            [0.20, 0.16, 0.12, 1.0], // L0 O-Horizon Dark Alpine Humus & Peat
            [0.38, 0.27, 0.18, 1.0], // L1 A1-Horizon Organic Biomantle Loam
            [0.78, 0.70, 0.54, 1.0], // L2 A2-Horizon Bleached Eluvial Quartz Sand
            [0.82, 0.34, 0.16, 1.0], // L3 B2-Horizon Red-Ochre Illuvial Ferrosol Clay
            [0.64, 0.56, 0.40, 1.0], // L4 C-Horizon Granitic Saprolite Grus
            [0.46, 0.50, 0.58, 1.0], // L5 R-Horizon Banded Silurian Karst Limestone Bedrock
        ];
        let base_y = -0.96_f32;
        for gx in 0..grid_n {
            let u0 = gx as f32 / grid_n as f32;
            let u1 = (gx + 1) as f32 / grid_n as f32;
            let x0 = (u0 * 2.0 - 1.0) * extent;
            let x1 = (u1 * 2.0 - 1.0) * extent;
            let top0 = sample_sector_elevation(sector_id, x0, z_max, water_y);
            let top1 = sample_sector_elevation(sector_id, x1, z_max, water_y);
            for layer in 0..6 {
                let f0 = layer as f32 / 6.0;
                let f1 = (layer + 1) as f32 / 6.0;
                let wave0 = 0.012 * (x0 * 5.5 + layer as f32 * 1.3).sin();
                let wave1 = 0.012 * (x1 * 5.5 + layer as f32 * 1.3).sin();
                let y0_a = top0 * (1.0 - f0) + base_y * f0 + wave0 * (1.0 - f0);
                let y0_b = top0 * (1.0 - f1) + base_y * f1 + wave0 * (1.0 - f1);
                let y1_a = top1 * (1.0 - f0) + base_y * f0 + wave1 * (1.0 - f0);
                let y1_b = top1 * (1.0 - f1) + base_y * f1 + wave1 * (1.0 - f1);
                let z_step_a = z_max + (layer as f32) * 0.012;
                let z_step_b = z_max + ((layer + 1) as f32) * 0.012;
                let col = strata_colors[layer];
                let norm = [0.0, 0.22, 0.975];
                let v0 = b_terrain.push_vertex([x0, y0_a, z_step_a], norm, col, [u0 * 4.0, f0]);
                let v1 = b_terrain.push_vertex([x1, y1_a, z_step_a], norm, col, [u1 * 4.0, f0]);
                let v2 = b_terrain.push_vertex([x1, y1_b, z_step_b], norm, col, [u1 * 4.0, f1]);
                let v3 = b_terrain.push_vertex([x0, y0_b, z_step_b], norm, col, [u0 * 4.0, f1]);
                b_terrain.push_tri(v0, v3, v2);
                b_terrain.push_tri(v0, v2, v1);
            }
        }
    }

    // 2. Build 3D Submerged & Shoreline Riverbed Cobbles, Pebbles & Valley Boulders/Granite Tors
    for i in 0..220 {
        let fi = i as f32;
        let z = -1.75 + (fi / 220.0) * 3.50;
        let meander_x = 0.28 * (z * 1.35 + 1.1).sin();
        let offset = (hash21(fi * 3.7, 9.1) - 0.5) * 0.68;
        let x = meander_x + offset;
        let y = sample_sector_elevation(sector_id, x, z, water_y);
        let r = 0.014 + 0.024 * hash21(fi * 7.3, 2.1);
        let tint_sel = hash21(fi * 11.1, 5.7);
        let (col_b, col_t) = if tint_sel > 0.70 {
            ([0.62, 0.56, 0.46, 1.0], [0.86, 0.80, 0.68, 1.0]) // alluvial quartz pebble
        } else if tint_sel > 0.40 {
            ([0.52, 0.34, 0.24, 1.0], [0.74, 0.48, 0.34, 1.0]) // red jasper / ironstone cobble
        } else {
            ([0.28, 0.30, 0.34, 1.0], [0.48, 0.50, 0.54, 1.0]) // basalt / granite river cobble
        };
        b_lithology.add_rock_or_pebble(
            [x, y + r * 0.25, z],
            [r * 1.35, r * 0.65, r * 1.15],
            fi * 1.9,
            col_b,
            col_t,
            fi,
        );
    }

    // Valley Boulders, Granite Tors, Sandstone Escarpment Blocks & Karst Outcrops
    for i in 0..55 {
        let fi = i as f32;
        let side = if i % 2 == 0 { 1.0_f32 } else { -1.0_f32 };
        let z = -1.65 + (fi / 55.0) * 3.30;
        let x = side * (0.48 + 1.25 * hash21(fi * 5.3, 1.7));
        let y = sample_sector_elevation(sector_id, x, z, water_y);
        let rx = 0.045 + 0.085 * hash21(fi * 2.9, 8.4);
        let ry = rx * (0.65 + 0.65 * hash21(fi * 4.1, 3.3));
        let (col_b, col_t) = match sector_id {
            "sydney_sandstone" => ([0.52, 0.38, 0.25, 1.0], [0.78, 0.62, 0.44, 1.0]),
            "yarrangobilly" | "jounama_karst" => ([0.44, 0.46, 0.50, 1.0], [0.72, 0.74, 0.78, 1.0]),
            _ => ([0.40, 0.38, 0.36, 1.0], [0.66, 0.64, 0.60, 1.0]),
        };
        b_lithology.add_rock_or_pebble(
            [x, y + ry * 0.3, z],
            [rx, ry, rx * 0.88],
            fi * 0.85,
            col_b,
            col_t,
            fi + 100.0,
        );
    }

    // 3. Evaluate AG-12 `VegetationEvaluator::update_instances` across Vegetation Instance Pool
    // and Populate Multi-Stratum Canopy Trees, Shrubs, Tree Ferns, Grass Trees, Tussock Grasses,
    // and Submerged Benthic *Vallisneria* Ribbon Weed!
    let mut veg_states = Vec::with_capacity(64);
    for i in 0..64 {
        let fi = i as f32;
        let side = if i % 2 == 0 { 1.0_f32 } else { -1.0_f32 };
        let z = -1.70 + (fi / 64.0) * 3.40;
        let meander_x = 0.28 * (z * 1.35 + 1.1).sin();
        let x = (meander_x + side * (0.36 + 1.38 * hash21(fi * 6.1, 4.2))).clamp(-1.82, 1.82);
        let y = sample_sector_elevation(sector_id, x, z, water_y);
        let h = 0.26 + 0.24 * hash21(fi * 9.7, 1.3);
        let transform = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ];
        veg_states.push(VegetationInstanceState::new(
            transform,
            h,
            q_hash(&format!("ecopunk:veg:{sector_id}:{i}")),
        ));
    }
    let colliders = [
        VegetationCollider {
            position: [0.42, sample_sector_elevation(sector_id, 0.42, 0.15, water_y), 0.15],
            radius: 0.25,
            velocity: [0.4, 0.0, 0.2],
        },
        VegetationCollider {
            position: [-0.38, sample_sector_elevation(sector_id, -0.38, -0.25, water_y), -0.25],
            radius: 0.22,
            velocity: [-0.2, 0.0, 0.3],
        },
    ];
    let bend_params = VegetationBendingParams {
        stiffness: 0.45,
        recovery_rate: 6.5,
        max_bend_angle: 0.85,
        wind_direction: [0.85, 0.52],
        wind_strength: 0.18,
    };
    let mut out_records = vec![
        GpuInstanceRecord::new(
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            0,
        );
        veg_states.len()
    ];
    let vegetation_instances_evaluated = VegetationEvaluator::update_instances(
        &mut veg_states,
        &colliders,
        &bend_params,
        0.016,
        1.25,
        &mut out_records,
    );

    // Emit the 64 Multi-Species Canopy & Mid-Story Trees + Tree Ferns + Grass Trees
    for (i, state) in veg_states.iter().enumerate() {
        let fi = i as f32;
        let root = [
            state.base_transform[3][0],
            state.base_transform[3][1],
            state.base_transform[3][2],
        ];
        let h = state.height;
        if i % 7 == 0 {
            // *Dicksonia antarctica* Soft Tree Fern in moist gullies
            b_foliage.add_tree_fern_or_grass_tree(&mut b_trunks, root, h * 0.52, false, fi);
        } else if i % 6 == 0 {
            // *Xanthorrhoea australis* Austral Grass Tree with flowering scape
            b_foliage.add_tree_fern_or_grass_tree(&mut b_trunks, root, h * 0.46, true, fi);
        } else {
            // Diverse Australian Canopy & Mid-Story Archetypes:
            // Ghost Gum (*Corymbia aparrerinja*), Snow Gum (*Eucalyptus pauciflora*),
            // Mountain Ash (*Eucalyptus regnans*), Golden Wattle (*Acacia pycnantha*),
            // Old Man Banksia (*Banksia serrata*), NSW Waratah (*Telopea speciosissima*)
            let (bark_b, bark_t, leaf_d, leaf_l, contorted) = match i % 5 {
                0 => (
                    // Ghost Gum / Snow Gum chalk-white & ochre-mottled smooth bark
                    [0.78, 0.74, 0.68, 1.0],
                    [0.94, 0.92, 0.88, 1.0],
                    [0.18, 0.38, 0.20, 1.0],
                    [0.36, 0.62, 0.32, 1.0],
                    true,
                ),
                1 => (
                    // Golden Wattle (*Acacia pycnantha*) with golden-yellow flower racemes
                    [0.30, 0.24, 0.18, 1.0],
                    [0.42, 0.34, 0.26, 1.0],
                    [0.24, 0.46, 0.18, 1.0],
                    [0.92, 0.78, 0.16, 1.0],
                    false,
                ),
                2 => (
                    // Mountain Ash / River Red Gum tall sclerophyll canopy
                    [0.34, 0.26, 0.20, 1.0],
                    [0.68, 0.64, 0.56, 1.0],
                    [0.14, 0.34, 0.18, 1.0],
                    [0.30, 0.56, 0.28, 1.0],
                    false,
                ),
                3 => (
                    // *Banksia serrata* / *Telopea speciosissima* Waratah crimson/amber blooms
                    [0.28, 0.22, 0.18, 1.0],
                    [0.40, 0.32, 0.26, 1.0],
                    [0.22, 0.40, 0.18, 1.0],
                    [0.82, 0.26, 0.18, 1.0],
                    true,
                ),
                _ => (
                    // Peppermint / Stringybark Eucalypt
                    [0.38, 0.30, 0.22, 1.0],
                    [0.56, 0.48, 0.38, 1.0],
                    [0.16, 0.36, 0.22, 1.0],
                    [0.34, 0.58, 0.34, 1.0],
                    true,
                ),
            };
            b_foliage.add_canopy_tree(
                &mut b_trunks,
                root,
                h,
                bark_b,
                bark_t,
                leaf_d,
                leaf_l,
                contorted,
                fi,
            );
        }
    }

    // 4. Dense Multi-Stratum Ground Cover: Cosserat Tussock Grasses (*Themeda triandra*, *Poa*),
    // Riparian Rushes (*Juncus*), and Submerged Benthic *Vallisneria* Ribbon Weed on the riverbed!
    for i in 0..240 {
        let fi = i as f32;
        let x = -1.78 + 3.56 * hash21(fi * 4.3, 7.1);
        let z = -1.78 + 3.56 * hash21(fi * 8.9, 2.7);
        let y = sample_sector_elevation(sector_id, x, z, water_y);
        if y < water_y - 0.03 {
            // Submerged benthic *Vallisneria australis* ribbon weed waving on the riverbed
            b_foliage.add_cosserat_grass_tuft(
                [x, y, z],
                6,
                (water_y - y - 0.02).clamp(0.04, 0.22),
                0.05,
                [0.08, 0.34, 0.20, 1.0],
                [0.18, 0.58, 0.32, 1.0],
                fi,
            );
        } else if y < water_y + 0.10 {
            // Riparian *Juncus usitatus* rushes & *Carex* sedges along the water's edge
            b_foliage.add_cosserat_grass_tuft(
                [x, y, z],
                8,
                0.11,
                0.06,
                [0.16, 0.38, 0.16, 1.0],
                [0.34, 0.62, 0.24, 1.0],
                fi,
            );
        } else {
            // *Themeda triandra* Kangaroo Grass & *Poa sieberiana* Snow Grass tussocks
            let is_themeda = i % 2 == 0;
            let (col_b, col_t) = if is_themeda {
                ([0.36, 0.34, 0.16, 1.0], [0.68, 0.56, 0.26, 1.0])
            } else {
                ([0.22, 0.38, 0.18, 1.0], [0.46, 0.64, 0.30, 1.0])
            };
            b_foliage.add_cosserat_grass_tuft([x, y, z], 7, 0.085, 0.065, col_b, col_t, fi);
        }
    }

    // 5. Articulated 3D Fauna Specimens across Terrestrial, Aerial, and Underwater Strata
    // Eastern Grey Kangaroos (*Macropus giganteus*) on the valley floor
    for (kx, kz, kyaw) in [
        (0.44_f32, 0.22_f32, -0.65_f32),
        (0.62_f32, -0.18_f32, -1.10_f32),
        (-0.48_f32, 0.35_f32, 0.75_f32),
    ] {
        let ky = sample_sector_elevation(sector_id, kx, kz, water_y);
        b_fauna.add_fauna_specimen(
            [kx, ky, kz],
            kyaw,
            0.18,
            "macropod",
            [0.38, 0.32, 0.26, 1.0],
            [0.62, 0.54, 0.44, 1.0],
        );
    }
    // Common Wombats (*Vombatus ursinus*)
    for (wx, wz, wyaw) in [(-0.55_f32, -0.32_f32, 0.45_f32), (0.38_f32, 0.65_f32, -0.85_f32)] {
        let wy = sample_sector_elevation(sector_id, wx, wz, water_y);
        b_fauna.add_fauna_specimen(
            [wx, wy, wz],
            wyaw,
            0.14,
            "wombat",
            [0.28, 0.22, 0.18, 1.0],
            [0.46, 0.38, 0.30, 1.0],
        );
    }
    // Submerged Platypus (*Ornithorhynchus anatinus*) & Murray Cod (*Maccullochella peelii*) inside the river!
    for (uz, uyaw, is_cod) in [(-0.45_f32, 0.2_f32, false), (0.35_f32, -0.3_f32, true)] {
        let ux = 0.28 * (uz * 1.35 + 1.1).sin();
        let bed_y = sample_sector_elevation(sector_id, ux, uz, water_y);
        let swim_y = (bed_y + 0.08).min(water_y - 0.04);
        let (cd, cl) = if is_cod {
            ([0.16, 0.34, 0.20, 1.0], [0.36, 0.58, 0.32, 1.0])
        } else {
            ([0.26, 0.18, 0.12, 1.0], [0.44, 0.32, 0.22, 1.0])
        };
        b_fauna.add_fauna_specimen([ux, swim_y, uz], uyaw, 0.12, "aquatic", cd, cl);
    }
    // Soaring Wedge-Tailed Eagle (*Aquila audax*) & Sulphur-Crested Cockatoo overhead
    b_fauna.add_fauna_specimen(
        [-0.25, 0.58, -0.15],
        0.55,
        0.18,
        "raptor",
        [0.22, 0.16, 0.12, 1.0],
        [0.44, 0.32, 0.22, 1.0],
    );
    b_fauna.add_fauna_specimen(
        [0.32, 0.48, 0.25],
        -0.75,
        0.12,
        "raptor",
        [0.88, 0.88, 0.84, 1.0],
        [0.98, 0.96, 0.62, 1.0],
    );

    // For `three_mile_dam` (GPR & Trophic HUD `0x02`/`0x05`), synthesize 3D volumetric Ground-Penetrating
    // Radar dielectric wave-reflector ribbons and Trophic Energy-Flow arcs across the valley!
    if sector_id == "three_mile_dam" {
        for r_idx in 0..7 {
            let z_line = -0.95 + (r_idx as f32) * 0.32;
            let mut gpr_axis = Vec::with_capacity(16);
            let mut gpr_st = Vec::with_capacity(16);
            for s in 0..16 {
                let u = s as f32 / 15.0;
                let x = -1.25 + u * 2.50;
                let y_surf = sample_sector_elevation(sector_id, x, z_line, water_y);
                let wave = 0.035 * (x * 7.5 + r_idx as f32).cos();
                gpr_axis.push(Point3::new(
                    x as f64,
                    (y_surf + 0.03 + wave) as f64,
                    z_line as f64,
                ));
                gpr_st.push(ExtrusionStation {
                    scale_x: 0.014,
                    scale_y: 0.008,
                    twist_rad: 0.0,
                });
            }
            b_fauna.add_bishop_swept_tube(
                &gpr_axis,
                &gpr_st,
                6,
                [0.06, 0.78, 0.96, 1.0],
                [0.25, 0.98, 1.0, 1.0],
            );
        }
        // Golden Trophic Energy-Flow parabolic arcs connecting primary producers (flora) to herbivores & apex raptors
        for &(p0, p1, apex_h) in &[
            ([-0.65_f32, 0.05_f32, 0.25_f32], [0.44_f32, 0.02_f32, 0.22_f32], 0.42_f32),
            ([0.44_f32, 0.02_f32, 0.22_f32], [-0.25_f32, 0.58_f32, -0.15_f32], 0.72_f32),
            ([0.62_f32, 0.04_f32, -0.18_f32], [0.32_f32, 0.48_f32, 0.25_f32], 0.64_f32),
        ] {
            let mut arc_axis = Vec::with_capacity(14);
            let mut arc_st = Vec::with_capacity(14);
            for s in 0..14 {
                let t = s as f32 / 13.0;
                let x = p0[0] * (1.0 - t) + p1[0] * t;
                let z = p0[2] * (1.0 - t) + p1[2] * t;
                let y = p0[1] * (1.0 - t) + p1[1] * t + 4.0 * t * (1.0 - t) * apex_h;
                let r = (0.013 * (1.0 - 0.35 * (t - 0.5).abs())) as f64;
                arc_axis.push(Point3::new(x as f64, y as f64, z as f64));
                arc_st.push(ExtrusionStation {
                    scale_x: r,
                    scale_y: r,
                    twist_rad: 0.0,
                });
            }
            b_fauna.add_bishop_swept_tube(
                &arc_axis,
                &arc_st,
                6,
                [0.98, 0.68, 0.12, 1.0],
                [1.0, 0.96, 0.38, 1.0],
            );
        }
    }

    // 6. Merge the 5 SubmeshBuilders into one unified scene mesh with 5 `MAT4` `MaterialRecord`s and `SubmeshRange`s
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut materials = Vec::new();
    let mut ranges = Vec::new();

    let submeshes = [
        (
            "00-terrain-soil-strata",
            b_terrain,
            tex_soil,
            0.86_f32,
            0.0_f32,
            false,
            false,
        ),
        (
            "01-lithology-boulders-pebbles",
            b_lithology,
            tex_pebble,
            0.68_f32,
            0.04_f32,
            false,
            false,
        ),
        (
            "02-flora-trunks-branches",
            b_trunks,
            tex_bark,
            0.78_f32,
            0.0_f32,
            false,
            false,
        ),
        (
            "03-flora-foliage-blades-grass",
            b_foliage,
            tex_leaf,
            0.48_f32,
            0.0_f32,
            true,
            true,
        ),
        (
            "04-fauna-specimens",
            b_fauna,
            tex_bark,
            0.58_f32,
            0.02_f32,
            false,
            false,
        ),
    ];

    for (mat_slug, sub, tex_digest, roughness, metallic, double_sided, wind_animated) in submeshes
    {
        if sub.indices.is_empty() {
            continue;
        }
        let first_index = indices.len() as u32;
        let base_vertex = positions.len() as u32;
        let index_count = sub.indices.len() as u32;

        positions.extend_from_slice(&sub.positions);
        normals.extend_from_slice(&sub.normals);
        colors.extend_from_slice(&sub.colors);
        uvs.extend_from_slice(&sub.uvs);
        for idx in sub.indices {
            indices.push(base_vertex + idx);
        }

        let mat_id = q_hash(&format!("urn:qualia:material:ecopunk:{sector_id}:{mat_slug}"));
        let mut mat = MaterialRecord::legacy_default();
        mat.id = mat_id;
        mat.shading_model = ShadingModel::MetallicRoughness;
        mat.opacity_mode = OpacityMode::Opaque;
        mat.double_sided = double_sided;
        mat.casts_shadow = true;
        mat.receives_shadow = true;
        mat.pickable = true;
        mat.multiply_vertex_color = true;
        mat.wind_animated = wind_animated;
        mat.base_color = [1.0, 1.0, 1.0, 1.0];
        mat.roughness = roughness;
        mat.metallic = metallic;
        mat.specular = 0.5;
        mat.base_color_texture = tex_digest;

        materials.push(mat);
        ranges.push(SubmeshRange {
            first_index,
            index_count,
            material_id: mat_id,
            semantic_id: mat_id,
        });
    }
    materials.sort_by_key(|m| m.id);

    let scene_10d_bytes = seal_mesh_to_10d(
        &positions,
        &normals,
        &colors,
        &uvs,
        &indices,
        &materials,
        &ranges,
        format!("ecopunk:scene:{sector_id}").into_bytes(),
    )?;
    let scene_crc32c = compiled_digest(&scene_10d_bytes);

    // 7. Build High-Resolution Water Surface Mesh (`10d/water_<sector>.10d`) for AG-10 `upload_water_geometry`
    let wn = 36usize;
    let mut water_positions = Vec::with_capacity((wn + 1) * (wn + 1));
    let mut water_normals = Vec::with_capacity((wn + 1) * (wn + 1));
    let mut water_colors = Vec::with_capacity((wn + 1) * (wn + 1));
    let mut water_uvs = Vec::with_capacity((wn + 1) * (wn + 1));
    let mut water_indices = Vec::with_capacity(wn * wn * 6);

    for wz in 0..=wn {
        for wx in 0..=wn {
            let u = wx as f32 / wn as f32;
            let v = wz as f32 / wn as f32;
            let z = -extent + v * (z_max + extent);
            let meander_x = 0.28 * (z * 1.35 + 1.1).sin() + 0.11 * (z * 2.7 - 0.66).cos();
            let half_w = if sector_id == "noosa_headland" {
                1.15_f32
            } else {
                0.48_f32
            };
            let x = if sector_id == "noosa_headland" {
                -extent + u * (meander_x + 0.35 + extent)
            } else {
                meander_x + (u * 2.0 - 1.0) * half_w
            };
            water_positions.push([x, water_y, z]);
            water_normals.push([0.0, 1.0, 0.0]);
            water_colors.push([water_color[0], water_color[1], water_color[2], 0.72]);
            water_uvs.push([u, v]);
        }
    }
    for wz in 0..wn {
        for wx in 0..wn {
            let i00 = (wz * (wn + 1) + wx) as u32;
            let i10 = i00 + 1;
            let i01 = ((wz + 1) * (wn + 1) + wx) as u32;
            let i11 = i01 + 1;
            water_indices.extend_from_slice(&[i00, i01, i11, i00, i11, i10]);
        }
    }

    let mut water_mat = MaterialRecord::legacy_default();
    water_mat.id = q_hash(&format!("urn:qualia:material:ecopunk:{sector_id}:water"));
    let water_range = SubmeshRange {
        first_index: 0,
        index_count: water_indices.len() as u32,
        material_id: water_mat.id,
        semantic_id: water_mat.id,
    };
    let water_10d_bytes = seal_mesh_to_10d(
        &water_positions,
        &water_normals,
        &water_colors,
        &water_uvs,
        &water_indices,
        &[water_mat],
        &[water_range],
        format!("ecopunk:water:{sector_id}").into_bytes(),
    )?;
    let water_crc32c = compiled_digest(&water_10d_bytes);

    Ok(CompiledSector10d {
        sector_id,
        title,
        scene_hmc_key: format!("10d/sector-{sector_id}.10d"),
        water_hmc_key: format!("10d/water-{sector_id}.10d"),
        scene_10d_bytes,
        water_10d_bytes,
        scene_crc32c,
        water_crc32c,
        positions,
        normals,
        colors,
        uvs,
        indices,
        materials,
        ranges,
        water_positions,
        water_indices,
        water_color,
        water_alpha: 0.68,
        sun_dir,
        cam_yaw,
        cam_pitch,
        cam_zoom,
        cam_target,
        sky_preset,
        vegetation_instances_evaluated,
    })
}

/// Build a close-up Botanical Specimen Loupe `.10d` scene (*Eucalyptus pauciflora* Snow Gum &
/// *Corymbia aparrerinja* Ghost Gum with 3D falcate leaf blades, midrib venation, and granite tors)
/// or a close-up Zoological Tracker `.10d` scene (*Macropus giganteus* Eastern Grey Kangaroo family).
pub fn build_closeup_specimen_10d(
    specimen_id: &'static str,
    title: &'static str,
    textures: &[ProceduralTextureAsset],
    is_fauna: bool,
) -> Result<CompiledSector10d, String> {
    let mut sector = build_sector_10d("talbingo", title, textures, false)?;
    sector.sector_id = specimen_id;
    sector.scene_hmc_key = format!("10d/specimen-{specimen_id}.10d");
    sector.water_hmc_key = format!("10d/specimen-water-{specimen_id}.10d");
    if is_fauna {
        // Focus camera directly on the foreground Eastern Grey Kangaroo & river shingle bar
        sector.cam_target = [0.44, -0.05, 0.22];
        sector.cam_yaw = -0.45;
        sector.cam_pitch = 0.20;
        sector.cam_zoom = 0.88;
    } else {
        // Focus camera directly on the foreground Ghost Gum / Snow Gum crown, falcate leaf blades & clear river pebbles
        sector.cam_target = [0.26, 0.08, -0.08];
        sector.cam_yaw = 0.32;
        sector.cam_pitch = 0.18;
        sector.cam_zoom = 0.92;
    }
    Ok(sector)
}

fn seal_mesh_to_10d(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[f32; 4]],
    uvs: &[[f32; 2]],
    indices: &[u32],
    materials: &[MaterialRecord],
    ranges: &[SubmeshRange],
    source_recipe: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    let mut triangles = Vec::with_capacity(indices.len() / 3);
    for tri in indices.chunks_exact(3) {
        triangles.push([tri[0], tri[1], tri[2]]);
    }
    let mesh = Mesh {
        positions: positions.to_vec(),
        triangles,
        min,
        max,
    };
    let provenance = ProvenanceSidecar::new(
        source_recipe,
        "application/vnd.ecopunk.vibe-scene;version=1",
        "Apache-2.0",
    );
    let bytes = compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        Some(colors),
        Some(normals),
        None,
    )
    .map_err(|e| format!("compile_mesh_to_10d: {e}"))?;

    let bytes = attach_material_section(&bytes, materials, ranges, indices.len() as u32)
        .map_err(|e| format!("attach_material_section: {e:?}"))?;
    let bytes = attach_texture_coordinate_section(&bytes, uvs)
        .map_err(|e| format!("attach_texture_coordinate_section: {e:?}"))?;
    Ok(bytes)
}

/// Render a compiled `.10d` sector natively on the physical GPU via `PortalGpu` (`wgpu` — Zero V8)
/// with resident mipmapped PBR textures, AG-22 `EnvironmentProbeSet` IBL, cascaded sun shadows,
/// half-resolution SSAO, analytic sky (`SKY_WGSL`), and AG-10 `WATER_QUALITY_CINEMATIC` shoreline water!
pub fn render_sector_via_portal_gpu_png(
    sector: &CompiledSector10d,
    textures: &[ProceduralTextureAsset],
    width: u32,
    height: u32,
    stylized_override: Option<(ShadingModel, u32, [f32; 3])>,
) -> Result<Vec<u8>, String> {
    let mut renderer = PortalGpu::new_offscreen(width, height, 256)?;

    // Upload all 4 resident PBR RGBA8 textures with GPU mip generation
    for tex in textures {
        renderer
            .upload_resident_texture_rgba8_with_mips(
                tex.sha256_bytes,
                TextureColorSpace::Srgb,
                TextureMipSemantic::Color,
                tex.width,
                tex.height,
                &tex.rgba8,
            )
            .map_err(|e| format!("upload_resident_texture {}: {e}", tex.name))?;
    }

    let mut materials = sector.materials.clone();
    let mut colors = sector.colors.clone();
    let mut water_color = sector.water_color;
    let mut water_alpha = sector.water_alpha;
    let mut sky_preset = sector.sky_preset;
    let mut probe_tint = [0.38_f32, 0.46_f32, 0.56_f32];
    if let Some((shading, preset, tint)) = stylized_override {
        sky_preset = preset;
        probe_tint = tint;
        for m in &mut materials {
            m.shading_model = shading;
        }
    }

    // Apply authentic multi-modal observer projections (`Cel-Toon`, `Namatjira Watercolor`,
    // `GPR & Trophic HUD`, and `Planck Blackbody Thermal IR 0x01`) when rendering specialized sectors:
    match sector.sector_id {
        "sydney_sandstone" => {
            // Botanical Cel-Toon Illustration (`05_sydney_sandstone_cel_toon.png`):
            // High-saturation quantized pigment bands with warm golden Hawkesbury sandstone & crimson Waratahs
            water_color = [0.06, 0.52, 0.68];
            for c in &mut colors {
                let q = |v: f32| ((v * 4.5).round() / 4.5).clamp(0.08, 1.0);
                c[0] = q(c[0] * 1.18);
                c[1] = q(c[1] * 1.14);
                c[2] = q(c[2] * 1.08);
            }
        }
        "noosa_headland" => {
            // Albert Namatjira Australian Watercolor Wash (`06_noosa_headland_watercolor.png`):
            // Warm red-ochre / terracotta earth, luminous cream ghost-gums, soft sage-eucalypt washes & turquoise Coral Sea
            water_color = [0.10, 0.58, 0.72];
            water_alpha = 0.62;
            for (vi, c) in colors.iter_mut().enumerate() {
                let p = sector.positions[vi];
                let paper_wash = 0.92 + 0.08 * (p[0] * 11.0 + p[2] * 9.0).sin();
                c[0] = (c[0] * 0.82 + 0.24).clamp(0.0, 1.0) * paper_wash;
                c[1] = (c[1] * 0.80 + 0.18).clamp(0.0, 1.0) * paper_wash;
                c[2] = (c[2] * 0.76 + 0.16).clamp(0.0, 1.0) * paper_wash;
            }
        }
        "three_mile_dam" => {
            // GPR Dielectric & Trophic Energy HUD (`07_three_mile_dam_gpr_hud.png`):
            // Subsurface radar cross-section slate-indigo substrate with glowing cyan GPR wave reflectors & golden trophic arcs
            water_color = [0.04, 0.62, 0.84];
            water_alpha = 0.55;
            if let Some(r_fauna) = sector.ranges.get(4) {
                for m in &mut materials {
                    if m.id == r_fauna.material_id {
                        m.emissive = [0.22, 0.58, 0.68];
                    }
                }
            }
        }
        "jounama_karst" => {
            // Radiometric Planck Blackbody Thermal IR (`0x01`, 8-14 um FLIR Ironbow) (`08_jounama_karst_thermal_ir.png`):
            // Cold river water (280 K) = deep thermal navy-violet;
            // Cool transpiring foliage (285 K) = indigo-cyan;
            // Soil & karst valley (287 K) = deep violet-magenta;
            // Sun-heated karst boulders (299 K) = crimson-orange;
            // Endothermic 310 K Australian Fauna (Kangaroos, Wombats, Wedge-Tailed Eagle) = incandescent white-gold!
            water_color = [0.05, 0.06, 0.26];
            water_alpha = 0.82;
            for (r_idx, r) in sector.ranges.iter().enumerate() {
                let start_tri = (r.first_index as usize) / 3;
                let end_tri = start_tri + (r.index_count as usize) / 3;
                for tri_i in start_tri..end_tri {
                    for k in 0..3 {
                        let vi = sector.indices[tri_i * 3 + k] as usize;
                        let p = sector.positions[vi];
                        let n = sector.normals[vi];
                        let solar = (n[0] * (-0.44) + n[1] * 0.74 + n[2] * 0.51).clamp(0.0, 1.0);
                        colors[vi] = match r_idx {
                            0 => {
                                // Terrain & Soil (282..290 K): deep midnight indigo to violet-magenta
                                [
                                    0.14 + 0.36 * solar,
                                    0.06 + 0.12 * solar,
                                    0.38 + 0.18 * (1.0 - solar),
                                    1.0,
                                ]
                            }
                            1 => {
                                // Solar-warmed Boulders & River Cobbles (294..303 K): magenta-crimson to warm orange
                                [
                                    0.68 + 0.28 * solar,
                                    0.18 + 0.42 * solar,
                                    0.42 * (1.0 - solar),
                                    1.0,
                                ]
                            }
                            2 => {
                                // Tree Trunks & Limbs (291 K): warm purple-crimson
                                [0.52 + 0.22 * solar, 0.12 + 0.18 * solar, 0.44, 1.0]
                            }
                            3 => {
                                // Transpiring Eucalypt Foliage & Grasses (284..288 K): cool thermal teal-indigo-violet
                                let h_warm = (p[1] + 0.3).clamp(0.0, 0.8);
                                [
                                    0.16 + 0.34 * h_warm,
                                    0.22 + 0.28 * solar,
                                    0.56 - 0.14 * h_warm,
                                    1.0,
                                ]
                            }
                            _ => {
                                // Endothermic 310 K Australian Fauna: blazing incandescent white-hot solar-gold!
                                [1.0, 0.92, 0.36, 1.0]
                            }
                        };
                    }
                }
                if r_idx == 4 {
                    for m in &mut materials {
                        if m.id == r.material_id {
                            m.emissive = [0.92, 0.58, 0.14];
                        }
                    }
                }
            }
        }
        _ => {}
    }

    // Configure analytic sky, directional sun, cascaded shadows, and SSAO
    renderer.set_sky_preset(sky_preset);
    renderer.set_lighting(
        sector.sun_dir[0],
        sector.sun_dir[1],
        sector.sun_dir[2],
        1.25,
        0.38,
    );
    renderer.set_shadows_enabled(true);
    renderer.set_screen_space_ao_enabled(true);
    renderer.set_screen_space_ao(0.42, 0.65, 0.02);
    renderer.set_screen_space_ao_sample_count(12);

    // Bind AG-22 EnvironmentProbeSet for diffuse sky/canopy bounce irradiance & specular reflection
    let mut probes = EnvironmentProbeSet::new();
    probes.push(EnvironmentProbe {
        position: [0.0, 0.25, 0.0],
        radius: 12.0,
        irradiance: probe_tint,
        specular_levels: [
            [probe_tint[0] * 1.15, probe_tint[1] * 1.15, probe_tint[2] * 1.25],
            probe_tint,
            [probe_tint[0] * 0.85, probe_tint[1] * 0.85, probe_tint[2] * 0.90],
            [probe_tint[0] * 0.70, probe_tint[1] * 0.70, probe_tint[2] * 0.75],
            [probe_tint[0] * 0.58, probe_tint[1] * 0.58, probe_tint[2] * 0.62],
        ],
        dominant_direction: sector.sun_dir,
        valid: true,
    });
    renderer.set_environment_probes(Some(&probes), [0.22, 0.26, 0.32]);

    // Upload multi-submesh MAT4 scene geometry
    renderer
        .upload_mesh_colored_with_frames_and_materials_and_uv0(
            &sector.positions,
            &colors,
            Some(&sector.normals),
            None,
            Some(&sector.uvs),
            &sector.indices,
            &materials,
            &sector.ranges,
        )
        .map_err(|e| format!("upload_mesh_colored_with_frames_and_materials_and_uv0: {e}"))?;

    // Upload AG-10 water geometry with BVH shoreline SDF distance (`shore_distance_accelerated_into`)
    // and `WATER_QUALITY_CINEMATIC` (tier 2) refractive depth-blended water & shoreline foam!
    renderer.set_water_surface(water_color, water_alpha, 0.012, 3.2);
    renderer.set_water_quality(2);
    renderer
        .upload_water_geometry(&sector.water_positions, &sector.water_indices)
        .map_err(|e| format!("upload_water_geometry: {e}"))?;

    // Position orbit camera
    renderer.set_camera_target(
        sector.cam_yaw,
        sector.cam_pitch,
        sector.cam_zoom,
        sector.cam_target[0],
        sector.cam_target[1],
        sector.cam_target[2],
    );

    // Execute native GPU render pass (`wgpu` — zero V8)
    renderer.render(1.25, &SystemTelemetry::default())?;

    let mut rgba = vec![0u8; renderer.required_rgba8_bytes()];
    renderer.read_rgba8_into(&mut rgba)?;

    Ok(encode_rgba8_to_png(width, height, &rgba))
}

