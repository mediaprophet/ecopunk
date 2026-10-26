# Biosphere integration: computation, Q42, EMF and selective detail

**Status:** proposed integration contract v0.1, 2026-10-26. Review and plan update;
not a new format release, installed streaming system or biological qualification.

Timothy's four considerations govern the next white-gum/wallaby work. This
contract expands [BSA-01–12](biosphere-earth-and-anatomy-capabilities.md), the
[species embodiment](assets/species-embodiment-spec.md),
[made-world](../made-world.md), [package](assets/asset-package-spec.md),
[spectral intent](tensor-10d.md), [spatial substrate](spatial-substrate.md) and
[delivery contract](pack-delivery.md). Preserve the existing IP/ECO/G/FND order.

## Current findings and required reuse

Three scoped reviews inspected actual working sources, without builds or edits
to the engine repositories:

- [Compute reuse](../../qa/implementation-2026-10-26/biosphere-integration-review/compute-reuse.md)
  covers Core geometry, `qualia-sim` plant/leaf/contact/deformation, native host
  boundaries, N3 rules and temporal trace evaluation. Sim depends on Core;
  adapters must not introduce a Core-to-sim dependency cycle.
- [Graph/resource streaming](../../qa/implementation-2026-10-26/biosphere-integration-review/graph-streaming-reuse.md)
  traces Q42 root manifests, indexed range queries, SPARQL pages, HMC entry and
  interior-section plans, typed payload resolution and current residency limits.
- [EMF source inventory](../../qa/implementation-2026-10-26/biosphere-integration-review/emf-source-inventory.md)
  records actual spectral/field/audio representations and their numerical domains.

The earlier audit was too narrow to allocate all growth/physics work: existing
`MetamerPlantGraph`, leaf hydraulics, botanical continuum coupling, XPBD contacts,
ragdolls and event queues must be considered first. Their source presence does
not establish species calibration, persistent native authoring or correct use.
Likewise, existing N3/LTL evaluation must be reused within its actual semantics;
a rule result is not an observed fact or a validated biological law.

Timothy's subsequent nine-location check is recorded in the
[explicit source coverage ledger](../../qa/implementation-2026-10-26/biosphere-integration-review/README.md).
Existing botanical/hydrological modalities decode semantic state; bind them to
the appropriate existing sim/field owner rather than calling them solvers.
The inspected blackboard/DAG consumer checks constraints after writes and does
not make final success depend on violation counts. Do not use it as a hard
admission gate without owner-level transactional/error handling qualification.
Semantic-instrument packaging, resolution and attestation are real source
foundations, while the current generic runner is explicitly demo execution.
Reuse those owners without treating a closed label, receipt or signature as
resolved transitive dependencies, biological computation or adopted permission.

The saved eleven botanical software tests remain evidence only for their
[bounded turtle/native path](../../qa/implementation-2026-10-26/biological-reference/execution.md).
Do not reimplement that connection or infer graph/physics/streaming acceptance.

## Q42 is the semantic spine of complete embodiments

**BGS-01 — graph and binary resource separation.** Q42 carries qualified taxon,
organism/artefact/component, occurrence, observation, model, parameter, rule,
source-rights, state and representation relationships. Dense geometry, sampled
spectra, images, volumes, sound, clips and simulation buffers remain native typed
resources referenced by that graph. Reuse Q42 querying, indexes and existing
HMC/.10d/M64 codecs; do not create an application-owned parallel graph or parser.

**BGS-02 — complete release versus selected working set.** A complete HMC retains
the available corpus at a stated revision, including knowledge unused by the
renderer, interpretation context, editable sources and resources. Opening it
does not require decoding everything. Graph partition, index and lexicon access
must themselves have bounded residency. Reconcile all available in-scope material
against the release inventory; rights-filtered/incomplete exports declare coverage.

**BGS-03 — referents and change.** Preserve multiple qualified identifiers and
distinguish concept, individual, organ/component, representation, state and release.
Simulation and observations use existing journal/state owners against an immutable
base. Manufacturing definitions, actual installed components and their observations
remain distinct. Graph cycles can be valid knowledge; dependency traversal and
containment have their own bounded semantics. Matching hashes/strings confer no
authority or biological equivalence.

## EMF, physical fields and organism responses

