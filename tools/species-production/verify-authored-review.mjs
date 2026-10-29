// Executes the actual WASM source/session/export API without a native host build.
// This is geometry/source verification, not browser rendering or human acceptance.
import {readFile,writeFile,mkdir,readdir} from 'node:fs/promises';
import {fileURLToPath,pathToFileURL} from 'node:url';
import path from 'node:path';
import {createHash} from 'node:crypto';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const evidence=path.join(root,'docs/qa/implementation-2026-10-29/species-production-round1');
const names=(await readdir(evidence)).filter(n=>/^wasm-build-\d+\.json$/.test(n)).sort();
const build=JSON.parse(await readFile(path.join(evidence,names.at(-1)),'utf8'));
if(build.exit_code!==0){
 const recovery=JSON.parse(await readFile(path.join(evidence,names.at(-1).replace('wasm-build-','wasm-packaging-')),'utf8'));
 if(recovery.exit_code!==0||recovery.build_record!==names.at(-1))throw Error('Latest WASM packaging has not succeeded');
 for(const [name,expected] of Object.entries(recovery.outputs)){
  if(createHash('sha256').update(await readFile(path.join(build.package,name))).digest('hex')!==expected)throw Error('Recovered package changed');
 }
}
const wasm=await import(pathToFileURL(path.join(build.package,'ecopunk_wasm.js')));
const binary=await readFile(path.join(build.package,'ecopunk_wasm_bg.wasm'));
await wasm.default({module_or_path:binary});
const report={scope:'Actual WASM source validation/export; no render or acceptance claim',
  wasm_sha256:createHash('sha256').update(binary).digest('hex'),build_record:names.at(-1),cases:[]};
let failed=false;
for(const [subject,species,filename] of [
  [0,'white-gum-review-r1','white-gum-r2.vibe'],[1,'swamp-wallaby-review-r1','swamp-wallaby-r3.vibe']]){
 const session=new wasm.SpeciesReviewSession(subject);
 try{
  const source=await readFile(path.join(root,'assets/production-studies',species,'authoring',filename),'utf8');
  const result=session.submit(source,0);
  if(session.revision()!==1||!result.bytes.length||!result.bundle.length)throw Error('Missing first source/export result');
  const before=session.revision();let invalidRefused=false,staleRefused=false;
  try{session.submit('species_asset = { invalid_schema: true };',before)}catch{invalidRefused=true;}
  if(!invalidRefused||session.revision()!==before)throw Error('Invalid source advanced the session');
  try{session.submit('= species_asset',0)}catch{staleRefused=true;}
  if(!staleRefused||session.revision()!==before)throw Error('Stale source advanced the session');
  const repeated=session.submit('= species_asset',before);
  if(repeated.metadata.vertex_count!==result.metadata.vertex_count||session.revision()!==2)throw Error('Accepted binding was not retained');
  const directory=path.join(root,'assets/production-studies',species,'exports');await mkdir(directory,{recursive:true});
  const model=path.join(directory,result.metadata.id+'.10d'),bundle=path.join(directory,result.metadata.id+'.hmc');
  await writeFile(model,result.bytes);await writeFile(bundle,result.bundle);
  const entry={subject,status:'passed',metadata:result.metadata,invalid_schema_refused:invalidRefused,
    stale_revision_refused:staleRefused,accepted_binding_retained:true,model,bundle,
    model_sha256:createHash('sha256').update(result.bytes).digest('hex'),bundle_sha256:createHash('sha256').update(result.bundle).digest('hex')};
  report.cases.push(entry);console.log(JSON.stringify({subject,status:entry.status,vertices:entry.metadata.vertex_count,triangles:entry.metadata.triangle_count}));
 }catch(error){failed=true;report.cases.push({subject,status:'failed',error:String(error)});console.error(subject,String(error));}
 finally{session.free();}
}
report.captured_utc=new Date().toISOString();
await writeFile(path.join(evidence,'authored-wasm-verification.json'),JSON.stringify(report,null,2)+'\n');
if(failed)process.exitCode=1;
