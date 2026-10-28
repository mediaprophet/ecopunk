//! Restricted native workbook controls over the existing authorized package owner.
use crate::graphical::{
    checkpoints::{PackageInventory, ResourceInspectionRequest, WorkbookCreateRequest, WorkbookEditRequest, WorkbookInspection},
    controller::EditorSnapshot,
    jobs::{self, Action, Context, HostHandle},
};
use dioxus::prelude::*;

#[derive(Clone)]
struct InspectedWorkbook { context: Context, result: WorkbookInspection }

fn dispatch(
    action: Action,
    expected_read: Option<ResourceInspectionRequest>,
    host: HostHandle,
    current: EditorSnapshot,
    mut inspected: Signal<Option<InspectedWorkbook>>,
    mut status: Signal<String>,
    mut busy: Signal<bool>,
    on_saved: EventHandler<()>,
) {
    if busy() || !current.publication_is_current() { return; }
    busy.set(true);
    inspected.set(None);
    let origin = Context::from_snapshot(&current);
    let guard = host.clone();
    spawn(async move {
        match crate::graphical::native_dispatch::run(host, action, origin.clone()).await {
            Ok(reply) if jobs::completion_is_current(&guard, &current, &reply) => {
                if let Some(expected) = expected_read {
                    match reply.workbook {
                        Some(result) if result.request == expected => {
                            inspected.set(Some(InspectedWorkbook { context: origin, result }));
                            status.set(reply.message);
                        }
                        _ => status.set(if reply.operation_succeeded { "Workbook result did not match the selected resource; inspect again.".into() } else { reply.message }),
                    }
                } else {
                    let success = reply.operation_succeeded;
                    status.set(reply.message);
                    if success { on_saved.call(()); }
                }
            }
            Ok(_) => status.set("Source, session or authority changed; workbook result withheld. Refresh the package before retrying.".into()),
            Err(error) => status.set(error),
        }
        busy.set(false);
    });
}

#[component]
pub(super) fn WorkbookControls(
    host: HostHandle,
    current: EditorSnapshot,
    filename: String,
    package: PackageInventory,
    mut busy: Signal<bool>,
    mut status: Signal<String>,
    on_saved: EventHandler<()>,
) -> Element {
    let mut inspected = use_signal(|| None::<InspectedWorkbook>);
    let mut key = use_signal(|| "workbook/native-v1".to_string());
    let mut cell = use_signal(|| "A1".to_string());
    let mut input = use_signal(String::new);
    let context = Context::from_snapshot(&current);
    let selected = inspected().filter(|entry| entry.context == context
        && current.publication_is_current()
        && entry.result.request.filename == filename
        && entry.result.request.package_digest == package.digest
        && package.resources.iter().any(|r| r.key == entry.result.request.key
            && r.kind == crate::graphical::workbook_p0::RESOURCE_KIND
            && r.digest == entry.result.request.resource_digest));
    rsx! {
        section { style:"padding:10px;border:1px solid #465463;display:flex;flex-direction:column;gap:6px;",
            h3 { "Native workbook (restricted)" }
            p { "Create or inspect a new native workbook in this working package. One sheet and literal cell inputs only; formulas, spreadsheet imports, formatting and full workbook editing are not yet supported. Empty input clears the selected cell." }
            label { "New workbook resource key"
                input { value:"{key}", disabled:busy(), oninput:move |event| key.set(event.value()) }
            }
            {let host=host.clone(); let current=current.clone(); let filename=filename.clone(); let package_digest=package.digest;
                rsx! { button { disabled:busy(), onclick:move |_| {
                    let request = WorkbookCreateRequest { filename:filename.clone(), package_digest, key:key() };
                    if let Err(error) = request.validate_bounds() { status.set(error); return; }
                    dispatch(Action::CreateWorkbook { request, expected:current.source.clone() }, None, host.clone(), current.clone(), inspected, status, busy, on_saved);
                }, "Create blank native workbook" } }
            }
            for resource in package.resources.iter().filter(|resource| resource.kind == crate::graphical::workbook_p0::RESOURCE_KIND) {
                { let request = ResourceInspectionRequest { filename:filename.clone(), package_digest:package.digest, key:resource.key.clone(), resource_digest:resource.digest };
                  let host=host.clone(); let current=current.clone(); let label=resource.key.clone();
                  rsx! { div { style:"display:flex;gap:8px;flex-wrap:wrap;",
                    span { style:"overflow-wrap:anywhere;", "{label}" }
                    button { disabled:busy(), onclick:move |_| {
                        dispatch(Action::InspectWorkbook { request:request.clone(), expected:current.source.clone() }, Some(request.clone()), host.clone(), current.clone(), inspected, status, busy, on_saved);
                    }, "Inspect workbook" }
                  } }
                }
            }
            if let Some(entry) = selected {
                p { "Workbook resource {entry.result.request.key} · {entry.result.view.sheet} · saved source revision {entry.result.saved_source_revision}" }
                if entry.result.view.cells.is_empty() { p { "No stored cells yet." } }
                table { aria_label:"Stored workbook cells", thead { tr { th { "Cell" } th { "Input" } th { "Displayed value" } } }
                    tbody { for item in &entry.result.view.cells { tr { td { "{item.cell}" } td { "{item.input}" } td { "{item.display}" } } } }
                }
                label { "Cell (A1 notation)" input { value:"{cell}", disabled:busy(), oninput:move |event| cell.set(event.value()) } }
                label { "Cell input; empty clears" input { value:"{input}", disabled:busy(), oninput:move |event| input.set(event.value()) } }
                { let host=host.clone(); let current=current.clone(); let resource=entry.result.request.clone(); let sheet=entry.result.view.sheet.clone();
                  rsx! { button { disabled:busy(), onclick:move |_| {
                    let request=WorkbookEditRequest { resource:resource.clone(), sheet:sheet.clone(), cell:cell(), input:input() };
                    if let Err(error)=request.validate_bounds() { status.set(error); return; }
                    dispatch(Action::EditWorkbook { request, expected:current.source.clone() }, None, host.clone(), current.clone(), inspected, status, busy, on_saved);
                  }, "Save cell" } }
                }
            }
        }
    }
}
