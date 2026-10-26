# Bounded authoring runtime repair review

Reviewed only the two-file `repairs/authoring-runtime/target-manifest.json`
candidate, manifest SHA-256
`8c09e582a6fe3c16ceb52c381d4b97f2839bf4a4ec88044a732f643df7e5b908`.
Both staged after hashes match; both retained before hashes match; both live
files still match that failed-run baseline. No source mutation or execution was
performed by this reviewer. The supervisor reports 12/14 initial cases passed
with two failures; this note does not replace that actual execution receipt.

The ReplaceGeometry repair builds one flat indexed list outside any lambda.
The selected record is reconstructed from exact existing member references with
only its geometry expression replaced; every other list entry is an indexed
reference to its original value. The generated source has one capability invoke,
does not capture the capability root and introduces no persistent scratch binding.
The inventory's 256-part limit and existing 8192-byte generated-source gate
remain in force. No new authority or geometry algorithm is introduced.

The focused test now checks one invocation in generated source, absence of map
capture, actual output changes and retention, and exhaustion of its one permitted
dispatch. Grant/work negatives require E300/E400 and unchanged source/revision;
an unrelated parser/capture failure can no longer count as the intended denial.
The 256-part fixture uses `mat` rather than the reserved `material` token and
requires canonical program parsing before session execution. Existing factor
delta, extension retention and source-depth assertions remain.

No immediate blocking flaw was found in this bounded source review. Both repairs
remain unqualified until the supervisor installs the exact hashes and completes
the selected actual runtime checks. No builds/tests/live writes were run here.

| Target suffix | Before SHA-256 | After SHA-256 |
|---|---|---|
| `authoring/assembly_source.rs` | `6377a5178e649abcd0e7ee45f2342cc56c32eee2545ffdb4c7b42d8d0b978896` | `531d9d3474ac5336564cb31e38f8f5415a4e42c94041cf787fc412bdd8f88ebb` |
| `authoring/structure_tests.rs` | `afb7e9130d81fc37b9817bace555a454f6279cc835223d41d7641a3a76cbf158` | `ca400b6c75de59790bc2e5977e613daac2611c70f4a839a6359e80e4dbea178c` |

Assets remain on hold. This scoped review makes no model, native GUI, human
cooperation, language-release or delivery-readiness claim.
