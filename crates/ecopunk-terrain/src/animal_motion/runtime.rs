//! CPU-only source-bound Core sampler. An error retires this handle; no frame is
//! returned for GPU publication. Internal Core mutation is not rolled back.
use super::*;
use crate::{swamp_wallaby_anatomical_r3::Source, wallaby_rig::RigLimits};
use qualia_core_db::render::skeletal::{
    solve_two_bone_motor_ik_zero_heap, Motor, PlaybackReceipt, RootMotionDelta,
    RootMotionExtractor, SkeletalPlayer, PoseTimePolicy,
};
use std::sync::atomic::{AtomicBool, Ordering};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionError {
    Retired,
    Cancelled,
    Input,
    UnknownClip,
    Kernel,
    Contact,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ContactReceipt {
    pub foot_joint: usize,
    pub target_model_m: [f64; 3],
    pub actual_model_m: [f64; 3],
    pub error_m: f64,
    pub knee_angle_rad: f32,
}
pub struct MotionFrame<'a> {
    pub receipt: PlaybackReceipt,
    pub positions: &'a [[f32; 3]],
    pub normals: &'a [[f32; 3]],
    pub tangents: &'a [[f32; 4]],
    /// Same post-contact/IK pose as the skin streams; borrowed for this frame only.
    pub joint_ids: &'a [u64],
    pub world_motors: &'a [Motor],
    pub contacts: [ContactReceipt; MAX_CONTACTS],
    pub contact_count: usize,
    pub root_delta: RootMotionDelta,
    pub model_sha256: [u8; 32],
    pub source_sha256: [u8; 32],
}
/// Non-mutating description of an explicitly authored clip. No availability
/// fallback or biological acceptance is implied by this descriptor.
#[derive(Clone, Copy, Debug)]
pub struct MotionClipDescriptor<'a> {
    pub id: u64,
    pub name: &'a str,
    pub duration_s: f64,
    pub looping: bool,
}
pub(super) struct BoundContact {
    pub indices: [usize; 3],
    pub parent: usize,
    pub source: ContactSource,
}
pub(super) struct BoundClip {
    pub id: u64,
    pub name: String,
    pub duration: f64,
    pub looping: bool,
    pub contacts: Vec<BoundContact>,
}
pub struct MotionPlayer {
    pub(super) player: SkeletalPlayer,
    pub(super) ids: Vec<u64>,
    pub(super) clips: Vec<BoundClip>,
    pub(super) selected: usize,
    pub(super) extractor: RootMotionExtractor,
    pub(super) last_time: f64,
    pub(super) retired: bool,
    pub(super) model_sha256: [u8; 32],
    pub(super) source_sha256: [u8; 32],
}
impl MotionPlayer {
    pub fn admit(
        model: &[u8],
        source: &Source,
        rig: RigLimits,
        limits: MotionLimits,
        cancel: &AtomicBool,
    ) -> Result<Self, String> {
        super::binding::admit(model, source, rig, limits, cancel)
    }
    pub fn model_sha256(&self) -> [u8; 32] {
        self.model_sha256
    }
    pub fn source_sha256(&self) -> [u8; 32] {
        self.source_sha256
    }
    pub fn clip_names(&self) -> impl Iterator<Item = &str> {
        self.clips.iter().map(|c| c.name.as_str())
    }
    pub fn clip_descriptor(&self, name: &str) -> Option<MotionClipDescriptor<'_>> {
        self.clips.iter().find(|c| c.name == name).map(|c| MotionClipDescriptor {
            id: c.id, name: &c.name, duration_s: c.duration, looping: c.looping,
        })
    }
    pub fn is_retired(&self) -> bool { self.retired }
    pub fn select_clip(&mut self, name: &str, fade_s: f64) -> Result<(), MotionError> {
        if self.retired {
            return Err(MotionError::Retired);
        }
        let Some(i) = self.clips.iter().position(|c| c.name == name) else {
            return self.fail(MotionError::UnknownClip);
        };
        if !fade_s.is_finite() || !(0. ..=10.).contains(&fade_s) {
            return self.fail(MotionError::Input);
        }
        if i != self.selected {
            if self.player.select_clip(self.clips[i].id, fade_s).is_err() {
                return self.fail(MotionError::Kernel);
            }
            self.selected = i;
            self.extractor.initialized = false;
            self.last_time = 0.;
        }
        Ok(())
    }
    pub fn seek(&mut self, time: f64) -> Result<(), MotionError> {
        if self.retired {
            return Err(MotionError::Retired);
        }
        if !time.is_finite() || time < 0. || time > self.clips[self.selected].duration {
            return self.fail(MotionError::Input);
        }
        if self.player.seek(time).is_err() {
            return self.fail(MotionError::Kernel);
        }
        self.extractor.initialized = false;
        self.last_time = time;
        Ok(())
    }
    /// Deterministic random access; cancels transitions and resets delta tracking.
    pub fn sample_at(
        &mut self,
        name: &str,
        time: f64,
        cancel: &AtomicBool,
    ) -> Result<MotionFrame<'_>, MotionError> {
        self.select_clip(name, 0.)?;
        self.seek(time)?;
        self.advance_with_policy(0., PoseTimePolicy::ExactClipInterval, cancel)
    }
    fn fail<T>(&mut self, error: MotionError) -> Result<T, MotionError> {
        self.retired = true;
        Err(error)
    }
    /// dt is bounded by Core to one second. Natural clip changes use Core's pose
    /// blend; contact targets apply to the selected clip even during that blend.
    pub fn advance(
        &mut self,
        dt: f64,
        cancel: &AtomicBool,
    ) -> Result<MotionFrame<'_>, MotionError> {
        self.advance_with_policy(dt, PoseTimePolicy::AuthoredLooping, cancel)
    }
    fn advance_with_policy(&mut self, dt: f64, policy: PoseTimePolicy,
        cancel: &AtomicBool) -> Result<MotionFrame<'_>, MotionError> {
        if self.retired {
            return Err(MotionError::Retired);
        }
        if cancel.load(Ordering::Acquire) {
            return self.fail(MotionError::Cancelled);
        }
        if !dt.is_finite() || !(0. ..=1.).contains(&dt) {
            return self.fail(MotionError::Input);
        }
        for &id in &self.ids {
            if self.player.set_joint_delta(id, Motor::identity()).is_err() {
                return self.fail(MotionError::Kernel);
            }
        }
        let base = match self.player.advance_pose_only_with_policy(dt, false, policy) {
            Ok(r) => r,
            Err(_) => return self.fail(MotionError::Kernel),
        };
        let mut receipts = [ContactReceipt::default(); MAX_CONTACTS];
        let mut active = 0;
        for c in &self.clips[self.selected].contacts {
            let s = &c.source;
            if !(s.start_s <= base.time
                && (base.time < s.end_s
                    || (base.time == self.clips[self.selected].duration && s.end_s == base.time)))
            {
                continue;
            }
            let worlds = self.player.world_motors();
            let [root, mid, end] = c.indices;
            let parent_world = worlds[c.parent];
            let before = [worlds[root], worlds[mid], worlds[end]];
            let (rot, _) = before[2].to_rotation_translation();
            let offset = Motor::from_rotation_translation(rot, [0.; 3])
                .transform_point(s.attachment_local_m);
            let target: [f32; 3] =
                std::array::from_fn(|a| (s.target_model_m[a] - offset[a]) as f32);
            let solved = match solve_two_bone_motor_ik_zero_heap(
                before[0],
                before[1],
                before[2],
                0.,
                0.,
                target,
                s.pole_model_m.map(|v| v as f32),
                s.min_knee_angle_rad as f32,
                s.max_knee_angle_rad as f32,
            ) {
                Ok(v) => v,
                Err(_) => return self.fail(MotionError::Contact),
            };
            let desired = solved.world_motors;
            for slot in 0..3 {
                let old_parent = if slot == 0 {
                    parent_world
                } else {
                    before[slot - 1]
                };
                let new_parent = if slot == 0 {
                    parent_world
                } else {
                    desired[slot - 1]
                };
                let old_local = old_parent.conjugate().mul(&before[slot]);
                let new_local = new_parent.conjugate().mul(&desired[slot]);
                let delta = old_local.conjugate().mul(&new_local).normalize();
                if self
                    .player
                    .set_joint_delta(self.ids[c.indices[slot]], delta)
                    .is_err()
                {
                    return self.fail(MotionError::Kernel);
                }
            }
            receipts[active] = ContactReceipt {
                foot_joint: end,
                target_model_m: s.target_model_m,
                actual_model_m: s.attachment_local_m,
                error_m: s.tolerance_m,
                knee_angle_rad: solved.knee_angle_rad,
            };
            active += 1;
        }
        let receipt = match self.player.advance_with_policy(0., false, policy) {
            Ok(r) => r,
            Err(_) => return self.fail(MotionError::Kernel),
        };
        for r in &mut receipts[..active] {
            let actual = self.player.world_motors()[r.foot_joint].transform_point(r.actual_model_m);
            let error = (0..3)
                .map(|a| (actual[a] - r.target_model_m[a]).powi(2))
                .sum::<f64>()
                .sqrt();
            if !error.is_finite() || error > r.error_m {
                return self.fail(MotionError::Contact);
            }
            r.actual_model_m = actual;
            r.error_m = error;
        }
        if cancel.load(Ordering::Acquire) {
            return self.fail(MotionError::Cancelled);
        }
        let mut root = [self.player.world_motors()[0]];
        self.extractor.lock_horizontal = false;
        self.extractor.lock_vertical = false;
        // Loop source endpoints are required equal. No seam displacement is invented.
        let root_delta = self.extractor.extract_and_lock(&mut root, false);
        self.last_time = receipt.time;
        Ok(MotionFrame {
            receipt,
            positions: self.player.positions(),
            normals: self.player.normals(),
            tangents: self.player.tangents(),
            joint_ids: &self.ids,
            world_motors: self.player.world_motors(),
            contacts: receipts,
            contact_count: active,
            root_delta,
            model_sha256: self.model_sha256,
            source_sha256: self.source_sha256,
        })
    }
}
