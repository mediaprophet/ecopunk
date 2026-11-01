use super::super::read_source;
use super::plan::{self, Disposition};

fn fixture() -> String {
    include_str!("../../../../assets/production-studies/white-gum-review-r1/authoring/white-gum-r2.authored.json").into()
}

#[test]
fn complete_existing_source_has_exact_all_fork_and_source_edge_coverage() {
    let source = read_source(&fixture()).unwrap();
    let plan = plan::build(&source, 0.25, 100_000, &|| false).unwrap();
    assert_eq!(plan.nodes.len(), 197);
    assert_eq!(plan.edges.len(), 196);
    assert_eq!(plan.chains.len(), 105);
    assert_eq!(plan.junctions.len(), 46);
    assert_eq!(plan.nodes[0].disposition, Disposition::RootJunction);
    assert_eq!(plan.junctions[0].incidents.len(), 8);
    assert_eq!(
        plan.nodes
            .iter()
            .filter(|n| n.disposition == Disposition::RetainedTerminal)
            .count(),
        60
    );
    assert_eq!(
        plan.nodes
            .iter()
            .filter(|n| n.disposition == Disposition::RetainedDegreeTwo)
            .count(),
        91
    );
    let mut edge_counts = vec![0usize; source.parents.len()];
    for chain in &plan.chains {
        for &edge in &chain.organs[1..] {
            edge_counts[edge] += 1;
        }
    }
    assert!(edge_counts[1..].iter().all(|&n| n == 1));
    for joint in &plan.junctions {
        assert_eq!(joint.incidents.len(), plan.nodes[joint.organ].degree);
        for incident in &joint.incidents {
            assert!(
                source.parents[incident.edge_child] == Some(joint.organ)
                    || incident.edge_child == joint.organ
            );
        }
    }
    assert_eq!(source.data.sprays.len(), 164);
    assert_eq!(source.total_leaves, 3280);
}

#[test]
fn consecutive_forks_leave_an_explicit_nonoverlapping_middle_edge() {
    let source = read_source(&fixture()).unwrap();
    let plan = plan::build(&source, 0.3, 100_000, &|| false).unwrap();
    let stem_five = source.lookup["stem-5"];
    let span = plan
        .edges
        .iter()
        .find(|e| e.edge_child == stem_five)
        .unwrap();
    assert_eq!(span.retained_start, 0.3);
    assert_eq!(span.retained_end, 0.7);
    assert!(plan.chains.iter().any(|c| c.organs.len() == 2
        && c.organs[1] == stem_five
        && c.start_joint.is_some()
        && c.end_joint.is_some()));
}

#[test]
fn root_degree_two_requires_a_connector_while_internal_degree_two_is_retained() {
    let mut source: serde_json::Value = serde_json::from_str(&fixture()).unwrap();
    source["botanical"]["organs"] = serde_json::json!([
        {"key":"collar","parent":null,"position_m":[0.,0.,0.]},
        {"key":"a","parent":"collar","position_m":[0.,1.,0.]},
        {"key":"a-tip","parent":"a","position_m":[0.,2.,0.]},
        {"key":"b","parent":"collar","position_m":[1.,0.,0.]}
    ]);
    source["radii_m"] = serde_json::json!([0.1, 0.1, 0.05, 0.05]);
    source["roles"] = serde_json::json!([
        "root_collar",
        "trunk_metamer",
        "trunk_metamer",
        "lateral_root"
    ]);
    source["sprays"] = serde_json::json!([]);
    let source = read_source(&source.to_string()).unwrap();
    let plan = plan::build(&source, 0.25, 1000, &|| false).unwrap();
    assert_eq!(plan.nodes[0].disposition, Disposition::RootConnector);
    assert_eq!(plan.nodes[1].disposition, Disposition::RetainedDegreeTwo);
    assert_eq!(plan.junctions.len(), 1);
    assert_eq!(plan.junctions[0].incidents.len(), 2);
    assert_eq!(plan.chains.len(), 2);
}

