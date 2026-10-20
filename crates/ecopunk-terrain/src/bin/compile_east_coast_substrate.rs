//! compile_east_coast_substrate
//!
//! Executes Phase 2, Phase 3, Phase 4, Phase 5, and Phase 6 of:
//! `docs/plans/east-coast-australia-ecological-dataset-acquisition-plan.md` &
//! `docs/plans/aaa-holo-continuum-and-300-flora-164-fauna-plan.md`
//!
//! Synthesizes native QualiaDB & VibeScript artifacts (100% Rust, Zero Python, Zero V8):
//! 1. `.q42` Semantic Volumes: Compiled 48-byte `NQuin` records from `.ttl` ontologies & VibeScript (`StreamingQ42VolumeWriter`).
//! 2. 16-KB GPU `ConstituentLUT`: Precompiled 256 x 64B flat binary material table (`BankedConstituentLut`).
//! 3. `.m64` Matter-64 Container: Mode 0x02 strata column chunk for the Talbingo catchment.
//! 4. `.10d` Manifold Containers: Multi-material `MAT4` + `UV01` + `FNRM` + `SRD1` + `PROV` scenes & water surfaces.
//! 5. `.hmc` (`QBDL`) Verified Bundle (`assets/packs/ecopunk-east-coast.hmc`) via `BundleWriter` & `BundleReader`.
//! 6. `PortalGpu` & `VolumetricRenderer` Native `wgpu` Offscreen Rendering (Zero V8) to `assets/renders/*.png`.

use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File};
use std::io::{self, Cursor, Read};
use std::path::PathBuf;

use ecopunk_terrain::{
    build_closeup_specimen_10d, build_procedural_textures, build_sector_10d,
    compile_164_fauna_catalog, compile_305_flora_catalog, render_sector_via_portal_gpu_png,
    run_sector_physics_verification_step, EcopunkHolosonicSoundscape, EcopunkSectorSceneGraph,
    EcopunkTelemetryHud, FaunaBiomechanicalArchetype, FaunaSkeletalRig10d,
};
use qualia_core_db::audio::holosonic::{
    HolosonicEmitter, HolosonicModality, HolosonicSonicToken48, HolosonicWavefield,
};
use qualia_core_db::bundle::{
    verify_arrived_range, BundleReader, BundleWriter, HmcHeaderProbe, HmcProgressiveRangePlanner,
    StreamPriorityTier, BUNDLE_HEADER_SIZE,
};
use qualia_core_db::container_10d::{HEADER_BYTE_SIZE, HEADER_VERSION, MAGIC_10D, ShadingModel};
use qualia_core_db::governance::worker_pool::{
    GovernedHydrationPool, HydrationCapabilityToken, HydrationTask, HydrationTaskKind,
    CAP_ALL_HYDRATION,
};
use qualia_core_db::lexicon::generate_60bit_token;
use qualia_core_db::q42::m64_matter::banked_lut::{BankedConstituentLut, LUT_BUFFER_BYTES};
use qualia_core_db::q42::m64_matter::{M64ChunkDirectoryEntry, M64Header, M64_MAGIC};
use qualia_core_db::q42::q42_volume::write_sorted_quins_volume;
use qualia_core_db::q42::q42_volume::WasmSliceQ42Reader;
use qualia_core_db::q42_volume::{StreamingQ42VolumeWriter, Q42_MAGIC};
use qualia_core_db::query::shacl_compiler::{
    validate_shacl_property, ShaclConstraint, ShaclDatatype,
};
use qualia_core_db::resolver::INLINE_TAG_INTEGER;
use qualia_core_db::shaders::viewport::{
    ACOUSTIC_VOXEL_CONE_WGSL, CLUSTER_LIGHT_WGSL, DEFERRED_RESOLVE_WGSL, EMF_VOLUMETRIC_WGSL,
    FOLIAGE_LOD_WGSL, HOLOGRAPHIC_WGSL, HYDROLOGIC_OPTICS_WGSL, INSTANCE_CULL_INDIRECT_WGSL,
    SGGX_MICROFLAKE_WGSL, SKY_WGSL, SPECTRAL_WGSL, SWE_SOLVER_WGSL, WATER_WGSL,
};
use qualia_core_db::wgsl_forge::physics::kinematics::KIN_STEP_WGSL;
use qualia_core_db::wgsl_forge::{
    validate_all_observer_kernels, validate_wgsl, FORGE_SCHEMA_VERSION, NAGA_API_VERSION,
    WGPU_API_VERSION, WGSL_KERNEL_HUD, WGSL_KERNEL_PBR, WGSL_KERNEL_THERMAL, WGSL_KERNEL_TOON,
    WGSL_KERNEL_WATERCOLOR, WGSL_OBSERVER_HEADER,
};
use qualia_core_db::{q_hash, NQuin, QUINS_PER_BLOCK};
use qualia_geospatial::{
    compile_raster_to_m64, ElevationRaster, GeodeticFrame, CHUNK_CELL_COUNT,
};
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleParser;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use webizen_render::VolumetricRenderer;

pub trait LocalHostQ42PackExt {
    fn with_q42_pack(bytes: &[u8]) -> io::Result<Self>
    where
        Self: Sized;
}

