//! Regression for the existing muscle producer; validation remains in its owner.
use crate::zoological_anatomy::{AnatomyMeshAccumulator, ZoologicalAnatomicalLayer};
#[test]
fn muscle_attachment_envelope_has_finite_endpoint_streams_and_retains_weights() {
    let mut mesh = AnatomyMeshAccumulator::new();
    let origin = [0., 0.2, 0.];
    let insertion = [0., 0.4, 0.1];
    mesh.push_fusiform_muscle(
        origin,
        insertion,
        0.02,
        0.03,
        [0.01, 0., 0.],
        [0.4, 0.1, 0.1, 1.],
        [0.8, 0.8, 0.7, 1.],
        ZoologicalAnatomicalLayer::MyologyAndTendons,
        6,
        7,
    );
    let n = mesh.positions.len();
    assert_eq!(n, 7 * 9 + 2);
    assert_eq!(mesh.normals.len(), n);
    assert_eq!(mesh.colors.len(), n);
    assert_eq!(mesh.uvs.len(), n);
    assert_eq!(mesh.layer_tags.len(), n);
    assert_eq!(mesh.joint_indices.len(), n);
    assert_eq!(mesh.joint_weights.len(), n);
    assert!(mesh
        .positions
        .iter()
        .flatten()
        .chain(mesh.normals.iter().flatten())
        .chain(mesh.colors.iter().flatten())
        .chain(mesh.uvs.iter().flatten())
        .chain(mesh.joint_weights.iter().flatten())
        .all(|v| v.is_finite()));
    assert_eq!(mesh.positions[7 * 9], origin);
    assert_eq!(mesh.positions[7 * 9 + 1], insertion);
    assert_eq!(mesh.joint_indices[7 * 9 + 1], [6, 7, 0, 0]);
    let weight = 1. - 0.70f32;
    assert_eq!(mesh.joint_weights[7 * 9 + 1], [weight, 1. - weight, 0., 0.]);
    assert_eq!(mesh.uvs[7 * 9], [0.5, 0.]);
    assert_eq!(mesh.uvs[7 * 9 + 1], [0.5, 1.]);
    assert!(mesh.indices.iter().all(|&i| (i as usize) < n));
}
