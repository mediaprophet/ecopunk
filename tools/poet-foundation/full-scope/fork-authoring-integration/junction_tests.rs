//! Source tests staged for later execution; no test/build was run for this file.
use super::{BotanicalLimits, BotanicalRecipe, JunctionSource, Organ};
use super::junction_edit;
use crate::graphical::geometry_host::GeometryHost;
use vibe::Value;
use std::collections::BTreeMap;

fn recipe() -> BotanicalRecipe {
    BotanicalRecipe { version: 1, part_id: "branch".into(), organs: vec![
        Organ {key:"root".into(),parent:None,position_m:[0.0,0.0,-1.0]},
        Organ {key:"center".into(),parent:Some("root".into()),position_m:[0.0,0.0,0.0]},
        Organ {key:"a".into(),parent:Some("center".into()),position_m:[1.0,0.0,1.0]},
        Organ {key:"b".into(),parent:Some("center".into()),position_m:[-1.0,0.0,1.0]},
    ], attractors_m:vec![[0.0,1.0,1.0]], node_length_m:0.2, kill_radius_m:0.05,
       perception_radius_m:0.8, cone_angle_rad:2.0, max_iterations:1,
       tip_radius_m:0.35, pipe_exponent:2.0, ring_slices:8 }
}
fn source() -> JunctionSource {
    JunctionSource::from_recipe(&recipe(),[-2.03,-2.02,-2.04],0.5,[9,9,9],0.2,1e-6,1e-8,0.99,1e-20,
        BotanicalLimits::default(),4096).unwrap()
}
fn parts(geometry: Value) -> Value {
    Value::List(vec![Value::Record(BTreeMap::from([
        ("id".into(),Value::String("branch".into())),("geometry".into(),geometry),
        ("marker".into(),Value::String("retained".into())),
    ]))])
}
fn geometry(vertices: Value, triangles: Value) -> Value {
    Value::Record(BTreeMap::from([("vertices".into(),vertices),("triangles".into(),triangles)]))
}
fn blank() -> Value { geometry(Value::List(vec![]),Value::List(vec![])) }
fn host() -> GeometryHost { GeometryHost::new([
    "capability.invoke", "ComputationalGeometry.space_colonize_3d",
    "ComputationalGeometry.pipe_model_radii", "ComputationalGeometry.extrude_profile",
].iter().map(|s| (*s).to_owned()).collect(),200_000) }
#[test]
fn source_value_roundtrip_and_position_edit_are_isolated() {
    let original=source(); let value=original.to_value(BotanicalLimits::default(),4096).unwrap();
    let parsed=JunctionSource::read(&value,BotanicalLimits::default(),4096).unwrap();
    assert_eq!(parsed.to_value(BotanicalLimits::default(),4096).unwrap(), value);
    let changed=parsed.with_position("a",[1.1,0.0,1.0],BotanicalLimits::default(),4096).unwrap();
    assert_ne!(changed.to_value(BotanicalLimits::default(),4096).unwrap(),value);
    assert_eq!(original.to_value(BotanicalLimits::default(),4096).unwrap(),value);
    assert!(vibe::parse_program(&format!("let mut x = {};",changed.source_expression(BotanicalLimits::default(),4096).unwrap())).is_ok());
}
#[test]
fn incompatible_numeric_value_variants_refuse_instead_of_normalizing() {
    let value=source().to_value(BotanicalLimits::default(),4096).unwrap();
    let mut altered=value.clone();
    let Value::Record(record)=&mut altered else {panic!("record")};
    let Some(Value::Record(graph))=record.get_mut("graph") else {panic!("graph")};
    graph.insert("version".into(),Value::U64(1));
    assert!(JunctionSource::read(&altered,BotanicalLimits::default(),4096).is_err());
    let mut altered=value.clone(); let Value::Record(record)=&mut altered else {panic!("record")};
    record.insert("spacing_m".into(),Value::I64(1));
    assert!(JunctionSource::read(&altered,BotanicalLimits::default(),4096).is_err());
    let mut altered=value; let Value::Record(record)=&mut altered else {panic!("record")};
    record.insert("dimensions".into(),Value::List(vec![Value::U64(9);3]));
    assert!(JunctionSource::read(&altered,BotanicalLimits::default(),4096).is_err());
}
#[test]
fn settings_update_keeps_accepted_source_and_other_graph_semantics() {
    let before=source(); let graph=before.graph(BotanicalLimits::default()).unwrap();
    let settings=super::JunctionSettings {tip_radius_m:0.4,origin_m:before.origin_m,spacing_m:before.spacing_m,
        dimensions:before.dimensions,blend_m:0.25,min_axis_m:before.min_axis_m,
        projection_tolerance_m:before.projection_tolerance_m,max_same_direction_cosine:before.max_same_direction_cosine,
        min_area_sq_m4:before.min_area_sq_m4};
    let after=before.with_settings(&settings,BotanicalLimits::default(),4096).unwrap();
    let next_graph=after.graph(BotanicalLimits::default()).unwrap();
    assert_eq!(graph.tip_radius_m,0.35); assert_eq!(next_graph.tip_radius_m,0.4);
    assert_eq!(graph.organs,next_graph.organs); assert_eq!(graph.attractors_m,next_graph.attractors_m);
    assert_eq!(before.blend_m,0.2); assert_eq!(after.blend_m,0.25);
    let mut invalid=settings.clone(); invalid.spacing_m=0.0;
    assert!(before.with_settings(&invalid,BotanicalLimits::default(),4096).is_err());
    assert_eq!(before.graph(BotanicalLimits::default()).unwrap(),graph);
}
#[test]
fn unknown_geometry_streams_refuse_without_touching_input() {
    for stream in ["uv0","normal","skin","unknown_extension"] {
        let mut geometry=match blank() { Value::Record(g)=>g,_=>unreachable!() };
        geometry.insert(stream.into(),Value::List(vec![]));
        let input=parts(Value::Record(geometry)); let before=input.clone();
        let mut host=host(); let limits=host.junction_limits().unwrap();
        assert!(host.begin_junction_operation().unwrap());
        assert!(junction_edit::prepare_adoption(&input,&source(),limits,&mut host).is_err());
        host.end_junction_operation(); assert_eq!(input,before);
    }
}
#[test]
fn adoption_generates_source_and_drifted_prior_mesh_is_refused() {
    let input=parts(blank()); let before=input.clone();
    let source=source(); let value=source.to_value(BotanicalLimits::default(),4096).unwrap();
    let mut h=host(); let limits=h.junction_limits().unwrap(); assert!(h.begin_junction_operation().unwrap());
    let cell=junction_edit::prepare_adoption(&input,&source,limits,&mut h).unwrap();
    assert!(vibe::parse_program(&cell).is_ok()); h.end_junction_operation(); assert_eq!(input,before);
    let drifted=parts(geometry(Value::List(vec![Value::List(vec![Value::F64(9.0);3])]),Value::List(vec![])));
    let before=drifted.clone(); assert!(h.begin_junction_operation().unwrap());
    assert!(junction_edit::prepare_position_edit(&drifted,&value,"branch","a",[1.1,0.0,1.0],limits,&mut h).is_err());
    h.end_junction_operation(); assert_eq!(drifted,before);
}
#[test]
fn adopted_source_and_followup_edit_roundtrip_through_vibe_session() {
    use vibe::{session::SourceSession, Budget, LocalHost};
    let mut session=SourceSession::default(); let mut runtime=LocalHost::default();
    session.submit(r#"let mut poet_parts=[{id:"branch",geometry:{vertices:[],triangles:[]},marker:"retained"}];"#, &mut runtime,Budget::default()).unwrap();
    let mut geometry_host=host(); let limits=geometry_host.junction_limits().unwrap();
    assert!(geometry_host.begin_junction_operation().unwrap());
    let cell=junction_edit::prepare_adoption(session.value("poet_parts").unwrap(),&source(),limits,&mut geometry_host).unwrap();
    geometry_host.end_junction_operation();
    session.submit(&cell,&mut runtime,Budget::default()).unwrap();
    let before=session.value("poet_botanical_junction").unwrap().clone();
    assert!(geometry_host.begin_junction_operation().unwrap());
    let cell=junction_edit::prepare_position_edit(session.value("poet_parts").unwrap(),&before,"branch","a",[1.1,0.0,1.0],limits,&mut geometry_host).unwrap();
    geometry_host.end_junction_operation();
    session.submit(&cell,&mut runtime,Budget::default()).unwrap();
    let after=session.value("poet_botanical_junction").unwrap();
    assert_ne!(after,&before);
    let parsed=JunctionSource::read(after,limits.botanical,limits.max_samples).unwrap();
    assert_eq!(parsed.graph(limits.botanical).unwrap().organs[2].position_m,[1.1,0.0,1.0]);
    let settings=super::JunctionSettings {tip_radius_m:0.4,origin_m:parsed.origin_m,spacing_m:parsed.spacing_m,
        dimensions:parsed.dimensions,blend_m:parsed.blend_m,min_axis_m:parsed.min_axis_m,
        projection_tolerance_m:parsed.projection_tolerance_m,max_same_direction_cosine:parsed.max_same_direction_cosine,
        min_area_sq_m4:parsed.min_area_sq_m4};
    let before_settings=after.clone();
    assert!(geometry_host.begin_junction_operation().unwrap());
    let cell=junction_edit::prepare_settings_edit(session.value("poet_parts").unwrap(),&before_settings,"branch",&settings,limits,&mut geometry_host).unwrap();
    geometry_host.end_junction_operation();
    session.submit(&cell,&mut runtime,Budget::default()).unwrap();
    let parsed=JunctionSource::read(session.value("poet_botanical_junction").unwrap(),limits.botanical,limits.max_samples);    assert_eq!(parsed.unwrap().graph(limits.botanical).unwrap().tip_radius_m,0.4);

}
#[test]
fn classifier_distinguishes_unrelated_binding() {
    use crate::graphical::authoring::junction_session::is_junction_binding;
    let known=source().to_value(BotanicalLimits::default(),4096).unwrap();
    assert!(is_junction_binding(&known)); assert!(!is_junction_binding(&Value::Null));
    let other=Value::Record(BTreeMap::from([("schema".into(),Value::String("other".into()))]));
    assert!(!is_junction_binding(&other));
}

#[test]
fn junction_profile_refuses_insufficient_replay_allowance() {
    use crate::graphical::authoring::junction_session::require_replay_limits;
    let mut limits=crate::graphical::execution::RebuildLimits::default();
    limits.total_geometry_work=63_999_999; limits.total_kernel_dispatches=256;
    assert!(require_replay_limits(&limits).is_err());
    limits.total_geometry_work=64_000_000; limits.total_kernel_dispatches=255;
    assert!(require_replay_limits(&limits).is_err());
    limits.total_kernel_dispatches=256;
    assert!(require_replay_limits(&limits).is_ok());
}