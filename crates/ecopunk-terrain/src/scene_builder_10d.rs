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

use bytemuck;
use image::ImageEncoder;
use qualia_core_db::container_10d::entity_ontology_section::{
    AffordanceBinaryRecord, DevelopmentalKeyframeRecord, EntityOntologyMiniHeader,
    AFFORDANCE_RECORD_SIZE, DEVELOPMENTAL_KEYFRAME_RECORD_SIZE,
    ENTITY_ONTOLOGY_MINI_HEADER_SIZE, ENTITY_ONTOLOGY_SECTION_VERSION, MAGIC_EONT,
};
use qualia_core_db::container_10d::header::{Container10dHeader, MAX_SECTION_COUNT};
use qualia_core_db::container_10d::integrity::seal_whole_file_crc32c;
use qualia_core_db::container_10d::morph_section::{
    encode_morph_section, morph_section_encoded_len, quantize_morph_delta, MorphSectionHeader,
    QuantizedMorphDelta, FLAG_MORPH_HAS_NORMALS, FLAG_MORPH_ONTOGENETIC_BIND, FLAG_MORPH_SPARSE,
    MAGIC_MRP1, MORPH_SECTION_VERSION,
};
use qualia_core_db::container_10d::procedural_synthesis::{
    encode_procedural_synthesis_section, evaluate_pga_motor_oscillation,
    ProceduralSynthesisHeader, FLAG_ALLOMETRIC_GROWTH, FLAG_CELLULAR_VORONOI,
    FLAG_IMPLICIT_FILLET, FLAG_PGA_MOTOR_DYNAMICS, MAGIC_PROC,
    PROCEDURAL_SYNTHESIS_HEADER_SIZE, PROCEDURAL_SYNTHESIS_SECTION_VERSION,
};
use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
use qualia_core_db::container_10d::section::{
    encode_container, parse_section_table, AlignmentTier, SectionInput, SectionTableError,
    SectionType,
};
use qualia_core_db::container_10d::{
    attach_material_section, attach_texture_coordinate_section, MaterialRecord, OpacityMode,
    ShadingModel, SubmeshRange,
};
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, compiled_digest, Compile10dExtras,
};
#[cfg(not(target_arch = "wasm32"))]
use qualia_core_db::render::gpu::ibl_cubemap::integrate_split_sum_brdf_zero_heap;
#[cfg(not(target_arch = "wasm32"))]
use qualia_core_db::render::gpu::{
    compute_csm_4_splits_zero_heap, PortalGpu, TextureColorSpace, TextureMipSemantic,
};
use qualia_core_db::render::instance_culling::GpuInstanceRecord;
use qualia_core_db::render::lighting::environment_lighting::{
    EnvironmentProbe, EnvironmentProbeSet,
};
use qualia_core_db::render::skeletal::gpu_skin::{
    evaluate_skinning_motor_palette_zero_heap, gpu_shader_skin_vertex_reference,
    GpuJointMotorPair, GpuSkinnedVertex,
};
use qualia_core_db::render::skeletal::Motor;
use qualia_core_db::render::telemetry::SystemTelemetry;
use qualia_core_db::render::vegetation::{
    VegetationBendingParams, VegetationCollider, VegetationEvaluator, VegetationInstanceState,
};
use qualia_core_db::specialized_libs::computational_geometry::authoring::loft_convex_sections_into;
use qualia_core_db::specialized_libs::computational_geometry::botanical_hulls::{
    asymmetric_hull_capacity, asymmetric_hull_into, AsymmetricHullParams,
};
use qualia_core_db::specialized_libs::computational_geometry::math_geometry::AffineFrame3;
use qualia_core_db::specialized_libs::computational_geometry::parametric_cad::revolve_profile;
use qualia_core_db::specialized_libs::computational_geometry::sdf_ops::{
    sdf_capsule, sdf_rounded_box, sdf_sphere, sdf_smooth_union,
};
use qualia_core_db::specialized_libs::computational_geometry::space_colonization::{
    space_colonization_3d, SpaceColonizationConfig, ROOT_PARENT,
};
use qualia_core_db::specialized_libs::computational_geometry::sweep_extrusion::{
    extrude_profile_along_path, ExtrusionStation,
};
use qualia_core_db::specialized_libs::computational_geometry::tree_skeleton_3d::pipe_model_radii;
use qualia_core_db::specialized_libs::computational_geometry::wheel_panel::{
    wheel_clearance_panel_into, WheelPanel,
};
use qualia_core_db::specialized_libs::computational_geometry::{marching_cubes, Point2, Point3};
use sha2::{Digest, Sha256};

pub const FLAG_SMOOTH_MIN_SDF: u32 = FLAG_IMPLICIT_FILLET;
pub const FLAG_ALLOMETRIC_SCALING: u32 = FLAG_ALLOMETRIC_GROWTH;

/// Canonical uniform SI metric world conversion factor for the 96 m × 96 m focal sector patch.
/// In 1:1 SI metric space (`scale = 1.0`), 1.0 unit = 1.0 meter:
/// - 4WD Field Utility Rover: 4.85 m long × 1.88 m wide × 1.92 m high
/// - Eastern Grey Kangaroo (*Macropus giganteus*): 1.65 m standing height
/// - Common Wombat (*Vombatus ursinus*): 0.98 m length × 0.36 m height
/// - Platypus (*Ornithorhynchus anatinus*): 0.50 m length × 0.15 m height
/// - Wedge-Tailed Eagle (*Aquila audax*): 2.10 m wingspan
/// - Canopy trees (*Corymbia*, *Eucalyptus*): 12 m – 22 m height
/// In `build_sector_10d`, multiplying canonical metric coordinates by `SECTOR_METRIC_SCALE`
/// maps the 96 m focal patch into the `[-1.95, +1.95]` normalized sector container.
pub const SECTOR_METRIC_SCALE: f32 = 0.045;

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
    pub water_y: f32,
    pub scene_bounds_min: [f32; 3],
    pub scene_bounds_max: [f32; 3],
    pub rover_vertex_range: std::ops::Range<usize>,
    pub rover_local_positions: Vec<[f32; 3]>,
    pub walker_vertex_range: std::ops::Range<usize>,
    pub walker_local_positions: Vec<[f32; 3]>,
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

fn crc32_ieee(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in bytes {
        crc ^= b as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn write_png_chunk(out: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let crc_start = out.len();
    out.extend_from_slice(chunk_type);
    out.extend_from_slice(data);
    let crc = crc32_ieee(&out[crc_start..]);
    out.extend_from_slice(&crc.to_be_bytes());
}

fn zlib_compress_png_scanlines(width: u32, height: u32, rgba: &[u8], use_sub_filter: bool) -> Vec<u8> {
    use std::io::Write;
    let stride = (width * 4) as usize;
    let mut raw = Vec::with_capacity((stride + 1) * (height as usize));
    if use_sub_filter {
        for y in 0..(height as usize) {
            raw.push(1u8); // PNG Filter Type 1: Sub
            let row = &rgba[y * stride..(y + 1) * stride];
            for i in 0..stride {
                let left = if i >= 4 { row[i - 4] } else { 0 };
                raw.push(row[i].wrapping_sub(left));
            }
        }
    } else {
        for y in 0..(height as usize) {
            raw.push(0u8); // PNG Filter Type 0: None (optimal for sparse transparent delta frames)
            raw.extend_from_slice(&rgba[y * stride..(y + 1) * stride]);
        }
    }
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&raw).expect("zlib write");
    encoder.finish().expect("zlib finish")
}

/// Encode a multi-frame RGBA8 sequence into a standards-compliant Animated PNG (`APNG`, ISO/IEC 15948)
/// with `acTL`, `fcTL`, `IDAT`, and delta-blended `fdAT` (`APNG_BLEND_OP_OVER`) chunks.
/// Plays natively at full 24-bit RGBA color inside any standard `<img src="...png">` with Zero V8 / Zero JS.
fn encode_rgba8_frames_to_apng(
    width: u32,
    height: u32,
    frames: &[Vec<u8>],
    delay_num: u16,
    delay_den: u16,
) -> Vec<u8> {
    if frames.len() <= 1 {
        return encode_rgba8_to_png(width, height, &frames[0]);
    }
    let mut out = Vec::with_capacity(frames[0].len() * 2);
    out.extend_from_slice(b"\x89PNG\r\n\x1a\n");

    // 1. IHDR
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]); // 8-bit RGBA
    write_png_chunk(&mut out, b"IHDR", &ihdr);

    // 2. acTL (Animation Control Chunk)
    let mut actl = Vec::with_capacity(8);
    actl.extend_from_slice(&(frames.len() as u32).to_be_bytes());
    actl.extend_from_slice(&0u32.to_be_bytes()); // 0 = infinite loop
    write_png_chunk(&mut out, b"acTL", &actl);

    let mut seq_num = 0u32;
    let mut canvas_rgba = frames[0].clone();
    let mut delta_rgba = vec![0u8; frames[0].len()];

    for (f_idx, rgba) in frames.iter().enumerate() {
        // fcTL (Frame Control Chunk — 26 bytes)
        let mut fctl = Vec::with_capacity(26);
        fctl.extend_from_slice(&seq_num.to_be_bytes());
        seq_num += 1;
        fctl.extend_from_slice(&width.to_be_bytes());
        fctl.extend_from_slice(&height.to_be_bytes());
        fctl.extend_from_slice(&0u32.to_be_bytes()); // x_offset
        fctl.extend_from_slice(&0u32.to_be_bytes()); // y_offset
        fctl.extend_from_slice(&delay_num.to_be_bytes());
        fctl.extend_from_slice(&delay_den.to_be_bytes());
        fctl.push(0u8); // dispose_op = APNG_DISPOSE_OP_NONE (0)
        fctl.push(if f_idx == 0 { 0u8 } else { 1u8 }); // blend_op = SOURCE (0) for frame 0, OVER (1) for delta frames
        write_png_chunk(&mut out, b"fcTL", &fctl);

        if f_idx == 0 {
            let zdata = zlib_compress_png_scanlines(width, height, rgba, true);
            write_png_chunk(&mut out, b"IDAT", &zdata);
        } else {
            // Compute delta against current composited canvas_rgba
            for px in (0..rgba.len()).step_by(4) {
                let dr = (rgba[px] as i16 - canvas_rgba[px] as i16).abs();
                let dg = (rgba[px + 1] as i16 - canvas_rgba[px + 1] as i16).abs();
                let db = (rgba[px + 2] as i16 - canvas_rgba[px + 2] as i16).abs();
                if dr <= 1 && dg <= 1 && db <= 1 {
                    delta_rgba[px] = 0;
                    delta_rgba[px + 1] = 0;
                    delta_rgba[px + 2] = 0;
                    delta_rgba[px + 3] = 0;
                } else {
                    delta_rgba[px] = rgba[px];
                    delta_rgba[px + 1] = rgba[px + 1];
                    delta_rgba[px + 2] = rgba[px + 2];
                    delta_rgba[px + 3] = 255;
                    canvas_rgba[px] = rgba[px];
                    canvas_rgba[px + 1] = rgba[px + 1];
                    canvas_rgba[px + 2] = rgba[px + 2];
                    canvas_rgba[px + 3] = 255;
                }
            }
            let zdata = zlib_compress_png_scanlines(width, height, &delta_rgba, false);
            let mut fdat = Vec::with_capacity(4 + zdata.len());
            fdat.extend_from_slice(&seq_num.to_be_bytes());
            seq_num += 1;
            fdat.extend_from_slice(&zdata);
            write_png_chunk(&mut out, b"fdAT", &fdat);
        }
    }

    write_png_chunk(&mut out, b"IEND", &[]);
    out
}

#[inline]
fn hash21(x: f32, y: f32) -> f32 {
    let qx = (x * 127.1 + y * 311.7).sin() * 43758.5453;
    qx - qx.floor()
}

