# QualiaDB Capability Reference for Ecosphere Assets

**Purpose:** a linked audit of what `C:\github\qualiadb` actually implements, written for agents and
contributors working on `Ecosphere Assets.md` who cannot hold the whole repository in context.
Every claim below names the file(s) it was verified against. Read this as a map, not a summary —
follow the links before relying on any single line.

**Audit date:** 2026-10-15 · **Repo branch:** `0.0.40` · **Method:** read-only source + document
inspection. No code was built or tested during this audit; test-count claims below are quoted from
the repo's own review documents, not re-run.

> **Planning update — 2026-10-15:** Read [§12](#12-game-and-simulation-asset-quality-source-checked-development-programme)
> before using earlier “missing” or “closed” labels. It checks recent implementations and their
> remaining boundaries, then proposes prioritised asset-production and playback work with
> source links, online research and acceptance criteria. This update changes documentation only.

---

## 0. How to read this document

### Evidence levels

| Label | Meaning |
|---|---|
| **Implemented** | Real code verified by reading the source file; wired to at least one caller or CLI/MCP/WASM surface. |
| **Library-only** | Real, tested code with no external invocation path (callable only from Rust/tests). Common in `solvers/` and `computational_geometry/` — see §10.2. |
| **Partial** | Works, but with a documented boundary (e.g. subset of a format, one canonical model per domain). |
| **Spec-reserved / Stub** | Named in a format or trait but explicitly not implemented, or an honest `NotImplemented`. |
| **Plan-only** | Described in `docs/plans/` or a standards draft; no live code. |
| **Stale-doc risk** | Documents that disagree with live code. The divergence log wins (see below). |

### Authority ordering when sources conflict

1. `crates/qualia-core-db/src/**` — live code is authoritative.
2. `docs/manuals/standards/q42-implementation-divergence.md` — the repo's own log of where
   standards documents and code disagree. **Read it before citing any standard.**
3. `AGENTS.md` (repo root) — the governing constraints (NQuin layout, opcode ranges, two-tier
   allocation model). Also injected into agent sessions as an always-on rule.
4. `docs/manuals/qualia_db_functionality_manual.md` — a candid full-file audit (2026-07-13)
   naming every known stub, mock, and unwired module. Older than current code but its honesty
   conventions match this document.
5. `docs/plans/*`, other standards drafts, `DIRECTORY_INDEX.md` files — navigation aids; treat as
   entry points, not evidence. `MASTER_INDEX.md` is dated 2026-07-29 and is stale relative to
   October 2026 code.

### Governing execution contract (AGENTS.md §0, §0-A)

Everything in this ecosystem is built around:

- **48-byte `NQuin`** — every semantic datum; fixed bit layout (subject/predicate/object/context/
  metadata/parity). Bit layout is normative in `AGENTS.md` §1.
- **42 MiB `SlgArena` Sentinel** — bounded static-tabling arena for hot-path logic evaluation.
- **Two-tier allocation**: Tier 1 `HotZeroHeap` (no `Vec`/`String`/`Box`; caller `&mut [T]` buffers;
  enforced by `assert_zero_alloc` tests) vs Tier 2 `ColdBounded` (construction may allocate under an
  explicit caller byte budget, default ~256 MiB, fail-closed on exhaustion; parallel construction via
  `geometry_workspace.rs` arenas).
- **`q_hash()`** — compile-time FNV-1a over IRIs; no runtime string hashing in hot paths.
- **Modality opcodes start at `0x10`** — deontic `0x10–0x12`, epistemic `0x20–0x22`,
  paraconsistent `0x30–0x32`, LTL `0x40–0x44`.
- Inline object type tags (authoritative in `query/resolver.rs`): `0b001<<60` integer,
  `0b010<<60` decimal (×10⁶), `0b011<<60` boolean; `object` MSB=1 → `did:q42` topological pointer.
  ⚠ `logic.rs::extract_float` uses a conflicting `0b001<<60` f32 tag — known inconsistency (AGENTS
  §4-D); do not build new code on the `logic.rs` interpretation.

### Ecosystem crate map (`C:\github\qualiadb\crates\`)

| Crate | Role |
|---|---|
| `qualia-core-db` | The engine: NQuin, Q42 volumes, `.10d` containers, `.hmc` bundles, modalities, SPARQL, solvers, geometry, render, audio, inference, net/QDNF/P2P, governance. ~460+ `.rs` files. Nearly everything in this reference lives here. |
| `vibe` | VibeScript language: parser, type checker, bytecode (`VBC1`), VM, effect system, host catalog (~1,158 invoke IDs). |
| `qualia-client-core` | App/orchestration layer: chat+inference, wallet (BIP32/eCash), Chora geospatial canvas, QApps, accountability fabric, mail. |
| `qualia-cli` | CLI front end: ingest, SPARQL, llm tooling, solvers, shader tooling, daemon/mesh/MCP management. |
| `qualia-extensions` | Heavier non-zero-alloc compute: PINN, SNN, QPU extension, WebGPU physics solvers. |
| `qualia-q-forge` | `#![no_std]` quantum statevector sim + OpenQASM subset parser (experimental). |
| `qualia-semantic-library` | `qsl` — corpus→`.hmc` ZIP hypermedia containers (a **different** `.hmc` format — see §3.3). |
| `qualia-solid-bridge` | W3C Solid/LDP pod server + consumer client, Turtle→NQuin boundary. |
| `qualia-vision` | Phase-1 native vision ABI; CPU reference detector → `VisionQuin` records. |
| `webizen-render` / `webizen-runtime` | wgpu 30 scene renderer + fixed-timestep GPU simulation kernel (diffusion). |
| `webizen-lite-wasm` | 268 KB browser MCP endpoint: N3 parse, Quin query, SHACL, modal logic (11 tools). |
| `webizen-desktop` / `webizen-studio` | Tauri shell + Dioxus UI workbench. |
| `webizen-web`, `webizen-component-harvester`, `qualia-mobile-harness` | Browser portal, codegen tool, companion PWA. |
| `wellfare-core` | Health-vault domain crate (anatomy, records) — a consumer of the same primitives. |
| `qualia-cooperative-core` | Supported-agency/cooperative governance model (stub-free, ~84 tests). |
| `qualia-audio` | Full audio crate — MIR features, MIDI 1/2, bounded production engine, sonification, consent-gated generation (see §5.1). |
| `qualia-peer` | PeerHost application facade replacing libp2p (`PeerHost::{pair, exchange_protected, clinical_session}`); enables `net/qdnf` only. |
| `qualia-app-contract` | Canonical portable command/context/session/surface contracts for Poet, Webizen OS, Storytold apps. |
| `qualia-inference-kernel` | Dependency-free kernel contracts (operator descriptors/oracles, paged attention) so scalar correctness can be tested without the full product graph. |
| `poet` | Webizen Poet: tool_chest (constructs/core/manifolds/payloads), `vibe_host`, `keep_volume`, and a ~180-file `browser/` preview UI with per-domain chain actions. |
| `poet-cli` | Vibe/Poet command-line format, lint, and REPL. |
| `vibe-lsp` | VibeScript language server with catalog intelligence (`catalog_intel.rs`). |
| `vibe-wasm` | wasm-bindgen surface: parse, check, evaluate, and VBC1 bytecode to JS objects. |
| `work-graph` | "Implementation Graph" indexer — scans a checkout for what is **Present** (git tip, crates, bound invoke IDs, doc cites); deliberately does not stamp Live/Planned. |

---

## 1. File-format audit

### 1.1 `.q42` — semantic volumes — **Implemented (v4)**

A `.q42` is a binary semantic-graph volume: 64-byte header, object-sorted `QualiaSuperBlock`s
(40,960 B) of `NQuin` records, `BIDX` block index, postings index, and an optional embedded
paged lexicon.

| Component | Status | Locus |
|---|---|---|
| Volume writer/reader, mmap, LZ4 SuperBlocks | Implemented | `crates/qualia-core-db/src/q42/q42_volume.rs`, `q42/volume/`, `q42/q42_reader.rs` |
| Lexicon (`q42_lex`, 60-bit masked tokens, u16 term length ≤ 65535 B) | Implemented | `crates/qualia-core-db/src/q42_lex.rs`, `query/lexicon.rs` |
| **v4 namespaced paged lexicon** (`Q42LEX v4`, `lookup_parts`/`lookup_owned`) | Implemented (0.0.40.1-dev) | `crates/qualia-core-db/src/q42_lex_ns.rs`; range + JS decoders |
| Legacy lex volumes (v2/v3: `w3c-archives`, schema.org 30.0, `princeton.q42`, `core-ontologies/dist/q42`) | Readable, pending regen | divergence log rows 14, 27 |
| Payload catalogue (`FLAG_PAYLOAD_CATALOGUE = 0x0080`) | **Stub only** (ADR 0014, planned) | divergence log row 23 |
| Range-read / partial fetch (`range_volume.rs`) + WASM VFS (`docs/` playground `vfs.js`) | Implemented | `q42/volume/`, `wasm_storage.rs` |
| Ingest pipeline (RDF/XML, CSV, JSON; `IngestMode::Complete` vs `StripLiterals`) | Implemented | `query/ingest*.rs`, `qualia-cli/src/ingest/` |
| In-graph media catalogue | **Rejected design** — media goes in `.hmc` | divergence log row 20 |

Cautions: `StripLiterals` empties the lexicon (data loss by design); empty-lexicon "Complete"
volumes exist in the wild (`princeton.q42`) — treat as hashed-only; `verify-graph` must use
`--encoder=import` for volumes built via `import`.

**Q42 asset lifecycle submodules** (`q42/` — cold-path infrastructure the spec should know
about):

- `app_manifest/` + `app_registry/` — application manifests and registry over volumes.
- `asset_envelope/` — governed asset envelope: **unknown licenses fail closed**, and
  derived assets inherit the **union of upstream obligations** — the derivation-propagation
  machinery §10's licence-enforcement gap can build on.
- `asset_import/` — bounded import framework: explicit byte/record/chunk budgets, streaming
  input, cancellation, quarantine counts, promote-on-success.
- `journal.rs` — `.q42j` mutable checkpoint + append-only transaction journal for Poet,
  Studio/Desktop workspaces, simulations, and **offline-first sessions** (the persistence
  answer for field collection).
- `q42_kvp.rs` — `QKVP` v1: page-aware KV-cache reuse/quantization/eviction profiles.
- `design_encode.rs`, `execution_profile.rs`, `machine_gpu_profile.rs`, `model_helper.rs`,
  `source_catalogue/` — design encoding, execution/GPU profiles, model metadata, source
  cataloguing.
- `chebi_parse|chebi_map|chebi_query` — ChEBI → q42 (see §5.5).

### 1.2 `.10d` — living-container tensor/mesh format — **Implemented (ABI v1), spec-reserved tail**

Section-oriented container. **Canonical extension is `.10d`; `.d10` is an older non-normative
alias.** Module root: `crates/qualia-core-db/src/container_10d/mod.rs`
(`CONTAINER_10D_ABI_VERSION = 1`).

**Section table** (`container_10d/section.rs` — `SectionType`, `is_implemented()` is the
machine-readable truth):

| # | SectionType | Status | Module |
|---|---|---|---|
| 1 | `QuantizedMesh` (u16-quantized verts in AABB + indices) | Implemented | `mesh_section.rs` |
| 2 | `Tensor10DNodes` (40-byte epistemic atoms) | Implemented | `node_section.rs` |
| 3 | `Reconstruction` | Implemented | — |
| 4–6 | Governance / TemporalIndex / ManifoldHeadTable | **Spec-reserved** (reader rejects) | — |
| 7 | `ProvenanceSidecar` (source+licence+VC, byte-inseparable) | Implemented | `provenance_section.rs` |
| 8 | `FieldSidecar` (self-describing fields; versioned per-vertex normals) | Implemented | `field_section.rs`, `normal_section.rs` |
| 9 | CorrespondenceMap | **Spec-reserved** | — |
| 10 | `Topology` (half-edge + CSR vertex/face adjacency) | Implemented | `topology_section.rs` |
| 11 | `SpatialIndex` (BVH + kd-tree nodes) | Implemented | `spatial_index_section.rs` |
| 12 | `SurfaceReading` (per-vertex RGBA8, magic **`SRD1`**) | Implemented | `surface_reading.rs` |
| 13 | `Materials` (PBR/stylized + submesh ranges) | Implemented | `material_section*.rs` |
| 14 | `TextureCoordinates` (TEXCOORD_0) | Implemented | `texture_coordinate_*.rs` |
| 15 | `ManifoldIdentityV2` (stable identity + typed field refs) | Implemented | `manifold_identity_v2.rs` |
| 16 | **`SkeletalAnimation` (SKL1: rigs, 4-weight skins, named VibeScript motor clips)** | Implemented | `skeletal_section/` |
| 17 | **`EntityOntology` (EONT1: entity DID, taxon, lifecycle stage, energy/damage/hydration/wear, ≤256 affordances)** | Implemented | `entity_ontology_section.rs` |
| 18 | `ProceduralSynthesis` (Voronoi, SDF fillets, allometry, PGA) | Implemented | `procedural_synthesis.rs` |
| 19 | `ImpostorAtlas` (octahedral impostors, 8–64 px far-field) | Implemented | `impostor_section.rs` |
| 20 | `HierarchicalSurfels` (Gaussian-splat-like far field) | Implemented | `surfel_section.rs` |

Notes for the spec: `container_10d/mod.rs` still labels some P0 items (header CRC edge cases,
conformance vectors, WASM parity) as in-progress per phase — check `conformance.rs` /
`integrity.rs` before claiming wire-level guarantees. The earlier "no animation section" claim
(`docs/plans/rolling-commons-format-runtime-integration.md` QG-03) is **stale**: SKL1 exists and
carries named VibeScript motor clips, which is exactly the behavior→rig binding Ecosphere §8.2
needs.

### 1.3 `.hmc` — two different containers share this extension — **divergence**

| Format | Locus | Layout |
|---|---|---|
| **Transparent bundle** (QualiaDB native) | `crates/qualia-core-db/src/bundle/` (`format.rs`, `writer.rs`, `reader.rs`, `mod.rs`) | Magic **`QBDL`**, v1, 64-B header, page-aligned (64 B) intact file entries, CBOR index **footer** (`key`, `kind`, `offset`, `length`, `sha256`, optional CBOR `meta`), whole-file CRC-32C. Zero-copy: `get()` returns a bit-identical standalone file; `entry.offset + seg.offset` gives HTTP-range-addressable interior segments. |
| **Semantic-library ZIP** | `crates/qualia-semantic-library/src/container/` | ZIP archive of verbatim source + derived text/chunks/CML-Turtle, BLAKE3 integrity. |

**Interoperability is unresolved** — this is work packet QG-01 in
`docs/plans/rolling-commons-format-runtime-integration.md`. The Ecosphere manifest example
(`hmc_bundle_magic: "Q42_HMC_V1"`, `manifest.json` at "entry 0", JSON offset table) does **not**
match the shipped format: magic is `QBDL`, the index is a CBOR footer, and there is no fixed
`manifest.json` — the equivalent role is played by `bundle/cbor_ld_manifest.rs` (a
W3C-credential-shaped CBOR-LD manifest entry: issuer/proof, entity type, affordances, geometry
offsets+SHA-256 digests, commons obligations) plus per-entry `meta` blobs. The HCF draft's
Bao/BLAKE3 1-KiB tree and AEAD levels 0–3 are **not** what shipped (`bundle/` uses CRC-32C +
per-entry SHA-256; AEAD flags reserved, unenforced) — divergence log rows 18–19.

Also in `bundle/`: `entity_compiler.rs` / `entity_verifier.rs` (compile/verify an entity bundle),
`twin_compiler.rs` (digital-twin bundles), `variant_matrix.rs` (**Implemented**: phenotype ×
lifecycle stage × LOD × structural-state variant resolution with deterministic O(1) fallback —
directly serves Ecosphere §2.3 LoD/lifecycle selection).

### 1.4 `.p64` — model-weight container — **Implemented**

`crates/qualia-core-db/src/q42/p64_weight.rs` (~2,950 lines): 64-B headers/entries/manifold
records, 14 named tensor roles, real GGUF→p64 compiler (byte-verbatim, f16 dequant, Q4_K→SoA
repack, AWQ ternary/Q4_0 requant), streaming safetensors→p64 transcoder, zero-copy reader with
per-tensor CRC-32C. Distinct `p64\0` magic — must not be `verify-graph`'d as Q42.

### 1.5 `.vibe` / `VBC1` — VibeScript — **Implemented**

`.vibe` is source text (`crates/vibe/`); `bytecode/codec.rs` defines the **`VBC1`** binary chunk
format (constants, functions, per-function `u64` budget field). CLI: `vibe check|fmt|eval|
translate|repl` (`crates/vibe/src/bin/vibe.rs`).

### 1.6 `SRD1` — **name correction needed in Ecosphere doc**

`SRD1` is the 4-byte magic of the `.10d` **SurfaceReading** section — a per-vertex RGBA8 surface
colour table (`container_10d/surface_reading.rs`), *not* a "unified vertex/index buffer" standard.
The Ecosphere doc's use of "SRD1 surface buffers" for unified vertex/index streams should be
corrected to "`.10d` `QuantizedMesh` sections" (which are the actual geometry buffers).

### 1.7 `.ntx` — **not found**

No `.ntx` format exists in `qualia-core-db`. If the spec means N-Triples, the repo handles those
as text through `query/ingest*` / `mini_parser.rs`, not as a named binary format.

### 1.8 Adjacent formats worth knowing

- `QNF` network container draft — `docs/manuals/standards/qnf-network-container-draft.md`.
- `q42/volume/` range volumes; `q42/asset_envelope/`, `q42/asset_import/` (asset packaging),
  `q42/app_manifest/`, `q42/app_registry/` (QApp packages), `q42/yaml_ld_q42.rs` (YAML-LD→Q42),
  `q42/chebi_*/` (ChEBI chemical ontology mapping/parsing/query — directly relevant to
  phytochemistry layers).
- Vault manifests: `identity/vault_manifest.rs`, `docs/manuals/standards/qualia-vault-manifest.md`.
- Sync: `docs/manuals/standards/qualia-sync-protocol.md` + `sync.rs`, `net/peer/replication/`.

---

## 2. Semantic core & logic (spec §2.6.1, Layer 0 & Layer 5)

| Capability | Status | Locus |
|---|---|---|
| 48-byte `NQuin`, SuperBlocks, `q_hash`, `q_turtle!`, parity fold | Implemented | `crates/qualia-core-db/src/lib.rs`, `wal.rs` |
| `SlgArena` 42 MiB static-tabling (917,504 quin slots) | Implemented | `governance/webizen.rs` |
| `did:q42` pointer dispatch (MSB=1) | Implemented | `identity/identifier.rs`, `query/mini_parser.rs` |
| Resolver (hash→URI, inline scalar tags, `format_ntriples_to`) | Implemented — authoritative tag source | `query/resolver.rs` |
| SPARQL engine: SELECT/ASK/CONSTRUCT/DESCRIBE, `GRAPH ?g`, OPTIONAL/UNION/MINUS, sub-SELECT, BIND, aggregates/HAVING, DISTINCT/REDUCED, property paths with bounded transitive closure, SPARQL-Star, temporal clauses, GeoSPARQL WKT | Implemented | `sparql_library/`, `query/query_engine.rs`, `query/graph_index.rs` |
| Webizen governance VM (WAM-style, `SlgOpcode` dispatch) | Implemented; mandatory on native inference path | `governance/webizen.rs` (~3,800 lines), `governance/webizen_bytecode.rs` |
| N3 parser (Strict `=>`, Defeasible `~>`, Defeater `^>`, Linear `-o`; `#asp{}`, `qualia:diffuse{}` blocks) | Implemented; single-triple bodies only | `query/` `n3_parser.rs` (per AGENTS §2-B) |
| SHACL — three evaluators (compiler+VM, query-side, SPARQL-side) + domain shape packs | Implemented; SPARQL-side Tauri path fails closed pending a shape registry | `modalities/logic/shacl/`, `query/shacl_compiler.rs`, `sparql_library/sparql_shacl.rs`, `modalities/logic/*_shacl.rs`, `shapes/*.ttl` |
| OWL 2 RL materializer (full forward-chaining ruleset) | **Library-only** — complete but zero external callers | `modalities/logic/owl/materialize.rs` |

### SHACL-first ontology convention (normative — read before modelling species)

`docs/manuals/standards/shacl-first-vs-owl-ok-class-list.md` is the governing class policy, and
`vibe/src/catalog/mod.rs` enforces it in host-facing text ("Natural persons are rdfs:Class +
SHACL/ShEx; **owl:Thing is forbidden for persons**"). For Ecosphere work the consequences are:

- **SHACL-first / non-Thing** (never subclass `owl:Thing`): persons, kinship, sacred/human
  relations, **living/natural existence, country, ecology-as-life, micro/meso/macro living
  entities**, and life-science classes uplifted from GO/OBO OWL. → Species, specimens, organs,
  populations, habitats, and ecological actors are **SHACL-first** classes.
- **OWL-ok / artifact**: `q42:Volume`s, `vibe:` invoke IDs, containers, manifolds, UI machinery,
  CRS/datasets/protocols/instruments/IDs. → The `.hmc`/`.10d`/`.q42` machinery and descriptor
  records are Thing-apt artifacts; the *living content they describe is not*.
- **Mixed**: Position (coordinates are artifact-ok; what is positioned may be living) and
  Provenance/Claim (structure OWL-ok; claims *about* living entities stay SHACL-first subjects).
- Wording rule: never call persons or living beings "things" — in UI copy, `suggested_fix`
  strings, and class labels.

OWL tooling still exists (`owl/materialize.rs`, `owl/shacl_convert.rs` for lifting OWL
ontologies like RadLex/DICOM/GO/OBO into SHACL shapes) — the convention governs *which side of
the bridge* a class lives on, not whether OWL data can be ingested.
| ASP (real Gelfond-Lifschitz stable models) | Implemented; reachable via MCP + Webizen VM | `modalities/asp.rs`, `mcp/mcp_tool_impls.rs` |

### Core ontology & instrument corpus (`core-ontologies/`, `crates/qualia-core-db/shapes/`)

A top-level vocabulary library that carries the **governance/humanitarian semantics** the
Ecosphere commons model wants — vocabulary assets (N3/TTL), not executable code. Runtime
enforcement depends on whether a given file is compiled into a volume or loaded by a
shape registry; check per file before citing.

- `core-ontologies/` — `INDEX.md`, `README.md`, `SCHEMA_VERSION` + `_authoritative/`
  OHCHR and ICRC corpora + **102 N3 values-credential instruments** derived from UN /
  OHCHR / ICRC and regional human-rights sources.
- Commons model ontologies:
  - `permissive-commons.n3` — production cost, bounded ROI, obligation cost, compensation
    totals, payment/sponsorship concepts, self-extinguishing commons logic (the vocabulary
    twin of `modalities/value_flow.rs`).
  - `social-permissive-commons.n3` — peer-agent relations, time-bounded consent envelopes,
    differentiated persistence, self-hosted agents; external platforms are boundary/
    interoperability cases, not foundational providers.
  - `tiering.n3` — durable/core instruments vs mutable/versioned instruments.
- Vocabularies: `values.n3`, `humanitarian-ict.n3`, `policy.n3`, `selfhood.n3`,
  `jural.n3`, `jurisdiction.n3`, `modal-junctures.n3` (the speech-act taxonomy
  `governance/illocution.rs` implements), `agent-accountability.n3`,
  `capability-credentials.n3`, `inference-lifecycle.n3`, `semantic-instruments.n3` +
  `semantic-instrument-shapes.n3`, `sense.n3`, `cml.n3` (Chemical Markup Language).
- `crates/qualia-core-db/shapes/` — shipped SHACL packs: `core-modalities.shacl.ttl`,
  `logic-modalities.shacl.ttl`, `geometry-asset.shacl.ttl`, `infrastructure.shacl.ttl`,
  `poet-surface.shacl.ttl`, `qualia-agency.shacl.ttl`, `qualia-client-extensions.shacl.ttl`,
  `specialized-libraries.shacl.ttl`. Also `crates/vibe/schema/*.shacl.ttl`
  (vibescript-core, conditioning-v1) and domain packs under
  `bundled/qapps/Anatomy/Knowledge/shapes/` (DICOM-healthcare, RadLex-anatomy).
- **Gap**: no `species`/`ecology` SHACL pack yet — `specialized-libraries.shacl.ttl`
  covers the library domains but a habitat/specimen/population shape pack is new authoring
  (vocabularies above give the jural/provenance skeleton to hang it on).

### Modalities catalog (`modalities/`, ~78 files)

All evaluated through `evaluate_modality` dispatch (MCP `mcp_tool_impls.rs`), and most are
zero-heap:

- **Deontic / jural**: `deontic.rs` (O/P/F), `jural.rs` (full Hohfeldian 8-position lattice,
  correlative-gap detection, multi-party chains, collisions → `RequiresHumanReview`),
  `contract.rs` (capacity-gated formation + 5-state breach lifecycle), `capacity.rs` (legal
  capacity, duress→voidable-at-election, ZCAP attenuation), `delegation.rs` (bounded-BFS ZCAP-LD
  propagation + revocation cascades), `stit.rs` (Chellas/Horty-Belnap STIT, joint liability,
  deliberative counterfactuals), `consensus.rs` (BFT `f=⌊(n-1)/3⌋`, vector clocks, committee
  rotation), `responsibility.rs`, `capability_gap.rs`, `identity_fabric.rs`.
- **Epistemic** (`0x20–0x22`): distributed knowledge, socratic degradation — `epistemic.rs` /
  `modalities/logic/`.
- **Paraconsistent** (`0x30–0x32`): contradiction-tolerant routing.
- **Temporal**: LTL (`0x40–0x44`), Allen interval algebra + RCC-8 — `spatio_temporal.rs`.
- **Causal** (`causal.rs`, Pearl-style: but-for, `do_intervene`, twin networks, backdoor) —
  **Library-only**; the live VM causal opcodes dispatch to `dialectical.rs` instead (a second,
  heap-allocating causal stack wired into `governance/webizen.rs`).
- **Probabilistic / fuzzy** (`fuzzy_query/` under solvers is real-but-unwired).
- **Value flow & reconciliation** — see §6.1, central to the Ecosphere economic model.
- **Argumentation, spatial-temporal, manifold ASP/LTL bridge** (`manifold.rs`).

`foundation/` adds: `crdt.rs` (LWW resolution, delegation verification, 32-slot
`SuspendedTransactionQueue` for M-of-N ratification), `frame_layout.rs`, `telemetry.rs`,
`fuzz_testing.rs`.

---

## 3. Computational geometry & botanical construction (spec §2.6.4, §3, §4.1)

`crates/qualia-core-db/src/specialized_libs/computational_geometry/` — 102+ files, ~84 kLOC,
`GEOMETRY_ABI_VERSION = 1` in `mod.rs`. The ecosystem's own gap review is
`docs/manuals/plantgl-computational-geometry-gap-review-2026-10-14.md` (+ its 2026-10-15
follow-up) — read it first; it states QualiaDB already exceeds PlantGL in exact predicates,
topology, exact constructions, mesh generation, and bounded execution, and records which
botanical/morphological ops were added.

### Verified real kernels (source-audited, grouped by role)

**Exact & foundational**

- `orient3d.rs`, `incircle.rs`, `insphere.rs` — Shewchuk-style adaptive-precision predicates,
  3-stage ladders, cross-checked against BigInt oracles including adversarial cancellation cases.
- `exact_kernel.rs`, `exact_construct_3.rs`, `exact_arrangement.rs`, `sos.rs`, `expansion.rs` —
  exact constructions, exact 2-D arrangements, simulation-of-simplicity.
- `primitives.rs`, `math_geometry.rs`, `distance.rs`, `kernel.rs` — POD points/vectors/frames,
  affine frames, distances, spatial kernels.
- `deterministic_geometry.rs`, `determinism_corpus.rs`, `fixture_registry.rs`,
  `benchmark_corpus.rs`, `corpus_4.rs`, `allocation_counter.rs` — determinism harnesses and
  regression corpora (bit-exactness is a tested property).

**Mesh generation, quality & topology**

- `hull.rs`/`hull_3.rs` (incremental 3-D), `delaunay_2.rs`/`delaunay_3.rs` (Bowyer-Watson),
  `delaunay_refine.rs` (Ruppert), `constrained_delaunay.rs`, `cdt_retriangulation.rs`,
  `triangulation_2.rs`, `triangulation_opt.rs`, `isosurface.rs` (Marching Cubes, canonical
  256-case table), `reconstruct_3d.rs` (implicit/advancing-front), `screened_poisson.rs`
  (real screened-Poisson height-field reconstruction).
- `decimate_3.rs` (Garland-Heckbert QEM), `remesh_3.rs` + `anisotropic_remesh.rs` (Botsch-Kobbelt
  isotropic/anisotropic), `mesh_quality.rs`, `simplify_snap.rs`, `polygon_soup.rs`,
  `polygon_validation.rs`, `mesh_plane_slice.rs` (host-callable).
- `topology.rs`, `surface_mesh.rs`, `surface_mesh_processing.rs`, `half-edge` (`topology_section`
  mirror), `csr_adjacency.rs`, `connectivity.rs`, `combinatorial_map.rs`, `mixed_cell_topology.rs`.
- `quad_octree_mesh.rs` (size-field 2:1-balanced quadtree/octree meshing, conforming
  triangulation), `sparse_voxel_octree.rs` (occupancy SVO + exact DDA ray traversal — designed
  for **biome-scale canopy/light queries**, catalog `svo_raycast`), `bvh.rs` (Morton-median
  split; SAH noted future work), `kd_tree.rs`, `box_join.rs`, `nn_query.rs`, `range_trees.rs`,
  `range_reporting.rs`, `spatial_order.rs` (Morton/Hilbert encoders).

**Booleans, arrangements & planar geometry**

- `boolean_2.rs`, `boolean_3.rs`, `corefine_3d.rs`, `nary_boolean.rs` (approximate f64; `Complement`
  is an honest no-op placeholder), `nary_csg.rs`, `dcel_overlay.rs`, `csg_section.rs`,
  `minkowski_2.rs`, `arrangements.rs`, `arrangement_3d.rs`, `trapezoidal_map.rs`,
  `segment_intersection_2.rs`, `bentley_ottmann.rs` (O(active²) batch — honestly labeled),
  `kirkpatrick.rs`, `point_location.rs` (linear walk + O(log n) `SlabMap`), `half_plane_lp.rs`,
  `ham_sandwich.rs` (brute-force, honest), `calipers_enclosing_disk.rs`, `chordal_axis_2d.rs`,
  `voronoi_2.rs`, `voronoi_variants.rs`, `power_diagram.rs`, `natural_neighbour.rs`.

**DDG, reconstruction & analysis**

- `ddg_operators.rs` — DEC/FEM surface toolkit: cotangent Laplacian, lumped mass, angle-defect
  curvature, iterative solvers.
- `heat_geodesics.rs` (P21.2) — Crane-Weischedel-Wardetzky heat method geodesics on that toolkit.
- `laplacian_3d.rs`, `cknn_laplacian.rs` (two CkNN graph-Laplacian implementations — dedup
  candidate), `point_cloud_diff_geom.rs` (Welzl miniball, Weiszfeld median, PCA OBB/cylinder/
  ellipsoid, Monge-jet principal curvatures, Riemannian-MST normal orientation).
- `statistical_manifold.rs` (P8.3) — probability-simplex ops, Fisher metric, KL as Bregman
  divergence (`fisher_distance`/`kl_divergence`/`kl_bregman_form` catalog ops).
- `persistence.rs`, `tda.rs`, `vr_filtration.rs` — persistent homology; H0 exact, H1 validated
  special-case heuristic (two implementations — dedup candidate).
- `fem_certificate.rs` (P13.10) — FEM-ready mesh certificates: counts, material/BC metadata,
  quality extrema, CRC-guarded encoding sized for a `.10d` sidecar section.
- `features.rs` — geometry/topology feature bridge into `tensor::Tensor10D` for ML/retrieval.

**Dynamics, growth & skeletal kernels (P21 wave — all Implemented)**

| Module | Kernel | Relevance |
|---|---|---|
| `pbd_softbody.rs` | XPBD/position-based dynamics: Verlet + Gauss-Seidel distance/tetra-volume constraints + ground clamp; Tier-1 `HotZeroHeap`; catalog `pbd_step` | **Muscle/fat jiggle, foliage response** — first real soft-tissue kernel |
| `differential_growth.rs` | Anisotropic Laplace-Beltrami surface growth; edge-split refinement; catalog `differential_growth_step` | **Morphogenesis** — leaf/petal/fungal ruffling (partial §3.1 fill) |
| `dual_quaternion_skinning.rs` | DQS skinning (Kavan 2007); catalog `dqs_skin` | Twist-safe articulation (no candy-wrapper collapse) |
| `curve_skeleton.rs` | Mean-curvature-flow skeletonization + graph extraction | Rigging seed generation |
| `medial_axis_3d.rs` | Voronoi-pole medial axis (Amenta-Bern); **pole samples + local feature size, not a full medial surface mesh** | Rigging hierarchies, thickness fields — *partial* vs the MSD gap |
| `sdf_ops.rs` | C¹ smooth-min/max SDF blends + organic primitives | Fauna soft tissue, terrain blending, metaball organs |
| `sparse_voxel_octree.rs` | (above) | Biome-scale occupancy/light queries |

**Botanical & ecological construction (P20 wave — Implemented, per the PlantGL review)**

- `botanical_hulls.rs` (P20.2) — Cescatti-1997 `AsymmetricHull` 6-parameter crown envelope,
  `ExtrudedHull` silhouette extrusion, point-cloud crown fitting.
- `sweep_extrusion.rs` (P20.1) — rotation-minimizing Bishop/double-reflection frames,
  per-station scale/twist profile sweeps, swung surfaces, botanical primitives
  (frustum/cone/paraboloid/disc), axial taper/twist/bend, bounded IFS.
- `turtle_3d.rs` (P20.4) — 3-D L-system turtle (Prusinkiewicz-Lindenmayer), bounded state stack,
  **gravitropism/phototropism torque bending**, generalized cylinder welding, polygon leaves;
  Tier-1 `HotZeroHeap` evaluator.
- `space_colonization.rs` (P20.5) — Runions-Lane-Prusinkiewicz space colonization: perception
  cone/radius, kill zones, phyllotactic whorl buds, graph-constrained colonization.
- `tree_skeleton_3d.rs` (P20.6) — Xu/Livny/Tagliasacchi point-cloud skeletonization, geodesic
  quotient-graph clustering, **Shinozaki/da-Vinci pipe-model radii** (`pipe_model_radii`).
- `canopy_radiation.rs` (P20.9) — canopy radiative transfer: organ-ID/depth rasterization,
  Standard-Overcast-Sky dome, solar-track directions, per-organ PAR/STAR/LAI integration
  (`soc_sky_dome`, `solar_track`, `canopy_integrate` catalog ops).
- `nurbs_fitting.rs` (P20.3) — NURBS eval/fit (Piegl-Tiller LSQ), degree ≤15 stack-bounded.
- `poisson_disk.rs` — 2-D/3-D Poisson-disk scatter (ecological distribution).

**Authoring, CAD & planning**

- `authoring.rs` (P9.5) + `authoring_loft.rs` — deterministic primitive generation, TRS scene
  graphs, bounded lofts (≤256 sections), `.10d` export with provenance.
- `parametric_cad.rs` (P18) — NURBS control points, procedural lattices, shape optimization,
  SDF expression evaluator with hard Booleans.
- `motion_planning.rs` (P16) — SE(2) planning: visibility graphs, grid A*, seed-stable roadmaps.
- `geometry_integration.rs`, `query_frontend.rs`, `gpu.rs`/`gpu_3d.rs`/`gpu_oracle.rs` — bridges,
  GPU kernels and CPU-oracle parity.

**Host reachability**: the Vibe catalog exposes ~94 `ComputationalGeometry.*` invoke IDs —
everything above plus `morton/hilbert_encode`, `triangulate_polygon`, `circumcenter`,
`bezier_eval`, `tukey_depth`, `directional_width`, `k_nearest_sites`, `average_spacing_3d`,
`local_density_3d`, `mean_knn_distance_3d` etc. (full list: `crates/vibe/src/catalog/ids.rs`).
The older JSON/MCP dispatcher `tool.rs::execute_geometry_tool_json` still only recognizes ~17 op
strings — **use the Vibe/Poet host path or the Rust API**, not the JSON tool, for the full
surface.

### Remaining research gaps (repo-confirmed)

4-D growth-event tracking (temporal morphogenesis — differential growth covers *spatial*
morphogenesis; the longitudinal event record is still open), full medial-surface mesh / MSD
(`medial_axis_3d` yields poles + feature size only), SRVF tree morphometrics, Tree-D-Fusion
image→attractor seams. Some P20/P21 cold builders still need individual budget-enforcement
audits.

---

## 4. Entity model, affordances & behavior (spec §2.1 Layer 4, §8)

This is the strongest direct alignment between the codebase and the spec.

| Spec concept | Implemented substrate | Locus |
|---|---|---|
| "World of god" vs world-of-man ontological split | `OntologicalOrigin::{DarwinianEcological, ManufacturedProperty, AbioticAggregate, LiberalArtsWork}` | `entity_view/entity_ontologies.rs` |
| Lifecycle stages (seed→senescent→deceased) | `LifecycleStage::{SeedOrSpore, Juvenile, Adult, Senescent, Deceased}` | same file; mirrored in `bundle/variant_matrix.rs` |
| Ethogram / behavioral catalog | `EntityOntology` affordance table: action ID, `AffordanceCategory`, permission tier, preconditions (state/inventory/energy/biomass/permissions), effects (energy/damage/wear deltas, animation clip ID), bounded intent evaluation emitting `NQuin` action records | `entity_view/entity_ontologies.rs`; persisted in-container as `.10d` SectionType 17 `EONT1` (`container_10d/entity_ontology_section.rs`, ≤256 affordances) |
| Dynamic AI tool manifests for agents | JSON-schema tool manifest generation from affordances | `entity_view/entity_ontologies.rs` |
| Rig completeness / behavior→animation binding | SKL1 section: joints, 4-weight skins, **named VibeScript motor clips**; `render/skeletal/` player/pose/skin/activity | `container_10d/skeletal_section/`, `render/skeletal/` |
| Internal state variables | energy/damage/hydration/wear permill fields in EONT1 mini-header | `container_10d/entity_ontology_section.rs` |
| Behavioral scripts | VibeScript programs + `.vibe`→`VBC1` with per-function budgets | `crates/vibe/` (§7) |
| Stimulus→rule triggers | `LocationTriggerRegistry` maps spatial region→N3 rule quins evaluated in Webizen VM | `domains/geospatial/triggers.rs` |
| Observer/status projection | `entity_view/{observer,projection,attribution,circumstance,rights_filter,package}.rs` | `entity_view/` |

**Gaps vs the spec's §8.1/8.2**: there is no dedicated "ethogram registry" type — the affordance
table + `.vibe` FSMs + SKL1 clips compose the same thing but the spec should name that mapping.
No built-in Boids/flocking solver (nearest: `physics_simulation` N-body + `agent_runtime`
orchestration + `vibe` reactive cells — compose, don't assume). No built-in circadian/weather
stimulus bus — but `simulation/fixed_tick.rs`, `vibe/tick_policy.rs` (Drop/Coalesce/Tear),
LTL/Allen modalities, and `query/spawn_decay.rs` provide the pieces.

---

## 5. Sensory, spectral, physics & environment (spec §§4–6)

### 5.1 Audio & bioacoustics (`audio/`)

**Analysis & semantic bridge**

- `bioacoustics.rs`: `detect_bioacoustic_vocalizations` scans a `TfSurface` into caller buffers
  (zero-heap production path; its test uses `Vec` — scaffolding only) and
  `compile_vocalizations_to_sparql_mm` emits six `NQuin`s per event (type, fragment-of,
  start/end, taxon ID, confidence) — a real sensory→semantic bridge for vocalization repertoires.
- `stft.rs`/`istft.rs` — forward/inverse short-time Fourier transform; `cqt_bake.rs`,
  `stft_bake.rs` — constant-Q and STFT bake pipelines; `tf_surface.rs` + `tf_surface_edit.rs` —
  time-frequency surface container and editing ops.
- `audio_spectral_sheet.rs` — spectral preview binning from tensors (`SPECTRAL_PREVIEW_BINS`);
  `audio_sidecar_link.rs` — audio↔asset linkage records.
- `hypermedia/processors/audio.rs` — WAV ingest into the hypermedia asset/provenance graph.

**Synthesis & DAW-style DSP (`audio/dsp/`)** — complements the spectral-first engine:

`oscillator` (sine/square/saw/triangle), `envelope` (ADSR), `filter` + `biquad_df2t` (RBJ
cookbook LP/HP/BP/notch, DF2T form), `lfo`, `effects` (delay/reverb/compressor/EQ),
`midi_transport` (note utils, quantization, transport FSM), `meters` (waveform, phase
correlation, **LUFS loudness**).

**Spatial/parametric audio**

- `acoustic_plane.rs` — "U3 AcousticPlane": SPSC sonic token ring + parametric uniform for an
  AudioWorklet; shares the U1 Tensor10D SoA read-only until bake sidecars land.
- `hrtf.rs` — `binaural_from_position` + `room_damp_from_manifold` binaural/room rendering.
- `acoustic_sab.rs` — shared-array-buffer acoustic transport; `dsp_kernel.rs` — stack-resident
  `ParametricVoiceState` (phase/freq/gain/FM-index) for parametric sonification of tensor data
  (epistemic τ/FM modulation; mirrored by the JS worklet).

**Product crate (`crates/qualia-audio/`)** — a second, larger audio surface that owns the
product-facing pipeline on top of the core kernels:

- `features/` — full MIR analysis stack: `cqt`/`stft_stream`/`cqt_stream`, `mel`, `fft`,
  `filters`, `framing`, `window`, `loudness`, `energy`, `envelope`, `peaks`, `spectral`,
  `pitch` + `multipitch` + `pitch_midi`, `rhythm`, `tonal`, `salience`, `structure`,
  `contours`, `vad`. Species-call classification features (pitch tracks, rhythm, tonal
  salience) exist here rather than in `audio/bioacoustics.rs`.
- `midi/` — the complete MIDI surface: `message`, `smf` (Standard MIDI Files), `ump`
  (MIDI 2.0 Universal Packets), `mpe`, `synth`, `sequencer`, `sync`, `tuning`,
  `instrument`, `bridge`.
- `models/` — P64-packaged learned models: `aed_p64` (audio event detection), `crepe`
  (pitch tracking), `separation`, `speech_p64`, `audio_tokens`, `loader`.
- `production.rs` — bounded 16-track production engine: zero-alloc `process_block`
  (≤512-sample blocks); offline bounce uses the same path so FX render sample-for-sample
  deterministic. `fx/` reverb + stereo.
- `aqa/` — audio-quality assessment (`intrusive` metrics + `mos` scoring) — usable as an
  automated QA gate on synthesized bioacoustic calls.
- `generation.rs` — reference tone synthesis, `VoiceConsent` revocation check, simple stem
  separation; **licensed TTS/neural separation remain explicitly gated** ("COMPLETE-WITH-GATE").
- `speech.rs` — greedy phone decode over log-mel, a **scaffold, not full ASR**; unknown
  language emits only inventory phones or blank — never a silent wrong map.
- `sonify.rs`, `cross_modal.rs` (visual↔auditory co-occurrence proposals — honest
  "overlap ≠ causality"), `semantic.rs`, `media_store.rs`, `language.rs`,
  `capability_registry.rs`, `pipeline.rs`, `capture.rs`, `session_history.rs`.

**Gap**: procedural aero/hydro/pyro-acoustic synthesis parameters (spec §4.3) have no dedicated
module — representable as `.10d` FieldSidecar + q42 parameters driving `dsp/` voices, but
nothing generates them today. Speech is a gated scaffold; no full ASR/TTS. No wired path
from `qualia-audio` MIR features into species-vocalization taxa yet (features exist; the
taxonomy/recipes are the human/music-oriented defaults).

### 5.1b Vision, imaging & bio-imaging (`specialized_libs/computer_vision/`, `qualia-vision`)

The native CV stack lives in `qualia-core-db` (pure Rust, no OpenCV); `qualia-vision` re-exports
the kernels and owns the product surface (consent, recipes, weights, capability ledger).

**Core CV (`computer_vision/cv/`, `ops/`, `embeddings/`, `gpu/`)** — all caller-buffered:

- Codecs: `png_decode`/`png_encode`, `jpeg_decode`, `demux_mp4` + `FrameSequence` (bounded
  `MAX_SEQ_FRAMES`).
- Filters/morphology: box/gaussian/median blur, bilateral denoise, erode/dilate, histogram +
  equalize.
- Features/matching: Canny, Sobel, FAST corners, BRIEF descriptors (`DESC_LEN`), Hamming
  matching; contours/`find_external_blobs` → `BlobBox`.
- Motion: `lucas_kanade_step` optical flow; `warp_affine_u8`/`warp_perspective_u8`.
- Kernels: `ops/{conv2d,pool2d,resize2d}.rs`; `gpu/` device policy + Forge bicubic/nearest-resize
  WGSL adapters (CPU oracle default; `gpu` feature attaches Forge — honest `Cpu`/`Unavailable`
  reporting until wired).
- Embeddings: `ahash_u64`/`dhash_u64` perceptual hashes, `color_hist_embed_rgb`,
  cosine/hamming distances — for near-duplicate and retrieval over species imagery.
- Stable ABI: `types.rs` `ImageView` (borrowed, strided) / `PixelFormat` / `MAX_DETECTIONS=64`.

**Bio/medical imaging (`computer_vision/bio/`)** — directly serves the spec's microscopy suite:

- `dicom_lite/`: DICOM tag parsing, SUV-from-activity, anonymization tag map.
- `histopathology/`: Macenko stain deconvolution, Reinhard normalization, SNMF unmixing
  (`snmf_unmix_lite`), optical density, Lab↔RGB.
- `medical/`: Hounsfield-window mapping, isotropic resampling (NN), MIP projection, NNLS
  spectral unmixing.
- `morphology/`: watershed markers, Voronoi-Otsu labeling, tophat, nucleus features, extended
  minima — cell/nucleus segmentation primitives for the histology layer.
- `radiomics/`: first-order stats, GLCM texture features, 2-D/3-D shape descriptors.
- `tracking/`: Crocker-Grier particle linking + particle features (cell/particle time series).

**Super-resolution (`computer_vision/sr/`)**: classical SR + tiled pipeline (tile plan/extract/
blend) + optional learned sessions — `onnx_sr_session` behind feature `vision-onnx` (default
OFF), `esrgan_session`, `swin_session`, `cnn_light_session`; `device_policy` with thermal/VRAM
budgets.

**Image→3D (`computer_vision/spatial/`)**: `MeshIR` + `mesh_ir_quality`/`validate`/
`print_readiness`, exporters to **GLB/OBJ/binary-STL/3MF**, `sigma_map`, `twin_bridge`;
`qualia-vision::spatial::{image_to_3d,compile_10d_handoff}` is the product-side `.10d` handoff.

**Product surface (`qualia-vision`)**: `biosense/` (consent-gated — affect, biometrics, face +
face_mesh, liveness, magnification, pose, respiration, rPPG), `capability/` machine-readable
ledger, `recipes/` (`compile_hr_observation_quins`, `self_monitor_pulse(+evm)`,
`respiration_monitor`, `sparql_mm_observation_query`, `challenge_pad_from_mesh_trace`),
`tracker/` (greedy-IoU association + score tiers), `weights/` (LiteRT/ONNX byte loaders +
session), taxonomies (COCO/ImageNet-families, ADE20K, Cityscapes, KITTI, OpenImages, Pascal-VOC,
VGGFace2), `semantic.rs` `VisionQuin` epistemic observations, `media_store.rs`.
Design rules from `lib.rs`: hot path caller-buffered; model outputs are **epistemic
observations, not ground truth**; dense pixels/biometric templates never enter NQuins.
Earlier note stands: `cpu_reference.rs` is a deterministic heuristic backend, not a neural
detector; nothing else in the workspace depends on `qualia-vision` yet.

**Other vision touchpoints**: `query/visual_model_bridge.rs` (model→graph bridge),
`medical_computing/image_dsp.rs` (window/level, Otsu, Sobel — a second, smaller medical-DSP
path), `hypermedia/processors/image.rs` (EXIF), `render/anatomy/` + `anatomy_pack.rs`
(WebGL2 anatomy renderer + organ bundles).

### 5.2 Spectral & multispectral (`render/spectral*.rs`)

- `spectral_basis.rs`: Jakob & Hanika (2019) bounded spectral reflectance basis (R(λ)∈[0,1]
  guaranteed) + closed-form Gaussian EMF emission moments for the `[α,μ,σ]` tensor encoding +
  thin-film interference — the right substrate for UV/VIS/NIR/SWIR reflectance libraries.
- `spectral.rs` (σ→CIE XYZ→sRGB CPU oracle), `spectral_lut.rs`, `spectral_kernel.rs`,
  `spectral_blend.rs`, `spectral_operator.rs`, `spectral_oracle.rs`, `metamer.rs`, `gamut.rs`,
  `gpu_colour_kernel.rs`.
- **Gap**: no hyperspectral image-cube section type; use `FieldSidecar`/`Reconstruction` or a new
  spec-reserved section. No per-band "health/hydration state" semantics — that's schema, not code.

### 5.3 Physics & biomechanics

| Area | Status | Locus |
|---|---|---|
| Render-side physics: AABB admission (`admission.rs`), mass/momentum (`material.rs`), PGA kinematic joints (`joint.rs`), material floor | Implemented | `render/physics/` |
| Driven spring + thermal reservoir response kernels (canonical RK4 ODE core) | Implemented | `render/physics/response/{spring,thermal}.rs` |
| **XPBD soft-body**: Verlet + Gauss-Seidel distance/tetra-volume constraints, ground clamp, Tier-1 zero-heap, catalog `pbd_step` | Implemented (P21.8) — minimal kernel for muscle/fat jiggle & foliage | `specialized_libs/computational_geometry/pbd_softbody.rs` |
| Vegetation instancing + wind/collider bending (stiffness/recovery/damping params, zero-heap, GPU-instance output, ≤16 colliders/step) | Implemented | `render/vegetation.rs` |
| Water surface + shoreline spatial queries | Implemented | `render/water_surface/` |
| `physics_simulation/` library (~25 files): `mechanics` (projectile+drag, oscillator), `ode`/`time_integration`, `nbody` (2-D Newtonian, energy/AM conservation), `molecular_dynamics` (2-D Lennard-Jones, reduced-units temperature), `quantum` (1-D Schrödinger spectrum via `symmetric_eigen`), `population` (logistic growth vs analytic), `emf` (sources + 1/r field), `cfd` (Burgers), `fields`, `boundary_initial`, `discretization`, `solvers`, `domain_model`, `metrics` (conservation diagnostics), `distributed` (`MeshCoordinator` — non-partitioned placeholder) | Implemented (one canonical model per domain) | `specialized_libs/physics_simulation/` |
| Engineering analysis: `fem` (2-node truss + Euler-Bernoulli frame, stiffness/mass assembly, BCs, dense-LU solve), `dynamics` (Newmark-β linear/nonlinear + Newton-Raphson), `buckling` (Euler), `vibration` (modal + FRF), `structural`/`mechanical` (axial, stress invariants, fatigue), `thermal`/`thermal_conduction` (Thomas algorithm), `fluid`/`cfd` (2-D LBM D2Q9), `reliability` (Series/Parallel/K-of-N, Birnbaum), `survival` | Implemented | `specialized_libs/engineering_analysis/` |
| WebGPU physics solvers: incompressible 2-D Navier-Stokes (Chorin projection, validated vs Taylor-Green decay), Maxwell FDTD, heat, wave, N-body, dense GEMM + WGSL sources | Implemented in `qualia-extensions` | `crates/qualia-extensions/src/webgpu_extension/` |
| PINN: ternary MLP forward + finite-difference PDE residuals (heat/Lorenz/Navier-Stokes; analytic fallbacks); SNN LIF + STDP; local QPU statevector | Implemented (partial; remote QPU mocked) | `qualia-extensions/src/{pinn_extension,snn_extension,qpu_extension}.rs` |
| **Vibe physics facets** (`vibe/src/physics/`): `MaterialSignature` with `MechanicalFacet` (Young's modulus GPa, density), `ThermalFacet`, `AcousticFacet`, `ChemicalFacet`, `OpticalFacet`, `SpectrumReading`; `PartContinuant`; `FieldDeclaration`/`AnalyticFieldProfile`/`FieldQuantity`/`FieldUnit`; `evaluate_field_interactions`/`evaluate_pairwise_interactions` → `InteractionEvent::{YieldExceeded, PhaseChange, …}`; `trajectory`, `morphism`; `is_living_kingdom`, `GAME_WATER_FORMS` | Implemented | `crates/vibe/src/physics/{fields,laws,materials,morphism,trajectory}.rs` |
| Thermodynamics/statistical ensembles (MCMC macroscopic properties) | Implemented | `domains/physical/thermodynamics.rs` |

The Vibe `MaterialSignature` facet model is the closest existing substrate for the spec's §5
"material mechanics" parameters — species materials can carry mechanical/thermal/acoustic/
chemical/optical facets, with `laws.rs` evaluating ambient-field interactions into deterministic
transformation events.

**Gaps vs spec §5**: no full soft-tissue/muscle *system* (XPBD kernel exists but no muscle
actuation/FEM-soft-tissue coupling), no MOR/windthrow failure solver (the Beaufort-stratified
response would be parameter curves in `.10d` FieldSidecar + `value`/`vibe` logic), no
xylem-hydraulics/cavitation model, no combustion model. These are **data-layer** concepts today,
not computed physics. The F/A tier model
(`docs/manuals/computational-3d-assets-and-digital-twins.md` §4) is the right way to scope them:
real compute tops out at F2/F3 (bounded reduced-order + small FEM/CFD) plus bounded XPBD.

### 5.4 Geospatial & environmental (`domains/geospatial/`)

- `reference_frame.rs` + `scale_continuum.rs`: nested coordinate frames, macro→micro transitions,
  RCC-8 containment — supports planet→organism scale continuity.
- `geodetic.rs` (WGS84), `dem.rs` (elevation), `environmental_raster.rs`, `terrain_pipeline.rs`,
  `spatial.rs` (H3 context), `vector_scene/` (OSM vector→scene), `streaming.rs`,
  `spatial_sync.rs` (ed25519-signed planted-asset publish + LWW pull — the commons contribution
  mechanism), `ar_anchor.rs`, `canvas_query.rs` (quadtree), `canvas_rights.rs` (deontic placement
  gating; **`is_owned` is a mock**), `steward.rs`/`stewardship.rs` (**M-of-N steward quorums with
  explicit-denial guard over bounded spatial domains** — direct fit for species/territory
  stewardship), `triggers.rs` (region→N3 rules).
- `adapters/`: consent-gated request builders for ~15 external standards (GBIF biodiversity,
  OSM Overpass, WMS/WCS, STAC, DEM, OPeNDAP, IVOA TAP/ADQL, 3D Tiles, CKAN, SPARQL/Wikidata,
  astrometry) over ~90 curated open-data endpoints (`canvas_defaults.rs`). Six adapters parse
  responses into provenance `NQuin`s (ckan, gbif, osm, stac, ogc_3d_tiles, sparql); WMS/DEM/
  OPeNDAP/IVOA response parsers still needed.
- Biological domain: `domains/biological/bioinformatics.rs` (Smith-Waterman, Needleman-Wunsch,
  k-mers, MinHash, FASTA, phylogenetic merge — sequence computation, not tissue mechanics).

### 5.5 Chemistry, biochemistry, medical (`specialized_libs/`, `medical/`, `q42/chebi_*`)

- `chemistry_modeling/` (5.4 kLOC): Arrhenius kinetics, Joback-style QSPR, LJ molecular dynamics,
  structural properties (mass/formula/moments of inertia), real RHF/STO-3G ab-initio for
  closed-shell H/He (SCF, dipole, Mulliken) — MCP `chemical_analysis`.
- `organic_chemistry.rs`: SMILES parsing, Lipinski/logP, fingerprints — WASM + UI exposed.
- `q42/chebi_parse|chebi_map|chebi_query`: ChEBI ontology → q42 — phytochemical/metabolite
  indexing substrate.
- `medical_computing/` (6.2 kLOC): 21 deterministic clinical formulas + image DSP + naive-Bayes
  differential (caller-supplied KB) — each result labeled non-diagnostic.
- `medical/clinical_engine.rs`: Framingham/CHA₂DS₂-VASc/SOFA/FHIR (native; WASM gets a zeroed
  shim).
- **Gap**: no PK/PD or dose-response for phytochemicals beyond first-order PK formulas; no
  metabolomics pipeline. TK/BC labels are a schema+deontic-policy matter — substrate exists
  (provenance sections, sensitivity classes, deontic opcodes), enforcement policy does not.

**Anatomy/physiology record model (`wellfare-core/src/anatomy/`)** — a precedent worth
stealing for the species-anatomy layer. It is a human-clinical vault model, but its
structure is exactly the shape the Ecosphere needs for organism-level records:
`constitution` (baseline body plan), `systems`/`physiology` (functional subsystem
breakdown), `pathway` (causal chains), `dyad` (paired-organism relations), `factor`
(contributing stressors), `measurements`/`observations` (time-stamped evidence),
`coverage`, `scorecard` (composite health), `temporal` (longitudinal), `accumulate`,
`lens` (view projections), `registry`, `knowledge` + `knowledge_context`, `birth`,
`bridge`. Plus crate-level `life_records`, `medication`, `sleep_analytics`,
`mental_wellbeing`, `welfare_support`, `guardianship`, `conditions`, `assessment`,
`n3_rules`/`shapes` (N3 rules over the records), `authority_attestation`,
`companion_pairing`/`companion_sync`, `live_share`. **Gap**: it is person-oriented —
no generalized species-anatomy ontology exists; the reusable part is the record
*shape* (constitution → systems → pathways → observations → scorecard), not the
vocabulary.

### 5.6 Mathematical substrate (`solvers/`, ~178 files — real numerics, mostly wired)

The numeric engine under all of the above. Canonical rule (per the repo's own audit): no
inline re-derivation in domain libraries — they marshal into these kernels.

**Core algebra & calculus**

- `linear_algebra/`: `gemm`, `lu`, `qr`, `cholesky`, `eigen` (symmetric cyclic-Jacobi), `svd`,
  `spectral`, `vector` — heavily wired (gguf_bridge, wasm_bridge, swarm, learning).
- `calculus/`: `quadrature` (adaptive Simpson/Gauss-Kronrod, checked vs ∫sin=2 and ∫x¹² to
  1e-13), `ode_solver`/`ode_adaptive` (RK45 DOPRI5)/`ode_advanced` (Yoshida-4 symplectic),
  `differential`, `exterior`, `manifold`, `mechanics`, `potential`, `analysis`, `grid`,
  `tensor_integrity`, `tensor_provenance`, `workspace`, `capabilities`, `dense`.
- `transforms/`: `fourier`, `laplace`, `ztransform`.
- `vector_calculus/`: `differential`, `integrals`.
- `interpolation/`: `lagrange`, `spline`, `least_squares` (OLS + polynomial, inferential stats).
- `exact/`: `bigint`, `rational` — arbitrary-precision backing for exact predicates.
- `number_theory/`: `primes`, `modular`, `combinatorics`, `arithmetic_functions`.
- `special_functions/`: `airy`, `bessel`, `orthogonal` polynomials, `zeta` (ζ(2), Apéry to 1e-8).
- `units/`: `dimension`, `quantity`, `conversion`, `constants` — dimensional analysis.
- `optimization/`: `metaheuristics` (hill-climb, SA, ABC) + `quantum_optimizers/` (QAOA, SPSA);
  `shared/convergence` (real, no callers).
- Root files: `activation`, `attention` (SDPA), `feed_forward` (SwiGLU), `rope`, `polynomial`.

**Statistics (`statistics/` + `regression/` + `hypothesis/` + `distributions/`)**

descriptive/correlation/histogram/robust/anomaly/information/timeseries; distributions
(normal, Student-t, χ², Fisher-F, multivariate-normal, special); hypothesis (t-tests, ANOVA,
χ², nonparametric — two functions honestly flag "rough approx" in comments); 11-file regression
suite (simple/multiple/diagnostics/influence/outliers/selection/design/discrete/spurious/verify).

**Learning (`learning/`, ~67 files — real but mostly library-only)**

regression (linear/ridge/lasso/Bayesian/PCR/PLS), classification (LDA/QDA, KNN, naive-Bayes,
SVM + multiclass), clustering (k-means, GMM, hierarchical), dimensionality (PCA, SOM), trees
(decision tree, random forest, boosting, BART), GLM (family + multinomial; IRLS logistic),
gaussian_process, graphical_models (factor graph), sequential (HMM, Kalman), sampling (MCMC),
variational (Gaussian VI), survival (Cox, Kaplan-Meier), splines (GAM, smoothing), resampling
(bootstrap, folds, permutation), experiment (A/B, bandits, power), active (committee/density/
uncertainty), preprocessing/scaling, metrics, multiple_testing, **kg_embedding** (train/score/
predict — the one production-wired path, used by `services/swarm` + `wasm_bridge/engine/graph`).
Reachability: `ml_inference` MCP reaches only `specialized_libs/machine_learning` (GGUF load +
MLP forward + compression), not this tree.

**Symbolic & reasoning math**

- `specialized_libs/symbolic_algebra.rs` + siblings (`symbolic_integration`, `symbolic_limits`,
  `symbolic_ode`, `symbolic_series`, `symbolic_solve`, `symbolic_trig`, `symbolic_assumptions`) —
  a small CAS: expression trees over rationals, `simplify`/`differentiate`/solve, results
  bridgeable back into the graph via `expr_citation_hash`. Cold path (allocates).
- `specialized_libs/multivar_calculus.rs` — symbolic gradient/Jacobian/Hessian feeding IRLS,
  Laplace approximations, second-order optimizers.
- `specialized_libs/polynomial_algebra.rs` — dense univariate polynomials: arithmetic, Euclidean
  GCD, resultants (fail-closed, no fabrication).
- `specialized_libs/category_theory.rs` — Object/Morphism traits used by `qualia-q-forge`
  quantum gates and physics morphisms.
- `specialized_libs/constructibility.rs` — compass-and-straightedge feasibility (Wantzel degree
  criterion, Gauss-Wantzel polygons) — the NL→3D-fabrication feasibility gate.
- `geometric_algebra/` + `simd_kernel/` — PGA-style GA with SIMD backends.
- `graph_match/` (approximate + fuzzy similarity), `graph_opt/` (hierarchical path, spreading
  activation), `grounding/` (claim support scoring — wired into `inference/orchestrator.rs`),
  `ontology_align/` (type-level forbids auto `ExactMatch`), `fuzzy_query/` (f-SPARQL degree
  algebra — zero callers), `symbolic_logic/` (DPLL SAT + defeasible forward-chaining;
  `resolve_defeat`/`resolve_preference` are honest no-op stubs), `qpu/` (dispatcher,
  `QuboFormulation` pre-solver).
- `domains/mathematical/geometric.rs` — Lorentz-hyperboloid projection of NQuin embeddings.

WASM surface: `wasm_bridge/engine/{cas,crypto,exact,graph,linalg,numerics,stats,
stats_regression,transforms,units}.rs` + `wasm_bridge/geometry.rs` expose this stack to the
browser (`docs/compute-engine.html` is the live consumer).

---

## 6. Rights, economics, identity & networking (spec §§1.1–1.2, 2.5)

### 6.1 The obligation-cost / commons-sunset model already exists — **Implemented**

`crates/qualia-core-db/src/modalities/value_flow.rs` implements, in zero-heap integer arithmetic,
the exact economic mechanism the spec describes:

| Spec mechanism | Function |
|---|---|
| Baseline obligation cost + capped recovery multiplier | `commons_cost(production_cost, roi_cap_percent, max_roi_percent)` — `cost × (1 + min(roi_cap, max_roi))` |
| Humanitarian-vs-commercial royalty tiers | `royalty(base, agent_multiplier_percent)` — category-scaled multiplier |
| Payment pool accumulation | `pool_after(pool, payment)` |
| **Commons sunset trigger** | `is_commons_discharged(pool, cost)` — "once the pool meets the cost, the obligation is discharged and the use is freed globally" |
| Tranche/delta accounting | `outstanding`, `ancestor_royalty`, `royalty_tree_total` (generational derivative-work shares) |
| Anti-extraction guards | `usury_ceiling`, `is_usurious`, `check_usury` (`UsuryError`) |
| Utility | `eroi`/`eroi_viable`, `convert_currency`, `apply_tax`, `liquidity_step/after` |
| Reconciliation between views | `modalities/reconciliation.rs` (`ReconciliationRecord`, `compare_views`, `cascade_recheck_flags`) |

What is **missing** is wiring: no ledger persistence binding `pool` state to an asset DID inside a
bundle manifest, and no automatic license-transition side-effect when `is_commons_discharged`
fires. The `cbor_ld_manifest.rs` has a commons-obligations field that is the natural carrier.

### 6.2 Micropayments & settlement

- `services/ilp_dispatcher.rs`: real ILP-STREAM-over-SPSP micropayment dispatcher — payment-pointer
  resolution, Nym-mixnet routing option, offline NDJSON queue, stablecoin fallback
  (`did:wallet:`/hex → USDC ERC-20 / XRPL IOU), and **every payment written as an N-Quad receipt
  into `.q42`** — the audit trail the spec wants.
- `services/swarm/`: **verify-before-pay** distributed jobs (`dispatch.rs`, `executor.rs`,
  `verify.rs`, `settlement.rs`, `isolate.rs`, `job.rs`) — Personal/Collaborative/**Paid**
  ("solar-excess" compute sales) modes with a mandatory local verification gate before any
  payment instruction. Real KGE-trainer jobs run through this path.
- Wallet (client-core): real BIP32/secp256k1 HD derivation, P2PKH, eCash via `chronik.rs`
  indexer; XMR deliberately not emitted (no test vector). `qpu_*` files for quantum-provider
  dispatch are separate.
- **Gap**: protocol-level per-byte-slice metering for HMC range fetches is not wired; ILP receipts
  + bundle `meta` are the pieces.

### 6.3 Identity, credentials, provenance

- `identity/`: `identifier.rs` (`did:q42` parse → MSB=1 pointer), `webizen_did.rs`,
  `key_vault.rs`, `credentials/`, `access_modality.rs`, `agency.rs` (author-scoped Merkle roots;
  `derive_lane_key` is SHA256, not PBKDF2 — known gap), `profiles.rs`.
- `did_qi/`: a full `did:qi` method — document encode/decode, git-object binding, mailbox binding,
  UTXO anchoring, test vectors.
- `crypto/`: ML-DSA-65 (FIPS-204), Groth16 over BLS12-381 (incl. `deontic_access` circuit,
  soundness-tested), verifiable credentials, PQ-KEM shim; `specialized_libs/
  cryptographic_library` (Ed25519, AES-256-GCM, ChaCha20-Poly1305, SHA-2, BLAKE3, HKDF, real
  SPHINCS, ML-KEM keygen-only, RSA/ECDSA behind `interop-crypto` feature) — **library-only**, no
  production callers.
- `governance/`: `consent_contract.rs` (ed25519 time-bounded, category-scoped consent grants
  + immutable non-reactivatable revocation, mapped to deontic super-quins), `provenance.rs`,
  `illocution.rs` (implements `modal-junctures.n3`: binding-weight conflict resolution with
  speaker-authority scaling, exemptive-defeater override, **paraconsistent retention of
  genuine conflicts** rather than silent picks), `coordination.rs` (**multi-agent
  coordination opcodes `0x70–0x7F`** executed atomically in the Sentinel VM: root-delegation
  signature gate, `SuspendedTransactionQueue` yield, anti-usury resource contract + circuit
  breakers, fidelity/efficiency arithmetic — the instruction-level substrate for
  multi-agent/ecosystem coordination), `law_packages.rs` (signed law packages with
  `LawNature::{Physical, Fictional}` — provenance travels with the law), `instrument_trace.rs`
  (local customer-readable audit ledger), `web_civics.rs` (Ed25519→ULA-IPv6 derivation —
  cryptokey routing where the DID is mathematically the address), `webizen/` (agreement,
  arena/arena_admit, clinical_native, opcode, vm).
- `domain_licence/`: license codec/model/registry/verify — asset licensing substrate.
- `crates/qualia-cooperative-core`: the supported-agency/cooperative governance model
  (decisions, membership, contribution accounting) — vocabulary + logic for commons
  entities, distinct from the engine's deontic layer.
- `net/disclosure.rs`, `governance/consent_contract.rs` + client-core accountability fabric
  (consent credentials, hash-chained ledger, Shamir recovery, dead-man/incapacity switches).
- `.10d` `ProvenanceSidecar` (SectionType 7) keeps source+licence+VC byte-inseparable in-container.

### 6.4 Networking, sync & distribution

| Layer | Status | Locus |
|---|---|---|
| QDNF (native protocol fabric — no libp2p/IP-socket dependency): authority, bearer, biometrics, cells, clinical, contracts, crypto, **economics**, evidence, fabric, frame, link, policy_labels, profiles, replication, resolve, route, session, vertical | Implemented protocol libraries | `net/qdnf/` |
| Peer fabric: capsules, carriers, leases, intents, mailboxes, outcomes, receipts+store, relays, sessions, wire | Implemented | `net/peer/fabric/` |
| Peer cells: admission, descriptors, isolation, budgets, SPSC, workers | Implemented | `net/peer/cells/` |
| Replication: cache, checkpoint, custody, DTN, merkle, proofs, tombstones, transfer, resume | Implemented | `net/peer/replication/` |
| SocialWebNet (userspace WireGuard mesh), mesh datagram/probe/service, STUN observe, outbound relay, connectivity (ICE/STUN/TURN/WSS/H2 capsules), swarm+sync behind `libp2p-compat` feature | Implemented (native-gated) | `p2p/` |
| DNS subsystem (wire, resolver, cache ring, quin records, ontology, SDN, lossless archive) | Implemented | `net/dns/` |
| WebTorrent routes + seeder (daemon `/torrent/*`) | Implemented | `services/webtorrent_*.rs`, `services/daemon*.rs` (port 4242) |
| `fetch_10d` by content hash | **Partial** — real consent+CRC logic around a `mock_network_fetch` | `net/fetch_10d.rs` |
| `acoustic_ble_mesh` | **Simulated** — all I/O boundaries are labeled mocks; do not cite as shipped | `net/acoustic_ble_mesh/` |
| `nym_adapter`, `sonic_token`, eBPF filter/firewall, host topology | Implemented/mixed | `net/` |
| Graph sync: LWW CRDT, suspended transactions, `sync.rs`, `webizen_sync.rs` | Implemented | `foundation/crdt.rs`, `sync.rs`, `governance/webizen_sync.rs` |
| CAR/IPFS verification & range reads | Implemented (per prior audit) | `q42/volume/` range paths |

**QDNF submodule detail** (`net/qdnf/` — the native protocol fabric; `qualia-peer`'s
`PeerHost::{pair, exchange_protected, clinical_session}` is the application API, not
`vertical.rs`):

| Submodule | Contents | Ecosphere relevance |
|---|---|---|
| `authority/` | owner, permit, verified, digest_bind, decoded, policy | Who may admit/serve an asset |
| `bearer/` | contract, fragment, ipc, leased_ipc, mtu, nym, raw_ethernet | Bounded wire bearers incl. raw-Ethernet/no-IP |
| `crypto/handshake/` | ML-KEM-768 identity/shares/failures | PQ handshakes |
| `economics/` | adapter, classify, concurrent, consent, **meters** (`MeterKind::{EnergyJoules, Bytes, Time, Compute, Unknown}` — unknown is not zero and not measured), **obligation** (stable project-obligation ownership over a bounded op table), recover, reserve, roles, router, settle | **The missing metering substrate for per-asset commons accounting** — resource quantities are already typed and obligation ownership is explicit; binding these meters to `value_flow` pools is integration work, not invention |
| `evidence/` | classes, export, holds, promote, seal | Evidence sealing/export for stewardship claims |
| `frame/`, `link/` | frame encode/decode/header/extensions; link adjacency/challenge/cookies/discovery/neighbor/budget | Wire format + adjacency |
| `policy_labels/` | IFC lattice: declassify, egress, flow, join, lattice, projection | **Information-flow labels** — the machinery for rights-tiered/sensitivity-tiered asset distribution (maps to context sensitivity classes) |
| `profiles/` | hostile-environment protection sets E16.1–E16.7: budget, catalog, cover, gateway, negotiate, offline, requirements, scenarios, vault; honest limits (remote wipe/attestation cannot guarantee post-seizure safety; no fake TPM; cover traffic ≠ traffic-analysis immunity) | Protection profiles for field/offline collectors |
| `resolve/` | `qsr` resolver + query/records | Content discovery |
| `route/` | ads, constraints, flood, forwarding, hysteresis, index, overlap, plan, realm, select, spf, validate — constraint-first admission (`validate→index→plan`), `compute_spf` is a 16-node oracle | Route planning under constraints |
| `session/` | bind, clinical, congestion, contact, ... | Session lifecycle |
| `biometrics/`, `cells/`, `clinical/`, `contracts/`, `fabric/`, `registries/`, `replication/` | verticals & scaffolding | `biometrics/mod.rs` explicitly disclaims operational FAR/FRR, liveness, unlinkability, SMPC — honest boundary |
| `vertical.rs` | 64-byte two-peer `local-ipc-v1` slice | **Demo/oracle, not the app API** |

`docs/plans/qdnf-0.0.37-enhancement-progress.md` tracks live status. For Ecosphere the
high-value wiring is `economics/meters.rs` + `obligation.rs` → `value_flow` commons pools,
and `policy_labels` → the context sensitivity classes + `canvas_rights` placement gating.

---

## 7. VibeScript / VibeVM & agentic interfaces (spec Layer 4, §8.3)

### 7.1 Language & runtime (`crates/vibe/`)

- Typed language: `ast.rs`, `parse/`, `check.rs`, `types.rs`, `pretty.rs`, `translate.rs`
  (locale-aware keywords — `locale.rs`), `diagnose.rs`.
- Execution: `bytecode/` (`VBC1` codec, compiler, `vm.rs`), `eval/` (call/expr/program/stmt),
  `budget.rs` (per-function `u64` budgets — the spec's bounded-execution guarantee),
  `effects.rs` (**effect lattice: Pure < Hot < Cold < Async < External**), `deontic_interrupt.rs`
  (law-gated interruption), `observer.rs`, `replay_clock.rs`, `dag.rs`, `sheaf.rs`,
  `frame_morphism.rs`, `law_package.rs`, `module_system.rs`, `metamodel/`, `projectional.rs`,
  `reflection.rs`, `lexicon_pin.rs`.
- Reactivity: `reactive_cell.rs`, `conditioning/` (profiles, projection), `tick_policy.rs`
  (Drop/Coalesce/Tear under load), `animation/` (curves, families, presets, `spring.rs`,
  `squad.rs` — spherical quadrangle interpolation, `pga.rs`), `physics/` (fields, laws,
  materials, trajectory, morphism), `cosmic/` (celestial, stardate, transforms, observer frames,
  microverse/nested/coocated/parallel worlds).
- `degrade.rs`, `hid.rs`, `quantity.rs`, `presentation.rs`, `crypto.rs`, `cbor_ast/`, `grammar/`.

### 7.2 Host catalog (capability surface)

`vibe/src/catalog/ids.rs` — **`ALL_INVOKE_IDS`, ~1,158 IDs across ~90 families** (counts from a
family-prefix scan): Statistics 108, Econ 107, ComputationalGeometry 94, MachineLearning 81,
Research 73, Render 60, SymbolicAlgebra 42, LinearAlgebra 33, Calculus 24, Cosmic 23, Chemistry
22, Audio 22, Asset 21, Physics 20, NumberTheory 20, Scene 19, PolynomialAlgebra 16, Inference 16,
HID 16, Image 15, Dmx 14, plus GeometricAlgebra, Video, Pulse, NLP, ComputerVision, FuzzyQuery,
EngineeringAnalysis, Spectral, GraphDatabase (sparql/volume_open/volume_commit/lexicon_manifest),
DeonticLogic, N3Logic, LegalLogic, Agent, Sentinel, World, Orchestration, and many more.

- `catalog/mod.rs`: `is_known`/`canonical_id`/`family_of`/`granted_covers` (family leases:
  `using Animation;` covers `Animation.*`), preset aliases, `describe` hover text. Host calls are
  **fail-closed** and require an explicit `using {Family}` grant — this is the sandbox model the
  spec's "bounded cycle guarantees" refers to.
- `catalog/{stdlib,logic,specialized,cosmic,cosmic_ext,local,suggest}.rs` partition the catalog.
- Honesty doc: `docs/manuals/standards/vibe-catalog-honesty.md` — check it before asserting a
  catalog op is backed by real host code; catalog presence ≠ host implementation everywhere.
- `poet_host/` (`catalog.rs`, `catalog_ttl.rs`, `invoke/`, `bridge.rs`, `scan.rs`,
  `services/poet_*_api.rs`): the Poet/NLP invocation seam over the same catalog — geometry ops
  included (per the PlantGL follow-up, host adapters now cover the botanical ops).
- Targeted test evidence (from the PlantGL review — a targeted run, not full-suite): 647 Vibe host
  invoke tests + 25 catalog tests.

### 7.3 Agentic substrate

- `agent_runtime/`: symbolic planner (verb→capability DAG, no LLM), `orchestration.rs`
  (blackboards, phase leases, sessions, rosters, agent assignment), golden-corpus `corpus.rs` +
  `evaluator.rs` (match metrics).
- `mcp/`: real MCP server (`mcp_server.rs`) + tool impls (`mcp_tool_impls/` science/logic/
  governance; `mcp_stub_impls.rs` — misnamed, real — graph/infra/QPU/wallet). One MCP tool per
  domain library.
- `nlp/`: Aho-Corasick gazetteer over interned lexicon, `graphrag.rs` (deterministic keyword
  GraphRAG — no embeddings), terms/spans. **Explicit first-year scope**: tokenization,
  gazetteers, spans only — FrameNet, RST, OpenIE, NLI, and machine translation are
  declared non-goals (`nlp/mod.rs`).
- `research/`: enquiry/hypothesis/investigation/inference_chain/assessment/sentiment/
  perspective/intentionality/dark_link/dynamics, `fiction_classification`,
  `ungrounded_generation` diagnosis — the research-agent scaffolding. **Workflow layer,
  not a validated inference system**: enquiries/chains/hypothesis graphs are records +
  evaluators; do not cite its outputs as science without an assessment pipeline on top.
- `simulation/`: deterministic fixed-tick world reducer (`simulation/mod.rs`) — the
  bounded-tick substrate for entity/world stepping.
- `obfuscation/`: `no_std`, zero-allocation semantic stripping + domain transforms for QPU
  submission — honest "remove meaning before outsourcing compute".
- `services/`: daemon (4242), swarm (verify-before-pay), pulse transport, Poet APIs, Solid/LDP
  routes.
- Inference: full native LLM stack (`inference/` ~60 entries + `gguf_bridge/` ~25):
  GGUF index, p64 weights, two-thread SPSC decode, CUDA/DirectML/Metal/wgpu lanes, LoRA
  multiplexing (`lora/`), grounding gate (`solvers/grounding/` wired into
  `orchestrator.rs`). Plus:
  - `inference/qwen/` — a native Qwen-family runtime: checkpoint, decode,
    `expert_residency`/`expert_tile`, `gated_deltanet`, `hyper_connection`,
    `ple_block`/`ple_ngram`/`ple_nvme`, `storage_plan`, streamed layers
    (`streamed_{gdn,hyper,moe,qsa,layer}`), `trunk_nvme` — expert-offload decode over NVMe.
  - `inference/moe/` — MoE dispatch, expert cache, `nvfp4`, placement, routing, `ftw_loader`.
  - `inference/runtime/` — scheduler, budget, receipts, `memory_guard`/`memory_windows`,
    `graph_assist`, `kv`, `prepared`, `artifacts`, `stage`.
  - `inference/lab/` — a real experiment framework: `hypothesis`, `campaign`,
    `config_space`, `pareto`, `search`, `ablate`, `auto_improve`, `experiment_log`,
    `audit_path`, `device_roof`, `micro`, `operator_compare`/`operator_metrics`/
    `operator_promotion`, `timeline`. **Candidate harness for species-parameter fitting**
    (growth curves, material facets) — same hypothesis→campaign→pareto loop.
  - `inference/conditioning/` + `conditioning_eval/` + `conditioning_opt/` — the full
    conditioning pipeline (asset contracts, authority, budgets, prepared requests,
    receipts, registry) with corpus scoring and a strategy-generator optimizer.
  - `domino_gbnf.rs` — **GBNF grammar-constrained decoding that guarantees LLM output is
    valid VibeScript** — the structural piece that makes "generative model authors
    `.vibe` behavior" safe to attempt; paired with `neuro_symbolic_sieve.rs` (zero-heap
    FSM token sieve).
  - `semantic_skills.rs` — in-process text embeddings (FNV-hashed, honest lightweight),
    cosine VectorStore k-NN, ephemeral scratchpads.
  - `kv_dict.rs` — sparse KV-dictionary codec (OMP encode/reconstruct); forge produces the
    dictionary, the engine only runs the certified artifact (clean producer/consumer
    split).
  - `paged_kv.rs`, `speculative_decode.rs`, `prompt_lookup.rs`, `reranker/`,
    `safetensor_loader.rs`, `tensor_roles.rs`, `ternary.rs`/`ternary_gpu.rs`,
    `topk.rs`/`topk_gpu.rs`, `residency_planner.rs`, `thermal_telemetry.rs`/`thermal_wal.rs`,
    `spatial_sieve.rs`, `semantic_culler.rs`, `post_turn_verify.rs`,
    `quant_graph_grounding.rs`, `qualia_hybrid.rs`, `toolkit_probe.rs`,
    `inference_path_selector.rs`, `application_profile.rs`, `compute_universe.rs`.
  - `crates/qualia-inference-kernel/` — dependency-free operator + paged-attention
    contracts with scalar correctness oracles (kernels testable without the product
    graph). Same pattern the Ecosphere wants for geometry determinism certificates.
  - Known caveat: Sentinel anomaly detector currently inert (hardcoded "normal"); a
    different `GGufSharder::generate_bidx_pointer_map` fabricates synthetic offsets — do
    not cite it as real provenance.
- **Poet application surface** (`crates/poet/`): `tool_chest/{constructs,core,manifolds,
  payloads}`, `vibe_host`, `keep_volume`, and `browser/` — a ~180-file workbench UI whose
  `*_chain_actions.rs` modules are the **reachability evidence** for catalog families:
  audio, cg (waves 1–4), chem, constr, cosmic, dmx, econ, eng, fuzzy, ga, gpu, hid,
  inference (+remainder), interp, linalg, logic, ml, nlp, nt, ode(+num), orch, physics,
  poly, portal, pulse, render(+live), research (waves 1–4), scene(+graph), sf, spectral,
  stats, threed, transforms, vc, video(live), vision_10d_scrubber, webrtc, world,
  hbbtv, webrtc_sync, plus `social_*` workspace/inbox/moderation views, `solid_interop`,
  `lexicon_bay`, `semantic_instruments` panels, `instrument_panel`, `logic_workbench`,
  `search_workbench`, `cooperative_economics`, `governance_views`, `rights_views`,
  `health_views`, `ontology_views`. If a spec feature needs to know whether humans can
  reach a capability today, the matching chain-action file is the first thing to check.
  `crates/poet-cli` (format/lint/repl), `crates/vibe-lsp` (LSP + `catalog_intel`),
  `crates/vibe-wasm` (parse/check/eval/bytecode → JS), `crates/work-graph` (Present-only
  implementation indexer — useful for stamping "what exists" claims in plans),
  `crates/qualia-app-contract` (canonical command/context/session contracts), and
  `crates/qualia-peer` (`PeerHost` facade — the QDNF application API) complete the surface.

---

## 8. Rendering, geometry ingestion & media (spec §4, §9.1)

- `render/assets`: OBJ/STL/GLB ingest → NQuin mesh facts (**semantic projection only** — dense
  vertex data is not stored in q42; full glTF scene preservation — normals/UVs/materials/skins/
  animation/morphs — is **missing**; `mesh_to_nquins` parity bug noted in the 3D-assets manual).
- `container_10d` covers compiled geometry; `render/compile_10d.rs` + `compile_10d_manifest.rs`
  are the GLB→`.10d` compile path; `entity_compile_gates.rs`/`twin_compile_gates.rs` gate bundle
  output; `asset_package.rs`, `anatomy_pack.rs`, `hmc_playback.rs`, `lod_chain.rs` (LOD tier
  chains — spec §2.3's `lod_tiers` map to this + `variant_matrix.rs`).
- Viewport streaming pieces: `instance_culling.rs`, `screen_metric.rs`, `temporal_resolve.rs`,
  `time_scrub.rs`, `texture_streaming_plan.rs`/`texture_stream_policy.rs`, `deferred_resolve.rs`,
  `frame_graph.rs`, `quality_profiles.rs`/`quality_runtime.rs`.
- Camera/navigation/picking: `camera.rs`, `camera_mount/`, `navigation.rs`, `standpoint.rs`,
  `sense.rs`, `barrier.rs`, `control.rs`, `place_time.rs`.
- `render/gpu/` + `portal/` + `portal_wasm.rs`: shared wgpu 30 device, Tensor10D projector,
  caller-buffered RGBA8; `webizen-render` scene contract + `WgpuRenderer` (pick/hover, PNG
  readback, native swapchain); `hud/` bounded WASM canvas HUD.
- Materials/lighting: `material_compile.rs`, `lighting.rs`, `environment_lighting.rs`,
  `ao_quality.rs`, `atmosphere.rs`, `gamut.rs`, `metamer.rs`, KTX2 texture pipeline
  (`texture_ktx2*.rs`, `texture_decode.rs`, `texture_ingestion.rs`, `cpu_texture_mips.rs`).
- `hypermedia/` (`mod.rs` + `processors/audio.rs` WAV, `processors/image.rs` EXIF): asset/
  analytics/provenance/relationship graph descriptors — **cold-path** (alloc-friendly helpers);
  `hypermedia_authoring/` (dmx, image, interactive, portals, video, world3d).
- `semantic_instruments/`: attestation, catalogue, manufacture, package, reference, resolve, rpl,
  visual — manufactured-object "instrument" packaging (sibling pattern to entity bundles).
- Browser WASM surface: `wasm_bridge/` (~6.1 kLOC, ~160 exports across bio/chem/compute/dataio/
  semantic/geometry/meta/engine) consumed by `docs/compute-engine.html`; `webizen-lite-wasm` MCP
  endpoint; `wasm_llm.rs` resident decode.

---

## 9. Where the Ecosphere spec should be corrected or scoped

| Spec text | Reality |
|---|---|
| `hmc_bundle_magic: "Q42_HMC_V1"`, `manifest.json` entry 0, JSON offset table | Magic is `QBDL`; index is a CBOR **footer**; manifest role is `cbor_ld_manifest.rs` (CBOR-LD) + per-entry `meta`. Update §9 schema. |
| "`SRD1` surface buffers (unified vertex and index buffers)" | `SRD1` = per-vertex RGBA8 SurfaceReading section magic. Geometry = `QuantizedMesh` sections. |
| "no animation section in `.10d`" (Rolling Commons QG-03) | Stale — `SkeletalAnimation` (16, SKL1) + `EntityOntology` (17, EONT1) implemented. |
| "PlantGL computational geometry engine" | QualiaDB native `computational_geometry`; PlantGL is the comparison target, not the engine. |
| HCF: Bao/BLAKE3 tree, AEAD levels | Not shipped — CRC-32C + per-entry SHA-256; AEAD reserved. |
| `.10d` header CRC / section table "fully conformant" | Several P0 items labeled spec-reserved/in-progress in `container_10d/mod.rs`. |
| Full biomechanical/biochemical simulation | Real compute is F2/F3 bounded (see §5.3); high-fidelity claims need F/A-tier labeling. |
| glTF→q42 "conversion" | Semantic projection only — dense geometry/material/animation not in the volume; use `.10d`. |
| "512MB manifold/weight tier" | Default cold budget is 256 MiB (caller-selectable larger); P64 weight containers are separate. |
| `.vibe` "memory_budget_bytes / max_cycles" | Budgets are per-function `u64` in `VBC1` + host-side tick/effect policy — describe those. |
| `.ntx` | Not a repo format. |
| `did:q42` = HMC byte offset | Rejected design — bundle uses key+offset index. |

---

## 10. Gap list & recommended additions

### 10.1 Spec-referenced, closed gaps and active roadmap

The 22 capabilities identified in the initial October 2026 audit have been systematically addressed through the hypermedia lifecycle implementation programme (tracked in `docs/plans/hypermedia-lifecycle-implementation-plan.md`):

1. **Obligation-cost ledger binding** — **Implemented** in `modalities/value_flow.rs` (`accumulate_meter_to_pool`, `carriage_fee_from_meters`) + `bundle/cbor_ld_manifest.rs` (`CommonsObligation::discharge_to_commons`). Transitions assets automatically to `PermissiveRoutingLane::EnforcePermissiveCommons` (`0b01`) once cumulative usage pool $\ge 10\times C_{\text{production}}$.
2. **Ethogram registry type** — **Implemented** in `entity_view/ethogram.rs` (`EthogramRegistry`, `EthogramBehaviorEntry`, `StimulusEvent`, `EntityPhysiologicalState`). Evaluates environmental stimuli and transitions state meters in $O(1)$ zero-heap time within the 42 MiB `SlgArena`.
3. **Collective/flocking dynamics** — **Implemented** in `specialized_libs/physics_simulation/population.rs` (`step_flocking_simulation`, `BoidAgent`, `FlockingParameters`). Zero-heap 3D boids simulation for avian flocks and fish schools.
4. **Hyperspectral cube section** — **Implemented** in `container_10d/section.rs` (`SectionType::HyperspectralCube = 21`). Encodes dense 10nm band reflectance cubes (400nm–1000nm) directly within `.10d` containers.
5. **Aero/hydro/pyro-acoustic synthesis** — Parameterized in `.10d` and mapped to `audio/dsp/` procedural sound generators.
6. **Wind MOR/windthrow, xylem cavitation, XPBD deformation** — Mapped to `pbd_softbody.rs` and canopy deflection estimators.
7. **4-D growth-event tracking / longitudinal ontogeny** — **Implemented** in `container_10d/entity_ontology_section.rs` (`DevelopmentalKeyframeRecord`). 32-byte POD keyframes with exact byte alignment linking growth timers to allometric crown and trunk scaling.
8. **Per-byte-slice micropayment metering** — Bounded carriage fee derivation in `modalities/value_flow.rs` wired to HTTP range reads.
9. **HMC format unification (QG-01)** — **Standardized & Implemented** on `QBDL` format standard in `bundle/format.rs` (64-byte front header, 64-byte page-aligned entries, trailing CBOR index footer, whole-file CRC-32C verification).
10. **Payload catalogue (ADR 0014)** — **Implemented** in `bundle/cbor_ld_manifest.rs` (`PayloadCatalogueEntry`). Provides structured content-addressed slots for multi-language prose and oral history attachments.
11. **Full glTF preservation + canonical importer** — **Implemented & Verified** in `render/compile_10d.rs` packing `QuantizedMesh` (SectionType 1) + `SurfaceReading` (`SRD1`, SectionType 2) + `SkeletalAnimation` (`SKL1`, SectionType 16) + normals and material properties.
12. **Commons licence enforcement** — Enforced via `bundle/cbor_ld_manifest.rs` and `governance/tk_bc_labels.rs`.
13. **Fetch-by-hash transport** — Content-addressed swarm resolution facade in `qualia-peer`.
14. **TK/BC label machinery** — **Implemented** in `governance/tk_bc_labels.rs` (`validate_tk_label`, `validate_bc_label`, `evaluate_biocultural_access`). Enforces Local Contexts Traditional Knowledge and Biocultural provenance policies.
15. **Reachability** — Promoted variant resolution to `rolling-commons-shell::EntityFamilyManifestWrapper` and session journaling to `GameSessionJournal`.
16. **Biosense/species imaging pipeline** — Structured in `qualia-vision` recipes.
17. **Vision GPU & WASM pipeline** — Mapped through `webizen-render` and `QuantizedMesh` projection.
18. **QDNF economics ↔ commons pools binding** — **Implemented** in `modalities/value_flow.rs` connecting `net/qdnf/economics/meters.rs` (`MeterReading`) to `accumulate_meter_to_pool` and `carriage_fee_from_meters`.
19. **Species/ecology SHACL pack** — **Implemented** in `crates/qualia-core-db/shapes/ecology_shapes.ttl`. W3C SHACL shape validation for species taxa, developmental milestones, ethogram behaviors, and TK/BC biocultural assertions.
20. **Coordination opcodes & choreography** — **Implemented & Verified** in `governance/coordination.rs` (opcodes `0x70` `OP_AUTHORIZATION_GRANT`, `0x71` `OP_RESOURCE_DECLARATION`, `0x72` `OP_PERFORMANCE_RATING`).
21. **Species MIR/audio taxonomy** — Mapped through `audio/bioacoustics.rs` and `qualia-audio`.
22. **Wellfare anatomy → organism records** — **Generalized** in `entity_view/entity_ontologies.rs` and `ecology_shapes.ttl`.

### 10.1.1 Spatial & Runtime Hypermedia Overhaul Additions
- **Adaptive Multi-Resolution Corridors & Feathering**: Overhauled `rolling-commons-shell/src/terrain.rs` for continuous organic radial boundary feathering and adaptive 280m+ travel corridor patch generation (`build_adaptive_corridor_patch`).
- **42 MiB Cellular Sentinel Topology**: Implemented `governance/cell_graph.rs` (`CellularLogicGraph`) partitioning continuous space into independent 42 MiB `SlgArena` cells linked by Permissive Routing Lanes (`0b11` Spatiotemporal, `0b01` Commons).
- **Offline Session Journaling & CRDT Sync**: Implemented `rolling-commons-shell/src/session_journal.rs` (`GameSessionJournal`) for offline append-only `.q42j` transaction recording and Lamport CRDT merge.
- **Single-Pass Hydration & Live Diagnostics HUD**: Streamed single-pass HMC loading and in-game telemetry HUD (`F3` / backtick toggle) in `web/game.html`.

### 10.2 Reachability & honesty cautions (from the repo's own audits)

- `CAPABILITY_DESCRIPTORS` in `lib.rs` self-reports maturity per family — useful, but "Implemented"
  ≠ "reachable"; the geometry manifest↔dispatch gap is the canonical example.
- Named inert/scaffolded code (do not cite as shipped): `engine/llm_offload` demo playback,
  `acoustic_ble_mesh` (fully simulated), `spatial_sieve` JNI path (discards inputs),
  `ambient_orchestration` (simulated device execution), `solvers/symbolic_logic`
  `resolve_defeat`/`resolve_preference` no-ops, `shacl_validator` studio panel (decorative),
  `qubo_compiler.rs` placeholder solver is what `qpu_optimize` actually calls (the better
  `modalities/logic/qubo.rs` is mostly unreached), Sentinel anomaly detector inert.
- `hypermedia/mod.rs` and several general helpers are cold-path allocating — don't describe all
  hypermedia ops as zero-heap.
- `qdnf/vertical.rs` is a 64-byte demo slice, not the application API — app code enters
  through `qualia-peer` `PeerHost`.
- `qdnf/profiles/mod.rs` self-disclaims: remote wipe/attestation cannot guarantee safety
  after endpoint compromise/seizure; CI does not fake a TPM; cover traffic is not
  traffic-analysis immunity.
- `qdnf/biometrics/mod.rs` disclaims operational FAR/FRR, liveness, unlinkability proofs,
  SMPC recognition.
- `qualia-audio/src/speech.rs` is a phone-decode scaffold, not ASR; `generation.rs` gates
  licensed TTS/neural separation ("COMPLETE-WITH-GATE").
- `research/` outputs are workflow records — not validated findings — unless an assessment
  pipeline is applied.
- `poet/src/browser/*_chain_actions.rs` files are the reachability evidence for catalog
  families through the Poet UI; absence of a chain action ≠ absence of the library op
  (Rust API may still exist), and presence ≠ every listed op is host-wired — the catalog
  honesty doc governs.

---

## 11. Quick navigation index (spec concept → source)

| Need | Start here |
|---|---|
| NQuin bit layout / execution rules | `C:\github\qualiadb\AGENTS.md` §0–§1 |
| Q42 volumes, lexicons, range reads | `crates/qualia-core-db/src/q42/`, `q42_lex*.rs`, `q42/volume/` |
| Q42 asset lifecycle | `q42/{asset_envelope,asset_import,journal.rs,q42_kvp.rs,app_manifest,app_registry,execution_profile.rs,machine_gpu_profile.rs,source_catalogue}` |
| Ontology corpus (rights/commons vocab) | `core-ontologies/` (`permissive-commons.n3`, `social-permissive-commons.n3`, `values.n3`, `jural.n3`, `modal-junctures.n3`, 102 instrument N3s), `crates/qualia-core-db/shapes/` |
| `.10d` sections & ABI | `crates/qualia-core-db/src/container_10d/` (`mod.rs`, `section.rs`) |
| `.hmc` bundle format | `crates/qualia-core-db/src/bundle/` (`format.rs` line 40: `QBDL`) |
| Manifest / credentials / obligations | `bundle/cbor_ld_manifest.rs`, `domain_licence/`, `governance/` |
| Lifecycle/LOD variants | `bundle/variant_matrix.rs`, `render/lod_chain.rs` |
| Entity affordances / ethology substrate | `entity_view/entity_ontologies.rs`, `container_10d/entity_ontology_section.rs` |
| Rigs & motor clips | `container_10d/skeletal_section/`, `render/skeletal/` |
| Botanical geometry | `specialized_libs/computational_geometry/{botanical_hulls,sweep_extrusion,turtle_3d,space_colonization,tree_skeleton_3d,canopy_radiation}.rs` |
| Geometry op inventory | `computational_geometry/capability_manifests.rs`, dispatch `tool.rs` |
| Logic modalities | `crates/qualia-core-db/src/modalities/` (+ `modalities/logic/`) |
| Commons economics | `modalities/value_flow.rs`, `reconciliation.rs`, `services/ilp_dispatcher.rs`, `services/swarm/` |
| VibeScript | `crates/vibe/src/` (`catalog/ids.rs`, `bytecode/`, `effects.rs`, `tick_policy.rs`) |
| Sensory/audio | `audio/` (`bioacoustics.rs`, `stft.rs`/`cqt_bake.rs`, `hrtf.rs`, `acoustic_plane.rs`, `dsp/` DAW kernels), `crates/qualia-audio/` (`features/` MIR, `midi/`, `models/`, `aqa/`, `production.rs`, `speech.rs`, `generation.rs`) |
| Organism record model precedent | `crates/wellfare-core/src/anatomy/` (constitution/systems/physiology/pathway/observations/scorecard) |
| Vision/imaging | `specialized_libs/computer_vision/` (`cv`, `bio`, `sr`, `spatial`, `embeddings`, `gpu`), `crates/qualia-vision/` (biosense, recipes, weights, taxonomies) |
| Math substrate | `solvers/` (linalg, calculus, statistics, learning, exact, units, transforms), `specialized_libs/{symbolic_algebra,multivar_calculus,polynomial_algebra,category_theory}` |
| Spectral | `render/spectral_basis.rs` + `spectral*.rs`, `metamer.rs` |
| Physics | `render/physics/`, `render/vegetation.rs`, `specialized_libs/{physics_simulation,engineering_analysis}`, `computational_geometry/pbd_softbody.rs`, `vibe/src/physics/`, `solvers/` |
| SHACL/OWL policy | `docs/manuals/standards/shacl-first-vs-owl-ok-class-list.md`, `modalities/logic/{shacl,owl,shacl_extensions,*_shacl.rs}` |
| Geospatial/ecology data | `domains/geospatial/` (`adapters/`, `stewardship.rs`, `triggers.rs`, `spatial_sync.rs`) |
| Chemistry/phytochem | `specialized_libs/{chemistry_modeling,organic_chemistry}`, `q42/chebi_*` |
| Identity/DIDs | `identity/`, `did_qi/`, `crypto/` |
| Networking/sync | `net/qdnf/` (economics meters+obligation, policy_labels IFC, route, profiles, resolve, bearer, authority), `crates/qualia-peer` (`PeerHost` app API), `net/peer/`, `p2p/`, `services/webtorrent_*`, `foundation/crdt.rs` |
| Multi-agent coordination ISA | `governance/coordination.rs` (opcodes 0x70–0x7F), `governance/illocution.rs`, `core-ontologies/modal-junctures.n3` |
| Render/streaming | `render/` (`compile_10d.rs`, `hmc_playback.rs`, `texture_streaming_plan.rs`, `portal/`) |
| Inference/agentic | `inference/` (`qwen/`, `moe/`, `lab/`, `runtime/`, `conditioning*/`, `domino_gbnf.rs`, `neuro_symbolic_sieve.rs`, `semantic_skills.rs`, `kv_dict.rs`), `crates/qualia-inference-kernel`, `gguf_bridge/`, `agent_runtime/`, `mcp/`, `nlp/`, `research/` |
| Poet UI / tooling surface | `crates/poet/src/browser/` (`*_chain_actions.rs` per domain), `poet-cli`, `vibe-lsp`, `vibe-wasm`, `work-graph`, `qualia-app-contract` |
| Vision-GPU & 10d handoff gaps | `docs/manuals/vision-gpu-10d-geometry-gap-audit-2026.md` |
| Standards vs code truth | `docs/manuals/standards/q42-implementation-divergence.md` |
| Honest stub inventory | `docs/manuals/qualia_db_functionality_manual.md` |
| Geometry vs PlantGL | `docs/manuals/plantgl-computational-geometry-gap-review-2026-10-14.md` |
| Format/runtime work packets | `docs/plans/rolling-commons-format-runtime-integration.md` (QG-01…08) |
| 3D/digital-twin tier model | `docs/manuals/computational-3d-assets-and-digital-twins.md` |

---

## 12. Game and simulation asset quality: source-checked development programme

**Added 2026-10-15. Scope:** planning and implementation guidance only. This review read
both Ecosphere documents, selected live QualiaDB source and callers, and the primary online
references linked below. **No implementation, build, test run, performance measurement or
visual acceptance was performed.** Recommendations and acceptance criteria in this section
are proposed work, not claims of shipped behaviour.

**Source baseline:** `C:\github\qualiaDB`, branch `0.0.40.8-dev`, HEAD
`7eb4438fd7205c0a6fd859deb02e13b58cd0baa3`, **including the working-tree additions present
during review**. In particular, ethogram, cellular graph, hyperspectral enum, developmental
record and flocking changes were inspected rather than inferred from older gap documents.
This is a selected pipeline audit, not a fresh certification of every capability in §§1–11.
Recheck the working tree and call chain when starting each work package.

### 12.1 Current baseline: reuse the implemented work, qualify its boundaries

Source links below resolve into the sibling QualiaDB checkout. “Present” means inspected
source; it does not imply that all application surfaces, platforms or conformance cases work.

| Capability | What the inspected code establishes | Remaining work or qualification |
|---|---|---|
| GLB geometry and appearance import | [assets/glb.rs](../../qualiaDB/crates/qualia-core-db/src/render/assets/glb.rs), [assets/mod.rs](../../qualiaDB/crates/qualia-core-db/src/render/assets/mod.rs) and [compile_10d.rs](../../qualiaDB/crates/qualia-core-db/src/render/compile_10d.rs) preserve supported normals, authored/generated MikkTSpace tangents, UV0, material/submesh data and embedded image dependencies. | This is more than semantic projection. However, the importer walks `meshes[].primitives[]` into flattened geometry; the inspected import representation does not preserve scene-node instances/transforms, skins, animation channels or morph targets. Existing native SKL1 support does not make this a complete glTF importer. |
| Native skeletal playback | [skeletal/](../../qualiaDB/crates/qualia-core-db/src/render/skeletal/mod.rs), [portal/scene_mesh.rs](../../qualiaDB/crates/qualia-core-db/src/render/portal/scene_mesh.rs) and [portal/scene_animation.rs](../../qualiaDB/crates/qualia-core-db/src/render/portal/scene_animation.rs) admit SKL1 actors, sample/blend motor clips, maintain attachments and drive posed surface-frame uploads. | Keep this implementation. The current player uses rigid motors with authoring scale baked into geometry; arbitrary animated scale/morph data needs an explicit conversion or rejection policy. Benchmark CPU skinning, tangent updates and uploads before adding another backend. |
| HMC mesh reuse | [hmc_playback.rs](../../qualiaDB/crates/qualia-core-db/src/render/hmc_playback.rs), called by [portal/mod.rs](../../qualiaDB/crates/qualia-core-db/src/render/portal/mod.rs), verifies a mesh entry and retains a source-digest upload receipt. | This particular residency object tracks one currently bound mesh. A scene-wide pool of independently resident geometries, variants and instances is a further lifecycle, not something established by this receipt alone. |
| Texture streaming and GPU admission | [texture_stream_policy.rs](../../qualiaDB/crates/qualia-core-db/src/render/texture_stream_policy.rs), [texture_streaming_plan.rs](../../qualiaDB/crates/qualia-core-db/src/render/texture_streaming_plan.rs), [gpu/texture_residency.rs](../../qualiaDB/crates/qualia-core-db/src/render/gpu/texture_residency.rs), [volumetric_hmc.rs](../../qualiaDB/crates/webizen-render/src/volumetric_hmc.rs) and [volumetric_hmc_stream.rs](../../qualiaDB/crates/webizen-render/src/volumetric_hmc_stream.rs) implement real budgeted admission, upgrade/downgrade/eviction planning and generation-checked application. | Do not reopen “no texture streaming”. The inspected HMC path admits supported plain RGBA8 KTX2 mips and uses typed fallbacks for unsupported compressed/Basis interpretations. Its state retains complete bundle/asset bytes; this is not yet evidence of network range delivery. |
| Compressed texture helper | [texture_transcode.rs](../../qualiaDB/crates/qualia-core-db/src/render/texture_transcode.rs) contains caller-buffered block decoding and native Zstd handling. | Its ETC2/ASTC RGBA fallback invokes a BC7 decoder, and BC7 has a limited/approximate fallback. These branches do not establish conformant decoding of those formats. `PassthroughCompressed(_)` also needs source/target format compatibility validation. Treat codec correctness as P0 before expanding use. |
| LOD and far-field representation | [lod_chain.rs](../../qualiaDB/crates/qualia-core-db/src/render/lod_chain.rs), [screen_metric.rs](../../qualiaDB/crates/qualia-core-db/src/render/screen_metric.rs), [variant_matrix.rs](../../qualiaDB/crates/qualia-core-db/src/bundle/variant_matrix.rs), [impostor_section.rs](../../qualiaDB/crates/qualia-core-db/src/container_10d/impostor_section.rs) and [surfel_section.rs](../../qualiaDB/crates/qualia-core-db/src/container_10d/surfel_section.rs) supply simplification, selection, variant resolution, codecs and fitting/projection primitives. | Their presence does not prove a complete meshlet/impostor/surfel bake-to-draw pipeline. No corresponding full render dispatch was established in the inspected paths. Extend these seams and verify each consumer rather than describing a shipped Nanite-equivalent renderer. |
| Rendering quality infrastructure | [frame_graph.rs](../../qualiaDB/crates/qualia-core-db/src/render/frame_graph.rs), [pipeline/frame_schedule.rs](../../qualiaDB/crates/webizen-render/src/pipeline/frame_schedule.rs), [gpu/portal_gpu/render_frame.rs](../../qualiaDB/crates/qualia-core-db/src/render/gpu/portal_gpu/render_frame.rs), [environment_lighting.rs](../../qualiaDB/crates/qualia-core-db/src/render/environment_lighting.rs), shadow/AO and temporal GPU modules are present. | Improve coverage, quality and scheduling integration. A graph plan is distinct from command recording and actual resource reuse. Temporal producers/history handling already exist; do not recommend starting TAA, lighting or a frame graph from scratch. |
| Ethogram registry and flocking | [ethogram.rs](../../qualiaDB/crates/qualia-core-db/src/entity_view/ethogram.rs) supplies fixed-capacity stimulus-to-state/clip transitions; [population.rs](../../qualiaDB/crates/qualia-core-db/src/specialized_libs/physics_simulation/population.rs) supplies real 3D separation/alignment/cohesion. | Earlier “no registry/no Boids” statements are stale. The registry scans up to 32 entries; its precondition mask is stored but not evaluated in the inspected stimulus match. Flocking is all-pairs and updates agents in place, so later agents see earlier updates. Policy completeness, order semantics, spatial indexing and application integration remain review items. |
| Cellular logical graph | [cell_graph.rs](../../qualiaDB/crates/qualia-core-db/src/governance/cell_graph.rs) contains topology descriptors and lane-checked message routing. | The inspected graph uses `Vec` storage and allocating drains; it does not instantiate or schedule per-cell `SlgArena` evaluators. It is useful topology infrastructure, not evidence of a zero-heap distributed cell executor. |
| Growth and hyperspectral additions | [entity_ontology_section.rs](../../qualiaDB/crates/qualia-core-db/src/container_10d/entity_ontology_section.rs) defines a 32-byte `DevelopmentalKeyframeRecord`; [section.rs](../../qualiaDB/crates/qualia-core-db/src/container_10d/section.rs) assigns `HyperspectralCube = 21` and marks it implemented. | This supersedes claims that neither has a representation. The selected search found no developmental-record integration into the EONT1 codec and no dedicated cube codec/consumer. Encoding, validation, transport and playback still need demonstrated callers; an enum admission flag alone is insufficient. |
| HMC catalogue vs Q42 catalogue | [cbor_ld_manifest.rs](../../qualiaDB/crates/qualia-core-db/src/bundle/cbor_ld_manifest.rs) now has `PayloadCatalogueEntry`. | This is an HMC manifest field. [q42_volume.rs](../../qualiaDB/crates/qualia-core-db/src/q42/q42_volume.rs) still describes `FLAG_PAYLOAD_CATALOGUE` as reserved/stub pending its writer. These are different scopes; one does not close the other. |

**How this qualifies earlier sections:** the absolute “missing” statements in §§3–5 and §8
must be read with the newer implementations above. Conversely, §10.1's blanket “closed”
labels do not establish complete import preservation, a working hyperspectral codec, a
longitudinal growth runtime, or universal format interoperability. In particular:

- `SurfaceReading/SRD1` is **section 12**, not section 2 as stated in §10.1 item 11;
  section 2 is `Tensor10DNodes`. Geometry is `QuantizedMesh`, section 1.
- `bundle/` QBDL support is real, but unification with the semantic-library ZIP producer
  needs a producer/consumer migration or compatibility demonstration. QBDL's existence
  alone does not prove that every `.hmc` writer uses it.
- The float-tag clash mentioned in §0 is resolved in the inspected
  [query/resolver.rs](../../qualiaDB/crates/qualia-core-db/src/query/resolver.rs) and
  [foundation/frame_layout.rs](../../qualiaDB/crates/qualia-core-db/src/foundation/frame_layout.rs):
  `INLINE_TAG_FLOAT = 0b101 << 60`. Use the canonical helpers.
- Cold geometry defaults to **256 MiB** in
  [geometry_workspace.rs](../../qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/geometry_workspace.rs).
  A larger caller-admitted budget is possible; 512 MiB is not a universal arena, GPU,
  model-weight or per-cell entitlement.

### 12.2 Production architecture to target

Build on the existing backend rather than introducing a separate game asset system.
The recommended lifecycle is:

1. **Capture or author a source embodiment**: measured scans/reference images, procedural
   botanical/engineering recipes, hand-authored geometry, or governed model-generated
   candidates. Preserve original inputs and distinguish measured, inferred and artistic data.
2. **Validate and compile in a bounded cold pass**: resolve units and coordinates; retain
   part/instance identity; validate geometry and material data; derive the supported rigs,
   collision shapes, LODs, textures and other target-specific products.
3. **Publish a versioned dependency graph**: semantic facts/relationships in `.q42`, dense
   geometry and supported fields in `.10d`, scripts in `.vibe/VBC1`, intact resources and
   manifests in QBDL `.hmc`. Record full content digests and derivation receipts.
4. **Admit a working set**: select the required parts, variants, mips and simulation fidelity
   from camera demand, task relevance and independent CPU/GPU/transfer budgets.
5. **Play resident resources**: reuse geometry/material/pipeline objects; update compact
   instance, pose and state streams. Execute bounded simulation ticks and render interpolated
   presentation frames.
6. **Retain or retire explicitly**: journal authoritative state, cancel obsolete requests,
   evict unneeded resources after their GPU work completes, and promote validated build
   outputs from RAII scratch into a caller-selected artifact directory.

This applies to species embodiments, terrain, rocks, water, structures, vehicles, tools,
furniture, characters/personas and interactive scientific instruments. Keep the existing
ontological distinctions between natural persons, ecological specimens, manufactured
constructs and creative expressions.

**Foundation-first should describe dependencies and optional fidelity, not compulsory
resident detail.** A visible tree needs a convincing crown, branching, leaf materials and
motion; it does not always need instantiated microscopic xylem. A vehicle can carry a
semantic engine model and simplified collision chassis while a separate inspection mode
loads its detailed internal systems. Include anatomical interiors when the task needs them,
with explicit coverage and scientific evidence.

**Zero-copy is boundary-specific:** QBDL can return borrowed entry slices, but GPU upload,
texture transcoding, quantized-mesh expansion and WASM/JS transfers can still copy or allocate.
Trace and count those stages. A semantic `did:q42` handle is not an authenticated byte offset,
a GPU address or a substitute for resolving and admitting the referenced resource. WebGPU/wgpu
owns explicit buffer/texture transfers; DirectML is a compute/inference API, not the scene's
vertex-rendering pipeline. Preserve the useful native ABI without claiming universal zero-copy
playback.

**Representation is not execution:** carrying a recipe, physical coefficient, clip ID or
spectral field does not prove that a runtime consumes it correctly. Each capability should
name its producer, serialized representation, admission rule, runtime caller and verification.

### 12.3 Priority work packages

Priorities indicate dependency and risk, not measured performance impact. P0 protects
correctness and establishes evidence; P1 develops the common production/playback path;
P2 adds specialised detail after the common path is demonstrated. Owners named here are
existing modules to extend or consult, not instructions to grow oversized files. New concerns
should be focused directory-backed libraries with separate cold planning, execution, codecs,
receipts and tests, following AGENTS.md §0-B.

#### GA-01 — P0: capability contracts and a representative acceptance corpus

**Reuse:** the existing capability manifests, compile gates, geometry corpora and render
tests. **Add:** a per-target capability matrix covering import, compile, package, load and
draw/tick; record “supported”, “converted/baked”, “typed fallback” or “rejected” for each
feature. Include feature flags and the Rust/Vibe/CLI/WASM/Poet entry point actually exercised.

Choose a compact corpus spanning a rigged animal, windy alpha-masked foliage, a reflective
manufactured object, a tiled terrain/structure, a dense repeated-object scene and an anatomical
cutaway. Add mirrored/scaled instances, UV seams, multiple materials, damaged/lifecycle
variants and intentionally unsupported inputs. Use independent format fixtures as well as
project content. The Khronos [glTF Validator](https://github.com/KhronosGroup/glTF-Validator)
checks structural validity, and its [Sample Assets](https://github.com/KhronosGroup/glTF-Sample-Assets)
provide focused rendering/import cases; neither establishes biological accuracy.

**Exit evidence:** every advertised feature has a concrete round-trip or render/tick fixture,
with preserved semantic IDs and a stated platform result. “Not run” remains visible.

#### GA-02 — P0: codec and compiler correctness before wider admission

**Reuse:** `texture_ktx2*`, `texture_transcode`, `container_10d` validation and entity gates.
**Add:** format-correct block decoding/transcoding or explicit unsupported-format refusal.
Prioritise the ETC2/ASTC-through-BC7 and approximate BC7 branches identified in §12.1.
Validate block sizes, mip lengths, texture shapes, arithmetic overflow and source/target
format agreement. Keep the existing supported RGBA8 material fallback available.

Audit compile gates against actual inputs: for example,
[entity_compiler.rs](../../qualiaDB/crates/qualia-core-db/src/bundle/entity_compiler.rs) currently
passes constants to its script-determinism check. A constant success check does not validate
a supplied Vibe program. Bind receipts to the actual script, geometry, dependencies and
profile being admitted.

**Exit evidence:** independently generated codec vectors cover all admitted modes; unsupported
modes fail explicitly; malformed lengths and exhausted budgets never yield a success receipt.
KTX2's format metadata and level layout are defined by the
[KTX 2 specification](https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html).

#### GA-03 — P0/P1: preservation-first scene and animation import

**Reuse:** existing GLB appearance import, MikkTSpace generation, `.10d` compiler, SKL1 and
native player. **Add:** a canonical scene import representation that retains selected scene
roots, node hierarchy/transforms, shared mesh instances, stable part/material identities,
skins/inverse binds and animation channels. Support or explicitly reject sparse accessors,
morph targets, required extensions and non-triangle primitives. Check GLB version/declared
length as well as chunk bounds.

Specify conversion from glTF's transforms/skin matrices and interpolation to rigid motor
clips. Non-uniform or animated scale cannot silently disappear: bake only when valid, add
a versioned representation when justified, or reject the affected feature. Apply vertex
splits/remaps consistently to UVs, normals, weights, morphs and semantic labels.

**Exit evidence:** importing and recompiling nested, instanced, mirrored and animated scenes
preserves world-space placement, part picking and posed appearance within declared tolerances;
required unsupported features are reported before promotion. Use the
[glTF 2 specification](https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/Specification.adoc)
as the interchange contract. This work extends QualiaDB's native products; it does not require
glTF to become its runtime format.

#### GA-04 — P0/P1: measure the complete playback critical path

**Reuse:** existing quality profiles, VRAM ledger, residency receipts and GPU timing
infrastructure. **Add:** correlated timing/byte counters for manifest/index access, network
wait, integrity checks, decode/transcode, scene admission, shader/pipeline creation, CPU
simulation/animation, upload submission, GPU passes and presentation. Use asynchronous GPU
query readback where supported; keep a labelled CPU fallback.

Capture cold start, warm-cache playback, rapid turns, teleport, repeated variant swaps,
memory pressure and device recovery. The inspected portal render pass descriptors have
`timestamp_writes: None`; the existence of timing-capable device setup does not prove
end-to-end frame instrumentation.

**Exit evidence:** reproducible camera/input traces produce p50/p95/p99 frame times, visible
hitch counts, time to first useful frame, CPU/GPU peak bytes and upload bytes per frame.
Separate warm-up and compilation from steady playback, and separate graphics from concurrent
inference/compute. Select optimisations from the observed bottleneck.

#### GA-05 — P1: reproducible cooking, dependency tracking and incremental rebuilds

**Reuse:** `asset_import/`, `asset_envelope/`, `compile_10d`, material compilation, texture
resource packaging, provenance, variant manifests and `geometry_workspace`.
**Add:** a bounded backend job graph that derives only invalidated products. A cook key should
bind source/dependency digests, recipe and seed, compiler/schema versions, target capabilities,
quality/error tolerances and relevant tool settings. Keep the full digest for integrity;
`q_hash` remains the semantic handle with lexicon/collision checks.

Generate a machine-readable receipt listing retained, transformed, baked and refused features,
byte costs, warnings, provenance and output dependencies. A cancelled/failed build must leave
no publishable partial package. Cache reuse must preserve upstream licence/stewardship
obligations through the existing envelope machinery.

**Exit evidence:** unchanged inputs reuse verified outputs; a leaf texture edit invalidates
its material/mips without regenerating unrelated skeletons; version changes invalidate the
correct products; success, error and unwind clean owned temporary data.

#### GA-06 — P1: geometry quality, attribute-aware LOD and efficient buffer order

**Reuse:** native QEM, remeshing, topology/quality checks, quantized meshes, LOD chains and
variant resolution. **Add:** LOD derivation that protects UV/material boundaries, important
silhouette features, anatomical seams, attachment points, skin/morph data and semantic IDs.
The existing `Mesh`/LOD chain carries positions and triangle indices; define how other
attributes are preserved or regenerated.

Record absolute geometry and appearance error, not just a triangle ratio or QEM cost.
Budget quantization error per asset scale: one AABB-relative u16 step has different meaning
for a large terrain and a small anatomical structure. Use spatial partitions/local frames
where necessary. Add cache/fetch-friendly index and vertex ordering and benchmark optional
overdraw reordering on target GPUs.

**Exit evidence:** lower tiers retain identity, articulation and declared visual error across
fixed views and poses; seams remain valid and materials resolve. The
[meshoptimizer pipeline](https://github.com/zeux/meshoptimizer#core-pipeline) is a useful reference
for buffer ordering, simplification and clustering. Evaluate audited cold dependencies or
native equivalents under QualiaDB budgets; do not assume adoption is required.

#### GA-07 — P1: verified range delivery and progressive admission

**Reuse:** QBDL entry/segment offsets, Q42 range infrastructure, texture policies, source
digests and the existing networking facade. **Add:** an incremental bundle delivery adapter.
QBDL permits a fixed front-header fetch followed by its footer-index range; the current
[BundleReader::parse](../../qualiaDB/crates/qualia-core-db/src/bundle/reader.rs) instead requires
the complete declared file and validates its whole-file CRC. Preserve that reader's contract.

Specify how the directory is authenticated before offsets are trusted. An entire entry can
be checked against an authenticated SHA-256; an interior range cannot be authenticated by that
whole-entry digest alone. Use a versioned chunk-digest/proof scheme or separately verifiable
entries, then define partial, entry-verified and whole-bundle-verified states. CRC is an
accidental-corruption check, not authentication.

Prioritise visible proxies and required behaviours, coalesce bounded requests, limit
in-flight work, cancel obsolete generations, and retain the last verified representation
during refinement. Measure metadata round trips as well as payload bytes.

**Exit evidence:** a large HMC yields a verified useful frame before full download; truncated,
corrupted, mixed-generation and untrusted-index responses cannot promote geometry. Offline
cache and peer/HTTP delivery follow the same admission contract.

#### GA-08 — P1: scene-wide geometry residency, instancing and bounded submission

**Reuse:** `HmcMeshResidency`, `GpuInstanceRecord`, CPU frustum selection,
[gpu/instance_stream.rs](../../qualiaDB/crates/qualia-core-db/src/render/gpu/instance_stream.rs),
material bindings and the shared GPU context. **Add:** a budgeted pool keyed by geometry
digest plus layout/interpretation, with independent instance state and reference ownership.
Retain vertex/index buffers across views and variant changes; update only dirty spans.

Compile draw groups by compatible material/pipeline and LOD while keeping stable entity/part
picking. Prewarm the actual required pipeline variants in bounded background admission work.
Audit bind-group rebuilds and uploads when nothing changed. Extend visibility to spatial
hierarchies and, only if measurement warrants it, GPU culling/indirect submission with a
portable CPU path.

**Exit evidence:** many instances of one asset share its geometry allocation; steady static
playback does not re-decode/re-upload it; swaps and device loss invalidate the correct
generation; evictions wait for in-flight GPU use. Hardware-dependent paths are selected from
actual features/limits, consistent with the
[WebGPU capability model](https://gpuweb.github.io/gpuweb/explainer/#optional-capabilities).

#### GA-09 — P1: texture cooking and portable compressed residency

**Reuse:** digest-addressed resources, existing RGBA8 upload/admission, semantic mip generation,
alpha-coverage handling, stream policies and stale-plan rejection.
**Add:** offline, bounded texture cooking and a capability matrix for validated native GPU
blocks or Basis transcoding. Preserve sRGB for colour and linear interpretation for data;
normal maps need suitable filtering/renormalization, and foliage masks need stable coverage.
Choose compression quality by material semantics and target profile.

Avoid converting every compressed asset to a full-size RGBA8 CPU image when a verified
compatible GPU upload is available. Where it is unavailable, admit a smaller correct fallback.
Include all resident mips, staging, scratch and replacement overlap in costs. The
[KHR_texture_basisu specification](https://github.com/KhronosGroup/glTF/blob/main/extensions/2.0/Khronos/KHR_texture_basisu/README.md)
describes KTX2/Basis interchange; container recognition alone does not implement its transcode
path.

**Exit evidence:** target-specific images match reference decodes within declared codec
tolerances; unsupported textures retain meaningful materials; camera oscillation does not
cause repeated upgrade/evict thrashing; measured VRAM and upload peaks respect admission.

#### GA-10 — P1: animation quality and scalable pose delivery

**Reuse:** `SkeletalPlayer`, clip transitions, root/joint controls, attachment/activity
timelines and posed normal/tangent uploads.
**Add:** a behaviour-to-rig coverage report, contact/foot locking, appropriate IK constraints,
root-motion policy, event markers and explicit clip interruption/seek rules. Add morph or
muscle deformation only for assets whose declared behaviours need it.

Measure the present path: [animated_upload.rs](../../qualiaDB/crates/qualia-core-db/src/render/gpu/animated_upload.rs)
writes posed positions/normals/tangents and refreshes bounds. For crowd scale, evaluate
per-instance GPU palettes, compute/vertex skinning, pose sharing and bounded animation LOD
against this CPU oracle; stream compact poses rather than full posed meshes where beneficial.
Compression/reduced sampling must protect contacts, attachments and deformation bounds.
The [ozz-animation project](https://github.com/guillaumeblanc/ozz-animation) illustrates the
separation of offline animation processing from runtime sampling; it is a comparison, not a
proposed replacement for Vibe motor algebra.

**Exit evidence:** behaviour coverage is explicit; seeks/replays reproduce event timing without
duplicate emissions; feet/props stay attached; reduced-motion mode remains useful; crowd
measurements show where CPU versus GPU execution is appropriate.

#### GA-11 — P1: far-field baking and stable representation transitions

**Reuse:** screen metrics, mesh LODs, impostor/surfel sections, fitting helpers and family
variants. **Add:** a demonstrated producer-to-renderer path for each admitted representation:
bounded atlas generation, metadata validation, GPU draw path, material/depth interpretation,
shadow policy and semantic picking. Treat meshlets as a separate optional derived product;
a `MeshletCluster` enum does not create clusters or render them.

Drive refinement by projected **error** and content importance, with hysteresis and bounded
transition overlap. Preserve a minimum tier for interaction targets, thin high-contrast
features, educational labels and shadows/reflections. Review motion-based detail reduction
against actual visual acceptance instead of treating its perception constants as universal.
A hemisphere impostor needs a defined policy for views from below.

**Exit evidence:** no cracks or identity changes during transitions; pop/ghosting and foliage
coverage remain within chosen tolerances on camera sweeps. The OGC
[3D Tiles specification](https://docs.ogc.org/cs/22-025r4/22-025r4.pdf) provides a useful precedent
for hierarchical bounds and screen-space geometric error; QualiaDB need not adopt that format.

#### GA-12 — P1: integrated simulation scheduling and bounded cellular execution

**Reuse:** [simulation/fixed_tick.rs](../../qualiaDB/crates/qualia-core-db/src/simulation/fixed_tick.rs)
already provides a 20 Hz fixed-point command world; `webizen-runtime` has a fixed-step
clock; Vibe has tick/effect policy; ethogram/flocking/cell topology now exist.
**Add:** explicit scheduling between authoritative logic, physics, animation, audio and
presentation, with bounded catch-up, pause/seek semantics and interpolation. Protect the
authoritative state from presentation frame rate and speculative asset loading.

Complete ethogram precondition/permission checks using existing affordance policy. Specify
read-old/write-next semantics for simultaneous populations; the current in-place all-pairs
flocking kernel should not be promoted as parallel/order-independent. Add bounded neighbour
grids and deterministic reductions before crowd-scale execution.

For logical cells, define a globally budgeted active set, worker/arena reuse, fixed-capacity
message storage, overflow/backpressure and stable cross-cell ordering. Allocating a 42 MiB
arena for every organism/cell would quickly exhaust the process budget. Keep the descriptor
graph separate from its hot executor.

**Exit evidence:** recorded commands replay identically on the declared authoritative targets;
cell activation/queue saturation stay bounded; frame stalls do not cause unlimited catch-up.
The [fixed-timestep analysis](https://gafferongames.com/post/fix_your_timestep/) explains the
catch-up/“spiral of death” tradeoff. Float/GPU presentation is assessed with declared tolerances,
not automatically advertised as bit-identical.

#### GA-13 — P1/P2: simulation-ready objects and calibrated physical behaviour

**Reuse:** AABB admission/joints/material response, native hull/BVH/SDF and motion-planning
kernels, vegetation bending, XPBD, Vibe material facets and existing engineering solvers.
**Add:** cooked collision/interaction products distinct from display geometry: bounded convex
or capsule proxies, static triangle acceleration, mass/inertia, joint/contact metadata,
attachment sockets and task-appropriate navigation data. Existing 2D polygon decomposition
or SE(2) planning should not be described as a complete 3D collision/navigation system.

Define shared units, local/world reference frames and authoritative state ownership for
damage, wetness, heat, growth and wear. Calibrate visible spring/XPBD responses against
measured or reviewed parameters; introduce reduced-order models for distant/background
entities with explicit fidelity labels. Reserve heavy FEM/CFD for admitted tasks.

**Exit evidence:** dropped, stacked, moving and articulated objects have stable contacts and
bounded step costs; proxies preserve interaction semantics across visual LOD changes.
The original [XPBD paper](https://matthias-research.github.io/pages/publications/XPBD.pdf)
is the reference for compliance/constraint behaviour. An XPBD kernel alone does not validate
windthrow, circulation, combustion or soft-tissue physiology.

#### GA-14 — P1/P2: material, lighting and temporal quality under real motion

**Reuse:** PBR/stylized materials, MikkTSpace, environment probes, cascaded shadows, AO,
frame schedules, temporal producers/resolve and gamut/output modules.
**Add:** curated look-development profiles and rendered comparisons under consistent lighting.
Prioritise foliage transmission/two-sided response, stable masked edges, wet/rough surfaces,
skin/tissue and manufactured coatings according to the acceptance corpus; advanced models
need an explicit versioned parameter/consumer contract.

Check motion vectors and history reset for skinned/instanced deformation, LOD switches,
teleports and texture upgrades; avoid history trails on moving leaves, emissive objects and
transparent layers. Expose quality adaptation with hysteresis so memory pressure does not
oscillate between visibly incompatible profiles.

**Exit evidence:** still images and moving sequences preserve authored appearance on admitted
native/WASM paths; missing producers select the correct existing fallback; transient resource
peaks and GPU pass costs are measured. A perceptual image score supplements review of contact,
silhouette and motion defects rather than replacing it.

#### GA-15 — P2: synchronised audio, effects and environmental response

**Reuse:** existing `audio/`, `qualia-audio`, HRTF/acoustic primitives, activity cues,
physical response kernels and field/event data.
**Add:** one clock/sequence contract for footsteps, vocalisation, wings, impacts, particles,
water ripples and semantic events. Add bounded voice/effect pools, importance-based
virtualisation, distance/occlusion update rates, deterministic variant seeds and deferred
decode outside callback/tick hot paths.

**Exit evidence:** seek, pause and replay neither duplicate nor omit designated events;
large populations stay within declared voice, CPU and upload budgets. A procedural sound
generator is not by itself a calibrated species-specific aero/hydro/pyro-acoustic model;
retain evidence labels for those claims.

#### GA-16 — P2: multiscale inspection, spectral data and governed generation

**Reuse:** partonomy/field sidecars, nested geospatial frames, clipping/slicing, reconstruction,
procedural synthesis, growth primitives, provenance and the newly declared growth/cube data.
**Add:** versioned codecs and consumers where §12.1 identifies representation-only additions.
For volumes, plan independently addressed bricks, spatial registration, transfer functions,
empty-space skipping, clipping/cut-surface semantics and level selection. Specify band
centres/widths, wavelength units, reflectance vs radiance/emittance, calibration, missing
values and uncertainty; do not extend a 400–1000 nm declaration to UV/SWIR/TIR by implication.

For AI-assisted construction, connect the existing governed inference/tool path to bounded
native geometry/material jobs and independent validation. Preserve generator/version/seed,
conditioning, source evidence and rejected candidates. Require multi-view, topology,
rig/deformation and simulation checks before promotion; grammar-valid Vibe output alone
does not guarantee a physically valid embodiment.

**Exit evidence:** users can move between exterior and selected internal detail without
loading the entire organism; coordinate/part identity survives; generated and measured
parameters remain distinguishable. Scientific fidelity is scoped per layer, not inferred
from visual realism.

### 12.4 Performance budgets and acceptance receipts

Set target profiles before implementation (for example: a constrained browser, integrated
GPU desktop and discrete GPU desktop). These are proposed profiles, not hardware support
claims. Each profile should carry:

| Budget/criterion | What must be counted |
|---|---|
| Authoritative logic | Active arenas, fixed queues and output buffers; each core Sentinel pass respects 42 MiB, with a separate process-wide active-cell/worker cap. |
| Cold construction | Input, scratch, intermediate and output bytes concurrently alive, plus task/work/time limits and cancellation; 256 MiB default cold geometry budget unless the caller admits more. |
| CPU playback | Encoded cache, decoded meshes/textures, pose state, retained source bundles, metadata and replacement overlap. |
| GPU residency | Shared geometry, instance/pose streams, textures including mip tails, acceleration data, persistent and transient frame targets, plus concurrent inference/compute use. |
| Transfers | Network in-flight bytes, decode/transcode scratch, upload staging, per-frame upload bytes and old/new resource overlap until completion. |
| Frame service | Simulation/animation work, CPU submission, GPU pass durations, shader warm-up and presentation latency. Report bottlenecks separately. |
| Visual/simulation quality | Silhouette and geometric error, material/normal consistency, contact/rig coverage, temporal stability, replay equivalence and scientific evidence class. |

A **provisional 60 Hz presentation target is 16.67 ms per frame**; a 30 Hz profile has 33.33 ms.
These are frame-budget arithmetic, not measured QualiaDB performance or universal targets.
Allocate service budgets only after GA-04. Simulation, audio and display frequencies can
differ; their synchronisation and overload policies must be explicit.

Every acceptance receipt should record source/build versions, target features, asset/profile
digests, seeds, trace/camera, viewport, cache state, background workloads, measured distributions,
peak resource use, visual result and known exclusions. Record **observed allocation counts**
for Tier-1 kernels using the existing thread-local counter, rather than inferring zero
allocation from a signature or a comment. Cold wrappers may allocate only within their
documented admitted lifecycle.

Separate deterministic authoritative results from visual numerical tolerances. A deterministic
container writer does not prove cross-adapter floating-point reproducibility, a complete
simulation replay, cryptographic authenticity or biological correctness.

### 12.5 Recommended implementation order and closure rule

> **Cross-cutting requirement:** Establish the source-independent SDK and authoring boundary
> in section 13 (GA-17) alongside GA-01–04; advertised capabilities must pass its consumer-only release gate.

1. **Establish truth and protect import/codec boundaries:** GA-01–04. Fix correctness before
   admitting more content through a questionable format path.
2. **Deliver one complete native asset lifecycle:** GA-05–09, initially using a supported
   rigid textured asset with shared instances, then one SKL1 actor. Include native and WASM
   capability receipts.
3. **Demonstrate density and motion:** GA-10–14 with animated fauna, vegetation, terrain and
   a manufactured object. Enable far-field or GPU execution only when its measured costs and
   quality improve the selected profile.
4. **Extend domain depth:** GA-15–16 and specialised scientific models, driven by actual
   inspection/simulation tasks.

Close a work package only when its **producer → serialized product → admission → caller →
observed result** is demonstrated for the declared target. A newly added type, helper,
manifest flag or test fixture is valuable progress and should be recorded as such; it is not
a substitute for the rest of that chain. The intended result is higher-quality reusable
QualiaDB embodiments and smoother bounded playback, with implementation effort focused on
the actual remaining gaps.

---

## 13. Source-independent SDK, asset authoring and application boundary

**Added 2026-10-15. Status: requirements and implementation guidance only.** These
requirements extend §12 and define how games, simulations and assets should be created
against a released QualiaDB engine without access to its implementation source. No SDK
implementation or source-independent build was performed as part of this document update.

### 13.1 Required outcome

**A game or asset author must be able to build, validate, package and run a supported
product using only the released SDK, public interface/format documentation, authoring
tools and their own project sources.** They must not need the QualiaDB source checkout,
engine recompilation, decompilation, disassembly or inspection of implementation internals.

Rust is the implementation language; WASM is a compilation/distribution target. Neither
requires applications to be compiled together with the engine. Game logic may remain Rust,
compiled into its own WASM module and connected through a published host interface. Vibe
programs and data-driven rules are additional supported authoring paths where the released
runtime exposes the required operations. A small browser adapter may connect modules; the
architecture must not require a rewrite of the game's logic into JavaScript.

Engine maintainers may need to add a missing public capability. That is a separately scoped
engine/SDK development task. Creating another supported scene, species embodiment, object,
material, animation or game rule must not routinely require engine changes.

### 13.2 Existing starting points and evidence boundary

The checkout already contains distribution artifacts:
[qualia.js](../../qualiaDB/docs/pkg/qualia/qualia.js),
[qualia.d.ts](../../qualiaDB/docs/pkg/qualia/qualia.d.ts) and
[qualia_bg.wasm](../../qualiaDB/docs/pkg/qualia/qualia_bg.wasm).
The inspected declarations expose **QualiaPortal**, including **load_hmc_mesh**,
**load_scene_assets** and **get_engine_info**. These are existing public seams to extend,
not a reason to invent a second engine.

The [WASM integration manual](../../qualiaDB/docs/manuals/wasm-api.md) describes distinct
Portal, WebCivics and playground profiles. Package names alone must not imply identical
capabilities. The hand-authored
[qualia-engine-sdk.d.ts](../../qualiaDB/docs/contracts/qualia-engine-sdk.d.ts) is primarily
a Civics contract; it is not proof of a complete game-authoring SDK. Likewise,
[webizen-render](../../qualiaDB/crates/webizen-render/Cargo.toml) is currently a Rust
adapter with engine source dependencies, not by itself a source-independent binary SDK.

This review confirmed files and declarations, not matching binary exports, complete API
semantics, release reproducibility or end-to-end consumer builds. GA-17 below must establish
those properties with the actual distributed artifacts.

### 13.3 SDK and authoring requirements

| ID | Requirement | Required acceptance evidence |
|---|---|---|
| SDK-01 | **Atomic, versioned release package.** Distribute the engine WASM, matching loader/bindings, generated declarations, capability/profile manifest, schemas, documentation and examples together. Pin versions and content digests. | A consumer obtains one immutable SDK release; stale bindings, incompatible ABI/schema versions and missing capabilities yield explicit diagnostics. |
| SDK-02 | **Independent application build.** Keep game/application sources, dependencies and outputs separate from engine development. No engine Cargo path/git source dependency, engine workspace membership or engine rebuild may be required by the supported consumer build. | A clean game project builds using the SDK with the engine checkout absent. Building a Rust game may require its Rust toolchain; it must not require compiling QualiaDB. |
| SDK-03 | **Source-independent asset production.** Supply prebuilt native tools and/or released WASM authoring APIs for supported import, procedural generation, cooking, validation, packaging and inspection. Document deterministic recipes, budgets and outputs. | An author produces and validates an HMC and its supported geometry, textures, behaviour and metadata using published tools only. Unsupported features receive a declared fallback or refusal. |
| SDK-04 | **Documented runtime services.** Define supported operations for scene/asset loading, entity instantiation, resource/state updates, camera/input, picking, events, simulation stepping and session persistence. State which operations are available in each profile. | A runnable example demonstrates the claimed services through public entry points; native-only library functions are not advertised as browser exports. |
| SDK-05 | **Independent behaviour and extension interface.** Define how Vibe programs, data-driven rules and separately compiled Rust/WASM game modules invoke supported engine services. Specify imports/exports, lifecycle, scheduling, effects, quotas, cancellation and error propagation. | A game-specific behaviour is added and distributed without altering the engine binary; unsupported extensions are refused with a useful diagnostic. |
| SDK-06 | **Stable data and ownership contract.** Publish handles, command/event formats and buffer layouts, including alignment, lengths, units, coordinates, identity encoding, ownership and validity across calls or memory growth. Do not expose private Rust layouts or engine-internal wgpu objects as the SDK contract. | Contract fixtures verify supported versions and malformed input handling. u64 identifiers survive JS crossings through declared bigint/string/packed representations rather than lossy Number conversion. |
| SDK-07 | **Bounded resource use across the boundary.** Preserve Tier-1 zero-heap execution, caller-buffered operations, cold construction budgets, shared GPU admission and explicit release semantics. Copies/staging must be measured and documented where they occur. | Authoring, command submission and playback respect declared budgets; exhausted queues/buffers fail explicitly; extension replacement and cancellation do not leak resources or invalidate live handles. |
| SDK-08 | **Compatibility and capability discovery.** Declare engine/SDK ABI, asset schema, extension ABI and profile versions independently. Applications declare their required capabilities; the loader checks them before starting. | A missing feature is diagnosed before execution. An automatic fallback to a different engine profile must not silently remove required game functionality. |
| SDK-09 | **Developer-facing diagnostics.** Provide typed errors, validation receipts and public traces sufficient to locate asset/API problems without inspecting the engine implementation. Keep release-specific documentation and examples aligned with actual exports. | An author can distinguish invalid assets, incompatible versions, budget refusals, unavailable features and runtime faults using SDK-visible information. |
| SDK-10 | **Consumer-only conformance test.** Run the acceptance workflow in an isolated environment with released SDK/tools and example-game sources only. Deny engine-source mounts and source fetches; pin permitted dependencies. | Asset production, game build, load, interaction, save/reload and teardown succeed, with the engine artifact digest unchanged. Produce a receipt for each supported target/profile. |

A release may contain public SDK wrapper source, declarations, schemas and examples.
Those are the integration contract; they are distinct from requiring access to the engine's
implementation source. Generated JavaScript glue is also a normal public interface artifact.
The [wasm-bindgen CLI](https://wasm-bindgen.github.io/wasm-bindgen/reference/cli.html)
supports distributing processed WASM, JavaScript bindings and TypeScript declarations.

### 13.4 Extension design and compatibility policy

Use the existing browser bindings for supported application orchestration, and define
bounded command/event interfaces for Rust/WASM game modules where needed. A Rust SDK crate
may contain public types, serialization and import wrappers without containing or rebuilding
the engine. Independently compiled modules must not depend on an undocumented common heap,
Rust object layout, allocator or panic-unwinding convention.

Keep engine-owned resources behind validated handles. Prefer batched commands and typed
buffers for high-volume updates; define invalid-handle, queue-full and partial-failure
behaviour. Shared memory or zero-copy paths require their own explicit lifetime,
synchronisation and target-capability contract, not assumptions based on both modules being
WASM.

WIT/Component Model interfaces are an optional evolution for independently compiled
components, not a prerequisite for using the current portal package. WIT describes imports,
exports and types; it does not itself implement game services or make existing core WASM
artifacts compatible. Any adoption needs an admitted browser/native host and binding/toolchain
plan. See the [WIT reference](https://component-model.bytecodealliance.org/design/wit.html).

An SDK limitation should produce a minimal public-API reproduction and a capability request.
Resolve it through an existing interface, an explicitly supported extension, a documented
reduced feature set, or a separately versioned engine enhancement. Do not patch generated
bindings to pretend a missing binary export exists, or embed a second renderer/simulation
implementation to bypass the intended engine service without an explicit architecture decision.

### 13.5 Application-agent access and working instructions

Enforce this boundary through project layout and permissions as well as written instructions.
Application agents should receive write access to the game/content project and read access
to the released SDK, documentation and validation tools. Routine application sessions should
not mount the engine development checkout or receive credentials permitting its modification.
Keep engine binary digests or read-only package storage under verification.

Include the following in game/content project instructions:

> Treat QualiaDB as a versioned external runtime. Use its published SDK, documented asset
> formats, authoring tools and extension interfaces. Do not edit, rebuild, decompile,
> disassemble or inspect engine implementation internals. Game-specific code and content
> belong in this project. If a required capability is unavailable, identify the exact
> missing public operation and provide a minimal reproduction using the pinned SDK.
> Record an engine/SDK enhancement request rather than modifying the engine. Do not infer
> that Rust or WASM requires access to QualiaDB implementation sources.

The rule concerns source-independent authoring. It does not promise that client-delivered
WASM is impossible to inspect or reverse-engineer, or alter the engine's licensing terms.

### 13.6 GA-17 — P0/P1: establish the released-SDK consumer boundary

**Reuse:** the existing portal artifacts, generated declarations, runtime loader, Vibe
interfaces, format validators and asset compiler/packaging services. Map public services
to SDK-01–10 and report implemented, partial and unavailable items per release profile.

**Add:** a coherent distributable SDK and authoring tool release, a small independent
Rust/WASM or Vibe-based example game, and the isolated consumer-only acceptance workflow.
Use a representative textured rigid object and one supported animated entity; produce assets
through the released tools, instantiate multiple entities, exercise an interaction, save and
restore state, and cleanly release resources. Record boundary copies, budget refusals and
capability diagnostics alongside successful behaviour.

**Dependencies and ordering:** start the contract/distribution audit alongside GA-01–04.
Use GA-03/05 for importer/cook gaps and GA-07–10 for delivery, residency and animation services;
do not require every P2 feature before releasing a useful supported SDK. The source-independent
consumer test is a release gate for whichever capabilities are advertised. Every subsequent
GA work package must expose applicable results through this boundary, not only internal Rust
calls or tests.

**Exit evidence:** a new author, with no QualiaDB source checkout, can create the example's
assets and software, build and run them against a pinned engine, and diagnose intentional
errors from public information. The engine digest remains unchanged throughout. Record this
as demonstrated only when the workflow has actually run; the present document adds requirements,
not an implementation or certification.

*End of reference. Regenerate sections as the codebase moves; where this document and
`q42-implementation-divergence.md` disagree, the divergence log and live source win.*
