from stage import *
assert not (HERE/"PREPARATION-FROZEN.json").exists(), "Historical preparation only; install pinned source images, do not rerun generators"
s=(POET/P/'native_project_spatial_tests.rs').read_text()
s=s.replace('spatial_authoring_native_intents_compile_typed_inspect_and_separate_package_review_reopen','surface_profile_native_intents_texture_closure_review_and_rebind_reopen')
needle='fn assembly_source()';pos=s.index(needle)
texture=r'''// Canonical decoder vector: ephemeral 2x2 RGBA8 sRGB KTX2, not a production asset.
fn synthetic_texture()->Vec<u8>{
 let mut b=vec![0u8;152];b[..12].copy_from_slice(&qualia_core_db::render::texture_ktx2::KTX2_IDENTIFIER);
 for(at,n)in[(12,43u32),(16,1),(20,2),(24,2),(36,1),(40,1),(48,104),(52,28),(104,28),(112,(24<<16)|2)]{b[at..at+4].copy_from_slice(&n.to_le_bytes());}
 for(at,n)in[(80,136u64),(88,16),(96,16)]{b[at..at+8].copy_from_slice(&n.to_le_bytes());}
 b[136..].copy_from_slice(&[255,0,0,255,0,255,0,255,0,0,255,255,255,255,255,255]);b
}
'''
s=s[:pos]+texture+s[pos:]
marker='        let mut current = operator.snapshot().unwrap();';assert marker in s
setup=r'''        let pixels=synthetic_texture();let texture_digest:[u8;32]=Sha256::digest(&pixels).into();
        let base=operator.save_checkpoint("textures.hmc",ExpectedFile::Absent,source.clone()).unwrap();
        let base=operator.edit_package_resources("textures.hmc",ExpectedFile::Sha256(base.digest),vec![ResourceEdit::Add(ResourceData{
            key:qualia_core_db::render::asset_package::texture_hmc_key(&texture_digest),kind:"image/ktx2".into(),bytes:pixels.clone(),meta:None})],source.clone()).unwrap();
        let epoch=operator.snapshot().unwrap().execution_epoch;
        assert!(operator.bind_surface_package(crate::graphical::surface_host::SurfacePackageRequest{filename:"textures.hmc".into(),digest:[1;32]},source.clone()).is_err());
        assert!(epoch.is_current(),"bad package digest must not retire execution");
        operator.bind_surface_package(crate::graphical::surface_host::SurfacePackageRequest{filename:"textures.hmc".into(),digest:base.digest},source.clone()).unwrap();
        assert!(!epoch.is_current(),"resource binding retires old frame/execution context");
        let mut surface=crate::graphical::surfaces::SurfaceDraft::from_material(&MaterialRecord::legacy_default());
        surface.material.base_color_texture=texture_digest;surface.material.roughness=0.25;surface.material.metallic=0.8;
'''
s=s.replace(marker,setup+marker,1)
start=s.index('        for edit in [');end=s.index('        ] {',start)
s=s[:start]+r'''        for edit in [
            AuthoringEdit::AddGeometry{id:"ring".into(),semantic_id:102,material_from:"root".into(),geometry:GeometryRecipe::ProfileSweep{
                profile:crate::graphical::authoring::ProfileRecipe::RoundedRectangle{width:0.4,height:0.2,radius:0.02,segments:4},length:0.8,caps:true}},
            AuthoringEdit::Reparent{id:"ring".into(),parent:Some("root".into())},
            AuthoringEdit::UniqueMaterial{id:"ring".into(),material_id:9},
            AuthoringEdit::UvProjection{id:"ring".into(),projection:Some(crate::graphical::surfaces::uv::PlanarUvDraft{axes:[0,2],metres_per_repeat:0.25})},
            AuthoringEdit::UvTransform{id:"ring".into(),transform:qualia_core_db::container_10d::TextureTransform{offset:[0.25,0.0],scale:[2.0,1.0],rotation:0.0,tex_coord:0}},
            AuthoringEdit::Surface{material_id:9,surface},
'''+s[end:]
marker='        assert!(typed.frames.is_some());';assert marker in s
s=s.replace(marker,marker+r'''
        let material=typed.materials.iter().find(|m|m.id==9).unwrap();assert_eq!(material.base_color_texture,texture_digest);
        let bytes=std::fs::read(hmc.join("review.hmc")).unwrap();
        let resources=crate::graphical::surfaces::TextureResources::from_hmc_bytes(&bytes,Default::default()).unwrap();
        let dependencies=resources.dependencies(&typed.materials).unwrap();assert_eq!(dependencies.len(),1);assert_eq!(dependencies[0].bytes,pixels);
''',1)
marker='    let sessions = Arc::new(Mutex::new(Some(reviewer.clone())));';assert marker in s
s=s.replace(marker,marker+r'''
    let mut resumed=SessionEditorHost::new(project.clone(),sessions.clone());
    let held=resumed.snapshot().unwrap();assert!(held.reconciliation_required);
    resumed.bind_surface_package(crate::graphical::surface_host::SurfacePackageRequest{filename:"review.hmc".into(),digest:saved.digest},held.source.clone()).unwrap();
    resumed.rebuild_execution(held.source).unwrap();assert!(!resumed.snapshot().unwrap().reconciliation_required);
''',1)
s=s.replace('let service = project.package_workflow_service(sessions);','let service = project.package_workflow_service(sessions.clone());',1)
put(P+'native_project_surface_tests.rs',s)
s=get(P+'native_project_tests.rs');s+='\n#[path="native_project_surface_tests.rs"]\nmod surface_workflow_tests;\n';put(P+'native_project_tests.rs',s)
print('Real native profile/UV/texture/review/rebind workflow staged')
