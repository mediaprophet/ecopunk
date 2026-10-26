//! Bounded command decode for the original Poet native intent owner.
use super::*;
#[derive(Deserialize)]
#[serde(tag = "command", deny_unknown_fields)]
pub(super) enum Command {
    #[serde(rename = "authoring.inspect")]
    Inspect,
    #[serde(rename = "source.check")]
    Check { source: String },
    #[serde(rename = "source.submit")]
    Submit { source: String },
    #[serde(rename = "source.undo")]
    Undo,
    #[serde(rename = "source.redo")]
    Redo,
    #[serde(rename = "source.rebuild")]
    Rebuild,
    #[serde(rename = "source.refresh")]
    Refresh,
    #[serde(rename = "package.save")]
    Save { name: String },
    #[serde(rename = "authoring.transform")]
    Transform {
        id: String,
        translation: [f64; 3],
        rotation: [f64; 3],
        scale: [f64; 3],
    },
    #[serde(rename = "authoring.material")]
    Material {
        material_id: u64,
        base_color: [f64; 4],
        roughness: f64,
        metallic: f64,
    },
    #[serde(rename = "authoring.parameter")]
    Parameter { key: String, value: f64 },
    #[serde(rename = "authoring.reparent")]
    Reparent { id: String, parent: Option<String> },
    #[serde(rename = "authoring.remove_leaf")]
    RemoveLeaf { id: String },
    #[serde(rename = "authoring.replace_geometry")]
    ReplaceGeometry {
        id: String,
        geometry: crate::graphical::authoring::GeometryRecipe,
    },
    #[serde(rename = "authoring.add_geometry")]
    AddGeometry {
        id: String,
        semantic_id: u64,
        material_from: String,
        geometry: crate::graphical::authoring::GeometryRecipe,
    },
    #[serde(rename = "authoring.unique_material")]
    UniqueMaterial { id: String, material_id: u64 },
    #[serde(rename = "authoring.compile")]
    Compile {
        name: String,
        key: String,
        #[serde(default)]
        expected_sha256: Option<[u8; 32]>,
    },
}
impl Command {
    pub(super) fn action_type(&self) -> ActionType {
        match self {
            Self::Inspect | Self::Check { .. } | Self::Refresh => ActionType::Query,
            _ => ActionType::Invoke,
        }
    }
    pub(super) fn scope(&self) -> &'static str {
        match self {
            Self::Inspect => "Studio.inspect",
            Self::Check { .. } => "Vibe.check",
            Self::Submit { .. } => "Studio.submit",
            Self::Undo => "Studio.undo",
            Self::Redo => "Studio.redo",
            Self::Rebuild => "Studio.rebuild",
            Self::Refresh => "Studio.inspect",
            Self::Save { .. } => "Studio.checkpoint",
            Self::Reparent { .. }
            | Self::RemoveLeaf { .. }
            | Self::ReplaceGeometry { .. }
            | Self::AddGeometry { .. }
            | Self::UniqueMaterial { .. }
            | Self::Transform { .. }
            | Self::Material { .. }
            | Self::Parameter { .. } => "Studio.edit",
            Self::Compile { .. } => "Studio.compile",
        }
    }
    pub(super) fn action(self, origin: &Context) -> Result<Action, String> {
        let expected = origin.source.clone();
        let text = |s: &str, max| {
            if s.is_empty() || s.len() > max || s.as_bytes().contains(&0) {
                Err("Poet intent text violates native command bounds".to_string())
            } else {
                Ok(())
            }
        };
        Ok(match self {
            Self::Inspect => Action::AuthoringInspect(expected),
            Self::Check { source } => {
                text(&source, 64 * 1024)?;
                Action::Check(source)
            }
            Self::Submit { source } => {
                text(&source, 64 * 1024)?;
                Action::Submit { source, expected }
            }
            Self::Undo => Action::Undo(expected),
            Self::Redo => Action::Redo(expected),
            Self::Rebuild => Action::Rebuild(expected),
            Self::Refresh => Action::Refresh,
            Self::Save { name } => {
                text(&name, 128)?;
                Action::Save { name, expected }
            }
            Self::Transform {
                id,
                translation,
                rotation,
                scale,
            } => {
                text(&id, 4096)?;
                if translation
                    .iter()
                    .chain(&rotation)
                    .chain(&scale)
                    .any(|n| !n.is_finite())
                    || scale.iter().any(|n| *n <= 0.0)
                {
                    return Err(
                        "Poet transform requires finite coordinates and positive scale".into(),
                    );
                }
                Action::AuthoringEdit {
                    edit: AuthoringEdit::Transform {
                        id,
                        translation,
                        rotation,
                        scale,
                    },
                    expected,
                }
            }
            Self::Material {
                material_id,
                base_color,
                roughness,
                metallic,
            } => {
                if material_id == 0
                    || base_color
                        .iter()
                        .chain([&roughness, &metallic])
                        .any(|n| !n.is_finite())
                {
                    return Err(
                        "Poet material intent needs a nonzero identifier and finite factors".into(),
                    );
                }
                Action::AuthoringEdit {
                    edit: AuthoringEdit::Material {
                        material_id,
                        base_color,
                        roughness,
                        metallic,
                    },
                    expected,
                }
            }
            Self::Parameter { key, value } => {
                text(&key, 4096)?;
                if !value.is_finite() {
                    return Err("Poet parameter must be finite".into());
                }
                Action::AuthoringEdit {
                    edit: AuthoringEdit::Parameter { key, value },
                    expected,
                }
            }
            Self::Reparent { id, parent } => {
                text(&id, 128)?;
                if let Some(p) = &parent {
                    text(p, 128)?;
                }
                Action::AuthoringEdit {
                    edit: AuthoringEdit::Reparent { id, parent },
                    expected,
                }
            }
            Self::RemoveLeaf { id } => {
                text(&id, 128)?;
                Action::AuthoringEdit {
                    edit: AuthoringEdit::RemoveLeaf { id },
                    expected,
                }
            }
            Self::ReplaceGeometry { id, geometry } => {
                text(&id, 128)?;
                geometry.validate()?;
                Action::AuthoringEdit {
                    edit: AuthoringEdit::ReplaceGeometry { id, geometry },
                    expected,
                }
            }
            Self::AddGeometry {
                id,
                semantic_id,
                material_from,
                geometry,
            } => {
                text(&id, 128)?;
                text(&material_from, 128)?;
                geometry.validate()?;
                if semantic_id == 0 || semantic_id > i64::MAX as u64 {
                    return Err("Part identifier outside positive control profile".into());
                }
                Action::AuthoringEdit {
                    edit: AuthoringEdit::AddGeometry {
                        id,
                        semantic_id,
                        material_from,
                        geometry,
                    },
                    expected,
                }
            }
            Self::UniqueMaterial { id, material_id } => {
                text(&id, 128)?;
                if material_id == 0 || material_id > i64::MAX as u64 {
                    return Err("Material identifier outside positive control profile".into());
                }
                Action::AuthoringEdit {
                    edit: AuthoringEdit::UniqueMaterial { id, material_id },
                    expected,
                }
            }
            Self::Compile {
                name,
                key,
                expected_sha256,
            } => {
                text(&name, 128)?;
                text(&key, 4096)?;
                Action::CompileAssembly {
                    name,
                    key,
                    file: expected_sha256.map_or(ExpectedFile::Absent, ExpectedFile::Sha256),
                    expected,
                }
            }
        })
    }
}
