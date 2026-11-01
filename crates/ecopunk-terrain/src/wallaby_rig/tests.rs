use super::*;
use crate::{species_authored_review, species_review_source::SpeciesReviewSourceSession};
use qualia_core_db::{
    bundle::BundleReader, container_10d::skeletal_container::read_skeletal_section,
    render::skeletal::Motor,
};
const SOURCE: &str = include_str!(
    "../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe"
);
fn fixture() -> (
    SpeciesReviewSourceSession,
    species_authored_review::AuthoredReviewAsset,
) {
    let mut session = SpeciesReviewSourceSession::default();
    let asset = session
        .submit(SOURCE, |s| species_authored_review::build(1, s))
        .unwrap();
    (session, asset)
}
#[test]
fn actual_r3_material_remap_preserves_exact_weights_and_rest_controls() {
    let (_, asset) = fixture();
    let section = read_skeletal_section(&asset.render.bytes).unwrap();
    let bundle = BundleReader::parse(&asset.bundle).unwrap();
    assert_eq!(
        bundle.get("review/model.10d"),
        Some(asset.render.bytes.as_slice())
    );
    let anatomy: serde_json::Value =
        serde_json::from_slice(bundle.get("authoring/anatomy.json").unwrap()).unwrap();
    let mut cursor = 0;
    for material in anatomy["render_bindings"].as_array().unwrap() {
        for source in material["source_skin_vertices"].as_array().unwrap() {
            let i = source.as_u64().unwrap() as usize;
            let weight = section.skin(cursor).unwrap();
            let joints: [u16; 4] =
                serde_json::from_value(anatomy["skin"]["joint_indices"][i].clone()).unwrap();
            let weights: [f32; 4] =
                serde_json::from_value(anatomy["skin"]["joint_weights"][i].clone()).unwrap();
            assert_eq!(weight.joints, joints);
            assert_eq!(weight.weights, weights);
            cursor += 1;
        }
    }
    assert_eq!(cursor, section.counts().vertices);
    assert_eq!(section.counts().joints, 21);
    assert_eq!(section.counts().clips, 1);
    assert!(!section.clip(0).unwrap().looping);
    assert!(admit_model(
        &asset.render.bytes,
        RigLimits::default(),
        TangentPolicy::RefuseStoredTangents
    )
    .is_err());
    let mut player = admit_model(
        &asset.render.bytes,
        RigLimits::default(),
        TangentPolicy::RegenerateFromRetainedUv,
    )
    .unwrap();
    let bind = player.positions().to_vec();
    let normals = player.normals().to_vec();
    player.advance(0.75, false).unwrap();
    assert_eq!(player.positions(), bind.as_slice());
    // A deliberately synthetic control displacement checks hierarchical weight
    // use. This is not an authored animal clip or anatomical bend acceptance.
    let selected = (1..section.counts().joints)
        .find(|&j| {
            (0..cursor).any(|v| {
                let w = section.skin(v).unwrap();
                w.joints
                    .iter()
                    .zip(w.weights)
                    .any(|(&i, w)| i as usize == j && w > 0.)
            })
        })
        .unwrap();
    player
        .set_joint_delta(
            section.joint_id(selected).unwrap(),
            Motor::from_translation(0.025, 0., 0.),
        )
        .unwrap();
    player.advance(0., false).unwrap();
    let mut moved = false;
    for (i, p) in player.positions().iter().enumerate() {
        let w = section.skin(i).unwrap();
        let mut fraction = 0.;
        for (&joint, weight) in w.joints.iter().zip(w.weights) {
            if weight == 0. {
                continue;
            }
            let mut j = joint as usize;
            loop {
                if j == selected {
                    fraction += weight;
                    break;
                }
                match section.joint(j).unwrap().parent {
                    Some(parent) => j = parent as usize,
                    None => break,
                }
            }
        }
        let expected = bind[i][0] + fraction * 0.025;
        assert!((p[0] - expected).abs() < 3e-5);
        assert!((p[1] - bind[i][1]).abs() < 3e-5);
        assert!((p[2] - bind[i][2]).abs() < 3e-5);
        moved |= fraction > 0.;
        for axis in 0..3 {
            assert!((player.normals()[i][axis] - normals[i][axis]).abs() < 2e-4);
        }
    }
    assert!(moved);
    assert!(player.tangents().iter().flatten().all(|v| v.is_finite()));
}
#[test]
fn source_edit_rebuilds_rig_and_reopens_without_fabricated_motion() {
    let (_, asset) = fixture();
    let first = read_skeletal_section(&asset.render.bytes).unwrap();
    let root_id = first.joint_id(0).unwrap();
    let mut reopened = crate::species_review_reopen::reopen(1, &asset.bundle).unwrap();
    assert_eq!(reopened.asset.render.bytes, asset.render.bytes);
    let edit = SOURCE
        .replace("let mut species_asset =", "species_asset =")
        .replace(
            "position_m: [0, 0.43, -0.12]",
            "position_m: [0, 0.44, -0.12]",
        );
    assert_ne!(
        edit,
        SOURCE.replace("let mut species_asset =", "species_asset =")
    );
    let changed = reopened.session.submit(&edit, 1).unwrap();
    assert_ne!(changed.render.bytes, asset.render.bytes);
    let next = read_skeletal_section(&changed.render.bytes).unwrap();
    assert_eq!(next.joint_id(0), Some(root_id));
    let player = admit_model(
        &changed.render.bytes,
        RigLimits::default(),
        TangentPolicy::RegenerateFromRetainedUv,
    )
    .unwrap();
    let position = player.attachment(root_id).unwrap().transform_point([0.; 3]);
    assert!((position[1] - 0.44).abs() < 1e-6);
    assert_eq!(
        crate::species_review_reopen::reopen(1, &changed.bundle)
            .unwrap()
            .session
            .revision(),
        2
    );
    let prior = reopened.session.sources().to_vec();
    let bad = edit.replace("parent: null", "parent: 0");
    assert!(reopened.session.submit(&bad, 2).is_err());
    assert_eq!(reopened.session.revision(), 2);
    assert_eq!(reopened.session.sources(), prior.as_slice());
}
#[test]
fn malformed_influences_lineage_hierarchy_and_budgets_refuse() {
    let mut candidate = crate::swamp_wallaby_anatomical_r3::build_swamp_wallaby_authored_review(
        crate::swamp_wallaby_anatomical_r3::DEFAULT_SOURCE,
    )
    .unwrap();
    let limits = RigLimits::default();
    let digest = "0".repeat(64);
    assert!(prepare(&candidate, &[u32::MAX], &digest, limits).is_err());
    assert!(prepare(
        &candidate,
        &[0, 1],
        &digest,
        RigLimits {
            max_vertices: 1,
            ..limits
        }
    )
    .is_err());
    assert!(prepare(
        &candidate,
        &[0, 1],
        &digest,
        RigLimits {
            cold_bytes: 1,
            ..limits
        }
    )
    .is_err());
    let original = candidate.skin.joint_weights[0];
    candidate.skin.joint_weights[0] = [-1., 2., 0., 0.];
    assert!(prepare(&candidate, &[0, 1], &digest, limits).is_err());
    candidate.skin.joint_weights[0] = original;
    candidate.source.joints[1].parent = Some(1);
    assert!(prepare(&candidate, &[0, 1], &digest, limits).is_err());
    let (_, asset) = fixture();
    assert!(admit_model(
        &asset.render.bytes,
        RigLimits {
            cold_bytes: 1,
            ..limits
        },
        TangentPolicy::RegenerateFromRetainedUv
    )
    .is_err());
    assert!(admit_model(
        &asset.render.bytes,
        RigLimits {
            player_bytes: 1,
            ..limits
        },
        TangentPolicy::RegenerateFromRetainedUv
    )
    .is_err());
    let mut corrupt = asset.render.bytes.clone();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 1;
    assert!(admit_model(&corrupt, limits, TangentPolicy::RegenerateFromRetainedUv).is_err());
}
