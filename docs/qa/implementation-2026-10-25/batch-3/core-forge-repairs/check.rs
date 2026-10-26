pub use qualia_core_db::CAPABILITY_DESCRIPTORS;
#[path="candidate/wgsl_forge/emit/coopmat.rs"]
mod coopmat;
mod seams {
    #[path="C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/args.rs"]
    pub mod args;
    #[path="C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/ids.rs"]
    pub mod ids;
    #[path="C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/core-forge-repairs/candidate/poet_host/invoke/wgsl_forge_seam.rs"]
    pub mod forge;
}
#[test]
fn candidate_cooperative_generators_validate_with_selected_etalon() {
    for source in [coopmat::matmul_tc_wgsl(),coopmat::matmul_tc_wgsl_tiled()] {
        assert!(source.starts_with("enable etalon_cooperative_matrix;"));
        qualia_core_db::wgsl_forge::validate_wgsl(&source).unwrap();
    }
}
#[test]
fn exact_ray_directive_patch_validates_without_downgrading_the_shader() {
    use qualia_core_db::wgsl_forge::*;
    let old=generate_builtin(BuiltinKernel::RayProbe,Schedule::default(),TargetBackend::Wgsl).unwrap();
    assert!(old.source.contains("enable wgpu_ray_query;"));
    // emit/wgsl.rs changes only this exact directive. This is a cached-generator
    // plus exact-source-delta probe; final Cargo must execute the rebuilt emitter.
    let source=old.source.replace("enable wgpu_ray_query;","enable etalon_ray_query;");
    assert!(source.contains("rayQuery"));
    validate_wgsl(&source).unwrap();
}
