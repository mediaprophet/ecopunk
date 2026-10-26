//! Native Aether projection of actual Poet source/container commands.
use super::controller::EditorSnapshot;
use super::jobs::{self, Action, Context, HostHandle};
use dioxus::prelude::*;
fn dispatch(
    action: Action,
    host: HostHandle,
    mut view: Signal<EditorSnapshot>,
    mut draft: Signal<String>,
    mut status: Signal<String>,
    mut busy: Signal<bool>,
    mut image: Signal<Option<String>>,
    replace_draft: bool,
) {
    if busy() {
        return;
    }
    busy.set(true);
    image.set(None);
    status.set("Native command queued; source and session are checked before execution.".into());
    let origin = Context::from_snapshot(&view());
    let preserve_draft = replace_draft && draft() != view().latest_source;
    let guard = host.clone();
    spawn(async move {
        match jobs::run(host, action, origin).await {
            Ok(reply) if jobs::completion_is_current(&guard, &view(), &reply) => {
                if let Some(snapshot) = reply.snapshot {
                    if replace_draft && !preserve_draft {
                        draft.set(snapshot.latest_source.clone());
                    }
                    view.set(snapshot);
                }
                if reply.image_uri.is_some() {
                    image.set(reply.image_uri);
                }
                status.set(if preserve_draft {
                    format!("{}; unsubmitted draft preserved", reply.message)
                } else {
                    reply.message
                });
            }
            Ok(_) => {
                image.set(None);
                status.set("Source/session could not be revalidated. Refresh source; no late frame installed.".into());
            }
            Err(error) => {
                image.set(None);
                status.set(error);
            }
        }
        busy.set(false);
    });
}

impl PartialEq for EditorSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.session_revision == other.session_revision
            && self.execution_epoch == other.execution_epoch
            && self.rebuild_supported == other.rebuild_supported
            && self.reconciliation_required == other.reconciliation_required
            && self.title == other.title
            && self.latest_source == other.latest_source
            && self.material_edit_id == other.material_edit_id
            && self.checkpoint_supported == other.checkpoint_supported
            && self.authorisation_stamp == other.authorisation_stamp
    }
}

