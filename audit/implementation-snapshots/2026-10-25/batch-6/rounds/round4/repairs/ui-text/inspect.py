from pathlib import Path
import hashlib, json

here = Path(__file__).parent
round4 = here.parents[1]
rows = json.loads((round4 / 'install-manifest.json').read_text())
names = {
    'package_controls.rs', 'package_georaster_create_view.rs', 'package_georaster_edit_view.rs',
    'package_review_controls.rs', 'package_grant_controls.rs', 'ui.rs',
}
checked = []
for row in rows:
    path = Path(row['target'])
    if not path.is_relative_to(Path('C:/github/poet/native/src/graphical')) or path.name not in names:
        continue
    source = path.read_text(encoding='utf-8')
    nonascii = [dict(line=i, text=line.encode('unicode_escape').decode('ascii'))
        for i, line in enumerate(source.splitlines(), 1) if any(ord(c) > 127 for c in line)]
    checked.append(dict(target=str(path), sha256=hashlib.sha256(path.read_bytes()).hexdigest(), nonascii=nonascii))
(here / 'inspection.json').write_text(json.dumps(checked, indent=2) + '\n')
print(json.dumps(checked, indent=2))
