from pathlib import Path
assert not (Path(__file__).resolve().parent/"PREPARATION-FROZEN.json").exists(), "Historical one-shot staging is frozen"
from staging import read,put,replace
P=Path('C:/github/poet/native/src/graphical')
C=Path('C:/github/qualiaDB/crates/qualia-core-db/src')
replace(C/'specialized_libs/computational_geometry/mod.rs','pub mod ddg_operators;','pub mod ddg_operators;\npub mod uv_atlas;')
replace(P/'surfaces/mod.rs','pub mod binding;','pub mod uv_atlas;\npub mod detail;\npub mod texture_inputs;\npub mod resolution;\npub mod inspection;\npub mod binding;')
replace(P/'authoring/mod.rs','mod surface_controls;','mod surface_controls;\nmod uv_atlas_controls;\nmod detail_controls;\nmod surface_tools;')
replace(P/'authoring/mod.rs','pub struct PartView {','pub struct PartView {\n    pub uv_atlas: Option<crate::graphical::surfaces::uv_atlas::AtlasDraft>,\n    pub detail: Option<crate::graphical::surfaces::detail::DetailRecipe>,')
replace(P/'authoring/mod.rs','pub enum AuthoringEdit {','pub enum AuthoringEdit {\n    UvAtlas { id: String, atlas: Option<crate::graphical::surfaces::uv_atlas::AtlasDraft> },\n    TextureDetail { material_id: u64, detail: Option<crate::graphical::surfaces::detail::DetailRecipe> },')
replace(P/'authoring/source.rs','result.push(PartView {','result.push(PartView {\n            uv_atlas: p.get("uv_atlas").filter(|v| !matches!(v,Value::Null)).map(crate::graphical::surfaces::uv_atlas::from_value).transpose()?,\n            detail: p.get("detail").filter(|v| !matches!(v,Value::Null)).map(crate::graphical::surfaces::detail::DetailRecipe::from_value).transpose()?,')
replace(P/'authoring/source.rs','let body = match edit {','let body = match edit {\n        AuthoringEdit::UvAtlas { id, atlas } => inventory.replace(id, BTreeMap::from([("uv_atlas", atlas.as_ref().map(crate::graphical::surfaces::uv_atlas::source).transpose()?.unwrap_or_else(||"null".into()))]))?,\n        AuthoringEdit::TextureDetail { material_id, detail } => inventory.shared_part_fields(*material_id, BTreeMap::from([("detail", detail.as_ref().map(|d| d.source_expression()).transpose()?.unwrap_or_else(||"null".into()))]))?,')
replace(P/'authoring/source.rs','if source.len() > 8192 {','if source.len() > 60 * 1024 {')
replace(P/'authoring/assembly_source.rs','    pub(super) fn structural(&self, edit: &AuthoringEdit)', '''    pub(super) fn shared_part_fields(&self,id:u64,updates:BTreeMap<&str,String>)->Result<String,String> {
        let mut replacements=Vec::new();
        for part in &self.parts {
            if integer(field(record(field(part,"material")?)?,"id")?)? == id {
                replacements.push((text(field(part,"id")?)?,copied(part,"p",updates.clone())?));
            }
        }
        if replacements.is_empty() {return Err("Unknown selected material".into());}
        self.mapped(replacements)
    }
    pub(super) fn structural(&self, edit: &AuthoringEdit)''')
# Geometry replacement must not silently invalidate an atlas recipe.
needle='            AuthoringEdit::ReplaceGeometry { id, geometry } => {'
replace(P/'authoring/assembly_source.rs',needle,needle+'\n                if self.selected(id)?.get("uv_atlas").is_some_and(|v| !matches!(v,Value::Null)) { return Err("Remove the editable UV atlas explicitly before replacing geometry".into()); }')
replace(P/'assembly/admission.rs','pub fn admit_assembly(value: &Value, limits: PreviewLimits) -> Result<CompiledAssembly, String> {','''pub fn admit_assembly(value: &Value, limits: PreviewLimits) -> Result<CompiledAssembly, String> {
    admit_assembly_with_materials(value,limits,&BTreeMap::new())
}
pub(crate) fn admit_assembly_with_materials(value:&Value, limits:PreviewLimits, overrides:&BTreeMap<u64,qualia_core_db::container_10d::MaterialRecord>)->Result<CompiledAssembly,String> {''')
s=read(P/'assembly/admission.rs');needle='"uv_projection",';assert needle in s
s=s.replace(needle,'"uv_atlas", "detail", '+needle,1);put(P/'assembly/admission.rs',s)
replace(P/'assembly/admission.rs','let (v, t, out) = geometry_counts(record(get(part, "geometry")?)?, limits)?;','''let (v, t, mut out) = geometry_counts(record(get(part, "geometry")?)?, limits)?;
        if let Some(atlas)=part.get("uv_atlas").filter(|v|!matches!(v,Value::Null)) {
            if part.get("uv_projection").is_some_and(|v|!matches!(v,Value::Null)) {return Err("Remove planar projection explicitly before applying an atlas".into());}
            if part.get("uv_transform").map(crate::graphical::surfaces::uv::from_value).transpose()?.is_some_and(|t|t!=qualia_core_db::container_10d::TextureTransform::IDENTITY) {return Err("Reset the part UV affine transform explicitly before applying an atlas".into());}
            let d=crate::graphical::surfaces::uv_atlas::from_value(atlas)?;
            out=crate::graphical::surfaces::uv_atlas::preflight(&d,v,t,crate::graphical::surfaces::inspection::atlas_limits(limits))?;
        }''')
