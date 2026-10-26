//! Native controls over the same current, authorised editor worker.
use super::{
    checkpoints::PackageInventory,
    controller::EditorSnapshot,
    jobs::{self, Action, Context, HostHandle},
    package::{hex_digest, ImportRequest},
};
use dioxus::prelude::*;
#[derive(Clone)]
struct InspectedPackage {
    context: Context,
    package: PackageInventory,
}

fn command(
    action: Action,
    host: HostHandle,
    current: EditorSnapshot,
    mut inventory: Signal<Option<InspectedPackage>>,
    mut status: Signal<String>,
    mut busy: Signal<bool>,
) {
    if busy() {
        return;
    }
    busy.set(true);
    inventory.set(None);
    let origin = Context::from_snapshot(&current);
    let guard = host.clone();
    spawn(async move {
        match jobs::run(host,action,origin).await {
            Ok(reply) if jobs::completion_is_current(&guard,&current,&reply)=>{
                inventory.set(reply.inventory.map(|package|InspectedPackage{context:Context::from_snapshot(&current),package}));
                status.set(reply.message);
            }
            Ok(_)=>status.set("Source/session/authority changed; resource result withheld. Refresh source and inventory before another edit.".into()),
            Err(error)=>status.set(error),
        }
        busy.set(false);
    });
}

#[component]
pub(super) fn PackageControls(
    host: HostHandle,
    current: EditorSnapshot,
    busy: Signal<bool>,
    status: Signal<String>,
) -> Element {
    let mut name = use_signal(|| "poet-checkpoint.hmc".to_string());
    let mut filename = use_signal(String::new);
    let mut key = use_signal(String::new);
    let mut kind = use_signal(|| "asset".to_string());
    let mut inventory = use_signal(|| None::<InspectedPackage>);
    let entries = inventory()
        .filter(|view| {
            view.context == Context::from_snapshot(&current) && current.publication_is_current()
        })
        .map(|view| view.package);
    let inventory_available = entries.is_some();
    let scan = {
        let host = host.clone();
        let current = current.clone();
        move |_| {
            command(
                Action::Inventory {
                    name: name(),
                    expected: current.source.clone(),
                },
                host.clone(),
                current.clone(),
                inventory,
                status,
                busy,
            );
        }
    };
    let import = {
        let host = host.clone();
        let current = current.clone();
        move |_| {
            if let Some(package) = inventory()
                .filter(|view| {
                    view.context == Context::from_snapshot(&current)
                        && current.publication_is_current()
                })
                .map(|view| view.package)
            {
                let expected_resource = package
                    .resources
                    .iter()
                    .find(|r| r.key == key())
                    .map(|r| r.digest);
                command(
                    Action::ImportResource {
                        name: name(),
                        file_digest: package.digest,
                        request: ImportRequest {
                            filename: filename(),
                            key: key(),
                            kind: kind(),
                            expected_resource,
                        },
                        expected: current.source.clone(),
                    },
                    host.clone(),
                    current.clone(),
                    inventory,
                    status,
                    busy,
                );
            }
        }
    };
    rsx! {
        section{style:"padding:12px;border:1px solid #465463;display:flex;flex-direction:column;gap:8px;",
            h2{style:"margin:0;font-size:19px;","HMC resources"}
            p{"Inspect a saved package before editing. Unedited resources and metadata are retained. Published releases require a new working copy."}
            div{style:"display:flex;gap:8px;flex-wrap:wrap;",
                input{value:"{name}",aria_label:"HMC resource package filename",disabled:busy(),
                    oninput:move|e|{name.set(e.value());inventory.set(None);}}
                button{disabled:busy(),onclick:scan,"Inspect resources"}
            }
            if let Some(package)=entries {
                p{style:"overflow-wrap:anywhere;","Inspected package snapshot SHA256 {hex_digest(&package.digest)}"}
                {let host=host.clone();let current=current.clone();let digest=package.digest;
                    rsx!{button{disabled:busy(),onclick:move |_|{
                        command(Action::SaveInspected{name:name(),file_digest:digest,expected:current.source.clone()},host.clone(),current.clone(),inventory,status,busy);
                    },"Save accepted source into this HMC"}}
                }
                for resource in package.resources {
                    {let host=host.clone();let current=current.clone();let remove_key=resource.key.clone();let digest=resource.digest;let file_digest=package.digest;
                    rsx!{div{style:"padding:8px;background:#0c141c;overflow-wrap:anywhere;",
                        strong{"{resource.key}"}
                        p{"{resource.kind} · {resource.bytes} bytes · {resource.metadata_bytes} metadata bytes"}
                        p{"SHA256 {hex_digest(&digest)}"}
                        button{disabled:busy(),onclick:move |_|{
                            command(Action::RemoveResource{name:name(),file_digest,key:remove_key.clone(),resource_digest:digest,expected:current.source.clone()},
                                host.clone(),current.clone(),inventory,status,busy);
                        },"Remove this resource"}
                    }}}
                }
            }
            div{style:"display:flex;gap:8px;flex-wrap:wrap;",
                input{value:"{filename}",placeholder:"Filename in native import library",aria_label:"Native resource library filename",disabled:busy(),oninput:move|e|filename.set(e.value())}
                input{value:"{key}",placeholder:"Resource key",aria_label:"Resource key",disabled:busy(),oninput:move|e|key.set(e.value())}
                input{value:"{kind}",placeholder:"Kind for new resource",aria_label:"Resource kind for additions",disabled:busy(),oninput:move|e|kind.set(e.value())}
                button{disabled:busy() || !inventory_available,onclick:import,"Import or replace contents"}
            }
            p{"Imports use the native host’s selected library. Replacement keeps the existing resource kind and metadata. Rendering and model acceptance require separate checks."}
        }
    }
}
