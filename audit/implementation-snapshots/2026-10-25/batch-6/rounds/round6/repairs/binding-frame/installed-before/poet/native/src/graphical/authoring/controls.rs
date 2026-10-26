//! Native part inspector and explicit source commands sharing the existing worker.
use super::{AuthoringEdit, AuthoringView};
use crate::graphical::{
    controller::EditorSnapshot,
    jobs::{self, Action, Context, HostHandle},
};
use dioxus::prelude::*;

fn fields(values: &[f64]) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}
fn numbers<const N: usize>(values: &[String]) -> Result<[f64; N], String> {
    if values.len() != N {
        return Err("Incomplete authoring fields".into());
    }
    let mut out = [0.0; N];
    for (i, s) in values.iter().enumerate() {
        out[i] = s
            .parse::<f64>()
            .map_err(|_| "Enter finite numeric authoring values")?;
        if !out[i].is_finite() {
            return Err("Authoring values must be finite".into());
        }
    }
    Ok(out)
}
#[allow(clippy::too_many_arguments)]
fn send(
    action: Action,
    host: HostHandle,
    mut view: Signal<EditorSnapshot>,
    mut draft: Signal<String>,
    mut status: Signal<String>,
    mut image: Signal<Option<String>>,
    mut busy: Signal<bool>,
    mut inspected: Signal<Option<AuthoringView>>,
    buffer: Option<crate::graphical::draft_buffer::DraftBuffer>,
    edit: bool,
) {
    if busy() {
        return;
    }
    if edit && draft() != view().latest_source {
        status.set("Unsubmitted draft preserved. Submit it or explicitly load accepted source before using part controls.".into());
        return;
    }
    let origin = Context::from_snapshot(&view());
    let guard = host.clone();
    busy.set(true);
    status.set("Native authoring command queued against the displayed source.".into());
    spawn(async move {
        match crate::graphical::native_dispatch::run(host, action, origin).await {
            Ok(reply) if jobs::completion_is_current(&guard, &view(), &reply) => {
                if let Some(snapshot) = reply.snapshot {
                    if edit {
                        if let Some(buffer) = buffer {
                            buffer.set(snapshot.latest_source.clone());
                        }
                        draft.set(snapshot.latest_source.clone());
                    }
                    view.set(snapshot);
                }
                if edit {
                    image.set(reply.image_uri);
                } else if reply.image_uri.is_some() {
                    image.set(reply.image_uri);
                }
                if let Some(authoring) = reply.authoring {
                    inspected.set(Some(authoring));
                }
                status.set(reply.message);
            }
            Ok(_) => {
                status.set("Source or contributor changed; late authoring result withheld.".into())
            }
            Err(error) => status.set(error),
        }
        busy.set(false);
    });
}

