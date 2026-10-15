# Ontology stack — the LOD-cloud vocabulary architecture

**Status:** draft v0.1 · scope: which vocabularies ecopunk consumes, which it
mints, how LOD-cloud resources are processed into `.q42`, and how the stack
supports **inference** (semantic reasoning + learned prediction) and
**decentralised training** in future.

**Verified anchors (this survey):**
- `manifests/ontologies.json` — a **downloadable ontology registry already
  exists** (edit-to-add; hosted JSON, no recompile). Current entries:
  schema.org 26.0, SKOS, FOAF, DC Terms, PROV-O, GeoNames 3.3, DBpedia
  ontology, WordNet RDF, **ChEBI OWL 231**, Wikidata properties, OpenCyc.
- `query/ontology_loader.rs` — startup loader: Turtle/N-Triples → NQuins,
  priority-resolved (`$QUALIA_ONTOLOGY_PATH` → `./ontologies/` → exe-dir),
  fail-open w/ warnings (native-only today).
- `core-ontologies/` — engine-native vocab layer incl. `values.n3` +
  **102 UN instruments as values-credentials**, `jural.n3` (Hohfeld),
  `modal-junctures.n3` (epistemic bands), `permissive-commons.n3` +
  `social-permissive-commons.n3`, `policy`, `agency`, `agent-accountability`,
  `capability-credentials`, `selfhood`, `sense`, `tiering`, `jurisdiction`,
  `semantic-instruments` (+ shapes + capability-profile), `humanitarian-ict`,
  `inference-lifecycle`, `directory`, `cml`; dirs: `concepts/`, `regional/`,
  `un-instruments/`, `traces/`, `_authoritative`, `acquisition-inbox`.
- `q42/chebi_{parse,map,query}` — the working per-source LOD subsystem template.
- `solvers/ontology_align` — `closeMatch` proposals only; `exactMatch`
  **type-forbidden** without human ratification.
- `sparql_library/sparql_mm` — `ma-ont:` fragment + C2PA predicate families.

---

## 1. Stack layers

```
L4  ecopunk-minted     eco: · bio: · soc: · env:        (bridges, gaps only)
L3  per-domain axioms  e.g. custodianship axiom sets     (SHACL/N3 packages)
L2  domain vocab       dwc:/TDWG · TERN/SOSA/SSN · ENVO+OBO · ChEBI/PubChem
                        · GeoSPARQL · QUDT · ma-ont · OntoLex-lemon · SKOS-XL
                        · CiTO/BIBO · ODRL · Local Contexts TK · DCAT · VoID
                        · **RDF Data Cube (qb:) · XKOS · SDMX artefacts**
                          (statistical observations, classifications, DSDs —
                          see statistics.md)
L1  LOD foundation     schema.org · SKOS · FOAF · DC · PROV-O · GeoNames
                        · DBpedia · WordNet · Wikidata props · OpenCyc   [registry ✓]
L0  engine-native      values: · jural: · modal-junctures · policy: · agency:
                        · agent-accountability · capability-credentials ·
                        cml: · directory · humanitarian-ict ·
                        inference-lifecycle · jurisdiction · sense ·
                        selfhood · tiering · social-permissive-commons ·
                        semantic-instruments · permissive-commons      [core-ontologies ✓]
L0a instrument corpus  un-instruments/ (102 text-layer provisions, verbatim
                        values:originalText + OHCHR/ICRC provenance) +
                        concepts/ (104 cml:Concept logic layers — one per
                        provision, cml:realizedBy text, cml:asserts deontic
                        norm; cml:Proposed pending human attestation)
```

**Rules:**
- **Reuse before minting** — a term from L0–L2 in its canonical namespace, not
  an `eco:` twin. Minting happens only where the layer genuinely lacks the
  concept (our novel work: Country-as-relation, standing axes, living/made
  provenance partition, pack/layer manifests, scale-ladder roles).
