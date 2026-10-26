from pathlib import Path
import hashlib, json

stage = Path(__file__).parent
poet = Path('C:/github/poet')
rows = []
for after in sorted((stage / 'after').rglob('*')):
    if not after.is_file():
        continue
    relative = after.relative_to(stage / 'after')
    before = stage / 'before' / relative
    target = poet / relative
    digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest() if p.is_file() else None
    rows.append(dict(target=str(target), before=str(before) if before.is_file() else None,
        before_sha256=digest(before), after=str(after), after_sha256=digest(after),
        target_sha256_at_manifest=digest(target), bytes=after.stat().st_size,
        lines=len(after.read_text().splitlines())))
data = dict(status='staged-uncompiled-unexecuted', sibling_write_performed=False,
    assets='on-hold; existing assets untouched', owner='round4_producer',
    review_merge_required=True, target_count=len(rows), targets=rows)
(stage / 'target-manifest.json').write_text(json.dumps(data, indent=2) + '\n')
print(f'Pinned {len(rows)} complete before/after target snapshots.')
