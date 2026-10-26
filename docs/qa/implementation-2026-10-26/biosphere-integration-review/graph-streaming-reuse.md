# Q42/HMC graph-selected streaming reuse review

2026-10-26; bounded source review and next-contract proposal. No builds, tests,
exports, sibling edits, packages, credentials or publication were performed.
Read architecture, development modes, species embodiment, durability/distribution,
format SDK foundation and Round 2/3/5 evidence. All FND gates and IP-03 C1–C8 remain open.

Reuse the actual Q42 graph, existing HMC entries and canonical payload codecs.
The graph describes referents, part/ecological relationships, representations,
qualified identifiers, provenance and resource bindings. Mesh vertices, pixels,
audio samples and simulation buffers remain typed binary resources; they must not
be recast as Quins merely to make selection graph-shaped. No separate assetgraph,
graph store or package format is proposed.

## Inspected native APIs and their limits

Paths below are exact local source paths, with one-based lines; all are source
presence/export evidence unless a prior executed receipt is explicitly named.
Sibling Git revision lookup was denied by the sandbox, so no HEAD is invented.
The inspected working tree may contain user changes; the four source SHA256 pins
below identify key observations independently of a Git revision.

| Seam | Actual public source | What it establishes / does not establish |
|---|---|---|
| HMC index | `C:/github/qualiaDB/crates/qualia-core-db/src/bundle/format.rs:77` `BundleEntry`; `reader.rs:46` `BundleReader::parse`, `:143` `get`, `:152` `segment`, `:163` `verify_entry` | QBDL contains intact entries, exact offsets/lengths, whole-entry SHA256 and opaque metadata. Borrowed slice/mmap access is not network acquisition or a graph dependency closure. |
| Header/index-only plan | `C:/github/qualiaDB/crates/qualia-core-db/src/bundle/streaming_loader.rs:69` `HmcHeaderProbe::parse_header_range`, `:133` `index_footer_range`, `:147` `HmcProgressiveRangePlanner::from_header_and_index` | Can discover entries using header/footer rather than payload download. Footer decoding allocates a Vec and is not caller-budgeted; count/duplicate/digest-size/overlap checks need qualification. This partial probe does not authenticate the release/index. |
| Range plans | Same `streaming_loader.rs:194` `plan_entry_ranges`, `:216` `plan_10d_interior_section_ranges`, `:322` `verify_arrived_range`; exported at `bundle/mod.rs:62` | Whole-entry plans schedule **every** entry, sorted by key/kind-derived tier. Interior plans schedule every section using section CRC32C. Neither API selects a graph query's resource closure; key substring LOD rules are not biological selection. |
| Exact reads | `C:/github/qualiaDB/crates/qualia-core-db/src/q42/volume/range.rs:30` `Q42ByteRange`, `:54` `Q42RangeSource::read_range_into`, `:65` `LocalFileRangeSource`, `:171` native `HttpRangeSource` | Caller-buffered offset/length contract. HTTP rejects full-body 200 fallback and validates 206 range coordinates. Exact addressing is not cryptographic integrity or cancellation. |
| Q42 root/partitions | `C:/github/qualiaDB/crates/qualia-core-db/src/q42/volume/manifest.rs:64` `Q42VolumeManifest`, `:310` `segment_range_for_object`, `:485` `Q42RangeVolumeSet`, `:507` `open_root_with_lexicon_factory`, `:524` `open_root` | Native manifest/segment factories already exist. Root constructor opens every child header and object bounds; attach opens lexicon shard metadata. It avoids whole payload reads but is not lazy selected-child opening. |
| Paged graph query | `C:/github/qualiaDB/crates/qualia-core-db/src/q42/volume/range_volume.rs:98` `Q42RangeQueryPlan::for_pattern`, `:217` `execute_query_page_into`, `:363` `lookup_lexicon_hash_into`, `:687` `bidx_block_range_for_hash`; `manifest.rs:648` volume-set paging | Object constants prune root intervals then BIDX blocks; s/p/context constraints use field-range/postings pruning; unbound scans can traverse all blocks. Output-page capacity alone does not cap scan work when matches are sparse. No application must query all payloads to discover a selected resource. |
| SPARQL reuse | `C:/github/qualiaDB/crates/qualia-core-db/src/sparql_library/sparql_executor.rs:609` `execute_range_triple_page_into`, `:671` `execute_range_volume_set_triple_page_into` | Existing native binding-row adapters drive the actual range readers with caller scratch and cursors. Native module is cfg-excluded on wasm32 (`:733`); this does not prove native Poet/browser runtime wiring. |
| Typed binding | `C:/github/qualiaDB/crates/qualia-core-db/src/container_10d/manifold_identity_v2.rs:79` `ManifoldIdentityV2`, `:120` `TypedFieldRecord`; `typed_payloads/resolver.rs:47` `TypedPayloadResolver::new`, `:87` `field`, `:96` `surface_frames`, `:102` `morph_targets` | MID2 preserves stable entity/profile/domain/convention and field kind/unit/section/range distinctions. Resolver currently borrows a complete contiguous `.10d` slice and validates all declarations; it is not a sparse transport resolver or transitive cross-entry dependency API. |
| Existing family selector | `C:/github/qualiaDB/crates/qualia-core-db/src/bundle/variant_matrix.rs:70` `ModelVariantDescriptor`, `:97` `TwinFamilyManifest`, `:146` `resolve_variant` | Existing phenotype/stage/LOD/state plus asset/ontology keys are reusable. It lacks the whole independent style/scale/fidelity/target/scientific coverage contract; fallback must not silently change biological referent/state. |
| Decode/mip budgets | `C:/github/qualiaDB/crates/qualia-core-db/src/render/texture_streaming_plan.rs:79` `rgba8_ktx2_mip_candidate`, `:138` `decode_rgba8_ktx2_mip_into`, `:190` `select_texture_mip`, `:267` `reserve_texture_mip` | Reusable typed selection, caller buffers and reservations for the declared texture subset; not general geometry/rig/field/graph streaming. |
| Native residency | `C:/github/qualiaDB/crates/webizen-render/src/volumetric_hmc_stream.rs:44` `HmcTextureResidencyRequest`, `:83` `request_texture_stream`, `:452` `apply_texture_stream`; public wrappers `volumetric_hmc.rs:1001` and `:1009` | Working-set omission evicts textures; generation/fingerprint guards reject stale plans. Existing load (`volumetric_hmc.rs:509`) accepts whole HMC bytes and retains bundle plus selected asset bytes (`volumetric_hmc_stream.rs:34`). GPU mip residency is therefore not proof of transport streaming or bounded whole-package CPU residency. |
| Rights/revisions | `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/mod.rs:296` `publication_is_current`, `:314` `transact_with_package`; `records.rs:88` `record_requested_source_review`; `entity_view/rights_filter.rs:26` `decide_view_with_grant` | Existing project/session/package/source review and view decisions have owners. Source rights, graph-partition/resource fetch authority and derivation continuity still require end-to-end binding; visibility filtering alone cannot authorize acquisition. |
| Job seam | `C:/github/qualiaDB/crates/qualia-client-core/src/api/jobs.rs:53` `cancel_local_job` | Existing scheduler supports queued/cooperative-running cancellation; it is not evidence that Q42/HMC synchronous readers observe cancellation. Poet browser `job_queue.rs` imports web_sys and seeds samples; do not cite that display as native streaming execution. |

