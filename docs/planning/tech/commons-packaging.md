# Commons packaging — distribution, discovery & provenance on the permissive commons

**Status:** draft v0.1 · **Date:** 2026-10-17
**Scope:** the applied layer between ecopunk's packaged content (`.hmc` packs,
`.q42` volumes, `.m64` spatial tiles, `.10d` assets) and the permissive-commons
distribution fabric — how content is stored decentrally, discovered by peers,
funded by micropayments, and *provenance-verified* end-to-end; including the
licence-misrepresentation defence and the temporal semantics of packaged
objects.

Grounded in: [durability-and-distribution.md](durability-and-distribution.md)
(dual addressing, `did:`/QDNF, `pc:` rails), [cooperative-layer.md](cooperative-layer.md)
(`spc:` social substrate, category-scoped fees, royalty tree),
[governance.md](governance.md) (licence lanes, provenance chain, honesty),
[pack-delivery.md](pack-delivery.md) (verified acquisition),
[network-nodes.md](network-nodes.md) (node roles),
[reconciliation.md](reconciliation.md) (staleness as data),
[spatial-substrate.md](spatial-substrate.md) (the payloads being distributed).

---

## 1. Review: what exists, what's missing for this application

| Requirement | Existing machinery | State | Gap for packaged spatial content |
|---|---|---|---|
| Content-addressed identity | digest + `did:` canonical identity; HTTP aliases as timestamped claims | verified | none — packs inherit it |
| Decentralised bytes | QDNF replication, WebTorrent seeder, IPFS partial-CAR, Bao slices, HTTP range serving | mechanism-verified | pack *layout* must be range/co-servable — layout is our choice |
| Verified partial fetch | `Q42ByteRange`, BIDX, partial-CAR per-block CID proof; trusted pinned index | mechanism-verified; **browser gap G34 open** | spatial entries must be independently verifiable units (per-entry SHA-256 already does this) |
| Peer discovery | `manifests/*.json` registries + QSR/DID resolution + VoID self-description | spec'd | **semantic discovery** ("find packs covering this place/time/theme") — no vocabulary yet; §4 |
| Economics | `pc:Micropayment`, `socioeconomics` pools, `royalty_tree`, category multipliers; ILP dispatcher implemented | rail exists; connectors open | gating payout on attested provenance — §5 |
| Provenance | GOV-PRV chain (snapshot → turtle → `.q42` → entry → CRC) + agent graph | spec'd, strong | spatial artefacts need manifest-quin provenance (spatial-substrate §5); vendor-claim epistemics — §6 |
| Licence honesty | GOV-LIC-1 (record-level licences), GOV-LIC-2 (strictest-wins), GOV-RGT profiles | spec'd for *honest* sources | **adversarial repackaging** (vendor claiming licences upstream never granted) is not yet a first-class object — §6 |
| Social fabric | `spc:` relations, consent envelopes, custody≠control | verified | serving policies keyed to relations — §7 |
| Temporal semantics | four clock levels (manifold), REC-AX staleness, resolution history | spec'd | *distribution-level* time model for packaged objects — §8 |

The verdict: **the rails are built; the cargo manifest is not.** What follows
is the packaging shape that makes a spatial pack a first-class commons
citizen.

## 2. The commons packaging model

A commons-distributable object has three concentric addressing layers:

```
pack digest (did:q42 / content hash)
  └─ entry digest (QBDL BundleEntry: key, kind, offset, length, sha256)
        └─ interior ranges (BIDX blocks / Bao slices / .m64 Hilbert chunks)
```

Every layer is independently verifiable and independently cacheable — a peer
may hold one `.m64` tile of a terrain pack and still be a *legitimate* serving
node for it (network-nodes §1: partial custody is valid service). Packaging
must therefore be designed so that **a fragment carries enough self-description
to be trusted and attributed alone**.

Concretely, per pack:

- **Pack identity block** (durability §6.2): canonical digest + `did:` + alias
  list + resolution-history refs.
