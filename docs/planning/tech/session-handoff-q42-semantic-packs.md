# Session handoff — q42 semantic packs in game-demo (G49/G50)

Session state as of 2026-10-12. Purpose: transfer the applied
ontology→game-demo work to a fresh session with the state, the remaining
implementation spec, and the operating rules the user has set.

**Subsequent authority clarification:** Timothy requires QualiaDB/VibeScript
changes to be investigated and documented here for him to arrange upstream
implementation, not applied by the game-demo/ecopunk agent. QualiaDB is
read-only for this work. Missing Vibe functionality is not authority to inject
JS substitutes; necessary browser interop needs a specific technical basis.
See [development-modes.md §5](development-modes.md#5-supervised-development-and-evaluation-integrity)
and [engine-requirements.md G50 follow-up](engine-requirements.md#g50-follow-up--byte-backed-q42vibe-host-qualification).
The implementation directions below are historical requirements, not current
permission to edit the engine. Existing edits remain unapproved/unverified;
no unilateral rollback or successful-baseline reclassification is authorised.

## Repositories and branches

| Repo | Branch | Role |
|---|---|---|
| `C:\github\game-demo` | `feat/taxon-binding` | Applied demo — assets, packs, WASM shell, `.vibe` fixtures |
| `C:\github\qualiaDB` | `feat/ecology-vocab` | Engine — `feat/ecology-vocab` holds the GeoSPARQL 1.0 manifest entry **and** the `q42_lex_ns.rs` determinism fix (uncommitted at handoff — see below) |
| `C:\github\ecopunk` | — | Planning corpus; this file; register `docs/planning/tech/engine-requirements.md` |

## What is implemented (and its verification state)

**Verified (executed in-session):**

- `web/fixtures/taxon-bindings.vibe` — `taxon_for(render_key)` binds 18
  render keys to GBIF backbone `u64` keys + IRIs + rank + accrual IRIs;
  `dwc_statements_for(render_key, asset_iri)` emits `{s,p,o,quin}` records
  via `quin.statement` (host seals parity, hashes IRIs with
  `generate_60bit_token`). `vibe check` + `vibe eval` verified.
- `crates/rolling-commons-shell/src/taxon_bindings.rs` — WASM exports
  `taxon_binding_json`/`taxon_dwc_json`; host packs the GBIF key quin as
  `INLINE_TAG_INTEGER` (f64 can't form the `0b001<<60` tag). wasm32 check
  was clean at that commit.
- `semantic_slice.rs` — `SemanticSlice` WASM class decodes `sem/*.quins`
  (48-byte NQuins) + `sem/*.lex` from a verified pack, re-checks parity,
  resolves terms, exposes `validate_shacl` over `validate_shacl_property`.
- `crates/pack-export/` — native Rust bin replacing the Node exporter for
  pack/manifest regeneration. Evaluates the `.vibe` fixture natively via
  `vibe::eval_function` + `PoetSnapshot`, stamps manifest assets, emits
  `sem/taxonomy.{quins,lex,q42}`, builds HMC via `BundleWriter`,
  re-verifies every entry (BundleReader digest pass), runs a SHACL shape
  pass (`validate_shacl_property`: minCount on dwc props + integer datatype
  on `urn:qualia:gbif:taxonKey`, foaf:depicts per bound asset). 3 unit
  tests pass. Packs were built and were byte-stable across runs **before**
  the `.q42` entry was added.
- `game.html`/`verify-game-release.mjs` — HMC count formula includes
  textures + semantic entries; compressed-transfer progress is
  indeterminate.

**Written but UNVERIFIED (no builds run since — intentional):**

- `pack-export` `.q42` emission: `StreamingQ42VolumeWriter` (deterministic
  — `assertion_timestamp: 0`, no wall-clock DAG stamps) →
  `sem/taxonomy.q42`, embedded lexicon via `with_lex_map`-style
  constructor `StreamingQ42VolumeWriter::new(&HashMap<u64,String>)`,
  `declare_permissive_commons()`, `push_block` on object-sorted chunks of
  `QUINS_PER_BLOCK`, `finish` to a `NamedTempFile`, bytes embedded as an
  HMC entry. `tempfile = "3"` added to pack-export deps.
- `q42_lex_ns.rs` (`serialize_namespaced_paged_lexicon`): **fixed a real
  engine determinism bug** — the per-page namespace table iterated a
  `HashMap`, so `ns_id` assignment permuted per process and volumes were
  not byte-reproducible. Now collects + `sort_unstable`s accepted
  namespaces before assigning ids. (Length drift of ±3 bytes seen in the
  pack was benign CBOR — `Vec<u8>` sha256 encodes as an int array whose
  width varies with the digest, not an integrity issue.)
- `lib.rs` `pack_game_hmc` accepts `sem/*.q42` with kind `"q42"`.

## The architecture the user specified

Ontologies are packaged in `.q42` files; `.10d` datum assets connect to
them (asset-IRI → taxon-IRI `foaf:depicts` quins are the link); the HMC
container packages both. The `.quins`/`.lex` loose entries were a
transitional codec — the `.q42` volume is the canonical semantic artifact.

## Remaining work — the engine gap

`qualia-core-db/src/q42/q42_volume.rs` is `#[cfg(not(target_arch =
"wasm32"))]` at `q42/mod.rs:31-32`, and its `use memmap2`, `std::fs`, and
`mod volume` are unconditional inside it. So the browser cannot decode a
`.q42` entry. Required upstream work:

1. **Split q42_volume.rs** so byte-format items compile on wasm32 —
   `Q42VolumeHeader`, `BlockDirectoryEntry`, constants (`Q42_MAGIC`,
   `HEADER_SIZE`, `QUINS_PER_BLOCK`, `SUPERBLOCK_*`), `header_from_bytes`,
   `encode_lex`/`encode_bidx`/`encode_field_range_index`,
   `encode_superblock`/decode, `UnifiedVolumeBuilder::finish_to_bytes`.
   Keep `cfg(!wasm32)` on: `Q42Volume` (memmap), `File`/`fs` paths,
   `UnifiedVolumeBuilder::finish(path)`, `migrate_v2_to_v3`,
   `is_unified_volume`, `write_sorted_quins_volume*`, `HttpRangeSource`,
   `LocalFileRangeSource`, `StreamingQ42VolumeWriter` (tempfile/fs),
   publish/compact paths.
2. **`volume/mod.rs`** — gate per-module: `range.rs` trait
   (`Q42RangeSource`, `Q42ByteRange`, `validate_exact_range_response`),
   `range_volume.rs` (`Q42RangeVolume<S>`), `index.rs`, `validate.rs`,
   `postings.rs`, `cursor.rs` are byte-pure and should un-gate; fs/http/
   tempfile modules stay native. Audit each file's imports.
3. **Slice-backed source** — implement `Q42RangeSource` over `&[u8]`
   (call it e.g. `SliceRangeSource`) so pack entries are readable without
   fs. Then `Q42RangeVolume::open(slice_source)` gives the browser header
   + BIDX + postings + `read_superblock_into` + `read_lexicon_into`.
4. **`semantic_slice.rs`** — `SemanticSlice::from_pack` reads `sem/*.q42`
   entries via the slice source; decode lexicon via `Q42LexMmap`
   (check whether `q42_lex` itself is wasm-gated — `lib.rs:334`); quins
   come from superblock decode rather than raw 48-byte chunks. Then
   **remove** the `.quins`/`.lex` emission from `pack-export` and their
   handling in `pack_game_hmc` — one canonical artifact.
5. **Entry-count formulas** in `game.html` + `verify-game-release.mjs`
   already count `manifest.semantics[]` generically — verify they still
   match when `.quins`/`.lex` collapse to a single `.q42` entry.
6. **Reconnect to 10d datum**: the depicts edges already link
   `rc:asset/…` subjects (the same IRIs stamped in the manifest `assets[]`)
   to taxon nodes; ensure whatever resolves assets can also resolve the
   semantic slice by IRI hash — that join is the "connected with datum in
   10d files" requirement.

## Verification checklist (when the user runs builds)

- `cargo test -p pack-export` (3 tests: bounded sealed quins, unbound →
  data-only, parity tamper)
- `cargo run -p pack-export -- --all` → all 3 styles; run twice and
  compare SHA-256 of each `.hmc` — **must be identical** now that the
  ns-table ordering is sorted.
- `cargo check -p rolling-commons-shell --target wasm32-unknown-unknown`
  after the engine un-gating.
- In-browser: SemanticSlice over `sem/taxonomy.q42`, `count` = quins,
  `resolve` a lexicon hash, `validate_shacl` on a taxon subject.

## Operating rules the user set (non-negotiable)

- **No commits unless explicitly asked.** No "Generated with Devin" /
  co-author trailers in any commit the user requests — plain messages.
- **No builds/tests without need** — repeated `cargo` runs recompile the
  full `qualia-core-db` dep tree and filled the disk; write code
  carefully, state what is unverified, let the user choose when to
  compile.
- **Do not narrow or reframe the request.** The user asked for q42-in-hmc
  with 10d linkage — implement that, not a conveniently smaller variant.
- **Honesty rules stand**: unknown taxa stay data-only (`bound: 0`),
  rank reports GBIF verbatim incl. quirks, no invented ontology URLs.
- VibeScript remains the authored source of taxon/semantic logic; Rust is
  the driver/adapter layer only.

## Open questions for the user

- Should `pack-export` also take over `.10d` scene regeneration
  (`scene_build`) when a native path exists, or stay a repacker?
- Does the wasm reader belong in `qualia-core-db` proper or behind a
  `wasm-full` feature?
- Commit attribution preference (if any) for future commits.

## Supervision review — existing edits and remaining proof

The subsequent instruction settles attribution: no provider bylines, commits,
exports, builds, or tests unless specifically requested. QualiaDB changes
listed here were made in the preceding implementation sequence and are not
approved upstream work. They have not been reverted or qualified. Inventory
is based on the recorded edits and reviewed diffs, not an attribution of every
change currently present in the shared working tree.

| QualiaDB path under `crates/qualia-core-db/src/` | Recorded change | Review state |
|---|---|---|
| `lib.rs`; `q42/mod.rs` | Expose Q42 volume/lexicon modules on WASM; remove the always-error byte lexicon stub. | Unverified portable exposure, not evidence of a formerly broken HMC filesystem. |
| `q42_lex.rs` | Gate native file loaders; add a public WASM file-handle shim whose `view` returns an empty view. | Shim needs upstream review; do not treat it as verified lexicon functionality. |
| `q42/q42_volume.rs` | Split exports and gate native filesystem/mmap/writer items. | No compile proof; format code was not physically decomposed; portable cursor/public helper contract remains to be settled upstream. |
| `q42/volume/mod.rs` | Per-module gates and exports; leave the Q42Volume-bound cursor native-only. | This differs from the historical handoff's cursor un-gating instruction; `cursor.rs` actually depends on the native reader. |
| `q42/volume/range.rs` | Gate local-file imports/source and native tests. | Trait and exact-range byte interfaces remain exposed in the edited source; WASM qualification absent. |
| `q42/volume/manifest.rs` | Gate file-backed descriptors/volume-set helpers; retain range-backed descriptors. | No native or WASM regression evidence. |
| `q42/volume/opfs_source.rs` | Gate local CAR file helper and native tests. | Existing `OpfsSliceRangeSource` was reused, not newly invented. |
| `q42/volume/publication.rs` | Gate filesystem/classification functions and native tests. | Duplicate cfg attribute remains; rights/policy regression verification absent. |
| `q42/volume/range_volume.rs` | Gate native test module. | No new reader regression fixtures executed. |

The namespace determinism edit in `q42_lex_ns.rs` predates this sequence and
is preserved. Unrelated modalities, orchestration tests, and roadmap changes
in the shared tree are not attributed to this work or touched by supervision.

### Game-demo follow-ups (not silently discarded)

- The duplicate `.add_file(key, kind, ...)` introduced into pack-export was
  removed; source re-read confirms one Q42 add call. No compile claim.
- `validate_game_hmc` delegates to `pack_validation::verify_game_pack`, whose
  inspected kind matcher rejects Q42 entries. The browser uses the separate
  digest/count-only `verify_game_hmc`; these paths must not be conflated.
- `SemanticSlice` allocates `lex_length` before a byte budget and duplicates
  terms into owned strings; its count/length limits are not an aggregate
  memory-budget proof. Browser asset-to-semantic integration is not present
  in the inspected loading path.
- The workflow's workspace-wide WASM check includes native pack-export, and
  the pinned engine does not incorporate the unapproved reader edits. No
  build pin was changed; no replacement artifact was generated.
- JS semantic serialization was removed, but the JS exporter still writes
  intermediate semantics-free packs before Rust repacking. A failed export
  must not relabel those artifacts as a completed semantic evaluation pack.

Historical successful pack/fixture receipts retain their stated artifact and
execution scope. The Q42-only candidate remains incomplete; this is not a
finding that earlier HMC/Q42 functionality was broken. Next upstream advice
and acceptance cases are in the G50 follow-up, including the distinction
between real PoetSnapshot queries and empty LocalHost query output.

### Single-asset harness correction

The first harness candidate wrongly placed review behaviour in JavaScript.
The corrected source uses `game-demo/web/fixtures/asset-review.vibe` for
catalog paging/filtering, selection, SHA/size admission decisions, metadata
panels, RDF source projection, texture admission, camera bounds, package
comparison and the manual UAT note schema. Taxon statements remain authored
in the existing `taxon-bindings.vibe`; no JS taxonomy/Quin implementation is
used. Generic browser code performs fetch/stream, data marshalling, DOM/canvas
IPC, Web Crypto and download operations only. The earlier JS evidence-policy
implementation was replaced, not retained as a fallback.

The interpreter entry point is the existing `eval_program_src`; no engine
edit or full-game build was made. Numeric browser facts cross as F64 values
so the evaluator's variant-sensitive equality and JS integer-string export
cannot silently change size/role checks; u64 identifiers stay strings.
Catalog calls are bounded to pages defined by the Vibe module rather than
re-interpreting all scene metadata per UI event. String construction uses the
existing list `join` method, not an assumed string-addition operator.

Launch from game-demo with `node scripts/serve.mjs`, then open
`/asset-review.html` on its local origin. Select a style and one `.10d` entry,
load it, and inspect model/metadata; an explicit render-key selection shows
Vibe-authored RDF terms. That panel is not pack-decoded RDF. The static server
also exposes narrowly contained `/asset-files/{style}/...` exported files.
At the source-only handoff, no server was running and no execution pass was
claimed. Subsequently Timothy explicitly authorised local browser testing:
seven standalone viewer checks passed in Chrome, including Vibe policy/RDF,
dynamic selection, corruption refusal and metadata layout. The existing WASM
was reused without a game build or export. Evidence is indexed in game-demo
`docs/qa/graphics-overhaul/README.md`, latest run
`asset-review-1791861101828`. The updated local server and review browser are
running on port 8090; the pre-existing port-8089 server remains untouched.
Art-quality acceptance and actual Q42-decoded RDF remain open.

The subsequent source audit confirms `.q42j` and the already-exported
`WasmQ42Session` mutable-session machinery exist. Do not describe mutability
as absent or implement a replacement. Journal persistence qualification and
compressed-Q42/embedded-lexicon admission are distinct; the viewer remains
read-only.
