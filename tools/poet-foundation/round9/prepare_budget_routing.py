from pathlib import Path
import hashlib,json
group=Path(__file__).resolve().parent/'budget-routing'
rows=[]
for name in ['native/src/graphical/controller.rs','native/src/graphical/execution.rs']:
    raw=(Path('C:/github/poet')/name).read_bytes();s=raw.decode().replace('\r\n','\n')
    a='crate::graphical::authoring::source_layers::validation_snapshot('
    assert s.count(a)==1;s=s.replace(a,a.replace('validation_snapshot','validation_snapshot_with_budget'))
    if name.endswith('controller.rs'):
        a='self.session.value("poet_parts"), self.session.value("poet_parameter_parts"))?'
        b='self.session.value("poet_parts"), self.session.value("poet_parameter_parts"),\n                self.factory_limits.retained_value_bytes.max(64 * 1024 * 1024))?'
    else:
        a='session.value("poet_parts"), session.value("poet_parameter_parts"))?'
        b='session.value("poet_parts"), session.value("poet_parameter_parts"),\n                limits.retained_value_bytes.max(64 * 1024 * 1024))?'
    assert s.count(a)==1;s=s.replace(a,b)
    before=group/'before/poet'/name;after=group/'after/poet'/name
    before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
    before.write_bytes(raw);after.write_text(s,encoding='utf-8',newline='\n')
    rows.append(dict(repository='poet',target=name,before_sha256=hashlib.sha256(raw).hexdigest(),after_sha256=hashlib.sha256(after.read_bytes()).hexdigest()))
(group/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',files=rows),indent=2)+'\n')
print(hashlib.sha256((group/'manifest.json').read_bytes()).hexdigest())
