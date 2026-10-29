# White-gum browser review — r1 rejected; authored r2 staged

Timothy rejected the actual r1 browser output; see `dossier/producer-visual-review-r1.md` and preserved `previews/whole-tree-r1.png`. The next staged candidate is the runtime authored graph in `tools/species-production/white-gum-review-r1/white-gum-r2.vibe`, with the adapter and exact integration details in `authored-r2-handoff.md` beside it. It adds explicit root axes and distributed branching using existing geometry owners. r2 has not been rendered or accepted by this worker; root-soil-wind coupling and joined fork surfaces remain unresolved. The following sections preserve r1's implementation history rather than describing r2 as delivered.

The current candidate adapts the later working `botanical_field_builder.rs` used by the user-preferred Talbingo scene. The staged source is `tools/species-production/white-gum-review-r1/botanical_field_builder.rs`; the integration entry is `build_white_gum_review_specimen(trunks, foliage)`. The parent coordinator integrates it into the existing Ecopunk WASM browser application. No separate authoring shell is introduced.

Working taxon concept: *Eucalyptus dalrympleana* subsp. *dalrympleana*. Exact individual, site, age and field measurements remain unresolved. This is an authored 6 m synthetic candidate; its scaffold/proportions/colour/seed are modeling choices. Adult blade dimensions are chosen within the referenced ranges. See `dossier/references.json`.

## Concrete adaptation

Retains current native tapered-branch-field collar calculation, Bishop swept bole, primary/secondary branches and the existing falcate leaf owner. Adds a distinct mountain-white-gum profile, pale bark, a slimmer authored bole and smaller basal spurs. For this candidate only, replaces oversized inner-crown fan lobes with actual attached leafy shoots, adds previously absent petiole geometry, narrows twig radii and uses green apical foliage. Other species guilds retain their prior path. Generic oversized flowers/fruit are suppressed because this candidate's reproductive state has not been established; that requirement remains missing.

All geometry is in metres at ground-collar origin with +Y up. The 6 m input is a nominal stature parameter, not a measured final bound. The browser must compute emitted bounds and frame the actual mesh. Expected source-derived working envelope is approximately x/z within ±6 m and y −0.02..8 m, subject to actual output measurement.

## Review

The task is not delivered merely because source was staged. Actual browser selection, visible whole-tree and closeup output, independent coordinator review and Timothy/contractor review remain required. The user and designated reviewers control acceptance. No visual improvement or publishable status is asserted before those outputs are inspected.

All 193 catalogue obligations, the expanded cases, TREE-01–13 and film/open-world-game/cartoon targets remain in `dossier/claims.json`. The candidate is static surface work. Missing work includes actual specimen identification, juvenile foliage, roots/anatomy, diagnostic reproductive organs, wind/behaviour, complete species knowledge/resources, target-specific derivatives and acceptance. Field-based collar evaluation does not itself prove fused watertight topology.

## Preserved scratch

`authoring/` contains an earlier independent native helper/source experiment. It is superseded for this task and is not the proposed browser candidate. It was not compiled or exported by this worker. Do not use it to claim current source/output identity, delivery or acceptance. The browser path above is authoritative for this round, and native-host restoration is outside its critical path.
