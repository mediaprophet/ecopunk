
#[test]
fn selected_rig_span_requires_exact_unshared_compiled_geometry() {
    use crate::graphical::rig_binding::{BindMesh, ViewportSpan};
    let source = source();
    let mut compiled = scene();
    let selected = PartSelection { source, frame_id: 1, part_key: "front".into(), semantic_id: 102, triangle_index: 1 };
    let positions = compiled.mesh.positions[3..6].to_vec();
    let normals = compiled.normals[3..6].to_vec();
    let uv = compiled.uv0[3..6].to_vec();
    let indices = [0, 1, 2];
    let bind = || BindMesh { positions: &positions, normals: &normals, uv0: &uv, indices: &indices };
    assert_eq!(renderer::selected_rig_span(&compiled, &selected, &bind()).unwrap(),
        ViewportSpan { first_vertex: 3, vertex_count: 3 });
    compiled.mesh.positions[3][0] += 0.25;
    assert!(renderer::selected_rig_span(&compiled, &selected, &bind()).is_err());
    compiled.mesh.positions[3][0] = positions[0][0];
    compiled.mesh.triangles[0][0] = 3;
    assert!(renderer::selected_rig_span(&compiled, &selected, &bind()).is_err());
    compiled.mesh.triangles[0][0] = 0;
    compiled.mesh.triangles[1].swap(0, 1);
    assert!(renderer::selected_rig_span(&compiled, &selected, &bind()).is_err());
    compiled.mesh.triangles[1].swap(0, 1);
    compiled.normals[3][1] = 1.0;
    assert!(renderer::selected_rig_span(&compiled, &selected, &bind()).is_err());
}
