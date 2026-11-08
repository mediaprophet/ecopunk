# Data sources — Earth & space systems (celestial · geomagnetic · solar · atmospheric)

**Status:** draft v0.1 · scope: dynamic Earth-system and space resources —
celestial catalogues, ephemerides, geomagnetic field, solar/space weather,
ionosphere, atmosphere/weather — including **API-sourced and model-sourced**
data, which follow different acquisition rules than static datasets.

**The key distinction this register makes:** not everything here is a
"dataset". These systems are *living processes* — the right acquired object
varies: **snapshots** (what was observed), **models** (WMM coefficients,
ephemeris kernels — versioned artifacts), and **capture events** (API reads
recorded as timestamped observations). §1 defines the acquisition modes.

---

## 1. Acquisition modes (dataset vs API — the answer to "which way?")

| Mode | Applies to | Rule |
|---|---|---|
| **A — Snapshot dataset** | static catalogues, reanalysis archives, released models | fetch dated artifact → hash → provenance → pack (same as biodiversity path) |
| **B — Versioned model** | WMM coefficients, ephemerides, forecast models | the **model + its coefficient version/epoch IS the artifact** — don't snapshot "answers", snapshot the model and its validity interval; evaluate deterministically later |
| **C — API capture event** | live feeds (solar wind, magnetometer minute-data, weather obs) | each fetch is an **observation record**: `{source, endpoint, request params, response, retrievedAt, contentHash}` — captured as observation quins, never asserted as timeless fact |
| **D — Rolling window aggregates** | derived from C | scheduled aggregation of C-streams into dated windows (daily/monthly/yearly compilations) → snapshot-packable; keeps `t`-axis queries honest about coverage gaps |

APIs are **acquisition surfaces, not corpus content** — what enters the corpus
is the *capture* (with full provenance), and windows of captures become packs.
This matches the engine's model exactly: adapters emit provenance quins;
`SpatiotemporalQuadTree` indexes `t0/t1`; `t` is a coordinate lane.

## 2. Already-registered endpoints (verified in `canvas_defaults.rs`)

The engine's adapter registry **already ships defaults** for much of this
domain:

- **Celestial (IVOA TAP)**: Gaia DR3 (`gea.esac.esa.int`), IRSA, VizieR,
  SIMBAD, Chandra, ESA CSA.
- **Ephemeris & minor bodies**: JPL **Horizons** API, NASA **NeoWs**,
  Minor Planet Center, SDSS CasJobs.
- **Planetary surfaces**: USGS Astrogeology WMS — **Mars, Moon, Venus,
  Mercury** basemaps.
- **Weather/atmospheric**: Open-Meteo forecast / archive / marine APIs,
  NWS (`api.weather.gov`).
- **Earth observation**: NASA GIBS WMTS (×4 projections) + WMS — includes
  atmospheric/aerosol/temperature products; STAC (LandsatLook, MS Planetary
  Computer, AWS Earth-Search).
- **Marine**: GEBCO, Copernicus Marine WMTS, EMODnet (seabed/bathymetry/
  biology/physics), NOAA coastal DEM.
- **AU administrative/physical**: full ABS ASGS suite, GA geophysical grids,
  GA earthquakes, GFW, UN Biodiversity Lab, openSenseMap.

## 3. Register — space weather & solar-terrestrial

| Source | Kind | Access | Licence/status | Feeds |
|---|---|---|---|---|
| **NOAA SWPC** — Space Weather Prediction Center | API feeds | open JSON endpoints (solar wind, Kp, alerts) | US-gov public domain, verify per-product | space-weather layer; `t`-series captures |
| **DSCOVR real-time solar wind** | API feed | SWPC JSON stream | PD | live solar-wind layer |
| **GOES space environment** | API/dataset | SWPC + NCEI archives | PD | particle/X-ray flux |
| **NASA DONKI** | API | registered key, free | NASA open | CME/flare/SEP event catalog |
| **IPS / Space Weather Services (BOM, AU)** | API/products | web products; verify programmatic access terms | AU terms — verify | AU ionospheric/auroral context |
| **ESA SSA space-weather** | portal/API | registered access | verify | EU corridor |
| **GFZ Potsdam** | datasets/API | open | verify | Kp/Dst definitive indices |

## 4. Register — geomagnetic