#[component]
pub(crate) fn AuthoringControls(
    host: HostHandle,
    view: Signal<EditorSnapshot>,
    draft: Signal<String>,
    mut status: Signal<String>,
    image: Signal<Option<String>>,
    busy: Signal<bool>,
    #[props(default)] buffer: Option<crate::graphical::draft_buffer::DraftBuffer>,
) -> Element {
    let inspected = use_signal(|| None::<AuthoringView>);
    let mut selected = use_signal(String::new);
    let mut transform = use_signal(|| fields(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]));
    let mut material = use_signal(|| fields(&[0.5, 0.5, 0.5, 1.0, 0.5, 0.0]));
    let mut material_id = use_signal(|| 0u64);
    let mut parameter = use_signal(String::new);
    let mut parameter_value = use_signal(String::new);
    let mut filename = use_signal(|| "growing-station-derived.hmc".to_string());
    let mut asset_key = use_signal(|| "growing-station-game-v1".to_string());
    let mut expected_digest = use_signal(String::new);
    let current = view();
    let fresh = inspected()
        .as_ref()
        .is_some_and(|v| v.source == current.source)
        && !current.reconciliation_required;
    let inspect_host = host.clone();
    let inspect_buffer = buffer.clone();
    let transform_host = host.clone();
    let transform_buffer = buffer.clone();
    let material_host = host.clone();
    let material_buffer = buffer.clone();
    let param_host = host.clone();
    let param_buffer = buffer.clone();
    let structure_host = host.clone();
    let structure_buffer = buffer.clone();
    let compile_host = host.clone();
    let compile_buffer = buffer.clone();
    let profile_host = host.clone();
    let profile_buffer = buffer.clone();
    let profile_source = current.source.clone();
    let cad_host = host.clone();
    let cad_buffer = buffer.clone();
    let cad_source = current.source.clone();
    let surface_host = host.clone();
    let surface_buffer = buffer.clone();
    let surface_source = current.source.clone();
    let binding_host = host.clone();
    let binding_buffer = buffer.clone();
    let binding_source = current.source.clone();
    let structure_source = current.source.clone();
    let transform_source = current.source.clone();
    let material_source = current.source.clone();
    let param_source = current.source.clone();
    let compile_source = current.source.clone();
    let data = inspected();
    rsx! {
        fieldset{disabled:busy(),style:"margin-top:12px;padding:10px;border:1px solid #405264;min-width:0;",
            legend{"Assembly · parts, profiles and materials"}
            button{disabled:busy()||current.reconciliation_required,onclick:move |_|send(Action::AuthoringInspect(view().source),inspect_host.clone(),view,draft,status,image,busy,inspected,inspect_buffer.clone(),false),"Inspect accepted assembly"}
            p{style:"font-size:12px;","Parent-relative transforms use metres/radians. Edits are checked Vibe source revisions; shared material edits affect every part using that material."}
            super::surface_binding_controls::SurfaceBindingControls{busy,onbind:move|request|send(Action::BindSurfacePackage{request,expected:binding_source.clone()},binding_host.clone(),view,draft,status,image,busy,inspected,binding_buffer.clone(),false)}
            if let Some(data)=data{
                p{"{data.parts.len()} parts · source {data.source.artifact_id}"}
                select{disabled:!fresh||busy(),value:"{selected}",aria_label:"Selected assembly part",onchange:move|event|{
                    let key=event.value();
                    if let Some(part)=inspected().as_ref().and_then(|s|s.parts.iter().find(|p|p.id==key).cloned()){
                        let mut trs=fields(&part.translation);trs.extend(fields(&part.rotation));trs.extend(fields(&part.scale));transform.set(trs);
                        let mut mat=fields(&part.base_color);mat.push(part.roughness.to_string());mat.push(part.metallic.to_string());material.set(mat);material_id.set(part.material_id);
                    }
                    selected.set(key);
                },option{value:"","Choose a part"}for part in data.parts{option{value:"{part.id}","{part.id} · material {part.material_id}"}}}
                div{style:"display:flex;flex-wrap:wrap;gap:6px;margin-top:8px;",
                    for (index,label) in ["Translate X","Translate Y","Translate Z","Rotate X","Rotate Y","Rotate Z","Scale X","Scale Y","Scale Z"].iter().enumerate(){
                        label{style:"width:100px;font-size:12px;","{label}" input{disabled:!fresh||selected().is_empty(),value:"{transform()[index]}",aria_label:"{label}",style:"width:94px;",oninput:move|e|transform.write()[index]=e.value()}}
                    }
                }
                button{disabled:!fresh||selected().is_empty(),onclick:move |_|{
                    match numbers::<9>(&transform()){
                        Ok(n)=>send(Action::AuthoringEdit{edit:AuthoringEdit::Transform{id:selected(),translation:[n[0],n[1],n[2]],rotation:[n[3],n[4],n[5]],scale:[n[6],n[7],n[8]]},expected:transform_source.clone()},transform_host.clone(),view,draft,status,image,busy,inspected,transform_buffer.clone(),true),
                        Err(e)=>status.set(e),
                    }
                },"Apply part transform"}
                super::profile_controls::ProfileControls{selected,inspected,fresh,onedit:move|edit|send(Action::AuthoringEdit{edit,expected:profile_source.clone()},profile_host.clone(),view,draft,status,image,busy,inspected,profile_buffer.clone(),true)}
                super::cad_controls::CadControls{selected,inspected,fresh,onedit:move|edit|send(Action::AuthoringEdit{edit,expected:cad_source.clone()},cad_host.clone(),view,draft,status,image,busy,inspected,cad_buffer.clone(),true)}
                super::surface_controls::SurfaceControls{selected,inspected,fresh,onedit:move|edit|send(Action::AuthoringEdit{edit,expected:surface_source.clone()},surface_host.clone(),view,draft,status,image,busy,inspected,surface_buffer.clone(),true)}
                super::structure_controls::StructureControls {
                    selected, inspected, fresh,
                    onedit: move |edit| send(Action::AuthoringEdit { edit, expected: structure_source.clone() }, structure_host.clone(), view, draft, status, image, busy, inspected, structure_buffer.clone(), true),
                }
                div{style:"display:flex;flex-wrap:wrap;gap:6px;margin-top:8px;",
                    for (index,label) in ["Linear red","Linear green","Linear blue","Alpha","Roughness","Metallic"].iter().enumerate(){
                        label{style:"width:100px;font-size:12px;","{label}" input{disabled:!fresh||selected().is_empty()||material_id()==0,value:"{material()[index]}",aria_label:"Assembly material {label}",style:"width:94px;",oninput:move|e|material.write()[index]=e.value()}}
                    }
                }
                button{disabled:!fresh||selected().is_empty()||material_id()==0,onclick:move |_|{
                    match numbers::<6>(&material()){
                        Ok(n)=>send(Action::AuthoringEdit{edit:AuthoringEdit::Material{material_id:material_id(),base_color:[n[0],n[1],n[2],n[3]],roughness:n[4],metallic:n[5]},expected:material_source.clone()},material_host.clone(),view,draft,status,image,busy,inspected,material_buffer.clone(),true),
                        Err(e)=>status.set(e),
                    }
                },"Apply shared material"}
                if !data.parameters.is_empty(){
                    h4{"Profile and geometry parameters"}
                    select{disabled:!fresh,value:"{parameter}",aria_label:"Authored geometry parameter",onchange:move|event|{
                        let key=event.value();if let Some((_,value))=inspected().as_ref().and_then(|v|v.parameters.iter().find(|(k,_)|k==&key).cloned()){parameter_value.set(value.to_string());}parameter.set(key);
                    },option{value:"","Choose a parameter"}for (key,value) in data.parameters{option{value:"{key}","{key} = {value}"}}}
                    input{disabled:!fresh,value:"{parameter_value}",aria_label:"Geometry parameter value",oninput:move|event|parameter_value.set(event.value())}
                    button{disabled:!fresh||parameter().is_empty(),onclick:move |_|match parameter_value().parse::<f64>(){
                        Ok(value)=>send(Action::AuthoringEdit{edit:AuthoringEdit::Parameter{key:parameter(),value},expected:param_source.clone()},param_host.clone(),view,draft,status,image,busy,inspected,param_buffer.clone(),true),
                        Err(_)=>status.set("Enter a finite geometry parameter value.".into()),
                    },"Rebuild parts from parameter"}
                    p{style:"font-size:12px;","Parameter rebuild regenerates parts from the retained recipe; explicit part overrides are replaced. Undo preserves the earlier source."}
                }
            }
            details{summary{"Compile accepted assembly into HMC"}
                label{"Package filename" input{value:"{filename}",aria_label:"Compiled HMC filename",oninput:move|e|filename.set(e.value())}}
                label{"Asset key" input{value:"{asset_key}",aria_label:"Compiled assembly asset key",oninput:move|e|asset_key.set(e.value())}}
                label{"Inspected package SHA256 (empty creates a new file)" input{value:"{expected_digest}",aria_label:"Compile expected package digest",oninput:move|e|expected_digest.set(e.value())}}
                button{disabled:current.reconciliation_required,onclick:move |_|{
                    let text=expected_digest();let file=if text.trim().is_empty(){Ok(qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Absent)}else{parse_digest(text.trim()).map(qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Sha256)};
                    match file{Ok(file)=>send(Action::CompileAssembly{name:filename(),key:asset_key(),file,expected:compile_source.clone()},compile_host.clone(),view,draft,status,image,busy,inspected,compile_buffer.clone(),false),Err(e)=>status.set(e)}
                },"Compile source, materials and parts into HMC"}
                p{style:"font-size:12px;","Compiles the accepted assembly without evaluating source again. Retained resources survive; existing compiled entries require the exact inspected package digest. Source review does not approve the complete package. Changed compiled entries require a new asset key, retaining all prior sections and graph facts."}
            }
        }
    }
}
fn parse_digest(text: &str) -> Result<[u8; 32], String> {
    if text.len() != 64 || !text.is_ascii() {
        return Err("Expected a 64-character hexadecimal SHA256".into());
    }
    let mut result = [0; 32];
    for (i, slot) in result.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16)
            .map_err(|_| "Invalid package SHA256")?;
    }
    Ok(result)
}
