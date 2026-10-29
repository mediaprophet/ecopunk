# ELVIS elevation source and the live demo seam

Timothy supplied <https://elevation.fsdf.org.au/> on 29 October 2026 during the demo rebuild. [Geoscience Australia](https://www.ga.gov.au/scientific-topics/national-location-information/digital-elevation-data) describes ELVIS as a discovery/delivery system combining participating agencies' elevation and bathymetry data. [NSW Spatial Services](https://www.nsw.gov.au/environment-land-and-water/spatial-data-and-mapping/foundation-spatial-data-framework/elevation-and-depth) identifies its point-cloud and DEM products as available through ELVIS. This verifies the source route, **not coverage, resolution or rights for a selected tile**. No ELVIS dataset has been downloaded or admitted by this batch.

## Existing implementation inspected

Read-only worker inspection used Ecopunk HEAD `c1ac16c33dceac75ab77893b9ecb5e59bf8d51fc` and QualiaDB HEAD `00d551ba66068bdfb6ce7f31abc95532c747ff21`. These are repository pins, not assertions that the working trees are clean.

- `crates/ecopunk-terrain/src/bin/compile_east_coast_substrate.rs` and `substrate_phases/dem_admission.rs`: existing producer admits `data/geospatial/talbingo_64x64_elev_mm.bin`, exactly 4096 little-endian i32 millimetre samples. Missing/invalid input errors unless its explicit synthetic-fixture option is chosen. The producer supplies a fixed 64×64, 30 m GDA2020/AHD frame; it does not recover that metadata from a TIFF. Its source-admission receipt is not proof of the original dataset's georeferencing.
- QualiaDB `crates/qualia-geospatial/src/geotiff.rs`: public `compile_raster_to_m64` accepts decoded samples and an explicit frame and writes native strata chunks. The local COG/first-strip/tile extraction supports a bounded subset, including uncompressed f32/i16/u16; do not infer arbitrary compressed ELVIS COG decoding from that.
- QualiaDB `crates/qualia-core-db/src/domains/geospatial/environmental_raster.rs`: `decode_geotiff_raster_into` accepts its supported classic TIFF strip/sample layouts; `compile_environmental_rasters_to_10d` writes field sections, not a terrain mesh.
- QualiaDB `domains/geospatial/gis_io/cog_wcs_wms_wmts.rs`: COG metadata/range planning is distinct from fetching, decompressing and admitting a particular dataset window.
- The active browser and this rebuild exporter call `build_sector_10d`; its `sample_sector_elevation` in `scene_builder_10d.rs` still synthesizes relief. The separate substrate `.m64` is **not the current site scene's terrain input**. Multiple placement, water/ground and actor-height callers use this same sampler and must remain consistent when it is connected to admitted terrain.

The user's concurrently updated substrate producer/phases were inspected and preserved, not edited by this batch. Source inspection is not an executed GeoTIFF/ELVIS ingestion test.

## Concrete next terrain upgrade

1. Select a bounded area for each actual demo locality; obtain the best suitable available DEM/DTM and, separately where relevant, surface/canopy and bathymetry products. Record the individual catalogue identifier, custodian, URL, capture date, native sample resolution, nodata, CRS, horizontal/vertical datums, units, accuracy statements, licence and attribution. Do not label a DSM as bare-earth terrain or assume all products use AHD/GDA2020.
2. Retain original bytes/digest and metadata. Route supported sample decoding and reprojection through existing QualiaDB owners; inspect the selected compression/sample layout before allocating a decoder adaptation. Use bounded source windows and preserve measurement resolution when producing `.m64` and `.10d` derivatives.
3. Connect one admitted height sampler to the scene builder and all ground-contact/placement/physics consumers. Preserve a separately labelled illustrative fallback; do not silently substitute it for missing measurements. Verify scale, coordinates, known control elevations, tile seams, water datum, nodata handling and fresh browser/native consumption.
4. Store source-qualified statements in the native Q42/project graph and retain them in HMC. Preserve RDF identifiers where supplied; otherwise qualify source fields with the source domain, dataset identifier and original field path. An internally minted namespace must not masquerade as a custodian's published ontology.
5. Preserve invalid/absent observations. Any inferred replacement records its user-selectable computation method, parameters, eligible inputs, source/authority restrictions, uncertainty and derivation lineage. Median/interpolation is a declared estimate, never a replacement measurement. Private inputs remain under their existing access rules.

Acceptance requires actual source ingestion and live scene consumption, not a catalogue link, parser name, generated fixture or pre-existing `.m64` file alone.
