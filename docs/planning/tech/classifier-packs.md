# Classifier-support packs — media, observations, embeddings & P64 weights in `.hmc`

**Status:** draft v0.1 · scope: what a **commons-lane** pack must carry so that
classifier support is *decentralizable* — any holder of the pack can run, score,
propose, correct, and extend classifiers over its media without needing a
central service. Also records the **P64-canonical** naming rule and the
SPARQL-MM media model.

---

## 0. Naming rule (recorded at user's direction)

- **`.p64` is the canonical weight container** — the Qualia-native alternative
  to GGUF/safe-tensors: sibling of `.q42` (independent magic `p64\0`), 64-byte
  cache-aligned entries, page-aligned contiguous quantized blobs for single-
  fetch mmap, **role-tagged tensors** (`P64_ROLE_ATTN_Q`, `FFN_GATE`, norms…),
  zero-copy relative WASM pointers, *no* NQuin epistemic scaffold inside
  ("q42 manages truth; p64 manages inference" — `q42/p64_weight/`).
- **`gguf_*` code paths are the import/bridge layer** — `gguf_bridge`,
  `gguf_sharder`, `p64_weight/transcode.rs` are the AOT **GGUF→P64 transcode**
  path into the canonical form, plus the runtime readers. When a doc or spec
  says "weights", it means `.p64`; "GGUF" only ever means the upstream
  interchange format being imported. Flag mislabelled usage when found.

## 1. The requirement

A permissive-commons pack that *supports decentralized classifiers* is not just
media + labels. The loop it must enable:

```
pack (media + observations + vocab + features + optional weights)
  → any node runs inference locally (P64 weights or shipped embeddings + head)
  → proposals emitted as observation quins (model + digest + score)
  → human review corrects/rejects (humanCorrectsClass / humanRejects quins)
  → corrected observations federate back as new pack versions/deltas
  → downstream nodes retrain heads/fine-tune over shared features
```

This is graph-native distributed learning: not parameter-server federated
learning, but **shared media + shared observation graph + shared vocabulary +
shared weights**, with human disposition preserved at every step.

## 2. The payload inventory — what goes in the `.hmc`

| Entry kind | Contents | Format | Notes |
|---|---|---|---|
| `media/*` | source images/audio/video or refs | intact payloads (page-aligned; slice = standalone file) | licence per item; sensitive media → restricted lane or exclusion |
| `graph/*.q42` | observation quins: `VisualObservation`, `proposesClass`, `hasBoundingBox`, `hasTrackId`, score, `modelDigest` + **human corrections** (`humanRejects`, `humanCorrectsClass`) | quin volumes | *the correction stream is the training signal* |
| `graph/fragments.q42` | **SPARQL-MM fragment structure** — `ma-ont:` edges: `hasFragment`, `hasTemporalFragment` (start/end), `hasSpatialFragment` (x/y/w/h), `hasTrackFragment`, `duration`, `hasTrack` | quin volume | makes "observations of taxon X in region R during season S" a graph query, not a file scan |
| `vocab/*.ttl` | class vocabulary — class IRIs bound to taxa/labels (shared label space for all decentralized classifiers) | RDF | binds detection classes → `eco:`/`dwc:` taxa |
| `features/*.bin` | frozen embeddings per media item (fixed-dim vectors + index) | dense sidecar (`.10d` FIEL or raw + manifest) | lets edge nodes retrain **heads** without shipping the base model |
| `models/*.p64` | optional shipped weights — e.g., a species-ID classifier | **`.p64`** (canonical) | role-tagged tensors → forge/decode-graph ready |
| `manifest` | pack manifest: entries, digests, licences, **`asset_envelope`** (upstream releases, toolchain versions, licence union), review status | CBOR/quin | integrity chain per `governance.md` |
| `provenance/*` | C2PA-style edges (`hasManifest`, `hasSignature`, `derivedFrom`, `verificationStatus`) + capture provenance | quins (+signature entry) | honest `C2paVerificationStatus` ladder |

## 3. The SPARQL-MM media model (verified `sparql_library/sparql_mm.rs`)

The engine already speaks **W3C Media Annotations** (`ma-ont:`) + SPARQL-MM:
`hasFragment` / `hasTemporalFragment` / `hasSpatialFragment` /
`hasTrackFragment` / `hasStartTime`/`hasEndTime` / `duration` / `hasX`,`hasY`,
`hasWidth`,`hasHeight` / `hasTrack` + a `mediaTimeMs` extension — plus a **C2PA
predicate family** (`hasManifest`, `hasSignature`, `hasProvenance`,
`derivedFrom`, `isVerified`, `verificationStatus`…) with an *honest*
verification-status enum (field presence ≠ verified).

Implication for pack design: **model media as fragment-structured entities** —
an audio file *hasTemporalFragments* (call windows), a frame *hasSpatialFragments*
(bounding regions), a video *hasTrackFragments*. Observations attach to
fragments, not just whole files — so "every proposal inside this bbox / this
time window / this track" is a first-class query. `qualia-vision` already emits
compatible predicates (`hasBoundingBox`, `hasTrackId`) — note the recipe's
verified subtlety: vision carries **frame index in metadata**, while
`SparqlMmHandler` keys temporal off `mediaTimeMs`/`hasStartTime` objects — our
mappers must emit whichever the query path reads (register this in
`engine-requirements.md`: unify or dual-emit fragment timing).

## 4. Governance rules for classifier-support packs

- **Lane discipline** — observation quins and embeddings inherit the pack's
  licence lane; a restricted-lane media item's *embeddings* don't leak to
  commons just because vectors feel anonymous (treat as derived works — union
  of obligations).
- **Corrections are attestations** — `humanCorrectsClass`/`humanRejects` carry
  the corrector + evidence; they're the epistemic layer's human-disposition
  record, not silent label edits.
- **Proposals ≠ truth** — `proposesClass` stays epistemic (certainty band,
  modelDigest) even after aggregation; a majority of proposals doesn't mint a
  fact.
- **Templates never ship** — the vision rule stands: dense pixels/templates
  don't live in quins; embeddings ship as *media-derived features* only where
  the source licence permits, and never for biometric-restricted content.
- **Sensitive-observation propagation** — a detection on a sensitive record
  inherits its sensitivity; the pack builder must not ship a public observation
  quin pointing at a restricted media item.

## 5. ecopunk use — species-ID as the pilot

A species-identification pack per bioregion/taxon group:

- `media/` — field photos/audio (licensed per item; iNaturalist CC subsets,
  TERN EcoImages/EcoAcoustics)
- `graph/` — `ma-ont` fragment structure + observation quins
- `vocab/` — class IRIs → `dwc:Taxon` bindings
- `features/` — per-image embeddings (edge head-retraining)
- `models/` — a `.p64` region classifier where licence permits
- Review: identifications are `proposesClass` until a verifier's
  `humanCorrectsClass`/attestation lands — the correction stream feeds back as
  pack deltas (federated improvement, decentralized corpus).

## 6. Open questions

1. Delta/versioning scheme for correction streams (append-only observation
   volumes? supersession quins?).
2. Embedding payload format — `.10d` FIEL vs raw+bin manifest entry.
3. Federated round-trip protocol — how do node corrections return (QDNF
   replication? pack-version deltas? a stewardship-attested merge?).
4. Whether `proposesClass`-heavy corpora get a consensus/weighting rule
   (agreement among N models + K humans?) or stay flat epistemic.
5. The metadata-vs-object temporal-fragment unification (§3) — upstream item.
