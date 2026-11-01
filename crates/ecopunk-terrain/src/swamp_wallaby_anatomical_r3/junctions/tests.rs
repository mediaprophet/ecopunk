use super::*;

fn fixture()->Source {
    let mut s=parse_source(DEFAULT_SOURCE).unwrap();
    let mut body=s.lofts[0].clone();
    body.id="torso-fixture".into();body.mirror_joints=None;body.skin_stations=None;body.mirror_skin_stations=None;
    body.radial_segments=16;body.subdivisions=5;body.coat=false;
    body.sections=[-0.4,0.0,0.4].map(|y|Section{center_m:[0.0,y,0.0],radii_m:[0.2,0.2]}).to_vec();
    let mut proximal=body.clone();proximal.id="proximal-fixture".into();proximal.subdivisions=2;
    proximal.sections=[0.1,0.3].map(|x|Section{center_m:[x,0.0,0.0],radii_m:[0.1,0.1]}).to_vec();
    let mut distal=proximal.clone();distal.id="distal-fixture".into();
    distal.sections=[0.4,0.6].map(|x|Section{center_m:[x,0.0,0.0],radii_m:[0.1,0.1]}).to_vec();
    s.lofts=vec![body,proximal,distal];
    s.loft_chains=vec![LoftChain{id:"limb-fixture".into(),members:vec![
        LoftChainMember{loft_id:"proximal-fixture".into(),first_section:0,last_section:1},
        LoftChainMember{loft_id:"distal-fixture".into(),first_section:0,last_section:1},
    ],radial_segments:16,subdivisions:2}];
    s.ellipsoids.clear();s.ears.clear();s.bones.clear();s.muscles.clear();s.coat.enabled=false;
    s.surface_junctions=Some(JunctionSet{cold_bytes:128*1024*1024,work_units:100_000_000,recipes:vec![JunctionRecipe{
        id:"flank-fixture".into(),torso_loft:"torso-fixture".into(),limb_chain:"limb-fixture".into(),mirrored:false,
        body_rows:[3,7],body_column_start:6,body_column_count:4,limb_cut_row:2,rotation:10,subdivisions:8,
        body_tangent_scale:0.8,limb_tangent_scale:0.8,interior_bulge_m:None,material:0,max_influence_loss:0.0,min_boundary_normal_dot:0.0,
    }]});
    s
}

#[test]
fn explicit_graft_closes_selected_body_limb_and_preserves_alias_skin() {
    let s=fixture();let c=build_from_json(&serde_json::to_string(&s).unwrap()).unwrap();
    let receipt=c.surface_junctions.as_ref().unwrap();
    assert_eq!(receipt.records.len(),1);
    let r=&receipt.records[0];
    assert_eq!(r.vertices.len(),9*17);
    assert!(!r.removed_source_faces.is_empty());
    assert!(r.boundary_min_normal_dot>=0.0);
    assert_eq!(r.maximum_discarded_weight,0.0);
    assert!(receipt.work_used>0&&receipt.work_used<s.surface_junctions.as_ref().unwrap().work_units);
    assert!(receipt.component_faces>0);
    for &[vertex,source] in &r.topology_aliases {
        let(v,p)=(vertex as usize,source as usize);
        assert_eq!(c.skin.positions[v],c.skin.positions[p]);
        assert_eq!(c.skin.normals[v],c.skin.normals[p]);
        assert_eq!(c.skin.joint_indices[v],c.skin.joint_indices[p]);
        assert_eq!(c.skin.joint_weights[v],c.skin.joint_weights[p]);
    }
    assert!(c.parts.iter().any(|p|p.source_id=="junction:flank-fixture"));
    assert!(!c.parts.iter().any(|p|p.source_id.contains("proximal-fixture[0]:external-cap")));
}

#[test]
fn invalid_port_perimeter_mirror_and_overlap_refuse() {
    let mut s=fixture();s.surface_junctions.as_mut().unwrap().recipes[0].body_column_count=3;
    assert!(validate(&s).is_err());
    let mut s=fixture();s.surface_junctions.as_mut().unwrap().recipes[0].mirrored=true;
    assert!(validate(&s).is_err());
    let mut s=fixture();let mut duplicate=s.surface_junctions.as_ref().unwrap().recipes[0].clone();duplicate.id="other".into();
    s.surface_junctions.as_mut().unwrap().recipes.push(duplicate);
    assert!(validate(&s).is_err());
    let mut s=fixture();s.surface_junctions.as_mut().unwrap().recipes[0].body_rows[1]=usize::MAX;
    assert!(validate(&s).is_err());
}

