"""Parse/format checks only. Does not invoke Cargo, rustc, or any product code."""
from pathlib import Path
import hashlib,json,subprocess
HERE=Path(__file__).resolve().parent
rows=json.loads((HERE/'sdk-save-manifest.json').read_text())
results=[]
for row in rows:
    path=HERE/'after'/row['relative']
    command=['C:/Users/tim_h/.cargo/bin/rustfmt.exe','--edition','2024','--emit','stdout',
             '--config','skip_children=true',str(path)]
    proc=subprocess.run(command,capture_output=True,text=True)
    results.append(dict(relative=row['relative'],exit_code=proc.returncode,stderr=proc.stderr,
                        sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
    if proc.returncode:raise RuntimeError(proc.stderr)
new=[row for row in rows if row['before_sha256'] is None]
assert all(len((HERE/'after'/row['relative']).read_text().splitlines())<500 for row in new)
receipt=dict(scope='rustfmt parser only; no Rust typecheck, Cargo, runtime or acceptance',
             staged_files=results,new_implementation_files_below_500_lines=True,
             live_core_writes=False,checks_executed=['rustfmt --emit stdout --config skip_children=true'])
(HERE/'static-checks.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('Rustfmt parsed',len(results),'staged Rust files; new-file line limits pass; no Cargo or Core writes')
