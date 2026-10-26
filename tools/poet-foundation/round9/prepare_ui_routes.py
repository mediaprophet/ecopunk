from staging import *
P='native/src/graphical/'
s=read(P+'ui.rs')
s=change(s,'    let image = use_signal(|| None::<String>);','    let image = use_signal(|| None::<String>);\n    let selected = use_signal(String::new);')
start=s.index('                        if let Some(uri) = image() {')
end=s.index('                        p { "Accepted source revision',start)
s=s[:start]+'''                        super::viewport_controls::ViewportControls{host:host.clone(),view,image,busy,status,selected}
'''+s[end:]
s=change(s,'AuthoringControls {host:host.clone(),view,draft,status,image,busy,buffer:','AuthoringControls {host:host.clone(),view,draft,status,image,busy,selected,buffer:')
stage(P+'ui.rs',s)
s=read(P+'authoring/controls.rs')
s=change(s,'    busy: Signal<bool>,\n    #[props(default)]','    busy: Signal<bool>,\n    mut selected: Signal<String>,\n    #[props(default)]')
s=change(s,'    let mut selected = use_signal(String::new);','    let mut loaded_part = use_signal(String::new);')
s=change(s,'    let current = view();','''    use_effect(move || {
        let key = selected();
        if key != loaded_part() {
            if let Some(part) = inspected().as_ref().and_then(|v|v.parts.iter().find(|p|p.id==key).cloned()) {
                let mut trs=fields(&part.translation);trs.extend(fields(&part.rotation));trs.extend(fields(&part.scale));transform.set(trs);
                let mut mat=fields(&part.base_color);mat.push(part.roughness.to_string());mat.push(part.metallic.to_string());material.set(mat);material_id.set(part.material_id);
                loaded_part.set(key);
            }
        }
    });
    let current = view();''')
s=change(s,'    let param_host = host.clone();','''    let layers_host = host.clone();
    let layers_buffer = buffer.clone();
    let layers_source = current.source.clone();
    let param_host = host.clone();''')
s=change(s,'                    h4{"Profile and geometry parameters"}','''                    h4{"Profile and geometry parameters"}
                    button{disabled:!fresh,onclick:move |_|send(Action::AuthoringEdit{edit:AuthoringEdit::EnableSourceLayers,expected:layers_source.clone()},layers_host.clone(),view,draft,status,image,busy,inspected,layers_buffer.clone(),true),"Enable non-destructive source layers"}''')
s=s.replace('Parameters regenerate the recorded procedural base. Set up poet_parameter_parts in the recipe before part/surface edits. Later edits are preserved by refusing regeneration; merge them explicitly into source or create a separate derivation. UV/detail remapping is not yet qualified.','Enable source layers for an existing procedural recipe, then change dimensions while retaining material and transform overrides. Changes without qualified geometry correspondence preserve the current source by refusing regeneration; UV/detail topology remapping remains unqualified.')
stage(P+'authoring/controls.rs',s)
s=read(P+'jobs_tests.rs');s=change(s,'    let reply = Reply {','    let reply = Reply {\n        viewport: None,');stage(P+'jobs_tests.rs',s)
s=read(P+'mod.rs');s=change(s,'mod viewport_controls;','mod viewport_controls;\nmod viewport_input;');stage(P+'mod.rs',s)
manifest()
print('Native viewport/part selection/source-layer controls staged')
