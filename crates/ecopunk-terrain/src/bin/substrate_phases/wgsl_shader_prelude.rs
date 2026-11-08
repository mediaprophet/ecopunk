//! WGSL Unified Pipeline Prelude & Vertex Shader Definitions
//!
//! Provides scene uniforms, entity uniforms, PBR/SGGX microfacet BRDFs,
//! PGA motor skinning, CSM shadow cascade evaluation, and the `vs_main` vertex entry.

pub const WGSL_PRELUDE: &str = r#"// ============================================================================
// EcoPunk Unified WebGPU Pipeline — Forged & Certified via qualia_core_db::wgsl_forge
// Includes: SPECTRAL_WGSL (CIE 1931 XYZ) + Observer Kernels (0x00..0x05)
//           + SGGX Anisotropic Microflake BRDF + Hydrologic Optics + 16KB ConstituentLUT
// ============================================================================

struct SceneUniforms {
    proj: mat4x4<f32>,
    view: mat4x4<f32>,
    camera_pos: vec4<f32>,      // xyz: eye, w: time_seconds
    sun_dir: vec4<f32>,         // xyz: sunDir, w: water_level
    wind_and_clip: vec4<f32>,   // xy: wind_vec, z: clip_plane_z, w: karst_fluting
    isogloss: vec4<f32>,        // x: centerX, y: radius, z: centerZ, w: active
    space_and_mode: vec4<f32>,  // x: kp_index, y: imf_bz, z: f32(observer_mode), w: exposure
};

struct EntityUniforms {
    model: mat4x4<f32>,
    fauna_anim: vec4<f32>,      // x: phase, y: speed, z: gait_code, w: entity_kind
    line_color: vec4<f32>,      // rgba for line rendering
};

@group(0) @binding(0) var<uniform> scene: SceneUniforms;
@group(0) @binding(1) var<storage, read> constituent_lut: array<vec4<u32>, 1024>;
@group(1) @binding(0) var<uniform> entity: EntityUniforms;

const PI_VAL: f32 = 3.14159265359;

fn distribution_ggx(n: vec3<f32>, h: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let n_dot_h = max(dot(n, h), 0.0);
    let n_dot_h2 = n_dot_h * n_dot_h;
    let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
    return a2 / (PI_VAL * denom * denom + 0.00001);
}

fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    return n_dot_v / (n_dot_v * (1.0 - k) + k + 0.00001);
}

fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (vec3<f32>(1.0) - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

fn evaluate_sggx_sheen(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, sigma_v: f32, sigma_t: f32) -> f32 {
    let h = normalize(v + l);
    let t_fiber = vec3<f32>(0.0, 1.0, 0.0);
    let sv2 = sigma_v * sigma_v;
    let st2 = sigma_t * sigma_t;
    let h_dot_t = dot(h, t_fiber);
    let aniso = sv2 * h_dot_t * h_dot_t + st2 * (1.0 - h_dot_t * h_dot_t);
    return clamp(0.00012 / max(PI_VAL * sv2 * st2 * aniso * aniso, 0.001), 0.0, 0.12) * max(dot(n, l), 0.0);
}

fn henyey_greenstein_phase(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    let denom = 1.0 + g2 - 2.0 * g * cos_theta;
    return (1.0 - g2) / (4.0 * PI_VAL * pow(max(denom, 0.0001), 1.5));
}

fn ironbow_thermal(t_norm: f32) -> vec3<f32> {
    let t = clamp(t_norm, 0.0, 1.0);
    let c_cold = vec3<f32>(0.04, 0.08, 0.52);
    let c_mid  = vec3<f32>(0.62, 0.05, 0.78);
    let c_warm = vec3<f32>(0.98, 0.48, 0.04);
    let c_hot  = vec3<f32>(1.0, 0.98, 0.86);
    if (t < 0.33) {
        return mix(c_cold, c_mid, t * 3.0);
    } else if (t < 0.66) {
        return mix(c_mid, c_warm, (t - 0.33) * 3.0);
    }
    return mix(c_warm, c_hot, (t - 0.66) * 3.0);
}

fn aces_film(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

fn hash21(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(234.34, 435.345));
    q += dot(q, q + 34.23);
    return fract(q.x * q.y);
}

fn sample_lut_dielectric(slot: u32) -> f32 {
    let idx = clamp(slot, 0u, 255u) * 4u + 2u;
    let word = constituent_lut[idx].x;
    let eps_x100 = f32(word & 0xFFFFu);
    return clamp(eps_x100 / 100.0, 1.0, 80.0);
}

struct PgaSkinMotor {
    r: vec4<f32>,
    d: vec4<f32>,
};

fn pga_blade4_to_quat(b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(b.x, b.y, b.z, b.w);
}

fn pga_quat_mul(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        a.x * b.x - a.y * b.y - a.z * b.z - a.w * b.w,
        a.x * b.y + a.y * b.x + a.z * b.w - a.w * b.z,
        a.x * b.z - a.y * b.w + a.z * b.x + a.w * b.y,
        a.x * b.w + a.y * b.z - a.z * b.y + a.w * b.x
    );
}

