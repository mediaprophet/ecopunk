# The 10D Q42 tensor — intended axis semantics (spec reconciled with code)

**Status:** draft v0.1 · based on the 0.0.13 architectural spec (2026-06-16,
authored synthesis of pipeline discussions with Timothy Holborn) + seven
companion diagrams, reconciled against the implemented
`container_10d/axis_role.rs`, `header.rs`, and `modalities/manifold.rs`.

This is the document behind the "10d grid / quantum-style address" idea: the
tensor is designed as a **pre-compiled, quantized physics engine for logic and
perception** — queries become geometric projections, v-switched distances,
temporal slices, context collapses, and spectral blends on SIMD/GPU.

---

## 1. Axis semantics — spec intent vs implemented role

Canonical order `[q, v, w, x, y, z, t, α, μ, σ]` — identical in spec and code.

| Axis | Spec semantics (0.0.13) | Implemented role (`axis_role.rs`) | Status |
|---|---|---|---|
| `q` | **Quantum Context / superposition index** — `q=0` collapsed ground truth; `q>0` pending GSR escrow, LLM sandboxes (e.g. `q=999`), parallel what-if branches; wavefunction collapse promotes winner to `q=0` + logs new `t` | Selector (excluded from distance) | spec semantics; role verified |
| `v` | **Topological/variety class** — the local "physics rules": `0` Euclidean, `1` Cyclic/toroidal (circadian, feedback), `2` Hyperbolic/tree (taxonomies!), `3+` sovereign boundary cliques (membership = byte compare, not edge tracing) | Selector — metric selector; the code's `full_distance` folds exactly these classes | verified (metric classes implemented; descriptor gate exists) |
| `w` | **Manifold/domain index** — multi-head bifurcation: separate knowledge universes in one memory block; spec examples `w=0` bio/medical, `w=1` legal/jurisdictional, `w=2` personal/agency, **`w=3` environmental/sensor**, `w=4` socioeconomic/wellbeing; cross-`w` correlation = batched matrix projection, no joins | Selector | role verified; domain enumeration is pack-level convention |
| `x,y,z` | **Semantic topology** — spatial embedding; relatedness = v-adjusted distance; kNN/rays/bounding-volumes on GPU/SIMD | Coordinate | verified |
| `t` | **Temporal state / version ledger** — immutable history; "what was knowable at time X"; every collapse/version logs `t+1` | Coordinate | verified (reconciled 2026-08-19: `t` = coordinate time, *not* the provenance lane) |
| `α` | **Amplitude / dynamic range / confidence weight** — linear intensity, HDR, verification weight; doubles as *mass* in the gravity analog | Coordinate | role verified |
| `μ` | **Modulation / phase / metadata carrier** — steganographic DID/provenance/consent layer in guard bands; WDM-style stacked streams | **CoordinateCarrier** — dual-role: coordinate *and* the in-band provenance lane | verified (code calls it "provenance carrier" — same lane the spec calls the metadata/DID carrier) |
| `σ` | **Spectral signature / logical class index** — SPD band vector (visual), timbral class (audio), or packed logical class | Coordinate | role verified |

The second ten (`Epistemic10D`/`ManifoldCoordinate10D`: scale, attention_depth,
epistemic_weight, topological_spin, temporal_decay, entropy_bias,
spatial_phase, recurrence_frequency, density_threshold, manifold_curvature) is
the *host-internal* epistemic geometry — the T14 rule holds (never conflate).

## 2. `q` — epistemic superposition as an axis (the distinctive move)

- **Pending is first-class**: an unresolved problem lives at `q>0` — the UI
  can proceed against a flagged-pending datum instead of blocking or lying.
- **Sandboxing by construction**: speculative evaluation at high `q` (e.g.
  `q=999`) cannot pollute `q=0` ground truth — drop the branch, keep the
  record.
- **Collapse = frame evolution**: a resolution promotes the winning context
  to `q=0`, updates `(x,y,z,v,w)`, increments `t`, and may *cascade* —
  previously-impossible inferences become available (the spec's "axiom
  insemination"). The audit trail shows exactly when truth became knowable.

This is the same "proposals, not assertions" discipline we've documented
elsewhere — expressed here as an *address-space* property rather than a
policy.

## 3. The GSR lane — scarce-quantum as a commons resource (Tier 3)

The spec's Ground-State Resolver design (diagram: escrow → QUBO → collapse →
cascade → mesh broadcast):

- **Classical exhaustion first ("99% rule")** — QPU only for proven-hard
  subproblems; never in the synchronous hot path.
- **Minimal QUBO extraction** — strip human semantics, ship only the
  deadlocked `(x,y,z)` sub-manifold as a pure matrix.
- **Stateless escrow** — write outbox + set `q>0` in-escrow flag, return a
  `Defeasible/Pending` state immediately; no thread waits years.
- **Proof-of-demand aggregation** — identical QUBOs from other nodes upvote
  each other; demand schedules the queue, not FIFO.
- **Holographic caching** — a GSR-proven result gossiped to the mesh patches
  every receiving volume; **the ecosystem computes each unique geometric
  problem once** — a permanent commons good (this is the decentralised-
  *resolution* counterpart of our decentralised-training loop).
