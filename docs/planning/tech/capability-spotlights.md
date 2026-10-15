# Capability spotlights — concrete scenarios, traditional vs QualiaDB

**Status:** draft v0.1 · each spotlight = one goal, the traditional-stack path,
the QualiaDB path (named modules), and an evidence note. These are the
*illustrations* behind `engine-comparison.md` — use them to sanity-check
interface/spec decisions. Status tags as in the capability map.

---

### S1 · "Show occurrences of this threatened species — but lawfully"

- **Traditional:** SPARQL query → application code applies licence filters,
  generalizes coordinates (write it yourself), enforces access tier (your code
  again). The graph returned rows; governance is your problem.
- **QualiaDB:** occurrence quins carry licence + sensitivity class in
  `context`/`metadata`; `entity_view::rights_filter` projects per-observer;
  `NetworkDisclosureRegistry` gates any egress; coordinate generalisation is a
  pipeline step recorded as provenance; `explicit_denial_guard` honours an
  owner's refusal fail-closed.
- **Evidence:** `entity_view/`, `geospatial/adapters/mod.rs`,
  `canvas_rights.rs` — **verified**.

### S2 · "A plant's medicinal use — without flattening it into a factoid"

- **Traditional:** a literal or a triple `plant → treats → condition`; hedging,
  source, and cultural context are modelling conventions the query layer can't
  see. Your UI decides what counts as a claim.
- **QualiaDB:** the claim carries an epistemic certainty band
  (assertive↔doxastic axis), RDF-star qualification, provenance anchors; if it
  reads as a health directive, `epistemic_boundaries::SocraticDegradation`
  degrades presentation to an evidence map + prompts (`BIO_DISCLAIMER`,
  `forbids_definitive_classification`). The teaching's language term, holder,
  and TK conditions stay attached.
- **Evidence:** `epistemic.rs`, `epistemic_boundaries.rs` — **verified**.

### S3 · "Where could this species live under climate scenario X?"

- **Traditional:** export occurrence + raster data → Python (`scikit-learn`/
  `maxent`) → re-import results (if ever). Model version, data licence, and
  run identity live in a notebook, not the graph.
- **QualiaDB:** `solvers/learning/gaussian_process` + `statistics` over the
  same governed records; climate rasters via `opendap_adapter` (consent-gated);
  results encoded as provenance quins (`computational_economics::ontology_bridge`
  pattern); the run packaged as a `semantic_instrument` with an attestation.
- **Evidence:** `solvers/learning/gaussian_process/`, `opendap_adapter.rs`,
  `semantic_instruments/` — **verified** (instrument packaging **[design]**).

### S4 · "Align ALA's vocabulary with TERN's — without auto-asserting equivalence"

- **Traditional:** run LogMap/AML externally → a mapping file → a human
  eyeballs it → someone writes the triples back. The "reviewed" bit is social
  convention.
- **QualiaDB:** `ontology_align` produces graded `closeMatch` proposals whose
  type is `RequiresHumanReview`. **The compiler cannot emit `exactMatch` —
  there is no constructor.** Ratified mappings carry the ratifier + evidence.
- **Evidence:** `solvers/ontology_align/correspondence.rs` — **verified**.

### S5 · "An engineered organism — living thing *and* regulated lineage"

- **Traditional:** one entity, two vocabularies, joined by a note.
- **QualiaDB:** dual membership via DL subsumption (multiple inheritance over
  the subClassOf DAG); the engineering act is an event quin; patent/biosafety
  instruments attach as `jural` relations (Claim/Power/Immunity…), checked for
  unmet correlatives; OGTR register data = a governed source like any other.
- **Evidence:** `dl.rs` (multi-inheritance), `jural.rs` — **verified**.

### S6 · "This border existed from 1901–1911, was replaced, and still matters"

- **Traditional:** a named-graph-per-version convention + a date literal; the
  "which border was valid when" logic is yours.
- **QualiaDB:** border = (kind, geometry, validity interval, authority); Allen
  algebra + LTL over validity; RCC8 for border-to-border relations; the
  quadtree indexes (x,y,t0,t1) natively; superseded versions remain first-class.
- **Evidence:** `spatio_temporal.rs`, `interval_reasoning.rs`,
  `spatial.rs` — **verified**.

### S7 · "Encyclopedia pages that differ by who is looking"