#[component]
pub(super) fn NativeStudio(
    host: HostHandle,
    initial: EditorSnapshot,
    initial_draft: String,
) -> Element {
    let view = use_signal(|| initial);
    let mut draft = use_signal(|| initial_draft);
    let status =
        use_signal(|| "Source ready. Check or submit an explicit Vibe command.".to_string());
    let busy = use_signal(|| false);
    let image = use_signal(|| None::<String>);
    let current = view();
    let mut checkpoint_name=use_signal(|| "poet-checkpoint.hmc".to_string());
    let mut save={
        let host=host.clone();
        move |_| dispatch(Action::Save { name:checkpoint_name(),expected:view().source },
            host.clone(),view,draft,status,busy,image,false)
    };
    let mut check = {
        let host = host.clone();
        move |_| {
            dispatch(
                Action::Check(draft()),
                host.clone(),
                view,
                draft,
                status,
                busy,
                image,
                false,
            )
        }
    };
    let mut submit = {
        let host = host.clone();
        move |_| {
            dispatch(
                Action::Submit {
                    source: draft(),
                    expected: view().source,
                },
                host.clone(),
                view,
                draft,
                status,
                busy,
                image,
                false,
            )
        }
    };
    let mut undo = {
        let host = host.clone();
        move |_| {
            dispatch(
                Action::Undo(view().source),
                host.clone(),
                view,
                draft,
                status,
                busy,
                image,
                true,
            )
        }
    };
    let mut redo = {
        let host = host.clone();
        move |_| {
            dispatch(
                Action::Redo(view().source),
                host.clone(),
                view,
                draft,
                status,
                busy,
                image,
                true,
            )
        }
    };
    let mut refresh = {
        let host = host.clone();
        move |_| {
            dispatch(
                Action::Refresh,
                host.clone(),
                view,
                draft,
                status,
                busy,
                image,
                true,
            )
        }
    };
    let mut rebuild = {
        let host = host.clone();
        move |_| {
            dispatch(
                Action::Rebuild(view().source),
                host.clone(),
                view,
                draft,
                status,
                busy,
                image,
                false,
            )
        }
    };
    rsx! {
        div { style: "height:100vh;box-sizing:border-box;overflow:auto;background:#111820;color:#e8eef4;display:flex;font-family:sans-serif;",
            aside { style: "width:160px;flex-shrink:0;box-sizing:border-box;padding:18px;background:#18232e;",
                h2 { "Poet" }
                p { "Tool-Chest" }
                strong { "Studio" }
                p { "Manifold · source and scene" }
            }
            main { style: "flex:1;min-width:0;min-height:0;box-sizing:border-box;padding:16px;display:flex;flex-direction:column;gap:10px;",
                header {
                    h1 { style: "margin:0 0 8px;font-size:24px;", "{current.title}" }
                    p { style: "margin:0;", "{current.artifact_id} · source revision {current.source_revision} · Vibe session {current.session_revision}" }
                }
                div { style: "display:flex;gap:12px;flex:1;min-width:0;min-height:0;align-items:flex-start;",
                    section { style: "flex:1 1 0;min-width:0;display:flex;flex-direction:column;gap:8px;",
                        h2 { style: "margin:0;font-size:20px;", "Vibe source draft" }
                        textarea { value: "{draft}", disabled: busy(), aria_label: "Vibe source draft",
                            style: "height:calc(100vh - 360px);min-height:220px;max-height:560px;width:100%;min-width:0;box-sizing:border-box;background:#081018;color:#e8eef4;padding:12px;font-family:monospace;font-size:15px;",
                            oninput: move |event| draft.set(event.value()),
                        }
                        div { style: "display:flex;gap:10px;flex-wrap:wrap;",
                            button { disabled: busy() || current.reconciliation_required, onclick: move |event| check(event), "Check" }
                            button { disabled: busy() || current.reconciliation_required, onclick: move |event| submit(event), "Submit source" }
                            button { disabled: busy(), onclick: move |event| undo(event), "Undo source" }
                            button { disabled: busy(), onclick: move |event| redo(event), "Redo source" }
                            button { disabled: busy(), onclick: move |event| refresh(event), "Refresh source" }
                            if current.rebuild_supported {
                                button { disabled: busy() || !current.reconciliation_required,
                                    onclick: move |event| rebuild(event), "Rebuild execution session" }
                            }
                        }
                        if current.reconciliation_required {
                            if current.rebuild_supported {
                                p { "Source is preserved. Rebuild the admitted geometry execution session before another submission. No source is resubmitted automatically." }
                            } else {
                                p { "Source is preserved. The native host must reconcile the execution session before another submission." }
                            }
                        }
                        if current.checkpoint_supported {
                            div { style: "display:flex;gap:8px;flex-wrap:wrap;",
                                input { value: "{checkpoint_name}", aria_label: "HMC checkpoint filename",
                                    disabled: busy(), oninput: move |event| checkpoint_name.set(event.value()) }
                                button { disabled: busy(), onclick: move |event| save(event), "Save accepted source as new HMC" }
                            }
                        }
                        p { "{status}" }
                        super::material_controls::MaterialControls { current: current.clone(), busy: busy(), draft, status }
                    }
                    section { style: "width:512px;max-width:45%;min-width:0;flex-shrink:0;display:flex;flex-direction:column;gap:8px;",
                        h2 { style: "margin:0;font-size:20px;", "Manifold preview" }
                        if let Some(uri) = image() {
                            img { src: "{uri}", width: "512", height: "512", style: "display:block;width:100%;max-width:512px;height:auto;", alt: "Native engine preview bound to the displayed source revision" }
                        } else {
                            div { style: "width:100%;max-width:512px;aspect-ratio:1;box-sizing:border-box;background:#060c12;padding:18px;",
                                "No current rendered frame. Submit an admitted geometry source command."
                            }
                        }
                        p { "Accepted source revision {current.source_revision} · Vibe session {current.session_revision}" }
                        p { "Factor material · SDR working preview. Master quality, source rights and human review remain separate." }
                    }
                }
            }
        }
    }
}
