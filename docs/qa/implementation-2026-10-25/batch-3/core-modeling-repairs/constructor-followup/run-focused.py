from pathlib import Path
import subprocess,json,hashlib,re
r=Path(__file__).resolve().parent
batch=r.parent.parent
rows=[]
for line in (batch/'native-build/library-final.log').read_text(encoding='utf-8-sig').splitlines():
 try: rows.append(json.loads(line))
 except json.JSONDecodeError: pass
def lib(name):
 found={p for x in rows if x.get('reason')=='compiler-artifact' and x.get('target',{}).get('name')==name and set(x.get('target',{}).get('kind',[])).intersection(['lib','rlib']) for p in x.get('filenames',[]) if p.endswith('.rlib')}
 assert len(found)==1,(name,found)
 return found.pop()
harness='''use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use qualia_core_db::specialized_libs::chemistry_modeling::{ChemistryError, PhaseType, SimulationMetrics, QuantumMetrics, ReactionMetrics, PropertyMetrics};
#[path = "candidate/types.rs"] mod types;
use types::*;
#[path = "candidate/properties.rs"] mod properties;
use properties::*;
'''
# Include the exact staged explicit-carbon helper without substituting cached Molecule.
s=(r/'candidate/tests.rs').read_text(); start=s.index('fn carbon_molecule()'); end=s.index('\n}\n',start)+3
harness+=s[start:end]+'''
#[test]
fn explicit_carbon_fixture_has_consistent_structure() {
    let carbon = carbon_molecule();
    assert_eq!(carbon.formula, "C");
    assert_eq!(carbon.atoms.len(), 1);
    assert_eq!(carbon.atoms[0].element, "C");
    assert_eq!(carbon.atoms[0].atomic_number, 6);
    assert_eq!(carbon.coordinates, vec![carbon.atoms[0].coordinates.clone()]);
    assert!(carbon.bonds.is_empty());
}
'''
(r/'focused.rs').write_text(harness)
args=['rustc','--test','--edition=2021',str(r/'focused.rs'),'--extern','qualia_core_db='+lib('qualia_core_db'),'--extern','serde='+lib('serde'),'-L','dependency=C:/github/ecopunk/target/debug/deps','-o',str(r/'focused.exe')]
for p in sorted({p for row in rows if row.get('reason')=='build-script-executed' for p in row.get('linked_paths',[])}):args+=['-L',p]
res=subprocess.run(args,capture_output=True,creationflags=subprocess.CREATE_NO_WINDOW)
(r/'compile.log').write_bytes(res.stdout+res.stderr)
if res.returncode: print((res.stdout+res.stderr).decode(errors='replace')[-6000:]);raise SystemExit(res.returncode)
res=subprocess.run([str(r/'focused.exe'),'--nocapture'],capture_output=True,creationflags=subprocess.CREATE_NO_WINDOW)
(r/'focused.log').write_bytes(res.stdout+res.stderr)
(r/'verification.json').write_text(json.dumps(dict(scope='Exact candidate production types.rs and properties.rs included as source; cached Core provides only unchanged supporting enum/error/metrics types; no whole-Core rebuild or cached constructor substitution.',exit_code=res.returncode,compiler_arguments=args,binary_sha256=hashlib.sha256((r/'focused.exe').read_bytes()).hexdigest(),candidate_pins={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (r/'candidate').glob('*.rs')}),indent=2)+'\n')
print((res.stdout+res.stderr).decode(errors='replace'));raise SystemExit(res.returncode)

