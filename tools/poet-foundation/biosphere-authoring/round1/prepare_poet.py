"""Root-owned shared integration candidates; no canonical mutation."""
from pathlib import Path
import hashlib
import json

ROOT = Path('C:/github/ecopunk')
WORK = ROOT / 'tools/poet-foundation/biosphere-authoring/round1/work'
POET = Path('C:/github/poet')
STAGE = WORK / 'ROOT/poet'
files = {}

def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def put(path, text):
    dst = STAGE / path
    dst.parent.mkdir(parents=True, exist_ok=True)
    original = POET / path
    before = digest(original) if original.exists() else None
    dst.write_text(text, encoding='utf-8', newline='\n')
    files[path] = dict(repo='poet', path=path, staged=str(dst), before=before, after=digest(dst))

def edit(path, replacements):
    text = (POET / path).read_text(encoding='utf-8')
    for old, new in replacements:
        assert text.count(old) == 1, (path, old, text.count(old))
        text = text.replace(old, new)
    put(path, text)

base = 'native/src/graphical/'
edit(base+'mod.rs', [('pub mod assembly;', 'pub mod assembly;\npub mod botanical_graph_host;\npub mod rig_binding;\npub mod animated_rig_preview;')])
edit(base+'authoring/mod.rs', [
    ('mod assembly_source;', 'mod assembly_source;\npub mod botanical;\npub(crate) mod botanical_session;'),
    ('pub struct AuthoringView {', 'pub struct AuthoringView {\n    pub botanical: Option<botanical::BotanicalRecipe>,'),
    ('pub enum AuthoringEdit {', 'pub enum AuthoringEdit {\n    Botanical { edit: botanical::BotanicalEdit },'),
])
edit(base+'authoring/source.rs', [('material_motion: Vec::new(),', 'material_motion: Vec::new(),\n        botanical: None,')])
edit(base+'authoring/controller.rs', [
    ('        Ok(view)', '''        view.botanical = super::super::authoring::botanical_session::validate(&self.session, &self.host)?;
        Ok(view)'''),
    ('        let source = if let AuthoringEdit::MaterialMotion', '''        let source = if let AuthoringEdit::Botanical { edit } = &edit {
            if self.session.value("poet_source_layer").is_some()
                || self.session.value("poet_parameter_parts").is_some() {
                return Err("Botanical graph edits require explicit source-layer correspondence; accepted recipe preserved".into());
            }
            if self.session.value("poet_botanical").is_some() && !self.session.is_mutable("poet_botanical") {
                return Err("Botanical binding is immutable; accepted source preserved".into());
            }
            let limits = self.host.botanical_limits()?;
            crate::graphical::authoring::botanical::edit_source(parts,
                self.session.value("poet_botanical"), edit, limits)?
        } else if let AuthoringEdit::MaterialMotion'''),
    ('        crate::graphical::package::ResourceLibrary::validate_filename(key)?;', '''        crate::graphical::authoring::botanical_session::validate(&self.session, &self.host)?;
        crate::graphical::package::ResourceLibrary::validate_filename(key)?;'''),
])
put(base+'authoring/botanical_session.rs', '''//! Validate derived organ/mesh correspondence before publishing or compiling.
use super::botanical::{BotanicalRecipe, read_recipe, validate_projection};
use crate::graphical::geometry_host::GeometryHost;
use vibe::session::SourceSession;

pub(crate) fn validate(session: &SourceSession, host: &GeometryHost) -> Result<Option<BotanicalRecipe>, String> {
    let Some(binding) = session.value("poet_botanical") else { return Ok(None) };
    let limits = host.botanical_limits()?;
    let recipe = read_recipe(Some(binding), limits)?.ok_or("Botanical recipe missing")?;
    let parts = session.value("poet_parts").ok_or("Botanical source has no parts")?;
    validate_projection(parts, Some(&recipe), &recipe, limits)?;
    Ok(Some(recipe))
}
''')
edit(base+'geometry_host.rs', [('    pub(crate) fn vertex_limit(&self) -> usize {', '''    /// Typed native edits require the same actual host capabilities as script effects.
    /// These bounded cold allocations are selected by the host, never by source.
    pub(crate) fn botanical_limits(&self) -> Result<crate::graphical::authoring::botanical::BotanicalLimits, String> {
        for required in ["capability.invoke", "ComputationalGeometry.space_colonize_3d",
            "ComputationalGeometry.pipe_model_radii", "ComputationalGeometry.extrude_profile"] {
            if !self.grants.iter().any(|grant| grant == required) {
                return Err(format!("Botanical operation requires actual host grant {required}"));
            }
        }
        if self.limits.dispatches < 3 || self.limits.scratch_bytes < 4096 {
            return Err("Botanical kernel dispatch/scratch budget unavailable".into());
        }
        let mut limits = crate::graphical::authoring::botanical::BotanicalLimits::default();
        limits.max_vertices = limits.max_vertices.min(self.max_vertices).min(self.limits.submission_work / 3)
            .min(self.limits.scratch_bytes / 512);
        limits.max_triangles = limits.max_triangles.min(limits.max_vertices.saturating_mul(2));
        limits.max_proposal_nodes = limits.max_proposal_nodes.min(self.limits.scratch_bytes / 256);
        limits.max_organs = limits.max_organs.min(limits.max_proposal_nodes);
        limits.max_attractors = limits.max_attractors.min(self.limits.scratch_bytes / 128);
        // Each iteration scans bounded node/attractor pairs. The profile also
        // bounds cold solver work independently of final visible vertex count.
        limits.max_iterations = limits.max_iterations.min(16);
        limits.max_proposal_nodes = limits.max_proposal_nodes.min(512);
        limits.max_attractors = limits.max_attractors.min(256);
        Ok(limits)
    }
    pub(crate) fn vertex_limit(&self) -> usize {''')])