- **Layer inventory** (map-layers §6/S8): each layer is a separately
  addressable entry — `eco:layer/<family>/<name>@<version>` with its own
  licence, temporal scope, sensitivity ceiling.
- **Manifest quins per dense entry** (spatial-substrate §5): coverage WKT
  footprint, SRID+epoch, Hilbert index range, DGGS `env:inCell` assertions,
  validity/observation interval, licence union, `prov:wasDerivedFrom` chain.
  This is both the in-graph join surface *and* the network discovery surface
  (§4).
- **Vocabulary closure** (ontology-stack §7.7, GOV-DUR-4): the pack carries
  the vocab versions it depends on — a 50-year `.hmc` explains itself offline.
- **Rights profile + attribution artefact** (GOV-RGT, GOV-LIC-3): computed,
  never hand-written; the attribution list is a `kind="doc"` entry so a peer
  can answer "who do I credit" *without* fetching the payload.

## 3. Storage economics applied

`pc:` rails already define the flow: **serving bytes is a micropayment event
into the pack's obligation pool**; `commons_cost` bounds the pool to production
cost × (1 + capped ROI); `is_commons_discharged` frees the work globally —
biosphere corpora get the fixed-cost rule, never perpetual rent.

Applied to spatial payloads:

| Serving event | Payment into pool | Notes |
|---|---|---|
| `.m64` tile range-fetch (terrain stream) | bytes-served event | Hilbert-ordered chunks make *region-coherent* caching natural — a rural cache node holding its own region's range is economically meaningful |
| WMS-equivalent tile serve (texture layer) | per-tile serve event | the cheap projection layer: low unit value, high volume |
| Resolution query ("which pack covers cell X") | resolve-node micropayment | discovery itself is paid commons work |
| SHACL/attestation verification services | attest-node fee | quality infrastructure funded the same way |

Pool disbursement respects the **royalty tree** (`ancestor_royalty` /
`royalty_tree_total`): a terrain pack deriving from GA DEM + GSNSW geology +
a custodial review contribution distributes obligation up the chain.
Important nuance for open government sources: CC-BY's obligation is
*attribution, not cash* — so the tree distributes between (a) pack production
labour (compilation, review, verification), (b) infrastructure cost, and
(c) upstream parties *where their licence actually creates an obligation*.
The agent-category multiplier applies at the rail: a commercial consumer's
fetch pays a higher multiple than a custodial community member's — the same
bytes, different terms, computed at payment time.

**Biosphere corpora discharge to free.** Once the pool clears its cap, the
pack is free for everyone — hosting beyond that is stewardship (seed nodes),
not rent extraction.

## 4. Peer discovery — three modes, one vocabulary

| Mode | Query shape | Mechanism | State |
|---|---|---|---|
| **Identity resolution** | have digest → find bytes | QDNF/DHT content addressing, resolve nodes | exists |
| **Registry browsing** | have pack name → find digest | `manifests/*.json`, DCAT/VoID self-description | spec'd |
| **Semantic discovery** | have place/time/question → find packs | manifest-quin coverage index | **new — this section** |

Semantic discovery is the piece decentralised storage usually lacks, and the
spatial substrate gives it for free: **the same DGGS cell vocabulary that
joins layers inside the graph joins packs across the network.** A resolve
node indexes manifest quins; a peer asking *"terrain ≥5 m covering cell
R7Q3…, valid asOf 2026-10"* resolves to candidate pack digests + serving
nodes — without any central catalogue of pack contents.

Discovery index predicates (minted where absent, per ontology-stack minting
rules):

- `env:inCell` — DGGS cell membership (already the graph join key)
- `eco:coversFootprint` — WKT coverage for free-form regions
- `eco:spatialIndexRange` — Hilbert range for direct byte-locality queries
- `eco:layerFamily` — the map-layers family registry term
- `prov:wasDerivedFrom` + `eco:sourceVintage` — provenance *and* freshness
  discoverable before download
- `eco:lane` + `eco:sensitivityCeiling` — discovery respects lanes: restricted
  packs are *resolvable* (identity exists) but their bytes route only through
  consented lanes (§7)

