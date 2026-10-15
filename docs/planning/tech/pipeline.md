# Pipeline — source artefacts → QualiaDB packages

**Status:** draft v0.1 · **Date:** 2026-10-12
**Scope:** how acquired ontologies, datasets, and media are processed into `.q42` volumes
and `.hmc` packs (which carry `.10d` 3D assets and all other artefacts).
Governance gates referenced here are specified in [governance.md](governance.md).

---

## 0. Terminology (engine-verified)

| Artefact | What it is | Verified against |
|---|---|---|
| `.q42` | Unified graph volume. `Q42\0` 256-byte header (v3 readable, v4 written), embedded namespaced Q42LEX, BIDX (+ optional FIDX/PIDX), block directory, LZ4-compressed 40,960-byte SuperBlocks of 48-byte `NQuin` records; optional temporal index + Merkle-DAG history sections. Publication flags: `FLAG_PERMISSIVE_COMMONS 0x0020`, `FLAG_SANCTUARY 0x0040`. | `qualiaDB/docs/manuals/standards/q42-format-internal-draft.md`, `crates/qualia-core-db/src/q42/` |
| `.hmc` | HyperMedia Container (`QBDL` magic, v1). 64-byte header + 64-byte-aligned **intact file payloads** + CBOR index footer of `BundleEntry{key, kind, offset, length, sha256, meta}` + whole-file CRC-32C. An entry slice is bit-identical to the standalone file — interior `.q42`/`.10d`/`.p64` offsets resolve unchanged under `mmap`, and HTTP range-fetch of an interior segment works directly (`entry.offset + seg.offset`). | `crates/qualia-core-db/src/bundle/format.rs` |
| `.10d` | Dense-asset container (`10d\0` magic). Sectioned payloads: QuantizedMesh, material (MAT1), normal frames (FRM1), texture coordinates, topology, spatial index, provenance, field sections (`FIEL` — scalar/vector/tensor over nodes/cells/voxels), skeletal animation (`SKL1`), v2 manifold-identity manifest (TypedFieldRecord → section IDs), axis roles. Whole-file CRC-32C seal. | `crates/qualia-core-db/src/container_10d/` |
| `.p64` | Model-weight container (GGUF-derived). Inference models only — **never** used for geometry or dataset payloads. | `crates/qualia-core-db/src/q42/p64_weight.rs` |
| `yaml-ld-q42` | Declarative QApp manifest format → compiled to NQuins on ingest (`POST /manifest`). | `docs/manuals/qapps_specification.md` |
| `manifests/*.json` | Registry JSON (`id/name/version/type/size/url/filename`) for downloadable resources; hosted so entries update without app rebuild. | `qualiaDB/manifests/ontologies.json`, `models.json` |
| HCF | Hypermedia Content Format — *document* representation; draft prose diverges from bundle code in places (`.d10` alias, Bao/BLAKE3 claims). Use `.10d` spelling; treat HCF as document-layer only. | game-demo `14-qualiadb-format-and-tooling-upstream-tasks.md` |

## 1. Pipeline overview

```
                       DISCOVERY
  source landscape ────────────────────────────────► data-sources/* registers
       │
   S1  ACQUIRE ── dated snapshot, hash, licence capture, manifest row
       ▼
   S2  STAGE ──── acquisition-inbox/<source>/<date>/   (raw, immutable, untrusted)
       ▼
   S3  REVIEW ─── human gate: licence tier, sensitivity, sovereignty, quality
       ▼                (fail → stays in inbox with reason recorded)
   S4  NORMALISE ── map to vocab stack (Darwin Core / TERN / NSL / GeoSPARQL / eco:)
                    emit Turtle/N3 + provenance graph
       ▼
   S5  VALIDATE ── SHACL shapes (incl. TERN profile shapes), parity, ID integrity
       ▼                (fail → back to S4; never patched in place)
   S6  COMPILE ─── Turtle/N3 ──► qualia-cli ingest ──► <name>.q42  (v4 volume)
                   GLB/geometry ──► computational-geometry compiler ──► asset.10d
                   media/docs ──► intact files (+ Audubon Core / HCF descriptions)
       ▼
   S7  PACK ────── bundle semantic volume + dense assets + media + manifest
                   ──► <package>.hmc  (QBDL)
       ▼
   S8  PUBLISH ─── lane decision: permissive-commons | restricted | internal
                   registry entry in manifests/*.json; magnet/web-seed iff Commons
       ▼
   S9  LEDGER ──── capability-ledger update: which engine features the pack
                   depends on, verified at which QualiaDB commit
```

