# Identifier qualification boundary — next work must reuse native authority

Source-only inspection, 2026-10-25. Existing `QualifiedIdentifier` already carries
value, scheme, issuer, project scope, evidence reference, validity and revocation.
Saved `IdentifierAssociationEvidence` retains the exact association used but
grants no authority when reopened. Native authority constructs a principal from
the current signed admission/session/grant state. These are existing capabilities,
not gaps to replace with another resolver.

FND-01 must distinguish this authenticated actor association from a MID2 entity
handle's qualified referent mapping. A local mesh/entity handle, package digest
or equal string is not proof of either. The next bounded integration should use
existing graph records to bind representation/referent/release/derivation under
qualified mappings, then demonstrate the actual native caller. This note does
not prove that integration exists or rerun any authority tests. Asset production
remains on hold and FND-01 remains open.

Source pins:
- `C:\github\qualiaDB\crates\qualia-cooperative-core\src\authoring\types.rs` — `9788d844d87055db4e1cfa1d71ffea4d4f0c9bae08ffdd49ebb950357abea68f`
- `C:\github\qualiaDB\crates\qualia-cooperative-core\src\authoring\records.rs` — `07bc8ce23c4d91acdb10accb219de00c78533956f2dc2158927390e859da54c6`
- `C:\github\qualiaDB\crates\qualia-core-db\src\poet_host\native_authority\mod.rs` — `4bdfcb3fbdd34f7c38b65ad9b785f9c6d818fe02719a4fa5cc65e767eb8b13c9`
