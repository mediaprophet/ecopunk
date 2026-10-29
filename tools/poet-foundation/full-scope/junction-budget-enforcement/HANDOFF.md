# Staged junction operation accounting

These staged copies are uncompiled, untested and uninstalled. The geometry
host copy must be reconciled against the current Poet source; the separate
`junction_budget.rs` depends on the staged junction source and generator.

The opt-in host operation has a separately named 64 million structural-work,
256-dispatch allowance; existing geometry retains its separate limit, and
`begin_submission` does not reset it while the operation is active. Total work
is bounded by both allowances together, not by a single shared counter. During
replay each junction reservation additionally debits the existing aggregate
replay work and dispatch limits, so a caller-selected smaller total refuses.
The
64 million figure is an unqualified diagnostic policy, not a timing claim.
`begin_junction_operation` returns whether it owns the scope; only that owner
ends it, on every success and error path. Replay needs
`begin_junction_rebuild` after `for_rebuild` so its total replay limits remain
in force. Host-only callers can use the provided closure scope; a controller
which also borrows the editor must provide equivalent cleanup. Panic-safe
controller cleanup has not been integrated.

`read_accounted` scans only primitive Vibe values with explicit node, depth and
logical-byte limits and charges traversal before cloning the junction source.
`serialize_accounted` reserves host-selected source work/logical payload before
calling the source codec. `generate_accounted` derives grid and field counts
from Core owners, reserves fixed loop visits, initialization work, full
extractor probe and topology work caps, and the worst extractor/topology vector
payload before invoking the staged generator. It refuses allocator capacity
above its precomputed payload cap. The source serializer's output check occurs
after String allocation, so it is not a hard allocation gate.

Controller adoption, candidate/previous generation, tentative validation and
replay must call these wrappers using the same mutable host. Replay wiring is
staged in `execution.rs` and depends on the separately staged session and Vibe
callback changes; nothing is installed. Strict handling of session/env clones, retained source, source-layer
snapshots, BTreeMap and allocator metadata, temporary Vec/String excess and
renderer allocations is still missing. Logical/cumulative payload accounting
must not be described as a process memory cap. Independent review found no
obvious omitted generator Vec but identified that worst-case topology sizing
and repeated charges may be too conservative for useful grids. Diagnostic
verification and profiling are required when authorized.

The restricted host's existing capabilities and tentative-binding validator are
being integrated independently. Preserve authority, source revision/CAS and
reconciliation semantics; no validation callback may pretend host effects have
been rolled back. Existing v1/v2 policies are not a completed aggregate budget.
The staged limits are bounded diagnostics, not accepted master-quality limits.
