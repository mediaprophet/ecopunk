# Semantic agreements, instruments and fair compensation — existing-work handoff

**Status:** inspected-source / requirements draft v0.2 · **Date:** 2026-10-24.
Timothy identifies four connected bodies of work: `C:/github/license`, semantic
instruments in QualiaDB, `C:/github/humanitarian-equity-framework`, and the earlier
`C:/github/semantic-tokens-xec`. Their purpose includes ontology-defined agreements
that preserve and fairly compensate useful contributions, particularly SDGs work,
and address the exploitation of unpaid work described as “digital slavery”.

Treat these as existing foundations to reconcile and advance. Do not replace
them with a private Ecopunk billing, licence, token or rights engine. This review
inspects selected local files; it does not execute generators/tests, certify
current builds, adopt licence terms, issue tokens, transact or edit sibling code.

Companions: [production backlog](ecopunk-production-backlog.md),
[purpose and civics terminology](high-grade-production-and-civics-experience.md),
[cooperative layer](cooperative-layer.md), [fair compensation](compensation.md),
[contribution chains](contribution-chains.md),
[semantic interpretation](interpretation-frameworks.md),
[HMC contract](assets/asset-package-spec.md).

## 1. Source inventory and evidence limits

Paths below are relative to `C:/github/`. README and tracker descriptions remain
source claims; inspected APIs are source presence, not new runtime passes.

| Source | What is present / required interpretation |
|---|---|
| `humanitarian-equity-framework/README.md` | Declares an early framework for useful-work attribution, contribution valuation, obligation costs and eventual compensation-linked release. Distinguishes inspectability from permission and records incomplete historical material. Preserve this purpose and history. |
| `humanitarian-equity-framework/ontology/{Contributors,cost-model,licenseontology,projectClaims,projectrules}.ttl` | Draft contribution/cost/licence/claim/rule vocabularies exist. Inspected files include provisional `example.org` identifiers. Validate syntax, namespaces, units and cross-vocabulary mappings before production import; file presence is not a qualified ontology release. |
| `license/README.md`, `RIGHTS-ONTOLOGY-AND-INSTRUMENTS.md` | Rights-centred agreement methodology separates normative authority, rights positions, legal instruments, technical instruments and remedies. Repository explicitly labels production terms unapproved and examples/placeholders incomplete. Source presence does not adopt them into another project. |
| `license/ontology/rights-instrument-model.ttl`, `contexts/rights-context.jsonld` | Provisional `hro:` vocabulary (`https://ns.webcivics.net/license-rights/`), ODRL/PROV and Web Civics jural/agency/social-commons references, rights statements, creator investments, instruments and SHACL shapes. Retain source/namespace/version provenance. |
| `license/src/license_generator.py`, `schemas/license-profile.schema.json`, `GENERATOR.md` | Dependency-free Python profile validator/document assembler source; generated bundle includes contract/schedules, JSON-LD policy, validation report, acceptance-receipt template and hash manifest. Validation/adoption checks exist in source. No generator was run here; a receipt template is not evidence anyone accepted terms. |
| `license/CONTRIBUTOR-TERMS-DRAFT.md`, `RIGHTS-REGISTER.md` | Draft contributor/adoption/rights-history records; source explicitly distinguishes historical lineage from ownership proof. Reconcile with cooperative contributor records instead of assigning every contributor's work to the current host or funder. |
| `semantic-tokens-xec/README.md` and nested application | Earlier RDF + IPFS + XEC/SLP project. Nested React application contains RDF/SHACL editing, browser/Solid/IPFS-related source and historical app versions. Preserve as a design/protocol reference; it is not qualified native implementation or proof of transaction/compensation success. |
| `qualiaDB/core-ontologies/{semantic-instruments,semantic-instrument-shapes,semantic-instrument-capability-profile}.n3` | `si:` vocabulary distinguishes instrument identifiers, immutable release, HCF/HMC package, visual badge, entry point, dependencies, capability evidence and results. An instrument is not its author, learner, badge or credential. |
| `qualiaDB/.../src/semantic_instruments/mod.rs` and its modules | Native package/build/open, manufacture/publish gates, catalogue, dependency resolution, execution/preflight/receipt, RPL and attestation APIs exist. Qualify the agreement/compensation entry points actually needed; generic packaging does not establish every model executes. |
| `qualiaDB/.../src/poet_host/invoke/semantic_instrument_bridge.rs`, `tests/semantic_instruments*.rs` | Poet bridge plus source tests/fixtures for package opening, typed execution/outcomes, dependency and identifiers distinctions. Inspected bridge includes demo/reference handling. Reconcile current implementation with older WIP status; no conformance run is claimed here. |
| `qualiaDB/core-ontologies/{jural,social-permissive-commons,permissive-commons}.n3` | Existing rights/social-relation/commons vocabularies. Commons describes bounded cost recovery, permanent attribution and economic discharge. It also retains an older collective-pool/per-agent-debt open choice; reconcile against Timothy's current pool-discharge direction. |
| `qualiaDB/crates/qualia-cooperative-core/src/projects.rs` | Projects carry `licensing_ontologies`; membership, immutable contribution/predecessor chains, unique-ID merge and derived effort/capital/score records exist. An ontology-URI field is a reference carrier, not proof that a full adopted agreement is resolved and enforced. |
| `webizen-desktop/src/commands/wellfair/projects.rs` | Native project creation receives the ontology list and calls `host.add_project`; project/obligation commands exist. Reuse this integration, qualify persistence/context and connect actual agreement semantics rather than inventing a parallel project registry. |
| `qualiaDB/.../src/modalities/socioeconomics.rs`, `poet_host/invoke/logic/governance_workbench.rs` | Integer cost/royalty/pool/outstanding/discharge/ancestry routines and a Vibe-facing governance adapter exist. Arithmetic is not proof of contribution valuation, receipt authenticity, duplicate-payment rejection or maximum collectible-charge enforcement. |
| `qualiaDB/.../src/net/qdnf/economics/{obligation,reserve,settle,recover,adapter,consent,router}.rs` | More developed native obligation ownership, capped holds/settlement, duplicate operation handling, reversal/provider reconciliation, micropayment aggregation and independent paid-credit/consent/resource budgets exist. Reuse these paths; inspected live rail entry points return `Unsupported`. In-process results do not establish external payment or the full period-credit workflow. |
| `qualiaDB/docs/manuals/adr/0003-permissive-commons-billing-gates.md`, routing/gating source | Accepted ADR describes native economic/access lanes; matching symbols occur in VM, publication, service and routing sources. Verify actual actor/policy/receipt binding and every relevant ingress; do not promote the ADR's “unbeatable” security claim to evidence. |
| `poet/wasm/src/browser/agreement_views/` | Agreement, licence, contribution and compensation surfaces exist in canonical Poet; inspected files contain example/static values and a live governance action. Preserve/port the useful UX, but prove real records and native outcomes rather than treating sample signing progress/prices as evidence. |
| `qualiaDB/.../src/domain_licence/` | Signed domain/capability licence format and verification APIs exist. This is a scoped technical grant mechanism; it does not alone establish a general agreement, agreed compensation or actual settlement. |
| `qualiaDB/crates/qualia-client-core/src/wallet/semantic_tokens.rs` | Rust semantic-token metadata/SLP payload helper exists. Inspected invalid-hex handling substitutes zero bytes. Qualify malformed input and actual protocol/transaction evidence before calling an output a verified token or payment. No wallet operation is performed here. |

