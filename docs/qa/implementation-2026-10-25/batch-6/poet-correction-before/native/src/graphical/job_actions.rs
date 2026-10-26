use super::*;
pub(super) fn perform(
    host: &mut dyn EditorHost,
    action: Action,
    inventory: &mut Option<crate::graphical::checkpoints::PackageInventory>,
    workflow: &mut Option<crate::graphical::cooperative_workflow::WorkflowView>,
    authoring: &mut Option<crate::graphical::authoring::AuthoringView>,
) -> Result<(String, Option<BoundImage>), String> {
    let image_uri = None;
    let message = match action {
        Action::AuthoringInspect(expected) => {
            *authoring = Some(host.authoring_view(expected)?);
            "Current accepted assembly inspected".into()
        }
        Action::AuthoringEdit { edit, expected } => {
            let result = host.authoring_edit(edit, expected)?;
            *authoring = host.authoring_view(result.snapshot.source.clone()).ok();
            return outcome_image(result);
        }
        Action::CompileAssembly {
            name,
            key,
            file,
            expected,
        } => {
            let receipt = host.compile_checkpoint(&name, &key, file, expected.clone())?;
            *inventory = host.package_inventory(&name, expected).ok();
            format!("Accepted assembly compiled into {name}; SHA256 {}. Complete model/package acceptance remains separate.",crate::graphical::package::hex_digest(&receipt.digest))+&crate::graphical::package::durability_note(&receipt)
        }
        Action::Workflow { action, .. } => {
            *workflow = Some(host.workflow(action)?);
            "Current source contribution/review evidence updated. Package and master acceptance are separate.".into()
        }
        Action::SaveInspected {
            name,
            file_digest,
            expected,
        } => {
            let receipt = host.save_checkpoint(
                &name,
                qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Sha256(
                    file_digest,
                ),
                expected.clone(),
            )?;
            *inventory = host.package_inventory(&name, expected).ok();
            format!("Accepted source and complete retained resource inventory saved in {name}; SHA256 {}",crate::graphical::package::hex_digest(&receipt.digest))+&crate::graphical::package::durability_note(&receipt)
        }
        Action::ResourcePlan {
            name,
            file_digest,
            plan,
            expected,
        } => {
            let receipt = host.apply_resource_plan(
                &name,
                qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Sha256(
                    file_digest,
                ),
                plan,
                expected.clone(),
            )?;
            *inventory = host.package_inventory(&name, expected).ok();
            format!(
                "Atomic resource plan saved in {name}; package SHA256 {}",
                crate::graphical::package::hex_digest(&receipt.digest)
            ) + &crate::graphical::package::durability_note(&receipt)
        }
        Action::Inventory { name, expected } => {
            let result = host.package_inventory(&name, expected)?;
            let message = format!(
                "{} preserved resources in {name}; package SHA256 {}",
                result.resources.len(),
                crate::graphical::package::hex_digest(&result.digest)
            );
            *inventory = Some(result);
            message
        }
        Action::ImportResource {
            name,
            file_digest,
            request,
            expected,
        } => {
            let receipt = host.import_package_resource(
                &name,
                qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Sha256(
                    file_digest,
                ),
                request,
                expected.clone(),
            )?;
            *inventory = host.package_inventory(&name, expected).ok();
            format!("Resource contents saved in {name}; package SHA256 {}. Unedited entries retained; refresh inventory if unavailable.",crate::graphical::package::hex_digest(&receipt.digest))+&crate::graphical::package::durability_note(&receipt)
        }
        Action::RemoveResource {
            name,
            file_digest,
            key,
            resource_digest,
            expected,
        } => {
            let receipt=host.edit_package_resources(&name,
                qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Sha256(file_digest),
                vec![qualia_core_db::poet_host::invoke::studio_store::checkpoint::ResourceEdit::Remove{key,expected:resource_digest}],expected.clone())?;
            *inventory = host.package_inventory(&name, expected).ok();
            format!(
                "Explicit resource removal saved in {name}; package SHA256 {}",
                crate::graphical::package::hex_digest(&receipt.digest)
            ) + &crate::graphical::package::durability_note(&receipt)
        }
        Action::Check(source) => host.check(&source)?,
        Action::Submit { source, expected } => {
            let outcome = host.submit(&source, expected)?;
            return outcome_image(outcome);
        }
        Action::Rebuild(expected) => {
            if !host.snapshot()?.rebuild_supported {
                return Err("Execution rebuild is unsupported by this host profile".into());
            }
            return outcome_image(host.rebuild_execution(expected)?);
        }
        Action::Undo(expected) => {
            host.undo(expected)?;
            "Editable source restored; session reconciliation required".into()
        }
        Action::Redo(expected) => {
            host.redo(expected)?;
            "Editable source reapplied; session reconciliation required".into()
        }
        Action::Save { name, expected } => {
            let receipt = host.save_checkpoint(
                &name,
                qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Absent,
                expected,
            )?;
            let digest = receipt
                .digest
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>();
            format!("Accepted source/history saved as {name} ({} bytes, SHA256 {digest}); draft and human acceptance are separate",receipt.bytes)+&crate::graphical::package::durability_note(&receipt)
        }
        Action::Refresh => "Current identified source read from canonical store".into(),
    };
    Ok((message, image_uri))
}

fn outcome_image(
    outcome: crate::graphical::controller::EditorOutcome,
) -> Result<(String, Option<BoundImage>), String> {
    let mut image_uri = None;
    let mut message = outcome.message;
    if let Some(frame) = outcome.frame {
        if frame.source == outcome.snapshot.source
            && frame.session_revision == Some(outcome.snapshot.session_revision)
            && frame.execution_epoch.as_ref() == Some(&outcome.snapshot.execution_epoch)
            && outcome.snapshot.execution_epoch.is_current()
        {
            match native_image_uri(&frame) {
                Ok(uri) => {
                    image_uri = Some(BoundImage {
                        context: Context::from_snapshot(&outcome.snapshot),
                        uri,
                    })
                }
                Err(error) => message.push_str(&format!("; frame withheld: {error}")),
            }
        } else {
            message.push_str("; frame source/session mismatch; withheld");
        }
    }
    Ok((message, image_uri))
}
