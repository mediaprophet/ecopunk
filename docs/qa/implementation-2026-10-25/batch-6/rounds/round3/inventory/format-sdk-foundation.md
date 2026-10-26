# Round 3 format/SDK inventory (read-only)

Captured 2026-10-25. Scope: seven relevant QualiaDB source files, round-2 recorded execution, and installed Rust targets. No source/assets were changed and no builds/tests/network/CU were used. `rustup target list --installed`: `wasm32-unknown-unknown`, `x86_64-pc-windows-gnu`, `x86_64-pc-windows-msvc` (availability only, not execution).

## Evidence by gate

| Gate | Implemented API / evidence | Remaining gap and owner |
|---|---|---|
| FND-01 | MID2 `StableEntityId` is opaque, distinct from external digest and from profile/domain/coordinate-convention IDs. `resolves_same_entity` is handle equality only; docs explicitly delegate qualified referent resolution to graph/authority. Round 2: format declaration and structure recorded; no qualified issuer/scope/source/validity mapping execution. | QualiaDB graph/identifier authority owns qualified resolution; Poet mapping/lineage caller owns demonstrating mappings across edit/derivation. Bounded job below. |
| FND-02 | `M64Reader::parse(bytes, limits, scratch)` validates structural layout/budgets and returns borrowed chunks; supports existing v1/v2 and m64/e64 magic, raw codec only. Round 2 logged 7 M64 tests and existing consumer suite; one fixture allocation count was zero. | FND-02 remains open: geospatial cell size, vertical datum identifier, nodata, precision, broader budgets. Geospatial job is assigned elsewhere; do not duplicate. |
| FND-03 | `TypedPayloadResolver` selects validated MID2 ranges; `surface_frames`, `morph_targets`, and field-by-ID APIs; legacy one-sidecar fallback. Round 2 typed suite logged 9 passes, including selected frames+morph+scientific-raster coexistence, malformed/range and transactional refusal cases. | The separate round-2 inspection predicted current `render/compile_10d.rs` and `render/portal/mod.rs` decode whole sidecar; callerbufferframes job is assigned elsewhere. FND-03 still open at broader compatibility/consumer scope. |
| FND-04 | Outer table parser checks bounds/alignment/overlap/CRC and rejects unknown section tags (`typ() -> None`; validation rejects undefined/unknown); `encode_container` sorts known `SectionType`s and rejects duplicate types. No generic API to retain/rebuild unknown section descriptors/payloads. `attach_material_section` does preserve/re-encode all recognized sections, but errors on an unknown tag. Round 2 recorded explicit caller limits/overflow refusal and one fixture allocation counter, not workload benchmarks. | QualiaDB Core owns unknown-section preservation/rebuild contract and tests. Focus next on explicit fail-closed vs preserve-on-rewrite behavior; do not silently map unknown bytes to known enum tags. |
| FND-05 | No Poet workflow evidence in this round-2 foundation runner. Runner is native Core/geospatial SDK + 6 existing consumers; does not prove contributor/reviewer C1–C8, save/reopen HMC, or usable Poet controls. | Poet owner: controls and distinct authorized contexts; HMC/Q42 owner: knowledge/resource/revision continuity. No bounded format test substitutes for this. |
| FND-06 | Installed wasm target exists. Round 2 runner executed only Windows x86_64 little-endian; no WASM parity/interoperability run or representative benchmark. | QualiaDB/browser binding owner plus host/WASM integration owner; test shared format behavior on actual WASM target separately. Target installation alone is not proof. |

## Two recommended bounded jobs after current assignments

1. **Unknown `.10d` section rewrite policy (Core owner; FND-04, supporting FND-01/03):** add a source-pinned regression fixture with an unrecognized section tag and payload. Determine whether canonical parse/rewrite must preserve byte-exact section tag, descriptor metadata, payload and unrelated known sections, or reject rewrite before output mutation. Test both safe refusal and preservation only if an explicit opaque-section API is approved by the format owner; verify CRC/reopen and caller-buffer atomicity. Do not change format version or accept unknown identifier semantics. Current `SectionType` enum/`SectionInput` cannot represent unknown values; material attach fails on `typ()==None`; generic writer only accepts known types.
2. **Qualified identifier mapping boundary (Core graph/authority owner with Poet consumer; FND-01):** use existing MID2 stable entity handle and native graph/authority APIs to record one identifier mapping with scheme/issuer/scope/source/validity, then test ambiguous, expired, revoked, split/merge and same-string/digest-but-distinct referents. Keep mapping separate from local record key, MID2 bytes, and external digest. Do not invent a format-level issuer or authority claim in MID2. Capture actual source/output/revision lineage. If graph API cannot express the qualifiers, return the exact missing contract/API as blocker rather than adding a codec-local resolver.

The first is implementation-ready within the Core-owned section table boundary; the second requires a narrow read-only API check before allocating implementation because authority resolution is explicitly outside the MID2 codec. Neither job overlaps the separately assigned geospatial codec or callerbufferframes consumer work.

## Source pins (SHA-256)

```text
2416394e69ac1646f3ec77a4904983806bb190dce82e15e650dcd1bb222e067e  C:\github\qualiaDB\crates\qualia-core-db\src\container_10d\section.rs
96f6585e9af1a0e7fa22108093162d45cb9413d10da7064852f40941be37b7bb  C:\github\qualiaDB\crates\qualia-core-db\src\container_10d\section\table_validation.rs
74af35954a905dd2086cc9618301c7ffebd59160823d026f4bf6220f23eaca47  C:\github\qualiaDB\crates\qualia-core-db\src\container_10d\typed_payloads\resolver.rs
9c8147b541ee6d8cd99589458ab8e6116e1e8f96e558c7ed27b1227d34e76647  C:\github\qualiaDB\crates\qualia-core-db\src\container_10d\manifold_identity_v2.rs
edf321c184ed9ceda846672421245f5bdf90950443ce7d0c36b5fe61b61c3771  C:\github\qualiaDB\crates\qualia-core-db\src\container_10d\material_container.rs
6e3ffe3603577c200db3c8fbd3e336b175baa5efa257374247f1535c3bde85dc  C:\github\qualiaDB\crates\qualia-core-db\src\q42\m64_matter\checked_reader\mod.rs
8477d7eb60c0c3ab77189c6e245058122afdfdf72d90735838d8bed262f02cbd  C:\github\qualiaDB\crates\qualia-core-db\src\q42\m64_matter\checked_reader\validation.rs
```

## Executed facts / boundaries

Round-2 `status-final.json` records native Windows x86_64 little-endian exit 0: 6 existing consumer tests + 7 M64 SDK + 9 typed-field SDK = 22 passed, none failed/ignored; command `cargo test --manifest-path tools/format-sdk-conformance/Cargo.toml --locked --offline --target-dir target`; build 19m59s. Source snapshot and post-test documentation-only corrections are tracked by round-2 receipts. This is execution evidence for that snapshot only. No WASM, broad workload benchmark, human review, Poet C1–C8, or foundation gate pass is implied. Assets listed in round-2 status remained read-only and unchanged.
