mod living_fixture;
#[test]
fn actual_importer_exposes_both_bind_and_loop_admission_failures() {
    use qualia_core_db::{container_10d::skeletal_section::*,render::skeletal::*};
    let scene=qualia_core_db::render::assets::parse_glb_living_scene(&living_fixture::living_fixture()).unwrap();
    assert!(scene.skl1_section.is_none());
    let counts=Counts{joints:scene.joints.len(),vertices:scene.skin_weights.len(),clips:scene.clips.len(),tracks:scene.tracks.len(),keys:scene.keys.len()};
    let mut output=vec![0;counts.encoded_len().unwrap()];
    let encode=|joints:&[Joint],clips:&[ClipRecord],output:&mut[u8]|encode_skeletal_section_into(&SkeletalData{joints,joint_ids:&scene.joint_ids,skin:&scene.skin_weights,clips,tracks:&scene.tracks,keys:&scene.keys},output);
    assert_eq!(encode(&scene.joints,&scene.clips,&mut output),Err(SkeletalSectionError::InvalidData));
    let mut clips=scene.clips.clone(); clips[0].looping=false;
    assert_eq!(encode(&scene.joints,&clips,&mut output),Err(SkeletalSectionError::InvalidData),"nonloop alone cannot repair mismatched bind rest");
    let mut joints=scene.joints.clone();for joint in &mut joints{joint.rest_local=Motor::identity();}
    assert_eq!(encode(&joints,&scene.clips,&mut output),Err(SkeletalSectionError::InvalidData),"bind rest alone cannot repair invented loop seam");
    assert!(encode(&joints,&clips,&mut output).is_ok());
    eprintln!("baseline_native_SKL1_blockers=authored_pose_vs_bind_rest,invented_loop_seam; silent_section_loss_reproduced=true");
}
#[test]
fn actual_bc7_arbitrary_payloads_legitimately_encode_fully_transparent_alpha_modes() {
    for mode in 0..8 {
        let mut block=[0u8;16];block[0]=1<<mode;block[1]=0xaa;block[2]=0x55;block[3]=0xff;
        let mut out=[[0u8;4];16];qualia_core_db::render::texture_transcode::decode_bc7_block(&block,&mut out);
        let (min,max)=(out.iter().map(|p|p[3]).min().unwrap(),out.iter().map(|p|p[3]).max().unwrap());
        eprintln!("baseline_BC7 mode={mode} alpha_min={min} alpha_max={max} first_rgba={:?}",out[0]);
        assert_eq!((min,max),if mode<4{(255,255)}else{(0,0)});
    }
}
