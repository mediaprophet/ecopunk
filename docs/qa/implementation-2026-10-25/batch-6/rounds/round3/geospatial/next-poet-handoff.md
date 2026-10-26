# Next native Poet georaster inspection slice — read-only handoff

Status: source inspection and proposed next work only, 2026-10-25. The current
Core/geospatial/Poet qualification run belongs to the supervisor; no result is
inferred here. Exactly eight source files were inspected, listed below. No
current/frozen source, asset or fixture was changed; no build, test, Computer
Use, credential access or provider call was performed. Only this new handoff
file was written.

The smallest useful next slice extends the existing native **HMC resources**
panel with a digest-bound, read-only georaster inspection result from the same
canonical owner. The inspected route already lists and preserves resource
payloads; it does not currently return decoded payload details. This is an
observation of these implementations, not a claim that every Poet inspector or
spatial capability was searched. The panel definition was inspected; mounting,
launching, graphical usability and actual contributor/reviewer work were not.

| Inspected source | Existing behavior and evidence |
|---|---|
| `C:/github/poet/native/src/graphical/package_controls.rs:44` | `PackageControls` implements the native HMC panel. `Inspect resources` submits `Action::Inventory`; results carry a `Context` and are withheld when source/session/authority or publication is stale. The panel shows package SHA and each resource's key, kind, payload bytes, metadata bytes and payload SHA. It offers `Save accepted source into this HMC`, guarded removal, and import/replacement from the selected native library. |
| `C:/github/poet/native/src/graphical/package/mod.rs:136` | `ResourceImportHost` forwards inventory to its inner `EditorHost`. `import_package_resource` checks current source, reserved keys/kinds, preflight, package SHA and old resource SHA before library access; `ReplaceBytes` preserves the old kind and opaque metadata. New resources require explicit kind. |
| `C:/github/poet/native/src/graphical/package/plan.rs:36` | `apply` validates the whole 1–64-entry logical plan before library IO, checks all expected resource digests, spends one aggregate import budget, rechecks source/preflight, and invokes one canonical `edit_package_resources`. It is reusable for a future guarded georaster replacement; it is not a geospatial compiler. |
| `C:/github/poet/native/src/graphical/cooperation/editor.rs:85` | `AuthorizedEditorHost::package_inventory` wraps the inner call with `with_authorized`, expected source and `AuthoringAction::ReadSource`. `resource_edit_preflight` requests `SaveCheckpoint`; import/edit/save operations delegate through `write_package`. No principal supplied by a UI result is used as authorization here. The implementation of `write_package` was outside this eight-file inspection. |
| `C:/github/poet/native/src/graphical/checkpoints/resources.rs:87` | `CheckpointService::inventory` loads with configured limits, checks the source/domain binding and immutable-release pin, then returns only `PackageInventory { digest, resources: checkpoint.inventory() }`. Its comment explicitly says listing does not decode arbitrary resources in UI. |
| `C:/github/QualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_store.rs:22` | The canonical `studio_store::checkpoint` API is routed to `studio_checkpoint/mod.rs`. This is the API imported by native Poet, distinct from historical QualiaDB Poet UI source. Only the module routing/reference was inspected in this file. |
| `C:/github/QualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_checkpoint/resources.rs:84` | `Checkpoint::resources()` exposes detached resource bytes to the native owner; `inventory()` returns digests/counts only. `with_resources_from` retains exact payloads, opaque metadata and source metadata for the same source/domain under limits. `apply_resource_edits` is detached/transactional, enforces old digests and budgets, reserves the source record, preserves kind/meta for `ReplaceBytes`, validates Q42 additions and invokes the existing dependency validator after edits. That validator's implementation was not inspected here. |
| `C:/github/QualiaDB/crates/qualia-core-db/src/poet_host/native_authority/package.rs:33` | `package_write_evidence` returns actor/session/admission/authentication/authority-generation/source-bound package write audit through the current session. Its contract explicitly says these records do not grant package review authority or constitute complete HMC acceptance. |

