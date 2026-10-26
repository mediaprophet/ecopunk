# Root resource-chain source review

## Final bounded closure

Both initial findings below are closed in the final staged source. The controller
resource field is now initialized inside `Self { ... }` and absent from the
positional constructor call. The staged surface validator scans materials only,
with no default PreviewLimits geometry re-admission or embedded NUL byte.

The historical preparation scripts are explicitly disabled by the existing
`PREPARATION-FROZEN.json` marker, entry assertions and `stage.put` guard. The
marker accurately identifies exact before/after images plus the manifest as
canonical inputs, and does not call the generators reproducible installers.
`prepare_install.py` assembles and hashes those images; it does not replay the
historical application-source substitutions. This disposition closes the prior
generator-drift finding without deleting the preparation history.

[Final review pins](root-resource-pins-final.json) bind these checks to the
53-target install manifest SHA-256
`0ee679a2e1024e909a3bb4cdb9f1db21d64808a631ff62dcf5a1145e82b220e8`.
All 53 staged image hashes match that manifest at this recheck. No compile,
runtime test, GPU execution or live sibling write was performed by this reviewer.
No remaining blocker from this bounded review prevents root's compile/check.

The authored native workflow is wired as
`native_project::tests::surface_workflow_tests::surface_profile_native_intents_texture_closure_review_and_rebind_reopen`.
Its source checks expected HMC SHA and old-epoch retirement; original typed
native profile/UV/material intents; actual encoded texture closure and native
`.10d`/Q42 witnesses; separate software contributor/reviewer decisions; retained
unknown bytes/metadata; and explicit resource rebind before reopening/rebuilding.
It uses `AssemblyNoPixels` and has no GUI/GPU acceptance claim. Its presence and
source review are not a test pass.

The added native helper test is
`native_revolution_kernel_requires_explicit_host_opt_in`. The creation path keeps
the legacy grant set unless `POET_NATIVE_REVOLVE_PROFILE=1`; source and actual
GeometryHost grants are still separately required. This recheck confirms the
authored branch and test, not environment execution or runtime authorization.

Initial review pins: [root-resource-pins-initial.json](root-resource-pins-initial.json).
Root integration is still being edited. No build, runtime test, GPU execution or
live sibling write was performed. Controls/intent/source edits are outside this
assignment; root's new native texture workflow test is not yet reviewed here.

## Initial findings, now resolved above

1. `integration/after/native/src/graphical/controller.rs:83` places
   `surface_resources:None` inside the `open_with_rebuild_limits(...)` positional
   argument list. The actual `Self { ... }` initializer lacks that field. Move
   the initializer into `Self`; the current source cannot compile.

2. Preparation scripts do not regenerate the current reviewed resource path.
   `prepare_common.py:23` recreates the initializer defect. `prepare_resources.py`
   emits `validate_value` by re-admitting assemblies with fixed
   `PreviewLimits::default()` and replaces the string spelling `\\0` with an
   actual NUL in Rust source. The current staged `surface_host.rs` avoids both
   defects with material-only validation and an explicit byte-zero test. Either
   update those generators to match the intended final stage, or label/preserve
   them as historical one-shot preparation that must not be replayed. A claim
   that these scripts reproduce the final candidate would currently be false.

## Reviewed chain

The intended production route holds the existing `ReadSource` authority callback
through HMC read and canonical installation. It checks the current project,
document, source revision/digest and requested SourceRef through the existing
cooperative owner. Filename and filesystem access remain in CheckpointService's
plain-file, bound-root path guard. The new service additionally binds artifact
identifier/domain, complete decoded HMC SHA-256 and immutable-release policy;
the package does not grant authority or restore a runtime session.

The service supplies an immutable bounded byte snapshot. Raw installation is
implemented only by CanonicalEditorHost. The other EditorHost wrappers forward
the narrow package request; their default `install_surface_resources` remains
unavailable, so deserialized bytes cannot bypass the outer authority chain.
Resource installation validates the active assembly, replaces the snapshot and
retires the prior execution epoch, invalidating previous frames/inspections.
Held sources can bind resources before explicit rebuild without executing source.

Canonical submit pre-promotion validation, explicit rebuild cell validation and
native MeshPreviewProvider preview all receive the same stored snapshot.
Compilation validates that snapshot and supplies canonical original-byte texture
dependencies to CompiledAsset/HMC closure. ReadSource and checkpoint source
guards retain their existing final current-source checks. The current staged
material-only surface validator adds no second fixed geometry ceiling; the
existing caller-selected provider/host geometry budgets remain in effect.

No further concrete source blocker was found in these assigned paths. This does
not qualify the unexecuted native workflow, whole-process budgets, GUI usability,
human review, complete graph/rights lineage or film/game/cartoon packages.
Assets remain HOLD and all governing readiness gates remain open.
