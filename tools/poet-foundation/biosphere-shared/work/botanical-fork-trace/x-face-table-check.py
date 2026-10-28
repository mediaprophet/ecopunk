"""Static x-face sign-table consistency diagnostic; not a topology proof."""
import pathlib,re
source=pathlib.Path(r'C:\github\qualiaDB\crates\qualia-core-db\src\specialized_libs\computational_geometry\isosurface.rs').read_text(encoding='utf8')
body=source.split('const TRI_TABLE_RAW: [&[i8]; 256] = [',1)[1].split('\n];',1)[0]
rows=[list(map(int,re.findall(r'-?\d+',x))) for x in re.findall(r'&\[([^\]]*)\]',body)]
assert len(rows)==256
face_a={1:3,10:11,5:7,9:8};face_b=set(face_a.values())
def segments(row,face,translate):
 result=set()
 for i in range(0,len(row),3):
  selected=[translate.get(e,e) for e in row[i:i+3] if e in face]
  if len(selected)==2:result.add(tuple(sorted(selected)))
 return result
mismatches=[]; checked=0
for a in range(256):
 for b in range(256):
  if not all(((a>>x)&1)==((b>>y)&1) for x,y in [(1,0),(2,3),(6,7),(5,4)]):continue
  checked+=1; first=segments(rows[a],set(face_a),face_a); second=segments(rows[b],face_b,{})
  if first!=second:mismatches.append((a,b,first,second))
print('sign-consistent x-face pairs',checked,'mismatches',len(mismatches))
