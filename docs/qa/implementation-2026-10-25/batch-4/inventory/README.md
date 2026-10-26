# Batch 4 native auth and HMC inventory

**Scope:** read-only reconnaissance for native Poet authentication integration and complete resource-preserving package editing. No source outside this Ecopunk inventory directory was changed. This note records source presence and current wiring; it does not claim implementation, session authentication, membership admission, package-edit completeness, executed verification, or human acceptance.

## Native identity and project membership

| Evidence (repository path and lines) | Finding | SHA-256 of inspected file |
|---|---|---|
| `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/governance/capability.rs:150-163` | `Identity.current_user` ignores arguments and returns the literal `did:qualia:timothy_charles_holborn`, with `source=principal_default`. This is not a verified native session principal. | `5386071117571974722891E82F17E35A837364268100B902EFBACE41A070CF32` |
| `C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/mod.rs:1168` and `.../ids.rs:1045` | Public dispatch maps exact legacy identifier `Identity.current_user` to that function. Preserve the spelling when replacing its owner/wiring. | mod `9862763CF1103C77C6ECC66EB489D7D0269E6D46EDFF549E38C0CBE76627F210`; ids `BF3D2DAFCAF3DF233C88673EA25C4671BE61FE9FE124CCECE9704ADF2BAE2E2F` |
| `C:/github/qualiaDB/crates/poet/src/browser/mod.rs:363-381` | Poet asks its native daemon for `Identity.current_user`, then sets observer DID only when a parseable result is returned. If no daemon is connected or invoke fails, it silently returns; this is a consumer, not proof of authenticated principal bootstrap. | `017B137DC8A5AB28DA9C7D5315968E34210D2FD1519E35788D2740FD548B623B` |
| `C:/github/qualiaDB/crates/poet/src/browser/manifest.rs:74,119` | Export manifest defaults to the same fixed actor DID. The value also appears in export tests and other UI/fixture code; auth integration must distinguish fixture defaults from live author provenance. | `5721EDE71962E1885A99285903F469C5903392729685878F8974B70751C89DC5` |
| `C:/github/wellfare/core/src/projects.rs:78-110` | `ProjectMembership` is a serializable data record (`project_id`, `member_did`, role, agreed-at time) and constructor. It defines a record shape, not a proof that the current caller authenticated or is admitted. | `F03FF7E6675451F847BD8962A5E86EAA9F8EA40D4EDB2FE8361CE2F764521887` |
| `C:/github/wellfare/client/api/coop.rs:37-57` | `WebizenHostApi::add_project_membership` builds/submits a membership envelope using host owner/author DIDs and journal storage. This is a candidate persistence owner to evaluate for integration, but does not itself provide evidence of the authenticated Poet session or authorization decision. | `84AE1433B998106AFDC7B0F71813AA69E4032BA194F66CAC6D02AF78DCBE7C37` |
| `C:/github/qualiaDB/crates/qualia-core-db/src/net/qdnf/authority/verified.rs:14-50,70-111` and `.../authority/owner.rs:63-110,145-183` | QDNF has owner-constructed verified credentials, authorized contact records, and generation-bearing execution permits; constructors are crate-private and grants are validated by `AuthorityOwner`. These are scoped grant/operation authority primitives, not an observed current-human login/session API or project membership lookup. | verified `3F64538ED3F63B7110FD88CF3C1111A2CD8E5017BECDBD2990FC62FC64CA8AF9`; owner `1BD4F33F33C8348CAF5EF35617A677F7C321B53A30FEECDF27EFA924CAE26110` |

### Auth wiring conclusion

The source-backed integration seam is the existing daemon invocation and `Identity.current_user` dispatch. Current returned DID is a demo principal hardcode; Poet observer binding trusts a returned DID string, while manifests also have a default actor. This scan found no `AuthContext`, authenticated current-session object, or verified identifier-to-principal mapping wired through that path. QDNF verified grant primitives and Wellfare membership records are distinct owners/contracts and must be reconciled rather than treating either as current-session identity. No authority should come from MCP/CLI text or reopened HMC data.

The searched `C:/github/webizen-os-shell` repository had no source matches for `current_user`, `Identity.current_user`, `ProjectMembership`, `Membership`, or matching auth/session context terms in its Rust/TOML/Markdown files; this bounded search does not establish absence elsewhere in Webizen Desktop or the app host.

## HMC bundle and checkpoint resource contracts

| Evidence (repository path and lines) | Finding | SHA-256 of inspected file |
|---|---|---|
| `C:/github/qualiaDB/crates/qualia-core-db/src/bundle/format.rs:1-12,77-99` | HMC is a transparent container: intact entry bytes, page-aligned offsets, CBOR index, SHA-256 per entry, domain-specific opaque metadata. `BundleEntry` stores `key`, `kind`, `offset`, `length`, `sha256`, and optional opaque `meta`. | `D67077A2D1AADB1B996FA85DA1B643994016708DC15CFAE23DB95D7707C4942E` |
| `.../bundle/reader.rs:39-61,125-177` | `BundleReader::parse` validates header/length/CRC/index and entry bounds; `entries`, `entry`, `get`, and `verify_entry` expose enumerated records, intact bytes, and entry integrity checks. `get` is zero-copy. | `E4D7B45C0E2467DD24C991AE5DB72A1250240851FB1B8838B2AE49D648FC2E74` |
| `.../bundle/writer.rs:20-63,92-148` | `BundleWriter::add_file` takes intact entry bytes and optional opaque metadata, rejects empty/duplicate keys; `build` preserves payload bytes, writes a fresh index/offsets and recalculates hashes/CRC. This API supports retaining entries while replacing explicitly edited bytes. | `E3010E2197019CE55C166FF8E68259F7E4870EEBD9BC9394326D9A24D32E976E` |
| `.../poet_host/invoke/studio_checkpoint/codec.rs:59-163` | Current checkpoint encoder writes source JSON, graph q42 records, compiled 10d assets, and texture dependencies. Its decoder validates each entry and graph resources, then returns only `Checkpoint { record, file_digest }`; it explicitly loads no principal mappings, grants, runtime handles, adoption, or rights decisions. | `B97100348AA8715137338EC160F0A89B338C93A5C80C44738AA8571488FCA64E` |

### Resource-editing conclusion

Generic HMC read/write contracts can preserve intact entries and metadata. Current checkpoint decode is a projection into a source-centric `Checkpoint`, so simply decoding, editing source, and re-encoding from that value cannot preserve arbitrary existing package resources. A complete editor needs an owned package representation (or equivalent retained raw entries) for every entry's key/kind/bytes/meta, explicit resource edits, and save/reopen checks that prove untouched resource witnesses survive. Recalculate the outer bundle index/integrity after edits. HMC presence, successful parsing, or source-only checkpoint recovery does not establish complete species/thing package preservation.

## Snapshot limits

Hashes above bind the inspected QualiaDB sources observed during this inventory pass. Wellfare hashes were not captured. No builds, tests, exports, UI runs, or writes outside this Ecopunk `inventory/` directory were made.

