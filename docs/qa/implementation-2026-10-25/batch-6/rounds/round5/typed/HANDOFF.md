# Round 5 selected `.10d` materials — staged, unqualified

Eight targets are pinned in `manifest.json`. Live QualiaDB/Poet siblings were
read-only. The only existing implementation target, `field_section.rs`, receives
four routing/re-export lines; its existing 717-line body is unchanged. New
implementation files are 109, 144, 158 and 291 lines. The staged test files are
separate. No assets, credentials, commits, builds or tests were touched/run.
`rustfmt --edition 2021` completed successfully on the staged new module roots
and their children. This is syntax/formatting evidence, not a compile/test pass.

## Canonical APIs and integration

Root owns common module routing, authorizations, EditorHost/session/shared owner,
jobs, stale publication checks and native controls. Add `mod typed_10d;` and
re-export `Typed10dLimits`, `Typed10dEditRequest`, `Typed10dInspection` and
`ui_typed_10d_limits` from checkpoints. `Typed10dLimits: Default` selects the UI
profile. The exact service APIs are:

```rust
inspect_typed_10d(
    &self, request: ResourceInspectionRequest,
    expected: &SourceRef, limits: Typed10dLimits,
) -> Result<Typed10dInspection, String>

edit_typed_10d(
    &self, request: Typed10dEditRequest, expected: &SourceRef,
    limits: Typed10dLimits, cancelled: &AtomicBool,
) -> Result<SaveReceipt, String>
```

The request has `resource`, `source_digest`, `material_id`, `base_color: [f32;4]`,
`emissive: [f32;3]`, `metallic: f32` and `roughness: f32`. Factors are linear.
They describe visual rendering and do not claim physical material properties.
Use the inspected source digest in the request; do not substitute a source-text
CRC or an output digest. Every inspection/edit checks the current SourceRef,
canonical authored-state SHA-256, saved source SHA-256 and exact package/resource
digests. Kind must remain `10d`; selected resource metadata must be absent.
The metadata refusal is deliberate until a qualified derivative contract exists.

Inspection exposes request, saved_source_revision, source_digest, outer version,
vertex_count, index_count, decoded materials/ranges, typed fields, optional frame
layout and edit_blocker. UI should label its source digest as an association
guard: it is not evidence that this source regenerates the selected `.10d`.
An outer job publication must also retain its current actor/source/inventory
guards. Hold current read or package-write authority across the relevant call;
the service does not manufacture permission from any file/request.

## Preservation and supported scope

Canonical `inspect_opaque_container` plus the strict section table establishes
CRC/bounds/alignment first. Canonical mesh/material/MID2/frame/UV readers supply
semantic data; Poet does not implement a binary codec. Canonical
`replace_known_section_bytes_preserving_layout` replaces the same-length MAT4
payload and its descriptor/whole-file CRCs. It neither appends a second material
section nor rewrites the container table/geometry. Original MAT4 must regenerate
byte-exactly before factors change. MAT1–3 inspect but never silently upgrade.

The supported edit section set is QuantizedMesh, Materials, FieldSidecar,
ManifoldIdentityV2 and TextureCoordinates. Other known dependent sections hold
edits. Unknown outer section tags, unsupported MID2 extensions, ambiguous/overlap
bindings, invalid frames/UVs, unclaimed sidecar bytes, unknown flags/metadata,
opaque tail and noncanonical material bytes refuse edits. All unrelated supported
sidecar bytes stay exact, including normal/tangent and scientific fields.
MID2 external_digest also holds edits: its relation to changed output is unknown.
Morph and electromagnetic payload schemas are retained for inspection where the
envelope permits but are outside this edit profile and hold edits.

The new Core-only public `field_section::inspect_field_section` is allocation-free
and reads unaligned LE bytes. `FieldInspectionLimits` selects maximum payload,
field and sample counts; the result records fields/grid_fields/samples/type_mask.
It checks the existing FIEL declaration grammar and contiguous scalar/region/grid
raster writer profile, enum/reserved bits, exact coverage, width/height/count,
bounds, budgets and finite samples. Unknown sample formats refuse. Typed scalar,
vector, tensor and spectral bindings require a homogeneous matching FIEL kind.
FIEL unit hashes and MID2 registry identifiers remain distinct declarations;
this work does not assert an authoritative mapping or scientific accuracy.

## Source/output lineage refusal

Canonical `studio_checkpoint/graph.rs::key` emits `graphs/{asset_key}.q42`.
Canonical authored compilation emits source and compiled-digest facts, and HMC
retains that Q42 alongside `.10d`. An unchanged Q42 replacement after changing
MAT bytes could retain a stale compiled digest. The service therefore refuses
direct edits to **any package containing a `q42` resource**, conservatively also
covering graph schemas/keys it cannot prove independent. It never creates an
unchanged companion edit to bypass the existing dependency guard. Use the
authored-source factor workflow and coherent compiler output for these packages.

For an admitted standalone resource, a successful edit is explicitly a visual
resource derivative. The existing canonical HMC transaction retains the exact
previous package as parent digest, source, unrelated resources, kinds and
metadata. It does not assert that untouched authored source can reproduce the
new resource. Root's authorized package audit/review layer must record the
before/after operation and invalidate package review/publication views. No human
acceptance or material/scientific/source-generation claim is established.

## Required execution by supervisor

Three Core tests are staged under `container_10d::field_section::checked::tests`:

- `checked_fiel_grid_unaligned_and_declarations_are_borrowed`
- `checked_fiel_refuses_reserved_shape_offsets_nonfinite_and_tail`
- `checked_fiel_budgets_precede_large_or_late_payload_work`

Eight Poet tests are staged under `graphical::checkpoints::typed_10d::tests`:

- `typed_10d_material_delta_preserves_frames_science_geometry_and_every_unrelated_byte`
- `typed_10d_corrupt_limits_cancel_and_nonfinite_refuse_without_mutating_input`
- `typed_10d_hmc_save_reopen_retains_unknown_resource_and_changes_selected_material`
- `typed_10d_stale_source_package_resource_and_opaque_metadata_leave_hmc_unchanged`
- `admission::typed_10d_rejects_malformed_science_overlapping_bindings_and_unknown_metadata`
- `admission::typed_10d_legacy_mat3_inspects_without_silent_upgrade_and_external_hint_holds_edit`
- `admission::typed_10d_unknown_extensions_and_mismatched_scientific_kinds_refuse`
- `admission::typed_10d_no_sidecar_material_mesh_and_empty_declarations_refuse_without_hmc_mutation`

Independent review found an initial policy placement defect: the per-field
binding check ran only when FieldSidecar was present. A legal generic MID2 field
could instead reference Materials or QuantizedMesh with no sidecar. The final
staged policy validates every declaration before optional sidecar processing and
refuses non-FieldSidecar and declaration-only bindings. The added negative case
covers all three forms at payload and actual unchanged-HMC boundaries. These
regressions remain unexecuted pending the supervisor's serial command.

Root should also qualify actual authorized owner/job/native-control routing,
compiled `.10d`+Q42 unchanged-on-refusal, source-based factor rebuild, cancelled
publication and same-source package review invalidation. No new HMC/application
codec or private Q42 rewriting is supplied. All FND-01–06 and IP-03 C1–C8 remain
open; assets stay on hold. No model/package completion is claimed.
