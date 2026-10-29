//! Unexecuted source regressions for staged general-graph adapters.
use super::{BotanicalLimits, BotanicalRecipe, Organ};
use super::general_source::GeneralBranchSource;
use crate::graphical::general_adapter::{adopted_general_edges_into, SourceEdge};
use vibe::Value;
fn graph() -> BotanicalRecipe {
    let organ = |key: &str, parent: Option<&str>, position_m| Organ {
        key:key.into(), parent:parent.map(str::to_owned), position_m,
    };
    BotanicalRecipe {version:1,part_id:"branch".into(),organs:vec![
        organ("root",None,[0.,0.,-1.]),organ("center",Some("root"),[0.,0.,0.]),
        organ("a",Some("center"),[1.,0.,1.]),organ("b",Some("center"),[-1.,0.,1.]),
        organ("a2",Some("a"),[2.,0.,1.]),organ("b2",Some("b"),[-2.,0.,1.]),
    ],attractors_m:vec![[0.,1.,1.]],node_length_m:0.2,kill_radius_m:0.1,
      perception_radius_m:0.8,cone_angle_rad:2.,max_iterations:1,tip_radius_m:0.1,
      pipe_exponent:2.,ring_slices:8}
}
fn radii() -> [[f64;2];5] {[[0.7,0.6],[0.5,0.2],[0.4,0.25],[0.15,0.07],[0.13,0.05]]}
fn source() -> GeneralBranchSource {
    GeneralBranchSource::from_recipe(&graph(),&radii(),[-4.,-4.,-4.],0.5,[9,9,9],0.1,1e-6,1e-8,0.99,1e-20,
        BotanicalLimits::default(),4096).unwrap()
}
#[test] fn independent_branch_endpoint_radii_roundtrip_and_edit() {
    let prior=source(); let encoded=prior.to_value(BotanicalLimits::default(),4096).unwrap();
    let parsed=GeneralBranchSource::read(&encoded,BotanicalLimits::default(),4096).unwrap();
    assert_eq!(parsed.to_value(BotanicalLimits::default(),4096).unwrap(),encoded);
    let next=prior.with_radius("a",[0.48,0.19],BotanicalLimits::default(),4096).unwrap();
    assert_eq!(next.radii()[1],[0.48,0.19]); assert_eq!(next.radii()[2],radii()[2]);
    assert_eq!(prior.radii(),&radii());
    assert!(prior.with_radius("root",[1.,1.],BotanicalLimits::default(),4096).is_err());
    assert!(prior.with_radius("a",[1.,0.],BotanicalLimits::default(),4096).is_err());
    let mut invalid=encoded; let Value::Record(ref mut r)=invalid else {panic!("record")};
    r.insert("radii_m".into(),Value::List(vec![Value::F64(1.)]));
    assert!(GeneralBranchSource::read(&invalid,BotanicalLimits::default(),4096).is_err());
}
#[test] fn edges_keep_parent_identity_and_user_taper() {
    let g=graph(); let mut parents=[0usize;6]; let mut edges=[SourceEdge::default();5];
    let count=adopted_general_edges_into(&g,&radii(),BotanicalLimits::default(),1e-6,0.99,108,&mut parents,&mut edges).unwrap();
    assert_eq!(count,5); assert_eq!((edges[3].parent,edges[3].child),(2,4));
    assert_eq!((edges[3].field.radius_a,edges[3].field.radius_b),(0.15,0.07));
    assert_ne!(edges[1].field.radius_a,edges[2].field.radius_a);
}
#[test] fn adjacency_budget_refusal_leaves_edge_output_unchanged() {
    let g=graph(); let mut parents=[usize::MAX;6]; let mut edges=[SourceEdge::default();5];
    assert!(adopted_general_edges_into(&g,&radii(),BotanicalLimits::default(),1e-6,0.99,107,&mut parents,&mut edges).is_err());
    assert!(edges.iter().all(|edge| edge.field.radius_a == 0.0));
}
