# Pre-funded credit, periodic reconciliation and provider-directed residuals

**Status:** specification draft v0.1 · **Clarified:** 2026-10-24.
Timothy requires both period-based reconciliation of agreed obligations and a
positive **pre-funded allocation** model. Providers may authorize an allocation
like a subscription, draw it down through permitted uses, and direct unused
funds back to themselves or toward a selected cause/purpose. The allocation is
not permission to place the provider in unagreed debt.

This extends the existing [semantic agreements/instruments](semantic-agreements-and-compensation.md)
and [cooperative/commons](cooperative-layer.md) mechanisms. No balances are read,
funds moved, subscriptions activated, tokens issued or contract terms adopted by
this specification. All examples below describe intended behaviour.

## 1. Existing native mechanisms and scope

| Inspected QualiaDB source | Evidence / reuse boundary |
|---|---|
| `crates/qualia-cooperative-core/src/finance.rs` | Unique-ID immutable ledger entries and derived per-currency balances; source tests cover duplicate/reordered entries. `amount_cents` is signed minor-unit money, not arbitrary token precision. Reuse records; qualify valuation/precision mapping and actual backing. |
| `crates/qualia-core-db/src/net/qdnf/economics/obligation.rs` | Stable obligation identifiers, target, settled recovery, authorized non-cash discharge and holds; source invariant `S + W + H <= T`. Bounded operation records support idempotency. This owner is not a copy of a balance or a provider's spending allowance. |
| Same directory: `reserve.rs`, `settle.rs`, `recover.rs` | Reserve/cancel/expire/finalise/reverse and provider-outcome reconciliation source exists. Operating cost, fees and obligation recovery are distinct quote fields. Qualify reuse rather than building parallel accounting. |
| Same directory: `adapter.rs` | Capped micropayment aggregation and in-process settlement source exists. The inspected live Stripe/ILP/generic rail entry points explicitly return `Unsupported`; in-process settlement is not evidence of an external payment. |
| Same directory: `consent.rs`, `router.rs` | Paid credit, consent budget, funding source and resource-budget/router mechanisms are distinct. Payment does not enlarge consent or host capacity. Reuse these boundaries in the allocation contract. |
| `crates/qualia-client-core/src/wallet/ledger.rs`, `semantic_tokens.rs` | Timestamped wallet event records, micro-cent amounts and token/payload metadata source exist. A queued/broadcast event or token payload must not be treated as final settlement; qualify the actual selected rail and protocol. |

Paths are relative to `C:/github/qualiaDB`. This review did not run those source
tests or establish a complete deployed credit/subscription or calendar-close flow.
Integration, persistence and target coverage remain to qualify.

## 2. Contract requirements

| ID | Requirement / acceptance |
|---|---|
| REQ-CREDIT-001 | An allocation must identify provider/principal and authority, actual funding/backing evidence, unit/precision, beneficiary/project/purpose scope, limits, period and applicable agreement/policy versions. A promised amount is not an available funded balance. |
| REQ-CREDIT-002 | Repeating/subscription-like allocations require explicit provider instructions for amount, cadence, renewal/replenishment caps, start/end and cancellation. A changed rate, purpose, owner or term cannot silently expand the grant. |
| REQ-CREDIT-003 | Admit usage against a bounded quote and atomically reserve available authorized funds; finalize actual consumption or release/cancel unused holds. Cap the full permitted quote, including separately declared operating fees and recovery, not just the obligation component. Exceeding funded authority requires a separate explicit agreement; it cannot silently become debt. |
| REQ-CREDIT-004 | Keep funded, available, reserved, consumed/settled, returned, delegated and disputed/reversed states distinct, with immutable event IDs and unit-safe accounting. Concurrent, replayed, delayed and offline operations cannot spend or return the same allocation twice. |
| REQ-CREDIT-005 | At the agreed cutoff reconcile the period, settled uses and outstanding holds/disputes, then identify the residual actually available. Provider instructions govern return, carry-forward or delegation to a named cause/purpose within the grant. Untaken credit is not automatically provider-independent revenue, a donation or renewal. |
| REQ-CREDIT-006 | Residual return/delegation needs amount, recipient/purpose, provider authority, fee treatment and actual status/receipt. Unresolved disposition remains attributed/accounted to its source pending valid instructions. Preserve separate provider, purpose and currency/token tranches when pooling; delegation grants only its stated scope. |
| REQ-CREDIT-007 | Periodic contracts for separately accepted obligations must define parties/authority, basis and valuation, unit, accrual/cutoff/due dates, payment allocation, partial settlement, disputes/corrections and any agreed netting. Carry forward legitimate remaining balances without creating duplicate debt or unagreed fees/penalties/renewal. A period statement or acknowledgement is not payment. |
| REQ-CREDIT-008 | Tokens are optional representations/transport for allocation, obligation, acknowledgement, entitlement or settlement evidence; identify which role each has. Compare native signed records, batching/netting and supported rails against a declared total cost/fee budget. No chain or supposedly cheap token is mandated without measurement and compatibility evidence. |
| REQ-CREDIT-009 | Token amounts/decimals, money minor units, micro-units, fees and valuation must map explicitly without silent rounding or invented exchange rates. Issuing/transferring an obligation token does not count as compensation unless the operative agreement accepts the specific evidenced settlement/non-cash discharge event. Queued, broadcast and final states remain distinct. |
| REQ-CREDIT-010 | A provider's unused allocation and a contributor's outstanding project obligation are separate balances. Pre-funding/reservation is not payment to a contributor; no-charge human-rights use is not a settlement debit. Apply actual eligible payments to the existing bounded pool and contributor records under policy, preserve attribution and stop its recovery charge at discharge. Authorized funding can support permitted human-rights activity without charging its beneficiary. |
| REQ-CREDIT-011 | Payment cannot enlarge consent, data/device authority or host resource budgets. Preserve scoped private records, refusal/cancellation, service loss/restart recovery and exact contract revisions through native/WASM operation; no fake live-rail success or duplicated residual action. |
| REQ-CREDIT-012 | Qualify period rollover, renewal off/on, fees, exhausted credit, multiple providers, held/disputed work, duplicates, reversal, partial/final/zero-cost settlement and return/delegation/carry-forward. Include more events than the bounded operation table holds, with qualified checkpoints/idempotency and retained audit history. |

