"""Narrow direct-dependency bookkeeping; never resolves or changes package versions."""
import hashlib, json
from pathlib import Path

root = Path(__file__).parent
records = []
for path in [Path('C:/github/QualiaDB/Cargo.lock'), Path('C:/github/poet/Cargo.lock')]:
    before = path.read_bytes()
    text = before.decode('utf-8')
    newline = '\r\n' if '\r\n' in text else '\n'
    marker = '[[package]]' + newline + 'name = "qualia-core-db"' + newline
    assert text.count(marker) == 1, path
    start = text.index(marker)
    end = text.find(newline + '[[package]]', start + len(marker))
    if end == -1:
        end = len(text)
    package = text[start:end]
    for dependency in ['keyring-core', 'windows-native-keyring-store']:
        assert f'name = "{dependency}"' + newline in text, (path, dependency)
        line = f' "{dependency}",' + newline
        if line not in package:
            lines = package.splitlines(keepends=True)
            dependency_lines = [i for i, value in enumerate(lines) if value.startswith(' "')]
            insertion = next((i for i in dependency_lines if lines[i] > line), dependency_lines[-1] + 1)
            lines.insert(insertion, line)
            package = ''.join(lines)
    after = (text[:start] + package + text[end:]).encode('utf-8')
    assert after[:start] == before[:start]
    path.write_bytes(after)
    records.append({'path': str(path), 'before_sha256': hashlib.sha256(before).hexdigest(),
                    'after_sha256': hashlib.sha256(after).hexdigest(),
                    'change': 'qualia-core-db dependency block only; all locked package versions retained'})
(root/'lock-receipt.json').write_text(json.dumps(records, indent=2)+'\n')
print(json.dumps(records, indent=2))
