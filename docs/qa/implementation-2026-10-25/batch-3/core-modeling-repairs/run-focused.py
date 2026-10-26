from pathlib import Path
import json,re,subprocess,hashlib,textwrap
root=Path(__file__).parent
batch=root.parent
def function(source,name):
    match=re.search(r'(?m)^\s*fn '+re.escape(name)+r'\b[^\{]*\{',source)
    assert match,name
    depth=1;end=match.end()
    while depth:
        char=source[end];end+=1
        depth+=(char=='{')-(char=='}')
    return textwrap.dedent(source[match.start():end]).strip()
pieces=[]
cases=[('chemistry','chemistry_modeling/properties.rs','chemistry_modeling',[
    ('water_molecule',False),('initialized_predictor_uses_registered_group_contribution_formulas',True)]),
    ('finance','financial_modeling/tests.rs','financial_modeling',[
    ('ingest_from_feed_fails_closed_without_cached_prices',True)]),
    ('engineering','engineering_analysis/tests.rs','engineering_analysis',[
    ('prismatic_model',False),('si_prismatic_model',False),('test_mechanical_analysis',True),
    ('structural_vibration_facade_matches_prismatic_closed_form_and_rejects_invalid_density',True)])]
for module,file,owner,names in cases:
    source=(root/'candidate'/file).read_text(encoding='utf-8')
    imports=f'use qualia_core_db::specialized_libs::{owner}::*;\nuse std::collections::HashMap;\n'
    body='\n'.join(('#[test]\n' if test else '')+function(source,name) for name,test in names)
    pieces.append(f'mod {module} {{\n{imports}{body}\n}}')
(root/'focused.rs').write_text('\n'.join(pieces),encoding='utf-8')
rows=[]
for line in (batch/'native-build/library-final.log').read_text(encoding='utf-8-sig').splitlines():
    try:rows.append(json.loads(line))
    except json.JSONDecodeError:pass
libs=set(p for row in rows if row.get('reason')=='compiler-artifact' and row.get('target',{}).get('name')=='qualia_core_db' for p in row['filenames'] if p.endswith('.rlib'))
assert len(libs)==1
core=libs.pop()
args=['rustc','--test','--edition=2021',str(root/'focused.rs'),'--extern','qualia_core_db='+core,'-L','dependency=C:/github/ecopunk/target/debug/deps','-o',str(root/'focused.exe')]
for path in sorted(set(p for row in rows if row.get('reason')=='build-script-executed' for p in row.get('linked_paths',[]))):args+=['-L',path]
result=subprocess.run(args,capture_output=True,creationflags=subprocess.CREATE_NO_WINDOW)
(root/'compile.log').write_bytes(result.stdout+result.stderr)
assert result.returncode==0,(result.stdout+result.stderr).decode(errors='replace')[-3000:]
result=subprocess.run([str(root/'focused.exe'),'--nocapture','--test-threads=1'],capture_output=True,creationflags=subprocess.CREATE_NO_WINDOW)
output=result.stdout+result.stderr
(root/'focused.log').write_bytes(output)
(root/'verification.json').write_text(json.dumps(dict(scope='Exact four candidate tests and helpers against the actual cached production modeling implementation',exit_code=result.returncode,compiler_arguments=args,binary_sha256=hashlib.sha256((root/'focused.exe').read_bytes()).hexdigest()),indent=2))
print(output.decode(errors='replace'))
raise SystemExit(result.returncode)
