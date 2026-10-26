# Round 7 — native UV unwrap/atlas and texture/detail editing

Started 2026-10-25; scoped verification completed 2026-10-26 (Australia/Sydney).
**58 distinct targets saved; 103 passing test executions / 98 unique cases.
Native build, actual SDK WASM compilation and final reconciliation passed.**
This bounded round is **complete_scoped**, not a foundation gate or model acceptance.
Asset production remains HOLD; all FND-01–06 and IP-03 C1–C8 stay open.

| Area | Saved implementation | Declared scope |
|---|---|---|
| UV unwrap | Canonical corner splitting at explicit seams; manifold/orientation/disk checks; existing harmonic parameterization | Explicit seams must produce disk charts; no automatic seam selection, conformal guarantee or distortion ceiling |
| Atlas | Deterministic shelf packing, pixel gutter reservation, requested area texels per local metre, measured delivered triangle density | Before part TRS; no implicit density reduction, optimal packing or mip-edge dilation claim; f32 delivery has subpixel tolerance |
| Native UV controls | Source atlas recipe, seam edge selection, resolution/gutter/density/iteration controls and actual current UV diagnostic | Stored source geometry/UVs remain intact; active atlas requires explicit removal of planar projection and reset of part affine UV |
| Paint | Immutable imported base plus editable ordered layers, visibility/opacity, stroke parameters and discrete pixel points | Initial base-colour profile is opaque, explicitly interpreted sRGB; encoded-sample compositing; no 3D surface brush claim |
| Detail | Canonical procedural colour, normal and shared ORM maps from retained parameters | Artistic detail; no physical material or imported-height bake claim |
| Image owner | Corrected straight-alpha source-over in canonical brush/compositing code | Normal blending correctness; other blend modes/masks are not exposed by this native profile |
| Native state | Full source/session/epoch/contributor context guards drafts and selected inspection; one existing worker and intent bus | Local field drafts remain disabled on context changes until explicit reload; real usability/restart acceptance remains open |
| HMC | Original authored maps and all bound non-source resources retain bytes/kind/opaque metadata through fresh compilation | Conflicts and unsupported flags/hash inconsistencies refuse; unknown source-entry metadata refuses snapshot-only compilation rather than disappearing |

The [initial save manifest](install-manifest.json), [58-target save receipt](applied-source.json)
and [ordered repairs](repair-order.json) are authoritative. Source/review evidence:
[dispatch](dispatch.md), [inventory](inventory/native-poet-uv-texture-detail-inventory.md),
[UV domain](uv/HANDOFF.md), [detail domain](detail/HANDOFF.md),
[UV integration review](uv/review/root-integration-review.md) and
[resource integration review](detail/review/root-resources-review.md).
Preparation scripts are frozen historical staging records, not replayable
producers; exact before/after images and pinned manifests govern installation.

Poet remains the unified interface. New operations compose canonical geometry,
image, PNG, HMC, UV01/MAT4 and Portal/Etalon APIs; no replacement codec/renderer or
required JS/Node/V8 application path is introduced. Authored edits are evaluated
Vibe 0.1 records; no per-script Rust build or new released dialect is claimed.

## Verification status

The initial actual compile check reached Core/Poet library checking but failed
three test-field accesses: compiled assets are `(key, asset)` pairs. The
[mechanically reviewed two-file correction](repairs/tuple-repair/HANDOFF.md) selects
the asset without changing assertions. The [failed command](check-before-tuple-repair-result.json)
is retained. The [rerun](check-result.json) passed with no drift across 3,245
recorded local source files. The first native authoring run exposed a rejected
planar/atlas proposal holding subsequent editing. The [reviewed compatibility
preflight](repairs/uv-edit-preflight/install-manifest.json) now refuses incompatible
UV proposals before candidate evaluation; final output guards remain. The
[original failure](authoring-before-uv-edit-preflight-result.json) is retained;
the unchanged [30-case selection](authoring-result.json) passed after repair.

Public atlas (10) and canonical image (4) cases passed, as did detail (6), recipe
adapter (3), resource/preservation (4), existing surface/wire (7), new and prior
native workflows (2), and canonical HMC texture-union regressions (6). This is
**103 passing executions / 98 unique cases** including the explicit GPU case;
partial passes in failed runs are not added. Jobs (15), original intent routing
(5) and inspection (10) also passed. Five inspection executions duplicate cases
already selected under jobs (3), authoring (1) and resource integration (1).

| Actual selection | Passing executions | Receipt |
|---|---:|---|
| Public canonical atlas operator | 10 | [UV](uvkernel-result.json) |
| Public canonical alpha/image correction | 4 | [image](image-result.json) |
| Native source/authoring, including prior profile/CAD cases | 30 | [authoring](authoring-result.json) |
| Detail recipe/render and UV recipe adapter | 6 + 3 | [detail](detail-result.json), [atlas](atlas-result.json) |
| Original inputs/metadata, integrity and working-budget integration | 4 | [integration](integration-result.json) |
| Existing surface/resource and strict wire regressions | 6 + 1 | [surface](surfaces-result.json), [wire](wire-result.json) |
| New UV/detail and previous surface contributor/reviewer workflows | 1 + 1 | [workflow](workflow-result.json), [prior workflow](legacyworkflow-result.json) |
| Public HMC texture-union regressions | 6 | [HMC](hmctextures-result.json) |
| Jobs, original intent routing, resource/selected-surface inspection | 15 + 5 + 10 | [jobs](jobs-result.json), [intents](intents-result.json), [inspection](inspection-result.json) |
| Explicit native atlas/paint/procedural GPU path | 1 | [GPU](gpu-result.json) |

