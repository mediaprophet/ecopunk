from pathlib import Path
assert not (Path(__file__).resolve().parent/"PREPARATION-FROZEN.json").exists(), "Historical one-shot staging is frozen"
from staging import read,put,replace
P=Path('C:/github/poet/native/src/graphical');C=Path('C:/github/qualiaDB/crates/qualia-core-db/src')
replace(P/'surfaces/inspection.rs','#[derive(Clone)]','#[derive(Clone,PartialEq)]')
replace(P/'surfaces/resolution.rs','    let mut inputs=BTreeMap::<u64,(MaterialRecord,Option<DetailRecipe>)>::new();','    resolve_parts(parts,resources)\n}\npub fn resolve_parts(parts:&[Value],resources:Option<&TextureResources>)->Result<ResolvedSurfaces,String> {\n    if parts.len()>256{return Err("Surface part working bound exceeded".into());}\n    let mut inputs=BTreeMap::<u64,(MaterialRecord,Option<DetailRecipe>)>::new();')
# Preserve every authored map, not just the effective maps after overrides.
replace(P/'surfaces/resolution.rs','    let mut cost_visits=0usize;','''    let mut source_digests=std::collections::BTreeSet::new();
    for (m,recipe) in inputs.values() {
        for role in super::TextureRole::ALL {let d=role.digest(m);if d!=[0;32]{source_digests.insert(d);}}
        if let Some(DetailRecipe::Paint(p))=recipe {source_digests.insert(super::binding::parse_digest(&p.base_sha256)?);}
    }
    let mut source_bytes=0usize;
    for d in &source_digests {source_bytes=source_bytes.checked_add(resources.ok_or("Original source texture resources unavailable")?.source_byte_len(*d)?).ok_or("Source input size overflow")?;}
    if source_bytes>SurfaceTextureLimits::default().encoded_bytes {return Err("Original texture inputs exceed aggregate encoded working profile".into());}
    let mut cost_visits=0usize;''')
replace(P/'surfaces/resolution.rs','.and_then(|n|n.checked_add(l.encoded_bytes))','.and_then(|n|n.checked_add(source_bytes))')
replace(P/'surfaces/resolution.rs','    for (id,(mut material,recipe)) in inputs {','''    for d in source_digests {let dep=resources.ok_or("Source texture resources unavailable")?.source_dependency(d)?;originals.insert(d,dep);}
    for (id,(mut material,recipe)) in inputs {''')
replace(P/'surfaces/resources.rs','    pub(crate) fn snapshot_bytes(&self)->usize {self.bytes.len()}','''    pub(crate) fn snapshot_bytes(&self)->usize {self.bytes.len()}
    pub(crate) fn source_byte_len(&self,digest:[u8;32])->Result<usize,String> {super::texture_inputs::length(&self.bytes,self.limits,digest)}
    pub(crate) fn retained_inputs(&self)->Result<Vec<qualia_core_db::poet_host::invoke::studio_store::checkpoint::ResourceData>,String> {super::texture_inputs::retained(&self.bytes,self.limits)}
    pub(crate) fn preview_input(&self,digest:[u8;32])->Result<super::texture_inputs::DetailInput,String> {super::texture_inputs::decode_any(&self.bytes,self.limits,digest,None)}''')
replace(C/'poet_host/invoke/studio_checkpoint/mod.rs','pub use codec::{decode, encode};','pub use codec::{decode, encode};\n/// Canonical reserved source entry; raw resource retention must never replace it.\npub const SOURCE_RESOURCE_KEY: &str = snapshot::SOURCE_KEY;')
replace(P/'surface_host.rs','''                let scene=crate::graphical::assembly::admit_assembly_with_materials(value,crate::graphical::preview::PreviewLimits::default(),&resolved.materials)?;
                return crate::graphical::surfaces::resources::validate_material_resources(&scene.materials,resolved.resources.as_ref());''','''                return crate::graphical::surfaces::resources::validate_material_resources(&resolved.materials.values().copied().collect::<Vec<_>>(),resolved.resources.as_ref());''')
