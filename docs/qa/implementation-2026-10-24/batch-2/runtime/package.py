from pathlib import Path
import hashlib, json

base=Path(__file__).parent
root=base/'candidate/vibe'
guards={Path(e['target']).as_posix().lower():e for e in json.loads((base/'source-guards.json').read_text())}
entries=[]
for file in sorted(p for p in root.rglob('*') if p.is_file()):
    target=Path('C:/github/qualiaDB/crates/vibe')/file.relative_to(root)
    guard=guards.get(target.as_posix().lower())
    entries.append(dict(target=str(target), candidate=str(file.resolve()),
        original_sha256=guard['sha256'] if guard else None,
        candidate_sha256=hashlib.sha256(file.read_bytes()).hexdigest(),
        original=guard['original'] if guard else None))
(base/'patch-map.json').write_text(json.dumps(entries,indent=2),encoding='utf-8')
print(f'{len(entries)} canonical files; {len(guards)} guarded originals; no parser edits')
