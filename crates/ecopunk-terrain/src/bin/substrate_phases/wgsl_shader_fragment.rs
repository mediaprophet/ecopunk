//! WGSL Unified Pipeline Fragment Shader & Line Overlay Pipelines
//!
//! Provides the primary multi-mode fragment shader `fs_main` (observer modes 0x00..0x05)
//! and the debug / CAD line overlay pipeline.

pub const WGSL_FRAGMENT: &str = r#"
@fragment
fn fs_main(input: VertexOutput) -> HoloFragmentOut {
    let n = normalize(input.normal);
    let cam_pos = scene.camera_pos.xyz;
    let u_time = scene.camera_pos.w;
    let view_dir = normalize(cam_pos - input.world_pos);
    let light_dir = normalize(scene.sun_dir.xyz);
    let water_level = scene.sun_dir.w;
    let karst_fluting = scene.wind_and_clip.w;
    let kp = scene.space_and_mode.x;
    let observer_mode = i32(round(scene.space_and_mode.z));
    let n_dot_l = max(dot(n, light_dir), 0.0);
    let n_dot_v = max(dot(n, view_dir), 0.0001);
    let entity_kind = i32(round(entity.fauna_anim.w));

    // Sky Dome & Auroral Curtain Rendering
    if (entity_kind == 4) {
        let ray_dir = normalize(input.world_pos - cam_pos);
        var sky_col = mix(vec3<f32>(0.58, 0.72, 0.88), vec3<f32>(0.12, 0.28, 0.62), clamp(ray_dir.y, 0.0, 1.0));
        let sun_dot = max(dot(ray_dir, light_dir), 0.0);
        sky_col += vec3<f32>(1.0, 0.88, 0.65) * pow(sun_dot, 64.0) * 1.5;

        if (kp > 3.5 || observer_mode == 5) {
            let aurora_band = sin(ray_dir.x * 6.0 + u_time * 0.8) * cos(ray_dir.z * 5.0 - u_time * 0.5);
            let curtain = smoothstep(0.15, 0.55, ray_dir.y) * (1.0 - smoothstep(0.55, 0.88, ray_dir.y));
            let intensity = select(clamp((kp - 3.5) / 5.5, 0.0, 0.55), 0.65, observer_mode == 5);
            let aurora_col = mix(sigma_to_linear_rgb(0.45), vec3<f32>(0.85, 0.15, 0.65), clamp(ray_dir.y * 1.4, 0.0, 1.0));
            sky_col += aurora_col * curtain * (0.5 + 0.5 * aurora_band) * intensity;
        }
        return HoloFragmentOut(vec4<f32>(aces_film(sky_col), 1.0), clamp(input.clip_pos.z, 0.0, 1.0));
    }

    var base_col = input.color;

    // Terrain, 3D Rocks/Boulders/Pebbles & 6-Layer SLGA Subsurface Strata
    if (entity_kind == 0) {
        let is_wall = input.v_meta.z;
        let slope = 1.0 - max(n.y, 0.0);
        if (is_wall < 0.5) {
            if (slope > 0.35) {
                let flute = sin(input.world_pos.x * 3.8 + input.world_pos.z * 3.8) * 0.5 + 0.5;
                let fracture = hash21(floor(input.world_pos.xz * 2.5));
                base_col *= (0.82 + 0.24 * mix(fracture, flute, karst_fluting));
            } else {
                let soil_grain = hash21(floor(input.world_pos.xz * 6.0)) * 0.12 - 0.06;
                base_col += vec3<f32>(soil_grain * 0.6, soil_grain * 0.5, soil_grain * 0.35);
            }

            let dist_above_water = input.world_pos.y - water_level;
            if (dist_above_water < 0.75) {
                let peb_uv = input.world_pos.xz * 5.2;
                let peb_id = floor(peb_uv);
                let peb_f = fract(peb_uv) - vec2<f32>(0.5, 0.5);
                let peb_rnd = hash21(peb_id);
                let peb_dist = length(peb_f * vec2<f32>(1.0, 1.25));
                let peb_mask = smoothstep(0.46, 0.24, peb_dist);
                var peb_tint = vec3<f32>(0.52, 0.48, 0.42);
                if (peb_rnd > 0.72) {
                    peb_tint = vec3<f32>(0.76, 0.68, 0.56);
                } else if (peb_rnd > 0.45) {
                    peb_tint = vec3<f32>(0.58, 0.36, 0.26);
                } else if (peb_rnd < 0.22) {
                    peb_tint = vec3<f32>(0.26, 0.28, 0.30);
                }
                let sand_ripple = 0.5 + 0.5 * sin(input.world_pos.x * 9.0 + sin(input.world_pos.z * 3.0) * 1.5);
                let sand_col = vec3<f32>(0.68, 0.60, 0.45) * (0.90 + 0.14 * sand_ripple);
                let bed_tex = mix(sand_col, peb_tint * (0.75 + 0.35 * peb_mask), 0.68);
                let blend_w = clamp((0.75 - dist_above_water) / 0.95, 0.0, 0.78);
                base_col = mix(base_col, bed_tex, blend_w);
            }

            if (dist_above_water >= -0.4 && dist_above_water < 1.25) {
                let wet_factor = 1.0 - clamp(abs(dist_above_water) / 1.25, 0.0, 1.0);
                base_col *= (1.0 - 0.34 * wet_factor);
            }

            if (dist_above_water < 0.0 && observer_mode == 0) {
                let w_depth = clamp(-dist_above_water, 0.0, 6.0);
                let c1 = sin(input.world_pos.x * 3.4 + u_time * 2.3) * cos(input.world_pos.z * 3.1 - u_time * 1.8);
                let c2 = sin(input.world_pos.x * 5.7 - u_time * 1.7 + input.world_pos.z * 4.3);
                let caustic = pow(clamp(0.5 + 0.28 * c1 + 0.22 * c2, 0.0, 1.0), 3.0);
                let caustic_atten = exp(-w_depth * 0.35);
                base_col += vec3<f32>(0.28, 0.46, 0.42) * caustic * caustic_atten * 0.85;
                let absorb = exp(-vec3<f32>(0.28, 0.11, 0.07) * w_depth * 0.55);
                base_col *= absorb;
            }
        } else {
            let strata_grain = sin(input.world_pos.y * 14.0 + input.world_pos.x * 0.8) * 0.06;
            base_col += vec3<f32>(strata_grain);
        }
    }

    // Transparent Refractive Water Shader (Snell Refraction + Beer-Lambert + Fresnel Alpha)
    if (entity_kind == 3) {
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
        if (observer_mode == 4) {
            water_col = vec3<f32>(0.05, 0.22, 0.68);
        }
        let water_alpha = select(
            clamp(fresnel.x * 0.58 + (1.0 - exp(-d * 0.26)) * 0.44 + spec * 0.35, 0.16, 0.74),
            0.92,
            observer_mode == 4
        );
        return HoloFragmentOut(vec4<f32>(aces_film(water_col), water_alpha), clamp(input.clip_pos.z, 0.0, 1.0));
    }

    var final_col = base_col;

    // 0x00: WGSL_KERNEL_PBR + SGGX Anisotropic Microflake Sheen + Multi-Km Clarity Fog
    if (observer_mode == 0) {
        let h = normalize(light_dir + view_dir);
        let roughness = select(0.76, select(0.54, 0.62, entity_kind == 2), entity_kind != 0);
        let ndf = distribution_ggx(n, h, roughness);
        let g = geometry_schlick_ggx(n_dot_v, roughness) * geometry_schlick_ggx(n_dot_l, roughness);
        let f = fresnel_schlick(max(dot(h, view_dir), 0.0), vec3<f32>(0.04));
        let spec = clamp((ndf * g * f) / max(4.0 * n_dot_v * n_dot_l, 0.02) * n_dot_l * 0.35, vec3<f32>(0.0), vec3<f32>(0.28));
        let sggx_sheen = evaluate_sggx_sheen(n, view_dir, light_dir, 0.22, 0.58) * 0.45;

        let sky_amb = mix(vec3<f32>(0.18, 0.16, 0.14), vec3<f32>(0.26, 0.34, 0.46), 0.5 + 0.5 * n.y);
        var sss = vec3<f32>(0.0);
        if (entity_kind == 1 && input.v_meta.w > 1.5) {
            let back_light = max(dot(view_dir, -light_dir), 0.0);
            sss = base_col * vec3<f32>(0.35, 0.55, 0.18) * pow(back_light, 2.0) * 0.48;
        }

        let sun_radiance = vec3<f32>(1.55, 1.46, 1.32) * max(0.15, light_dir.y);
        let lit = base_col * (sky_amb + sun_radiance * n_dot_l * 0.78) + sun_radiance * (spec + vec3<f32>(sggx_sheen)) + sss;

        let fog_col = vec3<f32>(0.58, 0.72, 0.84) * max(0.25, light_dir.y);
        let fogged_lit = apply_height_stratified_valley_fog_35m(lit, cam_pos, input.world_pos, fog_col);
        final_col = aces_film(fogged_lit);
    }
    // 0x01: WGSL_KERNEL_TOON (3-Tone Quantization & Silhouette Inking)
    else if (observer_mode == 1) {
        var cel = 0.38;
        if (n_dot_l > 0.68) {
            cel = 1.12;
        } else if (n_dot_l > 0.32) {
            cel = 0.78;
        }
        let rim = smoothstep(0.18, 0.28, n_dot_v);
        final_col = base_col * cel * (0.15 + 0.85 * rim);
    }
    // 0x02: WGSL_KERNEL_WATERCOLOR (Kubelka-Munk Wash & Coffee-Ring Edge Pooling)
    else if (observer_mode == 2) {
        let grain = sin(input.world_pos.x * 4.5) * cos(input.world_pos.z * 4.5) * 0.07;
        let wash = smoothstep(0.08, 0.92, n_dot_l + grain);
        let warm_paper = vec3<f32>(0.98, 0.95, 0.88);
        let pigment = mix(base_col * 0.68, min(vec3<f32>(1.0), base_col * 1.28 + vec3<f32>(0.06, 0.03, 0.0)), wash);
        let edge_pooling = 1.0 - 0.32 * pow(1.0 - n_dot_v, 2.2);
        final_col = mix(warm_paper, pigment * edge_pooling, 0.88);
    }
    // 0x03: WGSL_KERNEL_HUD (GPR Dielectric Contrast from 16-KB ConstituentLUT & Scanlines)
    else if (observer_mode == 3) {
        let lut_slot = u32(clamp(input.v_meta.y, 1.0, 6.0));
        let dielectric_perm = sample_lut_dielectric(lut_slot);
        let gpr_intensity = clamp(dielectric_perm / 40.0, 0.1, 1.0);
        let grid = step(0.93, fract(input.world_pos.x * 0.5)) + step(0.93, fract(input.world_pos.z * 0.5));
        let contour = step(0.92, fract(input.world_pos.y * 0.8));
        let pulse = sin(input.world_pos.y * 6.0 - u_time * 5.0) * 0.5 + 0.5;
        let backing = vec3<f32>(0.02, 0.06, 0.11);
        if (entity_kind == 2) {
            final_col = mix(vec3<f32>(0.95, 0.2, 0.6), vec3<f32>(1.0, 0.85, 0.2), n_dot_l);
        } else if (entity_kind == 1) {
            final_col = mix(backing, vec3<f32>(0.1, 0.92, 0.55), 0.35 + 0.45 * n_dot_l + grid * 0.3);
        } else {
            final_col = mix(backing, vec3<f32>(0.0, 0.88, 1.0) * (0.6 + 0.4 * gpr_intensity), clamp(grid * 0.65 + contour * 0.45 + pulse * 0.15, 0.0, 1.0));
        }
    }
    // 0x04: WGSL_KERNEL_THERMAL (Planck Blackbody Ironbow Pseudocolor)
    else if (observer_mode == 4) {
        var heat = 0.35;
        if (entity_kind == 2) {
            heat = clamp(input.v_meta.w + 0.15 * n_dot_l, 0.0, 1.0);
        } else if (entity_kind == 1) {
            heat = 0.28 + 0.24 * n_dot_l;
        } else {
            heat = clamp(0.18 + n_dot_l * 0.42 + (input.world_pos.y / 30.0) * 0.15, 0.0, 0.85);
        }
        final_col = ironbow_thermal(heat);
    }
    // 0x05: Heliophysics & CIE 1931 Spectral Bioacoustic Wavefield
    else if (observer_mode == 5) {
        let mag_field = abs(sin(input.world_pos.x * 0.25 + input.world_pos.z * 0.15 + u_time * 0.8));
        let bio_col = base_col * (0.45 + 0.65 * n_dot_l);
        let spectral_wave = sigma_to_linear_rgb(mag_field);
        final_col = mix(bio_col, spectral_wave, 0.32);
    }

    if (scene.isogloss.w > 0.5 && entity_kind == 0) {
        let dist_iso = length(input.world_pos.xz - vec2<f32>(scene.isogloss.x, scene.isogloss.z));
        let ring_dist = abs(dist_iso - scene.isogloss.y);
        if (ring_dist < 0.55) {
            let glow = 1.0 - (ring_dist / 0.55);
            final_col = mix(final_col, vec3<f32>(0.0, 0.96, 0.82), glow * 0.75);
        }
    }

    return HoloFragmentOut(vec4<f32>(final_col, 1.0), clamp(input.clip_pos.z, 0.0, 1.0));
}

struct LineVertexInput {
    @location(0) position: vec3<f32>,
};

struct LineVertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
};

@vertex
fn vs_line(input: LineVertexInput) -> LineVertexOutput {
    var out: LineVertexOutput;
    out.clip_pos = scene.proj * scene.view * entity.model * vec4<f32>(input.position, 1.0);
    return out;
}

@fragment
fn fs_line() -> @location(0) vec4<f32> {
    return entity.line_color;
}
"#;
