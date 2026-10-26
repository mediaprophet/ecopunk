# R3-POET selected M64 resource inspection

Development contract: Poet-only read-only extension. No Core behavior, dependency
manifest, package bytes, resources or source are edited. Existing ReadSource
authorization remains necessary over the same bound project/document/source
owner; neither result metadata, role labels nor a request principal grant it.

Request carries package filename/SHA, resource key/SHA and expected SourceRef.
The native worker uses fixed 64-entry interval scratch and explicit file/chunk/
record budgets. This is a bounded UI inspection profile, never a format/master
quality cap. Budget refusal is reported as unassessed under that profile, with
required size/count and configured limit where known, not as corrupt content.
The checkpoint owner's existing host limits additionally cap package loading.

Existing canonical Checkpoint::resources supplies inert bytes after bounded
package load and trusted binding/immutable-release checks. The selected resource
and package digests must match. Only M64/E64 magic invokes canonical M64Reader;
unknown payloads return explicit unsupported/unassessed errors. No reinterpretation
or fallback occurs. Exact typed metadata and verification counts return through
the same job worker and current source/session/authority publication guards.

Legacy v1/v2 explicitly lacks resolution/vertical-datum/nodata/source-extent
metadata. Opt-in v3 reports exact GRM3 declarations, absolute raw elevation mm
(independent of origin base), nodata and padding policies and structural/CRC
scope. No source identifiers, rights, scientific authority or human acceptance
are inferred. UI details remain attached to selected filename/package/resource
digests and current source/session context; logout/revocation/source changes
invalidate results. Existing controls and resource-preserving save remain intact.

The same native CheckpointService loads twice, dropping the first carrier before
the second bounded reload, to refuse disk changes during inspection. The result
identifies an inspected package snapshot, not perpetual filesystem freshness or
an external anti-rollback guarantee. No package mutation occurs on any path.

All work is staged for review. No Cargo, native GUI, credentials, asset work or
runtime acceptance is performed by this worker. FND-05/IP-03 C1–C8 remain open.
