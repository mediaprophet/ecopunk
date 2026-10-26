"""Normalize reviewed frozen candidates; application is a separate reviewed step."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
OWNER = Path("C:/github/qualiaDB")


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


rows = []


def add(target, stage, before, after, job):
    target, stage = Path(target), Path(stage)
    assert target.resolve().is_relative_to(OWNER.resolve())
    assert sha(stage) == after, str(stage)
    assert (sha(target) if target.exists() else None) == before, str(target)
    rows.append(dict(target=str(target), stage=str(stage), before_sha256=before,
                     after_sha256=after, job=job))


for row in load(HERE / "frames/manifest.json")["files"]:
    add(row["target"], HERE / "frames" / row["after"]["path"],
        row["before"]["sha256"] if row["before"] else None,
        row["after"]["sha256"], "R3-FRAMES")
for row in load(HERE / "geospatial/target-manifest.json"):
    add(row["target"], HERE / "geospatial" / row["after_snapshot"],
        row["before_sha256"], row["after_sha256"], "R3-GEO")
for row in load(HERE / "preservation/manifest.json")["owned_targets"]:
    add(OWNER / row["target"], HERE / "preservation/after" / row["target"],
        row["before_sha256"], row["after_sha256"], "R3-PRESERVE")

# Shared routing is merged by the supervisor without rewriting worker snapshots.
integration = HERE / "integration"
integration.mkdir(exist_ok=True)
route = next(row for row in rows if row["target"].replace("\\", "/").endswith(
    "/container_10d/mod.rs"))
text = Path(route["stage"]).read_text(encoding="utf-8")
text += """
// Explicit opaque retention is separate from strict typed section admission.
pub use section::{
    copy_opaque_container_into, inspect_opaque_container,
    replace_known_section_bytes_preserving_layout, OpaqueContainerView,
    OpaquePreservationBudget, OpaquePreservationError,
};
"""
merged = integration / "container_10d_mod.rs"
merged.write_text(text, encoding="utf-8")
route.update(stage=str(merged), after_sha256=sha(merged),
             job="R3-FRAMES + supervisor preservation exports")

docs = integration / "standards"
docs.mkdir(exist_ok=True)
canonical = OWNER / "docs/manuals/standards/m64-matter-container.md"
old = canonical.read_text(encoding="utf-8")
new = old + """

## Subsequent developmental georaster profile

An explicit opt-in M64 version-3 GRM3 georaster profile is described in
[its separate developmental contract](m64-georaster-v3.md). The current default
`M64_VERSION` remains 2 and the legacy raster writer still emits its unchanged
version-1 bytes. The newer checked reader additionally admits the complete v3
profile under its separate structural/metadata checks. Earlier readers refuse
v3; the original Round 2 execution above qualified only its recorded v1/v2 scope.
The new profile retains f64 origin, integer source elevations, cell spacing,
datum/epoch declarations, source extent and explicit padding/nodata policy.
It does not perform a CRS/datum transformation or validate identifier authority.
See the source-pinned Round 3 results for execution status; this addition alone
does not qualify the profile or establish production readiness.
"""
stage = docs / canonical.name
stage.write_text(new, encoding="utf-8")
add(canonical, stage, sha(canonical), sha(stage), "supervisor spec integration")

canonical = OWNER / "docs/manuals/standards/10d-opaque-preservation.md"
stage = docs / canonical.name
stage.write_text("# Explicit .10d structural opaque retention\n\n"
    "Status: developmental SDK contract. Semantic admission and required/optional "
    "extension negotiation remain separate.\n\n"
    + (HERE / "preservation/policy.md").read_text(encoding="utf-8")
    + "\n\n" + (HERE / "preservation/spec-delta.md").read_text(encoding="utf-8"),
    encoding="utf-8")
add(canonical, stage, sha(canonical) if canonical.exists() else None, sha(stage),
    "R3-PRESERVE canonical documentation")

assert len({row["target"].lower() for row in rows}) == len(rows)
(HERE / "install-manifest.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")
print(json.dumps({"prepared_targets": len(rows), "manifest_sha256": sha(HERE / "install-manifest.json"),
                  "live_files_written": False}))