Key source pins (SHA256):

- `bundle/streaming_loader.rs`: `1643cef6cc208330eed78f59feda599562ef683dd7c46e8bddc3622bd836ea6c`
- `q42/volume/range_volume.rs`: `36e5ddc1878ad31d6d018e8100a429a0f653dbfe6de2ce0d117cd1fde6fcb21c`
- `q42/volume/manifest.rs`: `821ded7e92e70d5113279f31da7bd7763a738b71b3d10a33a8c1aec9673862cb`
- `webizen-render/src/volumetric_hmc_stream.rs`: `1301e76c86ed9fd1835bfbded85ecea8de4e6a226fc0c22475db425448a8c782`

## Integrity and data path

The minimal intended path is **identified release/root and graph partition index
→ bounded native Q42 query/lexicon pages → selected representation/part/resource
identifiers → typed dependency closure → admitted exact entry/section ranges →
canonical decode → bounded CPU/GPU residency → eviction**. These pieces exist
separately; a complete connected magnifier path has not been demonstrated here.

HMC whole-entry SHA256 is not a chunk hash. Its interior-section CRC detects
accidental corruption relative to the received section table; it does not prove
authenticity against an independently trusted release. `verify_arrived_range`
also succeeds with neither expected digest set; malformed SHA length silently
becomes `None` in `plan_entry_ranges`. Do not promote such a request to verified.
Interior planning additionally needs checked section-table arithmetic, duplicate/
overlap/bounds/version/CRC policy and descriptor provenance before transport use.
The standard checked `.10d` parser remains the canonical validation owner.

Q42 `range.rs:132` `verify_source_sha256` and `manifest.rs:748`
`verify_segment_hashes` read all child bytes in bounded scratch to authenticate
whole children. They must not run implicitly on every magnifier query and be
described as selective acquisition. `range_volume.rs:25` decoded-block validation
checks size/count/occupied bounds only; it does not check Quin parity/order or a
cryptographic committed block hash. Full-file verification is a separate path.

`q42/volume/car.rs:27` `decode_and_verify_car` verifies each supplied block CID;
`:59` `extract_entity_bytes` concatenates verified raw leaves into an allocated
buffer. Those mechanism facts do not establish trusted requested-root membership,
UnixFS ordering/offset proofs or a universal bounded HMC HTTP chunk verifier.
Use an already authenticated installed entry as the first qualified mode. For
untrusted remote partial reads, negotiate and qualify a trusted existing transport
proof/index contract; absence is a blocking integrity gap, not permission to
invent a new container or claim a CRC is authentication.

