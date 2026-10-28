//! Bounded native workbook resource operations over the existing HMC owner.
//! The caller must retain the current cooperative authority guard.
use super::*;
use crate::graphical::{cooperation::{source_digest, GatewayLimits}, preview::SourceRef, workbook_p0};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkbookCreateRequest {
    pub filename: String,
    pub package_digest: [u8; 32],
    pub key: String,
}
impl WorkbookCreateRequest {
    pub fn validate_bounds(&self) -> Result<(), String> {
        if self.filename.is_empty() || self.filename.len() > 128 || self.filename.contains('\0')
            || self.key.is_empty() || self.key.len() > 4096 || self.key.contains('\0') {
            return Err("Workbook creation request exceeds native filename/key bounds".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkbookEditRequest {
    pub resource: ResourceInspectionRequest,
    pub sheet: String,
    pub cell: String,
    pub input: String,
}
impl WorkbookEditRequest {
    pub fn validate_bounds(&self) -> Result<(), String> {
        self.resource.validate_bounds()?;
        // Engine performs canonical coordinate and exact input validation.
        if self.sheet.len() > 128 || self.cell.len() > 32 || self.input.len() > 256 {
            return Err("Workbook request exceeds native input bounds".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct WorkbookInspection {
    pub request: ResourceInspectionRequest,
    pub saved_source_revision: u32,
    pub view: workbook_p0::WorkbookView,
}

fn live(cancelled: &AtomicBool) -> Result<(), String> {
    if cancelled.load(Ordering::Acquire) { Err("Workbook operation cancelled before package save".into()) } else { Ok(()) }
}

impl CheckpointService {
    fn workbook_source_digest(&self, expected: &SourceRef) -> Result<[u8; 32], String> {
        expected.check_current()?;
        if expected.artifact_id != self.binding.source_id { return Err("Workbook source association differs from native document".into()); }
        let (artifact, stamp) = studio_store::get_artifact_stamped(&expected.artifact_id).ok_or("Workbook source is absent")?;
        if stamp != expected.stamp { return Err("Workbook source stamp changed".into()); }
        source_digest(&artifact, GatewayLimits::default())
    }

    // The exact HMC is checked before any profile decoding or mutation. Bytes
    // and opaque metadata from unrelated resources stay in the source carrier.
    fn workbook_package(&self, filename: &str, digest: [u8; 32], expected: &SourceRef) -> Result<core::Checkpoint, String> {
        let current = self.workbook_source_digest(expected)?;
        let checkpoint = core::load(&self.path(filename)?, self.limits)?;
        self.matches(checkpoint.artifact())?;
        self.check_release_input(filename, digest)?;
        if checkpoint.file_digest() != Some(digest) { return Err("Workbook package changed; refresh inventory".into()); }
        if source_digest(checkpoint.artifact(), GatewayLimits::default())? != current {
            return Err("Workbook package source differs from current source; save or reopen the coherent base".into());
        }
        Ok(checkpoint)
    }

    fn workbook_selected(&self, request: &ResourceInspectionRequest, expected: &SourceRef, cancelled: &AtomicBool) -> Result<(Vec<u8>, u32), String> {
        live(cancelled)?;
        request.validate_bounds()?;
        let checkpoint = self.workbook_package(&request.filename, request.package_digest, expected)?;
        let resource = checkpoint.resources().iter().find(|r| r.key == request.key).ok_or("Workbook resource is absent")?;
        if resource.kind != workbook_p0::RESOURCE_KIND || resource.summary().digest != request.resource_digest {
            return Err("Workbook resource kind or digest changed; refresh inventory".into());
        }
        if resource.meta.is_some() { return Err("Workbook metadata is not qualified for editing or inspection".into()); }
        if resource.bytes.len() > 128 * 1024 || resource.bytes.len() > self.limits.file_bytes {
            return Err("Workbook resource exceeds the bounded native profile".into());
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(resource.bytes.len()).map_err(|_| "Workbook allocation refused")?;
        bytes.extend_from_slice(&resource.bytes);
        let revision = checkpoint.artifact().lamport_clock;
        drop(checkpoint);
        live(cancelled)?;
        // Recheck disk and current source after copying before exposing a result.
        self.workbook_package(&request.filename, request.package_digest, expected)?;
        Ok((bytes, revision))
    }

    /// Outer owner retains ReadSource. A profile is not a grant or full workbook acceptance.
    pub fn inspect_workbook(&self, request: ResourceInspectionRequest, expected: &SourceRef, cancelled: &AtomicBool) -> Result<WorkbookInspection, String> {
        let (bytes, saved_source_revision) = self.workbook_selected(&request, expected, cancelled)?;
        let view = workbook_p0::inspect(&bytes, cancelled)?;
        live(cancelled)?;
        self.workbook_package(&request.filename, request.package_digest, expected)?;
        Ok(WorkbookInspection { request, saved_source_revision, view })
    }

    /// Outer owner retains WritePackage. Creates only the explicitly selected,
    /// bounded new-native profile in an existing HMC; imports are not converted.
    pub fn create_workbook(&self, request: WorkbookCreateRequest, expected: &SourceRef, cancelled: &AtomicBool) -> Result<SaveReceipt, String> {
        request.validate_bounds()?;
        live(cancelled)?;
        let expected_file = ExpectedFile::Sha256(request.package_digest);
        self.preflight_resource_edit(&request.filename, expected_file)?;
        let checkpoint = self.workbook_package(&request.filename, request.package_digest, expected)?;
        if checkpoint.resources().iter().any(|r| r.key == request.key) { return Err("Workbook resource key already exists; no replacement".into()); }
        drop(checkpoint);
        let bytes = workbook_p0::blank();
        live(cancelled)?;
        expected.check_current()?;
        // The underlying atomic checkpoint save has no cancellation callback:
        // cancellation observed after this call begins cannot promise rollback.
        self.save_with_resource_edits(&expected.stamp, &request.filename, expected_file,
            Some(request.package_digest), vec![], vec![ResourceEdit::Add(ResourceData {
                key: request.key, kind: workbook_p0::RESOURCE_KIND.into(), bytes, meta: None,
            })]).map(|saved| saved.receipt)
    }

    /// Outer owner retains WritePackage through the single selected resource edit.
    pub fn edit_workbook(&self, request: WorkbookEditRequest, expected: &SourceRef, cancelled: &AtomicBool) -> Result<SaveReceipt, String> {
        request.validate_bounds()?;
        live(cancelled)?;
        let selected = &request.resource;
        let expected_file = ExpectedFile::Sha256(selected.package_digest);
        self.preflight_resource_edit(&selected.filename, expected_file)?;
        let (original, _) = self.workbook_selected(selected, expected, cancelled)?;
        let bytes = workbook_p0::edit(&original, &request.sheet, &request.cell, &request.input, cancelled)?;
        live(cancelled)?;
        expected.check_current()?;
        self.save_with_resource_edits(&expected.stamp, &selected.filename, expected_file,
            Some(selected.package_digest), vec![], vec![ResourceEdit::ReplaceBytes {
                key: selected.key.clone(), expected: selected.resource_digest, bytes,
            }]).map(|saved| saved.receipt)
    }
}
