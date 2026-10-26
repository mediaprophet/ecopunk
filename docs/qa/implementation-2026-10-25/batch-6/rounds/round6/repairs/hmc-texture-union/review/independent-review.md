# Independent bounded HMC texture-union review

**No remaining source blocker found.** Final manifest SHA-256:
`9c0dbd2fb3adcb9f9051748bbf543142569872053b281b3c840984062893714b`.
[Source pins](source-pins.json) confirm all five candidate hashes match that
manifest: three Core source files, six-case public integration test source and
the Poet native test-target registration. Root owns application and execution.
This reviewer ran no Cargo/build/test, GPU, provider, GUI or live sibling write.

The final repair plans an exact union of participating texture dependencies.
It verifies dependency SHA against actual payload, requires exact payload/MIME
agreement across declarations and with a matching retained record, and writes
the original retained ResourceData unchanged. That preserves opaque metadata,
source attribution, parent lineage and unrelated retained resources; it does not
invent texture provenance absent from the input. New canonical texture entries
are inserted once. Geometry, graph and source entries never gain that equality
shortcut: their keys are reserved before dependencies, and cross-kind/reserved
collisions refuse. Participating case aliases refuse explicitly without silently
normalizing or merging distinct resource keys.

Two early review findings were repaired before freeze:

- Borrowed key/graph-key length checks and per-entry payload/MIME charge checks
  now precede key clones and dependency SHA work. Unique aggregate file/entry
  charges still precede canonical packaging. Retained exact matches and repeated
  new declarations consume one actual output entry/charge.
- New validation applies to dependencies participating in a merge. Source-only
  encoding continues to preserve unrelated future/opaque entries, including
  unusual keys/kinds under the texture-like namespace. The public regression
  explicitly checks their exact bytes, metadata, source and parent preservation.

The codec remains detached/pure until it returns encoded bytes; it does not
mutate the checkpoint or an existing destination on conflict. The public test
uses the real save/reopen pipeline and verifies destination bytes and directory
entries remain exact after conflicting candidate composition refuses. Other
cases cover retained metadata and lineage, shared new images, exact unique entry
limits, MIME/digest/alias/cross-kind/reserved conflicts and preflight ordering.

The six tests use the public native importer/compiler on a synthetic in-memory
GLB/KTX2 fixture; the native Poet `foundation_hmc_texture_union` target references
that same public test file through the existing dependency closure and adds no
new dependency. Source inspection is not evidence the importer, tests or full
native consumer execute successfully. Root must run the public test target and
the originally failing contributor/reviewer HMC workflow on saved source before
citing repair qualification. No broader regression, renderer, semantic/rights
admission or model-completion claim is made. Assets remain HOLD.