`.../src/` above abbreviates `qualiaDB/crates/qualia-core-db/src/`.
Source research does not establish a byte-identical lineage or completed migration
among these repositories. Map those relationships, including existing tests and
receipts, before copying or rewriting source.

## 2. Distinct roles in one production path

| Layer | Existing role to retain |
|---|---|
| Humanitarian Equity Framework | Contribution/economic-justice foundations and historical useful-work/obligation models. |
| Licence/rights profiles | Define proposed or adopted relations, parties/capacity, acts/resources/purpose, conditions, authority, provenance and lifecycle. |
| Semantic instrument | Versioned, dependency-bound model/rules/method and typed entry points for evaluation; links inputs and outputs to the exact model used. |
| Qualia cooperative/commons mechanisms | Authoritative project/membership/contribution/valuation, applicable cost pool, usage/settlement and discharge records through shared native interfaces. |
| Credentials and semantic tokens | Evidence, entitlement or protocol carriers with identified scope/issuer/status; neither token possession nor a signature invents missing authority or proves fair compensation. |
| Poet / Webizen / HMC consumers | Author, inspect, review, invoke and preserve the same records and policy/resource lineage across native/WASM use and high-grade-master derivation. |

The required chain is: identified contribution and source rights → reviewed
agreement/profile and adopted release where applicable → exact semantic-instrument
definition/dependency lock → typed native evaluation → scoped decision and
receipt → authorized use/verified settlement → updated outstanding/discharge
state, with provenance and a contest/correction path throughout.

Draft terms, adopted terms, technical grants, proposed compensation, verified
payment, a permitted no-charge use and pool discharge are different states.
Keep mandatory protections, actor consent/capacity and source terms explicit;
an executable ontology or SDG label is not permission to override them.