## Magnification and completeness rules

Fish scales are identified parts of a specified organism/template, with graph
bindings to available surfaces/fields and representation resources. Grass blades,
insects and rock-associated lichens can be distinct organisms, colonies or taxa
with evidence-bearing relationships; they are not finer LODs of fish, grass or
rock. Magnification selects the intended subject/part and authored scale/profile,
then chooses compatible resources. It must not fabricate microscopic structure.

Complete HMC distribution means the finite corpus/release inventory is reconciled,
including all graph partitions, context, lexicon, evidence and available resources.
Resident selection does not reduce that inventory. A relation to another species
retains identifiers and evidence without recursively downloading its entire HMC.
Only the declared operation's required external closure is acquired, bounded by
depth/bytes/count and explicit rights; cycles are deduplicated by qualified release
and resource identifiers, not matching labels or 64-bit lookup hashes alone.

Offline inspection requires installed graph/vocab/lexicon/provenance and the
selected operation closure. Report unavailable/denied/unknown distinctly. A partial
cache cannot claim full offline support; a rights-filtered derivative declares
coverage exclusions without disclosing protected details or claiming unrestricted
species completeness. An acquisition job does not discharge commons obligations.

## Minimal next implementation contract, after review

1. **Ecopunk contract/fixture owner:** pin one small multisubject corpus and HMC
   release with graph-to-resource binding vocabulary, source rights, taxon/instance/
   part/representation/release distinctions, offline inclusion and expected answers.
   Provide fish-scale, grass/insect and rock/lichen queries with negative identity,
   missing-resource and external-cycle cases. No new graph store or production model.
2. **Core Q42 owner (separately authorized upstream):** retain existing root/BIDX/
   field-index/SPARQL APIs; add lazy selected-child opening where required, explicit
   per-call scanned-block/byte/work deadlines and cancellation checkpoints. A page
   with no match must still yield within budget. Tie cursors to immutable graph
   revision/query/admission, invalidate on replacement, and qualify result integrity.
3. **Core HMC/typed-resource owner:** specify a query-result-to-existing-resource
   binding and typed closure plan, not a second graph. Plan only selected required
   geometry/material/texture/rig/clip/science/context dependencies, reject unresolved
   required bindings and cycles beyond bounds, distinguish optional unavailable data.
   Preserve MID2 semantics; expose sparse checked descriptor/range validation using
   existing codecs instead of manufacturing a sparse full-file buffer. Explicitly
   declare authenticated installed mode versus qualified remote proof mode.
4. **Native host/jobs owner:** marshal Vibe-authored subject/part/profile selection
   into the public plans. Job key includes release digest, source/graph revision,
   subject, query, admission generation and selection epoch. Reserve transfer,
   compressed/decoded CPU, GPU and in-flight staging budgets before work; cancel
   superseded reads/decode/upload and discard late results before publication.
   Recheck rights and current generation at apply; cancellation releases reservations.
5. **Renderer owner:** adapt verified selected resources into the existing texture
   lifecycle and equivalent declared geometry/rig/field residency. Evict CPU staging
   as well as GPU resources, retain offline cache under an explicit cache budget,
   and preserve the last admitted view on failed/cancelled/stale refinement.
6. **Poet/consumer qualification owner:** expose actual native query/magnifier,
   progress, denied/missing/offline states and cancel controls through unified Poet.
   Record exact bytes/ranges/requests, query plans, source pins, peak resident and
   in-flight bytes, stale/cancel outcomes and source-to-output identifiers. Compare
   selected plan with an independent fixture oracle; prove unrelated species and
   unselected high-resolution payloads were not fetched. Native/WASM runtime results
   and human contributor/reviewer acceptance remain separate required evidence.

These are disjoint ownership seams, not authority to edit sibling repositories or
run Cargo now. First implement/review the smallest admitted installed-HMC native
query-to-selected-resource path; then qualify remote acquisition and broader types.
No streaming latency, model completeness, scientific accuracy or human acceptance
result is claimed by this review.

## Prior evidence retained

[Round 2](../../implementation-2026-10-25/batch-6/rounds/round2/status-final.json)
records 22 scoped native passes; [Round 3](../../implementation-2026-10-25/batch-6/rounds/round3/progress.md)
records 56, including typed frames/opaque preservation and WASM compilation;
[Round 5](../../implementation-2026-10-25/batch-6/rounds/round5/progress.md)
records 49 distinct selected native passes and source-to-`.10d`/Q42 HMC witnesses.
These are historical pinned executions, not tests rerun here or streaming corpus/
magnifier evidence. Round 5 deliberately refuses unsupported direct graph-bearing
package rewriting; coherent source recompilation retains old output under a new
explicit key. Preserve that safety and every prior receipt while extending reuse.
