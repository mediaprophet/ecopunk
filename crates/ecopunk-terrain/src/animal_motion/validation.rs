//! Bounded named-source validation; all angles/targets are authored constraints.
use super::*;
use crate::swamp_wallaby_anatomical_r3::Source;
use qualia_core_db::render::skeletal::{Motor, MAX_KEYS};
use std::collections::BTreeSet;
pub(super) fn index(source: &Source, id: &str) -> Result<usize, String> {
    source
        .joints
        .iter()
        .position(|j| j.id == id)
        .ok_or_else(|| format!("Motion joint absent: {id}"))
}
fn point(v: [f64; 3], bound: f64) -> bool {
    v.iter().all(|x| x.is_finite() && x.abs() <= bound)
}
pub(super) fn validate(
    source: &Source,
    motion: &MotionSource,
    limits: MotionLimits,
) -> Result<(usize, usize), String> {
    if source.joints.is_empty()
        || source.joints.len() > 128
        || source.identifier.is_empty()
        || source.identifier.len() > 1024
        || source.joints.iter().enumerate().any(|(i, j)| {
            j.id.is_empty()
                || j.id.len() > 128
                || j.parent.is_some_and(|p| p >= i)
                || j.parent.is_none() != (i == 0)
                || j.position_m.iter().any(|v| !v.is_finite() || v.abs() > 3.)
        })
        || source
            .joints
            .iter()
            .enumerate()
            .any(|(i, j)| source.joints[..i].iter().any(|old| old.id == j.id))
    {
        return Err("Motion named hierarchy refused".into());
    }
    if source
        .joints
        .first()
        .is_none_or(|j| j.id != "pelvis" || j.parent.is_some())
        || source.units != "m"
        || source.schema != crate::swamp_wallaby_anatomical_r3::SOURCE_SCHEMA
        || source.coordinate_frame != "+Y up; +Z front; ground y=0"
        || motion.schema != SCHEMA
        || motion.status != "authored-study-not-observed"
        || motion.frame != FRAME
        || motion.units != "m,s,radian"
        || motion.clips.is_empty()
        || motion.clips.len() > limits.max_clips.min(31)
    {
        return Err("Motion profile/source/clip count refused".into());
    }
    let mut clips = BTreeSet::new();
    let (mut track_count, mut key_count) = (0usize, 0usize);
    for clip in &motion.clips {
        if clip.id.is_empty()
            || clip.id.len() > 128
            || clip.id.chars().any(char::is_control)
            || !clips.insert(&clip.id)
            || !clip.duration_s.is_finite()
            || !(0.05..=30.0).contains(&clip.duration_s)
            || clip.tracks.is_empty()
            || clip.tracks.len() > limits.max_tracks.min(4096)
            || clip.contacts.len() > MAX_CONTACTS
        {
            return Err("Authored motion clip profile refused".into());
        }
        let mut joints = BTreeSet::new();
        for track in &clip.tracks {
            let joint = index(source, &track.joint_id)?;
            if !joints.insert(joint)
                || track.keys.len() < 2
                || key_count.checked_add(track.keys.len()).is_none_or(|n| {
                    n > limits
                        .max_keys
                        .min(MAX_KEYS.saturating_sub(source.joints.len()))
                })
                || track.keys.len() > u16::MAX as usize
                || !track.max_delta_angle_rad.is_finite()
                || !(0.0..=1.4).contains(&track.max_delta_angle_rad)
                || track.keys.first().is_none_or(|k| k.time_s != 0.0)
                || track
                    .keys
                    .last()
                    .is_none_or(|k| k.time_s != clip.duration_s)
            {
                return Err("Authored motion track profile refused".into());
            }
            let mut previous = -1.;
            for key in &track.keys {
                let q = key.rotation_wxyz;
                let norm = q.iter().map(|v| v * v).sum::<f64>();
                if !key.time_s.is_finite()
                    || key.time_s <= previous
                    || key.time_s > clip.duration_s
                    || q.iter().any(|v| !v.is_finite())
                    || (norm - 1.).abs() > 1e-6
                    || 2. * q[0].abs().min(1.).acos() > track.max_delta_angle_rad + 1e-6
                    || !point(key.translation_m, 4.)
                    || (joint != 0 && key.translation_m != [0.; 3])
                {
                    return Err("Motion key angle/translation/time refused".into());
                }
                previous = key.time_s;
            }
            if clip.looping {
                let a = &track.keys[0];
                let b = track.keys.last().unwrap();
                let dot = a
                    .rotation_wxyz
                    .iter()
                    .zip(b.rotation_wxyz)
                    .map(|(a, b)| a * b)
                    .sum::<f64>()
                    .abs();
                if (dot - 1.).abs() > 1e-6
                    || a.translation_m
                        .iter()
                        .zip(b.translation_m)
                        .any(|(a, b)| (a - b).abs() > 1e-6)
                {
                    return Err("Loop requires explicit identical end/start pose".into());
                }
            }
            track_count = track_count.checked_add(1).ok_or("Motion track overflow")?;
            key_count = key_count
                .checked_add(track.keys.len())
                .ok_or("Motion key overflow")?;
        }
        for (i, contact) in clip.contacts.iter().enumerate() {
            let names = contact.chain.each_ref().map(|s| s.as_str());
            if names != ["left_hip", "left_knee", "left_ankle"]
                && names != ["right_hip", "right_knee", "right_ankle"]
            {
                return Err("Contact v1 supports the explicitly named hindlimbs only".into());
            }
            let chain = contact
                .chain
                .each_ref()
                .map(|s| index(source, s))
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            if source.joints[chain[0]].parent != Some(0)
                || source.joints[chain[1]].parent != Some(chain[0])
                || source.joints[chain[2]].parent != Some(chain[1])
                || !contact.start_s.is_finite()
                || !contact.end_s.is_finite()
                || contact.start_s < 0.
                || contact.start_s >= contact.end_s
                || contact.end_s > clip.duration_s
                || !point(contact.attachment_local_m, 0.5)
                || !point(contact.target_model_m, 8.)
                || !point(contact.pole_model_m, 8.)
                || !contact.min_knee_angle_rad.is_finite()
                || !contact.max_knee_angle_rad.is_finite()
                || contact.min_knee_angle_rad < 0.1
                || contact.min_knee_angle_rad >= contact.max_knee_angle_rad
                || contact.max_knee_angle_rad > 3.1
                || !contact.tolerance_m.is_finite()
                || !(0.00001..=0.02).contains(&contact.tolerance_m)
            {
                return Err("Authored contact frame/interval/limits refused".into());
            }
            if clip.contacts[..i].iter().any(|old| {
                old.chain[0] == contact.chain[0]
                    && old.start_s < contact.end_s
                    && contact.start_s < old.end_s
            }) {
                return Err("Overlapping constraints on one foot are ambiguous".into());
            }
        }
    }
    let contact_count = motion
        .clips
        .iter()
        .try_fold(0usize, |n, c| n.checked_add(c.contacts.len()))
        .ok_or("Motion contact count overflow")?;
    // Simultaneously retained compact SKL arrays, nested metadata Values/maps,
    // vector growth/copies, scoped strings and bound runtime contact copies.
    // Logical conservative reservation, not a measured allocator/RSS claim.
    let cold = key_count
        .checked_mul(2048)
        .and_then(|n| track_count.checked_mul(2048).and_then(|v| n.checked_add(v)))
        .and_then(|n| {
            contact_count
                .checked_mul(8192)
                .and_then(|v| n.checked_add(v))
        })
        .and_then(|n| {
            motion
                .clips
                .len()
                .checked_mul(8192)
                .and_then(|v| n.checked_add(v))
        })
        .and_then(|n| {
            source
                .joints
                .len()
                .checked_mul(2048)
                .and_then(|v| n.checked_add(v))
        })
        .and_then(|n| n.checked_add(64 * 1024));
    if track_count > limits.max_tracks
        || key_count
            > limits
                .max_keys
                .min(MAX_KEYS.saturating_sub(source.joints.len()))
        || cold.is_none_or(|n| n > limits.cold_bytes)
    {
        return Err("Motion source exceeds explicit key/track/cold budget".into());
    }
    Ok((track_count, key_count))
}
pub(super) fn local(rest: Motor, key: &KeySource) -> Motor {
    rest.mul(&Motor::from_rotation_translation(
        key.rotation_wxyz,
        key.translation_m,
    ))
    .normalize()
}
