use super::*;
use crate::swamp_wallaby_anatomical_r3::{build_from_json,parse_source,DEFAULT_SOURCE};
use qualia_core_db::render::skeletal::{authoring::remap_skin_weights_into,skin_vertices_into,Motor};
use serde_json::{json,Value};

fn source()->Value {
    let mut source:Value=serde_json::from_str(DEFAULT_SOURCE).unwrap();
    source["lofts"]=json!([source["lofts"][0].clone()]);
    for name in ["ellipsoids","ears","bones","muscles"] {source[name]=json!([]);}
    source["coat"]["enabled"]=json!(false);
    source
}
fn profile(source:&Value)->Value {
    let count=source["lofts"][0]["sections"].as_array().unwrap().len();
    assert_eq!(count,9);
    json!([0,0,0,1,2,2,3,3,3].map(|joint|
        json!({"joints":[joint,joint],"weights":[1.0,0.0]})))
}
fn text(source:&Value)->String {serde_json::to_string(source).unwrap()}

#[test]
fn authored_stations_change_only_skin_streams_and_keep_caps_and_uv_aliases() {
    let mut definition=source();
    let legacy=build_from_json(&text(&definition)).unwrap();
    definition["lofts"][0]["skin_stations"]=profile(&definition);
    let shaped=build_from_json(&text(&definition)).unwrap();
    assert_eq!(legacy.skin.positions,shaped.skin.positions);
    assert_eq!(legacy.skin.normals,shaped.skin.normals);
    assert_eq!(legacy.skin.colors,shaped.skin.colors);
    assert_eq!(legacy.skin.uvs,shaped.skin.uvs);
    assert_eq!(legacy.skin.indices,shaped.skin.indices);
    let loft=&shaped.source.lofts[0];
    let stride=loft.radial_segments+1;
    let rows=(loft.sections.len()-1)*loft.subdivisions+1;
    let middle=3*loft.subdivisions*stride;
    assert_eq!(shaped.skin.joint_indices[middle],[1,0,0,0]);
    assert_eq!(shaped.skin.joint_weights[middle],[1.0,0.0,0.0,0.0]);
    assert_eq!(shaped.skin.joint_indices[rows*stride],[0;4]);
    assert_eq!(shaped.skin.joint_indices[rows*stride+1],[3,0,0,0]);
    for row in 0..rows {
        let a=row*stride; let b=a+loft.radial_segments;
        assert_eq!(shaped.skin.joint_indices[a],shaped.skin.joint_indices[b]);
        assert_eq!(shaped.skin.joint_weights[a],shaped.skin.joint_weights[b]);
        // The absent-field legacy path retains its actual old whole-loft formula.
        let f=if row+1==rows {1.0} else {
            ((row/loft.subdivisions) as f32+(row%loft.subdivisions) as f32/loft.subdivisions as f32)
                /(loft.sections.len()-1) as f32
        };
        assert_eq!(legacy.skin.joint_indices[a],[0,3,0,0]);
        let primary=1.0-f;
        assert_eq!(legacy.skin.joint_weights[a],[primary,1.0-primary,0.0,0.0]);
    }
}

#[test]
fn four_authored_influences_survive_core_remap_and_actual_skinning() {
    let a=SkinStation{joints:[0,1],weights:[0.5,0.5]};
    let b=SkinStation{joints:[2,3],weights:[0.5,0.5]};
    let blended=blend(&a,&b,0.5);
    assert_eq!(blended.joints,[0,1,2,3]);
    assert_eq!(blended.weights,[0.25;4]);
    let endpoints=[blend(&a,&a,0.),blended];
    let mut output=[blended;3];
    remap_skin_weights_into(&endpoints,&[1,0,1],&mut output).unwrap();
    let palette=std::array::from_fn::<_,4,_>(|i|
        Motor::from_rotation_translation([1.,0.,0.,0.],[i as f64+1.,0.,0.]));
    let mut positions=[[0.;3];3]; let mut normals=[[0.;3];3];
    skin_vertices_into(&[[0.;3];3],&[[0.,1.,0.];3],&output,&palette,&mut positions,&mut normals).unwrap();
    assert_eq!(positions,[[2.5,0.,0.],[1.5,0.,0.],[2.5,0.,0.]]);
    assert_eq!(normals,[[0.,1.,0.];3]);
    let duplicates=blend(&SkinStation{joints:[2,1],weights:[0.5,0.5]},
        &SkinStation{joints:[1,2],weights:[0.25,0.75]},0.5);
    assert_eq!(duplicates.joints,[1,2,0,0]);
    assert_eq!(duplicates.weights,[0.375,0.625,0.,0.]);
}

