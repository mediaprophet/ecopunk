from pathlib import Path
import shutil,hashlib,json
base=Path(__file__).parent
dest=base/'verification/poet/native'; dest.mkdir(parents=True,exist_ok=True)
records=[]
for name in ['main.rs','repl.rs','format.rs','lint.rs']:
    original=Path('C:/github/poet/native/src')/name
    candidate=base/'candidate/poet/native/src'/name
    source=candidate if candidate.exists() else original
    out=dest/'src'/name;out.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,out)
    records.append(dict(source=str(source.resolve()),sha256=hashlib.sha256(source.read_bytes()).hexdigest()))
(dest/'Cargo.toml').write_text('''[package]
name = "poet-run05-native-verification"
version = "0.0.0"
edition = "2021"
publish = false
[workspace]
[dependencies]
vibe = { path = "../../qualiaDB/crates/vibe" }
[[bin]]
name = "poet"
path = "src/main.rs"
''',encoding='utf8')
(base/'poet-verification-inputs.json').write_text(json.dumps(records,indent=2),encoding='utf8')
print('private native Poet verification manifest; exact main/repl candidates and original format/lint')
