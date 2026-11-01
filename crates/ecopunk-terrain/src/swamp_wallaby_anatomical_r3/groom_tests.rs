use super::*;

fn source() -> Source {
    let mut s=parse_source(DEFAULT_SOURCE).unwrap();s.coat.enabled=false;
    s.groom=Some(Groom{seed:191,density_per_m2:120.0,max_strands:2000,segments:3,
        length_m:[0.006,0.012],width_m:[0.0003,0.0007],lift_m:0.002,
        direction:[0.1,-0.9,-0.2],guard_fraction:0.2,root_tone:0.6,tip_tone:1.3,
        max_influence_loss:0.02,material:0,surface_materials:vec![0],regions:vec![],
        cold_bytes:64*1024*1024,work_units:30_000_000});s
}

#[test]
fn real_groom_is_repeatable_and_preserves_bound_skin_weights() {
    let s=source();let text=serde_json::to_string(&s).unwrap();
    let a=build_from_json(&text).unwrap();let b=build_from_json(&text).unwrap();
    let receipt=a.groom.as_ref().unwrap();assert!(receipt.strands>0);
    assert_eq!(receipt.vertices,receipt.strands*7);
    assert_eq!(receipt.triangles,receipt.strands*5);
    assert_eq!(a.skin.positions,b.skin.positions);assert_eq!(a.skin.indices,b.skin.indices);
    assert_eq!(a.skin.joint_indices,b.skin.joint_indices);assert_eq!(a.skin.joint_weights,b.skin.joint_weights);
    let p=a.parts.last().unwrap();assert!(p.source_id.starts_with("area-curved-groom:"));
    for i in p.vertices.clone(){
        assert!((a.skin.joint_weights[i].iter().sum::<f32>()-1.0).abs()<1e-5);
        assert!(a.skin.positions[i].iter().all(|x|x.is_finite()));
        assert!(a.skin.joint_indices[i].iter().zip(a.skin.joint_weights[i]).all(|(&j,w)|w==0.0||usize::from(j)<s.joints.len()));
    }
}

#[test]
fn density_and_operation_budgets_refuse_instead_of_silently_thinning() {
    let mut s=source();s.groom.as_mut().unwrap().max_strands=1;
    assert!(build_from_json(&serde_json::to_string(&s).unwrap()).is_err());
    let mut s=source();s.groom.as_mut().unwrap().work_units=1;
    assert!(build_from_json(&serde_json::to_string(&s).unwrap()).is_err());
    let mut s=source();s.coat.enabled=true;
    assert!(parse_source(&serde_json::to_string(&s).unwrap()).is_err());
}

#[test]
fn inherited_influence_loss_is_measured_and_never_hidden() {
    let weights=[SkinWeight{joints:[0,1,0,0],weights:[0.5,0.5,0.0,0.0]},
        SkinWeight{joints:[2,3,0,0],weights:[0.5,0.5,0.0,0.0]},
        SkinWeight{joints:[4,5,0,0],weights:[0.5,0.5,0.0,0.0]}];
    assert!(skin_profile::blend_weights(&weights,&[0.25,0.25,0.5],0.05).is_err());
    let weights=[weights[0],weights[0],weights[1]];
    let (weight,loss)=skin_profile::blend_weights(&weights,&[0.25,0.25,0.5],0.0).unwrap();
    assert_eq!(loss,0.0);assert_eq!(weight.joints,[0,1,2,3]);assert_eq!(weight.weights,[0.25;4]);
}

#[test]
fn tiny_fifth_influence_is_not_cancelled_or_waived_by_zero_loss_policy(){
    let weights=[SkinWeight{joints:[0,1,2,3],weights:[0.25;4]},
        SkinWeight{joints:[4,0,0,0],weights:[1.0,0.0,0.0,0.0]}];
    let factors=[1.0-1e-16,1e-16];
    assert!(skin_profile::blend_weights(&weights,&factors,0.0).is_err());
    let (_,loss)=skin_profile::blend_weights(&weights,&factors,2e-16).unwrap();
    assert!(loss>0.0 && loss<=2e-16);
}

