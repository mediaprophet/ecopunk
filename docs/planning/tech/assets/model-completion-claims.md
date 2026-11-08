# Per-model deliverables and completion claims

**Status:** specification draft v0.1 · **Updated:** 2026-10-24.
Timothy requires an explicit claim against **each and every deliverable** before
an agent can call a species/thing package done. This contract makes that
requirement persistent and reviewable. It applies to every complete embodiment,
not just the hero model. It does not claim a runtime gate already exists.

Companions: [production toolchain](production-toolchain-spec.md),
[reusable template](model-production-template/README.md),
[shared HMC contract](asset-package-spec.md), [per-asset requirements](digital-twin-asset-requirements.md).

## 1. Required per-model dossier

Create a dossier per entity/embodiment, retaining stable identifiers across package
revisions. Each dossier contains:

| File/group | Purpose |
|---|---|
| Scope/identifiers record | Referent/family, complete whole/parts, sources/corpus revision, three profile definitions, applicable variants and requirements baseline. |
| Inventories | Components; representations/profiles/LODs; surfaces; rigs; movements/clips/transitions; behaviours/states/interactions; lifecycle/damage; media; native qualification scenarios. |
| Editable source and native pipeline | Actual source documents/recipes, tool/library versions, parameters/seeds, conversions and dependency lineage. |
| `claims.json` | A copy of the template ledger, bound to the exact catalogue version/digest, with a disposition for every base row and expanded case. |
| Evidence index | Digest-bound source/artifact/check/run/capture references, exact scope and inputs, actual results and explicit failed/unexecuted cases. |
| Reviews/decisions | Technical verification and required visual/domain/rights/audio/accessibility acceptance; explicit authority and artifact scope. |
| HMC release and external receipt | Complete graph/resources/variants plus manifest/index; final validation and archive identifiers bound by an external receipt. |

The dossier's logical content belongs to or is linked from the HMC as appropriate
under the rights/evidence policy. A hidden external spreadsheet of labels cannot
replace package discoverability or graph authority. Keep large/private evidence
in its declared location with verified references and access conditions.

## 2. Three deliverable profiles, separate axes

Every complete package requested here MUST provide **film**, **open-world game**
and **cartoon** representations. These three cannot be removed by an agent's N/A
claim. Film/game describe delivery/quality targets; cartoon describes a visual
language that must also declare film/realtime delivery. Thus a cartoon may need
film-quality detail and animation, and a film model is not simply a game mesh
with more polygons. Additional illustrated/scientific/CAD profiles may be required
by the subject and existing specifications.

The dossier's JSON is an authoring/interchange projection, not a private graph
or simulation engine. Native integration must use the qualified shared
Q42/provenance/review/journal machinery as the authoritative record, with
lossless ID/revision linkage for any exported claim ledger.

Each profile needs source/derivation, meaningful geometry and material quality,
compatible rig/clip/behaviour presentation, actual native-runtime/render output,
resource/output settings and human visual acceptance. Shared sources/rigs/clips
are allowed where compatibility is evidenced; sharing does not eliminate the
per-profile claims. Scientific truth and authoritative state remain style-neutral.

Quality profiles and execution modes are separate axes. For each declared
native, WASM-local or native-assisted WASM target, the dossier must identify
the actual framework/backend, available capabilities, native providers and
versions, state ownership and expected absence/disconnect/recovery behaviour.
Expand required cases through the existing scenario/profile inventories and
record open coverage; do not omit a required target because it is unsupported.
The [environment/framework contract](../webizen-environment-framework.md) governs
these modes. A WASM screenshot cannot satisfy native no-JS/V8 evidence, and a
native result cannot prove browser/provider integration.

## 3. Catalogue and exhaustive expansion

The [high-grade/civics-experience requirements](../high-grade-production-and-civics-experience.md)
extend each applicable dossier with separate master-quality, source/derivation,
consumer-use and relevant place/scenario/experience cases. Use existing catalogue
rows plus explicit added requirement/case IDs; the unchanged 193 base rows are
not a claim that every new case has already been enumerated. Preserve source
masters, approved loss criteria and actual native/game/WASM artifacts. A reduced
game profile cannot approve the master, nor can a master approve its consumer.
For a place/scenario, maintain the composition/context and civics intent/action/
consequence records alongside its linked species/artefact dossiers.

