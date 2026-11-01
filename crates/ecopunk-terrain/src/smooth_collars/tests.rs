use super::*;
#[test]
fn exact_source_corner_has_exact_barycentric_and_explicit_cap_lineage() {
    let mut mesh = SubmeshBuilder::new();
    for (i, p) in [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
        .into_iter()
        .enumerate()
    {
        mesh.push_vertex(p, [0., 0., 1.], [1.; 4], [[0., 0.], [1., 0.], [0., 1.]][i]);
    }
    mesh.push_tri(0, 1, 2);
    let piece = CollarPiece {
        mesh,
        topology_aliases: vec![0, 1, 2],
        vertex_sources: vec![
            VertexSource {
                original_wood_vertices: Some([5, 5, 5]),
                barycentric: [1., 0., 0.]
            };
            3
        ],
        fade_coordinates: vec![0., 0., 1.],
        edge_child_key: "edge".into(),
        endpoint_keys: ["joint".into(), "end".into()],
    };
    let (_, w) = attributes::weights(seed::point([1., 0., 0.]), &piece, 0).unwrap();
    assert_eq!(w, [0., 1., 0.]);
    let cap = VertexSource {
        original_wood_vertices: None,
        barycentric: [0.; 3],
    };
    assert!(cap.original_wood_vertices.is_none());
}
#[test]
fn zero_blend_is_not_accepted_as_a_smooth_operation() {
    let c = CollarControls {
        blend_m: 0.,
        projection_tolerance_m: 1e-6,
        gradient_step_m: 1e-5,
        max_projection_step_m: 0.01,
        max_displacement_m: 0.1,
        projection_iterations: 32,
        refinement_rounds: 1,
        max_dihedral_radians: 0.7,
        min_field_normal_alignment: 0.8,
        minimum_triangle_area_sq_m4: 1e-24,
    };
    assert!(validate_controls(c).is_err());
}

#[test]
fn fixed_boundary_chart_streams_preserve_exact_source_bits() {
    use qualia_core_db::specialized_libs::computational_geometry::smooth_junction::FaceSource;
    let prescribed = [-0.0, 0.0, f32::from_bits(1.0f32.to_bits() + 1)];
    let mesh = SubmeshBuilder {
        positions: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        normals: vec![prescribed; 3],
        colors: vec![[-0., 0.5, 1., 1.]; 3],
        uvs: vec![[-0., 0.], [1., 0.], [0., 1.]],
        indices: vec![0, 1, 2],
    };
    let pieces = [CollarPiece {
        mesh,
        topology_aliases: vec![0, 1, 2],
        vertex_sources: (0..3)
            .map(|i| VertexSource {
                original_wood_vertices: Some([i, i, i]),
                barycentric: [1., 0., 0.],
            })
            .collect(),
        fade_coordinates: vec![0.; 3],
        edge_child_key: "edge".into(),
        endpoint_keys: ["joint".into(), "end".into()],
    }];
    let input = CollarInput {
        pieces: &pieces,
        ports: &[],
        field_segments: &[],
        joint_ids: &[],
    };
    let points: Vec<_> = pieces[0]
        .mesh
        .positions
        .iter()
        .copied()
        .map(seed::point)
        .collect();
    let normals = vec![seed::point([0., 0., 1.]); 3];
    let mut work = 10000;
    let out = attributes::project(
        &input,
        &points,
        &points,
        &normals,
        &[0.; 3],
        &[[0, 1, 2]],
        &[FaceSource { piece: 0, face: 0 }],
        &[0, 1, 2],
        &Default::default(),
        1e-24,
        &mut work,
    )
    .unwrap();
    for i in 0..3 {
        assert_eq!(
            out.mesh.normals[i].map(f32::to_bits),
            prescribed.map(f32::to_bits)
        );
        assert_eq!(
            out.mesh.uvs[i].map(f32::to_bits),
            pieces[0].mesh.uvs[i].map(f32::to_bits)
        );
        assert_eq!(
            out.mesh.colors[i].map(f32::to_bits),
            pieces[0].mesh.colors[i].map(f32::to_bits)
        );
    }
}

#[test]
fn maximum_alias_refuses_without_32_bit_wrap_or_allocation() {
    let mesh = SubmeshBuilder {
        positions: vec![[0., 0., 0.]; 4],
        normals: vec![[0., 1., 0.]; 4],
        colors: vec![[1.; 4]; 4],
        uvs: vec![[0.; 2]; 4],
        indices: vec![0; 12],
    };
    let pieces = [CollarPiece {
        mesh,
        topology_aliases: vec![u32::MAX; 4],
        vertex_sources: vec![
            VertexSource {
                original_wood_vertices: Some([0; 3]),
                barycentric: [1., 0., 0.],
            };
            4
        ],
        fade_coordinates: vec![0.; 4],
        edge_child_key: "edge".into(),
        endpoint_keys: ["joint".into(), "end".into()],
    }];
    let input = CollarInput {
        pieces: &pieces,
        ports: &[],
        field_segments: &[],
        joint_ids: &[],
    };
    let mut work = 10000;
    let result = seed::build(&input, CollarLimits::default(), &mut work, &|| false);
    assert!(matches!(result, Err(ref e) if e.contains("alias")));
}