fn pga_motor_blend_4(
    m0: PgaSkinMotor,
    m1: PgaSkinMotor,
    m2: PgaSkinMotor,
    m3: PgaSkinMotor,
    weights: vec4<f32>
) -> PgaSkinMotor {
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
}

fn pga_motor_transform_point(m: PgaSkinMotor, p: vec3<f32>) -> vec3<f32> {
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
}

fn pga_motor_transform_vector(m: PgaSkinMotor, v: vec3<f32>) -> vec3<f32> {
    let q_xyz = m.r.yzw;
    let t_rot = 2.0 * cross(q_xyz, v);
    return v + m.r.x * t_rot + cross(q_xyz, t_rot);
}

fn pga_eval_proc_motor_oscillation(
    pos: vec3<f32>,
    time: f32,
    freq: f32,
    damping: f32,
    drive: vec3<f32>
) -> vec3<f32> {
    let omega = 6.2831853 * freq;
    let decay = exp(-damping * max(time, 0.0));
    let phase = sin(omega * time) * decay;
    return pos + drive * phase;
}

fn eval_csm_4_cascade_blend(view_depth: f32, splits: vec4<f32>, blend_band: f32) -> vec3<f32> {
    let d = max(view_depth, 0.0);
    let band = clamp(blend_band, 0.02, 0.40);
    let s = array<f32, 4>(splits.x, splits.y, splits.z, splits.w);
    var prev_s: f32 = 0.05;
    for (var i: u32 = 0u; i < 4u; i = i + 1u) {
        let cur_s = s[i];
        if (d <= cur_s || i == 3u) {
            if (i < 3u) {
                let span = max(cur_s - prev_s, 1e-3);
                let band_start = cur_s - span * band;
                if (d > band_start) {
                    let alpha = smoothstep(band_start, cur_s, d);
                    return vec3<f32>(f32(i), f32(i + 1u), alpha);
                }
            }
            return vec3<f32>(f32(i), f32(i), 0.0);
        }
        prev_s = cur_s;
    }
    return vec3<f32>(3.0, 3.0, 0.0);
}

fn apply_height_stratified_valley_fog_35m(
    radiance: vec3<f32>,
    camera_pos: vec3<f32>,
    world_pos: vec3<f32>,
    inscatter_color: vec3<f32>
) -> vec3<f32> {
    let ray = world_pos - camera_pos;
    let dist = length(ray);
    let d_eff = max(0.0, dist - 35.0);
    if (d_eff <= 0.0) {
        return radiance;
    }
    let scale_height = 42.0;
    let base_sigma_t = 0.0045;
    let mid_height = 0.5 * (camera_pos.y + world_pos.y);
    let height_density = exp(-max(mid_height, -20.0) / scale_height);
    let optical_depth = clamp(base_sigma_t * height_density * d_eff, 0.0, 12.0);
    let transmittance = exp(-optical_depth);
    return mix(inscatter_color, radiance, transmittance);
}

