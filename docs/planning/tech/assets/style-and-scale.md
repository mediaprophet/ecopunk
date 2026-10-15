# Asset style tiers & the scale axis — model forms for ecopunk

**Status:** draft v0.1 · scope: the *forms* an asset may take for one entity —
the style spectrum (captured-real → diagrammatic) crossed with the scale ladder
(molecule → biome) — plus per-tier quality gates, provenance rules, and the
style↔semantics separation contract.

Trigger context: current generated assets (primitive-assembler blockouts, e.g.
game-demo's 578-asset packs) are **deliberately sub-final** — composition
fixtures, not finished models. The reference target shared by the downstream
consumer is *illustrated realism* (the Community Grounds concept frame:
believable proportions, designed materials, working infrastructure, warm
restrained palette). One entity must ship **multiple forms** across the style
spectrum; the consumer's profile selects.

---

## 1. The asset matrix: entity × style × scale

For any semantic entity (taxon, landform, artefact, process):

```
                photoreal  illustrated-CG  stylized  schematic  symbolic
  macro (biome)     □            □            □         □          □
  ecosystem         □            □            □         □          □
  organism          □            □            □         □          □
  organ/structure   □            □            □         □          □
  cell/tissue       □            □            □         □          □
  molecule          □            □            □         □          □
```

- One **semantic ID** per entity; each populated cell is a separate
  `.10d` payload + manifest entry with its own provenance and licence.
- **Style never changes the semantic facts** (adopted from game-demo's render
  profile rule): silhouettes, palette, and geometry treatment are
  presentational; identity, relations, and evidence don't move.
- A style selection is a **consumer-side profile**, not a different dataset —
  the same place/entity renders identically-meaningfully in any shipped style.

## 2. Style tiers (canonical set)

| Tier | Name | Character | Production origin | Provenance type |
|---|---|---|---|---|
| **T5** | **captured-real** | photogrammetry, laser/micro-CT scan, SEM imagery, video-derived | recorded reality | **capture provenance**: device, method, operator, subject consent/rights, location, date, processing chain |
| **T4** | **realistic-CG** | authored photoreal — PBR materials, physically-based light, authored rigs/animation | artist/studio authored | author attribution, source files, licence |
| **T3** | **illustrated-CG** | "Community Grounds class" — believable proportions, designed materials, readable detail, restrained palette; animated-film/storybook family | artist authored (procedural assist allowed) | author + recipe + sub-profile |
| **T2** | **stylized** | exaggerated/simplified game stylization, diorama reads | artist/procedural | author + recipe |
| **T1** | **schematic** | botanical plates, anatomical/scientific diagrams, cutaways, field-atlas drawings | illustrator/technical | author + reference source |
| **T0** | **symbolic** | glyphs, map markers, icons, signage | designer | author |

**Notes**
- T5 is a *category apart*: it's **captured**, not authored — the subject's
  rights and the capture context govern it (a scan of a sacred site or a
  restricted specimen is **not** a public asset by default).
- T3 is the tier the downstream consumer's reference frame currently targets
  (muted sage/olive/terracotta palette family; believable humans; designed
  infrastructure). Sub-profiles = palette/lighting/material families under T3 —
  e.g., the consumer's Storybook / Earthlight / Community Grounds presets —
  each a *separately reviewed* variant pack over shared semantic IDs.
- T1 schematic is the encyclopedia's workhorse (field-guide plates, anatomy
  cutaways) — "diagrammatic" is a real tier, not a fallback.
- Tier labels are about **form**, not quality rank — a bad photogrammetry
  capture is worse than an excellent schematic.

## 3. The scale rungs (asset forms per level)

| Rung | Example content | Likely tiers |
|---|---|---|
| molecule | compound models, reaction views | T5 (crystallography-derived), T4, T1, T0 |
| cell/tissue | ultrastructure, histology, pollen/spores | T5 (SEM/micro-CT), T4, T1 |
| organ/structure | leaf, flower, skull, wing anatomy | T5 (specimen scans), T4, T1 |
| organism | whole plant/animal models — rigged, growth states | T5 (photogrammetry), T4, T3, T2, T1 |
| population | stands, colonies, herds — instanced groups | T4, T3, T2, T0 |
| ecosystem | vegetation communities, habitat scenes | T4, T3, T2 |
| biome/macro | landscapes, terrain, Earth views | T5 (satellite/DEM), T4, T3, T1 |

A species pack should target **multiple rungs** — a eucalypt might ship:
whole-tree (T3) + leaf/bark anatomy (T1/T5) + flower dissection (T1) +
pollen SEM (T5) + stand/habitat context (T3). This is the micro↔macro
collection promise in charter §1 made concrete.

## 4. Quality gates (per tier — replaces "sub-standard" with criteria)

