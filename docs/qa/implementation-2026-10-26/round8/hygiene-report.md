# Round 8 hygiene and first usable production readiness

**Subsequent supervisor outcome:** the reviewed 733-file migration is now complete;
see [Round 8 progress](progress.md) and its final verification. The inspection
and non-execution statements below describe this worker's earlier handoff, not
the later supervisor execution. Historical receipts were preserved.

Date: 2026-10-26. Scope: inventory, non-mutating relocation plan, and evidence-based readiness estimate. No historical file was moved, deleted, edited, or executed in this task.

## Round 7 baseline

The governing plan points to `docs/qa/implementation-2026-10-25/batch-6/rounds/round7/progress.md`. It calls Round 7 `complete_scoped`, reports 58 distinct installed targets, 103 passing executions / 98 unique cases, native build and SDK WASM compilation passing, and keeps all FND-01–06 and IP-03 C1–C8 open. The demonstrated scope includes seam-split harmonic disk charts, bounded atlas packing, native UV/paint/detail controls, byte-preserving HMC resource retention, an explicit GPU readback, and software-actor save/reopen/review/rebind/rebuild. It does not establish GUI/human usability or an accepted master.

The record explicitly leaves advanced UV/unwrap, paint/bake/edge dilation and channels, CAD solids/lofts/constraints/joints, organic topology and rig/animation, full graph/identifier/rights derivation, representative fidelity/workload, runtime interoperability, and human usability unresolved. It directs asset production to remain HOLD.

## Staged code/harness inventory

Read-only enumeration of round-specific source, scripts and manifests in `docs/qa/implementation-2026-10-25/batch-6/rounds/`, excluding generated `target/` trees. Byte totals use filesystem lengths. These are historical candidate/evidence inputs; filenames alone do not establish production use.

| Round | Files | Bytes | Rust | Cargo manifests/locks | Python/PowerShell | Test/harness category | Before/after/candidate category | Unclassified implementation/tooling |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5 | 29,637 | 2 | 0 | 3 | 0 | 0 | 5 |
| 2 | 35 | 703,601 | 28 | 0 | 7 | 1 | 8 | 26 |
| 3 | 111 | 1,552,619 | 104 | 0 | 7 | 14 | 37 | 60 |
| 4 | 191 | 1,938,290 | 164 | 7 | 20 | 25 | 35 | 131 |
| 5 | 96 | 949,162 | 86 | 3 | 7 | 16 | 21 | 59 |
| 6 | 145 | 1,229,011 | 127 | 6 | 12 | 13 | 45 | 87 |
| 7 | 150 | 1,763,587 | 138 | 3 | 9 | 22 | 34 | 94 |
| **Total** | **733** | **8,165,907** | **649** | **19** | **65** | **91** | **180** | **462** |
`*`Exclusive heuristic category in the JSON map, assigning test/harness matches first, then before/after/original/candidate matches; unclassified means only “no path token matched,” not verified production code. These labels are not removability decisions. The exact 733-entry SHA-256/original-path/proposed-destination map is in [relocation-dry-run.json](../../../../tools/poet-foundation/round8/hygiene/relocation-dry-run.json). Its exclusive path-based categories distinguish test/harness, before/after/candidate snapshot, and unclassified implementation/tooling; the last category is not verified production code. The map sets `move_authorized: false` for every entry and is not relocation authorization. Earlier batch-6 non-round roots also contain substantial snapshots/harnesses (e.g. `ui-before/`, `craft/originals/`, `authoring-tests-before/`); they were intentionally excluded from round totals and should be inventoried as distinct cohorts before any later cleanup proposal.

No Round 8 hygiene directory or Round 8 progress artifact existed at inspection time. The root assignment's owned directories have now been created for this report and future inventory artifacts.

## Non-mutating relocation proposal

If the owner approves a later archive change, move eligible *non-executed evidence copies* to `audit/implementation-snapshots/2026-10-25/batch-6/rounds/roundN/<original-relative-path>`, preserving every leaf name and relative path. Keep receipts, manifests, command logs, applied-source ledgers, and report markdown in `docs/qa` as immutable audit records. Before move: enumerate every source file with SHA-256, size, original path, destination, classification, and references; confirm destination absent; scan repository text/config/scripts for literal path references; inspect scripts for relative paths and working-directory assumptions; confirm no Cargo metadata or operator guidance uses those staged paths. After move: verify every source/destination hash pair and update only live navigation links/configuration in a separately reviewed change. Never rewrite the historical receipts themselves; add a new migration index mapping their original paths to archived locations.

Likely impacts to check: scripts that use paths relative to their own directory, shell invocations in receipts, markdown links from round handoffs, JSON manifests with candidate/source paths, and tooling that resolves `include!`/`#[path]`/Cargo manifest paths. Archived harnesses and historical one-shot scripts are preserved for audit but should be treated as non-runnable historical inputs after relocation. Do not add an execution adapter or rewrite pinned scripts. Generated `target/` build output and binaries were excluded from the table; do not classify them as removable without separately proving they are generated and not pinned evidence.

