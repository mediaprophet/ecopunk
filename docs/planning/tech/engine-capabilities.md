# QualiaDB engine capability map — what ecopunk can build on

**Status:** draft v0.2 · **Surveyed:** 2026-10-12 against `qualiaDB/crates/` —
`qualia-core-db/src` alone: **~2,509 `.rs` files across 334 directories** — plus
sibling crates (`qualia-vision`, `qualia-semantic-library`, `qualia-extensions`,
`qualia-solid-bridge`, `qualia-peer`, `qualia-cooperative-core`, `qualia-client-core`,
`qualia-q-forge`, `qualia-app-contract`, `poet*`).

**Honesty rule:** **idx** = directory-index listed (auto-generated 2026-06-30, may lag);
**doc** = claimed in README/AGENTS/manuals; **verified** = code/comment read this survey.
Before implementation *depends* on a capability, pin its status + commit in the
capability ledger (game-demo `15-living-qualiadb-capability-ledger.md` pattern —
statuses: unverified / partial / blocked / upstream in progress / upstream verified /
integrated). Scaffold types exist in-tree; names are not evidence of wiring.

---

## 0. Reading guide — where things live

| Area | Path |
|---|---|
| Graph storage | `q42/` (volumes, volume/, p64_weight, **asset_envelope, asset_import, source_catalogue, chebi_*, app_manifest, app_registry**) · `storage/` · `wal.rs`, `wal_intent.rs` · `archive.rs` · `csd_storage.rs`, `zns_storage.rs` (computational/zoned-NVMe lanes) |
| Query & ingest | `query/` (query_engine, query_compiler, mini_parser, resolver, lexicon, rdf_star, shacl_compiler, ontology_loader, source_audit, graph_proof, graph_accel, ingest*, cbor_compiler) · `sparql_library/` (parsers, rdf_formats, serialisers, sparql_grammar, external_sort, immersive, **`sparql_mm` — SPARQL-MM over `ma-ont:` media fragments + C2PA provenance predicates**) |
| Weights | `q42/p64_weight/` — **`.p64` canonical weight container** (compiler/reader/transcode/layout; 64B header+entries, role-tagged tensors, page-aligned quantized blobs, no NQuin scaffold — "q42 manages truth; p64 manages inference"). `gguf_*` paths = GGUF→P64 import/bridge layer — **say "P64" for the canonical artifact** |
| Logic | `deontic_logic.rs`, `deontic_mapping.rs`, `epistemic.rs`, `modalities/` (~40 files + subdirs) |
| Math/science | `solvers/` (20+ areas) · `specialized_libs/` (17 areas) · `domains/` (biological, chemical, financial, geospatial, mathematical, physical) · top-level: `ode_solver.rs`, `quantum_dft.rs`, `qubo_compiler.rs`, `qpu_ingress.rs` |
| Dense assets | `container_10d/` · `bundle/` (.hmc) · `render/` · `tensor/` · `specialized_libs/computational_geometry` |
| Scripting surface | **`poet_host/invoke/`** — capability invocation by domain: agent, audio, chat_graph, chemistry, clinical, cooperative, crypto, docs, econ, engineering, geometry, governance, graph, hypermedia, inference, logic, manifold, math, medical, ml, net, nlp, render, research, runtime, science, sheet, social, stats, vision |
| Network | `net/` (peer fabric, dns, **acoustic_ble_mesh**, **qdnf/** — full decentralized fabric) · `p2p/` · `services/` (daemon, webtorrent seeder/routes, solid_ldp, swarm, poet APIs) |
| Crypto/privacy | `crypto/` (sanctuary_*, fiduciary_crypto, verifiable_credential, zk_*, deontic_circuit, pq_kem_shim, network/key_provider) |
| Identity/governance | `identity/` + `identity/credentials` · `did_qi/` · `governance/` + `governance/webizen` · `domain_licence/` · `entity_view/` · `foundation/` (crdt, frame_layout, fuzz_testing, telemetry) |
| Media/NLP/content | `audio/` (DSP: stft/istft/cqt, hrtf, tf surfaces, sidecar link) · `nlp/` (16 modules) · `hypermedia*` · `text_span` · `semantic_instruments/` · `agent_runtime/` |
| Inference | `inference/` (runtime: kv/paged/prefix/scheduler/receipt/graph_assist/prepared/budget/artifacts; gguf_sharder; qwen; moe; reranker; cuda_lane; conditioning*; ambient_orchestration; lab) · `gguf_bridge/` (incl. WASM WebGPU + wasm_cpu kernels + cuda_decode_plan) · `lora/` · `wasm_llm/` |

## 1. Governed-asset machinery (q42/) — the pack layer mostly exists

| Module | What it provides (verified from headers) | ecopunk use |
|---|---|---|
| `q42/asset_envelope` | `Q42AssetEnvelope` — governed envelope: `UpstreamRelease` identity, digests, formats, `RecordCounts`, parser/mapping `ToolchainVersions`, provenance, `LicencePolicy` (`LicenceClass`, `LicenceObligations`, `RedistributionClass`, `UseClass`), `AssetSensitivity`, `AssetRoutingLane`, `ChunkSpec` bounded chunk plans, `MAX_DERIVED_FROM` lineage, `SENTINEL_PASS_BUDGET_BYTES`. **Unknown licences fail closed; derived assets inherit the union of upstream obligations.** | **This is the GOV-LIC/GOV-PRV machine.** Our rights profiles map onto LicencePolicy; strictest-wins composition = "union of upstream obligations"; chunking solves large-dataset packaging |
| `q42/source_catalogue` | Upstream source/connector **descriptors**; `AcquisitionStatus::{Unverified, Restricted}` — deliberately *no* `Redistributable` variant; redistribution only after a verified `LicencePolicy` on a governed envelope | **The data-source register's engine counterpart** — our australia.md register rows become these descriptors |
| `q42/chebi_parse` / `chebi_map` / `chebi_query` | ChEBI TSV parse → quin mapping (`chebi:accession`, `hasName`, `hasParent`, `fromRelease`) → bounded in-memory query/export over `&[NQuin]` | **Compound/phytochemistry layer already partially exists**; the template for per-source parse→map→query subsystems (replicate for TERN, NSL…) |
| `q42/asset_import` | Import `job`/`budgets`/`status` machinery | Pipeline S6 job orchestration |
| `q42/app_manifest` / `app_registry` | App manifest codec/permissions/project paths; registry records + probe + compat + fixture | QApp/package registration surface |
| `query/graph_proof`, `integrity_omit` | proofs over graph results; integrity bookkeeping | verification receipts (GOV-INT) |

## 2. Logic & modalities (`modalities/` — 92 files, ~27.3k ln; verified against source)

Not a bag of logics — a **layered legal-computability stack** (per
`legal_logic.md` §-numbered phases) plus general reasoning families.

**Temporal & spatial** — `interval_reasoning` (`TemporalInterval` start/end/duration,
constraint satisfaction, temporal planning) · `spatio_temporal` (Epic 21: **Allen
interval algebra + RCC8** `Rcc8Relation`/`SpatialRegion`) · `temporal_ltl`
(OP_LTL_NEXT/UNTIL/GLOBALLY/FINALLY/RELEASE) · `ctl` · `modal`.

**The deontic-legal stack** — `jural` (**Hohfeld's 8 positions**: Claim↔Duty,
Privilege↔No-Right, Power↔Liability, Immunity↔Disability; `correlative`,
`find_unmet_correlatives` — a Claim with no Duty-bearer is a *legible structural
gap*; `personhood_category_error`; vocabulary surface `jural.n3` in
`core-ontologies` w/ `jural:opcode` 0x30–0x37 engine mapping) · `stit` (**agency**: "α sees-to-it-that φ" —
duty-bearer vs bystander, omission detection) · `deontic_compose` (deontic×LTL
`O(Gφ)`/`O(φ U ψ)`, ×epistemic `agent_knows`, **`classify_mens_rea`**,
obligation scope/until) · `meta_deontic` (breach records w/ `prov:wasDerivedFrom`
anchoring, endorsement credentials, WAL recording — *court-admissible record*) ·
`interaction_governance` (verdict→`PolicyMode`: `PreventiveBlock` halts before harm
for non-derogable violations) · `responsibility` (allegation held as **RDF-star
quoted statement** — only adjudication promotes to enforceable fact; §30
meta-guard protects the person *from the system*; `accountability_vacuum`,
`enforcer_overreach`, `rule_of_law_asymmetry`) · `capacity` (juridical capacity,
guardianship vocab, `CoercedConsentFlag`, `VoidableStipulation`, `survivesDeath`,
`posthumous_standing`) · `delegation` (**ZCAP-LD-style chains**, upstream-revocation
cascade `revoked_descendants`) · `contract` (Offer→Assent→Binding,
incorporates-by-reference, capacity-gated) · `legal_compose` (**ZK-gated
eligibility via real Groth16 `zk_proofs`**, selective disclosure,
`translation_status`; §26 **proportionality** — native-only, differentiates harm
expression via symbolic CAS) · `consensus` (obligations survive partition;
joint obligations suspend until sync — complements CRDT LWW) · **`value_flow`
(Permissive Commons economics**: `commons_cost` = production_cost × (1+capped ROI),
`royalty`, `pool_after`, `is_commons_discharged` — **usage obligations discharge at
a cost threshold, then use is freed globally**; integer arithmetic, zero-heap) ·
`capability_gap` (**RPL — Recognition of Prior Learning**: present-vs-lacking set
difference; *experiential/traditional knowledge counts as held* when an
authoritative `skos:closeMatch` links it) · `identity_fabric` (**"an identifier is
not an identity"** — identity = computed fabric of anchors; k-of-n relational
recovery survives key/device loss) · `carrier` (BLAKE3 content-addressed
media↔graph binding, tamper-evident).

**Epistemic & evidential** — `epistemic` (OP_KNOWS 0x20 / BELIEVES 0x21 /
COMMON_KNOWLEDGE 0x22 + **certainty bands** on the assertive/doxastic axis
(`core-ontologies/modal-junctures.n3`), intent/namespace locks, nesting) ·
`epistemic_boundaries` (**`SocraticDegradation`** — legal/medical claims degrade to
probabilistic maps + Socratic prompts: `LEGAL_DISCLAIMER`, `BIO_DISCLAIMER`,
`EMERGENCY_PROMPT`, `requires_physiological_quarantine`,
`forbids_definitive_classification`) · `evidential_etau` (**paraconsistent Eτ
evidential logic** — (μ,λ) f16-packed evidence in quin metadata + **W3C VC
artifact output**) · `inference_guard` (graph-backed boundary for generative
backends — returns an *action*, never emits text).

**General reasoning** — `defeasible` (`DefeasibleStatus`:
Strict/Overridden/Defeated/Active, OP 0x50, DEFEATER_BIT) · `paraconsistent`
(contradiction score, isolate, merge) · `dl` (subsumption over subClassOf **DAG —
multiple inheritance**, ≤256 classes, zero-heap) · `asp` (stable models,
MAX_STABLE_MODELS) · `abductive` (Peirce: chain/minimal/counter-abduction) ·
`argumentation` (Dung frameworks + VAF + bipolar — *honest heap cold-path* label) ·
`dialectical` (**do-calculus** P(Y|do(X)) interventions, counterfactual) ·
`causal` (but-for, overdetermination, voided-dependents) · `probabilistic`
(Bayesian nets ≤ nodes) · `fuzzy`/`fuzzy_quantifiers`/`fuzzy_rdf_schema`/
`fuzzy_type2` · `linear` (CONSUMED_BIT consume semantics) · `manifold` +
`manifold_logic` (∫Ψ > τ → Fact(p) continuous→discrete bridge).

**Coordination** — `graph_theory` (1,483 ln; bounded zero-heap tier +
honestly-quarantined heap tier; motifs, community spans, top-node scores) ·
`blackboard` (semantic channels + hard/soft constraint propagation) · `diffusion`
(GPU-gated passes) · `control_feedback` (MPC/adaptive/MIMO) · `calculus` opcodes ·
`likeliness` · `responsibility`/`stit`/`contract`/`delegation` above.

**`logic/` substrate** — `core` (**`WebizenCompiler`/`WebizenVM`/`WebizenOpcode`**),
`deontic` (1,592 ln; `compile_n3_rule_to_norm`, feature-gated), `n3_parser`
(1,035 ln; Strict `=>` / Defeasible `~>` / Defeater `^>` / Linear `-o`),
`n3_compiler`, `rules`, `qubo`, `owl`, plus **per-domain SHACL libraries**:
`core_modalities_shacl`, `logic_modalities_shacl`, `specialized_libs_shacl`,
`computational_maths_shacl`, `geometry_asset_shacl`, `infrastructure_shacl`.

## 3. Math & solvers (`solvers/` — 235 files)

Self-described "Zero-Allocation Solver Library" (honest note: crate-level
`#![no_std]` not applied; `qpu` pulls std+tokio and is non-WASM gated).

**Numerics** — `calculus` (DOPRI5 adaptive + symplectic integrators, quadrature,
exterior, manifold, mechanics, tensor integrity/provenance, workspace) ·
`exact` (bigint, rational) · `linear_algebra` (gemm, cholesky, eigen, lu, qr,
spectral, svd, vector) · `number_theory` · `polynomial` · `special_functions`
(airy, bessel, orthogonal, zeta) · `transforms` (fourier, laplace, z) ·
`interpolation` (lagrange, least-squares, spline) · `vector_calculus` ·
`geometric_algebra` + `simd_kernel` · `units` (quantity/dimension/conversion/
constants — QUDT-compatible) · `optimization` (metaheuristics incl.
`hill_climbing`) · `shared` (convergence).

**Statistics & learning** — `statistics` (descriptive, correlation, information,
robust, anomaly, histogram, timeseries; distributions×7; hypothesis: anova,
chi-square, nonparametric, t-tests; regression×11 files: multiple, selection,
diagnostics, influence, outliers, discrete, **spurious**, verify) ·
`learning/` (ISL built *on* shared foundation — reuses linear_algebra +
statistics + `compute_bridge`): active, classification (svm(+multiclass),
knn, naive_bayes, discriminant), clustering (kmeans, gmm, hierarchical),
dimensionality (pca, som), experiment (ab_test, bandit, power),
**gaussian_process**, glm (+multinomial), graphical_models (factor_graph),
**kg_embedding (TransE/DistMult/ComplEx/RotatE — honest affordability gating:
score/predict cheap anywhere; `train()` is a run-once artifact-producer on
capable hardware)**, metrics, multiple_testing, preprocessing, regression
(linear, ridge, lasso, bayesian, pcr, pls), resampling (bootstrap, folds,
permutation), sampling (mcmc), sequential (**kalman**, hmm), splines (gam),
survival (cox, kaplan-meier), trees (decision_tree, random_forest, boosting,
**bart**), variational.

**Graph & symbolic** — `ontology_align` (injective assignment via hill-climbing;
emits graded `closeMatch` → `RequiresHumanReview`; **structurally forbidden
from emitting `exactMatch` — the type system has no constructor for it**;
machine proposes, human disposes) · `graph_match` (approximate, fuzzy_similarity) ·
`graph_opt` (hierarchical_path, spreading_activation) · `grounding` (claim_support,
evaluate) · `fuzzy_query` · `symbolic_logic` · NN primitives (`activation`,
`attention`, `feed_forward`, `rope`).

**Quantum** — `qpu` (dispatcher, pre_solver — non-WASM) · `quantum_optimizers` ·
top-level `quantum_dft.rs`, `qubo_compiler.rs` (≤128 vars), `qpu_ingress.rs`
(provider-response firewall, ≤64 samples).

## 4. Domain science (`domains/`, `specialized_libs/`)

| Lib | Contents | ecopunk use |
|---|---|---|
| `domains/biological` | **bioinformatics.rs (1,014 ln, verified)**: Smith-Waterman affine-gap + Needleman-Wunsch, BLOSUM62/nucleotide matrices, k-mer freq + MinHash + Jaccard, **UPGMA tree building (`PhyloMerge`, ≤64 taxa)**, FASTA validation, Tanimoto/Dice fingerprints, DNA→protein translation, isoelectric point, peptide cleavage, SIMD x86_64/aarch64 dispatch. Maps to `qualia:align*` SHACL constraint names | micro layer: sequences, barcodes, eDNA, **phylogenetics** |
| `domains/chemical` | **organic_chemistry.rs (1,943 ln, verified)**: SMILES parse→molecular graph, formula/exact MW, Crippen–Wildman LogP, Ertl TPSA, Lipinski/Veber/Ghose/Egan, 20 functional groups, chiral centres, Morgan circular fingerprints, pKa estimate, InChI validation, thermochem (Arrhenius/Gibbs/van't Hoff/H-H), green chem (AE/E-factor/PMI/RME), `urea_equilibrium_extent`. `qualia:validateSmiles` etc. declared | phytochemistry/compound layer |
| `specialized_libs/chemistry_modeling` | 15 files: DFT, basis_set (1,631 ln), integrals, kinetics, molecular_dynamics, SCF, simulation (1,805 ln), properties, metrics | deep compound modelling |
| `specialized_libs/computational_economics` | **30 files** incl. **`environmental_resource` (HotZeroHeap: carbon/social cost, pollution damage, optimal extraction w/ stock externality, abatement cost-benefit)**, `ontology_bridge` (model configs/results → NQuin facts w/ parity), agent_based, game_theory, mechanism, market_design, econometrics, welfare, public_finance, spatial_economics, forensic_economics | **conservation/resource economics — direct ecopunk fit**; sociosphere economics |
| `domains/physical` | thermodynamics.rs (242 ln): Metropolis MCMC, Gibbs; **LithiumPack + SolarPanel models**, MPPT, heat loss, phase change | energy context; habitat physics |
| `domains/geospatial` | **see §4a — much larger than indexed** | place layers; occurrence indexing; scale ladder |
| `domains/mathematical` | geometric.rs (191 ln): Lorentz/hyperboloid quin embedding, tropical min-plus Voronoi, VectorSectorMap, HomologicalSieve (comment notes exploratory mapping) | exotic spatial/topological ops |
| `domains/financial` (+`financial/economics`, `computational_economics`) | economics: supply-shock I/O propagation, macro flows, bandwidth liability, resilience pricing, seeded Monte Carlo VaR + GBM; tax_schema: jurisdictional rulesets + TaxClearingHouse (AU GST is self-described "mock") | conservation economics; cooperative economics (sociosphere) |

### 4a. `domains/geospatial` — the external-data ingestion + scale-continuum core (verified)

**Adapter framework** (`adapters/mod.rs`, 646 ln): `DataAdapter` trait with
`build_fetch_request(bbox, time_range, registry)` — **every outbound request is
gated by `net::disclosure::NetworkDisclosureRegistry`** (consent-gated egress,
fail-closed `ConsentDenied`). `AdapterRegistry` produces honest `LayerFetchReport`s
(`ConsentDenied`/`ReadyToFetch`/`AdapterNotFound` — no fabricated payloads).
Native blocking fetch; WASM `fetch_region_features_async` parses responses into
**provenance `NQuin`s (title/licence)**.

| Adapter | Provides | ecopunk source |
|---|---|---|
| `gbif_adapter` | occurrence-API bbox queries — **self-described stub** (request building + page estimates; shallow parse) | GBIF/ALA occurrences |
| `stac_adapter` | STAC API, 4D x/y/z/t bbox → provenance quins | **DEA satellite catalogues**, Earth assets |
| `wms_adapter` | OGC WMS/WFS/WCS/WMTS | DCCEEW, state portals |
| `ckan_adapter` | CKAN discovery + JSON consumption | **data.gov.au / NSW / QLD portals are CKAN** |
| `dem_adapter` | **consent-gated Terrarium PNG tiles → decode → mosaic → `compile_terrain_tile` → `.10d` tile encode**; per-country attribution string (incl. *"Australia terrain data © Commonwealth of Australia (Geoscience Australia) 2017"*) | DEM/terrain — the raster→10D path exists here! |
| `opendap_adapter` | OPeNDAP/THREDDS 4D-5D NetCDF subsetting | **IMOS, BOM, NCI** |
| `ogc_3d_tiles` | tileset.json → provenance quins (canonical lexicon hashing) | 3D city/asset layers |
| `sparql_adapter` | SPARQL endpoints (Wikidata) → provenance quins | TERN GraphDB, LOD cloud |
| `osm_adapter` | Overpass + MVT (900 ln, deepest adapter) | basemap/context |
| `ivoa_tap_adapter`, `astrometry_adapter` | astronomy data | starfield/celestial context |

**Spatial stack:** `SpatiotemporalQuadTree` (xy-bounds + t0/t1 intervals —
a genuine spatio-temporal index); `canvas_query::query_region`; `geodetic`
(WGS84 ECEF/ENU); `dem` heightfield→`TerrainMesh`; `terrain_pipeline`
(DEM→geodetic anchoring→`.10d`); `streaming` (frustum prediction, `StreamBudget`
VRAM residency); `spatial_sync` ("who-planted-what-where" shared-world protocol);
`ar_anchor` (Spatial-Web UWB/VPS anchors); `vector_scene` OSM→mesh compiler
(bounded ENU, DEM drape, tile clip); `render_surface` (Chora canvas descriptor,
webgpu/canvas2d).

**Scale-continuum (the micro↔macro ladder):** `reference_frame` — hierarchical
`ReferenceFrame`s (id, parent, translation, rotation, **scale multiplier**)
explicitly designed "to bridge the micro-verse (e.g. molecular structures) with
the macro-verse (e.g. planetary scale)"; `scale_continuum` — LOD transitions
(>1000× zoom → child frame). **This is the engine-side answer to ecopunk's
microscopic↔macroscopic packaging requirement.**

**Stewardship-of-place governance:** `steward.rs` — `StewardContract`
(steward DID, asset hash, scope, expiry, quorum) + `validate_steward_unlock`
→ `StewardVerdict`, built on deontic `OP_PERMIT` extraction;
`stewardship.rs` — M-of-N steward quorum bounded to a spatial domain;
`canvas_rights.rs` — **G2 explicit-denial guard**: a location owner can
explicitly deny a principal or asset, fail-closed; `triggers.rs` —
`LocationTriggerRegistry`/`LocationTriggerEngine`: entering a region injects
governed N3 rules into the Webizen VM, keyed by
`modalities::spatio_temporal::Rcc8Relation` (**RCC-8 spatial reasoning exists**).
Directly aligned with ecopunk custodianship/sovereignty axioms.
| `specialized_libs/computational_geometry` | **~100 files** — exact predicates (orient3d, incircle, insphere, sos, exact_kernel/construct_3/arrangement), hull2/3, delaunay_2/3 + refine + constrained, arrangements + dcel_overlay + trapezoidal_map + point_location (kirkpatrick), boolean_2/3 + nary_csg + corefine_3d, bvh + kd_tree + range_trees + nn_query, motion_planning, remesh_3 (1,965 ln) + anisotropic + decimate_3 + mesh_quality + quad_octree_mesh + advancing_front, reconstruct_3d + screened_poisson + isosurface, **tda + vr_filtration + persistence** (topological data analysis), parametric_cad + authoring + authoring_loft, minkowski_2, natural_neighbour, alpha_shape, ham_sandwich, fem_certificate, gpu/gpu_3d/gpu_oracle, determinism_corpus + benchmark_corpus + fixture_registry, capability_manifests + geometry_workspace (Tier-2 arenas), `tet_quality_improve`, `wheel_panel` | **asset production; terrain; morphometrics; TDA on occurrence clouds** |
| `specialized_libs/computer_vision` | cv subdirs ×13 (buffer/codecs/color/contours/draw/edges/features/filter/flow/hist/morph/photo/sr/transform/video) · **bio** (dicom_lite, histopathology, medical, morphology, radiomics, tracking) · embeddings, gpu, ops, spatial, sr | specimen-image processing, leaf/trait extraction, morphology |
| `specialized_libs/engineering_analysis` | 16 files: fem (1,208 ln), structural (1,350), cfd, fluid, thermal(+conduction), vibration, dynamics, buckling, reliability (1,075), survival, mechanical | biomech-lite; structural validation |
| `specialized_libs/physics_simulation` | 22 files: nbody, cfd, molecular_dynamics, quantum, emf, **population** (!), fields, mechanics, ode, discretization, time_integration — delegates to real `solvers::calculus` integrators, never re-derives | ecological sim bounds; **population dynamics** |
| `specialized_libs/machine_learning` | training/inference/optimization/monitoring/version_control/model_manager/loader (bounded GGUF preview) | ecopunk models lifecycle |
| `specialized_libs/statistical_computing` | accelerator, analytics, catalog, compression, computation, datasets, indexing, scheduler + **privacy.rs: `NoiseMechanism` (Laplace/Gaussian/Exponential/Geometric) + `PrivacyAccountant` (ε/δ spend tracking)** | dataset stats + **DP aggregates for sensitive occurrences** |
| `specialized_libs/linear_algebra/privacy` | **BFV homomorphic encryption** + calibrated DP (Laplace/Gaussian, basic/advanced/RDP accounting) — AGENTS-verified complete | confidential queries over sensitive data |
| `specialized_libs/quantum_biology` | orchestrator (semantic router), gpu_pipeline, qpu_bridge (**IBM Quantum API via NativeQuantumDft**), quantum_state, entities, context | photosynthesis/quantum-bio research lane (future) |
| `specialized_libs/cryptographic_library` | encryption, hashing, signing, proofs, security + key_management | pack signing |
| `specialized_libs/medical_computing` | 12 files: clinical_formulas, diagnosis, differential, drug_discovery, cheminformatics, imaging + image_dsp, records, privacy (1,706 ln), compliance | Sanctuary-side health joins (user data never leaves their volume) |
| `specialized_libs/financial_modeling` | 14 files: portfolio(+risk), pricing, execution, trading, settlement, rebalancing, compliance, reporting, performance | cooperative economics (sociosphere) |
| `specialized_libs/computational_economics` | see row above — `environmental_resource` + `ontology_bridge` are the ecopunk keys | — |
| symbolic libs | symbolic_algebra/assumptions/integration/limits/ode/series/solve/trig | model derivation |

## 5. Rendering & assets (`render/`, `container_10d/`)

- `render/gpu` + `gpu/portal_gpu` · `render/portal` + `hud` (bounded WASM canvas docs) ·
  `render/physics` (+response) · `render/scene_graph` (nodes, lights, **ik**, smooth_damp)
  · **`render/skeletal`**: `SkeletalPlayer` — allocation-free clip playback: joints,
  SkinWeights, ClipRecords, Tracks/Keys, bind pose, joint deltas, `PlaybackReceipt`,
  `reduced_motion` flag — *skeletal playback exists further than the October AG ledger
  suggested* (verify browser path before claiming) · `render/water_surface` (shoreline —
  AG-10 verified upstream) · `render/anatomy` · `render/camera_mount` · `render/assets`
  (mesh→nquins importer — has the known parity bug).
- `container_10d/`: header (axis roles), mesh, node, field (`FIEL`: scalar/vector/tensor
  over nodes/cells/voxels — **the dense-field vehicle for climate/trait rasters when a
  writer lands**), topology, spatial_index, provenance, material (MAT1), normals (FRM1),
  texcoords, skeletal (SKL1), manifold_identity v2, surface_reading, conformance,
  metric_check.
- **The 10D substrate as an address space** (verified `container_10d/axis_role.rs`,
  `header.rs`, `modalities/manifold.rs`): the format is not "10 dimensions" loosely —
  it is a **ten-lane typed address grid**. Canonical axis order
  `[q, v, w, x, y, z, t, α, μ, σ]` with a normative role taxonomy
  (`AxisRole`, Option-A baseline confirmed by the author 2026-07-04):
  - `q, v, w` = **Selector** lanes — excluded from every distance sum; they select
    the *kind of space* (v is the metric selector: Euclidean v=0, Cyclic v=1,
    Hyperbolic v=2, Boundary-clique v≥3).
  - `x, y, z, t, α, σ` = **Coordinate** lanes — participate in distance.
  - `μ` = **CoordinateCarrier** — dual role: a measured coordinate *and* the
    in-band provenance/consent carrier (t-vs-μ reconciliation resolved
    2026-08-19: `t` is coordinate time, `μ` is the provenance lane).
  - **Two distinct tens** (`manifold.rs` T14 — do not conflate): `Tensor10D`
    (pose/query lanes above; VibeScript-visible) vs `ManifoldCoordinate10D` /
    `Epistemic10D` (`scale, attention_depth, epistemic_weight, topological_spin,
    temporal_decay, entropy_bias, spatial_phase, recurrence_frequency,
    density_threshold, manifold_curvature` — epistemic/attention geometry,
    host-internal). A host-internal morphism projects pose→epistemic.
  - **Why it reads as a 'quantum' address foundation:** an address =
    *which-metric-space* (selectors) + *where-in-it* (coordinates incl. time)
    + *carried-provenance* (μ) — a grid position whose metric, temporal, and
    provenance semantics travel with the address itself. `did:q42` topological
    pointers (MSB-dispatch, bit-63 hardware-pointer path) resolve through it.
    **The intended axis semantics (0.0.13 spec):** `q` = epistemic
    superposition/pending-escrow lane (`q=0` collapsed truth, `q>0` sandbox/
    what-if/GSR-escrow; collapse = frame evolution at `t+1`); `w` = domain
    bifurcation (w=3 environmental/sensor is the ecopunk lane); `v` classes
    include `v=2` hyperbolic (native taxonomy terrain) and `v=3+` sovereign
    cliques (byte-level membership boundaries — the geometric form of
    custodial gating); `[α,μ,σ]` = spectral-logical payload (SPD/timbral
    truth layer — RGB is a projection, not the stored truth) + GSR
    quantum-resolver escrow design (classical exhaustion → QUBO →
    proof-of-demand → holographic caching). Full reconciliation:
    `tech/tensor-10d.md`.
  - **Dimensionalities enumerate against the grid** — 2D rasters, 3D meshes,
    4D spacetime, FIEL fields are sections/role-assignments *on* the grid;
    `scale_continuum`/`ReferenceFrame` adds the scale bridging. Ecopunk
    datasets therefore inherit an addressing scheme: every layer, asset, and
    field value has a position in one typed 10D address space.
- `tensor/`: manifold, spacetime, spectral, topology, volume_gpu, payload, coordinate,
  gsr, kv_provenance, q42_integration, bake_pipeline, buffer_export, resident_substrate.

## 6. Network fabric & services

`net/qdnf/` — the Qualia Decentralized Network Fabric: authority, bearer (incl.
**raw_ethernet**), biometrics, cells, clinical/mailbox, contracts(+compile),
crypto/handshake, economics, evidence, fabric, frame, harness (multihop/oracles/
scenarios), link, **policy_labels**, profiles, registries, replication, resolve(+qsr),
route, session(+handshake), vertical — a full governed networking substrate.
`net/peer/` (cells, connectivity, fabric/wire, host, replication, runtime),
`net/dns`, `net/acoustic_ble_mesh` (!). `services/`: daemon family (graph/query/swarm/
tensor), ilp_dispatcher, rpc, **webtorrent seeder/routes** (pack distribution),
**solid_ldp** (+ `qualia-solid-bridge`: pod_store, ldp_translator, solid_proxy,
oidc_micro_idp), `poet_*_api` surfaces, `swarm`.

## 7. Identity, crypto, governance

`did_qi` (method, document, mailbox_bind, utxo, vectors) · `identity/` + credentials ·
`crypto/`: sanctuary_crypto/keychain/audit(+dag), fiduciary_crypto, verifiable_credential,
zk_predicates/zk_proofs, pq_kem_shim, deontic_circuit, network/key_provider ·
`deontic_logic` (VC eval → subgraph key release, accessible layers,
compile_n3_rule_to_norm) · `domain_licence` (model/codec/**registry**/verify —
machine-checkable licences; align our P-tier rights profiles here) · `entity_view`
(attribution, capability_report, circumstance, entity_id, observer, package, projection,
**rights_filter** — per-observer lawful views of entities) · `governance/` +
`governance/webizen` · `foundation/crdt` (LWW, delegation, suspended M-of-N queue),
frame_layout, fuzz_testing, telemetry · `qualia-cooperative-core` (agency_delegation/
domain, authority_type, finance, projects, provenance, qapp_package incl. PWA + remote
controller, record, **taxonomy**, trigger, work_item).