REQ-CIVICS-001–005 additionally require applicable contribution/derivation and
cooperative-use cases: originating authors/work, actual funding/settlement,
commercial cost recovery, permitted human-rights no-charge use and discharge.
Keep people-led civics distinct from civic/government capacity. Use the existing
native cooperative/provenance records; no-charge is not a payment or a waiver.

Applicable [REQ-AGR-001–010 / AGR-01–11](../semantic-agreements-and-compensation.md)
add source-rights/agreement/instrument/valuation/token/settlement linkage to the
dossier. Do not treat generated draft terms, demonstration values or token
metadata as accepted agreements or paid contributions. Source/derivative/HMC
references retain operative versions and privacy-scoped compensation provenance.

Applicable [credit/period cases](../credit-allocations-and-periodic-reconciliation.md)
also distinguish provider backing/available/held funds, actual usage, agreed
obligation balances, settlement and residual disposition. A prepaid allocation,
token payload or queued transfer cannot certify a contributor was paid; preserve
the source policy, authority and actual native/rail evidence.

[deliverables.json](model-production-template/deliverables.json) is the canonical
versioned catalogue. The readable template and initial claim ledger are derived
from it. Stable deliverable IDs are never renumbered; revisions/additions/retirements
retain lineage. Existing DTA/REQ requirements still apply: the model's trace matrix
must identify any requirement not yet covered and add subject-specific rows.

Every base row requires an explicit disposition, including inapplicable family
supplements. For each `repeat_by`, enumerate the required cases from the dossier
inventories. A base row with 40 clips is not satisfied by testing one walk loop.
For example, `MOV-03` expands by profile × clip × compatible variant; `MOV-07`
expands by every allowed transition/profile. Compatibility excludes impossible
combinations explicitly; it is not a reason to invent a full Cartesian product.

An empty inventory cannot vacuously pass. Record why no instances apply and
resolve applicability through an existing authorized rule or designated reviewer.
When an inventory gains a part, action, variant or interaction, new claims become
required and affected coverage summaries reopen. “All movements/behaviours” means
the subject's fully researched applicable repertoire at the declared knowledge
revision, with uncertainty recorded; it is not permission to select only actions
the demo happens to show. Known required gaps prevent complete status.

Required repertoire research includes locomotion and special traversal, starts/
stops/turns/transitions, idle/rest/groom/feeding/sensing/communication, social and
reproductive actions, plant growth/season/wind/tropic responses, and machine
operating/fault/service motions as applicable. There is no universal mammal rig
or walking assumption for every species or artefact. Include unusual
species-specific movements and functional operations identified by the sources.

## 4. What an agent must claim per row/case

| Field | Rule |
|---|---|
| Deliverable ID and case dimensions | Exact catalogue ID plus explicit inventory identifiers for each repeated dimension; not an ambiguous free-text checkbox. |
| Applicability | `unassessed`, `required`, `not_applicable_proposed`, or `not_applicable_authorized`. N/A needs a subject-specific rationale and authority/rule reference; an unavailable implementation is not inapplicable. |
| Claim status | `unassessed`, `missing`, `partial`, `blocked`, `failed`, or `satisfied`. Only satisfied applicable claims can contribute to completion. |
| Claim/result | A concrete statement of what was delivered and what was actually observed; describe limits rather than claiming a broad capability from a narrow result. |
| Claimant and evaluation time | Identify the producer/agent role and time. An identifier records attribution, not proof of authentication. |
| Artifact/source references | Exact outputs, selected profile/variant/part, source/dependency revisions and digests relevant to the claim. |
| Evidence references | Evidence scope/type, artifact/input identifiers, actual procedure/result and reproducibility/location; screenshots do not prove hidden state, source does not prove execution. |
| Verification/review references | Actual independent check/reviewer decisions required by the row, their authority, artifact scope and time. Producer claims cannot invent or overwrite human acceptance. |
| Blockers and remaining work | Concrete missing deliverables/interfaces/evidence and ownership. Record a blocked row even if the prerequisite is upstream. |
| Expanded cases | Every expected case has the same claim fields and inventory keys; parent status is derived from the cases, not an override. |

