# Independent Round 7 UV atlas source review

Reviewed the eight staged targets from `uv/target-manifest.json`; exact observed
hashes are retained in [uv-reviewed-pins.json](uv-reviewed-pins.json). All eight
matched their manifest at this review. This is source review; no Cargo, execution,
UI, live sibling or production-asset action ran. It does not certify target builds
or authored native usability.

**No blocking source finding identified in the declared explicit-seam,
oriented-disk, harmonic-solver, shelf-pack profile.** Acceptance still depends on
the supervisor's actual consumer tests and retained cooperative source/HMC path.

The corner disjoint-set cut follows source-index incidence, joins both endpoint
corners across retained oriented edges, and separately joins face components.
This handles a nonseparating cylinder cut within one face component; using chart
IDs alone would not. Output source-vertex mapping and face order/winding are
retained. Duplicate/reversed/absent seams, >2-face edges and inconsistent adjacent
winding refuse. Per-chart boundary successor/incoming counts, single-loop walk
and Euler characteristic one supply the disk admission proof after corner fan
splitting. Original source topology/resources remain the root's responsibility.

The implementation calls the existing canonical
`harmonic_parameterize_disk` once per chart with the complete iteration budget;
it does not introduce a replacement solver or claim conformality. It rejects
folded/unconverged triangles and positive-area overlaps. The overlap predicate
uses canonical orientation tests and a separating-axis test allowing touching
boundaries. A second pass checks delivered f32 UVs for collapse and overlap, then
recomputes actual delivered area-density extrema. Stable shelf packing reserves
separate outer pixel rectangles and refuses insufficient resolution/density,
rather than silently downscaling. No automatic seam, optimal packing, shape
distortion ceiling or uniform local-density claim is supported.

All caller output writes occur after topology, solver, pack, delivered-stream
validation and the final cancellation probe. Thus returned errors preserve all
public slices. Input/count/options, conservative scratch and work charges precede
allocation; wrapper overhead is separately subtracted. Cancellation occurs during
topology and pairwise validation and around each chart solve. Cancellation
latency within the inherited solver remains one complete admitted chart solve,
explicitly documented; this is bounded cancellation, not immediate interrupt.

The public/native cases include nonseparating cuts, separated charts, closed
surface refusal, explicit all-face cuts, winding/index/degeneracy/nonmanifold-edge
refusal, output capacity, memory/work/density refusal, cancellation including the
last probe, f32 collapse, and exact source/normal provenance. These are authored
tests awaiting execution, not passing evidence.

Nonblocking qualification note: `atlas_size` accepts arbitrary positive sizes,
while most retained fixtures use 256. The reserved integer rectangles are exact,
but division to normalized f32 UVs has ordinary rounding. A non-power-of-two atlas
fixture should quantify the subpixel tolerance before describing **delivered UV
coordinates** as preserving an exact pixel gutter. The worker subsequently added
a public 257-side/3-gutter case asserting delivered coordinates within 0.001 pixel
of reserved bounds and disclosed that tolerance in its handoff. Refreshed review
pins include that authored, still-unrun regression. This does not invalidate the
current claim of reserved outer rectangles. Required texture filtering/mip-padding
content beyond that reservation remains a separate authoring/consumer task.

Root integration must reject tangent/skin/morph/additional-UV/extension streams
before projecting richer geometry into this narrow Mesh API, preserve original
source UVs, retain remapped normal/source bindings, and regenerate tangent frames
from the delivered UVs when normal maps are used. The wrapper explicitly states
that precondition; the root's assembled source and resource path was not part of
these eight targets and is not reviewed here. The added owner manifest correctly
classifies Scalar/Tolerance/ApproximateMetric/ColdBounded and explicitly describes
Foundation as the intended post-compilation state, without creating host grants.
The final adapter delta also checks an already-cancelled callback before wrapper
allocation; refreshed pins cover that change. No new finding from that delta.