Two deliverable shapes fall out of S6/S7:

- **Ontology/dataset → `.q42`** when the artefact is purely semantic (taxonomy backbone,
  occurrence volume, ontology itself). May ship loose for dev or inside an `.hmc` for
  distribution.
- **Package → `.hmc`** when the artefact is composite: a semantic `.q42` **plus** `.10d`
  models, textures, audio, documents, shapes, licences, and a pack manifest entry.

## 2. Path A — ontology/dataset → `.q42`

### Inputs the engine actually ingests (verified)

`SemanticFormat` in `qualia-cli/src/ingest/detect.rs`: N-Triples(‑Star), N-Quads(‑Star),
Turtle(‑Star), TriG(‑Star), N3, RDF/XML, JSON-LD(‑Star), CBOR-LD, KML, Agent-Intent-JSONL,
`.q42` (merge/re-index), Mesh (OBJ/STL/GLB → mesh *facts* only, see §4).
CSV/JSON need `csv_mapper`/`json_mapper` pre-mapping to RDF — they are **not** direct ingest
formats. NetCDF/GeoTIFF/parquet/audio/sequence formats have **no ingest path** today
(→ engine gap GAP-E-*).

### Compilation steps

1. Normalize to **Turtle or N3** (canonical source of truth, kept in-repo).
2. Assign the dataset a **named context** (per-source graph DID) and a sensitivity class
   in the NQuin context field — this is where Restricted/Classified lives.
3. Ensure provenance triples travel *with* the data (PROV-O + licence per §governance).
4. `qualia-cli ingest <file.ttl> -o <name>.q42` → v4 unified volume.
   Memory contract: ingest pipeline enforces a 512 MB ceiling and 64 KB I/O pages;
   very large datasets must be **partitioned per named graph / source / time slice**
   into multiple volumes rather than one monolith (see §6 sizing).
5. Set publication flags at write time: Commons only if the pack manifest affirmatively
   clears the licence tier; Sanctuary for restricted holdings. **Fail closed.**
6. Record receipts: source snapshot hash, ingest tool version + QualiaDB commit,
   record counts, checksum of emitted volume.

### Ontology-specific steps (the "existing ontologies" the user named)

Ontologies are datasets with extra obligations:

- Import the ontology **source unchanged** (upstream file → inbox snapshot).
- Write a *profile overlay* rather than editing upstream vocab: `eco:` alignment file
  carrying `rdfs:subClassOf`/`owl:equivalentClass`/`skos:exactMatch` mappings into our stack.
- Ship the ontology's own SHACL shapes alongside (TERN ships `tern.shacl.ttl`) — they
  become validation rules for incoming data.
- Ontology `.q42` volumes are reference volumes: they should be queryable standalone
  and also embedded as entries in dataset packs that use them.

## 3. Path B — composite package → `.hmc`

### Bundle structure (QBDL v1)

```
64-byte header: magic "QBDL", version, entry_count, index_offset/length,
                total_length, whole-file CRC-32C
per entry:      64-byte-aligned intact payload
footer:         CBOR index — BundleEntry { key, kind, offset, length,
                                          sha256, meta? }
```

- `kind` is a string tag — ecopunk convention (to register): `"q42"`, `"10d"`, `"p64"`,
  `"manifest"`, `"shacl"`, `"media"`, `"doc"`, `"licence"`, `"provenance"`.
- `meta` is opaque CBOR agreed by producer/consumer — the Anatomy pack precedent stores
  domain semantics there (organ → body-system/position/colour). Ecopunk `meta` schema is
  defined per pack family in the asset spec (pending doc).
