"""Stage original Poet intent adapters and package controls without live writes."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent;OUT=HERE/'integration';POET=Path('C:/github/poet');P='native/src/graphical/'
rows=json.loads((OUT/'target-manifest.json').read_text())
def save(rel,text):
    target=POET/rel;data=target.read_bytes() if target.exists() else None;stage=OUT/'after'/rel
    if data is not None:
        before=OUT/'before'/rel;before.parent.mkdir(parents=True,exist_ok=True);before.write_bytes(data)
    stage.parent.mkdir(parents=True,exist_ok=True);stage.write_text(text,encoding='utf-8',newline='\n')
    row=dict(target=str(target),stage=str(stage),before_sha256=hashlib.sha256(data).hexdigest() if data is not None else None,after_sha256=hashlib.sha256(stage.read_bytes()).hexdigest())
    old=next((r for r in rows if r['target']==str(target)),None)
    if old:rows.remove(old)
    rows.append(row)
def read(rel):return (POET/rel).read_text(encoding='utf-8')
s=read(P+'native_dispatch.rs')
marker='                AuthoringEdit::Parameter { key, value } => {'
extra='''                AuthoringEdit::Reparent{id,parent}=>json!({"command":"authoring.reparent","id":id,"parent":parent}),
                AuthoringEdit::RemoveLeaf{id}=>json!({"command":"authoring.remove_leaf","id":id}),
                AuthoringEdit::ReplaceGeometry{id,geometry}=>json!({"command":"authoring.replace_geometry","id":id,"geometry":geometry}),
                AuthoringEdit::AddGeometry{id,semantic_id,material_from,geometry}=>json!({"command":"authoring.add_geometry","id":id,"semantic_id":semantic_id,"material_from":material_from,"geometry":geometry}),
                AuthoringEdit::UniqueMaterial{id,material_id}=>json!({"command":"authoring.unique_material","id":id,"material_id":material_id}),
'''
assert s.count(marker)==1;s=s.replace(marker,extra+marker,1);save(P+'native_dispatch.rs',s)
s=read(P+'poet_workspace/intents.rs');start=s.index('#[derive(Deserialize)]');end=s.index('struct Bounded {',start)
commands=s[start:end].replace('enum Command {','pub(super) enum Command {',1)
commands=commands.replace('    fn action_type(','    pub(super) fn action_type(',1).replace('    fn scope(','    pub(super) fn scope(',1).replace('    fn action(','    pub(super) fn action(',1)
marker='    #[serde(rename = "authoring.compile")]'
extra='''    #[serde(rename="authoring.reparent")]
    Reparent{id:String,parent:Option<String>},
    #[serde(rename="authoring.remove_leaf")]
    RemoveLeaf{id:String},
    #[serde(rename="authoring.replace_geometry")]
    ReplaceGeometry{id:String,geometry:crate::graphical::authoring::GeometryRecipe},
    #[serde(rename="authoring.add_geometry")]
    AddGeometry{id:String,semantic_id:u64,material_from:String,geometry:crate::graphical::authoring::GeometryRecipe},
    #[serde(rename="authoring.unique_material")]
    UniqueMaterial{id:String,material_id:u64},
'''
assert commands.count(marker)==1;commands=commands.replace(marker,extra+marker,1)
commands=commands.replace('Self::Transform { .. } | Self::Material { .. } | Self::Parameter { .. } => {','Self::Reparent{..}|Self::RemoveLeaf{..}|Self::ReplaceGeometry{..}|Self::AddGeometry{..}|Self::UniqueMaterial{..}|Self::Transform { .. } | Self::Material { .. } | Self::Parameter { .. } => {',1)
marker='            Self::Compile {\n'
extra='''            Self::Reparent{id,parent}=>{
                text(&id,128)?;if let Some(p)=&parent{text(p,128)?;}
                Action::AuthoringEdit{edit:AuthoringEdit::Reparent{id,parent},expected}
            }
            Self::RemoveLeaf{id}=>{text(&id,128)?;Action::AuthoringEdit{edit:AuthoringEdit::RemoveLeaf{id},expected}}
            Self::ReplaceGeometry{id,geometry}=>{text(&id,128)?;geometry.validate()?;
                Action::AuthoringEdit{edit:AuthoringEdit::ReplaceGeometry{id,geometry},expected}}
            Self::AddGeometry{id,semantic_id,material_from,geometry}=>{
                text(&id,128)?;text(&material_from,128)?;geometry.validate()?;
                if semantic_id==0 || semantic_id>i64::MAX as u64{return Err("Part identifier outside positive control profile".into());}
                Action::AuthoringEdit{edit:AuthoringEdit::AddGeometry{id,semantic_id,material_from,geometry},expected}
            }
            Self::UniqueMaterial{id,material_id}=>{
                text(&id,128)?;if material_id==0 || material_id>i64::MAX as u64{return Err("Material identifier outside positive control profile".into());}
                Action::AuthoringEdit{edit:AuthoringEdit::UniqueMaterial{id,material_id},expected}
            }
'''
assert commands.count(marker)==1;commands=commands.replace(marker,extra+marker,1)
save(P+'poet_workspace/native_commands.rs','//! Bounded command decode for the original Poet native intent owner.\nuse super::*;\n'+commands)
s=s[:start]+'#[path="native_commands.rs"]\nmod commands;\nuse commands::Command;\n'+s[end:];save(P+'poet_workspace/intents.rs',s)
s=read(P+'package_controls.rs').replace('use georaster_create_view::GeoRasterCreateView;','use georaster_create_view::GeoRasterCreateView;\n#[path="package_typed_10d_view.rs"]\nmod typed_10d_view;\nuse typed_10d_view::Typed10dControls;',1)
marker='                        p{"SHA256 {hex_digest(&digest)}"}'
extra='''
                        {let key=format!("{}:{}:{}",name(),resource.key,hex_digest(&package.digest));
                            rsx!{Typed10dControls{key:"{key}",host:host.clone(),current:current.clone(),
                                request:super::checkpoints::ResourceInspectionRequest{filename:name(),package_digest:file_digest,key:resource.key.clone(),resource_digest:digest},
                                busy,status,on_saved:move |_|{inventory.set(None);inspection.set(None);}}}}
'''
assert s.count(marker)==1;s=s.replace(marker,marker+extra,1);save(P+'package_controls.rs',s)
rel=P+'package_typed_10d_view.rs';save(rel,(OUT/'after'/rel).read_text())
(OUT/'target-manifest.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
print(json.dumps(dict(staged_targets=len(rows),live_writes=False)))
