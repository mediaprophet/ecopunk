# Independent root resource integration review

Initial review of `round7/integration/after/poet/native/src/graphical`:
`surfaces/{texture_inputs,resolution,inspection}.rs`, `surface_host.rs`,
`editor_contract.rs`, and `authoring/controller.rs`. The root is still editing
these candidates; final source pins and repair disposition are pending its
freeze notice. No source edits, Cargo, UI or live sibling actions by this reviewer.

## Actionable findings sent to the root

1. **P1 — retain authored map closure before derived overrides.** Initial
   `resolution::resolve` records only imported paint bases in `original_inputs`.
   Procedural overrides, and paint whose selected base differs from the original
   authored base-colour map, can therefore leave original map digests in the
   editable source while dropping their dependency bytes from a newly compiled
   checkpoint. Collect every original nonzero `TextureRole::ALL` digest before
   applying overrides, resolve exact bytes/MIME with the canonical owner, charge
   the unique union, and retain it in the returned dependencies. A fresh save,
   reopen and remove-recipe case should prove restoration of all authored maps.

2. **P1 — charge complete merge peak before allocating.** Initial aggregate
   admission counts recipe peak, one snapshot, generated output and a single
   encoded allowance. `texture_inputs::merge` simultaneously retains caller
   snapshot/original-input/generated copies, clones every snapshot entry into
   `BundleWriter`, creates a complete output Vec, then copies that Vec into
   `TextureResources`' Arc. Multiple complete packages can coexist despite the
   192 MiB working gate admitting only one. Calculate all live phases, including
   retained source copies and actual metadata/index ownership, before decode or
   merge. An owned-buffer constructor could remove an intermediate copy.

3. **P1 — validate the same effective materials with actual provider geometry
   limits before promotion.** Initial `surface_host::validate_value` resolves
   detail and calls full assembly admission using `PreviewLimits::default()`.
   That inserts an unrelated 200k vertex ceiling and repeats atlas construction.
   Conversely, deleting that geometry admission alone would leave default
   `MeshPreviewProvider::validate` checking original materials: derived procedural
   maps could introduce a UV requirement after that check, failing preview only
   after source promotion. Override `MeshPreviewProvider::validate_with_resources`
   to resolve, admit with `self.limits` and effective materials, and validate
   effective resources. Keep generic surface validation geometry-independent.

4. **P2 — refuse inconsistent retained entry SHA instead of repairing it during
   merge.** The initial merge verifies a retained entry only when a new dependency
   uses its key. Every other retained entry is copied through `BundleWriter`,
   which recomputes its SHA. Canonical `BundleReader::parse` verifies whole CRC and
   entry bounds, not each entry SHA. An internally inconsistent entry with valid
   whole CRC can therefore be silently rehashed into a valid derived snapshot.
   Verify every copied entry before any writer insertion; exercise an inconsistent
   entry-SHA index with recomputed whole CRC as a refusal case.

5. **P1 — retain original entry metadata on fresh checkpoints.** Original
   `TextureDependency` contains only digest, MIME and bytes. Ephemeral snapshot
   merge retains source entry `meta`, but `compile_authored` returns only these
   narrow dependencies to `CompiledAsset`. Existing-destination union preserves
   prior metadata, while `ExpectedFile::Absent -> service.save` has no prior
   metadata to recover. Use the existing native resource transaction/retention
   owner for the fresh package, or explicitly refuse an unsupported preservation
   case. Include nonempty original texture-entry metadata in the fresh-save
   regression; payload metadata embedded inside image bytes is a separate fact.

## Other inspected behavior

The typed derived material/dependency route is shared by native preview and
source compilation. Shared material identifiers reject conflicting complete
material/recipe pairs; repeated equal declarations render only once. Derived
map digests are applied to a detached MaterialRecord, preserving authored input
records. Paint input resolves canonical digest/key/entry integrity, exact square
extent and explicit sRGB interpretation; embedded Unknown metadata is explicitly
distinguished from that interpretation. Linear transfer metadata refuses.

Selected inspection rechecks source before and after processing and returns the
selected part identifier; canonical controller checks current execution, while
cooperative wrappers enforce source-read authority and job delivery checks the
source/epoch context. Actual stale-selection UI publication still needs the root's
final controls/jobs review and runtime cases. The worker-produced raster preview
is bounded at 256 square; explicit map pixel inspection is distinct from GPU
material appearance and human acceptance.

