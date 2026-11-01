use super::*;

#[test]
fn absent_profile_keeps_legacy_source_shape_and_unknown_profile_refuses() {
    let old = parse_source(DEFAULT_SOURCE).unwrap();
    assert_eq!(old.numeric_profile, None);
    let mut value = serde_json::to_value(&old).unwrap();
    assert!(value.get("numeric_profile").is_none());
    value["numeric_profile"] = serde_json::json!("portable-software-v2");
    assert!(parse_source(&value.to_string()).is_err());
    value["numeric_profile"] = serde_json::json!("portable-software-v1");
    let admitted = parse_source(&value.to_string()).unwrap();
    assert_eq!(admitted.numeric_profile, Some(NumericProfile::PortableSoftwareV1));
    assert_eq!(serde_json::to_value(&admitted).unwrap()["numeric_profile"], "portable-software-v1");
}

#[test]
fn portable_profile_reaches_all_anatomy_layers_without_changing_source_correspondence() {
    let legacy = build_from_json(DEFAULT_SOURCE).unwrap();
    let mut source = legacy.source.clone();
    source.numeric_profile = Some(NumericProfile::PortableSoftwareV1);
    let portable = build_from_json(&serde_json::to_string(&source).unwrap()).unwrap();
    for (a, b) in [(&legacy.skin, &portable.skin),
                   (&legacy.osteology, &portable.osteology),
                   (&legacy.myology, &portable.myology)] {
        assert_eq!(a.numeric_profile(), None);
        assert_eq!(b.numeric_profile(), source.numeric_profile);
        assert_eq!(a.positions.len(), b.positions.len());
        assert!(!b.positions.is_empty());
        assert_eq!(a.indices, b.indices);
        assert_eq!(a.uvs, b.uvs);
        assert_eq!(a.joint_indices, b.joint_indices);
        assert_eq!(a.joint_weights, b.joint_weights);
        assert!(b.positions.iter().flatten().all(|x| x.is_finite()));
        assert!(b.normals.iter().flatten().all(|x| x.is_finite()));
        assert!(b.colors.iter().flatten().all(|x| x.is_finite()));
    }
    assert_eq!(legacy.parts.len(), portable.parts.len());
    for (a, b) in legacy.parts.iter().zip(&portable.parts) {
        assert_eq!((&a.source_id, a.material, &a.vertices, &a.indices),
                   (&b.source_id, b.material, &b.vertices, &b.indices));
    }
    // This establishes topology/attribute correspondence, not native/WASM bytes.
    // The coordinator must separately compare complete generated HMC resources.
}

#[test]
fn bishop_angular_column_uses_declared_owner_and_preserves_legacy_constructor() {
    let ring = AnatomicalLoftRing {
        center: [0.0, 0.0, 0.0], radii: [1.0, 1.0],
        dorsal_color: [0.8;4], flank_color: [0.5;4], ventral_color: [0.2;4],
        primary_joint: 0, secondary_joint: 1, primary_weight: 0.5,
    };
    let mut end = ring; end.center = [0.0, 1.0, 0.0];
    let rings = [ring, end];
    for profile in [None, Some(NumericProfile::PortableSoftwareV1)] {
        let mut mesh = AnatomyMeshAccumulator::with_numeric_profile(profile);
        mesh.push_countershaded_loft(&rings, 48, Layer::ExternalIntegument, true);
        apply_bishop_sections(&mut mesh, &rings, 48, 0).unwrap();
        let angle = 28.0_f32 / 48.0 * core::f32::consts::TAU;
        // Straight +Y: initial u=-X, v=+Z. This exact column differed in R5.
        let (c,s) = match profile {
            None => (angle.cos(), angle.sin()),
            Some(_) => (libm::cosf(angle), libm::sinf(angle)),
        };
        assert_eq!(mesh.positions[28][0].to_bits(), (-c).to_bits());
        assert_eq!(mesh.positions[28][2].to_bits(), s.to_bits());
    }
    assert_eq!(AnatomyMeshAccumulator::new().numeric_profile(), None);
}