replace(P/'assembly/admission.rs','let mesh = mesh_from_invoke(geometry, limits)?;','let mut mesh = mesh_from_invoke(geometry, limits)?;')
replace(P/'assembly/admission.rs','let normals = g\n            .get','let mut normals = g\n            .get')
needle='        if let Some(v) = p.get("uv_projection").filter(|v| !matches!(v, Value::Null)) {'
replace(P/'assembly/admission.rs',needle,'''        if let Some(v)=p.get("uv_atlas").filter(|v| !matches!(v,Value::Null)) {
            let d=crate::graphical::surfaces::uv_atlas::from_value(v)?;
            let applied=crate::graphical::surfaces::uv_atlas::apply(&d,&mesh,normals.as_deref(),crate::graphical::surfaces::inspection::atlas_limits(limits),&||false)?;
            mesh=applied.mesh;normals=applied.normals;uv0=Some(applied.uv0);
        }
'''+needle)
replace(P/'assembly/admission.rs','let material = materials::material_from_value(get(p, "material")?)?;','let authored = materials::material_from_value(get(p, "material")?)?;\n        let material = overrides.get(&authored.id).copied().unwrap_or(authored);')
replace(P/'assembly/mod.rs','pub use admission::{admit_assembly, validate_compiled};','pub use admission::{admit_assembly, validate_compiled};\npub(crate) use admission::admit_assembly_with_materials;')
# Resource adapter additions are implemented separately, with existing private snapshot ownership.
replace(P/'surfaces/resources.rs','impl TextureResources {','''impl TextureResources {
    pub(crate) fn detail_input(&self,digest:[u8;32],side:u32)->Result<super::texture_inputs::DetailInput,String> {
        super::texture_inputs::decode(&self.bytes,self.limits,digest,side)
    }
    pub(crate) fn with_dependencies(&self,deps:&[TextureDependency])->Result<Self,String> {
        let bytes=super::texture_inputs::merge(&self.bytes,self.limits,deps)?;
        Self::from_hmc_bytes(&bytes,self.limits)
    }
    pub(crate) fn source_dependency(&self,digest:[u8;32])->Result<TextureDependency,String> {
        super::texture_inputs::dependency(&self.bytes,self.limits,digest)
    }
''')
# Effective surfaces are validated before accepted source promotion, not after rendering.
replace(P/'surface_host.rs','        if let Some(Value::List(parts)) = record.get("parts") {','''        if let Some(Value::List(parts)) = record.get("parts") {
            if parts.iter().any(|p|matches!(p,Value::Record(r) if r.get("detail").is_some_and(|d|!matches!(d,Value::Null)))) {
                let resolved=crate::graphical::surfaces::resolution::resolve(value,resources)?;
                let scene=crate::graphical::assembly::admit_assembly_with_materials(value,crate::graphical::preview::PreviewLimits::default(),&resolved.materials)?;
                return crate::graphical::surfaces::resources::validate_material_resources(&scene.materials,resolved.resources.as_ref());
            }''')
replace(P/'editor_contract.rs','            let scene = crate::graphical::assembly::admit_assembly(value, self.limits)?;','''            let resolved=crate::graphical::surfaces::resolution::resolve(value,resources)?;
            let scene = crate::graphical::assembly::admit_assembly_with_materials(value,self.limits,&resolved.materials)?;''')
replace(P/'editor_contract.rs','                resources,\n            );','                resolved.resources.as_ref(),\n            );')
# Explicit selected-part inspection traverses the same source and authority owner.
replace(P/'editor_contract.rs','pub trait EditorHost: Send {','''pub trait EditorHost: Send {
    fn inspect_surface(&self,_id:&str,_expected:SourceRef)->Result<crate::graphical::surfaces::inspection::SurfaceInspection,String> {Err("Native surface inspection unavailable".into())}
''')
replace(P/'controller.rs','impl EditorHost for CanonicalEditorHost {','''impl EditorHost for CanonicalEditorHost {
    fn inspect_surface(&self,id:&str,expected:SourceRef)->Result<crate::graphical::surfaces::inspection::SurfaceInspection,String> {
        self.expected(&expected)?;
        if self.reconciliation_required || !self.execution_epoch.is_current() {return Err("Rebuild accepted execution before surface inspection".into());}
        let parts=self.session.value("poet_parts").ok_or("No authored assembly")?;
        crate::graphical::surfaces::inspection::inspect(parts,id,expected,self.surface_resources.as_ref(),self.host.vertex_limit())
    }
''')
# Compile actual resolved material maps and retain every imported base dependency.
replace(P/'authoring/controller.rs','let scene = crate::graphical::assembly::admit_assembly(\n            &result,','let resolved=crate::graphical::surfaces::resolution::resolve(&result,self.surface_resources.as_ref())?;\n        let scene = crate::graphical::assembly::admit_assembly_with_materials(\n            &result,')
replace(P/'authoring/controller.rs','indices: self\n                    .host\n                    .vertex_limit()\n                    .checked_mul(6)\n                    .ok_or("Compiled assembly index budget overflow")?,\n            },\n        )?;','indices: self\n                    .host\n                    .vertex_limit()\n                    .checked_mul(6)\n                    .ok_or("Compiled assembly index budget overflow")?,\n            },\n            &resolved.materials,\n        )?;')
replace(P/'authoring/controller.rs','texture_dependencies: crate::graphical::surface_host::dependencies(\n                    &scene.materials,\n                    self.surface_resources.as_ref(),\n                )?,','texture_dependencies: resolved.dependencies(&scene.materials)?,')
