# Review & ratification workflow — the human gate, specified

**Status:** draft v0.1 · how a proposal becomes corpus truth: who reviews,
what standing is required, how attestation lands. Machinery verified:
`cml:Proposed`→`cml:Attested` pattern (concepts corpus), `ontology_align`
`RequiresHumanReview`/`skos:closeMatch` proposals, `PolicyOutcome::NeedsHuman`,
steward quorums, `SuspendedTransactionQueue` M-of-N ratification.

---

## 1. The ratification ladder (statuses, not booleans)

Every consequential claim travels a ladder — same shape as the CML corpus:

```
proposed (machine or contributor)
   → triaged (routed to standing-holder)
      → under-review (evidence attached)
         → ratified (attested act: who, when, evidence, standing-axis)
        ↘ contested (CONTEST_CONTEXT record) → resolved
        ↘ rejected (recorded, with reason — never silent deletion)
```

Statuses are **recorded states with provenance**, not flags: a rejection is a
queryable fact about the claim and the reviewer, not an absence.

## 2. Standing matrix — who may ratify what

| Contribution kind | Ratifier standing | Quorum shape |
|---|---|---|
| Taxon identification (observation) | declared identifier + corroboration | 1 qualified + 1 corroborating (or recognised expert solo) |
| Ontology equivalence (`exactMatch`) | domain authority for *both* sides' vocabs | human attestation — never machine (type-forbidden upstream) |
| Traditional/custodial knowledge | **the custodial community's own process** — external ratification is wrong by design | community-defined (instrument: standing axes SOC-AX-3/10) |
| Licence/redistribution decisions | licence review (fail-closed default) | 1 qualified reviewer, recorded basis |
| Sensitivity reclassifications | steward/custodian for the affected class | steward quorum where existing contract |
| Pack promotion to curated corpus | corpus editors | review checklist + attestation |
| Instrument-layer norms (CML concepts) | jurisprudential review | pending upstream attestation — inherit `cml:Proposed` status |

**Standing is qualified, not boolean** (SOC-AX-3): a reviewer's authority is
`(agent, domain, axis, degree, evidence)` — a taxonomist's ratification counts
in taxonomy, not in custodianship. `find_unmet_correlatives` can audit the
matrix itself: a claim-type with no configured ratifier is a structural gap.

## 3. Review lanes (throughput vs rigour)

- **Fast lane** — low-consequence, high-volume: observation corrections,
  metadata fixes, flag reviews. Single qualified reviewer; batch attestation;
  sampling audits.
- **Standard lane** — identifications, mappings, pack promotions: evidence
  attached, reviewer standing recorded, `t`-stamped.
- **Sovereign lane** — custodial knowledge, sensitivity classes, rights-floor
  changes: routes to the *defined standing-holder's own process*; ecopunk
  tooling records the outcome, never substitutes the process.
- **Instrument lane** — CML/rights-layer attestations: jurisprudential
  review; stays `Proposed` until attested upstream or by a delegated
  authority.

## 4. Anti-abuse in the loop (ties to threat-model.md)

- Reviews are **labour quins** (`labelledBy`/`moderatedBy`) — reviewer standing
  is itself accountable; a pattern of bad ratifications is discoverable.
- `PolicyOutcome::NeedsHuman` is a designed outcome, not an error — the
  system may *route to* humans, never simulate them.
- Contest ≠ veto-by-volume: contests carry reason + standing; a flood of
  unreasoned contests is an anomaly signal, not a block.
- No silent auto-promotion: time-based "uncontested ⇒ accepted" rules are
  **prohibited** for consequential classes (identity, equivalence, custodial
  status) — absence of review stays "unratified", not "accepted".

## 5. Operational surfaces

- **Reviewer QApp** (webcivics-profile minimum): proposal inbox by standing-
  matched queue, evidence viewer, attest/reject/contest actions — each
  emitting labour + attestation quins.
- **Contributor visibility**: contributors see their proposals' status +
  reviewer reasoning; rejection reasons are required fields (teaching loop,
  not bureaucratic dead-end).
- **Audit surface**: per-pack "ratification report" — coverage %, outstanding
  proposals, contest ledger — travels in the manifest.

## 6. Open questions

1. Reviewer onboarding/accreditation process per standing axis (who attests
   the attesters — community-recognised vs credential-verified paths per
   SOC-AX-10).
2. Workload economics: does review labour accrue obligation costs like other
   contributions? (yes under contribution-chains — confirm multiplier class)
3. Escalation: deadlock between equally-standing reviewers → arbitration
   path (steward quorum? instrument-defined?).
4. Batch-attestation safety limits (max proposals per batch, cooldown
   between attestation bursts).
