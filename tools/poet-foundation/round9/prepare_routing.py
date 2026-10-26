"""Compose exact native Poet routing against reviewed live preimages, stage only."""
from staging import *
P='native/src/graphical/'
s=read(P+'editor_contract.rs')
s=change(s,'pub trait PreviewProvider: Send {','''pub trait PreviewProvider: Send {
    fn set_execution_context(&mut self, _epoch: &SessionEpoch) {}
    fn viewport(&mut self, _request: crate::graphical::viewport::ViewportCommand,
        _source: SourceRef, _value: &Value,
        _resources: Option<&crate::graphical::surfaces::TextureResources>,
        _cancelled: &std::sync::atomic::AtomicBool,
    ) -> Result<crate::graphical::viewport_contract::ViewportOutput, String> {
        Err("This preview provider does not support native viewport interaction".into())
    }
''')
s=change(s,'pub trait EditorHost: Send {','''pub trait EditorHost: Send {
    fn viewport_action(&mut self, _request: crate::graphical::viewport::ViewportCommand,
        _expected: SourceRef, _cancelled: &std::sync::atomic::AtomicBool,
    ) -> Result<crate::graphical::viewport_contract::ViewportOutput, String> {
        Err("Native viewport is not available from this host".into())
    }
''')
stage(P+'editor_contract.rs',s)
s=read(P+'controller.rs')
start=s.index('fn material_edit_id(');end=s.index('impl CanonicalEditorHost {',start)
support=s[start:end].replace('fn material_edit_id(', 'pub(super) fn material_edit_id(',1).replace('fn source_cells(', 'pub(super) fn source_cells(',1)
stage(P+'controller_support.rs','//! Existing projections extracted to keep the native controller focused.\nuse super::*;\n'+support)
s=s[:start]+'''#[path = "controller_support.rs"]
mod support;
use support::{material_edit_id, source_cells};
#[path = "controller_viewport.rs"]
mod viewport_implementation;

'''+s[end:]
s=change(s,'        let session = SourceSession::new(host.actual_grants().to_vec(), factory_limits.clone());','''        let session = SourceSession::new(host.actual_grants().to_vec(), factory_limits.clone());
        let execution_epoch = SessionEpoch::new();
        let mut preview = preview;
        preview.set_execution_context(&execution_epoch);''')
s=change(s,'            execution_epoch: SessionEpoch::new(),','            execution_epoch,')
s=change(s,'        self.execution_epoch = next_epoch;','''        self.execution_epoch = next_epoch;
        self.preview.set_execution_context(&self.execution_epoch);''')
s=change(s,'        self.execution_epoch = SessionEpoch::new();','''        self.execution_epoch = SessionEpoch::new();
        self.preview.set_execution_context(&self.execution_epoch);''')
s=change(s,'impl EditorHost for CanonicalEditorHost {','''impl EditorHost for CanonicalEditorHost {
    fn viewport_action(&mut self, request: crate::graphical::viewport::ViewportCommand,
        expected: SourceRef, cancelled: &std::sync::atomic::AtomicBool,
    ) -> Result<crate::graphical::viewport_contract::ViewportOutput, String> {
        self.viewport_native(request, expected, cancelled)
    }
''')
s=change(s,'        self.host.begin_submission();','''        let (layer_parts, layer_base) = if source.contains("_poet_source_layer") {
            crate::graphical::authoring::source_layers::validation_snapshot(
                self.session.value("poet_parts"), self.session.value("poet_parameter_parts"))?
        } else { (None, None) };
        self.host.begin_submission();''')
s=change(s,'            |value| {\n                provider','''            |value| {
                crate::graphical::authoring::source_layers::validate_result(
                    value, layer_parts.as_ref(), layer_base.as_ref()).map_err(|message|
                        vibe::Diagnostic::new(vibe::DiagCode::E701, vibe::Span::point(0), message))?;
                provider''')
stage(P+'controller.rs',s)
s=read(P+'execution.rs')
s=change(s,'        source.check_current()?;\n        session','''        source.check_current()?;
        let (layer_parts, layer_base) = if cell.contains("_poet_source_layer") {
            crate::graphical::authoring::source_layers::validation_snapshot(
                session.value("poet_parts"), session.value("poet_parameter_parts"))?
        } else { (None, None) };
        session''')
s=change(s,'                |value| {\n                    validator','''                |value| {
                    crate::graphical::authoring::source_layers::validate_result(
                        value, layer_parts.as_ref(), layer_base.as_ref()).map_err(|message|
                            vibe::Diagnostic::new(vibe::DiagCode::E701, vibe::Span::point(0), message))?;
                    validator''')
stage(P+'execution.rs',s)
method='''    fn viewport_action(&mut self, request: crate::graphical::viewport::ViewportCommand,
        expected: SourceRef, cancelled: &std::sync::atomic::AtomicBool,
    ) -> Result<crate::graphical::viewport_contract::ViewportOutput, String> {
        BODY
    }
'''
wrappers={'session_host.rs':('SessionEditorHost','self.current()?.viewport_action(request, expected, cancelled)'),
 'shared_host.rs':('SharedEditorHost','self.current()?.viewport_action(request, expected, cancelled)'),
 'checkpoints/editor.rs':('CheckpointEditorHost','self.inner.viewport_action(request, expected, cancelled)'),
 'package/mod.rs':('ResourceImportHost','self.inner.viewport_action(request, expected, cancelled)')}
for name,(host,body) in wrappers.items():
    s=read(P+name);tag=f'impl EditorHost for {host} {{'
    s=change(s,tag,tag+'\n'+method.replace('BODY',body));stage(P+name,s)
s=read(P+'cooperation/editor.rs')
s=change(s,'impl EditorHost for AuthorizedEditorHost {','''impl EditorHost for AuthorizedEditorHost {
'''+method.replace('BODY','''crate::graphical::viewport_contract::validate_command(&request)?;
        with_authorized(self.owner.as_ref(), &self.scope, Some(&expected),
            AuthoringAction::ReadSource, self.limits,
            |_, _| self.inner.viewport_action(request, expected.clone(), cancelled))'''))
stage(P+'cooperation/editor.rs',s)
s=read(P+'entry.rs');s=change(s,'        Box::new(provider),','        Box::new(super::viewport_provider::InteractivePreviewProvider::new(provider)?),')
stage(P+'entry.rs',s)
s=read(P+'native_entry/create.rs')
s=change(s,'    Box::new(MeshPreviewProvider {','    Box::new(crate::graphical::viewport_provider::InteractivePreviewProvider::new(MeshPreviewProvider {')
# Host-selected constant limits are validated by RetainedViewport at its first use.
s=change(s,'        limits: PreviewLimits::default(),\n    })','        limits: PreviewLimits::default(),\n    }).expect("host-selected native preview limits are valid"))')
stage(P+'native_entry/create.rs',s)
s=read(P+'mod.rs');s=change(s,'pub mod preview;','''pub mod preview;
pub mod viewport;
pub mod viewport_contract;
pub mod viewport_provider;
mod viewport_controls;
pub mod cooperative_workbench;''');stage(P+'mod.rs',s)
manifest()
print('Native routing staged, not installed')
