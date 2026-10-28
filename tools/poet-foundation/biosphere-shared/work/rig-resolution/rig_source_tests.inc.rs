// Include inside rig_binding::tests; requires coordinator registration of
// graphical::authoring::rig_source and rig_binding::rig_candidate.
#[test]
fn rig_source_candidate_evaluates_and_refuses_unsupported_contexts() {
    use crate::graphical::authoring::rig_source;
    use vibe::{session::SourceSession, Budget, LocalHost, Value};
    let f = Fixture::new();
    let descriptor_json = descriptor_source_literal(&f.descriptor).unwrap();
    let encoded = serde_json::to_string(&descriptor_json).unwrap();
    let mut session = SourceSession::default();
    let initial = format!("let mut poet_parts = []; let mut poet_rig = {encoded}; ({{parts:poet_parts}});");
    session.submit(&initial, &mut LocalHost::default(), Budget::default()).unwrap();
    let mesh = BindMesh { positions: &f.positions, normals: &f.normals, uv0: &f.uv, indices: &f.indices };
    let candidate = super::rig_candidate::stage_rig_control(
        f.source.clone(), f.descriptor.clone(), &f.descriptor.resource_key, &f.bytes,
        mesh, ViewportSpan { first_vertex: 4, vertex_count: 3 }, 42 * 1024 * 1024,
        RigControl::Seek { seconds: 1.0 },
    ).unwrap();
    let before = session.value("poet_rig").unwrap().clone();
    assert!(rig_source::edit_source(None, true, &f.source, &candidate).is_err());
    assert!(rig_source::edit_source(Some(&before), false, &f.source, &candidate).is_err());
    assert!(rig_source::edit_source(Some(&Value::Bool(true)), true, &f.source, &candidate).is_err());
    let mut changed = f.descriptor.clone();
    changed.seek_seconds = 0.5;
    let changed = Value::String(descriptor_source_literal(&changed).unwrap());
    assert!(rig_source::edit_source(Some(&changed), true, &f.source, &candidate).is_err());
    assert_eq!(session.value("poet_rig"), Some(&before));
    let cell = rig_source::edit_source(Some(&before), true, &f.source, &candidate).unwrap();
    session.submit(&cell, &mut LocalHost::default(), Budget::default())
        .unwrap_or_else(|e| panic!("rig source cell failed: {e:?}\n{cell}"));
    assert_eq!(rig_source::read_binding(session.value("poet_rig"), &f.source).unwrap(), Some(candidate.after().clone()));
    assert_eq!(session.value("poet_parts"), Some(&Value::List(vec![])));
}
