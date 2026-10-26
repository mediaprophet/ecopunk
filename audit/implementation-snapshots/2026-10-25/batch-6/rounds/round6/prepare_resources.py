from stage import *
assert not (HERE/"PREPARATION-FROZEN.json").exists(), "Historical preparation only; install pinned source images, do not rerun generators"
# Resume the common script only at its previously unexecuted worker routing.
script=(HERE/'prepare_common.py').read_text();exec(script[script.index("s=get(P+'jobs.rs')"):])
put(P+'surface_host.rs',r'''//! Native package selection; content hashes do not grant authority.
use crate::graphical::{surfaces::TextureResources,preview::PreviewLimits};
use qualia_core_db::{container_10d::MaterialRecord,render::assets::TextureDependency};
use vibe::Value;
#[derive(Clone,Debug,serde::Serialize,serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfacePackageRequest {pub filename:String,pub digest:[u8;32]}
impl SurfacePackageRequest {pub fn validate(&self)->Result<(),String>{
 if self.filename.is_empty()||self.filename.len()>128||self.filename.contains('\0')||self.digest==[0;32]{return Err("Select a bounded HMC filename and its inspected SHA256".into());} Ok(())
}}
pub fn validate_value(value:&Value,resources:Option<&TextureResources>)->Result<(),String>{
 if crate::graphical::assembly::is_assembly(value){let scene=crate::graphical::assembly::admit_assembly(value,PreviewLimits::default())?;return crate::graphical::surfaces::resources::validate_material_resources(&scene.materials,resources);}
 if let Value::Record(record)=value {let material=if let Some(m)=record.get("material"){Some(m)}else if record.contains_key("base_color"){Some(value)}else{None};if let Some(m)=material{let m=crate::graphical::materials::material_from_value(m)?;crate::graphical::surfaces::resources::validate_material_resources(&[m],resources)?;}} Ok(())
}
pub fn dependencies(materials:&[MaterialRecord],resources:Option<&TextureResources>)->Result<Vec<TextureDependency>,String>{
 crate::graphical::surfaces::resources::validate_material_resources(materials,resources)?;
 match resources {Some(r)=>r.dependencies(materials),None=>Ok(Vec::new())}
}
'''.replace("'\\0'","'\0'"))
put(P+'checkpoints/surface_resources.rs',r'''//! Bounded immutable HMC selection inside the existing checkpoint owner.
use super::*;
use crate::graphical::{surface_host::SurfacePackageRequest,preview::SourceRef,surfaces::{TextureResources,SurfaceTextureLimits}};
use std::io::Read;
impl CheckpointService {
 pub fn surface_resources(&self,request:&SurfacePackageRequest,expected:&SourceRef)->Result<TextureResources,String>{
  request.validate()?;expected.check_current()?;
  if expected.artifact_id!=self.binding.source_id{return Err("Foreign surface resource source".into());}
  let limits=SurfaceTextureLimits::default();let maximum=self.limits.file_bytes.min(limits.package_bytes);
  let file=std::fs::File::open(self.path(&request.filename)?).map_err(|e|e.to_string())?;
  let length=usize::try_from(file.metadata().map_err(|e|e.to_string())?.len()).map_err(|_|"HMC length overflow")?;
  if length>maximum{return Err("Surface HMC exceeds native snapshot budget".into());}
  let mut bytes=Vec::new();bytes.try_reserve_exact(length.checked_add(1).ok_or("Surface capacity overflow")?).map_err(|_|"Surface allocation refused")?;
  file.take(maximum as u64+1).read_to_end(&mut bytes).map_err(|e|e.to_string())?;
  if bytes.len()>maximum{return Err("Surface HMC grew beyond snapshot budget".into());}
  let checkpoint=core::decode(&bytes,self.limits)?;self.matches(checkpoint.artifact())?;
  if checkpoint.file_digest()!=Some(request.digest){return Err("Surface HMC digest changed; refresh inventory".into());}
  self.check_release_input(&request.filename,request.digest)?;drop(checkpoint);
  let resources=TextureResources::from_hmc_bytes(&bytes,limits)?;expected.check_current()?;Ok(resources)
 }
}
''')
s=get(P+'checkpoints/mod.rs');put(P+'checkpoints/mod.rs',s.replace('mod resources;','mod resources;\nmod surface_resources;',1))
# Identity is a reversible declaration and must not lock future plain-geometry edits.
s=get(P+'authoring/assembly_source.rs').replace('selected.contains_key("uv_transform")','selected.get("uv_transform").map(crate::graphical::surfaces::uv::from_value).transpose()?.is_some_and(|t|t!=qualia_core_db::container_10d::TextureTransform::IDENTITY)');put(P+'authoring/assembly_source.rs',s)
print('Root resource and worker routing staged')