## 8. Media, NLP, research

`audio/`: stft/istft/cqt, dsp + dsp_kernel (parametric voices — sonification),
oscillator/filter/envelope/lfo/meters/midi_transport, hrtf, acoustic_plane/sab,
tf_surface(+edit — STFT/CQT rasters as 2D manifold surfaces: time×freq×magnitude),
audio_sidecar_link — **the EcoAcoustics DSP substrate** (no dedicated bioacoustic
*classifier* yet — tf_surface→classifier path is a gap candidate). `nlp/`:
tokenize, normalize, terms, **gazetteer (Aho-Corasick → Hit{span, iri, surface} —
the text-side entity recogniser)**, link, **relation (shallow-pattern RDF-star
triple extraction: "X is Y"→type, "X located in Y", etc., WASM-compatible, no LLM)**,
coref(+coref/), frame, fst, span, substrate, graphrag, contracts — text→graph for
encyclopedic literature processing (Flora of Australia, BHL/Trove). `text_span`:
byte-span document→quin provenance anchors.
`hypermedia/processors`, `hypermedia_authoring` (dmx/image/interactive/portals/video/
world3d). `research/`: hypothesis/investigation/enquiry/corpus/dynamics/inference_chain/
perspective/sentiment/intentionality/dark_link/assessment. `semantic_instruments/`:
attestation, execution, resolve, catalogue, category, manifest, manufacture, package,
reference, rpl, canonical, visual — **packaged attestable capability objects** (model
our "instruments": SDM runs, ID workflows). `agent_runtime` (evaluator/orchestration/
planner). `agent_intent` ingest format.

