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
    spawn(async move {
        match jobs::run(host, action, origin).await {
            Ok(reply) if jobs::completion_is_current(&view(), &reply) => {
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
            && self.reconciliation_required == other.reconciliation_required
            && self.title == other.title
            && self.latest_source == other.latest_source
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
    rsx! {
        div { style: "height:100vh;background:#111820;color:#e8eef4;display:flex;font-family:sans-serif;",
            aside { style: "width:160px;padding:18px;background:#18232e;",
                h2 { "Poet" }
                p { "Tool-Chest" }
                strong { "Studio" }
                p { "Manifold · source and scene" }
            }
            main { style: "flex:1;padding:20px;display:flex;flex-direction:column;gap:14px;",
                header {
                    h1 { "{current.title}" }
                    p { "{current.artifact_id} · source revision {current.source_revision} · Vibe session {current.session_revision}" }
                }
                div { style: "display:flex;gap:12px;flex:1;",
                    section { style: "flex:1;display:flex;flex-direction:column;gap:10px;",
                        h2 { "Vibe source draft" }
                        textarea { value: "{draft}", disabled: busy(),
                            style: "height:490px;width:100%;background:#081018;color:#e8eef4;padding:12px;font-family:monospace;font-size:15px;",
                            oninput: move |event| draft.set(event.value()),
                        }
                        div { style: "display:flex;gap:10px;",
                            button { disabled: busy() || current.reconciliation_required, onclick: move |event| check(event), "Check" }
                            button { disabled: busy() || current.reconciliation_required, onclick: move |event| submit(event), "Submit source" }
                            button { disabled: busy(), onclick: move |event| undo(event), "Undo source" }
                            button { disabled: busy(), onclick: move |event| redo(event), "Redo source" }
                            button { disabled: busy(), onclick: move |event| refresh(event), "Refresh source" }
                        }
                        if current.reconciliation_required {
                            p { "Source is preserved. The native host must reconcile/reopen the execution session before another submission." }
                        }
                        p { "{status}" }
                    }
                    section { style: "width:512px;display:flex;flex-direction:column;gap:10px;",
                        h2 { "Manifold preview" }
                        if let Some(uri) = image() {
                            img { src: "{uri}", width: "512", height: "512", alt: "Native engine preview bound to the displayed source revision" }
                        } else {
                            div { style: "width:512px;height:512px;background:#060c12;padding:18px;",
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
