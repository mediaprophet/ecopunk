# Factor-only metal renderer correction

**Staged, not applied or requalified.** The real accepted-source native GPU test
changed 295 pixels for base colour and 211 for roughness, then failed because
metallic changed zero pixels. See
[original failure](../harness/renderer-original-failed-pixels.log).

The actual `mesh.wgsl` unbound metallic-roughness sample is `(0,1,0,1)`, followed
by `metallic = material.factors.x * mr_sample.b`. Therefore the unbound B channel
forces every factor-only metal to zero, regardless of the accepted authored
factor. This is a demonstrated shader multiplication error, not an inferred
lighting explanation. Actual glTF import names the slot metallic-roughness;
the mesh consumer explicitly uses G for roughness and B for metallic.

The two-file correction changes unbound G/B to neutral `1,1` and keeps the
bound fallback resource consistent. Bound maps are unchanged; factor-zero legacy
materials remain nonmetallic. The material factor is still the authoritative
metal amount; an absent map does not introduce an automatic metal surface.
The fallback resource is created once by `material_textures::create_defaults`
and selected by the actual `materials.rs` MR bind path. Normal, colour, AO,
emissive, shading model and texture admission behavior remain unchanged.

Parent integration must rebuild the canonical core (shader is `include_str!`)
and rerun the exact opt-in GPU test. Do not claim the cached original rlib
contains this correction or reload it by editing source alone. The shader
test's assertions remain unchanged; metallic must actually change readback
pixels. No native window/computer use or human material acceptance is implied.
