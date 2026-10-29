//! Staged operation ledger. The fork admission remains deliberately disabled.
//! Use a fresh instance for each inspection, edit/submit, or complete rebuild;
//! pass the same mutable instance across all phases of that operation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Limits {
    pub work: usize,
    pub dispatches: usize,
    /// Conservative cumulative allocation allowance: this ledger never
    /// refunds, so its bound is at least the peak of accounted allocations.
    pub allocation_bytes: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Error { InvalidLimits, Overflow, Exhausted, ForkAccountingIncomplete }

/// Deliberately not Clone or Copy: multiple regenerations must not duplicate
/// the remaining budget. A failed kernel cannot refund already charged work.
pub(crate) struct Operation {
    remaining_work: usize,
    remaining_dispatches: usize,
    remaining_allocation_bytes: usize,
}

impl Operation {
    pub(crate) fn new(host_selected: Limits) -> Result<Self, Error> {
        // Zero limits remain valid for legacy hosts; any positive reservation
        // then refuses. Introducing the scope must not reject old operations.
        Ok(Self { remaining_work: host_selected.work,
            remaining_dispatches: host_selected.dispatches,
            remaining_allocation_bytes: host_selected.allocation_bytes })
    }

    /// Reserve atomically before work or allocation. Callers must derive each
    /// amount from checked owner bounds, never accept authored cost claims.
    pub(crate) fn reserve(&mut self, work: usize, dispatches: usize, bytes: usize)
        -> Result<(), Error> {
        if work > self.remaining_work || dispatches > self.remaining_dispatches
            || bytes > self.remaining_allocation_bytes { return Err(Error::Exhausted); }
        self.remaining_work -= work;
        self.remaining_dispatches -= dispatches;
        self.remaining_allocation_bytes -= bytes;
        Ok(())
    }

    /// The combined source parsing, session clones, serialization and full
    /// working set have not been accounted. No route may admit a fork until
    /// the owner supplies and reviews those charges at every entry point.
    pub(crate) fn admit_fork(&mut self) -> Result<(), Error> {
        Err(Error::ForkAccountingIncomplete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn operation() -> Operation {
        Operation::new(Limits { work: 20, dispatches: 3, allocation_bytes: 100 }).unwrap()
    }
    #[test] fn reserves_accumulate_across_phases_and_failure_is_atomic() {
        let mut ledger = operation();
        ledger.reserve(10, 1, 40).unwrap();
        assert_eq!(ledger.reserve(11, 1, 1), Err(Error::Exhausted));
        ledger.reserve(10, 2, 60).unwrap();
        assert_eq!(ledger.reserve(1, 0, 0), Err(Error::Exhausted));
    }
    #[test] fn every_independent_operation_starts_fresh() {
        let mut a = operation();
        let mut b = operation();
        a.reserve(20, 3, 100).unwrap();
        b.reserve(20, 3, 100).unwrap();
    }
    #[test] fn fork_refuses_even_when_all_resources_are_available() {
        assert_eq!(operation().admit_fork(), Err(Error::ForkAccountingIncomplete));
    }
}
