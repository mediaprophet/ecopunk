use super::*;
fn fixture()->String {
    r#"let mut species_asset = {
        source_version:"white-gum-connected-study-v1",
        botanical:{schema:"poet.botanical/v2",version:2,part_id:"study",organs:[
            {key:"root",parent:null,position_m:[0.0,0.0,0.0]},
            {key:"first",parent:"root",position_m:[0.0,0.0,1.0]},
            {key:"second",parent:"first",position_m:[0.0,0.0,2.0]},
            {key:"tip",parent:"second",position_m:[0.0,0.0,3.0]},
            {key:"a",parent:"first",position_m:[1.0,0.0,1.0]},
            {key:"b",parent:"second",position_m:[-1.0,0.0,2.0]},
            {key:"excluded",parent:"tip",position_m:[0.0,0.0,4.0]}],
            attractors_m:[],node_length_m:0.2,kill_radius_m:0.05,perception_radius_m:0.8,
            cone_angle_rad:2.0,max_iterations:1,tip_radius_m:0.22,pipe_exponent:2.0,ring_slices:8},
        radii_m:[0.22,0.22,0.22,0.22,0.22,0.22,0.1],
        roles:["root_collar","trunk_metamer","trunk_metamer","trunk_metamer","scaffold_branch","scaffold_branch","trunk_metamer"],sprays:[],
        connected_study:{schema:"ecopunk.closed-branch-study/v1",selected_organs:["root","first","second","tip","a","b"],
            origin_m:[-1.43,-0.43,-0.43],spacing_m:0.1,dimensions:[30,10,40],minimum_radius_samples:2.0,
            min_axis_m:0.000001,projection_tolerance_m:0.00000001,min_area_sq_m4:0.000000000000000000000001,
            max_incident_cosine:0.99,uv_axes:[0,2],metres_per_repeat:1.0}};"#.into()
}
#[test]
fn source_profile_reaches_existing_bundle_route_and_failed_edit_does_not_promote(){
    let mut session=crate::species_review_source::SpeciesReviewSourceSession::default();
    let accepted=session.submit(&fixture(),|a|crate::species_authored_review::build(0,a)).unwrap();
    assert_eq!(accepted.render.metadata["id"],"white-gum-connected-study-v1");
    assert_eq!(accepted.render.metadata["detail"]["topology"]["components"],1);
    assert_eq!(accepted.render.metadata["detail"]["excluded_organs"],serde_json::json!(["excluded"]));
    assert!(!accepted.bundle.is_empty());
    let prior=session.sources().to_vec();
    let bad=fixture().replace("let mut species_asset","species_asset").replace("spacing_m:0.1","spacing_m:0.2");
    assert!(session.submit(&bad,|a|crate::species_authored_review::build(0,a)).is_err());
    assert_eq!(session.revision(),1);assert_eq!(session.sources(),prior.as_slice());
    let mut reopened=crate::species_review_source::SpeciesReviewSourceSession::default();
    let replay=reopened.submit(&prior[0],|a|crate::species_authored_review::build(0,a)).unwrap();
    assert_eq!(accepted.render.bytes,replay.render.bytes);
}
#[test]
fn omitted_interior_parent_and_cancellation_refuse(){
    let mut session=crate::species_review_source::SpeciesReviewSourceSession::default();
    let bad=fixture().replace("selected_organs:[\"root\",\"first\",\"second\",\"tip\",\"a\",\"b\"]",
        "selected_organs:[\"root\",\"second\",\"tip\",\"a\",\"b\"]");
    assert!(session.submit(&bad,|a|crate::species_authored_review::build(0,a)).is_err());
    assert_eq!(session.revision(),0);
    assert!(build("",BranchGraphLimits::default(),&||true).err().unwrap().contains("cancelled"));
}
#[test]
fn retained_white_gum_two_junction_selection_uses_authored_axes_and_radii(){
    let mut source:serde_json::Value=serde_json::from_str(include_str!(
        "../../../assets/production-studies/white-gum-review-r1/authoring/white-gum-r2.authored.json")).unwrap();
    source["source_version"]=serde_json::json!("white-gum-connected-study-v1");
    source["connected_study"]=serde_json::json!({"schema":"ecopunk.closed-branch-study/v1",
        "selected_organs":["stem-3","stem-4","stem-5","stem-6","scaffold-0-1","scaffold-1-1"],
        "origin_m":[-0.60,1.35,-0.18],"spacing_m":0.018,"dimensions":[70,96,36],
        "minimum_radius_samples":2.0,"min_axis_m":0.000001,"projection_tolerance_m":0.00000001,
        "min_area_sq_m4":1e-24,"max_incident_cosine":0.99,"uv_axes":[0,1],"metres_per_repeat":0.25});
    let result=build(&source.to_string(),BranchGraphLimits::default(),&||false).unwrap();
    assert_eq!(result.metadata["selected_source_indices"],serde_json::json!([3,4,5,6,47,62]));
    assert_eq!(result.metadata["local_parent_indices"],serde_json::json!([cg::ROOT_PARENT,0,1,2,1,2]));
    assert_eq!(result.metadata["topology"]["components"],1);
    assert_eq!(result.metadata["authored_source"]["botanical"],source["botanical"]);
    assert_eq!(result.metadata["authored_source"]["radii_m"],source["radii_m"]);
    assert_eq!(result.metadata["excluded_organs"].as_array().unwrap().len(),source["botanical"]["organs"].as_array().unwrap().len()-6);
    assert_eq!(result.metadata["excluded_sprays"].as_array().unwrap().len(),source["sprays"].as_array().unwrap().len());
}
#[test]
fn projected_face_loss_and_native_quantization_refuse_before_delivery(){
    assert!(check_face([0.,0.,0.],[0.,0.,1.],0.,"f32 projection",0).is_err());
    assert!(check_face([0.,0.,-1.],[0.,0.,1.],0.,"f32 projection",0).is_err());
    for (points,expected) in [
        ([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],true),
        // Native bbox quantization merges vertex 2 into vertex 0.
        ([[0.,0.,0.],[1.,1.,0.],[0.00000001,0.,0.]],false),
    ] {
        let mut mesh=SubmeshBuilder::new();
        for (i,p) in points.into_iter().enumerate(){mesh.push_vertex(p,[0.,0.,1.],[1.;4],[[0.,0.],[1.,0.],[0.,1.]][i]);}
        mesh.push_tri(0,1,2);let witness=projection_witness(&mesh,1e-24).unwrap();
        let artifact=crate::species_review::compile_review_parts(vec![crate::species_review::ReviewPart{
            name:"projection-check".into(),mesh,roughness:0.7,double_sided:false}],serde_json::json!({"id":"projection-check"})).unwrap();
        assert_eq!(qualify_compiled_projection(&artifact.bytes,&witness).is_ok(),expected);
    }
}
