from math import sqrt
from collections import Counter
vertices=[(0,0,0),(1,0,0),(1,1,0),(0,1,0),(0,0,1),(1,0,1),(1,1,1),(0,1,1)]
edges=[(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(6,7),(7,4),(0,4),(1,5),(2,6),(3,7)]
faces=[(0,1,2,3),(4,5,6,7),(0,1,5,4),(1,2,6,5),(2,3,7,6),(3,0,4,7)]
def field(point):
 def tapered(arm):
  denom=sum(x*x for x in arm)
  t=max(0.0,min(1.0,sum(point[i]*arm[i] for i in range(3))/denom))
  return sqrt(sum((point[i]-arm[i]*t)**2 for i in range(3)))-(.37+(.22-.37)*t)
 def blend(a,b,k):
  h=max(0.0,min(1.0,.5+.5*(b-a)/k))
  return b+(a-b)*h-k*h*(1-h)
 values=[tapered(arm) for arm in [(0,0,-1),(1,0,.7),(-1,0,.7)]]
 return blend(blend(values[0],values[1],.13),values[2],.13)
def is_connected(mask,sign):
 nodes={i for i in range(8) if ((mask>>i)&1)==sign};seen={next(iter(nodes))};pending=list(seen)
 while pending:
  node=pending.pop()
  for a,b in edges:
   if a==node and b in nodes and b not in seen:seen.add(b);pending.append(b)
   if b==node and a in nodes and a not in seen:seen.add(a);pending.append(a)
 return len(seen)==len(nodes)
rejected=Counter(); surface=0
for x in range(16):
 for y in range(16):
  for z in range(16):
   values=[field(((x+dx)*.18-1.4,(y+dy)*.18-1.4,(z+dz)*.18-1.4)) for dx,dy,dz in vertices]
   mask=sum((value<0)<<i for i,value in enumerate(values));count=mask.bit_count()
   if count==0 or count==8:continue
   surface+=1
   if not is_connected(mask,0) or not is_connected(mask,1):rejected['disconnected signs']+=1
   elif count==4 and not any(all(mask>>i&1 for i in face) for face in faces):rejected['ambiguous four']+=1
print('surface cells',surface,'rejected by conservative mask',dict(rejected))
