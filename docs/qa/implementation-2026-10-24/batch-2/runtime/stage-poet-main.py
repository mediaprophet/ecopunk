from pathlib import Path
import hashlib,json
base=Path(__file__).parent
target=Path('C:/github/poet/native/src/main.rs');data=target.read_bytes()
original=base/'original/poet/native/src/main.rs';original.parent.mkdir(parents=True,exist_ok=True)
if original.exists(): assert original.read_bytes()==data
else: original.write_bytes(data)
text=data.decode('utf8').replace('\r\n','\n')
text=text.replace('poet repl           Start an interactive VibeScript REPL','poet repl [--grant <capability>]...  Interactive REPL; default grants: none')
text=text.replace('            if let Err(e) = repl::run_repl() {','''            let grants = match repl::parse_repl_grants(&args[2..]) {
                Ok(grants) => grants,
                Err(error) => { eprintln!("REPL option error: {error}"); process::exit(1); }
            };
            if let Err(e) = repl::run_repl_with_grants(grants) {''',1)
out=base/'candidate/poet/native/src/main.rs';out.write_text(text,encoding='utf8',newline='\n')
mapping=json.loads((base/'poet-patch-map.json').read_text())
mapping=[entry for entry in mapping if Path(entry['target'])!=target]
mapping.append(dict(target=str(target),original=str(original.resolve()),original_sha256=hashlib.sha256(data).hexdigest(),candidate=str(out.resolve()),candidate_sha256=hashlib.sha256(out.read_bytes()).hexdigest()))
for entry in mapping:
    entry['candidate_sha256']=hashlib.sha256(Path(entry['candidate']).read_bytes()).hexdigest()
(base/'poet-patch-map.json').write_text(json.dumps(mapping,indent=2),encoding='utf8')
print('explicit host-level repeatable --grant option, default none')