## 3. Requirements

| ID | Requirement / acceptance |
|---|---|
| REQ-AGR-001 | Reconcile existing repositories and canonical owners before implementation: version/digest the source ontologies, profiles, native APIs, UI and token protocols, with tested/untested status and preserved authorship. Do not assume older WIP means no implementation exists. |
| REQ-AGR-002 | Define loss-aware mappings among framework contribution/cost vocabularies, `hro:`, `jural:`, `spc:`, `pc:` and `si:`. Identify actor/capacity, instrument/release/package, source resource, act/purpose/context, valuation units, authority and lifecycle; reject unknown required mappings rather than silently flattening them to labels. |
| REQ-AGR-003 | Agreement authoring must preserve exact reviewed/adopted terms, party/capacity/authority and actual acceptance evidence, separately from generated draft templates. Bind semantic decisions to the operative version; later edits or government/institutional adoption do not rewrite earlier contributions or imply consent/payment. |
| REQ-AGR-004 | Fair compensation must use explicit unit-safe valuation and contributor/source evidence, actual compensation, outstanding amount and contestable policy. Effort, currency, capital and non-cash resource costs cannot be added as if they already share a unit of account; apply the existing compensation model and native owners. |
| REQ-AGR-005 | Commercial recovery and applicable human-rights no-charge cases must follow the identified cooperative/commons policy, preserving bounded pools, ancestry and permanent attribution. Qualify partial/final/zero-cost/gifted states, no-charge versus waiver, duplicate receipts, cap crossing and post-discharge use; no automatic new per-agent debt or ancestor-pool reset. |
| REQ-AGR-006 | Native evaluation must resolve the actual agreement/instrument releases, source rights, dependencies and authority, return bounded typed outcomes and meaningful evidence, and connect to the real admission/use path. Unknown/incompatible/stale/missing required terms cannot be represented as a completed fair-use decision. |
| REQ-AGR-007 | Credentials/token/ledger records must bind issuer, subject/resource, agreement/release/digest, scope, validity and verified event/status as appropriate. Token artwork, a metadata file, zero-hash fallback or a transaction identifier alone cannot certify assent, entitlement or settlement. Historical React/CDN clients cannot become required JS/V8 dependencies of the native workflow. |
| REQ-AGR-008 | Native Poet must expose actual agreement/contribution/valuation/review/evaluation records and results, with shared Vibe/Qualia interfaces and honest demo/reference/unavailable states. HMC/master/derivative references preserve policy and compensation lineage while private records retain their existing access controls. |
| REQ-AGR-009 | SDGs/humanitarian work must preserve useful-work attribution, actual funded/unfunded cost and fair-compensation claims without treating humanitarian purpose as coerced free labour. Qualify eligible no-charge use without false payment/discharge or erasing creator obligations. Report measured cases and unresolved gaps; the anti-exploitation goal is not an already-achieved software claim. |
| REQ-AGR-010 | Support period-based reconciliation of separately accepted obligations and provider-defined pre-funded allocations, with drawdown and provider-controlled residual return/carry/purpose delegation. [REQ-CREDIT-001–012](credit-allocations-and-periodic-reconciliation.md) qualifies native accounting, no unagreed debt, optional low-cost token/rail evidence and actual period/settlement states. |

## 4. Concrete reconciliation issues

- **Valuation units:** inspected `derive_obligations` computes
  `(effort_minutes + capital_cents) × roi_multiplier` into a score. Preserve its
  current source/evidence but define and qualify the rate/unit mapping before
  presenting that score as fair monetary compensation. Do not silently rename
  it “amount paid” or “amount legally due”.
- **Pool semantics:** the current user direction is recovery until the relevant
  obligation cost is compensated. Older ontology alternatives remain to reconcile;
  a discharged collective work must not silently become a fresh debt per consumer.
- **Zero cost and credits:** `is_commons_discharged` currently requires `cost > 0`;
  `pool_after` adds amounts. Explicitly qualify zero-cost/gifted status, external
  receipt deduplication and collectible-charge caps in the owning policy path,
  rather than assuming those arithmetic helpers establish all of them.
- **Use the developed owner:** QDNF already has a finite obligation record with
  `settled + authorized non-cash discharge + holds <= target`, reserve/finalise/
  reverse/reconcile and aggregation source. The earlier arithmetic observations
  are not a claim that the entire stack lacks caps or idempotency. Qualify their
  binding to actual valuation, backing, period records and selected live rails.
- **Draft versus adopted:** `license` states that production terms are unapproved
  and examples contain placeholders. Preserve this status until actual adoption;
  generator validation or a model's packaging pass is not an adoption event.
