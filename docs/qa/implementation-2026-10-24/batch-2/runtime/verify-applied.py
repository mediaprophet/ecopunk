from pathlib import Path
import hashlib,json,datetime
base=Path(__file__).parent
def sha(file): return hashlib.sha256(Path(file).read_bytes()).hexdigest()
rows=[]
for name in ['patch-map.json','poet-patch-map.json']:
    for row in json.loads((base/name).read_text()):
        actual=sha(row['target'])
        assert actual==row['candidate_sha256'], f'applied source mismatch {row["target"]}'
        rows.append(dict(target=row['target'],sha256=actual,expected_sha256=row['candidate_sha256']))
inputs={str(path):sha(path) for path in [
    Path('C:/github/qualiaDB/crates/vibe/Cargo.toml'),
    Path('C:/github/qualiaDB/crates/vibe/src/parse/mod.rs'),
    Path('C:/github/poet/Cargo.toml'),Path('C:/github/poet/native/Cargo.toml'),
    Path('C:/github/poet/Cargo.lock'),
    base.parent/'parser/harness/Cargo.toml',base.parent/'parser/harness/Cargo.lock'] if path.is_file()}
record=dict(status='all actual applied source bytes equal frozen candidates',
    verified_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    maps={name:sha(base/name) for name in ['patch-map.json','poet-patch-map.json']},
    inputs=inputs,targets=rows)
(base/'actual-source-verification.json').write_text(json.dumps(record,indent=2),encoding='utf8')
print(f'{len(rows)} actual source targets match frozen candidate digests')