Catalogue review codes combine the named domains: `technical` requires actual
qualified checks for the row's scope; `visual`/`human_visual` requires authorized
human appearance review; `domain` requires competent scientific/design review;
`rights`, `accessibility` and `audio` require their corresponding review evidence.
Combined codes require all named domains. `human_audio` explicitly requires a
human listening decision. Review records identify reviewer kind and authority;
an agent or native-verifier record cannot satisfy a required human judgement.
An automated check may satisfy technical verification without a separate human
for each technical row, but cannot self-certify missing execution or appearance.

Baseline family applicability rules may authorize, for example, fauna-only rows
being inapplicable to a tree. Cite the adopted rule and approved subject family;
this need not create a fresh permission question for each row. A producer cannot
invent a waiver for missing skinning, behaviour or a required target profile.
Exceptions to a required deliverable require an explicit scope decision and
remain disclosed; they cannot silently earn an unqualified complete status.

## 5. Evidence, freshness and hash boundaries

Keep source inspection, build/format checks, executed engine scenarios, image/
audio output and human judgement as different evidence types. Every evidence
record names what it actually covers, including implementation/profile, subject,
dependency versions, camera/time/seed, hardware and numerical tolerances when
relevant. Negative results are retained. No agent-generated receipt text can
stand in for an unrun tool, measured value, signature or reviewer decision.

Changes reopen dependent claims unless a recorded dependency/scope comparison
proves the evidence remains valid. Reusing a clip/material/review across profiles
needs compatibility and scope evidence; one screenshot cannot qualify an entire
catalogue or animation library.

Avoid self-referential hashes: embedded claims can bind source and content-entry
digests under the agreed manifest scheme. The final whole-HMC digest and final
validation result belong in an external release receipt binding the package and
claim/evidence inventory. Do not require an HMC to contain its own final hash or
misrepresent a prepack receipt as validation of final bytes. Exact receipt/
manifest encoding remains an upstream schema decision.

## 6. Completion algorithm and enforcement

The native validator must:

1. Resolve the pinned catalogue, normative requirement baseline, dossier
   inventories, source/package artifacts and authentic review/authority records.
2. Reject missing, unknown or duplicate base IDs and invalid case dimensions;
   compute expected expansions from the compatible inventories and compare exact
   coverage, including subject-specific rows. A manual total/percentage is not proof.
3. Require every row's applicability disposition. Unassessed/proposed N/A or
   unsupported self-waivers keep completion open. Do not waive film/game/cartoon.
4. For applicable rows/cases, require satisfied claims, matching output/evidence
   references and the row's actual verification/review decisions. Check freshness,
   failed cases and required domain/visual/listening/accessibility approval.
5. Verify HMC inventory and graph/resource reconciliation, final artifact checks,
   native workflow/profile coverage, state separation and reproducibility.
6. Derive a result with satisfied/open/failed/blocked/N/A counts and exact IDs,
   without overwriting producer claims or pretending human review occurred.
   Return complete only when no required row, case or acceptance remains open.

The JSON Schema checks record structure; **it cannot by itself** prove exact
catalogue/case coverage, evidence truth, native execution, visual quality or
reviewer authority. Those require the native validator and review integration.

Repository [AGENTS.md](../../../../AGENTS.md) instructs agents to obey this
contract now. TOOL-12 / engine G78 specifies the future native task/release gate.
This is instruction-level enforcement today, not a sandbox, installed agent hook
or already-enforced build/CI control. Do not advertise stronger enforcement.

For a bounded partial task, retain the complete ledger and report the exact
changed IDs, satisfied scope and remaining model-wide work. “Rig editing is
delivered; the package remains incomplete” is a valid scoped statement. “Model
done” with missing film/cartoon/behaviour/graph deliverables is not.

## 7. Agent completion report template

```text
Subject / entity ID / dossier:
Catalogue version + digest / requirement baseline:
Source revision / package-content IDs / final HMC receipt:
Claim ledger and expanded-inventory references:
Film status / open-world game status / cartoon status:
Declared execution modes / native provider requirements / open coverage:
Changed deliverable IDs and concrete claims:
Verification/evidence + required human decisions:
Open / failed / blocked / unassessed / proposed-N/A IDs:
Authorized-N/A IDs + authority/rule:
Native gate result + evidence freshness:
Overall: incomplete | claimed complete awaiting verification | verified complete
```

The report must link every row through the ledger; it need not paste hundreds of
rows into chat. A specification/template being finished is not a claim that a
model using it is finished. All rows in the supplied starter ledger are unassessed.
