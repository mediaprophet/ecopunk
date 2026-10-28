// Include inside rig_binding::tests, alongside its existing Fixture.
#[test]
fn staged_rig_control_keeps_existing_preview_and_refuses_wrong_resource() {
    let f = Fixture::new();
    let mut preview = AnimatedRigPreview::new(f.admit(f.descriptor.clone(), 42 * 1024 * 1024).unwrap());
    let before = preview.advance(0.0, false).unwrap().positions.to_vec();
    let mesh = || BindMesh {
        positions: &f.positions, normals: &f.normals, uv0: &f.uv, indices: &f.indices,
    };
    let span = ViewportSpan { first_vertex: 4, vertex_count: 3 };
    assert!(super::rig_candidate::stage_rig_control(
        f.source.clone(), f.descriptor.clone(), "wrong", &f.bytes, mesh(), span,
        42 * 1024 * 1024, RigControl::Seek { seconds: 1.0 },
    ).is_err());
    let candidate = super::rig_candidate::stage_rig_control(
        f.source.clone(), f.descriptor.clone(), &f.descriptor.resource_key, &f.bytes,
        mesh(), span, 42 * 1024 * 1024, RigControl::Seek { seconds: 1.0 },
    ).unwrap();
    assert_eq!(preview.rig().descriptor(), &f.descriptor);
    assert_eq!(preview.advance(0.0, false).unwrap().positions, before);
    assert_eq!(parse_descriptor_source_literal(candidate.literal()).unwrap(), *candidate.after());
    let mut rebuilt = AnimatedRigPreview::new(f.admit(candidate.after().clone(), 42 * 1024 * 1024).unwrap());
    assert!((rebuilt.advance(0.0, false).unwrap().positions[0][0] - 1.0).abs() < 1e-5);
}
