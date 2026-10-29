//! Staged structural-work and scratch-payload accounting; not production wired.
//! Counts owner loop visits and separately reserved probes, not CPU time.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundsError { Invalid, Overflow }

fn add(a: usize, b: usize) -> Result<usize, BoundsError> {
    a.checked_add(b).ok_or(BoundsError::Overflow)
}
fn mul(a: usize, b: usize) -> Result<usize, BoundsError> {
    a.checked_mul(b).ok_or(BoundsError::Overflow)
}
fn sum(parts: &[usize]) -> Result<usize, BoundsError> {
    parts.iter().try_fold(0, |a, b| add(a, *b))
}

/// Caller must pass the results of Core's `tetra_surface_bounds`, not authored
/// counts. Loop-visit units cover sample preflight, both hash-slot clears,
/// corner and tetrahedron processing, and at most three directed edges for
/// each output triangle. The explicit probe budget is reserved separately.
/// One tetrahedron processes four classifications, at most four averaging
/// visits, at most four edge requests and at most two triangle emissions.
pub fn tetra_fixed_work(samples: usize, cells: usize, triangles: usize, slots: usize)
    -> Result<usize, BoundsError> {
    if samples == 0 || cells == 0 || triangles == 0 || slots == 0 {
        return Err(BoundsError::Invalid);
    }
    // A cell has eight corners and six tetrahedra. Each tetrahedron also
    // incurs one fixed visit in addition to its bounded inner operations.
    sum(&[samples, mul(2, slots)?, mul(cells, 8 + 6 * (1 + 4 + 4 + 4 + 2))?,
        mul(3, triangles)?])
}

/// For the field owner, charge sample-position creation, sample and segment
/// input scans, and one evaluation per sample/segment pair. The pair unit is
/// the Core `tapered_field_work` unit; this is an accounting convention, not
/// an assertion that these operations have equivalent costs.
pub fn field_work(samples: usize, segments: usize) -> Result<usize, BoundsError> {
    if samples == 0 || segments == 0 { return Err(BoundsError::Invalid); }
    sum(&[mul(2, samples)?, segments, mul(samples, segments)?])
}

/// Counts taken directly from Core's `surface_qualification_bounds`.
/// Generic types must be the exact Core HalfEdge and EdgeSlot types. Payload
/// only: allocator overhead and all other concurrently live data are excluded.
pub fn topology_scratch_payload<HalfEdge, EdgeSlot>(
    half_edges: usize, edge_slots: usize, vertex_starts: usize,
    visited_half_edges: usize, face_labels: usize, face_queue: usize,
) -> Result<usize, BoundsError> {
    use core::mem::size_of;
    sum(&[mul(half_edges, size_of::<HalfEdge>())?,
        mul(edge_slots, size_of::<EdgeSlot>())?,
        mul(vertex_starts, size_of::<u32>())?,
        mul(visited_half_edges, size_of::<bool>())?,
        mul(face_labels, size_of::<u32>())?,
        mul(face_queue, size_of::<u32>())?])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn field_includes_preflight_and_position_visits() {
        assert_eq!(field_work(10, 3), Ok(53));
    }
    #[test] fn invalid_and_overflow_refuse() {
        assert_eq!(field_work(0, 1), Err(BoundsError::Invalid));
        assert_eq!(field_work(usize::MAX, 2), Err(BoundsError::Overflow));
        assert_eq!(tetra_fixed_work(1, usize::MAX, 1, 1), Err(BoundsError::Overflow));
        assert_eq!(topology_scratch_payload::<u64,u32>(usize::MAX, 0, 0, 0, 0, 0), Err(BoundsError::Overflow));
    }
    #[test] fn fixed_formula_accounts_for_slot_clears_and_worst_case_cells() {
        assert_eq!(tetra_fixed_work(8, 1, 12, 128), Ok(8 + 256 + 98 + 36));
    }
    #[test] fn topology_payload_accounts_for_each_buffer() {
        assert_eq!(topology_scratch_payload::<u64,u32>(3, 8, 2, 3, 1, 1), Ok(24 + 32 + 8 + 3 + 4 + 4));
    }
}
