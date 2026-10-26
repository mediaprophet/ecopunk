"""Reconcile exact staged owners and format root routing before pinned install."""
from pathlib import Path
import hashlib,json,subprocess
HERE=Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
root=json.loads((HERE/'integration/target-manifest.json').read_text())
rust=[r['stage'] for r in root if r['target'].endswith('.rs')]
for name in rust:
    p=Path(name);p.write_bytes(p.read_bytes().replace(b'\r\r\n',b'\n').replace(b'\r\n',b'\n'))
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*rust],check=True)
for row in root:row['after_sha256']=sha(row['stage'])
(HERE/'integration/target-manifest.json').write_text(json.dumps(root,indent=2),encoding='utf-8')
groups=[('authoring',json.loads((HERE/'authoring/target-manifest.json').read_text())['targets']),
        ('typed',json.loads((HERE/'typed/manifest.json').read_text())['files']),('integration',root)]
rows=[];seen=set()
for owner,group in groups:
    for r in group:
        target=Path(r['target']);stage=Path(r['stage']);key=str(target.resolve()).lower()
        assert key not in seen,key;seen.add(key)
        assert sha(stage)==r['after_sha256'],stage
        assert (sha(target) if target.exists() else None)==r['before_sha256'],target
        rows.append(dict(job=owner,target=str(target),stage=str(stage.resolve()),before_sha256=r['before_sha256'],after_sha256=r['after_sha256']))
manifest=HERE/'install-manifest.json';manifest.write_text(json.dumps(rows,indent=2),encoding='utf-8')
print(json.dumps(dict(targets=len(rows),by_job={n:len(g) for n,g in groups},manifest_sha256=sha(manifest))))
