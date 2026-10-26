//! Per-controller unsubmitted text. Never part of admission or shared source.
use std::sync::{Arc, Mutex};
#[derive(Clone)]
pub struct DraftBuffer(Arc<Mutex<String>>);
impl DraftBuffer {
    pub fn new(text: String) -> Self {
        Self(Arc::new(Mutex::new(text)))
    }
    pub fn read(&self) -> Result<String, String> {
        self.0
            .lock()
            .map(|s| s.clone())
            .map_err(|_| "Local draft buffer unavailable".into())
    }
    pub fn set(&self, text: String) {
        if let Ok(mut value) = self.0.lock() {
            *value = text;
        }
    }
}
impl PartialEq for DraftBuffer {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Explicit private-draft replacement only. Neither accepted source nor review
/// authority changes. Revalidate the exact displayed source/session at the click.
pub(crate) fn accepted_source_for_draft(
    current: &super::controller::EditorSnapshot,
    confirmed: bool,
) -> Result<String, String> {
    if !confirmed {
        return Err(
            "Confirm replacement of the local draft with the displayed accepted source".into(),
        );
    }
    if !current.publication_is_current() {
        return Err(
            "Accepted source or native session changed; refresh before replacing the draft".into(),
        );
    }
    Ok(current.latest_source.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphical::{
        controller::{CanonicalEditorHost, EditorHost, PreviewProvider},
        geometry_host::GeometryHost,
        preview::{PreviewFrame, SourceRef},
    };
    use qualia_core_db::poet_host::invoke::studio_store::{self, StudioArtifact};
    struct NoPixels;
    impl PreviewProvider for NoPixels {
        fn preview(&mut self, _: SourceRef, _: &vibe::Value) -> Result<PreviewFrame, String> {
            Err("headless draft-only fixture".into())
        }
    }
    #[test]
    fn returning_reviewer_explicitly_adopts_current_source_without_changing_shared_state() {
        let id = "poet-private-draft-accepted-replacement";
        studio_store::create_artifact_if_absent(StudioArtifact::new(
            id,
            "Geometry",
            "Draft recovery",
        ))
        .unwrap();
        let mut host = CanonicalEditorHost::open(
            id.into(),
            vibe::session::SessionLimits::default(),
            GeometryHost::default(),
            Box::new(NoPixels),
        )
        .unwrap();
        let initial = host.snapshot().unwrap();
        let accepted = host.submit("1;", initial.source).unwrap().snapshot;
        let private = DraftBuffer::new("previous round or unsubmitted local work".into());
        assert!(accepted_source_for_draft(&accepted, false).is_err());
        assert_eq!(
            private.read().unwrap(),
            "previous round or unsubmitted local work"
        );
        let chosen = accepted_source_for_draft(&accepted, true).unwrap();
        private.set(chosen.clone());
        assert_eq!(chosen, "1;");
        assert_eq!(host.snapshot().unwrap().source, accepted.source);
        studio_store::upsert_artifact(studio_store::get_artifact(id).unwrap());
        assert!(accepted_source_for_draft(&accepted, true).is_err());
        assert_eq!(private.read().unwrap(), chosen);
    }
}