## 9. Compute substrate

`platform/compute_bridge`: `KernelClass` taxonomy → open `ProbeableBackend` registry →
capability matrix → `ComputePolicy::select` → `Plan` — hardware dispatch deferred to
measured per-class capability; CPU reference always present; native-only (WASM narrower).
`gpu_context` (shared GPU, OperationalMode pressure tiers, VRAM ledger).
`wgsl_forge`: ir/emit/execute/oracle/calibration/graph_ops+executor/physics/audio —
certified WGSL generation. `inference/`: runtime (kv paged/prefix, scheduler, receipts,
graph_assist, prepared, budget, artifacts), gguf_sharder+tokenizer, qwen, moe, reranker,
cuda_lane (mega_pass/paged_attention/q8/dp4a/tuning), conditioning(+codec/eval/opt —
Prompt Precision), ambient_orchestration, inference_agent, inference_bench, lab.
`gguf_bridge`: attention, browser/webgpu, wasm_cpu/kernels, cuda_decode_plan, mc8_wasm.
`lora/`. `wasm_llm/`, `wasm_bridge/engine`, `wasm_edge.rs`, `spatial_wasm.rs`,
`geometry_wasm.rs`, `wasm_playground.rs` — the WASM surface.

## 10. Sibling crates

**`qualia-vision`** — the **product surface for visual intelligence** (MIG-V2:
pure kernels live in core `specialized_libs/computer_vision`; this crate owns
product layers): `detector.rs` (`GridMultiObjectDetector` — cell-grid + class-
agnostic NMS; **honest scope: not a neural YOLO head** — cell scorer by channel/
edge energy, "replace with P64 encoder when weights land"), `classifier.rs`
(frozen-embedding + linear head, closed vocabulary), `cpu_reference`, `preprocess`
(NMS), `tracker/` (greedy-IoU multi-object tracks), **`weights/` (load_onnx_bytes,
onnx_session, load_litert_bytes, resolve_vision_asset — real model loading for
when weights ship via P64)**, `spatial/` (**`image_to_3d`, `compile_10d_handoff`
— vision→geometry bridge**), `media_store`, `metrics`, `overlay`, `synthetic`,
`capability/` (self-describing registry), **taxonomies: COCO, Pascal VOC,
OpenImages, ADE20K, Cityscapes, KITTI, CIFAR, VGGFace2**, and `biosense/` —
consent+policy-gated biosensing: YuNet face decode, face_mesh (mediapipe idx),
liveness/PAD (challenge/jitter/pose/asymmetry), rPPG heart-rate, respiration,
EVM magnification, affect (valence/arousal **proposals**, not determinations),
biometrics (sface embed, template hash), `consent/` (purpose-bound) +
`policy/` (`evaluate_processing_act`, `policy_permit_check`). **Detections emit
epistemic-observation quins** — `proposesClass`, `hasBoundingBox`, `hasTrackId`,
`humanRejects`, `humanCorrectsClass`, `modelDigest` — so a model *proposes*
and a human can reject/correct (the same human-disposition invariant as
`ontology_align`); `recipes/sparql_mm_observation_query` makes them SPARQL-
queryable. lib.rs rules: caller-buffered hot path, observations≠ground truth,
**dense pixels/biometric templates never in NQuins**, no Python/OpenCV ·
`qualia-semantic-library` (`.hmc` ZIP variant, ingest
acquire/chunk/cml/extract, embeddings, ontology, ollama llm) · `qualia-extensions`
(**PINN** physics-informed NN, **SNN** spiking NN, QPU ext, WebGPU physics shaders:
electromagnetics/fluid/heat/particles/wave/tensor) · `qualia-peer` (peer protocol:
clinical, inventory, negotiate, ops) · `qualia-cooperative-core` (cooperative/agency/
qapp_package) · `qualia-client-core` (client API surface incl. **`chora/layers`**: catalog,
mesh_gen, **nasa_gibs** — NASA GIBS satellite-imagery layer integration already exists,
starfield) · `qualia-q-forge` (QASM parser + statevector sim) · `qualia-app-contract`
(adapter, catalog, delegation, descriptor, effects, gesture, id, request, target) ·
`poet`, `poet-cli`.

