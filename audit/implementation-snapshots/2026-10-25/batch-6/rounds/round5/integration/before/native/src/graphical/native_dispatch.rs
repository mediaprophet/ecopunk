//! Adapt native controls to Poet's existing payload contract and the same worker.
use super::{
    authoring::AuthoringEdit,
    jobs::{self, Action, Context, HostHandle, Reply},
    poet_workspace::{
        core::{ActionType, TargetIdentifier, VibeScriptPayload},
        NativeIntentBus,
    },
};
use qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile;
use serde_json::json;
pub(crate) async fn run(
    host: HostHandle,
    action: Action,
    origin: Context,
) -> Result<Reply, String> {
    // An explicitly labelled local diagnostic has no cooperative actor to stamp.
    // Authenticated native projects always use the original Poet intent contract.
    if origin.authorisation_stamp.is_none() {
        return jobs::run(host, action, origin).await;
    }
    let (kind, parameters) = match &action {
        Action::AuthoringInspect(_) => (ActionType::Query, json!({"command":"authoring.inspect"})),
        Action::AuthoringEdit { edit, .. } => {
            let p = match edit {
                AuthoringEdit::Transform {
                    id,
                    translation,
                    rotation,
                    scale,
                } => {
                    json!({"command":"authoring.transform","id":id,"translation":translation,"rotation":rotation,"scale":scale})
                }
                AuthoringEdit::Material {
                    material_id,
                    base_color,
                    roughness,
                    metallic,
                } => {
                    json!({"command":"authoring.material","material_id":material_id,"base_color":base_color,"roughness":roughness,"metallic":metallic})
                }
                AuthoringEdit::Parameter { key, value } => {
                    json!({"command":"authoring.parameter","key":key,"value":value})
                }
            };
            (ActionType::Invoke, p)
        }
        Action::CompileAssembly {
            name, key, file, ..
        } => (
            ActionType::Invoke,
            json!({"command":"authoring.compile","name":name,"key":key,"expected_sha256":match file{ExpectedFile::Absent=>None,ExpectedFile::Sha256(d)=>Some(*d)}}),
        ),
        Action::Check(source) => (
            ActionType::Query,
            json!({"command":"source.check","source":source}),
        ),
        Action::Submit { source, .. } => (
            ActionType::Invoke,
            json!({"command":"source.submit","source":source}),
        ),
        Action::Undo(_) => (ActionType::Invoke, json!({"command":"source.undo"})),
        Action::Redo(_) => (ActionType::Invoke, json!({"command":"source.redo"})),
        Action::Rebuild(_) => (ActionType::Invoke, json!({"command":"source.rebuild"})),
        Action::Refresh => (ActionType::Query, json!({"command":"source.refresh"})),
        Action::Save { name, .. } => (
            ActionType::Invoke,
            json!({"command":"package.save","name":name}),
        ),
        // Existing package/cooperation controls keep their qualified same-owner
        // adapters until their original Poet tool payloads are ported explicitly.
        _ => return jobs::run(host, action, origin).await,
    };
    let payload = VibeScriptPayload::new(
        kind,
        TargetIdentifier::component_id(&origin.source.artifact_id),
        parameters,
    );
    let result = NativeIntentBus::new(host, origin)?
        .dispatch_with_reply(payload)
        .await?;
    result
        .reply
        .ok_or_else(|| format!("Poet intent rejected: {:?}", result.receipt.status))
}
