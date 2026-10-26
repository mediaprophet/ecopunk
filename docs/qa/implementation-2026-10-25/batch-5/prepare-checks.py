from pathlib import Path
root=Path(__file__).parent;prior=root.parent/'batch-4'
freeze=(prior/'freeze.py').read_text(encoding='utf-8-sig').replace("root.parent/'batch-3/latest-source-snapshot.json'","root.parent/'batch-4/latest-source-snapshot.json'")
freeze=freeze.replace("'C:/github/poet/native/src',","'C:/github/poet/native/src','C:/github/qualiaDB/crates/qualia-core-db/src/identity',")
freeze=freeze.replace("paths.add(Path(p).resolve())","paths.add(Path(p).resolve())\npaths.add(Path('C:/github/qualiaDB/crates/qualia-core-db/Cargo.toml').resolve())\npaths.update(p.resolve() for p in Path('C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke').rglob('studio_*.rs'))")
freeze=freeze.replace('Batch4 changed-source','Batch5 changed-source')
(root/'freeze.py').write_text(freeze,encoding='utf-8')
(root/'run-check.ps1').write_text((prior/'run-check.ps1').read_text(encoding='utf-8-sig'),encoding='utf-8')
print('Prepared source-bound RUN10 checks; none executed by this helper.')
