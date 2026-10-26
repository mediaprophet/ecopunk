"""Freeze reviewed staged bytes; this does not regenerate application source."""
from pathlib import Path
import hashlib,json,subprocess
HERE=Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
root_manifest=HERE/'integration/target-manifest.json';root=json.loads(root_manifest.read_text())
for row in root:
    p=Path(row['stage'])
    if p.suffix=='.rs':
        data=p.read_bytes().replace(b'\r\r\n',b'\n').replace(b'\r\n',b'\n');assert b'\0' not in data,p;p.write_bytes(data)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*[r['stage'] for r in root if r['target'].endswith('.rs')]],check=True)
for row in root:row['after_sha256']=sha(row['stage'])
root_manifest.write_text(json.dumps(root,indent=2),encoding='utf-8')
surface_rows=json.loads((HERE/'surfaces/manifest.json').read_text())
if isinstance(surface_rows,dict):surface_rows=surface_rows['files']
groups=[('profiles',json.loads((HERE/'profiles/target-manifest.json').read_text())['targets']),
        ('surfaces',surface_rows),('integration',root)]
rows=[];seen=set()
for job,group in groups:
    for row in group:
        target=Path(row['target']);stage=Path(row['stage']);key=str(target.resolve()).lower()
        assert key not in seen,key;seen.add(key)
        assert sha(stage)==row['after_sha256'],stage
        assert (sha(target) if target.exists() else None)==row['before_sha256'],target
        rows.append(dict(job=job,target=str(target),stage=str(stage.resolve()),before_sha256=row['before_sha256'],after_sha256=row['after_sha256']))
manifest=HERE/'install-manifest.json';manifest.write_text(json.dumps(rows,indent=2),encoding='utf-8')
print(json.dumps(dict(targets=len(rows),by_job={name:len(group) for name,group in groups},manifest_sha256=sha(manifest))))
