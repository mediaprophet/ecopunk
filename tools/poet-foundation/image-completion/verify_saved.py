"""Verify current reviewed image-completion sources, without building or editing them.

This verifies file integrity, not human acceptance or runtime correctness.
"""
from pathlib import Path
import hashlib
import json

ROOT = Path('C:/github/ecopunk')
STAGE = ROOT / 'tools/poet-foundation/image-completion/work/integration'
QA = ROOT / 'docs/qa/implementation-2026-10-26/image-completion'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    results = []
    for name in ('manifest-core.json', 'manifest-poet.json'):
        manifest = json.loads((STAGE / name).read_text(encoding='utf-8'))
        for row in manifest['files']:
            dst = Path(row['destination'])
            src = Path(row['staged'])
            actual = sha(dst) if dst.is_file() else None
            assert sha(src) == row['after'], f'Staged hash mismatch: {src}'
            assert actual == row['after'], f'Canonical hash mismatch: {dst}'
            results.append({'path': str(dst), 'sha256': actual})
    assert len({row['path'] for row in results}) == len(results), 'Duplicate destination'
    report = {'scope': 'Reviewed saved-source integrity only; runtime checks have separate logs',
              'asset_production': 'HOLD', 'native_window_human_acceptance': 'UNASSESSED',
              'files': results}
    (QA / 'saved-source-integrity.json').write_text(
        json.dumps(report, indent=2) + '\n', encoding='utf-8', newline='\n')
    print(f'Verified {len(results)} saved canonical files against reviewed candidates')


if __name__ == '__main__':
    main()
