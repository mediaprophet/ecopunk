from pathlib import Path
assert not (Path(__file__).resolve().parent/"PREPARATION-FROZEN.json").exists(), "Historical one-shot staging is frozen"
from staging import replace,read,put
P=Path('C:/github/poet/native/src/graphical')
signature='fn inspect_surface(&self,id:&str,expected:SourceRef)->Result<crate::graphical::surfaces::inspection::SurfaceInspection,String>'
for file,owner,target in [('session_host.rs','SessionEditorHost','self.current()?'),('shared_host.rs','SharedEditorHost','self.current()?'),('package/mod.rs','ResourceImportHost','self.inner'),('checkpoints/editor.rs','CheckpointEditorHost','self.inner'),('entry.rs','DiagnosticEditor','self.inner')]:
    replace(P/file,f'impl EditorHost for {owner} {{',f'impl EditorHost for {owner} {{\n    {signature} {{ {target}.inspect_surface(id,expected) }}\n')
replace(P/'cooperation/editor.rs','impl EditorHost for AuthorizedEditorHost {','''impl EditorHost for AuthorizedEditorHost {
    '''+signature+''' {
        with_authorized(self.owner.as_ref(),&self.scope,Some(&expected),AuthoringAction::ReadSource,self.limits,|_,_|self.inner.inspect_surface(id,expected.clone()))
    }
''')
replace(P/'jobs.rs','pub enum Action {','pub enum Action {\n    InspectSurface { id:String, expected:SourceRef },')
replace(P/'jobs.rs','pub struct Reply {','pub struct Reply {\n    pub surface:Option<super::surfaces::inspection::SurfaceInspection>,')
replace(P/'jobs.rs','    let mut authoring = None;','    let mut authoring = None;\n    let mut surface = None;')
replace(P/'jobs.rs','Action::BindSurfacePackage {expected,..} |','Action::InspectSurface {expected,..} | Action::BindSurfacePackage {expected,..} |')
replace(P/'jobs.rs','&mut typed_10d, cancelled)','&mut typed_10d, &mut surface, cancelled)')
replace(P/'jobs.rs','    Reply {\n        typed_10d:','    Reply {\n        surface: if inspection_is_current {surface} else {None},\n        typed_10d:')
replace(P/'job_actions.rs','    cancelled: &std::sync::atomic::AtomicBool,','    surface: &mut Option<crate::graphical::surfaces::inspection::SurfaceInspection>,\n    cancelled: &std::sync::atomic::AtomicBool,')
replace(P/'job_actions.rs','    let message = match action {','''    let message = match action {
        Action::InspectSurface{id,expected}=> {
            if id.is_empty() || id.len()>128 || cancelled.load(std::sync::atomic::Ordering::Acquire) {return Err("Surface inspection refused before dispatch".into());}
            *surface=Some(host.inspect_surface(&id,expected)?);
            "Current selected source UV and texture pixels inspected; no asset acceptance claim.".into()
        }
''')
replace(P/'native_dispatch.rs','        Action::BindSurfacePackage { expected, .. }','        Action::InspectSurface { expected, .. }\n        | Action::BindSurfacePackage { expected, .. }')
replace(P/'native_dispatch.rs','    let (kind, parameters) = match &action {','''    let (kind, parameters) = match &action {
        Action::InspectSurface{id,..}=>(ActionType::Query,json!({"command":"authoring.inspect_surface","id":id})),''')
replace(P/'native_dispatch.rs','            let p = match edit {','''            let p = match edit {
                AuthoringEdit::UvAtlas{id,atlas}=>json!({"command":"authoring.uv_atlas","id":id,"atlas":atlas}),
                AuthoringEdit::TextureDetail{material_id,detail}=>json!({"command":"authoring.texture_detail","material_id":material_id,"detail":detail}),''')
replace(P/'poet_workspace/native_commands.rs','pub(super) enum Command {','''pub(super) enum Command {
    #[serde(rename="authoring.inspect_surface")]
    InspectSurface{id:String},
    #[serde(rename="authoring.uv_atlas")]
    UvAtlas{id:String,atlas:Option<crate::graphical::surfaces::uv_atlas::AtlasDraft>},
    #[serde(rename="authoring.texture_detail")]
    TextureDetail{material_id:u64,detail:Option<crate::graphical::surfaces::detail::DetailRecipe>},''')
replace(P/'poet_workspace/native_commands.rs','Self::BindSurface { .. } | Self::Inspect | Self::Check','Self::InspectSurface{..} | Self::BindSurface { .. } | Self::Inspect | Self::Check')
replace(P/'poet_workspace/native_commands.rs','Self::BindSurface { .. } => "Studio.inspect",','Self::InspectSurface{..} | Self::BindSurface { .. } => "Studio.inspect",\n            Self::UvAtlas{..} | Self::TextureDetail{..} => "Studio.edit",')
replace(P/'poet_workspace/native_commands.rs','        Ok(match self {','''        Ok(match self {
            Self::InspectSurface{id}=>{text(&id,128)?;Action::InspectSurface{id,expected}},
            Self::UvAtlas{id,atlas}=>{text(&id,128)?;if let Some(a)=&atlas {crate::graphical::surfaces::uv_atlas::source(a)?;}Action::AuthoringEdit{edit:AuthoringEdit::UvAtlas{id,atlas},expected}},
            Self::TextureDetail{material_id,detail}=>{if material_id==0{return Err("Nonzero material identifier required".into());}if let Some(d)=&detail {d.source_expression()?;}Action::AuthoringEdit{edit:AuthoringEdit::TextureDetail{material_id,detail},expected}},''')
replace(P/'authoring/controls.rs','    let data = inspected();','    let data = inspected();\n    let tools_host=host.clone();let tools_source=current.source.clone();let tools_buffer=buffer.clone();')
needle='                super::surface_controls::SurfaceControls{'
s=read(P/'authoring/controls.rs');i=s.index(needle);s=s[:i]+'''                super::surface_tools::SurfaceTools{host:tools_host.clone(),view,selected,inspected,fresh: fresh && !busy(),busy,status,
                    onedit:move|edit|send(Action::AuthoringEdit{edit,expected:tools_source.clone()},tools_host.clone(),view,draft,status,image,busy,inspected,tools_buffer.clone(),true)}
'''+s[i:];put(P/'authoring/controls.rs',s)
replace(Path('C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/capability_manifests/registry.rs'),'pub const GEOMETRY_OP_MANIFESTS: &[OpManifest] = &[','pub const GEOMETRY_OP_MANIFESTS: &[OpManifest] = &[\n    super::super::uv_atlas::UV_ATLAS_MANIFEST,')
f=Path('C:/github/poet/native/Cargo.toml');s=read(f)
for name,file in [('foundation_uv_atlas','uv_atlas_native.rs'),('foundation_image_source_over','image_source_over.rs')]:
    assert f'name = "{name}"' not in s
    s+=f'\n[[test]]\nname = "{name}"\npath = "../../qualiaDB/crates/qualia-core-db/tests/{file}"\nrequired-features = ["native-graphical"]\n'
put(f,s)
