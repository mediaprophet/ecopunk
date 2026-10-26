# Strict TextureEffect deserialization repair

Root's actual serial check attempt 1 reported exit 101, 266 passed / 2 failed / 9 ignored. The two failing cases were authored strict-field tests: direct domain rejection and signed MCP refusal for an `invert` object with an extra `strength` field. This is execution evidence supplied by root; this worker did not run or re-run checks. The failure exposed Serde internally tagged unit variants accepting extra payload fields despite enum-level deny_unknown_fields.

Only staged domain recipe.rs changed. Public TextureEffect variants and derived Serialize remain unchanged. Manual Deserialize delegates to a private internally tagged helper whose Grayscale and Invert are empty STRUCT variants under deny_unknown_fields, then maps them to public unit variants. Sepia retains its required intensity field and strict object shape. No test changed or weakened. Root must run the unchanged positive/negative tests after review/application.

The original handoff is preserved as handoff-before-serde-repair.json; failed candidate source is recipe-before-serde-repair.rs. Updated handoff distinguishes initial original preimage, now-installed failed candidate, and repair candidate. Only the recipe.rs entry is a repair application request; other file entries remain historical initial-wave metadata.

No canonical edits, Cargo/tests/build/formatter or duplicate audit ran. A read-only before/after diff confirmed only derived Deserialize removal and the strict adapter addition. Expected git diff exit 1 indicates differences.

Installed/failed preimage SHA-256: DA250D3EE60670BB71BD03A687EF20E0A5CF76264A0B640FB7C344EE8BE7BAA7.

Repair candidate SHA-256: 4B41224AA502D9DA2317837322C7D9BAC56FF96CBFE0F038592B0897AB58EA30.
