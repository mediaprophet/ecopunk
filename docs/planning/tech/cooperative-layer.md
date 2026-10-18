# The cooperative layer — social commons, category-scoped sharing & obligation costs

**Status:** draft v0.1 · scope: the socially-defined networking substrate, the
category-of-agent fee/sharing rules, contribution→obligation-cost accounting
across disconnected contributor networks, and the cooperative/collaborative
work surface — as verified upstream.

**Why this matters to ecopunk:** biosphere corpora are assembled by *many
contributors across an otherwise disconnected network* — collectors, taxonomists,
data stewards, custodial communities, volunteer identifiers, pack authors.
Their obligation costs (what they're *due*) must be computable, attributable,
and survivable — and biosphere/humanitarian-class work carries a **fixed cost**
rule: obligation = work done (+ capped ROI), discharged, freed globally.

---

## 1. The social commons substrate (`social-permissive-commons.n3` — verified)

`spc:` — the general framework for any social environment of 2+ **agents**
(natural persons, legal persons, artificial agents):

- **Each agent is its own platform provider** (self-hosted) and its own data
  controller — relations are *peer agent↔agent*, not platform-mediated.
  `ExternalPlatformBoundary` marks legacy-platform interop only (GDPR-shaped).
- **The social relation is the consent envelope** — `SocialRelation` between
  2+ agents within which a `sharedHolding` corpus + governing rules form;
  use-case classes are *open* `skos:Concept`s (`Household`, `Employment`,
  `GuardianshipArrangement`, `ExternalPlatformBoundary` — extensible, never a
  closed enum).
- **Differentiated persistence** — the heart of the gating machinery:
  - `RevocableGrant` — revokes on exit (house keys, joint accounts, access)
  - `ThirdPartyAnchoredObligation` — **persists until an external authoritative
    act; anchored to a third party's interest (e.g. the child) → NO party-side
    defeater**. Two parties *cannot collude* to drop a child's guardianship —
    this is the structural answer to "gating social-web functionality in
    relation to children": the gate isn't a platform rule, it's an obligation
    anchored to the child that outlives the relation.
  - `ArtifactTravellingObligation` — persists *with the artifact* (no-publish
    rules on co-created media — image-based-abuse prevention by construction).
- **Consent as a structure, not a checkbox** — `consentScope` (consent to one
  scope ≠ consent to another), `consentWithdrawable`, `consentBasis` (**one of
  six** GDPR bases — consent is not collapsed to the only basis),
  `CoercedConsentFlag` + asymmetric-power guards (GDPR Recital 43 — consent
  under clear power imbalance is invalid).
- **SHACL-encoded rules**: artifact rules must travel; third-party-anchored
  obligations must name their anchor; asymmetric relations need heightened
  consent validity.

## 2. Category-of-agent fee & sharing rules (`socioeconomics` — verified)

`royalty(base, agent_multiplier_percent)` — **the royalty scales by agent
category** ("a corporate user pays a higher multiple than a non-profit").
Categories of entity/actor/agent therefore *define* sharing/fee terms at the
data level — matching the requirement that "ontological definitions about
sharing and/or fees can be defined by category of entity, actor or agent":

| Agent category | Example treatment |
|---|---|
| Natural person (learning/personal) | lowest multiplier / possibly free-below-threshold |
| Community / non-profit / custodial body | low multiplier; possibly fee-waiver class |
| Cooperative / commons project | reciprocity terms |
| Commercial entity | full/higher multiplier |
| Artificial agent (acting under a principal) | inherits the accountable principal's category — *no anonymous-agent discount*; authority is human-rooted (SOC-AX-14 in sociosphere.md — every agent act resolves to a human root through the delegation chain) |

Combined with `commons_cost` (production × (1+capped ROI)) and
`is_commons_discharged`, **biosphere/humanitarian work gets the fixed-cost
rule**: the obligation is bounded by work done (+ cap), and once the pool
discharges the work is free globally — humanitarian/biosphere corpora don't
become perpetual rent.

The `base` rates and multipliers this machinery consumes are not hardcoded —
they resolve through the benchmark-anchored **fair-compensation model** in
[compensation.md](compensation.md): work-type labour classes, place factors
(PPP, living-wage floors, statutory minima), and agent-kind modulation
(SOC-AX-14), all cited to statistical sources with vintage (COMP-AX-7).

## 3. Contribution → obligation-cost accounting (`qualia-cooperative-core` — verified)

This is the "difficult to advance" cooperative area — and the substrate is
further along than it looked:

- **`projects.rs`** — Restricted records for shared work: membership +
  `Contribution` records forming **author chains via `predecessor_id`**;
  obligations are a **pure derivation over the unique-id set** keyed by
  `(project_id, contributor_did)`. Contributions merge **add-wins by stable
  entry id, never by raw sum** — so "contributions of many across an otherwise
  disconnected network" are *structurally* safe: duplicated, reordered, or
  replayed sync frames can never double-count effort. **This is the
  disconnected-contributor mechanism.**
- **`work_item.rs`** — replay-safe Kanban: immutable items + separate status
  events; current status is a *derived projection*.
- **`finance.rs`** — signed replay-safe ledger; balances derived, never summed.
- **`record.rs`** — `RecordEnvelope` with `EpistemicStatus`, `EvidenceType`,
  `SensitivityClass`, `q_hash` parity (canonical hashing).
- **`agency_domain.rs`** — ~17 **domains of agency** for delegation (financial,
  medical, it-social-work, communication, social-welfare…).
- **`authority_type.rs`** — **supported-agency** authority, not custodial
  control: delegation that "amplifies the principal's personhood" (accountant,
  clinician, IT-social-worker; custodial parens-patriae = heavily-restricted,
  audited edge case) — grounded in the "Agency / Social Book" + "Digital Good
  Samaritan" works.
- **`taxonomy.rs`** — extensible term registries as *data* (open URI-keyed
  `TaxonomyTerm`s, attribute bags for ABAC/jurisdiction facets) — new agent
  categories/authority types are vocabulary additions, not code changes.
- **`qapp_package/`** — cooperative QApp manifests incl. PWA + remote controller.

## 4. Derivative attribution — the royalty tree

`ancestor_royalty` / `royalty_tree_total` (verified): obligation flows *up*
derivation chains geometrically — a derived work's royalty shares among its
ancestors by `share_percent` per generation. For ecopunk: a species pack
deriving from an ALA snapshot + a TERN survey + a custodial-knowledge
contribution + a translator's work distributes obligation **up the chain to
each contributor** — attribution and cost are computed, not declared.
A **usury circuit-breaker** (multi-agent token ceiling) caps spend projection —
the extraction guard at the multi-agent level.

## 5. How it applies to ecopunk

| Requirement | Machinery |
|---|---|
| Many contributors, disconnected network | `Contribution` author-chains + add-wins merge — sync-safe credit |
| "Reasonably due obligation costs" | derived obligations over `(project, contributor)` sets + royalty-tree flow-up |
| Biosphere/humanitarian fixed cost | `commons_cost` capped-ROI + discharge-to-free |
| Sharing/fees by agent category | `royalty` multiplier keyed to `values:Agent` classes (incl. ArtificialAgent under accountable principal) |
| Child-gating in social/educational interfaces | `ThirdPartyAnchoredObligation` — the child's interest anchors; no party-side defeater; `capacity`/`GuardianshipArrangement` use-cases |
| Social features in ecopunk interfaces | `spc:` peer relations — each user self-hosts; no platform holding the corpus |
| Coercion safety | asymmetric-power consent guards + `CoercedConsentFlag` |
| Cooperative project work (the "hard area") | cooperative-core: work items, ledgers, agency domains, supported-agency types — the substrate exists; the hard part was always the *model*, and it's been modelled |

## 6. Lifecycle custody — guardianship, probate, organisations

Storage can be decentralised while provenance stays **strictly anchored to an
accountable entity** (human or organisation) — the permissive-commons split of
custody (where bits live) from authority (whose standing governs them). But the
*lifecycle* cases differ semantically, and the vocabulary must too:

| Case | Anchor | Terminates into | Key semantics |
|---|---|---|---|
| **Guardianship (child)** | the child (`ThirdPartyAnchoredObligation`) | **the ward's own agency** — authority returns to the person; but the path is *non-standard and case-specific*, never a linear progression | multi-guardian, multi-scope arrangements (financial vs personal vs medical scopes — cf. the ~17 `agency_domain`s); **supported** vs **substituted** decision-making (`authority_type` treats custodial intervention as the audited edge); domain-scoped and *fluctuating* capacity (capable of X decisions, not Y — not a single boolean); informal/kinship care with no court order; contested arrangements held as records (`CONTEST_CONTEXT`); the ward's own will/preference participates early and asymmetrically; "maturation" is a reviewable, sometimes contested, process — not one event |
| **Digital probate** | the person's estate/instrument | **succession** — executor/next-of-kin per instrument; obligation entitlements and attribution flow to the estate; agents' delegated chains need defined death-semantics | will/instrument as the authority act; k-of-n key succession (`identity_fabric` recovery path); posthumous privacy + memorialisation are separate policies from custody |
| **Organisational custody** | the legal person | **successors / wind-up** — authority sits with the *office*, not the officer; dissolution/merger transfers assets+obligations per instruments | no maturation; office-succession, incorporation/dissolution events, and wind-up distributions are the lifecycle |

**The semantic difference that matters:** guardianship and probate are *person-
terminating* (authority returns to the person, or to persons' successors) —
human-rooted authority preserved through life and death. Organisational
custody is *construct-terminating* — authority passes to whatever the
instruments say, and the organisation itself remains a channel for its human
constituents (SOC-AX-14). Same custody machinery, different obligations:
a guardian must account *to the developing ward*; an executor accounts *to
the estate's instruments*; an officer accounts *to the organisation's
constituted authority*.

**The child's anchor has an instrument-defined floor.** `core-ontologies/`
carries the rights corpus the arrangement's anchor references: `jural.n3`
(the Hohfeld vocabulary surface of `jural.rs` — Claim/Duty, Privilege/No-Right,
Power/Liability, Immunity/Disability, `jural:opcode` 0x30–0x37 mapped into the
engine) and a two-layer instrument corpus: `un-instruments/` (102 files —
verbatim `values:originalText` + OHCHR/ICRC provenance per provision) and
`concepts/` (104 `cml:Concept` logic layers — `cml:realizedBy` the text,
`cml:asserts` a deontic norm, `values:borneBy Agent/State`, all
`cml:Proposed`/`HeuristicDerived` **pending human attestation**). The
**Convention on the Rights of the Child** is already one of these — so "the
child's interest" isn't just a free-text anchor: it can resolve to
instrument-expressed rights (plus CRPD for supported-agency contexts).
Guardianship arrangements can then be evaluated *against* a rights floor, not
only against the arrangement's own terms — noting the corpus is
machine-derived and pending jurisprudential attestation.

**Non-standardness is the norm for guardianship.** The model must not assume
one guardian, one scope, one threshold: arrangements are concurrent (several
guardians, different scopes), partially informal (kinship care, community
arrangements without instruments), contested at points in their lifecycle
(with the *contest itself* recorded), and reviewable. The ward's will and
preference are a first-class input from the start — supported-agency
(`authority_type`) is the default register; substituted decision-making is
the restricted, audited edge. Where the corpus holds data *about* a child —
observations, images, school/clinical records — those records ride the
arrangement's scope map, and transitions (a new order, a kinship change,
majority) are **events with their own provenance**, not silent state flips.

## 7. Open questions

1. Where does `spc:` sit relative to `soc:` (our sociosphere vocab)? Likely:
   `spc:` is the *generic social-relation substrate*; `soc:` carries
   place/knowledge/custodianship specifics on top — confirm no overlap-conflict.
2. The ~17 `AgencyDomain`s — enumerate and map to ecopunk contributor roles
   (collector/verifier/custodian/translator/pack-author/host).
3. Is `pc:Micropayment` bound to a live ILP rail (also open from
   durability-and-distribution.md)?
4. Cooperative-project packaging — is a cooperative project itself an `.hmc`-
   carryable record set (restricted lane)?
5. Child-gating in *pack consumption* (age-appropriate interface modes) —
   does an `entity_view` context class exist for minors, or is it `capacity`
   + observer-class on the app side?
6. Capacity semantics — is capacity modelled per-domain (per `agency_domain`
   scope) and *fluctuating* rather than boolean? What carries will-and-
   preference participation, supported-vs-substituted mode, informal/kinship
   arrangements without instruments, and contested states? Guardianship is
   deliberately non-standard — spec the shape, not a ladder.
7. Death-semantics for delegation chains — when a principal dies, do their
   agents' delegations auto-revoke, escrow to the estate, or persist under
   an executor? Needs an upstream-defined rule.
8. Probate instruments — is there a digital-will/estate ontology upstream,
   or does ecopunk mint one (probate vocabulary: executor, beneficiary,
   instrument, succession event)?
