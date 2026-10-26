# RUN10 native enrollment, operator entry and cooperative Poet workflow

**Status:** all final qualifying RUN10 checks have completed successfully at current snapshot `2e7ab1b867bfa3ac4e5ed49f8dd2d99e46e27113a77fdf7a0752cb9a3263f1bc` (166 inputs): full native-boundary Core passed 9,855 tests with 0 failures and 3 ignored; Poet native passed 77 with 1 ignored plus 15 CLI tests; checkpoint, focused fresh-process, native library, WASM, actual CLI, and GPU checks passed. These results qualify only the bounded software scope; real OS-provider use, GUI workflow, human acceptance and complete high-grade production remain open. Earlier failed attempts are retained below. All 120 production packets remain open.

## Scope and saved evidence

RUN10 connects the existing native controller/admission/session owners to Poet's native project entry, enrollment decisions, actor switching and source contribution/review. Implementation ownership and saved evidence are split into [enrollment](enrollment/README.md), [native entry](native-entry/README.md), [shared cooperative workflow](workflow/README.md), [source inventory and pre-edit backups](inventory/README.md), and [independent source review](review/README.md). These are source descriptions and saved-source receipts, not proof of executed tests, real OS credential access, or human acceptance.

The concrete native route is `poet studio --native-project EXISTING_DIRECTORY`. It reads bounded public project configuration, then requires explicit creation/adoption/profile/login gestures. Project-key fingerprint adoption is separate from key possession and project authority; profile login uses the strict selected native credential owner, proves a fresh challenge against current signed admission, then opens current HMC source. `--check-project` is a headless public-configuration inspection path: it does not access OS credentials, source/HMC files, adoption, or a window. Any CLI proof remains diagnostic evidence only. See the source-based [operator walkthrough](native-entry/operator-workflow.md) for create/adopt, profile enrollment, contributor switching, source work and distinct review steps; it is not GUI acceptance evidence.

The canonical authority retains enrollment requests, admission decisions, distinct review grants, contribution and source-review records. Native Poet shares the same project/session/editor owner for UI and MCP. Actor switching uses a fresh worker/session lease and separate in-memory private drafts; late source/job publication is checked against current actor and source context. These mechanisms do not attest a real person, personhood or authority, human capacity, human review, agreement adoption, or model acceptance. The new use of `identifiers` preserves scheme/issuer/scope and validity; source legacy names are cited exactly where relevant.

## Dependency and verification state

The initial native Poet integration check exited 101 because its offline lockfile did not yet include the newly selected optional encryption dependencies. The offline lock reconciliation added `argon2 0.5.3`, `base64ct 1.8.3`, `password-hash 0.5.0` and `pbkdf2 0.13.0`; the saved lock receipt reports all 850 existing package versions/source/checksums preserved. The subsequent check also exited 101 because `--locked` detected that lock update. Those are superseded setup attempts, not final source verdicts; see [initial receipts](native-check-initial.json), [integrated attempt](native-check-integrated.json), [lock reconciliation](poet-lock-reconciled.json), and [lock metadata](poet-lock-metadata.json).

The enrollment feature boundary was corrected so encrypted-key methods/tests require `sanctuary-crypto`; the selected native profile and QualiaDB core qualification explicitly enable that feature. Windows OS credential APIs are disabled in core unit-test construction paths. Poet integration tests may exercise the ordinary core qualification but are inspected not to call OS credential methods. No real human or OS credential adoption was performed by the implementation worker.






The current frozen source snapshot is `2e7ab1b867bfa3ac4e5ed49f8dd2d99e46e27113a77fdf7a0752cb9a3263f1bc` (166 inputs). The independent source review covered 53 inputs and matched the frozen snapshot with zero mismatches ([review summary](review/README.md), [manifest](review/reviewed-inputs.json)).

