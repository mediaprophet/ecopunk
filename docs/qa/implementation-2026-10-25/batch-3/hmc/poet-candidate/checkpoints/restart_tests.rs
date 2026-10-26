//! Native diagnostic integration: actual Vibe, geometry/material admission,
//! checkpoint decorator, fresh-process load and explicit read-only rebuild.
//! Direct diagnostic host construction is not authenticated-human evidence.
use super::{
    CheckpointEditorHost, CheckpointLimits, CheckpointService, ExpectedFile, TrustedBinding,
};
use crate::graphical::{
    controller::{CanonicalEditorHost, EditorHost, PreviewProvider},
    geometry_host::GeometryHost,
    materials::{self, MaterialDraft},
    preview::{PreviewFrame, PreviewLimits, SourceRef},
};
use qualia_core_db::{
    container_10d::MaterialRecord,
    poet_host::invoke::studio_store::{self, checkpoint, StudioArtifact},
    render::{
        assets::Mesh,
        compile_10d::{compile_mesh_to_10d, decode_10d_mesh},
    },
};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use vibe::{session::SessionLimits, Value};

const SOURCE_ID: &str = "urn:poet:diagnostic:native-checkpoint-restart";
const CHILD_TEST: &str = "graphical::checkpoints::restart_tests::native_restart_child";
const FILE: &str = "native-restart.hmc";

#[derive(Clone)]
struct Observed {
    mesh: Mesh,
    material: MaterialRecord,
}
#[derive(Clone, Default)]
struct HeadlessAdmission(Arc<Mutex<Vec<Observed>>>);
impl PreviewProvider for HeadlessAdmission {
    fn validate(&self, value: &Value) -> Result<(), String> {
        let (mesh, material) = materials::authored_scene(
            value,
            MaterialRecord::legacy_default(),
            PreviewLimits::default(),
        )?;
        self.0
            .lock()
            .map_err(|_| "diagnostic observation poisoned")?
            .push(Observed { mesh, material });
        Ok(())
    }
    fn preview(&mut self, _: SourceRef, _: &Value) -> Result<PreviewFrame, String> {
        Err(
            "headless diagnostic deliberately does not render a frame; GPU acceptance is separate"
                .into(),
        )
    }
}
impl HeadlessAdmission {
    fn last(&self) -> Observed {
        self.0.lock().unwrap().last().unwrap().clone()
    }
    fn count(&self) -> usize {
        self.0.lock().unwrap().len()
    }
}

fn binding() -> TrustedBinding {
    TrustedBinding {
        project_id: "urn:project:diagnostic-restart".into(),
        document_id: "urn:document:diagnostic-restart".into(),
        source_id: SOURCE_ID.into(),
        domain: "Geometry".into(),
    }
}
fn service(root: &Path) -> CheckpointService {
    CheckpointService::new(root, binding(), CheckpointLimits::default()).unwrap()
}
fn host(root: &Path, admission: HeadlessAdmission) -> CheckpointEditorHost {
    let grants = vec![
        "capability.invoke".into(),
        "ComputationalGeometry.extrude_profile".into(),
    ];
    let canonical = CanonicalEditorHost::open(
        SOURCE_ID.into(),
        SessionLimits::default(),
        GeometryHost::new(grants, 200_000),
        Box::new(admission),
    )
    .unwrap();
    CheckpointEditorHost::new(Box::new(canonical), service(root)).unwrap()
}
fn first_edit() -> MaterialDraft {
    MaterialDraft {
        base_color: [0.05, 0.4, 0.8, 1.0],
        roughness: 0.13,
        metallic: 0.75,
        ..Default::default()
    }
}
fn second_edit() -> MaterialDraft {
    MaterialDraft {
        base_color: [0.7, 0.1, 0.04, 1.0],
        roughness: 0.82,
        metallic: 0.04,
        ..Default::default()
    }
}
fn assert_material(observed: &Observed, draft: &MaterialDraft) {
    assert_eq!(observed.material.id, materials::diagnostic_material_id());
    assert_eq!(observed.material.base_color, draft.base_color);
    assert_eq!(observed.material.roughness, draft.roughness);
    assert_eq!(observed.material.metallic, draft.metallic);
    assert!(!observed.mesh.positions.is_empty());
    assert!(!observed.mesh.triangles.is_empty());
}

fn write_phase(root: &Path) {
    assert!(studio_store::get_artifact_stamped(SOURCE_ID).is_none());
    let mut artifact = StudioArtifact::new(
        SOURCE_ID,
        "Geometry",
        "Diagnostic native material source restart",
    );
    artifact.properties.insert(
        "contribution.status".into(),
        "diagnostic; unreviewed; no human attribution asserted".into(),
    );
    studio_store::create_artifact_if_absent(artifact).unwrap();
    let admission = HeadlessAdmission::default();
    let mut editor = host(root, admission.clone());
    assert!(editor.checkpoint_supported());
    assert!(editor.snapshot().unwrap().checkpoint_supported);
    let initial = materials::diagnostic_source();
    let initial_source = editor.snapshot().unwrap().source;
    let outcome = editor.submit(&initial, initial_source).unwrap();
    assert!(outcome.frame.is_none());
    let before = admission.last();
    assert_material(&before, &MaterialDraft::default());
    let edit = first_edit()
        .edit_source(
            outcome.snapshot.session_revision + 1,
            outcome
                .snapshot
                .material_edit_id
                .expect("actual accepted mutable material binding"),
        )
        .unwrap();
    let outcome = editor
        .material_edit(&edit, outcome.snapshot.source)
        .unwrap();
    assert!(outcome.frame.is_none());
    let after = admission.last();
    assert_material(&after, &first_edit());
    assert_eq!(before.mesh.positions, after.mesh.positions);
    assert_eq!(before.mesh.triangles, after.mesh.triangles);
    assert_eq!(
        editor
            .source_cells(outcome.snapshot.source.clone())
            .unwrap(),
        vec![initial, edit]
    );
    let receipt = editor
        .save_checkpoint(FILE, ExpectedFile::Absent, outcome.snapshot.source)
        .unwrap();
    let saved = checkpoint::load(&receipt.path, Default::default()).unwrap();
    assert_eq!(
        saved.artifact(),
        &studio_store::get_artifact_stamped(SOURCE_ID).unwrap().0
    );
    assert_eq!(admission.count(), 2);
    // Native derived geometry is test evidence only, never an alternate source
    // store or a complete embodiment. Its actual codec proves cross-process mesh equality.
    let proof = compile_mesh_to_10d(&after.mesh).unwrap();
    assert!(proof.len() < 1024 * 1024);
    std::fs::write(root.join("observed-geometry.10d"), proof).unwrap();
}

