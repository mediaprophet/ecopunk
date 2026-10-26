# FND-04 opaque preservation policy — declared before implementation

This additive canonical Core SDK operation inspects and copies `.10d` v1/v2
bytes structurally. It does not admit unknown payloads to rendering, execution,
governance or authoring. Existing `parse_section_table` and material APIs retain
their strict compatibility behavior. Unsupported outer header grammar/version
refuses. Header flags and descriptor reserved metadata are retained without
interpretation in the explicit opaque inspection/copy path only.

Every descriptor is checked before a view or output is returned: known alignment
tier, bounds, metadata and payload overlaps, unique nonzero tags, stride product,
padding, per-payload and whole-file CRC. Explicit caller byte and section limits
bound processing; Core's existing maximum section count also applies. Unaligned
zero-copy tables refuse with a typed error. Unknown alignment grammar refuses.
Returned views borrow immutable source; safe Rust prevents aliased mutable output.
Exact copy retains all bytes, including unknown tag/payload/metadata, supported
outer version, flags, ordering, offsets and opaque tail bytes.

Known-section byte replacement is deliberately limited to the existing layout
and length. It is structural surgery; payload validity and cross-section semantic
admission remain the caller/typed owner's responsibility. Byte-identical no-ops
may copy any structurally accepted file. A semantic change refuses if any header
flag is unknown, any descriptor has reserved metadata, or any section tag has
unimplemented/unknown semantics. Nonzero uninterpreted tail bytes also refuse a
changed replacement; zero tail padding can remain. Required/optional and dependency negotiation for
unknown sections is unresolved; changing a known payload could invalidate an
unknown dependency even if all offsets stay unchanged. The SDK therefore does
not pretend that a CRC makes that change safe. No section addition, relocation,
version upgrade or flags rewrite is provided. No new format is defined.

Source mutation is never performed. All input checks, refusal decisions, length,
capacity and budget checks finish before the first output write. On success only
target payload, target descriptor CRC and whole-file CRC may change; all other
bytes are retained. This scope is useful retention tooling, not a completed
FND-04 gate, native Poet workflow, model, package or asset acceptance. Assets stay
ON HOLD. Root independently reviews/installs and runs native tests; staged source
and test presence are not executed evidence.

Ownership: `section.rs` module/re-export delegation only; implementation and tests
in focused child/new integration files. `section.rs` is already oversized. This
additive child extraction tracks decomposition without moving or changing the
strict parser/writer. Further parser/writer and legacy test decomposition remains
separate maintenance work; no blanket exemption for substantial new behavior.
