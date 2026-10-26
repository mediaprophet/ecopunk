# Round 7 editable texture/detail stage

**Staged, not applied or compiled.** Eight Rust targets are pinned by
[manifest.json](manifest.json); one existing Core target has an exact preimage.
All writes were confined to this assigned directory. No Cargo, UI, provider,
production asset, live sibling, credential, commit or publication action ran.
`rustfmt --edition 2021 --config skip_children=true` passed for the eight staged
files. Six domain tests, one controls test and four public canonical Core tests
are authored and **unrun**. Supervisor owns target registration, assembly/resource
composition, authority/revision guards, actual GPU/HMC/review/reopen qualification.

## Source and operator contract

`surfaces::detail::DetailRecipe` is an internally tagged strict-serde `kind`
record. Both variants require `version:1` and square `side` 8–1024.

* `paint`: immutable `base_sha256` plus ordered layers (`name`, `visible`,
  `opacity`, `strokes`). Strokes retain `size`, opaque RGBA `colour`, `opacity`,
  `hardness`, and discrete image-coordinate `[x,y]` points (+Y down). Initial
  imported profile is opaque base colour, exact extent and explicitly interpreted
  sRGB samples. No implicit resizing, path interpolation, 3D painting, blend/mask
  support or linear-light claim.
* `procedural`: exact canonical `SurfaceRecipe` fields `seed`, `tint`, `contrast`,
  `relief`, `bands`, `band_strength`, `roughness`, `metallic`. Core generates actual
  colour/tangent-normal/ORM pixels. Artistic roughness/metallic/relief controls do
  not invent measured physical properties or imported-height baking.

Public helpers: `DetailRecipe::{from_value,validate,budget_plan,side,source_expression,digest}`;
`render_detail(&DetailRecipe, Option<DetailBase>, DetailLimits)` returns
`DetailOutput {overrides, dependencies, base_digest, recipe_sha256}`. Overrides
are typed `(TextureRole, digest)`. Paint overrides base colour; procedural detail
overrides base colour, normal, metallic-roughness and AO, with three unique PNG
dependencies (ORM supplies both last roles).

`DetailBase` receives borrowed bytes from the existing authorized native HMC
resource owner: `digest`, `width`, `height`, `rgba`, `colour_space`. It requires
`TextureColorSpace::Srgb`. Canonical metadata `Unknown` must be explicitly
interpreted by the root's admitted base-colour policy before constructing it;
embedded transfer metadata and role interpretation remain distinct facts.
The root must retain original encoded dependencies, recipe-bearing source,
new derived dependencies, original resource keys and metadata in actual HMC.
The recipe digest is a source witness and grants no authority.

The recipe adapter validates before pixel construction: maximum eight layers,
128 total strokes, 4096 total points, default 128 MiB working admission,
16 MiB output admission and 128 Mi pixel visits. Work estimates conservatively
charge complete images per dab rather than only brush rectangles. Source
expressions refuse above 48 KiB. These are initial working scopes, not asset
quality ceilings. The memory plan includes an 8 MiB PNG scratch allowance over
caller buffers; the inherited canonical encoder's internal allocation was not
instrumented in this stage. Whole-workload peak qualification stays open.
`budget_plan` returns the same calculation as `validate`, as `DetailCost`
(`working_bytes`, `output_bytes`, `pixel_visits`), for root aggregate admission
before any per-material rendering. Imported-base baseline passes are charged
even with zero authored layers; a zero-work regression covers that case.

## Canonical alpha defect and correction

Inspection found that `BrushStroke::apply_to` wrote already-attenuated RGB into
transparent pixels while `ImageDocument::composite` multiplied by alpha again.
A 50% red dab could therefore display approximately 63 red over black instead of
128. The staged correction adds focused `image_source_over.rs` and thin routes
in `image.rs`, preserving straight RGBA8 throughout brush and layer composition.
Source colour alpha, destination alpha, stroke opacity and layer opacity are
applied once. The hardness=1 boundary also avoids a zero denominator.

The initial inventory's proposed opaque-snapshot workaround was superseded by
the supervisor's requirement to repair the canonical owner. Paint now uses
normal transparent source-over layers over its admitted opaque imported base.
Existing unsupported blend/mask/offset semantics remain outside this bounded
Poet recipe and are not claimed fixed. Working-space arithmetic remains encoded
samples, with byte rounding; this is an explicit corrected compatibility change.

Public `tests/image_source_over.rs` covers transparent straight colour, the
display over opaque black, layer opacity once, intersecting strokes, input colour
alpha, visibility, opaque legacy pixels and full-hardness boundaries. Root will
mount it as `foundation_image_source_over` against the actual native consumer.

## Native controls and verification still required

`DetailControls` has the approved props `detail`, `context_token`, `fresh`, `preview_uri`,
`onapply`, `oninspect`. Explicit load protects unsent drafts. Changes to accepted
recipe/selection context disable application until explicit reload. The root
provides the displayed-source/selected-part token for comparison only; it grants
no authority. Returning to the same context restores a retained local draft.
Layers and strokes
can be added, removed, reordered and revised; visibility/opacity are retained.
Explicit numeric point controls are used; framework pointer-coordinate semantics
were not qualified. The bounded accepted pixel preview is supplied by root's
native inspection job, never generated by hidden UI state.

Domain tests assert pinned Vibe source round-trip, unknown-field refusal, actual
canonical PNG decode after a paint edit, immutable input pixels, repeated replay,
visibility/reordering effects, digest/extent/transfer/alpha/budget refusal, and
generated normal changes plus ORM packing. Root must additionally exercise stale
authority/revision refusals, conflicting shared-material recipes, actual source
contribution/review, save and two fresh reopens, new-digest retention and preview
GPU effects. Those checks are not replaced by the local domain tests.

Asset production remains on hold; no model/package completion is claimed.
FND-01–06, IP-03 C1–C8 and full film/open-world-game/cartoon packages remain open.
