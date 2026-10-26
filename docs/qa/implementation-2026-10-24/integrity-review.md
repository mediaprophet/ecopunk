# RUN-04 asset-integrity independent source review

2026-10-24. Scope: `tools/asset-integrity` as a native metadata auditor for the
existing `ecopunk-model-review-v1` component and graph declarations. Read-only
source review; no source edits, build, test rerun, HMC decoding, model review,
human acceptance or native product qualification was performed by this reviewer.
The supervisor reported six existing tests passing; that result is not an
independent rerun.

## Reviewed snapshot

SHA-256, before any remediation (line references below use this snapshot):

| File | SHA-256 |
|---|---|
| `tools/asset-integrity/src/main.rs` | `fb330c17de12eefdfb5ffa1e6677cab951dc8bdfa5bcc5ff8108bd111443bafb` |
| `tools/asset-integrity/src/tests.rs` | `cfeca041a2159bd1053eaf223a87b8c7bfbc52796c44cebbb320d3bbd7642610` |
| `tools/asset-integrity/Cargo.toml` | `b20f17f05907d58e7837d78c8fd17785df19084df4531c0502a00761baa5c154` |
| `tools/asset-integrity/Cargo.lock` | `2a29a38ac9deb4c05ddb1581e064aa9a62c6082be20e360d4d8793e4ea63acb2` |

Read governing AGENTS/dispatch, architecture/development-mode contracts and the
real Corymbia manifest's structure. This review does not validate that manifest's
stored resources. Findings apply to the hashes above; subsequent revisions need
focused inspection and relevant new tests.

## Mandatory corrections

1. **P1: actual resource reads are unbounded** (`main.rs:44–48,65–79`). Limits
   account only for untrusted declared lengths; `hash_file` reads to EOF. A file
   declared as zero bytes can exceed the 512 MiB limit, and several understated
   files can exceed the 2 GiB aggregate. A growing file can continue beyond both.
   Check opened-handle metadata and regular-file type, reserve actual canonical
   resource lengths with checked arithmetic, and enforce a remaining read budget
   while streaming. A rejected/skipped resource must have an explicit finding.
   Reject non-regular manifest/resources rather than allowing a named pipe or
   similar target to block before a read cap helps. Test actual length above a
   small injectable cap with understated metadata, aggregate actual overflow,
   and growth via an injected reader; retain fixed-buffer streaming.

2. **P1: aliases can be reported as verified without verification**
   (`main.rs:69–81`). Hash/length/read findings use the first declaration's raw
   path, while reference statuses look up raw paths. Two aliases resolving to
   one canonical file therefore give the later alias `ok` when the shared file
   is corrupt. Conflicting metadata marks only the later declaration, leaving
   the earlier declaration `ok` even though the canonical group conflicts.
   Aggregate declaration accounting also counts raw aliases, contradicting the
   documented distinct-resource budget. Key resource outcomes by canonical
   `PathBuf`, attach them to every associated reference, and leave every skipped
   or conflicting group explicitly unverified. Test two canonical aliases with
   a shared digest mismatch, conflicting declarations in both orders and
   canonical deduplication at the aggregate boundary. Canonical paths identify
   paths, not hard-link equivalence; preserve that stated scope.

3. **P1: findings expansion is quadratic** (`main.rs:71,81`). N conflicting
   references to one raw path produce roughly N conflict issues, then clone all
   those issues into each of N references. A bounded 4 MiB manifest can thus
   trigger vastly larger allocation/output. Record shared findings once and
   associate bounded stable issue identifiers or a group outcome with each
   reference. Avoid merely moving the same Cartesian expansion to canonical
   aliases. Test thousands of repeated conflicts and assert linear report size
   and preserved reference count.

4. **P2: lexical path contract is not enforced on Windows**
   (`main.rs:36–41`). `Path::components` normalizes repeated slashes and interior
   `.`; `a//b`, `a/./b` and `a/` can be admitted despite the stated strict
   nonempty/non-current component rule. Checking a colon only at byte 1 also
   leaves names such as `asset.bin:stream` eligible for Windows alternate data
   streams. Reject empty, `.` and `..` raw slash-delimited segments and all
   colons before platform parsing, alongside existing absolute/backslash checks.
   Test these lexical cases independently of file existence, plus drive, UNC
   and canonical symlink escape cases on an available Windows environment.
   The existing canonical containment check is useful and should remain.

## Sound elements and limits

The manifest read uses `take(MAX_MANIFEST + 1)` and checks the resulting length;
ordinary oversize JSON cannot bypass that byte bound by growing after stat.
SHA syntax is checked and normalized, streamed byte counting uses checked
addition, and digest/length mismatch causes overall exit 1. Invocation/read/schema
errors return exit 2. Schema/declaration iteration and BTree ordering support
repeatable output for one fixed filesystem/manifest snapshot. Absolute canonical
paths and OS error text mean byte-identical output across different machines or
failure environments is not established. Saturation prevents integer wrap but
does not enforce actual-byte limits.

Existing tests do not establish Windows symlink/ADS behavior, canonical alias
failure propagation, bounded actual reads, report-size bounds or concurrent
mutation handling. Length checks detect observed length changes, not all
same-length content replacements. Canonicalize-before-open has a filesystem race
under a hostile concurrent writer; no race-resistant filesystem sandbox is
claimed or demonstrated. These limitations should remain explicit rather than
being described as a native admission/completion gate.
