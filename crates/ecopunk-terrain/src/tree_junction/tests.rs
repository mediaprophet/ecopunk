use super::*;
fn fixture() -> serde_json::Value {
    let mut data:Value=serde_json::from_str(include_str!("../../../../assets/production-studies/white-gum-review-r1/authoring/white-gum-r2.authored.json")).unwrap();
    data["botanical"]["organs"] = json!([
        {"key":"collar","parent":null,"position_m":[0.,0.,0.]},
        {"key":"fork","parent":"collar","position_m":[0.,0.5,0.]},
        {"key":"continuation","parent":"fork","position_m":[0.,1.,0.]},
        {"key":"untouched","parent":"fork","position_m":[0.7,0.7,0.]}]);
    data["botanical"]["ring_slices"] = json!(8);
    data["radii_m"] = json!([0.2, 0.2, 0.2, 0.1]);
    data["roles"] = json!([
        "root_collar",
        "trunk_metamer",
        "trunk_metamer",
        "scaffold_branch"
    ]);
    data["sprays"] = json!([]);
    data
}
fn selected(mut data: Value) -> Value {
    data["source_version"] = json!("white-gum-chain-union-v1");
    data["chain_union"] = json!({
        "schema":"ecopunk.incident-chain-union/v1","chains":["fork","continuation"],
        "attribute_policy":"source-face-barycentric-v1","min_area_sq_m4":1e-24});
    data
}
#[test]
fn matching_incident_rings_remove_internal_caps_and_preserve_untouched_streams() {
    let legacy =
        crate::white_gum_authored_review::build_white_gum_authored_review(&fixture().to_string())
            .unwrap();
    let data = selected(fixture());
    let joined =
        crate::white_gum_authored_review::build_white_gum_authored_review(&data.to_string())
            .unwrap();
    assert_eq!(
        &joined.wood.positions[..legacy.wood.positions.len()],
        legacy.wood.positions
    );
    assert_eq!(
        &joined.wood.normals[..legacy.wood.normals.len()],
        legacy.wood.normals
    );
    assert_eq!(&joined.wood.uvs[..legacy.wood.uvs.len()], legacy.wood.uvs);
    assert_eq!(
        &joined.wood.colors[..legacy.wood.colors.len()],
        legacy.wood.colors
    );
    let old = &legacy.metadata["bindings"][2];
    let new = &joined.metadata["bindings"][2];
    assert_eq!(
        &legacy.wood.indices[range(&old["wood_indices"]).unwrap()],
        &joined.wood.indices[range(&new["wood_indices"]).unwrap()]
    );
    assert_eq!(
        joined.metadata["chain_union"]["receipt"]["component_count"],
        1
    );
    assert_eq!(joined.metadata["chain_union"]["receipt"]["faces"], 48); // two 8-sided walls, only two terminal caps
    assert_eq!(joined.metadata["authored_source"], data);
    assert!(!joined.metadata["junctions"]
        .as_str()
        .unwrap()
        .contains("all welded"));
}
#[test]
fn new_profile_roundtrips_existing_source_package_and_failed_edit_retains_revision() {
    let mut session = crate::species_review_source::SpeciesReviewSourceSession::default();
    let source = format!("let mut species_asset = {};", selected(fixture()));
    let asset = session
        .submit(&source, |a| crate::species_authored_review::build(0, a))
        .unwrap();
    assert_eq!(asset.render.metadata["id"], "white-gum-chain-union-v1");
    let mut reopened = crate::species_review_reopen::reopen(0, &asset.bundle).unwrap();
    assert_eq!(reopened.asset.render.bytes, asset.render.bytes);
    let bad = source
        .replace("let mut species_asset =", "species_asset =")
        .replace("source-face-barycentric-v1", "invented-normal-policy");
    assert!(reopened.session.submit(&bad, 1).is_err());
    assert_eq!(reopened.session.revision(), 1);
}
#[test]
fn unknown_nonincident_budget_and_policy_requests_refuse() {
    let mut data = selected(fixture());
    data["chain_union"]["chains"] = json!(["fork", "missing"]);
    assert!(
        crate::white_gum_authored_review::build_white_gum_authored_review(&data.to_string())
            .is_err()
    );
    data = selected(fixture());
    let mut limits = TreeJunctionLimits::default();
    limits.bytes = 1;
    assert!(
        crate::white_gum_authored_review::build_with_junction_limits(&data.to_string(), limits)
            .is_err()
    );
    limits = TreeJunctionLimits::default();
    limits.work = 1;
    assert!(
        crate::white_gum_authored_review::build_with_junction_limits(&data.to_string(), limits)
            .is_err()
    );
    data["source_version"] = json!("white-gum-authored-review-r2");
    assert!(
        crate::white_gum_authored_review::build_white_gum_authored_review(&data.to_string())
            .is_err()
    );
}
#[test]
fn actual_white_gum_retains_source_and_does_not_silently_bypass_host_cap() {
    let mut data:Value=serde_json::from_str(include_str!("../../../../assets/production-studies/white-gum-review-r1/authoring/white-gum-r2.authored.json")).unwrap();
    data = selected(data);
    data["chain_union"]["chains"] = json!(["stem-1", "stem-5"]);
    let mut limits = TreeJunctionLimits::default();
    limits.input_faces = 1;
    assert!(
        crate::white_gum_authored_review::build_with_junction_limits(&data.to_string(), limits)
            .is_err()
    );
}
#[test]
fn actual_native_quantization_collapse_refuses_changed_face() {
    let mut wood = SubmeshBuilder::new();
    for (i, p) in [[0., 0., 0.], [1., 1., 0.], [1e-8, 0., 0.]]
        .into_iter()
        .enumerate()
    {
        wood.push_vertex(p, [0., 0., -1.], [1.; 4], [[0., 0.], [1., 0.], [0., 1.]][i]);
    }
    wood.push_tri(0, 1, 2);
    let foliage = SubmeshBuilder::new();
    let metadata = json!({"chain_union":{"wood_indices":[0,3],"wood_vertices":[0,3],"render_vertex_lineage":[{"topology_vertex":0},{"topology_vertex":1},{"topology_vertex":2}],"min_area_sq_m4":1e-24}});
    let witness = projection_witness(&wood, &foliage, &metadata).unwrap();
    let asset = crate::species_review::compile_review_parts(
        vec![crate::species_review::ReviewPart {
            name: "wood".into(),
            mesh: wood,
            roughness: 0.7,
            double_sided: false,
        }],
        json!({"id":"tree-native-projection-loss"}),
    )
    .unwrap();
    assert!(qualify_compiled_projection(&asset.bytes, &witness).is_err());
}

