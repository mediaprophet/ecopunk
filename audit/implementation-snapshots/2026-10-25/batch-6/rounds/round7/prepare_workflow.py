from pathlib import Path
assert not (Path(__file__).resolve().parent/"PREPARATION-FROZEN.json").exists(), "Historical one-shot staging is frozen"
from staging import put,replace
P=Path('C:/github/poet/native/src/graphical')
s=(P/'native_project_surface_tests.rs').read_text()
s=s.replace('fn surface_profile_native_intents_texture_closure_review_and_rebind_reopen()', 'fn uv_atlas_paint_native_intents_resource_closure_review_and_reopen()')
a=s.index('fn synthetic_texture()');z=s.index('fn assembly_source()',a)
s=s[:a]+'''fn synthetic_texture()->Vec<u8> {
    let pixels=vec![80u8,120,160,255].repeat(8*8);
    let mut encoded=vec![0;8192];
    let n=qualia_core_db::render::surface_authoring::encode_surface_png_into(8,&pixels,&mut encoded).unwrap();encoded.truncate(n);encoded
}
'''+s[z:]
s=s.replace('kind: "image/ktx2".into(),','kind: "image/png".into(),')
s=s.replace('bytes: pixels.clone(),\n                    meta: None,','bytes: pixels.clone(),\n                    meta: Some(vec![0xa1,0x61,b\'p\',7]),')
needle='''            AuthoringEdit::Surface {
                material_id: 9,
                surface,
            },'''
assert s.count(needle)==1
s=s.replace(needle,needle+'''
            AuthoringEdit::UvAtlas{id:"root".into(),atlas:Some(crate::graphical::surfaces::uv_atlas::AtlasDraft{atlas_size:64,gutter_pixels:2,texels_per_metre:8.0,..Default::default()})},
            AuthoringEdit::TextureDetail{material_id:9,detail:Some(crate::graphical::surfaces::detail::DetailRecipe::Paint(crate::graphical::surfaces::detail::PaintRecipe{
                version:1,side:8,base_sha256:crate::graphical::surfaces::binding::digest_hex(&texture_digest),layers:vec![crate::graphical::surfaces::detail::PaintLayer{name:"Editable marks".into(),visible:true,opacity:0.7,strokes:vec![crate::graphical::surfaces::detail::PaintStroke{size:4.,colour:[220,30,20,255],opacity:0.8,hardness:0.5,points:vec![[3.,3.],[4.,4.]]}]}]
            }))},
            AuthoringEdit::TextureDetail{material_id:1,detail:Some(crate::graphical::surfaces::detail::DetailRecipe::Procedural(crate::graphical::surfaces::detail::ProceduralRecipe{
                version:1,side:16,seed:7,tint:[0.2,0.5,0.8],contrast:0.4,relief:0.2,bands:[2,3],band_strength:0.3,roughness:0.5,metallic:0.1,
            }))},''')
needle='''        worker.finish().unwrap();
        let mut operator = SessionEditorHost::new(project.clone(), sessions.clone());'''
assert s.count(needle)==1
s=s.replace(needle,'''        for id in ["root","ring"] {
            let reply=block_on(native_dispatch::run(handle.clone(),jobs::Action::InspectSurface{id:id.into(),expected:current.source.clone()},jobs::Context::from_snapshot(&current))).unwrap();
            assert!(reply.operation_succeeded,"{}",reply.message);
            let data=reply.surface.unwrap();assert!(data.uv_image.starts_with("data:image/png;base64,"));assert!(data.texture_image.is_some());
            if id=="root"{assert!(data.summary.contains("disk charts"));assert_eq!(data.edges.len(),3);}
        }
        let before=current.source.clone();
        let rejected=block_on(native_dispatch::run(handle.clone(),jobs::Action::AuthoringEdit{edit:AuthoringEdit::UvAtlas{id:"ring".into(),atlas:Some(Default::default())},expected:before.clone()},jobs::Context::from_snapshot(&current))).unwrap();
        assert!(!rejected.operation_succeeded,"planar/affine must not be silently replaced by atlas");assert_eq!(rejected.snapshot.unwrap().source,before);
        worker.finish().unwrap();
        // A new native project/service has no retained carrier from the import
        // write. Binding alone must preserve source images and opaque metadata.
        drop(project);
        let project=Arc::new(NativeProject::open(spatial_config(scope.clone(),&hmc,&library),authority.clone(),author.clone(),None,Box::new(AssemblyNoPixels)).unwrap());
        let mut operator = SessionEditorHost::new(project.clone(), sessions.clone());
        let held=operator.snapshot().unwrap();
        operator.bind_surface_package(crate::graphical::surface_host::SurfacePackageRequest{filename:"textures.hmc".into(),digest:base.digest},held.source.clone()).unwrap();
        operator.rebuild_execution(held.source).unwrap();''')
s=s.replace('assert_eq!(material.base_color_texture, texture_digest);','assert_ne!(material.base_color_texture, texture_digest);')
s=s.replace('''        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0].bytes, pixels);''','''        assert_eq!(dependencies.len(), 4);
        let reader=qualia_core_db::bundle::BundleReader::parse(&bytes).unwrap();
        let key=qualia_core_db::render::asset_package::texture_hmc_key(&texture_digest);
        assert_eq!(reader.get(&key),Some(pixels.as_slice()));
        assert_eq!(reader.entry(&key).unwrap().meta,Some(vec![0xa1,0x61,b'p',7]));''')
needle='''    assert!(!resumed.snapshot().unwrap().reconciliation_required);'''
s=s.replace(needle,needle+'''
    let accepted=resumed.authoring_view(source.clone()).unwrap();
    assert!(accepted.parts.iter().find(|p|p.id=="root").unwrap().uv_atlas.is_some());
    assert!(accepted.parts.iter().all(|p|p.detail.is_some()));
    let maps=resumed.compiled_assets("reopened",source.clone()).unwrap();
    assert!(maps[0].texture_dependencies.iter().any(|d|d.digest==crate::graphical::surfaces::binding::parse_digest(match &accepted.parts.iter().find(|p|p.id=="ring").unwrap().detail {Some(crate::graphical::surfaces::detail::DetailRecipe::Paint(p))=>&p.base_sha256,_=>panic!("paint recipe missing")}).unwrap()));''')
put(P/'native_project_uv_detail_tests.rs',s)
replace(P/'native_project_tests.rs','mod surface_workflow_tests;','mod surface_workflow_tests;\n\n#[path="native_project_uv_detail_tests.rs"]\nmod uv_detail_workflow_tests;')
# The two reviewed retention files compose with the new internal trait above.
B=Path(__file__).resolve().parent/'detail/retention-repair/after/native/src/graphical'
for f in ['authoring/package.rs','checkpoints/resources.rs']:put(P/f,(B/f).read_text())
