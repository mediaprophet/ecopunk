# Human principals and delegated agent authority

2026-10-26. Required by Timothy's clarification during the first biosphere authoring swarm. This is an implementation contract and gap register, **not a claim that the hierarchy is enforced already**. It refines IP-03 C2/C4/C5/C7; it does not replace the governing implementation order or start a separate authentication system.

## Authentication and authority are separate

A human principal authorises an agent to act within a defined remit. The agent remains an identified, separately authenticated actor. Authentication proves the admitted controller/key/session relationship; it does not by itself establish the human principal, delegation, resource rights or permission for the requested operation.

Preserve the existing public names `AuthenticatedPrincipal`, `PrincipalKind`, `SubjectAdmission`, `AuthorityOwner` and `SessionHandle` when integrating. In the product, distinguish the **human principal**, **acting agent**, and **delegation chain**. Do not relabel an agent as a human or collapse several humans into one global owner. A principal and an agent can each have several qualified identifiers; matching names, strings or digests alone establish no authority.

The required chain is:

1. An admitted human principal and evidence of their authorised project role.
2. An adopted delegation binding that principal to an authenticated agent/controller, an explicit scope and a validity period.
3. Optional further delegation only where explicitly permitted, bounded in depth, and strictly confined to the parent's authority and remaining budgets.
4. The acting agent's live authentication and an operation authorised against the current chain, project, source/resource revision and applicable rights.

A project trust anchor or membership signature is not automatically the human's delegation. Human admission must use the ecosystem's adopted human/controller evidence; this contract does not prescribe government identification or invent proof of personhood. Existing records require explicit migration/re-adoption where they lack the necessary evidence.

## Required checks and records

The verified native adapter must bind and enforce:

- Human principal and acting-agent identifier associations, issuer/scheme/scope, admitted controller keys, parent delegation references and adoption/provenance evidence.
- Project, document, asset/resource and data scope; permitted commands/effects/actions and purpose. A broad role cannot silently expand a narrower delegation or source permission.
- Start/expiry, cancellation, revocation and ancestor revocation. Recheck at admission, durable commit and delayed publication; an expired or withdrawn delegation cannot remain useful through a retained handle.
- Explicit subdelegation permission, bounded depth, cycle refusal and attenuation. A child cannot add rights, extend expiry, change the responsible principal or increase a shared budget.
- Applicable operational/resource/provider budgets and cancellation. No financial permission is inferred from authentication; payment rails remain later work.
- Actual acting agent, controlling human/delegation ancestry, source/output/package identifiers, requested operation, decision and evidence references. Preserve authorship, provenance and originating contributions; a delegation record is not proof of payment, adopted licensing terms or legal liability.

Native controls, Vibe effects, MCP, jobs and package review must use the same verified authority owner. A request field, imported HMC record or caller-supplied list of delegation quins must not mint authority. Existing expected-source, signed admission, resource selection, journal and package-byte checks remain necessary.

Reviewer independence must consider the controlling human and delegation ancestry as well as the acting agent. Two agents under one human cannot be presented as independent reviewers merely because their subject identifiers differ. Record the actual project review policy and any permitted same-control review honestly. Software-agent review is always distinguished from actual human/domain acceptance.

Concurrent sessions and a window's selected actor are different concerns. A new reviewer login must preserve unrelated author sessions. Changing the selected actor must withhold late output and prevent an in-flight operation from crossing that selection change. Logging out one session must not revoke all other sessions unless an explicit wider revocation applies.

## Existing owners to reuse and gaps to close

Read the [bounded source audit](../../qa/implementation-2026-10-26/biosphere-authoring-round1/human-agent-authority-audit.md) and its pinned source ranges. Existing QualiaDB owners include cooperative-core `AgencyDelegation`, Core delegation modalities, app-contract task delegation, native signed project admission and QDNF authority. These must be composed and qualified rather than replaced by a Poet-private hierarchy.

The current native `SubjectAdmission.kind`, membership, issuer signature and key-possession proof establish an admitted actor, not a complete human-to-agent delegation chain. Current engineering fixtures signed for `SoftwareAgent` actors demonstrate source/role/session behavior only. Do not migrate them to purported human evidence by changing a tag.