Retain **EMF** as the ecosystem's native spectral/field framework and retain exact
legacy API names. Its shared orchestration must carry physically typed modalities.
Visible light and infrared are electromagnetic radiation; sound is a mechanical
wave in a medium. Sharing frequency units does not make their propagation,
amplitude, sensing or biological effects interchangeable. [NASA distinction](https://science.nasa.gov/asset/webb/electromagnetic-wave-light-wave-vs-mechanical-wave/).

**BGS-04 — typed quantities before coupling.** The logical descriptor of each
resource or model invocation declares:

| Group | Required meaning |
|---|---|
| Referent and domain | Source/receiver/region/component identifiers; electromagnetic radiation or static field, acoustic/mechanical, thermal, pressure, moisture/humidity, chemical or other declared modality. |
| Quantity and units | Physical quantity, unit, reference/normalization and whether absolute, relative, density-per-band or integrated. A dB value needs its reference and weighting. Temperature is not an infrared radiance measurement. |
| Sample/basis description | Frequency/wavelength/time axes, sample centres/edges, bandwidth, resolution, valid domain and basis/model. Wavelength-to-frequency conversion preserves the relevant spectral-density measure. |
| Space and time | Frame/orientation/units, support/extent, sampling/observation times, timestep and validity. Scalar ambient pressure differs from oscillating acoustic pressure. |
| Interpretation | Measured, reconstructed, simulated, authored or illustrative; calibration, source model/version, uncertainty, assumptions, validity range and rights. Unknown bands remain unknown. |
| Execution | Required existing capability, compatible profile, backing resources, work limits, transfer/working-set limits and declared failure behavior. |

These are semantic requirements to reconcile with existing graph/typed-field
owners, not a declaration of new binary fields or a released vocabulary.

**BGS-05 — stimulus to response chain.** Preserve a traceable chain:

```text
source state/signature -> field/propagation through a medium -> receiver exposure
-> sourced sensor/receptor response -> authored biological rule/physiology
-> state/event change -> selected visual/audio/scientific presentation
```

Light transport, heat transfer, acoustics, moisture, pressure and chemical cues
use their existing native owners and explicit couplings. A recoloured texture,
sonification or field overlay does not establish the receiver's perception or a
behavioural response. Observer-specific visual/sensory models require sourced
response curves and limits; unavailable models remain unavailable. Do not infer
infrared perception for the wallaby from other animals. Research on snake pit
organs illustrates a specific thermal transduction mechanism, not a universal
"infrared eye" model. [Gracheva et al., 2010](https://pmc.ncbi.nlm.nih.gov/articles/PMC2855400/).

**BGS-06 — respect actual source scope.** Current Core `Spd` has 41 samples over
380–780 nm; its three-parameter EMF-to-SPD function reconstructs a Gaussian within
that domain. This is not captured full-spectrum data or a valid arbitrary IR
extension. The inspected Etalon field visualizer maps phase to display hue; its
stored frequency is not used in that shown projection. Existing sigma-to-audio
mapping is authored sonification. Retain these useful operations with truthful
labels while allocating broader calibrated quantity/basis/receiver support.

Preserve original RGB/media and their acquisition limits; spectral reconstruction
must not invent measured bands. Human RGB display, false colour, acoustic output
and an organism-response model are distinct projections of identified resources.
Metadata embedded in a signal is not proof of consent, authority or permission.

## Query, inspection and magnification-driven streaming

**BGS-07 — selection is an intent with physical context.** Record selected subject,
part/relationship, query, required observation/instance state, style/target,
inspection footprint and physical scale, lens/magnification model where applicable,
spectral/receiver mode and resource budget. Magnification changes which supported
detail is useful; it cannot create acquired or validated microscopic information.
Choose fidelity using projected error/footprint and declared task accuracy, not
distance alone. A graph query can request a hidden internal part or nonvisual data.

**BGS-08 — graph-selected dependency closure.** Use the native path:

```text
identified release/root + admitted partition/index
-> bounded Q42 query and lexicon pages
-> referent/part/relationship and compatible representation/resource identifiers
-> bounded typed dependency closure + rights checks
-> authenticated installed bytes OR qualified remote range/proof acquisition
-> canonical validation/decode -> bounded CPU/GPU residency -> presentation/eviction
```

Do not download the entire graph or every payload to answer every inspection.
The closure includes required geometry, materials, normals/UVs, rig/clip/state,
scientific/spectral resources and interpretation/provenance dependencies. External
species relations retain their release identifiers; acquire only the declared
operation's necessary closure, not every related species recursively. Bound
count/depth/work/bytes and deduplicate qualified resource versions.

**BGS-09 — preserve subject distinctions in refinement.** Fish scales may be
identified parts and surface detail of that fish. An insect in grass and lichen
on rock can be separate organisms with evidenced spatial/ecological relationships,
not additional LODs of the grass or rock. Selection may reveal existing small
subjects; it must not spawn new ecological facts on zoom. Separate simulation
existence, observation visibility and rendering residency. Continuous camera
movement preserves instance, part, time, pose, material and interaction state.

**BGS-10 — integrity must match the access mode.** HMC whole-entry SHA256, `.10d`
section CRC, Q42 internal block addressing and transport proofs are different
mechanisms. A received range or checksum alone does not prove authenticity.
The inspected HMC verifier can return success with no expected digest; callers
must not promote that to authenticated data. Malformed digest metadata must
refuse qualified acquisition rather than silently dropping verification.

First qualify selective local access to already authenticated installed resources.
For untrusted remote partial access, require a trusted descriptor/root and a
qualified existing per-entry/chunk proof contract. If verification requires a
whole entry, report that granularity and its actual transfer/resident costs;
partition resources appropriately under existing format rules or allocate the
needed owner-level authenticated sparse support. Never pad missing bytes into a
fake whole file, skip canonical validation or silently label CRC as authentication.

**BGS-11 — bounded, cancellable lifetime.** Reserve transfer, compressed and decoded
CPU data, GPU data, staging and cache separately. Output paging does not cap scan
work: no-match queries also need work/byte/deadline bounds and cancellation.
Tie cursors/jobs to immutable release, graph/query/state revision, authority and
selection generation. Reject stale results and revoked access before publication;
preserve the last valid view on failed refinement. Release both CPU and GPU
resources, cancel speculative work and expose missing/denied/offline states.
Use hysteresis and bounded prefetch to prevent zoom thrashing. Persistent cache
and offline install are explicit scopes; partial residency is not bundle completeness.

**BGS-12 — known integration work.** Current source opens every Q42 child header
at root construction, plans all HMC entries by default, and requires complete
contiguous `.10d` input for typed payload resolution. Existing texture residency
does not bound whole-HMC CPU retention. Reuse these owners and extend the needed
lazy query, selected closure, sparse descriptor and resource-lifetime paths;
do not treat their presence as finished magnifier streaming.

## Concrete qualification cases and output evidence

The fixtures below are required workflow tests, not new accepted species masters.
Keep all existing per-model catalogue obligations and target statuses open.

| ID | Case and concrete evidence required |
|---|---|
| BGS-Q1 | White-gum graph -> selected leaf/branch -> source edit -> regenerated representation: preserve organ mappings and unaffected edits through undo/HMC reopen. Use existing plant/geometry owners with declared domain parameters. |
| BGS-Q2 | Wallaby anatomy -> selected mesh/rig/clip -> actual animated positions/normals/bounds: source, graph and native viewport agree; retain semantic parts and state on detail changes. |
| BGS-Q3 | Fish -> scale detail; grass -> identified insect; rock -> identified lichen: native graph answers and requested resources match a pinned fixture oracle. Include wrong referent/state, missing detail and external relationship cycles. |
| BGS-Q4 | Magnifier changes sampling/detail with a scale bar and declared physical footprint; record exact graph blocks, HMC ranges, requested/fetched bytes, cache hits, decoded/staging/GPU high-water marks and latency. Prove unrelated payloads were not requested. No universal latency is promised before measurement. |
| BGS-Q5 | Broad-domain signal descriptors: preserve measured nonvisible bands; refuse wrong units/basis/domain; separate electromagnetic, acoustic, thermal/humidity/pressure quantities and authored sonification. Execute only the declared model-validity range. |
| BGS-Q6 | Actual cue -> bounded existing solver/rule -> state event -> presentation with causal provenance; missing species response stays unknown. Compare against a declared analytic/domain fixture with predefined tolerances; visual plausibility is insufficient. |
| BGS-Q7 | Corrupt/truncated/wrong-range/missing-digest/wrong-release/denied/late/cancelled requests cannot publish. No-match graph scans yield within admitted work. Offline missing dependencies produce an accurate availability result without full-package claims. |
| BGS-Q8 | Native contributor/reviewer performs source/query/detail edits, rejects and accepts exact revised output, then fresh-process reopens with graph/resources/rights/review lineage. Human actions and acceptance must be actual records. |

## Execution preparation

Follow the [scoped implementation dispatch](../biosphere-implementation-dispatch.md).
Workers own disjoint staged files and handoff receipts. A single integration
coordinator owns shared module/manifest/lock changes and serial Cargo/native/GPU
checks against a frozen source set. No source mutation during qualification.
Long-running child commands retain an owner through captured exit; a lost chat
does not authorize killing processes, deleting locks or starting duplicate builds.

This review closes no foundation/model/education gate. Native/WASM, scientific
accuracy, full HMC completeness and film/game/cartoon acceptance remain separate.
