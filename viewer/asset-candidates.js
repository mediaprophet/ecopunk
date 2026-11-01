// Inventory/navigation only. The existing Rust/Core viewer owns all decoding
// and rendering; this page performs no geometry, animation or semantic inference.
import {fetchBounded,sha256} from './showasset-engine.js';
import {createAssetPlayback} from './asset-playback.js';
const $=id=>document.getElementById(id);
const MAX_INDEX=256*1024;
const root=new URL('../',import.meta.url);
let lifecycle=0,indexAbort,fallback=()=>{};
const playback=createAssetPlayback(root,()=>fallback());
async function loadIndex(url,expectedSha=null,max=MAX_INDEX){
  const bytes=await fetchBounded(url,max,indexAbort.signal);
  if(expectedSha&&await sha256(bytes)!==expectedSha)throw Error('Pinned motion information differs');
  return JSON.parse(new TextDecoder('utf-8',{fatal:true}).decode(bytes));
}
async function start(){
  const token=++lifecycle;indexAbort?.abort();indexAbort=new AbortController();playback.close();
  const current=()=>{if(token!==lifecycle)throw new DOMException('Retired review request','AbortError');};
  for(const id of ['behaviour','candidate','sources'])$(id).replaceChildren();
  $('mode-motion').disabled=true;$('preview').src='about:blank';
  $('motion-review').hidden=true;$('snapshot-review').hidden=false;
  const set=new URLSearchParams(location.search).get('set');
  if(!set||!/^\/assets\/production-studies\/[a-zA-Z0-9_./-]+\/review-index\.json$/.test(set))throw Error('Provide a production-study review-index.json in the set parameter.');
  const indexUrl=new URL(set.slice(1),root);
  if(indexUrl.origin!==root.origin||!indexUrl.pathname.startsWith(root.pathname+'assets/production-studies/'))throw Error('Inventory path is outside the local study collection');
  const index=await loadIndex(indexUrl);current();
  if(!Array.isArray(index.files)||index.files.length>256)throw Error('Invalid candidate inventory');
  const seen=new Set();
  const entries=index.files.map(row=>{
    if(!row||typeof row.file!=='string'||!/^[a-zA-Z0-9_.-]+$/.test(row.file)
      ||!/^[a-f0-9]{64}$/.test(row.sha256)||!Number.isSafeInteger(row.bytes)||row.bytes<=0||row.bytes>128*1024*1024)throw Error('Invalid candidate resource');
    if(seen.has(row.file))throw Error('Candidate inventory contains duplicate resource names');seen.add(row.file);
    const url=new URL(row.file,indexUrl);return {...row,url};
  });
  const candidates=entries.filter(e=>e.file.endsWith('.10d')&&e.bytes<=16*1024*1024);
  const packageRecord=entries.find(e=>e.file==='package-review.json');
  let admittedPackage;
  if(packageRecord){
    // Native inspector transport projection, not a semantic store or graph.
    const projection=await loadIndex(packageRecord.url,packageRecord.sha256);current();
    const packages=entries.filter(e=>e.file.endsWith('.hmc'));
    if(packages.length!==1||packages[0].bytes>64*1024*1024
      ||projection.schema!=='ecopunk.species-package-review-projection/v1'
      ||projection.package_sha256!==packages[0].sha256||projection.package_bytes!==packages[0].bytes
      ||!Array.isArray(projection.resources)||projection.resources.length>165
      ||!Array.isArray(projection.coverage)||projection.coverage.length>128)throw Error('Native package projection binding differs');
    const packageBytes=await fetchBounded(packages[0].url,64*1024*1024,indexAbort.signal);current();
    if(packageBytes.length!==packages[0].bytes||await sha256(packageBytes)!==projection.package_sha256)throw Error('Exact HMC package bytes differ');
    current();admittedPackage={bytes:packageBytes,packageSha:projection.package_sha256};
    const graphs=projection.resources.filter(r=>r.kind==='q42');
    for(const r of graphs){
      if(typeof r.key!=='string'||!r.key.startsWith('resources/')||!r.key.endsWith('.q42')
        ||!/^[a-f0-9]{64}$/.test(r.sha256)||!Number.isSafeInteger(r.bytes)||r.bytes<=0)throw Error('Invalid native Q42 inventory');
    }
    $('knowledge-status').textContent=graphs.length
      ?`${graphs.length} native Q42 resource${graphs.length===1?'':'s'} retained in the SHA-verified HMC. The review graph describes the authored study and its provenance; the biological species corpus remains incomplete. JSON here is a navigation projection.`
      :'Q42 knowledge is missing from this HMC. JSON records and geometry do not constitute an ontological species corpus.';
    const dimensions=projection.coverage.filter(r=>['phenotype','reproductive_condition','parent_offspring_relation','lifecycle_stage','age','growth'].includes(r.dimension));
    const gaps=dimensions.filter(r=>r.status!=='declared_unaccepted');
    $('form-status').textContent=`${gaps.length} recorded form, age and growth coverage gaps. Source declarations require review; absent forms are never substituted with the current model.`;
  }
  if(!candidates.length)throw Error('No candidate fits the current viewer profile');
  const groups=[];
  const master=candidates.find(e=>e.file==='wallaby-motion-master.10d');
  if(master)groups.push({id:'rest',title:'Rest · rig master',samples:[{entry:master,time:0}],duration:0});
  const metadata=entries.find(e=>e.file==='master-manifest.json');
  const manifest=metadata?await loadIndex(metadata.url,metadata.sha256):null;current();
  const clips=manifest?.detail?.rig?.authored_motion?.clips??[];
  if(!Array.isArray(clips)||clips.length>32)throw Error('Motion inventory exceeds this review profile');
  let phaseCount=0;
  for(const clip of clips){
    if(typeof clip.source_clip_id!=='string'||!/^[a-zA-Z0-9_-]{1,128}$/.test(clip.source_clip_id)
      ||!Number.isFinite(clip.duration_s)||clip.duration_s<=0||clip.duration_s>30)throw Error('Invalid named clip');
    const prefix=clip.source_clip_id+'-';const samples=[];
    for(const entry of candidates.filter(e=>e.file.startsWith(prefix)&&/^\d+\.10d$/.test(e.file.slice(prefix.length)))){
      if(++phaseCount>64)throw Error('Too many motion phases');
      const record=entries.find(e=>e.file===entry.file.replace(/\.10d$/,'.json'));
      if(!record)throw Error('Pinned phase information missing');
      const phase=await loadIndex(record.url,record.sha256,16*1024);current();
      if(!Number.isFinite(phase.time_s)||phase.time_s<0||phase.time_s>clip.duration_s
        ||phase.master_sha256!==master?.sha256)throw Error('Phase/master binding differs');
      samples.push({entry,time:phase.time_s});
    }
    samples.sort((a,b)=>a.time-b.time);
    if(samples.length){
      const name=clip.source_clip_id.replace(/-study$/,'').replaceAll('-',' ');
      groups.push({id:clip.source_clip_id,title:name[0].toUpperCase()+name.slice(1),samples,duration:clip.duration_s,looping:clip.looping===true});
    }
  }
  if(!groups.length)groups.push({id:'geometry',title:'Geometry views',samples:candidates.map(entry=>({entry,time:0})),duration:0});
  for(const [i,group]of groups.entries()){
    const option=document.createElement('option');option.value=String(i);
    option.textContent=group.title+(group.duration?` · ${group.duration.toFixed(2)} s`:'');$('behaviour').append(option);
  }
  for(const entry of entries.filter(e=>/\.(hmc|vibe|q42|m64|ttl|json|txt)$/.test(e.file))){
    const li=document.createElement('li'),link=document.createElement('a');link.href=entry.url.href;link.download=entry.file;
    link.textContent=`${entry.file} · ${(entry.bytes/1024).toFixed(1)} KiB`;li.append(link);$('sources').append(li);
  }
  let mode='snapshots';
  function select(){
    const group=groups[Number($('behaviour').value)],sample=group?.samples[Number($('candidate').value)];
    if(!sample)return;const entry=sample.entry;
    const view=new URL('viewer/showasset.html',root);view.searchParams.set('asset',entry.url.pathname);view.searchParams.set('sha256',entry.sha256);
    $('open').href=view.href;$('download').href=entry.url.href;
    const embedded=new URL(view);embedded.searchParams.set('embed','1');if(mode==='snapshots')$('preview').src=embedded.href;
    $('identity').textContent=`${entry.file} · ${(entry.bytes/1024/1024).toFixed(2)} MiB · SHA-256 ${entry.sha256}`;
    $('phase-status').textContent=group.duration?`${group.title} · ${sample.time.toFixed(2)} / ${group.duration.toFixed(2)} s · ${Number($('candidate').value)+1} of ${group.samples.length} samples${group.looping?' · looping source clip':''}`:'Editable rig in its authored rest view';
    $('previous').disabled=Number($('candidate').value)===0;
    $('next').disabled=Number($('candidate').value)===group.samples.length-1;
  }
  function behaviour(){
    const group=groups[Number($('behaviour').value)];$('candidate').replaceChildren();
    for(const [i,sample]of group.samples.entries()){
      const option=document.createElement('option');option.value=String(i);
      option.textContent=group.duration?`${sample.time.toFixed(2)} s`:group.id==='rest'?'Rest view':sample.entry.file;$('candidate').append(option);
    }
    $('candidate').value=String(Math.floor(group.samples.length/2));$('candidate').disabled=group.samples.length<2;select();
  }
  $('behaviour').value=String(Math.max(0,groups.findIndex(g=>g.id==='hop-step-study')));$('behaviour').disabled=false;
  $('behaviour').onchange=behaviour;$('candidate').onchange=select;
  for(const [id,offset]of [['previous',-1],['next',1]])$(id).onclick=()=>{
    $('candidate').value=String(Number($('candidate').value)+offset);select();
  };
  function snapshots(){
    mode='snapshots';playback.close();$('motion-review').hidden=true;$('snapshot-review').hidden=false;
    $('mode-motion').setAttribute('aria-pressed','false');$('mode-snapshots').setAttribute('aria-pressed','true');select();
  }
  fallback=()=>{if(token!==lifecycle)return;$('mode-motion').disabled=true;snapshots();};
  $('mode-snapshots').onclick=snapshots;
  function motion(){
    if(!admittedPackage||!master||!manifest||token!==lifecycle)return;
    $('mode-motion').disabled=true;
    mode='motion';$('preview').src='about:blank';$('snapshot-review').hidden=true;$('motion-review').hidden=false;
    $('mode-motion').setAttribute('aria-pressed','true');$('mode-snapshots').setAttribute('aria-pressed','false');
    playback.open({...admittedPackage,modelSha:master.sha256,manifest}).then(ready=>{
      if(token===lifecycle&&mode==='motion')$('mode-motion').disabled=ready!==true;
    });
  }
  $('mode-motion').onclick=motion;
  $('message').textContent=`Choose a behaviour, then inspect its sampled phases. ${clips.length} authored motion studies are available; additional species behaviours remain to be authored and reviewed.`;
  behaviour();
  if(admittedPackage&&master&&manifest)motion();
  else $('motion-status').textContent='No SHA-verified HMC/master pair is available for continuous source playback. Inspection snapshots remain available.';
}
function boot(){const expected=lifecycle+1;start().catch(error=>{if(expected===lifecycle)$('message').textContent=`Review unavailable: ${error.message}`;});}
window.addEventListener('pagehide',()=>{lifecycle++;indexAbort?.abort();playback.close();$('preview').src='about:blank';});
window.addEventListener('pageshow',event=>{if(event.persisted)boot();});
boot();
