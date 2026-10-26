from pathlib import Path
import re,json,hashlib
base=Path(__file__).parent
log=base/'verification-full-final.log'
text=log.read_text(encoding='utf-8-sig')
target=None; rows=[]
for line in text.splitlines():
    if 'Running unittests' in line: target='vibe library' if 'src\\lib.rs' in line else 'vibe binary'
    elif 'Running tests' in line: target=re.search(r'Running tests[\\/]([^ ]+)',line).group(1)
    elif 'Doc-tests' in line: target='doc-tests'
    match=re.search(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;',line)
    if match:
        rows.append(dict(target=target,status=match[1],passed=int(match[2]),failed=int(match[3]),ignored=int(match[4])))
assert rows and all(r['status']=='ok' and r['failed']==0 for r in rows)
receipt=dict(status='private exact-source snapshot passed; canonical siblings not yet applied',
    command='cargo test --offline --manifest-path verification/qualiaDB/crates/vibe/Cargo.toml',
    exit_code=0,log_sha256=hashlib.sha256(log.read_bytes()).hexdigest(),
    targets=rows,total_passed=sum(r['passed'] for r in rows),total_ignored=sum(r['ignored'] for r in rows),
    boundary_isolation='five separate default-stack test binary processes; no raised stack',
    limitations=['private synthetic manifest and exact overlays, not canonical applied-source run',
        'ignored boundary cases executed separately with named logs',
        'no native graphical authoring/cooperation or model acceptance claim'])
(base/'verification-receipt.json').write_text(json.dumps(receipt,indent=2),encoding='utf8')
print(json.dumps(dict(total_passed=receipt['total_passed'],total_ignored=receipt['total_ignored'],targets=rows),indent=2))
