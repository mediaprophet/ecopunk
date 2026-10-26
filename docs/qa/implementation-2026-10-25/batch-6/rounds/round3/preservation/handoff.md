# Preservation worker handoff

Staged only; no sibling writes, build, test execution, assets, network, GUI,
provider calls, credentials, commits or publication. Root reviews and installs
the frozen exact targets, merges optional top-level exports, and runs native
tests serially. Policy was declared in `policy.md` before implementation and
approved by the root. The spec delta and manifest are evidence, not a new codec.

Four proposed source targets:

- `crates/qualia-core-db/src/container_10d/section.rs`: only child delegation.
- `crates/qualia-core-db/src/container_10d/section/opaque_preservation.rs`: new API.
- `crates/qualia-core-db/tests/ten_d_opaque_preservation.rs`: thirteen native tests.
- `crates/qualia-core-db/tests/opaque_preservation_support/mod.rs`: fixtures and
  thread-local allocation counter, separate from the implementation lifecycle.

Optional addition to the other agent's `container_10d/mod.rs` section re-export:

```rust
copy_opaque_container_into, inspect_opaque_container,
replace_known_section_bytes_preserving_layout, OpaqueContainerView,
OpaquePreservationBudget, OpaquePreservationError,
```

Recommended necessary command, not executed by worker:

```text
cargo test -p qualia-core-db --test ten_d_opaque_preservation
```

Root should also use its existing narrow section/material regression selection
to establish unchanged strict behavior. The new API tests cover strict unknown
refusal, exact unknown tag/flags/metadata/payload/tail retention and fresh reopen;
v1/v2 and MID2 compatibility; whole and last-section CRC errors; last-section
offset/stride overflow; duplicates/sentinel/alignment/metadata/payload overlaps;
short/unaligned and later-table layouts; empty descriptors; byte/section budgets;
short output and unsupported length/target; unknown dependency/flags/metadata/
tail refusal; preserved known unrelated bytes and offsets; thread-local measured
allocation fixtures. All thirteen are authored, not reported passed.

Self-review caught and corrected a fixture offset: the zero-gap corruption case
uses byte 160, after the field and before the page-aligned material. Source review
also added explicit refusal to edit while uninterpreted nonzero tail bytes exist.
No operational success or allocation measurement is inferred from source review.
`rustfmt --check` is the only local executable check; root records real compiler,
runtime, counts and measurements separately. `section.rs` remains oversized;
the tracked child delegation adds no behavior to its strict parser/writer.

Useful capability delivered within this scope is structural inspection and exact
copy. It is explicitly not a safe unknown-dependent semantic rewrite or material
attachment, and does not close FND-04 or lift assets ON HOLD.
