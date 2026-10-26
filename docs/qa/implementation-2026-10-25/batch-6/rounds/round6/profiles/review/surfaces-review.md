# Independent surface source cross-review

Review scope: the 12 surface candidate files in `surfaces/manifest.json`, plus
their actual canonical role, decoder, resource, tangent and GPU consumers. The
[candidate pins](surface-source-pins.json) all match the final reviewed manifest
(12/12). [Canonical dependency pins](surface-canonical-pins.json) record the read
owners. Root's unfinished common source/authority/resource integration is outside
this review. No build, test, GPU execution, GUI or live sibling write occurred.

**No remaining concrete blocker found in this bounded candidate after the two
repairs below.** This is source review, not execution qualification or acceptance.

## Findings and repaired disposition

1. **Normal-map slot transform versus tangent frame.** The initial candidate
   admitted arbitrary normal-slot rotation and reflection. Core's actual
   `gpu/mesh_upload.rs` derives tangent frames from the supplied vertex UV0;
   `shaders/viewport/mesh.wgsl:344` samples the separately transformed normal UV
   and applies unchanged tangent/bitangent directions. A rotated/reflected slot
   therefore cannot establish correctly oriented world normals. The repaired
   `surfaces/draft.rs` now refuses a bound normal slot when rotation is nonzero
   or either scale is nonpositive. This shared validator covers source material,
   strict serde DTO and direct material-resource/render admission. Added source,
   DTO and direct-draft negatives are present. Global part UV transformations
   remain valid: they occur before the canonical tangent generator. The broader
   transform-aware normal-slot capability remains a Core qualification task.

2. **Unsupported legacy single-mesh promotion.** After material projection gained
   texture records, `materials::authored_scene` initially returned textured
   single-mesh envelopes without its old factor-only boundary. The legacy
   MeshPreviewProvider's preflight could accept this output, then its renderer
   would reject it after source promotion. The repaired envelope path calls
   `validate_factor_material(&material)` before return; a direct envelope refusal
   case is present. Resource-aware textured output stays on the assembly route.

Both repairs were made by the surface owner and re-inspected in the pinned
candidate. No passing test is inferred from those changes.

## Other reviewed paths

Six fixed map roles enforce explicit transfer/channel meaning, sampler state,
digest and UV0 transforms. Original encoded resources resolve through canonical
SHA-verified HMC keys; decode and transfer checks precede pixel use. Shared ORM
bytes retain one dependency while each semantic/color-space interpretation has
its own residency key. Texture dependency closure returns original bytes rather
than replacing editable/source resource payloads with a preview re-encode.

Planar projection uses Core's metric operator on part-local positions before
assembly TRS. Part UV affine edits preflight the entire stream before mutation;
the original source UV stream remains retained. Assembly flat-normal expansion
keeps UV vertices aligned. Rendering passes current UV0 and normals to Core's
canonical UV-aligned tangent generation and uses the admitted material/range set.

Resource preflight bounds package, unique encoded data, aggregate decoded data,
map uses, dimensions and the declared decoded/mip/codec-scratch charge. Actual
GPU residency/upload staging also uses Core's separate global VRAM ledger. This
review does not establish whole-process memory or workload performance; it does
not turn the per-tool working-charge scope into a production-wide budget proof.

The ignored GPU case has an executable explicit selector:

```text
graphical::surfaces::pixels::canonical_maps_and_uv_transform_change_actual_native_pixels
--ignored --exact --nocapture --test-threads=1
```

It requires an actual offscreen GPU, uploads decoded semantic mips, draws native
materials, reads the framebuffer and requires changed pixels for all six roles
and a UV offset. Missing GPU and failed upload/readback panic; there is no return
or skip fallback. Root must execute it explicitly before citing a GPU pass. Its
existing checks establish visible changes, not fidelity or human acceptance.

Root still owns expected-source/resource epochs, trusted ReadSource-bound HMC
selection, every pre-promotion/rebuild call, preview snapshot propagation,
compiled dependency retention, save/reopen and contributor/reviewer execution.
Those integration requirements are not cleared by this leaf review. Assets
remain HOLD; IP-03/FND and film/open-world-game/cartoon completeness remain open.