- Full QualiaDB Core on the native permission boundary passed 9,855, failed 0, and ignored 3 in 497.24 seconds, with no source changes. The three ignored tests are `q42::q42_volume::volume::codec_probe::tests::probe_monarch_lexicon_lz4` (440 MiB local shard), `sparql_library::external_sort::volume_publisher::tests::monarch_volume_set_resolves_a_lexicon_term` (local 8+ GiB volume set), and `sparql_library::sparql_federated::tests::test_execute_service_remote_unreachable_fails_gracefully` (localhost network integration). See [current Core receipt](core-final.json).
- The earlier restricted-boundary full Core run is preserved with 9,834 passed, 21 failed, 3 ignored; the same full suite later passed under native permissions. The paired exact loopback test also failed in sandbox and passed natively using the same binary and source snapshot ([sandbox proof](network-sandbox-proof.json), [native proof](network-native-proof.json)). These results demonstrate differing outcomes for the recorded execution boundaries. This report does not attribute all 21 restricted-run failures individually to network policy; see the preserved [restricted full-core receipt](network-restricted-results/core-final.json) and [log](network-restricted-results/core-final.log).
- Checkpoint retry regression passed 15/15 in 0.87 seconds. The focused fresh-process signer/login test passed 1/1 in 1.23 seconds. The low-level checkpoint subprocess test `restart_process_restores_exact_source_and_private_undo_redo` also passed in the Core suite. These fixture tests exercise persisted checkpoint/source recovery and key/admission reopening; full native operator/profile authenticated session-to-HMC/source recovery after restart remains unqualified. See [checkpoint receipt](checkpoint-final.json), [fresh-process login receipt](core-fresh-process-corrected.json), and [Core log](core-final.log).
- Poet native passed 77 with 1 ignored, plus 15 CLI tests passed. Native library build and WASM check both exited zero with unchanged sources ([native](native-final.json), [library](library-final.json), [WASM](wasm-final.json)).
- The current CLI public-configuration check passed with source/HMC open, OS credential access, adoption, and GUI launch recorded false. The source-linked executable alias is [`target/debug/poet-batch5-2e7ab1b867bf.exe`](../../../../target/debug/poet-batch5-2e7ab1b867bf.exe); the executed check's artifact SHA-256 is `5f29320457b05c0567025361fb228280882d0ea01cae6e2c99d44ed8f0a0c091`. The alias was not launched into the native GUI. See [CLI receipt](direct-cli/native-project-check.json) and [artifact receipt](direct-cli/2e7ab1b867bf/artifact.json).
- GPU test `accepted_material_edits_change_native_gpu_pixels` passed in 10.77 seconds with actual pixel differences: base colour 295, roughness 211, metallic 295, emissive 295. This is a native saved-source readback check, not window, human, or master-quality acceptance ([GPU receipt](gpu-final.json), [log](gpu-final.log)).
- Windows checkpoint persistence retries only OS errors 5/32/33; six bounded pauses and a 315 ms admission/wait deadline limit retries, with expected-file digest checks before every attempt. The save holds its OS writer lock and staged file, does not reevaluate authored source or repeat contribution/audit effects, and does not delete or truncate the destination. Synchronous filesystem calls themselves are not interrupted by the deadline.
## Limits and remaining outcomes

Saved regression source and bounded tests cover enrollment proof/decisions, source contribution/review, actor-private drafts, publication guards, checkpoint persistence, native profile/admission entry, and a fixture fresh login. They do not prove real OS credential-provider persistence, real external project adoption, a live graphical contributor/reviewer workflow, or full operator/profile authenticated session-to-HMC/source recovery after restart. Source review remains distinct from GUI acceptance.

High-grade editable Ecopunk masters, complete model dossiers and accepted film/open-world-game/cartoon embodiments remain incomplete; game-demo still requires explicitly reviewed derivatives. IP-03 C1–C8, VibeScript 0.2 release, all 120 production packets, and product mission delivery remain open. No production-ready, human-accepted, model-complete, payment, publication, or mission-complete claim is made.

The [machine-readable refresh receipt](refresh-console.json) records the refreshed current-pin evidence; it does not close production packets or claim human, model, or mission acceptance.