- **Traditional:** role flags in the app → two code paths → drift.
- **QualiaDB:** one entity, `entity_view` projections per observer/context;
  deontic verdicts decide what each view may contain; Indigenous-knowledge
  gates ride `capacity`/`steward`/`epistemic` quins; the page is a QApp whose
  panes bind to views — the rules are in the data, not the renderer.
- **Evidence:** `entity_view/`, `qapps` spec — **verified** (QApp binding
  **[doc]**).

### S8 · "A pack users can verify — data, app, geometry, licence, all of it"

- **Traditional:** zip + a README licence + a detached hash (if you're
  disciplined).
- **QualiaDB:** `.hmc` bundle: page-aligned entries (each slice *is* the file),
  CBOR index, per-entry SHA-256; `.q42` volumes carry parity + Merkle roots +
  `asset_envelope` (upstream releases, toolchain versions, licence policy —
  unknown licence fails closed); signature/VC attaches; `source_audit` +
  `text_span` trace records to sources.
- **Evidence:** `bundle/`, `q42/asset_envelope/`, `query/source_audit.rs`,
  `text_span/` — **verified**.

### S9 · "From a cell to a continent in one scroll"

- **Traditional:** three different platforms (microscopy viewer, map, model
  viewer) and a demo video.
- **QualiaDB:** `ReferenceFrame` hierarchy (parent + scale) + `scale_continuum`
  LOD transitions were *built* to bridge "micro-verse ↔ macro-verse"; `.10d`
  fields carry molecular/terrain data; bioinformatics at the bottom,
  geodetic/DEM at the top.
- **Evidence:** `reference_frame.rs`, `scale_continuum.rs`,
  `terrain_pipeline.rs` — **verified**.

### S10 · "Run it on the user's device, offline, with their private records"

- **Traditional:** impossible by architecture — the browser is a client.
- **QualiaDB:** a WASM profile (e.g. `wasm-scientific`/`portal`) runs the
  engine in-browser; the user's Sanctuary volume stays local; commons packs are
  queried *into* it; `entity_view` + deontic verdicts govern what renders;
  `wasm_edge` can offload/federate when permitted.
- **Evidence:** `wasm_bridge/` exports, `render/portal/`,
  `wasm_edge.rs` — **verified** (profile composition **[doc]**).

### S11 · "Field audio → identified species, inside the pack"

- **Traditional:** export to an audio pipeline (librosa + model) → re-import
  labels.
- **QualiaDB:** `audio/` DSP (STFT/CQT/HRTF, spectral sheets) + inference
  runtimes (GGUF/WASM kernels incl. `mc8_wasm`) + `audio_sidecar_link` binding
  results to the semantic record; the sonification layer (`acoustic_plane`)
  can render graph state as audio for accessibility.
- **Evidence:** `audio/`, `gguf_bridge/wasm_cpu`, `render/portal` acoustic
  imports — **verified**.

### S12 · "A community decides who may see a sacred site record — collectively"

- **Traditional:** an admin role; at best an approval workflow table.
- **QualiaDB:** `StewardshipContract` — M-of-N steward quorum bounded to the
  spatial domain; `validate_steward_unlock` on deontic `OP_PERMIT`; denials via
  `explicit_denial_guard`; the decision record is itself a governed quin with
  provenance (who held standing, under which axiom set).
- **Evidence:** `stewardship.rs`, `steward.rs` — **verified**.

### S13 · "Knowledge work that pays its cost, then becomes free"

- **Traditional:** a licence text + goodwill.
- **QualiaDB:** `value_flow` — `commons_cost` (production + *capped* ROI),
  per-use royalties into a pool, `is_commons_discharged` flips the work to
  globally-free at threshold. Extraction-resistant by construction; the
  accounting is integer arithmetic in the graph.
- **Evidence:** `value_flow.rs` — **verified**.

### S14 · "Ask why the answer is what it is"

- **Traditional:** query log + the app's word for it.
- **QualiaDB:** receipts — inference receipts, `PlaybackReceipt`s, `LayerFetchReport`s
  (incl. `ConsentDenied` — an honest "no"), `graph_proof`, breach records w/
  `prov:wasDerivedFrom`, evidential (μ,λ) values on claims.
- **Evidence:** `query/graph_proof.rs`, `meta_deontic.rs`,
  `adapters/mod.rs` — **verified**.

---

**Pattern across spotlights:** the traditional path pushes the hard part into
ungoverned application code; the QualiaDB path keeps the hard part *inside the
data substrate*, where it inherits provenance, licence, and evidence. That is
the property ecopunk's 50-year corpus actually needs.
