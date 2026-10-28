import hashlib,os,pathlib,shutil
stage=pathlib.Path(r'C:\github\ecopunk\tools\poet-foundation\full-scope\fork-linear-connectivity'); q=pathlib.Path(r'C:\github\qualiaDB\crates\qualia-core-db\src\specialized_libs\computational_geometry').resolve(); p=pathlib.Path(r'C:\github\poet\native\src\graphical').resolve(); backup=stage/'before'; backup.mkdir(exist_ok=True)
items=[('connectivity.rs',q/'connectivity.rs','dd0681ae646b1bea1242b9ff45f16d1fba789781b86688e3d9809ad7fee3bf5b','8459ff21b97fb13a39f346c9637fe13a758c475939c9b46b6654b31fb47e5469'),('surface_topology_qualification.rs',q/'surface_topology_qualification.rs','ABSENT','c5a1810e8bb2940351e7cba5247a061e8a34d7c98aebd8ba94b9199bf8650604'),('core_mod.rs',q/'mod.rs','b4ab4b2cd66683d35ebfb9c717664d641e72259aaea09f5382b75cc54cf51776','451dfbb940b6382564c16f87f22c0ae784d60f0f79d10fb6d6a4cb5a94de9a6d' if False else '451dfbb940b6382564c16f87f22c0ae784d60f0f79d10fb6d6a4cb5a94de9a6d'),('poet_surface_topology_tests.rs',p/'surface_topology_public_tests.rs','ABSENT','ac1f4a16bf4c160d2c202a0ec4352f074e709f94e7ebecbcb7238083e4ada5ee'),('poet_mod.rs',p/'mod.rs','7aa6d84cc794bee6b3c0021937a04406e065dad2f59b5fad0c90c502701f9ef3','1e514037d05d804444eb200d8aedbc9da923b1ede031e6bd364f5bf75fcefa1c')]
# Exact pin from the reviewed handoff.
items[2]=(items[2][0],items[2][1],items[2][2],'451dfbb940b6382564c16f87f22c0ae784d60f0f79d10fb6d6a4cb5a94de9a6d')
def sha(x):return hashlib.sha256(x.read_bytes()).hexdigest()
for name,dst,pre,pin in items:
 src=stage/name
 if not any(dst.resolve().is_relative_to(r) for r in (q,p)) or dst.is_symlink() or not dst.parent.is_dir():raise RuntimeError('unsafe target '+str(dst))
 if sha(src)!=pin:raise RuntimeError('candidate changed '+name+' observed '+sha(src))
 if (pre=='ABSENT' and dst.exists()) or (pre!='ABSENT' and (not dst.is_file() or sha(dst)!=pre)):raise RuntimeError('preimage changed '+str(dst))
for i,(name,dst,pre,pin) in enumerate(items):
 if dst.exists():shutil.copyfile(dst,backup/f'{i:02}')
for name,dst,pre,pin in items:
 shutil.copyfile(stage/name,dst);os.utime(dst,None)
 if sha(dst)!=pin:raise RuntimeError('verification failed '+str(dst))
print('Installed five reviewed topology/connectivity files; prior sources backed up',flush=True)
