# ecopunk — Project Charter

**Status:** draft v0.4 · **Date:** 2026-10-24 · **v0.4:** high-grade master production,
downstream civics use and the tooling-to-Webizen experience loop made explicit.

---

## 1. Purpose

**ecopunk** produces ecology-related assets, datasets, ontologies, and structured
resources — plus the interfaces to model, explore, present, and teach with them — as
first-class citizens of the **QualiaDB** ecosystem.

**Production direction:** Ecopunk creates high-grade reusable masters of species,
things and places, with complete knowledge/resource embodiments and editable
sources. Deliberately reduced, reviewed versions serve `C:/github/game-demo`,
which relates to dev.civics.au and illustrates social problems, solutions and
opportunities through gamified spatial experience. The current consumer's
quality or device budget cannot become the master-production ceiling.

Advancing native authoring and dependency capabilities must improve the delivered
content and Webizen Desktop's integrated spatial, knowledge and creative
experience. Recognisable places, credible flora/fauna and understandable systems
are essential to that purpose. See the governing
[high-grade/civics-experience contract](tech/high-grade-production-and-civics-experience.md).

The software is also an applied development programme for QualiaDB and its
custom dependency stack. Authoring, package construction, queries, simulation
and presentation must work through that native stack without requiring
JavaScript, Node.js or V8. Missing capability generates an upstream requirement
and qualification case. See the governing [architecture](architecture.md).

Aether, the controlled Dioxus fork at `C:/github/rust-app-framework`, underpins the
Webizen Desktop **native launcher/host** and harmonised application versions.
Poet remains the unified authoring interface. Supported WASM versions share
application semantics and use native services where the operation requires
them; target-specific dependencies and availability must be explicit. See the
[environment/framework contract](tech/webizen-environment-framework.md).

Chora, the Servo-derived engine at `C:/github/chora`, remains a separate code
base with its own licensing/provenance boundary. Integration follows the
[Chora contract](tech/chora-engine-boundary.md), using generic interfaces and
host-owned adapters without merging engine implementation into the other projects.

**Etalon**, `C:/github/etalon`, is the developed wgpu fork and a continuing
development home for the GPU stack. Assess its actual local implementations
and consumer evidence rather than assuming upstream wgpu's capability limits.
The [production backlog](tech/ecopunk-production-backlog.md) connects that work
to native authoring, complete HMCs and reviewed visual/functional output.

Scope is **biocultural**, not merely biological: the project models the *biosphere*
(flora, fauna, funga, micro-organisms, ecosystems, biomes — molecule to planet) **and** the
*sociosphere* (peoples, communities, institutions, languages, custodianship, standing,
knowledge systems) as **sibling domain layers bridged through shared relations** — Country,
totemic species, custodianship, phenological calendars — rather than merged into one mush
or split into separate projects. Earth-system context layers (climate, landforms, soils,
waters) provide the substrate both domains stand on. See
[sociosphere.md](sociosphere.md) for the sociosphere axiom family and
[domain-model.md](domain-model.md) for the layer structure. The **made world**
adds explicit designs, manufactured objects, vehicles, structures and functional
systems alongside natural entities and the sociosphere; see [made-world.md](made-world.md).
Living organisms and people retain their own identifiers and authority when
interacting with or being depicted beside made-world artefacts.

## 2. Relationship to QualiaDB

ecopunk and QualiaDB share a creator and co-evolve:

- **ecopunk is powered by QualiaDB.** All datasets are processed into QualiaDB-native
  formats (N3/Turtle → `.q42` volumes); all interfaces are QualiaDB QApps; all 3D/media
  assets follow the QualiaDB compiled-asset-bundle model.
- **QualiaDB is developed *for* requirements like ecopunk's.** Working this applied
  use-case is a deliberate engine-discovery mechanism: requirements that expose missing,
  stubbed, or incompletely-wired engine capability are recorded in
  [`tech/engine-requirements.md`](tech/engine-requirements.md) and feed upstream work in
  the `qualiaDB` repository. Ecopunk planning never assumes an engine feature exists —
  it verifies, and logs gaps where it doesn't.

## 3. What ecopunk produces

| Output class | Description |
|---|---|
| **Datasets** | Biosphere data acquired from online sources, normalized, licensed, provenanced, and compiled into `.q42` volumes |
| **Ontologies** | A biosphere vocabulary stack aligned to Darwin Core / ENVO / NCBITaxon / Wikidata, plus an `eco:` bridging vocabulary for cross-scale relations |
| **Asset packages** | 3D (and other media) assets in multiple styles — photoreal/scientific through stylized to cartoon/game — organized into microscopic and macroscopic collections |
| **Embodiments** | Complete species HMC releases containing the available graph corpus and associated resources; made-world HMC families containing design, assembly, operation and representation resources. Consumers select what they need from the same embodiment. |
| **Interfaces** | QApp interface families: exploration/atlas, encyclopedic, modelling/workbench, presentation/storytelling |
| **Models** | Cross-dataset analytical products: species×climate envelopes, food webs, trait matrices, scale-joined entity profiles |
| **Specifications** | The planning corpus itself — reusable by sibling projects powered by QualiaDB |

## 4. Governing principles

1. **Fifty-year durability.** Prefer open standards, stable identifiers, archived
   provenance, and formats that can be re-decoded without us. Every artefact must survive
   the death of any single institution — including this one.
2. **Provenance is non-optional.** Every triple and every asset carries its lineage:
   source, retrieval date, transform chain, licence, reviewer.