## 11. Confirmed gaps (register seeds for `engine-requirements.md`)

| Gap | Note |
|---|---|
| Full glTF scene preservation / compiled geometry sidecar | manual §2.1 "Missing"; GLB→.10d in progress (AG-14) |
| `mesh_to_nquins` parity bug | manual §2.1 |
| Raster→`.10d` coverage beyond terrain | **DEM/Terrarium→`.10d` path exists** (`dem_adapter`+`terrain_pipeline`); OPeNDAP adapter subsets NetCDF. Still open: GeoTIFF/general raster→`FIEL` writer, gridded-field (non-terrain) ingest |
| `.hmc` canonical variant (QBDL vs semantic-library ZIP) | QG-01 open upstream |
| `.10d` animation *browser* contract | SKL1 section + SkeletalPlayer exist; portal path to verify (AG-11) |
| Terrain/geography streaming, LOD compiler | AG-09, AG-16 not started |
| Character/botanical asset families | AG-15 not started — *ecopunk-relevant* |
| Coordinate-obfuscation for sensitivity | `obfuscation/` is QPU-offload semantic-stripping, *not* location generalisation — SNES-style generalisation is pipeline work (possibly statistical_computing::privacy DP aggregates) |
| Rights-profile ⇄ `domain_licence`/`asset_envelope::LicencePolicy` alignment spec | both exist; join needs authoring |

## 12. Survey open questions

1. SPARQL/QISP/VibeScript reachability per solver (native-only vs scriptable via
   `poet_host/invoke/*`).
2. `entity_view::rights_filter` runtime wiring to deontic quins — confirm before spec'ing
   interfaces on it.
3. `asset_envelope` ⇄ pack manifest: is the envelope embedded as an `.hmc` entry or the
   q42 volume's own record?
4. `chora` layers — current coverage beyond GIBS/starfield (relevant to env layer feeds).
5. `governance/webizen` — agreement→deontic bridge state (Task G items in AGENTS.md).
