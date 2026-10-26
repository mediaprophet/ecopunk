use super::*;

impl EditorHost for AuthorizedEditorHost {
    fn resource_edit_preflight(
        &self,
        name: &str,
        file: studio_store::checkpoint::ExpectedFile,
        expected: SourceRef,
    ) -> Result<(), String> {
        with_authorized(
            self.owner.as_ref(),
            &self.scope,
            Some(&expected),
            AuthoringAction::SaveCheckpoint,
            self.limits,
            |_, _| {
                self.inner
                    .resource_edit_preflight(name, file, expected.clone())
            },
        )
    }
    fn resource_import_supported(&self) -> bool {
        self.inner.resource_import_supported()
    }
    fn package_inventory(
        &self,
        name: &str,
        expected: SourceRef,
    ) -> Result<crate::graphical::checkpoints::PackageInventory, String> {
        with_authorized(
            self.owner.as_ref(),
            &self.scope,
            Some(&expected),
            AuthoringAction::ReadSource,
            self.limits,
            |_, _| self.inner.package_inventory(name, expected.clone()),
        )
    }
    fn edit_package_resources(
        &mut self,
        name: &str,
        expected_file: studio_store::checkpoint::ExpectedFile,
        edits: Vec<studio_store::checkpoint::ResourceEdit>,
        expected: SourceRef,
    ) -> Result<studio_store::checkpoint::SaveReceipt, String> {
        self.write_package(name, expected_file, &expected, |inner| {
            inner.edit_package_resources(name, expected_file, edits, expected.clone())
        })
    }
    fn import_package_resource(
        &mut self,
        name: &str,
        expected_file: studio_store::checkpoint::ExpectedFile,
        request: crate::graphical::package::ImportRequest,
        expected: SourceRef,
    ) -> Result<studio_store::checkpoint::SaveReceipt, String> {
        self.write_package(name, expected_file, &expected, |inner| {
            inner.import_package_resource(name, expected_file, request, expected.clone())
        })
    }
    fn snapshot(&self) -> Result<EditorSnapshot, String> {
        self.read(|inner, generation| {
            let mut snapshot = inner.snapshot()?;
            snapshot.authorisation_stamp = Some(AuthorisationStamp {
                owner: self.owner.clone(),
                generation,
            });
            Ok(snapshot)
        })
    }
    fn check(&self, source: &str) -> Result<String, String> {
        self.read(|inner, _| inner.check(source))
    }
    fn source_cells(&self, expected: SourceRef) -> Result<Vec<String>, String> {
        with_authorized(
            self.owner.as_ref(),
            &self.scope,
            Some(&expected),
            AuthoringAction::ReadSource,
            self.limits,
            |_, _| self.inner.source_cells(expected.clone()),
        )
    }
    fn checkpoint_supported(&self) -> bool {
        self.inner.checkpoint_supported()
    }
    fn submit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.write(&expected, AuthoringAction::ExecuteSource, |inner| {
            inner.submit(source, expected.clone())
        })
    }
    fn material_edit(
        &mut self,
        source: &str,
        expected: SourceRef,
    ) -> Result<EditorOutcome, String> {
        self.write(&expected, AuthoringAction::MaterialEdit, |inner| {
            inner.material_edit(source, expected.clone())
        })
    }
    fn undo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.write(&expected, AuthoringAction::Undo, |inner| {
            inner.undo(expected.clone())
        })
    }
    fn redo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.write(&expected, AuthoringAction::Redo, |inner| {
            inner.redo(expected.clone())
        })
    }
    fn rebuild_execution(&mut self, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.write(&expected, AuthoringAction::Rebuild, |inner| {
            inner.rebuild_execution(expected.clone())
        })
    }
    fn save_checkpoint(
        &mut self,
        name: &str,
        expected_file: studio_store::checkpoint::ExpectedFile,
        expected: SourceRef,
    ) -> Result<studio_store::checkpoint::SaveReceipt, String> {
        self.write_package(name, expected_file, &expected, |inner| {
            inner.save_checkpoint(name, expected_file, expected.clone())
        })
    }
}