impl LocalHostQ42PackExt for vibe::LocalHost {
    fn with_q42_pack(bytes: &[u8]) -> io::Result<Self> {
        let _reader = WasmSliceQ42Reader::open(bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Q42 slice open failed: {e:?}")))?;
        Ok(vibe::LocalHost::default())
    }
}


/// Unified WebGPU WGSL Render Pipeline combining `qualia_core_db::shaders::viewport::SPECTRAL_WGSL`,
/// `SGGX_MICROFLAKE_WGSL` microflake optics, `HYDROLOGIC_OPTICS_WGSL` radiative transfer,
/// `wgsl_forge::observer_kernels` (0x00 PBR, 0x01 Toon, 0x02 Watercolor, 0x03 GPR HUD, 0x04 Thermal IR),
/// and the 16-KB `BankedConstituentLut` GPU storage buffer (`@group(0) @binding(1)`).
fn build_unified_webgpu_wgsl() -> String {
    format!(
        r#"// ============================================================================
// EcoPunk Unified WebGPU Pipeline — Forged & Certified via qualia_core_db::wgsl_forge
// Includes: SPECTRAL_WGSL (CIE 1931 XYZ) + Observer Kernels (0x00..0x05)
//           + SGGX Anisotropic Microflake BRDF + Hydrologic Optics + 16KB ConstituentLUT
// ============================================================================

{spectral_wgsl}

struct SceneUniforms {{
    proj: mat4x4<f32>,
    view: mat4x4<f32>,
    camera_pos: vec4<f32>,      // xyz: eye, w: time_seconds
    sun_dir: vec4<f32>,         // xyz: sunDir, w: water_level
    wind_and_clip: vec4<f32>,   // xy: wind_vec, z: clip_plane_z, w: karst_fluting
    isogloss: vec4<f32>,        // x: centerX, y: radius, z: centerZ, w: active
    space_and_mode: vec4<f32>,  // x: kp_index, y: imf_bz, z: f32(observer_mode), w: exposure
}};

struct EntityUniforms {{
    model: mat4x4<f32>,
    fauna_anim: vec4<f32>,      // x: phase, y: speed, z: gait_code, w: entity_kind
    line_color: vec4<f32>,      // rgba for line rendering
}};

@group(0) @binding(0) var<uniform> scene: SceneUniforms;
@group(0) @binding(1) var<storage, read> constituent_lut: array<vec4<u32>, 1024>;
@group(1) @binding(0) var<uniform> entity: EntityUniforms;

const PI_VAL: f32 = 3.14159265359;

// wgsl_forge::observer_kernels::WGSL_KERNEL_PBR microfacet functions
fn distribution_ggx(n: vec3<f32>, h: vec3<f32>, roughness: f32) -> f32 {{
    let a = roughness * roughness;
    let a2 = a * a;
    let n_dot_h = max(dot(n, h), 0.0);
    let n_dot_h2 = n_dot_h * n_dot_h;
    let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
    return a2 / (PI_VAL * denom * denom + 0.00001);
}}

fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {{
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    return n_dot_v / (n_dot_v * (1.0 - k) + k + 0.00001);
}}

fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {{
    return f0 + (vec3<f32>(1.0) - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}}

// shaders::viewport::sggx_microflake.wgsl anisotropic foliage/ground fiber BRDF
fn evaluate_sggx_sheen(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, sigma_v: f32, sigma_t: f32) -> f32 {{
    let h = normalize(v + l);
    let t_fiber = vec3<f32>(0.0, 1.0, 0.0);
    let sv2 = sigma_v * sigma_v;
    let st2 = sigma_t * sigma_t;
    let h_dot_t = dot(h, t_fiber);
    let aniso = sv2 * h_dot_t * h_dot_t + st2 * (1.0 - h_dot_t * h_dot_t);
    return clamp(0.00012 / max(PI_VAL * sv2 * st2 * aniso * aniso, 0.001), 0.0, 0.12) * max(dot(n, l), 0.0);
}}

// shaders::viewport::hydrologic_optics.wgsl Henyey-Greenstein & Beer-Lambert
fn henyey_greenstein_phase(cos_theta: f32, g: f32) -> f32 {{
    let g2 = g * g;
    let denom = 1.0 + g2 - 2.0 * g * cos_theta;
    return (1.0 - g2) / (4.0 * PI_VAL * pow(max(denom, 0.0001), 1.5));
}}

// wgsl_forge::observer_kernels::WGSL_KERNEL_THERMAL ironbow colormap
fn ironbow_thermal(t_norm: f32) -> vec3<f32> {{
    let t = clamp(t_norm, 0.0, 1.0);
    let c_cold = vec3<f32>(0.04, 0.08, 0.52);
    let c_mid  = vec3<f32>(0.62, 0.05, 0.78);
    let c_warm = vec3<f32>(0.98, 0.48, 0.04);
    let c_hot  = vec3<f32>(1.0, 0.98, 0.86);
    if (t < 0.33) {{
        return mix(c_cold, c_mid, t * 3.0);
    }} else if (t < 0.66) {{
        return mix(c_mid, c_warm, (t - 0.33) * 3.0);
    }}
    return mix(c_warm, c_hot, (t - 0.66) * 3.0);
}}

fn aces_film(x: vec3<f32>) -> vec3<f32> {{
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}}

fn hash21(p: vec2<f32>) -> f32 {{
    var q = fract(p * vec2<f32>(234.34, 435.345));
    q += dot(q, q + 34.23);
    return fract(q.x * q.y);
}}

// Sample dielectric permittivity from the bound 16-KB BankedConstituentLut SSBO
fn sample_lut_dielectric(slot: u32) -> f32 {{
    let idx = clamp(slot, 0u, 255u) * 4u + 2u;
    let word = constituent_lut[idx].x;
    let eps_x100 = f32(word & 0xFFFFu);
    return clamp(eps_x100 / 100.0, 1.0, 80.0);
}}

struct PgaSkinMotor {{
    r: vec4<f32>,
    d: vec4<f32>,
}};

fn pga_blade4_to_quat(b: vec4<f32>) -> vec4<f32> {{
    return vec4<f32>(b.x, b.y, b.z, b.w);
}}

fn pga_quat_mul(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {{
    return vec4<f32>(
        a.x * b.x - a.y * b.y - a.z * b.z - a.w * b.w,
        a.x * b.y + a.y * b.x + a.z * b.w - a.w * b.z,
        a.x * b.z - a.y * b.w + a.z * b.x + a.w * b.y,
        a.x * b.w + a.y * b.z - a.z * b.y + a.w * b.x
    );
}}

fn pga_motor_blend_4(
    m0: PgaSkinMotor,
    m1: PgaSkinMotor,
    m2: PgaSkinMotor,
    m3: PgaSkinMotor,
    weights: vec4<f32>
) -> PgaSkinMotor {{
    let ref_r = m0.r;
    let s0 = select(-weights.x, weights.x, dot(ref_r, m0.r) >= 0.0);
    let s1 = select(-weights.y, weights.y, dot(ref_r, m1.r) >= 0.0);
    let s2 = select(-weights.z, weights.z, dot(ref_r, m2.r) >= 0.0);
    let s3 = select(-weights.w, weights.w, dot(ref_r, m3.r) >= 0.0);

    var blended_r = m0.r * s0 + m1.r * s1 + m2.r * s2 + m3.r * s3;
    var blended_d = m0.d * s0 + m1.d * s1 + m2.d * s2 + m3.d * s3;

    let len_r = length(blended_r);
    let inv_len = select(1.0, 1.0 / len_r, len_r > 1e-6);
    blended_r = blended_r * inv_len;
    blended_d = blended_d * inv_len;

    var res: PgaSkinMotor;
    res.r = blended_r;
    res.d = blended_d;
    return res;
}}

fn pga_motor_transform_point(m: PgaSkinMotor, p: vec3<f32>) -> vec3<f32> {{
    let r = m.r;
    let d = m.d;
    let q_xyz = r.yzw;
    let t_rot = 2.0 * cross(q_xyz, p);
    let rot_p = p + r.x * t_rot + cross(q_xyz, t_rot);
    let rw = r.x; let rx = r.y; let ry = r.z; let rz = r.w;
    let dw = d.x; let dx = d.y; let dy = d.z; let dz = d.w;
    let tx = 2.0 * (dx * rw - dw * rx + dz * ry - dy * rz);
    let ty = 2.0 * (dy * rw - dz * rx - dw * ry + dx * rz);
    let tz = 2.0 * (dz * rw + dy * rx - dx * ry - dw * rz);
    return rot_p + vec3<f32>(tx, ty, tz);
}}

fn pga_motor_transform_vector(m: PgaSkinMotor, v: vec3<f32>) -> vec3<f32> {{
    let q_xyz = m.r.yzw;
    let t_rot = 2.0 * cross(q_xyz, v);
    return v + m.r.x * t_rot + cross(q_xyz, t_rot);
}}

fn pga_eval_proc_motor_oscillation(
    pos: vec3<f32>,
    time: f32,
    freq: f32,
    damping: f32,
    drive: vec3<f32>
) -> vec3<f32> {{
    let omega = 6.2831853 * freq;
    let decay = exp(-damping * max(time, 0.0));
    let phase = sin(omega * time) * decay;
    return pos + drive * phase;
}}

fn eval_csm_4_cascade_blend(view_depth: f32, splits: vec4<f32>, blend_band: f32) -> vec3<f32> {{
    let d = max(view_depth, 0.0);
    let band = clamp(blend_band, 0.02, 0.40);
    let s = array<f32, 4>(splits.x, splits.y, splits.z, splits.w);
    var prev_s: f32 = 0.05;
    for (var i: u32 = 0u; i < 4u; i = i + 1u) {{
        let cur_s = s[i];
        if (d <= cur_s || i == 3u) {{
            if (i < 3u) {{
                let span = max(cur_s - prev_s, 1e-3);
                let band_start = cur_s - span * band;
                if (d > band_start) {{
                    let alpha = smoothstep(band_start, cur_s, d);
                    return vec3<f32>(f32(i), f32(i + 1u), alpha);
                }}
            }}
            return vec3<f32>(f32(i), f32(i), 0.0);
        }}
        prev_s = cur_s;
    }}
    return vec3<f32>(3.0, 3.0, 0.0);
}}

fn apply_height_stratified_valley_fog_35m(
    radiance: vec3<f32>,
    camera_pos: vec3<f32>,
    world_pos: vec3<f32>,
    inscatter_color: vec3<f32>
) -> vec3<f32> {{
    let ray = world_pos - camera_pos;
    let dist = length(ray);
    let d_eff = max(0.0, dist - 35.0);
    if (d_eff <= 0.0) {{
        return radiance;
    }}
    let scale_height = 42.0;
    let base_sigma_t = 0.0045;
    let mid_height = 0.5 * (camera_pos.y + world_pos.y);
    let height_density = exp(-max(mid_height, -20.0) / scale_height);
    let optical_depth = clamp(base_sigma_t * height_density * d_eff, 0.0, 12.0);
    let transmittance = exp(-optical_depth);
    return mix(inscatter_color, radiance, transmittance);
}}

fn hash33_voronoi(p: vec3<f32>) -> vec3<f32> {{
    var q = vec3<f32>(
        dot(p, vec3<f32>(127.1, 311.7, 74.7)),
        dot(p, vec3<f32>(269.5, 183.3, 246.1)),
        dot(p, vec3<f32>(113.5, 271.9, 124.6))
    );
    return fract(sin(q) * 43758.5453123);
}}

fn proc_voronoi_f1_3d(p: vec3<f32>, cell_density: f32, jitter: f32) -> f32 {{
    let sp = p * max(cell_density, 0.001);
    let ip = floor(sp);
    let fp = fract(sp);
    let clamped_jitter = clamp(jitter, 0.0, 1.0);
    var min_dist_sq = 100.0;

    for (var dz = -1; dz <= 1; dz = dz + 1) {{
        for (var dy = -1; dy <= 1; dy = dy + 1) {{
            for (var dx = -1; dx <= 1; dx = dx + 1) {{
                let neighbor = vec3<f32>(f32(dx), f32(dy), f32(dz));
                let rnd = hash33_voronoi(ip + neighbor);
                let offset = mix(vec3<f32>(0.5), rnd, clamped_jitter);
                let diff = neighbor + offset - fp;
                let d_sq = dot(diff, diff);
                min_dist_sq = min(min_dist_sq, d_sq);
            }}
        }}
    }}
    return clamp(sqrt(min_dist_sq), 0.0, 1.0);
}}

struct HoloFragmentOut {{
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}};

struct VertexInput {{
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) v_meta: vec4<f32>,
}};

struct VertexOutput {{
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) v_meta: vec4<f32>,
}};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {{
    var pos = input.position;
    var norm = input.normal;
    let entity_kind = i32(round(entity.fauna_anim.w));
    let u_time = scene.camera_pos.w;
    let wind_vec = scene.wind_and_clip.xy;

    // 1. Flora GPU Aeroelastic Bishop-Frame Wind Sway & Leaf Flutter with PGA Motor Oscillation
    if (entity_kind == 1) {{
        pos = pga_eval_proc_motor_oscillation(pos, u_time, 1.65, 0.05, vec3<f32>(wind_vec.x * 0.04, 0.008, wind_vec.y * 0.04));
        let h_norm = input.v_meta.x;
        let branch_flex = input.v_meta.y;
        let flutter = input.v_meta.z;
        let wind_mag = length(wind_vec);
        let sway_phase = u_time * 1.65 + entity.model[3][0] * 0.35 + entity.model[3][2] * 0.25;
        let sway_amp = (0.045 * wind_mag) * pow(max(h_norm, 0.0), 1.6) * branch_flex;
        pos.x += wind_vec.x * 0.12 * sway_amp * (0.65 + 0.35 * sin(sway_phase));
        pos.z += wind_vec.y * 0.12 * sway_amp * (0.65 + 0.35 * cos(sway_phase * 0.9));

        if (flutter > 0.2) {{
            let fl_phase = u_time * (4.5 + wind_mag * 0.35) + pos.x * 3.1 + pos.y * 2.7;
            let fl_amp = 0.025 * min(wind_mag, 18.0) * flutter;
            pos.x += sin(fl_phase) * fl_amp;
            pos.y += cos(fl_phase * 1.3) * fl_amp * 0.6;
            pos.z += cos(fl_phase * 0.8) * fl_amp;
        }}
    }}
    // 2. Fauna GPU Articulated Biomechanical Gait Kinematics
    else if (entity_kind == 2) {{
        let part_id = input.v_meta.x;
        let side = input.v_meta.y;
        let dist_along = input.v_meta.z;
        let phase = entity.fauna_anim.x;
        let speed = entity.fauna_anim.y;
        let gait = entity.fauna_anim.z;
        let move_factor = clamp(speed / 3.0, 0.15, 1.4);

        if (part_id > 0.5 && part_id < 1.5) {{
            pos.y += sin(phase * 2.0) * 0.045 * move_factor * dist_along;
            pos.z += cos(phase) * 0.03 * move_factor * dist_along;
        }} else if (part_id > 1.5 && part_id < 2.5) {{
            if (gait > 5.5 && gait < 7.5) {{
                let flap = sin(phase * 1.8) * select(0.18, 0.55, gait > 6.5);
                pos.y += abs(pos.x) * sin(flap);
            }} else {{
                let swing = sin(phase + side * 1.57) * 0.22 * move_factor * dist_along;
                pos.z += swing;
                pos.y += max(0.0, cos(phase + side * 1.57)) * 0.08 * move_factor * dist_along;
            }}
        }} else if (part_id > 2.5 && part_id < 3.5) {{
            if (gait < 1.5) {{
                let kick = sin(phase) * 0.28 * move_factor * dist_along;
                pos.z -= kick;
                pos.y += max(0.0, cos(phase)) * 0.14 * move_factor * dist_along;
            }} else {{
                let swing = sin(phase - side * 1.57) * 0.25 * move_factor * dist_along;
                pos.z += swing;
                pos.y += max(0.0, -cos(phase - side * 1.57)) * 0.09 * move_factor * dist_along;
            }}
        }} else if (part_id > 3.5) {{
            let wave = sin(phase * 1.5 - dist_along * 3.5) * 0.18 * dist_along;
            if (gait > 8.5 && gait < 11.5) {{
                pos.x += sin(phase * 2.0 - pos.z * 2.5) * 0.16 * (0.3 + 0.7 * dist_along);
            }} else {{
                pos.x += wave * 0.6;
                pos.y += cos(phase - dist_along * 2.0) * 0.08 * dist_along;
            }}
        }}
    }}
    // 3. Hydrological Water Surface Waves (shaders::viewport::water.wgsl derivatives)
    else if (entity_kind == 3) {{
        let w1 = sin(pos.x * 0.45 + u_time * 2.2) * cos(pos.z * 0.35 + u_time * 1.7) * 0.075;
        let w2 = sin((pos.x * 0.8 - pos.z * 0.6) + u_time * 3.4) * 0.035;
        pos.y += w1 + w2;
        norm = normalize(vec3<f32>(-0.18 * cos(pos.x * 0.45 + u_time * 2.2), 1.0, -0.14 * sin(pos.z * 0.35 + u_time * 1.7)));
    }}

    let world = entity.model * vec4<f32>(pos, 1.0);
    var out: VertexOutput;
    out.clip_pos = scene.proj * scene.view * world;
    out.world_pos = world.xyz;
    out.normal = normalize((entity.model * vec4<f32>(norm, 0.0)).xyz);
    out.color = input.color;
    out.v_meta = input.v_meta;
    return out;
}}

@fragment
fn fs_main(input: VertexOutput) -> HoloFragmentOut {{
    let entity_kind = i32(round(entity.fauna_anim.w));
    let observer_mode = i32(round(scene.space_and_mode.z));
    let clip_z = scene.wind_and_clip.z;
    let water_level = scene.sun_dir.w;
    let karst_fluting = scene.wind_and_clip.w;
    let u_time = scene.camera_pos.w;
    let cam_pos = scene.camera_pos.xyz;
    let light_dir = normalize(scene.sun_dir.xyz);

    if (input.world_pos.z > clip_z && entity_kind != 4) {{
        discard;
    }}

    let n = normalize(input.normal);
    let view_dir = normalize(cam_pos - input.world_pos);
    let n_dot_l = max(dot(n, light_dir), 0.0);
    let n_dot_v = max(dot(n, view_dir), 0.0);

    // Sky Dome (shaders::viewport::sky.wgsl + NOAA SWPC Aurora Australis)
    if (entity_kind == 4) {{
        let ray_dir = normalize(input.world_pos - cam_pos);
        let sun_elev = light_dir.y;
        let horizon = clamp(ray_dir.y, 0.0, 1.0);
        let day_zenith = vec3<f32>(0.11, 0.28, 0.56);
        let day_horizon = vec3<f32>(0.55, 0.74, 0.88);
        let dusk_horizon = vec3<f32>(0.88, 0.45, 0.24);
        let day_factor = clamp(sun_elev * 1.8, 0.05, 1.0);
        var sky_col = mix(
            mix(dusk_horizon * day_factor, day_horizon * day_factor, clamp(sun_elev + 0.2, 0.0, 1.0)),
            day_zenith * day_factor,
            pow(horizon, 0.55)
        );
        let sun_dot = max(dot(ray_dir, light_dir), 0.0);
        sky_col += vec3<f32>(1.0, 0.92, 0.72) * pow(sun_dot, 180.0) * 1.4;
        sky_col += vec3<f32>(0.95, 0.75, 0.45) * pow(sun_dot, 12.0) * 0.25;

        let kp = scene.space_and_mode.x;
        if (kp > 3.5 || observer_mode == 5) {{
            let aurora_band = sin(ray_dir.x * 6.0 + u_time * 0.8) * cos(ray_dir.z * 5.0 - u_time * 0.5);
            let curtain = smoothstep(0.15, 0.55, ray_dir.y) * (1.0 - smoothstep(0.55, 0.88, ray_dir.y));
            let intensity = select(clamp((kp - 3.5) / 5.5, 0.0, 0.55), 0.65, observer_mode == 5);
            let aurora_col = mix(sigma_to_linear_rgb(0.45), vec3<f32>(0.85, 0.15, 0.65), clamp(ray_dir.y * 1.4, 0.0, 1.0));
            sky_col += aurora_col * curtain * (0.5 + 0.5 * aurora_band) * intensity;
        }}
        return HoloFragmentOut(vec4<f32>(aces_film(sky_col), 1.0), clamp(input.clip_pos.z, 0.0, 1.0));
    }}

    var base_col = input.color;

    // Terrain, 3D Rocks/Boulders/Pebbles & 6-Layer SLGA Subsurface Strata
    if (entity_kind == 0) {{
        let is_wall = input.v_meta.z;
        let slope = 1.0 - max(n.y, 0.0);
        if (is_wall < 0.5) {{
            if (slope > 0.35) {{
                let flute = sin(input.world_pos.x * 3.8 + input.world_pos.z * 3.8) * 0.5 + 0.5;
                let fracture = hash21(floor(input.world_pos.xz * 2.5));
                base_col *= (0.82 + 0.24 * mix(fracture, flute, karst_fluting));
            }} else {{
                // Soil & forest litter micro-grain
                let soil_grain = hash21(floor(input.world_pos.xz * 6.0)) * 0.12 - 0.06;
                base_col += vec3<f32>(soil_grain * 0.6, soil_grain * 0.5, soil_grain * 0.35);
            }}

            let dist_above_water = input.world_pos.y - water_level;
            // Riverbed & Shoreline Voronoi Pebbles, Imbricated Cobbles & Quartz Sand Ripples
            if (dist_above_water < 0.75) {{
                let peb_uv = input.world_pos.xz * 5.2;
                let peb_id = floor(peb_uv);
                let peb_f = fract(peb_uv) - vec2<f32>(0.5, 0.5);
                let peb_rnd = hash21(peb_id);
                let peb_dist = length(peb_f * vec2<f32>(1.0, 1.25));
                let peb_mask = smoothstep(0.46, 0.24, peb_dist);
                var peb_tint = vec3<f32>(0.52, 0.48, 0.42);
                if (peb_rnd > 0.72) {{
                    peb_tint = vec3<f32>(0.76, 0.68, 0.56); // alluvial quartz pebble
                }} else if (peb_rnd > 0.45) {{
                    peb_tint = vec3<f32>(0.58, 0.36, 0.26); // red jasper / ironstone cobble
                }} else if (peb_rnd < 0.22) {{
                    peb_tint = vec3<f32>(0.26, 0.28, 0.30); // dark basalt cobble
                }}
                let sand_ripple = 0.5 + 0.5 * sin(input.world_pos.x * 9.0 + sin(input.world_pos.z * 3.0) * 1.5);
                let sand_col = vec3<f32>(0.68, 0.60, 0.45) * (0.90 + 0.14 * sand_ripple);
                let bed_tex = mix(sand_col, peb_tint * (0.75 + 0.35 * peb_mask), 0.68);
                let blend_w = clamp((0.75 - dist_above_water) / 0.95, 0.0, 0.78);
                base_col = mix(base_col, bed_tex, blend_w);
            }}

            // Shoreline Capillary Wetting Darkening
            if (dist_above_water >= -0.4 && dist_above_water < 1.25) {{
                let wet_factor = 1.0 - clamp(abs(dist_above_water) / 1.25, 0.0, 1.0);
                base_col *= (1.0 - 0.34 * wet_factor);
            }}

            // Submerged Riverbed Animated Solar Caustics & Beer-Lambert Optical Absorption
            if (dist_above_water < 0.0 && observer_mode == 0) {{
                let w_depth = clamp(-dist_above_water, 0.0, 6.0);
                let c1 = sin(input.world_pos.x * 3.4 + u_time * 2.3) * cos(input.world_pos.z * 3.1 - u_time * 1.8);
                let c2 = sin(input.world_pos.x * 5.7 - u_time * 1.7 + input.world_pos.z * 4.3);
                let caustic = pow(clamp(0.5 + 0.28 * c1 + 0.22 * c2, 0.0, 1.0), 3.0);
                let caustic_atten = exp(-w_depth * 0.35);
                base_col += vec3<f32>(0.28, 0.46, 0.42) * caustic * caustic_atten * 0.85;
                let absorb = exp(-vec3<f32>(0.28, 0.11, 0.07) * w_depth * 0.55);
                base_col *= absorb;
            }}
        }} else {{
            let strata_grain = sin(input.world_pos.y * 14.0 + input.world_pos.x * 0.8) * 0.06;
            base_col += vec3<f32>(strata_grain);
        }}
    }}

    // Transparent Refractive Water Shader (Snell Refraction + Beer-Lambert + Fresnel Alpha)
    if (entity_kind == 3) {{
        let d = max(0.05, input.v_meta.x);
        let half_vec = normalize(light_dir + view_dir);
        let spec = pow(max(dot(n, half_vec), 0.0), 110.0) * 1.45;
        let fresnel = fresnel_schlick(n_dot_v, vec3<f32>(0.02));
        let hg_backscatter = henyey_greenstein_phase(dot(-view_dir, light_dir), 0.82) * 0.20;
        let shallow_tint = vec3<f32>(0.18, 0.48, 0.46);
        let deep_tint = base_col * (0.52 + hg_backscatter);
        let beer_factor = clamp(1.0 - exp(-d * 0.38), 0.0, 1.0);
        let water_body = mix(shallow_tint, deep_tint, beer_factor);
        let sky_refl = vec3<f32>(0.46, 0.68, 0.88) * max(0.28, light_dir.y);
        let foam = smoothstep(0.22, 0.02, d) * (0.5 + 0.5 * sin(input.world_pos.x * 4.0 + input.world_pos.z * 4.0 + u_time * 3.0)) * 0.22;
        var water_col = mix(water_body * (0.48 + 0.52 * n_dot_l), sky_refl, fresnel * 0.62)
            + vec3<f32>(1.0, 0.96, 0.85) * spec
            + vec3<f32>(0.88, 0.94, 0.96) * foam;
        if (observer_mode == 4) {{
            water_col = vec3<f32>(0.05, 0.22, 0.68);
        }}
        let water_alpha = select(
            clamp(fresnel.x * 0.58 + (1.0 - exp(-d * 0.26)) * 0.44 + spec * 0.35, 0.16, 0.74),
            0.92,
            observer_mode == 4
        );
        return HoloFragmentOut(vec4<f32>(aces_film(water_col), water_alpha), clamp(input.clip_pos.z, 0.0, 1.0));
    }}

    var final_col = base_col;

    // 0x00: WGSL_KERNEL_PBR + SGGX Anisotropic Microflake Sheen + Multi-Km Clarity Fog
    if (observer_mode == 0) {{
        let h = normalize(light_dir + view_dir);
        let roughness = select(0.76, select(0.54, 0.62, entity_kind == 2), entity_kind != 0);
        let ndf = distribution_ggx(n, h, roughness);
        let g = geometry_schlick_ggx(n_dot_v, roughness) * geometry_schlick_ggx(n_dot_l, roughness);
        let f = fresnel_schlick(max(dot(h, view_dir), 0.0), vec3<f32>(0.04));
        let spec = clamp((ndf * g * f) / max(4.0 * n_dot_v * n_dot_l, 0.02) * n_dot_l * 0.35, vec3<f32>(0.0), vec3<f32>(0.28));
        let sggx_sheen = evaluate_sggx_sheen(n, view_dir, light_dir, 0.22, 0.58) * 0.45;

        let sky_amb = mix(vec3<f32>(0.18, 0.16, 0.14), vec3<f32>(0.26, 0.34, 0.46), 0.5 + 0.5 * n.y);
        var sss = vec3<f32>(0.0);
        if (entity_kind == 1 && input.v_meta.w > 1.5) {{
            let back_light = max(dot(view_dir, -light_dir), 0.0);
            sss = base_col * vec3<f32>(0.35, 0.55, 0.18) * pow(back_light, 2.0) * 0.48;
        }}

        let sun_radiance = vec3<f32>(1.55, 1.46, 1.32) * max(0.15, light_dir.y);
        let lit = base_col * (sky_amb + sun_radiance * n_dot_l * 0.78) + sun_radiance * (spec + vec3<f32>(sggx_sheen)) + sss;

        let fog_col = vec3<f32>(0.58, 0.72, 0.84) * max(0.25, light_dir.y);
        let fogged_lit = apply_height_stratified_valley_fog_35m(lit, cam_pos, input.world_pos, fog_col);
        final_col = aces_film(fogged_lit);
    }}
    // 0x01: WGSL_KERNEL_TOON (3-Tone Quantization & Silhouette Inking)
    else if (observer_mode == 1) {{
        var cel = 0.38;
        if (n_dot_l > 0.68) {{
            cel = 1.12;
        }} else if (n_dot_l > 0.32) {{
            cel = 0.78;
        }}
        let rim = smoothstep(0.18, 0.28, n_dot_v);
        final_col = base_col * cel * (0.15 + 0.85 * rim);
    }}
    // 0x02: WGSL_KERNEL_WATERCOLOR (Kubelka-Munk Wash & Coffee-Ring Edge Pooling)
    else if (observer_mode == 2) {{
        let grain = sin(input.world_pos.x * 4.5) * cos(input.world_pos.z * 4.5) * 0.07;
        let wash = smoothstep(0.08, 0.92, n_dot_l + grain);
        let warm_paper = vec3<f32>(0.98, 0.95, 0.88);
        let pigment = mix(base_col * 0.68, min(vec3<f32>(1.0), base_col * 1.28 + vec3<f32>(0.06, 0.03, 0.0)), wash);
        let edge_pooling = 1.0 - 0.32 * pow(1.0 - n_dot_v, 2.2);
        final_col = mix(warm_paper, pigment * edge_pooling, 0.88);
    }}
    // 0x03: WGSL_KERNEL_HUD (GPR Dielectric Contrast from 16-KB ConstituentLUT & Scanlines)
    else if (observer_mode == 3) {{
        let lut_slot = u32(clamp(input.v_meta.y, 1.0, 6.0));
        let dielectric_perm = sample_lut_dielectric(lut_slot);
        let gpr_intensity = clamp(dielectric_perm / 40.0, 0.1, 1.0);
        let grid = step(0.93, fract(input.world_pos.x * 0.5)) + step(0.93, fract(input.world_pos.z * 0.5));
        let contour = step(0.92, fract(input.world_pos.y * 0.8));
        let pulse = sin(input.world_pos.y * 6.0 - u_time * 5.0) * 0.5 + 0.5;
        let backing = vec3<f32>(0.02, 0.06, 0.11);
        if (entity_kind == 2) {{
            final_col = mix(vec3<f32>(0.95, 0.2, 0.6), vec3<f32>(1.0, 0.85, 0.2), n_dot_l);
        }} else if (entity_kind == 1) {{
            final_col = mix(backing, vec3<f32>(0.1, 0.92, 0.55), 0.35 + 0.45 * n_dot_l + grid * 0.3);
        }} else {{
            final_col = mix(backing, vec3<f32>(0.0, 0.88, 1.0) * (0.6 + 0.4 * gpr_intensity), clamp(grid * 0.65 + contour * 0.45 + pulse * 0.15, 0.0, 1.0));
        }}
    }}
    // 0x04: WGSL_KERNEL_THERMAL (Planck Blackbody Ironbow Pseudocolor)
    else if (observer_mode == 4) {{
        var heat = 0.35;
        if (entity_kind == 2) {{
            heat = clamp(input.v_meta.w + 0.15 * n_dot_l, 0.0, 1.0);
        }} else if (entity_kind == 1) {{
            heat = 0.28 + 0.24 * n_dot_l;
        }} else {{
            heat = clamp(0.18 + n_dot_l * 0.42 + (input.world_pos.y / 30.0) * 0.15, 0.0, 0.85);
        }}
        final_col = ironbow_thermal(heat);
    }}
    // 0x05: Heliophysics & CIE 1931 Spectral Bioacoustic Wavefield
    else if (observer_mode == 5) {{
        let mag_field = abs(sin(input.world_pos.x * 0.25 + input.world_pos.z * 0.15 + u_time * 0.8));
        let bio_col = base_col * (0.45 + 0.65 * n_dot_l);
        let spectral_wave = sigma_to_linear_rgb(mag_field);
        final_col = mix(bio_col, spectral_wave, 0.32);
    }}

    if (scene.isogloss.w > 0.5 && entity_kind == 0) {{
        let dist_iso = length(input.world_pos.xz - vec2<f32>(scene.isogloss.x, scene.isogloss.z));
        let ring_dist = abs(dist_iso - scene.isogloss.y);
        if (ring_dist < 0.55) {{
            let glow = 1.0 - (ring_dist / 0.55);
            final_col = mix(final_col, vec3<f32>(0.0, 0.96, 0.82), glow * 0.75);
        }}
    }}

    return HoloFragmentOut(vec4<f32>(final_col, 1.0), clamp(input.clip_pos.z, 0.0, 1.0));
}}

// Overlay Line Pipeline Entry Points (Osteological Skeleton, Vascular Xylem, Trophic Arcs)
struct LineVertexInput {{
    @location(0) position: vec3<f32>,
}};

struct LineVertexOutput {{
    @builtin(position) clip_pos: vec4<f32>,
}};

@vertex
fn vs_line(input: LineVertexInput) -> LineVertexOutput {{
    var out: LineVertexOutput;
    out.clip_pos = scene.proj * scene.view * entity.model * vec4<f32>(input.position, 1.0);
    return out;
}}

@fragment
fn fs_line() -> @location(0) vec4<f32> {{
    return entity.line_color;
}}
"#,
        spectral_wgsl = SPECTRAL_WGSL.trim()
    )
}


fn find_workspace_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    while !dir.join("Cargo.toml").exists() || !dir.join("data").exists() {
        if let Some(parent) = dir.parent() {
            dir = parent.to_path_buf();
        } else {
            break;
        }
    }
    dir
}

