import gzip, hashlib, pathlib
base=pathlib.Path(r'C:\github\ecopunk\target\debug\deps')
out=pathlib.Path(r'C:\github\ecopunk\docs\qa\implementation-2026-10-27\full-scope-round1')
for name in ['poet_cli-a57a4a3c176ed7d8.exe','poet_cli-c90abe74ab5c9ced.exe']:
 src=base/name; dest=out/(name+'.gz')
 if dest.exists() or not src.is_file() or src.resolve().parent != base.resolve() or src.is_symlink(): raise RuntimeError(f'unsafe path or destination exists: {name}')
 orig=hashlib.sha256()
 with src.open('rb') as fin, gzip.open(dest,'xb',compresslevel=6) as fout:
  while chunk:=fin.read(1024*1024): orig.update(chunk); fout.write(chunk)
 check=hashlib.sha256()
 with gzip.open(dest,'rb') as stream:
  while chunk:=stream.read(1024*1024): check.update(chunk)
 current=hashlib.file_digest(src.open('rb'),'sha256').hexdigest()
 if check.hexdigest()!=orig.hexdigest() or current!=orig.hexdigest(): raise RuntimeError(f'verification failed; original retained: {name}')
 size=src.stat().st_size; packed=dest.stat().st_size
 src.unlink()
 print(f'{name} sha256={current} original_bytes={size} gzip_bytes={packed}',flush=True)
