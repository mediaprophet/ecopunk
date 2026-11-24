# Species artifact process

**Status:** working notes from *Corymbia aparrerinja* (ghost gum), 2026-11-17.  
**Purpose:** uplift this run into a repeatable process for every living taxon.  
**Companions:** [species-embodiment-spec.md](species-embodiment-spec.md), [evidence-led-asset-production.md](../evidence-led-asset-production.md), [data-sources/README.md](../../data-sources/README.md), Synth producer [living-world-habitat-hmc-task.md](../../../../../synth/docs/production-core/templates/living-world-habitat-hmc-task.md).

The encyclopedia is an **RDF index**. Each species is a **repo (or folder) from one template**. One **species HMC** is the model. Habitat instances are individuals (stage + parameters + transform). GitHub Releases carry signed bytes. JSON catalogs are projections only.

---

## 0. What a species artifact is

| Object | Identity | Storage |
|---|---|---|
| Taxon concept | `urn:qualia:taxon:{slug}` plus APC/GBIF/NSL evidence | `taxon.ttl` |
| Working index | DCAT members, gaps, latest version | `index.ttl` |
| Organism template | stage + FSPM + root + organ params | same RDF + `.vibe` |
| Individual | template + seed/params + scene transform | habitat `scene.json` pins URN + stage |
| Release | frozen `index.ttl` + `index.q42` + members + `.sig` | GitHub Release |

Do not mint a new HMC or repo per tree. Do not invent `InstanceState64`, per-asset WGSL, or HTTP CAS.

Made-world (vehicles, camp) uses a different template.

---

## 1. Gate: evidence before model

Production rule `biosphere-evidence-before-model` (`ecopunk/docs/production-rules.json`):

1. Read `docs/planning/data-sources/` (AU spine first).
2. Bookmark landing pages, SPARQL endpoints, RDF dumps (`synth bookmark add URL` or MCP `synth_bookmarks`). Secrets stay in `.synth/secrets.json`.
3. Fetch with Synth jobs (`synth job add URL --kind ingest` or MCP `synth_jobs`). Snapshots land in `.synth/inbox` with a PROV/DCT/DCAT wrap. Compile Turtle/N3 with `synth_compile_q42` (or `kind=ingest` / `kind=compile`). Query with `query_sparql vault_path`. The operator ticks due jobs; inbox is staging until review.
4. Only then `fspm_vascular_grow`, `compile_botanical_organ_10d`, `assemble_corpus`, `synth produce`.

Fiction taxa skip the gate. Non-fiction biosphere never does.

---

## 2. Datasource register (bookmark these families)

From `docs/planning/data-sources/australia.md`. For a vascular plant, bind at least:

| Layer | Source | Typical URL |
|---|---|---|
| Names | NSL / APC / APNI | `https://id.biodiversity.org.au/taxon/apni/…` |
| Names aggregator | ALA BIE | taxon page + `https://api.ala.org.au` |
| Global names | GBIF | `https://www.gbif.org/species/{key}` |
| Occurrences | ALA Biocache / GBIF DwC-A | APIs; record-level licence |
| Traits | AusTraits | `https://austraits.org` (CC-BY 4.0) |
| Descriptions | Flora of Australia / EUCLID | `https://profiles.ala.org.au`, Lucid EUCLID |
| Ecology RDF | TERN GraphDB + TERN ontology | `https://graphdb.tern.org.au`, `https://w3id.org/tern` |
| Media | ALA images, TERN EcoImages, Wikimedia Commons | per-image licence + attribution |
| Environment | IBRA/NVIS, BOM/SILO, DEA STAC, SLGA | DCAT/STAC; not scraped HTML |
| Country / TK | AIATSIS, Local Contexts TK/BC | never treat Country as cadastral |

Optional for woodland scenes: AusPIX, Geofabric, OSM `osm2rdf`, Wikidata SPARQL.

Bookmark **before** fetch. Classify `kind`: `sparql` | `rdf` | `rest` | `csv` | `page`. Fill `data_endpoint` when the user pasted a docs URL. Mark `requires_api_key` without storing the key.

---

## 3. Media: find, then generate

Two lanes. Keep them labelled.

**Evidence stills** (cite, do not invent):

- Wikimedia Commons category for the taxon (CC-BY / CC-BY-SA; keep author + URL).
- ALA image service, TERN EcoImages, CSIRO ScienceImage.
- Store under `refs/` with `refs/sources.ttl` (`dct:license`, `dct:creator`, `dct:identifier`).
- Use for bark colour, crown void, lignotuber, leaf, flower, habit.

**Modelling stills** (generated, not evidence):

- `image_gen` / `image_edit` for missing angles (root plate, wind load, scale-4 leaf).
- File name prefix `generated-`. RDF: `dct:type` modelling-still; never `prov:wasDerivedFrom` a field observation.
- Compare produce previews (`front/side/three-quarter-view-assumed.png`) against both lanes. A triangle count is not a review.

