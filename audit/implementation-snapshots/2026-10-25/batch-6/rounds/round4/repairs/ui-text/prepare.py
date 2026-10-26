from pathlib import Path
import hashlib, json

here = Path(__file__).parent
poet = Path('C:/github/poet')
inspection = json.loads((here / 'inspection.json').read_text())
replacements = [
    ('\u00c3\u201a\u00c2\u00b7'.encode('utf-8'), b'|', 'double-misencoded middle-dot separator'),
    ('\u00c2\u00b7'.encode('utf-8'), b'|', 'misencoded middle-dot separator'),
    ('\u00e2\u20ac\u2122'.encode('utf-8'), b"'", 'misencoded apostrophe'),
]
rows, details = [], []
for inspected in inspection:
    target = Path(inspected['target'])
    before = target.read_bytes()
    assert hashlib.sha256(before).hexdigest() == inspected['sha256'], f'Source changed after inspection: {target}'
    after, edits = before, []
    for old, new, reason in replacements:
        count = after.count(old)
        if count:
            edits.append(dict(reason=reason, count=count, old_hex=old.hex(), replacement=new.decode('ascii')))
            after = after.replace(old, new)
    if after == before:
        continue
    relative = target.relative_to(poet)
    before_path, stage = here / 'before' / relative, here / 'after' / relative
    before_path.parent.mkdir(parents=True, exist_ok=True)
    stage.parent.mkdir(parents=True, exist_ok=True)
    before_path.write_bytes(before)
    stage.write_bytes(after)
    assert all(old not in after for old, _, _ in replacements)
    rows.append(dict(target=str(target), stage=str(stage.resolve()),
        before_sha256=hashlib.sha256(before).hexdigest(), after_sha256=hashlib.sha256(after).hexdigest(),
        job='R4 literal UI text encoding repair'))
    details.append(dict(target=str(target), replacements=edits))
manifest = here / 'install-manifest.json'
manifest.write_text(json.dumps(rows, indent=2) + '\n', encoding='utf-8')
(here / 'replacement-details.json').write_text(json.dumps(details, indent=2) + '\n', encoding='utf-8')
print(json.dumps(dict(changed_files=len(rows), replacement_count=sum(e['count'] for d in details for e in d['replacements']),
    manifest_sha256=hashlib.sha256(manifest.read_bytes()).hexdigest(), manifest=str(manifest.resolve()))))
