# Product surfaces — the end-user applications

**Status:** draft v0.1 · the products users actually touch, over the Chora/
Poet substrate. Each declares its mode (native/std-web), minimum WASM profile,
and delivery targets — per wasm-edge §6's participation-tier rule.

---

## 1. The product register

| Product | What it is | Mode / min profile | Substrate |
|---|---|---|---|
| **Encyclopedia** | entity-first reading surface — a taxon/place/compound/artefact page rendered from governed quins: names in-language, images, distribution, uses, provenance-visible claims | native + std-web / `wasm-webcivics` | `entity_view` rights-filtered views — *same entity, different lawful view per observer* |
| **Explorer** | the canvas: Chora `biosphere_world` + layer families; space×time×scale navigation (zoom out → biome; zoom in → tissue → cell → molecule) | native `portal` preferred / `wasm-scientific` | `query_region` + `SpatiotemporalQuadTree` + scale-continuum + AR layers where present |
| **Educational / guided** | curriculum-shaped paths through the graph (lesson = sequence + activities + assessment hooks) | webcivics | pack-declared learning paths; `q`>0 sandboxing for exercises |
| **Field / AR** | in-situ: anchored annotations "in-air", species proposals from live capture, offline-first | native mobile / edge profile | `ar_anchor` + location triggers + observation→Poet chain; offline packs |
| **Cinematic / linear** | non-interactive deliverables — documentary sequences, museum displays, broadcast assets from the same governed corpus | offline render / no runtime profile | max-fidelity variants (style-and-scale §8 film/TV class); poet-filmcraft production |
| **Sanctuary-side companion** | the personal join: *my* health/place/interest meets the commons — pathology-vs-phytochemistry queries run on-device, never server-side | native / scientific | Sanctuary volume + downloaded packs + in-browser kernels |

## 2. Cross-surface invariants (all products share)

- **Honesty surfaces everywhere** — modelled vs observed vs captured labelled;
  certainty bands visible (not flattened to fact-tone); world norms render
  ("biosphere outputs are hypothesis under F/A").
- **Provenance is reachable** — every claim/asset can surface its chain to the
  curious user ("where does this come from" is a UI path, not a hidden log).
- **Sensitivity lanes enforced per observer** — `entity_view` rights-filter;
  a restricted-lane item simply isn't in your lawful view — no greyed-out
  teases.
- **Contribution one step away** — every surface can hand off to the Poet
  contribution chain (found an error → propose a correction; captured a
  specimen → propose an observation).
- **Child-appropriate modes** — where products serve minors, interface modes
  honour guardianship arrangements (cooperative-layer §6; `capacity` +
  observer-class gating).

## 3. Product-specific notes

- **Encyclopedia**: page = projection of quins (not a CMS record); alternate
  observer classes (public / researcher / custodial) are explicit view
  switches where standing permits; concept-in-language terms render as
  language-tagged entities, not translated-flattened labels (SOC-AX-9).
- **Explorer**: scale-continuum transitions must remain honest — a zoom from
  "forest" to "cell" crosses *pack boundaries*; surfaces declare when detail
  below a rung is modelled vs observed.
- **Educational**: learning paths are *pack sections* (versioned, licensed,
  attributing); assessment hooks produce records in the learner's own vault —
  ecopunk does not hold grades.
- **Field/AR**: elevation-aware anchors; placement rights via canvas_rights/
  location triggers; visibility classes (public/custodial/private notes);
  contested anchors are records.
- **Cinematic**: deterministic render receipts (seed + input digests + model
  versions) make a film's frames reproducible — the "making-of" provenance is
  itself a pack artifact.
- **Sanctuary companion**: the defining product of the human-centric claim —
  the private join never leaves the device; results may cite commons data
  without exfiltrating the private side.

## 4. What each needs before it can ship (dependency check)

| Product | Hard dependencies (gap register) |
|---|---|
| Encyclopedia | G16 namespace, G17 review lanes, G18 band-rendering, ontology registry extension |
| Explorer | G1 raster→FIEL (some layers), G7 browser animation, G21 pack-bound worlds |
| Educational | encyclopedia first + learning-path section spec (eco-spec) |
| Field/AR | G22 anchor→layer pipeline, offline pack format confirmed |
| Cinematic | G26 hero assets, filmcraft maturity, spectral-truth assets (G28) |
| Sanctuary | wasm-scientific stability, pack distribution (G11–G15) |

## 5. Open questions

1. Product→pack licensing: does an encyclopedia render carry forward
   per-record attribution visibly (likely yes — render-time obligation check).
2. Which product leads? (Explorer likely first — it's closest to game-demo's
   existing canvas machinery and proves the pack pipeline end-to-end.)
3. AR product: standalone app vs mode within Explorer? (likely mode — the
   canvas is the renderer).
4. Assessment records: learner-vault schema ownership (user's own WellFair
   vault presumably — verify).
