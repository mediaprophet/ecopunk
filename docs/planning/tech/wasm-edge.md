# The WASM / edge dimension — capabilities where the data lives

**Status:** draft v0.1 · the browser/WASM story deserves its own doc because it
is the difference between "the graph answers queries" and "the governed engine
runs wherever the user is". Everything here verified against
`qualia-core-db` feature gates + `wasm_bridge` + `render/portal` this survey.

---

## 1. The profile ladder (verified `Cargo.toml` features)

| Profile | Contents | For |
|---|---|---|
| `wasm-webcivics` | logic + Solid/CML + JSON-LD/SHACL/modal/SPARQL — **no GPU/LLM** | decision-evidence, civic QApps |
| `wasm-ontology` | ontology MCP kernel (exclusive) | ontology tooling in-browser |
| `wasm-logic` | logic + `wasm-scientific` | reasoning surfaces |
| `wasm-scientific` | + `gpu-runtime` — the science stack | domain compute in-browser |
| `portal` | logic + science + **WebGPU render** | full browser engine |
| `wasm-llm` | + browser model runtime | local inference |
| `wasm-full` / `wasm-playground` | everything + playground | explorer/devtools |
| `wasm_simd` | SIMD128 kernels | hot-path width |

Gating is *applied*, not declared — this survey counted **469 `gpu-runtime`,
406 `wasm-scientific`, 57 `wasm-full`, 52 `wasm-logic`, 44 `wasm-webcivics`,
39 `portal`, 22 `wasm-llm`** `#[cfg(feature=…)]` sites in `src/`.

## 2. What the browser actually gets (`wasm_bridge/` — the JS surface)

Always-on: `semantic`, `logic`, `dataio` (+ `compute` under scientific/webcivics).
Under `wasm-scientific`: **`bio` (bioinformatics), `chemistry` (organic chem),
`medical` (clinical formulae/records)**. Under `engine/`: **cas, crypto, exact,
graph, linalg, numerics, stats (+stats_regression), transforms, units** — i.e.
the solver suite is exported to JavaScript, feature-gated per profile.

Plus: `spatial_wasm.rs` + `geometry_wasm.rs` (portal hot paths),
`wasm_playground.rs` (JSON wrappers for browser demos), `wasm_edge.rs`
(browser-edge router — **topological pruning, capability discovery, federated
swarm offloading**), `gguf_bridge/browser` + `wasm_cpu` + `mc8_wasm` kernels
(**browser-side model inference**, quantized).

## 3. The browser renderer is real

`render/portal/` is a canvas/WebGPU surface: `body_scene`, `camera_lens`,
`capabilities`, `cpu_mesh`, `exposure`, `graphics` (+ a `graphics_backend.vibe`
shader source), `paint`, `recovery`, `scene_animation`, `scene_mesh` (+tests),
`texture_upload` — and it already consumes the audio layer
(`acoustic_plane`/`acoustic_sab`/`audio_sidecar_link` imports at the top of
`mod.rs`), so **sonification of graph state is part of the portal** — an
accessibility-and-immersion affordance semantic stacks don't contemplate.

`render/hud/` ships bounded WASM canvas documents (panels, meters, semantic
markers, mirror widget); `render/skeletal::SkeletalPlayer` is allocation-free
playback — **verify its portal wiring before promising animation in-browser**
(capability-ledger discipline).

## 4. What deliberately stays native (fail-honest boundaries)

- `solvers::qpu` — std+tokio, `#[cfg(not(wasm32))]`.
- `legal_compose::marginal_harm`/`proportionality_met` — native-only (CAS
  dependency) — *proportionality reasoning is a native capability*.