fn read_phase(root: &Path) {
    assert!(studio_store::get_artifact_stamped(SOURCE_ID).is_none());
    let service = service(root);
    let prepared = service.prepare_reopen(FILE).unwrap();
    let previous_digest = prepared.file_digest();
    let exact_saved_source = prepared.artifact().clone();
    let (_, fresh_stamp) = service.activate(prepared).unwrap();
    let admission = HeadlessAdmission::default();
    let mut editor = host(root, admission.clone());
    let held = editor.snapshot().unwrap();
    assert_eq!(held.source.stamp, fresh_stamp);
    assert!(held.reconciliation_required);
    assert!(held.material_edit_id.is_none());
    assert_eq!(
        admission.count(),
        0,
        "load/open never evaluates retained source"
    );
    let rebuilt = editor.rebuild_execution(held.source).unwrap();
    assert!(rebuilt.frame.is_none());
    assert!(!rebuilt.snapshot.reconciliation_required);
    assert!(!held.execution_epoch.is_current());
    assert_eq!(rebuilt.snapshot.session_revision, 2);
    assert_eq!(
        studio_store::get_artifact_stamped(SOURCE_ID).unwrap().0,
        exact_saved_source
    );
    let rebuilt_observation = admission.last();
    assert_material(&rebuilt_observation, &first_edit());
    let proof_bytes = std::fs::read(root.join("observed-geometry.10d")).unwrap();
    assert!(proof_bytes.len() < 1024 * 1024);
    let proof = decode_10d_mesh(&proof_bytes).unwrap();
    assert_eq!(proof.positions, rebuilt_observation.mesh.positions);
    assert_eq!(proof.triangles, rebuilt_observation.mesh.triangles);
    let edit = second_edit()
        .edit_source(
            rebuilt.snapshot.session_revision + 1,
            rebuilt
                .snapshot
                .material_edit_id
                .expect("rebuild restores actual mutable material binding"),
        )
        .unwrap();
    let changed = editor
        .material_edit(&edit, rebuilt.snapshot.source)
        .unwrap();
    assert!(changed.frame.is_none());
    let edited = admission.last();
    assert_material(&edited, &second_edit());
    assert_eq!(edited.mesh.positions, proof.positions);
    assert_eq!(edited.mesh.triangles, proof.triangles);
    let receipt = editor
        .save_checkpoint(
            FILE,
            ExpectedFile::Sha256(previous_digest),
            changed.snapshot.source.clone(),
        )
        .unwrap();
    let saved = checkpoint::load(&receipt.path, Default::default()).unwrap();
    assert_eq!(saved.parent_digest(), Some(previous_digest));
    assert_eq!(
        saved.artifact(),
        &studio_store::get_artifact_stamped(SOURCE_ID).unwrap().0
    );
    assert_eq!(
        editor.source_cells(changed.snapshot.source).unwrap(),
        vec![
            materials::diagnostic_source(),
            first_edit()
                .edit_source(2, materials::diagnostic_material_id())
                .unwrap(),
            edit
        ]
    );
}

struct RunDirectory {
    root: PathBuf,
    path: PathBuf,
}
impl RunDirectory {
    fn new() -> Self {
        let root = std::env::var_os("POET_NATIVE_CHECKPOINT_TEST_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .canonicalize()
            .expect("existing host-selected test root");
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = root.join(format!(
            "poet-native-restart-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).unwrap();
        std::fs::write(
            path.join(".owned-native-restart"),
            b"Poet diagnostic RAII run",
        )
        .unwrap();
        Self { root, path }
    }
}
impl Drop for RunDirectory {
    fn drop(&mut self) {
        if self.path.parent() == Some(self.root.as_path())
            && self.path.join(".owned-native-restart").is_file()
        {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}

#[test]
fn source_material_checkpoint_survives_fresh_native_process() {
    let run = RunDirectory::new();
    for phase in ["write", "read"] {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", CHILD_TEST, "--nocapture", "--test-threads=1"])
            .env("POET_NATIVE_RESTART_PHASE", phase)
            .env("POET_NATIVE_RESTART_DIRECTORY", &run.path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "native child {phase}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let final_source = checkpoint::load(&run.path.join(FILE), Default::default()).unwrap();
    assert_eq!(final_source.artifact().nodes.len(), 3);
    assert!(final_source.parent_digest().is_some());
}

#[test]
fn native_restart_child() {
    let Some(phase) = std::env::var_os("POET_NATIVE_RESTART_PHASE") else {
        return;
    };
    let root = PathBuf::from(
        std::env::var_os("POET_NATIVE_RESTART_DIRECTORY").expect("parent-owned run directory"),
    );
    match phase.to_str().unwrap() {
        "write" => write_phase(&root),
        "read" => read_phase(&root),
        _ => panic!("unknown native restart diagnostic phase"),
    }
}