---

## 4. RDF layout in the species folder/repo

```text
taxon.ttl              # dwc:Taxon, APC/GBIF, sourced claims
index.ttl              # dcat:Dataset, members, gaps, organism template
sources/*.ttl          # extra graphs
refs/sources.ttl       # media provenance
*.vibe                 # generator (one species model)
.synth/bookmarks.json  # public datasource metadata
```

Compile `taxon.ttl` → `taxon.q42`. Release copies `index.ttl` + `index.q42` + SHA-256 + Ed25519 (`intenteco.species.sha256.v1` when that key exists).

Encyclopedia harvests each species `index.q42` and unions the graph. SPARQL is the catalog.

---

## 5. One HMC, many individuals

VibeScript calls admitted morphisms only.

| Member | Morphism | Notes |
|---|---|---|
| Vascular skeleton | `hypermedia.manifold.fspm_vascular_grow` | Age via `generations` (1..=6) + `gdd_accumulated`. Form via `base_radius`, `base_length`, `branch_angle_deg`, `turgor_kpa`. |
| Root / lignotuber | `hypermedia.manifold.revolve` | Tap + swell + collar. Fixed XPBD body. |
| Laterals | `hypermedia.locus.construct` + `extrude` | Shallow plate. |
| Organs 1–4 | `hypermedia.manifold.compile_botanical_organ_10d` | Shared; instance onto FSPM. `species_id` non-empty. |
| Wind | `hypermedia.kinetics.step_island_xpbd` | Lignotuber `inv_mass: 0`. Area3D `wind` + gravity + damping. `ground_y=0`. |
| Seal | `compile_10d` / `compile_10d_pbr` | MAT1 multiplier `[1,1,1,1]`. |

Boolean-union of open FSPM tubes with the root plate can exceed budget (`E701`). Keep **named members in the same HMC** and composite in the template until a watertight union fits.

Packed example (ghost gum): `individuals/{seedling,sapling,mature,ancient}-skeleton.10d`, `root/*-lignotuber.10d`, `root/lateral-*.10d`, `organs/{whole-organism,bark-wedge,floral-gumnut,leaf}.10d`.

Habitat pins `urn:qualia:taxon:…` + stage + transform. GPU `index_count` = authored triangles × 3.

---

## 6. Produce, present, review

Producer: `C:\github\synth\target-prod\debug\synth.exe`. Do not rebuild `target` / `target-studio`. Do not kill Studio MCP.

```powershell
$synth = "C:\github\synth\target-prod\debug\synth.exe"
$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$run = "C:\github\synth\target-prod\runs\{taxon}-$stamp"
$env:SYNTH_INPUT_ROOT = "C:\github"
$env:SYNTH_BOOKMARK_ROOT = "{species-dir}"
& $synth produce --profile files --source "{species-dir}\{taxon}-model.vibe" `
  --out "$run.hmc" --export-dir $run --present "$run.html"
Get-ChildItem $run -Recurse -Filter *.10d | ForEach-Object {
  & $synth preview --asset $_.FullName --out "$run-preview-$($_.BaseName)"
}
```

MCP: `synth_produce_present` (same source; `present_html` is the showcase). Operator: `GET /operator` and `GET /project` when `--project` is the species dir.

Review: read the three assumed PNGs **and** the evidence stills. Log gaps in `index.ttl` (`rdfs:comment` on the working dataset).

---

## 7. Release (when the twin is qualified)

1. Freeze `index.ttl` with `spdx:checksum` per member.
2. `synth_compile_q42` → `index.q42`.
3. Sign each artifact (same Ed25519 bind as Synth releases).
4. `gh release create` on `intenteco/{slug}`.
5. Encyclopedia harvests `…/releases/download/{tag}/index.q42`.

Until `intenteco` exists, keep the working twin at `ecopunk/entities/taxa/{slug}/`.

---

## 8. Checklist for the next taxon

- [ ] Taxon URN + APC/GBIF/NSL in `taxon.ttl`
- [ ] Bookmarks from data-sources (names, traits, SPARQL, media)
- [ ] Evidence stills in `refs/` with `sources.ttl`
- [ ] Modelling stills prefixed `generated-` if an angle is missing
- [ ] `.q42` sealed and SPARQL-inspected
- [ ] One `{slug}-model.vibe` (FSPM ages, root plate, organs, XPBD)
- [ ] Produce `--present` HTML via CLI or `synth_produce_present`
- [ ] Preview PNGs read against stills
- [ ] Gaps recorded in `index.ttl`
- [ ] Habitat pin only after visual acceptance

First worked example: `entities/taxa/corymbia-aparrerinja/` (ghost gum).
