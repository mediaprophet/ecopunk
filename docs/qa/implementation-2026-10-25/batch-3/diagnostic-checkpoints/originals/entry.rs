//! Native Poet Studio admission and launch route. No default grants or replacement import.
use super::controller::{CanonicalEditorHost, MeshPreviewProvider};
use super::geometry_host::GeometryHost;
use super::materials::diagnostic_source;
use super::preview::{PreviewCamera, PreviewLimits};
use qualia_core_db::poet_host::invoke::studio_store::{create_artifact_if_absent, StudioArtifact};
use vibe::session::{SessionLimits, SourceSession};

pub fn run(args: &[String]) -> Result<(), String> {
    let mut grants = Vec::new();
    let mut diagnostic = false;
    let mut check_source = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--diagnostic-new-source" => diagnostic = true,
            "--check-source" => check_source = true,
            "--grant" if args.get(index + 1).is_some_and(|value| matches!(value.as_str(),
                "capability.invoke" | "ComputationalGeometry.extrude_profile" | "ComputationalGeometry.swung_surface")) => {
                let grant = args[index + 1].clone();
                if !grants.contains(&grant) { grants.push(grant); }
                index += 1;
            }
            _ => return Err("Usage: poet-native-authoring --diagnostic-new-source [--grant capability.invoke] [--grant ComputationalGeometry.extrude_profile] [--grant ComputationalGeometry.swung_surface] [--check-source]".into()),
        }
        index += 1;
    }
    if !diagnostic {
        return Err("Explicit diagnostic source admission required; existing project admission belongs to the native host".into());
    }
    let id = format!("urn:ecopunk:diagnostic:poet-source:{}", std::process::id());
    create_artifact_if_absent(StudioArtifact::new(
        &id,
        "Geometry",
        "Poet Studio · editable surface diagnostic",
    ))?;
    let session = SourceSession::new(grants.clone(), SessionLimits::default());
    if check_source {
        session
            .prepare(&diagnostic_source())
            .map_err(|error| error.message)?;
        println!("Canonical source preparation passed; no evaluation/render/window attempted.");
        return Ok(());
    }
    let mut material = qualia_core_db::container_10d::MaterialRecord::legacy_default();
    material.id = qualia_core_db::q_hash("urn:ecopunk:diagnostic:preview-material");
    material.base_color = [0.36, 0.20, 0.085, 1.0];
    material.roughness = 0.65;
    let provider = MeshPreviewProvider {
        material,
        camera: PreviewCamera {
            yaw: 0.45,
            pitch: 0.2,
            distance: 3.2,
            target: [0.0, 0.0, 0.0],
        },
        limits: PreviewLimits::default(),
    };
    let host = CanonicalEditorHost::open(
        id,
        SessionLimits::default(),
        GeometryHost::new(grants, 200_000),
        Box::new(provider),
    )?;
    super::launch(Box::new(host), diagnostic_source())
}
