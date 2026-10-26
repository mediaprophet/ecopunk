# Round 6 surfaces candidate

Scoped staged implementation only. Live Poet, Core and other sibling repositories
were read-only in this worker. Root owns integration, review, installation and
serial Cargo/native GPU verification. No builds, tests, GUI, production assets,
provider calls, credentials, commits or publication were performed by this worker.
`rustfmt --edition 2021 --config skip_children=true` parsed/formatted the candidate
Rust files; that does not establish type correctness or execution.

## Native contracts

- `graphical::surfaces::SurfaceDraft { material: MaterialRecord }` retains the
  canonical native visual MAT4 fields. `updates()` emits source member updates
  excluding identifier/name, for root's field-preserving assembly source adapter.
  Existing factor-only source retains its canonical defaults. Physical material
  properties remain separate. Factor-only single-mesh APIs retain their refusal
  of maps; new maps are admitted through the resource-aware assembly path.
- Six canonical map slots require explicit role, SHA-256, colour space, channel
  semantics, UV0 transform and native sampler declaration. Base/emissive/ramp use
  sRGB; normal uses linear tangent RGB; packed metallic/roughness uses linear G/B;
  occlusion uses linear R. Unknown or mismatched declarations refuse admission.
  A stylized ramp requires the Stylized shading model. Missing fields, arbitrary
  source paths/URLs, unsupported UV channels and all-zero declared digests refuse.
  Per-map normal rotation and nonpositive scale refuse until transform-aware
  tangent-frame qualification exists; positive normal scale and offsets preserve
  the current UV0 frame orientation. Part-level UV transforms remain supported
  because Core generates tangent frames from the resulting geometry UV0 stream.
- Optional part `uv_projection` uses existing Core
  `render::surface_authoring::planar_uvs_into`, in part-local metres with XY/XZ/YZ
  axes and explicit metres-per-repeat. It overrides stored authored UV0 without
  changing that source; removing it restores stored UV0. Optional `uv_transform`
  then applies offset + rotation(scale × UV). Nonidentity transforms require UV0;
  actual texture use always requires authored or explicitly projected UV0.
- `TextureResources::from_hmc_bytes(bytes, limits)` owns an immutable bounded HMC
  snapshot. Root must supply the current authorised package selection; this type
  performs no filesystem/network lookup or authority grant. `validate(materials)`
  actually decodes referenced data, not just headers. `dependencies(materials)`
  returns canonical `TextureDependency` original encoded bytes for HMC closure.
  `upload(materials, PortalGpu)` decodes into bounded caller-owned buffers and
  performs real semantic mip uploads before mesh binding. It reuses Core HMC key
  resolution, image signature/decode, tangent generation, mip/residency and MAT4
  APIs; no codec, tangent engine or renderer is duplicated.
- Default resource bounds: HMC snapshot 64 MiB, unique original encoded data
  16 MiB, decoded pixels 64 MiB, decoded/GPU-mip/scratch working charge 192 MiB,
  64 map uses and 4096 × 4096 dimensions. These are caller-selected tool bounds,
  not master-quality ceilings. Aggregate preflight precedes decode/upload.
  Transcode-only payloads, corrupt/missing data, SHA mismatch, incompatible known
  KTX2 transfer functions and exhausted bounds refuse the operation. PNG/JPEG
  unknown transfer metadata requires the explicit source role declaration.
- `SurfaceDraft` has strict owned serde intent DTOs, including exactly six
  optional map bindings and explicit role/colour-space/channels. Unknown nested
  or top-level fields refuse deserialization. No source text is evaluated for
  intent conversion. `wire::UvTransformWire` provides the canonical four field
  names and validated conversion; `PlanarUvDraft` has strict serde fields.
- `assembly::render_assembly_with_resources(source, assembly, settings, limits,
  probes, Option<&TextureResources>)` checks current SourceRef before/after and
  uses actual uploaded pixels. Legacy render entry points delegate with no
  resources and refuse textured assemblies. Root must export this new function.

## Required root integration

Declare `graphical::surfaces` and `authoring::surface_controls`. Add typed
`Surface`, `UvTransform` and `UvProjection` authoring edits and PartView surface,
UV transform/projection fields as communicated. Mount `SurfaceControls` through
the existing revision/actor/worker guarded edit callback. The controls pin their
loaded fields to selected part and inspected SourceRef, and use the same native
intent path rather than evaluating source themselves.

Root binds trusted HMC package snapshots through current native source/resource
read authority, expected SHA-256 and epoch invalidation. Root must call
`resources::validate_material_resources` in every pre-promotion/rebuild path,
forward the same snapshot to preview, and include `dependencies()` in compiled
assets/checkpoints. Keep exact current source and compiled witnesses; resource
selection is not authority and must not restore an execution session on reopen.
Save/reopen and separate contributor/reviewer proof belong to root integration.

## Authored checks, unexecuted

- `graphical::surfaces::tests`: all MAT4 factors/flags and six bindings through
  pinned Vibe evaluation; role mismatch; canonical planar metres and affine
  refusal; actual decode/dependency closure through native `.10d` material/HMC
  packaging and reopen; missing/corrupt/transfer mismatch; aggregate resource and
  binding budgets. Test textures are memory-only synthetic 2 × 2 KTX2 data.
- `graphical::authoring::surface_controls::tests`: loaded material fields survive
  control parsing; a source path cannot become a texture digest.
- Explicit serial GPU selector:
  `graphical::surfaces::pixels::canonical_maps_and_uv_transform_change_actual_native_pixels`
  with `--ignored --exact --nocapture --test-threads=1`. It requires actual GPU
  residency/readback for all six maps and a UV transform, and fails on unavailable
  GPU or unchanged output. Ignored source presence is not passing evidence.
  When root explicitly supplies `POET_SURFACE_TEST_ARTIFACTS` as an absolute,
  existing owned directory, the test optionally retains its already-read pixels
  using `preview::native_image_uri` plus existing base64 decode. It writes at most
  fourteen new 64 × 64 PNGs (each role before/after plus UV before/offset), capped
  at 32 KiB each and 512 KiB aggregate, with fixed filenames and no overwrite.
  Each frame is bound to an actual current ephemeral calibration record carrying
  its material DTO and pixel SHA-256. These records identify direct GPU test
  evidence, not an authored project, human review or model acceptance. No image
  is rerendered for retention; absent the environment selection no files are
  retained. The worker did not execute this optional path.
- Retain material/assembly/authoring/native intent regressions; root adds actual
  authorisation, stale-source/resource epoch, source save/reopen and texture
  dependency retention/review tests, then native/WASM compilation as applicable.

All readiness gates, human native usability acceptance, complete embodiments and
film/open-world-game/cartoon targets remain open. Assets remain HOLD. This packet
does not claim a complete UV toolset (atlas/unwrapping/other channels), physical
material system, high-grade output or any model/package completion.
