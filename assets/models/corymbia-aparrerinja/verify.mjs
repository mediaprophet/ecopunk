import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import init, { asset_verify_10d, entity_verify_10d } from '../../../../game-demo/web/pkg/rolling_commons_shell.js';

const here = dirname(fileURLToPath(import.meta.url));
const manifest = JSON.parse(await readFile(join(here, 'model-set.json'), 'utf8'));
await init({ module_or_path: await readFile(resolve(here, '../../../../game-demo/web/pkg/rolling_commons_shell_bg.wasm')) });
assert.equal(manifest.schema, 'ecopunk-model-review-v1');
assert.equal(manifest.taxon, 'urn:qualia:taxon:corymbia_aparrerinja');
assert.equal(manifest.models.length, 25);
for (const [file, digest, length] of [
  [manifest.graph.source, manifest.graph.sourceSha256, manifest.graph.sourceBytes],
  [manifest.graph.volume, manifest.graph.volumeSha256, manifest.graph.volumeBytes]
]) {
  const bytes = await readFile(join(here, file));
  assert.equal(bytes.length, length);
  assert.equal(createHash('sha256').update(bytes).digest('hex'), digest);
}
const graph = JSON.parse(await readFile(join(here, manifest.graph.source), 'utf8'));
assert.equal(graph.claims.length, manifest.graph.claims);
assert.equal(graph.gaps.length, manifest.graph.gaps);
assert.equal((await readFile(join(here, manifest.graph.volume))).subarray(0, 4).toString(), 'Q42\0');
for (const animation of manifest.animations) {
  assert.ok(animation.frames.length >= 2);
  assert.ok(animation.frames.every(frame => manifest.models.some(model => model.id === frame.model)));
}
const seen = new Set();
let components = 0;
for (const model of manifest.models) {
  assert.ok(model.components.length >= 2);
  for (const part of model.components) {
    assert.match(part.file, /^10d\/[a-z0-9-]+\.10d$/);
    assert.ok(!seen.has(part.file), `duplicate ${part.file}`);
    seen.add(part.file);
    const bytes = await readFile(join(here, part.file));
    assert.equal(bytes.length, part.bytes);
    assert.equal(createHash('sha256').update(bytes).digest('hex'), part.sha256);
    const entity = entity_verify_10d(bytes);
    assert.equal(entity.valid, true, `${part.file}: ${JSON.stringify(entity.violations)}`);
    const mesh = asset_verify_10d(bytes);
    assert.ok(mesh.vertices > 0 && mesh.triangles > 0, `${part.file}: empty geometry`);
    assert.equal(mesh.vertices, part.vertices);
    assert.equal(mesh.triangles, part.triangles);
    components++;
  }
}
console.log(`Verified ${manifest.models.length} ecopunk models and ${components} QualiaDB .10d components`);