- Header+index are fetchable alone → catalogue/browse a pack without downloading payloads;
  64-byte-aligned entry offsets make interior range-fetch *a format affordance* —
  the `.hmc` QBDL footer entry index here is **separate from** the `.q42` `BIDX`
  object→block index (§0), and an **authenticated browser range reader is not
  qualified in the current game path** (native `HttpRangeSource` is `cfg !wasm32`;
  verification still needs a committed trusted index, not raw ranges).
  Acquisition modes — streaming *and* explicit install through one verified
  service — are specified in [pack-delivery.md](pack-delivery.md); do not claim
  streaming is demonstrated in the browser client today.

### Reference pack layout (ontology + data + assets)

```
biosphere-pack.hmc
 ├─ manifest.ecopack.jsonld     kind="manifest"   pack identity, lanes, deps, contents
 ├─ graph/core.ttl.q42          kind="q42"        semantic volume (taxa, traits, places)
 ├─ graph/shapes.shacl.ttl      kind="shacl"      validation shapes used at build
 ├─ ont/tern.ttl.q42            kind="q42"        embedded reference ontologies
 ├─ assets/3d/<entity>/<style>/<lod>.10d   kind="10d"
 ├─ media/img/…  media/audio/…              kind="media" (intact; Audubon Core rows in q42)
 ├─ docs/*.hcf|html|md          kind="doc"        encyclopedic text (HCF when stable)
 └─ prov/provenance.n3          kind="provenance" full transform chain + licences
```

Open decision (upstream, tracked as QG-01 in game-demo): **canonical `.hmc` variant** —
core `QBDL` BundleWriter vs `qualia-semantic-library` ZIP variant are not interchangeable.
Ecopunk adopts **QBDL core** for spec purposes and flags the upstream decision; do not emit
the ZIP variant until reconciled.

## 4. Path C — 3D/dense assets → `.10d` (entries inside `.hmc`)

1. **Author or acquire source geometry** — GLB preferred interchange; OBJ/STL legacy;
   parametric/procedural sources via computational-geometry authoring (game-demo precedent:
   578+ `.10d` assets generated from original recipes).
2. **Compile** via `render::compile_10d` / computational-geometry authoring → sections:
   QuantizedMesh + MAT1 material + FRM1 normals + texcoords + topology + spatial index +
   provenance + optional FIEL fields + SKL1 skeletal (exists in-tree — validate before use).
3. **Bind identity**: v2 manifold-identity manifest inside the `.10d` links typed records to
   sections; the `.q42` volume holds the semantic entity the asset *depicts*
   (`eco:depicts`/stable `rc:asset/`-style IDs precedent).
4. **Seal** (per-section + whole-file CRC-32C) → embed as intact `.10d` entry in the pack.

**Known engine boundaries (do not spec around them — record as gaps):**
- Full glTF scene preservation (hierarchy/animation/morphs) is **missing** upstream.
- Animated `.10d`: SKL1 exists in `container_10d/`; browser playback contract unresolved
  (QG-03 / AG-11 "Not started").
- `mesh_to_nquins` parity bug on the legacy import path.
- No raster/volumetric → `.10d`/`q42` path (climate grids, satellite layers).
- Terrain/geography streaming AG-09 not started; LOD compiler AG-16 not started.

## 5. Partitioning & sizing rules

| Rule | Rationale |
|---|---|
| One `.q42` per **acquisition unit** (source × snapshot-date × region/licence cohort) | Immutable, independently verifiable, independently licensed |
| Merge/derived volumes are *new* volumes with PROV `wasDerivedFrom`, never mutation | LWW-CRDT dedup + Merkle history stay meaningful |
| Licence cohorts never mix: a P3 (non-commercial) record never enters a P0/P1 pack | Licence laundering prevention — enforced at S5 |
| Sensitivity cohorts never mix: raw sensitive coordinates → Sanctuary volume only | 42-bit context sensitivity class + publication fail-closed |
| Media/dense payloads are `.hmc` entries, **never** NQuin expansions | 48-byte record + dense-asset reference model |
| Pack size target: streamable in parts — keep manifest+index+`.q42` ahead of heavy `.10d`/media entries | Range-fetch browsing of micro→macro collections |

## 6. Tooling inventory (verified vs needed)

