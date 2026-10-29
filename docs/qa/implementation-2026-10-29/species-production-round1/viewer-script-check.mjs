
const canvas=document.getElementById('model-canvas'), image=document.getElementById('saved-image');
const status=document.getElementById('status'), metadata=document.getElementById('metadata'),caption=document.getElementById('caption');
let wasm,portal,initializing,active=false,subject=-1,yaw=.7,pitch=.12,distance=2.6,target=[0,0,0],downloadUrl,currentMeta,admission,awaitingFrame=false;
let halfBounds=[.85,.85,.85],modelCenter=[0,0,0],modelScale=1;
const choose=id=>document.querySelectorAll('.model-buttons button').forEach(b=>b.setAttribute('aria-pressed',String(b.id===id)));
document.getElementById('reload').onclick=()=>location.reload();
function showImage(id,path,alt,text){active=false;image.src=path;image.alt=alt;image.hidden=false;canvas.hidden=true;status.textContent=text;caption.textContent=alt;metadata.textContent='';document.getElementById('download-model').hidden=true;document.getElementById('details-link').href=id==='scene-reference'?'../assets/demo-rebuild-2026-10-29/previews/site-00-talbingo.png':'../assets/production-studies/eucalypt-spray-r1/revisions/2026-10-29/dossier/README.md';choose(id);}
document.getElementById('saved-study').onclick=()=>showImage('saved-study','../assets/production-studies/eucalypt-spray-r1/revisions/2026-10-29/hero.png','Earlier illustrative eucalypt branch — saved output, not a completed white-gum species.','Showing the saved branch image. New whole-tree candidate is a separate revision.');
document.getElementById('scene-reference').onclick=()=>showImage('scene-reference','../assets/demo-rebuild-2026-10-29/previews/site-00-talbingo.png','Retained Talbingo render identified by Timothy as the better scene.','Visual comparison reference; preserved without alteration.');
function resize(){const d=Math.min(devicePixelRatio||1,2);canvas.width=Math.max(1,Math.round(canvas.clientWidth*d));canvas.height=Math.max(1,Math.round(canvas.clientHeight*d));}
window.addEventListener('resize',resize);
async function ensureEngine(){
 if(portal)return; if(initializing)return initializing;
 initializing=(async()=>{
  const manifest=await fetch('./model-review-manifest.json',{cache:'no-store'}).then(r=>{if(!r.ok)throw Error('Review manifest unavailable');return r.json()});
  const base=manifest.package_base; wasm=await import(`${base}ecopunk_wasm.js?v=${manifest.revision}`);
  await wasm.default({module_or_path:`${base}ecopunk_wasm_bg.wasm?v=${manifest.revision}`});
  canvas.hidden=false;image.hidden=true;resize();
  let gpu=false;try{gpu=await wasm.init_webgpu(canvas)}catch{} if(!gpu)wasm.init_webgl2(canvas);
  portal=new wasm.QualiaPortal(canvas);portal.set_preserve_authored_frame(false);
  portal.set_tensor_projection_enabled(false);portal.set_ambient_enabled(false);
  portal.set_environment_probe_data(new Float32Array(0),new Float32Array([0,0,0]));
  portal.set_lighting(.4,.85,.5,1.5,.35);portal.set_exposure_compensation(.05);
  portal.set_sky_preset(1);portal.set_shadows_enabled(true);portal.set_screen_space_ao_enabled(true);portal.set_screen_space_ao(.35,.45,.018);
  requestAnimationFrame(frame);
 })();try{await initializing}catch(e){initializing=null;throw e}
}
function load(bytes,id,meta,index){
 const receipt=portal.load_assets_colored([{id,bytes,r:1,g:1,b:1,a:1}]);
 if(receipt.assets_refused||receipt.assets_loaded!==1)throw Error('Model was not fully admitted for rendering');
 subject=index;currentMeta=meta;admission=receipt;awaitingFrame=true;
 modelCenter=meta.bounds_min_m.map((v,i)=>(v+meta.bounds_max_m[i])/2);
 modelScale=1.7/Math.max(...meta.bounds_min_m.map((v,i)=>meta.bounds_max_m[i]-v),1e-6);
 halfBounds=meta.bounds_min_m.map((v,i)=>(meta.bounds_max_m[i]-v)*modelScale/2);
 canvas.hidden=false;image.hidden=true;resize();active=true;setView('three-quarter');
 metadata.textContent=`${meta.taxon}\n${Number(receipt.vertex_count).toLocaleString()} vertices · ${Number(receipt.total_triangles).toLocaleString()} source triangles\n${receipt.renderer} · ${receipt.material_pipeline?'PBR materials':'colour fallback'}\nSource ${meta.source_sha256.slice(0,16)}`;
 if(downloadUrl)URL.revokeObjectURL(downloadUrl);downloadUrl=URL.createObjectURL(new Blob([bytes],{type:'application/octet-stream'}));
 const link=document.getElementById('download-model');link.href=downloadUrl;link.download=id+'.10d';link.hidden=false;
 document.getElementById('save-record').hidden=false;
}
async function showCandidate(index){
 status.textContent='Loading the species model…';
 try{await ensureEngine();
  if(typeof wasm.build_species_review_asset!=='function')throw Error('The new species WASM build is still in progress. The saved branch and scene reference remain available.');
  const result=wasm.build_species_review_asset(index);load(result.bytes,result.metadata.id,result.metadata,index);
  choose(index===0?'white-gum':index===2?'white-gum-foliage':'wallaby');status.textContent='Model loaded · waiting for the first rendered frame.';
  caption.textContent=result.metadata.title+' · static external form; species package remains incomplete.';
  document.getElementById('details-link').href=result.metadata.dossier;
 }catch(e){status.textContent=String(e.message||e);if(!active){image.hidden=false;canvas.hidden=true;}}
}
document.getElementById('white-gum').onclick=()=>showCandidate(0);document.getElementById('wallaby').onclick=()=>showCandidate(1);
document.getElementById('white-gum-foliage').onclick=()=>showCandidate(2);
function fittedDistance(){const sy=Math.sin(yaw),cy=Math.cos(yaw),sp=Math.sin(pitch),cp=Math.cos(pitch),aspect=canvas.width/canvas.height;let fit=.1;for(const x of [-halfBounds[0],halfBounds[0]])for(const y of [-halfBounds[1],halfBounds[1]])for(const z of [-halfBounds[2],halfBounds[2]]){const right=cy*x-sy*z,up=-sp*sy*x+cp*y-sp*cy*z,depth=cp*sy*x+sp*y+cp*cy*z;fit=Math.max(fit,depth+Math.abs(right)/(.41421357*aspect),depth+Math.abs(up)/.41421357);}return fit*1.12;}
function setView(view){target=[0,0,0];pitch=.12;yaw=.7;if(view==='front'){yaw=0;pitch=0}if(view==='side'){yaw=Math.PI/2;pitch=0}distance=fittedDistance();if(view==='detail'){yaw=.5;pitch=.08;if(subject===1){const head=[0,1.34*.78-.06+.01,.32*.88];target=head.map((v,i)=>(v-modelCenter[i])*modelScale);distance=.65;}else{target=[0,halfBounds[1]*.55,0];distance=fittedDistance()*.48;}}}
document.querySelectorAll('[data-view]').forEach(b=>b.onclick=()=>setView(b.dataset.view));
document.getElementById('zoom-in').onclick=()=>distance=Math.max(.15,distance*.8);document.getElementById('zoom-out').onclick=()=>distance=Math.min(8,distance*1.25);
let drag;canvas.onpointerdown=e=>{drag=[e.clientX,e.clientY];canvas.setPointerCapture(e.pointerId)};canvas.onpointermove=e=>{if(!drag)return;yaw+=(e.clientX-drag[0])*.006;pitch=Math.max(-1.2,Math.min(1.3,pitch+(e.clientY-drag[1])*.006));drag=[e.clientX,e.clientY]};canvas.onpointerup=canvas.onpointercancel=()=>drag=null;
canvas.addEventListener('wheel',e=>{e.preventDefault();distance=Math.max(.15,Math.min(8,distance*Math.exp(e.deltaY*.001)))},{passive:false});
let last=performance.now();function frame(t){if(active){try{portal.set_camera_target(yaw,pitch,distance,...target);portal.tick(canvas,Math.min(50,t-last));if(awaitingFrame){const rendered=portal.body_render_receipt();if(rendered.success){awaitingFrame=false;const full=admission.material_pipeline&&rendered.vertex_count===currentMeta.vertex_count&&rendered.index_count===currentMeta.triangle_count*3;status.textContent=full?'Rendered candidate · awaiting coordinator review and your decision.':'Rendered with a reduced or colour fallback · full material review remains pending.';}}}catch(e){active=false;status.textContent='Render failed: '+String(e)}}last=t;requestAnimationFrame(frame)}
document.getElementById('save-record').onclick=()=>{if(!active)return;const record={model:currentMeta,admission,render:portal.body_render_receipt(),camera:{yaw,pitch,distance,target},canvas:{width:canvas.width,height:canvas.height},captured_utc:new Date().toISOString(),human_acceptance:'pending'};const a=document.createElement('a');a.href=URL.createObjectURL(new Blob([JSON.stringify(record,null,2)],{type:'application/json'}));a.download=currentMeta.id+'-review.json';a.click();setTimeout(()=>URL.revokeObjectURL(a.href),1000)};
document.getElementById('save-image').onclick=()=>{if(!active){const a=document.createElement('a');a.href=image.src;a.download=image.src.split('/').pop();a.click();return;}portal.tick(canvas,0);canvas.toBlob(blob=>{if(!blob){status.textContent='Image capture unavailable';return;}const a=document.createElement('a');a.href=URL.createObjectURL(blob);a.download=(subject===0?'white-gum':subject===1?'swamp-wallaby':'white-gum-foliage')+'-review.png';a.click();setTimeout(()=>URL.revokeObjectURL(a.href),1000)},'image/png')};
