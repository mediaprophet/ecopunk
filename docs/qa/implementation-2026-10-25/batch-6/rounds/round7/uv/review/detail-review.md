# Independent texture/detail source review

**No remaining blocking source finding in the declared bounded profile after
the two corrections below.** Eight current staged target hashes matched the
worker manifest; exact reviewed hashes are in
[detail-reviewed-pins.json](detail-reviewed-pins.json). Review used read-only
source inspection and preimage/candidate diff. No Cargo, test execution, UI,
sibling write, credentials, provider or production-asset action was performed.
These findings do not establish compilation, runtime acceptance or usability.

## Corrected review findings

1. `DetailRecipe::validate` originally charged paint image visits only inside
   the layer loop. A valid imported recipe with no authored layers therefore
   admitted the full base copy/composite/PNG operation with a custom zero-work
   budget. The final `budget_plan` charges four baseline image passes before
   the loop and three further passes per layer, then conservative full-image
   dab visits. `validate` delegates to that same plan. A new zero-layer budget
   regression is authored. Static reinspection confirms the bypass is closed;
   the supervisor still needs its actual execution result.
2. `DetailControls` originally compared only accepted recipe equality. Switching
   between parts with equal recipes (including no recipe) could leave an unsent
   local draft eligible for the new part. Final controls require a root-supplied
   `context_token` to match the explicitly loaded context in addition to current
   recipe equality and freshness. Explicit reload/start records that token;
   local drafts remain visible when the token changes. Root must bind the token
   to actual selected part and source context. It remains a UI stale-draft guard,
   not an authority grant or replacement for root session/source checks.

## Canonical alpha and compatibility

The focused Core helper computes straight RGBA8 source-over: source alpha times
coverage, destination alpha, their combined alpha, and alpha-weighted straight
colour divided by combined alpha. It applies stroke and layer opacity once.
The brush route retains unattenuated straight colour on transparent layers;
document composite no longer attenuates that colour a second time. The hardness
endpoint now uses `<=`, avoiding the zero denominator at hardness one. Public
consumer cases cover a transparent red dab, compositing it over black, layer
opacity, intersecting transparent strokes, source alpha, visibility and the
hardness-one edge. Source review agrees with their expected math; they remain
unexecuted regression definitions.

The old image owner also exposes blend modes, masks and offsets whose semantics
are not implemented by its existing compositor. The staged bounded Poet recipe
cannot author those fields and strict unknown-field admission refuses them.
The patch changes only the canonical brush/composite alpha calculation and its
hardness endpoint; it does not claim to repair these other image-owner features.
Encoded-sample arithmetic and byte rounding are explicit. No linear-light or
ICC-managed compositing claim is justified or made.

## Bounds, source and transfer semantics

The recipe validates square side 8–1024 before deriving pixel sizes. All pixel
construction in `render_detail` follows validation of explicit layer/stroke/
point, working-byte, output-byte and pixel-visit limits. Point work charges the
whole image per dab, so clipping does not weaken admission. The shared
`budget_plan` returns the same charged cost root can aggregate across a project.
Paint input requires exact side/byte length, opaque pixels, admitted digest and
explicitly interpreted sRGB. The source never selects a filesystem/network path.
Original borrowed pixels are copied rather than mutated.

`DetailRecipe::from_value` bounds recursive conversion by depth/nodes/list and
record counts and string lengths, then strict serde and semantic validation.
Version, unsupported blend/mask/extension fields, nonfinite coordinates and
out-of-range controls refuse. Source generation validates first and emits only
the schema's quoted values and record/list expressions, capped at 48 KiB. The
recipe digest witnesses authored recipe bytes; base digest identifies retained
encoded source, and derived output digests identify the actual new PNG bytes.
These hashes establish neither ownership nor authorization.

Pixel generation and PNG encoding reuse the existing canonical
`render::surface_authoring` functions. That PNG encoder does not write a transfer
chunk; decoded output can therefore have metadata `Unknown`. The existing Poet
`TextureRole` contract explicitly chooses sRGB for base colour and linear data
for normal/ORM/AO, and resource admission rejects explicit opposite metadata.
Root must preserve that role interpretation and retain the original encoded base
dependency. No silent claim of embedded sRGB metadata is appropriate. Procedural
normal/ORM values and channel roles match the canonical generator: tangent normal
RGB, roughness G, metallic B, and AO R=255. Sharing the same ORM dependency for AO
and metallic/roughness is intentional, with one retained encoded resource.

The fixed 8 MiB inherited PNG encoder scratch allowance is documented but has
not been instrumented here. Whole-workload peak-memory/cancellation/latency
qualification remains open. Current caps and source accounting are not a measured
peak-heap claim. Root owns native role/material binding, aggregate budget
admission, actual GPU results, source/revision/authority validation, HMC resource
union and durable contribution/review/save/reopen tests. Controls' numeric
discrete dab coordinates make no pointer-painting or 3D-painting claim.

Assets remain on hold; no model/package, human acceptance or production-quality
completion follows from this review.
