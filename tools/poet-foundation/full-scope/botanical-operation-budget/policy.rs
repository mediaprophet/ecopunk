//! Staged policy prototype; not wired into Poet or QualiaDB.
//! Units count bounded structural work, not instructions, time, or bytes.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BudgetError { Overflow, Exhausted, Invalid }

fn mul(a: usize, b: usize) -> Result<usize, BudgetError> {
    a.checked_mul(b).ok_or(BudgetError::Overflow)
}
fn add(a: usize, b: usize) -> Result<usize, BudgetError> {
    a.checked_add(b).ok_or(BudgetError::Overflow)
}

/// Limits for *one* call to the existing space-colonization implementation.
/// Each pass is bounded independently; this is an accounting model, not a
/// claim about measured runtime or a substitute for internal cancellation.
#[derive(Clone, Copy, Debug)]
pub struct ColonizationBounds {
    pub seeds: usize,
    pub nodes: usize,
    pub attractors: usize,
    pub iterations: usize,
}

impl ColonizationBounds {
    pub fn structural_work(self) -> Result<usize, BudgetError> {
        let Self { seeds, nodes, attractors, iterations } = self;
        if seeds == 0 || seeds > nodes || attractors == 0 { return Err(BudgetError::Invalid); }
        // Initial kill scan: attractors x seeds. Each iteration can scan
        // attractors x nodes for association, nodes x nodes for duplicates,
        // and attractors x nodes for kills; linear bookkeeping is reserved
        // separately. Initial/final allocation and scans are also charged.
        let initial = add(mul(attractors, seeds)?, add(mul(4, nodes)?, mul(3, attractors)?)?)?;
        let pairs = add(mul(2, mul(attractors, nodes)?)?, mul(nodes, nodes)?)?;
        let iteration = add(pairs, add(mul(8, nodes)?, mul(4, attractors)?)?)?;
        add(initial, mul(iterations, iteration)?)
    }
}

/// Pipe model processes only adopted nodes. At most n scans of n entries are
/// needed: each successful pass processes at least one node; an unsuccessful
/// pass exits immediately. Five additional n-sized loops fill scratch/radii,
/// count children, initialize tips, and finalize radii.
pub fn pipe_structural_work(adopted_nodes: usize) -> Result<usize, BudgetError> {
    if adopted_nodes == 0 { return Err(BudgetError::Invalid); }
    add(mul(adopted_nodes, adopted_nodes)?, mul(5, adopted_nodes)?)
}

/// Exact payload bytes for the two caller-owned initialized pipe buffers;
/// excludes allocator overhead and all other live storage.
pub fn pipe_buffer_bytes(adopted_nodes: usize) -> Result<usize, BudgetError> {
    if adopted_nodes == 0 { return Err(BudgetError::Invalid); }
    add(mul(adopted_nodes, core::mem::size_of::<u32>())?,
        mul(adopted_nodes, core::mem::size_of::<f64>())?)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridBufferCounts {
    pub samples: usize, pub cells: usize, pub vertices: usize,
    pub triangles: usize, pub edge_slots: usize,
}

/// Mirrors the inspected Core extractor's preflight capacities. Real integration
/// must call the Core owner directly and compare its version/contract, not rely
/// on this independent model as a replacement implementation.
pub fn grid_buffer_counts(dims: [usize; 3], max_cells: usize) -> Result<GridBufferCounts, BudgetError> {
    let [x,y,z] = dims;
    if x < 2 || y < 2 || z < 2 { return Err(BudgetError::Invalid); }
    let samples=mul(mul(x,y)?,z)?;
    if samples == usize::MAX { return Err(BudgetError::Overflow); }
    let cells=mul(mul(x-1,y-1)?,z-1)?;
    if cells > max_cells { return Err(BudgetError::Invalid); }
    let vertices=mul(cells,36)?; let triangles=mul(cells,12)?;
    if vertices > u32::MAX as usize { return Err(BudgetError::Overflow); }
    let edge_slots=mul(vertices,2)?.checked_next_power_of_two().ok_or(BudgetError::Overflow)?;
    Ok(GridBufferCounts { samples,cells,vertices,triangles,edge_slots })
}

/// Payload for initialized sample positions/scalars, extraction outputs and
/// hash slots only. `Slot` must be the exact Core TetraEdgeSlot type. Does not
/// include topology, old source, allocator overhead or any other live storage.
pub fn grid_buffer_bytes<Slot>(b: GridBufferCounts) -> Result<usize, BudgetError> {
    let entries = [
        (b.samples,core::mem::size_of::<[f64;3]>()),
        (b.samples,core::mem::size_of::<f64>()),
        (b.vertices,core::mem::size_of::<[f64;3]>()),
        (b.vertices,core::mem::size_of::<[usize;2]>()),
        (b.triangles,core::mem::size_of::<[u32;3]>()),
        (b.edge_slots,core::mem::size_of::<Slot>()),
    ];
    entries.into_iter().try_fold(0usize, |total,(count,size)| add(total,mul(count,size)?))
}

/// All limits are host-selected caps, not claims about observed output.
/// Fixed extraction/qualification work is provided by the owning algorithms'
/// checked bounds; probe allowances are reserved in full because extraction
/// currently does not report consumed probes. Producer and host must agree on
/// the same units, and validate these caps before any allocation or call.
#[derive(Clone, Copy, Debug)]
pub struct RegenerationBounds {
    pub colonization: ColonizationBounds,
    pub field_samples: usize,
    pub field_segments: usize,
    pub extraction_fixed: usize,
    pub extraction_probes: usize,
    pub topology_fixed: usize,
    pub topology_probes: usize,
    pub source_and_projection: usize,
    pub kernel_dispatches: usize,
    /// Concurrent buffers, retained session/source, and other caller-owned
    /// memory included in this operation's preflight, in bytes.
    pub peak_bytes: usize,
}

impl RegenerationBounds {
    pub fn work(self) -> Result<usize, BudgetError> {
        if self.field_samples == 0 || self.field_segments == 0 || self.kernel_dispatches == 0 {
            return Err(BudgetError::Invalid);
        }
        let mut total = add(self.colonization.structural_work()?,pipe_structural_work(self.colonization.seeds)?)?;
        for item in [mul(self.field_samples, self.field_segments)?, self.extraction_fixed,
            self.extraction_probes, self.topology_fixed, self.topology_probes,
            self.source_and_projection] {
            total = add(total, item)?;
        }
        Ok(total)
    }
}

/// One independently scoped edit, inspection, or rebuild; never shared across
/// independent requests. All repeated regenerations in that operation reserve
/// from the same instance. Failed reservations do not debit any resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OperationBudget {
    pub work_remaining: usize,
    pub dispatches_remaining: usize,
    pub peak_bytes_limit: usize,
}

