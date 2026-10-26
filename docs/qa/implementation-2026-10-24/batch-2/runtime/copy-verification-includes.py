from pathlib import Path
import re,shutil,hashlib,json
base=Path(__file__).parent
source=Path('C:/github/qualiaDB/crates/vibe')
reference_root=Path('C:/github')
entries=[]
for folder in ['src','tests']:
    for src in (source/folder).rglob('*.rs'):
        for relative in re.findall(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)',src.read_text(encoding='utf8')):
            target=(src.parent/relative).resolve()
            if not target.is_file(): raise RuntimeError(f'missing original include {target}')
            assert target.stat().st_size<=2*1024*1024, 'oversized reference: review required'
            dest=base/'verification'/target.relative_to(reference_root)
            dest.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(target,dest)
            entries.append(dict(source=str(target),destination=str(dest.resolve()),sha256=hashlib.sha256(target.read_bytes()).hexdigest()))
(base/'verification-includes.json').write_text(json.dumps(entries,indent=2),encoding='utf8')
print(f'copied {len(entries)} exact referenced include files')
