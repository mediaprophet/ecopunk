# Round 1 bounded mesh codec review

Independent source review, 2026-10-25. **No blocking source finding established in the final patch.** This assessment covers dequantization semantics and the public regression-test source. Compile/runtime results belong to the parent agent's checks; this reviewer ran neither.

| Current source | SHA-256 |
| --- | --- |
| `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/mesh_section.rs` | `a7b7c5ee9a09311931e88f6b393554a88c4ecf1a66dceec349d4983e2813a797` |
| `C:/github/qualiaDB/crates/qualia-core-db/tests/mesh_section_bounds.rs` | `b13138544ee5b33cf6ff201110508c84a014181f793a89a7353614c2e560de09` |

Both current hashes match the producer's `mesh_section.rs.after` and `mesh_section_bounds.rs` snapshots. Inspected provenance includes `mesh_section.diff`, the final test snapshot and `decoded-bounds-probe.json`.

The producer's probe binds the observed failure to `.10d` SHA-256 `efe475f331decf3c68176432b76a56e28bf418ebd82d0af55682c8d816ef8bee`: 239952 vertices, 79984 triangles, 199 parts. It reports 16 Z positions with q=65535 decoding to `0.8110000491142273`, beyond the stored maximum `0.8109999895095825` by `5.960464477539063e-08` (one f32 ULP). These are the producer's recorded measurements, not independent measurements by this reviewer. The inspected former f32 `min + fraction * rounded_extent` implementation explains that endpoint overshoot.

The final decoder uses actual stored endpoints, returns their exact bits for q=0/65535, handles equal bounds, and performs interior fraction/extent arithmetic in f64 before rounding to f32. Finite ordered f32 endpoints have a finite f64 difference even where f32 subtraction overflows. Interior interpolation remains ordered and uses the declared frame; no epsilon, enlarged bound or post-hoc validator tolerance is introduced. The serialized layout, quantizer and encoder remain unchanged. Interior decoded floats may differ from the former f32 calculation; this is corrected decode arithmetic, not a promise of bitwise preservation of faulty decoded positions.

The parser additionally rejects non-finite or reversed bounds with a public `InvalidBounds` error. Existing reserved-field, flag, count and truncation guards remain. This strengthens malformed-frame handling; the new error variant is a public Rust API addition. The patch does not address the encoder's existing extreme-frame f32 quantization arithmetic and must not be described as qualifying that separate case.

Public API inspection confirmed that `container_10d::mesh_section`, all imported constants/functions/errors, `render::assets` and `Mesh` are public. The tests exercise every u16 coordinate for the measured frame, sampled asymmetric/negative/zero-span/large finite frames, endpoint bits, monotonicity and malformed headers. An initial test-helper signed-zero issue was reported during review: encoder bbox recomputation can normalize the supplied endpoint bits. The final helper explicitly writes the intended min/max bytes at documented header offsets 12/24, resolving that test issue and correctly isolating decoder semantics. The extreme-frame test also explicitly patches q samples; it does not claim successful extreme-frame encoder round trips.

No source edits, builds, tests, GUI/credential operations, runtime HMC reopen or image review were performed by this reviewer. Final native source-to-package/render qualification remains pending actual checks. Film, open-world game, cartoon and overall model/package acceptance remain incomplete.
