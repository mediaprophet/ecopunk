# Profile/CAD candidate cross-review

Source-only independent review of the twelve entries in
`../profiles/target-manifest.json` on 2026-10-25. All twelve stage hashes matched
the manifest when checked. No Cargo builds/tests, kernels, GPU, GUI or model
production were executed for this review.

No blocking source issue was found in the staged profile/CAD-owned files within
their declared bounded scope. This is not execution or human acceptance.

- Native `GeometryHost::capability_invoke` checks the actual target grant before
  dispatch. Registered catalogue IDs do not grant execution. New revolution and
  profile paths retain dispatch/work limits plus current native vertex, triangle,
  profile-point and scratch admission. Failed adapters return before charging a
  successful dispatch; parser/source/session promotion remains canonical.
- The revolution adapter uses the existing Core `revolve_profile` caller-buffered
  kernel. Inspected Core triangle indices and coordinate parameterization give
  outward radial winding for strictly descending heights. Station/segment bounds,
  finite positive radius, decreasing heights distinct at f32 output precision,
  checked counts, output-index checks and finite output bounds are consistent.
  The source test independently computes radial cross products and boundary edge
  counts; it remains unexecuted here. Open ends, no poles/caps/partial turns and
  positive-radius monotonic height profile restrictions are disclosed.
- Metric profile source construction reuses Poet profile tessellation and Core
  polygon validation; actual execution repeats native admission and reuses the
  existing sweep/cap triangulator. Strict recipe ingress and native argument
  whitelists refuse unknown fields/units. Source replacement invokes one native
  capability outside pinned Vibe 0.1 lambdas and preserves unrelated part fields.
- No new mid-kernel cancellation capability exists. Bounded adapters and existing
  worker cancellation/late-result refusal are separate owners. The revolution
  control/handoff explicitly disclose the limitation; no interruptible-kernel
  claim is made.
- The required topology-attribute refusal is root-owned in
  `authoring/assembly_source.rs`. Its conservative rejection of geometry fields
  beyond vertices/triangles prevents dropping authored UV/normal/skin/morph or
  unknown data. The staged profile test includes that required refusal.

One integration usability issue was reported to root separately: an identity
`uv_transform` part field blocked geometry replacement merely by existing. The
surface controls can reset a transform to identity but did not remove the field,
so an otherwise no-op reset could permanently block control-based replacement.
Root owns the repair to distinguish parsed identity from a real transform and
retain/refuse actual topology-bound attributes. This observation is outside the
twelve profile/CAD-owned targets; no profile files were edited by this reviewer.

Required remaining verification includes the authored positive/negative grant,
work/scratch/point, source-preservation and winding cases; native authority,
source/epoch/cancellation guards; actual source-to-output/HMC/reopen and separate
review; GPU output and native usability. All IP-03/FND gates, complete models and
film/open-world-game/cartoon target acceptance remain open. Assets remain HOLD.
