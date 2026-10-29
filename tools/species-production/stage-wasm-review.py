"""Stage a successful local WASM review build without replacing the scene package."""
from pathlib import Path
import hashlib
import json
import shutil

root = Path(__file__).resolve().parents[2]
evidence = root / "docs/qa/implementation-2026-10-29/species-production-round1"
build_path = sorted(evidence.glob("wasm-build-*.json"))[-1]
build = json.loads(build_path.read_text())
if build.get("exit_code") != 0:
    recovery_path = evidence / build_path.name.replace("wasm-build-", "wasm-packaging-")
    recovery = json.loads(recovery_path.read_text()) if recovery_path.exists() else {}
    if recovery.get("exit_code") != 0 or recovery.get("build_record") != build_path.name:
        raise SystemExit("Latest WASM build/packaging has not succeeded; nothing staged")
    for name, expected in recovery["outputs"].items():
        if hashlib.sha256((Path(build["package"])/name).read_bytes()).hexdigest() != expected:
            raise SystemExit("Recovered package bytes changed")
package = Path(build["package"])
digest = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
wasm_hash = digest(package / "ecopunk_wasm_bg.wasm")
js_hash = digest(package / "ecopunk_wasm.js")
revision = wasm_hash[:16] + js_hash[:16]
destination = root / "viewer/model-revisions" / revision
destination.mkdir(parents=True, exist_ok=True)
outputs = {}
for filename in ("ecopunk_wasm_bg.wasm", "ecopunk_wasm.js", "ecopunk_wasm.d.ts", "ecopunk_wasm_bg.wasm.d.ts", "package.json"):
    source = package / filename
    target = destination / filename
    if target.exists() and digest(target) != digest(source):
        raise SystemExit(f"Refusing to overwrite different retained bytes: {target}")
    shutil.copy2(source, target)
    outputs[str(target.relative_to(root)).replace("\\", "/")] = digest(target)
sources = [
    "Cargo.lock", "Cargo.toml", "crates/ecopunk-terrain/Cargo.toml",
    "crates/ecopunk-terrain/src/species_review.rs",
    "crates/ecopunk-terrain/src/species_review_source.rs",
    "crates/ecopunk-terrain/src/species_authored_review.rs",
    "crates/ecopunk-terrain/src/white_gum_authored_review.rs",
    "crates/ecopunk-terrain/src/swamp_wallaby_anatomical_r3.rs",
    "crates/ecopunk-terrain/src/zoological_anatomy.rs",
    "crates/ecopunk-terrain/src/botanical_field_builder.rs",
    "crates/ecopunk-terrain/src/zoological_loft_builder.rs",
    "crates/ecopunk-terrain/src/zoological_loft_builder/swamp_wallaby_review.rs",
    "crates/ecopunk-terrain/src/pbr_material_synth.rs",
    "crates/ecopunk-terrain/src/scene_builder_10d.rs",
    "crates/ecopunk-wasm/src/species_review_wasm.rs", "crates/ecopunk-wasm/src/lib.rs",
    "viewer/models.html",
]
record = dict(revision=revision, build_record=str(build_path.relative_to(root)),
              outputs=outputs, selected_source_sha256={path:digest(root/path) for path in sources},
              scope="Local review package; actual browser render and human acceptance remain separate",
              human_acceptance="pending")
prior = evidence / "staged-wasm-review.json"
if prior.exists():
    old_record = json.loads(prior.read_text())
    retained = evidence / ("staged-wasm-review-" + old_record["revision"] + ".json")
    if not retained.exists():
        shutil.copy2(prior, retained)
receipt = json.dumps(record, indent=2)+"\n"
prior.write_text(receipt)
(evidence / ("staged-wasm-review-" + revision + ".json")).write_text(receipt)
(destination / "build-receipt.json").write_text(receipt)
manifest = dict(package_base=f"./model-revisions/{revision}/", revision=revision,
                species_candidate_status="Authored revisions built; visual and human review pending",
                build_receipt_sha256=hashlib.sha256(receipt.encode()).hexdigest(),
                human_review="pending", review_message="Rendered revision · awaiting coordinator review and your decision.")
(root / "viewer/model-review-manifest.json").write_text(json.dumps(manifest, indent=2)+"\n")
print(json.dumps(record, indent=2))
