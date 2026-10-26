# Bounded Round 5 repair recheck

Reviewed install manifest SHA-256:
`394c0b03c7f4756b1273e16ca7ea7235ed70e55618be3b9982a148ea0c342125`.
All 37 staged target hashes matched that manifest at the recheck. This was a
source-only review of the four reported repairs and their focused regressions.
No build, test, live sibling edit, asset operation or broader audit was performed.

The four implementation remedies address the reported mechanisms:

- `assembly_source.rs` now emits signed/unsigned alternatives for material
  identifiers within signed range, and a u64 literal for wider identifiers. The
  new actual SourceSession case covers mixed variants and u64::MAX while checking
  retained identifiers, extensions and factor changes.
- `native_dispatch.rs` checks every current source-bearing Action against the
  origin before serialization or diagnostic fallback. The native workflow now
  includes stale-Action/current-origin refusal.
- `authoring/controller.rs` uses canonical raw `crc32c` for retained source,
  keeping the container checksum helper for `.10d`. The reopened native workflow
  reads Q42 with the canonical reader and compares actual source/compiled facts,
  including a source-byte-52 change oracle.
- `authoring/package.rs` refuses differing existing compiled-entry digests before
  save and instructs the operator to choose a new asset key. The focused test now
  checks unchanged HMC on same-key refusal and exact retention of every old
  resource and metadata under new-key compilation. This avoids pretending to
  implement a safe merge of richer sections or additional graph facts.

The initial review report incorrectly retained a stale assertion finding.
An actual installed-file recheck at SHA-256
`afb7e9130d81fc37b9817bace555a454f6279cc835223d41d7641a3a76cbf158`
confirms the 256-part test already counts `"if "`, not the old
`"if p.material.id"` prefix. That finding is resolved by the installed-source
recheck; no additional source edit was needed. This is not a test-run result.

Reviewed repair target pins:

| File | SHA-256 |
|---|---|
| `authoring/assembly_source.rs` | `6377a5178e649abcd0e7ee45f2342cc56c32eee2545ffdb4c7b42d8d0b978896` |
| `authoring/structure_tests.rs` | `afb7e9130d81fc37b9817bace555a454f6279cc835223d41d7641a3a76cbf158` |
| `authoring/tests.rs` | `89697f5331daecf87975c17855ed994fa598b2e560fd12093cdf33a5575dc25c` |
| `native_dispatch.rs` | `97e8ba9eb1a53dfdcbfd9c8bf3a9ff9cc0b36eda0b704de370a2ff287a900964` |
| `native_project_spatial_tests.rs` | `1e4ef573b7542555867378a25c8633e86ef0e735a4697e3ddc09f8ac0e2c9c31` |
| `authoring/controller.rs` | `00ca57a90e260a64059ec8827ce7f9b49f240778a81cd91f3800bbe3e97e5cd9` |
| `authoring/package.rs` | `e3bd3058ee1513a950737eea2ac766403e01baf4d8f011d332ea4193c8f0f659` |

No additional implementation blocker was found within this bounded recheck.
Actual compile/test qualification belongs to the supervisor's serial commands.
Assets stay on hold; no native GUI/human/model acceptance is implied.
