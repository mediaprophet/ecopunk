from pathlib import Path
import json,hashlib,subprocess,os
root=Path(__file__).parent
batch=root.parent
rows=[]
for line in (batch/'native-build/library-final.log').read_text(encoding='utf-8-sig').splitlines():
    try:rows.append(json.loads(line))
    except json.JSONDecodeError:pass
def artifact(target,owner):
    found=[]
    for r in rows:
        if r.get('reason')=='compiler-artifact' and r.get('target',{}).get('name')==target and r.get('manifest_path','').replace('\\','/').lower()==owner.lower():
            found.extend(p for p in r['filenames'] if p.endswith('.rlib'))
    found=set(found)
    assert len(found)==1,(target,found)
    return found.pop()
core=artifact('qualia_core_db','C:/github/qualiaDB/crates/qualia-core-db/Cargo.toml')
vibe=artifact('vibe','C:/github/qualiaDB/crates/vibe/Cargo.toml')
native=sorted(set(p for r in rows if r.get('reason')=='build-script-executed' for p in r.get('linked_paths',[])))
binary=root/'focused.exe'
args=['rustc','--edition=2021','--test',str(root/'check.rs'),'-C','debuginfo=0','--extern','qualia_core_db='+core,'--extern','vibe='+vibe,'-L','dependency=C:/github/ecopunk/target/debug/deps','-o',str(binary)]
for path in native:args+=['-L',path]
p=subprocess.run(args,capture_output=True,creationflags=subprocess.CREATE_NO_WINDOW)
(root/'compile.log').write_bytes(p.stdout+p.stderr)
assert p.returncode==0,(p.returncode,(p.stdout+p.stderr).decode(errors='replace')[-4000:])
p=subprocess.run([str(binary),'--nocapture','--test-threads=1'],capture_output=True,creationflags=subprocess.CREATE_NO_WINDOW)
(root/'focused.log').write_bytes(p.stdout+p.stderr)
print((p.stdout+p.stderr).decode(errors='replace'))
(root/'verification.json').write_text(json.dumps(dict(scope='Exact candidate cooperative emitters and disabled seam; cached ray generator with sole exact directive delta; no rebuilt whole-core claim',exit_code=p.returncode,compiler_arguments=args,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),core_rlib_sha256=hashlib.sha256(Path(core).read_bytes()).hexdigest()),indent=2))
raise SystemExit(p.returncode)
