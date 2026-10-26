# Native GLB skin import and BC7 precision regressions

Staged proposal only. No sibling source was written by this worker. Root owns
review, application, coherent compilation and the GPU lanes. All six target
files are guarded by `apply-manifest.json`; new files require absence.

## GLB: actual production defect

The unchanged canonical fixture contains a default root translation of one metre,
a child translation of half a metre, identity inverse-bind matrices, and a
non-cyclic translation animation. The original importer uses those default node
transforms as native bind-rest transforms and unconditionally marks the clip
looping. Both conflict with SKL1 validation. It then discards the encoder error
and returns a scene with no SKL1 section. The baseline cached-engine probe
reproduces this. Correcting only either invariant still fails native encoding.

[glTF 2.0 skins](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#skins)
keep inverse-bind matrices distinct from the node's default/current transforms.
The proposal derives native bind-rest transforms from the actual authored
inverse-bind matrices, which it preserves; missing matrices use the glTF identity
default. The source default pose is preserved in explicit constant animation
tracks for unanimated joints. Clips are nonlooping because glTF does not assert a
loop. Source joint indices are topologically remapped, including vertex weights,
without changing the legacy joint identifier spelling. SKL1 and MRP1 encoder
errors now return explicit import errors instead of silently losing sections.

The rigid native slice preserves pure LINEAR translation or rotation channels
with fixed other components. Motor Sclerp differs from independently varying
glTF translation/rotation; combined varying channels, STEP/CUBICSPLINE joint
channels, scale/shear/reflection, multiple skins, transformed mesh instance
frames, sparse skin/animation accessors and additional influence sets are
explicitly refused. These are visible capability gaps, not successful imports
with lost motion. Node hierarchy, positive finite weights, native budgets and
skin/animation accessor ranges are checked before conversion. Unchanged morph
and KHR parsing remains outside a universal glTF compliance claim.

Six real-source regression tests validate SKL1 roundtrip and actual native CPU
pose/palette/linear skin playback against the glTF linear blend equation,
including default pose, reversed joint order, authored nonidentity inverse binds,
nonlooping end behavior and malformed/unsupported input refusal. The source
default pose remains present in `nodes`; deriving native bind-rest data does not
replace it with a synthetic source pose.

CPU skinning is linear blend; the separate native GPU skin path currently blends
dual quaternions. GPU equivalence for multiweight rotating joints is an open
qualification issue. This proposal does not claim to repair that distinct path.

## BC7: proven incorrect test assumption

The original arbitrary payload produces alpha zero in modes 4–7 because their
explicit alpha endpoints are zero. These are valid transparent colored blocks,
not the reserved-zero fallback. Modes 0–3 have implicit opaque alpha. This follows
the primary [Khronos BPTC format](https://raw.githubusercontent.com/KhronosGroup/DataFormat/main/bptc.txt)
and [Microsoft BC7 format](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/bc7-format).
The baseline probe records exact output for all eight modes. No production BC7
decoder defect was demonstrated, so its implementation is unchanged.

The failed test name is retained, with exact opaque-white all-mode and explicit
transparent-color mode 4–7 vectors. Separate tests check exact partition-zero
subset colors and P bits, dual-index selection and all component rotations,
and every four-bit interpolation weight with the mode-6 fix-up index. Their
test-only bit writer uses the published format table and independent expected
RGBA values, not decoder tables/helpers. The existing ETC2/ASTC test is retained
unchanged when extracting the tests into a focused file. No threshold is relaxed.

## Executed evidence and remaining work

`harness/baseline-tests.log`: two real cached-core diagnostic probes passed while
recording the original section loss and legal transparent BC7 results.
`harness/candidate-tests.log`: 11 tests passed, zero failed/ignored. The harness
compiles the exact proposed modules with actual current engine types, geometry
import, SKL1 encoder/reader, pose/palette/skin execution and KTX2 parser. The
legacy private VEC3/VEC4 readers are copied verbatim into the harness module;
this is a visibility adapter, not replacement engine behavior. The initial
private-reader re-export compilation failures remain in the working history;
the final compiler log and exact command are retained with hashes.

No Cargo/core build, GPU run, Computer Use, paid call, commit or publication was
performed for this repair. Root must run the unchanged canonical GLB test and
full rebuilt native suite. The model/package, film/game/cartoon targets, IP-03
usable cooperative-authoring acceptance and all 120 broad production packets
remain incomplete; these bounded regressions cannot close them.
