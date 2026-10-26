# Round 6 — native profile/CAD and surface authoring

2026-10-25. **58 distinct targets saved; 70 selected native case executions passed
(69 unique), including the repaired HMC contribution/review/reopen workflow.**
This bounded round is **complete_scoped**: final source/evidence reconciliation
passed. The native binary built and the actual SDK compiled for WASM.
Asset production remains on hold. All FND-01–06 and IP-03 C1–C8 remain open.

This is the historical Round 6 snapshot. [Round 7](../round7/progress.md) supersedes
some saved source targets with UV atlas, texture/detail and resource-retention
work. Round 6 receipts retain their original tested source hashes and results.

| Area | Saved implementation | Scope boundary |
|---|---|---|
| Profiles | Metric rounded rectangle, U channel and explicit simple XY polygon; canonical sweep length/caps | Source, material and part records retained; richer existing topology data refuses replacement |
| CAD | Full-turn radius/height revolution around local +Y with explicit winding and open ends | No end caps, thickness, partial angle or manufacturing qualification; explicit host opt-in required |
| UV | Canonical part-local planar projection plus reversible UV0 affine transform | Base geometry UVs retained; no atlas/general unwrap/other UV channels |
| Visual materials | Full admitted MAT4 factors/flags, six explicit map roles, colour space/channel/transform/sampler controls | Physical material properties are separate; unsupported normal-slot rotation/reflection refuses |
| Resources | Native project import library and SHA-guarded immutable HMC snapshot binding | No source-selected path/network access; binding never grants authority or restores a session |
| Production path | Pre-promotion/rebuild map validation, actual dependency bytes in coherent `.10d`/Q42 HMC, new keys retain older output | Complete graph/rights derivation and native usability remain open |
| HMC composition | Canonical exact texture union retains original metadata and charges unique output; conflicting SHA/MIME/bytes and reserved-key collisions refuse | Six public native consumer tests passed, including actual save/reopen and destination preservation on refusal |

[Installed manifest](install-manifest.json) and [save receipt](applied-source.json)
pin the initial targets and preimages; [ordered repairs](repair-order.json) include
the [five-file canonical HMC repair](repairs/hmc-texture-union/applied-source.json)
and its [independent review](repairs/hmc-texture-union/review/independent-review.md).
All 58 latest target hashes were checked after saving. Source/review contracts are in
[profiles](profiles/HANDOFF.md), [CAD](profiles/CAD-HANDOFF.md),
[surfaces](surfaces/HANDOFF.md), [source inventory](inventory/native-poet-profile-surface-inventory.md)
and [dispatch](dispatch.md). Canonical geometry, UV projection, image decoding,
HMC keys and GPU upload are reused. No replacement codec/renderer or JS/V8 path
was introduced.

The revolution target is not added to retained projects automatically. Native
launcher opt-in `POET_NATIVE_REVOLVE_PROFILE=1` enables the target in the host
profile; authored requests and actual grants remain necessary. Existing explicit
file-based host configurations can declare it. No real environment or credential
was changed in this pass. Profile limits are working scopes, not master ceilings.

Native entry exposes its project-owned `imports` directory through the existing
bounded resource-library controls. Image bytes must be imported under the
canonical content-derived texture key and explicit MIME kind. Bind the inspected
HMC SHA256 in assembly controls, then select planar UVs/material map digests.
After reopening textured source, explicitly bind the resource snapshot and
rebuild; source/resource/session authority is revalidated. Changed same-key
compiled resources still refuse; use a new explicit asset key.

Preparation generators are historical one-shot staging records and disabled by
[the freeze marker](PREPARATION-FROZEN.json). Exact before/after images and the
hash-pinned installer are the authoritative application inputs. This does not
claim a generator can be rerun to reproduce its intermediate states.

Actual native checking and selected CPU/GPU runs passed; the retained workflow
failure exposed duplicate insertion of an already-retained canonical texture.
The independently reviewed Core repair is saved; post-repair compilation and six
public texture-union tests passed. The strict native contributor/reviewer workflow
rerun also passed, and the native executable built. [SDK WASM compilation](wasm-result.json)
and [final read-only reconciliation](saved-evidence-verification.json) passed.
The latter verifies all 58 latest installed target hashes, command/log/source
pins, calibration images, the executable hash and unchanged production fixtures.
Six map roles and UV offset each changed
2304 actual GPU pixels; [14 retained calibration PNGs](gpu-calibration) are
synthetic checks, and five frames were inspected by the root software agent.
GPU diagnostic fixtures are synthetic
2x2 textures and calibration geometry, not production assets or model claims.
PNG retention is explicit and bounded; source presence/ignored tests are not
executed results. No Computer Use or human usability acceptance is part of this
pass. The three previously disclosed private journal cases remain outside this
scope. All 193 catalogue plus eight subject-specific claims remain unassessed;
film, open-world game and cartoon packages remain incomplete.
The existing [dossier](../../../../../production/community-grounds-growing-station/README.md),
[claim ledger](../../../../../production/community-grounds-growing-station/claims.json)
and [inventories](../../../../../production/community-grounds-growing-station/inventories.md)
are unchanged; this round is software qualification, not model production.

