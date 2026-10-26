"""Read-only numerical probe mirroring inspected MeshMiniHeader/dequantize f32 steps.
Not a replacement for the native container/integrity/admission validation.
"""
import hashlib,json,math,pathlib,struct
root=pathlib.Path(r'C:\github\ecopunk\assets\reference\community-grounds-growing-station\outputs\round1-film-006')
blob=(root/'growing-station.10d').read_bytes()
f32=lambda x:struct.unpack('<f',struct.pack('<f',x))[0]
offset,count=struct.unpack_from('<II',blob,56)
sections={}
for n in range(count):
    kind,tier,res,start,size,stride,elements,crc=struct.unpack_from('<BBHIIIII',blob,offset+n*24)
    assert start+size<=len(blob)
    sections[kind]=(start,size)
start,size=sections[1]
flags,res,nv,nt,*tail=struct.unpack_from('<HHII6fI',blob,start)
lo,hi=tail[:3],tail[3:6]
extent=[f32(hi[a]-lo[a]) for a in range(3)]
actual_lo=[math.inf]*3;actual_hi=[-math.inf]*3;outside=[0,0,0];examples=[];nonfinite=0
for i in range(nv):
    qs=struct.unpack_from('<HHH',blob,start+40+i*6)
    p=[f32(lo[a]+f32(f32(qs[a]/65535.0)*extent[a])) for a in range(3)]
    for a in range(3):
        if not math.isfinite(p[a]):nonfinite+=1
        actual_lo[a]=min(actual_lo[a],p[a]);actual_hi[a]=max(actual_hi[a],p[a])
        if p[a]<lo[a] or p[a]>hi[a]:
            outside[a]+=1
            if len(examples)<12:examples.append(dict(vertex=i,axis=a,q=qs[a],decoded=p[a],min=lo[a],max=hi[a],excess=max(lo[a]-p[a],p[a]-hi[a])))
idxfmt='<HHH' if flags&1 else '<III';idxsize=6 if flags&1 else 12
idxstart=start+40+nv*6
bad_indices=sum(any(v>=nv for v in struct.unpack_from(idxfmt,blob,idxstart+i*idxsize)) for i in range(nt))
uvstart,uvsize=sections[14];uv_count=struct.unpack_from('<I',blob,uvstart+8)[0]
uv_invalid=sum(not math.isfinite(v) or abs(v)>1000000 for uv in struct.iter_unpack('<ff',blob[uvstart+16:uvstart+uvsize]) for v in uv)
nstart,nsize=sections[8];normal_magic,version,nflags,ncount,res=struct.unpack_from('<4sHHII',blob,nstart)
assert normal_magic in (b'NRM1',b'FRM1')
stride=4 if normal_magic==b'NRM1' else (4 if nflags&1 else 0)+(8 if nflags&2 else 0)
normal_bad=0;min_len2=math.inf;max_len2=0
for i in range(ncount):
    a,b=struct.unpack_from('<hh',blob,nstart+16+i*stride)
    x=max(f32(a/32767.0),-1.0);y=max(f32(b/32767.0),-1.0);z=f32(f32(1.0-abs(x))-abs(y))
    if z<0:
        oldx=x;x=f32(f32(1-abs(y))*(-1.0 if oldx<0 else 1.0));y=f32(f32(1-abs(oldx))*(-1.0 if y<0 else 1.0))
    length=f32(math.sqrt(f32(f32(f32(x*x)+f32(y*y))+f32(z*z))))
    if length>2**-23:x,y,z=[f32(v/length) for v in (x,y,z)]
    length2=sum(float(v)**2 for v in (x,y,z));min_len2=min(min_len2,length2);max_len2=max(max_len2,length2)
    normal_bad+=int(not math.isfinite(length2) or length2<=2**-52)
parts=json.loads((root/'parts.json').read_text())['parts']
report=dict(schema='ecopunk-round1-decoded-bounds-probe/1',source_file=str(root/'growing-station.10d'),sha256=hashlib.sha256(blob).hexdigest(),vertices=nv,triangles=nt,index_count=nt*3,parts=len(parts),stored_min=lo,stored_max=hi,f32_extents=extent,decoded_min=actual_lo,decoded_max=actual_hi,outside_positions_by_axis=outside,outside_examples=examples,nonfinite_coordinates=nonfinite,bad_index_triangles=bad_indices,uv_count=uv_count,invalid_uv_coordinates=uv_invalid,normal_count=ncount,invalid_normals=normal_bad,normal_length_squared_range=[min_len2,max_len2],scope='Read-only Python mirror of inspected codec with explicit float32 rounding per arithmetic step; native decode succeeded earlier, this is not a native re-execution or integrity certificate.')
print(json.dumps(report,indent=2))