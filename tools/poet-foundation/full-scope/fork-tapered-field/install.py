import hashlib,os,pathlib,shutil
stage=pathlib.Path(r'C:\github\ecopunk\tools\poet-foundation\full-scope\fork-tapered-field'); q=pathlib.Path(r'C:\github\qualiaDB\crates\qualia-core-db\src\specialized_libs\computational_geometry').resolve(); p=pathlib.Path(r'C:\github\poet\native\src\graphical').resolve(); backup=stage/'before'; backup.mkdir(exist_ok=True)
items=[('sdf_ops.rs',q/'sdf_ops.rs','8878a47afb5ba7bf392c9addb3be62378ca5cab8f1409a6ba3264a783eb08877','ff4c6bfcb9f0601bb0452ee3c6d2b6e38973a211e5036254c47b7c2d5cba512b'),('tapered_branch_field.rs',q/'tapered_branch_field.rs','ABSENT','5a78f146f37ddb5f001dc089cee0fda545d39cc777e7e4f9f404e174eadc4058'),('core_mod.rs',q/'mod.rs','451dfbb940b6382564c16f87f22c0ae784d60f0f79d10fb6d6a4cb5a94de9a6d','712745423038c0f1cea9f1ec06874a5d69efec5e41c3246b9ec61361579a05c3'),('poet_tapered_field_tests.rs',p/'tapered_branch_field_public_tests.rs','ABSENT','520003e9d4268232113dbf1dfc5abf6bdcea191b808d59d52deb096b8e5d90db'),('poet_mod.rs',p/'mod.rs','1e514037d05d804444eb200d8aedbc9da923b1ede031e6bd364f5bf75fcefa1c','3948528b12d63947ddb1e000f18ef8f04d5859b04b59e9ecc612ccce52414418')]
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
print('Installed five reviewed tapered-field source/test files; prior sources backed up',flush=True)
