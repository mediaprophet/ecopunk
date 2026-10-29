import hashlib,os,pathlib,shutil
stage=pathlib.Path(r'C:\github\ecopunk\tools\poet-foundation\full-scope\fork-budgeted-topology'); q=pathlib.Path(r'C:\github\qualiaDB\crates\qualia-core-db\src\specialized_libs\computational_geometry').resolve(); p=pathlib.Path(r'C:\github\poet\native\src\graphical').resolve(); backup=stage/'before'; backup.mkdir(exist_ok=True)
items=[('topology.rs',q/'topology.rs','c0f566cee0f9e89cb3c6b986ea82359199e58f8c6840507a8ed51fbc8f482249','a6439f95c7fe3916469c8734350e43f0050174670ce37d3632f8d1600de38783'),('surface_topology_qualification.rs',q/'surface_topology_qualification.rs','c5a1810e8bb2940351e7cba5247a061e8a34d7c98aebd8ba94b9199bf8650604','3ca4a2338a7ec23ad872d68ec728409878c94a546379fce2ff9e57cdd4de80d6'),('core_mod.rs',q/'mod.rs','712745423038c0f1cea9f1ec06874a5d69efec5e41c3246b9ec61361579a05c3','8c6f1ac527c3ee144c17e7509e3e73ac6b2a2257ce1ee53d96fd998e93f484e2'),('poet_surface_topology_public_tests.rs',p/'surface_topology_public_tests.rs','ac1f4a16bf4c160d2c202a0ec4352f074e709f94e7ebecbcb7238083e4ada5ee','4a5a76e32e96ce114d03fa2d248d6a6135faf367f89b364dcd8694a6c409a74d')]
def sha(x):return hashlib.sha256(x.read_bytes()).hexdigest()
for name,dst,pre,pin in items:
 src=stage/name
 if not any(dst.resolve().is_relative_to(r) for r in (q,p)) or dst.is_symlink() or not dst.parent.is_dir():raise RuntimeError('unsafe target '+str(dst))
 if sha(src)!=pin:raise RuntimeError('candidate changed '+name+' observed '+sha(src))
 if not dst.is_file() or sha(dst)!=pre:raise RuntimeError('preimage changed '+str(dst))
for i,(name,dst,pre,pin) in enumerate(items):shutil.copyfile(dst,backup/f'{i:02}')
for name,dst,pre,pin in items:
 shutil.copyfile(stage/name,dst);os.utime(dst,None)
 if sha(dst)!=pin:raise RuntimeError('verification failed '+str(dst))
print('Installed four reviewed budgeted topology files; prior sources backed up',flush=True)
