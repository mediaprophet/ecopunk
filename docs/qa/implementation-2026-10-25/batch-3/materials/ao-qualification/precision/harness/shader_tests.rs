//! Exact candidate shaders through the actual cached Etalon Naga parser.
#[test]
fn ao_log_depth_candidate_shaders_parse_and_validate() {
    for (name, source) in [
        ("AO_PREPASS", include_str!("../candidate/src/shaders/viewport/ao_prepass.wgsl")),
        ("AO_GENERATE", include_str!("../candidate/src/shaders/viewport/ao_generate.wgsl")),
        ("MESH", include_str!("../candidate/src/shaders/viewport/mesh.wgsl")),
    ] {
        let module = naga::front::wgsl::parse_str(source).unwrap_or_else(|error| panic!("{name}: {error:?}"));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap_or_else(|error| panic!("{name}: {error:?}"));
        eprintln!("candidate_naga_validated={name}");
    }
}