This is a proposed layout only. The exact source digest/path → proposed destination map and read-only repository reference scan are prepared. No relocation, deletion, or link edit has been performed. The map and script do not authorize movement.

## First usable growing-area-equipment authoring readiness

The decision boundary is an actual contributor and reviewer, using Poet and canonical native operators, to edit/revise a growing-area-equipment source package, save/reopen it with source and required HMC graph/resources intact, review a specific revision, resolve a conflict/recovery case, and produce output that a human accepts for the intended production brief. A green software fixture, renderer window, API/MCP route, or saved receipt alone is not usable authoring. This scoped readiness target does not require completing every organism in the broader flora/fauna/rig ecosystem, but any species/thing included in the equipment brief retains its applicable catalogue claims and human acceptance obligations.

| Requirement | Evidence present | Status for first usable equipment workflow |
|---|---|---|
| Poet native source geometry/material/UV editing and selected inspection | Rounds 4–7, bounded source workflows, actual GPU readback and software-actor save/reopen cases | Partial; no actual GUI contributor/reviewer acceptance. Equipment assembly still needs usable selection/navigation/transform interaction; current UI evidence is snapshot image output, with no orbit/gizmo/pick interaction located in inspected UI source |
| Equipment-specific editable geometry operations | Dossier source declares frame, channels, openings/collars, lids, pipe branches, sump/cabinet, pump representation; Rounds 6–7 include software-tested parameter/revolve catalogue paths and the Round 7 explicit-seam UV unwrap/atlas; automatic seam selection and optimal packing remain outside that scope | Partially covered. No exact mapping from source-declared assembly components to required editing operations and accepted engineering criteria is established. Existing profile/revolve catalogue paths have software-test coverage; remaining question is whether the required equipment operations are all reachable and editable through interactive Poet UI. Joints/constraints only when explicit equipment intent needs them |
| Cooperative identifiers, membership admission, permission freshness, durable contribution/review commit | RUN-09/10 and Rounds 4–7 have implementation and software-test evidence for native auth/admission/contribution/review mechanisms (per supervisor correction); RUN-08-AUTH still marks end-to-end native publication freshness, qualified identifier associations and actual durable human workflow as open | Implemented in source and software-tested; native UI/host closure and actual contributor/reviewer acceptance unverified. Reuse and qualify, do not rebuild by assumption |
| Shared UI and MCP owner/session with per-action permission checks | RUN-08-MCP requires canonical owner/session, real host admission, complete required operators and no receipt-only/JSON-principal success | Source/session foundations and API-level policy evidence exist; full native UI-to-MCP parity and host-backed human workflow unverified |
| Concurrent contributor/reviewer revisions, conflict refusal, recovery, fresh reopen | Some source/revision journals and software-actor reopen paths exist in Rounds 4–7; Round 7 lists human usability open | Partial selected cases; actual cooperative usability/recovery unverified |
| Complete HMC/graph/identifier/rights/resource continuity for derived equipment package | HMC bytes/metadata retention improved; dispatch and Round 7 leave full embodiment/graph/rights/derivation and source-entry continuity open | Blocking for production acceptance |
| Runtime/target interoperability and resource/workload qualification | Native builds and actual SDK WASM compilation have scoped passes; runtime interoperability and representative benchmark remain open | Unverified; qualify the target actually required by deployment |
| High-grade equipment master + human review | No accepted production master is claimed; catalogue claims remain unassessed | Missing; no asset production before gates |

### Equipment component-to-operator map

This maps dossier-declared components to the smallest plausible authoring operator family. It does not assert dimensions, function, topology, suitability, or that every optional detail is required by the production brief. Exact criteria must be derived from the source and reviewed brief before production.

| Dossier component | Likely editable operation(s) | Existing evidence / open point |
|---|---|---|
| Station root, datum plate, hierarchy | Assembly/source hierarchy, datum and transforms | Round 4/5 source and hierarchy editing/reparenting cases exist; interactive viewport selection/manipulation and engineering datum acceptance unverified |
| Frame rails, posts, feet, braces, gussets, bolts | Primitive solids; profile/path sweeps; repeated/placed parts; possibly joints/constraints if movable assembly is required | Basic geometry and bounded profile sweep exist; authored path/station sweep and assemblies' actual joint/constraint behavior not qualified. Do not require parametric joints unless brief demands them |
| Six grow channels / formed sections | Profile extrusion or path sweep; caps and openings | Existing straight profile sweep; no qualified authored path framing/twist/scale or channel wall/opening acceptance |
| 36 openings/collars | Repeated placement, cut/boolean or ring/collar profiles | Source intends 36 positions; dossier says actual independent port count/fit unqualified. No evidence that native editor makes/selects these interactively |
| Channel lids / removable bays / end details | Repeated part placement, profile/revolve/sweep; attachment/removal | Source inventory only; no human interaction or fit acceptance |
| Feed/return pipes, valves, levers, couplings | Path sweep plus revolve/profile components; assembly links if operational behavior required | Straight profile sweep and Core revolve/catalogue paths have software-test coverage; interactive equipment route-path editing and connection semantics remain unverified |
| Sump/cabinet panels, door, gasket, handle, hinges, vents | Primitive/profile solid, panel/hinge placement and assembly hierarchy | Simple editable geometry exists; no qualified hinge/open-state or service-clearance workflow |
| Pump representation | Editable component geometry | Dossier explicitly calls this a visual representation; no pump capacity/electrical/operating claim should be inferred |
| Plant dependencies | Separate species instances and species-specific authoring | Not part of equipment manufactured-parts prerequisite; only becomes an obligation if brief selects a plant species, then its own model ledger applies |
### Narrow blockers versus broad ecosystem completion

