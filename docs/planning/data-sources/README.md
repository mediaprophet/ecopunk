# Data sources — method & register index

**Status:** draft v0.1 · **Date:** 2026-10-12

How ecopunk discovers, evaluates, acquires, and tracks external datasets and ontologies.

## Regional registers

| Register | Scope | Status |
|---|---|---|
| [australia.md](australia.md) | AU-first discovery — aggregators (ALA/data.gov.au), federal science (DCCEEW/NSL/DEA/BOM), NCRIS infra (TERN/IMOS/Microbiome), state atlases, collections & traits, media, LOD ontology layer, format inventory, AU sensitivity & sovereignty rules | **Active discovery** |
| [earth-space.md](earth-space.md) | Dynamic Earth/space systems — celestial (Gaia/Horizons/NeoWs/MPC), geomagnetic (WMM/INTERMAGNET), solar & space weather (SWPC/DONKI/IPS), atmospheric (BOM/Open-Meteo/ERA5/GFS), ionospheric — with the dataset-vs-API acquisition-mode model | draft v0.1 |
| [socioeconomic.md](socioeconomic.md) | Socioeconomic & official statistics — ABS Data API (SDMX 2.1)/ASGS/SEEA accounts, AIHW, ATO/RBA/ABARES, classifications, international systems (UN SDG, World Bank, OECD, ILO, FAO, WHO, OWID), demographics & vital statistics (UN WPP, ERP, censuses), wage/PPP/living-wage benchmarks, MRIO footprint tables, restricted microdata tier, claims-vs-facts sources | draft v0.1 |
| `global.md` *(planned)* | GBIF global, Catalogue of Life, NCBI, WoRMS/OBIS, IUCN, Open Tree of Life, Wikidata, iNaturalist, WorldClim/CHELSA, ESA/Copernicus, WDPA, MorphoSource/Sketchfab/Smithsonian-3D, OBO ontologies | Not started |
| `regional/*.md` *(planned)* | Other regions once the AU pattern is proven | Not started |

## Evaluation rubric

Each candidate source is scored on:

1. **Authority** — is it the authoritative origin or an aggregator? (NSL > ALA > GBIF for AU names.)
2. **Access mechanism** — open download / API / registered / request-only / restricted.
3. **Format** — RDF-native (best) → DwC-A/CSV/JSON (mappable) → geospatial (transform needed) → binary/dense (sidecar needed) → scrape-only (flag risk).
4. **Licence** — tier per [licensing.md](licensing.md): P0 PD/CC0 · P1 attribution · P2 share-alike · P3 non-commercial · P4 restricted/registered. Record-level licences override portal-level claims.
5. **Identifier durability** — does it mint persistent IDs (DOI, LSID, GUID, PID-register URI) we can cite for 50 years?
6. **Update cadence & versioning** — releases, DOI'd snapshots, changelogs.
7. **Sensitivity** — does the source already obfuscate (SNES grids, ALA SDS, BioNet tiers)? We inherit their classifications.
8. **Sovereignty** — TK/BC labels, CARE-principle obligations, community authority.
9. **Fit to layers** — which ecopunk layer(s) it feeds (names backbone / occurrences / traits / environment / media / models).
10. **Pipeline cost** — estimated effort to normalize into the ontology stack.

## Acquisition workflow (mirrors QualiaDB's acquisition-inbox pattern)

```
discover → register row (this file family)
        → fetch dated snapshot → acquisition-inbox/<source-slug>/
        → manifest record: { sourceURL, retrievalDate, contentHash, licence,
                             accessTier, records, coverage }
        → human review (licence, sensitivity, quality, fitness)
        → normalize to vocab stack → SHACL-validate → compile .q42
        → promote to curated corpus + dataset manifest entry
        → publish lane decision: permissive-commons | restricted | never-redistribute
```

Rules (non-negotiable, adopted from QualiaDB's inbox convention):

- The inbox is **staging, not corpus** — nothing is queried or shipped until reviewed.
- A fetch run writes a `MANIFEST.md` listing everything fetched plus failures with reasons —
  never a fabricated stand-in.
- Raw snapshots are kept immutable under the authoring area; published packs contain only
  reviewed, licence-cleared derivatives.
- Sensitivity is inherited from the source's own classification, then re-evaluated — we may
  classify *stricter* than the source, never looser.
- Record-level provenance (per-record licence, attribution, identifiers) survives into the
  normalized graph; portal-level licence claims are not sufficient evidence.

## Key precedents to reuse

- `qualiaDB/core-ontologies/acquisition-inbox/` — the staging/review/promotion pattern.
- `game-demo/docs/art/australian-ecology-data-contract.md` — a working AU ecology data
  contract (ALA occurrence filtering, IBRA/NVIS context layers, DOI preservation, sensitive
  coordinates, evidence status). Ecopunk data contracts should follow this shape per source.
- `qualiaDB/manifests/ontologies.json` — the registry JSON format for downloadable ontology
  entries (id/name/version/type/size/url/filename) — extend with licence tier + provenance fields.
