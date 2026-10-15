// Authored, deterministic geometry for the first ecopunk biosphere review set.
// QualiaDB's pinned game-demo WASM compiler produces the .10d containers.
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir, copyFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import init, { asset_compile_10d_with_provenance, asset_verify_10d } from '../../../../game-demo/web/pkg/rolling_commons_shell.js';

const here = dirname(fileURLToPath(import.meta.url));
const output = join(here, '10d');
const wasm = await readFile(resolve(here, '../../../../game-demo/web/pkg/rolling_commons_shell_bg.wasm'));
await init({ module_or_path: wasm });
await mkdir(output, { recursive: true });

const mesh = () => ({ positions: [], triangles: [] });
function vertex(m, p) { const i = m.positions.length / 3; m.positions.push(...p); return i; }
function tri(m, a, b, c) { m.triangles.push(a, b, c); }
function vec(a,b) { return [a[0]-b[0],a[1]-b[1],a[2]-b[2]]; }
function cross(a,b) { return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]; }
function norm(a) { const l=Math.hypot(...a)||1; return a.map(x=>x/l); }
function add(a,b,s=1) { return a.map((x,i)=>x+b[i]*s); }
function tube(m, start, end, radiusA, radiusB, sides=10) {
  const axis=norm(vec(end,start)); const guide=Math.abs(axis[1])<0.85?[0,1,0]:[1,0,0];
  const u=norm(cross(axis,guide)), v=cross(axis,u); const rings=[];
  for (const [center,radius] of [[start,radiusA],[end,radiusB]]) {
    const ring=[];
    for(let j=0;j<sides;j++){const a=2*Math.PI*j/sides;ring.push(vertex(m,add(add(center,u,Math.cos(a)*radius),v,Math.sin(a)*radius)));}
    rings.push(ring);
  }
  for(let j=0;j<sides;j++){const n=(j+1)%sides;tri(m,rings[0][j],rings[0][n],rings[1][n]);tri(m,rings[0][j],rings[1][n],rings[1][j]);}
  const a=vertex(m,start),b=vertex(m,end);
  for(let j=0;j<sides;j++){const n=(j+1)%sides;tri(m,a,rings[0][n],rings[0][j]);tri(m,b,rings[1][j],rings[1][n]);}
}
function polyTube(m, points, radii, sides=12, roughness=0) {
  const rings=[];
  for(let i=0;i<points.length;i++){
    const axis=norm(vec(points[Math.min(i+1,points.length-1)],points[Math.max(i-1,0)]));
    const guide=Math.abs(axis[1])<0.88?[0,1,0]:[0,0,1];
    const u=norm(cross(axis,guide)),v=cross(axis,u),ring=[];
    for(let j=0;j<sides;j++){
      const a=2*Math.PI*j/sides;
      const r=radii[i]*(1+roughness*(0.6*Math.sin(a*7+i*1.31)+0.4*Math.sin(a*13-i*0.83)));
      ring.push(vertex(m,add(add(points[i],u,Math.cos(a)*r),v,Math.sin(a)*r)));
    }
    rings.push(ring);
  }
  for(let i=0;i<rings.length-1;i++)for(let j=0;j<sides;j++){
    const n=(j+1)%sides;
    tri(m,rings[i][j],rings[i][n],rings[i+1][n]);
    tri(m,rings[i][j],rings[i+1][n],rings[i+1][j]);
  }
  const low=vertex(m,points[0]),high=vertex(m,points.at(-1));
  for(let j=0;j<sides;j++){
    const n=(j+1)%sides;
    tri(m,low,rings[0][n],rings[0][j]);
    tri(m,high,rings.at(-1)[j],rings.at(-1)[n]);
  }
}
function refinePath(points,radii,steps=8){
  const ps=[],rs=[];
  for(let i=0;i<points.length-1;i++)for(let s=0;s<steps;s++){
    const t=s/steps;
    ps.push(points[i].map((v,k)=>v+(points[i+1][k]-v)*t));
    rs.push(radii[i]+(radii[i+1]-radii[i])*t);
  }
  ps.push(points.at(-1));rs.push(radii.at(-1));
  return [ps,rs];
}
function ellipsoid(m, center, radii, lat=7, lon=10) {
  const rings=[];
  for(let i=0;i<=lat;i++){
    const theta=Math.PI*i/lat, ring=[];
    for(let j=0;j<=lon;j++){
      const phi=2*Math.PI*j/lon;
      ring.push(vertex(m,[center[0]+radii[0]*Math.sin(theta)*Math.cos(phi),center[1]+radii[1]*Math.cos(theta),center[2]+radii[2]*Math.sin(theta)*Math.sin(phi)]));
    }
    rings.push(ring);
  }
  for(let i=0;i<lat;i++)for(let j=0;j<lon;j++){
    const a=rings[i][j],b=rings[i+1][j],c=rings[i+1][j+1],d=rings[i][j+1];
    tri(m,a,b,c);tri(m,a,c,d);
  }
}
function leaf(m, base, tip, width) {
  const axis=norm(vec(tip,base));let side=norm(cross(axis,[0,0,1]));
  if(Math.hypot(...side)<0.5)side=[1,0,0];
  const p0=vertex(m,base),p1=vertex(m,add(add(base,axis,Math.hypot(...vec(tip,base))*0.46),side,width)),
    p2=vertex(m,tip),p3=vertex(m,add(add(base,axis,Math.hypot(...vec(tip,base))*0.46),side,-width));
  tri(m,p0,p1,p2);tri(m,p0,p2,p3);tri(m,p2,p1,p0);tri(m,p3,p2,p0);
}
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const models=[];
async function saveModel(id, label, scale, components) {
  const rows=[];
  for(const [name,color,m] of components){
    if(!m.triangles.length)continue;
    const source=JSON.stringify({creator:'ecopunk',model:id,component:name,
      taxon:'Corymbia aparrerinja',basis:'procedural illustration from local taxon traits',
      sourceFiles:['entities/taxa/corymbia-aparrerinja/traits.json','entities/taxa/corymbia-aparrerinja/corymbia-aparrerinja-manifest.json']});
    const bytes=Buffer.from(asset_compile_10d_with_provenance(new Float32Array(m.positions),new Uint32Array(m.triangles),source));
    const report=asset_verify_10d(bytes);
    if(report?.valid===false)throw new Error(`${id}/${name}: QualiaDB rejected compiled .10d`);
    const file=`${id}-${name}.10d`;
    await writeFile(join(output,file),bytes);
    rows.push({id:`eco:model/corymbia-aparrerinja/${id}/${name}`,label:name,file:`10d/${file}`,sha256:hash(bytes),bytes:bytes.length,
      rgba:color,vertices:m.positions.length/3,triangles:m.triangles.length/3});
  }
  models.push({id,label,scale,components:rows});
}