- **Terminology**: GSR / Epistemic Anchor Pool — deliberately not "oracle"
  (corporate) or "god-machine" (deistic): truth is mathematically derived and
  cryptographically verified, not handed down.

## 4. Gravito-thermodynamic baking (physics stage, not new axes)

Per the spec's pragmatic path — the core 10D stays 10D; gravity/temperature/
pressure arrive through **baking + modulated operators**:

- **Gravity** — `α` as mass; a physics-sim stage self-organizes `(x,y,z)` into
  class-consistent clusters (GRAVITY-style attractive dynamics); `v=2`
  hyperbolic gives natural gravity wells.
- **Temperature** — region/point activation: high-T widens diffusion radius,
  softens blending, favors reaction/exploration; low-T freezes facts.
  Annealing schedules during baking reach equilibrium.
- **Pressure** — local density/constraint load: high-P tightens bounding
  volumes and constraint strength; CkNN-style density-aware construction keeps
  the manifold geometrically faithful.
- Optional 12D variant `[q,v,w,x,y,z,t,T,P,α,μ,σ]` reserved for physics-
  research workloads; not the default.

## 5. Spectral payload — truth is the spectrum, not the pixel

- **Storage/truth layer is spectral**: SPD for visual, STFT/CQT sheets for
  audio + linear `α` amplitude + `μ` modulation metadata. **RGB is a lossy,
  device-specific projection** — never the stored truth.
- **Render = last-mile projection**: SPD → CIE XYZ → device gamut; psycho-
  acoustic curves for audio — applied per-device, re-projectable forever as
  hardware evolves. This is the format-level guarantee behind our T5
  captured-real tier: capture the spectrum once, project honestly forever.
- **`μ` steganography layer** — DIDs, provenance, consent flags embedded in
  guard bands (sub-20 Hz / near-20 kHz audio; IR/UV visual) — *identity and
  rights travel inside the signal*, not beside it.

## 6. What this means for ecopunk

- **`w=3` is our lane**: the spec's environmental/sensor domain index is where
  ecopunk volumes live; `w` bifurcation lets an encyclopedia query cross to
  legal (`w=1`) or socioeconomic (`w=4`) without leaving the tensor.
- **Taxonomies are native terrain**: `v=2` hyperbolic is *built for*
  hierarchies — Darwin Core / ChEBI / IBRA trees ride the geometry, not a
  traversal algorithm.
- **Sovereign cliques (`v=3+`)** give custodial communities a *byte-level*
  membership boundary — the geometric form of TK-label gating.
- **`q`>0 pending + `t` audit** is exactly the right shape for contested
  identifications, disputed borders, and unratified alignments — pending is an
  address, not an exception.
- **Spectral truth** aligns with the asset spec: captured-real assets store
  SPD/STFT + amplitude + modulation; every render tier (photoreal → symbolic)
  is a *projection* of that truth, never a re-encoding of a projection.
- **GSR economics** connect to `pc:Micropayment` — scarce-quantum resolution
  is a meterable commons service; proof-of-demand is demand-registered
  ontology-level scheduling.
- **Probability fields are tensor-native**: [probability-geometry.md](probability-geometry.md)
  defines `ProbabilityField` as a measure over typed state/factor/epistemic
  axes carried *by* this geometry — `q` for rival un-collapsed structures,
  `α` for measure weight, `μ` for per-slice epistemic binding, `v` metrics
  per region class, `t` for field vintage. Accrual stocks, trajectories and
  windows ([accrual.md](accrual.md)) inhabit the same manifold.
- **The workload explains the format**: the accrual / reconciliation /
  probability-geometry requirements are part of *why* the container is
  shaped this way — superposition (`q`), metric classes (`v`), domain
  partition (`w`), vintage (`t`), measure weight (`α`), per-slice provenance
  (`μ`), logical/spectral class (`σ`); and at the datum level, sensitivity
  class + contract context + Lamport clock + modality payload packed into
  the 48-byte quin's `context`/`metadata` fields. Rights, provenance,
  epistemic state and time travel **in-band** because the honesty rules
  require them per-datum — the format was structured to carry ontological
  geometry natively, not to retrofit it onto a row-store.

## 7. Reconciliation notes (spec → code, honest gaps)

- Header/axis roles: **implemented** (P0.1 gates pass); taxonomy marked
  *proposed-not-frozen* until P0.7 conformance vectors land.
- Section table, per-section CRC, quantized-mesh + Tensor10D node sections,
  renderer upload, WASM parity: listed as **spec-reserved** in the v2 header
  docs at survey time — verify current state before asserting in specs.
- `q`-superposition semantics, GSR escrow, gravito-thermo baking, spectral-
  payload ingest: **design-level** — treat as roadmap primitives; check
  `qpu_dispatcher`/`qubo_compiler`/`spectral` module status before citing as
  available functionality.
- The spec's `t` "provenance ledger" phrasing is reconciled: `t` = time;
  provenance/consent carrier is `μ`.
