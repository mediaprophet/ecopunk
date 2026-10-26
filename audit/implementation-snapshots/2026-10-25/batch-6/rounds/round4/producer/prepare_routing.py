from pathlib import Path

stage = Path(__file__).parent
root = Path('C:/github/poet')

def update(relative, transform):
    source = root / relative
    before = stage / 'before' / relative
    after = stage / 'after' / relative
    before.parent.mkdir(parents=True, exist_ok=True)
    after.parent.mkdir(parents=True, exist_ok=True)
    original = source.read_bytes()
    before.write_bytes(original)
    after.write_text(transform(original.decode('utf-8')), encoding='utf-8', newline='\n')

types = 'crate::graphical::checkpoints'
def method(name, request, body):
    return f'''    fn {name}(
        &mut self,
        request: {types}::{request},
        expected: SourceRef,
        limits: {types}::GeoRasterEditLimits,
        cancelled: &std::sync::atomic::AtomicBool,
    ) -> Result<qualia_core_db::poet_host::invoke::studio_store::checkpoint::SaveReceipt, String> {{
{body}
    }}
'''
def route(s, marker, methods):
    assert s.count(marker) == 1, marker
    return s.replace(marker, marker + '\n' + methods)

defaults = ''.join(method(name, req, '        let _ = (request, expected, limits, cancelled);\n        Err("Native georaster authoring is unavailable for this host".into())')
    for name, req in [('edit_georaster', 'GeoRasterEditRequest'), ('create_georaster', 'GeoRasterCreateRequest')])
update('native/src/graphical/editor_contract.rs', lambda s: route(s, 'pub trait EditorHost: Send {', defaults))
for relative, implementation, expression in [
    ('native/src/graphical/shared_host.rs', 'SharedEditorHost', 'self.current()?'),
    ('native/src/graphical/session_host.rs', 'SessionEditorHost', 'self.current()?'),
    ('native/src/graphical/package/mod.rs', 'ResourceImportHost', 'self.inner'),
]:
    methods = ''.join(method(name, req, f'        {expression}.{name}(request, expected, limits, cancelled)')
        for name, req in [('edit_georaster', 'GeoRasterEditRequest'), ('create_georaster', 'GeoRasterCreateRequest')])
    update(relative, lambda s, implementation=implementation, methods=methods: route(s, f'impl EditorHost for {implementation} {{', methods))

methods = ''
for name, req, field in [('edit_georaster', 'GeoRasterEditRequest', 'request.resource'), ('create_georaster', 'GeoRasterCreateRequest', 'request')]:
    methods += method(name, req, f'''        request.validate_bounds()?;
        let name = {field}.filename.clone();
        let file = studio_store::checkpoint::ExpectedFile::Sha256({field}.package_digest);
        self.write_package(&name, file, &expected, |inner| {{
            inner.{name}(request, expected.clone(), limits, cancelled)
        }})''')
update('native/src/graphical/cooperation/editor.rs', lambda s: route(s, 'impl EditorHost for AuthorizedEditorHost {', methods))

def jobs(s):
    s = route(s, 'pub enum Action {', '''    EditGeoRaster {
        request: super::checkpoints::GeoRasterEditRequest,
        expected: SourceRef,
    },
    CreateGeoRaster {
        request: super::checkpoints::GeoRasterCreateRequest,
        expected: SourceRef,
    },''')
    assert s.count('execute(host.as_mut(), request.action, request.origin)') == 1
    s = s.replace('execute(host.as_mut(), request.action, request.origin)', 'execute(host.as_mut(), request.action, request.origin, &worker_closing)')
    s = s.replace('fn execute(host: &mut dyn EditorHost, action: Action, origin: Context) -> Reply {',
        'fn execute(host: &mut dyn EditorHost, action: Action, origin: Context, cancelled: &AtomicBool) -> Reply {')
    s = s.replace('Action::InspectResource{expected,..} | Action::AuthoringInspect',
        'Action::EditGeoRaster{expected,..} | Action::CreateGeoRaster{expected,..} | Action::InspectResource{expected,..} | Action::AuthoringInspect')
    s = s.replace('perform(host, action, &mut inventory,&mut workflow,&mut authoring,&mut inspection)',
        'perform(host, action, &mut inventory,&mut workflow,&mut authoring,&mut inspection, cancelled)')
    marker = 'pub async fn run(host: HostHandle, action: Action, origin: Context) -> Result<Reply, String> {'
    return route(s, marker, '''    match &action {
        Action::EditGeoRaster { request, .. } => request.validate_bounds()?,
        Action::CreateGeoRaster { request, .. } => request.validate_bounds()?,
        _ => {}
    }''')
update('native/src/graphical/jobs.rs', jobs)

def actions(s):
    marker = '    inspection: &mut Option<crate::graphical::checkpoints::ResourceInspection>,'
    assert s.count(marker) == 1
    s = s.replace(marker, marker + '\n    cancelled: &std::sync::atomic::AtomicBool,')
    methods = ''
    for variant, name, field, message in [
        ('EditGeoRaster', 'edit_georaster', 'request.resource', 'Persisted developmental v3 raster edited'),
        ('CreateGeoRaster', 'create_georaster', 'request', 'Explicit source compiled into a new developmental v3 raster resource'),
    ]:
        methods += f'''        Action::{variant} {{ request, expected }} => {{
            let name = {field}.filename.clone();
            let receipt = host.{name}(request, expected.clone(),
                crate::graphical::checkpoints::ui_georaster_edit_limits(), cancelled)?;
            *inventory = host.package_inventory(&name, expected).ok();
            format!("{message} in {{name}}; package SHA256 {{}}. Retained source/resources preserved; refresh inventory. Package review and scientific acceptance remain separate.", crate::graphical::package::hex_digest(&receipt.digest))
                + &crate::graphical::package::durability_note(&receipt)
        }}
'''
    return route(s, '    let message = match action {', methods)
update('native/src/graphical/job_actions.rs', actions)

def controls(s):
    s = route(s, 'use inspection_view::M64InspectionView;', '''#[path = "package_georaster_edit_view.rs"]
mod georaster_edit_view;
use georaster_edit_view::GeoRasterEditView;
#[path = "package_georaster_create_view.rs"]
mod georaster_create_view;
use georaster_create_view::GeoRasterCreateView;''')
    # Child modules are nested below package_controls and use parent imports:
    # root can equally mount them as graphical children during merged routing.
    s = s.replace('M64InspectionView{view:view.result}', '''M64InspectionView{view:view.result.clone()}
                    GeoRasterEditView{key:"{hex_digest(&view.result.request.resource_digest)}",
                        view:view.result,host:host.clone(),current:current.clone(),busy,status,
                        on_saved:move |_|{inventory.set(None);inspection.set(None);}}
''')
    marker = '                for resource in package.resources {'
    assert s.count(marker) == 1
    s = s.replace(marker, '''                GeoRasterCreateView{key:"{hex_digest(&package.digest)}",
                    host:host.clone(),current:current.clone(),filename:name(),package_digest:package.digest,
                    busy,status,on_saved:move |_|{inventory.set(None);inspection.set(None);}}
''' + marker)
    return s
update('native/src/graphical/package_controls.rs', controls)
print('Prepared producer routing snapshots; sibling sources unchanged.')
