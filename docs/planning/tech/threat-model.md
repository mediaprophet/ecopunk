# Threat model — adversarial considerations for the corpus

**Status:** draft v0.1 · threats against the corpus, contributors, and
distribution — and which are already answered structurally vs which need work.

Engine-verified substrate that anchors this: `net/qdnf/authority/policy.rs`
(*"no universal join key, reputation score, or authority from similarity,
wallet possession or nominal signature count"*; `PolicyOutcome` includes
`Challenge`/`NeedsHuman`/`Incomplete` — fail-honest), resource-budget ledgers
(`sybil_does_not_multiply_host_admission` — a *test*), `CONTEST_CONTEXT`,
fail-closed licence/sensitivity gates, content-addressed packs.

---

## 1. Threat register

| # | Threat | Vector | Structural answers (verified) | Residual work |
|---|---|---|---|---|
| T1 | **Sybil contributors** — fake identities multiply influence | sock-puppet ratifications, vote-stuffing standing | Authority can't derive from signature counts or similarity (policy.rs axiom); budget-ledger per-host admission; standing is *qualified* (SOC-AX-3), not numeric | **G29** — contributor-standing accrual spec (how does reputation legitimately form? evidence + community recognition, not counts) |
| T2 | **Corpus poisoning** — false records enter as truth | bogus observations, fabricated identifications | Proposals aren't assertions; ratification ladder; `proposesClass` + human correction stream | reviewer standing matrix (review-workflow §2); anomaly detection on proposal patterns |
| T3 | **Poisoned alignments** — flood of bogus `closeMatch` | degrade graph navigability, launder false equivalence | `exactMatch` type-forbidden upstream; proposals can't self-ratify | **G30** — flood detection (rate/entropy signals on proposal streams) |
| T4 | **Contest abuse** — DoS via contest records | legitimate data drowned in disputes | contests carry reason + standing; unreasoned-flood is anomaly signal | **G31** — contest-rate governance + escalation paths |
| T5 | **Pack tampering / substitution** | corrupted or swapped artifacts in distribution | content digests + `.hmc` SHA-256/CRC-32C + `did:` identity = substitution detectable | signature chains on packs (who signed this build) — verify coverage |
| T6 | **Identifier takeover** — DNS/domain compromise (lived scenario) | service references hijacked | canonical identity is digest + `did:`, HTTP is timestamped alias | G12/G15 resolver+node-identity conventions |
| T7 | **Governance capture** — stewards or reviewers compromised | rights-floor erosion from inside | authority human-rooted + auditable; breach records to `meta_deontic` (court-admissible); contests preserve allegations-as-quotes until adjudication | quorum design per standing domain; revocation cascade drills (testnet) |
| T8 | **Sensitive-data exfiltration** — species/coordinate/custodial leaks | poaching-vulnerable coords, custodial knowledge, people-in-frame | sensitivity classes in-record (context bits); fail-closed unknown licences; consent-gated biosense; DP aggregates possible | ingest-side detectors (G25 people-in-frame); coordinate-precision policy per class |
| T9 | **Dependency/API capture** — upstream source feeds poisoned or altered silently | a source changes content retroactively | snapshot + hash + retrieval provenance; source_catalogue `Unverified`/`Restricted` statuses | drift-detection on re-fetch (hash-compare cadence) |
| T10 | **Economic attack** — spam obligations, wash-hosting for micropayments | obligation-pool manipulation | capped-ROI + discharge rules; usury circuit-breaker; category-scoped multipliers; budgets independent (energy/time/compute never mix) | settlement-rail specifics once G11 lands |
| T11 | **Coerced contribution** — contributors pressured to surrender rights/consent | asymmetric-power relations | `CoercedConsentFlag` + heightened-validity rules (GDPR-Recital-43-style); asymmetric-power detection in `spc:` | contribution-flow UX must surface the coercion check, not bury it |
| T12 | **Model adversarial inputs** — crafted images/audio fool classifiers | proposal streams polluted at inference edge | proposals stay proposals; modelDigest recorded per observation | confidence-distribution monitoring; adversarial-input awareness in classifier packs |

## 2. The security posture this encodes

- **Detection-over-prevention where prevention is impossible** — you can't
  stop someone submitting a false observation; you *can* make it a proposal,
  attributed, reviewed, and permanently accountable. Accountability is the
  control; prevention is a bonus.
- **Standing can't be bought or farmed** — reputation/authority are evidence
  + community-recognition based (SOC-AX-10), never wallet/similarity/count-
  derived (policy.rs axiom). The interesting question isn't "how to stop
  sybil" (designed out) but "how legitimate standing accrues" (G29).
- **Failure modes are honest** — `Challenge`, `NeedsHuman`, `Incomplete` are
  policy *outcomes*; the system routes to humans rather than fake certainty.
- **Every abuse pattern leaves evidence** — contests, labour quins, breach
  records, provenance chains — auditability is the deterrent substrate.

## 3. Residual-work summary

| Priority | Item | Why first |
|---|---|---|
| High | G29 contributor-standing accrual | everything social depends on it |
| High | T8 ingest-side sensitivity detectors | a leak is un-unleakable |
| Med | G30/G31 flood/abuse detection | scales with public contribution |
| Med | T5 pack signature-chain coverage | trust at distribution edge |
| Low | T9 source-drift detection | re-fetch cadence tooling |
| Low | T12 adversarial-input monitoring | classifier maturity first |
