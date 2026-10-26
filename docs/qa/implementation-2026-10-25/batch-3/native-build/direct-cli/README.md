# Direct actual-source native CLI artifact

`link_and_check.py` uses the successful parent's `library-final.json` receipt and
Cargo JSON `library-final.log`. It selects exactly one native Poet library and
canonical Vibe library by package owner URI, version, target name/kind/source,
non-test profile and required features. It never selects a newest file by time.
Every build-script-executed linked path is passed to rustc.

The immutable output alias is `C:/github/ecopunk/target/debug/poet-batch3-<source-snapshot-prefix>.exe`, compiled from
the actual `C:/github/poet/native/src/main.rs` and its actual sibling CLI modules,
linking the exact actual native Poet library. This is a direct rustc artifact,
not a normal Cargo bin-build receipt. The pre-existing running Poet process and
`target/debug/poet.exe` are not queried, closed, killed or overwritten. The script
refuses to overwrite an existing alias.

Each source snapshot has a separate immutable evidence subdirectory. Execution writes selected Cargo artifact records, input hashes, rustc version,
exact argv, full bounded link stdout/stderr, executable digest and byte size.
It then runs only hidden owned headless children: help, explicit diagnostic
no-launch source checking and MCP stdio initialization/list/unbound source denial.
Every child has a timeout; only a timed-out child owned by this script is stopped.
No GUI, Computer Use, production principal, membership or authority is supplied.

Only an exit-zero `checks.json` establishes those headless checks passed. Before
execution this directory contains preparation, not a successful artifact claim.
Link/check failures retain a failure receipt and do not trigger Cargo rebuilding.
The operator UI, authenticated cooperative production, master/model acceptance
and IP-03 C1–C8 remain separate requirements.