fn single_triangle()->(AnatomyMeshAccumulator,Vec<Part>){
    let mut a=AnatomyMeshAccumulator::new();
    for p in [[0.,0.,0.],[1.,0.,0.],[0.,0.,1.]] {
        push_vertex(&mut a,p,[0.,1.,0.],[0.5,0.4,0.3,1.],[0.,0.],0);
    }
    a.indices=vec![0,1,2];
    let parts=vec![Part{source_id:"density-crossing".into(),material:0,vertices:0..3,indices:0..3}];
    (a,parts)
}

#[test]
fn point_region_zero_density_excludes_roots_even_when_centroid_is_outside(){
    let mut s=source();let g=s.groom.as_mut().unwrap();
    g.density_per_m2=8000.0;g.max_strands=5000;
    g.regions=vec![GroomRegion{id:"empty-right".into(),surface_materials:vec![0],
        bounds_min_m:[0.4,-1.,-1.],bounds_max_m:[2.,1.,2.],direction:[0.,0.,1.],
        density_scale:0.,length_scale:1.,width_scale:1.}];
    let (mut a,mut parts)=single_triangle();
    let receipt=append(&s,&mut a,&mut parts).unwrap().unwrap();
    assert!(receipt.strands>0 && receipt.strands<4000);
    assert_eq!(receipt.region_strands[1].1,0);
    let output=parts.last().unwrap();
    for first in (output.vertices.start..output.vertices.end).step_by(7){
        let root_x=(a.positions[first][0]+a.positions[first+1][0])*0.5;
        assert!(root_x<=0.400001,"root in excluded region: {root_x}");
    }
}

#[test]
fn retained_input_capacity_is_reserved_before_any_groom_vertex(){
    let mut s=source();s.groom.as_mut().unwrap().cold_bytes=8*1024*1024;
    let (mut a,mut parts)=single_triangle();a.positions.reserve_exact(500_000);
    let before=a.positions.clone();let indices=a.indices.clone();
    let error=append(&s,&mut a,&mut parts).err().unwrap();
    assert!(error.contains("old/new buffers"),"{error}");
    assert_eq!(a.positions,before);assert_eq!(a.indices,indices);assert_eq!(parts.len(),1);
}

#[test]
fn retained_part_slots_have_portable_conservative_accounting(){
    let a=AnatomyMeshAccumulator::new();
    let empty=Vec::<Part>::new();
    let parts=Vec::<Part>::with_capacity(99);
    let slots=parts.capacity();
    assert_eq!(part_slot_reservation(slots).unwrap(),slots*64);
    assert!(part_slot_reservation(slots).unwrap()>=slots*std::mem::size_of::<Part>());
    assert_eq!(input_reservation(&a,&parts).unwrap()-input_reservation(&a,&empty).unwrap(),slots*128);
    assert!(part_slot_reservation(usize::MAX/64+1).is_err());

    // The real append path must still reject a one-byte-short cold budget
    // before promotion; changing accounting must not waive its resource gate.
    let s=source();let (mut mesh,mut owned_parts)=single_triangle();
    let required=output_reservation(s.groom.as_ref().unwrap()).unwrap()
        +input_reservation(&mesh,&owned_parts).unwrap();
    let mut short=s.clone();short.groom.as_mut().unwrap().cold_bytes=required-1;
    let before=mesh.positions.clone();let indices=mesh.indices.clone();
    assert!(append(&short,&mut mesh,&mut owned_parts).err().unwrap().contains("old/new buffers"));
    assert_eq!(mesh.positions,before);assert_eq!(mesh.indices,indices);assert_eq!(owned_parts.len(),1);
}

#[test]
fn native_curve_scratch_refusal_preserves_output_and_known_cubic_shape() {
    use qualia_core_db::specialized_libs::computational_geometry::parametric_cad::CadError;
    let cp=[[0.,0.,0.],[0.,1.,0.],[1.,1.,0.],[1.,0.,0.]].map(|p|Point3{x:p[0],y:p[1],z:p[2]});
    let mut scratch=[Point3::default();4];let p=bezier_eval_into(&cp,0.5,&mut scratch).unwrap();
    assert_eq!([p.x,p.y,p.z],[0.5,0.75,0.0]);
    let before=scratch;
    assert!(matches!(bezier_eval_into(&cp,0.5,&mut scratch[..3]),Err(CadError::OutputTooSmall{required:4})));
    assert!(scratch.iter().zip(before).all(|(a,b)|[a.x,a.y,a.z]==[b.x,b.y,b.z]));
}