- Blocking adapter fetches — `fetch_region` refuses on wasm32 ("call
  `fetch_region_async`"); the async path returns parsed provenance quins.
- `kg_embedding::train` — artifact-producer; in-browser you *score/predict*
  shipped tables, you don't train.
- `compute_bridge` — native registry is full; WASM registers a narrower set
  (WebGPU-or-CPU) — by design, CPU reference kernels always exist.

**Design rule that falls out:** an interface declares its minimum profile; a
capability declares where it can run. Ecopunk specs should carry both labels —
don't promise in-browser what the profile ladder says is native.

## 5. Why this changes the architecture (vs traditional stacks)

| | Traditional semantic stack | QualiaDB |
|---|---|---|
| Browser role | thin HTTP client → endpoint | **runtime** — engine executes locally |
| Private data | must round-trip to the server holding it | Sanctuary volume stays on the device; commons data is queried *into* it |
| Offline | none | volumes + packs are files; the engine runs without a network |
| Edge devices | gateway proxies | same engine, narrower profile (mesh/field nodes plausible) |
| Federation | federated SPARQL (query shipping) | `wasm_edge` capability-discovery + offload — *compute* moves, not just queries |

Concretely for ecopunk:

- **Field use** — an identification instrument or site-survey QApp can run
  offline on a device, sync volumes later (QDNF replication; consensus modality
  knows obligations survive partitions).
- **Privacy-first features** — the pathology-vs-phytochemistry join *runs on
  the person's machine*: their Sanctuary volume + downloaded commons packs +
  in-browser scientific kernels. The server never sees the join.
- **Sovereignty by deployment** — a community-controlled pack can be served
  from community infrastructure *and* executed client-side; the stewardship
  verdicts travel in the data, not in a policy server they don't run.
- **Interfaces inherit the ladder** — encyclopedia (webcivics/portal), explorer
  (portal), scientific workbench (scientific/llm), modelling studio (native +
  portal).

## 6. Participation *is* deployment (the architectural consequence)

Much of this ecosystem's functionality **requires the user to be running the
foundations** — there is no "free hosted service" tier the way conventional
platforms assume. This is deliberate, and it follows from the `spc:` axiom:
*each agent is their own platform provider*. The consequence for ecopunk
planning:

- **The profile ladder is the on-ramp ladder.** Participation isn't binary
  (run the whole engine or nothing) — it's progressive:

  | Participation tier | Profile | What they can do |
  |---|---|---|
  | **Consume** | any viewer / packs + webcivics | read packs, query local volumes, use interface QApps |
  | **Participate** | `wasm-webcivics` | contribute observations + corrections, hold volumes, run decision-evidence/civic QApps |
  | **Host** | native (or org server) | serve packs, run adapters, relay QDNF, hold the commons |
  | **Compute** | `wasm-scientific`/`wasm-llm`/native | lab analysis, classifier inference, local training artifacts |

- **`wasm-webcivics` is the for-purpose entry point** — logic + Solid/CML +
  JSON-LD/SHACL/modal/SPARQL with **no GPU or LLM dependency**. That's a
  deliberate fit for non-profit and for-purpose organisations: it runs on
  ordinary hardware, stays verifiable, and carries the governance stack
  (deontic/jural/consent) without requiring the compute tiers. A community
  organisation can hold custodial volumes, adjudicate proposals, and run
  civic QApps without ever owning a GPU.

- **Human-centric is a property, not a posture.** The profile doesn't remove
  the human loop — it's *built around it*: machine outputs arrive as
  proposals, humans ratify, correction streams federate. A webcivics
  deployment supports accountable human actors (with capacity/guardianship
  handling where needed); it isn't an automation tier.

- **The economics follow the same ladder.** Non-profit/community agent
  categories get their own royalty multipliers (`value_flow`), and a hosting
  org accrues `pc:Micropayment` for the infrastructure it provides — so a
  for-purpose org's participation can partially self-fund rather than being
  pure cost.

- **Planning consequence:** interface specs must state *which tier the user
  occupies* for each feature. A feature that needs `wasm-scientific` is not
  available to a webcivics-only organisation — and that's a *communicated
  boundary*, not a bug to hide.

## 7. Open verification items before interface specs

1. `SkeletalPlayer` portal wiring — browser animation path.
2. Per-serialiser RDF 1.1/SPARQL 1.1 conformance status in-browser profiles.
3. `render/portal` frame model vs `.10d` sections (which sections stream).
4. `wasm_edge` offload governance — what consent/policy wraps an offload.
5. `compute_bridge` WASM registry — actual measured matrix per device class.