Narrow first-use blockers: (1) close the native UI/host route over the existing auth/admission/contribution/review implementation, with current permission checks and durable evidence; (2) build a real interactive viewport/source editing loop sufficient for the dossier component operations, including selection/navigation and the CAD operators actually required by the equipment brief; (3) demonstrate two real people editing/reviewing revisions with conflict/recovery and fresh-process reopen; (4) verify complete source, HMC graph, identifiers/rights and referenced-resource lineage through edit/derivation; (5) obtain human usability and equipment-master review. Do not treat cooperative source/API presence or software tests as the delivered workflow.

Broader still-important work, not necessarily a gate for the initial equipment class: generalized automatic/conformal unwrap and optimal packing; broad flora/fauna topology, full rigs and animation; unrelated species packages and film/game/cartoon embodiments; universal runtime/format superiority. Any included part/organism still needs its own declared obligations. Do not waive FND/IP-03: passing this narrow class would advance evidence, not close C1–C8 globally.

### Bounded-round estimate (work estimate, not calendar or price)

These ranges are provisional work estimates, not calendar or price promises. Estimate assumes reuse of Rounds 1–7 and RUN-09/10 source, a stable equipment brief/operator map, no major upstream redesign, disjoint source ownership, and reviewer availability. It excludes external-host changes until explicit authority/owners exist and excludes provider spend.

| Work packet | Likely bounded rounds | Dependency / exit evidence |
|---|---:|---|
| A. Qualify existing admission/cooperative services in native UI/host | 1–2 | Reuse RUN-09/10 and Rounds 4–7 owners; prove current permissions, publication freshness and actual durable contributor/reviewer records through Poet |
| B. Interactive Poet viewport and equipment operator closure | 2–4 | Map master.vibe component intents to operations; viewport selection/navigation/manipulation; wire existing Core/Poet operators only where needed; actual intent → editable source → geometry |
| C. Equipment source/HMC lineage and reopen closure | 1–3 | Depends on B and canonical package model; fresh-process edit/reopen and exact graph/identifier/rights/resource evidence |
| D. Contributor/reviewer tryout and workflow hardening | 1–3 | Depends on A–C integration; actual sessions, conflicts, recovery, usability and equipment review |
| E. Integrated qualification / repair reserve | 1–2 | Re-run only affected plus end-to-end cases, actual host/runtime needed by product; review evidence |
| **First interactive equipment tryout** | **3–5 rounds / 12–25 active hours** | Provisional: one brief-relevant operator through interactive source edit and visible result; not cooperative production readiness |
| **Growing-area equipment production-ready tooling + acceptance** | **8–12 rounds / 30–60 active hours** | Reuse existing cooperative APIs; actual two-person UI workflow, equipment operation coverage, lineage/reopen, conflict/recovery and human acceptance |
| **Full flora/fauna/rig ecosystem** | **Not estimated** | Separate species/thing obligations, three target embodiments and required acceptance; intentionally outside equipment estimate |

Separately, broad FND/IP-03 closeout and all flora/fauna/rig outputs remain unestimated; do not roll them into the equipment estimate. No calendar date, dollar amount, provider cost, or guaranteed delivery is implied.

## Checks and limits

Read only: implementation dispatch, governing implementation plan, Round 7 progress and inventory, growing-station dossier/inventory, and file metadata/hash enumeration for the seven round directories. Cooperative API status uses the supervisor-provided RUN-09/10 correction; full evidence records still need direct linking in the parent integration report. No Cargo build/test, asset export, sibling write, paid call, UI control, commit, or external mutation. File category matches are path heuristics; no claim that snapshots are safe to move or harnesses are executable is made.

## Dependency scan follow-up

The hash-guarded but unexecuted relocation script is [prepare-relocation.ps1](../../../../tools/poet-foundation/round8/hygiene/prepare-relocation.ps1). Archived scripts are marked as historical, non-runnable inputs; no execution adapter is supplied. The read-only original-path resolver is [resolve-archived-path.ps1](../../../../tools/poet-foundation/round8/hygiene/resolve-archived-path.ps1); it reads the post-move migration index and verifies bytes/digest without executing archived content. The read-only script/Cargo/reference scan is in [dependency-reference-scan.md](../../../../tools/poet-foundation/round8/hygiene/dependency-reference-scan.md) with machine-readable details in [dependency-reference-scan.json](../../../../tools/poet-foundation/round8/hygiene/dependency-reference-scan.json). It identifies scripts that invoke live sibling Cargo workspaces, Rust path/include status, path-bearing pinned evidence references, Markdown no-change review, and other QA code cohorts. It records the completed read-only repository reference scan and does not authorize or execute a move.




