from stage import *
assert not (HERE/"PREPARATION-FROZEN.json").exists(), "Historical preparation only; install pinned source images, do not rerun generators"
s=get(P+'controller.rs').replace('            surface_resources:None,\n','',1)
needle='''        Ok(Self {
            artifact_id,''';assert needle in s;s=s.replace(needle,'''        Ok(Self {
            surface_resources:None,
            artifact_id,''',1);put(P+'controller.rs',s)
s=get(P+'native_entry/create.rs');start=s.index('        geometry_grants: vec![');end=s.index('        max_vertices:',start)
s=s[:start]+'''        geometry_grants: native_geometry_grants(std::env::var("POET_NATIVE_REVOLVE_PROFILE").as_deref()==Ok("1")),
'''+s[end:]
start=s.index('pub(super) fn project_config')
helper='''// Explicit native-host opt-in; retained project attachment never adds the new kernel by default.
fn native_geometry_grants(revolve:bool)->Vec<String>{
 let mut grants=["capability.invoke","ComputationalGeometry.extrude_profile","ComputationalGeometry.swung_surface","ComputationalGeometry.profile_sweep","ComputationalGeometry.uv_sphere","ComputationalGeometry.torus"].into_iter().map(str::to_string).collect::<Vec<_>>();
 if revolve{grants.push("ComputationalGeometry.revolve_profile".into());} grants
}
#[cfg(test)]
#[test]
fn native_revolution_kernel_requires_explicit_host_opt_in(){
 let legacy=native_geometry_grants(false);assert!(!legacy.iter().any(|g|g=="ComputationalGeometry.revolve_profile"));
 let enabled=native_geometry_grants(true);assert_eq!(enabled.len(),legacy.len()+1);assert!(enabled.starts_with(&legacy));assert_eq!(enabled.last().unwrap(),"ComputationalGeometry.revolve_profile");
}
'''
s=s[:start]+helper+s[start:]
# Native project-owned import directory enables the existing authorized library controls.
s=s.replace('        import_library: None,','        import_library:Some(crate::graphical::package::ResourceLibrary::new(&files.directory("imports",true)?,16*1024*1024)?),',1);put(P+'native_entry/create.rs',s)
put(P+'authoring/resource_workflow.md','''# Native surface resource workflow
Texture inputs enter the existing authorized import library at the native project\'s
`imports` directory. Use existing package import controls with canonical texture
key and explicit MIME kind, then bind the inspected HMC SHA256. No source path or
network URL grants access. Existing-key changed compiled data needs a new asset
key. Bind immutable resources and explicitly rebuild after reopening textured
source; rebinding never restores a session or changes source authority.

The revolution builder requires native host opt-in
`POET_NATIVE_REVOLVE_PROFILE=1`, plus authored requests and actual target grants.
The legacy host profile stays unchanged without this explicit setting.
''')
print('Root compile/profile policy repairs staged')
