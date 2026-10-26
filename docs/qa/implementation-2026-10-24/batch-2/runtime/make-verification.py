from pathlib import Path
import shutil, hashlib, json
base=Path(__file__).parent
source=Path('C:/github/qualiaDB/crates/vibe')
dest=base/'verification/qualiaDB/crates/vibe'
assert not dest.exists(), 'verification snapshot already exists: preserve rather than overwrite'
inputs=[]
for folder in ['src','tests','fixtures','grammar']:
    for src in (source/folder).rglob('*'):
        if src.is_file():
            dst=dest/src.relative_to(source); dst.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(src,dst)
            inputs.append(dict(source=str(src),sha256=hashlib.sha256(src.read_bytes()).hexdigest()))
for file in ['LICENSE']:
    if (source/file).exists(): shutil.copyfile(source/file,dest/file)
manifest='''[package]
name = "vibe"
version = "0.0.40"
edition = "2021"
license-file = "LICENSE"
[workspace]
[dependencies]
bytemuck = { version = "1.25.2", features = ["derive"] }
unicode-xid = "0.2.6"
'''
(dest/'Cargo.toml').write_text(manifest,encoding='utf8')
for src in (base/'candidate/vibe/src').rglob('*.rs'):
    dst=dest/'src'/src.relative_to(base/'candidate/vibe/src'); dst.parent.mkdir(parents=True,exist_ok=True)
    shutil.copyfile(src,dst)
external=[
('C:/github/qualiaDB/docs/manuals/standards/vibescript-core.md','qualiaDB/docs/manuals/standards/vibescript-core.md'),
('C:/github/ecopunk/vibe/ecopunk_continuum_drive.vibe','ecopunk/vibe/ecopunk_continuum_drive.vibe'),
('C:/github/ecopunk/vibe/ecopunk_biological_lab.vibe','ecopunk/vibe/ecopunk_biological_lab.vibe'),
('C:/github/game-demo/web/fixtures/talbingo-drive.vibe','game-demo/web/fixtures/talbingo-drive.vibe'),
]
for src,rel in external:
    src=Path(src); dst=base/'verification'/rel; dst.parent.mkdir(parents=True,exist_ok=True)
    shutil.copyfile(src,dst); inputs.append(dict(source=str(src),sha256=hashlib.sha256(src.read_bytes()).hexdigest()))
# Dedicated candidate regression integration target, same code as actual-path harness.
shutil.copyfile(base/'harness/tests.rs', dest/'tests/run05_sessions.rs')
(base/'verification-inputs.json').write_text(json.dumps(dict(
    status='private verification snapshot, not applied canonical source',
    manifest=manifest,inputs=inputs),indent=2),encoding='utf8')
print(dest)