#[test]
fn directional_coat_inherits_intermediate_joint_influences() {
    let mut definition=source();
    definition["lofts"][0]["skin_stations"]=profile(&definition);
    definition["coat"]["enabled"]=json!(true);
    let candidate=build_from_json(&text(&definition)).unwrap();
    let coat=candidate.parts.iter().find(|p|p.source_id=="directional-short-coat-study").unwrap();
    assert!(!coat.vertices.is_empty());
    assert!(coat.vertices.clone().any(|v|candidate.skin.joint_indices[v].iter()
        .zip(candidate.skin.joint_weights[v]).any(|(&j,w)|(j==1||j==2)&&w>0.)));
    for base in coat.vertices.clone().step_by(5) {
        for corner in 1..5 {
            assert_eq!(candidate.skin.joint_indices[base],candidate.skin.joint_indices[base+corner]);
            assert_eq!(candidate.skin.joint_weights[base],candidate.skin.joint_weights[base+corner]);
        }
    }
}

#[test]
fn mirror_is_explicit_and_invalid_profiles_refuse_source() {
    let mut definition=source();
    let good=profile(&definition);
    definition["lofts"][0]["skin_stations"]=good.clone();
    definition["lofts"][0]["mirror_joints"]=json!([9,11]);
    assert!(parse_source(&text(&definition)).is_err());
    definition["lofts"][0]["mirror_skin_stations"]=json!((0..9)
        .map(|_|json!({"joints":[9,11],"weights":[1.,0.]})).collect::<Vec<_>>());
    let candidate=build_from_json(&text(&definition)).unwrap();
    let mirrored=&candidate.parts[1];
    assert!(mirrored.vertices.clone().all(|v|candidate.skin.joint_indices[v]==[9,0,0,0]));
    for invalid in [json!([]),json!([{"joints":[0,1],"weights":[0.,0.]}]),
        json!((0..9).map(|_|json!({"joints":[0,999],"weights":[1.,0.]})).collect::<Vec<_>>())] {
        definition["lofts"][0]["skin_stations"]=invalid;
        assert!(parse_source(&text(&definition)).is_err());
    }
    definition["lofts"][0]["skin_stations"]=good;
    definition["lofts"][0]["skin_stations"][2]["weights"]=json!([0.2,0.2]);
    assert!(parse_source(&text(&definition)).is_err());
    definition["lofts"][0]["skin_stations"][2]["weights"]=json!([-0.1,1.1]);
    assert!(parse_source(&text(&definition)).is_err());
}

#[test]
fn actual_material_projection_retains_station_weights_in_canonical_skl1() {
    use crate::{species_authored_review,species_review_source::AuthoredSpeciesSource};
    use qualia_core_db::{bundle::BundleReader,
        container_10d::skeletal_container::read_skeletal_section};
    let mut definition=source();
    definition["lofts"][0]["skin_stations"]=profile(&definition);
    // This exercises the builder/material/codec seam, not Vibe replay or rights.
    let authored=AuthoredSpeciesSource {data:definition,
        sources:vec!["// synthetic skin-station builder fixture".into()],revision:1,
        source_sha256:"1".repeat(64)};
    let asset=species_authored_review::build(1,&authored).unwrap();
    let section=read_skeletal_section(&asset.render.bytes).unwrap();
    let bundle=BundleReader::parse(&asset.bundle).unwrap();
    let anatomy:Value=serde_json::from_slice(bundle.get("authoring/anatomy.json").unwrap()).unwrap();
    let mut cursor=0;
    let mut saw_intermediate=false;
    for material in anatomy["render_bindings"].as_array().unwrap() {
        for original in material["source_skin_vertices"].as_array().unwrap() {
            let original=original.as_u64().unwrap() as usize;
            let joints:[u16;4]=serde_json::from_value(anatomy["skin"]["joint_indices"][original].clone()).unwrap();
            let weights:[f32;4]=serde_json::from_value(anatomy["skin"]["joint_weights"][original].clone()).unwrap();
            let decoded=section.skin(cursor).unwrap();
            assert_eq!(decoded.joints,joints);
            assert_eq!(decoded.weights,weights);
            saw_intermediate|=joints.iter().zip(weights).any(|(&j,w)|(j==1||j==2)&&w>0.);
            cursor+=1;
        }
    }
    assert_eq!(cursor,section.counts().vertices);
    assert!(saw_intermediate);
}
