import hashlib,os,pathlib,shutil
stage=pathlib.Path(r'C:\github\ecopunk\tools\poet-foundation\full-scope\fork-indexed-tetra'); q=pathlib.Path(r'C:\github\qualiaDB\crates\qualia-core-db\src\specialized_libs\computational_geometry').resolve(); p=pathlib.Path(r'C:\github\poet\native\src\graphical').resolve(); backup=stage/'before'; backup.mkdir(exist_ok=True)
items=[('indexed_tetra_surface.rs',q/'indexed_tetra_surface.rs','ABSENT','3c062ebd250ceb45f3c8b76add5576e069ccc553dc626554ce0526298b9fbe12'),('core_mod.rs',q/'mod.rs','59db2cb4e0386946fdf05e724ba9686c86d5c8d5a09343a86d69ddf044ff529e','b4ab4b2cd66683d35ebfb9c717664d641e72259aaea09f5382b75cc54cf51776'),('poet_indexed_tetra_tests.rs',p/'indexed_tetra_public_tests.rs','ABSENT','3ddaacf183fbc794affc5d73e6dc04df56bd156f4baef558ceeee4763b620e24'),('poet_mod.rs',p/'mod.rs','bb9f50620a57a0e2b966c62582b08a824b5144852ecb495dc4520b7558c86bc5','7aa6d84cc794bee6b3c0021937a04406e065dad2f59b5fad0c90c502701f9ef3')]
def sha(x):return hashlib.sha256(x.read_bytes()).hexdigest()
for name,dst,pre,pin in items:
 src=stage/name
 if not any(dst.resolve().is_relative_to(r) for r in (q,p)) or dst.is_symlink() or not dst.parent.is_dir(): raise RuntimeError('unsafe target '+str(dst))
 if sha(src)!=pin: raise RuntimeError('candidate changed '+name)
 if (pre=='ABSENT' and dst.exists()) or (pre!='ABSENT' and (not dst.is_file() or sha(dst)!=pre)): raise RuntimeError('preimage changed '+str(dst))
for i,(name,dst,pre,pin) in enumerate(items):
 if dst.exists(): shutil.copyfile(dst,backup/f'{i:02}')
for name,dst,pre,pin in items:
 shutil.copyfile(stage/name,dst); os.utime(dst,None)
 if sha(dst)!=pin: raise RuntimeError('install verification failed '+str(dst))
print('Installed four reviewed indexed-extraction source/test files; preimages preserved',flush=True)
