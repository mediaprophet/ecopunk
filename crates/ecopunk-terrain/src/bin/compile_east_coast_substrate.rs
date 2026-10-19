//! compile_east_coast_substrate
//!
//! Executes Phase 2, Phase 3, Phase 4, and Phase 5 of:
//! `docs/plans/east-coast-australia-ecological-dataset-acquisition-plan.md`
//!
//! Synthesizes native QualiaDB artifacts:
//! 1. `.q42` Semantic Volumes: Compiled 48-byte `NQuin` records from ALL `.ttl` ontologies in `data/ontologies/`.
//! 2. 16-KB GPU `ConstituentLUT`: Precompiled 256 x 64B flat binary material table (`BankedConstituentLut`).
//! 3. `.m64` Matter-64 Container: Mode 0x02 strata column chunk for the Talbingo micro-scene.
//! 4. `.10d` Manifold Terrain Container: Header + Field Section (FIEL).
//! 5. `wgsl_forge` & `shaders::viewport` Certification & WebGPU Bundle (`wgsl_forge_manifest.json` + `wgsl_shader_bundle.js`).

use std::fs::{self, File};
use std::io::{self, Cursor, Read};
use std::path::PathBuf;

use qualia_core_db::container_10d::{HEADER_BYTE_SIZE, HEADER_VERSION, MAGIC_10D};
use qualia_core_db::q42::m64_matter::banked_lut::{BankedConstituentLut, LUT_BUFFER_BYTES};
use qualia_core_db::q42::m64_matter::{M64ChunkDirectoryEntry, M64Header, M64_MAGIC};
use qualia_core_db::q42::q42_volume::write_sorted_quins_volume;
use qualia_core_db::shaders::viewport::{
    ACOUSTIC_VOXEL_CONE_WGSL, EMF_VOLUMETRIC_WGSL, FOLIAGE_LOD_WGSL, HYDROLOGIC_OPTICS_WGSL,
    SGGX_MICROFLAKE_WGSL, SKY_WGSL, SPECTRAL_WGSL, SWE_SOLVER_WGSL, WATER_WGSL,
};
use qualia_core_db::wgsl_forge::physics::kinematics::KIN_STEP_WGSL;
use qualia_core_db::wgsl_forge::{
    validate_all_observer_kernels, validate_wgsl, FORGE_SCHEMA_VERSION, NAGA_API_VERSION,
    WGPU_API_VERSION, WGSL_KERNEL_HUD, WGSL_KERNEL_PBR, WGSL_KERNEL_THERMAL, WGSL_KERNEL_TOON,
    WGSL_KERNEL_WATERCOLOR, WGSL_OBSERVER_HEADER,
};
use qualia_core_db::{q_hash, NQuin};
use qualia_geospatial::{
    compile_raster_to_m64, ElevationRaster, GeodeticFrame, CHUNK_CELL_COUNT,
};
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleParser;
use serde_json::json;

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

    // 1. Flora GPU Aeroelastic Bishop-Frame Wind Sway & Leaf Flutter
    if (entity_kind == 1) {{
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
        let w1 = sin(pos.x * 0.45 + u_time * 2.2) * cos(pos.z * 0.35 + u_time * 1.7) * 0.14;
        let w2 = sin((pos.x * 0.8 - pos.z * 0.6) + u_time * 3.4) * 0.06;
        pos.y += w1 + w2;
        norm = normalize(vec3<f32>(-0.22 * cos(pos.x * 0.45 + u_time * 2.2), 1.0, -0.18 * sin(pos.z * 0.35 + u_time * 1.7)));
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
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {{
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
        return vec4<f32>(aces_film(sky_col), 1.0);
    }}

    var base_col = input.color;

    // Terrain & 6-Layer SLGA Subsurface Strata (with 16-KB ConstituentLUT lookup)
    if (entity_kind == 0) {{
        let is_wall = input.v_meta.z;
        let slope = 1.0 - max(n.y, 0.0);
        if (is_wall < 0.5) {{
            if (slope > 0.35) {{
                let flute = sin(input.world_pos.x * 3.8 + input.world_pos.z * 3.8) * 0.5 + 0.5;
                let fracture = hash21(floor(input.world_pos.xz * 2.5));
                base_col *= (0.82 + 0.24 * mix(fracture, flute, karst_fluting));
            }}
            let dist_above_water = input.world_pos.y - water_level;
            if (dist_above_water >= -0.5 && dist_above_water < 1.35) {{
                let wet_factor = 1.0 - clamp(dist_above_water / 1.35, 0.0, 1.0);
                base_col *= (1.0 - 0.38 * wet_factor);
            }}
        }} else {{
            let strata_grain = sin(input.world_pos.y * 14.0 + input.world_pos.x * 0.8) * 0.06;
            base_col += vec3<f32>(strata_grain);
        }}
    }}

    // Hydrologic Optics Water Shader (shaders::viewport::hydrologic_optics.wgsl + water.wgsl)
    if (entity_kind == 3) {{
        let half_vec = normalize(light_dir + view_dir);
        let spec = pow(max(dot(n, half_vec), 0.0), 96.0) * 1.35;
        let fresnel = fresnel_schlick(n_dot_v, vec3<f32>(0.02));
        let hg_backscatter = henyey_greenstein_phase(dot(-view_dir, light_dir), 0.82) * 0.18;
        let deep_water = base_col * (0.55 + hg_backscatter);
        let sky_refl = vec3<f32>(0.42, 0.65, 0.85) * max(0.25, light_dir.y);
        var water_col = mix(deep_water * (0.45 + 0.55 * n_dot_l), sky_refl, fresnel * 0.65) + vec3<f32>(1.0, 0.96, 0.85) * spec;
        if (observer_mode == 4) {{
            water_col = vec3<f32>(0.05, 0.22, 0.68);
        }}
        return vec4<f32>(aces_film(water_col), 1.0);
    }}

    var final_col = base_col;

    // 0x00: WGSL_KERNEL_PBR + SGGX Anisotropic Microflake Sheen + Near-Field Clarity Fog
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
            sss = base_col * vec3<f32>(0.35, 0.55, 0.18) * pow(back_light, 2.0) * 0.45;
        }}

        let sun_radiance = vec3<f32>(1.55, 1.46, 1.32) * max(0.15, light_dir.y);
        let lit = base_col * (sky_amb + sun_radiance * n_dot_l * 0.78) + sun_radiance * (spec + vec3<f32>(sggx_sheen)) + sss;

        let dist = length(input.world_pos - cam_pos);
        let d_eff = max(0.0, dist - 38.0);
        let height_factor = exp(-max(0.0, input.world_pos.y - water_level) * 0.08);
        let fog_factor = clamp(1.0 - exp(-d_eff * 0.0035 * (0.4 + 0.6 * height_factor)), 0.0, 0.65);
        let fog_col = vec3<f32>(0.58, 0.72, 0.84) * max(0.25, light_dir.y);
        final_col = aces_film(mix(lit, fog_col, fog_factor));
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

    return vec4<f32>(final_col, 1.0);
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
            "sky_analytic": sky_report
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
            "kinematics": KIN_STEP_WGSL
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

    let viewer_data_dir = root.join("viewer").join("data");
    fs::create_dir_all(&viewer_data_dir)?;
    let bundle_js_path = viewer_data_dir.join("wgsl_shader_bundle.js");
    let bundle_js = format!(
        "// Auto-generated by compile_east_coast_substrate (qualia_core_db::wgsl_forge + shaders::viewport)\nwindow.ECOPUNK_WGSL_FORGE = {};\n",
        serde_json::to_string(&forge_manifest)?
    );
    fs::write(&bundle_js_path, &bundle_js)?;
    println!(
        "[OK] Wrote Browser WebGPU WGSL Bundle: {} ({} bytes)",
        bundle_js_path.display(),
        bundle_js.len()
    );

    println!("\n============================================================");
    println!("✅ All Qualia Substrates & WGSL Forge Shaders Successfully Compiled.");
    println!("============================================================");

    Ok(())
}
