# Engine requirements register — consolidated gap list

**Status:** draft v0.1 · consolidates every "open item" discovered across the
planning docs into one register. Each entry: what's needed, where it surfaced,
upstream status (as last verified), and what it blocks.

**Current digital-twin asset handoff:** [GAP-12 / G52–G65](#gap-12--interactive-digital-twin-assets)
records upstream requirements from [the per-asset specification](assets/digital-twin-asset-requirements.md).
It includes complete-object HMC preview/UAT, not just individual `.10d` parts.
Its source-confirmed evidence labels are not runtime qualification claims.

**Status vocabulary:** `upstream-verified` (exists in code) · `upstream-partial`
· `upstream-stub` · `upstream-planned` · `eco-spec` (our work) · `open` (needs
investigation).

---

## GAP-01 — Formats & ingest

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G1 | General raster/GeoTIFF → `FIEL` writer (non-terrain grids: climate, soil, species-distribution) | pipeline §6, map-layers §6, earth-space §7 | **upstream-landed (substrate)** — `environmental_raster.rs` + `container_10d/field_section.rs` (Track 4): FIEL section w/ QUDT-typed units + env-layer registry; GeoTIFF decode → f32 path + pack integration unverified | climate/environmental layer packs |
| G2 | Bioacoustic classifier path (`tf_surface` → classifier → observation quins for calls) | engine-capabilities §8 | **upstream-landed (substrate)** — `audio/bioacoustics.rs` (Track 4): TfSurface → `BioacousticVocalization` → SPARQL-MM observation quins w/ `taxon_id`+confidence; species-ID classifier models + taxon-source breadth unverified | EcoAcoustics data, sound packs |
| G3 | SPARQL federation→ingest path (harvest TERN GraphDB etc. into `.q42`, not just live query) | pipeline §6 | **open** — federation supported "local or explicitly requested" | TERN/linked-data ingestion |
| G4 | `.hmc` `meta` CBOR schema per pack family | pipeline §7, classifier-packs | **eco-spec** | pack authoring |
| G5 | Manifest→QBDL pack compile step (Poet `ManifestChain` → `.hmc`) | interfaces §4 | **open** — verify implemented vs spec | Poet authoring workflow |

## GAP-02 — Semantics & query

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G6 | Vision↔SPARQL-MM temporal encoding unify (frame-index metadata vs `mediaTimeMs`/`hasStartTime` objects) | classifier-packs §6 | **upstream-partial** — two query paths exist with different encodings | media-observation queries |
| G7 | Browser animation path (`SkeletalPlayer` → portal wiring) | wasm-edge §3, style-and-scale | **upstream-partial** — source confirmed: `render/portal/scene_animation.rs`, `render/skeletal/player.rs` (qualiaDB), game-side `animation_bridge.rs`; rendered browser pixels/performance still unverified | animated assets in-browser |
| G8 | Graduated/multi-scope capacity semantics (per-domain, fluctuating, informal kinship, contested) | cooperative-layer §6/§7 | **upstream-landed (substrate)** — `modalities/capacity.rs` (Track 4): guardian/dependent, supporter sets, death-aware edges, probate fixtures in tests; graduated/fluctuating-scope coverage vs spec unverified | guardianship modelling |
| G9 | Digital-probate vocabulary (executor, estate, instrument, succession event) + delegation death-semantics | cooperative-layer §6/§7 | **upstream-landed (substrate)** — `modalities/delegation.rs` + capacity.rs (Track 4): `survivesDeath` flag, `revoked_descendants`, `authority_after_revocation`, testator/heir/oracle tests; full probate vocabulary ours | lifecycle custody |
| G10 | `vibe`→standard-web projection toolchain (native-authored → WASM-mode surface) | development-modes §4 | **open** | single-source UI authoring |

## GAP-03 — Network & distribution

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G11 | `pc:Micropayment` live settlement rail (ILP/streaming or equivalent) | durability §7, network-nodes §5, ontology-stack | **upstream-partial** — `ilp_dispatcher.rs` verified: SPSP→ILP-STREAM w/ receipts, Nym routing, offline queue, stablecoin fallback; live connector/endpoint + denomination policy are deploy-side, not code-side | node economics, hosting incentives |
| G12 | QSR public resolver service spec (QDNF naming → resolution) | network-nodes §5 | **open** | identifier resolution service |
| G13 | Test identifier ranges (reserved `did:`/`q42:` testnet space) | network-nodes §5 | **eco-spec + upstream decision** | testnet bring-up |
| G14 | Stewardship-transfer ceremony (key rotation/re-signing protocol) | network-nodes §5 | **open** | custody handovers |
| G15 | Node-service identity convention (`did:qi` doc w/ service endpoints?) | network-nodes §5 | **eco-spec** | node registry |

## GAP-04 — Governance & epistemics

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G16 | Namespace/PID & self-contained `.hmc` packaging strategy (`https://webizen.org/` / `https://ns.webizen.org/` + `did:qi`/`did:q42` digests; ontologies, `.vibe`/`.qbc`, `.wasm` with CUDA/wgpu capability manifests, `.10d`, `.m64`, `.p64` shipped inside `.hmc` with zero external DNS dependency) | ontology-stack §6, durability | **resolved (`https://webizen.org/` + self-contained `.hmc`)** — `ecopunk.au` and runtime reliance on external `/def/` endpoints are prohibited | all minted vocabularies, `.vibe` modules & `.hmc` application packs |
| G17 | Review/ratification workflow spec (who ratifies, quorum per contribution kind, attestation landing) | review-workflow.md | **eco-spec** — machinery exists (`cml:Proposed`→`Attested`, quorum, `PolicyOutcome::NeedsHuman`) | corpus quality gates |
| G18 | `epistemic`/`modal-junctures` → pack-level certainty-band rendering conventions | contribution-chains, product-surfaces | **eco-spec** | interface honesty |
| G19 | `epistemic_boundaries` coverage for biosphere claims (medicinal/definitive-classification guards) | capability-spotlights S2 | **upstream-partial** — medical/legal boundaries exist; domain extension needed | traditional-knowledge claims |

## GAP-05 — Interfaces & surfaces

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G20 | ecopunk/biosphere `invoke/` domain surface (occurrence submission, specimen media, taxon proposal, custodial-consent) | interfaces §4 | **eco-spec + upstream** | Poet contribution flows |
| G21 | Flagship world → pack binding (worlds consume reviewed pack IDs, not raw `adapter://` endpoints) | interfaces §7 | **open** | Chora exploration product |
| G22 | AR anchored-layer pipeline (`ar_anchor` → anchored-overlay layer family → pack sections) | map-layers §2 | **upstream-partial** — anchor module exists | AR product surface |
| G23 | Chora/WellFair VibeScript hosting (does canvas host `vibe` or consume artifacts?) | development-modes §4 | **open** | native-mode surfaces |
| G24 | `vibe` subsystem maturity map (which modules stable for UI chains) | development-modes §4 | **open** | interface authoring |

## GAP-06 — Media & quality

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G25 | Media-pack spec beyond 3D (bioacoustic `.wav`+observations, field video, plates/documents) | assets/media-types.md | **eco-spec** | media contribution kinds |
| G26 | Hero-asset production chain in studio tools (which species first, which tool) | implementation-strategy; remediation R5 (hero-scene capture priorities) | **eco-spec** | quality uplift; R5 visual slice — biosphere species uplift remains the future driver |
| G27 | AI-assisted asset disclosure+review label (generated ≠ authored ≠ captured) | style-and-scale §8 | **eco-spec + governance decision** | art_studio-produced assets |
| G28 | Raster spectral ingest (SPD/CQT sheets → `αμσ` payload lanes) | tensor-10d §5/§7 | **upstream-planned** | spectral-truth assets |

## GAP-07 — Trust & security

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G29 | Sybil/reputation spec for contributor graph (authority can't come from signature count — verified axiom in `qdnf/authority/policy.rs`; how *does* contributor standing accrue?) | threat-model.md | **eco-spec** | contribution trust |
| G30 | Poisoned-alignment detection (bogus `closeMatch` floods) | threat-model.md | **open** — machinery exists (human-ratified exactMatch blocks silent assertion; flood-detection is the gap) | ontology alignment at scale |
| G31 | Contest-abuse handling (denial-of-service via contest records) | threat-model.md | **open** | public contribution surfaces |

## GAP-08 — Runtime, execution & delivery (game-demo remediation)

Surfaced by the 2026-10-12 game-demo diagnosis — see
[game-demo-remediation.md](game-demo-remediation.md) findings F1–F10 and work
packages R0–R6.

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G32 | Webizen-governed worker admission/scheduler for native+browser, Vibe interfaces, incl. ordering qualification | webizen-execution.md; remediation R4 | **upstream-partial + integration** — admission mechanics confirmed in source (`governance/coordination.rs`; full guard-path tests not executed); native scheduler `cfg !wasm32`; a browser pool is not demonstrated/qualified in the inspected game path; scheduler ordering fix **landed** (Track 4 — `pending_tasks.remove(0)` + ordering tests) | R4 governed workers |
| G33 | Versioned state projection / Vibe ownership of semantic state (no graph-text parsing in frame path) | development-modes, remediation F4/F8 | **upstream-partial + integration** — `.vibe` drive rules run via `GameDriveSimulation`; JS currently owns walking/cameras/scene lifecycle/HUD text bridge | R2 Vibe boundary |
| G34 | Range-backed HMC browser acquisition + verified cache/install/offline lifecycle | pack-delivery.md, remediation F5 | **upstream-partial** — native `HttpRangeSource` exists (`cfg !wasm32`); browser range reader, trusted index auth, and cache/install service open | R3 pack service |
| G35 | Loader-generation / first-frame lifecycle contract (job vs scene state machines, one RAF owner) | remediation F10; pack-delivery §5 | **eco-spec + game integration** | R1 loader lifecycle |
| G36 | Dirty animation bounds / actor batching / animation-visibility LOD | remediation F3 | **upstream-partial** — hot paths identified in source; dominance **not measured** | R2 hot-path work |
| G37 | Reproducible graphics/performance device qualification (baseline capture per device/profile) | remediation R0, QA register | **eco-spec + integration** | R0 baseline gate |

## GAP-09 — Statistics & socioeconomic data

Surfaced by the socioeconomic expansion (2026-10-12) — see
[statistics.md](statistics.md) and
[../data-sources/socioeconomic.md](../data-sources/socioeconomic.md).

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G38 | SDMX ingest family — structure harvest (dataflow/DSD/codelist/concept-scheme discovery, versioned) + observation readers (SDMX-ML/-JSON/-CSV) → `qb:`-shaped NQuins | statistics §4, socioeconomic §7 | **upstream-partial** — `query/sdmx_cube.rs` (Track 5): qb: predicate set + `ConfidentialityMark`→sensitivity binding; SDMX-ML/-JSON readers + dataflow/DSD discovery still open | all statistical sources (ABS/OECD/ILO/UN SDG) |
| G39 | Vintage/release-state volume model — a revised series is a new volume linked `wasRevisionOf`; "as-reported-at" queries (STAT-AX-4) | statistics §3/§4 | **eco-spec** — DOM-AX-2/CRDT/Merkle-history machinery is the fit; convention + tooling ours | longitudinal/trust-critical stats |
| G40 | Classification/concordance layer — XKOS-shaped governed schemes + edition concordances as lossy proposals (STAT-AX-3/8) | statistics §3, socioeconomic §2 | **upstream-landed (substrate)** — `solvers/ontology_align/alignment_guard.rs` (Track 5); XKOS shape + edition-concordance semantics still ours | ASGS editions, ANZSIC↔ISIC joins |
| G41 | SEEA/environment-economy bridge — account cells (extent/condition/services) join to `bio:`/`env:` entities via IUCN-GET + geography (STAT-AX-9 modelled-claim rules for MRIO footprints) | statistics §5 | **eco-spec** — ABS NEA/Land Account verified as sources; join schema ours | biosphere↔economy products |
| G42 | Confidentiality-mark semantics — suppressed/perturbed/modelled cell attributes as first-class data; unit-record exclusion enforcement (STAT-AX-2/5) | statistics §3, socioeconomic §5/§8 | **upstream-landed (substrate)** — `ConfidentialityMark` {Public,SuppressedForPrivacy,Perturbed,ModelledEstimate}→sensitivity binding in sdmx_cube.rs (Track 5); `stat:` minted vocab + unit-record enforcement ours | honest aggregate publication |
| G43 | Compensation benchmark layer — labour-class vocabulary + benchmark observations (PPP/living-wage/minima) feeding `royalty(base, agent_multiplier)` per COMP-AX; claim-style computed obligations; worker-stock drawdown attribution (COMP-AX-11) + delivery-structure records (COMP-AX-12) | compensation.md | **eco-spec** — royalty/cost machinery verified (`socioeconomics`, `commons_cost`); benchmark-binding + labour vocab are ours | fair-compensation for contributors |

## GAP-10 — Interpretation & instruments

Surfaced by the interpretation-frameworks expansion (2026-10-12) — see
[interpretation-frameworks.md](interpretation-frameworks.md).

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G44 | Instrument runtime for dataset interpretation — `si:EntryPoint` dispatch, dependency resolution, execution-receipt generation | interpretation-frameworks §2/§8 | **upstream-partial** — `si:` ontology + SHACL shapes + pos/neg fixtures landed (SI-01); dispatch/resolver/receipts are WIP packets; carrier + digest algorithm HELD upstream | instrument-mediated interpretation, G38-derived claims |
| G45 | Axiom-package publishing convention — SOC/STAT/DOM/COMP axiom sets packaged as `si:`-compatible release units so interpretations declare axiom context as dependency | interpretation-frameworks §3/§6 | **eco-spec** | plural-interpretation model |

## GAP-11 — Temporal dynamics & reconciliation

Surfaced by the accrual/reconciliation expansion (2026-10-12) — see
[accrual.md](accrual.md) and [reconciliation.md](reconciliation.md).

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G46 | Reconciliation machinery — `ReconciliationEvent` records, holding→feed trigger binding (geometry/entity/vintage keys), staleness (`reconciledAt`+expected-lag) as queryable state, `reCheckable` propagation through receipts | reconciliation §3/§4 | **upstream-landed (substrate)** — `modalities/reconciliation.rs` (Track 5): `ReconciliationStatus` {Converged,Diverged,Deferred,Contested}, `is_stale` (vintage+expected-lag), `RECHECKABLE_FLAG` metadata bit 15; holding→feed trigger binding + event-driven re-check cascade ours | corpus freshness honesty; event-driven updates |
| G47 | Accrual/time-depth vocabulary — `Stock` aspect, `accrualProfile` dimension registry, `irreversibilityClass`, `LossEvent`, rate pairs, spent-lifetime/window-bound/constitutive-investment classes (AC-AX-11–14) | accrual §2/§3A | **upstream-landed (substrate)** — `modalities/accrual.rs` (Track 5): `MaturityClass`, 4-tier `IrreversibilityClass`, `asymmetry_ratio`, `eval_accrual_loss`/`AccrualLossReport` (AC-AX-1..6); AC-AX-11–14 investor-side/window vocab + dimension registry ours | loss accounting; non-commodity valuation |
| G48 | Probability-field machinery — `ProbabilityField` as Q42-Tensor10D-native structure (intended carrier: measure on `α`, epistemic binding on `μ`, vintage on `t`, rival structures on `q`), path-space representation, declared transports/divergences, hazard/survival parameterisation, typed-friction fabric vocabulary (translational/epistemic/sovereign/dimensional, asymmetric, impermeable first-class) | probability-geometry §2–§5A | **eco-spec + upstream-partial** — carrier axes verified; q-superposition/epistemic dynamics design-level (tensor-10d §7) | probabilistic claims; lifespan/actuarial surfaces; event re-weighting |
| G49 | Taxon-identity convention + entity binding — engine canonical scheme is GBIF `u64` key (`gbif_adapter`, `bioacoustics.taxon_id`); game assets used name strings. Needs `renderKey`→GBIF-key + `dwc:`/auNSL IRI bridge and asset→ontology typing map | game-demo ecology profile, `bioacoustics.rs`, `ontologies.json` | **binding + dwc emission landed** — game-demo `feat/taxon-binding`: profile schemaVersion 2 + `web/fixtures/taxon-bindings.vibe` as single authored source (`taxon_for` → GBIF key/IRI/rank/assetFamily; `dwc_statements_for` emits sealed `quin.statement` records — `rdf:type dwc:Taxon`, `dwc:taxonID/scientificName/taxonRank/vernacularName`, `foaf:depicts` asset→taxon edge; vibe check+eval verified, 6 quins incl. depicts edge), `taxon_bindings.rs` exports `taxon_binding_json` + `taxon_dwc_json` (host packs `urn:qualia:gbif:taxonKey` quin as `INLINE_TAG_INTEGER` — f64 cannot form the 0b001<<60 tag above 2^53), exporter stamps per-asset `taxon.quins`; remaining: pack-compile ingestion of quins into binary sections (G50 path), auNSL flora IRIs | ontology-typed models in packs |
| G50 | Browser ontology delivery — `query/ontology_loader.rs` is `cfg !wasm32`; startup ontologies are native-daemon-only. Browser/WASM needs pack-compiled ontology slices (G4/G5) or a compiled-in volume | ontology_loader.rs | **first slice landed** — game-demo `feat/taxon-binding`: `pack_game_hmc` accepts `sem/*.quins` (48-byte NQuins) + `sem/*.lex` (hash→IRI lexicon) entries with kind `quins`; exporter emits `sem/taxonomy.quins`+`.lex` per pack from the vibe-authored statements; `SemanticSlice` (WASM) parses verified packs, re-checks parity, resolves terms, SHACL-validates via `validate_shacl_property` (game builds `wasm-full`, so the SHACL/N3 surface is in-profile); manifest records `semantics[]` and all three verify formulas count them. **Node/npm dependency removed**: `crates/pack-export` (native bin) repacks `.10d` assets + textures + `sem/` sections via `BundleWriter`, evaluates the `.vibe` fixture with the real interpreter + `PoetSnapshot` host, re-verifies per-entry sha256 at pack time, round-trip-verifies the pack, and emits deterministic output (identical SHA-256 across runs; 905 assets + 39 textures + 328 quins + 2 semantic entries per style). Remaining: live-graph commit path for N3/SPARQL over slice quins, larger ontology slices beyond taxonomy, browser re-verify of rebuilt packs | ontology use in browser/game |
| G51 | Ecology vocabulary coverage — manifest lacks GeoSPARQL (scene placement semantics), fauna backbone, IUCN GET ecosystem typology, trait vocabularies (e.g. TOP) | ontologies.json, data-sources/australia.md | **partial** — auNSL fauna scope **verified**: auNSL unions AFD (~130k faunal taxa) + APC + bryophytes/fungi; `nslvoc` is nomenclatural (Taxon/TaxonName/TaxonNameUsage, SKOS-XL) — name-level, not trait/occurrence backbone. GeoSPARQL 1.0 registered (qualiaDB `feat/ecology-vocab`; 1.1 RDF artifacts moved to Geosemantics DWG repo — pin when defs-server URL verified). IUCN GET has **no published machine-readable vocabulary** ("under preparation" per IUCN; TERN hosts a linked-data copy — verify resolvable URL before manifest entry). GBIF backbone is a DwC-A **dataset**, not an ontology file — its `u64` keys are already the canonical identity scheme; no manifest entry needed | scene/ecology semantics |

**Register maintenance:** entries move to `upstream-verified` on code
confirmation (with path evidence) and out to a "closed" section — keep the
history; a gap closed is evidence the loop works. Source confirmation is not
compiled, executed, or browser-integrated proof; record those separately.

## G50 follow-up — byte-backed Q42/Vibe host qualification

**Authority:** documentation for Timothy to arrange upstream implementation
or qualification. No QualiaDB edits are authorised by this game-demo/ecopunk
work. Historical G50 results above retain their original scope; they do not
qualify the later Q42-only migration. A browser integration gap is not a
finding that the existing HMC/Q42 filesystem or native reader is broken.

**Evidence:** source inspection of the current working trees only. No build,
fixture execution, pack regeneration, or browser qualification was performed
in this supervision review. Existing unapproved Q42 reader edits are not a
release baseline; identify the approved engine revision/profile and actual
WASM/pack digests before interpreting runtime results.

| Inspected path (relative to qualiaDB) | Finding | Qualification consequence |
|---|---|---|
| `crates/vibe/src/catalog/ids.rs`; `crates/vibe/fixtures/graph_sparql.vibe`; `crates/vibe/fixtures/volume_sanctuary.vibe` | Existing GraphDatabase language bindings and fixtures include query and native volume open/commit. | Do not declare the language lacks these operations or invent a replacement grammar. Fixture presence is not execution proof. |
| `crates/vibe/src/catalog/local.rs`, `GraphDatabase.sparql` branch | LocalHost returns an empty list. | Empty LocalHost output cannot prove an empty engine graph or successful engine querying. |
| `crates/qualia-core-db/src/poet_host/invoke/mod.rs`; `invoke/graph/sparql.rs`; `poet_host/mod.rs`, `with_live_quins` | PoetSnapshot dispatches SPARQL to the real parser/planner/executor over resident committed Quins or an attached native graph. | Reuse and qualify the existing engine query path; no JS query engine is needed. A resident snapshot is not persistent/native graph access. |
| `crates/qualia-core-db/src/poet_host/invoke/graph/volume.rs` | `GraphDatabase.volume_open` and `volume_commit` use native filesystem paths and explicitly reject WASM. | A verified-pack byte source needs a public, capability-scoped host seam; native path denial alone says nothing about Q42 format validity. |
| `crates/qualia-core-db/src/poet_host/invoke/graph/lexicon.rs` | `lexicon_manifest` reads a native filesystem JSON manifest; it is not an embedded-Q42LEX lookup API and rejects WASM. | Do not conflate manifest discovery with term resolution from an admitted Q42 volume. |
| `crates/qualia-core-db/src/poet_host/bridge.rs`, `NATIVE_BRIDGE_BINDINGS` | SPARQL route metadata declares standalone semantics unavailable, while the inspected invoke path executes over a snapshot. | Upstream must qualify and reconcile advertised route/semantics with actual execution. Do not silently select the more favourable claim. |
| `crates/qualia-core-db/src/q42/volume/opfs_source.rs`; `range_volume.rs`; `q42_lex.rs` | Slice-backed exact-range source, range reader, and embedded lexicon parser already exist in source; exposure has unapproved local changes. | Qualify portable public exposure at the approved profile, rather than replacing codecs or claiming the edits already work. |

### Required upstream interface behaviour

1. A public byte-backed Q42 read surface usable by the selected browser/WASM
   profile, with checked header/section/block bounds and caller-buffered
   block decoding. Preserve native reader/writer behaviour and format ABI.
   Reuse the existing slice source instead of adding a duplicate codec.
2. A capability-scoped Vibe host seam that admits an already digest-verified
   HMC semantic entry into an explicitly isolated read-only semantic view.
   The host must validate Q42 structure and Quin parity, retain embedded
   lexicon resolution and provenance (pack identity, entry key/digest, Q42
   version), and reject failed admission without replacing prior state.
   Exact binding naming and handle lifecycle belong to the upstream public
   contract; the game must not invent a private substitute.
3. Existing engine SPARQL/SHACL operations must access that admitted view
   through VibeScript, with exact u64 identifiers and the manifest asset-IRI
   to taxon-IRI join preserved. Pack installation must not implicitly mutate
   a native/live graph or claim persistence. Mutable save/commit remains a
   separate requirement, not a prerequisite for read-only ontology use.
4. Declared route, capability permission, profile availability, memory budget,
   and snapshot/persistence semantics must match actual execution. Enforce
   admission budgets before allocation; diagnostic denial is not permission
   for a JS fallback. No language extension is established as necessary by
   the inspected source; the evidenced need is host/profile qualification.

### Acceptance cases for upstream and subsequent game integration

- On the approved revision/profile, the same admitted Q42 bytes decode to
  the same Quin fields and embedded lexicon terms on native and WASM paths.
- A VibeScript fixture invokes the actual engine host to query an asset's
  depicts edge, resolves the taxon term including namespaced Q42LEX entries,
  and SHACL-validates the known subject. Record the host route and artifact
  identities; a LocalHost empty list or game-owned substitute cannot pass.
- Denied capability, bad entry digest, malformed/truncated header or block,
  parity corruption, unsupported version, and budget exhaustion fail closed
  without losing the previously admitted view. Missing/duplicate semantic
  entry behaviour is explicit in the pack contract.
- Browser integration exercises the same manifest asset IRI/hash used for
  `.10d` identity. Digest/count verification alone is not this join proof.
- Existing native volume open/commit fixtures retain their declared rights
  and persistence semantics. No browser test is reported as a native save
  or durable graph transaction.

**Game-side status:** the duplicate/undefined pack-export call was removed by
source edit and re-read; compilation remains unverified. The Q42-only release
validator, semantic-loader allocation bounds, browser asset/semantic join,
artifact pin alignment, and native-vs-WASM pipeline targeting remain open.
The JS exporter still writes interim packs without semantics before native
repacking; do not publish those as completed ontology packs or claim that this
interim path eliminates Node. These are explicit follow-ups, not grounds to
rewrite the historical successful pack receipts.

### Single-asset UAT surface — existing capabilities and remaining metadata gaps

Timothy requested inspection of one asset and its RDF/metadata without a
whole-game compile. The inspection unit is one exported `.10d` manifest
entry, which may be a component rather than a complete organism. This does
not require scene generation or a whole HMC load: the checked-in WASM surface
advertises `asset_verify_10d`, `QualiaPortal.load_assets_colored`, encoded
texture admission, camera controls, and scene render receipts. Source/API
inspection is not proof of a successful rendered UAT session.

The browser host may select files, transport bounded bytes, show metadata,
and operate existing engine camera/render interfaces. It must not generate
geometry, infer taxa from appearance, parse Q42, or implement RDF/query logic
in JavaScript. Human dispositions are local unsigned notes, not automated
quality scores or proof of engine conformance.

The checked-in JS/TypeScript package surface does not advertise the newer
`taxon_binding_json`, `taxon_dwc_json`, or `SemanticSlice` exports present in
current game source. Its existing `run_program_bytecode` can request named
functions from the authored taxon fixture through LocalHost. This is a
source-term preview only: LocalHost inherits the unmasked `Host::hash_iri`
(`crates/vibe/src/bind/host.rs`), whereas the pack authoring host uses
`generate_60bit_token`. Its complex QuinRef JS output is a tagged debug value
(`crates/vibe-wasm/src/lib.rs`, `value_to_js`), not the pack's structured hex
Quin record. Do not advertise those preview hashes as matching Q42 or append
the host-packed integer Quin in JS. Runtime failure must show unavailable,
not trigger substitute semantics.

For Timothy to arrange upstream/public-surface qualification:

- A bounded `.10d` inspector should expose exact referenced texture digests
  and roles, semantic identities, provenance, material/submesh information,
  and authored bounds through the public WASM interface. The currently
  advertised verifier reports counts/bounds, not an exact per-asset texture
  dependency list; the initial viewer may admit the bounded shared manifest
  texture set but must label that scope.
- Qualify a Vibe metadata invocation against the same host/hash contract used
  by pack authoring, with structured lossless u64 fields and a declared
  source-vs-pack provenance distinction. This is host/package qualification,
  not an evidenced need to invent a new Vibe language feature.
- The G50 byte-backed semantic-view requirement above must enable actual
  Q42-decoded RDF associated with the selected asset. Authored RDF terms,
  manifest taxon declarations, reviewer-supplied RDF text, and pack-decoded
  terms are separate evidence sources and must remain separately labelled.

Acceptance cases (not executed): select one manifest entry; verify its byte
length and SHA-256 before engine ingestion; report the loaded package's actual
identity and any manifest mismatch; render only that geometry with explicit
texture-admission status; exercise camera changes without generating the
scene; retain original authored bounds in metadata despite orbit framing;
refuse corrupt/oversized input; prevent stale async selections from becoming
current; expose unavailable metadata bindings without fallback fabrication.
Actual image quality and taxonomic fidelity remain human UAT decisions.

**Corrected implementation boundary:** `web/fixtures/asset-review.vibe` now
owns review selection/admission, metadata/RDF projection, camera bounds and
human-note decisions. `eval_program_src` interprets it with the existing taxon
fixture. The browser adapter marshals data using keyed field lists because
Vibe record literals require identifier keys and manifest binding keys may
contain hyphens. Unlike that interpreter route, the inspected
`run_program_bytecode` argument converter accepts only primitive JS values
and maps objects/lists to Null (`crates/vibe-wasm/src/lib.rs`, `js_to_value`).
Do not claim direct structured argument support for that entry point.
No engine change, JavaScript semantic fallback, or game rebuild was made for
this correction. Browser execution remains unverified.

### Existing mutable Q42 journal — not a missing engine capability

`crates/qualia-core-db/src/q42/journal.rs` implements `.q42j` (magic `Q42J`,
version 1), with `Q42Journal` and `MutableQ42Session` exported by core `lib.rs`.
`wasm_bridge/dataio.rs` implements `WasmQ42Session` over that session using
`Cursor<Vec<u8>>`. The checked-in game JS/TypeScript surface already exposes
construction, staged Quin add/remove, commit/rollback, queries, tick rewind,
`export_journal_bytes`, and `load_from_bytes`.

This is positive source/API evidence for existing mutable-session machinery,
not a newly invented requirement. `load_from_bytes` takes journal bytes and a
flat 48-byte Quin snapshot; it is not a compressed Q42 volume decoder or an
embedded Q42LEX resolver. Exporting an in-memory journal is not by itself a
browser-restart/OPFS persistence receipt. Qualify the existing storage and
host integration before proposing new functionality. The asset viewer is
read-only and must not mutate an asset ontology merely because journalling
exists.

## GAP-12 — Interactive digital-twin assets

**Normative scope:** [Interactive digital-twin assets — per-asset requirements](assets/digital-twin-asset-requirements.md).
The DTA requirement IDs in that specification are the traceability contract.
This section is the upstream handoff for Timothy to allocate to the QualiaDB
agent; it does not authorise game-demo/ecopunk agents to edit the engine.

The deliverable is a high-resolution, interactive, semantically grounded
asset authoring/runtime path, with complete-object HMC preview and qualification
in the existing asset-review app. A single branch/wheel passing a container
check is not proof that a complete plant/vehicle or its lifecycle behaves
correctly. Rendering, physics, behaviour and operational-twin claims must be
qualified separately.

**Evidence scope:** the capability pointers below were inspected in the current
working tree, which includes unrelated/unapproved edits. They establish source
or public-surface presence, not a new release pin or native/browser conformance.
The prior single-asset browser run is evidence only for its exact compiled
package and tests; it does not qualify current source, HMC root preview, LOD,
physiology, destruction or replay. No new builds/tests were run for this spec.

### Implementation/qualification packets

| ID | Required capability and DTA coverage | Existing evidence / status | Upstream outcome and acceptance |
|---|---|---|---|
| G52 | Versioned asset-definition, semantic/physical metadata descriptor and public inspector. DTA-ID, DTA-SEM, DTA-RIGHTS. | **source-confirmed substrate + integration open** — `container_10d/manifold_identity_v2.rs` distinguishes stable entity ID, manifold state and integrity hint with typed field references; `q42/asset_envelope/` provides dataset provenance/licence envelopes; `material_section`/`provenance_section` exist. G49/G50 cover taxon/semantic slices. None alone is a complete interactive-asset descriptor. | Ratify a compatible schema/public Vibe inspector exposing root/part identity, type-vs-instance, typed quantities/units/uncertainty, physics/rig/resource links, rights and evidence. Bind to admitted package/version; actual pack-decoded RDF resolves selected root/parts. Qualify wrong identity/version, dimensional mismatch, unknown feature and restricted metadata cases. |
| G53 | Complete-object/component HMC graph, exact dependency closure and aggregate loading. DTA-PKG; DTA-UAT-01–03 and DTA-UAT-13. | **source-confirmed packaging + aggregate-contract qualification open** — core `bundle/`; `render/asset_package.rs::build_hmc_bundle` packages multiple `.10d` assets and de-duplicates verified textures; `resolve_hmc_texture_resource` checks references. Current game `GamePortal.load_hmc_scene` selects a bounded named scene using an asset table/index list; compiled `QualiaPortal.load_hmc_mesh` loads one entry. General root/member/transform/variant contracts are not established by these facts. | Reuse the actual HMC codec; provide versioned family/root/variant enumeration, component/attachment graph, exact selected-variant dependency plan, inspected manifest/index, transactional root admission/unload and Vibe bindings. Multi-model/behaviour families distribute an HMC release, including male/female variants where applicable, shared resources and explicit compatible sex/phenotype, stage, rig, LOD and behaviour combinations. Qualify a complete multi-part tree/vehicle, shared resources, component isolation, transform-once/unit agreement, missing members, cycles, conflicting IDs, corruption and stale replacement. Link range/cache work to G34, not a JS parser. |
| G54 | High-fidelity source to device-aware geometry/texture/rig/collision LODs. DTA-VIS, DTA-LOD. | **source-confirmed LOD substrate + profile integration open** — `render/lod_chain.rs` implements QEM/mesh-section chain APIs and Full/Eco/Reserve selection; computational geometry and material/UV modules exist. Module existence does not prove a canonical per-asset LOD manifest, every field's fidelity ladder, or browser switching. | Qualify compatible container/manifest representation, deterministic source lineage, detail/error envelopes, device budget selection, hysteresis, state/pick/attachment continuity and instancing. Enforce principled algorithmic/procedural generation (Voronoi/Delaunay cellular micro-surfaces, SDF implicit fillets, allometric scaling, spectral kernels) over brute-force static mesh/texture bloat. Demonstrate high-detail and constrained views of the same entity without changing authoritative state; expose unavailable levels instead of silently changing semantics. |
| G55 | Typed physical model, material facets and simulation/render coupling. DTA-PHY. | **source-confirmed primitives + model/host integration open** — `render/physics/{aabb,admission,joint,material}.rs`; `response/` delegates spring/thermal work to calculus; `crates/vibe/src/physics/materials.rs` has mechanical/thermal/optical/acoustic/chemical facets and part bindings. `Body::mass` uses box-volume × caller-unit density: this is not proof of measured organism/solid mass or full contact/continuum physics. | Audit/reuse existing solvers and units before extending them. Expose cited typed parameters, model/approximation selection, collision/joint/response bindings and coherent material state. Qualify units, physical bounds, contact, environmental forcing, conservation/error envelope and corresponding pose/sound/material updates for declared models. Do not claim every material facet is wired to every target. |
| G56 | Rig/clip authoring and state-linked animation with contacts, seeking and device variants. DTA-ANIM; G7/G36. | **source-confirmed skeletal substrate + family/profile qualification open** — `container_10d/skeletal_section/`, `skeletal_container.rs`; `render/skeletal/{player,pose,skin,authoring,activity}.rs`; `render/portal/scene_animation.rs`; public scene-animation surfaces exist. | Reuse motor/rig codecs and bounded player. Qualify stable joint/clip/attachment discovery, admitted root motion, interruption/blending, semantic picking, deformed normals/bounds/shadows, contact/event synchronization, reduced-motion and stage/LOD compatibility. Show behaviour-triggered animation in complete-object native/browser fixtures, not only clip-count metadata. |
| G57 | Semantic/physical damage, destruction, repair and fragment lineage. DTA-DMG. | **material-response substrate; destruction topology/contract open** — Vibe material facets include yield/response and stress-related readings; spring/thermal response is present. This inspection did not establish a complete fracture/debris/repair asset pipeline. Absence is not presumed beyond that inspected path. | Audit existing mechanics/geometry first, then expose damage causes/typed thresholds, component failure, bounded fragmentation/detachment, resource accounting, repair/replacement and disposal through Vibe/public engine APIs. Qualify cosmetic vs functional vs structural damage, lineage/contact changes, denied actions, repeatable replay and aggregate fragment budgets. A damage texture/clip cannot close this packet. |
| G58 | Individual growth, maturity, life-stage and phenology construct. DTA-LIFE; existing G47 accrual semantics. | **biophysical/temporal substrate; individual asset lifecycle integration open** — `specialized_libs/physics_simulation/population.rs::run_logistic_growth` models population count, not individual anatomy; G47 records accrual machinery; Vibe temporal/material/modeling surfaces exist. | Define species/instance-qualified stage transitions and drivers, geometry/rig/material/physics/behaviour correspondence, component/reproduction lineage, simulated-vs-observed time, resource constraints and irreversible loss. Qualify juvenile/mature/senescent and applicable seasonal states, regrowth/death, sourced sex/reproductive-form compatibility and LOD continuity. Family variant selection must not change an operational twin's individual traits without an explicitly qualified lifecycle transition. Do not substitute uniform scaling or a population curve for individual development. |
| G59 | Discoverable behaviour, affordance, functional-system and interaction contracts. DTA-BHV. | **authored Vibe/game behaviour and scheduling substrate; generic asset contract open** — current Vibe fixtures own drive/campaign/taxon rules; `PoetSnapshot`/invoke graph and logic routes exist; skeletal `ActivityTimeline` schedules cues. A cue is not proof of feeding, social interaction or machine throughput. | Expose parameterized Vibe-authored policies over exact admitted instance state: actors, prerequisites, capabilities, resources, outcomes, seeds and cancellation. Every digital twin representation (flora, fauna, funga, or constructed object) MUST embed a self-describing **Personal Behavioral & Affordance Ontology** declaring discoverable actions (`wot:ActionAffordance`, `schema:Action`), internal drives/states, and machine-readable preconditions/effects to enable direct **AI agent triggering, LLM tool-calling, and multi-agent scriptability** without client-side hardcoding. Reuse existing graph/logic/simulation kernels. Qualify eating/moving/social scenarios, evidenced shared vs variant-specific behaviour sets, and an operating/failing/repairing machine with state/animation/contact agreement and explicit denial. No JS rule fallback. |
| G60 | Definition/instance/observation separation; journal, replay, branches and durability. DTA-STATE. | **existing mutable-session machinery; asset/host/storage qualification open** — `q42/journal.rs` exports Q42Journal/MutableQ42Session; `wasm_bridge/dataio.rs` and compiled `WasmQ42Session` expose transaction/query/export/reload/rewind using `Cursor<Vec<u8>>`. These are not missing wholesale. | Integrate stable instance/base-package identity and qualified Vibe graph access. Reuse `.q42j`; demonstrate atomic state/lineage changes, deterministic reset/replay, isolated UAT branches, wrong-base refusal and explicitly qualified OPFS/restart recovery. Keep compressed-Q42 admission separate (G50) and never overwrite published asset packs. |
| G61 | Observations, sensor calibration, reconciliation and operational-twin validity. DTA-TWIN. | **related typed-field/reconciliation substrate; twin integration open** — MID2 typed field references are present; G46 records reconciliation machinery. No live referent/sensor-to-asset validation is established by this inspection. | Declare referent/source/units/calibration/time/uncertainty, freshness and dropout policies, model-observation alignment, and read-only vs explicitly authorised actuation. Qualify recorded-observation replay, divergence and held-out predictive validity where claimed. A realistic CG model is not automatically a measured twin. |
| G62 | Vibe-first high-resolution authoring, import, per-root compilation and reproducible packing. DTA-AUTH. | **source authoring/compilation substrate; complete workflow open** — computational geometry, `render/compile_10d`, `render/skeletal/authoring`, surface authoring and `render/asset_package` exist; game `pack-export` evaluates Vibe sources. This does not prove an integrated source-editor/rig/physical/lifecycle authoring workflow. | Provide public contracts for editable whole/part sources, approved import/conversion reports, parameter/identity/rights retention, derived variants and incremental compile of one asset/HMC root. Export deterministic validated artifacts without rebuilding the game. Bound scratch and preserve user work on cancellation; qualify source-to-preview revision tracing. |
| G63 | Extend the asset-review app to complete-object HMC and interactive conformance UAT. DTA-UAT. | **single-entry browser slice qualified; HMC/interactive scope open** — game `web/fixtures/asset-review.vibe` and existing WASM APIs passed seven checks on a banksia branch, metadata/RDF source preview, camera, swap, note, corruption and layout. No whole-object HMC, pack-decoded RDF, lifecycle, destruction or behaviour proof exists in that run. | Upstream supplies engine inspection/root/LOD/rig/state APIs and Vibe bindings; game-demo composes them into the existing app. Qualify `.10d` and HMC selection, family/sex/phenotype variant discovery where applicable, complete-root and part drill-down, actual RDF/physics inspector, compatible stage/rig/LOD/behaviour selection, animation/life/damage scenarios, reset/replay and reproducible captures. No full-game load/build prerequisite and no JS format/simulation/ontology implementation. |
| G64 | Object-associated audio/media, accessible presentation and device projection. DTA-MEDIA. | **material/media substrate; per-object/profile integration open** — acoustic/optical material facets exist; audio/media capabilities are separately described in the existing engine/media registers. Their presence is not a complete object-specific sound/event/rights path. | Reuse qualified media codecs/DSP/render capabilities; bind sources/calls/operating sounds to actor/action/state, explicit spectral or projection fidelity, spatial timing, caption/text/reduced-motion alternatives and bounded residency. Qualify missing/corrupt media and object unload without hidden emitters; do not treat synthetic sound as recorded evidence. |
| G65 | Admission, resource accounting, runtime profiles and recovery for complete interactive assets. DTA-RUN; G32/G34/G36/G37. | **existing limits/reader/runtime substrate; complete workload qualification open** — bounded geometry/rig/container code and governance/scheduler mechanisms exist; prior records distinguish source presence from browser pool/runtime qualification. | Enforce per-pass and aggregate CPU/GPU/media/transfer limits, bounded components/fragments/instances, caller-buffered hot paths, actual backend selection and compatible degradation. Demonstrate cancellation/device loss/unload, dirty updates rather than unrelated rebuilds, stale-load rejection and preserved state on device/LOD switches. Measure complete selected-object workloads; do not extrapolate kernel metrics. |

### Allocation and dependency order

1. **Contract and inspection first (G52/G53):** identify the approved engine
   revision/profile and existing APIs; ratify root/member/variant/identity and
   typed-property schemas. Capture what already works before any change.
2. **Authoring and static fidelity (G62/G54), with G63 UAT in parallel:** make
   one complete hero object exportable and previewable, with lower-resolution
   derivatives, exact resource closure and accessible metadata. Existing
   independent art improvement must not be blocked by unrelated engine gaps.
3. **Physical/rig/action coherence (G55/G56/G59):** qualify behaviour-to-state
   and state-to-animation/material/contact paths on that same object.
4. **Life/damage/state (G57/G58/G60):** add applicable transitions and bounded
   lineage/journal replay without changing immutable source identity.
5. **Operational twin and media/profile breadth (G61/G64/G65):** qualify real
   observations and claimed target coverage; do not retrospectively classify
   synthetic fixture results as real-world calibration.

Rights, truthful provenance, bounded execution and failure preservation apply
throughout, not only in the last stage. One complete tree and one articulated
mechanical assembly are useful initial fixtures; the normative specification
also covers animals, people, infrastructure, materials and scientific models.
Fixture choice and visual/scientific acceptance remain human decisions.

### Required handoff from the QualiaDB agent

For each packet, return:

- DTA/G IDs addressed; exact pre-existing implementation and chosen public
  API/format/host seam, including why extensions are necessary;
- source/asset/package/schema/rule versions and platform/feature availability;
- interfaces and reproducible authoring/preview instructions, plus explicit
  unsupported/not-applicable/unknown behaviours;
- focused positive/negative fixtures, actual native/WASM/browser results and
  resource evidence at the claimed scope; no invented receipts or pass claims;
- format/state migration and compatibility behaviour that preserves existing
  working assets, user changes and historical receipts;
- remaining integration/curation decisions that only Timothy can resolve.

Generic core capability work belongs upstream. Art/reference/species curation,
consumer fixture composition and the asset-review UI belong to ecopunk/game-demo.
Existing policies stay in VibeScript; core kernels/serialization stay in the
engine; necessary browser transport/presentation remains a narrow adapter.
This packet is not permission to substitute a convenient generic game, invent
missing ontologies, rewrite formats or silently make a source-only capability
claim into a shipped digital-twin feature.

---

## GAP-13 — Open-World Earth Substrate (STAC / COG / AusPIX / `osm2rdf`) & Unified Species-Physics Conformance (`G66`–`G70`)

Surfaced by the 2026-10-21 Open-World NSW Bush & Species-Physics Conformance review — see
[spatial-substrate.md](spatial-substrate.md) §4.1 & §5.1,
[../data-sources/australia.md](../data-sources/australia.md) §2A, §2B & §5B, and
[assets/digital-twin-asset-requirements.md](assets/digital-twin-asset-requirements.md) §2.1.

### Engine Capacity Assessment (2026-10-21): What `qualiaDB` + `etalon` Already Supports vs. Remaining Upstream Gaps

To answer the architectural question *"Did or does the engine lack the capacity to model a true 1:1 metric open world with unified species data and physics conformance?"* honestly against verified source in `C:\github\qualiaDB` and `C:\github\etalon`:

1. **Capabilities that ALREADY EXIST in `qualiaDB` + `etalon` (no excuse — consumer `ecopunk-terrain` failed to use them):**
   - **75+ Native Computational Geometry, Parametric CAD, NURBS, SDF & Botanical Kernels (`qualia-core-db::specialized_libs::computational_geometry`):**
     - **Vehicles & Manufactured CAD (`made:Artefact`):** [`wheel_panel::wheel_clearance_panel_into`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/wheel_panel/mod.rs) (thin body guards with circular wheel-clearance arches), [`authoring::loft_convex_sections_into`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/authoring_loft.rs) (closed convex-section CAD lofts), [`parametric_cad`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/parametric_cad.rs) (`bezier_eval`, `bspline_eval`, `nurbs_eval`, `loft_profiles`, `revolve_profile`, `tube_along_polyline`, `sdf_compose`), [`nurbs_fitting`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/nurbs_fitting.rs) (`nurbs_patch_eval`, `discretize_bspline_by_curvature`), and [`boolean_3`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/boolean_3.rs) / [`corefine_3d`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/corefine_3d.rs).
     - **Organic Fauna & Anatomical Surfaces (`bio:DarwinianTaxon`):** [`sdf_ops`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/sdf_ops.rs) (`sdf_capsule`, `sdf_rounded_box`, `sdf_smooth_union`, `sdf_metaballs`, `sdf_compose_smooth`), [`isosurface::marching_cubes`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/isosurface.rs), [`screened_poisson::screened_poisson_reconstruct_3d`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/screened_poisson.rs), [`remesh_3::isotropic_remesh`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/remesh_3.rs), [`anisotropic_remesh`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/anisotropic_remesh.rs), [`ddg_operators`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/ddg_operators.rs) (`mean_curvature_flow_step`, `cotangent_laplacian`, `vertex_normals`), and [`dual_quaternion_skinning`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/dual_quaternion_skinning.rs).
     - **Botanical Architecture & Crown Radiative Transfer:** [`space_colonization::space_colonization_3d`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/space_colonization.rs), [`tree_skeleton_3d::pipe_model_radii`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/tree_skeleton_3d.rs), [`sweep_extrusion`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/sweep_extrusion.rs) (`compute_bishop_frames`, `extrude_profile_along_path`, `swung_surface`, `deform_taper_in_place`), [`turtle_3d::execute_turtle_3d`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/turtle_3d.rs), [`botanical_hulls`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/botanical_hulls.rs), and [`canopy_radiation`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/canopy_radiation.rs).
     - *Root cause of poor models in `ecopunk-terrain`:* `scene_builder_10d.rs` did not call ANY of these 75+ `computational_geometry` modules in `qualiaDB` — it wrote a 5-sided cylinder loop (`push_tube` with `segs = 5`) from scratch and used broken scale numbers (`0.22` kangaroo vs `0.042` 4WD).
   - **Self-Contained `.hmc` Multi-Payload Container & Hardware Capability Manifests (`qualia-core-db::bundle` & `capability_manifests`):**
     - [`BundleWriter`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/bundle/mod.rs) / [`BundleReader`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/bundle/mod.rs) already packs arbitrary page-aligned entries by `(key, kind)` with SHA-256 + CRC-32C verification — supporting `.q42` ontologies/graphs, `.vibe` / `.qbc` VibeScript programs, `.wasm` application binaries, `.10d` models, `.m64` terrain, `.p64` weights, and `.emf` media in a single `.hmc` file.
     - [`capability_manifests`](file:///C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/capability_manifests/mod.rs) already models per-operation resource limits and execution backends (`scalar`, `simd`, `wgpu`, `cuda`, `wasm`, `cpu` fallback).
   - **1:1 SI Metric World Frame (`preserve_authored_frame`):** `QualiaPortal::set_preserve_authored_frame(true)` (`crates/qualia-core-db/src/render/portal/mod.rs:969`) already exists in `qualia-core-db` and bypasses `normalise_to_orbit_frame()`, preserving 1:1 SI metre vertex positions across multi-asset scenes.
   - **Metric 3D Physics, Geomechanics, Hydrodynamics & Botanical FSPM (`qualia-sim`):** `qualia-sim` already provides metric 3D rigid/articulated XPBD (`physics3d::XpbdWorld3D`), analytical SDF + B-Rep collision (`BroadphaseBvh`, `SdfCollider`), 6-layer Bekker/Mohr-Coulomb soil sinkage (`geomechanics.rs`), 3D Archimedes river buoyancy & shallow-water equations (`hydro.rs`), Pipe Model & stem elastic deflection (`metamer_fspm.rs`), 4WD Ackermann/suspension (`kinematics.rs`), and 3D capsule character stepping (`character.rs`).
   - **Living `.10d` Containers, Hardware GPU PGA `Motor` Skinning & Multi-Modal Culling (`qualia-core-db`):** `.10d` `PROC1` (allometric/Voronoi/SDF synthesis), `EONT1` (ontogenetic stages & affordances), `MRP1` (quantized morph targets), `SKL1` (PGA `Motor` dual-quaternion skinning), `MeshSlabArenaGpu`, and `WasmSliceQ42Reader` are implemented in `qualia-core-db`.
   - **Geodetic Transforms, Hilbert `.m64` Strata & Hermite Corridors (`qualia-geospatial`):** `GeodeticFrame` (`GDA2020`, `MGA55/56`, `AHD`), `xy_to_hilbert`, `M64StrataColumnRecord`, and `HermiteCorridorSegment` (`conform_terrain_height_to_corridor`) exist in `qualia-geospatial`.

2. **Remaining Upstream Engine Gaps (`G66`–`G70`) Required for Full Open-World Earth & Species-Physics Conformance:**

| ID | Need | Surfaced in | Status | Blocks |
|---|---|---|---|---|
| G66 | **Cloud-Optimised GeoTIFF (COG) IFD Parser & HTTP `Range: bytes=...` Tile Window Streamer** (`qualia-geospatial::geotiff`): parse real GeoTIFF/COG Image File Directories (`ImageWidth`, `ImageLength`, `TileWidth`, `TileLength`, `TileOffsets`, `TileByteCounts`, `ModelPixelScaleTag`, `ModelTiepointTag`, `GeoKeyDirectoryTag`, `GDAL_NODATA`) and fetch/decompress $512 \times 512$ `f32`/`i16` elevation, FABDEM bare-earth, SLGA 6-layer soil, and GEDI Canopy Height (`CHM = DSM - DEM`) windows via HTTP byte ranges into `.m64` and `.10d` `FIEL` | spatial-substrate §4.1, australia.md §2A, G1 | **upstream-partial** — `qualia-geospatial/src/geotiff.rs` currently wraps a pre-decoded `&[i32]` slice (`ElevationRaster`) and emits `.m64` Hilbert chunks, but lacks a real TIFF/COG IFD header parser and HTTP `Range` tile window fetcher | Direct streaming of NSW Spatial Services 1 m LiDAR DEM COGs, 30 m FABDEM, and 1 m GEDI Canopy Height COGs |
| G67 | **STAC JSON-LD / W3C DCAT Asset Resolver & `osm2rdf` / Geofabric WKT Vector Ingester** (`qualia-geospatial`): ingest STAC `Item` / `Collection` JSON-LD (`stac:hasAsset`, `dcat:downloadURL`, `eo:cloud_cover`, `proj:epsg`, `gsd`) and parse `osm2rdf` (`tourism=camp_site`, `highway=track`, `surface=unpaved`, `barrier=gate`) + NSW Geofabric (`ahgf:HydrographicFeature`) `geo:wktLiteral` `LINESTRING`/`POLYGON` features directly into `.q42` NQuins and `HermiteCorridorSegment` splines | spatial-substrate §4.1, australia.md §2B, G3 | **open** — `HermiteCorridorSegment` math exists in `qualia-geospatial/src/corridor.rs`, but STAC JSON-LD asset resolution and WKT vector-to-corridor ingestion are not yet wired | Automated NSW bush campsite, 4WD fire-trail, and Geofabric river network ingestion from RDF/STAC |
| G68 | **AusPIX rHEALPix DGGS Cell Hierarchy & Directional Neighbour Graph** (`qualia-geospatial::frame` / `dggs`): implement Geoscience Australia's AusPIX rHEALPix projection (`http://linked.data.gov.au/def/auspix`, resolutions R0–R15, `apo:Cell` URI minting/parsing, exact WKT cell boundary polygon, and `apo:hasNeighbourUp/Down/Left/Right` topology) | spatial-substrate §2, §4.1, §6; australia.md §2B | **upstream-stub** — `qualia-geospatial/src/frame.rs:146` currently computes a synthetic FNV hash (`to_dggs_cell_hash`) rather than the normative AusPIX rHEALPix cell ID and neighbour graph | Deterministic DGGS cell addressing between `.q42` TERN/ABIS queries and `.m64` terrain tiles |
| G69 | **Fail-Closed QUDT Species-Trait Dimensional & Biophysical Admission Gate** (`qualia-core-db::container_10d` & `SceneMeshAccum::append_asset_10d`): validate every admitted `.10d` organism or vehicle asset's authored bounding box (`AABB` in SI metres), tissue/material density, and `qualia-sim` mass/inertia against its `.q42` QUDT trait envelope (`AusTraits` / `AFD` / `AVONET` / `made:Artefact` specs per `DTA-ID-05`, `DTA-SEM-05`, `DTA-PHY-02`), refusing admission on dimensional mismatch or scale inversion | digital-twin-asset-requirements §2.1, spatial-substrate §5.1, australia.md §5B, G52/G55 | **upstream-partial** — `asset_verify_10d` checks binary section digests and finite coordinates, and `traits.json` / `.q42` hold species traits, but `append_asset_10d` does not yet cross-check mesh AABB dimensions against the taxon's `.q42` QUDT height/length/mass bounds | Structural prevention of scale inversions (e.g. oversized fauna vs 4WD vehicles) and ungrounded physics masses |
| G70 | **Multi-Kilometre 1:1 SI Metric Camera Frustum & Logarithmic/Cascaded Depth Range in `QualiaPortal`** (`qualia-core-db/src/render/portal/camera.rs` & `mod.rs`): widen `CAMERA_FAR_PLANE` (currently hardcoded to `200.0`) and `zoom.clamp(0.35, 48.0)` when `preserve_authored_frame == true` so 1:1 metric open-world scenes support both $0.05\text{ m}$ close-up botanical inspection and $15,000\text{ m}$ ($15\text{ km}$) mountain horizon distances without frustum clipping | spatial-substrate §5.1, digital-twin-asset-requirements §2.1 | **upstream-partial** — `set_preserve_authored_frame(true)` exists (`portal/mod.rs:969`), but `camera.rs:19` hardcodes `CAMERA_FAR_PLANE: f32 = 200.0` and `camera.rs:58` clamps `zoom` to `48.0`, which clips any 1:1 metric terrain beyond 200 metres | True 1:1 SI metric open-world rendering in `QualiaPortal` without unit-cube compression |
