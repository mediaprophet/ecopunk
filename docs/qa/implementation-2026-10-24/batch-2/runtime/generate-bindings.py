from pathlib import Path
import re
base=Path(__file__).parent
source=Path('C:/github/qualiaDB/crates/vibe/src/bind')
paths=set(['quin.statement'])
for name in ['math.rs','rdf.rs','dispatch.rs']:
    paths.update(re.findall(r'^\s*"([a-z][a-z0-9_.]*)"\s*=>', (source/name).read_text(encoding='utf8'),re.M))
paths={p for p in paths if '.' in p or p=='receipt_empty'}
out='//! Pinned 0.1 dispatch-path registry; regenerate/review with bind dispatch changes.\n'
out+='pub(super) const BINDING_PATHS: &[&str] = &[\n'
out+=''.join(f'    "{p}",\n' for p in sorted(paths))+'];\n'
(base/'candidate/vibe/src/check/bindings.rs').write_text(out,encoding='utf8')
print(len(paths))
