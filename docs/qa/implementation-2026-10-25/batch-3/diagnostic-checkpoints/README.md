# Explicit local diagnostic checkpoint workflow

Staged entry factory and five headless tests; not applied or executed by this
worker. No Cargo/core build, GUI launch or Computer Use was performed.

`entry::prepare(&[String]) -> Result<PreparedStudio, String>` returns one host and
initial draft without source evaluation or rendering. `run` uses this factory
before the main-thread window launch. Example operator arguments:

```text
--diagnostic-source --checkpoint-root C:/my-existing-checkpoints --grant capability.invoke --grant ComputationalGeometry.extrude_profile
--diagnostic-source --checkpoint-root C:/my-existing-checkpoints --reopen saved.hmc --grant capability.invoke --grant ComputationalGeometry.extrude_profile
```

The configured existing directory enables the UI checkpoint service and Save
control. Native CLI configuration is explicit local diagnostic intent; it is not
MCP path authority or production cooperative admission. No principal, human role,
membership, reviewer grant or production project bootstrap is synthesized.

The old `--diagnostic-new-source` remains supported for new sources, optionally
with a configured checkpoint root. It cannot reopen. Unsupported grants and
authority/role flags fail. Reopen requires the new diagnostic flag plus root and
one plain filename. Traversal, absolute paths, nested paths, OS device names,
links, directories and non-Geometry source fail before activation. Discovery uses
the native bounded checkpoint loader; the native service checks the selected
file/root again and activates only into an absent store identifier. Existing
accepted records are never replaced.

Files supply inert source identifier/domain data only. Diagnostic project/document
URIs come from the explicit local profile. File grants are never restored; the
new canonical session uses only the actual supported CLI grants. Reopened
execution, including an empty source, stays held until explicit Rebuild. The
initial draft is the latest accepted source cell. Required omitted grants make
rebuild fail; the file does not confer them.

A diagnostic projection wrapper labels every snapshot/outcome and keeps stable
checkpoint availability visible through editing. Accepted stored title/source,
revision, history, provenance and HMC bytes are unchanged. `run` also emits an
explicit local diagnostic banner. Source/history save remains available while
execution is held. Diagnostic file/source budgets do not cap production master
quality.

`--check-source` remains a no-launch canonical source-preparation path. Reopen
combined with it fails before discovery or activation. `prepare` itself rejects
that flag so library callers cannot mistake a prepared host for a check receipt.

Tests cover configured empty factory/save availability, no evaluation or authority
stamp, old flag compatibility, unique new source identifiers, path/authority/grant
negative cases, existing-store conflict preservation and explicit empty-session
reopen hold/rebuild. They are staged UNRUN for the parent's coherent native test
build. Fresh-process factory reopen and operator UI acceptance remain to verify.
Production authentication/bootstrap, cooperative readiness, IP-03 C1–C8 and all
complete model/master/consumer outcomes remain open.
