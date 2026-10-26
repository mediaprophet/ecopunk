//! API/engine smoke evidence. No native-window access or model acceptance.
mod graphical {
    #[path = "C:/github/poet/native/src/graphical/controller.rs"]
    pub mod controller;
    #[path = "C:/github/poet/native/src/graphical/geometry_host.rs"]
    pub mod geometry_host;
    #[path = "C:/github/poet/native/src/graphical/preview.rs"]
    pub mod preview;
}
use graphical::controller::{
    CanonicalEditorHost, EditorHost, MeshPreviewProvider, PreviewProvider,
};
use graphical::geometry_host::{GeometryHost, ORGANIC_SURFACE_SOURCE};
use graphical::preview::{self, PreviewCamera, PreviewFrame, PreviewLimits, SourceRef};
use qualia_core_db::poet_host::invoke::studio_store::{self, StudioArtifact};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use vibe::{session::SessionLimits, Value};

#[derive(Clone, Debug, PartialEq)]
struct GeometryObservation {
    positions: Vec<[f32; 3]>,
    triangles: Vec<[u32; 3]>,
}
struct ObservedPreview {
    real: MeshPreviewProvider,
    geometry: Arc<Mutex<Vec<GeometryObservation>>>,
}
impl PreviewProvider for ObservedPreview {
    fn preview(&mut self, source: SourceRef, value: &Value) -> Result<PreviewFrame, String> {
        let mesh = preview::mesh_from_invoke(value, self.real.limits)?;
        self.geometry
            .lock()
            .map_err(|_| "observation lock poisoned")?
            .push(GeometryObservation {
                positions: mesh.positions,
                triangles: mesh.triangles,
            });
        // This is the actual applied mounted provider, not a replacement renderer.
        self.real.preview(source, value)
    }
}

