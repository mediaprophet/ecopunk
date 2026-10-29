"""Stage editable model inputs without building or generating model geometry."""
from pathlib import Path
import json
import re
import shutil

root = Path(__file__).resolve().parents[2]

def vibe_literal(value):
    if isinstance(value, dict):
        if any(not re.fullmatch(r"[A-Za-z_][A-Za-z_0-9]*", key) for key in value):
            raise ValueError("Unsupported source record key")
        return "{ " + ", ".join(key + ": " + vibe_literal(item) for key, item in value.items()) + " }"
    if isinstance(value, list):
        return "[" + ", ".join(vibe_literal(item) for item in value) + "]"
    return json.dumps(value, ensure_ascii=False, allow_nan=False)

for species, json_name, vibe_name in [
    ("white-gum-review-r1", "white-gum-r2.authored.json", "white-gum-r2.vibe"),
    ("swamp-wallaby-review-r1", "swamp_wallaby_anatomical_r3.json", "swamp-wallaby-r3.vibe"),
]:
    source = root / "tools/species-production" / species
    target = root / "assets/production-studies" / species / "authoring"
    target.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source / json_name, target / json_name)
    if (source / vibe_name).exists():
        shutil.copy2(source / vibe_name, target / vibe_name)
    else:
        data = json.loads((source / json_name).read_text(encoding="utf-8"))
        (target / vibe_name).write_text(
            "// Editable synthetic anatomical candidate; measurements and rig remain unqualified.\n"
            "let mut species_asset = " + vibe_literal(data) + ";\n", encoding="utf-8")
    print(target / vibe_name)
