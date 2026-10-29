# Staged botanical operation budget policy

`policy.rs` is an isolated, uncompiled prototype, not a Poet change or an
admitted v3 profile. It does not change v1/v2 limits or current behavior. The
associated tests are source tests only and have not been executed.

The policy models a **separately named, host-selected** structural-work budget
for a single inspection, edit or rebuild. Multiple regenerations within that
operation reserve against one ledger; independently initiated operations start
fresh ledgers. A reservation checks all arithmetic, work, dispatch and peak
memory before changing counters. It reserves extraction and topology probe caps
in full, so it does not depend on extraction returning actual probe counts.
Peak bytes are a per-operation high-water requirement, not a cumulative sum;
the caller must include retained source/session and all simultaneously live
buffers. Failed kernel calls after a reservation do not refund it.

The colonization formula counts conservative structural visits: initial
attractor-by-seed scan, per-iteration attractor-by-node association and kill
scans, node-by-node duplicate scans, plus linear bookkeeping. It is an
abstract accounting convention, not elapsed time, machine instructions, a
certified bound for all implementation changes, or a proof of cancellation.
Numerical operations, tree construction, extraction, topology,
serialization and projection still require owner-reviewed checked formulas
and consistent weights; `extraction_fixed`, `topology_fixed`,
`source_and_projection`, `kernel_dispatches` and `peak_bytes` are trusted
caller-supplied placeholders until those formulas and memory ownership are
implemented. In particular, field sample/segment counts alone omit the cost
of creating sample positions. Do not wire this prototype as a production
resource guard or assume arbitrary caller numbers are safe.

The inspected `pipe_model_radii` implementation has two fill passes, child
count, tip initialization and final radius pass, and at most `n` topological
passes scanning `n` adopted nodes; the prototype charges `n² + 5n` structural
visits with checked arithmetic. It separately calculates the exact element
payload for its `n` initialized `u32` scratch entries and `n` `f64` radii.
This is not a bound on the time taken by floating point `powf`, allocation
overhead, or surrounding graph work. The inspected Poet adapter sends only the
adopted prefix into this pipe operator.

For the grid extractor, the prototype independently mirrors the inspected
Core `tetra_surface_bounds` counts and offers a checked partial payload size for
sample positions/scalars, maximum output vertices/triangles, provenance pairs
and edge slots. The real host must call the Core bounds owner, supply the exact
Core slot type for `size_of`, and account for initialization work and all
simultaneously live memory. This calculation omits topology buffers, source,
retained values, intermediate clones, allocator and renderer overhead; it is
not a complete memory preflight. Extraction fixed work and serialization remain
unresolved. Equality of formulas or tests is not evidence of execution.

To integrate, identify all live buffers and checked sizes (including source
strings and Vibe values), establish and review the fixed work formulas against
the exact owner implementations, and reserve before allocation or mutation.
Pass one operation-scoped ledger through prior and candidate generation,
submission validation, and any additional work, including Vibe dispatches if
they share the chosen unit. A rebuild must share its ledger over source-cell
replay and final botanical validation; inspection gets its own bounded ledger.
Retain source/selection/epoch and authority checks, no-refund failure behavior,
and atomic publication. Specify how cancellation and stored v3 replay behave
when host-selected budgets change. Maintain old v1/v2 behavior unless an
explicit compatibility migration is approved. New limits should not be
presented as production-qualified before test, performance and memory evidence.

The draft tests cover repeated reservations, separate operation lifetimes,
atomic resource refusal and integer overflow. They do not exercise host wiring,
allocation, Vibe, HMC, GPU, cancellation or authoring acceptance. Tests for
those cases are required before adoption, along with proof that each workload
charge derives from trusted limits rather than authored values.

Inspected sources (SHA-256):

- Poet `native/src/graphical/execution.rs`: `83AA85FA508A5FA8377D4987D5A2AEFD54ABF125AB3B851B98B713E8D121163D`.
- Poet `native/src/graphical/geometry_host.rs`: `63C3CD4C115073157040D51387F28001EF8CA34EE1B5B8EC6411667959DC467E`.
- Poet `native/src/graphical/authoring/controller.rs`: `7137BDD7EE875736471E7F7FC5C72BB2683A3B55E435A44AAEE4D36A1F46E669`.
- Poet `native/src/graphical/authoring/botanical_session.rs`: `B4C6DE89980BFBC9A70A4449E6B596422406769A9A64AC9674EB01BDFC8EE1DC`.
- Poet `native/src/graphical/botanical_graph_host.rs`: `B360DE5701E2620540419A79483D2EFB0081175249C229683BD01E7CEF8CAD85`.
- QualiaDB `crates/qualia-core-db/src/specialized_libs/computational_geometry/space_colonization.rs`: `5C26DDACD9F558F40D9FDA16DA5FE1B35A13609183BD7AB0BCD47E181BFCB2FE`.
- QualiaDB `crates/qualia-core-db/src/specialized_libs/computational_geometry/tree_skeleton_3d.rs`: `9F5FA03B660CB064F19AF95A4F5F5780EA41BE021C04018E954A1690A86FD8F7`.
- QualiaDB `crates/qualia-core-db/src/specialized_libs/computational_geometry/indexed_tetra_surface.rs`: `3C062EBD250CEB45F3C8B76ADD5576E069CCC553DC626554CE0526298B9FBE12`.
