from stage import *
assert not (HERE/"PREPARATION-FROZEN.json").exists(), "Historical preparation only; install pinned source images, do not rerun generators"
import re
s=get(P+'surface_host.rs');s=re.sub(r'self.filename.contains\(.*?\)', 'self.filename.as_bytes().contains(&0)', s)
# Resource validation does not impose a second fixed geometry-quality ceiling.
start=s.index('pub fn validate_value(');end=s.index('pub fn dependencies(',start)
s=s[:start]+'''pub fn validate_value(value:&Value,resources:Option<&TextureResources>)->Result<(),String>{
 if let Value::Record(record)=value {
  if let Some(Value::List(parts))=record.get("parts"){
   if parts.len()>256{return Err("Surface part working bound exceeded".into());}
   let mut materials=std::collections::BTreeMap::new();
   for part in parts {let Value::Record(part)=part else{return Err("Expected assembly part record".into());};
    let m=crate::graphical::materials::material_from_value(part.get("material").ok_or("Part lacks material")?)?;
    if let Some(old)=materials.insert(m.id,m){if old!=m{return Err("Conflicting material identifier declarations".into());}}
   }
   return crate::graphical::surfaces::resources::validate_material_resources(&materials.values().copied().collect::<Vec<_>>(),resources);
  }
  let material=if let Some(m)=record.get("material"){Some(m)}else if record.contains_key("base_color"){Some(value)}else{None};
  if let Some(m)=material{let m=crate::graphical::materials::material_from_value(m)?;crate::graphical::surfaces::resources::validate_material_resources(&[m],resources)?;}
 }Ok(())
}
'''+s[end:];put(P+'surface_host.rs',s)
s=get(P+'authoring/mod.rs').replace('mod surface_controls;','mod surface_controls;\nmod surface_binding_controls;',1);put(P+'authoring/mod.rs',s)
s=get(P+'authoring/controls.rs');s=s.replace('    let data = inspected();','''    let profile_host=host.clone();let profile_buffer=buffer.clone();let profile_source=current.source.clone();
    let cad_host=host.clone();let cad_buffer=buffer.clone();let cad_source=current.source.clone();
    let surface_host=host.clone();let surface_buffer=buffer.clone();let surface_source=current.source.clone();
    let binding_host=host.clone();let binding_buffer=buffer.clone();let binding_source=current.source.clone();
    let structure_source=current.source.clone();let transform_source=current.source.clone();let material_source=current.source.clone();let param_source=current.source.clone();let compile_source=current.source.clone();
    let data = inspected();''',1)
s=s.replace('expected:view().source},transform_host','expected:transform_source.clone()},transform_host',1).replace('expected:view().source},material_host','expected:material_source.clone()},material_host',1).replace('expected:view().source},param_host','expected:param_source.clone()},param_host',1).replace('expected:view().source},compile_host','expected:compile_source.clone()},compile_host',1)
s=s.replace('edit, expected: view().source }, structure_host','edit, expected: structure_source.clone() }, structure_host',1)
marker='            if let Some(data)=data{'
s=s.replace(marker,'''            super::surface_binding_controls::SurfaceBindingControls{busy,onbind:move|request|send(Action::BindSurfacePackage{request,expected:binding_source.clone()},binding_host.clone(),view,draft,status,image,busy,inspected,binding_buffer.clone(),false)}
'''+marker,1)
marker='                super::structure_controls::StructureControls {'
s=s.replace(marker,'''                super::profile_controls::ProfileControls{selected,inspected,fresh,onedit:move|edit|send(Action::AuthoringEdit{edit,expected:profile_source.clone()},profile_host.clone(),view,draft,status,image,busy,inspected,profile_buffer.clone(),true)}
                super::cad_controls::CadControls{selected,inspected,fresh,onedit:move|edit|send(Action::AuthoringEdit{edit,expected:cad_source.clone()},cad_host.clone(),view,draft,status,image,busy,inspected,cad_buffer.clone(),true)}
                super::surface_controls::SurfaceControls{selected,inspected,fresh,onedit:move|edit|send(Action::AuthoringEdit{edit,expected:surface_source.clone()},surface_host.clone(),view,draft,status,image,busy,inspected,surface_buffer.clone(),true)}
'''+marker,1);put(P+'authoring/controls.rs',s)
put(P+'authoring/surface_binding_controls.rs',r'''//! Explicit texture snapshot binding; package names are native-host selected paths.
use crate::graphical::surface_host::SurfacePackageRequest;
use dioxus::prelude::*;
#[component]
pub(super) fn SurfaceBindingControls(busy:Signal<bool>,onbind:EventHandler<SurfacePackageRequest>)->Element{
 let mut name=use_signal(||"working.hmc".to_string());let mut digest=use_signal(String::new);let mut error=use_signal(String::new);
 rsx!{details{summary{"Bind verified HMC texture resources"}
 p{"Use a project HMC filename and its inspected SHA-256. Preview and compilation use immutable verified bytes; edits to that file do not change the bound snapshot. After reopening textured source, bind resources before explicit rebuild."}
 input{disabled:busy(),value:"{name}",aria_label:"Texture resource HMC filename",oninput:move|e|name.set(e.value())}
 input{disabled:busy(),value:"{digest}",aria_label:"Inspected texture resource HMC SHA256",oninput:move|e|digest.set(e.value())}
 button{disabled:busy(),onclick:move |_|{
  let result=(||->Result<SurfacePackageRequest,String>{let text=digest();if text.len()!=64||!text.is_ascii(){return Err("Expected 64 hexadecimal SHA256 characters".into());}let mut hash=[0u8;32];for(i,byte)in hash.iter_mut().enumerate(){*byte=u8::from_str_radix(&text[i*2..i*2+2],16).map_err(|_|"Invalid SHA256")?;}let r=SurfacePackageRequest{filename:name(),digest:hash};r.validate()?;Ok(r)})();
  match result{Ok(r)=>{error.set(String::new());onbind.call(r)},Err(e)=>error.set(e)}
 },"Bind immutable texture snapshot"}
 p{"{error}"}
 }}
}
''')
s=get(P+'native_dispatch.rs');s=s.replace('        Action::InspectTyped10d { expected, .. }','        Action::BindSurfacePackage{expected,..}\n        | Action::InspectTyped10d { expected, .. }',1)
s=s.replace('                AuthoringEdit::Reparent { id, parent } =>','''                AuthoringEdit::Surface{material_id,surface}=>json!({"command":"authoring.surface","material_id":material_id,"surface":surface}),
                AuthoringEdit::UvTransform{id,transform}=>json!({"command":"authoring.uv_transform","id":id,"transform":crate::graphical::surfaces::wire::UvTransformWire::from(*transform)}),
                AuthoringEdit::UvProjection{id,projection}=>json!({"command":"authoring.uv_projection","id":id,"projection":projection}),
                AuthoringEdit::Reparent { id, parent } =>''',1)
