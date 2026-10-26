from pathlib import Path
import hashlib,json
group=Path(__file__).resolve().parent/'reference-profile'
name='native/src/graphical/reference_authoring_tests.rs'
raw=(Path('C:/github/poet')/name).read_bytes();s=raw.decode().replace('\r\n','\n')
start=s.index('fn host(root: &Path) -> CheckpointEditorHost {');end=s.index('fn diagnostic(',start)
s=s[:start]+'''fn host(root: &Path) -> CheckpointEditorHost {
    let profile=super::native_authoring_profile::NativeAuthoringProfile::Equipment;
    let provider=InteractivePreviewProvider::with_budget(MeshPreviewProvider {
        material:MaterialRecord::legacy_default(),
        camera:PreviewCamera{yaw:std::f32::consts::PI,pitch:0.15,distance:5.0,target:[0.0,0.8,0.0]},
        limits:profile.preview_limits(),
    },profile.viewport_budget()).unwrap();
    let grants=["capability.invoke","ComputationalGeometry.profile_sweep","ComputationalGeometry.uv_sphere","ComputationalGeometry.torus"].into_iter().map(str::to_owned).collect();
    let host=CanonicalEditorHost::open_with_rebuild_limits(ID.into(),profile.session_limits(),
        GeometryHost::new(grants,profile.max_vertices()),Box::new(provider),profile.rebuild_limits()).unwrap();
    CheckpointEditorHost::new(Box::new(host),service(root)).unwrap()
}
'''+s[end:]
s=s.replace('CanonicalEditorHost, EditorHost, MeshPreviewProvider, RebuildLimits','CanonicalEditorHost, EditorHost, MeshPreviewProvider')
s=s.replace('preview::{PreviewCamera, PreviewFrame, PreviewLimits}','preview::{PreviewCamera, PreviewFrame}')
s=s.replace('viewport::{ViewportBudget, ViewportCommand}','viewport::ViewportCommand')
s=s.replace('use vibe::session::SessionLimits;\n','')
s=s.replace('for name in ["observed.json","reopened.json"] {','for name in ["observed.json","reopened.json",HMC,"accepted-source.vibe"] {')
a='''        assert_eq!(cells[0], master);
        h.compile_checkpoint'''
b='''        assert_eq!(cells[0], master);
        std::fs::write(root.join("accepted-source.vibe"),cells.join("\n\n")).unwrap();
        h.compile_checkpoint'''
assert s.count(a)==1;s=s.replace(a,b)
before=group/'before/poet'/name;after=group/'after/poet'/name
before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
before.write_bytes(raw);after.write_text(s,encoding='utf-8',newline='\n')
rows=[dict(repository='poet',target=name,before_sha256=hashlib.sha256(raw).hexdigest(),after_sha256=hashlib.sha256(after.read_bytes()).hexdigest())]
(group/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',files=rows),indent=2)+'\n')
print(hashlib.sha256((group/'manifest.json').read_bytes()).hexdigest())
