from pathlib import Path
import hashlib, json
repair = Path(__file__).parent
producer = repair.parent
before_root = repair / 'before'
relative = sorted(p.relative_to(before_root) for p in before_root.rglob('*') if p.is_file())
relative.append(Path('native/src/graphical/checkpoints/georaster_edit/source.rs'))
rows = []
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest() if p.is_file() else None
for rel in relative:
    before, after = before_root / rel, producer / 'after' / rel
    rows.append(dict(relative=rel.as_posix(), target=str(Path('C:/github/poet') / rel),
        before=str(before) if before.is_file() else None, before_sha256=digest(before),
        after=str(after), after_sha256=digest(after), lines=len(after.read_text().splitlines())))
(repair / 'target-manifest.json').write_text(json.dumps(dict(
    status='staged-only-uncompiled-unexecuted', changed_target_count=len(rows),
    prior_candidate='frozen-19-target-manifest.json', targets=rows), indent=2) + '\n')
print(f'Pinned {len(rows)} precise provenance-repair target transitions.')
