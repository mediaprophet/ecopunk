from pathlib import Path
import json,hashlib
base=Path(__file__).parent
target=Path('C:/github/qualiaDB/crates/vibe/src/eval/expr.rs')
data=target.read_bytes();original=base/'original/vibe/src/eval/expr.rs'
original.parent.mkdir(parents=True,exist_ok=True)
if original.exists(): assert original.read_bytes()==data
else: original.write_bytes(data)
guards=json.loads((base/'source-guards.json').read_text())
if not any(Path(e['target'])==target for e in guards):
    guards.append(dict(target=str(target),original=str(original),sha256=hashlib.sha256(data).hexdigest()))
text=data.decode('utf8').replace('\r\n','\n')
assert text.count('pub fn eval_expr(')==1
text=text.replace('pub fn eval_expr(', 'pub(super) fn eval_expr_inner(',1)
out=base/'candidate/vibe/src/eval/expr.rs';out.write_text(text,encoding='utf8',newline='\n')
(base/'source-guards.json').write_text(json.dumps(guards,indent=2),encoding='utf8')
print('existing evaluator unchanged except guarded inner method name')
