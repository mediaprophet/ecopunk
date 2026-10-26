# RUN09 native authentication integration and resource-preserving HMC editing

**Status:** bounded saved-source checks passed. The direct CLI proof also passed its recorded checks. This does not qualify production authentication, a human-operated cooperative workflow, complete Ecopunk models, product readiness, or human acceptance. All 120 packets remain open.

## Scope and saved source evidence

RUN09 adds a bounded application-level native project admission/session owner to the existing Poet/QualiaDB ownership path and extends native checkpoints to retain and edit HMC resources. Guarded source receipts are recorded in [authentication/save-receipt.json](authentication/save-receipt.json) and [HMC source transitions](hmc/source-transitions.json). The admission contract and limitations are in [authentication/contract.md](authentication/contract.md). The [independent source review](review/README.md) reviewed 51 exact source inputs against the frozen 127-file source snapshot; all 51 matched with zero mismatches. The review corrected source defects but does not establish runtime behavior or human acceptance.

A native host explicitly pins an adopted project's authority identifier, qualified issuer identifier and public key. A signed admission descriptor binds the project, qualified identifier associations, memberships, review grants and controller keys. A one-use OS nonce and controller proof establish the locally authenticated authoring session, with one active principal at a time; current admission is rechecked at each authorized operation. This is an application-level signed admission record, not a new credential standard or QDNF network handshake. No proof of actual human enrollment or project adoption exists without real host trust anchors, adopted admission signatures and controller proofs. Do not infer authority from MCP/CLI inputs, a `Identity.current_user` fallback, fixture actors, saved HMC data, or private Poet JavaScript. Keep authentication, qualified identifier association, membership/adoption and operation-specific authorization distinct. The authority journal has documented rollback/hostile-writer and Windows directory durability limits.

The HMC implementation retains inert untouched entries and opaque metadata, guards explicit edits and validates save/reopen data. GUI and MCP resource operators submit an atomic typed multi-entry plan (up to 64 edits under an aggregate byte budget) through the same core `ResourceEdit` vector and one core transaction. A legitimate material/geometry edit can require the existing companion graph to be explicitly included with an expected digest even when graph bytes remain identical; this records the paired edit without fabricating a knowledge change. Unknown semantic links and mesh/graph meaning remain unqualified. Source review covers editable content only; it does not attest package resources, whole-HMC meaning or model acceptance.

## Verification

Both initial native Cargo checks exited 101: the first on a root-module import error, the next on two test-only preview alias references. Both were corrected. [The corrected check receipt](native-check-corrected.json) records exit code 0, `source_unchanged: true`, and snapshot digest `130ad0c42d5e10e607f649ede387c1450b8d696495debaeb1d6f5ef51983ec9f`.

The frozen 127-input source snapshot is `130ad0c42d5e10e607f649ede387c1450b8d696495debaeb1d6f5ef51983ec9f`. Core passed **9,800 tests, 0 failed, 3 ignored** in 339.05 seconds ([receipt](core-final.json)); native passed **69 tests, 0 failed, 1 ignored**, plus **15 CLI tests** ([receipt](native-final.json)). Each command exited 0 with `source_unchanged: true` and the exact snapshot digest.

A separate native GPU readback exited 0 and was also source-bound: base colour changed 295 pixels, roughness 211, metallic 295 and emissive 295 ([receipt](gpu-final.json)). The native library build ([receipt](library-final.json)) and WASM check ([receipt](wasm-final.json)) exited 0 with unchanged sources and the same snapshot pin.

The direct CLI was linked from the frozen source snapshot and its checks passed ([checks](direct-cli/130ad0c42d5e/checks.json), [artifact](direct-cli/130ad0c42d5e/artifact.json), binary SHA-256 `90f9f267d917e5bb1b8d2be2f3c1b245dea6346a70cd25740c258095f99b58d9`). Help, no-window diagnostic source checking, strict JSON-RPC framing and unbound source-read denial passed. No production authority was supplied and no GUI was launched; this is headless protocol evidence, not production admission or human acceptance.

`refresh-records.py` gates the ledger refresh on final source-bound receipts and the completed direct CLI proof. It also checks the 51 reviewed source digests against the 127-file frozen snapshot, records `live_builds: 0`, and preserves all 120 packets open plus the starter template's blank milestone rows. The saved checks do not replace real host trust anchors, adopted project authority, human enrollment, contributor/reviewer GUI work or full model acceptance. [QualiaDB's saved session handoff](engine-handoff-saved.json) records the bounded source handoff without further QualiaDB code changes.

## Remaining work

- Demonstrate real host trust anchors, adopted project admission and controller-backed subject verification for enrolled contributors and reviewers, with current operation authorization and durable contribution/review records.
- Demonstrate usable graphical Poet contributor and reviewer work with current operation authorization, durable contribution/review records and actual source-to-output acceptance. Direct CLI evidence is headless and supplies no production authority.
- Complete semantic package/resource review, full model dossiers and accepted editable masters. Resource-byte preservation and source review do not prove complete HMC semantics.
- Complete every applicable embodiment packet, including film, open-world game and cartoon derivatives, then produce reviewed game-demo derivatives from accepted masters.

IP-03 C1–C8 and the full 120-packet plan remain open. No human acceptance, model completion, publication, payment or completed mission is claimed.


Machine-readable [ledger refresh receipt](refresh-console.json) records the refreshed evidence statuses and open-packet/template checks.
