//! Ordered texture effects use the existing authorised detail/source operation.
use super::*;
use crate::graphical::{
    authoring::AuthoringEdit,
    surfaces::{binding::digest_hex, detail::{validate_effects, DetailLimits, DetailRecipe, TextureEffectStep}},
};

pub(super) fn tools() -> Vec<AuthoringTool> {
    let effect = json!({"oneOf":[
        {"type":"object","properties":{"kind":{"const":"grayscale"}},"required":["kind"],"additionalProperties":false},
        {"type":"object","properties":{"kind":{"const":"invert"}},"required":["kind"],"additionalProperties":false},
        {"type":"object","properties":{"kind":{"const":"sepia"},"intensity":{"type":"number","minimum":0,"maximum":1}},"required":["kind","intensity"],"additionalProperties":false}
    ]});
    let step = json!({"type":"object","properties":{"enabled":{"type":"boolean"},"effect":effect},"required":["enabled","effect"],"additionalProperties":false});
    vec![
        tool("poet_texture_effects_inspect", "Inspect the selected accepted detail recipe and actual resolved base-colour resource digest; no human or scientific acceptance inferred",
            json!({"type":"object","properties":{"context":{"type":"string","maxLength":128},"part_id":{"type":"string","minLength":1,"maxLength":128}},"required":["context","part_id"],"additionalProperties":false})),
        tool("poet_texture_effects_edit", "Replace only the ordered post-composite effects of an existing imported Paint recipe; preserve source layers and immutable base through the native authority/history operation",
            json!({"type":"object","properties":{"context":{"type":"string","maxLength":128},"material_id":{"type":"integer","minimum":1},"effects":{"type":"array","items":step,"maxItems":8}},"required":["context","material_id","effects"],"additionalProperties":false})),
    ]
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Inspect {
    context: String,
    part_id: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EditEffects {
    context: String,
    material_id: u64,
    effects: Vec<TextureEffectStep>,
}

pub(super) fn call(backend: &mut NativeAuthoringBackend, name: &str, arguments: Value) -> Result<Value, ToolError> {
    if name == "poet_texture_effects_inspect" {
        let request: Inspect = args(arguments)?;
        if request.context.len() > 128 || request.part_id.is_empty() || request.part_id.len() > 128 {
            return Err(failure("invalid_arguments", "Texture inspection arguments exceed native bounds"));
        }
        let expected = backend.expected(&request.context)?;
        let view = backend.host()?.authoring_view(expected.source.clone()).map_err(editor_error)?;
        let part = view.parts.into_iter().find(|p| p.id == request.part_id)
            .ok_or_else(|| failure("native_operation_failed", "Selected part is unavailable"))?;
        let inspection = backend.host()?.inspect_surface(&request.part_id, expected.source.clone()).map_err(editor_error)?;
        receipt_authority(&expected.authorisation_stamp)?;
        expected.source.check_current().map_err(editor_error)?;
        let base = match &part.detail {
            Some(DetailRecipe::Paint(p)) => Some(p.base_sha256.clone()),
            _ => None,
        };
        return Ok(json!({
            "artifact_id":expected.artifact_id,"source_revision":expected.source_revision,
            "session_revision":expected.session_revision,"part_id":part.id,"material_id":part.material_id,
            "detail":part.detail,"base_sha256":base,
            "output_sha256":inspection.texture_sha256.map(|d|digest_hex(&d)),
            "working_space":"encoded sRGB RGBA8; opaque imported square base",
            "human_review":"unassessed"
        }));
    }
    let request: EditEffects = args(arguments)?;
    if request.context.len() > 128 || request.material_id == 0 {
        return Err(failure("invalid_arguments", "Texture effect arguments exceed native bounds"));
    }
    validate_effects(&request.effects)
        .map_err(|_|failure("invalid_arguments", "Texture effects violate the supported native profile"))?;
    // A valid-shaped mutation attempt consumes its connection-local context even
    // if a later target, resource, authority or source admission refuses it.
    let expected_result = backend.expected(&request.context);
    backend.context = None;
    let expected = expected_result?;
    let authority = expected.authorisation_stamp.clone();
    let view = backend.host()?.authoring_view(expected.source.clone()).map_err(editor_error)?;
    let mut detail = None;
    for part in view.parts.into_iter().filter(|p| p.material_id == request.material_id) {
        let Some(DetailRecipe::Paint(paint)) = part.detail else {
            return Err(failure("native_operation_failed", "All shared target parts require the same existing imported Paint recipe"));
        };
        if detail.as_ref().is_some_and(|old| old != &paint) {
            return Err(failure("native_operation_failed", "Shared material recipes conflict; accepted source preserved"));
        }
        detail = Some(paint);
    }
    let mut paint = detail.ok_or_else(|| failure("native_operation_failed", "Imported paint target is unavailable"))?;
    paint.version = 2;
    paint.effects = request.effects;
    let detail = DetailRecipe::Paint(paint);
    detail.validate(DetailLimits::default()).map_err(editor_error)?;
    backend.host()?.authoring_edit(AuthoringEdit::TextureDetail {
        material_id: request.material_id, detail: Some(detail),
    }, expected.source).map_err(editor_error)
        .and_then(|result|outcome(result, &authority))
}
