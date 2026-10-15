# Network nodes — roles, testnet, and stewardship transfer

**Status:** draft v0.1 · defines the deployable "network node" unit for the
Qualia/ecopunk fabric, the test-net pattern, and the stewardship-transfer
model (node operatorship follows data custody — by design).

Substrate: `net/qdnf` (cells, authority, route/session, resolve+QSR naming,
replication, evidence, economics, policy_labels), `did_qi`, `.hmc` packs +
`webtorrent_seeder`. Node *service* definitions are ecopunk-side planning;
the fabric machinery is engine-side.

---

## 1. Node roles (a node = a bundle of declared services)

A Qualia network node is an operator-controlled deployment offering any subset
of these roles — roles are **declared capabilities**, not classes of product:

| Role | Service | Economics |
|---|---|---|
| **Cache** | content-addressed pack/object caching — serves peers by digest and, **where advertised and qualified**, by byte-range (`Q42ByteRange` contract; note `BIDX` is the `.q42` object→block index, distinct from the `.hmc` QBDL entry index — see [pack-delivery.md](pack-delivery.md) §2; a cache may hold/serve *partial* volumes, serving one section is valid service) (the "holographic" distribution path) | `pc:Micropayment` on served bytes/requests |
| **Seed** | authoritative pack hosting for a pack family (long-term availability duty) | micropayments + stewardship obligations |
| **Relay** | QDNF route/session transport (incl. non-IP bearers — acoustic/BLE/mesh per fabric) | transport micropayments |
| **Resolve** | QSR/DID/HTTP-alias resolution service — answers "what is this identifier, where is it now" | micropayments |
| **Custody** | holds a steward's volumes/packs under their keys — *custody without control*: operator provides storage/bandwidth, steward retains authority | contracted stewardship terms |
| **Attest** | DLT/git-object anchoring submissions (UTXO/OP_RETURN attestation path) | attestation fees |
| **Edge** | field/low-power node: offline-capable capture + delayed sync (the ecopunk field pattern) | local-first; syncs when connected |

Rules: a node advertises only what it verifiably does (capability manifests —
receipts, not marketing); a node serving a pack inherits the pack's lane
obligations (cache of a restricted-lane pack is itself restricted-lane
infrastructure). Range-serving is a **proposed, advertised capability** —
`q42/volume/range.rs` defines the transport-neutral contract, but which
transports a given deployment actually honours is an advertised/attested
fact, not something proven for all of QDNF today. A partial-range cache needs
the authenticated index metadata covering the ranges it *advertises* — not
possession of the whole dependency closure (the consumer acquires the rest)
— and holding bytes never grants disclosure rights: rights on sensitive
scopes are checked against the governing external data, not inferred from
possession. The mechanism is specified in
[pack-delivery.md](pack-delivery.md). Where advertised, a cache node needs no
whole-pack custody to be useful, and serving nodes are range servers, not
just blob hosts.

## 2. The testnet pattern

A deployable **test-net** is required infrastructure, not a nicety — the whole
stack (replication, resolution, economics, stewardship flows) needs a
disposable network to validate against:

- **Isolated namespace** — test identifiers (test `did:` domain / reserved
  `q42:` ranges), sandbox packs, no production commons entanglement.
- **Deterministic scenarios** — `qdnf/harness` (multihop/oracles/scenarios)
  already exists: scripted partition/merge, consent-denial, obligation
  discharge, and stewardship-transfer drills.
- **Economic simulation** — micropayment flows in test denominations;
  discharge-to-free thresholds exercised end-to-end.
- **Reproducible bring-up** — a testnet is a defined pack-set + node-role
  roster + scenario scripts; anyone can stand one up (the *deployable* part).
- **Graduation path** — testnet → commons is a *new network*, not a flag flip:
  identifiers and provenance don't silently promote from test to production.

## 3. Stewardship transfer — the designed-for success mode

**If the work succeeds, source/ontology owners may prefer to operate their own
stewardship.** That outcome is a *goal state*, and the architecture should
make it a clean transition rather than a hostage negotiation:

- **Custody ≠ control** — ecopunk/operator nodes hold data under steward
  authority from the start (`canvas_rights`/steward contracts already encode
  owner-can-refuse semantics). Taking over operation later is a *change of
  operator*, not a change of authority — no rights transfer is needed because
  authority never moved.
- **Transferable state** — a steward taking over needs: the pack manifests,
  the obligation pools (`pc:`), the key material or a key-rotation ceremony
  (k-of-n, `identity_fabric` relational recovery), the provenance/contest
  histories, and the serving records. All are already content-addressed
  records — transfer = re-sign custody + point resolution.
- **Identifier durability pays off here** — because identity is
  digest+`did:`-based (not DNS-bound), changing *where* a pack is served
  changes a service reference, not the object's identity. The ABN/domain
  failure mode can't orphan a stewarded corpus.
- **Records of handover** — stewardship change is itself a provenance event
  (who, when, under what instrument) — written to the same contest/prov
  contexts.
- **Graceful non-transfer** — a steward can equally choose to remain an
  authority while ecopunk operates nodes: the model separates them precisely
  so both arrangements are valid configurations.

## 4. Ecopunk specifics

- First nodes needed: **seed + cache** (pack availability), **resolve**
  (identifier service), and **edge** (field capture). Relay/attest/custody
  follow.
- Node software = engine + node-role services; participation profiles from
  wasm-edge §6 map to node roles (a webcivics-profile org can run a *cache/
  resolve* node without GPU).
- Stewardship-transfer drills belong in the testnet scenario set from day
  one — the handover path is the feature to exercise, not the failure to
  patch.

## 5. Open questions

1. Node identity: `did:qi` for operators/agents; what identifies the *node
   service* itself — a `did:qi` document with service endpoints, presumably.
2. QSR resolution — is a public resolver service defined upstream, or does
   ecopunk spec the first one?
3. Economics rails — same open `pc:Micropayment` live-rail question.
4. Stewardship-transfer ceremony — key rotation / re-signing protocol
   upstream vs an ecopunk-defined procedure?
5. Testnet identifier ranges — reserved `did:`/`q42:` test space convention
   needed upstream (avoid test data colliding with production namespaces).