#[test]
fn invalid_cuts_work_exhaustion_and_cancellation_cannot_yield_a_partial_plan() {
    let source = read_source(&fixture()).unwrap();
    for cut in [0.0, -0.1, 0.5, 1.0, f64::NAN, f64::INFINITY] {
        assert!(plan::build(&source, cut, 100_000, &|| false).is_err());
    }
    assert!(plan::build(&source, 0.25, 1, &|| false)
        .unwrap_err()
        .contains("caller work"));
    assert!(plan::build(&source, 0.25, 100_000, &|| true)
        .unwrap_err()
        .contains("cancelled"));
}

#[test]
fn triangle_cuts_retain_source_diagonals_even_when_the_side_quad_is_not_planar() {
    use crate::scene_builder_10d::SubmeshBuilder;
    let mut mesh = SubmeshBuilder::new();
    for station in 0..2 {
        for side in 0..5 {
            let angle = std::f32::consts::TAU * side as f32 / 5.0;
            // Deliberately twisted and tapered: matching-ring interpolation
            // alone cannot describe both source triangles' surface.
            let a = angle + station as f32 * 0.4;
            mesh.push_vertex(
                [
                    a.cos() * (1.0 - station as f32 * 0.2),
                    station as f32,
                    a.sin(),
                ],
                [angle.cos(), 0.0, angle.sin()],
                [0.2, 0.3, 0.4, 1.0],
                [side as f32 / 5.0, station as f32],
            );
        }
    }
    let middle = super::cuts::ring(&mesh, 0, 0, 5, 0.3).unwrap();
    assert_eq!(middle.len(), 10);
    assert_eq!(middle[1].source_vertices, [0, 6, 6]);
    assert_eq!(middle[1].barycentric, [0.7, 0.3, 0.0]);
    for axis in 0..3 {
        assert_eq!(
            middle[1].position[axis],
            (f64::from(mesh.positions[0][axis]) * 0.7 + f64::from(mesh.positions[6][axis]) * 0.3)
                as f32
        );
    }
    let lower = super::cuts::ring(&mesh, 0, 0, 5, 0.0).unwrap();
    let upper = super::cuts::ring(&mesh, 0, 0, 5, 1.0).unwrap();
    let mut triangles = Vec::new();
    super::cuts::strip(&lower, &upper, |t| {
        triangles.push(t.map(|v| v.source_vertices[0]));
        Ok(())
    })
    .unwrap();
    assert_eq!(triangles.len(), 10);
    assert_eq!(triangles[0], [0, 1, 6]);
    assert_eq!(triangles[1], [0, 6, 5]);
}

#[test]
fn explicit_clusters_require_connected_unique_required_junctions_and_cover_all_jobs() {
    let source = read_source(&fixture()).unwrap();
    let plan = plan::build(&source, 0.4, 100_000, &|| false).unwrap();
    let stems = (4..=12).map(|i| format!("stem-{i}")).collect::<Vec<_>>();
    let (clusters, _) = plan::clusters(&source, &plan, &[stems], 10_000_000, &|| false).unwrap();
    assert_eq!(clusters.len(), 38);
    assert_eq!(
        clusters.iter().map(|g| g.junctions.len()).sum::<usize>(),
        46
    );
    assert!(clusters
        .iter()
        .any(|g| g.junctions.len() == 9 && g.interior_chains.len() == 8));
    for bad in [
        vec![vec!["stem-4".into(), "stem-4".into()]],
        vec![vec!["stem-4".into()], vec!["stem-4".into()]],
        vec![vec!["stem-4".into(), "scaffold-0-3".into()]],
        vec![vec!["stem-1".into()]],
        vec![vec!["missing".into()]],
    ] {
        assert!(plan::clusters(&source, &plan, &bad, 10_000_000, &|| false).is_err());
    }
}