impl OperationBudget {
    pub fn reserve(&mut self, bounds: RegenerationBounds) -> Result<(), BudgetError> {
        let work = bounds.work()?;
        if work > self.work_remaining || bounds.kernel_dispatches > self.dispatches_remaining
            || bounds.peak_bytes > self.peak_bytes_limit {
            return Err(BudgetError::Exhausted);
        }
        self.work_remaining -= work;
        self.dispatches_remaining -= bounds.kernel_dispatches;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn bounds() -> RegenerationBounds {
        RegenerationBounds { colonization: ColonizationBounds { seeds:4,nodes:8,attractors:3,iterations:2 },
            field_samples:10, field_segments:3, extraction_fixed:20, extraction_probes:10,
            topology_fixed:8, topology_probes:6, source_and_projection:15,
            kernel_dispatches:4, peak_bytes:100 }
    }
    #[test]
    fn repeated_passes_share_one_operation_budget() {
        let b=bounds(); let w=b.work().unwrap();
        let mut budget=OperationBudget { work_remaining:w*2,dispatches_remaining:8,peak_bytes_limit:100 };
        budget.reserve(b).unwrap(); budget.reserve(b).unwrap();
        assert_eq!(budget.work_remaining,0); assert_eq!(budget.dispatches_remaining,0);
        assert_eq!(budget.reserve(b),Err(BudgetError::Exhausted));
    }
    #[test]
    fn refusal_does_not_debit_any_dimension() {
        let b=bounds(); let before=OperationBudget { work_remaining:b.work().unwrap(),dispatches_remaining:4,peak_bytes_limit:99 };
        let mut after=before; assert_eq!(after.reserve(b),Err(BudgetError::Exhausted)); assert_eq!(before,after);
        let before=OperationBudget { peak_bytes_limit:100,dispatches_remaining:3,..before };
        let mut after=before; assert_eq!(after.reserve(b),Err(BudgetError::Exhausted)); assert_eq!(before,after);
    }
    #[test]
    fn overflow_and_invalid_caps_fail_before_debit() {
        let before=OperationBudget { work_remaining:usize::MAX,dispatches_remaining:usize::MAX,peak_bytes_limit:usize::MAX };
        let mut b=bounds(); b.field_samples=usize::MAX; let mut after=before;
        assert_eq!(after.reserve(b),Err(BudgetError::Overflow)); assert_eq!(before,after);
        b=bounds(); b.colonization.seeds=9;
        assert_eq!(after.reserve(b),Err(BudgetError::Invalid)); assert_eq!(before,after);
    }
    #[test]
    fn independent_operation_does_not_debit_another() {
        let b=bounds(); let template=OperationBudget { work_remaining:b.work().unwrap(),dispatches_remaining:4,peak_bytes_limit:100 };
        let mut edit=template; edit.reserve(b).unwrap(); let inspection=template;
        assert_eq!(inspection,template); assert_ne!(edit,inspection);
    }
    #[test]
    fn pipe_quadratic_work_and_buffers_are_checked() {
        assert_eq!(pipe_structural_work(4),Ok(36));
        assert_eq!(pipe_buffer_bytes(4),Ok(4*(core::mem::size_of::<u32>()+core::mem::size_of::<f64>())));
        assert_eq!(pipe_structural_work(0),Err(BudgetError::Invalid));
        assert_eq!(pipe_structural_work(usize::MAX),Err(BudgetError::Overflow));
        assert_eq!(pipe_buffer_bytes(usize::MAX),Err(BudgetError::Overflow));
    }
    #[test]
    fn grid_preflight_counts_are_checked() {
        let b=grid_buffer_counts([15,15,15],2744).unwrap();
        assert_eq!(b,GridBufferCounts { samples:3375,cells:2744,vertices:98784,triangles:32928,edge_slots:262144 });
        assert_eq!(grid_buffer_counts([15,15,15],2743),Err(BudgetError::Invalid));
        assert_eq!(grid_buffer_counts([usize::MAX,2,2],usize::MAX),Err(BudgetError::Overflow));
        assert!(grid_buffer_bytes::<[usize;3]>(b).unwrap() > 0);
        assert_eq!(grid_buffer_bytes::<u64>(GridBufferCounts { samples:usize::MAX,..b }),Err(BudgetError::Overflow));
    }
}
