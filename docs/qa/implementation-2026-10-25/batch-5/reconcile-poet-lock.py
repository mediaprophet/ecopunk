from pathlib import Path
import hashlib,json,subprocess,tomllib
root=Path(__file__).parent;lock=Path('C:/github/poet/Cargo.lock')
before=lock.read_bytes();(root/'poet-lock.before.toml').write_bytes(before)
with (root/'poet-lock-metadata.json').open('wb') as output,(root/'poet-lock-metadata.stderr.log').open('wb') as error:
 result=subprocess.run(['cargo','metadata','--offline','--manifest-path','C:/github/poet/Cargo.toml','--format-version','1','--filter-platform','x86_64-pc-windows-msvc','--features','poet-cli/native-graphical'],stdout=output,stderr=error,timeout=120)
assert result.returncode==0,'Offline metadata failed; inspect retained diagnostics'
old=tomllib.loads(before.decode());after=lock.read_bytes();new=tomllib.loads(after.decode())
def keys(record):return {(p['name'],p['version'],p.get('source','')):p.get('checksum') for p in record['package']}
prior=keys(old);current=keys(new)
assert all(key in current and current[key]==digest for key,digest in prior.items()),'Existing package version/source/checksum changed'
receipt=dict(exit_code=result.returncode,before_sha256=hashlib.sha256(before).hexdigest(),after_sha256=hashlib.sha256(after).hexdigest(),existing_packages_preserved=len(prior),added_packages=[dict(name=k[0],version=k[1],source=k[2]) for k in sorted(current.keys()-prior.keys())],scope='Offline optional encryption feature resolution; no existing package upgrade')
(root/'poet-lock-reconciled.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
print(json.dumps(receipt))