The provider allocation answers **what spending has been authorized and funded**.
The contribution/commons obligation answers **what compensation remains due**.
The settlement record answers **what actually discharged which obligation**.
Keep all three linked without replacing one with another.

## 3. Period close and residual example

Illustrative units only; this is not an account, valuation or payment receipt.

| Event/state | Consumed/settled | Reserved | Available | Returned | Delegated |
|---|---:|---:|---:|---:|---:|
| Provider funds 100 units for the period | 0 | 0 | 100 | 0 | 0 |
| Approved use settles 60; another use holds 10 | 60 | 10 | 30 | 0 | 0 |
| The held operation cancels and releases 10 | 60 | 0 | 40 | 0 | 0 |
| Provider directs 20 back and 20 to a named purpose; both actions settle | 60 | 0 | 0 | 20 | 20 |

The allocation remains reconciled to 100. A still-held amount would not have
been freely disposable. A failed or pending return/delegation must retain its
actual state and source attribution rather than pretending the action settled.
Where rail fees make a tiny residual impractical to move, use the provider's
agreed fee limit and authorized carry/aggregation/disposition policy; preserve
the balance while an unresolved instruction is settled.

For periodic debt/obligation reconciliation, show opening balance, valid new
accruals, verified credits/settlements, corrections and closing balance with
the exact statement period and event references. Netting requires the relevant
parties and obligations to authorize it. Eligible human-rights no-charge uses
must not silently accrue a new personal debt.

## 4. Work packets within AGR-11

Priority follows [implementation-plan.md](../implementation-plan.md): retain
clear native allocation/period/receipt/adapter contracts early, while live rails
and full funding-product qualification are later IP-08 support work. They do not
block the first usable authoring/high-grade asset milestones. Required real
payment/refund/delegation functions remain open until their selected rail and
evidence actually work; fixture success cannot close them.

All remain open; these refine existing ECO packets and native owners.

| Packet | Deliverable / evidence | Allocation |
|---|---|---|
| FUND-01 | Versioned allocation, period, accepted-obligation and provider-disposition semantics mapped to existing project/finance/commons/instrument APIs. Explicit unit/authority/backing, source-policy and migration cases. | ECO-024/034/044; AGR-02/03/04 |
| FUND-02 | Native funded-allocation/quote/reservation/drawdown integration, with independent resource/consent limits and bounded recovery. Actual precision, concurrency, cancellation and no-overdraft cases. | ECO-111/112; AGR-05/06 |
| FUND-03 | Deterministic period statements, authorized accrual/credit/netting, late events, due/partial/disputed balances and durable rollover/checkpoints without duplicate claims or payments. | ECO-111–113/119; AGR-06 |
| FUND-04 | Provider-controlled residual return, carry and purpose delegation, with separate tranches, actual status/receipts, fee limits and failure/revocation/restart cases. | ECO-049/111–113/116; AGR-06/08 |
| FUND-05 | Optional token/rail adapter and cost comparison; qualify malformed metadata, signatures/status, actual funding/settlement, tiny amounts and unsupported rails. No issuance or transfer is implied by source presence. | ECO-034/038/049/112; AGR-07 |
| FUND-06 | Native Poet/host and required WASM end-to-end funded project case across two periods, partial usage and chosen return/delegation; separately demonstrate an explicitly accepted obligation contract and applicable no-charge use. Human/evidence reviews retain all unresolved cases. | ECO-050/100/114–120; AGR-08/10 |

Open: actual unit/precision and funding instruments; calendar/cadence/timezone and
statement-finality rules; permitted netting; provider residual and fee defaults;
budget/resource owner integration; persistence/checkpoint mechanism; rail/token
costs and available live adapters; exact native/WASM and UI scope. No interest,
liability, exchange rate or market price is inferred by this technical contract.
