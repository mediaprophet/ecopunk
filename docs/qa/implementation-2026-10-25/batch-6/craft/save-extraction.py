"""Authorized bounded Craft extraction; originals are read-only inputs."""
from pathlib import Path
import hashlib, json, shutil, datetime
origin = Path('C:/github/poet-cadcraft')
target = Path('C:/github/craft')
if target.exists():
    raise SystemExit('Craft target already exists; inspect and preserve it instead of replacing')
files = [Path('crates/geom/Cargo.toml')]
files += sorted(p.relative_to(origin) for p in (origin/'crates/geom/src').glob('*.rs'))
notices = [Path(name) for name in ['LICENSE-MIT','LICENSE-APACHE','NOTICE','ATTRIBUTION.md']]
records = []
target.mkdir()
for relative in files + notices:
    source = origin/relative
    destination = (target/relative if relative in files else target/'migration/origins/cadcraft'/relative)
    destination.parent.mkdir(parents=True, exist_ok=True)
    data = source.read_bytes()
    destination.write_bytes(data)
    digest = hashlib.sha256(data).hexdigest()
    records.append(dict(origin=str(source),destination=str(destination),origin_sha256=digest,copied_sha256=hashlib.sha256(destination.read_bytes()).hexdigest()))
(target/'migration/origins/cadcraft/source-map.json').write_text(json.dumps(dict(
    saved_at_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    disposition='Copied selected reusable geometry crate only. Original files unmodified. No GUI, codecs, fonts, DWG or vendor code copied.',
    files=records),indent=2)+'\n')
manifest='''[workspace]
resolver = "3"
members = ["crates/geom", "crates/authoring"]

[workspace.package]
version = "0.4.0"
edition = "2024"
license = "MIT OR Apache-2.0"
rust-version = "1.90"

[workspace.dependencies]
cadcraft-geom = { path = "crates/geom" }
serde = { version = "1", features = ["derive"] }
qualia-core-db = { path = "C:/github/qualiaDB/crates/qualia-core-db", default-features = false, features = ["profile_target_1024", "zk-culling", "gpu-runtime", "gpu-native-portable", "gpu-native-dx12", "wgsl-forge", "privacy-he", "sanctuary-crypto"] }

[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
dbg_macro = "warn"
todo = "warn"
too_many_arguments = "allow"
type_complexity = "allow"
'''
(target/'Cargo.toml').write_text(manifest)
(target/'crates/authoring/src').mkdir(parents=True)
(target/'crates/authoring/Cargo.toml').write_text('''[package]
name = "craft-authoring"
description = "Bounded source-backed profile and identified assembly editing over shared native kernels"
version.workspace = true
edition.workspace = true
license.workspace = true

[lints]
workspace = true

[dependencies]
cadcraft-geom.workspace = true
qualia-core-db.workspace = true
''')
receipt=Path(__file__).parent/'extraction-receipt.json'
receipt.write_text(json.dumps(records,indent=2)+'\n')
print(f'Saved {len(records)} exact copied source/notice inputs and new workspace manifests; originals unchanged.')