#[test]
fn valid_canonical_frames_must_match_changed_decoded_uv_direction_and_handedness() {
    use qualia_core_db::{
        container_10d as native,
        render::{assets::Mesh, compile_10d::compile_mesh_to_10d_with_extras_and_frames},
    };
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
    let uv = [[0., 0.], [1., 0.], [0., 1.]];
    let normals = [[0., 0., 1.]; 3];
    let mut wood = SubmeshBuilder::new();
    for i in 0..3 {
        wood.push_vertex(positions[i], normals[i], [1.; 4], uv[i]);
    }
    wood.push_tri(0, 1, 2);
    let metadata = json!({"chain_union":{"wood_indices":[0,3],"wood_vertices":[0,3],
        "render_vertex_lineage":[{"topology_vertex":0},{"topology_vertex":1},{"topology_vertex":2}],"min_area_sq_m4":1e-24}});
    let witness = projection_witness(&wood, &SubmeshBuilder::new(), &metadata).unwrap();
    let mesh = Mesh {
        positions: positions.to_vec(),
        triangles: vec![[0, 1, 2]],
        min: [0.; 3],
        max: [1., 1., 0.],
    };
    for (tangent, accepted) in [
        ([1., 0., 0., 1.], true),
        ([1., 0., 0., -1.], false), // Valid typed frame, wrong chart handedness.
        ([0., 1., 0., 1.], false),  // Valid typed frame, rotated 90 degrees in its plane.
    ] {
        let authored = [tangent; 3];
        let bytes = compile_mesh_to_10d_with_extras_and_frames(
            &mesh,
            &[],
            None,
            Default::default(),
            None,
            Some(&normals),
            Some(&authored),
        )
        .unwrap();
        let mut bytes = native::attach_texture_coordinate_section(&bytes, &uv).unwrap();
        native::integrity::seal_whole_file_crc32c(&mut bytes);
        native::integrity::check_whole_file_crc32c(&bytes).unwrap();
        let limits = native::MeshFrameDecodeLimits {
            max_vertices: 3,
            max_payload_bytes: bytes.len(),
            max_decoded_bytes: 3 * 28,
        };
        let mut decoded_normals = [[0.; 3]; 3];
        let mut decoded_tangents = [[0.; 4]; 3];
        native::read_container_mesh_frames_into(
            &bytes,
            3,
            limits,
            &mut decoded_normals,
            &mut decoded_tangents,
        )
        .unwrap();
        assert_eq!(decoded_normals, normals);
        assert_eq!(decoded_tangents, authored); // Persisted, codec-valid mismatch; no byte corruption.
        let result = qualify_compiled_projection(&bytes, &witness);
        if accepted {
            result.unwrap();
        } else {
            assert!(result.unwrap_err().contains("Stored tree tangent deviates"));
        }
    }
}