fn save_frame(directory: &Path, label: &str, frame: &PreviewFrame) -> Result<(), String> {
    frame.source.check_current()?;
    assert!(frame.execution_epoch.as_ref().unwrap().is_current());
    assert_eq!(
        frame.rgba8.len(),
        frame.side as usize * frame.side as usize * 4
    );
    let mut png = vec![0; frame.rgba8.len() + frame.side as usize + 4096];
    let size = qualia_core_db::render::surface_authoring::encode_surface_png_into(
        frame.side,
        &frame.rgba8,
        &mut png,
    )
    .map_err(|e| format!("PNG encoding: {e:?}"))?;
    std::fs::write(directory.join(format!("{label}.png")), &png[..size])
        .map_err(|e| e.to_string())?;
    std::fs::write(directory.join(format!("{label}.rgba8")), &frame.rgba8)
        .map_err(|e| e.to_string())?;
    let uri = preview::native_image_uri(frame)?;
    assert!(uri.starts_with("data:image/png;base64,"));
    let min = frame
        .rgba8
        .chunks_exact(4)
        .flat_map(|p| p[..3].iter())
        .copied()
        .min()
        .unwrap();
    let max = frame
        .rgba8
        .chunks_exact(4)
        .flat_map(|p| p[..3].iter())
        .copied()
        .max()
        .unwrap();
    assert!(max > min, "actual frame must have varying colour values");
    println!("FRAME {label}: side={} png_bytes={size} native_uri_bytes={} rgb_range={min}..{max}; current_at_capture=true source_revision={} session_revision={}",
        frame.side, uri.len(), frame.source.stamp.revision(), frame.session_revision.unwrap());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("SOURCE_SMOKE_FAILED: {error}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    let id = format!("urn:ecopunk:diagnostic:source-smoke:{}", std::process::id());
    let grants = vec![
        "capability.invoke".into(),
        "ComputationalGeometry.extrude_profile".into(),
    ];
    studio_store::create_artifact_if_absent(StudioArtifact::new(
        &id,
        "Geometry",
        "API source/render diagnostic",
    ))?;
    let mut material = qualia_core_db::container_10d::MaterialRecord::legacy_default();
    material.id = qualia_core_db::q_hash("urn:ecopunk:diagnostic:preview-material");
    material.base_color = [0.36, 0.20, 0.085, 1.0];
    material.roughness = 0.65;
    let geometry = Arc::new(Mutex::new(Vec::new()));
    let provider = ObservedPreview {
        real: MeshPreviewProvider {
            material,
            camera: PreviewCamera {
                yaw: 0.45,
                pitch: 0.2,
                distance: 3.2,
                target: [0.0; 3],
            },
            limits: PreviewLimits {
                side: 256,
                ..PreviewLimits::default()
            },
        },
        geometry: geometry.clone(),
    };
    let mut host = CanonicalEditorHost::open(
        id.clone(),
        SessionLimits::default(),
        GeometryHost::new(grants, 200_000),
        Box::new(provider),
    )?;
    let first_source = ORGANIC_SURFACE_SOURCE;
    std::fs::write(directory.join("first.vibe"), first_source).map_err(|e| e.to_string())?;
    println!("PROFILE actual canonical SourceSession + GeometryHost + MeshPreviewProvider + PortalGpu; explicit grants=[capability.invoke,ComputationalGeometry.extrude_profile]; side=256; no UI access");
    println!("CHECK {}", host.check(first_source)?);
    let first = host.submit(first_source, host.snapshot()?.source)?;
    println!("FIRST {}", first.message);
    let first_frame = first
        .frame
        .ok_or("first actual renderer produced no frame")?;
    save_frame(directory, "first", &first_frame)?;

    // Runtime source edit in the same process; no Rust/Cargo step between submissions.
    let edited_source = first_source.replace("[0.18, 0.02, 0.3]", "[0.65, 0.02, 0.3]");
    assert_ne!(edited_source, first_source);
    std::fs::write(directory.join("edited.vibe"), &edited_source).map_err(|e| e.to_string())?;
    let second = host.submit(&edited_source, host.snapshot()?.source)?;
    let second_frame = second
        .frame
        .ok_or("edited actual renderer produced no frame")?;
    save_frame(directory, "edited", &second_frame)?;
    let changed_pixels = first_frame
        .rgba8
        .chunks_exact(4)
        .zip(second_frame.rgba8.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        changed_pixels > 0,
        "source edit must change actual output pixels"
    );
    {
        let observations = geometry.lock().unwrap();
        assert_eq!(observations.len(), 2);
        assert_ne!(observations[0].positions, observations[1].positions);
        println!("EDIT actual geometry changed: vertices={}/{} triangles={}/{} changed_pixels={changed_pixels}",
            observations[0].positions.len(), observations[1].positions.len(),
            observations[0].triangles.len(), observations[1].triangles.len());
    }
    let accepted = host.snapshot()?;
    let before_invalid = studio_store::get_artifact_stamped(&id).unwrap().0;
    assert!(host.submit("let = ;", accepted.source.clone()).is_err());
    let invalid = host.snapshot()?;
    assert_eq!(invalid.source, accepted.source);
    assert_eq!(invalid.session_revision, accepted.session_revision);
    assert_eq!(invalid.execution_epoch, accepted.execution_epoch);
    assert!(!invalid.reconciliation_required);
    assert_eq!(
        studio_store::get_artifact_stamped(&id).unwrap().0,
        before_invalid
    );
    println!("INVALID source rejected with no source/session/epoch promotion");

    let bad_args = r#"requires [capability("capability.invoke"), capability("ComputationalGeometry.extrude_profile")];
        capability.invoke("ComputationalGeometry.extrude_profile", {axis:[],section:[]});"#;
    let error = host
        .submit(bad_args, accepted.source.clone())
        .err()
        .ok_or("runtime badargs unexpectedly succeeded")?;
    println!("RUNTIME expected rejection: {error}");
    let held = host.snapshot()?;
    assert!(held.reconciliation_required);
    assert_eq!(held.source, accepted.source);
    assert!(host.submit(first_source, held.source.clone()).is_err());
    let rebuilt = host.rebuild_execution(held.source)?.snapshot;
    assert!(!rebuilt.reconciliation_required);
    assert_eq!(rebuilt.source, accepted.source);
    assert_eq!(rebuilt.session_revision, 2);
    assert!(!accepted.execution_epoch.is_current());
    assert!(rebuilt.execution_epoch.is_current());
    assert_eq!(
        geometry.lock().unwrap().len(),
        2,
        "no GPU frame during reconstruction"
    );
    println!("REBUILD runtime failure recovered from exact accepted cells; old epoch retired; no preview replay");

    let undone = host.undo(rebuilt.source)?;
    assert!(undone.reconciliation_required);
    assert_eq!(undone.latest_source, first_source);
    let recovered = host.rebuild_execution(undone.source)?.snapshot;
    assert_eq!(recovered.session_revision, 1);
    assert!(!recovered.reconciliation_required);
    assert!(!rebuilt.execution_epoch.is_current());
    assert!(recovered.execution_epoch.is_current());
    assert_eq!(geometry.lock().unwrap().len(), 2);
    println!("UNDO source restored; explicit rebuild preserves accepted first cell and retires prior epoch");

    let final_source = first_source.replace("[0.12, 0.06, -0.4]", "[0.12, 0.40, -0.4]");
    std::fs::write(directory.join("final.vibe"), &final_source).map_err(|e| e.to_string())?;
    let final_outcome = host.submit(&final_source, recovered.source)?;
    let final_frame = final_outcome
        .frame
        .ok_or("post-undo edited renderer produced no frame")?;
    save_frame(directory, "final", &final_frame)?;
    let artifact = studio_store::get_artifact_stamped(&id).unwrap().0;
    assert_eq!(artifact.nodes.len(), 2);
    assert!(artifact.redo_stack.is_empty());
    assert_eq!(final_outcome.snapshot.session_revision, 2);
    assert_eq!(final_outcome.snapshot.latest_source, final_source);
    {
        let observations = geometry.lock().unwrap();
        assert_ne!(observations[0].positions, observations[2].positions);
    }
    std::fs::write(
        directory.join("accepted-artifact-debug.txt"),
        format!("{artifact:#?}"),
    )
    .map_err(|e| e.to_string())?;
    std::fs::write(
        directory.join("geometry-observations.txt"),
        format!("{:#?}", geometry.lock().unwrap()),
    )
    .map_err(|e| e.to_string())?;
    println!("PASS same-process source→real geometry→real RGBA; edit/invalid/runtime hold/rebuild/Undo/rebuild/edit. Static factor material; no UI verification or master/model acceptance.");
    Ok(())
}
