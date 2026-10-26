from pathlib import Path
import hashlib, json

root=Path(__file__).parent/'native-integration'
root.mkdir(exist_ok=True)
candidates=root/'candidate'
candidates.mkdir(exist_ok=True)
changes=[]
def edit(relative, transform):
    target=Path('C:/github/poet/native')/relative
    old=target.read_bytes() if target.exists() else None
    source=transform(old.decode('utf-8') if old else '')
    candidate=candidates/relative
    candidate.parent.mkdir(parents=True, exist_ok=True)
    candidate.write_text(source,encoding='utf-8',newline='\n')
    changes.append(dict(target=str(target),candidate='candidate/'+relative,
        original_sha256=hashlib.sha256(old).hexdigest() if old else None,
        candidate_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest()))

def replace(source,old,new):
    assert source.count(old)==1,old[:100]
    return source.replace(old,new)

def controller(s):
    s=replace(s,'    pub reconciliation_required: bool,','    pub reconciliation_required: bool,\n    pub material_edit_id: Option<u64>,\n    pub checkpoint_supported: bool,')
    s=replace(s,'pub trait PreviewProvider: Send {','''pub trait PreviewProvider: Send {
    /// Native output admission before local session/source promotion; no GPU.
    fn validate(&self, _value: &Value) -> Result<(), String> { Ok(()) }''')
    s=replace(s,'impl PreviewProvider for MeshPreviewProvider {','''impl PreviewProvider for MeshPreviewProvider {
    fn validate(&self, value: &Value) -> Result<(), String> {
        super::materials::authored_scene(value, self.material, self.limits).map(|_| ())
    }''')
    s=replace(s,'        let mesh = preview::mesh_from_invoke(value, self.limits)?;\n        preview::render_mesh(source, &mesh, self.material, self.camera, self.limits)','''        let (mesh, material) = super::materials::authored_scene(value, self.material, self.limits)?;
        preview::render_mesh(source, &mesh, material, self.camera, self.limits)''')
    s=replace(s,'pub trait EditorHost: Send {','''pub trait EditorHost: Send {
    fn source_cells(&self, _expected: SourceRef) -> Result<Vec<String>, String> {
        Err("identified source-cell read is unavailable for this host".into())
    }
    fn material_edit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String> {
        let _ = (source, expected);
        Err("material-only source admission is unavailable for this host".into())
    }
    fn checkpoint_supported(&self) -> bool { false }
    fn save_checkpoint(&mut self, _name: &str,
        _expected_file: studio_store::checkpoint::ExpectedFile, _expected: SourceRef,
    ) -> Result<studio_store::checkpoint::SaveReceipt, String> {
        Err("native checkpoint destination/admission is unavailable for this host".into())
    }''')
    s=replace(s,'                reconciliation_required: false,','''                reconciliation_required: false,
                material_edit_id: material_edit_id(&candidate.session, false),
                checkpoint_supported: false,''')
    s=replace(s,'            reconciliation_required: self.reconciliation_required || stamp != self.active_stamp,','''            reconciliation_required: self.reconciliation_required || stamp != self.active_stamp,
            material_edit_id: material_edit_id(&self.session, self.reconciliation_required || stamp != self.active_stamp),
            checkpoint_supported: false,''')
    s=replace(s,'impl EditorHost for CanonicalEditorHost {','''impl EditorHost for CanonicalEditorHost {
    fn source_cells(&self, expected: SourceRef) -> Result<Vec<String>, String> {
        if expected.artifact_id != self.artifact_id { return Err("foreign source read".into()); }
        expected.check_current()?;
        let (artifact, stamp) = studio_store::get_artifact_stamped(&self.artifact_id)
            .ok_or("source artifact disappeared")?;
        if stamp != expected.stamp { return Err("source changed before read".into()); }
        let cells = source_cells(&artifact)?.into_iter().map(str::to_string).collect();
        expected.check_current()?;
        Ok(cells)
    }''')
    s=replace(s,'            &self.rebuild_limits,\n        )','            &self.rebuild_limits,\n            self.preview.as_ref(),\n        )')
    s=replace(s,'        let result = match self\n            .session\n            .submit(source, &mut self.host, Budget::default())','''        let provider = &self.preview;
        let result = match self.session.submit_validated(source, &mut self.host, Budget::default(), |value| {
            provider.validate(value).map_err(|message| vibe::Diagnostic::new(
                vibe::DiagCode::E701, vibe::Span::point(0), message))?;
            expected.check_current().map_err(|message| vibe::Diagnostic::new(
                vibe::DiagCode::E701, vibe::Span::point(0), message))
        })''')
    s=replace(s,'fn source_cells(artifact: &StudioArtifact)', '''fn material_edit_id(session: &SourceSession, held: bool) -> Option<u64> {
    if held || !session.is_mutable("poet_geometry") || !session.is_mutable("poet_material") {
        return None;
    }
    let material = super::materials::material_from_value(session.value("poet_material")?).ok()?;
    if !matches!(session.value("poet_geometry")?, Value::Record(_)) { return None; }
    Some(material.id)
}

fn source_cells(artifact: &StudioArtifact)''')
    return s