/// Synthesize 4 high-detail 256×256 procedural RGBA8 textures for Foliage Venation, Bark Micro-Relief,
/// Riverbed Pebbles/Lithology, and SLGA Forest Loam / Moss / Litter Micro-Relief.
pub fn build_procedural_textures() -> Vec<ProceduralTextureAsset> {
    let w = 256u32;
    let h = 256u32;
    let mut out = Vec::with_capacity(4);

    // Smooth value noise + 4-octave FBM helper for natural organic micro-relief (zero periodic striping)
    let value_noise = |x: f32, y: f32| -> f32 {
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
    };
    let fbm4 = |mut x: f32, mut y: f32| -> f32 {
        let mut v = 0.0_f32;
        let mut amp = 0.5_f32;
        for _ in 0..4 {
            v += amp * value_noise(x, y);
            x = x * 2.03 + 17.1;
            y = y * 2.01 + 31.7;
            amp *= 0.5;
        }
        v
    };

    // 1. Sclerophyllous Leaf Venation & Cuticle Texture (`LeafVascularSkeleton` midrib + lateral veins + oil glands)
    let mut leaf_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / (w - 1) as f32;
            let v = y as f32 / (h - 1) as f32;
            let midrib_dist = (u - 0.5).abs();
            let midrib = (1.0 - (midrib_dist / 0.035).clamp(0.0, 1.0)).powf(1.8);
            let vein_coord = (v * 22.0 - midrib_dist * 14.0).sin().abs();
            let secondary_vein = (1.0 - (vein_coord / 0.14).clamp(0.0, 1.0))
                * (1.0 - (midrib_dist * 1.8).clamp(0.0, 1.0));
            let cell_grain = fbm4(u * 28.0, v * 28.0) * 0.14;
            let edge_darken = 1.0 - 0.14 * (midrib_dist * 2.0).powf(2.2);
            let r = ((0.78 + 0.14 * midrib + 0.07 * secondary_vein + cell_grain) * edge_darken)
                .clamp(0.0, 1.0);
            let g = ((0.86 + 0.11 * midrib + 0.06 * secondary_vein + cell_grain) * edge_darken)
                .clamp(0.0, 1.0);
            let b = ((0.74 + 0.07 * midrib + 0.04 * secondary_vein + cell_grain * 0.6) * edge_darken)
                .clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            leaf_rgba[idx] = (r * 255.0) as u8;
            leaf_rgba[idx + 1] = (g * 255.0) as u8;
            leaf_rgba[idx + 2] = (b * 255.0) as u8;
            leaf_rgba[idx + 3] = 255;
        }
    }

    // 2. Eucalyptus / Ghost Gum / Snow Gum Smooth & Fibrous Bark Texture
    let mut bark_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / (w - 1) as f32;
            let v = y as f32 / (h - 1) as f32;
            let grain = fbm4(u * 24.0, v * 8.0);
            let mottling = fbm4(u * 6.5 + 4.2, v * 6.5 + 9.1);
            let shade = (0.76 + 0.16 * grain + 0.14 * mottling - 0.06).clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            bark_rgba[idx] = (shade * 252.0) as u8;
            bark_rgba[idx + 1] = (shade * 246.0) as u8;
            bark_rgba[idx + 2] = (shade * 238.0) as u8;
            bark_rgba[idx + 3] = 255;
        }
    }

    // 3. Riverbed Voronoi Pebbles, Imbricated Cobbles, Alluvial Sand & Lithology Texture
    let mut pebble_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / w as f32 * 14.0;
            let v = y as f32 / h as f32 * 14.0;
            let ix = u.floor();
            let iy = v.floor();
            let fx = u - ix;
            let fy = v - iy;
            let mut min_d = 8.0_f32;
            let mut best_rnd = 0.5_f32;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let cx = ix + dx as f32;
                    let cy = iy + dy as f32;
                    let jx = dx as f32 + 0.18 + 0.64 * hash21(cx * 17.3, cy * 31.7) - fx;
                    let jy = dy as f32 + 0.18 + 0.64 * hash21(cx * 29.1, cy * 13.9) - fy;
                    let d = (jx * jx * 1.15 + jy * jy * 1.35).sqrt();
                    if d < min_d {
                        min_d = d;
                        best_rnd = hash21(cx * 13.7, cy * 29.3);
                    }
                }
            }
            let pebble_mask = (1.0 - (min_d / 0.56).clamp(0.0, 1.0)).powf(0.65);
            let (pr, pg, pb) = if best_rnd > 0.72 {
                (0.86, 0.83, 0.78) // quartz-feldspar grain
            } else if best_rnd > 0.44 {
                (0.80, 0.74, 0.67) // warm sandstone / ironstone grain
            } else if best_rnd < 0.24 {
                (0.72, 0.73, 0.75) // dark biotite / basalt grain
            } else {
                (0.78, 0.76, 0.72) // weathered granite grain
            };
            let rock_grain = 0.88 + 0.12 * fbm4(u * 3.5, v * 3.5);
            let r = ((0.76 * (1.0 - pebble_mask) + pr * pebble_mask) * rock_grain).clamp(0.0, 1.0);
            let g = ((0.74 * (1.0 - pebble_mask) + pg * pebble_mask) * rock_grain).clamp(0.0, 1.0);
            let b = ((0.70 * (1.0 - pebble_mask) + pb * pebble_mask) * rock_grain).clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            pebble_rgba[idx] = (r * 255.0) as u8;
            pebble_rgba[idx + 1] = (g * 255.0) as u8;
            pebble_rgba[idx + 2] = (b * 255.0) as u8;
            pebble_rgba[idx + 3] = 255;
        }
    }

    // 4. Isotropic Multi-Octave Forest Loam, Moss & Leaf-Litter Micro-Relief Texture (Zero striping!)
    let mut soil_rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let u = x as f32 / (w - 1) as f32;
            let v = y as f32 / (h - 1) as f32;
            // Seamless toroidal / multi-scale FBM so tiling across the terrain has zero seams or stripes
            let ax = (u * std::f32::consts::TAU).cos() * 3.2 + (v * std::f32::consts::TAU).sin() * 1.4;
            let ay = (v * std::f32::consts::TAU).cos() * 3.2 - (u * std::f32::consts::TAU).sin() * 1.4;
            let macro_patch = fbm4(ax + 5.0, ay + 5.0);
            let micro_litter = fbm4(u * 26.0, v * 26.0);
            let val = (0.82 + 0.14 * macro_patch + 0.10 * micro_litter - 0.06).clamp(0.0, 1.0);
            let idx = ((y * w + x) * 4) as usize;
            soil_rgba[idx] = (val * 248.0) as u8;
            soil_rgba[idx + 1] = (val * 245.0) as u8;
            soil_rgba[idx + 2] = (val * 234.0) as u8;
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
        let stacks = 4usize;
        let (req_v, req_t) = asymmetric_hull_capacity(slices, stacks);
        let mut hull_v = vec![Point3::new(0.0, 0.0, 0.0); req_v];
        let mut hull_t = vec![[0u32; 3]; req_t];
        // Full shaded foliage crown core (`AsymmetricHull`) with lacy outer 3D falcate leaf blades
        let core_scale = 0.82_f64;
        let params = AsymmetricHullParams {
            neg_x_radius: (rx as f64) * core_scale * 0.92,
            pos_x_radius: (rx as f64) * core_scale * 1.06,
            neg_y_radius: (rz as f64) * core_scale * 0.95,
            pos_y_radius: (rz as f64) * core_scale * 1.04,
            neg_x_height: 0.0,
            pos_x_height: (ry as f64) * 0.08,
            neg_y_height: (-ry as f64) * 0.05,
            pos_y_height: (ry as f64) * 0.06,
            bottom: Point3::new(0.0, 0.0, (-ry * 0.52) as f64),
            top: Point3::new(0.0, 0.0, (ry * 0.64) as f64),
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
        let total_blades = blade_count.max(12);
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
        let slices = 11usize;
        let stacks = 5usize;
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
            // 12 continuous arching 3D *Dicksonia antarctica* fern fronds (connected V-cross-section ribbons)
            let frond_count = 12usize;
            for f in 0..frond_count {
                let ang = (f as f32 / frond_count as f32) * std::f32::consts::TAU + seed * 0.37;
                let ca = ang.cos();
                let sa = ang.sin();
                let px = -sa;
                let pz = ca;
                let frond_len = height * 1.05;
                let segs = 7usize;
                for seg in 0..segs {
                    let t0 = seg as f32 / segs as f32;
                    let t1 = (seg + 1) as f32 / segs as f32;
                    let r0 = frond_len * t0;
                    let r1 = frond_len * t1;
                    let y0 = crown[1] + (t0 * std::f32::consts::PI * 0.82).sin() * height * 0.32
                        - t0 * t0 * height * 0.22;
                    let y1 = crown[1] + (t1 * std::f32::consts::PI * 0.82).sin() * height * 0.32
                        - t1 * t1 * height * 0.22;
                    let w0 = frond_len * 0.14 * (t0 * std::f32::consts::PI).sin().max(0.08);
                    let w1 = frond_len * 0.14 * (t1 * std::f32::consts::PI).sin().max(0.04);

                    let c0 = [crown[0] + ca * r0, y0, crown[2] + sa * r0];
                    let c1 = [crown[0] + ca * r1, y1, crown[2] + sa * r1];
                    let l0 = [c0[0] - px * w0, y0 - w0 * 0.16, c0[2] - pz * w0];
                    let r0_p = [c0[0] + px * w0, y0 - w0 * 0.16, c0[2] + pz * w0];
                    let l1 = [c1[0] - px * w1, y1 - w1 * 0.16, c1[2] - pz * w1];
                    let r1_p = [c1[0] + px * w1, y1 - w1 * 0.16, c1[2] + pz * w1];

                    let n_up = [ca * 0.22, 0.95, sa * 0.22];
                    let col0 = [
                        0.16 * (1.0 - t0) + 0.30 * t0,
                        0.40 * (1.0 - t0) + 0.58 * t0,
                        0.15 * (1.0 - t0) + 0.22 * t0,
                        1.0,
                    ];
                    let col1 = [
                        0.16 * (1.0 - t1) + 0.30 * t1,
                        0.40 * (1.0 - t1) + 0.58 * t1,
                        0.15 * (1.0 - t1) + 0.22 * t1,
                        1.0,
                    ];
                    let vc0 = self.push_vertex(c0, n_up, col0, [0.5, t0]);
                    let vl0 = self.push_vertex(l0, n_up, col0, [0.0, t0]);
                    let vr0 = self.push_vertex(r0_p, n_up, col0, [1.0, t0]);
                    let vc1 = self.push_vertex(c1, n_up, col1, [0.5, t1]);
                    let vl1 = self.push_vertex(l1, n_up, col1, [0.0, t1]);
                    let vr1 = self.push_vertex(r1_p, n_up, col1, [1.0, t1]);
                    self.push_tri(vc0, vl0, vl1);
                    self.push_tri(vc0, vl1, vc1);
                    self.push_tri(vc0, vc1, vr1);
                    self.push_tri(vc0, vr1, vr0);
                }
            }
        }
    }

    /// Add an articulated 3D Australian canopy tree (*Corymbia aparrerinja*, *Eucalyptus pauciflora*,
    /// *Eucalyptus regnans*, *Eucalyptus camaldulensis*, *Acacia pycnantha*, *Banksia serrata*)
    /// using `space_colonization_3d`, `pipe_model_radii` (Leonardo da Vinci / Shinozaki pipe model),
    /// Bishop-frame swept limbs, Cescatti 6-point `AsymmetricHull` canopy cores, and 3D falcate leaf blades!
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

        // 1. Generate 28 crown attractors in an asymmetric crown dome envelope
        let attractor_count = 28usize;
        let mut attractors = Vec::with_capacity(attractor_count);
        let golden_angle = 2.399_963_1_f64;
        let crown_center_y = (root[1] + height * 0.72) as f64;
        let crown_rx = (height * 0.38) as f64;
        let crown_ry = (height * 0.28) as f64;

        for i in 0..attractor_count {
            let fi = i as f64;
            let theta = fi * golden_angle + seed as f64;
            let t = (fi + 0.5) / (attractor_count as f64);
            let y_norm = 1.0 - t;
            let r_ring = (1.0 - y_norm * y_norm).max(0.06).sqrt();
            let jitter = 0.85 + 0.30 * hash21(i as f32, seed) as f64;
            let ax = (root[0] as f64) + (bend_x as f64) * 0.8 + theta.cos() * crown_rx * r_ring * jitter;
            let ay = crown_center_y + (y_norm - 0.2) * crown_ry * jitter;
            let az = (root[2] as f64) + (bend_z as f64) * 0.8 + theta.sin() * crown_rx * r_ring * jitter;
            attractors.push(Point3::new(ax, ay, az));
        }

        // 2. Trunk seed nodes
        let seed_nodes = [
            Point3::new(root[0] as f64, (root[1] - 0.015) as f64, root[2] as f64),
            Point3::new(
                (root[0] + bend_x * 0.35) as f64,
                (root[1] + height * 0.28) as f64,
                (root[2] + bend_z * 0.35) as f64,
            ),
            Point3::new(
                (root[0] + bend_x * 0.75) as f64,
                (root[1] + height * 0.54) as f64,
                (root[2] + bend_z * 0.75) as f64,
            ),
        ];
        let seed_parents = [ROOT_PARENT, 0, 1];

        let cfg = SpaceColonizationConfig {
            node_length: (height * 0.16) as f64,
            kill_radius: (height * 0.14) as f64,
            perception_radius: (height * 0.55) as f64,
            cone_angle_rad: std::f64::consts::PI * 0.95,
            min_attractors_per_bud: 1,
            max_iterations: 12,
        };

        let max_nodes = 48usize;
        let mut out_nodes = vec![Point3::default(); max_nodes];
        let mut out_parents = vec![0u32; max_nodes];

        let colonized = space_colonization_3d(
            &attractors,
            &seed_nodes,
            &seed_parents,
            &cfg,
            &mut out_nodes,
            &mut out_parents,
        );

        if let Ok(receipt) = colonized {
            let node_count = receipt.node_count;
            if node_count > seed_nodes.len() {
                let mut child_counts = vec![0u32; node_count];
                for i in 0..node_count {
                    let p = out_parents[i];
                    if p != ROOT_PARENT && (p as usize) < node_count && (p as usize) != i {
                        child_counts[p as usize] += 1;
                    }
                }
                let mut scratch = vec![0u32; node_count];
                let mut out_radii = vec![0.0f64; node_count];
                let tip_r = (height * 0.014).max(0.004) as f64;
                let _ = pipe_model_radii(
                    &out_parents[..node_count],
                    tip_r,
                    2.15,
                    &mut scratch,
                    &mut out_radii,
                );

                // Extrude branch segments from parent to child
                for i in 1..node_count {
                    let p_idx = out_parents[i] as usize;
                    if p_idx < node_count && p_idx != i {
                        let p0 = out_nodes[p_idx];
                        let p1 = out_nodes[i];
                        let r0 = out_radii[p_idx].max(tip_r);
                        let r1 = out_radii[i].max(tip_r);

                        let axis = [p0, p1];
                        let stations = [
                            ExtrusionStation { scale_x: r0, scale_y: r0, twist_rad: 0.0 },
                            ExtrusionStation { scale_x: r1, scale_y: r1, twist_rad: 0.3 },
                        ];
                        let sides = if p_idx == 0 { 8 } else if p_idx == 1 { 7 } else { 5 };
                        trunk_builder.add_bishop_swept_tube(&axis, &stations, sides, bark_base, bark_top);
                    }
                }

                // At terminal leaf tips attach Cescatti canopies with falcate leaf blades
                for i in seed_nodes.len()..node_count {
                    if child_counts[i] == 0 {
                        let tip_p = out_nodes[i];
                        let tip_pos = [tip_p.x as f32, tip_p.y as f32, tip_p.z as f32];
                        let crx = height * (0.22 + 0.08 * hash21(i as f32, seed));
                        let cry = height * (0.16 + 0.05 * hash21(seed, i as f32));
                        self.add_cescatti_canopy_with_leaf_blades(
                            tip_pos,
                            crx,
                            cry,
                            crx * 0.94,
                            leaf_dark,
                            leaf_lit,
                            12,
                            seed + i as f32 * 1.7,
                        );
                    }
                }

                // Apical crown dome at top of highest node
                let mut highest_node = out_nodes[0];
                for n in out_nodes.iter().take(node_count) {
                    if n.y > highest_node.y {
                        highest_node = *n;
                    }
                }
                self.add_cescatti_canopy_with_leaf_blades(
                    [highest_node.x as f32, highest_node.y as f32, highest_node.z as f32],
                    height * 0.26,
                    height * 0.18,
                    height * 0.25,
                    leaf_dark,
                    leaf_lit,
                    24,
                    seed + 12.3,
                );
                return;
            }
        }

        // Deterministic botanical bough architecture fallback
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

    /// Evaluate an analytical composite SDF on a local 3D grid, extract closed 2-manifold
    /// triangle mesh via `marching_cubes`, weld shared vertices, compute smooth analytical SDF
    /// central-difference normals, and transform into world coordinates with yaw and scale.
    fn append_smooth_sdf_body<F>(
        &mut self,
        bmin: [f32; 3],
        bmax: [f32; 3],
        res: (usize, usize, usize),
        pos: [f32; 3],
        yaw: f32,
        scale: f32,
        col_base: [f32; 4],
        col_top: [f32; 4],
        sdf: F,
    ) where
        F: Fn(Point3) -> f64,
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

            // Weld shared vertices with spatial hashing (0.5 mm binning)
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

            let eps = 0.003_f64;
            let cy = yaw.cos();
            let sy = yaw.sin();
            let span_y = (bmax[1] - bmin[1]).max(0.01);
            let base_idx = self.positions.len() as u32;

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
                let lnx = (gx / glen) as f32;
                let lny = (gy / glen) as f32;
                let lnz = (gz / glen) as f32;

                let rnx = lnx * cy - lnz * sy;
                let rny = lny;
                let rnz = lnx * sy + lnz * cy;

                let lx = uv.x as f32;
                let ly = uv.y as f32;
                let lz = uv.z as f32;

                let wx = pos[0] + (lx * cy - lz * sy) * scale;
                let wy = pos[1] + ly * scale;
                let wz = pos[2] + (lx * sy + lz * cy) * scale;

                let t = ((ly - bmin[1]) / span_y).clamp(0.0, 1.0);
                let col = [
                    col_base[0] * (1.0 - t) + col_top[0] * t,
                    col_base[1] * (1.0 - t) + col_top[1] * t,
                    col_base[2] * (1.0 - t) + col_top[2] * t,
                    1.0,
                ];
                let u = ((lx - bmin[0]) / (bmax[0] - bmin[0]).max(0.01)).clamp(0.0, 1.0);
                let v_coord = ((lz - bmin[2]) / (bmax[2] - bmin[2]).max(0.01)).clamp(0.0, 1.0);

                self.push_vertex([wx, wy, wz], [rnx, rny, rnz], col, [u, v_coord]);
            }

            for t in 0..tri_count {
                let i0 = welded_indices[t * 3];
                let i1 = welded_indices[t * 3 + 1];
                let i2 = welded_indices[t * 3 + 2];
                if i0 != i1 && i1 != i2 && i0 != i2 {
                    self.push_tri(base_idx + i0, base_idx + i1, base_idx + i2);
                }
            }
        }
    }

    /// Add an articulated 3D Australian Fauna Specimen with anatomically accurate 1:1 SI metric proportions:
    /// - *Macropus giganteus* Eastern Grey Kangaroo (1.65 m standing height)
    /// - *Vombatus ursinus* Common Wombat (0.98 m length × 0.36 m height)
    /// - *Ornithorhynchus anatinus* Platypus (0.50 m length × 0.15 m height)
    /// - *Aquila audax* Wedge-Tailed Eagle (2.10 m wingspan)
    /// - *Maccullochella peelii* Murray Cod (1.10 m length)
    /// - *Dromaius novaehollandiae* Emu (1.75 m height)
    /// - *Varanus varius* Lace Monitor (1.80 m length)
    /// - *Pseudophryne corroboree* Corroboree Frog (0.28 m length)
    /// Synthesizes organic bodies via `sdf_ops` C¹ smooth-union + `marching_cubes` isosurface extraction,
    /// vertex welding, and smooth analytical central-difference normals!
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
            // 1. Organic Marsupial Torso, Muscular Haunches & Neck via SDF Smooth-Union + Marching Cubes
            let bmin = [-0.32_f32, 0.20_f32, -0.42_f32];
            let bmax = [0.32_f32, 1.50_f32, 0.45_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (20, 22, 16),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_haunch = sdf_sphere(p, Point3::new(0.0, 0.48, -0.10), 0.24);
                    let d_abdomen = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.50, -0.06),
                        Point3::new(0.0, 0.80, 0.06),
                        0.18,
                    );
                    let d_chest = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.78, 0.06),
                        Point3::new(0.0, 1.08, 0.16),
                        0.14,
                    );
                    let d_neck = sdf_capsule(
                        p,
                        Point3::new(0.0, 1.06, 0.16),
                        Point3::new(0.0, 1.30, 0.25),
                        0.095,
                    );
                    let d_head = sdf_capsule(
                        p,
                        Point3::new(0.0, 1.28, 0.24),
                        Point3::new(0.0, 1.32, 0.42),
                        0.068,
                    );
                    let d_body = sdf_smooth_union(d_haunch, d_abdomen, 0.12);
                    let d_torso = sdf_smooth_union(d_body, d_chest, 0.10);
                    let d_upper = sdf_smooth_union(d_torso, d_neck, 0.09);
                    sdf_smooth_union(d_upper, d_head, 0.08)
                },
            );

            // 2. Thick Tapered Pentapedal Tail resting on ground behind (-Z)
            let s = scale as f64;
            let tail_axis = [
                to_world(0.0, 0.42, -0.22),
                to_world(0.0, 0.20, -0.48),
                to_world(0.0, 0.05, -0.76),
                to_world(0.0, 0.02, -1.05),
            ];
            let tail_st = [
                ExtrusionStation { scale_x: 0.11 * s, scale_y: 0.11 * s, twist_rad: 0.0 },
                ExtrusionStation { scale_x: 0.085 * s, scale_y: 0.085 * s, twist_rad: 0.0 },
                ExtrusionStation { scale_x: 0.052 * s, scale_y: 0.052 * s, twist_rad: 0.0 },
                ExtrusionStation { scale_x: 0.024 * s, scale_y: 0.024 * s, twist_rad: 0.0 },
            ];
            self.add_bishop_swept_tube(&tail_axis, &tail_st, 12, coat_dark, coat_lit);

            // 3. Anatomical Z-Folded Saltatorial Hindlimbs & Horizontal 4th-Metatarsal Foot Pads flat on ground
            for side in [-1.0_f32, 1.0_f32] {
                let leg_axis = [
                    to_world(side * 0.18, 0.48, -0.10), // femur head / haunch
                    to_world(side * 0.20, 0.36, 0.10),  // knee
                    to_world(side * 0.18, 0.12, -0.16), // raised calcaneum hock
                    to_world(side * 0.16, 0.03, -0.10), // ankle
                    to_world(side * 0.16, 0.02, 0.26),  // plantigrade 4th metatarsal toe pad flat on ground
                ];
                let leg_st = [
                    ExtrusionStation { scale_x: 0.10 * s, scale_y: 0.09 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.07 * s, scale_y: 0.065 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.048 * s, scale_y: 0.048 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.040 * s, scale_y: 0.035 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.025 * s, scale_y: 0.022 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&leg_axis, &leg_st, 12, coat_dark, coat_lit);

                // Short tucked forelimbs held against chest
                let arm_axis = [
                    to_world(side * 0.11, 0.90, 0.14),
                    to_world(side * 0.12, 0.72, 0.18),
                    to_world(side * 0.08, 0.64, 0.28),
                ];
                let arm_st = [
                    ExtrusionStation { scale_x: 0.032 * s, scale_y: 0.032 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.024 * s, scale_y: 0.024 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.016 * s, scale_y: 0.016 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&arm_axis, &arm_st, 10, coat_dark, coat_lit);

                // Long erect rotated pinnae ears reaching standing height 1.65 m
                let ear_axis = [
                    to_world(side * 0.06, 1.36, 0.25),
                    to_world(side * 0.10, 1.65, 0.22),
                ];
                let ear_st = [
                    ExtrusionStation { scale_x: 0.032 * s, scale_y: 0.022 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.010 * s, scale_y: 0.008 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&ear_axis, &ear_st, 8, coat_dark, coat_lit);
            }
        } else if archetype == "wombat" {
            // Low barrel-shaped fossorial body (0.36 m height, 0.98 m length)
            let bmin = [-0.26_f32, 0.04_f32, -0.52_f32];
            let bmax = [0.26_f32, 0.38_f32, 0.52_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (20, 16, 20),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_torso = sdf_rounded_box(
                        p,
                        Point3::new(0.0, 0.21, -0.06),
                        Point3::new(0.20, 0.14, 0.32),
                        0.08,
                    );
                    let d_head = sdf_rounded_box(
                        p,
                        Point3::new(0.0, 0.20, 0.34),
                        Point3::new(0.14, 0.11, 0.13),
                        0.06,
                    );
                    sdf_smooth_union(d_torso, d_head, 0.08)
                },
            );

            // 4 short stout digging legs resting on ground at y = 0.02
            let s = scale as f64;
            for side in [-1.0_f32, 1.0_f32] {
                // Front digging legs
                let f_leg = [
                    to_world(side * 0.17, 0.18, 0.24),
                    to_world(side * 0.18, 0.02, 0.25),
                ];
                let f_st = [
                    ExtrusionStation { scale_x: 0.060 * s, scale_y: 0.060 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.050 * s, scale_y: 0.045 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&f_leg, &f_st, 10, coat_dark, coat_lit);

                // Rear digging legs
                let r_leg = [
                    to_world(side * 0.17, 0.18, -0.22),
                    to_world(side * 0.18, 0.02, -0.23),
                ];
                let r_st = [
                    ExtrusionStation { scale_x: 0.065 * s, scale_y: 0.065 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.052 * s, scale_y: 0.048 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&r_leg, &r_st, 10, coat_dark, coat_lit);

                // Small rounded ears
                let ear = [
                    to_world(side * 0.11, 0.30, 0.30),
                    to_world(side * 0.14, 0.36, 0.28),
                ];
                let ear_st = [
                    ExtrusionStation { scale_x: 0.025 * s, scale_y: 0.025 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.012 * s, scale_y: 0.012 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&ear, &ear_st, 8, coat_dark, coat_lit);
            }
        } else if archetype == "platypus" || archetype == "aquatic" {
            // Streamlined semi-aquatic body (0.50 m length, 0.15 m height)
            let bmin = [-0.12_f32, 0.01_f32, -0.27_f32];
            let bmax = [0.12_f32, 0.16_f32, 0.27_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (18, 14, 20),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_torso = sdf_rounded_box(
                        p,
                        Point3::new(0.0, 0.08, 0.0),
                        Point3::new(0.075, 0.045, 0.14),
                        0.03,
                    );
                    let d_bill = sdf_rounded_box(
                        p,
                        Point3::new(0.0, 0.07, 0.19),
                        Point3::new(0.045, 0.012, 0.05),
                        0.008,
                    );
                    let d_tail = sdf_rounded_box(
                        p,
                        Point3::new(0.0, 0.07, -0.18),
                        Point3::new(0.065, 0.016, 0.07),
                        0.01,
                    );
                    let d_tb = sdf_smooth_union(d_torso, d_bill, 0.03);
                    sdf_smooth_union(d_tb, d_tail, 0.035)
                },
            );

            // 4 webbed paddling feet
            let s = scale as f64;
            for side in [-1.0_f32, 1.0_f32] {
                let f_paddle = [
                    to_world(side * 0.08, 0.05, 0.08),
                    to_world(side * 0.14, 0.02, 0.11),
                ];
                let f_st = [
                    ExtrusionStation { scale_x: 0.022 * s, scale_y: 0.012 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.032 * s, scale_y: 0.008 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&f_paddle, &f_st, 8, coat_dark, coat_lit);

                let r_paddle = [
                    to_world(side * 0.08, 0.05, -0.08),
                    to_world(side * 0.13, 0.02, -0.10),
                ];
                let r_st = [
                    ExtrusionStation { scale_x: 0.020 * s, scale_y: 0.010 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.028 * s, scale_y: 0.007 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&r_paddle, &r_st, 8, coat_dark, coat_lit);
            }
        } else if archetype == "raptor" {
            // Wedge-Tailed Eagle (*Aquila audax*) - 2.10 m wingspan, hooked beak, diamond wedge tail
            let bmin = [-0.14_f32, -0.06_f32, -0.32_f32];
            let bmax = [0.14_f32, 0.14_f32, 0.32_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (16, 14, 18),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_body = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.02, -0.18),
                        Point3::new(0.0, 0.04, 0.14),
                        0.075,
                    );
                    let d_head = sdf_sphere(p, Point3::new(0.0, 0.06, 0.18), 0.045);
                    let d_beak = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.05, 0.20),
                        Point3::new(0.0, 0.02, 0.25),
                        0.018,
                    );
                    let d_bh = sdf_smooth_union(d_body, d_head, 0.04);
                    sdf_smooth_union(d_bh, d_beak, 0.025)
                },
            );

            // Cambered 2.10 m wingspan airfoils & primary flight feathers
            let n_up = [0.0, 1.0, 0.0];
            for side in [-1.0_f32, 1.0_f32] {
                let p_shoulder = to_world_f32(side * 0.08, 0.03, 0.08);
                let p_wrist = to_world_f32(side * 0.55, 0.09, 0.14);
                let p_elbow_te = to_world_f32(side * 0.44, 0.05, -0.14);
                let p_root_te = to_world_f32(side * 0.08, 0.01, -0.18);
                let p_tip = to_world_f32(side * 1.05, 0.16, 0.02);
                let p_tip_te = to_world_f32(side * 0.85, 0.11, -0.16);

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

            // Authentic wedge-shaped diamond tail fan (*Aquila audax*)
            let t_root = to_world_f32(0.0, 0.01, -0.22);
            let t_l = to_world_f32(-0.22, -0.02, -0.52);
            let t_c = to_world_f32(0.0, -0.03, -0.65);
            let t_r = to_world_f32(0.22, -0.02, -0.52);
            let vt0 = self.push_vertex(t_root, n_up, coat_dark, [0.5, 0.0]);
            let vt1 = self.push_vertex(t_l, n_up, coat_dark, [0.0, 0.8]);
            let vt2 = self.push_vertex(t_c, n_up, coat_lit, [0.5, 1.0]);
            let vt3 = self.push_vertex(t_r, n_up, coat_dark, [1.0, 0.8]);
            self.push_tri(vt0, vt1, vt2);
            self.push_tri(vt0, vt2, vt3);
        } else if archetype == "cod" {
            // Murray Cod (*Maccullochella peelii*) - 1.10 m length
            let bmin = [-0.18_f32, 0.0_f32, -0.55_f32];
            let bmax = [0.18_f32, 0.28_f32, 0.55_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (16, 16, 20),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_body = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.12, -0.38),
                        Point3::new(0.0, 0.14, 0.28),
                        0.13,
                    );
                    let d_head = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.14, 0.22),
                        Point3::new(0.0, 0.13, 0.48),
                        0.10,
                    );
                    sdf_smooth_union(d_body, d_head, 0.08)
                },
            );
            // Caudal tail fin
            let n_up = [0.0, 1.0, 0.0];
            let t0 = to_world_f32(0.0, 0.12, -0.38);
            let t_top = to_world_f32(0.0, 0.24, -0.55);
            let t_bot = to_world_f32(0.0, 0.02, -0.55);
            let vt0 = self.push_vertex(t0, n_up, coat_dark, [0.0, 0.5]);
            let vt1 = self.push_vertex(t_top, n_up, coat_lit, [1.0, 1.0]);
            let vt2 = self.push_vertex(t_bot, n_up, coat_dark, [1.0, 0.0]);
            self.push_tri(vt0, vt1, vt2);
            self.push_tri(vt0, vt2, vt1);
        } else if archetype == "emu" {
            // Emu (*Dromaius novaehollandiae*) - 1.75 m height cursorial bird
            let bmin = [-0.28_f32, 0.80_f32, -0.38_f32];
            let bmax = [0.28_f32, 1.75_f32, 0.42_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (18, 22, 18),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_body = sdf_sphere(p, Point3::new(0.0, 0.98, -0.06), 0.28);
                    let d_neck = sdf_capsule(
                        p,
                        Point3::new(0.0, 1.08, 0.10),
                        Point3::new(0.0, 1.58, 0.22),
                        0.065,
                    );
                    let d_head = sdf_sphere(p, Point3::new(0.0, 1.62, 0.24), 0.06);
                    let d_beak = sdf_capsule(
                        p,
                        Point3::new(0.0, 1.62, 0.26),
                        Point3::new(0.0, 1.60, 0.38),
                        0.018,
                    );
                    let d_bn = sdf_smooth_union(d_body, d_neck, 0.10);
                    let d_bnh = sdf_smooth_union(d_bn, d_head, 0.05);
                    sdf_smooth_union(d_bnh, d_beak, 0.02)
                },
            );
            // Long cursorial stilt legs down to ground y = 0.0
            let s = scale as f64;
            for side in [-1.0_f32, 1.0_f32] {
                let leg_axis = [
                    to_world(side * 0.12, 0.96, -0.04),
                    to_world(side * 0.13, 0.52, 0.02),
                    to_world(side * 0.12, 0.02, 0.06),
                ];
                let leg_st = [
                    ExtrusionStation { scale_x: 0.045 * s, scale_y: 0.045 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.025 * s, scale_y: 0.025 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.018 * s, scale_y: 0.018 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&leg_axis, &leg_st, 8, coat_dark, coat_lit);
            }
        } else if archetype == "monitor" {
            // Lace Monitor (*Varanus varius*) - 1.80 m length sprawling lizard
            let bmin = [-0.22_f32, 0.02_f32, -0.45_f32];
            let bmax = [0.22_f32, 0.24_f32, 0.65_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (18, 14, 22),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_body = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.10, -0.28),
                        Point3::new(0.0, 0.11, 0.30),
                        0.09,
                    );
                    let d_head = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.11, 0.28),
                        Point3::new(0.0, 0.09, 0.58),
                        0.055,
                    );
                    sdf_smooth_union(d_body, d_head, 0.06)
                },
            );
            // Long whip-like tail
            let s = scale as f64;
            let tail_axis = [
                to_world(0.0, 0.09, -0.28),
                to_world(0.08, 0.06, -0.65),
                to_world(-0.12, 0.04, -1.05),
                to_world(0.05, 0.02, -1.22),
            ];
            let tail_st = [
                ExtrusionStation { scale_x: 0.065 * s, scale_y: 0.065 * s, twist_rad: 0.0 },
                ExtrusionStation { scale_x: 0.042 * s, scale_y: 0.042 * s, twist_rad: 0.0 },
                ExtrusionStation { scale_x: 0.024 * s, scale_y: 0.024 * s, twist_rad: 0.0 },
                ExtrusionStation { scale_x: 0.010 * s, scale_y: 0.010 * s, twist_rad: 0.0 },
            ];
            self.add_bishop_swept_tube(&tail_axis, &tail_st, 8, coat_dark, coat_lit);
            // 4 sprawling lateral legs
            for side in [-1.0_f32, 1.0_f32] {
                let f_leg = [
                    to_world(side * 0.08, 0.10, 0.24),
                    to_world(side * 0.18, 0.08, 0.28),
                    to_world(side * 0.22, 0.02, 0.32),
                ];
                let f_st = [
                    ExtrusionStation { scale_x: 0.035 * s, scale_y: 0.035 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.025 * s, scale_y: 0.025 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.016 * s, scale_y: 0.016 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&f_leg, &f_st, 8, coat_dark, coat_lit);

                let r_leg = [
                    to_world(side * 0.08, 0.10, -0.22),
                    to_world(side * 0.20, 0.08, -0.24),
                    to_world(side * 0.24, 0.02, -0.20),
                ];
                let r_st = [
                    ExtrusionStation { scale_x: 0.038 * s, scale_y: 0.038 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.026 * s, scale_y: 0.026 * s, twist_rad: 0.0 },
                    ExtrusionStation { scale_x: 0.016 * s, scale_y: 0.016 * s, twist_rad: 0.0 },
                ];
                self.add_bishop_swept_tube(&r_leg, &r_st, 8, coat_dark, coat_lit);
            }
        } else if archetype == "frog" {
            // Southern Corroboree Frog (*Pseudophryne corroboree*) - 0.28 m length
            let bmin = [-0.10_f32, 0.01_f32, -0.16_f32];
            let bmax = [0.10_f32, 0.12_f32, 0.16_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (14, 14, 16),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_body = sdf_sphere(p, Point3::new(0.0, 0.05, -0.02), 0.075);
                    let d_head = sdf_sphere(p, Point3::new(0.0, 0.055, 0.07), 0.052);
                    sdf_smooth_union(d_body, d_head, 0.04)
                },
            );
        } else {
            // Streamlined generic body via smooth SDF
            let bmin = [-0.18_f32, 0.02_f32, -0.40_f32];
            let bmax = [0.18_f32, 0.32_f32, 0.40_f32];
            self.append_smooth_sdf_body(
                bmin,
                bmax,
                (16, 16, 18),
                pos,
                yaw,
                scale,
                coat_dark,
                coat_lit,
                |p| {
                    let d_body = sdf_capsule(
                        p,
                        Point3::new(0.0, 0.14, -0.24),
                        Point3::new(0.0, 0.18, 0.24),
                        0.14,
                    );
                    let d_head = sdf_sphere(p, Point3::new(0.0, 0.20, 0.32), 0.11);
                    sdf_smooth_union(d_body, d_head, 0.06)
                },
            );
        }
    }

    /// Add an axis-aligned 3D box in local coordinates `[min_xyz, max_xyz]` with outward normals and vertex colors.
    fn add_local_box(&mut self, min: [f32; 3], max: [f32; 3], col_bottom: [f32; 4], col_top: [f32; 4]) {
        let faces: [([f32; 3], [[f32; 3]; 4]); 6] = [
            // +Y top
            ([0.0, 1.0, 0.0], [[min[0], max[1], max[2]], [max[0], max[1], max[2]], [max[0], max[1], min[2]], [min[0], max[1], min[2]]]),
            // -Y bottom
            ([0.0, -1.0, 0.0], [[min[0], min[1], min[2]], [max[0], min[1], min[2]], [max[0], min[1], max[2]], [min[0], min[1], max[2]]]),
            // +X right
            ([1.0, 0.0, 0.0], [[max[0], min[1], max[2]], [max[0], min[1], min[2]], [max[0], max[1], min[2]], [max[0], max[1], max[2]]]),
            // -X left
            ([-1.0, 0.0, 0.0], [[min[0], min[1], min[2]], [min[0], min[1], max[2]], [min[0], max[1], max[2]], [min[0], max[1], min[2]]]),
            // +Z front
            ([0.0, 0.0, 1.0], [[min[0], min[1], max[2]], [max[0], min[1], max[2]], [max[0], max[1], max[2]], [min[0], max[1], max[2]]]),
            // -Z rear
            ([0.0, 0.0, -1.0], [[max[0], min[1], min[2]], [min[0], min[1], min[2]], [min[0], max[1], min[2]], [max[0], max[1], min[2]]]),
        ];
        for (norm, corners) in faces {
            let v0 = self.push_vertex(corners[0], norm, if corners[0][1] >= (min[1] + max[1]) * 0.5 { col_top } else { col_bottom }, [0.0, 0.0]);
            let v1 = self.push_vertex(corners[1], norm, if corners[1][1] >= (min[1] + max[1]) * 0.5 { col_top } else { col_bottom }, [1.0, 0.0]);
            let v2 = self.push_vertex(corners[2], norm, if corners[2][1] >= (min[1] + max[1]) * 0.5 { col_top } else { col_bottom }, [1.0, 1.0]);
            let v3 = self.push_vertex(corners[3], norm, if corners[3][1] >= (min[1] + max[1]) * 0.5 { col_top } else { col_bottom }, [0.0, 1.0]);
            self.push_tri(v0, v1, v2);
            self.push_tri(v0, v2, v3);
        }
    }

    /// Build a true-scale (`4.85 m × 1.88 m × 1.92 m` at `scale = 1.0`) CAD/lofted 3D 4WD Field Research Rover
    /// using native computational geometry kernels:
    /// - `wheel_panel::wheel_clearance_panel_into` (`WheelPanel`) for side panels with circular wheel-clearance cutouts
    /// - `authoring::loft_convex_sections_into` for sculpted bonnet, dual-cab greenhouse & field research canopy
    /// - `parametric_cad::revolve_profile` (24 radial segments) for toric off-road tyres & alloy wheel rims
    /// - `sweep_extrusion::extrude_profile_along_path` for Australian tubular steel bullbar, snorkel, and roof rack
    fn add_field_rover_4wd_local(&mut self, scale: f32) {
        let s = scale;

        // 1. Rugged dark-steel chassis ladder frame & underbody bash/skid plate
        // Ground clearance = 0.24 * s, chassis rails extend from x = -2.425 * s to x = +2.25 * s
        self.add_local_box(
            [-2.425 * s, 0.24 * s, -0.58 * s],
            [2.25 * s, 0.38 * s, 0.58 * s],
            [0.15, 0.16, 0.18, 1.0],
            [0.24, 0.26, 0.28, 1.0],
        );

        // 2. Body Side Panels with Circular Wheel-Clearance Arches via `wheel_clearance_panel_into`
        // Wheelbase = 2.85 m (axles at x = -1.425 * s and x = +1.425 * s), clearance radius = 0.48 * s
        let wheel_axles = [-1.425 * s, 1.425 * s];
        for &side_z in &[-0.86_f32, 0.86_f32] {
            let recipe = WheelPanel {
                center_x: side_z * s,
                thickness: 0.06 * s,
                z: [-2.38 * s, 2.38 * s],
                bottom: 0.36 * s,
                top: [1.15 * s, 1.15 * s],
                wheel_y: 0.41 * s,
                clearance_radius: 0.48 * s,
                axles: &wheel_axles,
                segments: 16,
            };
            let sections = recipe.segments as usize + 1;
            let mut panel_v = vec![[0.0f32; 3]; sections * 10];
            let mut panel_t = vec![[0u32; 3]; sections * 8];
            if let Ok(receipt) = wheel_clearance_panel_into(recipe, &mut panel_v, &mut panel_t) {
                let base_idx = self.positions.len() as u32;
                let col_bot = [0.76, 0.72, 0.58, 1.0];
                let col_top = [0.92, 0.88, 0.76, 1.0];
                for p in &panel_v[..receipt.vertex_count] {
                    // In recipe: station is along Z, thickness along X, height along Y.
                    // Map to vehicle frame: vehicle X = station (pz), vehicle Y = py, vehicle Z = px
                    let vx = p[2];
                    let vy = p[1];
                    let vz = p[0];
                    let t = ((vy - 0.36 * s) / (0.79 * s).max(1e-4)).clamp(0.0, 1.0);
                    let col = [
                        col_bot[0] * (1.0 - t) + col_top[0] * t,
                        col_bot[1] * (1.0 - t) + col_top[1] * t,
                        col_bot[2] * (1.0 - t) + col_top[2] * t,
                        1.0,
                    ];
                    let sign_z = if side_z > 0.0 { 1.0 } else { -1.0 };
                    let norm = [0.0, 0.15, sign_z * 0.98];
                    let u = ((vx + 2.38 * s) / (4.76 * s).max(1e-4)).clamp(0.0, 1.0);
                    self.push_vertex([vx, vy, vz], norm, col, [u, t]);
                }
                for tri in &panel_t[..receipt.triangle_count] {
                    if side_z > 0.0 {
                        self.push_tri(base_idx + tri[0], base_idx + tri[2], base_idx + tri[1]);
                    } else {
                        self.push_tri(base_idx + tri[0], base_idx + tri[1], base_idx + tri[2]);
                    }
                }
            }
        }

        // 3. Lofted Tapered Front Bonnet via `authoring::loft_convex_sections_into`
        // x: +0.65 .. +2.30 * s, y: 0.82 .. 1.18 * s
        // Sections are authored in XY plane ordered along Z, then mapped (x, y, z) -> (z, y, x)
        {
            let bonnet_stations = [
                (0.65 * s, 0.85 * s, 0.82 * s, 1.18 * s),
                (1.20 * s, 0.84 * s, 0.82 * s, 1.16 * s),
                (1.75 * s, 0.81 * s, 0.82 * s, 1.14 * s),
                (2.30 * s, 0.77 * s, 0.82 * s, 1.12 * s),
            ];
            let mut sections = Vec::with_capacity(bonnet_stations.len());
            for &(x_station, half_w, y_bot, y_top) in &bonnet_stations {
                sections.push(vec![
                    [-half_w, y_bot, x_station],
                    [half_w, y_bot, x_station],
                    [half_w, y_top, x_station],
                    [-half_w, y_top, x_station],
                ]);
            }
            let req_v = (sections.len() - 1) * 5 * 2 + 5 * 2;
            let req_t = sections.len() * 4 * 2;
            let mut out_v = vec![[0.0f32; 3]; req_v * 2 + 32];
            let mut out_t = vec![[0u32; 3]; req_t * 2 + 32];
            if let Ok(receipt) = loft_convex_sections_into(&sections, &mut out_v, &mut out_t) {
                let base_idx = self.positions.len() as u32;
                for p in &out_v[..receipt.vertex_count] {
                    let vx = p[2];
                    let vy = p[1];
                    let vz = p[0];
                    let t = ((vy - 0.82 * s) / (0.36 * s).max(1e-4)).clamp(0.0, 1.0);
                    let col = [
                        0.80 * (1.0 - t * 0.3) + 0.94 * (t * 0.3),
                        0.74 * (1.0 - t * 0.3) + 0.88 * (t * 0.3),
                        0.60 * (1.0 - t * 0.3) + 0.74 * (t * 0.3),
                        1.0,
                    ];
                    let nx = if vx > 2.25 * s { 0.95 } else { 0.15 };
                    let ny = if vy > 1.10 * s { 0.95 } else { 0.10 };
                    let nz = if vz.abs() > 0.70 * s { vz.signum() * 0.95 } else { 0.0 };
                    let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-4);
                    self.push_vertex([vx, vy, vz], [nx / nlen, ny / nlen, nz / nlen], col, [0.5, t]);
                }
                for tri in &out_t[..receipt.triangle_count] {
                    self.push_tri(base_idx + tri[0], base_idx + tri[2], base_idx + tri[1]);
                }
            }
        }

        // Front radiator grille insert & LED headlights
        self.add_local_box(
            [2.28 * s, 0.84 * s, -0.68 * s],
            [2.33 * s, 1.10 * s, 0.68 * s],
            [0.12, 0.14, 0.16, 1.0],
            [0.18, 0.20, 0.22, 1.0],
        );
        for side in [-1.0_f32, 1.0_f32] {
            // High-intensity LED headlamp bezels
            self.add_local_box(
                [2.31 * s, 0.90 * s, (side * 0.58 - 0.12) * s],
                [2.34 * s, 1.06 * s, (side * 0.58 + 0.12) * s],
                [0.96, 0.96, 0.88, 1.0],
                [1.00, 1.00, 0.95, 1.0],
            );
            // Amber turn signal indicators
            self.add_local_box(
                [2.30 * s, 0.92 * s, (side * 0.74 - 0.04) * s],
                [2.33 * s, 1.04 * s, (side * 0.74 + 0.04) * s],
                [0.98, 0.60, 0.05, 1.0],
                [1.00, 0.72, 0.10, 1.0],
            );
        }

        // 4. Lofted Sloped-Windshield Dual-Cab Greenhouse via `authoring::loft_convex_sections_into`
        // x: -0.95 .. +0.75 * s, y: 1.15 .. 1.86 * s
        {
            let cab_stations = [
                (-0.95 * s, 0.82 * s, 1.15 * s, 1.86 * s),
                (-0.35 * s, 0.81 * s, 1.15 * s, 1.86 * s),
                (0.20 * s, 0.80 * s, 1.15 * s, 1.84 * s),
                (0.65 * s, 0.82 * s, 1.15 * s, 1.38 * s),
                (0.75 * s, 0.84 * s, 1.15 * s, 1.20 * s),
            ];
            let mut sections = Vec::with_capacity(cab_stations.len());
            for &(x_station, half_w, y_bot, y_top) in &cab_stations {
                sections.push(vec![
                    [-half_w, y_bot, x_station],
                    [half_w, y_bot, x_station],
                    [half_w, y_top, x_station],
                    [-half_w, y_top, x_station],
                ]);
            }
            let req_v = (sections.len() - 1) * 5 * 2 + 5 * 2;
            let req_t = sections.len() * 4 * 2;
            let mut out_v = vec![[0.0f32; 3]; req_v * 2 + 32];
            let mut out_t = vec![[0u32; 3]; req_t * 2 + 32];
            if let Ok(receipt) = loft_convex_sections_into(&sections, &mut out_v, &mut out_t) {
                let base_idx = self.positions.len() as u32;
                for p in &out_v[..receipt.vertex_count] {
                    let vx = p[2];
                    let vy = p[1];
                    let vz = p[0];
                    let is_roof = vy > 1.82 * s;
                    let col = if is_roof {
                        [0.94, 0.95, 0.96, 1.0]
                    } else {
                        [0.18, 0.32, 0.46, 1.0]
                    };
                    let nx = if vx > 0.60 * s { 0.65 } else if vx < -0.90 * s { -0.85 } else { 0.0 };
                    let ny = if is_roof { 0.95 } else { 0.25 };
                    let nz = if vz.abs() > 0.75 * s { vz.signum() * 0.90 } else { 0.0 };
                    let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-4);
                    self.push_vertex([vx, vy, vz], [nx / nlen, ny / nlen, nz / nlen], col, [0.5, 0.5]);
                }
                for tri in &out_t[..receipt.triangle_count] {
                    self.push_tri(base_idx + tri[0], base_idx + tri[2], base_idx + tri[1]);
                }
            }
        }

        // 5. Lofted Rear Field Research Canopy & Utility Tray via `authoring::loft_convex_sections_into`
        // x: -2.35 .. -0.95 * s, y: 1.12 .. 1.84 * s
        {
            let canopy_stations = [
                (-2.35 * s, 0.84 * s, 1.12 * s, 1.84 * s),
                (-1.65 * s, 0.85 * s, 1.12 * s, 1.84 * s),
                (-0.96 * s, 0.84 * s, 1.12 * s, 1.84 * s),
            ];
            let mut sections = Vec::with_capacity(canopy_stations.len());
            for &(x_station, half_w, y_bot, y_top) in &canopy_stations {
                sections.push(vec![
                    [-half_w, y_bot, x_station],
                    [half_w, y_bot, x_station],
                    [half_w, y_top, x_station],
                    [-half_w, y_top, x_station],
                ]);
            }
            let req_v = (sections.len() - 1) * 5 * 2 + 5 * 2;
            let req_t = sections.len() * 4 * 2;
            let mut out_v = vec![[0.0f32; 3]; req_v * 2 + 32];
            let mut out_t = vec![[0u32; 3]; req_t * 2 + 32];
            if let Ok(receipt) = loft_convex_sections_into(&sections, &mut out_v, &mut out_t) {
                let base_idx = self.positions.len() as u32;
                for p in &out_v[..receipt.vertex_count] {
                    let vx = p[2];
                    let vy = p[1];
                    let vz = p[0];
                    let is_roof = vy > 1.80 * s;
                    let col = if is_roof {
                        [0.92, 0.93, 0.95, 1.0]
                    } else {
                        [0.78, 0.74, 0.60, 1.0]
                    };
                    let nx = if vx < -2.30 * s { -0.95 } else { 0.0 };
                    let ny = if is_roof { 0.95 } else { 0.10 };
                    let nz = if vz.abs() > 0.80 * s { vz.signum() * 0.95 } else { 0.0 };
                    let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-4);
                    self.push_vertex([vx, vy, vz], [nx / nlen, ny / nlen, nz / nlen], col, [0.5, 0.5]);
                }
                for tri in &out_t[..receipt.triangle_count] {
                    self.push_tri(base_idx + tri[0], base_idx + tri[2], base_idx + tri[1]);
                }
            }
        }

        // Hi-Vis Ecological Survey Orange Stripe along utility canopy sides
        for side in [-1.0_f32, 1.0_f32] {
            self.add_local_box(
                [-2.32 * s, 1.32 * s, (side * 0.855 - 0.015) * s],
                [-0.98 * s, 1.48 * s, (side * 0.855 + 0.015) * s],
                [0.94, 0.45, 0.10, 1.0],
                [0.98, 0.58, 0.16, 1.0],
            );
            // Red rear taillights on tailgate
            self.add_local_box(
                [-2.37 * s, 0.92 * s, (side * 0.72 - 0.08) * s],
                [-2.34 * s, 1.10 * s, (side * 0.72 + 0.08) * s],
                [0.85, 0.12, 0.10, 1.0],
                [0.98, 0.22, 0.18, 1.0],
            );
        }

        // 6. Australian Heavy Steel Tubular Bullbar via `add_bishop_swept_tube`
        // Polyline spanning front of vehicle: x = +2.32 .. +2.425 * s
        {
            let bar_axis = [
                Point3::new(2.32 * s as f64, 0.44 * s as f64, -0.84 * s as f64),
                Point3::new(2.41 * s as f64, 0.45 * s as f64, -0.62 * s as f64),
                Point3::new(2.425 * s as f64, 0.45 * s as f64, 0.0),
                Point3::new(2.41 * s as f64, 0.45 * s as f64, 0.62 * s as f64),
                Point3::new(2.32 * s as f64, 0.44 * s as f64, 0.84 * s as f64),
            ];
            let r_tube = (0.038 * s) as f64;
            let bar_st = vec![ExtrusionStation { scale_x: r_tube, scale_y: r_tube, twist_rad: 0.0 }; bar_axis.len()];
            self.add_bishop_swept_tube(&bar_axis, &bar_st, 8, [0.26, 0.28, 0.30, 1.0], [0.45, 0.48, 0.52, 1.0]);

            // Bullbar upper loop protecting radiator
            let loop_axis = [
                Point3::new(2.38 * s as f64, 0.48 * s as f64, -0.58 * s as f64),
                Point3::new(2.40 * s as f64, 0.88 * s as f64, -0.52 * s as f64),
                Point3::new(2.42 * s as f64, 0.98 * s as f64, 0.0),
                Point3::new(2.40 * s as f64, 0.88 * s as f64, 0.52 * s as f64),
                Point3::new(2.38 * s as f64, 0.48 * s as f64, 0.58 * s as f64),
            ];
            let loop_st = vec![ExtrusionStation { scale_x: r_tube * 0.85, scale_y: r_tube * 0.85, twist_rad: 0.0 }; loop_axis.len()];
            self.add_bishop_swept_tube(&loop_axis, &loop_st, 8, [0.26, 0.28, 0.30, 1.0], [0.45, 0.48, 0.52, 1.0]);
        }

        // 7. Raised Air-Intake Snorkel on Right A-Pillar
        {
            let snorkel_axis = [
                Point3::new(1.15 * s as f64, 0.92 * s as f64, 0.88 * s as f64),
                Point3::new(0.72 * s as f64, 1.20 * s as f64, 0.86 * s as f64),
                Point3::new(0.22 * s as f64, 1.76 * s as f64, 0.82 * s as f64),
                Point3::new(0.18 * s as f64, 1.86 * s as f64, 0.80 * s as f64),
            ];
            let r_snork = (0.035 * s) as f64;
            let snork_st = vec![ExtrusionStation { scale_x: r_snork, scale_y: r_snork, twist_rad: 0.0 }; snorkel_axis.len()];
            self.add_bishop_swept_tube(&snorkel_axis, &snork_st, 6, [0.12, 0.14, 0.16, 1.0], [0.18, 0.20, 0.22, 1.0]);
            // Ram-air intake scoop head
            self.add_local_box(
                [0.10 * s, 1.82 * s, 0.74 * s],
                [0.26 * s, 1.91 * s, 0.86 * s],
                [0.10, 0.12, 0.14, 1.0],
                [0.20, 0.22, 0.24, 1.0],
            );
        }

        // 8. Expedition Roof Rack & Monocrystalline Solar Telemetry Array
        // Over cabin & canopy: x: -2.15 .. +0.45 * s, y: 1.88 .. 1.92 * s
        self.add_local_box(
            [-2.15 * s, 1.88 * s, -0.76 * s],
            [0.45 * s, 1.91 * s, 0.76 * s],
            [0.20, 0.22, 0.24, 1.0],
            [0.32, 0.35, 0.38, 1.0],
        );
        // Solar telemetry panel
        self.add_local_box(
            [-1.85 * s, 1.91 * s, -0.65 * s],
            [-0.45 * s, 1.92 * s, 0.65 * s],
            [0.10, 0.14, 0.28, 1.0],
            [0.16, 0.22, 0.44, 1.0],
        );
        // Amber telemetry strobe beacon reaching 1.96 m
        self.add_local_box(
            [-0.15 * s, 1.91 * s, -0.10 * s],
            [0.05 * s, 1.96 * s, 0.10 * s],
            [0.95, 0.60, 0.05, 1.0],
            [1.00, 0.85, 0.20, 1.0],
        );

        // 9. Revolved Off-Road Mud-Terrain Tyres & 6-Spoke Metallic Hubs via `parametric_cad::revolve_profile`
        // 4 wheels at x = [-1.425, 1.425] * s, z = [-0.88, 0.88] * s, y = 0.41 * s
        // Tyre radius = 0.41 * s (820 mm diameter), width = 0.26 * s (half_w = 0.13 * s)
        {
            let r_outer = 0.41 * s as f64;
            let r_inner = 0.22 * s as f64;
            let r_shoulder = 0.38 * s as f64;
            let hw = 0.13 * s as f64;

            let tyre_profile = [
                Point3::new(r_inner, -hw * 0.65, 0.0),
                Point3::new(r_inner + 0.06 * s as f64, -hw, 0.0),
                Point3::new(r_shoulder, -hw * 0.95, 0.0),
                Point3::new(r_outer, 0.0, 0.0),
                Point3::new(r_shoulder, hw * 0.95, 0.0),
                Point3::new(r_inner + 0.06 * s as f64, hw, 0.0),
                Point3::new(r_inner, hw * 0.65, 0.0),
            ];

            let segments = 24usize;
            let req_v = tyre_profile.len() * segments;
            let req_t = (tyre_profile.len() - 1) * segments * 2;
            let mut out_v = vec![Point3::default(); req_v];
            let mut out_t = vec![[0u32; 3]; req_t];

            if let Ok((vc, tc)) = revolve_profile(&tyre_profile, segments, &mut out_v, &mut out_t) {
                for &wx in &[-1.425_f32 * s, 1.425_f32 * s] {
                    for &wz in &[-0.88_f32 * s, 0.88_f32 * s] {
                        let base_idx = self.positions.len() as u32;
                        let wy = 0.41_f32 * s;
                        let sign_z = if wz >= 0.0 { 1.0_f32 } else { -1.0_f32 };

                        for p in out_v.iter().take(vc) {
                            let rx = p.x as f32;
                            let ry = p.z as f32;
                            let rz = (p.y as f32) * sign_z;
                            let vx = wx + rx;
                            let vy = wy + ry;
                            let vz = wz + rz;
                            let r_len = (rx * rx + ry * ry).sqrt().max(1e-4);
                            let norm = [rx / r_len, ry / r_len, rz / hw as f32 * 0.2];
                            let nlen = (norm[0] * norm[0] + norm[1] * norm[1] + norm[2] * norm[2]).sqrt().max(1e-4);
                            let col = [0.12, 0.13, 0.14, 1.0];
                            self.push_vertex([vx, vy, vz], [norm[0] / nlen, norm[1] / nlen, norm[2] / nlen], col, [0.5, 0.5]);
                        }
                        for tri in out_t.iter().take(tc) {
                            if sign_z > 0.0 {
                                self.push_tri(base_idx + tri[0], base_idx + tri[1], base_idx + tri[2]);
                            } else {
                                self.push_tri(base_idx + tri[0], base_idx + tri[2], base_idx + tri[1]);
                            }
                        }

                        // Recessed 6-spoke silver metallic alloy hub cap
                        let hub_face_z = wz + sign_z * (hw as f32 * 0.75);
                        self.add_local_box(
                            [wx - 0.14 * s, wy - 0.14 * s, hub_face_z - 0.015 * s],
                            [wx + 0.14 * s, wy + 0.14 * s, hub_face_z + 0.015 * s],
                            [0.72, 0.75, 0.80, 1.0],
                            [0.90, 0.92, 0.96, 1.0],
                        );
                        for sp in 0..6 {
                            let ang = sp as f32 * (std::f32::consts::TAU / 6.0);
                            let ca = ang.cos();
                            let sa = ang.sin();
                            let r_mid = 0.16 * s;
                            let sx = wx + ca * r_mid;
                            let sy = wy + sa * r_mid;
                            self.add_local_box(
                                [sx - 0.03 * s, sy - 0.03 * s, hub_face_z - 0.012 * s],
                                [sx + 0.03 * s, sy + 0.03 * s, hub_face_z + 0.012 * s],
                                [0.78, 0.80, 0.85, 1.0],
                                [0.92, 0.94, 0.98, 1.0],
                            );
                        }
                    }
                }
            }
        }
    }

    /// Build a 3D Field Ecologist Bushwalker Avatar in 1:1 SI metric dimensions (1.80 m standing height at `scale = 1.0`).
    fn add_ecologist_walker_local(&mut self, scale: f32) {
        let s = scale;
        // Boots & Khaki Field Trousers (left & right legs)
        for side in [-1.0_f32, 1.0_f32] {
            self.add_local_box(
                [(side * 0.16 - 0.08) * s, 0.0, -0.11 * s],
                [(side * 0.16 + 0.08) * s, 0.18 * s, 0.15 * s],
                [0.24, 0.18, 0.13, 1.0],
                [0.36, 0.26, 0.18, 1.0],
            );
            self.add_local_box(
                [(side * 0.16 - 0.08) * s, 0.18 * s, -0.09 * s],
                [(side * 0.16 + 0.08) * s, 0.76 * s, 0.09 * s],
                [0.48, 0.42, 0.30, 1.0],
                [0.62, 0.55, 0.40, 1.0],
            );
        }
        // Eucalypt-green & Hi-Vis Ochre Field Expedition Jacket Torso
        self.add_local_box(
            [-0.26 * s, 0.74 * s, -0.14 * s],
            [0.26 * s, 1.36 * s, 0.14 * s],
            [0.20, 0.42, 0.26, 1.0],
            [0.94, 0.58, 0.14, 1.0],
        );
        // Botanical Specimen & Sensor Backpack
        self.add_local_box(
            [-0.22 * s, 0.80 * s, -0.30 * s],
            [0.22 * s, 1.30 * s, -0.12 * s],
            [0.32, 0.28, 0.20, 1.0],
            [0.48, 0.42, 0.30, 1.0],
        );
        // Left & Right Arms
        for side in [-1.0_f32, 1.0_f32] {
            self.add_local_box(
                [(side * 0.34 - 0.07) * s, 0.75 * s, -0.08 * s],
                [(side * 0.34 + 0.07) * s, 1.30 * s, 0.12 * s],
                [0.24, 0.46, 0.28, 1.0],
                [0.92, 0.56, 0.14, 1.0],
            );
        }
        // Head & Neck
        self.add_local_box(
            [-0.13 * s, 1.36 * s, -0.12 * s],
            [0.13 * s, 1.66 * s, 0.12 * s],
            [0.76, 0.58, 0.46, 1.0],
            [0.88, 0.70, 0.56, 1.0],
        );
        // Broad-Brimmed Australian Akubra Bush Hat
        self.add_local_box(
            [-0.30 * s, 1.63 * s, -0.28 * s],
            [0.30 * s, 1.69 * s, 0.28 * s],
            [0.36, 0.26, 0.18, 1.0],
            [0.48, 0.36, 0.24, 1.0],
        );
        self.add_local_box(
            [-0.16 * s, 1.68 * s, -0.15 * s],
            [0.16 * s, 1.80 * s, 0.15 * s],
            [0.38, 0.28, 0.19, 1.0],
            [0.52, 0.40, 0.28, 1.0],
        );
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

    let sector_seed = match sector_id {
        "talbingo" => 1.1_f32,
        "yarrangobilly" => 2.4_f32,
        "three_mile_dam" => 3.7_f32,
        "sydney_sandstone" => 5.1_f32,
        "noosa_headland" => 6.5_f32,
        _ => 7.9_f32,
    };

    let (water_y, water_color, sun_dir, cam_yaw, cam_pitch, cam_zoom, cam_target, sky_preset) =
        match sector_id {
            "talbingo" => (
                -0.22_f32,
                [0.06, 0.36, 0.50],
                [0.48, 0.78, 0.44],
                0.35,
                0.22,
                1.18,
                [0.54, 0.06, 0.05],
                1u32,
            ),
            "yarrangobilly" => (
                -0.24_f32,
                [0.06, 0.48, 0.54],
                [-0.22, 0.80, 0.56],
                0.25,
                0.22,
                1.20,
                [0.52, 0.04, 0.05],
                1u32,
            ),
            "three_mile_dam" => (
                -0.20_f32,
                [0.05, 0.44, 0.60],
                [0.48, 0.76, 0.48],
                0.32,
                0.22,
                1.18,
                [0.54, 0.06, 0.05],
                1u32,
            ),
            "sydney_sandstone" => (
                -0.25_f32,
                [0.08, 0.38, 0.46],
                [0.40, 0.82, 0.44],
                -0.30,
                0.24,
                1.20,
                [0.50, 0.06, 0.05],
                1u32,
            ),
            "noosa_headland" => (
                -0.18_f32,
                [0.04, 0.52, 0.64],
                [0.32, 0.84, 0.44],
                0.28,
                0.22,
                1.20,
                [0.52, 0.05, 0.05],
                1u32,
            ),
            _ => (
                -0.23_f32,
                [0.06, 0.38, 0.50],
                [-0.44, 0.76, 0.50],
                0.30,
                0.24,
                1.20,
                [0.54, 0.06, 0.05],
                2u32,
            ),
        };

    // 6 Submesh Builders mapped to 6 distinct MAT4 MaterialRecords:
    // 0: Terrain & 6-Layer SLGA Soil Strata + Banked Road Corridor (`tex_soil`)
    // 1: 3D Lithology (Granite Tors, Sandstone Benches, Karst, Road Guideposts) & 3D Submerged Riverbed Pebbles/Cobbles (`tex_pebble`)
    // 2: Tree Trunks, Buttresses, Roots & Fallen Hollow Logs (`tex_bark`)
    // 3: 3D Foliage Crowns, Falcate Leaf Blades, Tree Ferns, Grass Trees, Cosserat Tussock Grasses & Submerged Ribbon Weed (`tex_leaf`, `wind_animated: true`, `double_sided: true`)
    // 4: Articulated 3D Australian Fauna (`tex_bark`)
    // 5: Dynamic 3D 4WD Field Rover & 3D Field Ecologist Bushwalker Actors (`tex_pebble`)
    let mut b_terrain = SubmeshBuilder::new();
    let mut b_lithology = SubmeshBuilder::new();
    let mut b_trunks = SubmeshBuilder::new();
    let mut b_foliage = SubmeshBuilder::new();
    let mut b_fauna = SubmeshBuilder::new();
    let mut b_actors = SubmeshBuilder::new();

    // 1. Build High-Resolution Terrain Continuum Grid (`[-1.95, z_max]`)
    let grid_n = 108usize;
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

            let eps = 0.020_f32;
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

            // Smooth organic micro-variation (no stripes)
            let organic_var = 0.90 + 0.10 * ((x * 7.3 + z * 5.1).sin() * (x * 4.1 - z * 8.7).cos());
            let dist_water = y - water_y;
            let meander_x = 0.28 * (z * 1.35 + sector_seed).sin()
                + 0.11 * (z * 2.7 - sector_seed * 0.6).cos();
            let track_dist = (x - (meander_x + 0.54)).abs();

            let col = if track_dist < 0.0055 && (z * 22.0).sin() > -0.15 {
                // Dashed yellow highway centerline along the Hermite corridor
                [0.95, 0.80, 0.18, 1.0]
            } else if (track_dist - 0.064).abs() < 0.0055 {
                // Crisp white highway edge line
                [0.92, 0.92, 0.90, 1.0]
            } else if track_dist < 0.068 {
                // Compacted dark-charcoal bitumen & crushed basalt corridor pavement
                [0.28 * organic_var, 0.29 * organic_var, 0.31 * organic_var, 1.0]
            } else if track_dist < 0.092 {
                // Graded red-ochre gravel road shoulder
                [0.64 * organic_var, 0.42 * organic_var, 0.26 * organic_var, 1.0]
            } else if dist_water < -0.02 {
                // Submerged riverbed cobble armor & quartz sand
                let pebbles = 0.86 + 0.14 * hash21(x * 19.0, z * 19.0);
                [0.46 * pebbles, 0.43 * pebbles, 0.35 * pebbles, 1.0]
            } else if dist_water < 0.055 {
                // Shingle pebble point-bar & golden alluvial sand beach
                let sand = 0.90 + 0.10 * hash21(x * 23.0, z * 23.0);
                [0.72 * sand, 0.62 * sand, 0.46 * sand, 1.0]
            } else if slope > 0.30 {
                // Exposed Hawkesbury sandstone / Silurian karst limestone / Bogong granite cliff
                match sector_id {
                    "sydney_sandstone" => [0.72 * organic_var, 0.54 * organic_var, 0.36 * organic_var, 1.0],
                    "yarrangobilly" | "jounama_karst" => [0.60 * organic_var, 0.62 * organic_var, 0.65 * organic_var, 1.0],
                    _ => [0.54 * organic_var, 0.51 * organic_var, 0.47 * organic_var, 1.0],
                }
            } else {
                // Rich Australian sclerophyll forest biomantle loam, dry eucalypt leaf litter & mossy riparian banks
                let moisture = (-((dist_water / 0.28).powi(2))).exp();
                let r = (0.31 * (1.0 - moisture) + 0.22 * moisture) * organic_var;
                let g = (0.40 * (1.0 - moisture) + 0.46 * moisture) * organic_var;
                let b = (0.21 * (1.0 - moisture) + 0.19 * moisture) * organic_var;
                [r, g, b, 1.0]
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
        let meander_b = 0.28 * (z * 1.35 + 1.1).sin() + 0.11 * (z * 2.7 - 0.66).cos();
        let road_x_b = meander_b + 0.54;
        // Keep boulders off the road corridor and out of the foreground kangaroo clearing
        if (x - road_x_b).abs() < 0.28 || (x > 0.90 && x < 1.38 && z > -0.35 && z < 0.45) {
            continue;
        }
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
    let sector_seed = match sector_id {
        "talbingo" => 1.1_f32,
        "yarrangobilly" => 2.4_f32,
        "three_mile_dam" => 3.7_f32,
        "sydney_sandstone" => 5.1_f32,
        "noosa_headland" => 6.5_f32,
        _ => 7.9_f32,
    };
    let mut veg_states = Vec::with_capacity(64);
    for i in 0..64 {
        let fi = i as f32;
        let side = if i % 2 == 0 { 1.0_f32 } else { -1.0_f32 };
        let z = -1.70 + (fi / 64.0) * 3.40;
        let meander_x = 0.28 * (z * 1.35 + sector_seed).sin()
            + 0.11 * (z * 2.7 - sector_seed * 0.6).cos();
        let x = if side > 0.0 {
            // Upslope of the banked road corridor (road is at meander_x + 0.54)
            // Leave a natural grassy clearing around z in [-0.28, 0.35], x in [1.02, 1.32] for the kangaroo mob
            let raw_x = (meander_x + 0.82 + 0.88 * hash21(fi * 6.1, 4.2)).clamp(-1.82, 1.82);
            if z > -0.32 && z < 0.38 && raw_x < 1.32 {
                (raw_x + 0.34).min(1.82)
            } else {
                raw_x
            }
        } else {
            // Opposite river bank (-X side)
            (meander_x - 0.52 - 1.15 * hash21(fi * 6.1, 4.2)).clamp(-1.82, 1.82)
        };
        let y = sample_sector_elevation(sector_id, x, z, water_y).max(water_y + 0.06);
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
        let meander_g = 0.28 * (z * 1.35 + sector_seed).sin()
            + 0.11 * (z * 2.7 - sector_seed * 0.6).cos();
        if (x - (meander_g + 0.54)).abs() < 0.16 {
            continue;
        }
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
    // Eastern Grey Kangaroos (*Macropus giganteus*) in the off-road grassy woodland clearing beside the road
    for (kx_off, kz, kyaw, is_road_side) in [
        (0.24_f32, 0.16_f32, -2.35_f32, true),
        (0.32_f32, -0.12_f32, -1.95_f32, true),
        (-0.68_f32, 0.35_f32, 0.85_f32, false),
    ] {
        let meander_k = 0.28 * (kz * 1.35 + sector_seed).sin()
            + 0.11 * (kz * 2.7 - sector_seed * 0.6).cos();
        let kx = if is_road_side {
            meander_k + 0.54 + kx_off
        } else {
            kx_off
        };
        let ky = sample_sector_elevation(sector_id, kx, kz, water_y).max(water_y + 0.08);
        b_fauna.add_fauna_specimen(
            [kx, ky, kz],
            kyaw,
            SECTOR_METRIC_SCALE,
            "macropod",
            [0.42, 0.35, 0.28, 1.0],
            [0.68, 0.58, 0.46, 1.0],
        );
    }
    // Common Wombats (*Vombatus ursinus*) foraging near tussock grass
    for (wx, wz, wyaw) in [(-0.55_f32, -0.32_f32, 0.45_f32), (0.38_f32, 0.65_f32, -0.85_f32)] {
        let wy = sample_sector_elevation(sector_id, wx, wz, water_y);
        b_fauna.add_fauna_specimen(
            [wx, wy, wz],
            wyaw,
            SECTOR_METRIC_SCALE,
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
        let arch = if is_cod { "cod" } else { "platypus" };
        b_fauna.add_fauna_specimen([ux, swim_y, uz], uyaw, SECTOR_METRIC_SCALE, arch, cd, cl);
    }
    // Soaring Wedge-Tailed Eagle (*Aquila audax*) & Sulphur-Crested Cockatoo overhead
    b_fauna.add_fauna_specimen(
        [-0.25, 0.58, -0.15],
        0.55,
        SECTOR_METRIC_SCALE,
        "raptor",
        [0.22, 0.16, 0.12, 1.0],
        [0.44, 0.32, 0.22, 1.0],
    );
    b_fauna.add_fauna_specimen(
        [0.32, 0.48, 0.25],
        -0.75,
        SECTOR_METRIC_SCALE,
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

    // 5b. Australian Roadside Reflector Guideposts along the Banked Hermite Corridor
    for gp in 0..16 {
        let z_gp = -1.70 + (gp as f32 / 15.0) * 3.40;
        if z_gp > z_max - 0.08 {
            continue;
        }
        let meander_gp = 0.28 * (z_gp * 1.35 + sector_seed).sin()
            + 0.11 * (z_gp * 2.7 - sector_seed * 0.6).cos();
        for side in [-1.0_f32, 1.0_f32] {
            let x_gp = meander_gp + 0.54 + side * 0.082;
            let y_gp = sample_sector_elevation(sector_id, x_gp, z_gp, water_y);
            // White timber/PVC post
            b_lithology.add_local_box(
                [x_gp - 0.005, y_gp, z_gp - 0.005],
                [x_gp + 0.005, y_gp + 0.042, z_gp + 0.005],
                [0.88, 0.88, 0.86, 1.0],
                [0.98, 0.98, 0.96, 1.0],
            );
            // Red/White reflector cap
            let ref_col = if side < 0.0 {
                [0.95, 0.16, 0.12, 1.0]
            } else {
                [0.98, 0.86, 0.20, 1.0]
            };
            b_lithology.add_local_box(
                [x_gp - 0.006, y_gp + 0.034, z_gp - 0.006],
                [x_gp + 0.006, y_gp + 0.044, z_gp + 0.006],
                ref_col,
                ref_col,
            );
        }
    }

    // 5c. Build Dynamic 3D 4WD Field Rover & 3D Field Ecologist Bushwalker Actors (`b_actors`)
    let mut rover_template = SubmeshBuilder::new();
    rover_template.add_field_rover_4wd_local(SECTOR_METRIC_SCALE);
    // Orient for driving physics in WASM ([lp[2], lp[1], lp[0]]) so +Z is forward:
    let rover_local_positions: Vec<[f32; 3]> = rover_template
        .positions
        .iter()
        .map(|p| [p[2], p[1], p[0]])
        .collect();

    let mut walker_template = SubmeshBuilder::new();
    walker_template.add_ecologist_walker_local(SECTOR_METRIC_SCALE);
    let walker_local_positions = walker_template.positions.clone();

    // Place the initial 4WD Rover and Ecologist Walker on the banked road corridor at z = 0.0 / z = 0.18
    let z_r0 = 0.0_f32;
    let x_r0 = 0.28 * (z_r0 * 1.35 + sector_seed).sin()
        + 0.11 * (z_r0 * 2.7 - sector_seed * 0.6).cos()
        + 0.54;
    let y_r0 = sample_sector_elevation(sector_id, x_r0, z_r0, water_y);
    let rover_local_start = b_actors.positions.len();
    for i in 0..rover_template.positions.len() {
        let lp = rover_template.positions[i];
        // lp[0] is vehicle length, lp[2] is lateral. Road runs along Z.
        // Place rover facing along road (+Z):
        b_actors.push_vertex(
            [lp[2] + x_r0, lp[1] + y_r0, lp[0] + z_r0],
            [rover_template.normals[i][2], rover_template.normals[i][1], rover_template.normals[i][0]],
            rover_template.colors[i],
            rover_template.uvs[i],
        );
    }
    for &idx in &rover_template.indices {
        b_actors.indices.push(rover_local_start as u32 + idx);
    }
    let rover_local_end = b_actors.positions.len();

    let z_w0 = 0.18_f32;
    let x_w0 = x_r0 - 0.09;
    let y_w0 = sample_sector_elevation(sector_id, x_w0, z_w0, water_y);
    let walker_local_start = b_actors.positions.len();
    for i in 0..walker_template.positions.len() {
        let lp = walker_template.positions[i];
        b_actors.push_vertex(
            [lp[0] + x_w0, lp[1] + y_w0, lp[2] + z_w0],
            walker_template.normals[i],
            walker_template.colors[i],
            walker_template.uvs[i],
        );
    }
    for &idx in &walker_template.indices {
        b_actors.indices.push(walker_local_start as u32 + idx);
    }
    let walker_local_end = b_actors.positions.len();

    // 6. Merge the 6 SubmeshBuilders into one unified scene mesh with 6 `MAT4` `MaterialRecord`s and `SubmeshRange`s
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut materials = Vec::new();
    let mut ranges = Vec::new();
    let mut actors_base_vertex = 0usize;

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
        (
            "05-rover-and-walker-actors",
            b_actors,
            tex_pebble,
            0.38_f32,
            0.12_f32,
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
        if mat_slug == "05-rover-and-walker-actors" {
            actors_base_vertex = base_vertex as usize;
        }
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

    let mut scene_bounds_min = [f32::INFINITY; 3];
    let mut scene_bounds_max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for k in 0..3 {
            scene_bounds_min[k] = scene_bounds_min[k].min(p[k]);
            scene_bounds_max[k] = scene_bounds_max[k].max(p[k]);
        }
    }
    let rover_vertex_range =
        (actors_base_vertex + rover_local_start)..(actors_base_vertex + rover_local_end);
    let walker_vertex_range =
        (actors_base_vertex + walker_local_start)..(actors_base_vertex + walker_local_end);

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
            let meander_x = 0.28 * (z * 1.35 + sector_seed).sin()
                + 0.11 * (z * 2.7 - sector_seed * 0.6).cos();
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
        water_y,
        scene_bounds_min,
        scene_bounds_max,
        rover_vertex_range,
        rover_local_positions,
        walker_vertex_range,
        walker_local_positions,
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
        // Focus camera directly on the foreground Eastern Grey Kangaroo mob in the grassy woodland clearing beside the 4WD
        sector.cam_target = [1.02, 0.12, 0.06];
        sector.cam_yaw = -0.52;
        sector.cam_pitch = 0.20;
        sector.cam_zoom = 0.52;
    } else {
        // Focus camera directly on the foreground Ghost Gum / Snow Gum crown, falcate leaf blades & clear river pebbles
        sector.cam_target = [0.45, 0.42, 0.15];
        sector.cam_yaw = 0.25;
        sector.cam_pitch = 0.18;
        sector.cam_zoom = 0.65;
    }
    Ok(sector)
}

fn attach_raw_section(
    container: &[u8],
    section_type: SectionType,
    alignment_tier: AlignmentTier,
    stride: u32,
    element_count: u32,
    payload: &[u8],
) -> Result<Vec<u8>, String> {
    let header = Container10dHeader::parse(container).map_err(|e| format!("10d header parse: {e:?}"))?;
    let descriptors = parse_section_table(container, &header).map_err(|e| format!("parse_section_table: {e:?}"))?;
    if descriptors.len() >= MAX_SECTION_COUNT as usize {
        return Err("MAX_SECTION_COUNT reached".into());
    }

    let mut inputs = Vec::with_capacity(descriptors.len() + 1);
    for descriptor in descriptors {
        let start = descriptor.byte_offset as usize;
        let end = start
            .checked_add(descriptor.byte_length as usize)
            .filter(|&end| end <= container.len())
            .ok_or_else(|| {
                format!(
                    "SectionTableError::OutOfBounds offset={} length={}",
                    descriptor.byte_offset, descriptor.byte_length
                )
            })?;
        inputs.push(SectionInput {
            section_type: descriptor
                .typ()
                .ok_or_else(|| format!("UndefinedSectionType: {}", descriptor.section_type))?,
            alignment_tier: descriptor
                .tier()
                .ok_or_else(|| format!("UndefinedAlignmentTier: {}", descriptor.alignment_tier))?,
            stride: descriptor.stride,
            element_count: descriptor.element_count,
            payload: &container[start..end],
        });
    }

    inputs.push(SectionInput {
        section_type,
        alignment_tier,
        stride,
        element_count,
        payload,
    });

    let needed = match encode_container(&header, &inputs, &mut []) {
        Err(SectionTableError::OutputBufferTooSmall { needed, .. }) => needed,
        Ok(size) => size,
        Err(error) => return Err(format!("encode_container dry-run error: {error:?}")),
    };
    let mut output = vec![0u8; needed];
    let written = encode_container(&header, &inputs, &mut output)
        .map_err(|e| format!("encode_container error: {e:?}"))?;
    output.truncate(written);
    seal_whole_file_crc32c(&mut output);
    Ok(output)
}

fn attach_procedural_synthesis_section(
    container: &[u8],
    header: &ProceduralSynthesisHeader,
) -> Result<Vec<u8>, String> {
    let mut buf = vec![0u8; PROCEDURAL_SYNTHESIS_HEADER_SIZE];
    encode_procedural_synthesis_section(header, &mut buf)
        .map_err(|e| format!("encode_procedural_synthesis_section: {e:?}"))?;
    attach_raw_section(
        container,
        SectionType::ProceduralSynthesis,
        AlignmentTier::CacheLine,
        0,
        0,
        &buf,
    )
}

fn attach_entity_ontology_section(
    container: &[u8],
    header: &EntityOntologyMiniHeader,
    affordances: &[AffordanceBinaryRecord],
    keyframes: &[DevelopmentalKeyframeRecord],
) -> Result<Vec<u8>, String> {
    let mut hdr = *header;
    hdr.affordance_count = affordances.len() as u32;
    hdr.reserved_u32 = keyframes.len() as u32;
    let mut payload = Vec::with_capacity(
        ENTITY_ONTOLOGY_MINI_HEADER_SIZE
            + affordances.len() * AFFORDANCE_RECORD_SIZE
            + keyframes.len() * DEVELOPMENTAL_KEYFRAME_RECORD_SIZE,
    );
    payload.extend_from_slice(bytemuck::bytes_of(&hdr));
    for a in affordances {
        payload.extend_from_slice(bytemuck::bytes_of(a));
    }
    for k in keyframes {
        payload.extend_from_slice(bytemuck::bytes_of(k));
    }
    attach_raw_section(
        container,
        SectionType::EntityOntology,
        AlignmentTier::CacheLine,
        0,
        0,
        &payload,
    )
}

fn attach_morph_section(
    container: &[u8],
    header: &MorphSectionHeader,
    deltas: &[QuantizedMorphDelta],
) -> Result<Vec<u8>, String> {
    let needed = morph_section_encoded_len(deltas.len())
        .ok_or_else(|| format!("morph_section_encoded_len overflow: deltas={}", deltas.len()))?;
    let mut buf = vec![0u8; needed];
    encode_morph_section(header, deltas, &mut buf)
        .map_err(|e| format!("encode_morph_section: {e:?}"))?;
    attach_raw_section(
        container,
        SectionType::Reconstruction,
        AlignmentTier::CacheLine,
        0,
        0,
        &buf,
    )
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
    let mut bytes = compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        Some(colors),
        Some(normals),
        None,
    )
    .map_err(|e| format!("compile_mesh_to_10d: {e}"))?;

    bytes = attach_material_section(&bytes, materials, ranges, indices.len() as u32)
        .map_err(|e| format!("attach_material_section: {e:?}"))?;
    bytes = attach_texture_coordinate_section(&bytes, uvs)
        .map_err(|e| format!("attach_texture_coordinate_section: {e:?}"))?;

    // 1. Procedural Synthesis Section (`MAGIC_PROC`, SectionType 18)
    let proc_header = ProceduralSynthesisHeader {
        magic: MAGIC_PROC,
        version: PROCEDURAL_SYNTHESIS_SECTION_VERSION,
        flags: (FLAG_CELLULAR_VORONOI
            | FLAG_IMPLICIT_FILLET
            | FLAG_ALLOMETRIC_GROWTH
            | FLAG_PGA_MOTOR_DYNAMICS) as u16,
        seed: 0x9E37_79B9_7F4A_7C15,
        voronoi_frequency: 18.5,
        voronoi_jitter: 0.85,
        fillet_blend_radius: 0.12,
        fillet_threshold: 0.05,
        allometry_exponent_k: 0.67,
        allometry_base_b: 1.15,
        spiral_growth_rate_k: 0.22,
        spiral_initial_radius_a: 0.04,
        motor_frequency_hz: 1.65,
        motor_damping_ratio: 0.08,
        critical_stress_threshold: 24.5,
        reserved_f32: 0.0,
    };
    bytes = attach_procedural_synthesis_section(&bytes, &proc_header)?;

    // 2. Entity Ontology Section (`MAGIC_EONT`, SectionType 17)
    let onto_header = EntityOntologyMiniHeader {
        magic: MAGIC_EONT,
        version: ENTITY_ONTOLOGY_SECTION_VERSION,
        stage: 2, // Adult
        reserved_u8: 0,
        entity_did_hash: 0xD1D0_A057_FA11_9921,
        taxon_iri_hash: 0x7A00_ECA1_CA7A_1001,
        energy_permill: 920,
        damage_permill: 40,
        hydration_permill: 860,
        wear_permill: 110,
        affordance_count: 3,
        reserved_u32: 4,
    };
    let affordances = [
        AffordanceBinaryRecord {
            action_id: 0x0001,
            required_capability_hash: 0xCA00_1001,
            emitted_topic_hash: 0x7000_1001,
            category: 1, // Forage / Pollination
            min_observer_status: 0,
            min_energy_permill: 100,
            max_damage_permill: 800,
            min_hydration_permill: 150,
            max_wear_permill: 900,
            energy_delta: 35,
            damage_delta: 0,
            hydration_delta: 20,
            wear_delta: 5,
            reserved_u16: 0,
            animation_clip_id: 1,
        },
        AffordanceBinaryRecord {
            action_id: 0x0002,
            required_capability_hash: 0xCA00_2002,
            emitted_topic_hash: 0x7000_2002,
            category: 2, // Cultural Harvest / Resin
            min_observer_status: 1,
            min_energy_permill: 200,
            max_damage_permill: 700,
            min_hydration_permill: 200,
            max_wear_permill: 850,
            energy_delta: -10,
            damage_delta: 15,
            hydration_delta: -15,
            wear_delta: 25,
            reserved_u16: 0,
            animation_clip_id: 2,
        },
        AffordanceBinaryRecord {
            action_id: 0x0003,
            required_capability_hash: 0xCA00_3003,
            emitted_topic_hash: 0x7000_3003,
            category: 3, // Tree Hollow Nesting
            min_observer_status: 0,
            min_energy_permill: 50,
            max_damage_permill: 950,
            min_hydration_permill: 50,
            max_wear_permill: 980,
            energy_delta: 50,
            damage_delta: -10,
            hydration_delta: 30,
            wear_delta: -10,
            reserved_u16: 0,
            animation_clip_id: 3,
        },
    ];
    let keyframes = [
        DevelopmentalKeyframeRecord {
            age_days: 30.0,
            stage: 0, // Seedling / Spore
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 0.12,
            allometric_trunk_scale: 0.08,
            lai_multiplier: 0.15,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 365.0,
            stage: 1, // Juvenile
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 0.45,
            allometric_trunk_scale: 0.38,
            lai_multiplier: 0.52,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 3650.0,
            stage: 2, // Mature Adult
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 1.0,
            allometric_trunk_scale: 1.0,
            lai_multiplier: 1.0,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 36500.0,
            stage: 3, // Ancient Hollow-Bearing Veteran
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 1.28,
            allometric_trunk_scale: 1.45,
            lai_multiplier: 0.88,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
    ];
    bytes = attach_entity_ontology_section(&bytes, &onto_header, &affordances, &keyframes)?;

    // 3. Morph Target Section (`MAGIC_MRP1`, SectionType::MorphTargets)
    if !positions.is_empty() {
        let n_verts = positions.len();
        let mut morph_deltas = Vec::new();
        let step = (n_verts / 16).max(1);
        for vi in (0..n_verts).step_by(step) {
            let p = positions[vi];
            let n = normals.get(vi).copied().unwrap_or([0.0, 1.0, 0.0]);
            let flex = (p[1] * 0.15).abs().min(0.25);
            let d0 = quantize_morph_delta(
                [flex * 0.08, flex * 0.02, flex * 0.06],
                [n[0] * 0.1, n[1] * 0.05, n[2] * 0.1],
                [0.0, 0.0, 0.0],
                0.5,
                0,
                vi as u32,
            );
            morph_deltas.push(d0);

            let r_exp = (p[0] * p[0] + p[2] * p[2]).sqrt() * 0.08;
            let d1 = quantize_morph_delta(
                [n[0] * r_exp, p[1] * 0.05, n[2] * r_exp],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                0.5,
                1,
                vi as u32,
            );
            morph_deltas.push(d1);
        }

        let morph_header = MorphSectionHeader {
            magic: MAGIC_MRP1,
            version: MORPH_SECTION_VERSION,
            target_count: 2,
            vertex_count: n_verts as u32,
            flags: FLAG_MORPH_SPARSE | FLAG_MORPH_HAS_NORMALS | FLAG_MORPH_ONTOGENETIC_BIND,
            max_displacement: 0.5,
            ontogenetic_bind_mask: 0b0011,
            reserved: [0u8; 8],
        };
        bytes = attach_morph_section(&bytes, &morph_header, &morph_deltas)?;
    }

    Ok(bytes)
}

/// Render a compiled `.10d` sector natively on the physical GPU via `PortalGpu` (`wgpu` — Zero V8)
/// with resident mipmapped PBR textures, AG-22 `EnvironmentProbeSet` IBL, cascaded sun shadows,
/// half-resolution SSAO, analytic sky (`SKY_WGSL`), and AG-10 `WATER_QUALITY_CINEMATIC` shoreline water!
#[cfg(not(target_arch = "wasm32"))]
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

    // Configure analytic sky, warm Australian directional sun, cascaded shadows, and SSAO
    renderer.set_sky_preset(sky_preset);
    renderer.set_lighting(
        sector.sun_dir[0],
        sector.sun_dir[1],
        sector.sun_dir[2],
        1.52,
        0.26,
    );
    renderer.set_shadows_enabled(true);
    renderer.set_screen_space_ao_enabled(true);
    renderer.set_screen_space_ao(0.52, 0.72, 0.018);
    renderer.set_screen_space_ao_sample_count(16);

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

    // Upload AG-10 water geometry once with BVH shoreline SDF distance (`shore_distance_accelerated_into`)
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

    // Precompute vertex membership per submesh range so we can animate:
    // - Range 2 (Trunks & Branches): elastic wind sway
    // - Range 3 (Foliage, 3D Falcate Leaf Blades, Tree Ferns, Grass Trees & Cosserat Grass): travelling wind waves + high-frequency leaf blade flutter
    // - Range 4 (3D Fauna & GPR/Trophic Arcs): saltatory kangaroo hopping, raptor thermal soaring & wing-flapping, underwater platypus/cod swimming
    let n_verts = sector.positions.len();
    let mut vert_range_tag = vec![0u8; n_verts];
    for (r_idx, r) in sector.ranges.iter().enumerate() {
        let start = r.first_index as usize;
        let end = start + r.index_count as usize;
        for &vi in &sector.indices[start..end] {
            vert_range_tag[vi as usize] = r_idx as u8;
        }
    }

    let num_frames = 10usize;
    let mut frames = Vec::with_capacity(num_frames);
    let mut anim_positions = sector.positions.clone();
    let req_bytes = renderer.required_rgba8_bytes();

    let _csm_splits = compute_csm_4_splits_zero_heap(0.08, 260.0, 0.75);
    let _brdf_sample = integrate_split_sum_brdf_zero_heap(sector.sun_dir[1].max(0.05), 0.48, 64);

    let inv_bind = [Motor::identity(); 4];
    let prev_skin = [Motor::identity(); 4];
    let mut gpu_palette = [GpuJointMotorPair::identity(); 4];
    let mut curr_skin = [Motor::identity(); 4];

    for frame_idx in 0..num_frames {
        let phase = (frame_idx as f32 / num_frames as f32) * std::f32::consts::TAU;
        let time_secs = 1.25_f32 + (frame_idx as f32) * 0.42_f32;

        for (vi, &p) in sector.positions.iter().enumerate() {
            match vert_range_tag[vi] {
                2 => {
                    // Range 2: Tree Trunks & Upper Branches — PGA motor elastic wind sway
                    let h_sway = (p[1] + 0.15).clamp(0.0, 0.45);
                    let drive = [
                        sector.sun_dir[0] * 0.045 * h_sway,
                        0.0,
                        sector.sun_dir[2] * 0.035 * h_sway,
                    ];
                    anim_positions[vi] = evaluate_pga_motor_oscillation(p, time_secs, 0.85, 0.06, drive);
                }
                3 => {
                    // Range 3: 3D Falcate Leaf Blades, Tree Ferns, Grass Trees & Cosserat Grasses —
                    // PGA motor wind oscillations + individual leaf-blade harmonic flutter!
                    let amp = if p[1] > -0.08 { 0.026 } else { 0.016 };
                    let drive = [amp * 1.2, 0.008, amp * 0.95];
                    let base = evaluate_pga_motor_oscillation(p, time_secs, 2.2, 0.03, drive);
                    let flutter = (2.0 * phase + p[0] * 19.0 + p[1] * 23.0 + p[2] * 17.0).sin();
                    anim_positions[vi] = [
                        base[0] + 0.010 * flutter,
                        base[1] + 0.008 * flutter,
                        base[2] - 0.009 * flutter,
                    ];
                }
                4 => {
                    // Range 4: Articulated 3D Fauna Specimens & GPR/Trophic Arcs via PGA Dual-Quaternion Skinning
                    if sector.sector_id == "three_mile_dam" && (p[0].abs() > 0.75 || p[1] > 0.62) {
                        // GPR Dielectric Wave Ribbons & Trophic Arcs pulse
                        let pulse = 0.022 * (phase + p[0] * 5.5).sin();
                        anim_positions[vi] = [p[0], p[1] + pulse, p[2]];
                    } else if p[1] > 0.36 {
                        // Aerial Raptors (Wedge-Tailed Eagle & Sulphur-Crested Cockatoo):
                        // Circular thermal patrol + articulated wing-flapping via PGA dual-quaternion skinning!
                        let cx = if p[0] < 0.0 { -0.25_f32 } else { 0.32_f32 };
                        let cz = if p[0] < 0.0 { -0.15_f32 } else { 0.25_f32 };
                        let wing_dist = ((p[0] - cx).hypot(p[2] - cz) - 0.022).max(0.0);
                        let flap = wing_dist * 0.58 * (2.0 * phase).sin();
                        let world_curr = [
                            Motor::from_translation(
                                (0.14 * phase.sin()) as f64,
                                (0.025 * (2.0 * phase).cos() + flap) as f64,
                                (0.09 * phase.cos()) as f64,
                            ),
                            Motor::identity(),
                            Motor::identity(),
                            Motor::identity(),
                        ];
                        evaluate_skinning_motor_palette_zero_heap(
                            &inv_bind,
                            &world_curr,
                            &prev_skin,
                            &mut gpu_palette,
                            &mut curr_skin,
                        );
                        let skin_attr = GpuSkinnedVertex {
                            joint_indices: [0, 0, 0, 0],
                            joint_weights: [1.0, 0.0, 0.0, 0.0],
                        };
                        let (skinned, _, _, _) = gpu_shader_skin_vertex_reference(
                            p,
                            sector.normals[vi],
                            [0.0, 0.0, 1.0, 1.0],
                            &skin_attr,
                            &gpu_palette,
                        );
                        anim_positions[vi] = skinned;
                    } else if p[1] < -0.14 {
                        // Submerged Platypus & Murray Cod: swimming with lateral tail undulation via PGA skinning!
                        let swim_wave = (2.0 * phase + p[2] * 18.0).sin();
                        let world_curr = [
                            Motor::from_translation(
                                (0.028 * swim_wave) as f64,
                                (0.010 * phase.cos()) as f64,
                                (0.085 * phase.sin()) as f64,
                            ),
                            Motor::identity(),
                            Motor::identity(),
                            Motor::identity(),
                        ];
                        evaluate_skinning_motor_palette_zero_heap(
                            &inv_bind,
                            &world_curr,
                            &prev_skin,
                            &mut gpu_palette,
                            &mut curr_skin,
                        );
                        let skin_attr = GpuSkinnedVertex {
                            joint_indices: [0, 0, 0, 0],
                            joint_weights: [1.0, 0.0, 0.0, 0.0],
                        };
                        let (skinned, _, _, _) = gpu_shader_skin_vertex_reference(
                            p,
                            sector.normals[vi],
                            [0.0, 0.0, 1.0, 1.0],
                            &skin_attr,
                            &gpu_palette,
                        );
                        anim_positions[vi] = skinned;
                    } else {
                        // Terrestrial Eastern Grey Kangaroos & Common Wombats:
                        // Bipedal saltatory hopping gait via PGA dual-quaternion skinning!
                        let hop_arc = 0.024 * (phase.sin()).abs();
                        let stride_x = -0.018 * phase.sin();
                        let stride_z = 0.018 * phase.sin();
                        let world_curr = [
                            Motor::from_translation(stride_x as f64, hop_arc as f64, stride_z as f64),
                            Motor::identity(),
                            Motor::identity(),
                            Motor::identity(),
                        ];
                        evaluate_skinning_motor_palette_zero_heap(
                            &inv_bind,
                            &world_curr,
                            &prev_skin,
                            &mut gpu_palette,
                            &mut curr_skin,
                        );
                        let skin_attr = GpuSkinnedVertex {
                            joint_indices: [0, 0, 0, 0],
                            joint_weights: [1.0, 0.0, 0.0, 0.0],
                        };
                        let (skinned, _, _, _) = gpu_shader_skin_vertex_reference(
                            p,
                            sector.normals[vi],
                            [0.0, 0.0, 1.0, 1.0],
                            &skin_attr,
                            &gpu_palette,
                        );
                        anim_positions[vi] = skinned;
                    }
                }
                _ => {
                    anim_positions[vi] = p;
                }
            }
        }

        renderer
            .upload_mesh_colored_with_frames_and_materials_and_uv0(
                &anim_positions,
                &colors,
                Some(&sector.normals),
                None,
                Some(&sector.uvs),
                &sector.indices,
                &materials,
                &sector.ranges,
            )
            .map_err(|e| format!("upload_mesh_colored_with_frames_and_materials_and_uv0: {e}"))?;

        // Advance `time_secs` so AG-10 `WATER_WGSL` waves, shoreline foam, and `MESH_WGSL` wind animate in lockstep
        renderer.render(time_secs, &SystemTelemetry::default())?;

        let mut rgba = vec![0u8; req_bytes];
        renderer.read_rgba8_into(&mut rgba)?;
        frames.push(rgba);
    }

    Ok(encode_rgba8_frames_to_apng(width, height, &frames, 9, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compute_aabb(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
        assert!(!positions.is_empty(), "Mesh must contain vertices");
        let mut min = positions[0];
        let mut max = positions[0];
        for p in positions {
            for i in 0..3 {
                if p[i] < min[i] {
                    min[i] = p[i];
                }
                if p[i] > max[i] {
                    max[i] = p[i];
                }
            }
        }
        (min, max)
    }

    #[test]
    fn test_field_rover_and_fauna_metric_proportions() {
        // 1. Build 1:1 SI Metric Rover (scale = 1.0)
        let mut rover_builder = SubmeshBuilder::new();
        rover_builder.add_field_rover_4wd_local(1.0);
        let (r_min, r_max) = compute_aabb(&rover_builder.positions);
        let r_length = r_max[0] - r_min[0];
        let r_height = r_max[1] - r_min[1];
        let r_width = r_max[2] - r_min[2];

        // Rover canonical bounds check
        // Canonical length: 4.85 m (allow +/- 0.35 m for bullbar and tow hitch)
        assert!(
            r_length >= 4.5 && r_length <= 5.2,
            "Rover length {r_length} m should be ~4.85 m"
        );
        // Canonical width: 1.88 m body / 2.02 m tyre track (allow 1.7 to 2.2 m)
        assert!(
            r_width >= 1.7 && r_width <= 2.2,
            "Rover width {r_width} m should be ~1.88 m body / 2.02 m track"
        );
        // Canonical height: 1.92 m roof / 1.96 m beacon
        assert!(
            r_height >= 1.7 && r_height <= 2.2,
            "Rover height {r_height} m should be ~1.92-1.96 m"
        );

        // 2. Build 1:1 SI Metric Eastern Grey Kangaroo (scale = 1.0)
        let mut kanga_builder = SubmeshBuilder::new();
        let coat_dark = [0.22, 0.20, 0.17, 1.0];
        let coat_lit = [0.46, 0.42, 0.36, 1.0];
        kanga_builder.add_fauna_specimen([0.0, 0.0, 0.0], 0.0, 1.0, "macropod", coat_dark, coat_lit);
        let (k_min, k_max) = compute_aabb(&kanga_builder.positions);
        let k_height = k_max[1] - k_min[1];

        // Kangaroo standing height ~1.65 m (pinnae ears)
        assert!(
            k_height >= 1.45 && k_height <= 1.85,
            "Kangaroo standing height {k_height} m should be ~1.65 m"
        );

        // Crucial proportional assertion: Rover is > 2.5x longer than Kangaroo's standing height!
        // (Guaranteed: 4.85 m / 1.65 m = 2.94x; Kangaroo is never twice the size of a 4WD!)
        assert!(
            r_length > 2.5 * k_height,
            "Rover length ({r_length} m) must be > 2.5x Kangaroo height ({k_height} m)"
        );

        // 3. Build 1:1 SI Metric Common Wombat (scale = 1.0)
        let mut wombat_builder = SubmeshBuilder::new();
        wombat_builder.add_fauna_specimen([0.0, 0.0, 0.0], 0.0, 1.0, "wombat", coat_dark, coat_lit);
        let (w_min, w_max) = compute_aabb(&wombat_builder.positions);
        let w_height = w_max[1] - w_min[1];
        let w_length = (w_max[2] - w_min[2]).max(w_max[0] - w_min[0]);

        assert!(
            w_height >= 0.28 && w_height <= 0.46,
            "Wombat height {w_height} m should be ~0.36 m"
        );
        assert!(
            w_length >= 0.80 && w_length <= 1.20,
            "Wombat length {w_length} m should be ~0.98 m"
        );

        // Kangaroo vs Wombat height ratio: 1.65 / 0.36 = 4.58x
        assert!(
            k_height > 3.0 * w_height,
            "Kangaroo height ({k_height} m) must be > 3.0x Wombat height ({w_height} m)"
        );

        // 4. Build 1:1 SI Metric Platypus (scale = 1.0)
        let mut platypus_builder = SubmeshBuilder::new();
        platypus_builder.add_fauna_specimen([0.0, 0.0, 0.0], 0.0, 1.0, "platypus", coat_dark, coat_lit);
        let (p_min, p_max) = compute_aabb(&platypus_builder.positions);
        let p_length = (p_max[2] - p_min[2]).max(p_max[0] - p_min[0]);
        let p_height = p_max[1] - p_min[1];

        assert!(
            p_length >= 0.40 && p_length <= 0.65,
            "Platypus length {p_length} m should be ~0.50 m"
        );
        assert!(
            p_height >= 0.10 && p_height <= 0.25,
            "Platypus height {p_height} m should be ~0.15 m"
        );

        // 5. Build 1:1 SI Metric Ecologist Bushwalker (scale = 1.0)
        let mut walker_builder = SubmeshBuilder::new();
        walker_builder.add_ecologist_walker_local(1.0);
        let (wk_min, wk_max) = compute_aabb(&walker_builder.positions);
        let wk_height = wk_max[1] - wk_min[1];
        assert!(
            wk_height >= 1.70 && wk_height <= 1.90,
            "Ecologist bushwalker height {wk_height} m should be ~1.80 m"
        );
        assert!(
            wk_height > k_height,
            "Adult bushwalker ({wk_height} m) stands taller than resting kangaroo ({k_height} m)"
        );

        // 6. Build 1:1 SI Metric Wedge-Tailed Eagle (scale = 1.0)
        let mut eagle_builder = SubmeshBuilder::new();
        eagle_builder.add_fauna_specimen([0.0, 0.0, 0.0], 0.0, 1.0, "raptor", coat_dark, coat_lit);
        let (e_min, e_max) = compute_aabb(&eagle_builder.positions);
        let e_wingspan = (e_max[0] - e_min[0]).max(e_max[2] - e_min[2]);
        assert!(
            e_wingspan >= 1.90 && e_wingspan <= 2.30,
            "Eagle wingspan {e_wingspan} m should be ~2.10 m"
        );
    }

    #[test]
    fn test_canopy_tree_pipe_model_and_sector_scaling() {
        let mut foliage_builder = SubmeshBuilder::new();
        let mut trunk_builder = SubmeshBuilder::new();
        foliage_builder.add_canopy_tree(
            &mut trunk_builder,
            [0.0, 0.0, 0.0],
            12.0,
            [0.52, 0.48, 0.44, 1.0],
            [0.88, 0.86, 0.82, 1.0],
            [0.18, 0.34, 0.20, 1.0],
            [0.32, 0.52, 0.30, 1.0],
            false,
            42.0,
        );
        assert!(!trunk_builder.positions.is_empty(), "Canopy tree trunk must generate mesh vertices");
        assert!(!foliage_builder.positions.is_empty(), "Canopy tree foliage must generate mesh vertices");

        let (t_min, _) = compute_aabb(&trunk_builder.positions);
        let (_, f_max) = compute_aabb(&foliage_builder.positions);
        let t_height = f_max[1] - t_min[1];
        assert!(
            t_height >= 10.0 && t_height <= 16.0,
            "Tree height {t_height} m should be ~12.0 m"
        );

        // Verify sector metric scale factor
        assert_eq!(SECTOR_METRIC_SCALE, 0.045);
        let sector_extent = 96.0 * SECTOR_METRIC_SCALE;
        assert!(
            sector_extent < 4.5,
            "Sector extent {sector_extent} must comfortably fit in GPU viewport"
        );
    }

    #[test]
    fn test_sector_compilation_and_rover_vehicle_driving_forward_axis() {
        let textures = build_procedural_textures();
        assert_eq!(textures.len(), 4, "Expected 4 procedural terrain textures");

        let sector = build_sector_10d("talbingo", "Talbingo Test", &textures, false)
            .expect("Sector build should succeed");

        assert!(!sector.positions.is_empty(), "Sector positions must not be empty");
        assert!(!sector.indices.is_empty(), "Sector indices must not be empty");
        assert!(!sector.rover_local_positions.is_empty(), "Rover local positions must be extracted");
        assert!(!sector.walker_local_positions.is_empty(), "Walker local positions must be extracted");

        // Verify that rover_local_positions has vehicle length along Z for WASM driving physics
        let (rl_min, rl_max) = compute_aabb(&sector.rover_local_positions);
        let z_len = rl_max[2] - rl_min[2];
        let x_len = rl_max[0] - rl_min[0];
        let y_len = rl_max[1] - rl_min[1];

        // In rover_local_positions, Z must be the longest axis (length > width and length > height)
        assert!(
            z_len > x_len && z_len > y_len,
            "Rover local positions must have vehicle length along local Z for WASM vehicle steering/drive"
        );

        // Rover scaled length in sector coordinates: 4.85 * 0.045 = ~0.218
        assert!(
            z_len >= 0.19 && z_len <= 0.25,
            "Scaled rover length {z_len} must correspond to 4.85 m * SECTOR_METRIC_SCALE"
        );
    }

    #[test]
    fn test_closeup_specimens_generation() {
        let textures = build_procedural_textures();
        let kanga_specimen = build_closeup_specimen_10d("specimen-kangaroo", "Eastern Grey Kangaroo", &textures, true)
            .expect("Closeup specimen kangaroo build should succeed");
        assert!(!kanga_specimen.positions.is_empty());
        assert_eq!(kanga_specimen.cam_target, [1.02, 0.12, 0.06]);

        let flora_specimen = build_closeup_specimen_10d("specimen-flora", "Corymbia Ghost Gum", &textures, false)
            .expect("Closeup specimen flora build should succeed");
        assert!(!flora_specimen.positions.is_empty());
        assert_eq!(flora_specimen.cam_target, [0.45, 0.42, 0.15]);
    }
}