| Step | Tool today | Status |
|---|---|---|
| RDF → `.q42` | `qualia-cli ingest` (v4 unified writer, incremental parallel cell resolution, 512 MB ceiling) | Implemented |
| Format detection | `ingest/detect.rs` (13 RDF-family formats + KML + mesh) | Implemented |
| CSV/JSON → RDF | `ingest/csv_mapper.rs`, `json_mapper.rs` | Implemented (mapping profiles needed per source — ecopunk work) |
| `.10d` compile | `render::compile_10d`, container_10d writers, primitive/parametric authoring | Implemented for primitives; GLB→`.10d` full-scene import in progress upstream (AG-14) |
| `.hmc` write/read | `bundle::{writer,reader}` (QBDL v1, per-entry SHA-256, CRC-32C) | Implemented; canonical-variant decision open upstream |
| Pack → Solid/LDP | `services/solid_ldp/bundle.rs` | Implemented surface — verify scope |
| Raster/NetCDF ingest | — | **Gap** (GAP register) |
| SPARQL federation harvest (TERN GraphDB etc.) | engine supports HTTP federation "local or explicitly requested" | Verify federation-to-ingest path |
| SHACL validation at S5 | `shacl_compiler.rs` full-vocabulary compiler | Implemented — wire into pipeline tooling |

## 7A. External compute lanes (Colab et al.)

Pragmatic reality: once the foundations are defined, several jobs are too heavy
for a laptop — bulk ontology compilation, LOD dataset normalisation, embedding/
alignment-candidate generation, raster/field processing, model training. These
are all **cold-path artifact producers** — the right candidates for external
compute (Colab, RunPod, institutional HPC). This is not a compromise of the
topology; it's the spec's own tier model applied to development practice —
laptop = Tier 0/1, external = Tier 2-equivalent, and the rules below keep the
guarantees intact:

1. **Egress is a disclosure event.** Only data cleared for external processing
   leaves: open-licence sources, public ontologies, commons-lane intermediates.
   Restricted-lane sources, sensitive/custodial content, Sanctuary material,
   and anything carrying non-redistribution obligations **never** go external —
   processed locally or not at all. (Same gate as the adapter
   `NetworkDisclosureRegistry` — consent/policy evaluated before egress.)
2. **Job manifests.** Each external job is a recorded manifest: inputs by
   content digest, environment (image/library pins), script/notebook hash,
   seeds, and output digests. The manifest enters the artifact's provenance
   chain — "processed on Colab" is a provenance *fact*, not an informal note.
3. **Outputs are candidates, not assertions.** External runs produce the same
   proposal statuses as internal ones — ontology-alignment candidates, SDM
   rasters, trained `.p64` weights all re-enter through the human review gate
   (S4) and, where consequential, human ratification.
4. **Determinism where feasible** — seeds pinned, versions locked, and receipts
   reproduced; where the platform can't guarantee bit-exactness (GPU variance),
   record the variance honestly in the manifest.
5. **Pack-friendly job shapes**: bulk `.q42` compile of large LOD dumps;
   kg_embedding training (run-once artifact producer — its own design note
   says train once, ship tables); SDM fits; motif/frequency analyses; large
   TDA/geometry builds; spectral decompositions over media corpora.

## 8. Open questions

1. Pack identity scheme: `eco:pack/<family>/<id>@<version>` IRI + content digest — or reuse
   `did:q42:` topological pointers? (DID route favours engine-native resolution.)
2. Whether per-occurrence volumes (potentially 100M+ records for ALA) live as one
   multi-volume family keyed by `ibra7` region × year, or by dataset licence cohort first.
   Benchmark before committing — Sentinel/arena constraints bound single-pass evaluation.
3. `.hmc` `meta` CBOR schema per pack family (micro collection vs macro collection vs
   ontology bundle) — belongs in `tech/assets/asset-package-spec.md`.
4. Does `POST /manifest` accept pack-manifest JSON-LD directly, or only `yaml-ld-q42`
   workspace docs? Verify and record.
5. Update/snapshot cadence per source (ALA moving corpus vs NSL versioned releases) →
   version-pinning convention in pack names vs manifests.
