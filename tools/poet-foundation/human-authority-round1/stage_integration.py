"""Stage coordinator-owned registration/lock/fixture seams; no canonical writes."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

BASE = Path(__file__).resolve().parent


def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None


def main():
    rows = []

    def stage(target, transform):
        target = Path(target)
        before = target.read_text(encoding="utf-8")
        after = transform(before)
        if before == after:
            raise ValueError(f"integration change absent: {target}")
        candidate = BASE / "integration" / target.relative_to("C:/github")
        candidate.parent.mkdir(parents=True, exist_ok=True)
        candidate.write_text(after, encoding="utf-8")
        if candidate.suffix == ".rs":
            subprocess.run(["rustfmt", "--edition", "2021", "--config", "skip_children=true", str(candidate)], check=True)
        rows.append(dict(target=str(target), candidate=str(candidate), before_sha256=digest(target), after_sha256=digest(candidate)))

    stage("C:/github/qualiaDB/crates/qualia-cooperative-core/src/authoring/mod.rs",
          lambda s: s.replace("pub mod package_review;", "pub mod delegation;\nmod human_authority;\npub use human_authority::{HumanAuthorityEvidence, HumanAuthorityStatus};\npub mod package_review;", 1))
    stage("C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/mod.rs",
          lambda s: s.replace("mod admission;", "mod admission;\npub mod delegation;", 1))
    stage("C:/github/poet/native/src/graphical/mod.rs",
          lambda s: s.replace("pub mod cooperation;", "pub mod cooperation;\nmod cooperation_selection;", 1))
    stage("C:/github/poet/native/src/graphical/cooperative_workbench/projection.rs",
          lambda s: s.replace("        WorkflowEvidence {", "        WorkflowEvidence {\n            human_authority: qualia_cooperative_core::authoring::HumanAuthorityEvidence::legacy(\n                \"actor:reviewer\", PrincipalKind::Human),", 1))
    stage("C:/github/qualiaDB/crates/qualia-cooperative-core/Cargo.toml",
          lambda s: s.replace("[dependencies]\n", "[dependencies]\nqualia-app-contract = { workspace = true }\n", 1))

    def lock_update(s):
        pattern = r'(\[\[package\]\]\nname = "qualia-cooperative-core"\n.*?dependencies = \[\n)(.*?)(\n\])'
        def edit(m):
            if '"qualia-app-contract"' in m[2]:
                raise ValueError("dependency already present")
            lines = sorted(m[2].splitlines() + [' "qualia-app-contract",'])
            return m[1] + "\n".join(lines) + m[3]
        result, count = re.subn(pattern, edit, s, count=1, flags=re.DOTALL)
        if count != 1:
            raise ValueError("expected one cooperative lock entry")
        if '\nname = "qualia-app-contract"\n' not in result:
            # Poet consumes the contract library, not its dev-only Core tests.
            # All normal dependency versions are already pinned in this lock.
            block = '''[[package]]
name = "qualia-app-contract"
version = "0.0.40"
dependencies = [
 "ciborium",
 "serde",
 "serde_json",
 "thiserror 2.0.21",
]

'''
            result = result.replace('[[package]]\nname = "qualia-cooperative-core"',
                                    block + '[[package]]\nname = "qualia-cooperative-core"', 1)
        return result
    for repo in ("qualiaDB", "poet"):
        stage(f"C:/github/{repo}/Cargo.lock", lock_update)
    (BASE / "integration-manifest.json").write_text(json.dumps(dict(files=rows), indent=2) + "\n", encoding="utf-8")
    print(json.dumps(dict(staged_integration_files=len(rows), canonical_writes=False)))


if __name__ == "__main__":
    main()
