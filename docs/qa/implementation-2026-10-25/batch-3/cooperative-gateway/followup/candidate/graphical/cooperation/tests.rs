//! Explicit fixture authority only. These are not production credentials.
use super::*;
use crate::graphical::{
    controller::{CanonicalEditorHost, PreviewProvider},
    geometry_host::GeometryHost,
    preview::PreviewFrame,
};
use qualia_cooperative_core::{
    authoring::*,
    projects::{Project, ProjectMembership, ProjectRole},
};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};
use vibe::session::SessionLimits;

#[path = "mcp_tests.rs"]
mod mcp_tests;

static NEXT: AtomicU64 = AtomicU64::new(1);
fn scope() -> Scope {
    let serial = NEXT.fetch_add(1, Ordering::Relaxed);
    let id = format!(
        "urn:poet:cooperation-fixture:{}:{serial}",
        std::process::id()
    );
    studio_store::create_artifact_if_absent(StudioArtifact::new(&id, "Geometry", "fixture source"))
        .unwrap();
    Scope {
        project_id: "fixture-project".into(),
        document_id: format!("fixture-document-{serial}"),
        source_id: id,
        domain: "Geometry".into(),
    }
}
fn document(scope: &Scope) -> DocumentRevision {
    let (source, _) = studio_store::get_artifact_stamped(&scope.source_id).unwrap();
    DocumentRevision {
        project_id: scope.project_id.clone(),
        document_id: scope.document_id.clone(),
        source_id: scope.source_id.clone(),
        revision: source.lamport_clock.into(),
        source_digest: source_digest(&source, GatewayLimits::default()).unwrap(),
        author_subject: Some("fixture-subject".into()),
    }
}
struct FixtureState {
    role: ProjectRole,
    document: DocumentRevision,
    generation: u64,
    revoked: bool,
    calls: usize,
}
struct FixtureOwner {
    scope: Scope,
    state: Mutex<FixtureState>,
}
impl FixtureOwner {
    fn new(scope: &Scope, role: ProjectRole) -> Self {
        Self {
            scope: scope.clone(),
            state: Mutex::new(FixtureState {
                role,
                document: document(scope),
                generation: 1,
                revoked: false,
                calls: 1,
            }),
        }
    }
    fn refresh_document(&self) {
        self.state.lock().unwrap().document = document(&self.scope);
    }
}
impl AuthorityOwner for FixtureOwner {
    fn with_current(
        &self,
        operation: &mut dyn FnMut(&HostAuthorityView<'_>) -> Result<(), String>,
    ) -> Result<(), String> {
        let state = self.state.lock().map_err(|_| "fixture poisoned")?;
        let project = Project {
            id: self.scope.project_id.clone(),
            name: "fixture".into(),
            description: String::new(),
            licensing_ontologies: vec![],
            created_at_unix: 1,
            created_at_instant: None,
        };
        let member = ProjectMembership {
            id: "fixture-member".into(),
            project_id: self.scope.project_id.clone(),
            member_did: "did:test:contributor".into(),
            role: state.role.clone(),
            agreed_at_unix: 1,
            agreed_at_instant: None,
        };
        let ids = [QualifiedIdentifier {
            value: "did:test:contributor",
            scheme: "did:test",
            issuer: "fixture-issuer",
            project_id: &self.scope.project_id,
            evidence_ref: "fixture-association",
            valid_from_unix: 1,
            valid_until_unix: 1000,
            revoked: false,
        }];
        let principal = AuthenticatedPrincipal {
            subject: "fixture-subject",
            kind: PrincipalKind::SoftwareAgent,
            authentication_ref: "fixture-only",
            authenticated_until_unix: 1000,
            identifiers: &ids,
        };
        let members = [AdmittedMembership {
            membership: &member,
            member_subject: "fixture-subject",
            admission_ref: "fixture-admission",
            valid_from_unix: 1,
            valid_until_unix: 1000,
            revoked: state.revoked,
        }];
        let view = HostAuthorityView {
            principal: Some(&principal),
            project: &project,
            memberships: &members,
            review_grants: &[],
            current_document: &state.document,
            generation: state.generation,
            now_unix: 100,
            limits: AuthoringLimits::default(),
        };
        for _ in 0..state.calls {
            operation(&view)?;
        }
        Ok(())
    }
}
struct NoPreview;
impl PreviewProvider for NoPreview {
    fn preview(&mut self, _: SourceRef, _: &vibe::Value) -> Result<PreviewFrame, String> {
        Err("fixture does not create fake rendered frames".into())
    }
}
fn editor(scope: Scope, owner: Arc<dyn AuthorityOwner>) -> AuthorizedEditorHost {
    let inner = CanonicalEditorHost::open(
        scope.source_id.clone(),
        SessionLimits::default(),
        GeometryHost::default(),
        Box::new(NoPreview),
    )
    .unwrap();
    AuthorizedEditorHost::new(Box::new(inner), scope, owner, GatewayLimits::default()).unwrap()
}
fn reference(scope: &Scope) -> SourceRef {
    SourceRef {
        artifact_id: scope.source_id.clone(),
        stamp: studio_store::get_artifact_stamped(&scope.source_id)
            .unwrap()
            .1,
    }
}

#[test]
fn unbound_native_owner_denies_reads_checks_and_writes_without_source_mutation() {
    let scope = scope();
    let original = reference(&scope);
    let mut editor = editor(scope.clone(), Arc::new(UnboundAuthorityOwner));
    assert!(editor.snapshot().is_err());
    assert!(editor.check("1;").is_err());
    assert!(editor.submit("let x = 1;", original.clone()).is_err());
    assert_eq!(reference(&scope), original);
}

#[test]
fn observer_reads_actual_source_but_every_edit_route_denies() {
    let scope = scope();
    let owner = Arc::new(FixtureOwner::new(&scope, ProjectRole::Observer));
    let mut editor = editor(scope.clone(), owner);
    let snapshot = editor.snapshot().unwrap();
    assert_eq!(snapshot.source, reference(&scope));
    assert!(editor.check("1;").is_ok());
    assert!(editor
        .submit("let x = 1;", snapshot.source.clone())
        .is_err());
    assert!(editor
        .material_edit("let x = 1;", snapshot.source.clone())
        .is_err());
    assert!(editor.undo(snapshot.source.clone()).is_err());
    assert!(editor.redo(snapshot.source.clone()).is_err());
    assert!(editor.rebuild_execution(snapshot.source.clone()).is_err());
    assert!(editor
        .save_checkpoint(
            "never-written.hmc",
            studio_store::checkpoint::ExpectedFile::Absent,
            snapshot.source.clone()
        )
        .is_err());
    assert!(studio_store::get_artifact_stamped(&scope.source_id)
        .unwrap()
        .0
        .nodes
        .is_empty());
}

#[test]
fn real_canonical_write_requires_native_owner_document_update_before_next_action() {
    let scope = scope();
    let owner = Arc::new(FixtureOwner::new(&scope, ProjectRole::Contributor));
    let mut editor = editor(scope.clone(), owner.clone());
    let before = editor.snapshot().unwrap();
    let accepted = editor
        .submit("let mut fixture_value: I64 = 1;", before.source)
        .unwrap();
    assert_eq!(accepted.snapshot.session_revision, 1);
    assert!(accepted.frame.is_none());
    assert!(
        editor.snapshot().is_err(),
        "stale native document must not follow a source write implicitly"
    );
    owner.refresh_document();
    assert_eq!(editor.snapshot().unwrap().source, accepted.snapshot.source);
    let sources = editor.source_cells(accepted.snapshot.source).unwrap();
    assert_eq!(sources, vec!["let mut fixture_value: I64 = 1;"]);
}

#[test]
fn foreign_scope_stamp_source_digest_and_current_generation_fail_closed() {
    let scope = scope();
    let foreign = super::tests::scope();
    let owner = Arc::new(FixtureOwner::new(&scope, ProjectRole::Contributor));
    let mut editor = editor(scope.clone(), owner.clone());
    assert!(editor.submit("let x = 1;", reference(&foreign)).is_err());
    owner.state.lock().unwrap().document.source_digest = [9; 32];
    assert!(editor.snapshot().is_err());
    owner.refresh_document();
    owner.state.lock().unwrap().document.document_id = "different-document".into();
    assert!(editor.snapshot().is_err());
    owner.refresh_document();
    owner.state.lock().unwrap().generation = 0;
    assert!(editor.snapshot().is_err());
    owner.state.lock().unwrap().generation = 2;
    owner.state.lock().unwrap().revoked = true;
    assert!(editor.snapshot().is_err());
}

#[test]
fn provider_never_calls_or_calls_twice_never_repeats_underlying_operation() {
    let scope = scope();
    let owner = FixtureOwner::new(&scope, ProjectRole::Contributor);
    let mut actual_calls = 0;
    owner.state.lock().unwrap().calls = 0;
    assert!(with_authorized(
        &owner,
        &scope,
        None,
        AuthoringAction::ReadSource,
        GatewayLimits::default(),
        |_, _| {
            actual_calls += 1;
            Ok(())
        }
    )
    .is_err());
    assert_eq!(actual_calls, 0);
    owner.state.lock().unwrap().calls = 2;
    assert!(with_authorized(
        &owner,
        &scope,
        None,
        AuthoringAction::ReadSource,
        GatewayLimits::default(),
        |_, _| {
            actual_calls += 1;
            Ok(())
        }
    )
    .is_err());
    assert_eq!(actual_calls, 1);
}

#[test]
fn self_review_and_unbounded_source_digest_are_refused() {
    let scope = scope();
    let owner = FixtureOwner::new(&scope, ProjectRole::Contributor);
    let error = with_authorized(
        &owner,
        &scope,
        None,
        AuthoringAction::ReviewDecision,
        GatewayLimits::default(),
        |_, _| Ok(()),
    )
    .unwrap_err();
    assert!(error.contains("SelfReview"));
    let (artifact, _) = studio_store::get_artifact_stamped(&scope.source_id).unwrap();
    assert!(source_digest(
        &artifact,
        GatewayLimits {
            max_digest_bytes: 8,
            ..GatewayLimits::default()
        }
    )
    .is_err());
    let mut bad = artifact;
    bad.nodes.insert(
        "poet.native.submission/00000000000000000002".into(),
        std::collections::BTreeMap::from([("text".into(), "1;".into())]),
    );
    assert!(source_digest(&bad, GatewayLimits::default()).is_err());
}