| Gate | Rule |
|---|---|
| **Hero-first** | One hero asset per family reaches final-tier quality (silhouette, topology, materials, LOD set, rig variants); LODs and siblings **derive** from it — adopted from the consumer's overhaul plan |
| **Silhouette test** | identifiable at its declared LOD distance without text (a eucalypt ≠ generic round-canopy; a wombat ≠ generic blob) |
| **Material truth** | bark/soil/leaf/cloth/water react differently to light; consistent colour script per sub-profile |
| **LOD bands** | atlas (billboard/marker) → corridor (low mesh) → ground (full mesh) → walk/inspect (hero + interior detail) → micro (section/anatomy) |
| **Topology discipline** | manifold geometry, documented decimation; receipts per compile (`.10d` conformance) |
| **Animation-ready** | declared rig/skeleton where the tier needs it (organism locomotion, growth states); `reduced_motion` variants |
| **No false fidelity** | an asset below its tier's bar is a **labelled placeholder**, never shipped as that tier (consumer rule: "a crude model is not a realistic object") |
| **Deterministic variation** | seeded variants recorded (recipe + seed) so re-bakes are reproducible |
| **Acceptance evidence** | render receipts at canonical poses + review sign-off recorded in the pack manifest |

## 5. Provenance & licence per tier

- **T5 captured** — the strictest: capture provenance (device/method/operator/
  date/site), subject rights (museum specimen licence, site consent, Indigenous
  cultural restrictions), processing chain. A photogrammetry scan can carry
  **more** restrictions than the dataset describing the same entity — licence
  is per-asset, never inherited.
- **T4/T3/T2/T1/T0 authored** — author/studio attribution, source-file lineage,
  licence; procedural assists record generator recipe + version + seed.
- **AI-assisted generation** — honesty label required (generator + review
  status); an unreviewed generated asset cannot ship above placeholder status.
- **Derivative tracking** — `asset_envelope` `derived_from` chains; LODs and
  style variants declare their hero.

## 6. The style↔semantics separation contract

- Semantic entity (`.q42` quins + identity) is style-neutral.
- Each shipped style = its own reviewed pack variant; the manifest binds
  variant → semantic IDs → style tier/sub-profile → integrity digests.
- A consumer can preview the *same* entity/scene across styles (the consumer's
  style-compare affordance) because semantics are shared.
- New styles = new packs, not new entities.

## 7. Source channels per tier (AU-first)

| Tier | Acquisition channels |
|---|---|
| T5 | MorphoSource/specimen scans, museum 3D programmes, TERN EcoImages (field photos as texture/reference), SEM/micro-CT collections, photogrammetry capture campaigns |
| T4/T3 | commissioned/authoring pipeline; procedural botanical engines (branching, species-specific leaves, growth states) feeding `.10d` |
| T1 | Flora of Australia plates, historical botanical illustration (BHL public-domain), commissioned technical illustration |
| T0 | design system |

## 8. Delivery-target classes (orthogonal to style tier)

Same entity + same style tier → different **delivery targets**; a pack records
which targets each artifact was built/budgeted for:

| Target | Constraint shape | Notes |
|---|---|---|
| **Mobile / tablet** | tight memory+thermal budgets; reduced LOD set; compressed textures; battery-aware | entry profile — a degraded-but-honest tier, never false-fidelity |
| **Desktop / laptop** | full LOD set, hero-quality within style tier | reference target |
| **AR** | low-latency pose-stable assets; small LOD set near eye-distance; anchor-compatible bounds; brightness/readability in real-world light | anchors per `ar_anchor.rs`; in-air overlays need authored spatial bounds |
| **Film / TV (linear HD)** | pre-rendered quality ceiling — offscreen budgets; deterministic frames; full spectral fidelity (`αμσ` truth layer feeds the grade); **no real-time LOD compromise needed** — deliverable is the *video*, so assets should be authored at max fidelity even where no device could show them live | `poet-filmcraft` is the production tool (clean-room NLE, WASM-capable) — linear output is a *first-class ecopunk product*, not a render demo |

Rule: delivery class is **declared per artifact**, like style tier — the same
eucalypt may ship T3-mobile + T3-desktop + T5-cinematic variants; all derive
from one hero (§4 hero-first), each carrying its own budget/receipt.

## 9. Open questions

1. Canonical tier/sub-profile ID scheme (`eco:style/t3/community-grounds@n`?).
2. Do style variants share one `.10d` (multi-material) or one per variant —
   consumer precedent is *separate reviewed pack per style*; adopt unless a
   better engine path exists.
3. AI-assisted asset policy — generator disclosure + human-review gate
   (GOV label needed in governance.md).
4. Micro-rung packaging — is a pollen SEM image a `.10d` FIEL, an image entry,
   or both?
5. Which tier is *default* for encyclopedic vs explorer interfaces — probably
   T1/T3 respectively, but decide with `interfaces.md`.
