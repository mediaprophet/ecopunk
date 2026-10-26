from stage import *
assert not (HERE/"PREPARATION-FROZEN.json").exists(), "Historical preparation only; install pinned source images, do not rerun generators"
s=get(P+'mod.rs').replace('pub mod assembly;','pub mod surfaces;\npub mod surface_host;\npub mod assembly;',1);put(P+'mod.rs',s)
s=get(P+'assembly/mod.rs');s+='\npub use render::render_assembly_with_resources;\n';put(P+'assembly/mod.rs',s)
s=get(P+'authoring/mod.rs').replace('mod geometry_source;','mod geometry_source;\nmod profile_recipe;\nmod profile_controls;\nmod cad_recipe;\nmod cad_controls;\nmod surface_controls;\n#[cfg(test)] mod profile_tests;\n#[cfg(test)] mod cad_tests;\npub use profile_recipe::ProfileRecipe;\nuse crate::graphical::surfaces::{SurfaceDraft,UvTransformDraft};\nuse crate::graphical::surfaces::uv::PlanarUvDraft;',1)
s=s.replace('pub struct PartView {','pub struct PartView {\n    pub surface:SurfaceDraft,\n    pub uv_transform:UvTransformDraft,\n    pub uv_projection:Option<PlanarUvDraft>,',1)
s=s.replace('pub enum AuthoringEdit {','pub enum AuthoringEdit {\n    Surface{material_id:u64,surface:SurfaceDraft},\n    UvTransform{id:String,transform:UvTransformDraft},\n    UvProjection{id:String,projection:Option<PlanarUvDraft>},',1)
s=s.replace('pub enum GeometryRecipe {','pub enum GeometryRecipe {\n    ProfileSweep{profile:ProfileRecipe,length:f64,caps:bool},\n    RevolveProfile{points:Vec<[f64;2]>,segments:u32},',1);put(P+'authoring/mod.rs',s)
s=get(P+'authoring/source.rs').replace('result.push(PartView {','''result.push(PartView {
            surface:crate::graphical::surfaces::SurfaceDraft::from_material(&crate::graphical::materials::material_from_value(field(p,"material")?)?),
            uv_transform:p.get("uv_transform").map(crate::graphical::surfaces::uv::from_value).transpose()?.unwrap_or(qualia_core_db::container_10d::TextureTransform::IDENTITY),
            uv_projection:match p.get("uv_projection"){None|Some(Value::Null)=>None,Some(v)=>Some(crate::graphical::surfaces::uv::projection_from_value(v)?)},''',1)
s=s.replace('let body = match edit {','''let body = match edit {
        AuthoringEdit::Surface{material_id,surface}=>inventory.material(*material_id,surface.updates()?)?,
        AuthoringEdit::UvTransform{id,transform}=>inventory.replace(id,BTreeMap::from([("uv_transform",crate::graphical::surfaces::uv::source(transform)?)]))?,
        AuthoringEdit::UvProjection{id,projection}=>inventory.replace(id,BTreeMap::from([("uv_projection",projection.as_ref().map(crate::graphical::surfaces::uv::projection_source).transpose()?.unwrap_or_else(||"null".into()))]))?,''',1);put(P+'authoring/source.rs',s)
