# Interfaces — the explore ↔ author loop (Chora & Poet)

**Status:** draft v0.2 · updated 2026-10-24 with actual local Poet/Craft workspace
mapping. Earlier verified-source descriptions below retain their original scope,
not a current end-to-end native qualification claim.

**Current production specification:** [production-toolchain-spec.md](assets/production-toolchain-spec.md)
maps `poet` and all eleven Craft repositories to actual roles/evidence and needed
native 3D/rig/material/graph/HMC handoffs. Poet's current `native/` is a Vibe CLI;
LightCraft is photo/RAW processing; FilmCraft is an NLE; CADCraft's inspected
model lacks full 3D solids/surfaces/meshes. Do not infer production capability
from the historical `*_studio` labels below. Canonical Poet changes belong in
`C:/github/poet`, not the retained `qualiaDB/crates/poet` copy.

**Unified-interface clarification:** the Craft apps are extraction sources for
one reusable library collection under `C:/github/craft`. Their editing
functionality belongs in **Poet's** tool chest, content surfaces, panels,
inspectors and timelines. Preserve/reconcile existing Poet behaviour and native
host services. [poet-craft-consolidation.md](poet-craft-consolidation.md) records
the current source/public-UI findings, target layout and staged migration;
standalone Craft GUIs are not the intended authoring interface.

**Framework and environment:** the [controlled framework/launcher contract](webizen-environment-framework.md)
places shared UI and native/WASM adapters in **Aether**, the controlled
`rust-app-framework` fork, launcher/service lifecycle in Webizen Desktop, and editing in Poet.
WASM versions use authorized native capabilities where required. The older
host/source descriptions below are recovery evidence, not a mandate to retain
the failed desktop UX or a webview as the native renderer.

**Chora name and code boundary:** Timothy identifies **Chora** as the separate
Servo-derived engine at `C:/github/chora`. The “Chora”/`chora_canvas` exploration
references below describe historical application/source concepts; their actual
relationship to that engine must be mapped, not assumed. The
[Chora engine contract](chora-engine-boundary.md) keeps its source/workspace and
license provenance separate, with generic integration and host-owned adapters.
No historical name establishes permission to merge engine and application code.

**The interface architecture is two-sided over one substrate:**

- **Chora** — the exploration surface: the spatio-temporal commons *canvas*
  (inside the WellFair host shell), WASM & native.
- **Poet** — the authorship/contribution environment: tool-chest + capability
  invocation + harmonised native and WASM authoring surfaces.

Explore finds; author contributes; both produce governed artifacts over the
same quins/volumes/identifiers substrate.

---

## 1. Chora — the exploration canvas (verified)

