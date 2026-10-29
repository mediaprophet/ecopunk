# Staged botanical kernel accounting

`kernel_bounds.rs` is a staged, uncompiled supplement to the prior botanical
budget prototype. It is not a host guard and does not complete aggregate
operation budgeting. No builds or tests were run.

It adds checked structural counts for the current indexed tetrahedron owner's
fixed loops and the tapered-field owner's input scanning and sample/segment
evaluation, and exact element-payload arithmetic for the six scratch buffers in
`surface_qualification_bounds`. It excludes extraction's separately reserved
hash probes and all unlisted memory. The visit units describe bounded loop
iterations and fixed operations; they are not instructions, equal-duration
operations, elapsed time, allocation overhead or an enforced CPU limit.

Integration must use `tetra_surface_bounds` and
`surface_qualification_bounds` directly as the authoritative count owners,
use the exact Core types for generic buffer sizes, and reserve the entire
separately host-selected probe cap before calling the extractor. Its probe
counter is not returned. Topology's budgeted owner already charges its actual
probes, but the combined operation needs an explicit consistent convention to
avoid undercharging or double-charging. Keep each failed kernel output private.

This is not safe to use as the aggregate guard: source and projection work,
serialization, all concurrent session/renderer/intermediate allocations,
allocator overhead, retained source values, and Vibe replay remain unresolved.
The candidate edit is generated before `begin_submission`, and validation
regenerates afterwards using an immutable host. Establish an operation-scoped
ledger across those phases and rebuild replay, with host-selected limits,
before publication or allocation. Preserve existing v1/v2 behaviour. Source
tests in this staged file have not run and are not execution evidence.

Reviewed relevant implementation in `qualia-core-db`:
`specialized_libs/computational_geometry/indexed_tetra_surface.rs`,
`tapered_branch_field.rs`, `surface_topology_qualification.rs`, and `topology.rs`.
Poet seams reviewed: `native/src/graphical/geometry_host.rs`,
`authoring/controller.rs`, `authoring/botanical_session.rs`, and `execution.rs`.
The earlier handoff records source pins; re-pin exact source before promotion.