## 5. Provenance that survives the network

GOV-PRV-3's verification chain (source snapshot hash → normalized Turtle →
`.q42` digest → entry SHA-256 → whole-file CRC) is **already
transport-independent** — every carrier is untrusted, every byte verifies
against the chain. Two distribution-era additions:

1. **The attribution artefact travels as its own entry.** A cache can answer
   "who made this" without serving the payload — attribution is cheap public
   metadata even when payload access is gated.
2. **Serving events are provenance events.** A seed/cache node's service is
   recorded (host role, GOV-SOC-2) — "served by whom, when" joins "made by
   whom, when." An honest log of *custody* distinct from *authority*.

## 6. Licence-claim epistemics — the bad-actor defence

The hard case the user named: a vendor republishes upstream data (e.g. an ALA
extract whose records are mixed CC-BY/CC-BY-NC) under a *claimed* CC-BY
aggregate licence. In a payment economy, paying the vendor funds laundering.
The defence is structural — **licence claims are epistemic objects, not
booleans**, and money follows the attested chain, not the claimant.

### 6.1 `eco:LicenceClaim` — claims with evidence classes

Every `dcterms:license` assertion on an ingested artefact is an object:

```
eco:LicenceClaim
  ├── claimedBy        : the asserting party (vendor DID/agent)
  ├── claimedLicence   : e.g. CC-BY-4.0
  ├── evidenceClass    : portal-asserted | record-level | republisher-asserted
  │                      | source-verified | human-reviewed
  ├── evidence         : retrieval digest, portal URI+asOf, record sample refs
  ├── evaluatedAt      : temporal scope of the claim evaluation
  └── evaluator        : the agent/tool that assigned evidenceClass
```

Evidence classes align to GOV-HON epistemic classes: **a licence claim is a
claim about the world** — it can be observed (record-level field), asserted
(portal/vendor statement), or verified (checked against source). Lane
computation uses the *verified* tier, and treats `republisher-asserted`
claims conservatively: **an unverified vendor claim computes the lane from
the strictest upstream tier the provenance chain can establish** — the
vendor's optimism never lowers the bar.

### 6.2 Detection surfaces

- **Identifier-density check** (S3 gate): repackaged sources that strip
  upstream identifiers (DwC `occurrenceID`s, NSL LSIDs, ALA download DOIs)
  show an absence that is itself a review signal — a dataset claiming GBIF
  origin with zero GBIF identifiers is a flag.
- **Provenance fingerprinting**: digest-correlate normalized records against
  known upstream snapshots (S0 inbox corpus). A vendor file whose rows match
  an ALA snapshot at >threshold similarity is re-attributed to the real
  source in *our* graph — the vendor's contribution becomes "repackaging,"
  not "authorship." Cold-path job (external compute lane, pipeline §7A).
- **Licence-consistency check**: record-level licences inside the payload
  (where preserved) that contradict the dataset-level claim = automatic
  `contested`.

### 6.3 The economic consequence — where "don't fund bad actors" lands

1. **Obligation follows `wasDerivedFrom`, not `claimedBy`.** The royalty tree
   pays attributed ancestors — a vendor that cannot evidence its derivation
   chain receives no ancestry share. Claimed-but-unverified provenance gets
   queue position, not cash.
2. **Escrow on contest.** A contested licence/provenance claim puts the
   pack's obligation pool into escrow pending human disposition (REC-AX-2's
   contested outcome applied to economics). Dispute freezes the money; it
   does not move it.
3. **Honest aggregators are unaffected.** A legitimate vendor adding real
   value (cleaning, aligning, annotating) under honest claims earns its
   *own* contribution share — the model pays work, it just doesn't pay lies.

## 7. The socially-defined serving fabric

`spc:` relations are the consent envelope for *who may serve what to whom*:

- A **commons-lane** pack: any node may cache/serve; `pc:` terms apply by
  requester category.
