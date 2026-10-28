"""Count crossings in the staged test's synthetic 15^3 three-arm sampled field.
Independent in-memory diagnostic, not a Core execution, renderer or asset."""
from math import sqrt
D=(15,15,15); O=(-3.57,-3.59,-3.53); step=.5
arms=[(-1.4,.3,1.2),(1.3,.1,1.3),(0,-.2,-1.7)]; radii=[.52,.43,.62]
def capsule(p,b,r):
 den=sum(v*v for v in b); h=max(0.,min(1.,sum(x*y for x,y in zip(p,b))/den)); return sqrt(sum((x-y*h)**2 for x,y in zip(p,b)))-r
def smooth(a,b,k):
 h=max(0.,min(1.,.5+.5*(b-a)/k));return b+(a-b)*h-k*h*(1.-h)
g=[]
for z in range(D[2]):
 for y in range(D[1]):
  for x in range(D[0]):
   p=tuple(O[i]+(x,y,z)[i]*step for i in range(3));a,b,c=(capsule(p,arms[i],radii[i]) for i in range(3));g.append(smooth(smooth(a,b,.2),c,.2))
C=((0,0,0),(1,0,0),(1,1,0),(0,1,0),(0,0,1),(1,0,1),(1,1,1),(0,1,1));T=((0,1,2,6),(0,2,3,6),(0,3,7,6),(0,7,4,6),(0,4,5,6),(0,5,1,6))
faces=0; edges=set(); surface_cells=0; boundary_inside=0; exact_zero=0
for z in range(D[2]):
 for y in range(D[1]):
  for x in range(D[0]):
   v=g[x+y*D[0]+z*D[0]*D[1]];exact_zero += v==0
   if x in (0,D[0]-1) or y in (0,D[1]-1) or z in (0,D[2]-1):boundary_inside += v<0
for z in range(D[2]-1):
 for y in range(D[1]-1):
  for x in range(D[0]-1):
   ids=[x+dx+(y+dy)*D[0]+(z+dz)*D[0]*D[1] for dx,dy,dz in C]; vals=[g[i]<0 for i in ids]; surface_cells += any(vals) and not all(vals)
   for tet in T:
    inside=[k for k in tet if vals[k]]; outside=[k for k in tet if not vals[k]]
    if inside and outside:
     faces += 1 if len(inside) in (1,3) else 2
     edges.update(tuple(sorted((ids[a],ids[b]))) for a in inside for b in outside)
V=len(edges);F=faces;H=3*F;S=1<<(max(1,6*F)-1).bit_length();hash_work=2*H*S;component=F*H;house=32*H+8*V+4*S+64; work=hash_work+component+house
print(f'cells={(D[0]-1)*(D[1]-1)*(D[2]-1)} active_cells={surface_cells} vertices={V} faces={F} halfedges={H} slots={S} boundary_inside={boundary_inside} exact_zero={exact_zero}')
print(f'hash_worst={hash_work} component_worst={component} housekeeping={house} total_work_bound={work}')
print('Synthetic Python count replicates topology case counting only; no QualiaDB execution, no geometric closure, no production asset.')
