from pathlib import Path
import json,hashlib
group=Path(__file__).resolve().parent/'lookdev-integration';rows=[]
for name in ['native/src/graphical/mod.rs','native/src/graphical/viewport_contract.rs','native/src/graphical/reference_authoring_tests.rs']:
    raw=(Path('C:/github/poet')/name).read_bytes();s=raw.decode().replace('\r\n','\n')
    if name.endswith('/mod.rs'):s+='\nmod lookdev_controls;\n'
    elif name.endswith('viewport_contract.rs'):
        assert 'key.len()<=128' in s;s=s.replace('key.len()<=128','key.len()<=4096')
    else:
        a='''        let framed = h
            .viewport_action(ViewportCommand::FrameAll, source.clone(), &cancel)'''
        b='''        h.viewport_action(super::viewport::preview_lighting(super::viewport::ViewLighting::neutral_studio()),source.clone(),&cancel).unwrap();
        let framed = h
            .viewport_action(ViewportCommand::FrameAll, source.clone(), &cancel)'''
        assert s.count(a)==1;s=s.replace(a,b)
        a='''        let frame = h
            .viewport_action(ViewportCommand::FrameAll, source, &cancel)'''
        b='''        h.viewport_action(super::viewport::preview_lighting(super::viewport::ViewLighting::neutral_studio()),source.clone(),&cancel).unwrap();
        let frame = h
            .viewport_action(ViewportCommand::FrameAll, source, &cancel)'''
        assert s.count(a)==1;s=s.replace(a,b)
    before=group/'before/poet'/name;after=group/'after/poet'/name
    before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
    before.write_bytes(raw);after.write_text(s,encoding='utf-8',newline='\n')
    rows.append(dict(repository='poet',target=name,before_sha256=hashlib.sha256(raw).hexdigest(),after_sha256=hashlib.sha256(after.read_bytes()).hexdigest()))
(group/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',files=rows),indent=2)+'\n')
print(hashlib.sha256((group/'manifest.json').read_bytes()).hexdigest())