s=s.replace('    let (kind, parameters) = match &action {','''    let (kind, parameters) = match &action {
        Action::BindSurfacePackage{request,..}=>(ActionType::Query,json!({"command":"authoring.bind_surface_package","request":request})),''',1);put(P+'native_dispatch.rs',s)
s=get(P+'poet_workspace/native_commands.rs');needle='    #[serde(rename = "authoring.inspect")]'
s=s.replace(needle,'''    #[serde(rename="authoring.surface")] Surface{material_id:u64,surface:crate::graphical::surfaces::SurfaceDraft},
    #[serde(rename="authoring.uv_transform")] UvTransform{id:String,transform:crate::graphical::surfaces::wire::UvTransformWire},
    #[serde(rename="authoring.uv_projection")] UvProjection{id:String,projection:Option<crate::graphical::surfaces::uv::PlanarUvDraft>},
    #[serde(rename="authoring.bind_surface_package")] BindSurface{request:crate::graphical::surface_host::SurfacePackageRequest},
'''+needle,1)
s=s.replace('Self::Inspect | Self::Check { .. } | Self::Refresh =>','Self::BindSurface{..} | Self::Inspect | Self::Check { .. } | Self::Refresh =>',1)
s=s.replace('            Self::Inspect => "Studio.inspect",','            Self::BindSurface{..}=>"Studio.inspect",\n            Self::Surface{..}|Self::UvTransform{..}|Self::UvProjection{..}=>"Studio.edit",\n            Self::Inspect => "Studio.inspect",',1)
s=s.replace('            Self::Inspect => Action::AuthoringInspect(expected),','''            Self::BindSurface{request}=>{request.validate()?;Action::BindSurfacePackage{request,expected}}
            Self::Surface{material_id,surface}=>{if material_id==0{return Err("Material identifier must be nonzero".into());}surface.updates()?;Action::AuthoringEdit{edit:AuthoringEdit::Surface{material_id,surface},expected}}
            Self::UvTransform{id,transform}=>{text(&id,128)?;let transform=transform.try_into()?;Action::AuthoringEdit{edit:AuthoringEdit::UvTransform{id,transform},expected}}
            Self::UvProjection{id,projection}=>{text(&id,128)?;if let Some(p)=&projection{crate::graphical::surfaces::uv::projection_source(p)?;}Action::AuthoringEdit{edit:AuthoringEdit::UvProjection{id,projection},expected}}
            Self::Inspect => Action::AuthoringInspect(expected),''',1);put(P+'poet_workspace/native_commands.rs',s)
print('Root native intent and control integration staged')
