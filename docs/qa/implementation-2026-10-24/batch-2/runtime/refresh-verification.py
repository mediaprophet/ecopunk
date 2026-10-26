from pathlib import Path
import shutil,hashlib,json
base=Path(__file__).parent
dest=base/'verification/qualiaDB/crates/vibe'
entries=[]
for src in (p for p in (base/'candidate/vibe').rglob('*') if p.is_file()):
    dst=dest/src.relative_to(base/'candidate/vibe')
    dst.parent.mkdir(parents=True,exist_ok=True); shutil.copyfile(src,dst)
    entries.append(dict(path=str(src.relative_to(base)),sha256=hashlib.sha256(src.read_bytes()).hexdigest()))
shutil.copyfile(base/'harness/tests.rs',dest/'tests/run05_sessions.rs')
(base/'verification-overlay.json').write_text(json.dumps(entries,indent=2),encoding='utf8')
print(f'refreshed {len(entries)} exact staged candidate files only')