Root repair/review disposition and exact final pins will be added after freeze.
The root subsequently assigned this reviewer the two-file preservation remedy
for finding 5; it is separately staged at
[`retention-repair/HANDOFF.md`](../retention-repair/HANDOFF.md), with its own exact
preimages/manifest. That remedy is not treated as independently reviewed or
executed by this report. Root owns composition and actual native regressions.

## Freeze 1 repair reread

Manifest SHA256 `1e368ffe65d60a03571a1b485652e87c69d849e32e345a49f52dd6831d92623f`
was checked against all 58 staged targets and live preimages, with observations
in [root-resource-pins-freeze1.json](root-resource-pins-freeze1.json). This is source
and hash checking, not compilation, test execution or permission to apply.

The first five source findings above are addressed by the frozen candidate:
all original authored map digests are uniquely precharged and retained; complete
non-source ResourceData records flow through exact metadata-aware native save;
merge verifies every entry SHA and rejects unsupported flags while conservatively
charging live copies/metadata; actual MeshPreviewProvider validates effective
materials with its selected limits before promotion; generic surface validation
no longer introduces a geometry ceiling. The metadata retention remedy was
authored by this reviewer and remains subject to the root/other reviewer's
independent check and actual native consumer regressions.

The bounded retention extractor **explicitly refuses** a bound native-source entry
whose `meta` is Some (including an empty blob), because replacing that source entry
requires qualified source-carrier metadata continuity. This preserve-or-fail
behavior is accepted for the current software scope. It is an FND-04 open
limitation, not a claim that all source/header metadata can be retained. Ordinary
non-source resource metadata is retained exactly. No metadata-framework expansion
is requested merely to avoid a safe declared refusal in this round.

One additional P1 admission finding remains in Freeze 1: selected inspection
retains resolved original-input copies and selected geometry while
`preview_input -> decode_any` charges only decoded RGBA, codec scratch and one
encoded image. A 4096-square RGBA PNG has canonical 64 MiB output and 64 MiB
scratch; in a 64 MiB bound snapshot with up to 16 MiB copied original inputs,
at least 208 MiB can remain live before geometry/pixel buffers, although the
local 192 MiB decode gate admits it. Add a combined caller-live inspection plan
or safe refusal before decoding, including snapshot/retained inputs/geometry/
pixel buffers/canonical scratch. This is a source-level counterexample; no
unmeasured runtime peak is claimed. Root was notified promptly.

Selected inspection otherwise now uses shared borrowed preflight, resolves and
constructs only the selected part, refuses ambiguous identifiers, clips unit-tile
edges correctly and obtains actual current derived/original texture pixels.
The root's complete Context/part cursor replaces ambiguous recipe-only selection
comparison. The required GPU/cooperative/reopen tests remain unrun at review time.

## Freeze 2 final source disposition

**No blocking source finding remains in this review's resource/closure scope.**
Final manifest SHA256
`87c893608a76cda2c1abb38bc6484a58a3ea63c0820d415a720282867b7f2605`
and all 58 staged target/live preimage observations are pinned in
[root-resource-pins-freeze2.json](root-resource-pins-freeze2.json). Compilation,
consumer/GPU/workflow execution and native human acceptance remain separate and
unrun by this reviewer. This is not independent authorship review of the two-file
retention remedy written by this reviewer.

Freeze 2 addresses the additional inspection admission finding: the inspector
subtracts the original bound snapshot, retained input copies, the larger of source
and expanded geometry/edge estimates, and three diagnostic image buffers from
the working cap before requesting texture decode. `preview_input` applies the
minimum of that remainder and its host limit. `decode_any` independently charges
its resolved snapshot, twice the opaque metadata bytes, decoded RGBA, canonical
codec scratch and encoded copy before RGBA/scratch allocation. These combined
checks reject the 4k PNG counterexample instead of allowing its hidden live peak.
The new bounded 8-pixel positive/negative remaining-budget consumer case is
authored; it is not asserted executed here. Estimates remain conservative source
admission, not instrumented whole-workload memory measurements.

The explicit refusal of source-entry metadata Some remains accepted preserve-or-
fail scope and open FND-04 work. Exact non-source resource metadata retention,
required source-carrier continuity and complete embodiment/graph/rights lineage
must not be collapsed into one completion claim. No additional metadata framework
work is required merely to retain this honest bounded refusal.