All successful selections report zero failed/ignored cases. [Native build](build-result.json)
and [actual Core/geospatial/Portal SDK WASM compilation](wasm-result.json) passed;
neither establishes native GUI acceptance or WASM runtime interoperability. The
[final read-only reconciliation](saved-evidence-verification.json) verifies all
58 latest installed target hashes, all 18 successful command/log/source pins,
the current 3,245-file local source set, three calibration PNGs, executable hash,
and unchanged reference master/regression artifacts. Earlier tests/GPU run before
the final authoring compatibility repair retain their own snapshots; affected
authoring/native workflows reran after it. No prior round's tests are added.

The [GPU case](gpu-result.json) executed the actual atlas/assembly/material/map
upload and readback path: paint and procedural detail each changed 2304 pixels.
[Three 64-pixel calibration images](gpu-calibration) were retained and inspected
by the root software agent. They prove functional pixel changes, not asset quality.
The new [native workflow](workflow-result.json) demonstrates explicit software
actors, source edits, current selected UV/texture inspection, a fresh-service HMC
save preserving original bytes/opaque image metadata, decoded serialized UV01
matching canonical atlas output, review, restart/rebind/rebuild and source/recipe
reopen. This is not GUI or human acceptance.

Only synthetic meshes, pixels and ephemeral software-actor projects are test
inputs. Any retained PNGs are functional calibration, not production artwork.
No Computer Use or human acceptance is claimed. The five private Core UV unit
cases are not the selected public consumer suite; a full-Core/full-ecosystem test
claim is not made. The previously disclosed three private journal cases remain
outside this bounded round.

## Foundation gate disposition

| Gate | Bounded progress | Remaining qualification |
|---|---|---|
| FND-01 | Versioned application recipes over unchanged canonical containers and identifiers | Adopted version/extension negotiation, qualified mappings and complete derivation contracts |
| FND-02 | No new M64/E64 implementation claim | Required geospatial precision/datum/nodata/edge coverage and native production acceptance |
| FND-03 | Atlas UV0 uses existing UV01; actual serialized UV/canonical comparison and HMC/review/reopen passed | Complete typed coexistence, graph-aware rewriting and fidelity remain open |
| FND-04 | Atlas/detail/inspection bounds and exact resource/metadata retention/refusal cases passed | Whole-workload measured peak/latency/precision, cancellation latency and source-entry metadata continuity remain open |
| FND-05 | Native source controls and selected inspection wired; software-actor fresh save/review/reopen passed | Automatic/advanced UV, painting/baking, CAD/organic/rig, full graph/rights lineage and actual native human usability remain open |
| FND-06 | Actual native GPU readback, native build and SDK WASM compilation passed | Runtime native/WASM interoperability and representative benchmarks remain open |

IP-03 **C1–C8 remain open**: complete shared briefs/criteria, actual authorized
cooperation, full required native authoring, revisions/effort/review, conflicts,
reproducible complete HMC output, recovery, and usable controls with required
human acceptance are not closed by selected software tests.

## Next dependency order

1. Complete required high-grade authoring operations: advanced seam/unwrap/packing,
   paint/bake/edge-dilation and texture channels, procedural edit-stack safety,
   required CAD solids/lofts/constraints/joints, organic topology and rigs/animation.
2. Finish graph/identifier/rights/resource derivation, compatible extensions,
   safe semantic changes and representative fidelity/resource qualification.
3. Qualify native/WASM runtime interoperability and actual contributor/reviewer
   native use, including error recovery and required human acceptance.
4. Resume reference production only after the applicable gates pass; preserve
   editable masters and reviewed derivatives for game-demo. No reliable total
   remaining round/cost estimate is established.

All **193 catalogue + eight subject-specific claims remain unassessed**. Film,
open-world game and cartoon packages remain incomplete; the existing reference
master and regression artifacts are preserved. This round does not declare any
model complete or waive a required deliverable.
The [dossier](../../../../../production/community-grounds-growing-station/README.md),
[claim ledger](../../../../../production/community-grounds-growing-station/claims.json)
and [inventories](../../../../../production/community-grounds-growing-station/inventories.md)
remain unchanged.

New implementations are focused files below 500 lines. The existing 646-line
geometry module and 2,005-line capability registry received routing/constant
registration only; operator bodies live in the new library. The existing image
module's correction extracts pixel math into a focused helper. Long native
workflow/regression files are cohesive test fixtures, not new mixed-lifecycle
implementation exceptions. No full-Core suite, measured process-memory ceiling,
universal format superiority or accepted high-grade model is claimed.
