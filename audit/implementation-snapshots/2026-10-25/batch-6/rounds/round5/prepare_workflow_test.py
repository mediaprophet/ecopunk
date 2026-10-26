"""Stage real geometry authoring into the already-qualified native review harness."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent;OUT=HERE/'integration';P=Path('C:/github/poet/native/src/graphical')
s=(P/'native_project_package_tests.rs').read_text()
start=s.index('use crate::graphical::checkpoints::{');end=s.index('use crate::graphical::{',start)
s=s[:start]+'''use crate::graphical::{authoring::{AuthoringEdit,GeometryRecipe},jobs,native_dispatch};
use crate::graphical::checkpoints::{ResourceInspectionRequest,typed_10d::{Typed10dLimits,Typed10dEditRequest}};
'''+s[end:]
start=s.index('use qualia_core_db::{');end=s.index('use std::sync',start)
s=s[:start]+'''use qualia_core_db::poet_host::native_authority::package_review::PackageReviewGrant;
'''+s[end:]
helper='''
struct AssemblyNoPixels;
impl PreviewProvider for AssemblyNoPixels {
    fn validate(&self,value:&Value)->Result<(),String>{crate::graphical::assembly::admit_assembly(value,PreviewLimits::default()).map(|_|())}
    fn preview(&mut self,_:SourceRef,_:&Value)->Result<PreviewFrame,String>{Err("No GUI/GPU claimed by spatial workflow test".into())}
}
fn spatial_config(scope:Scope,root:&Path,library:&Path)->ProjectConfig{
    let mut cfg=config(scope,root,library);
    cfg.geometry_grants.extend(["ComputationalGeometry.uv_sphere".into(),"ComputationalGeometry.torus".into(),"ComputationalGeometry.profile_sweep".into()]);cfg
}
fn block_on<F:std::future::Future>(future:F)->F::Output{
    struct Wake(std::thread::Thread);
    impl std::task::Wake for Wake{fn wake(self:Arc<Self>){self.0.unpark();}fn wake_by_ref(self:&Arc<Self>){self.0.unpark();}}
    let waker=std::task::Waker::from(Arc::new(Wake(std::thread::current())));let mut cx=std::task::Context::from_waker(&waker);
    let mut future=std::pin::pin!(future);loop{match future.as_mut().poll(&mut cx){std::task::Poll::Ready(v)=>return v,std::task::Poll::Pending=>std::thread::park()}}
}
fn assembly_source()->&'static str{r#"
let m={id:1,colour_space:"linear",base_color:[0.3,0.4,0.5,1.0],emissive:[0.0,0.0,0.0],metallic:0.1,roughness:0.5,specular:1.0,alpha_cutoff:0.5,opacity_mode:"Opaque",double_sided:true};
let tri={vertices:[[0.0,0.0,0.0],[1.0,0.0,0.0],[0.0,1.0,0.0]],triangles:[[0,1,2]]};
let mut poet_parts=[{id:"root",semantic_id:101,geometry:tri,material:m,translation:[0.0,0.0,0.0],rotation:[0.0,0.0,0.0],scale:[1.0,1.0,1.0],annotation:"retained source field"}];
({parts:poet_parts});
"#}
'''
pos=s.index('#[test]');s=s[:pos]+helper+s[pos:]
s=s.replace('package_workflow_real_producer_separate_reviewer_reopen_and_same_source_successor','spatial_authoring_native_intents_compile_typed_inspect_and_separate_package_review_reopen')
s=s.replace('config(scope.clone(),','spatial_config(scope.clone(),').replace('Box::new(NoPixels)','Box::new(AssemblyNoPixels)')
s=s.replace('&materials::diagnostic_source()','assembly_source()')
marker='''        let saved = operator
            .save_checkpoint("review.hmc", ExpectedFile::Absent, source.clone())'''
extra='''        let mut current=operator.snapshot().unwrap();
        let (worker,handle)=jobs::Worker::start(Box::new(operator)).unwrap();
        for edit in [
            AuthoringEdit::AddGeometry{id:"ring".into(),semantic_id:102,material_from:"root".into(),geometry:GeometryRecipe::Sphere{radius:0.2,lat_segments:6,lon_segments:8}},
            AuthoringEdit::ReplaceGeometry{id:"ring".into(),geometry:GeometryRecipe::Torus{major_radius:0.3,minor_radius:0.05,major_segments:12,minor_segments:6}},
            AuthoringEdit::Reparent{id:"ring".into(),parent:Some("root".into())},
            AuthoringEdit::UniqueMaterial{id:"ring".into(),material_id:9},
            AuthoringEdit::Material{material_id:9,base_color:[0.2,0.6,0.3,1.0],roughness:0.25,metallic:0.8},
        ]{
            let reply=block_on(native_dispatch::run(handle.clone(),jobs::Action::AuthoringEdit{edit,expected:current.source.clone()},jobs::Context::from_snapshot(&current))).unwrap();
            assert!(reply.operation_succeeded,"native Poet intent failed: {}",reply.message);
            current=reply.snapshot.unwrap();
        }
        assert_ne!(current.source,source,"real source revisions required");
        worker.finish().unwrap();
        let mut operator=SessionEditorHost::new(project.clone(),sessions.clone());
        let source=current.source;
        let parts=operator.authoring_view(source.clone()).unwrap();
        let ring=parts.parts.iter().find(|p|p.id=="ring").unwrap();
        assert_eq!(ring.parent.as_deref(),Some("root"));assert_eq!(ring.material_id,9);assert_eq!(ring.roughness,0.25);
        let saved = operator
            .compile_checkpoint("review.hmc","spatial", ExpectedFile::Absent, source.clone())'''
assert marker in s;s=s.replace(marker,extra,1)
start=s.index('        let metadata = M64GeoRasterMetadata {');end=s.index('        let selection = PackageSelection {',start)
s=s[:start]+'''        let inventory=operator.package_inventory("review.hmc",source.clone()).unwrap();
        let entry=inventory.resources.iter().find(|r|r.key.ends_with(".10d")).unwrap();
        let request=ResourceInspectionRequest{filename:"review.hmc".into(),package_digest:saved.digest,key:entry.key.clone(),resource_digest:entry.digest};
        let typed=operator.inspect_typed_10d(request.clone(),source.clone(),Typed10dLimits::default()).unwrap();
        assert_eq!(typed.materials.iter().find(|m|m.id==9).unwrap().roughness,0.25);
        assert!(typed.frames.is_some());assert!(typed.edit_blocker.is_some(),"compiled Q42 pair must require source-route editing");
        let before=std::fs::read(hmc.join("review.hmc")).unwrap();
        assert!(operator.edit_typed_10d(Typed10dEditRequest{resource:request,source_digest:typed.source_digest,material_id:9,
            base_color:[0.1,0.2,0.3,1.0],emissive:[0.0;3],metallic:0.0,roughness:0.9},source.clone(),Typed10dLimits::default(),&AtomicBool::new(false)).is_err());
        assert_eq!(before,std::fs::read(hmc.join("review.hmc")).unwrap(),"refused companion edit must be atomic");
'''+s[end:]
s=s.replace('Explicit synthetic grid plus retained opaque resource','Native geometry/material/assembly source and retained opaque resource')
start=s.index('    let raster = carrier');end=s.index('    let unknown = carrier',start)
s=s[:start]+'''    let geometry=carrier.resources().iter().find(|r|r.key.ends_with(".10d")).unwrap();
    assert!(!geometry.bytes.is_empty());
    assert!(carrier.resources().iter().any(|r|r.key.ends_with(".q42")));
'''+s[end:]
start=s.index('    let successor = other_frontend');end=s.index('    assert_ne!(successor.digest, saved.digest);',start)
s=s[:start]+'''    let successor=other_frontend.edit_package_resources("review.hmc",ExpectedFile::Sha256(saved.digest),
        vec![ResourceEdit::Add(ResourceData{key:"notes/changed".into(),kind:"test/opaque".into(),bytes:vec![1,2],meta:None})],source.clone()).unwrap();
'''+s[end:]
rel='native/src/graphical/native_project_spatial_tests.rs';stage=OUT/'after'/rel;stage.parent.mkdir(parents=True,exist_ok=True);stage.write_text(s,encoding='utf-8')
rows=json.loads((OUT/'target-manifest.json').read_text());rows.append(dict(target=str(P/'native_project_spatial_tests.rs'),stage=str(stage),before_sha256=None,after_sha256=hashlib.sha256(stage.read_bytes()).hexdigest()))
target=P/'native_project_tests.rs';data=target.read_bytes();stage=OUT/'after/native/src/graphical/native_project_tests.rs';stage.write_text(data.decode()+ '\n#[path="native_project_spatial_tests.rs"]\nmod spatial_workflow_tests;\n',encoding='utf-8');before=OUT/'before/native/src/graphical/native_project_tests.rs';before.parent.mkdir(parents=True,exist_ok=True);before.write_bytes(data)
rows.append(dict(target=str(target),stage=str(stage),before_sha256=hashlib.sha256(data).hexdigest(),after_sha256=hashlib.sha256(stage.read_bytes()).hexdigest()))
(OUT/'target-manifest.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
print(json.dumps(dict(staged_targets=len(rows),workflow_lines=len(s.splitlines()),live_writes=False)))