- **Generic instruments versus agreement results:** native instrument code exists,
  but a demo conversion result, badge or reference package cannot establish a
  production compensation-policy decision. Select the real entry point and inputs.
- **Historical tooling and token evidence:** recover RDF/shape/rights authoring
  and protocol intent without importing a required browser/Node runtime. Validate
  metadata and actual event provenance; never fabricate credentials, settlement
  receipts, balances or transaction success in a demonstration.

## 5. Dependency-ordered work packets

The [governing implementation plan](../implementation-plan.md) starts project,
source-rights, contribution and review records in IP-00/03. Native instrument/
agreement/credit qualification continues on IP-08's support track. Payment rails
must be supportable but are not the first-output priority; do not postpone usable
cooperative authoring and exceptional assets until live financial adapters exist.

All packets remain open; they extend existing ECO packets and engine G87, rather
than replacing the 120-packet production index or the native cooperative system.

| Packet | Deliverable / completion evidence | Existing allocation |
|---|---|---|
| AGR-01 | Source/history/status and capability map across the four bodies of work plus native host/Poet. Preserve exact vocabulary/profile/artifact versions, existing passes and missing coverage. | ECO-002/003/007/009 |
| AGR-02 | Ratified ontology/profile/semantic-instrument mappings, units, state transitions and source-rights/authority boundaries. Resolve old pool-choice and example namespaces with versioned migration cases. | ECO-021/024/034 |
| AGR-03 | Unit-safe native fair-value/obligation/actual-compensation calculation with declared benchmarks/rates, currency/resource units, caps and disputes; prove the minutes/cents case cannot become a misleading monetary total. | ECO-044/111/112; existing compensation owners |
| AGR-04 | Native authoring/import/review/adoption and exact terms/acceptance/version records; no implicit adoption of repository drafts. Reuse/qualify the generator and native ontology/codec paths, with implementation home explicitly allocated. | ECO-025/044/049/050 |
| AGR-05 | An actual agreement-policy instrument package, dependency lock, typed invocation and admission/use decision through existing native runner/Vibe APIs. Positive/negative cases retain exact inputs, outputs and receipts. | ECO-027–030/034/111 |
| AGR-06 | Cooperative usage/settlement/discharge integration: commercial, allowed human-rights no-charge, partial/final/zero-cost/gifted, cap crossing, duplicate/reordered/revoked/disputed events and ownership/derivation changes. No real transfer is inferred from a fixture. | ECO-111–113/119 |
| AGR-07 | Native semantic-token/credential protocol and issuer/status/data binding, with malformed/wrong-digest/expired/revoked cases. Map historical XEC/RDF/IPFS work to current mechanisms; actual issuance/transactions are separately allocated actions. | ECO-034/038/049/112 |
| AGR-08 | Restore useful agreement/instrument/contribution/compensation UI inside native Poet and shared WASM projections, backed by real authoritative records. Replace hardcoded example state with honest live or explicitly labelled fixture state. | ECO-037/041/045/049/050 |
| AGR-09 | Bind high-grade master, complete HMC, reduced derivative and institutional-use lineage to operative terms, contribution/pool references and permitted-use outcomes without leaking private ledgers or losing full-embodiment semantics. | ECO-024–030/079/104/115/116 |
| AGR-10 | Qualify an end-to-end SDGs/civics production case: useful work → terms/instrument → native evaluation → authorized commercial/no-charge case → verified or explicitly simulated settlement → derivative and preserved attribution. Record distinct native/WASM, human review and remaining cases. | ECO-094/100/114–120 |
| AGR-11 | Qualify [funded credit and periodic reconciliation](credit-allocations-and-periodic-reconciliation.md), FUND-01–06: provider allocations/limits, native drawdown, accepted-obligation statements, period close, residual return/carry/delegation and optional cost-qualified token/rail use. Keep funds, consent, compensation and outstanding claims distinct. | ECO-024/034/044/049/111–120 |

Graphic diagnostic fixtures and master authoring may progress alongside this
reconciliation. Do not substitute a new economics prototype for improving the
models, or silently treat unanswered agreement obligations as resolved. Expose
actual policy/context and allocate its missing native mechanism where needed.

## 6. Open decisions

Canonical ontology releases and mappings; exact rate/unit/valuation policy;
agreement adoption and acceptance mechanism; qualified licence-policy entry points;
pool/zero-cost/settlement state mapping; token/credential transport scope; native
authoring adapter home; HMC policy/reference encoding; which historical tests and
receipts remain applicable. The user's purpose and continued development mandate
are established; no claim is made that the inspected components already provide
the complete integrated or legally adopted system.