- A **restricted-lane** pack: serving inherits the lane (network-nodes rule)
  — a bilateral SocialRelation is the access envelope; a custodial
  community's own node serving its packs is the reference case.
- **Custody ≠ control** (network-nodes §3): operators hold bytes under
  steward authority — custody transfers are re-signing events, not rights
  transfers; the steward can refuse serving terms regardless of who stores.
- Consent scopes are differentiated per `spc:` — serving a pack ≠ deriving
  from it ≠ relicensing it; each is a separately consentable scope.

## 8. Temporal semantics of packaged objects

Four distinct clocks on the distribution layer — conflation is the failure
mode (the same warning domain-model §5 gives for the manifold, one level up):

| Clock | Meaning | Carrier |
|---|---|---|
| **Source vintage** | what the upstream asserted, when | `eco:sourceVintage` in manifest quins (GOV-PRV-1) |
| **Pack version-time** | this immutable build | digest identity + `prov:wasRevisionOf` chain (GOV-DUR-3) |
| **Resolution-time** | what an IRI resolved to, when | timestamped alias edges (durability §2) |
| **Serving-time** | how stale the served copy is | `reconciledAt` + expected-lag in the serve response (REC-AX-3) |

Rules:

- **A served pack declares its vintage.** A node serving `terrain-pack@v3`
  in 2027 answers "v3, source vintage 2026-03, unreconciled 19 months,
  expected lag 6 months — **stale**." The response carries staleness; it
  never presents old bytes as current (REC-AX-3/8 applied to serving).
- **Multiple versions coexist legitimately.** Peers may hold v2 and v5;
  "latest" is a resolution claim with `asOf`, not a fact. Historical packs
  remain servable — DOM-AX-2 at distribution scale.
- **Payload time is data, distribution time is provenance.** A pack's
  *internal* temporal semantics (validity intervals, observation dates,
  construct contexts) are payload quins; they neither age nor conflict with
  serving-time — a pack about 1850 can be fresh; a pack about 2026 can be
  stale.
- **Update = new digest.** Peers pin versions; a `wasRevisionOf` chain lets
  a resolve node answer "what supersedes this" without any pack ever being
  mutated.

## 9. Required machinery — the build register

| Item | Where | Type |
|---|---|---|
| `eco:LicenceClaim` + evidence classes (§6.1) | new ontology mint (`eco:`) + SHACL | spec |
| `eco:SpatialAssetManifest` quin shape (manifest quins formalised) | `eco:` + SHACL; implements spatial-substrate §5 | spec |
| Discovery predicates (`env:inCell` reuse + coverage/index-range terms) | `eco:`/`env:` mint or adopt | spec |
| Provenance fingerprinting (digest-correlation vs upstream snapshots) | pipeline S3 tooling; external-compute job shape | build |
| Escrow-on-contest payment pool semantics | upstream `pc:`/`socioeconomics` extension or ecopunk profile | upstream |
| Coverage-indexed resolve service (manifest quins → node index) | network-nodes resolve role | build |
| Conservative lane computation over unverified claims | pipeline S5 | spec |
| Testnet drills: laundering-vendor, contested-pack escrow, stale-serving honesty | network-nodes §2 harness | build |

## 10. Open questions

1. Does `pc:`/`socioeconomics` already express escrow-on-contest, or is that a
   new obligation state upstream?
2. Fingerprint-match threshold for repackaging detection — per-source or a
   global policy? And does it run at ingest-time S3 or as a cold sweep?
3. Coverage index residency — does every resolve node index all manifest
   quins, or do nodes specialize by region/family (and does the index itself
   become a payable object)?
4. Upstream-source parties in the royalty tree — do CC-BY government sources
   want agent records + pools, or is the attribution artefact the complete
   discharge of their obligation?
5. Restricted-lane bilateral exchange mechanics (same open q as
   governance.md §10-2) — the custody node's enforcement surface.
6. Semantic-discovery query language — a GeoSPARQL profile over the
   manifest-quin index, or a lighter predicate surface for resolve nodes?
