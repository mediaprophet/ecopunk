# QualiaDB — Verified Functional Inventory

*Data appendix to the Investment Brief. Generated 2026-10-17 from the machine audit of
`C:\github\qualiadb` (branch `0.0.40`, working tree). Source registry:
`C:\github\qualiadb\audit-docs\` — regenerate with `audit docs`.*

## Audit totals

| Metric | Value |
|---|---|
| Source files scanned (`.rs`/`.py`/`.wgsl`) | 5,056 (7,573 files inventoried total) |
| Functions extracted | 54,077 (15,451 documented ≈ 29%) |
| `.rs` files under `crates/qualia-core-db/src/` | 2,568 |
| Workspace crates | 30 |
| RDF triples emitted for the whole codebase | 1,393,972 (67 MB `.q42`, ~16 s native ingest) |
| Largest module docs | `crates_qualia-core-db.md` (38 module registries) |

## Domain classification (keyword-derived, `CAPABILITIES.md`)

| Domain | Functions |
|---|---|
| graph | 1,549 |
| 3d | 1,107 |
| semantics | 943 |
| computational geometry | 913 |
| numerics | 911 |
| physics | 593 |
| economics | 256 |
| 2d | 252 |

*Domain tags are heuristic (keyword matching); counts overlap. Treat as scale indicators,
not taxonomy.*

## Directory coverage — `crates/qualia-core-db/src/` (verified 2026-10-17)

| Area | `.rs` files | Area | `.rs` files |
|---|---|---|---|
| `specialized_libs/` | 474 | `render/` | 181 |
| `net/` (peer + QDNF) | 323 | `modalities/` | 85 |
| `inference/` | 271 | `sparql_library/` | 66 |
| `poet_host/` | 221 | `query/` | 44 |
| `solvers/` | 190 | `services/` | 34 |
| `wgsl_forge/` | 72 | `platform/` | 21 |

## Crate map (30 crates — selected)

| Crate | Role |
|---|---|
| `qualia-core-db` | The engine: NQuin, Q42 volumes, .10d/.hmc/.p64 containers, modalities, SPARQL, solvers, geometry, render, audio, inference, net/QDNF, governance |
| `vibe` | VibeScript language: parser, type checker, `VBC1` bytecode, VM, effect system, ~1,086 invoke IDs across ~90 families |
| `qualia-audio` | 1,502 functions — MIR features, MIDI 1/2, bounded production engine, AQA, sonification, gated generation |
| `qualia-client-core` | App layer: chat/inference, BIP32/eCash wallet, Chora canvas, QApps, mail |
| `qualia-extensions` | PINN, SNN, QPU extension, WebGPU physics solvers (Navier-Stokes, FDTD) |
| `qualia-vision` | Native vision ABI → `VisionQuin` records |
| `qualia-semantic-library` | corpus → `.hmc` ZIP hypermedia containers |
| `qualia-solid-bridge` | W3C Solid/LDP pod server + Turtle→NQuin boundary |
| `webizen-render` / `webizen-runtime` | wgpu 30 renderer + fixed-timestep GPU simulation |
| `webizen-lite-wasm` | ~268 KB browser MCP endpoint: N3 parse, Quin query, SHACL, modal logic |
| `wellfare-core` | Health-vault domain crate (anatomy, records) — consumer of the primitives |
| `qualia-cooperative-core` | Supported-agency/cooperative governance (~84 tests, per repo docs) |
| `work-graph` | "Implementation Graph" indexer — scans the checkout for what is Present |
| `poet`, `poet-cli`, `vibe-lsp`, `vibe-wasm`, `qualia-cli`, `qualia-inference-kernel`, `webizen-desktop/studio/web/…` | Tooling & application surface |

## Live in-browser today (site inventory, `docs/`)

| Page | Demonstrates |
|---|---|
| `compute-engine.html` | **87 WASM exports**: linalg, stats (t/ANOVA/chi²/regression), CAS (differentiate/simplify/expand/factor/solve), numerics (special functions, primes, interpolation, Nelder-Mead), exact bigint/rational, units (CODATA), transforms (DFT/Z/Laplace), graph (Dijkstra, spreading activation, fuzzy similarity, KGE) |
| `modalities-showcase.html` | 21 logic families live (deontic, epistemic, paraconsistent, LTL, DL, ASP, linear, dialectical, argumentation, Allen intervals, diffusion, neuro-symbolic, CRDT, CogAI, probabilistic, spatio-temporal, control theory, graph theory, comorbidity, DICOM, agency) |
| `playground/` | SPARQL + SHACL validation, N3 rules, bytecode, RDF→Q42 comparator, 3D anatomy, semantic drift, neuro-symbolic transcompiler, WordNet graph, benchmark harness |
| `science-playground.html` | Clinical risk, G(3,0,0) PGA multivectors, organic chemistry, RK4, sequence alignment, thermodynamic MC |
| `sparql-showcase` / `sparql-examples` | SPARQL engine + query gallery |
| `spatial.html` | GeoSPARQL / spatial mathematics |
| `universe.html` | Knowledge-universe visualization |
| `vibe/*` | VibeScript index, live playground, showcase, spec, dev-docs, benchmarks |
| LLM pages | `edge-llm*`, `online-llm-demo`, `wasm-llm-test`, `wasm-bench-compare`, `benchmarks.html` |
| `semantic-instruments.html`, `agreement-*`, `human-centric-authoring` | Rights/agreement surfaces (narrative + partial demos) |

## Verified capability, NOT publicly demonstrated (the demo gap)

| Capability | Evidence locus |
|---|---|
| Accrual modality — the flagship ecological engine | `crates/qualia-core-db/src/modalities/accrual.rs` — no test suite, no page |
| Botanical/flora entity kernels (turtle_3d, sweep_extrusion, space_colonization, tree_skeleton_3d, canopy_radiation) | `specialized_libs/computational_geometry/` — 0 site mentions |
| Physics sim & engineering analysis (boids, nbody, MD, LBM, FEM, Newmark-β, vibration, reliability) | `specialized_libs/{physics_simulation,engineering_analysis}/` — 0 demos |
| Audio (1,502-fn crate + holosonic + HRTF + bioacoustics) | `crates/qualia-audio/`, `audio/` — `js/qualia-audio-worklet.js` orphaned, 0 pages |
| Vision / bio-imaging | `specialized_libs/computer_vision/`, `crates/qualia-vision` — 0 pages |
| Economics suite | `specialized_libs/computational_economics/` (~33 files) — 0 demos |
| Ethograms / developmental keyframes / EONT1 | `entity_view/ethogram.rs`, `container_10d/entity_ontology_section.rs` — 0 pages |
| Spectral pipeline (Jakob-Hanika basis, metamer, hyperspectral) | `render/spectral*.rs` — 0 mentions |
| Commons economics mechanics (discharge, meters→pool) | `modalities/socioeconomics.rs`, `bundle/cbor_ld_manifest.rs` — test-invoked only |

## Gap / honesty register (things a reviewer will find)

- `discharge_to_commons`, `accumulate_meter_to_pool` — mechanisms real, **zero production callers**.
- `SectionType::HyperspectralCube = 21` — enum admitted; no dedicated codec/consumer yet.
- `PermissiveRoutingLane::EnforcePermissiveCommons` exists and is enforced in `cell_graph.rs`,
  but nothing binds obligation-discharge to a lane transition.
- `logic.rs::extract_float` uses a conflicting `0b001<<60` object-tag interpretation vs.
  `resolver.rs` — documented inconsistency (AGENTS §4-D).
- `.ntx` does not exist as a format; `FILTER NOT EXISTS` returns wrong results in QualiaDB
  SPARQL (use `MINUS`); SPARQL rejects `;` sugar and `#` comments.
- glTF full scene preservation (normals/UVs/materials/skins/animation/morphs) is missing —
  `render/assets` emits semantic projections only.
- Speech is a gated scaffold (no full ASR/TTS); licensed generation paths are explicitly gated.
- Site defects: broken `vibe-host-native-bridge.html` nav link; orphaned JS (`qualia-audio-worklet.js`,
  `ambient-viz.js`); content dated ~Oct 1–10 missing Oct 15–17 features.

## Provenance & licensing

- **License:** CC BY-NC-ND 4.0 International — Copyright © 2026 Timothy Charles Holborn.
  Non-commercial, no derivatives; compatible with commons/public-good funding, restricts
  commercial exploitation and downstream forking.
- **Formats:** `.q42` (v4 semantic volumes), `.10d` (tensor/mesh, ABI v1 + spec-reserved tail),
  `.p64` (GGUF/safetensors weight compiler, byte-verbatim + CRC-32C), `.hmc` (two distinct
  containers share the extension — documented divergence), `VBC1` VibeScript bytecode,
  `SRD1` (surface-reading section inside `.10d`), `M64` matter container (in progress).
- **Audit methodology:** tree-sitter extraction (Rust/Python/WGSL) → RDF/Turtle (1.39 M
  triples) → QualiaDB `.q42`; validates existence/location/wiring. Behavioral correctness
  derives from the project's own test reports — quoted, not re-run.