Persistence already has a bounded path in the inspected native checkpoint
service. `save_with_resource_edits` requires a current owner guard, checks
immutable releases and expected existing package SHA, captures exact current
source, combines the verified retained resource base with explicit edits,
encodes/decodes under limits, rechecks source, and calls canonical guarded save.
Only after save does it replace the retained carrier. This retains unknown
payload and opaque metadata bytes; rendering/inspection must not reserialize
unrecognized resources. Portable external-writer and power-loss qualifications
remain those of the canonical save implementation, not a new guarantee here.

Proposed next implementation, subject to its own scoped source assignment:

1. Add one **selected-resource inspection** request beside inventory, carrying
   expected source, package SHA, resource key and resource SHA. Route it through
   the existing job/context freshness mechanism and shared `EditorHost` owner;
   extend required forwarding adapters coherently. Read permission must be
   checked before loading/decoding and again before publishing results. Reuse
   the current inventory authorization only if the host contract explicitly
   covers these resource metadata; `ReadSource` must not silently become a new
   grant to expose arbitrary payload content. Contributor/reviewer roles alone
   do not grant access.
2. In the native checkpoint service, perform the same bounded package load,
   source/domain/immutable pin checks and expected package/resource SHA checks.
   Borrow the selected `Checkpoint::resources()` entry; use the canonical
   `M64Reader::parse` with explicit byte/chunk/record limits and caller-owned
   interval scratch. Return a small typed inspection result rather than raw
   payload bytes to the UI. Parse only the selected resource; cancellation and
   resource-work limits must be explicit. An unknown payload remains listed,
   unchanged and explicitly unassessed; malformed declared M64 returns a visible
   failure. Do not infer a format or authority solely from kind/name.
3. Render the existing `M64Reader::georaster()` metadata: version, exact first-cell
   geographic anchor, independent origin elevation, source W/H, cell size, epoch,
   SRID/vertical-datum declarations, row direction, anchor/basis, edge and nodata
   policy, and checked record counts. Show declaration/precision limits. Legacy
   v1/v2 metadata absence must remain visible, rather than inferring resolution
   or datum. Source samples are absolute mm; origin elevation is not an offset.
   Padding is outside source extent; nodata is distinct from valid elevation.
4. Prove inspection across the existing guarded import/replace → save → fresh
   reopen route without rewriting unrelated HMC resources. Bind every displayed
   result to exact source/package/resource digests and current authority context.
   This read-only slice needs no new semantic JSON schema, geospatial simulation,
   duplicate state owner or standalone application. A later native georaster edit
   must invoke the canonical opt-in v3 writer and then the existing guarded
   resource plan; a label change or byte import alone is not authoring proof.

Required next evidence includes explicit denied/revoked/stale-context refusal,
wrong package/resource digest, malformed/unsupported M64, exhausted scan/scratch
budgets, cancellation with no package mutation, v3 save/reopen metadata equality,
legacy/unknown payload preservation, unknown CBOR metadata preservation, and
operator-visible failures. Distinct contributor/reviewer sessions must use real
current host authority and exact source/output lineage. Source acceptance and a
package-write audit remain separate from resource/HMC review; a whole-resource
review capability and accepted review record cannot be fabricated from the
existing source-review or save controls.

FND-05 remains open: no actual native operator import/inspection/edit/save/reopen
or contributor/reviewer work was executed by this inspection; no GUI/human
acceptance or complete HMC semantic/rights review is established. The proposed
read-only inspector can make persisted format facts visible, but does not alone
fulfil native source-to-output authoring, IP-03 C1–C8, scientific datum/physical
accuracy, complete models, production acceptance or FND-06 interoperability and
benchmarks. The next assignment should first trace the existing job dispatcher,
controller trait/adapters and current resource read/review authority contracts;
those were deliberately outside this bounded eight-file check.
