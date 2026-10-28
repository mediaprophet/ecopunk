# PA-08 spreadsheet integration: traced seam and first executable slice

Status: source inspection only, 2026-10-27. No GridCraft source was changed and no build or test was run by this worker. This is a restartable handoff, not a claim that Poet has a working spreadsheet, that an HMC workbook format is adopted, or that source tests were executed.

## Existing owners and provenance

* GridCraft's `C:/github/poet-gridcraft/crates/engine/src/lib.rs` exposes `Session::execute`; its documentation identifies the existing egui UI, CLI, control channel and MCP as callers of this same journaled command route. `crates/engine/src/cmd/edit.rs`, `cell.set`, owns workbook cell parsing, protected-cell checks and recalculation. `crates/engine/src/tests.rs` contains source tests for formula edits, recalculation and protected sheets; their presence is not execution evidence. `crates/mcp/src/headless.rs` exposes the same engine session through its existing headless caller. The engine also has workbook load/save operations in `crates/engine/src/io.rs`; format support does not prove lossless round trips.
* QualiaDB `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/sheet/range.rs` exposes Vibe numeric-grid statistics and range sums. This is a related existing caller, not evidence of a workbook editor; reconcile it with the future common action rather than replace it speculatively.
* GridCraft is a separate source workspace and has no resolvable Git HEAD. Its source includes MIT notices, a `LICENSE-MIT` and `ATTRIBUTION.md`; workspace metadata declares MIT OR Apache-2.0. Preserve the applicable notices and inspect transitive dependencies and exact provenance before integration or redistribution. Hashes below pin only the inspected individual files, not all transitive source.
* Poet HEAD inspected: `3d167fdff42eb7fd913d998d786a0a39639b5d6e`; QualiaDB HEAD inspected: `69ec4ce5d499a271da74b74018631c388124f15e`. HEADs do not identify uncommitted changes. Verify the current working sources before adapting.

GridCraft SHA-256:

| File relative to `C:/github/poet-gridcraft` | SHA-256 |
| --- | --- |
| `crates/engine/src/lib.rs` | `86D9CB2FD8A81DF275742B950A15E9AEFEDA9182CDF2CDAB90DDF3042CB67FFE` |
| `crates/engine/src/cmd/edit.rs` | `01077E180839B3DC1CA20A9B3B4A56FBEF8FCEB3FB64CC033891F555C5C7172E` |
| `crates/engine/src/io.rs` | `BFB3A8BB4D06A41B7728394A0CDBD3700B66C69B2D86D058C2BBD41A91ACAB7F` |
| `crates/engine/src/tests.rs` | `8BA9FAFD903EA40C801AC3776B96CE765A15BB9108FBD00D6F469CCC9978D8E1` |
| `LICENSE-MIT` | `A87109BBB1AD43CA3920A5D7FF3806863B7E8470A73F4DED97A1447AC93333F5` |

## The actual Poet seam and unresolved admission

`C:/github/poet/native/src/graphical/poet_workspace/model.rs::WorkspaceProjection::for_source` mounts the existing Studio projection for a source and describes the spatial context; no spreadsheet selection, cell-edit surface or cell-edit intent was found in that inspected workspace. This is an observation about this mounted surface, not a claim that no spreadsheet code exists elsewhere.

`C:/github/poet/native/src/graphical/checkpoints/georaster_edit/mod.rs::CheckpointService::edit_georaster` demonstrates the existing authority and preservation pattern: outer `AuthorizedEditorHost` retains write authorization, source stamp and selected package/resource digests are checked, the typed payload is safely validated, then `save_with_resource_edits` replaces the selected resource with expected digest while preserving other resources. That route is specifically an M64 georaster editor, not a spreadsheet editor.

There is no traced binding from an admitted spreadsheet workbook HMC resource and native grid surface to GridCraft's session in this selected Poet source workspace. In particular, the inspected code does not establish a workbook resource type/profile, import limits, round-trip preservation for unknown workbook fields, or a human/agent session that commits through the authorized host. GridCraft's native model JSON serialization is not automatically an adopted HMC contract. Treating an arbitrary XLSX or JSON load/save as lossless could erase unsupported features; never silently rewrite such source.

## First vertical implementation slice

1. The shared-source coordinator traces existing HMC resource metadata, project capability/authorization, relevant domain-session contracts and previous spreadsheet execution evidence. Select an already-supported typed workbook resource if present; otherwise first establish a bounded native workbook resource profile, including ownership, schema/version, size and decode bounds, round-trip equivalence/unknown-field behavior, provenance and exact refusal conditions. Preserve original XLSX as source when safe preservation cannot be established. Decide capability through existing owners, not file extensions or actor-supplied identity.
2. Add GridCraft engine integration at Poet's existing dependency/adapter seam after provenance and source distribution are resolved; retain engine command ownership. Map a bounded selected cell edit to its existing `cell.set` command. Do not reimplement formula parsing, calculation or protection. Route human and authorized agent edits to one shared document session with explicit revision, invalidation and undo semantics; an ephemeral wrapper or a separate GUI does not meet this requirement.
3. Carry selected source identifier, package digest, resource digest, expected source/session revision and authenticated delegated authority through the existing Poet intent and authorized checkpoint host. On stale source, denied authority, unsupported content, serialization failure, cancellation or budget failure, make no package write. On success use the existing resource edit mechanism and record the resulting real revision and provenance. Do not fabricate `qualia-app-contract` receipt effects or treat its permissive adapter defaults as authorization.
4. Mount a native Poet grid using existing workspace and intent patterns: select workbook, display an actual cell and formula result, edit via the same command, show conflict/refusal honestly, reopen saved source, and share the authoritative result with the agent entry point. The API/headless path alone is not a native human workflow.
5. Add narrow tests for protected cells, dependent-formula recalculation, malformed formulas, bounded input, unsupported/unknown content, source/digest conflicts, denied authority, cancellation, preservation of unrelated resources, persisted reopen, and equivalent human/agent action. Source tests in GridCraft are inputs to this work, not proof of Poet integration. Run the coordinator's disk-guarded serial checks and record exact commands, results, revisions and acceptance evidence. Request human review for the real native contributor workflow before declaring the gate complete.

## Why no disconnected code is staged

The existing session can execute cell edits, but a direct call has no validated source resource, current digest or retained write authority. A fabricated local workbook envelope or unmounted adapter would create a second authority and falsely imply a working authoring path. The concrete implementation dependency is the existing selected HMC workbook resource/admission and its native intent + host binding, not a lack of a spreadsheet engine. This handoff confines the next work to those owner seams; it does not claim the full PA-08 scope is complete.
