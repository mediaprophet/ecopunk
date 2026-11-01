use super::*;

fn fixture()->Source {
    let mut s=parse_source(DEFAULT_SOURCE).unwrap();
    s.lofts.retain(|l|["thigh-integument","calf-integument","hind-pes"].contains(&l.id.as_str()));
    s.ellipsoids.clear();s.ears.clear();s.bones.clear();s.muscles.clear();s.coat.enabled=false;
    s.loft_chains=vec![LoftChain{id:"hind-continuous".into(),members:vec![
        LoftChainMember{loft_id:"thigh-integument".into(),first_section:0,last_section:3},
        LoftChainMember{loft_id:"calf-integument".into(),first_section:1,last_section:3},
        LoftChainMember{loft_id:"hind-pes".into(),first_section:1,last_section:5},
    ],radial_segments:16,subdivisions:3}];
    s
}

#[test]
fn serial_limb_has_only_external_caps_and_exact_shared_uv_seam() {
    let s=fixture();let input=serde_json::to_string(&s).unwrap();
    let candidate=build_from_json(&input).unwrap();
    let rows=(12-1)*3+1;let stride=17;let per_side=rows*stride+2;
    assert_eq!(candidate.skin.positions.len(),per_side*2);
    assert_eq!(candidate.skin.indices.len(),rows*16*6*2);
    assert_eq!(candidate.parts.iter().filter(|p|p.source_id.ends_with("external-cap")).count(),4);
    let mut ownership=vec![0u8;candidate.skin.indices.len()];
    for part in &candidate.parts {
        assert!(part.source_id.starts_with("chain:hind-continuous:"));
        for n in part.indices.clone(){ownership[n]+=1;assert!(part.vertices.contains(&(candidate.skin.indices[n]as usize)));}
    }
    assert!(ownership.iter().all(|&n|n==1));
    for side in 0..2 {for row in 0..rows {
        let first=side*per_side+row*stride;let last=first+16;
        assert_eq!(candidate.skin.positions[first],candidate.skin.positions[last]);
        assert_eq!(candidate.skin.normals[first],candidate.skin.normals[last]);
        assert_eq!(candidate.skin.joint_weights[first],candidate.skin.joint_weights[last]);
        assert_eq!(candidate.skin.uvs[first][0],0.0);assert_eq!(candidate.skin.uvs[last][0],1.0);
    }}
    // Exact original source records survive the derived topology operation.
    assert_eq!(serde_json::to_value(&candidate.source.lofts).unwrap(),serde_json::to_value(&s.lofts).unwrap());
    assert_eq!(candidate.skin.joint_indices[0][0],9);
    assert_eq!(candidate.skin.joint_indices[per_side][0],6);
    assert_eq!(candidate.skin.joint_indices[rows*stride+1][0],11);
    assert_eq!(candidate.skin.joint_indices[per_side+rows*stride+1][0],8);
}

#[test]
fn overlapping_missing_mirror_and_coincident_inputs_refuse() {
    let mut s=fixture();s.loft_chains.push(s.loft_chains[0].clone());s.loft_chains[1].id="other".into();
    assert!(validate(&s).is_err());
    let mut s=fixture();s.loft_chains[0].members[1].loft_id="missing".into();assert!(validate(&s).is_err());
    let mut s=fixture();s.lofts[1].mirror_joints=None;assert!(validate(&s).is_err());
    let mut s=fixture();s.lofts[1].sections[1].center_m=s.lofts[0].sections[3].center_m;
    assert!(validate(&s).is_err());
    let mut s=fixture();s.loft_chains[0].members[0].first_section=1;assert!(validate(&s).is_err());
}

#[test]
fn absent_recipe_preserves_legacy_output_and_serialization() {
    let mut s=fixture();s.loft_chains.clear();let text=serde_json::to_string(&s).unwrap();
    assert!(!text.contains("loft_chains"));
    let mut explicit=serde_json::to_value(&s).unwrap();explicit["loft_chains"]=serde_json::json!([]);
    let a=build_from_json(&text).unwrap();let b=build_from_json(&explicit.to_string()).unwrap();
    assert_eq!(a.skin.positions,b.skin.positions);assert_eq!(a.skin.normals,b.skin.normals);
    assert_eq!(a.skin.indices,b.skin.indices);assert_eq!(a.skin.uvs,b.skin.uvs);
    assert_eq!(a.skin.joint_indices,b.skin.joint_indices);assert_eq!(a.skin.joint_weights,b.skin.joint_weights);
}

#[test]
fn qualification_work_exhaustion_refuses_detached_candidate() {
    let s=fixture();let (loft,_)=combined(&s,&s.loft_chains[0]).unwrap();
    let rings=sampled_sections(&loft,&s.palettes[loft.material],1.0,loft.joints).unwrap();
    let mut a=AnatomyMeshAccumulator::new();a.push_countershaded_loft(&rings,16,Layer::ExternalIntegument,true);
    let mut work=0;
    assert!(qualification::check(&a,0,0,rings.len(),16,&mut work).is_err());
}

#[test]
fn owner_winding_is_independent_of_provisional_shading_normals() {
    let s=fixture();let (loft,_)=combined(&s,&s.loft_chains[0]).unwrap();
    let mut rings=sampled_sections(&loft,&s.palettes[loft.material],1.0,loft.joints).unwrap();
    rings.truncate(3);
    for (row,r) in rings.iter_mut().enumerate(){r.center=[0.0,row as f32*0.1,0.0];r.radii=[0.025,0.03];}
    let make=||{
        let mut a=AnatomyMeshAccumulator::new();
        a.push_countershaded_loft(&rings,16,Layer::ExternalIntegument,true);
        apply_bishop_sections(&mut a,&rings,16,0).unwrap();
        for row in 0..3{a.positions[row*17+16]=a.positions[row*17];}
        a
    };
    let mut a=make();let mut b=make();
    // A shading/reference change must never reverse isolated topological faces.
    for (i,n) in b.normals.iter_mut().enumerate(){if i%3!=0{*n=mul(*n,-1.0);}}
    build::finish_chain_surface(&mut a,0,0,3,16).unwrap();
    build::finish_chain_surface(&mut b,0,0,3,16).unwrap();
    assert_eq!(a.indices,b.indices);assert_eq!(a.normals,b.normals);
    let mut work=MAX_TOPOLOGY_WORK;
    qualification::check(&a,0,0,3,16,&mut work).unwrap();
    let volume6=a.indices.chunks_exact(3).map(|t|{
        let p=a.positions[t[0]as usize];let q=a.positions[t[1]as usize];let r=a.positions[t[2]as usize];
        f64::from(dot(p,cross(q,r)))
    }).sum::<f64>();
    assert!(volume6>0.0,"Closed owner winding must face outward");
    assert!(a.normals.iter().all(|n|finite(n)&&(dot(*n,*n)-1.0).abs()<1e-5));
}
