//! WGSL Forge Shader Pipeline Certification & Manifest Generation
//!
//! Integrates Naga validation across viewport shaders, observer kernels (0x00..0x05),
//! and the unified multi-material WebGPU shader pipeline.

use std::fs;
use std::io;
use std::path::Path;

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
use serde_json::{json, Value};

use super::wgsl_shader_fragment::WGSL_FRAGMENT;
use super::wgsl_shader_prelude::WGSL_PRELUDE;

/// Builds the unified WebGPU pipeline WGSL string.
pub fn build_unified_webgpu_wgsl() -> String {
    format!(
        "{}\n\n{}\n\n{}",
        WGSL_PRELUDE.trim(),
        SPECTRAL_WGSL.trim(),
        WGSL_FRAGMENT.trim()
    )
}

/// Validates all viewport shaders and observer kernels via Naga and writes `wgsl_forge_manifest.json`.
pub fn validate_and_emit_wgsl_forge_manifest(
    assets_dir: &Path,
    lut_bytes: &[u8],
) -> io::Result<Value> {
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

    Ok(forge_manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_wgsl_string_assembly() {
        let shader = build_unified_webgpu_wgsl();
        assert!(shader.contains("struct SceneUniforms"));
        assert!(shader.contains("fn vs_main"));
        assert!(shader.contains("fn fs_main"));
        assert!(shader.contains("fn vs_line"));
        assert!(shader.contains("fn fs_line"));
    }
}
