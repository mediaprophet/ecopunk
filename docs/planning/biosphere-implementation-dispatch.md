# Biosphere implementation dispatch and restart handoff

**Current direction, 2026-10-26:** implementation is active for the exact jobs in [biosphere authoring round 1](biosphere-authoring-round1.md), following Timothy's explicit request to update plans and deploy the first round. The [governing plan](implementation-plan.md) and [register](biosphere-work-packets.json) retain all original phases, IDs and acceptance. Earlier generic wave examples remain templates; they do not activate every packet. Asset production remains HOLD; no species/package or human acceptance is claimed.

The [full Poet authoring review](tech/poet-comprehensive-authoring-review.md)
adds PA-R01–12 research handoffs for original workspace recovery and non-CAD
media/document/data capabilities for people and agents. These share this
register and coordination policy; they do not replace R-01–06, duplicate their
plant/rig study or activate implementation. Poet internal domain capabilities
reuse existing QualiaDB contracts and source engines, with one authoritative
session per domain document. Full editor/native parity must be demonstrated.

## Resume without relying on this chat

1. Read repository instructions, the governing plan and the two biosphere
   capability/integration contracts. Then read
   [the current review index](../qa/implementation-2026-10-26/biosphere-integration-review/README.md)
   and the relevant packet. Source inventories are not semantic review or tests.
2. Inspect the packet's handoff and current processes/lease. Recheck source bytes
   against its preimages. A dirty tree is preserved, not reset; changed preimages
   require reconciliation before installing a patch.
3. Continue the actual owned command if still running. Tool session identifiers
   may be agent-scoped; verify resumability rather than handing off an untested
   session ID or starting another build. Missing final exit is not a pass.
4. At each handoff record completed changes, exact files/hashes, API assumptions,
   dependencies, command/session/PID, output/log paths, exit or running state,
   failures, open cases and the next bounded action. No automatic human acceptance.

## Exclusive ownership and Cargo discipline

Four concurrent agents means one integration coordinator and up to three workers.
Workers may inspect shared sources; they write only their own packet staging and
fixtures. They do not run Cargo, format the whole workspace, resolve locks, alter
manifests/registries, or edit another worker's canonical source. The coordinator
alone reviews and applies preimage-pinned patches to the owning native repository.
Shared module registration, dispatch, renderer/controller integration and manifests
are coordinator-owned patches, applied after worker handoffs.

Workers reuse canonical QualiaDB/Etalon/sim/graph/logic APIs. If a required shared
contract is unresolved, write the precise contract/test vector and continue
independent work; do not invent a parallel solver, graph or package format.
Implementation authority does not authorize source-boundary violations, commits,
publication, money movement, or deletion of existing work.

The coordinator freezes a coherent source set before checks and owns serial
Cargo, native GPU and fresh-process recovery qualification through final exit.
Use one dependency closure and the existing `C:/github/ecopunk/target` artifacts;
record actual environment/profile/source pins and results. No simultaneous
source edits during a check. Reuse current receipts until changes or failures
justify new checks; no duplicate expensive builds to compensate for lost context.

The [serial-check lease helper](../../tools/poet-foundation/biosphere-shared/serial_check.py)
refuses a second cooperating coordinator while its lease exists. It requires
locked/offline Cargo and the agreed target directory, records owner/coordinator/
child PIDs, command, UUID and UTC start, and retains an interrupted/unknown lease.
There is no automatic lock stealing or force-unlock. Investigate a retained lease
and actual child state before resuming or explicitly reconciling it. Existing
historical runners do not automatically acquire this lease; future commands must
use it. This is coordination for cooperating callers, not a security boundary or
a claim to control arbitrary Cargo invocations.

Future example, after source review and check selection:

```powershell
python tools/poet-foundation/biosphere-shared/serial_check.py --owner BS-90 -- cargo test --manifest-path C:/github/poet/Cargo.toml -p poet-cli --features native-graphical --locked --offline --target-dir C:/github/ecopunk/target --lib SELECTED_TEST -- --test-threads=1
```

The coordinator supplies the actual runtime executable and selected meaningful
test; this template was not executed as an engine check by the review.

## Jobs, sequence and cost routing

