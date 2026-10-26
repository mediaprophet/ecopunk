// Read-only audit of shipped inputs; writes only this audit's evidence.json.
// Run from any directory with Node 18+: node <this file>
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
const dir = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(dir, '../../..');
const sha = b => createHash('sha256').update(b).digest('hex');
const modelRoot = path.join(root, 'assets/models/corymbia-aparrerinja');
const manifest = JSON.parse(await fs.readFile(path.join(modelRoot, 'model-set.json'), 'utf8'));
const entries = manifest.models.flatMap(m => m.components.map(c => ({model:m.id,...c})));
const checks = await Promise.all(entries.map(async e => {
  try {
    const b = await fs.readFile(path.join(modelRoot,e.file));
    return {model:e.model,file:e.file,declaredBytes:e.bytes,actualBytes:b.length,
      declaredSha256:e.sha256,actualSha256:sha(b),digestMatches:sha(b)===e.sha256,sizeMatches:b.length===e.bytes};
  } catch (error) { return {model:e.model,file:e.file,error:error.message}; }
}));
const published = [];
for (const file of ['index.html','viewer/index.html','viewer/pkg/ecopunk_wasm.js','viewer/pkg/ecopunk_wasm_bg.wasm']) {
  const local = await fs.readFile(path.join(root,file));
  const record = {file,localBytes:local.length,localSha256:sha(local)};
  try {
    const response = await fetch('https://mediaprophet.github.io/ecopunk/'+file,{signal:AbortSignal.timeout(30000)});
    record.status=response.status;
    if(response.ok) {
      const b=Buffer.from(await response.arrayBuffer());
      Object.assign(record,{publishedBytes:b.length,publishedSha256:sha(b),matches:sha(b)===sha(local)});
    }
  } catch(error) { record.error=error.message; }
  published.push(record);
}
const mismatches=checks.filter(c=>c.error||!c.digestMatches||!c.sizeMatches);
const result={capturedAt:new Date().toISOString(),scope:'Byte identity and manifest references only; not container or visual acceptance',
  published,modelSet:{modelCount:manifest.models.length,componentReferences:entries.length,
    uniqueFiles:new Set(entries.map(e=>e.file)).size,lodGroups:manifest.lodGroups.length,
    readmeBuildScriptExists:await fs.access(path.join(modelRoot,'build.mjs')).then(()=>true,()=>false),
    mismatchedReferences:mismatches.length,uniqueDigestMismatchFiles:[...new Set(checks.filter(c=>c.digestMatches===false).map(c=>c.file))],
    uniqueSizeMismatchFiles:[...new Set(checks.filter(c=>c.sizeMatches===false).map(c=>c.file))],checks}};
await fs.writeFile(path.join(dir,'evidence.json'),JSON.stringify(result,null,2)+'\n');
console.log(JSON.stringify({published,modelSet:{...result.modelSet,checks:undefined}},null,2));
