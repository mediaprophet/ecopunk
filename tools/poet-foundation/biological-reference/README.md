# Bounded native turtle admission — staged for review

**Current result:** the reviewed source is now installed and verified. See the
[executed delivery report](../../../docs/qa/implementation-2026-10-26/biological-reference/execution.md).
The text below preserves the original stage boundary before installation.
Three independent artistic branching command fixtures are retained in
`fixtures/`; their labels do not assert biological development or species form.

2026-10-26. Staged source only; no sibling source writes, Cargo build/test, GUI launch, biological model generation or species acceptance. `rustfmt` passed. The [manifest](manifest.json) pins pre/post bytes and confirms that the live targets matched the recorded preimages when captured. Root review, installation and actual execution remain separate work.

## Source and interface

The staged [`geometry_host.rs`](after/poet/native/src/graphical/geometry_host.rs) admits only the additional exact target `ComputationalGeometry.turtle_3d`, after its existing runtime target-grant check. [`turtle_host.rs`](after/poet/native/src/graphical/turtle_host.rs) performs borrowed-input admission and conservative cost accounting; it has no geometry implementation. Dispatch remains:

```text
SourceSession + actual operation grant
  -> GeometryHost::capability_invoke
  -> turtle_host::admit
  -> PoetSnapshot::capability_invoke
  -> Core wave27_host::turtle_3d_host
  -> Core computational_geometry::execute_turtle_3d
```

The Core host/kernel are consumed as-is from `C:/github/qualiaDB`. Their implementation was read, not copied into Poet.

Input is a record with exactly `commands` plus optional integer `ring_slices` (default 8) and integer `capacity` (default 8192). No list-root alias, unknown fields, missing command arguments, numeric-string coercion or extra row arguments are admitted by this new diagnostic profile. Counts must be integer Values, not floats. Command tags and exact row shapes:

| Tag | Arguments |
|---|---|
| `F` | positive length, optional positive top width |
| `f`, `!` | positive move length / width |
| `+`, `-`, `&`, `^`, `\`, `/` | signed finite angle in degrees |
| `T` | finite x/y/z direction and elasticity; direction nonzero |
| `$`, `[`, `]`, `{`, `.`, `}`, `G`, `g` | none |

All numeric magnitudes are at most 1,000,000 in this bounded profile; commands at most 65,536, ring slices 3–32. Stack depth is at most Core's 64; polygons record 3–32 vertices and all stack/polygon/cylinder scopes must close. Nested polygons/cylinders are refused. Core's generalized-cylinder active flag is global (only its previous-ring pointer is saved by push/pop), so fixtures must not assume nested `G/g` restores an outer cylinder. Separate `F` segments are the safe early branching diagnostic.

Before any Core allocation, checked arithmetic charges capacity-sized Point3/triangle buffers, both nested Value output streams, parsed TurtleCmd storage and a 64 KiB fixed allowance against the host scratch budget. A conservative output bound assumes two rings and two triangles per slice for every `F`, even where cylinder welding would save vertices; closing polygons adds their recorded vertex/triangle counts. Both output bounds must fit capacity, and capacity cannot exceed the host vertex limit or u32 indexing. Command/tessellation/materialization/zeroing work and actual dispatches consume the existing cumulative host counters. Excess or malformed input returns E400; missing actual operation grant remains E300. No source-selected limit increase is introduced.

Output is Core's existing `{vertices:[[x,y,z],…], triangles:[[u64,u64,u64],…]}` record. Although its wrapper comment mentions `vertex_count`, the inspected implementation's mesh constructor emits only those two fields; the patch preserves the real interface. This shape feeds existing native mesh preview admission. It carries no invented taxon, development simulation, rig, material, release acceptance or grow-tree GUI.

## Supplied verification, not run

Seven [`turtle_host_tests.rs`](after/poet/native/src/graphical/turtle_host_tests.rs) cases cover actual Core branch/tropism/welded-cylinder/leaf execution through SourceSession and preview, denied dynamic targets, capacity/tessellation/scratch limits, exact malformed input and scope rejection without source promotion, stack/polygon maxima, nonfinite direct-host inputs, and cumulative work/dispatch counters with explicit reset. The positive case checks known output topology and actual coordinates. Negative cases do not invoke a substitute kernel. HMC fresh-process replay and actual operator/reviewer work remain required qualification after this narrow seam passes.

Each staged Rust file is below 500 lines. No installed file or dependency was changed. The existing R9 apply script restricts groups beneath its own `round9` directory; this sibling stage requires a reviewed installer that explicitly admits this path and verifies the same preimages. Do not weaken its path check or overwrite a changed live source.
