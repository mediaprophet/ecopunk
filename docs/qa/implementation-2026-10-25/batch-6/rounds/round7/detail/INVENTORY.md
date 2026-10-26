# Round 7 native texture/detail inventory and proposed contract

Status: source inspection only; no implementation, execution or human acceptance is inferred.

Canonical Core owner `render/surface_authoring.rs` exposes `generate_surface_maps_into(SurfaceRecipe, side, colour, normal, orm)` and `encode_surface_png_into(side, rgba, out)`. The map builder accepts square 8–1024 RGBA8 outputs, caller buffers, validated 0..1 factors and <=128 integer bands. It generates seeded tileable artistic colour/tangent-normal/ORM detail. It is not imported-height baking or calibrated physical material measurement. The PNG encoder already serves native preview and avoids a Poet codec.

`hypermedia_authoring::image` exposes `ImageDocument::new/add_layer/composite`, `ImageLayer::fill/set_pixel/set_opacity/set_visible`, `BrushStroke::new/add_point/apply_to` and `apply_filter`. The operations write pixels. `ImageDocument::composite` currently ignores blend mode/mask/offset and uses approximate straight-alpha accumulation. Any bounded adapter must reject unsupported layer settings and partial-alpha base images rather than imply full PhotoCraft compositing. Brush samples are discrete dabs; no path interpolation is performed. Preserve that truthful scope in controls.

Existing `Image.*` host dispatch in `poet_host/invoke/hypermedia/mod.rs` produces sketches and receipts; it does not retain the pixel document for the native project. The Vibe `n9_image_editing.vibe` fixture alone is not durable native editing evidence.

Poet `surfaces/resources.rs` already owns immutable authorized HMC snapshot admission, canonical resolve/decode, resource budgets and actual GPU residency. Native surface roles retain sRGB base colour/emissive/ramp versus linear normal/ORM/AO conventions, with normal transform restrictions. Reuse this resource owner and existing checkpoint exact texture union. Source must never select a filesystem/network path.

Craft currently contains `geom` and `authoring` crates, without raster/paint APIs. Existing PhotoCraft `crates/paint` provides recorded `Stroke`, `StrokeRenderer`, `render_stroke`; `crates/codecs` provides depth/metadata-preserving bounded decode and encode. They are recovery/migration owners to retain, not permission for a separate PhotoCraft UI or a new Poet codec. ArtCraft inspection found provider/frontend image generation controls, not a qualified native pixel owner.

Proposed focused stage: `surfaces/detail/{mod,recipe,render,tests}.rs`, `authoring/detail_controls.rs`; root owns route/source/member integration. Rebuild from a strictly validated editable source recipe and immutable base digest, with caller-selected byte/work budgets. Return standard PNG `TextureDependency` and explicit base/recipe lineage. Preserve old keys/metadata and bind new digests through existing native resource/checkpoint owners. No hidden cache or authority/revision owner. Actual cooperative/reopen/GPU checks remain supervisor-owned.