edit(base+'controller.rs', [('        // External effects and the session may already have advanced.', '''        // A source edit can alter recipe bindings without returning them. Inspect
        // the actual resulting session before committing editable source/history.
        // Runtime effects are not rolled back; require explicit rebuild on failure.
        if let Err(error) = crate::graphical::authoring::botanical_session::validate(&self.session, &self.host) {
            self.reconciliation_required = true;
            return Err(format!("{error}; accepted source preserved; execution rebuild required"));
        }
        // External effects and the session may already have advanced.''')])
edit(base+'execution.rs', [('    source.check_current()?;\n    Ok(PreparedRebuild {', '''    crate::graphical::authoring::botanical_session::validate(&session, &host)?;
    source.check_current()?;
    Ok(PreparedRebuild {''')])
edit(base+'native_dispatch.rs', [('                AuthoringEdit::MaterialMotion { edit } => {', '''                AuthoringEdit::Botanical { edit } => {
                    json!({"command":"authoring.botanical","edit":edit})
                }
                AuthoringEdit::MaterialMotion { edit } => {''')])
edit(base+'poet_workspace/native_commands.rs', [
    ('pub(super) enum Command {', '''pub(super) enum Command {
    #[serde(rename = "authoring.botanical")]
    Botanical { edit: crate::graphical::authoring::botanical::BotanicalEdit },'''),
    ('Self::EnableSourceLayers | Self::MaterialMotion { .. }', 'Self::Botanical { .. } | Self::EnableSourceLayers | Self::MaterialMotion { .. }'),
    ('            Self::MaterialMotion { edit } => {', '''            Self::Botanical { edit } => Action::AuthoringEdit {
                edit: AuthoringEdit::Botanical { edit }, expected,
            },
            Self::MaterialMotion { edit } => {'''),
])
edit(base+'authoring/controls.rs', [
    ('    let motion_source = current.source.clone();', '''    let motion_source = current.source.clone();
    let botanical_source = current.source.clone();
    let botanical_host = host.clone();
    let botanical_buffer = buffer.clone();'''),
    ('                super::material_motion_controls::MaterialMotionControls', '''                super::botanical::BotanicalControls {
                    accepted: data.botanical.clone(), selected_part: selected(), source: botanical_source.clone(),
                    fresh: fresh && !busy(),
                    onedit: move |edit| send(Action::AuthoringEdit {edit:AuthoringEdit::Botanical {edit}, expected:botanical_source.clone()}, botanical_host.clone(),view,draft,status,image,busy,inspected,botanical_buffer.clone(),true),
                }
                super::material_motion_controls::MaterialMotionControls'''),
])
edit(base+'mcp/mod.rs', [
    ('mod material_motion;', 'mod material_motion;\nmod botanical;'),
    ('        tools.extend(material_motion::tools());', '        tools.extend(botanical::tools());\n        tools.extend(material_motion::tools());'),
    ('        if matches!(name, "poet_texture_effects_inspect"', '''        if matches!(name, "poet_botanical_inspect" | "poet_botanical_edit") {
            return botanical::call(self, name, arguments);
        }
        if matches!(name, "poet_texture_effects_inspect"'''),
])
put(base+'mcp/botanical.rs', '''//! Same source-owner operation for admitted native agent and operator edits.
use super::*;
use crate::graphical::authoring::{AuthoringEdit, botanical::BotanicalEdit};

pub(super) fn tools() -> Vec<AuthoringTool> {
    let key=json!({"type":"string","minLength":1,"maxLength":128});
    let positive=json!({"type":"number","exclusiveMinimum":0,"maximum":1000000});
    let point=json!({"type":"array","items":{"type":"number","minimum":-1000000,"maximum":1000000},"minItems":3,"maxItems":3});
    let organ=json!({"type":"object","properties":{"key":key,"parent":{"type":["string","null"]},"position_m":point},"required":["key","parent","position_m"],"additionalProperties":false});
    let recipe=json!({"type":"object","properties":{"version":{"const":1},"part_id":key,
        "organs":{"type":"array","items":organ,"minItems":2,"maxItems":256},
        "attractors_m":{"type":"array","items":point,"minItems":1,"maxItems":256},
        "node_length_m":positive,"kill_radius_m":positive,"perception_radius_m":positive,
        "cone_angle_rad":{"type":"number","exclusiveMinimum":0,"maximum":std::f64::consts::TAU},
        "max_iterations":{"type":"integer","minimum":0,"maximum":16},"tip_radius_m":positive,
        "pipe_exponent":{"type":"number","minimum":1,"maximum":8},"ring_slices":{"type":"integer","minimum":3,"maximum":64}},
        "required":["version","part_id","organs","attractors_m","node_length_m","kill_radius_m","perception_radius_m","cone_angle_rad","max_iterations","tip_radius_m","pipe_exponent","ring_slices"],"additionalProperties":false});
    let edit=json!({"oneOf":[
        {"type":"object","properties":{"operation":{"const":"set_recipe"},"recipe":recipe},"required":["operation","recipe"],"additionalProperties":false},
        {"type":"object","properties":{"operation":{"const":"set_tip_radius"},"part_id":key,"tip_radius_m":positive},"required":["operation","part_id","tip_radius_m"],"additionalProperties":false},
        {"type":"object","properties":{"operation":{"const":"set_organ_position"},"part_id":key,"organ_key":key,"position_m":point},"required":["operation","part_id","organ_key","position_m"],"additionalProperties":false}
    ]});
    vec![tool("poet_botanical_inspect", "Inspect adopted organ keys and source-linked graph; not species acceptance", schema(false,true)),
        tool("poet_botanical_edit", "Adopt a bounded graph, set tip radius or move an adopted organ through existing source history and authority; unsupported remapping is refused",
            json!({"type":"object","properties":{"context":{"type":"string","maxLength":128},"edit":edit},"required":["context","edit"],"additionalProperties":false}))]
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request { context:String, edit:BotanicalEdit }
pub(super) fn call(backend:&mut NativeAuthoringBackend, name:&str, arguments:Value) -> Result<Value,ToolError> {
    if name == "poet_botanical_inspect" {
        let request:Context = args(arguments)?;
        if request.context.len()>128 { return Err(failure("limit","Context exceeds native bound")); }
        let expected=backend.expected(&request.context)?;
        let view=backend.host()?.authoring_view(expected.source.clone()).map_err(editor_error)?;
        receipt_authority(&expected.authorisation_stamp)?;
        expected.source.check_current().map_err(editor_error)?;
        return Ok(json!({"source_revision":expected.source_revision,"botanical":view.botanical,"human_review":"unassessed"}));
    }
    let request:Request=args(arguments)?;
    if request.context.len()>128 { return Err(failure("limit","Context exceeds native bound")); }
    let expected=backend.expected(&request.context)?;
    backend.context=None;
    let authority=expected.authorisation_stamp.clone();
    backend.host()?.authoring_edit(AuthoringEdit::Botanical { edit:request.edit },expected.source)
        .map_err(editor_error).and_then(|result|outcome(result,&authority))
}
''')
# Old singleton-login assumptions become coexistence assertions; retain explicit
# signed revocation/logout tests elsewhere rather than weaken denial coverage.
edit(base+'native_project_workflow_tests.rs', [
    ('    assert!(editor.snapshot().is_err());\n    assert!(!author_publication.publication_is_current());',
     '    assert!(editor.snapshot().is_ok(), "reviewer login preserves the author session");\n    assert!(author_publication.publication_is_current());'),
])
edit(base+'native_project_texture_effect_tests.rs', [
    ('    assert!(ui.snapshot().is_err(), "single active authority login retires author\'s handles");\n    assert!(mcp.call("poet_source_read", json!({})).is_err());',
     '    assert!(ui.snapshot().is_ok(), "reviewer login preserves the author session");\n    assert!(mcp.call("poet_source_read", json!({})).is_ok());'),
])
edit(base+'native_project_motion_tests.rs', [
    ('    // This native owner has one active authenticated actor. A successful login\n    // revokes its previous session; sharing a project does not preserve old handles.',
     '    // An independent reviewer proof does not revoke the existing author.'),
    ('    assert!(ui.snapshot().is_err(), "actor handoff retires the author\'s old host");\n    assert!(mcp.call("poet_source_read", json!({})).is_err());',
     '    assert!(ui.snapshot().is_ok(), "reviewer login preserves independent author handles");\n    assert!(mcp.call("poet_source_read", json!({})).is_ok());'),
])
edit(base+'native_entry/create.rs', [
    ('fn native_geometry_grants(revolve: bool)', 'fn native_geometry_grants(revolve: bool, botanical: bool)'),
    ('    grants\n}', '''    if botanical {
        grants.push("ComputationalGeometry.space_colonize_3d".into());
        grants.push("ComputationalGeometry.pipe_model_radii".into());
    }
    grants
}'''),
    ('native_geometry_grants(false);','native_geometry_grants(false, false);'),
    ('native_geometry_grants(true);','native_geometry_grants(true, false);'),
    ('            std::env::var("POET_NATIVE_REVOLVE_PROFILE").as_deref() == Ok("1"),',
     '            std::env::var("POET_NATIVE_REVOLVE_PROFILE").as_deref() == Ok("1"),\n            std::env::var("POET_NATIVE_BOTANICAL_PROFILE").as_deref() == Ok("1"),'),
    ('pub(super) fn project_config(', '''#[cfg(test)]
#[test]
fn native_botanical_kernels_require_host_opt_in() {
    let legacy = native_geometry_grants(false, false);
    let enabled = native_geometry_grants(false, true);
    assert_eq!(enabled.len(), legacy.len()+2);
    assert!(enabled.starts_with(&legacy));
    assert!(enabled.iter().any(|g| g == "ComputationalGeometry.space_colonize_3d"));
    assert!(enabled.iter().any(|g| g == "ComputationalGeometry.pipe_model_radii"));
}
pub(super) fn project_config('''),
])
test_module = (POET/(base+'native_project_tests.rs')).read_text(encoding='utf-8')
put(base+'native_project_tests.rs', test_module + '''
#[path = "native_project_biosphere_round1_tests.rs"]
mod biosphere_round1_tests;
#[path = "native_selected_session_tests.rs"]
mod native_selected_session_tests;
''')
edit(base+'cooperative_workflow/service.rs', [
    ('        let session = self.session()?;', '''        // A selected actor cannot change between source inspection and durable
        // contribution/review commit. Independent project sessions coexist.
        // Match the selection -> authority -> raw editor lock order.
        let selection = self.sessions.try_lock().map_err(|_| "Native workflow actor selection busy or poisoned")?;
        let session = selection.as_ref().ok_or("Sign in before cooperative work")?.clone();'''),
])

manifest = dict(scope='W3R1 root shared Poet botanical source/native/MCP integration and rig registration', files=list(files.values()))
(WORK/'poet-root-install.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
print('Prepared',len(files),'shared Poet candidates')
