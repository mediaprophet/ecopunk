import gzip,hashlib,pathlib
items=[(pathlib.Path(r'C:\github\ecopunk\target\debug\deps\poet_cli-c90abe74ab5c9ced.exe'),pathlib.Path(r'C:\github\ecopunk\docs\qa\implementation-2026-10-27\full-scope-round1\poet-tapered-field-test.exe.gz')),(pathlib.Path(r'C:\github\ecopunk\tools\poet-foundation\full-scope\fork-budgeted-topology\micro_harness.exe'),pathlib.Path(r'C:\github\ecopunk\tools\poet-foundation\full-scope\fork-budgeted-topology\micro_harness.exe.gz'))]
for src,dst in items:
 if dst.exists() or not src.is_file() or src.is_symlink():raise RuntimeError('unsafe path or archive exists')
 h=hashlib.sha256()
 with src.open('rb') as i,gzip.open(dst,'xb',compresslevel=6) as o:
  while c:=i.read(1024*1024):h.update(c);o.write(c)
 v=hashlib.sha256()
 with gzip.open(dst,'rb') as i:
  while c:=i.read(1024*1024):v.update(c)
 with src.open('rb') as i:a=hashlib.file_digest(i,'sha256').hexdigest()
 if h.hexdigest()!=v.hexdigest() or a!=h.hexdigest():raise RuntimeError('digest mismatch')
 size=src.stat().st_size;z=dst.stat().st_size;src.unlink();print(f'{src.name} sha256={a} bytes={size} gzip_bytes={z}',flush=True)
