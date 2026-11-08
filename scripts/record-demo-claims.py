"""Instantiate the existing production claim template for each exported demo mesh.

Documentation/interchange only; does not create an alternative claim runtime.
No row is awarded acceptance by this script.
"""
import copy
import csv
import hashlib
import json
from pathlib import Path
import sys

repo = Path(__file__).resolve().parents[1]
root = Path(sys.argv[1]).resolve()
template_path = repo / "docs/planning/tech/assets/model-production-template/claims.template.json"
template = json.loads(template_path.read_text(encoding="utf-8-sig"))
rows = list(csv.DictReader((root / "inventory.tsv").open(encoding="utf-8-sig"), delimiter="\t"))
links = []
for row in rows:
    relative = Path(row["resource"])
    if relative.is_absolute() or ".." in relative.parts or relative.suffix != ".10d":
        raise ValueError("Unexpected inventory resource")
    artifact = root / relative
    digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
    name = relative.stem
    dossier = root / "dossier" / "cases" / name
    dossier.mkdir(parents=True, exist_ok=True)
    ledger = copy.deepcopy(template)
    ledger["document_kind"] = "claim_ledger"
    ledger["subject"]["entity_id"] = f"urn:ecopunk:demo:2026-10-29:{name}"
    ledger["subject"]["dossier_id"] = f"urn:ecopunk:dossier:demo:2026-10-29:{name}"
    ledger["subject"]["source_revision"] = "demo-working-source-2026-10-29; see rebuild receipt"
    ledger["subject"]["requirements_baseline_ref"] = "docs/planning/tech/assets/model-completion-claims.md"
    ledger["subject"]["package_content_refs"] = [relative.as_posix(), "knowledge.q42", "sources/"]
    (dossier / "claims.json").write_text(json.dumps(ledger, indent=2) + "\n", encoding="utf-8")
    (dossier / "README.md").write_text(
        f"# Demo case: {name}\n\n"
        f"Resource: `{relative.as_posix()}`. SHA-256: `{digest}`.\n\n"
        f"Decoded mesh: {row['vertices']} vertices, {row['triangles']} triangles. "
        "This technical observation does not award catalogue acceptance.\n\n"
        "[claims.json](claims.json) preserves the complete existing template. "
        "Film, game and cartoon completion, expanded subject-specific obligations, "
        "scientific validity and actual human acceptance remain open. "
        "This illustrative demo case is not a complete species embodiment. "
        "The claim projection does not replace native Q42/project/review authority.\n",
        encoding="utf-8",
    )
    links.append(f"- [{name}](cases/{name}/README.md) — `{digest}`")
(root / "dossier/README.md").write_text(
    "# Demo case dossiers\n\n"
    "Each exported mesh has the full pinned production-template claim projection. "
    "No claim was marked accepted by generation. Required expanded cases remain "
    "unassessed; none is silently waived. These external dossiers accompany a "
    "resource pack, not a complete accepted model release.\n\n" + "\n".join(links) + "\n",
    encoding="utf-8",
)
print(f"Recorded {len(links)} dossiers using catalogue {template['catalogue']['version']}.")
