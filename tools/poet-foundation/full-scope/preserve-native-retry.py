import gzip,hashlib,pathlib
src=pathlib.Path(r'C:\github\ecopunk\target\debug\deps\poet_cli-c90abe74ab5c9ced.exe')
dest=pathlib.Path(r'C:\github\ecopunk\docs\qa\implementation-2026-10-27\full-scope-round1\poet-native-suite-retry.exe.gz')
if dest.exists() or not src.is_file() or src.is_symlink() or src.resolve().parent != pathlib.Path(r'C:\github\ecopunk\target\debug\deps').resolve(): raise RuntimeError('unsafe path or existing destination')
h=hashlib.sha256()
with src.open('rb') as i, gzip.open(dest,'xb',compresslevel=6) as o:
 while chunk:=i.read(1024*1024): h.update(chunk); o.write(chunk)
v=hashlib.sha256()
with gzip.open(dest,'rb') as i:
 while chunk:=i.read(1024*1024): v.update(chunk)
with src.open('rb') as i: current=hashlib.file_digest(i,'sha256').hexdigest()
if h.hexdigest()!=v.hexdigest() or current!=h.hexdigest(): raise RuntimeError('digest mismatch; original retained')
size=src.stat().st_size; packed=dest.stat().st_size; src.unlink()
print(f'preserved sha256={current} original_bytes={size} gzip_bytes={packed}',flush=True)