replace(P/'editor_contract.rs','impl PreviewProvider for MeshPreviewProvider {','''impl PreviewProvider for MeshPreviewProvider {
    fn validate_with_resources(&self,value:&Value,resources:Option<&crate::graphical::surfaces::TextureResources>)->Result<(),String> {
        if crate::graphical::assembly::is_assembly(value) {
            let resolved=crate::graphical::surfaces::resolution::resolve(value,resources)?;
            let scene=crate::graphical::assembly::admit_assembly_with_materials(value,self.limits,&resolved.materials)?;
            crate::graphical::surfaces::resources::validate_material_resources(&scene.materials,resolved.resources.as_ref())
        } else {self.validate(value)?;crate::graphical::surface_host::validate_value(value,resources)}
    }
''')
# Allocate the one aggregate work budget across all atlas parts before any solve.
replace(P/'assembly/admission.rs','    let mut input_vertices =','    let atlas_count=parts.iter().filter(|p|matches!(p,Value::Record(r) if r.get("uv_atlas").is_some_and(|v|!matches!(v,Value::Null)))).count().max(1);\n    let mut atlas_limits=crate::graphical::surfaces::inspection::atlas_limits(limits);\n    atlas_limits.work_units/=atlas_count as u64;\n    let mut input_vertices =')
s=read(P/'assembly/admission.rs');s=s.replace('crate::graphical::surfaces::inspection::atlas_limits(limits)','atlas_limits');s=s.replace('let mut atlas_limits=atlas_limits;','let mut atlas_limits=crate::graphical::surfaces::inspection::atlas_limits(limits);');put(P/'assembly/admission.rs',s)
# Typed complete-context draft cursor; Debug of an opaque epoch is not an identifier.
replace(P/'authoring/surface_tools.rs','use dioxus::prelude::*;','''use dioxus::prelude::*;
#[derive(Default)]
pub(crate) struct DraftCursor {last:Option<(Context,String)>,generation:u64}
impl DraftCursor {
    pub(crate) fn token(&mut self,context:Context,part:&str)->Option<String> {
        let next=(context,part.to_owned());
        if self.last.as_ref()!=Some(&next) {self.generation=self.generation.checked_add(1)?;self.last=Some(next);}
        Some(self.generation.to_string())
    }
}''')
replace(P/'authoring/surface_tools.rs','    let token=format!("{:?}:{}",current.source,id);','    let cursor=use_hook(||std::rc::Rc::new(std::cell::RefCell::new(DraftCursor::default())));\n    let token=cursor.borrow_mut().token(Context::from_snapshot(&current),&id);\n    let context_valid=token.is_some();let token=token.unwrap_or_default();')
replace(P/'authoring/surface_tools.rs','let fresh=fresh&&part.is_some()&&!busy();','let fresh=fresh&&context_valid&&part.is_some()&&!busy();')
# Raw retained inputs are an internal same-owner compilation projection.
signature='fn retained_surface_inputs(&self,expected:SourceRef)->Result<Vec<qualia_core_db::poet_host::invoke::studio_store::checkpoint::ResourceData>,String>'
replace(P/'editor_contract.rs','pub trait EditorHost: Send {','pub trait EditorHost: Send {\n    '+signature+' {let _=expected;Ok(Vec::new())}\n')
replace(P/'controller.rs','impl EditorHost for CanonicalEditorHost {','impl EditorHost for CanonicalEditorHost {\n    '+signature+' {self.expected(&expected)?;if self.reconciliation_required||!self.execution_epoch.is_current(){return Err("Rebuild accepted source before retaining resource inputs".into());}self.surface_resources.as_ref().map(|r|r.retained_inputs()).transpose().map(|v|v.unwrap_or_default())}\n')
for file,owner,target in [('session_host.rs','SessionEditorHost','self.current()?'),('shared_host.rs','SharedEditorHost','self.current()?'),('package/mod.rs','ResourceImportHost','self.inner'),('checkpoints/editor.rs','CheckpointEditorHost','self.inner'),('entry.rs','DiagnosticEditor','self.inner')]:
    replace(P/file,f'impl EditorHost for {owner} {{',f'impl EditorHost for {owner} {{\n    {signature} {{ {target}.retained_surface_inputs(expected) }}\n')
replace(P/'cooperation/editor.rs','impl EditorHost for AuthorizedEditorHost {','impl EditorHost for AuthorizedEditorHost {\n    '+signature+' {with_authorized(self.owner.as_ref(),&self.scope,Some(&expected),AuthoringAction::ReadSource,self.limits,|_,_|self.inner.retained_surface_inputs(expected.clone()))}\n')