| Job | Dependencies | Substantive bounded deliverable | Agent route |
|---|---|---|---|
| BS-01 | Current review | Pin the shared graph/resource/cue descriptor contract and small existing-source fixtures. Catalogue actual units, frames, referents, rights, expected queries and missing data. Supervisor owns schema decisions; worker does deterministic metadata and source checks. | gpt-6-luna, medium; supervisor reviews semantics. |
| BS-10 | BS-01 relevant contract | Native bounded botanical graph/radii operators using existing Core kernels, graph-to-existing geometry recipes, stable organ selection/editing and HMC replay. Keep current turtle integration. Identify sim-side growth/leaf parameter/state adapters separately; no Core-to-sim cycle. | gpt-6.1-sol, high. |
| BS-20 | BS-01 relevant contract | Native connected-surface/rig/weight/clip admission and actual animated viewport, preserving graph/component/frame/normal/UV mappings through edit and reopen. Use skeletal/player/upload owners; no unreviewed capsule animal or invented physiology. | gpt-6.1-sol, high. |
| BS-30 | BS-01 graph/resource contract | Native Q42 query -> selected typed resource closure -> bounded installed-HMC access. Fix/admit required metadata and verification behavior, query work/cancellation and lazy access. Prove unrelated payloads were not accessed. | gpt-6.1-sol, high. |
| BS-40 | BS-01 cue contract | Typed EMF/physical/acoustic/environmental descriptors and existing operator bindings; retain original spectra, declared model validity, receiver evidence and state-event provenance. Scope one honest supported case before broader coupling; missing physiology remains unknown. | gpt-6.1-sol, high for semantics/runtime; lower-cost worker can enter reviewed metadata. |
| BS-50 | Relevant BS-10/20/30 and cue metadata | Integrate actual native source/query/selection/magnifier controls, progress and recovery under contributor/reviewer authority. Produce identified branch/leaf and rig-motion engineering outputs, measured resource traces and intact HMC reopen evidence. | Supervisor + gpt-6.1-sol integration; gpt-6-luna for bounded controls/receipts after interface freeze. |
| BS-60 | BS-30 | Qualify remote selective acquisition using trusted descriptors and existing proof/codec owners. Document whole-entry granularity where necessary; add missing authenticated sparse support in the correct owner. No unauthenticated range promotion. | gpt-6.1-sol, high; targeted specialist escalation only for a specific unresolved proof/API question. |
| BS-90 | A coherent reviewed group | Serial integration checks and independent review; retain failed attempts, final source pins and real output artifacts. Reports distinguish source, execution, human/domain acceptance and all remaining IDs. | Coordinator; gpt-6-luna for exact receipt/hash reconciliation, stronger independent reviewer for runtime/authority findings. |

Future implementation wave: BS-01 contract/metadata, BS-10 botanical adapter work
and BS-20 rig admission can proceed in separate staging areas, with dependent
edits waiting for the relevant BS-01 contract. Next, BS-30 graph-selection and
BS-40 cues can run alongside accepted operator/UI integration. The coordinator
reviews each coherent group and checks it serially. BS-60 extends acquisition
after the installed-resource path is correct; it is not permission to call
installed-file access remote streaming. No fixed round/hour estimate is justified
until these exact call paths and representative workloads are measured.

## Required packet handoff

Use `tools/poet-foundation/biosphere-shared/work/<JOB-ID>/` for each worker's
`before/`, `after/`, `manifest.json` and `handoff.json`; evidence goes to
`docs/qa/implementation-2026-10-26/biosphere-shared/<JOB-ID>/`. A patch manifest
lists repository, target, expected preimage and staged after SHA256 for every
file, including new-file absence. No silent overlap or direct application.

The handoff includes `status` (`working`, `staged`, `applied`, `verified`,
`needs_contract` or `failed`), source/API facts, exact modified paths, commands
and process state, evidence/result paths, open case IDs and next action. These
are engineering statuses, not model completion or human acceptance. Failure
must name the cause and remedy; staged source is not installed source, a build
is not a runtime result, and generated signing/review records are not human acts.

## Concrete review outputs and gates

Require BGS-Q1–Q8 as applicable to each job: real native graph/part/source
correspondence, geometry/animation output, exact selected bytes/ranges/residency,
typed cue interpretation, negative/error/cancel/stale cases, and fresh-process
HMC source/resource recovery. Each reference retains a full pinned dossier before
model production; all three profiles and scientific/domain acceptance remain
separate from bounded software delivery. Payments remain later work.
