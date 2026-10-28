//! Native HMC workbook profile fixture; not a human workflow or XLSX claim.
use super::*;
use crate::graphical::preview::SourceRef;
use std::sync::atomic::AtomicBool;

fn fixture(root: &std::path::Path, id: &str) -> (CheckpointService, SourceRef, SaveReceipt) {
    let (_, stamp) = studio_store::create_artifact_if_absent(StudioArtifact::new(id, "Geometry", "Workbook fixture")).unwrap();
    let binding = TrustedBinding { project_id: "project".into(), document_id: id.into(), source_id: id.into(), domain: "Geometry".into() };
    let service = CheckpointService::new(root, binding, Default::default()).unwrap();
    let saved = service.save(&stamp, "working.hmc", ExpectedFile::Absent, None, vec![]).unwrap();
    let saved = service.save_with_resource_edits(&stamp, "working.hmc", ExpectedFile::Sha256(saved.receipt.digest),
        Some(saved.receipt.digest), vec![], vec![ResourceEdit::Add(ResourceData {
            key: "opaque/original".into(), kind: "future/source".into(), bytes: vec![1, 2, 3], meta: Some(vec![0xa1, 0x61, b'x', 1]),
        })]).unwrap();
    (service, SourceRef { artifact_id: id.into(), stamp }, saved.receipt)
}

#[test]
fn workbook_hmc_create_edit_reopen_and_refusals_preserve_other_resources() {
    let root = tempfile::tempdir().unwrap();
    let id = format!("urn:poet:workbook-host:{}", std::process::id());
    let (service, source, saved) = fixture(root.path(), &id);
    let cancel = AtomicBool::new(false);
    let created = service.create_workbook(WorkbookCreateRequest {
        filename: "working.hmc".into(), package_digest: saved.digest, key: "workbook/native".into(),
    }, &source, &cancel).unwrap();
    let inventory = service.inventory("working.hmc").unwrap();
    let resource = inventory.resources.iter().find(|r| r.key == "workbook/native").unwrap();
    assert_eq!(resource.kind, crate::graphical::workbook_p0::RESOURCE_KIND);
    let selected = ResourceInspectionRequest { filename: "working.hmc".into(), package_digest: created.digest,
        key: resource.key.clone(), resource_digest: resource.digest };
    let before = std::fs::read(root.path().join("working.hmc")).unwrap();
    assert!(service.edit_workbook(WorkbookEditRequest { resource: selected.clone(), sheet: "Sheet1".into(), cell: "A1".into(), input: "=1+1".into() }, &source, &cancel).is_err());
    assert_eq!(std::fs::read(root.path().join("working.hmc")).unwrap(), before);
    let cancelled = AtomicBool::new(true);
    assert!(service.edit_workbook(WorkbookEditRequest { resource: selected.clone(), sheet: "Sheet1".into(), cell: "A1".into(), input: "42".into() }, &source, &cancelled).is_err());
    let mut stale = selected.clone(); stale.resource_digest = [7; 32];
    assert!(service.edit_workbook(WorkbookEditRequest { resource: stale, sheet: "Sheet1".into(), cell: "A1".into(), input: "42".into() }, &source, &cancel).is_err());
    assert_eq!(std::fs::read(root.path().join("working.hmc")).unwrap(), before);
    let edited = service.edit_workbook(WorkbookEditRequest { resource: selected, sheet: "Sheet1".into(), cell: "A1".into(), input: "42".into() }, &source, &cancel).unwrap();
    let inventory = service.inventory("working.hmc").unwrap();
    let resource = inventory.resources.iter().find(|r| r.key == "workbook/native").unwrap();
    let selected = ResourceInspectionRequest { filename: "working.hmc".into(), package_digest: edited.digest,
        key: resource.key.clone(), resource_digest: resource.digest };
    let reopened = CheckpointService::new(root.path(), service.binding().clone(), Default::default()).unwrap();
    let view = reopened.inspect_workbook(selected, &source, &cancel).unwrap();
    assert_eq!(view.view.sheet, "Sheet1");
    assert_eq!(view.view.cells[0].cell, "A1");
    assert_eq!(view.view.cells[0].input, "42");
    let decoded = core::load(&root.path().join("working.hmc"), Default::default()).unwrap();
    let original = decoded.resources().iter().find(|r| r.key == "opaque/original").unwrap();
    assert_eq!(original.kind, "future/source");
    assert_eq!(original.bytes, vec![1, 2, 3]);
    assert_eq!(original.meta, Some(vec![0xa1, 0x61, b'x', 1]));
}