#[test]
fn cold_or_work_refusal_does_not_change_previously_generated_streams() {
    let authored=fixture();let mut legacy=authored.clone();legacy.surface_junctions=None;
    let mut c=build_from_json(&serde_json::to_string(&legacy).unwrap()).unwrap();
    let before_positions=c.skin.positions.clone();let before_normals=c.skin.normals.clone();let before_indices=c.skin.indices.clone();
    let before_parts:Vec<_>=c.parts.iter().map(|p|(p.source_id.clone(),p.indices.clone())).collect();
    let mut refused=authored.clone();refused.surface_junctions.as_mut().unwrap().work_units=1;
    assert!(apply(&refused,&mut c.skin,&mut c.parts).is_err());
    assert_eq!(c.skin.positions,before_positions);assert_eq!(c.skin.normals,before_normals);assert_eq!(c.skin.indices,before_indices);
    assert_eq!(c.parts.iter().map(|p|(p.source_id.clone(),p.indices.clone())).collect::<Vec<_>>(),before_parts);
    refused.surface_junctions.as_mut().unwrap().work_units=100_000_000;
    refused.surface_junctions.as_mut().unwrap().cold_bytes=8*1024*1024;
    assert!(apply(&refused,&mut c.skin,&mut c.parts).is_err());
    assert_eq!(c.skin.positions,before_positions);assert_eq!(c.skin.indices,before_indices);
}

#[test]
fn optional_bulge_is_explicit_bounded_and_absent_from_legacy_source(){
    let mut s=fixture();
    let recipe=&s.surface_junctions.as_ref().unwrap().recipes[0];
    let json=serde_json::to_value(recipe).unwrap();assert!(json.get("interior_bulge_m").is_none());
    for bad in [[0.0;3],[f64::NAN,0.0,0.0],[0.2,0.2,0.0],[0.2001,0.0,0.0]]{
        s.surface_junctions.as_mut().unwrap().recipes[0].interior_bulge_m=Some(bad);
        assert!(validate(&s).is_err());
    }
    s.surface_junctions.as_mut().unwrap().recipes[0].interior_bulge_m=Some([-0.06,0.0,0.0]);
    assert!(validate(&s).is_ok());
    let encoded=serde_json::to_string(&s.surface_junctions.as_ref().unwrap().recipes[0]).unwrap();
    let decoded: JunctionRecipe=serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.interior_bulge_m,Some([-0.06,0.0,0.0]));
}

#[test]
fn degree_five_control_adds_only_declared_interior_displacement(){
    use qualia_core_db::specialized_libs::computational_geometry::parametric_cad::bezier_eval_into;
    let p=[[0.1,0.4,0.2],[0.12,0.43,0.21],[0.14,0.3,0.24],[0.15,0.25,0.26]].map(|p|Point3::new(p[0],p[1],p[2]));
    let plain=build::curve_controls(p,None);assert_eq!(plain.len,4);
    for i in 0..4 {assert_eq!([plain.points[i].x,plain.points[i].y,plain.points[i].z],[p[i].x,p[i].y,p[i].z]);}
    let bulge=[0.06,-0.01,0.02];let elevated=build::curve_controls(p,Some(bulge));assert_eq!(elevated.len,6);
    for (a,b,c,d) in [(elevated.points[1],elevated.points[0],p[1],p[0]),(elevated.points[5],elevated.points[4],p[3],p[2])]{
        let delta=[a.x-b.x,a.y-b.y,a.z-b.z];let old=[c.x-d.x,c.y-d.y,c.z-d.z];
        for (delta,old) in delta.into_iter().zip(old){assert!((5.0*delta-3.0*old).abs()<1e-13);}
    }
    for t in [0.0,0.1,0.25,0.5,0.75,0.9,1.0]{
        let old=bezier_eval_into(&plain.points[..4],t,&mut[Point3::default();6]).unwrap();
        let new=bezier_eval_into(&elevated.points,t,&mut[Point3::default();6]).unwrap();
        let amplitude=10.0*t*t*(1.0-t)*(1.0-t);
        for (delta,b) in [new.x-old.x,new.y-old.y,new.z-old.z].into_iter().zip(bulge){
            assert!((delta-amplitude*b).abs()<1e-13);
        }
    }
}
