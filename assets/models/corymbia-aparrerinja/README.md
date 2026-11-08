# Corymbia aparrerinja model studies

First ecopunk biosphere model set for local inspection in the game-demo
`asset-review.html` studio. Select this folder with **Open an ecopunk model set**.
The six studies are mature tree, juvenile tree, leaf, flower, gumnut, and bark.
Their 18 components are real QualiaDB `.10d` meshes, compiled from the
deterministic source in `build.mjs`; `model-set.json` binds files to SHA-256
digests and presentation colours. The studio loads all components of one study
as a single review selection.

The taxon identity and dimensional targets come from
`entities/taxa/corymbia-aparrerinja/`. The geometry is an authored procedural
illustration, not a scan or a verified botanical reconstruction. The source
credential's proof text is not used as cryptographic evidence. The T2 style
label is provisional pending visual and cultural review. No locality
occurrence is inferred from this model set.

Rebuild from this repository root:

```powershell
node assets/models/corymbia-aparrerinja/build.mjs
```

The build uses game-demo's pinned QualiaDB WASM compiler. Its `.10d` files
verify with QualiaDB container checks and can be rendered in the review studio.
They currently carry a quantized mesh only: spectral material sections,
ontological declarations inside the container, geometry LOD sidecars, HMC
bundling, and the guide's five qualification gates remain separate work.
The browser's local model receipt reports actual digest, container, load, and
render evidence; it does not claim those additional gates passed.
