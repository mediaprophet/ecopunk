from staging import *
def read(name): return (Path('C:/github/poet')/name).read_text(encoding='utf-8').replace('\r\n','\n')
P='native/src/graphical/'
s=read(P+'jobs.rs')
s=change(s,'pub enum Action {','''pub enum Action {
    Viewport { request: super::viewport::ViewportCommand, expected: SourceRef },''')
s=change(s,'pub struct Reply {','''pub struct Reply {
    pub viewport: Option<super::viewport_contract::ViewportState>,''')
s=change(s,'    let mut surface = None;','    let mut surface = None;\n    let mut viewport = None;')
s=change(s,'Action::InspectSurface {expected,..} |','Action::Viewport {expected,..} | Action::InspectSurface {expected,..} |')
s=change(s,'&mut typed_10d, &mut surface, cancelled)','&mut typed_10d, &mut surface, &mut viewport, cancelled)')
s=change(s,'    Reply {\n        surface:','''    Reply {
        viewport: if inspection_is_current && image_uri.is_some() { viewport } else { None },
        surface:''')
s=change(s,'    match &action {\n        Action::BindSurfacePackage','''    if let Action::Viewport {request,..} = &action {
        super::viewport_contract::validate_command(request)?;
    }
    match &action {
        Action::BindSurfacePackage''')
stage(P+'jobs.rs',s)
s=read(P+'job_actions.rs')
s=change(s,'    cancelled: &std::sync::atomic::AtomicBool,','''    viewport: &mut Option<crate::graphical::viewport_contract::ViewportState>,
    cancelled: &std::sync::atomic::AtomicBool,''')
s=change(s,'    let message = match action {','''    let message = match action {
        Action::Viewport {request,expected} => {
            let output = host.viewport_action(request, expected, cancelled)?;
            let snapshot = host.snapshot()?;
            *viewport = Some(output.state);
            return outcome_image(crate::graphical::controller::EditorOutcome {
                snapshot, frame: Some(output.frame), message: "Native viewport updated; source unchanged.".into()
            });
        }''')
stage(P+'job_actions.rs',s)
s=read(P+'native_dispatch.rs')
s=change(s,'        Action::InspectSurface { expected, .. }','        Action::Viewport { expected, .. }\n        | Action::InspectSurface { expected, .. }')
s=change(s,'    let (kind, parameters) = match &action {','''    let (kind, parameters) = match &action {
        Action::Viewport {request,..} => (ActionType::Query, json!({"command":"viewport.navigate","request":request})),''')
s=change(s,'            let p = match edit {','''            let p = match edit {
                AuthoringEdit::EnableSourceLayers => json!({"command":"authoring.enable_source_layers"}),''')
stage(P+'native_dispatch.rs',s)
s=read(P+'poet_workspace/native_commands.rs')
s=change(s,'pub(super) enum Command {','''pub(super) enum Command {
    #[serde(rename = "viewport.navigate")]
    Viewport { request: crate::graphical::viewport::ViewportCommand },
    #[serde(rename = "authoring.enable_source_layers")]
    EnableSourceLayers,''')
s=change(s,'            Self::InspectSurface { .. }\n            | Self::BindSurface', '            Self::Viewport { .. }\n            | Self::InspectSurface { .. }\n            | Self::BindSurface')
s=change(s,'            Self::InspectSurface { .. } | Self::BindSurface { .. } => "Studio.inspect",','            Self::InspectSurface { .. } | Self::BindSurface { .. } => "Studio.inspect",') if False else s
# There are two matches with different indentation/content: the query branch above and tool route.
s=change(s,'            Self::Inspect => "Studio.inspect",','''            Self::Viewport { .. } => "Studio.inspect",
            Self::EnableSourceLayers => "Studio.edit",
            Self::Inspect => "Studio.inspect",''')
s=change(s,'        Ok(match self {','''        Ok(match self {
            Self::Viewport {request} => {
                crate::graphical::viewport_contract::validate_command(&request)?;
                Action::Viewport {request,expected}
            }
            Self::EnableSourceLayers => Action::AuthoringEdit {edit: AuthoringEdit::EnableSourceLayers, expected},''')
stage(P+'poet_workspace/native_commands.rs',s)
manifest()
print('Worker/native intent routing staged')