s=get(P+'authoring/assembly_source.rs');needle='''                let index = self.ids[id];''';assert needle in s
s=s.replace(needle,'''                let g=record(field(selected,"geometry")?)?;
                if g.keys().any(|key|!matches!(key.as_str(),"vertices"|"triangles")) || selected.get("uv_projection").is_some_and(|v|!matches!(v,Value::Null)) || selected.contains_key("uv_transform") {
                    return Err("Geometry replacement would discard authored UV/normal/projection or richer data; retain the source and use a new part or an explicitly qualified regeneration operation".into());
                }
'''+needle,1);put(P+'authoring/assembly_source.rs',s)
s=get(P+'authoring/controller.rs').replace('                texture_dependencies: vec![],','                texture_dependencies: crate::graphical::surface_host::dependencies(&scene.materials,self.surface_resources.as_ref())?,',1);put(P+'authoring/controller.rs',s)
s=get(P+'controller.rs').replace('pub struct CanonicalEditorHost {','pub struct CanonicalEditorHost {\n    surface_resources:Option<crate::graphical::surfaces::TextureResources>,',1).replace('            preview,','            preview,\n            surface_resources:None,',1)
s=s.replace('provider.validate(value)','provider.validate_with_resources(value,self.surface_resources.as_ref())',1)
# Keep closure borrows disjoint from the mutable canonical session.
s=s.replace('        let provider = &self.preview;','        let provider = &self.preview;\n        let resources=self.surface_resources.as_ref();',1).replace('provider.validate_with_resources(value,self.surface_resources.as_ref())','provider.validate_with_resources(value,resources)',1)
s=s.replace('self.preview.preview(binding, &result.value)','self.preview.preview_with_resources(binding, &result.value,self.surface_resources.as_ref())',1)
s=s.replace('            self.preview.as_ref(),','            self.preview.as_ref(),\n            self.surface_resources.as_ref(),',1)
marker='impl EditorHost for CanonicalEditorHost {';assert marker in s
s=s.replace(marker,marker+'''
    fn install_surface_resources(&mut self,resources:crate::graphical::surfaces::TextureResources,expected:SourceRef)->Result<(),String>{
        self.expected(&expected)?;
        if let Some(parts)=self.session.value("poet_parts"){
            let value=Value::Record(std::collections::BTreeMap::from([("parts".into(),parts.clone())]));
            self.preview.validate_with_resources(&value,Some(&resources))?;
        }
        self.surface_resources=Some(resources);
        self.execution_epoch.retire();self.execution_epoch=SessionEpoch::new();
        Ok(())
    }
''',1);put(P+'controller.rs',s)
s=get(P+'execution.rs').replace('    validator: &dyn super::PreviewProvider,','    validator: &dyn super::PreviewProvider,\n    resources:Option<&crate::graphical::surfaces::TextureResources>,',1).replace('validator.validate(value)','validator.validate_with_resources(value,resources)',1);put(P+'execution.rs',s)
s=get(P+'editor_contract.rs');marker='pub trait PreviewProvider: Send {'
s=s.replace(marker,marker+'''
    fn validate_with_resources(&self,value:&Value,resources:Option<&crate::graphical::surfaces::TextureResources>)->Result<(),String>{
        self.validate(value)?;crate::graphical::surface_host::validate_value(value,resources)
    }
    fn preview_with_resources(&mut self,source:SourceRef,value:&Value,_resources:Option<&crate::graphical::surfaces::TextureResources>)->Result<PreviewFrame,String>{self.preview(source,value)}
''',1)
s=s.replace('impl PreviewProvider for MeshPreviewProvider {','''impl PreviewProvider for MeshPreviewProvider {
    fn preview_with_resources(&mut self,source:SourceRef,value:&Value,resources:Option<&crate::graphical::surfaces::TextureResources>)->Result<PreviewFrame,String>{
        if crate::graphical::assembly::is_assembly(value){
            let scene=crate::graphical::assembly::admit_assembly(value,self.limits)?;
            return crate::graphical::assembly::render_assembly_with_resources(source,&scene,crate::graphical::assembly::AssemblyRenderSettings::for_camera(self.camera),self.limits,None,resources);
        }
        self.preview(source,value)
    }
''',1)
s=s.replace('pub trait EditorHost: Send {','''pub trait EditorHost: Send {
    fn bind_surface_package(&mut self,_request:crate::graphical::surface_host::SurfacePackageRequest,_expected:SourceRef)->Result<(),String>{Err("Native surface package binding unavailable".into())}
    /// Internal raw owner only; no command deserializes resources or forwards this through authority wrappers.
    fn install_surface_resources(&mut self,_resources:crate::graphical::surfaces::TextureResources,_expected:SourceRef)->Result<(),String>{Err("Raw surface resource owner unavailable".into())}
''',1);put(P+'editor_contract.rs',s)
for rel,marker,body in [
    ('session_host.rs','impl EditorHost for SessionEditorHost {','self.current()?.bind_surface_package(request,expected)'),
    ('shared_host.rs','impl EditorHost for SharedEditorHost {','self.current()?.bind_surface_package(request,expected)'),
    ('package/mod.rs','impl EditorHost for ResourceImportHost {','self.inner.bind_surface_package(request,expected)'),
    ('checkpoints/editor.rs','impl EditorHost for CheckpointEditorHost {','{ let resources=self.service.surface_resources(&request,&expected)?;self.inner.install_surface_resources(resources,expected) }'),
    ('cooperation/editor.rs','impl EditorHost for AuthorizedEditorHost {','with_authorized(self.owner.as_ref(),&self.scope,Some(&expected),AuthoringAction::ReadSource,self.limits,|_,_|self.inner.bind_surface_package(request,expected.clone()))')]:
    s=get(P+rel);s=s.replace(marker,marker+'\n    fn bind_surface_package(&mut self,request:crate::graphical::surface_host::SurfacePackageRequest,expected:SourceRef)->Result<(),String>{request.validate()?;'+body+'}\n',1);put(P+rel,s)
s=get(P+'jobs.rs').replace('pub enum Action {','pub enum Action {\n    BindSurfacePackage{request:super::surface_host::SurfacePackageRequest,expected:SourceRef},',1)
needle='            Action::InspectTyped10d{expected,..}';assert needle in s;s=s.replace(needle,'            Action::BindSurfacePackage {expected,..} | Action::InspectTyped10d{expected,..}',1)
s=s.replace('    match &action {\n','    match &action {\n        Action::BindSurfacePackage{request,..}=>request.validate()?,\n',1);put(P+'jobs.rs',s)
s=get(P+'job_actions.rs').replace('    let message = match action {','''    let message = match action {
        Action::BindSurfacePackage{request,expected}=>{
            if cancelled.load(std::sync::atomic::Ordering::Acquire){return Err("Surface binding cancelled before admission".into());}
            host.bind_surface_package(request,expected)?;
            "Verified immutable HMC texture snapshot bound; old execution/frame context retired. Rebuild held source explicitly if required.".into()
        }
''',1);put(P+'job_actions.rs',s)
for rel in ['entry.rs','native_entry/create.rs']:
    s=get(P+rel);needle='"ComputationalGeometry.torus"'
    if rel=='entry.rs':s=s.replace(needle,needle+'\n                                | "ComputationalGeometry.revolve_profile"',1)
    else:s=s.replace(needle+'.into(),',needle+'.into(),\n            "ComputationalGeometry.revolve_profile".into(),',1)
    put(P+rel,s)
print('Root common integration staged',len(rows))