function tree(height,seed,phase='healthy',form='single',detail='hero') {
  const bark=mesh(),twigs=mesh(),canopy=mesh(),leaves=mesh(),flowers=mesh(),char=mesh(),regrowth=mesh();
  const broad=form==='multi-stem', rocky=form==='rocky-open';
  const p=(x,y,z)=>[x*height,y*height,z*height];
  const trunk=[p(0,0,0),p(0.012,0.13,0),p(0.035,0.28,0.006),p(0.045,0.42,0.014),p(0.03,0.55,0.018),p(0.01,0.64,0.012)];
  polyTube(bark,...refinePath(trunk,[0.052,0.047,0.039,0.030,0.020,0.010].map(n=>n*height),detail==='hero'?10:3),detail==='hero'?32:12,0.018);
  if(broad)for(const side of [-1,1]){
    polyTube(bark,...refinePath([p(0,0,0),p(side*0.035,0.12,0.015),p(side*0.09,0.30,0.025),p(side*0.16,0.52,0.04)],
      [0.037,0.034,0.024,0.009].map(n=>n*height),detail==='hero'?8:2),detail==='hero'?24:10,0.015);
  }
  const branchCount=rocky?8:12;
  for(let k=0;k<branchCount;k++){
    const theta=k*2.399963+seed, dx=Math.cos(theta),dz=Math.sin(theta);
    const forkY=(broad?0.28:0.34)+0.027*k, reach=(broad?0.35:rocky?0.22:0.30)*(0.82+0.13*(k%4));
    const fork=p(0.03+0.01*Math.sin(k),forkY,0.01);
    const elbow=p(0.04+dx*reach*0.44,forkY+0.12+0.02*(k%3),dz*reach*0.42);
    const tip=p(0.04+dx*reach,forkY+0.18+0.024*(k%4),dz*reach*0.94);
    polyTube(bark,...refinePath([fork,elbow,tip],[0.014,0.010,0.003].map(n=>n*height),detail==='hero'?5:2),detail==='hero'?16:8,0.012);
    for(let j=0;j<3;j++)for(let q=0;q<2;q++){
      const spread=theta+(j-1)*0.65+(q?0.28:-0.28);
      const radius=height*(0.055+0.017*((k+j+q)%3));
      const shoulder=add(tip,[Math.cos(spread)*radius*0.38,height*(0.015+0.007*j),Math.sin(spread)*radius*0.38]);
      const endTwig=add(tip,[Math.cos(spread)*radius,height*(0.016+0.014*q),Math.sin(spread)*radius]);
      polyTube(twigs,[tip,shoulder,endTwig],[height*0.003,height*0.0018,height*0.0005],detail==='hero'?7:5);
      if(phase==='canopy-scorched' && (k+j+q)%4!==0)continue;
      for(let l=0;l<(detail==='far'?1:2);l++){
        const f=0.38+0.38*l;
        const center=add(shoulder,vec(endTwig,shoulder).map(v=>v*f));
        const radius=height*(0.018+0.005*((k*3+j+q+l)%3));
        ellipsoid(canopy,add(center,[0,height*0.002,0]),[radius*1.35,radius*0.66,radius],6,9);
      }
      const count=phase==='canopy-scorched'?(detail==='hero'?20:detail==='mid'?7:2):detail==='hero'?160:detail==='mid'?38:9;
      for(let n=0;n<count;n++){
        const f=((n*71+11)%count)/count;
        const a=n*2.399963+spread;
        const base=add(shoulder,vec(endTwig,shoulder).map(v=>v*f));
        const spreadRadius=height*(0.012+0.018*Math.sin(Math.PI*f))*((n*17)%29)/29;
        const stem=add(base,[Math.cos(a)*spreadRadius,height*0.006*Math.sin(a*3+n),Math.sin(a)*spreadRadius]);
        const bladeLength=(height<6?0.085:0.12)*(0.75+0.4*((n*17)%23)/23);
        const bladeTip=add(stem,[Math.cos(a)*bladeLength*0.42,-bladeLength*0.8,Math.sin(a)*bladeLength*0.42]);
        leaf(leaves,stem,bladeTip,bladeLength*0.19);
      }
      if(phase==='healthy' && (k+j+q)%9===0){
        for(let b=0;b<5;b++)ellipsoid(flowers,add(endTwig,[0.12*b-0.2,0.06,0]),[0.035,0.045,0.035],5,6);
      }
    }
  }
  if(phase==='ground-scorched'||phase==='epicormic-regrowth'){
    polyTube(char,[p(0,0,0),p(0.012,0.12,0),p(0.03,0.24,0.005)],
      [0.053,0.048,0.041].map(n=>n*height),16);
  }
  if(phase==='epicormic-regrowth')for(let i=0;i<20;i++){
    const a=i*2.399963,y=height*(0.17+(i%12)*0.028);
    const base=[height*0.035,y,0],tip=add(base,[Math.cos(a)*0.32,0.09,Math.sin(a)*0.32]);
    ellipsoid(regrowth,tip,[0.23,0.12,0.19],6,8);
  }
  const result=[
    ['chalk-bark',[0.82,0.79,0.70,1],bark],
    ['branchwork',[0.43,0.37,0.28,1],twigs],
    ['irregular-canopy',phase==='canopy-scorched'?[0.16,0.17,0.15,1]:[0.25,0.35,0.27,1],canopy],
    ['falcate-foliage',phase==='canopy-scorched'?[0.20,0.17,0.14,1]:[0.32,0.43,0.34,1],leaves],
    ['cream-buds',[0.89,0.83,0.60,1],flowers],
  ];
  if(char.triangles.length)result.push(['lower-trunk-scorch',[0.17,0.14,0.12,1],char]);
  if(regrowth.triangles.length)result.push(['epicormic-shoots',[0.46,0.61,0.32,1],regrowth]);
  return result;
}
await saveModel('mature-tree','Mature ghost gum · branching silhouette','organism, 15 m',tree(15,0.34));
await saveModel('juvenile-tree','Juvenile ghost gum · growth stage','organism, 4 m',tree(4,1.4));
await saveModel('multi-stem-tree','Multi-stem ghost gum · broad crown','organism, 12 m',tree(12,0.8,'healthy','multi-stem'));
await saveModel('rocky-open-tree','Rocky-site ghost gum · open crown','organism, 7 m',tree(7,2.1,'healthy','rocky-open'));
await saveModel('ground-scorched-tree','Ground-fire scorch · illustrative state','organism, 15 m',tree(15,0.34,'ground-scorched'));
await saveModel('canopy-scorched-tree','Canopy scorch · illustrative state','organism, 15 m',tree(15,0.34,'canopy-scorched'));
await saveModel('epicormic-regrowth-tree','Post-fire shoots · illustrative state','organism, 15 m',tree(15,0.34,'epicormic-regrowth'));
const treeVariants=[
  ['mature-tree',15,0.34,'healthy','single'],
  ['juvenile-tree',4,1.4,'healthy','single'],
  ['multi-stem-tree',12,0.8,'healthy','multi-stem'],
  ['rocky-open-tree',7,2.1,'healthy','rocky-open'],
  ['ground-scorched-tree',15,0.34,'ground-scorched','single'],
  ['canopy-scorched-tree',15,0.34,'canopy-scorched','single'],
  ['epicormic-regrowth-tree',15,0.34,'epicormic-regrowth','single']
];
for(const [id,height,seed,phase,form] of treeVariants){
  for(const [detail,label] of [['mid','mid-distance'],['far','distant']]){
    await saveModel(`${id}-${detail}`,`${id} · ${label} geometry`,`organism, ${height} m`,tree(height,seed,phase,form,detail));
  }
}
{
  const blade=mesh(),midrib=mesh();
  leaf(blade,[0,0,0],[0.025,0.12,0.002],0.009);
  tube(midrib,[0,0,0],[0.025,0.12,0.002],0.0007,0.00015,6);
  await saveModel('leaf-study','Falcate leaf · anatomy study','organ, 120 mm',[
    ['blade',[0.38,0.49,0.39,1],blade],['midrib',[0.69,0.73,0.57,1],midrib]]);
}
{
  const bud=mesh(),stamens=mesh(),stem=mesh();
  tube(stem,[0,-0.025,0],[0,-0.003,0],0.0014,0.0010,8);
  ellipsoid(bud,[0,0,0],[0.009,0.012,0.009],10,14);
  for(let i=0;i<36;i++){const a=2*Math.PI*i/36,r=0.007+0.002*(i%3);tube(stamens,[r*Math.cos(a),0.005,r*Math.sin(a)],[(r+0.005)*Math.cos(a),0.014+0.003*(i%4),(r+0.005)*Math.sin(a)],0.00038,0.0002,5);}
  await saveModel('flower-study','Cream flower · reproductive detail','organ, 35 mm',[
    ['receptacle',[0.72,0.65,0.45,1],bud],['stamens',[0.96,0.91,0.74,1],stamens],['pedicel',[0.39,0.42,0.29,1],stem]]);
}
{
  const fruit=mesh(),rim=mesh(),stem=mesh();
  ellipsoid(fruit,[0,0,0],[0.008,0.010,0.008],10,14);
  for(let i=0;i<12;i++){const a=2*Math.PI*i/12,b=2*Math.PI*(i+1)/12;tube(rim,[0.007*Math.cos(a),0.008,0.007*Math.sin(a)],[0.007*Math.cos(b),0.008,0.007*Math.sin(b)],0.0008,0.0008,5);}
  tube(stem,[0,-0.022,0],[0,-0.008,0],0.0011,0.0008,7);
  await saveModel('gumnut-study','Woody capsule · seed vessel','organ, 25 mm',[
    ['capsule',[0.47,0.40,0.31,1],fruit],['rim',[0.66,0.57,0.43,1],rim],['pedicel',[0.35,0.32,0.26,1],stem]]);
}
{
  const bark=mesh(),fissures=mesh();
  tube(bark,[0,0,0],[0,1.4,0],0.18,0.14,20);
  for(let i=0;i<19;i++){const a=2*Math.PI*i/19,y=(i*0.618033)%1.2;
    tube(fissures,[0.175*Math.cos(a),y,0.175*Math.sin(a)],[0.15*Math.cos(a+0.04),y+0.18,0.15*Math.sin(a+0.04)],0.003,0.001,5);
  }
  await saveModel('bark-study','Powdery chalk bark · surface study','structure, 1.4 m',[
    ['surface',[0.84,0.81,0.73,1],bark],['weathering',[0.58,0.52,0.42,1],fissures]]);
}
const graphSource=resolve(here,'../../../entities/taxa/corymbia-aparrerinja/ecology-graph.json');
const graphVolume=resolve(here,'../../../entities/taxa/corymbia-aparrerinja/ecology.q42');
const graphFiles=join(here,'graph');
await mkdir(graphFiles,{recursive:true});
await copyFile(graphSource,join(graphFiles,'ecology-graph.json'));
await copyFile(graphVolume,join(graphFiles,'ecology.q42'));
const graphJson=await readFile(graphSource),graphQ42=await readFile(graphVolume);
const graph=JSON.parse(graphJson);
const manifest={schema:'ecopunk-model-review-v1',taxon:'urn:qualia:taxon:corymbia_aparrerinja',scientificName:'Corymbia aparrerinja',
  title:'Ghost gum biosphere model studies',status:'authored prototype; visual and cultural review pending',styleTier:'T2 stylized prototype',
  epistemicStatus:'Procedural illustration derived from project traits; not a scan, occurrence record, or verified botanical specimen',
  sourceFiles:['entities/taxa/corymbia-aparrerinja/traits.json','entities/taxa/corymbia-aparrerinja/corymbia-aparrerinja-manifest.json'],
  compiler:'game-demo pinned QualiaDB WASM asset_compile_10d_with_provenance',
  visualReferences:[
    {url:'https://bie.ala.org.au/species/https://id.biodiversity.org.au/taxon/apni/51289371#gallery',
      observer:'Fagg, M.',observation:'Photograph of a mature multi-stem tree with a broad, irregular crown; gallery image dated 2016-10-03.',
      use:'visual reference for multi-stem-tree; no photograph bytes included in the model set'},
    {url:'https://bie.ala.org.au/species/https://id.biodiversity.org.au/taxon/apni/51289371#gallery',
      observer:'Dean Nicolle',observation:'Gallery photographs show smaller, open-crowned ghost gums among rocky outcrops.',
      use:'visual reference for rocky-open-tree; no photograph bytes included in the model set'}
  ],
  lodGroups:treeVariants.map(([id])=>({root:id,levels:[
    {model:id,level:'hero',intent:'close inspection'},
    {model:`${id}-mid`,level:'mid',intent:'mid-distance'},
    {model:`${id}-far`,level:'far',intent:'distant view'}
  ]})),
  graph:{source:'graph/ecology-graph.json',sourceSha256:hash(graphJson),sourceBytes:graphJson.length,
    volume:'graph/ecology.q42',volumeSha256:hash(graphQ42),volumeBytes:graphQ42.length,
    claims:graph.claims.length,gaps:graph.gaps.length},
  animations:[
    {id:'ground-fire-sequence',label:'Ground fire and observed epicormic response',
      evidence:'Illustrative state sequence; timing, fire temperature and survival probability are unknown',
      frames:[{model:'mature-tree',time:0},{model:'ground-scorched-tree',time:1},{model:'epicormic-regrowth-tree',time:2}]},
    {id:'canopy-scorch-sequence',label:'Canopy scorch scenario',
      evidence:'Illustrative exposure state only; crown-fire outcome is not established for this species',
      frames:[{model:'mature-tree',time:0},{model:'canopy-scorched-tree',time:1}]}
  ],models};
await writeFile(join(here,'model-set.json'),JSON.stringify(manifest,null,2)+'\n');
console.log(`Built ${models.length} models, ${models.reduce((n,m)=>n+m.components.length,0)} QualiaDB .10d components in ${output}`);
