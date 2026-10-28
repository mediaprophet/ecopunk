import csv,hashlib,os,pathlib,shutil
stage=pathlib.Path(r'C:\github\ecopunk\tools\poet-foundation\full-scope\botanical-continuity-integrated'); root=pathlib.Path(r'C:\github\poet\native\src\graphical').resolve(); backup=stage/'before'; backup.mkdir(exist_ok=True)
with (stage/'manifest.tsv').open(encoding='utf8',newline='') as f: rows=list(csv.DictReader(f,delimiter='\t'))
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
for row in rows:
 src=pathlib.Path(row['staged']); dst=pathlib.Path(row['target']); pre=row['preimage_sha256']
 if not dst.resolve().is_relative_to(root) or dst.is_symlink() or not dst.parent.is_dir(): raise RuntimeError('unsafe target: '+str(dst))
 if sha(src)!=row['candidate_sha256']: raise RuntimeError('candidate changed: '+str(src))
 if (pre=='ABSENT' and dst.exists()) or (pre!='ABSENT' and (not dst.is_file() or sha(dst)!=pre)): raise RuntimeError('preimage changed: '+str(dst))
for i,row in enumerate(rows):
 dst=pathlib.Path(row['target'])
 if dst.exists(): shutil.copyfile(dst,backup/f'{i:02}')
for row in rows:
 dst=pathlib.Path(row['target']); shutil.copyfile(row['staged'],dst); os.utime(dst,None)
 if sha(dst)!=row['candidate_sha256']: raise RuntimeError('verification failed: '+str(dst))
print(f'Installed and verified {len(rows)} integrated source/test files; originals backed up',flush=True)