- **Versions are pack dependencies** — a dataset compiled against
  `chbi.owl@231` + `tern@x.y` records them in its `asset_envelope`; vocabulary
  is data, versioned like data.
- **Alignment is a proposal stream** — `solvers::ontology_align` emits graded
  `skos:closeMatch` + `RequiresHumanReview`; ratification is an attested act
  (who, when, evidence). Never machine-assert `owl:equivalentClass`/`exactMatch`.

## 2. LOD-cloud source processing

The LOD cloud is a *corpus of vocabularies + datasets + linksets*, and the
pipeline treats each differently:

| Resource kind | How it's processed | Examples |
|---|---|---|
| **Vocabularies** (OWL/RDFS/SKOS/JSON-LD) | extend the `ontologies.json` registry → load → map into namespaces → SHACL-profile per use | TERN (OWL+SHACL ships), dwc suite, ENVO, QUDT, OntoLex-lemon |
| **Datasets/dumps** | selective ingest via `sparql_adapter`/dumps → normalise → licence-lane → compile | Wikidata subsets, DBpedia, GeoNames, CoL |
| **Linksets/mappings** | ingest as *proposals* — `closeMatch`/`exactMatch` claims carry source+confidence; never trusted at face | LOD linksets, Wikidata crossrefs |
| **Catalogues** | the LOD-cloud catalogue + `linked.data.gov.au` + prefix.cc processed as source registries (`source_catalogue` descriptors — `Unverified`/`Restricted` by default) | lod-cloud.net data.json, AGLDWG |
| **SPARQL endpoints** | live-query adapters (consent-gated egress); results land as provenance quins | TERN GraphDB, WDQS, linked.data.gov.au |

**LOD-cloud realities the pipeline must handle:**
- dead links & drift → **snapshot vocab files into packs**; pin versions;
  never hotlink the cloud at runtime (durability rule);
- uneven quality → source descriptors stay `Unverified` until licence + schema
  review;
- licence scatter → per-dataset `LicencePolicy`; many LOD dumps assert one
  dataset-level licence but mix sources inside — flag for review;
- namespace sprawl → a bounded prefix registry; unregistered namespaces are
  visible, not silently dropped.

## 3. The minted layer (what we actually coin)

| Prefix | Domain | Contents |
|---|---|---|
| `eco:` | bridging | scale-ladder roles, layer/pack manifests, entity↔pack links, eco profile descriptors |
| `bio:` | biosphere | taxon-side extensions beyond dwc (lineage kinds `wild/bred/engineered`, profile entities, biome×season context) |
| `soc:` | sociosphere | place-relation family (custodianship/belonging/Country), standing axes, community recognition, concept-in-language |
| `env:`/`geo:` | earth system | landform/hydro/climate context classes not in ENVO/GeoSPARQL |
| `stat:` | statistics | confidentiality marks, vintage/release machinery, claim evidence-classes — gaps in qb:/SDMX-RDF only ([statistics.md](statistics.md)) |

Each minted namespace: `AGLDGW`-convention metadata (dc:title, versionIRI,
imports, licence), SHACL shapes, and a published `.ttl` per version — the same
form as the L1/L2 vocabularies we consume.

## 4. Inference support (built-in, over the same quins)

The stack isn't storage — the modalities/solvers *are* the inference ladder:

| Need | Engine machinery |
|---|---|
| Class reasoning | `modalities/dl` — subsumption over the subClassOf DAG (**multiple inheritance** — needed for dual-membership engineered entities) |
| Rules | N3 Strict `=>` / Defeasible `~>` / Defeater `^>` / Linear `-o` — defeasible = taxonomic revision, contested claims |
| Time | Allen intervals + LTL/CTL — validity, history, "as-of" queries |
| Space | RCC8 + quadtree + geospatial adapters |
| Authority/rights | deontic + jural + epistemic bands (claim *kinds*: asserted/believed/stated-by/contested) |
| Consistency-under-contradiction | paraconsistent + `evidential_etau` (μ/λ evidence + VC outputs) |
| **Learned inference** | `solvers/learning/kg_embedding` — TransE/DistMult/ComplEx/RotatE → **link prediction** over the aligned graph; missing relations arrive as *proposals* (epistemic), not assertions |
| **Text→graph** | `nlp/` gazetteer + relation patterns → RDF-star triples (literature ingest: Flora of Australia, BHL) |

