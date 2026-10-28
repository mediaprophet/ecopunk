import gzip,hashlib,pathlib
src=pathlib.Path(r'C:\github\ecopunk\target\debug\deps\qualia_mcp-4f3870c83ac40c2a.exe'); dst=pathlib.Path(r'C:\github\ecopunk\docs\qa\implementation-2026-10-27\full-scope-round1\previous-qualia-mcp-test.exe.gz'); root=pathlib.Path(r'C:\github\ecopunk\target\debug\deps').resolve()
if dst.exists() or not src.is_file() or src.is_symlink() or src.resolve().parent!=root: raise RuntimeError('unsafe path or existing destination')
h=hashlib.sha256()
with src.open('rb') as i,gzip.open(dst,'xb',compresslevel=6) as o:
 while c:=i.read(1024*1024): h.update(c); o.write(c)
v=hashlib.sha256()
with gzip.open(dst,'rb') as i:
 while c:=i.read(1024*1024): v.update(c)
with src.open('rb') as i: actual=hashlib.file_digest(i,'sha256').hexdigest()
if h.hexdigest()!=v.hexdigest() or actual!=h.hexdigest(): raise RuntimeError('digest mismatch; source retained')
size=src.stat().st_size; packed=dst.stat().st_size; src.unlink(); print(f'preserved sha256={actual} original_bytes={size} gzip_bytes={packed}',flush=True)