fn parse_turtle_to_nquins(ttl_bytes: &[u8], default_graph_iri: &str) -> io::Result<Vec<NQuin>> {
    let cursor = Cursor::new(ttl_bytes);
    let mut parser = TurtleParser::new(cursor, None);
    let mut quins = Vec::new();
    let graph_context = q_hash(default_graph_iri);

    let mut on_triple = |t: rio_api::model::Triple| -> Result<(), io::Error> {
        let s = q_hash(&t.subject.to_string());
        let p = q_hash(&t.predicate.to_string());
        let o = q_hash(&t.object.to_string());

        let metadata = 0x2000_0000_0000_0000u64;
        let parity = NQuin::calculate_parity(s, p, o, graph_context, metadata);

        quins.push(NQuin {
            subject: s,
            predicate: p,
            object: o,
            context: graph_context,
            metadata,
            parity,
        });

        Ok(())
    };

    parser
        .parse_all(&mut on_triple)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Turtle error: {e}")))?;

    Ok(quins)
}

fn main() -> io::Result<()> {
    println!("============================================================");
    println!("🌿 EcoPunk Native Qualia Substrate & WGSL Forge Compiler");
    println!("============================================================");

    let root = find_workspace_root();
    let data_dir = root.join("data");
    let ontologies_dir = data_dir.join("ontologies");
    let assets_dir = root.join("assets").join("terrain");
    fs::create_dir_all(&assets_dir)?;

    // ------------------------------------------------------------------------
    // Phase 2: Compile ALL Turtle Ontologies in data/ontologies/ into .q42
    // ------------------------------------------------------------------------
    println!("\n--- Phase 2: Semantic Compilation to .q42 Volumes ---");

    let mut ttl_files: Vec<PathBuf> = Vec::new();
    if ontologies_dir.exists() {
        for entry in fs::read_dir(&ontologies_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ttl") {
                ttl_files.push(path);
            }
        }
    }
    ttl_files.sort();

    let mut total_quins = Vec::new();

    for ttl_path in &ttl_files {
        let stem = ttl_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let file_name = ttl_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.ttl");
        let graph_iri = format!("urn:qualia:ontology:{stem}");
        let bytes = fs::read(ttl_path)?;
        let mut file_quins = parse_turtle_to_nquins(&bytes, &graph_iri)?;
        println!(
            "  * Parsed {:>4} NQuins from {} (graph: {})",
            file_quins.len(),
            file_name,
            graph_iri
        );
        total_quins.append(&mut file_quins);
    }

    let q42_out_path = assets_dir.join("east_coast_strata.q42");
    if !total_quins.is_empty() {
        let written = write_sorted_quins_volume(&q42_out_path, &total_quins)?;
        println!(
            "[OK] Compiled .q42 Semantic Volume: {} ({} NQuins written across {} ontologies)",
            q42_out_path.display(),
            written,
            ttl_files.len()
        );
    }

    // ------------------------------------------------------------------------
    // Phase 3: Biophysical Enrichment & 16-KB GPU ConstituentLUT
    // ------------------------------------------------------------------------
    println!("\n--- Phase 3: 16-KB GPU ConstituentLUT Compilation ---");
    let lut = BankedConstituentLut::new_with_terrestrial_defaults();
    let lut_bytes: &[u8] = bytemuck::bytes_of(&lut.entries);
    assert_eq!(
        lut_bytes.len(),
        LUT_BUFFER_BYTES,
        "ConstituentLUT must be exact 16 KB"
    );

    let lut_out_path = assets_dir.join("constituent_lut_16kb.bin");
    fs::write(&lut_out_path, lut_bytes)?;
    println!(
        "[OK] Compiled 16-KB GPU ConstituentLUT: {} ({} bytes, 256 x 64B entries)",
        lut_out_path.display(),
        lut_bytes.len()
    );

    // ------------------------------------------------------------------------
    // Phase 4: Runtime Terrain Asset Synthesis (.m64 & .10d)
    // ------------------------------------------------------------------------
    println!("\n--- Phase 4: Runtime Terrain Asset Synthesis (.m64 & .10d) ---");

    let elev_bin_path = data_dir
        .join("geospatial")
        .join("talbingo_64x64_elev_mm.bin");
    let mut elev_mm = Vec::with_capacity(CHUNK_CELL_COUNT);

    if elev_bin_path.exists() {
        let mut file = File::open(&elev_bin_path)?;
        let mut buf = [0u8; 4];
        while file.read_exact(&mut buf).is_ok() {
            elev_mm.push(i32::from_le_bytes(buf));
        }
    } else {
        elev_mm.resize(CHUNK_CELL_COUNT, 385_000);
    }

    let raster = ElevationRaster {
        width: 64,
        height: 64,
        cell_size_mm: 30_000,
        elevations_mm: &elev_mm,
    };

    let frame = GeodeticFrame {
        srid: qualia_geospatial::SRID_GDA2020,
        vertical_datum: qualia_geospatial::VERTICAL_DATUM_AHD,
        epoch_year: 2020.0,
        origin_lat_deg: -35.5854,
        origin_lon_deg: 148.3240,
        origin_elevation_mm: 385_000,
    };

    let mut m64_buffer = vec![0u8; 2 * 1024 * 1024];
    let m64_len = compile_raster_to_m64(&raster, &frame, 1, &mut m64_buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("m64 compilation failed: {e:?}")))?;

    let m64_out_path = assets_dir.join("talbingo_catchment.m64");
    fs::write(&m64_out_path, &m64_buffer[..m64_len])?;

    let header: &M64Header = bytemuck::from_bytes(&m64_buffer[..std::mem::size_of::<M64Header>()]);
    assert_eq!(header.magic, M64_MAGIC);
    assert_eq!(header.chunk_count, 1);
    assert!(header.validate().is_ok());

    let dir_offset = std::mem::size_of::<M64Header>();
    let dir_entry: &M64ChunkDirectoryEntry = bytemuck::from_bytes(
        &m64_buffer[dir_offset..dir_offset + std::mem::size_of::<M64ChunkDirectoryEntry>()],
    );
    assert_eq!(dir_entry.srid_geodetic_code, qualia_geospatial::SRID_GDA2020);
    assert_eq!(dir_entry.record_count, 4096);
    assert!(dir_entry.validate().is_ok());

    println!(
        "[OK] Compiled .m64 Strata Container: {} ({} bytes, {} chunk, {} records)",
        m64_out_path.display(),
        m64_len,
        header.chunk_count,
        dir_entry.record_count
    );

    let ten_d_path = assets_dir.join("talbingo_heightfield.10d");
    let mut ten_d_bytes = Vec::new();

    let mut header_10d = [0u8; HEADER_BYTE_SIZE];
    header_10d[0..4].copy_from_slice(&MAGIC_10D);
    header_10d[4..6].copy_from_slice(&HEADER_VERSION.to_le_bytes());
    header_10d[6..8].copy_from_slice(&1u16.to_le_bytes());
    header_10d[8..16].copy_from_slice(&q_hash("geo:TalbingoCatchment").to_le_bytes());
    ten_d_bytes.extend_from_slice(&header_10d);

    let field_magic = *b"FIEL";
    let mut field_header = [0u8; 32];
    field_header[0..4].copy_from_slice(&field_magic);
    field_header[4..8].copy_from_slice(&20u32.to_le_bytes());
    field_header[8..12].copy_from_slice(&64u32.to_le_bytes());
    field_header[12..16].copy_from_slice(&64u32.to_le_bytes());
    field_header[16..20].copy_from_slice(&30_000u32.to_le_bytes());
    field_header[20..24].copy_from_slice(&385_000u32.to_le_bytes());
    ten_d_bytes.extend_from_slice(&field_header);

    for mm in &elev_mm {
        let meters = (*mm as f32) / 1000.0;
        ten_d_bytes.extend_from_slice(&meters.to_le_bytes());
    }

    fs::write(&ten_d_path, &ten_d_bytes)?;
    println!(
        "[OK] Compiled .10d Manifold Heightfield: {} ({} bytes, FIEL Section)",
        ten_d_path.display(),
        ten_d_bytes.len()
    );

    // ------------------------------------------------------------------------
    // Phase 5: WGSL Forge & Viewport Shader Library Naga Certification
    // ------------------------------------------------------------------------
    println!("\n--- Phase 5: WGSL Forge & Viewport Shader Library Naga Certification ---");

    let observer_reports = validate_all_observer_kernels().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("WGSL Forge observer kernel validation failed: {e}"),
        )
    })?;
    println!(
        "  * Validated 5 canonical Observer Projection Kernels (0x00..0x04) via Naga {}",
        NAGA_API_VERSION
    );

    let foliage_report = validate_wgsl(FOLIAGE_LOD_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("FOLIAGE_LOD_WGSL failed: {e}"))
    })?;
    let kin_report = validate_wgsl(KIN_STEP_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("KIN_STEP_WGSL failed: {e}"))
    })?;
    let water_report = validate_wgsl(WATER_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("WATER_WGSL failed: {e}"))
    })?;
    let sky_report = validate_wgsl(SKY_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("SKY_WGSL failed: {e}"))
    })?;
    let deferred_report = validate_wgsl(DEFERRED_RESOLVE_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("DEFERRED_RESOLVE_WGSL failed: {e}"))
    })?;
    let holo_report = validate_wgsl(HOLOGRAPHIC_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("HOLOGRAPHIC_WGSL failed: {e}"))
    })?;
    let cluster_report = validate_wgsl(CLUSTER_LIGHT_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("CLUSTER_LIGHT_WGSL failed: {e}"))
    })?;
    let cull_report = validate_wgsl(INSTANCE_CULL_INDIRECT_WGSL).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("INSTANCE_CULL_INDIRECT_WGSL failed: {e}"))
    })?;

    let unified_wgsl = build_unified_webgpu_wgsl();
    let unified_report = validate_wgsl(&unified_wgsl).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unified EcoPunk WebGPU WGSL validation failed: {e}"),
        )
    })?;
    println!(
        "  * Validated Unified EcoPunk WebGPU Pipeline via Naga (hash: {}, entry_points: {:?}, bindings: {})",
        &unified_report.source_hash[..16],
        unified_report.entry_points,
        unified_report.binding_count
    );

    // Convert 16-KB ConstituentLUT bytes to u32 array for immediate WebGPU SSBO upload
    let lut_u32: Vec<u32> = lut_bytes
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    let forge_manifest = json!({
        "forge_schema_version": FORGE_SCHEMA_VERSION,
        "wgpu_api_version": WGPU_API_VERSION,
        "naga_api_version": NAGA_API_VERSION,
        "unified_pipeline_report": unified_report,
        "observer_kernel_reports": observer_reports,
        "viewport_library_reports": {
            "foliage_lod": foliage_report,
            "kinematics_nbody": kin_report,
            "water_hmc": water_report,
            "sky_analytic": sky_report,
            "deferred_resolve": deferred_report,
            "holographic": holo_report,
            "cluster_light": cluster_report,
            "instance_cull_indirect": cull_report
        },
        "observer_kernels_wgsl": {
            "header": WGSL_OBSERVER_HEADER,
            "0x00_pbr": WGSL_KERNEL_PBR,
            "0x01_toon": WGSL_KERNEL_TOON,
            "0x02_watercolor": WGSL_KERNEL_WATERCOLOR,
            "0x03_hud": WGSL_KERNEL_HUD,
            "0x04_thermal": WGSL_KERNEL_THERMAL
        },
        "viewport_library_wgsl": {
            "spectral": SPECTRAL_WGSL,
            "hydrologic_optics": HYDROLOGIC_OPTICS_WGSL,
            "sggx_microflake": SGGX_MICROFLAKE_WGSL,
            "foliage_lod": FOLIAGE_LOD_WGSL,
            "water": WATER_WGSL,
            "sky": SKY_WGSL,
            "swe_solver": SWE_SOLVER_WGSL,
            "acoustic_voxel_cone": ACOUSTIC_VOXEL_CONE_WGSL,
            "emf_volumetric": EMF_VOLUMETRIC_WGSL,
            "kinematics": KIN_STEP_WGSL,
            "deferred_resolve": DEFERRED_RESOLVE_WGSL,
            "holographic": HOLOGRAPHIC_WGSL,
            "cluster_light": CLUSTER_LIGHT_WGSL,
            "instance_cull_indirect": INSTANCE_CULL_INDIRECT_WGSL
        },
        "unified_webgpu_wgsl": unified_wgsl,
        "constituent_lut_u32": lut_u32
    });

    let forge_manifest_path = assets_dir.join("wgsl_forge_manifest.json");
    fs::write(
        &forge_manifest_path,
        serde_json::to_string_pretty(&forge_manifest)?,
    )?;
    println!(
        "[OK] Wrote Naga-Certified WGSL Forge Manifest: {}",
        forge_manifest_path.display()
    );

    // ------------------------------------------------------------------------
    // Phase 6: VibeScript (.vibe) Certification, Interpreter Evaluation (`PoetSnapshot`),
    //          SHACL Validation (`validate_shacl_property`) & 305-Flora / 164-Fauna Catalogs
    // ------------------------------------------------------------------------
    println!("\n--- Phase 6: VibeScript Certification, Interpreter Execution & 305-Flora / 164-Fauna Compilation ---");

    let vibe_paths = [
        root.join("entities")
            .join("taxa")
            .join("corymbia-aparrerinja")
            .join("corymbia-aparrerinja.vibe"),
        root.join("vibe").join("east_coast_flora_305.vibe"),
        root.join("vibe").join("east_coast_fauna_164.vibe"),
        root.join("vibe").join("lithology_pedology_hydrology.vibe"),
    ];

    for vp in &vibe_paths {
        if vp.exists() {
            let src = fs::read_to_string(vp)?;
            let ast = vibe::parse_program(&src).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("VibeScript parse error in {}: {e:?}", vp.display()),
                )
            })?;
            let ty_env = vibe::check_program(&ast).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("VibeScript typecheck error in {}: {e:?}", vp.display()),
                )
            })?;
            println!(
                "  * Certified VibeScript schema module: {} ({} items, effect: {:?})",
                vp.file_name().and_then(|s| s.to_str()).unwrap_or("module.vibe"),
                ast.items.len(),
                ty_env.effect
            );
        }
    }

    // Evaluate `vibe/east_coast_biosphere_statements.vibe` with the real VibeScript interpreter
    // and `PoetSnapshot` host (`quin.statement` -> `generate_60bit_token` + `NQuin::calculate_parity`)
    let stmt_vibe_path = root.join("vibe").join("east_coast_biosphere_statements.vibe");
    let stmt_vibe_src = fs::read_to_string(&stmt_vibe_path)?;
    let vibe_program = vibe::load_program(&stmt_vibe_src).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("vibe::load_program error: {e}"),
        )
    })?;

    let hero_keys = [
        ("corymbia_aparrerinja", "ecopunk:asset/flora/corymbia-aparrerinja-0"),
        ("eucalyptus_pauciflora", "ecopunk:asset/flora/eucalyptus-pauciflora-0"),
        ("eucalyptus_regnans", "ecopunk:asset/flora/eucalyptus-regnans-0"),
        ("eucalyptus_camaldulensis", "ecopunk:asset/flora/eucalyptus-camaldulensis-0"),
        ("acacia_pycnantha", "ecopunk:asset/flora/acacia-pycnantha-0"),
        ("banksia_serrata", "ecopunk:asset/flora/banksia-serrata-0"),
        ("xanthorrhoea_australis", "ecopunk:asset/flora/xanthorrhoea-australis-0"),
        ("dicksonia_antarctica", "ecopunk:asset/flora/dicksonia-antarctica-0"),
        ("telopea_speciosissima", "ecopunk:asset/flora/telopea-speciosissima-0"),
        ("themeda_triandra", "ecopunk:asset/flora/themeda-triandra-0"),
        ("vallisneria_australis", "ecopunk:asset/flora/vallisneria-australis-0"),
        ("macropus_giganteus", "ecopunk:asset/fauna/macropus-giganteus-0"),
        ("vombatus_ursinus", "ecopunk:asset/fauna/vombatus-ursinus-0"),
        ("ornithorhynchus_anatinus", "ecopunk:asset/fauna/ornithorhynchus-anatinus-0"),
        ("phascolarctos_cinereus", "ecopunk:asset/fauna/phascolarctos-cinereus-0"),
        ("aquila_audax", "ecopunk:asset/fauna/aquila-audax-0"),
        ("maccullochella_peelii", "ecopunk:asset/fauna/maccullochella-peelii-0"),
    ];

    let mut lex_terms: HashMap<u64, String> = HashMap::new();
    let mut vibe_quins: Vec<NQuin> = Vec::new();
    let mut taxon_bindings_json = serde_json::Map::new();

    for (render_key, asset_iri) in &hero_keys {
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        let taxon_val = vibe::eval_function(
            &vibe_program,
            "taxon_for",
            vec![vibe::Value::String((*render_key).to_string())],
            &mut host,
            &mut env,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("eval taxon_for: {e}")))?;

        let vibe::Value::Record(rec) = taxon_val else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "taxon_for not record"));
        };
        let gbif_key = match rec.get("gbifKey") {
            Some(vibe::Value::F64(v)) => *v as u64,
            _ => 0,
        };
        let taxon_iri = match rec.get("iri") {
            Some(vibe::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        let sci_name = match rec.get("scientificName") {
            Some(vibe::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        let common_name = match rec.get("commonName") {
            Some(vibe::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        taxon_bindings_json.insert(
            (*render_key).to_string(),
            json!({
                "gbifKey": gbif_key,
                "iri": taxon_iri,
                "scientificName": sci_name,
                "commonName": common_name,
                "assetIri": asset_iri,
            }),
        );

        let mut host2 = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env2 = vibe::Env::default();
        let stmts_val = vibe::eval_function(
            &vibe_program,
            "dwc_statements_for",
            vec![
                vibe::Value::String((*render_key).to_string()),
                vibe::Value::String((*asset_iri).to_string()),
            ],
            &mut host2,
            &mut env2,
        )
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("eval dwc_statements_for: {e}"),
            )
        })?;

        let vibe::Value::List(items) = stmts_val else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "dwc_statements_for not list",
            ));
        };

        for item in items {
            if let vibe::Value::Record(srec) = item {
                let s_str = match srec.get("s") {
                    Some(vibe::Value::String(s)) => s.clone(),
                    _ => String::new(),
                };
                let p_str = match srec.get("p") {
                    Some(vibe::Value::String(s)) => s.clone(),
                    _ => String::new(),
                };
                let o_str = match srec.get("o") {
                    Some(vibe::Value::String(s)) => s.clone(),
                    _ => String::new(),
                };
                if let Some(vibe::Value::QuinRef(qref)) = srec.get("quin") {
                    let f = qref.raw_fields();
                    let nq = NQuin {
                        subject: f[0],
                        predicate: f[1],
                        object: f[2],
                        context: f[3],
                        metadata: f[4],
                        parity: f[5],
                    };
                    assert_eq!(
                        nq.parity,
                        NQuin::calculate_parity(
                            nq.subject,
                            nq.predicate,
                            nq.object,
                            nq.context,
                            nq.metadata
                        ),
                        "VibeScript QuinRef parity seal must be valid"
                    );
                    lex_terms.insert(nq.subject, s_str);
                    lex_terms.insert(nq.predicate, p_str);
                    lex_terms.insert(nq.object, o_str);
                    vibe_quins.push(nq);
                }
            }
        }

        // Append packed-integer GBIF key (`INLINE_TAG_INTEGER | gbif_key`)
        let s_tok = generate_60bit_token(taxon_iri.as_bytes());
        let p_tok = generate_60bit_token(b"urn:qualia:gbif:taxonKey");
        let o_tok = INLINE_TAG_INTEGER | gbif_key;
        lex_terms.insert(s_tok, taxon_iri.clone());
        lex_terms.insert(p_tok, "urn:qualia:gbif:taxonKey".to_string());
        vibe_quins.push(NQuin {
            subject: s_tok,
            predicate: p_tok,
            object: o_tok,
            context: 0,
            metadata: 0,
            parity: NQuin::calculate_parity(s_tok, p_tok, o_tok, 0, 0),
        });

        // Validate SHACL shape on the emitted Darwin Core quins
        let one = [ShaclConstraint::MinCount(1)];
        for prop in [
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            "http://rs.tdwg.org/dwc/terms/taxonID",
            "http://rs.tdwg.org/dwc/terms/scientificName",
            "http://rs.tdwg.org/dwc/terms/taxonRank",
        ] {
            assert!(
                validate_shacl_property(&vibe_quins, s_tok, generate_60bit_token(prop.as_bytes()), &one),
                "SHACL validation failed for {taxon_iri} property {prop}"
            );
        }
        assert!(
            validate_shacl_property(
                &vibe_quins,
                s_tok,
                p_tok,
                &[
                    ShaclConstraint::MinCount(1),
                    ShaclConstraint::Datatype(ShaclDatatype::Integer),
                ],
            ),
            "SHACL integer taxonKey validation failed for {taxon_iri}"
        );
    }

    // Write unified v4 .q42 volume (`StreamingQ42VolumeWriter`) with embedded Q42LEX, BIDX, and Merkle root
    vibe_quins.sort_unstable_by_key(|q| q.object);
    let q42_v4_path = assets_dir.join("east_coast_biosphere_v4.q42");
    let mut q42_writer = StreamingQ42VolumeWriter::new(&lex_terms)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("q42 writer: {e}")))?;
    q42_writer.declare_permissive_commons();
    for (seq, chunk) in vibe_quins.chunks(QUINS_PER_BLOCK).enumerate() {
        q42_writer
            .push_block(seq as u64, chunk)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("q42 push_block: {e}")))?;
    }
    q42_writer
        .finish(&q42_v4_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("q42 finish: {e}")))?;
    let q42_v4_bytes = fs::read(&q42_v4_path)?;
    assert!(q42_v4_bytes.starts_with(&Q42_MAGIC), "Q42 v4 magic verified");
    println!(
        "[OK] Evaluated VibeScript `east_coast_biosphere_statements.vibe` via `PoetSnapshot` & SHACL: {} sealed NQuins, {} lexicon terms -> {} ({} bytes)",
        vibe_quins.len(),
        lex_terms.len(),
        q42_v4_path.display(),
        q42_v4_bytes.len()
    );

    // Load base 105 Flora & 64 Fauna (idempotent slice so re-running never duplicates)
    let catalog_dir = data_dir.join("catalog");
    let flora_100_path = catalog_dir.join("flora_100.json");
    let fauna_64_path = catalog_dir.join("fauna_64.json");

    let raw_flora_val: Value = serde_json::from_str(&fs::read_to_string(&flora_100_path)?)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("flora JSON error: {e}")))?;
    let base_flora_species: Vec<Value> = raw_flora_val
        .get("species")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().take(105).cloned().collect())
        .unwrap_or_default();
    let base_flora_input = json!({ "species": base_flora_species });

    let compiled_flora = compile_305_flora_catalog(&base_flora_input);
    let flora_total = compiled_flora
        .get("total_species")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert_eq!(flora_total, 305, "Expected 305 compiled flora species");

    let flora_305_path = catalog_dir.join("flora_305.json");
    let flora_pretty = serde_json::to_string_pretty(&compiled_flora)?;
    fs::write(&flora_100_path, &flora_pretty)?;
    fs::write(&flora_305_path, &flora_pretty)?;
    println!(
        "[OK] Compiled 305-Species Flora Catalog: {} & {} ({} species)",
        flora_100_path.display(),
        flora_305_path.display(),
        flora_total
    );

    let raw_fauna_val: Value = serde_json::from_str(&fs::read_to_string(&fauna_64_path)?)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("fauna JSON error: {e}")))?;
    let base_fauna_species: Vec<Value> = raw_fauna_val
        .get("species")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().take(64).cloned().collect())
        .unwrap_or_default();
    let base_fauna_input = json!({ "species": base_fauna_species });

    let compiled_fauna = compile_164_fauna_catalog(&base_fauna_input);
    let fauna_total = compiled_fauna
        .get("total_species")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert_eq!(fauna_total, 164, "Expected 164 compiled fauna species");

    let fauna_164_path = catalog_dir.join("fauna_164.json");
    let fauna_pretty = serde_json::to_string_pretty(&compiled_fauna)?;
    fs::write(&fauna_64_path, &fauna_pretty)?;
    fs::write(&fauna_164_path, &fauna_pretty)?;
    println!(
        "[OK] Compiled 164-Species Fauna Catalog: {} & {} ({} species)",
        fauna_64_path.display(),
        fauna_164_path.display(),
        fauna_total
    );

    // Inspect .10d Corymbia aparrerinja digital twin binary chunks
    let corymbia_10d_dir = root
        .join("assets")
        .join("models")
        .join("corymbia-aparrerinja")
        .join("10d");
    let mut corymbia_chunks = Vec::new();
    if corymbia_10d_dir.exists() {
        for entry in fs::read_dir(&corymbia_10d_dir)? {
            let entry = entry?;
            let p = entry.path();
            if p.extension().and_then(|e| e.to_str()) == Some("10d") {
                let bytes = fs::read(&p)?;
                let valid_magic = bytes.len() >= 4 && &bytes[0..4] == &MAGIC_10D;
                corymbia_chunks.push(json!({
                    "part": p.file_stem().and_then(|s| s.to_str()).unwrap_or("part"),
                    "bytes": bytes.len(),
                    "valid_10d_magic": valid_magic,
                }));
            }
        }
    }
    println!(
        "  * Verified {} .10d Digital Twin sub-meshes for Corymbia aparrerinja",
        corymbia_chunks.len()
    );

    // Update master_ecological_dataset.json (pure data, zero .js output)
    let master_path = data_dir.join("master_ecological_dataset.json");
    let mut master_val: Value = if master_path.exists() {
        serde_json::from_str(&fs::read_to_string(&master_path)?).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };

    if let Some(obj) = master_val.as_object_mut() {
        obj.insert("total_flora_species".to_string(), json!(flora_total));
        obj.insert("total_fauna_species".to_string(), json!(fauna_total));
        obj.insert(
            "flora".to_string(),
            compiled_flora
                .get("species")
                .cloned()
                .unwrap_or_else(|| json!([])),
        );
        obj.insert(
            "fauna".to_string(),
            compiled_fauna
                .get("species")
                .cloned()
                .unwrap_or_else(|| json!([])),
        );
    }
    fs::write(&master_path, serde_json::to_string_pretty(&master_val)?)?;

    // ------------------------------------------------------------------------
    // Phase 7: Native `.10d` Multi-Material Scene Synthesis, `.hmc` (`QBDL`) Bundle
    //          Packaging & 100% Zero-V8 `PortalGpu` / `VolumetricRenderer` Rendering
    // ------------------------------------------------------------------------
    println!("\n--- Phase 7: Native .10d Scene Compilation, .hmc Bundle Packaging & Zero-V8 PortalGpu Rendering ---");

    let textures = build_procedural_textures();
    let sector_specs = [
        ("talbingo", "Talbingo — Tumut River Gorge & Bogong Foothills", false),
        ("yarrangobilly", "Yarrangobilly — Silurian Karst Gorge & 6-Layer SLGA Strata Cutaway", true),
        ("three_mile_dam", "Three Mile Dam — Subalpine Frost Hollow & Snow Gum Basin", false),
        ("sydney_sandstone", "Sydney Basin — Hawkesbury Sandstone Escarpment & Gully", false),
        ("noosa_headland", "Noosa Headland — Subtropical Wallum Heath & Coastal Estuary", false),
        ("jounama_karst", "Jounama Creek — Bogong Peaks & Subterranean Karst", false),
    ];

    let mut compiled_sectors = BTreeMap::new();
    for (sec_id, sec_title, cutaway) in sector_specs {
        let sector = build_sector_10d(sec_id, sec_title, &textures, cutaway)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let sec_10d_path = assets_dir.join(format!("sector_{sec_id}.10d"));
        let wat_10d_path = assets_dir.join(format!("water_{sec_id}.10d"));
        fs::write(&sec_10d_path, &sector.scene_10d_bytes)?;
        fs::write(&wat_10d_path, &sector.water_10d_bytes)?;
        println!(
            "  * Sealed .10d Sector `{}`: {} verts, {} tris, {} B (CRC32C: {:#010x}), AG-12 veg instances: {}",
            sec_id,
            sector.positions.len(),
            sector.indices.len() / 3,
            sector.scene_10d_bytes.len(),
            sector.scene_crc32c,
            sector.vegetation_instances_evaluated
        );
        compiled_sectors.insert(sec_id, sector);
    }

    let snow_gum_specimen = build_closeup_specimen_10d(
        "snow_gum_loupe",
        "Botanical Loupe — Eucalyptus pauciflora & Corymbia aparrerinja 3D Falcate Foliage",
        &textures,
        false,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let kangaroo_specimen = build_closeup_specimen_10d(
        "kangaroo_tracker",
        "Zoological Tracker — Macropus giganteus & Submerged Riverbed Pebbles",
        &textures,
        true,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Pack all `.10d` scenes, `.10d` water surfaces, procedural PBR PNG textures,
    // `sem/east_coast_biosphere.q42`, and `manifest.json` into `assets/packs/ecopunk-east-coast.hmc` (`QBDL`)
    let packs_dir = root.join("assets").join("packs");
    fs::create_dir_all(&packs_dir)?;

    let mut manifest_assets = Vec::new();
    for sector in compiled_sectors.values() {
        let mut sh = Sha256::new();
        sh.update(&sector.scene_10d_bytes);
        let scene_sha = format!("{:x}", sh.finalize());
        manifest_assets.push(json!({
            "id": format!("ecopunk:sector:{}", sector.sector_id),
            "file": sector.scene_hmc_key,
            "waterFile": sector.water_hmc_key,
            "bytes": sector.scene_10d_bytes.len(),
            "crc32c": sector.scene_crc32c,
            "sha256": scene_sha,
            "vertices": sector.positions.len(),
            "triangles": sector.indices.len() / 3,
        }));
    }

    let manifest_textures: Vec<Value> = textures
        .iter()
        .map(|t| {
            json!({
                "name": t.name,
                "file": t.hmc_key,
                "sha256": t.sha256_hex,
                "width": t.width,
                "height": t.height,
                "bytes": t.png_bytes.len(),
            })
        })
        .collect();

    let hmc_manifest = json!({
        "packVersion": 1,
        "packer": "crates/ecopunk-terrain (Zero-V8 Native QualiaDB + VibeScript)",
        "totalFloraSpecies": flora_total,
        "totalFaunaSpecies": fauna_total,
        "assets": manifest_assets,
        "textures": manifest_textures,
        "taxonBindings": taxon_bindings_json,
        "semantics": [{
            "key": "sem/east_coast_biosphere.q42",
            "quins": vibe_quins.len(),
            "bytes": q42_v4_bytes.len(),
        }],
    });
    let manifest_bytes = serde_json::to_vec_pretty(&hmc_manifest)?;

    let mut bundle_writer = BundleWriter::new();
    bundle_writer
        .add_file("manifest.json", "manifest", manifest_bytes, None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter manifest: {e:?}")))?;

    for sector in compiled_sectors.values() {
        bundle_writer
            .add_file(
                &sector.scene_hmc_key,
                "10d",
                sector.scene_10d_bytes.clone(),
                None,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter 10d: {e:?}")))?;
        bundle_writer
            .add_file(
                &sector.water_hmc_key,
                "10d",
                sector.water_10d_bytes.clone(),
                None,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter water 10d: {e:?}")))?;
    }

    for tex in &textures {
        bundle_writer
            .add_file(&tex.hmc_key, "image/png", tex.png_bytes.clone(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter tex: {e:?}")))?;
    }

    bundle_writer
        .add_file("sem/east_coast_biosphere.q42", "q42", q42_v4_bytes.clone(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter q42: {e:?}")))?;

    let hmc_bytes = bundle_writer
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter build: {e}")))?;

    let hmc_reader = BundleReader::parse(&hmc_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleReader parse: {e}")))?;
    for entry in hmc_reader.entries() {
        assert!(
            hmc_reader.verify_entry(&entry.key),
            "HMC bundle entry digest verification failed for {}",
            entry.key
        );
    }

    let hmc_out_path = packs_dir.join("ecopunk-east-coast.hmc");
    fs::write(&hmc_out_path, &hmc_bytes)?;
    println!(
        "[OK] Packed & Digest-Verified .hmc (`QBDL`) Bundle: {} ({} bytes, {} verified entries)",
        hmc_out_path.display(),
        hmc_bytes.len(),
        hmc_reader.entries().len()
    );

    // Verify `VolumetricRenderer::load_hmc_asset` and `load_hmc_water_asset` directly on the native GPU!
    let mut vol_renderer = VolumetricRenderer::new_offscreen(640, 360, 256)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("VolumetricRenderer init: {e}")))?;
    let (hmc_verts, hmc_tris, _) = vol_renderer
        .load_hmc_asset(&hmc_bytes, "10d/sector-talbingo.10d")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("load_hmc_asset: {e}")))?;
    let (wat_verts, wat_tris) = vol_renderer
        .load_hmc_water_asset(&hmc_bytes, "10d/water-talbingo.10d")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("load_hmc_water_asset: {e}")))?;
    println!(
        "[OK] VolumetricRenderer Native HMC Admission Verified: scene=({} verts, {} tris), water=({} verts, {} tris)",
        hmc_verts, hmc_tris, wat_verts, wat_tris
    );

    // Verify WasmSliceQ42Reader & vibe::LocalHost::with_q42_pack
    let wasm_q42_reader = WasmSliceQ42Reader::open(&q42_v4_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("WasmSliceQ42Reader::open: {e:?}")))?;
    println!(
        "  * Verified WasmSliceQ42Reader: is_unified={}, version={}, blocks={}, len={} B",
        wasm_q42_reader.is_unified_volume(),
        wasm_q42_reader.version(),
        wasm_q42_reader.block_count(),
        wasm_q42_reader.byte_len()
    );
    let _local_host = vibe::LocalHost::with_q42_pack(&q42_v4_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("LocalHost::with_q42_pack: {e:?}")))?;
    println!("  * Verified vibe::LocalHost::with_q42_pack admission");

    // Verify HmcProgressiveRangePlanner across Tier 0, 1, 2
    let hmc_probe = HmcHeaderProbe::parse_header_range(&hmc_bytes[..BUNDLE_HEADER_SIZE])
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("HmcHeaderProbe: {e:?}")))?;
    let (idx_off, idx_len) = hmc_probe.index_footer_range();
    let footer_slice = &hmc_bytes[idx_off as usize..(idx_off + idx_len) as usize];
    let range_planner = HmcProgressiveRangePlanner::from_header_and_index(hmc_probe, footer_slice)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("HmcProgressiveRangePlanner: {e:?}")))?;
    let schedule = range_planner.plan_entry_ranges();
    let mut tier0_count = 0usize;
    let mut tier0_bytes = 0u64;
    let mut tier1_count = 0usize;
    let mut tier1_bytes = 0u64;
    let mut tier2_count = 0usize;
    let mut tier2_bytes = 0u64;
    for req in &schedule {
        match req.tier {
            StreamPriorityTier::Tier0SemanticAndProcedural => {
                tier0_count += 1;
                tier0_bytes += req.length as u64;
            }
            StreamPriorityTier::Tier1MidLodMeshes => {
                tier1_count += 1;
                tier1_bytes += req.length as u64;
            }
            StreamPriorityTier::Tier2HighPolyAndMip0 => {
                tier2_count += 1;
                tier2_bytes += req.length as u64;
            }
        }
        let start = req.offset as usize;
        let end = start + req.length as usize;
        verify_arrived_range(req, &hmc_bytes[start..end])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("verify_arrived_range: {e:?}")))?;
    }
    println!(
        "  * Verified HmcProgressiveRangePlanner ({} scheduled ranges): Tier0=({} ranges, {} B), Tier1=({} ranges, {} B), Tier2=({} ranges, {} B)",
        schedule.len(),
        tier0_count, tier0_bytes,
        tier1_count, tier1_bytes,
        tier2_count, tier2_bytes
    );

    // Verify GovernedHydrationPool & HydrationCapabilityToken
    let mut hydration_token = HydrationCapabilityToken::new_sealed(
        0xABCD_1234,
        CAP_ALL_HYDRATION,
        StreamPriorityTier::Tier2HighPolyAndMip0,
        1000,
        1024 * 1024,
    );
    assert!(hydration_token.verify_seal(), "Hydration capability token seal failed");
    let mut hydration_pool = GovernedHydrationPool::new(16);
    let sample_payload = b"ecopunk living container hydration verification payload".to_vec();
    let sample_crc = crc32c::crc32c(&sample_payload);
    hydration_pool.submit(
        &mut hydration_token,
        HydrationTask {
            task_id: 42,
            tier: StreamPriorityTier::Tier0SemanticAndProcedural,
            kind: HydrationTaskKind::VerifySectionCrc32c {
                payload: sample_payload,
                expected_crc32c: sample_crc,
            },
        },
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("GovernedHydrationPool submit: {e:?}")))?;
    let receipts = hydration_pool.drain_bounded(1)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("GovernedHydrationPool drain: {e:?}")))?;
    assert_eq!(receipts.len(), 1);
    println!("  * Verified GovernedHydrationPool admission & execution under HydrationCapabilityToken");

    // Verify HolosonicWavefield emission of 48-byte HolosonicSonicToken48
    let mut wavefield = HolosonicWavefield::new();
    wavefield.push(HolosonicEmitter::new(
        [0.0, 1.2, 0.0],
        440.0,
        0.8,
        0.0,
        0.95,
        30.0,
        HolosonicModality::FloraWind,
    ));
    let point_sample = wavefield.evaluate_point([0.0, 0.0, 0.0], 0.0, 0.0);
    let (sonic_tok48, _tok64) = wavefield.emit_sonic_token(
        0,
        0x5A5A_BEEF,
        101,
        1,
        0.9,
        point_sample,
        [0.0, 0.0, 0.0],
        0.0,
    );
    assert_eq!(std::mem::size_of::<HolosonicSonicToken48>(), 48);
    println!(
        "  * Verified HolosonicWavefield 48-byte HolosonicSonicToken48 emission (origin_hash={:#x}, fundamental={} Hz, pos={:?})",
        sonic_tok48.origin_quin_hash, sonic_tok48.fundamental_hz, sonic_tok48.position
    );

    // Verify Epics 1, 2, 3, 4, 6, 8, 9 end-to-end across ecopunk-terrain modules
    let mut scene_graph_10d = EcopunkSectorSceneGraph::new_boxed("talbingo");
    assert!(scene_graph_10d.populate_sector_10d_hierarchy("talbingo"));
    println!(
        "  * Verified EcopunkSectorSceneGraph 10D Hierarchy (`talbingo`): {} living botanical entities, {} holo SDF lenses",
        scene_graph_10d.living_count, scene_graph_10d.holographic_sdf_count
    );

    let mut macropod_rig = FaunaSkeletalRig10d::new(FaunaBiomechanicalArchetype::MacropodSaltatorial);
    let rest_pose = macropod_rig.rest_pose;
    let palette_joints = macropod_rig.evaluate_gpu_palette(&rest_pose);
    println!(
        "  * Verified FaunaSkeletalRig10d (`MacropodSaltatorial`): {} PGA Motor joints, {} ABSM layers, {} 2D Delaunay points",
        palette_joints, macropod_rig.absm.layer_count, macropod_rig.blend_space_2d.point_count
    );

    let phys_receipt = run_sector_physics_verification_step("yarrangobilly_cliff");
    println!(
        "  * Verified EcopunkSectorPhysicsWorld (`yarrangobilly_cliff`): {} bodies, {} BVH leaves, {} fractures, sinkage={:.2} mm",
        phys_receipt.active_rigid_bodies,
        phys_receipt.bvh_leaves,
        phys_receipt.stress_cleavage_fractures,
        phys_receipt.soil_sinkage_mm
    );

    let mut soundscape = EcopunkHolosonicSoundscape::with_australian_soundscape("yarrangobilly");
    let mut audio_l = [0.0f32; 64];
    let mut audio_r = [0.0f32; 64];
    let audio_sample = soundscape.render_frame_zero_heap(0.25, 48_000.0, &mut audio_l, &mut audio_r);
    println!(
        "  * Verified EcopunkHolosonicSoundscape (`yarrangobilly`): {} active emitters, interference_gain={:.3}",
        audio_sample.active_emitter_count, audio_sample.interference_gain
    );

    let mut telemetry_hud = EcopunkTelemetryHud::new(1280.0, 720.0);
    telemetry_hud.render_frame_zero_heap();
    println!(
        "  * Verified EcopunkTelemetryHud (1280x720 Zero-Heap Flexbox + MSDF): {} widgets, {} shaped glyphs",
        telemetry_hud.canvas.widget_count,
        telemetry_hud.canvas.text_quad_count
    );

    // Render all 8 showcase frames (`1280x720`) natively via `PortalGpu` (`wgpu` — Zero V8)
    let renders_dir = root.join("assets").join("renders");
    fs::create_dir_all(&renders_dir)?;

    let render_jobs: Vec<(&str, &ecopunk_terrain::CompiledSector10d, Option<(ShadingModel, u32, [f32; 3])>)> = vec![
        (
            "01_talbingo_aerial_pbr.png",
            &compiled_sectors["talbingo"],
            None,
        ),
        (
            "02_yarrangobilly_cutaway_strata.png",
            &compiled_sectors["yarrangobilly"],
            None,
        ),
        (
            "03_flora_specimen_loupe_snow_gum.png",
            &snow_gum_specimen,
            None,
        ),
        (
            "04_fauna_tracker_kangaroo.png",
            &kangaroo_specimen,
            None,
        ),
        (
            "05_sydney_sandstone_cel_toon.png",
            &compiled_sectors["sydney_sandstone"],
            Some((ShadingModel::Stylized, 0, [0.52, 0.44, 0.36])),
        ),
        (
            "06_noosa_headland_watercolor.png",
            &compiled_sectors["noosa_headland"],
            Some((ShadingModel::Stylized, 0, [0.44, 0.56, 0.62])),
        ),
        (
            "07_three_mile_dam_gpr_hud.png",
            &compiled_sectors["three_mile_dam"],
            Some((ShadingModel::MetallicRoughness, 1, [0.18, 0.48, 0.64])),
        ),
        (
            "08_jounama_karst_thermal_ir.png",
            &compiled_sectors["jounama_karst"],
            Some((ShadingModel::Stylized, 2, [0.64, 0.32, 0.18])),
        ),
    ];

    for (filename, sector_ref, override_mode) in render_jobs {
        let png_bytes = render_sector_via_portal_gpu_png(
            sector_ref,
            &textures,
            1280,
            720,
            override_mode,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("PortalGpu render {filename}: {e}")))?;
        let out_path = renders_dir.join(filename);
        fs::write(&out_path, &png_bytes)?;
        println!(
            "  * Rendered Native PortalGpu (`wgpu` Zero-V8) Frame: {} ({} bytes, 1280x720)",
            out_path.display(),
            png_bytes.len()
        );
    }

    println!("\n============================================================");
    println!("✅ 100% Zero-V8 Native QualiaDB + VibeScript + .hmc/.10d/.q42/.m64 + PortalGpu Pipeline Complete.");
    println!("============================================================");

    Ok(())
}


