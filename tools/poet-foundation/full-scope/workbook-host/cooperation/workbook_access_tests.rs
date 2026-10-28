//! Software-agent test owner, not a human review or admission claim.
use super::*;
use crate::graphical::{
    checkpoints::{CheckpointEditorHost, CheckpointService, ExpectedFile, ResourceInspectionRequest, TrustedBinding, WorkbookCreateRequest, WorkbookEditRequest},
    shared_host::SharedEditorHost,
};
use std::sync::atomic::AtomicBool;

#[test]
fn workbook_cooperative_owner_allows_read_but_denies_observer_edit_and_revocation() {
    let root = tempfile::tempdir().unwrap();
    let scope = scope();
    let source = reference(&scope);
    let binding = TrustedBinding { project_id: scope.project_id.clone(), document_id: scope.document_id.clone(), source_id: scope.source_id.clone(), domain: scope.domain.clone() };
    let service = CheckpointService::new(root.path(), binding, Default::default()).unwrap();
    let saved = service.save(&source.stamp, "working.hmc", ExpectedFile::Absent, None, vec![]).unwrap();
    let canonical = CanonicalEditorHost::open(scope.source_id.clone(), SessionLimits::default(), GeometryHost::default(), Box::new(NoPreview)).unwrap();
    let checkpoint = CheckpointEditorHost::new(Box::new(canonical), service).unwrap();
    let shared = SharedEditorHost::new(Box::new(checkpoint));
    let contributor = Arc::new(FixtureOwner::new(&scope, ProjectRole::Contributor));
    let observer = Arc::new(FixtureOwner::new(&scope, ProjectRole::Observer));
    let mut author = AuthorizedEditorHost::new(Box::new(shared.clone()), scope.clone(), contributor.clone(), GatewayLimits::default()).unwrap();
    let mut reader = AuthorizedEditorHost::new(Box::new(shared), scope.clone(), observer.clone(), GatewayLimits::default()).unwrap();
    let cancel = AtomicBool::new(false);
    let created = author.create_workbook(WorkbookCreateRequest { filename: "working.hmc".into(), package_digest: saved.receipt.digest, key: "workbook/native".into() }, source.clone(), &cancel).unwrap();
    let resources = author.package_inventory("working.hmc", source.clone()).unwrap();
    let workbook = resources.resources.iter().find(|r| r.key == "workbook/native").unwrap();
    let selected = ResourceInspectionRequest { filename: "working.hmc".into(), package_digest: created.digest, key: workbook.key.clone(), resource_digest: workbook.digest };
    assert!(reader.inspect_workbook(selected.clone(), source.clone(), &cancel).unwrap().view.cells.is_empty());
    let before = std::fs::read(root.path().join("working.hmc")).unwrap();
    let edit = WorkbookEditRequest { resource: selected.clone(), sheet: "Sheet1".into(), cell: "A1".into(), input: "42".into() };
    assert!(reader.edit_workbook(edit.clone(), source.clone(), &cancel).is_err());
    assert_eq!(std::fs::read(root.path().join("working.hmc")).unwrap(), before);
    observer.state.lock().unwrap().revoked = true;
    assert!(reader.inspect_workbook(selected.clone(), source.clone(), &cancel).is_err());
    let saved = author.edit_workbook(edit, source.clone(), &cancel).unwrap();
    assert_ne!(saved.digest, created.digest);
    contributor.state.lock().unwrap().revoked = true;
    assert!(author.inspect_workbook(selected, source, &cancel).is_err());
}
