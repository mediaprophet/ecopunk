# Durability & distribution — identifiers beyond DNS, and commons infrastructure economics

**Status:** draft v0.1 · scope: the identifier model for a 50-year corpus that
**must not die when a domain does**, the dual addressing rule (HTTP +
non-HTTP), the `did:qi`/QDNF substrate, DHT/DLT/offline distribution, and the
ontologically-defined micropayment rails that pay for hosting, energy, and
communications.

---

## 1. The DNS threat model (lived, not hypothetical)

The design pressure is concrete: **a natural person's ABN was cancelled,
and the domain names bound to it were lost** — identity, hosting, and every
HTTP URI minted under those domains broke at once. This is not an edge case:

- domains lapse, get repossessed, get transferred, get seized;
- organisations fold, merge, rebrand — their `/def/` URIs rot;
- ccTLD and registrar policy shifts are outside any individual's control;
- individuals are *disproportionately* exposed — a company can lose a domain
  and survive; a sole trader's identifier graph can die with one registry event.

**Rule:** for a 50-year corpus, **canonical identity must not depend on DNS**.
An HTTP URI is a resolution *service* — a claim with provenance and datetime —
never the thing's identity.

## 2. The dual-addressing model

Every governed object (entity, layer, pack, term, observation) carries:

| Address class | Nature | Rules |
|---|---|---|
| **Canonical identity** | content digest (hash/parity family) + DID (`did:qi`, `did:q42` coord) | stable for the object's lifetime; derivable from the object itself — *resolution-independent* |
| **HTTP URI aliases** | service claims — `resolvedTo`, `asOf`, `via`, `operator` | timestamped + provenance-bearing; many per object; can all fail without killing the object |
| **Non-HTTP URIs** | `did:`, content-addressed, DHT keys, QDNF QSR names | first-class, not second-class (the user's original DID motivation: *the commons must support non-HTTP URIs to remain decentralised*) |

Historical correctness is preserved by modelling resolution as **time-stamped
provenance edges**: "this HTTP URI resolved to this digest, attested by this
operator, during this interval" — so "what did `eco:...` mean on 2027-03-01"
stays answerable after the domain is gone. (This is the identifier-level form
of our border/version rule: *everything assertable has a time dimension*.)

## 3. The substrate that exists (verified)

| Piece | What it is | Evidence |
|---|---|---|
| **`did:qi`** | DID-Core CRUD runtime — multibase `z` (base58btc) of the SHA-256 genesis digest; QCDE-1 (RFC 8785) canonical documents; git-object store | `did_qi/` — method/document/service/vectors |
| **DLT anchoring** | **Bitcoin-family UTXO attestation** — `OP_RETURN "QI"` live/tombstone records, CAIP-2 chain IDs, caller-supplied tx bytes (honest scope: no node/network) | `did_qi/utxo.rs` |
| **QDNF** | the decentralised network fabric — cells, authority, resolve/QSR naming, route/session, evidence, replication, raw-ethernet + acoustic-BLE bearers; docs + CI workflow exist | `net/qdnf/`, `docs/qdnf*.html` |
| **Torrent/DHT path** | `services/webtorrent_seeder` + routes — pack distribution without central hosting | `services/` |
| **`did:q42` pointer** | topological resource coordinate (bit-63 MSB in object field) — resource→physical-address indirection that survives transport changes | `identifier.rs`, AGENTS §1 |
| **10D address grid** | the `.10d`/Tensor10D axis-role taxonomy — selectors (`q,v,w` pick the metric/kind of space) + coordinates (`x,y,z,t,α,σ` + `μ` provenance-carrier) — a *typed positional address* where metric, time, and provenance travel in the address itself | `container_10d/axis_role.rs`, `header.rs`, `modalities/manifold.rs` (T14) |
| **Exact-range contract** | `Q42ByteRange` — the transport-neutral exact-range contract. Verified implementations: the native `HttpRangeSource` (`cfg !wasm32` — native, not browser), the local file adapter (reference impl), and the IPFS partial-CAR path (per-block CID verification of the addressed entity). Any other transport's range support is an advertised, attested property — not universal | `q42/volume/range.rs` |
| **BIDX object index** | object hash → contiguous block interval, paged enumeration w/o per-block allocation (`BidxMatchPage`); plus a field-range index | `q42/volume/index.rs` |
| **HTTP Range serving** | `Range: bytes=` → `206`/`Content-Range` on the pack serve path | `services/webtorrent_routes.rs` |
| **IPFS partial-CAR** | trustless partial retrieval — every CAR block CID-verified *before* payload delivery; Q42 offsets address the UnixFS entity | `q42/volume/car.rs` |
| **ILP dispatcher** | SPSP→ILP-STREAM micropayments with receipts, optional Nym routing, offline queue retry, stablecoin fallback | `services/ilp_dispatcher.rs` |

The 10D address-grid row is the 'quantum'-style addressing foundation: durable identity
(digest + `did:`) names *what* an object is; the 10D grid names *where* it sits
— and every ecopunk layer/asset/field value inherits a position in that one
typed address space. Detail + the two-tens distinction (pose vs epistemic
geometry) in engine-capabilities.md §5.

## 4. Distribution model

A pack is a **content-addressed object**; every channel below is a *carrier*:

```
              ┌─ HTTP(S) hosting   (one carrier; conventional)
   .hmc pack ─┼─ QDNF replication  (cells, QSR resolution, evidence)
  (digest is  ├─ WebTorrent / DHT  (services/seeder)
   the addr)  └─ offline/sneakernet (volumes are files; sync later)
```

`did:qi` UTXO/DLT anchoring is **attestation, not a payload carrier** — it
anchors live/tombstone records about an object; it never delivers pack bytes.

Range-fetch is a **substrate contract**, not a deployed guarantee — the
range-addressable machinery is verified *at mechanism level*
(`q42/volume/`), and verification semantics differ per mechanism:

- `Q42ByteRange` is the single exact-range contract range-serving transports
  are required to implement;
- **BIDX** maps object hash → contiguous block interval (plus a field-range
  index) inside `.q42` volumes — it is **not** the `.hmc` QBDL footer entry
  index; the two indexes are distinct (see
  [pack-delivery.md](pack-delivery.md) §2);
- the WebTorrent serve path honours HTTP `Range: bytes=` (a native serving
  path — not browser/WASM client proof); IPFS partial-CAR CID-verifies every
  block *before* delivering payload — on that specific path, **partial reads
  are trustless**; raw HTTP ranges elsewhere are unauthenticated without a
  committed trusted index/digests.

Consequences: clients **stream sections or explicitly install packs** — both
modes through one verified service, which is the planned delivery contract in
[pack-delivery.md](pack-delivery.md). The current game-demo still performs an
unintegrated whole-pack fetch; the **browser range-acquisition gap is open**
(remediation finding F5, gap **G34**). A cache node can legitimately hold and
serve *partial* volumes (whole-pack custody isn't required to be useful);
decentralised availability is a multi-carrier redundancy property, not a
hosting SLA.

## 5. Commons infrastructure economics (the ontologically-defined rails)

Verified in `core-ontologies/permissive-commons.n3` + `modalities/socioeconomics`:

- **`pc:Micropayment`** — "ILP / streaming micropayments into the obligation"
  (ILP = Interledger Protocol); **`pc:SponsorshipLumpSum`** — a sponsor pays
  down the pool, liberating the work for all; **`pc:paidVia`** links works →
  payment methods.
- **`socioeconomics`** semantics: `commons_cost` = production cost × (1 + capped
  ROI), per-use royalty accrues into the pool, `is_commons_discharged` frees
  the work globally at threshold.

Apply it to infrastructure: **serving/hosting/relaying a pack is a micropayment
event into that pack's obligation pool** — energy, bandwidth, storage, and
compute costs are paid down ontologically, not by ad-hoc billing. A host that
mirrors a commons pack earns the micropayment stream its service generates;
sponsors can liberate high-value corpora outright. This is how the commons
*self-funds* its own availability — the economics are in the data model.

The rail is **implemented, not just vocabulary**: `ilp_dispatcher.rs` executes
a `TaxDispatchPlan` as ILP STREAM payments — SPSP resolution (`$pointer` →
`.well-known/pay` → STREAM → `PaymentReceipt`), optional Nym mixnet routing,
an offline queue (`pending_payments.ndjson`, retried next cycle), and a
stablecoin fallback. Remaining open: live connector/endpoint deployment and
denomination policy — a dispatcher exists; the counterparty network is
deploy-side, not code-side.

## 6. Consequences for ecopunk specs

1. **PID strategy (amends `ontology-stack.md` §6):** HTTP PIDs (AGLDGW `/def/`,
   w3id, own domains) are **durable aliases, not canonical identity**.
   Canonical identity = digest + `did:` identifier; HTTP resolution records
   carry `asOf`/operator provenance so post-loss reconstruction is provable.
   Registering a `/def/` PID is still worth doing (human-facing discoverability)
   — but the graph must never *depend* on it.
2. **Pack manifest** gains an identity block: canonical digest + `did:` +
   alias list + resolution-history references.
3. **Every ingest/spec** that names an external resource records *both*
   address classes; an HTTP URL alone is an incomplete reference (the LOD
   dead-link rule extended to ourselves).
4. **Availability becomes an economic object**: `pc:paidVia` + `socioeconomics`
   pools on packs; hosting/relay is incentivised commons work, not unpaid
   dependency.

## 7. Open questions

1. `did:qi` vs `did:q42` for pack identity — which method anchors *objects*
   (vs persons/agents)? Probably `did:q42` for resources, `did:qi` for agents —
   confirm upstream convention.
2. QDNF QSR naming ↔ HTTP alias bridging — who operates resolution gateways,
   and is that itself a micropayment-earning service?
3. ~~ILP integration maturity~~ → **partially resolved** — dispatcher is
   implemented (`ilp_dispatcher.rs`); the remaining question is live
   connector/endpoint deployment + denomination policy, not the code.
4. DLT anchor policy — which chain(s), and is anchoring optional per pack?
