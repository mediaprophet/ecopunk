//! Compile named delta keys to canonical SKL1 records; no motion serializer.
use super::*;
use crate::swamp_wallaby_anatomical_r3::Source;
use qualia_core_db::{
    container_10d::skeletal_section::ClipRecord,
    q_hash,
    render::skeletal::{Joint, Key, Track},
};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
pub struct CompiledMotion {
    pub clips: Vec<ClipRecord>,
    pub tracks: Vec<Track>,
    pub keys: Vec<Key>,
    pub metadata: serde_json::Value,
}
pub fn clip_id(source_identifier: &str, id: &str) -> u64 {
    q_hash(&format!(
        "urn:ecopunk:authored-rig:{:x}:clip:{:x}",
        Sha256::digest(source_identifier.as_bytes()),
        Sha256::digest(id.as_bytes())
    ))
}
/// Offsets name existing rest clip tracks/keys. Output appends to those arrays.
pub fn compile(
    source: &Source,
    joints: &[Joint],
    first_track: usize,
    first_key: usize,
    limits: MotionLimits,
    cancel: &AtomicBool,
) -> Result<CompiledMotion, String> {
    if cancel.load(Ordering::Acquire) {
        return Err("Motion compile cancelled".into());
    }
    let Some(motion) = source.motion.as_ref() else {
        return Ok(CompiledMotion {
            clips: vec![],
            tracks: vec![],
            keys: vec![],
            metadata: serde_json::Value::Null,
        });
    };
    let (tracks, keys) = validation::validate(source, motion, limits)?;
    if joints.len() != source.joints.len()
        || first_key.checked_add(keys).is_none_or(|n| n > 4096)
        || first_track.checked_add(tracks).is_none_or(|n| n > 4096)
    {
        return Err("Motion canonical offsets exceed SKL1 profile".into());
    }
    let mut out = CompiledMotion {
        clips: Vec::new(),
        tracks: Vec::new(),
        keys: Vec::new(),
        metadata: serde_json::Value::Null,
    };
    out.clips
        .try_reserve_exact(motion.clips.len())
        .map_err(|_| "Motion clip allocation refused")?;
    out.tracks
        .try_reserve_exact(tracks)
        .map_err(|_| "Motion track allocation refused")?;
    out.keys
        .try_reserve_exact(keys)
        .map_err(|_| "Motion key allocation refused")?;
    let mut mapped = Vec::new();
    for clip in &motion.clips {
        if cancel.load(Ordering::Acquire) {
            return Err("Motion compile cancelled".into());
        }
        let id = clip_id(&source.identifier, &clip.id);
        if id == 0 || out.clips.iter().any(|c| c.id == id) {
            return Err("Motion clip identifier collision".into());
        }
        let start = first_track + out.tracks.len();
        for track in &clip.tracks {
            let joint = validation::index(source, &track.joint_id)?;
            out.tracks.push(Track {
                joint: joint as u16,
                first_key: (first_key + out.keys.len()) as u32,
                key_count: track.keys.len() as u16,
            });
            for key in &track.keys {
                out.keys.push(Key {
                    time: key.time_s,
                    local: validation::local(joints[joint].rest_local, key),
                });
            }
        }
        out.clips.push(ClipRecord {
            id,
            duration: clip.duration_s,
            first_track: start as u32,
            track_count: clip.tracks.len() as u32,
            looping: clip.looping,
        });
        mapped.push(serde_json::json!({"source_clip_id":clip.id,"render_clip_id":id,"duration_s":clip.duration_s,"looping":clip.looping,
            "tracks":clip.tracks,"contacts":clip.contacts,"scope":"Authored study; contacts require source-bound runtime IK; SKL1 keys alone are not contact proof"}));
    }
    out.metadata = serde_json::json!({"schema":SCHEMA,"status":"authored-study-not-observed","frame":FRAME,"clips":mapped,
        "root_motion":"translation remains embedded in pose; runtime displacement is a report, never applied twice",
        "contact_scope":"Explicit local attachment to authored model-space targets; no terrain/collision or measured anatomy claim"});
    Ok(out)
}
