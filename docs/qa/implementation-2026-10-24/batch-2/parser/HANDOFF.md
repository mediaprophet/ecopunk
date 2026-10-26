# RUN-05A — canonical cell full-input parser candidate

**Scope:** `VIBE-02`, `VBC-004`, blocker `RUN05-FULL-INPUT` only.

The exact candidate is `candidate/parse/mod.rs`; `candidate.patch` contains the minimal parser hunk. The fix consumes one optional `;` via `Parser::bump()` and then requires `TokenKind::Eof`. Since `bump()` calls the existing lexer, invalid trailing characters and malformed strings/comments continue to return their original lexer diagnostics. No grammar, version, or lexer changes are included.

Focused cases are staged in `tests/cell_full_input.rs`: bare/trailing-semicolon inputs, whitespace/comments, tokens and second statements/semicolons after the terminator, malformed lexical input, and nested records/conditionals. The test file is intended for the canonical `vibe` crate integration-test directory; it has **not been run**. A bounded harness would need the actual crate modules/dependencies and was not assembled, so this deliverable is unqualified pending supervisor application and execution against QualiaDB.

`source-guard.json` records the source SHA256 before and after staging. Both are `89B666C571E52BBDE29267EA4EAF13CCA17577E07C7F66091398C3EF98E6D26B`. The sibling source remains read-only and unchanged. No checker/session/effect work is included.