Known audit concerns must remain tracked until repaired and executed against their intended callers: the foundation CRDT delegation verifier does not verify the supplied cryptographic proof; cooperative delegation evaluation alone does not bind all requesting-agent/purpose/action constraints; low-level modality results depend on trusted graph inputs; and generic QDNF binding attenuation needs qualification before it is used for the richer hierarchy. These observations do not establish a public bypass of NativeAuthority's privately retained fixed session binding.

## Dependency-ordered implementation and acceptance

| ID | Work | Acceptance evidence required | Current state |
|---|---|---|---|
| AUTH-H1 | Pin reusable owners, actual callers and unresolved authority semantics; define native adoption/serialization migration | Source/API audit; reviewed human/agent distinction and rights contract | Source audit and contract recorded; contract acceptance open |
| AUTH-H2 | Verify adopted human-to-agent binding and scoped authority through the existing native owner | Positive admitted chain; forged/wrong-human/wrong-key/unadopted/replayed/foreign-project refusal; legacy migration refuses invented delegation | Missing integration |
| AUTH-H3 | Enforce bounded attenuation, expiry, cancellation and ancestor revocation | Parent/child rights intersection; depth/cycle/widening refusal; revoked/expired ancestor blocks commit and delayed publication | Missing integration/qualification |
| AUTH-H4 | Apply the same chain to Poet UI, Vibe/MCP/jobs, source/HMC/resources and review | Same operation/rights decision across all routes; stale/revoked/denied cases preserve accepted work | Missing integration/qualification |
| AUTH-H5 | Preserve actor/principal attribution and reviewer independence | Two agents sharing human control are detected; actual human/domain acceptance remains separate; records survive reopen | Missing integration/qualification |
| AUTH-H6 | Native usable human enrolment/delegation/review workflow and recovery | Actual authorised operator/reviewer trial, clear scope/expiry/revoke controls, durable source-to-output evidence | Open; no human trial claimed |

H1–H6 are child obligations of the existing authorisation/cooperation gates. Source presence, new tests, passing diagnostic software fixtures and concurrency improvements do not close them. Asset production remains HOLD until the relevant production foundation and acceptance gates pass.

### Specific unimplemented verification seams

These are tracked repairs, not newly dispatched workers or proof that every affected consumer was audited. Do not make an unrelated consumer's repair a prerequisite for Poet unless the selected authority path depends on it.

| Repair ID | Existing owner / required action | Required verification | State |
|---|---|---|---|
| AUTH-H2-V1 | Core foundation `verify_delegation` and governance `verify_root_delegation`: separate structural checks from actual proof/root verification; retain clear compatibility for structural callers | Forged, empty, wrong-key/root/context and expired proof refusal; positive adopted proof; actual selected caller tests | Missing; these helpers are not admitted as the Poet hierarchy gate |
| AUTH-H2-V2 | Cooperative `AgencyDelegation` and app-contract `TaskDelegation` adapter: bind requesting agent/controller, principal adoption, purpose, action/data scope, command/effect mode and budget enforcement | Correct-agent success; foreign-agent/purpose/action, unapproved mode, expired and exhausted-budget refusal | Missing native composition |
| AUTH-H2-V3 | Existing signed agency persistence caller: propagate failed finalisation/checkpoint outcomes instead of discarding them | Failed persistence cannot be reported as completed durable adoption; retained source and reconciliation evidence | Missing selected-caller qualification/repair |
| AUTH-H3-V1 | QDNF authority record/permit consumption: pin scope and operation with the admitted grant before relying on it for hierarchical attenuation | A supplied binding cannot change admitted scope/operation; preserve fixed native session binding behavior and existing grant lifecycle | Missing richer-binding qualification/repair; no public NativeAuthority bypass established |

The [audit](../../qa/implementation-2026-10-26/biosphere-authoring-round1/human-agent-authority-audit.md) identifies the exact inspected owners, caller boundaries and source ranges for each repair. The hierarchy adapter must not treat these incomplete checks as verified permission.
