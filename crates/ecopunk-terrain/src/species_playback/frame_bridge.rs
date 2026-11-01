//! Borrow the authoritative MotionPlayer result without serializing a new .10d,
//! resampling a second player or retaining another world/transport graph.
use crate::animal_motion::MotionFrame;
use qualia_core_db::render::source_frame::{SourceFrameBinding, SourceFrameError, SourceMeshFrame};

pub fn frame<'a>(
    motion: &MotionFrame<'a>,
    binding: SourceFrameBinding,
    frame_sequence: u64,
    timeline_tick: u64,
) -> Result<SourceMeshFrame<'a>, SourceFrameError> {
    if motion.model_sha256 != binding.model_sha256 || motion.source_sha256 != binding.source_sha256
    {
        return Err(SourceFrameError::Stale);
    }
    if motion.receipt.vertices != motion.positions.len()
        || !motion.receipt.time.is_finite()
        || motion.receipt.time < 0.0
        || motion.contact_count > motion.contacts.len()
    {
        return Err(SourceFrameError::Input);
    }
    Ok(SourceMeshFrame {
        binding,
        frame_sequence,
        timeline_tick,
        positions: motion.positions,
        normals: motion.normals,
        tangents: motion.tangents,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animal_motion::{ContactReceipt, MAX_CONTACTS};
    use qualia_core_db::render::skeletal::{PlaybackReceipt, RootMotionDelta};

    #[test]
    fn only_exact_source_model_pair_reaches_borrowed_stream_adapter() {
        let p = [[0., 0., 0.]];
        let n = [[0., 0., 1.]];
        let t = [[1., 0., 0., 1.]];
        let mut motion = MotionFrame {
            receipt: PlaybackReceipt {
                clip_id: 3,
                time: 0.5,
                vertices: 1,
                joints: 1,
                reduced_motion: false,
            },
            positions: &p,
            normals: &n,
            tangents: &t,
            joint_ids: &[1],
            world_motors: &[qualia_core_db::render::skeletal::Motor::identity()],
            contacts: [ContactReceipt::default(); MAX_CONTACTS],
            contact_count: 0,
            root_delta: RootMotionDelta::default(),
            model_sha256: [1; 32],
            source_sha256: [2; 32],
        };
        let binding = SourceFrameBinding {
            model_sha256: [1; 32],
            source_sha256: [2; 32],
            source_generation: 4,
            mesh_generation: 1,
            transport_epoch: 2,
            vertex_count: 1,
            index_count: 3,
        };
        let projected = frame(&motion, binding, 9, 127_008_000_000).unwrap();
        assert!(std::ptr::eq(
            projected.positions.as_ptr(),
            motion.positions.as_ptr()
        ));
        assert_eq!(projected.timeline_tick, 127_008_000_000);
        assert!(matches!(
            frame(
                &motion,
                SourceFrameBinding {
                    source_sha256: [3; 32],
                    ..binding
                },
                9,
                0
            ),
            Err(SourceFrameError::Stale)
        ));
        motion.receipt.vertices = 2;
        assert!(matches!(
            frame(&motion, binding, 9, 0),
            Err(SourceFrameError::Input)
        ));
    }
}
