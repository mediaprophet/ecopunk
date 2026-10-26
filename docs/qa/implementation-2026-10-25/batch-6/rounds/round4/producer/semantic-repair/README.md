# Selected metadata semantic safety repair

Seven precise staged target changes against the prior 20-file producer candidate
produce a 21-target candidate. Six existing files have complete `before/`
snapshots; `cooperation/georaster_metadata_tests.rs` is new. The root's actual
integration candidate remains root-owned; this worker changed no sibling source,
live package, existing asset, credential, build or test output.

`SourceAnnotation` now requires `origin_payload_digest`, computed from the actual
created canonical M64 bytes. Its description/reference/declaration describes
that original source payload only. A different current payload digest denotes
derived data, not another observation covered by the source claim. The existing
canonical HMC save/audit supplies current package/resource identity, writer and
predecessor evidence; no new graph, private ledger or scientific qualification
is introduced. The original payload digest alone does not prove source bytes
remain available or complete source/rights lineage.

Editing uses a conservative metadata profile. Absent selected metadata has no
metadata claim to carry forward. Present metadata must be no more than 16 KiB,
decode entirely as the known `SourceAnnotation`, pass `deny_unknown_fields` and
source-declaration validation, contain a nonzero origin SHA and declare exactly
Unverified qualification, Synthetic status and no observation claim. Cursor
position must equal the entire metadata length; trailing CBOR objects/bytes are
refused. Opaque, Provided, Unknown or observational selected metadata refuses the
edit without a package write until a qualified typed derivation contract exists.
This is a bounded unsupported/unassessed result, not a corrupt-content claim.

Synthetic sources created in the native form remain editable. Their original
SHA and exact source annotation bytes stay unchanged through `ReplaceBytes` while
the current raster SHA changes. The fixture verifies origin SHA equals the
initial produced raster, remains distinct from later edited bytes and survives
fresh-process reopen. The seeded editable raster now has an explicit canonical
Synthetic source annotation. Unrelated unknown resource bytes and opaque CBOR
remain exact. Byte preservation is never described as semantic metadata proof.

The newly added ninth case is:
`graphical::cooperation::tests::georaster_edit_tests::metadata_tests::georaster_edit_refuses_selected_opaque_provided_unknown_and_observation_metadata`.
It prepares canonical ephemeral packages with each selected metadata category,
then expects authorized native raster edit refusal and exact unchanged HMC bytes.
The source-declaration unit also checks strict trailing-byte refusal and source
status/observation/origin-digest bounds. All nine expected library names are
listed in the producer README. No cases ran in this worker; root owns the actual
library command, full frozen source pins and execution outcome.

`rustfmt --edition 2021 --config skip_children=true` parsed/formatted all seven
changed staged Rust files. There were no Cargo invocations. Asset production
remains on hold. FND-01–06, IP-03 C1–C8, scientific/rights qualification, whole-HMC
completion and accepted film/open-world-game/cartoon packages remain open.