fn hash33_voronoi(p: vec3<f32>) -> vec3<f32> {
    var q = vec3<f32>(
        dot(p, vec3<f32>(127.1, 311.7, 74.7)),
        dot(p, vec3<f32>(269.5, 183.3, 246.1)),
        dot(p, vec3<f32>(113.5, 271.9, 124.6))
    );
    return fract(sin(q) * 43758.5453123);
}

fn proc_voronoi_f1_3d(p: vec3<f32>, cell_density: f32, jitter: f32) -> f32 {
    let sp = p * max(cell_density, 0.001);
    let ip = floor(sp);
    let fp = fract(sp);
    let clamped_jitter = clamp(jitter, 0.0, 1.0);
    var min_dist_sq = 100.0;

    for (var dz = -1; dz <= 1; dz = dz + 1) {
        for (var dy = -1; dy <= 1; dy = dy + 1) {
            for (var dx = -1; dx <= 1; dx = dx + 1) {
                let neighbor = vec3<f32>(f32(dx), f32(dy), f32(dz));
                let rnd = hash33_voronoi(ip + neighbor);
                let offset = mix(vec3<f32>(0.5), rnd, clamped_jitter);
                let diff = neighbor + offset - fp;
                let d_sq = dot(diff, diff);
                min_dist_sq = min(min_dist_sq, d_sq);
            }
        }
    }
    return clamp(sqrt(min_dist_sq), 0.0, 1.0);
}

struct HoloFragmentOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) v_meta: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) v_meta: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var pos = input.position;
    var norm = input.normal;
    let entity_kind = i32(round(entity.fauna_anim.w));
    let u_time = scene.camera_pos.w;
    let wind_vec = scene.wind_and_clip.xy;

    if (entity_kind == 1) {
        pos = pga_eval_proc_motor_oscillation(pos, u_time, 1.65, 0.05, vec3<f32>(wind_vec.x * 0.04, 0.008, wind_vec.y * 0.04));
        let h_norm = input.v_meta.x;
        let branch_flex = input.v_meta.y;
        let flutter = input.v_meta.z;
        let wind_mag = length(wind_vec);
        let sway_phase = u_time * 1.65 + entity.model[3][0] * 0.35 + entity.model[3][2] * 0.25;
        let sway_amp = (0.045 * wind_mag) * pow(max(h_norm, 0.0), 1.6) * branch_flex;
        pos.x += wind_vec.x * 0.12 * sway_amp * (0.65 + 0.35 * sin(sway_phase));
        pos.z += wind_vec.y * 0.12 * sway_amp * (0.65 + 0.35 * cos(sway_phase * 0.9));

        if (flutter > 0.2) {
            let fl_phase = u_time * (4.5 + wind_mag * 0.35) + pos.x * 3.1 + pos.y * 2.7;
            let fl_amp = 0.025 * min(wind_mag, 18.0) * flutter;
            pos.x += sin(fl_phase) * fl_amp;
            pos.y += cos(fl_phase * 1.3) * fl_amp * 0.6;
            pos.z += cos(fl_phase * 0.8) * fl_amp;
        }
    } else if (entity_kind == 2) {
        let gait_phase = entity.fauna_anim.x;
        let gait_speed = entity.fauna_anim.y;
        let hop_y = abs(sin(gait_phase * 3.14159)) * 0.28 * gait_speed;
        let spine_flex = sin(gait_phase * 6.28318) * 0.08 * gait_speed;
        pos.y += hop_y * max(input.v_meta.x, 0.2);
        pos.z += spine_flex * input.v_meta.y;
    }

    let world_pos = (entity.model * vec4<f32>(pos, 1.0)).xyz;
    let world_norm = normalize((entity.model * vec4<f32>(norm, 0.0)).xyz);

    var out: VertexOutput;
    out.clip_pos = scene.proj * scene.view * vec4<f32>(world_pos, 1.0);
    out.world_pos = world_pos;
    out.normal = world_norm;
    out.color = input.color;
    out.v_meta = input.v_meta;
    return out;
}
"#;