## Actual verification

| Selection | Passing case executions | Receipt |
|---|---:|---|
| Native authoring, including profile/CAD/UV/surface source edits | 24 | [authoring](authoring-result.json) |
| Texture resource admission/preservation and strict material wire format | 6 + 1 | [surfaces](surfaces-result.json), [wire](wire-result.json) |
| Canonical geometry catalogue and explicit native host opt-in | 6 + 1 | [catalogue](catalog-result.json), [host opt-in](hostoptin-result.json) |
| Public canonical HMC texture-union consumer | 6 | [HMC](hmctextures-result.json) |
| Original native intent path through textured source/HMC/contribution/review/reopen | 1 | [workflow](workflow-result.json) |
| Original Poet intent routing and authority | 5 | [intents](intents-result.json) |
| Job/epoch/stale-result handling | 13 | [jobs](jobs-result.json) |
| Selected resource inspection | 6 | [inspection](inspection-result.json); one case also ran in jobs |
| Explicit actual GPU resident maps, readback and UV change | 1 | [GPU](gpu-result.json); six map roles and one UV measurement are assertions within this case |

Total **70 passing executions / 69 unique cases**, zero failed/ignored in the
successful selections. Earlier failures remain retained and are not erased or
counted as passing runs. These are selected tests, not the full ecosystem suite.
Every command has a local-source snapshot; the earlier authoring/GPU/regression
runs precede later disclosed repairs. The HMC tests and workflow reran against
the saved Core repair. Native checking and [executable build](build-result.json)
passed against the final recorded 3,219-file local source set. Compilation is
not native GUI/human acceptance or WASM runtime execution.

The [original compile failure](check-before-signature-repair-result.json) was an
old controller test call missing the new resource argument. Its scoped repair
and successful subsequent checking are retained. Review also corrected stale
displayed preview retirement after resource rebinding. The [original textured
workflow failure](workflow-before-texture-union-result.json) was the actual
duplicate texture insertion in Core. Its [reviewed canonical repair](repairs/hmc-texture-union/HANDOFF.md),
six public passing cases and strict application workflow rerun are recorded;
the test did not drop original resources or weaken texture-closure assertions.

## Foundation gate disposition

| Gate | This round's bounded contribution | Still required; gate stays open |
|---|---|---|
| FND-01 | Existing canonical formats/keys and exact identifier/source pins retained | Adopted revision/extension contracts, qualified mappings and interoperability lineage |
| FND-02 | No new M64/E64 implementation claim | Geospatial domain/CRS qualification, full required precision/nodata/edge cases and native production acceptance |
| FND-03 | MAT4 map/source authoring and coherent `.10d`/Q42 dependency packaging path | Full typed coexistence/fidelity coverage, safe graph-aware semantic changes and extension negotiation |
| FND-04 | Explicit texture decode/upload limits, unique HMC charging and conflict/preservation checks | Representative whole-project memory/latency/precision measurements and cancellation coverage |
| FND-05 | Added original Poet intent-based profile/CAD/UV/surface controls, package resource binding and software-actor workflow coverage | Required CAD/organic/rig operators, complete graph/rights derivation and actual usable native contributor/reviewer acceptance |
| FND-06 | Actual native GPU map/UV readback, native executable build and actual SDK WASM compilation passed | Native/WASM runtime round trips, required host-service integration and reproducible workload benchmarks |

IP-03 **C1–C8 all remain open**. Shared briefs/tasks/criteria (C1), real authorized
contributor/reviewer use (C2), full native geometry/surfaces/rig and preview work
(C3), durable authorship/effort/review (C4), conflict resolution (C5), reproducible
complete HMC production (C6), restart/rejoin recovery (C7) and usable controls with
required human acceptance (C8) must pass their complete cases. The selected
software tests exercise portions of those cases; they do not close them.

## Remaining dependency order

1. Complete the required reference-scope authoring operators: general UV unwrap/
   atlas and texture/detail editing, required CAD solids/lofts/constraints/joints,
   organic topology and rig/animation operations. Reuse existing owning kernels;
   source/UI bindings and genuinely missing kernels require separate evidence.
2. Finish graph/identifier/rights/resource derivation, adopted version/extension
   rules and whole-workload fidelity/budget qualification. Safe same-key semantic
   rewrites remain unresolved; explicit new output keys preserve prior records.
3. Qualify runtime native/WASM interoperability and the real native cooperative
   workflow with required human acceptance, including recovery and error paths.
4. Resume reference masters only when the applicable foundation/authoring gates
   pass; then retain full film/game/cartoon dossier claims and reviewed game-demo
   derivatives. No reliable total remaining round/cost estimate is established.