| Source | Kind | Access | Licence | Feeds |
|---|---|---|---|---|
| **WMM — World Magnetic Model** | **Mode B model** | coefficient download (NCEI/BGS joint) | PD | declination/inclination field — the *model is the artifact*; versioned ~5yr epochs |
| **INTERMAGNET** | observatory minute-data | registered access / open subset | per-observatory terms | ground truth for field models |
| **GA geomagnetic observatories (AU)** | dataset/API | GA services | AU terms — verify | AU field reference (AGRF) |
| **SuperMAG** | aggregated dataset | registered | academic terms — verify | global chain aggregates |
| **CHAOS / IGRF model series** | Mode B models | open | PD-ish, verify | main-field models |

## 5. Register — atmospheric & weather

| Source | Kind | Access | Licence | Feeds |
|---|---|---|---|---|
| **BOM (AU)** — observations, radar, warnings | API/products | weather API (public products; `api.weather.bom.gov.au` unofficial patterns — verify sanctioned endpoints) + WMS + OPeNDAP where offered | AU terms — verify carefully | AU weather layer |
| **Open-Meteo** | API | **registered default** (forecast/archive/marine) | open (CC-BY-ish) | rapid prototype path — already wired |
| **NOAA NOMADS / GFS** | **OPeNDAP/THREDDS** | adapter exists in-engine (`opendap_adapter`) — subsetting queries | PD | gridded forecast fields → FIEL candidates |
| **ECMWF / Copernicus CDS (ERA5 reanalysis)** | API (registered) | CDS API + licence clickthrough | Copernicus licence (attribution) | historical reanalysis — the Mode-A/B hybrid backbone |
| **NASA POWER** | API | open | PD | solar/meteorological parameters |
| **Air quality** — state EPAs (AU), OpenAQ | API | open/varies | mixed | AQ layer |
| **Ozone / atmospheric composition** — NASA OMI/TEMPO, Copernicus Atmosphere (CAMS/ADS) | API/dataset | ADS API | verify | composition layers |
| **GNSS-TEC / ionosphere** | datasets | IGS/Madrigal, GIRO Digisonde | mostly open-academic | ionospheric layer — couples to space weather |

## 6. Register — celestial beyond the registered defaults

| Source | Kind | Access | Feeds |
|---|---|---|---|
| **Gaia DR3+** | TAP | **registered** | stellar catalog |
| **JPL Horizons** | ephemeris API | **registered** | solar-system body positions — Mode B/C hybrid (ephemeris is computable; captures freeze chosen epochs) |
| **IAU Minor Planet Center** | dataset/API | **registered** | minor-body catalogue |
| **NASA NeoWs** | API | **registered** | near-Earth object feed |
| **VizieR / SIMBAD / Chandra / ESA-CSA / IRSA** | TAP | **registered** | deep catalogues |
| **USNO / IERS (Earth orientation, leap seconds)** | bulletins | open | `t`-axis correctness — the boring-but-critical dependency |

## 7. What each needs in the pipeline

- **Solar/geomagnetic/ionospheric data is `t`-series-heavy** — Mode C captures
  + Mode D windows; sensitive to *sampling honesty* (a pack must declare its
  capture coverage, gaps included — honesty-label rule).
- **WMM/ephemeris models**: keep the *model* + coefficient epoch; evaluate
  in-engine later (deterministic, offline-safe — a 2060 node can still
  evaluate a 2025-epoch WMM without the network).
- **Atmospheric grids** (GFS/ERA5 via OPeNDAP) → the **`FIEL` dense-field**
  section — same raster gap note as terrain: general gridded-field→FIEL writer
  is the open implementation item.
- **Celestial layers** feed the synthetic environment directly — sky/ephemeris
  state for game-demo's day/night, seasonal positioning, star fields.
- **Sensitivity**: generally low (physical phenomena, not species/people) —
  but note *exceptions*: observatory infrastructure locations, and space-
  weather data used in infrastructure-risk contexts.

## 8. Open questions

1. BOM sanctioned programmatic access — which endpoints are officially open
   vs. scrape-risk? (BOM access terms have history here.)
2. IPS/SWS API terms and rates.
3. Which capture cadences per feed (per minute? per hour?) — drives pack
   sizing on `t`-series.
4. Does OPeNDAP adapter cover the specific NOMADS/THREDDS URL patterns needed,
   or does it need extension work?
5. Ephemeris evaluation in-engine vs captured positions — Horizons is an API
   *service*; do we store evaluated positions (Mode C) or a local ephemeris
   model (Mode B)?