`chora` is **the spatio-temporal commons canvas inside WellFair** ("Peace
Infrastructure") — one area within the rights-grounded personal vault/host
shell, reached via `WebizenHostApi`. Not a standalone app bolted on: it lives
inside the vault context, so governance and identifiers are ambient.

**Flagship worlds** (`flagship_worlds.rs`) — seeded `CanvasWorldConfig`s:

| World | Layers | Governance norm (baked in) |
|---|---|---|
| **`q42:world:biosphere`** | `CanvasLayer::Biosphere` (`adapter://gbif/v1/occurrence`) + global DEM terrain | *"Biosphere outputs are Hypothesis under F/A — never ground truth"* — epistemic honesty is a **world norm** |
| `q42:world:council-sydney` | council open data + OSM buildings | placement requires council placement right |
| `q42:world:sdg-dashboard` | infosphere metrics + council context | dashboard norms |
| `q42:world:history` / `q42:world:glam` | historical & GLAM layers | stratum-scoped |

`biosphere_world` is **ecopunk's home surface** — its layer-stack model
(`CanvasLayer` variants × `WorldStratum` + `CanvasNorm` rules + origin +
`temporal_range`) is exactly the governed-layer contract from map-layers.md:
each layer declares endpoint, stratum, and norm.

**Layer subsystem** (`layers/`): `catalog`, `mesh_gen`, **`nasa_gibs`**
(satellite imagery already integrated), **`starfield`** (celestial backdrop —
connects to the earth-space register's celestial layer).

**Canvas API** (`api.rs`): `query_region` (geospatial), `RenderSurfaceDescriptor`
(backend=WebGPU), `SpatiotemporalQuadTree`, `SpatialSyncCell`/`PlantedAsset`
(asset placement + sync), `spawn_decay_alpha` — exploration is over the
*same* governed spatio-temporal machinery, not a separate map library.

## 2. Poet — the authorship environment (verified)

`poet` = "Tool-Chest, Manifolds, and Browser Preview":

- **`tool_chest::core`** — the authoring model: `Manifest`/`ManifestChain`/
  `ManifestTool` (chained tool invocations = *recorded pipelines*),
  `ConstructSeed`/`ConstructSource`, `ContextRef`, `IntentBus` +
  `IntentReceipt`/`IntentStatus` (**authored actions produce receipts**),
  `ManifoldSeed`/`ManifoldParticipant`/`ManifoldSociality`,
  `OntologyClass/Module/Property` (ontology authoring in-surface),
  `DockPosition`, `ActionType`.
- **`browser/`** — a full browser app surface: `chora_canvas` (canvas embedded
  in authoring), `canvas_state`, `command_palette`, `checkpoint_panel`,
  `agreement_views`, `construct_shelf`, accessibility, and per-domain
  **`chain_actions`** — `chem_`, `audio_`, `audio_fx_`, `cg_` (+live chains),
  `calc_`, `constr_`, `asset_`, `agent_`, `anim_`, `anim_live_` — i.e. the
  authoring UI is *already organised by domain chains*.
- **`keep_volume`**, **`vibe_host`** (frozen `vibe-host-0.1` four-op surface).
- **`poet-cli`** alongside for non-browser authoring.

**`poet_host/` in core-db** — the capability bridge the UI calls:
`bridge`, `catalog` (+`catalog_ttl`), `scan` (capability discovery), `values`,
and **`invoke/`** — invocation organised by domain: agent, audio,
biosignal, chat_graph, chemistry, clinical, **cooperative**, crypto, docs,
econ, engineering, geometry, governance, graph, hypermedia, inference, logic,
manifold, math, medical, ml, net, nlp, render, research, runtime, sampler,
science, sheet, social, stats, vision. Plus `cosmic_bind`, `hid_seam`,
`asset_bind`/`asset_store`, `poet_shell`, `args`, `ids`, `coverage`.

## 3. The loop ecopunk needs

```
CHORA (explore)                    POET (author)
─────────────                      ─────────────
biosphere_world layers    ──►   contribution kind detected
  "what is here, when,           (observation / ID / prep /
   under which norm"               chemistry / literature)
        │                              │
        ▼                              ▼
  gap/error/contest found    ──►   chain_action → invoke/<domain>
  (misidentification,                → IntentReceipt (recorded act)
   missing record)                   → contribution quins + labour quin
        │                              │
        └────── updated pack ◄──  manifest/compile → .hmc
              re-seeded into             (QBDL bundle)
              the world
```

Contribution-chains' kinds map onto invoke domains directly:
observation→`vision`, preparation/use-knowledge→`research`+`docs`,
chemistry→`chemistry`, literature→`research`+`hypermedia`,
linguistic→`nlp`, governance acts→`governance`/`cooperative`.

## 4. Wiring work implied ("update & wire Poet")

1. **Domain coverage** — `invoke/` needs an ecopunk/biosphere domain surface
   (occurrence submission, specimen media attach, taxon proposal,
   custodial-consent flow) — likely as new invoke module + chain_actions.
2. **Pack emission** — tool-chest `ManifestChain` → pack compile (QBDL
   `.hmc` + `.q42` + `.10d`/`.p64` entries) — is the manifest→bundle step
   implemented or spec? **Verify.**
3. **Chora→Poet handoff** — `chora_canvas` exists inside poet's browser tree;
   define the "found in canvas → open in author context" contract (ContextRef
   carries the selected entity/layer/point).
4. **Profile targeting** — author surface spans profiles: simple contribution
   (webcivics) vs scientific work (scientific) vs asset authoring (portal/
   native); each chain_action declares its minimum profile.
5. **Receipts → labour quins** — `IntentReceipt` should emit the
   `labelledBy`/`wasGeneratedBy` provenance quins so authored acts feed the
   contribution/oblication-cost graph.
6. **CML documents** — `cml_document.rs` in browser surface: the CML
   (civic markup) path likely carries authored declarative content — wire
   pack-section authoring through it.

## 5. What each surface must never do

- **Chora**: never render a proposal as fact (world norms enforce);
  never expose restricted-lane data outside its lawful view.
- **Poet**: never emit an unattributed contribution; never let a chain
  publish to commons lanes its sources' licences don't permit (union-of-
  obligations check at manifest time); never let a machine proposal
  self-ratify.

## 6. The Poet/Craft suite — historical labels and current map

Poet is the intended unified authoring interface; the actual **`poet-*craft`
repositories** are extraction sources for per-medium library/editor capabilities
(each currently has its own repo/conventions — several carry independent rule systems
such as studiorules/never-crash + clean-room asset rules):

Current names: `poet-artcraft`, `poet-cadcraft`, `poet-deckcraft`,
`poet-filmcraft`, `poet-gridcraft`, `poet-lightcraft`, `poet-pdfcraft`,
`poet-photocraft`, `poet-soundcraft`, `poet-vectorcraft`, `poet-wordcraft`.
The old `art_studio` / `film_studio` etc. names are aliases in prior planning.
Native engines/commands support the proposed reproducible authoring chain through
Poet; GUI controls and specialised content surfaces must be ported into that
same interface. The native capabilities, formats and source handoffs still
require qualification. A capability's missing interface remains
upstream work, not permission for a JS controller or private exporter.

For ecopunk this is the **asset-production chain**: hero assets, cinematic
renders, field-guide documents, audio sonifications, CAD-derived anatomy —
authored in studio tools, bound into packs through Poet. Uplifting model
quality = uplifting what these tools produce, plus agent-friendly authoring
(the user's current upstream direction) so contribution chains can be
agent-assisted while remaining human-ratified.

## 7. Open questions

1. Does `poet_host` have a stable external API boundary (versioned) that
   ecopunk chains should target, or do we code to `invoke/*` module surface?
2. `vibe_host` four-op surface — is VibeScript authoring part of the Poet
   contract for pack-logic authoring (per game-demo's modality adoption)?
3. Flagship world → pack binding: should `q42:world:biosphere` declare
   *required pack IDs* rather than raw `adapter://` endpoints (so the world
   consumes reviewed packs, not live adapters)?
4. Multi-user authoring sessions — `ManifoldSociality`/`ManifoldParticipant`
   suggests co-presence semantics; define the cooperative-authoring story
   (links to cooperative-layer.md).
