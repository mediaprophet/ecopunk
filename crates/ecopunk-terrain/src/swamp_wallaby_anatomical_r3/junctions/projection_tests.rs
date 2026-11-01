use super::*;
use crate::{scene_builder_10d::SubmeshBuilder,species_review::compile_review_parts};

fn encoded(second_normal:[f32;3],collapse:bool)->Vec<u8>{
    let mut mesh=SubmeshBuilder::new();
    mesh.positions=if collapse {vec![[0.,0.,0.],[0.0001,0.,0.],[0.,0.0001,0.],[100.,0.,0.],[100.,100.,0.],[0.,100.,0.]]}
        else {vec![[0.,0.,0.],[1.,0.,0.],[0.,1.,0.],[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]]};
    mesh.normals=vec![[0.,0.,1.];6];mesh.normals[3..].fill(second_normal);
    mesh.colors=vec![[0.5,0.4,0.3,1.];6];
    mesh.uvs=vec![[0.,0.],[1.,0.],[0.,1.],[0.,0.],[1.,0.],[0.,1.]];
    mesh.indices=vec![0,1,2,3,4,5];
    compile_review_parts(vec![ReviewPart{name:"graft-projection-fixture".into(),mesh,roughness:0.8,double_sided:false}],
        serde_json::json!({"id":"graft-projection-fixture"})).unwrap().bytes
}
fn witness(collapse:bool)->JunctionProjection{
    JunctionProjection{vertices:6,triangles:2,faces:vec![(0,[0,1,2],[0.,0.,if collapse{1e-8}else{1.}])],
        aliases:if collapse{vec![]}else{vec![[0,3],[1,4],[2,5]]},boundaries:vec![],remaining:1_000_000,
        initial:1_000_000,source_work:0,reserved:64*1024*1024,changed:1}
}
#[test]
fn actual_canonical_attachment_normals_must_match(){
    qualify_junction_projection(&encoded([0.,0.,1.],false),witness(false)).unwrap();
    let error=qualify_junction_projection(&encoded([0.,1.,0.],false),witness(false)).err().unwrap();
    assert!(error.contains("position/normal aliases"),"{error}");
}
#[test]
fn actual_native_quantization_collapse_is_not_success(){
    let error=qualify_junction_projection(&encoded([0.,0.,1.],true),witness(true)).err().unwrap();
    assert!(error.contains("collapsed/reversed"),"{error}");
}