3. **License honesty.** A permissive-commons lane and a restricted lane; non-commercial
   and access-restricted sources are never laundered into the commons.
4. **Sensitivity is real.** Biodiversity data can kill — precise locations of
   poaching-vulnerable species are genuinely classified data. Sensitivity classes and
   deontic gating are functional requirements, not decoration.
5. **Data sovereignty.** Indigenous and traditional ecological knowledge follows CARE
   principles and Local Contexts TK Labels — authority remains with the community of
   origin, technically enforced where possible.
6. **One graph, many views.** The same manifold supports a 3D scene, a 2D pane, an
   encyclopedia page, and a game asset — per the QApp manifold-world model.
7. **Scale is a first-class axis.** The microscopic↔macroscopic ladder is a navigable
   dimension of the graph, not a set of disconnected datasets.
8. **Honesty in labelling.** Modelled ≠ observed; reconstructed ≠ scanned; generated ≠
   captured. Every artefact declares what it actually is (see QualiaDB's honesty-chip
   convention).
9. **Local-first, commons-capable.** Everything works on-device; sharing happens through
   permissive-commons publication, not platform dependency.
10. **Fail closed.** Where licence, consent, or sensitivity is ambiguous, the artefact is
    withheld, not published.
11. **Human centric — a defined term, not an aspiration.** "Human centric" was defined by
    this project's author in relation to the *tools*: W3C Verifiable Credentials, DIDs,
    RWW/Solid, WebPayments — human-anchored infrastructure where the **human being** is the
    sovereign endpoint (history in the W3C public archives, e.g. the public-webpayments DID
    thread, May 2014). Deliberately *human*, not "person": in many jurisdictions "person"
    includes legal persons and companies — the term names the flesh-and-blood root, not the
    juridical wrapper. It is required, among other things, to ensure **human beings can
    independently maintain agency over their own biometrics** — face, voice, physiology
    cannot be held as platform property. In this architecture it is a **structural
    property**: authority roots in human beings (SOC-AX-14), agents self-host (`spc:`),
    machine outputs are proposals a human ratifies, biosensing is consent-gated
    (`biosense`/`evaluate_processing_act`). This predates and differs from the later
    industry usage ("human-centric AI" as a design *attitude* — ~7,300 Google News hits
    2023+, mostly decision-support framing). Here it is load-bearing: remove it and the
    delegation chains, consent envelopes, and economic commons stop making sense. This
    ecosystem — QualiaDB plus the projects it powers — is that term's implementation.

    **Scope and legitimacy.** Human centric is this ecosystem's constitutive design — it
    does not claim to bind systems outside it. Its warrant is human rights: the methods
    (human-rooted authority, structured consent, non-delegable biometric agency,
    accountability-to-a-human audit trails) exist *to support and protect* human rights —
    and in this codebase that basis is machine-checkable, not rhetorical: the ontology
    corpus carries 102 UN human-rights instruments (OHCHR/ICRC/regional charters)
    re-expressed as N3 values-credentials. Nation-state advocacy followed the early
    definition (Japan's *Social Principles of Human-Centric AI*; G7 ministerial language
    on "responsible, human-centric AI based on democratic values", 2023 — visible in the
    same news record), corroborating the direction without diluting its structural meaning
    here.

    **Precedent.** When Berners-Lee built the Web, alternatives existed — Gopher, the
    pay-TV/walled-garden networks; when Cerf and Kahn built TCP/IP, carriers had their own
    network ideas. The open, end-to-end, human-anchored topology had to be *allowed to be
    made* — it was not the architecture its gatekeepers would have chosen. The same holds
    here: an information topology that structurally refuses agent-capture — where software
    agents cannot control, enclose, or manipulate human experience the way
    platform-centric topologies permit — is deliberately not the most extractive design.
    It is nonetheless the one being made, and its permission to exist is the point.

| Non-goal | Reason deferred |
|---|---|
| Real-time sensor ingestion (live biotelemetry feeds) | Acquire-and-normalize static/periodic sources first |
| Clinical/biomedical application | Covered by WellFair; ecopunk is environmental/organismal |
| Full Earth-system simulation | We model *information about* the biosphere; coupled simulation is an F4-tier engine concern |
| Authoritative taxonomic judgement | We aggregate and align backbones; we do not publish novel taxonomy |
| Unbounded encyclopedia of every topic | Scope follows the declared natural, made and social domain contracts; external sources remain attributed references |

## 6. Sibling context

Related projects in the same ecosystem that this planning should stay aware of:
`wellfare` (personal health/welfare vault — the micro-anatomy precedent via the Anatomy
qapp), `webizen-*` (platform surfaces), `ontology` (shared vocabulary work), `poet` +
`poet-*studio` (the authoring environment family — asset production chain: art_studio,
cad_studio, film_studio, photo_studio, sound_studio, vector_studio, etc.), `game-demo` (Maslows
Challenge — the downstream consumer AND the RD&D driver: its needs force real engine
implementation).

## 7. Open questions

- Canonical namespace for ecopunk IRIs: resolved to **`https://webizen.org/`** (and `https://ns.webizen.org/` for minted ontology prefixes `eco:`, `bio:`, `soc:`, `env:`, `stat:`), never `ecopunk.au`.
- Governance/review process for promoting acquired data to curated corpus (mirror
  QualiaDB's `acquisition-inbox` → human review → promotion flow?).
- Distribution channels beyond bundled/installer (WebTorrent mirrors? Solid pods?
  institutional CDN for large volumes?).