edit('src/graphical/controller.rs',controller)
edit('src/graphical/jobs_tests.rs',lambda s:replace(s,'        reconciliation_required: false,','        reconciliation_required: false,\n        material_edit_id: None,\n        checkpoint_supported: false,'))
def execution(s):
    s=replace(s,'            .submit(\n','            .submit_validated(\n')
    s=replace(s,'    bound: &RebuildLimits,\n','    bound: &RebuildLimits,\n    validator: &dyn super::PreviewProvider,\n')
    return replace(s,'                    workspace_left: (limits.retained_value_bytes as u64).min(vibe::SENTINEL_BYTES),\n                },','''                    workspace_left: (limits.retained_value_bytes as u64).min(vibe::SENTINEL_BYTES),
                },
                |value| {
                    validator.validate(value).map_err(|message|
                        vibe::Diagnostic::new(vibe::DiagCode::E701, vibe::Span::point(0), message))
                },''')
edit('src/graphical/execution.rs',execution)
def jobs(s):
    s=replace(s,'    Refresh,','    Save { name: String, expected: SourceRef },\n    Refresh,')
    s=replace(s,'        Action::Refresh => "Current identified source read from canonical store".into(),','''        Action::Save { name, expected } => {
            let receipt=host.save_checkpoint(&name,
                qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Absent,expected)?;
            let digest=receipt.digest.iter().map(|b|format!("{b:02x}")).collect::<String>();
            format!("Accepted source/history saved as {name} ({} bytes, SHA256 {digest}); draft and human acceptance are separate",receipt.bytes)
        }
        Action::Refresh => "Current identified source read from canonical store".into(),''')
    s=replace(s,'Action::Rebuild(expected) => Some(expected),','Action::Rebuild(expected) | Action::Save { expected, .. } => Some(expected),')
    s=replace(s,'    let (sender, receiver) = futures_channel::oneshot::channel();','''    if matches!(&action, Action::Save { name, .. } if name.len()>128) {
        return Err("Checkpoint filename exceeds native request budget".into());
    }
    let (sender, receiver) = futures_channel::oneshot::channel();''')
    return s
edit('src/graphical/jobs.rs',jobs)
def ui(s):
    s=replace(s,'            && self.material_edit_id == other.material_edit_id','            && self.material_edit_id == other.material_edit_id\n            && self.checkpoint_supported == other.checkpoint_supported')
    s=replace(s,'    let current = view();','''    let current = view();
    let mut checkpoint_name=use_signal(|| "poet-checkpoint.hmc".to_string());
    let mut save={
        let host=host.clone();
        move |_| dispatch(Action::Save { name:checkpoint_name(),expected:view().source },
            host.clone(),view,draft,status,busy,image,false)
    };''')
    return replace(s,'                        p { "{status}" }','''                        if current.checkpoint_supported {
                            div { style: "display:flex;gap:8px;flex-wrap:wrap;",
                                input { value: "{checkpoint_name}", aria_label: "HMC checkpoint filename",
                                    disabled: busy(), oninput: move |event| checkpoint_name.set(event.value()) }
                                button { disabled: busy(), onclick: move |event| save(event), "Save accepted source as new HMC" }
                            }
                        }
                        p { "{status}" }''')
edit('src/graphical/ui.rs',ui)
edit('src/graphical/mod.rs',lambda s:replace(s,'pub mod controller;','pub mod controller;\npub mod cooperation;\npub mod checkpoints;\npub mod mcp;'))
edit('src/lib.rs',lambda s:'''//! Poet native authoring services, shared by Aether UI and trusted host MCP embeddings.
#[cfg(feature = "native-graphical")]
pub mod graphical;
''')
def main(s):
    s=replace(s,'mod graphical;','use poet_cli::graphical;')
    return replace(s,'    match args[1].as_str() {','''    match args[1].as_str() {
        #[cfg(feature = "native-graphical")]
        "mcp" => {
            if args.len() != 2 {
                eprintln!("Usage: poet mcp; project admission is supplied only by a native embedding");
                process::exit(1);
            }
            if let Err(error) = graphical::mcp::run_unbound_stdio() {
                eprintln!("Poet MCP: {error}"); process::exit(1);
            }
        }''')
edit('src/main.rs',main)
def cargo(s):
    s=replace(s,'[features]','''qualia-cooperative-core = { path = "C:/github/qualiaDB/crates/qualia-cooperative-core", optional = true }
qualia-mcp = { path = "C:/github/qualiaDB/crates/qualia-mcp", default-features = false, features = ["authoring"], optional = true }
serde = { workspace = true, features = ["derive"], optional = true }
serde_json = { workspace = true, optional = true }
sha2 = { version = "0.10", optional = true }

[features]''')
    return replace(s,'"dep:futures-channel"]','"dep:futures-channel", "dep:qualia-cooperative-core", "dep:qualia-mcp", "dep:serde", "dep:serde_json", "dep:sha2"]')
edit('Cargo.toml',cargo)
# The new MCP adapter is authored in the workspace, then promoted by this same
# guarded manifest. An existing canonical adapter is never overwritten here.
adapter=candidates/'src/graphical/mcp/mod.rs'
adapter.write_text(adapter.read_text().replace('use super::super::controller','use super::controller').replace('use super::super::cooperation','use super::cooperation'),encoding='utf-8',newline='\n')
target=Path('C:/github/poet/native/src/graphical/mcp/mod.rs')
assert not target.exists()
changes.append(dict(target=str(target),candidate='candidate/src/graphical/mcp/mod.rs',original_sha256=None,candidate_sha256=hashlib.sha256(adapter.read_bytes()).hexdigest()))
(root/'apply-manifest.json').write_text(json.dumps(dict(scope='Canonical native pre-promotion material admission and editor service extension',mutations=changes),indent=2))
