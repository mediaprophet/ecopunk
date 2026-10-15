# Media asset types — beyond the 3D mesh

**Status:** draft v0.1 · pack section conventions for non-3D media:
bioacoustic audio, field video, documents/plates, photographs — each with its
own fidelity truth layer, fragments model, and provenance requirements.

The 3D asset matrix lives in [style-and-scale.md](style-and-scale.md); this
doc covers the media that *isn't* a model — and where the spectral-truth rule
(tensor-10d §5) applies: **the captured signal is the truth; every render is
a projection.**

---

## 1. The media-type register

| Type | Truth layer | Pack sections | Fragment model | Classifier hook |
|---|---|---|---|---|
| **Bioacoustic audio** (calls, soundscapes) | STFT/CQT spectral sheet + linear amplitude + `μ` carrier (`audio/` substrate: `stft`, `cqt_bake`, `tf_surface`, `audio_spectral_sheet`) | `media/audio/*` + `graph/observations.q42` + spectral sheets | `ma-ont` temporal fragments (call onset/offset) | `proposesClass` per call → G2 species-call classifier |
| **Field video** | full-rate capture (or lossless master) + derived proxies | `media/video/*` + proxies | `ma-ont` temporal + spatial + track fragments (`hasTrackFragment`) | detection tracks → observation quins |
| **Still photographs** | highest-fidelity capture (raw or lossless) + derived JPEG proxies | `media/stills/*` + proxies | `ma-ont` spatial fragments (ROIs) | grid detector proposals |
| **Documents / plates** (field guides, botanical plates, papers) | page scans (TIFF-class masters) + OCR/extraction sidecar | `media/docs/*` + `graph/extractions.q42` | page/figure region fragments | text→graph extraction proposals (`nlp/relation`) |
| **Specimen scans** (micro-CT, SEM, photogrammetry) | instrument-native masters | `media/scans/*` + derived `.10d` mesh sections | volumetric regions | segmentation proposals |
| **Sensor recordings** (weather stations, loggers) | raw series + calibrated series (both kept — calibration is a derivation) | `data/series/*` + `graph/observations.q42` | time-window fragments | anomaly/threshold proposals |

## 2. Shared rules

- **Truth layer is never the presentation render** — compressed web proxies
  are derivations; the master carries `αμσ`-equivalent fidelity metadata
  (sampling, bit depth, colorimetry class).
- **`μ`-lane metadata travels in-band** where the format supports it —
  DIDs, consent flags, provenance marks embedded per the spectral-payload
  design; for formats that can't carry it, the sidecar `.q42` is normative.
- **Fragments are how media joins the graph** — an observation isn't "this
  file contains a frog"; it's `media frag @t0-t1 (x,y,w,h) proposesClass
  Litoria fallax, confidence, modelDigest` — then the correction stream
  (`humanCorrects`/`humanRejects`) is the training signal.
- **Sensitivity generalises**: a photo's embedded GPS, a call's recording
  location, a video's captured faces — media sensitivity ≠ the *subject's*
  sensitivity class; both are checked (people in frame invoke consent scope
  rules even when the species is public).
- **C2PA-style provenance ladders** apply (parsed → integrity-checked →
  signature-verified → trust-chain-evaluated — honest status levels, never
  "verified" by field presence).

## 3. Per-type pipeline notes

- **Bioacoustic**: `tf_surface` gives time-freq raster surfaces ready for
  classifier heads; EcoAcoustics-class sources are Mode-C/API or bulk-dataset
  captures (earth-space acquisition modes apply to sensor streams too).
- **Field video**: track fragments let "the same animal across 40s" be one
  entity with N fragment observations — dedupe is graph work, not filename
  convention.
- **Documents/plates**: extraction proposals (`nlp/relation` shallow-pattern
  → RDF-star claims) stay proposals until ratified; the scanned page and its
  extraction are linked but epistemically distinct.
- **Specimen scans**: subject-consent/status governs (sacred-site or
  restricted-specimen scans are not commons assets by default — T5 rule).
- **Sensor series**: raw + calibrated both retained; the calibration
  transform is itself a provenance-edged artifact.

## 4. Open questions

1. Master-format policy per type (FFV1/lossless video masters? TIFF plates?
   WAV-96k bioacoustic?) — the 50-year decodability rule argues for
   openly-specified masters.
2. Whether sensor streams arrive as rolling windows (Mode-D packs) vs
   continuous feeds — likely both: feeds for field ops, windowed packs for
   the corpus.
3. Face/person detection on media ingest — does the vision pipeline
   auto-flag people-in-frame for consent review? (biosense machinery exists;
   wiring to ingest is open.)