fn spans(projection: &super::super::WhiteGumAuthoredReview) -> Vec<super::ChainSpan> {
    let range = |value: &serde_json::Value| {
        value[0].as_u64().unwrap() as usize..value[1].as_u64().unwrap() as usize
    };
    projection.metadata["bindings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|b| b["kind"] == "chain")
        .map(|b| super::ChainSpan {
            vertices: range(&b["wood_vertices"]),
            indices: range(&b["wood_indices"]),
            effective_radii_m: b["effective_station_radii_m"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect(),
        })
        .collect()
}

#[test]
fn closed_stub_has_exact_guard_fade_cap_only_generated_point_and_source_aliases() {
    let source = read_source(&fixture()).unwrap();
    let projection = super::super::build_white_gum_authored_review(&fixture()).unwrap();
    let plan = plan::build(&source, 0.4, 100_000, &|| false).unwrap();
    let spans = spans(&projection);
    let incident = &plan.junctions[0].incidents[0];
    let chain = &plan.chains[incident.chain];
    let a = source.data.botanical.organs[chain.organs[0]].position_m;
    let b = source.data.botanical.organs[chain.organs[1]].position_m;
    let delta = std::array::from_fn::<_, 3, _>(|k| f64::from(b[k]) - f64::from(a[k]));
    let length = delta.iter().map(|v| v * v).sum::<f64>().sqrt();
    let (piece, port) = super::pieces::stub(
        &projection.wood,
        &spans[incident.chain],
        &chain.organs,
        source.data.botanical.ring_slices,
        true,
        0.4,
        0.8,
        0.5,
        "stem-1".into(),
        "collar".into(),
        ["collar".into(), "stem-1".into()],
        0,
        delta.map(|v| v / length),
        length,
    )
    .unwrap();
    assert_eq!(port.cut_vertices.len(), 24);
    assert_eq!(port.guard_vertices.len(), 24);
    for &v in &port.guard_vertices {
        assert_eq!(piece.fade_coordinates[v as usize], 0.0);
    }
    for &v in &port.cut_vertices {
        assert!(piece.fade_coordinates[v as usize] < 0.0);
    }
    let generated = piece
        .vertex_sources
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.original_wood_vertices.is_none().then_some(i))
        .collect::<Vec<_>>();
    assert_eq!(generated.len(), 1);
    let cap = port
        .cut_cap_faces
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    for (face, t) in piece.mesh.indices.chunks_exact(3).enumerate() {
        if t.contains(&(generated[0] as u32)) {
            assert!(cap.contains(&(face as u32)));
        }
    }
    let (receipt, _) = super::topology::qualify(
        &piece.mesh,
        &piece.topology_aliases,
        &piece.mesh.indices,
        16 * 1024 * 1024,
        10_000_000,
        &|| false,
    )
    .unwrap();
    assert_eq!(receipt["components"], 1);
}

fn controls() -> crate::smooth_collars::CollarControls {
    crate::smooth_collars::CollarControls {
        blend_m: 0.02,
        projection_tolerance_m: 1e-5,
        gradient_step_m: 1e-5,
        max_projection_step_m: 0.01,
        max_displacement_m: 0.1,
        projection_iterations: 32,
        refinement_rounds: 1,
        max_dihedral_radians: 1.5,
        min_field_normal_alignment: 0.4,
        minimum_triangle_area_sq_m4: 1e-24,
    }
}

#[test]
fn aggregate_budget_cancellation_or_invalid_required_group_preserves_both_mesh_and_metadata() {
    let source = read_source(&fixture()).unwrap();
    let projection = super::super::build_white_gum_authored_review(&fixture()).unwrap();
    let spans = spans(&projection);
    let spec = super::AllForkSpec {
        schema: "ecopunk.all-fork-collars/v1".into(),
        cut_fraction: 0.4,
        guard_fraction: 0.8,
        transition_fraction: 0.5,
        collar: controls(),
        coupled_junctions: Vec::new(),
    };
    for case in 0..3 {
        let mut wood = projection.wood.clone();
        let mut metadata = projection.metadata.clone();
        let mut limits = super::AllForkLimits::default();
        let mut selected = spec.clone();
        if case == 0 {
            limits.bytes = 1024;
        }
        if case == 1 {
            selected.coupled_junctions = vec![vec!["missing-required-fork".into()]];
        }
        let result = super::apply(
            &mut wood,
            &projection.foliage,
            &mut metadata,
            &source,
            &spans,
            &selected,
            limits,
            &|| case == 2,
        );
        assert!(result.is_err());
        assert_eq!(wood.positions, projection.wood.positions);
        assert_eq!(wood.normals, projection.wood.normals);
        assert_eq!(wood.colors, projection.wood.colors);
        assert_eq!(wood.uvs, projection.wood.uvs);
        assert_eq!(wood.indices, projection.wood.indices);
        assert_eq!(metadata, projection.metadata);
    }
}

fn small_fork_source() -> serde_json::Value {
    let mut value: serde_json::Value = serde_json::from_str(&fixture()).unwrap();
    value["botanical"]["organs"] = serde_json::json!([
        {"key":"collar","parent":null,"position_m":[0.,0.,0.]},
        {"key":"fork","parent":"collar","position_m":[0.,0.8,0.]},
        {"key":"up","parent":"fork","position_m":[0.,1.6,0.]},
        {"key":"side","parent":"fork","position_m":[0.8,0.8,0.]}
    ]);
    value["radii_m"] = serde_json::json!([0.1, 0.08, 0.05, 0.055]);
    value["roles"] = serde_json::json!([
        "root_collar",
        "trunk_metamer",
        "trunk_metamer",
        "scaffold_branch"
    ]);
    value["botanical"]["ring_slices"] = serde_json::json!(8);
    value["sprays"] = serde_json::json!([{"key":"retained-shoot","edge_end":"up","edge_fraction":0.8,
        "direction":[0.4,0.2,0.1],"length_m":0.2,"leaf_length_m":0.12,"leaf_width_m":0.018,
        "leaf_count":6,"seed":1.0}]);
    value
}

#[test]
fn all_three_small_source_stubs_are_individually_closed_before_boolean_union() {
    let value = small_fork_source();
    let text = value.to_string();
    let source = read_source(&text).unwrap();
    let baseline = super::super::build_white_gum_authored_review(&text).unwrap();
    let plan = plan::build(&source, 0.45, 100_000, &|| false).unwrap();
    let spans = spans(&baseline);
    assert_eq!(plan.junctions.len(), 1);
    let joint = &plan.junctions[0];
    assert_eq!(joint.incidents.len(), 3);
    let mut directions = Vec::new();
    for (piece_index, incident) in joint.incidents.iter().enumerate() {
        let chain = &plan.chains[incident.chain];
        let edge = if incident.at_start {
            0
        } else {
            chain.organs.len() - 2
        };
        let parent = chain.organs[edge];
        let child = chain.organs[edge + 1];
        let a = source.data.botanical.organs[parent]
            .position_m
            .map(f64::from);
        let b = source.data.botanical.organs[child]
            .position_m
            .map(f64::from);
        let delta = std::array::from_fn::<_, 3, _>(|k| b[k] - a[k]);
        let length = delta.iter().map(|v| v * v).sum::<f64>().sqrt();
        let key = |n: usize| source.data.botanical.organs[n].key.clone();
        let (piece, port) = super::pieces::stub(
            &baseline.wood,
            &spans[incident.chain],
            &chain.organs,
            8,
            incident.at_start,
            0.45,
            0.9,
            0.25,
            key(child),
            key(joint.organ),
            [key(parent), key(child)],
            piece_index,
            delta.map(|v| v / length * if incident.at_start { 1.0 } else { -1.0 }),
            length,
        )
        .unwrap();
        assert_eq!(
            piece.mesh.positions.len(),
            50,
            "piece {}",
            piece.edge_child_key
        );
        assert_eq!(
            piece.mesh.indices.len() / 3,
            80,
            "piece {}",
            piece.edge_child_key
        );
        assert_eq!(
            piece.topology_aliases.iter().copied().max(),
            Some(41),
            "piece {}",
            piece.edge_child_key
        );
        assert_eq!(port.cut_vertices.len(), 16);
        assert_eq!(port.guard_vertices.len(), 16);
        let (receipt, _) = super::topology::qualify(
            &piece.mesh,
            &piece.topology_aliases,
            &piece.mesh.indices,
            16 * 1024 * 1024,
            10_000_000,
            &|| false,
        )
        .unwrap_or_else(|e| {
            panic!(
                "Source stub {} at_start={} before Boolean: {e}",
                piece.edge_child_key, incident.at_start
            )
        });
        assert_eq!(receipt["components"], 1);
        for &v in &port.guard_vertices {
            assert_eq!(piece.fade_coordinates[v as usize], 0.0);
        }
        directions.push(incident.at_start);
    }
    assert_eq!(directions, [false, true, true]);
}

/// End-to-end source-level regression, deliberately small enough to expose the
/// first failing public operator before attempting the 46-junction source.
#[test]
fn one_required_fork_uses_positive_blend_and_preserves_authored_spray_and_leaf_streams() {
    let mut value = small_fork_source();
    let baseline = super::super::build_white_gum_authored_review(&value.to_string()).unwrap();
    value["source_version"] = serde_json::json!("white-gum-all-forks-v1");
    let mut c = controls();
    c.blend_m = 0.015;
    value["all_forks"] = serde_json::json!({"schema":"ecopunk.all-fork-collars/v1",
        "cut_fraction":0.45,"guard_fraction":0.9,"transition_fraction":0.25,
        "collar":c,"coupled_junctions":[]});
    let adopted = super::super::build_white_gum_authored_review(&value.to_string()).unwrap();
    assert_eq!(
        &adopted.wood.positions[..baseline.wood.positions.len()],
        baseline.wood.positions
    );
    assert_eq!(
        &adopted.wood.normals[..baseline.wood.normals.len()],
        baseline.wood.normals
    );
    assert_eq!(
        &adopted.wood.colors[..baseline.wood.colors.len()],
        baseline.wood.colors
    );
    assert_eq!(
        &adopted.wood.uvs[..baseline.wood.uvs.len()],
        baseline.wood.uvs
    );
    assert_eq!(adopted.foliage.positions, baseline.foliage.positions);
    assert_eq!(adopted.foliage.normals, baseline.foliage.normals);
    assert_eq!(adopted.foliage.uvs, baseline.foliage.uvs);
    assert_eq!(adopted.foliage.indices, baseline.foliage.indices);
    assert_eq!(
        adopted.metadata["all_forks"]["plan"]["junctions"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        adopted.metadata["all_forks"]["structural_topology"]["components"],
        1
    );
    assert_eq!(adopted.metadata["all_forks"]["retained_sprays"], 1);
    assert_eq!(adopted.metadata["all_forks"]["retained_leaves"], 6);
    let last = adopted.metadata["bindings"]
        .as_array()
        .unwrap()
        .last()
        .unwrap();
    assert_eq!(last["key"], "retained-shoot");
    assert_eq!(
        last["leaf_identifier_pattern"],
        "retained-shoot/leaf/{index}"
    );
    let range =
        |v: &serde_json::Value| v[0].as_u64().unwrap() as usize..v[1].as_u64().unwrap() as usize;
    let old = baseline.metadata["bindings"]
        .as_array()
        .unwrap()
        .last()
        .unwrap();
    assert_eq!(
        &adopted.wood.indices[range(&last["wood_indices"])],
        &baseline.wood.indices[range(&old["wood_indices"])]
    );
    super::projection_witness(&adopted.wood, &adopted.foliage, &adopted.metadata).unwrap();
}
