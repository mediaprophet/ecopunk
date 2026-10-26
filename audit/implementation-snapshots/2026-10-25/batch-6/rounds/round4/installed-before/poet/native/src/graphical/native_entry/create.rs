//! Explicit local-root creation. No existing project or legal identity is adopted automatically.
use super::{
    files::{self, HostFiles, PublicProject},
    runtime::{self, Runtime},
};
use crate::graphical::{
    checkpoints::{CheckpointLimits, ExpectedFile, PackageWritePolicy},
    controller::{EditorHost, MeshPreviewProvider, PreviewProvider},
    cooperation::{source_digest, GatewayLimits},
    native_project::{NativeProject, ProjectConfig},
    preview::{PreviewCamera, PreviewLimits},
};
use qualia_cooperative_core::{
    authoring::{DocumentRevision, PrincipalKind},
    projects::{Project, ProjectRole},
};
use qualia_core_db::{
    container_10d::MaterialRecord,
    poet_host::{
        invoke::studio_store::{self, StudioArtifact},
        native_authority::{enrollment::NativeSigner, AdmissionSnapshot, NativeAuthority},
    },
};
use std::sync::Arc;
use vibe::session::SessionLimits;

pub(super) fn preview() -> Box<dyn PreviewProvider> {
    let mut material = MaterialRecord::legacy_default();
    material.base_color = [0.36, 0.20, 0.085, 1.0];
    material.roughness = 0.65;
    Box::new(MeshPreviewProvider {
        material,
        camera: PreviewCamera {
            yaw: 0.45,
            pitch: 0.2,
            distance: 3.2,
            target: [0.0; 3],
        },
        limits: PreviewLimits::default(),
    })
}
pub(super) fn project_config(
    files: &HostFiles,
    public: &PublicProject,
) -> Result<ProjectConfig, String> {
    Ok(ProjectConfig {
        scope: public.scope(),
        checkpoint_root: files.directory("packages", false)?,
        checkpoint_limits: CheckpointLimits::default(),
        write_policy: PackageWritePolicy::WorkingCopy,
        import_library: None,
        session_limits: SessionLimits::default(),
        gateway_limits: GatewayLimits::default(),
        geometry_grants: vec![
            "capability.invoke".into(),
            "ComputationalGeometry.extrude_profile".into(),
            "ComputationalGeometry.swung_surface".into(),
            "ComputationalGeometry.profile_sweep".into(),
            "ComputationalGeometry.uv_sphere".into(),
            "ComputationalGeometry.torus".into(),
        ],
        max_vertices: 200_000,
    })
}
pub(super) fn create_project(
    runtime: &mut Runtime,
    name: &str,
    label: &str,
    kind: PrincipalKind,
    evidence: &str,
) -> Result<(), String> {
    if runtime.config.is_some() || runtime.authority.is_some() {
        return Err(
            "This directory already has a project; creation cannot replace its authority".into(),
        );
    }
    files::bounded_text(name, 256)?;
    files::bounded_text(evidence, 4096)?;
    files::profile_name(label)?;
    // This explicit user operation creates a bounded local project root only.
    // Human capacity is the operator's local assertion, not verified legal identity.
    let path = runtime.files.profile_path(label, true)?;
    let profile = if path.join("profile.json").exists() {
        runtime.files.profile(label)?
    } else {
        runtime.create_profile(label, kind)?
    };
    if profile.requested_kind != kind {
        return Err(
            "Selected profile's declared capacity differs; choose its recorded capacity".into(),
        );
    }
    let (_, vault) = runtime.unlock(label)?;
    let controller = NativeSigner::from_vault(&vault, files::CONTROLLER_LABEL)?;
    let authority_signer = NativeSigner::from_vault(&vault, files::AUTHORITY_LABEL)?;
    let now = runtime::actions::now()?;
    let until = now
        .checked_add(365 * 86400)
        .ok_or("Project authority validity overflow")?;
    let project=Project::new(name,"Explicit locally created native cooperative project; no external authority or legal identity adoption inferred",Vec::new(),now.try_into().map_err(|_|"Project time exceeds canonical u32 range")?);
    let public = PublicProject {
        version: 1,
        name: name.into(),
        project_id: project.id.clone(),
        document_id: format!("urn:qualia:native-document:{}", project.id),
        source_id: format!("urn:qualia:native-source:{}", project.id),
        domain: "Geometry".into(),
        issuer_identifier: format!(
            "urn:qualia:native-authority:{}",
            files::hex_key(&authority_signer.public_key())
        ),
        authority_public_key: authority_signer.public_key(),
        filename: "working.hmc".into(),
    };
    let steward = runtime::actions::candidate(
        &profile,
        &project.id,
        &public.issuer_identifier,
        ProjectRole::Steward,
        evidence,
        now,
        until,
    )?;
    let admission = authority_signer.sign_admission(AdmissionSnapshot {
        version: 1,
        sequence: 1,
        issuer_identifier: public.issuer_identifier.clone(),
        project,
        valid_from_unix: now,
        valid_until_unix: until,
        revoked: false,
        subjects: vec![steward],
    })?;
    runtime.files.directory("packages", true)?;
    let journal = runtime.files.directory("authority", true)?;
    let artifact = StudioArtifact::new(&public.source_id, "Geometry", name);
    let document = DocumentRevision {
        project_id: public.project_id.clone(),
        document_id: public.document_id.clone(),
        source_id: public.source_id.clone(),
        revision: artifact.lamport_clock.into(),
        source_digest: source_digest(&artifact, GatewayLimits::default())?,
        author_subject: Some(files::subject(&controller.public_key())),
    };
    studio_store::create_artifact_if_absent(artifact)?;
    let owner = Arc::new(NativeAuthority::open(
        &journal,
        public.anchor(),
        admission,
        document,
        4 * 1024 * 1024,
        4096,
    )?);
    let challenge = owner.begin_session(&files::subject(&controller.public_key()), 3600)?;
    let session =
        owner.complete_session(challenge.clone(), &controller.prove_session(&challenge)?)?;
    let project = Arc::new(NativeProject::open(
        project_config(&runtime.files, &public)?,
        owner.clone(),
        session.clone(),
        None,
        preview(),
    )?);
    let mut host = project.connect(session.clone())?;
    let snapshot = host.snapshot()?;
    // This writes a genuine empty editable working HMC through actual native
    // session authority and records its digest audit. It is not a master fixture.
    host.save_checkpoint(&public.filename, ExpectedFile::Absent, snapshot.source)?;
    runtime.files.save_project(&public)?;
    runtime.config = Some(public.clone());
    runtime.authority = Some(owner);
    runtime.project = Some(project);
    runtime.adopted = true;
    runtime.active_filename = Some(public.filename);
    *runtime
        .sessions
        .lock()
        .map_err(|_| "Native session slot unavailable")? = Some(session);
    runtime.current_profile = Some(label.into());
    runtime.start_studio(label)
}