## 5. Decentralised training (future dimension — design hooks now)

Extends `classifier-packs.md` to the semantic layer:

- **Ship the substrate, not just the answers**: packs carry vocab + graph +
  *embedding tables* (`kg_embedding` artefacts are distributable — train is a
  run-once producer at capable nodes; score/predict is cheap at the edge) +
  optional `.p64` heads.
- **The proposal flow is the learning loop**: a node runs link-prediction /
  alignment / classification → emits `proposes*`/`closeMatch` quins (with
  modelDigest + certainty) → human review ratifies/corrects → corrections
  federate as pack deltas → the shared graph *improves decentrally* while the
  human-disposition invariant holds.
- **Head-level local training**: linear heads / fine-tune over shipped
  embeddings run on-device (`classifier.rs` pattern); base training stays an
  attested artifact-producer with recorded hardware/model provenance.
- **What we will NOT do**: silent graph mutation from a model's output; an
  embedding that leaks a restricted source; `exactMatch` minted by a machine.

## 6. Namespace & PID strategy (open decision — needs your call)

| Option | For | Notes |
|---|---|---|
| `linked.data.gov.au/def/` | AU civic legitimacy, government-minted PIDs, 50-yr durability | AGLDWG offers help minting; strong for `env:`/`geo:` + AU domain vocabs |
| `w3id.org` | durable, community-standard, cheap | good default for `eco:`/`bio:`/`soc:` |
| own domain (`ecopunk.*`/`qualia.*`) | full control | needs its own 50-yr commitment |
| `ns.webizen.org` | the ecosystem's existing prefix family | consistent w/ `q42/vision/class/*` IRIs already in vision crate |

All four can coexist under one `eco`-family brand; the *rule* is: minted IRIs
must be dereferenceable-or-stable, versioned, and never repurposed.

**Amended by `durability-and-distribution.md`:** HTTP PIDs are *durable aliases*,
not canonical identity — canonical identity is content digest + `did:`
identifier, with HTTP resolution recorded as timestamped provenance edges.
Register `/def/` + w3id PIDs for human-facing discoverability, but the graph
must never *depend* on DNS.

## 7. Processing-checklist per LOD source (folds into pipeline S0–S9)

1. Register in `source_catalogue` descriptor (`Unverified` until licence+schema review).
2. Acquire vocab/dataset snapshot → integrity hash + retrieval date + version.
3. Normalise namespaces (bounded prefix registry; map to L-layer).
4. Align: `ontology_align` proposals → human ratification → asserted mappings.
5. SHACL profile for the source's claimed shapes → validation receipts.
6. Compile `.q42`; record `asset_envelope` (upstream release, toolchain, licence union).
7. Pack with *its vocabulary closure* — the pack carries the vocab versions it depends on.
8. Publish w/ VoID-style self-description → the packs become LOD citizens themselves.

## 8. Open questions

1. PID strategy — §6 (your call).
2. `OntoLex-lemon` vs `SKOS-XL` for concept-in-language — likely **lemon**
   (lexical entries + senses + language), verify against the linguistic LOD
   conventions.
3. Whether endpoint-federation (live SPARQL) is a first-class interface or
   ingest-only (lean: ingest for governance; live-query optional layer).
4. Wikidata scale — WDQS live vs curated dump slices (slice-per-domain, e.g.
   `species+places+chemicals` subsets).
5. VoID/DCAT emission format for our published packs — the LOD self-description
   story.
