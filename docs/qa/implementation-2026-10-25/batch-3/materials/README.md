# Native Poet material authoring proposal

Bounded ENG-04/08, ECO-033/034/036/044 material authoring work for IP-03.
This proposal is **staged, not applied**. Parent-owned controller/source admission
integration and actual coherent native tests are required. Film, open-world game,
cartoon and complete HMC/model packages remain incomplete; all broad production
packets and IP-03 C1–C8 acceptance remain open.

The eight guarded files are listed in [apply-manifest.json](apply-manifest.json).
The new material adapter and controls are split by responsibility. Native UI
controls stage an editable Vibe draft; the existing explicit source submission
owns evaluation, revision acceptance, undo/redo and rebuild. No Computer Use,
window control, paid call, commit, sibling source write or new host capability
was used by this worker.

## Source and engine contract

The pinned 0.1 canonical Vibe `material ...: Material` declaration evaluates to
an existing `Value::Record`. `material_from_value` translates that value into
the actual native `MaterialRecord`, checks its exact fields, and calls public
`validate_material_bindings`, shared with MAT4 encoding and GPU admission.
The source result `({geometry: poet_geometry, material: poet_material});` is an
application projection over canonical Vibe values, not a private file codec or
new language syntax. Parentheses disambiguate a record expression from a block
statement in the pinned parser.

The initial diagnostic defines mutable `poet_geometry` and `poet_material`
aliases. Material edits create a fresh named declaration and assign the mutable
material alias; the geometry value remains unchanged. Name collisions visibly
fail canonical preparation. The UI requires the parent snapshot's
`material_edit_id: Option<u64>` gate, based on actual mutable bindings, valid
material values and coherent execution; source cannot grant itself eligibility.
Controls preserve an unsubmitted source draft and require explicit submission.
They are labelled as a new draft rather than a current-material inspector.

Accepted material information lives in the existing stamped source-cell nodes
`poet.native.submission/{:020}/text`. The canonical source journal owns undo/redo;
the same unchanged read-only geometry profile can rebuild it. No independent
material state store, replay effect or grants are added. Parent integration must
validate the evaluated result before local session/source promotion, and retain
source/runtime coherence on invalid values. HMC checkpoint ownership is separate;
retaining source cells does not qualify complete embodiment retention or export.

Material fields: nonzero integer `id`, required `colour_space: "linear"`, linear
RGBA `base_color`, linear RGB `emissive`, dimensionless `roughness`, `metallic`,
`specular`, `alpha_cutoff`, native `opacity_mode` (Opaque/Mask/Blend), and boolean
`double_sided`. All scalar ranges come from native MAT4 validation. This slice
selects canonical MetallicRoughness shading and preserves native defaults for
unexposed flags. The material name supplied by the canonical declaration is
display/source information; it does not establish physical or rights authority.

Bare geometry output preserves the existing caller-material compatibility path.
If either scene-envelope field is present, both must be present with no extra
fields; failure cannot silently fall back to caller material. Texture fields
and unrecognized physical properties are explicitly refused. `SurfaceRecipe`
exists but generates sRGB texture maps; maps are deferred until the actual
role/colour/texture admission is verified. `ThreeD.set_material` currently returns
a status record without owning a renderable scene, so it is not used as proof.
Preview is SDR working output; native emissive factors do not qualify HDR masters.
Existing caller-selected geometry/master budgets are unchanged.

## Evidence

The [focused harness](harness/tests.rs) uses actual current cached Vibe/core/render
rlibs, current production geometry/controller sources and candidate material/
preview sources. It neither rebuilds the core nor substitutes mock types. Five
material tests passed, covering real geometry generation, canonical material
declarations, persistent edits, deterministic rebuild from source, colour/range/
texture refusals, fallback compatibility and grant denial. See
[tests.log](harness/tests.log). Initial parser-disambiguation failure is preserved
in [initial-failed-tests.log](harness/initial-failed-tests.log); it was corrected
before the passing run. This focused harness is distinct from a coherent current
source Cargo build and does not compile or interact with native UI controls.

The opt-in GPU test uses actual `CanonicalEditorHost`, stamped source acceptance,
an authored scene preview provider and real `PortalGpu` render/readback. It
changes base colour, roughness, metallic and emissive independently in accepted
source cells. [pixels.log](harness/pixels.log) records actual execution status;
there is no adapter-absence success shortcut. Native UI interaction and human
visual/domain acceptance remain unassessed.

The original GPU run failed: base colour changed 295 pixels, roughness 211,
metallic zero; emissive was not reached. The concrete cause and guarded fix are
in [renderer-fix](renderer-fix/README.md). The correction remains unqualified
until the parent rebuilds the actual canonical core and repeats the same GPU
assertions. [verification.json](verification.json) pins the executed artifacts
and failed/passed scope; [commands.ps1](harness/commands.ps1) retains exact commands.
