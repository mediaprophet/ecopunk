//! Fresh motion runtime from the existing strict resource-preserving HMC replay.
use super::*;
use qualia_core_db::bundle::BundleReader;
use sha2::{Digest, Sha256};
impl AnimalMotionReviewSession {
    /// Reopens the existing limited review profile without inventing a source edit
    /// or revision. Rebuild uses existing fixed DEFAULT rig projection limits;
    /// supplied resource and motion limits cannot enlarge that projection profile.
    /// This is not arbitrary Studio/HMC admission, authority or motion acceptance.
    pub fn reopen(
        bytes: &[u8],
        resources: ReviewResourceLimits,
        motion: MotionLimits,
        cancel: &AtomicBool,
    ) -> Result<Self, String> {
        if cancel.load(Ordering::Acquire) {
            return Err("Motion reopen cancelled".into());
        }
        let reopened = crate::species_review_reopen::reopen_with_limits(1, bytes, resources)?;
        if cancel.load(Ordering::Acquire) {
            return Err("Motion reopen cancelled after bounded source replay".into());
        }
        // Strict owner above has already checked source replay, revision, digests,
        // every resource hash and exact current-generator reproduction of this bundle.
        let reader = BundleReader::parse(&reopened.asset.bundle)
            .map_err(|e| format!("Checked motion package read: {e:?}"))?;
        let evaluated = reader
            .get("authoring/evaluated-source.json")
            .ok_or("Checked motion source missing")?;
        let text =
            std::str::from_utf8(evaluated).map_err(|_| "Checked motion source is not UTF8")?;
        let typed = crate::swamp_wallaby_anatomical_r3::parse_source(text)?;
        let rig = RigLimits::default();
        let player = if typed.motion.is_some() {
            Some(MotionPlayer::admit(
                &reopened.asset.render.bytes,
                &typed,
                rig,
                motion,
                cancel,
            )?)
        } else {
            None
        };
        // Same serialization domain as SpeciesReviewSourceSession's exact history
        // digest; stream into the existing SHA owner rather than cloning source text.
        let mut digest = SourceHistoryDigest {
            hasher: Sha256::new(),
            remaining: 16 * 1024 * 1024,
        };
        serde_json::to_writer(&mut digest, reopened.session.sources())
            .map_err(|_| "Checked source history digest exceeds replay profile")?;
        let vibe_source_sha256 = format!("{:x}", digest.hasher.finalize());
        if cancel.load(Ordering::Acquire) {
            return Err("Motion reopen cancelled before publication".into());
        }
        Ok(Self {
            source: reopened.session,
            current: Some(Candidate {
                asset: reopened.asset,
                player,
                vibe_source_sha256,
            }),
            rig_limits: rig,
            motion_limits: motion,
        })
    }
}
struct SourceHistoryDigest {
    hasher: Sha256,
    remaining: usize,
}
impl std::io::Write for SourceHistoryDigest {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        if data.len() > self.remaining {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Source history digest cap",
            ));
        }
        self.remaining -= data.len();
        self.hasher.update(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
