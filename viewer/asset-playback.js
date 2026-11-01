// Browser scheduling only: the Rust source session owns time, pose and binding.
import { loadVerifiedEngine, sha256 } from './showasset-engine.js';

const MAX_PACKAGE = 64 * 1024 * 1024, MAX_MODEL = 16 * 1024 * 1024;
const decimal = value => typeof value === 'string' && /^\d{1,20}$/.test(value);
const $ = id => document.getElementById(id);

export function createAssetPlayback(root, fallback) {
  const canvas = $('motion-canvas'), panel = $('motion-stage');
  let generation = 0, live, serial = Promise.resolve(), raf = 0;
  let lastRaf = null, dirty = false, frameDeadline = 0, lastStatus = 0;
  let yaw = .7, pitch = .12, distance = 3, target = [0, 0, 0], bounds;
  const pointers = new Map(); let pinch;

  function status(text) { $('motion-status').textContent = text; }
  function enable(ready) {
    for (const id of ['motion-play', 'motion-pause', 'motion-clip', 'motion-seek', 'motion-loop', 'motion-rate', 'motion-fit']) $(id).disabled = !ready;
  }
  function release(owner) {
    try { owner?.session?.cancel(); } catch {}
    try { owner?.portal?.free(); } catch {}
    try { owner?.session?.free(); } catch {}
  }
  function retire() {
    if (raf) cancelAnimationFrame(raf); raf = 0; lastRaf = null; dirty = false;
    release(live); live = undefined; enable(false); canvas.hidden = true;
    for (const id of pointers.keys()) { try { canvas.releasePointerCapture(id); } catch {} }
    pointers.clear(); pinch = undefined;
    bounds = undefined; $('motion-identity').textContent = ''; $('motion-progress').textContent = 'Paused';
  }
  function refuse(error) {
    retire();
    status(`Continuous HMC playback unavailable: ${String(error?.message || error).slice(0, 700)}. Static inspection remains available.`);
    fallback();
  }
  function nativeState() {
    const state = live.session.state();
    if (!decimal(state.position_tick) || !decimal(state.duration_tick) || !decimal(state.sequence)
        || !decimal(state.source_generation) || !decimal(state.transport_epoch)
        || BigInt(state.duration_tick) === 0n || BigInt(state.position_tick) > BigInt(state.duration_tick)
        || state.runtime_retired === true || typeof state.playing !== 'boolean') throw Error('Native playback context is unavailable');
    return state;
  }
  function update(state) {
    const fraction = Number(BigInt(state.position_tick) * 1000n / BigInt(state.duration_tick));
    $('motion-seek').value = String(fraction);
    $('motion-progress').textContent = `${(fraction / 10).toFixed(1)}% · ${state.playing ? 'playing' : 'paused'}`;
    const ended = !state.looping && state.position_tick === state.duration_tick;
    $('motion-play').disabled = state.playing || ended;
    $('motion-pause').disabled = !state.playing;
    $('motion-loop').checked = state.looping === true;
    $('motion-clip').value = state.clip;
  }
  function requestFrame() { if (live && !raf) raf = requestAnimationFrame(frame); }
  function pendingFrame() {
    if (!dirty) {
      dirty = true; frameDeadline = performance.now() + 15000;
    }
  }
  function changed() {
    pendingFrame();
    if (live) status('Waiting for a fresh frame of the current source pose/view…');
    if (live) requestFrame();
  }
  function fit() {
    if (!bounds) return;
    target = bounds.min.map((v, i) => (v + bounds.max[i]) * .5);
    const aspect = canvas.width / canvas.height, s = Math.sin(yaw), c = Math.cos(yaw), sp = Math.sin(pitch), cp = Math.cos(pitch);
    let required = .1;
    // Camera fitting of pinned producer bounds only; no mesh interpretation.
    for (const x of [bounds.min[0], bounds.max[0]]) for (const y of [bounds.min[1], bounds.max[1]]) for (const z of [bounds.min[2], bounds.max[2]]) {
      const a = x - target[0], b = y - target[1], d = z - target[2];
      const right = c * a - s * d, up = -sp * s * a + cp * b - sp * c * d, depth = cp * s * a + sp * b + cp * c * d;
      required = Math.max(required, depth + Math.abs(right) / (.41421357 * aspect), depth + Math.abs(up) / .41421357);
    }
    distance = required * 1.25; changed();
  }
  function resize() {
    const box = panel.getBoundingClientRect();
    const ratio = Math.min(devicePixelRatio || 1, 2, 2048 / Math.max(1, box.width, box.height), Math.sqrt(3_000_000 / Math.max(1, box.width * box.height)));
    const width = Math.max(1, Math.round(box.width * ratio)), height = Math.max(1, Math.round(box.height * ratio));
    if (canvas.width !== width || canvas.height !== height) { canvas.width = width; canvas.height = height; fit(); }
  }
  const resizeObserver = new ResizeObserver(resize); resizeObserver.observe(panel);

  function frame(time) {
    raf = 0;
    if (!live) return;
    try {
      let state = nativeState();
      const elapsed = state.playing && lastRaf !== null ? time - lastRaf : 0;
      lastRaf = time;
      if (!Number.isFinite(elapsed) || elapsed < 0 || elapsed > 1000) {
        live.session.pause(); live.session.present_ticks(live.portal, '0'); lastRaf = null; pendingFrame();
        state = nativeState();
        status('Playback paused after a browser scheduling interruption. Resume explicitly; no skipped interval was synthesized.');
      } else {
        if (live.session.present_elapsed_ms(live.portal, elapsed)) pendingFrame();
      }
      const binding = live.session.binding_state(live.portal);
      const submission = live.session.last_submission();
      state = nativeState();
      if (!binding || !submission || binding.source_generation !== state.source_generation
          || binding.transport_epoch !== state.transport_epoch || submission.source_generation !== binding.source_generation
          || submission.mesh_generation !== binding.mesh_generation || submission.transport_epoch !== binding.transport_epoch
          || submission.sequence !== state.sequence) throw Error('Source, pose and mesh context no longer match');
      const before = Number(live.portal.body_render_receipt().frames_presented);
      live.portal.set_camera_target(yaw, pitch, distance, ...target);
      // Core's external source binding disables its generic SKL1 player. Zero
      // delta renders this pose; only present_elapsed_ms advances the source.
      live.portal.tick(canvas, 0);
      const rendered = live.portal.body_render_receipt(), after = Number(rendered.frames_presented);
      if (!Number.isSafeInteger(before) || !Number.isSafeInteger(after) || after < before) throw Error('Invalid renderer frame counter');
      if (after > before) {
        if (rendered.success !== true || rendered.uploaded !== true
            || Number(rendered.vertex_count) !== live.admission.vertex_count
            || Number(rendered.index_count) !== live.admission.total_triangles * 3) throw Error('Complete posed mesh was not presented');
        dirty = false;
        if (time - lastStatus > 500 || !state.playing || $('motion-status').textContent.startsWith('Waiting')) {
          const ended = !state.looping && state.position_tick === state.duration_tick;
          status(`Current ${state.playing ? 'moving' : 'paused'} source pose rendered by ${rendered.renderer}.${ended ? ' Clip ended; seek before playing again.' : ''} Visual quality and human acceptance remain pending; audio is separate.`);
          lastStatus = time;
        }
      } else if (dirty) {
        status('Waiting for a fresh frame of the current source pose/view…');
        if (time > frameDeadline) throw Error('No fresh posed frame was reported within 15 seconds');
      }
      update(state);
      if (state.playing || dirty) requestFrame(); else lastRaf = null;
    } catch (error) { refuse(error); }
  }
  function command(action) {
    if (!live) return;
    let committed = false;
    try {
      action(live.session); committed = true; lastRaf = null;
      // Publish the new epoch/seek pose through Rust before querying its binding.
      live.session.present_ticks(live.portal, '0');
      update(nativeState()); changed();
    } catch (error) {
      if (committed) { refuse(error); return; }
      try { update(nativeState()); status(`Control refused: ${String(error?.message || error).slice(0, 400)}. Current playback remains admitted; seek or choose a supported option.`); }
      catch (failure) { refuse(failure); }
    }
  }
  $('motion-play').onclick = () => command(session => session.play());
  $('motion-pause').onclick = () => command(session => session.pause());
  $('motion-clip').onchange = () => command(session => session.select_clip($('motion-clip').value));
  $('motion-loop').onchange = () => command(session => session.set_looping($('motion-loop').checked));
  $('motion-rate').onchange = () => command(session => {
    const rate = { 'half': [1, 2], 'normal': [1, 1], 'double': [2, 1] }[$('motion-rate').value];
    if (!rate) throw Error('Unknown playback rate'); session.set_rate(...rate);
  });
  $('motion-seek').onchange = () => command(session => {
    const value = $('motion-seek').value;
    if (!/^\d{1,4}$/.test(value) || Number(value) > 1000) throw Error('Seek selection is invalid');
    // A UI seek proposal in the native timebase; never an accumulated JS clock.
    session.seek_ticks((BigInt(nativeState().duration_tick) * BigInt(value) / 1000n).toString());
  });
  $('motion-fit').onclick = fit;
  canvas.onpointerdown = event => {
    if (!live || pointers.size >= 2) return;
    canvas.focus(); pointers.set(event.pointerId, [event.clientX, event.clientY]); canvas.setPointerCapture(event.pointerId); pinch = undefined;
  };
  canvas.onpointermove = event => {
    const old = pointers.get(event.pointerId); if (!live || !old) return;
    pointers.set(event.pointerId, [event.clientX, event.clientY]);
    if (pointers.size === 1) { yaw += (event.clientX - old[0]) * .006; pitch = Math.max(-1.3, Math.min(1.3, pitch + (event.clientY - old[1]) * .006)); }
    else { const [a, b] = [...pointers.values()]; const span = Math.hypot(a[0] - b[0], a[1] - b[1]); if (pinch && span > 0) distance = Math.max(.12, Math.min(1000, distance * pinch / span)); pinch = span; }
    changed();
  };
  canvas.onpointerup = canvas.onpointercancel = event => { pointers.delete(event.pointerId); pinch = undefined; };
  canvas.addEventListener('wheel', event => { if (!live) return; event.preventDefault(); distance = Math.max(.12, Math.min(1000, distance * Math.exp(event.deltaY * .001))); changed(); }, { passive: false });
  canvas.onkeydown = event => {
    if (!live || !['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', '+', '-', '='].includes(event.key)) return;
    event.preventDefault();
    if (event.key === 'ArrowLeft') yaw -= .1; if (event.key === 'ArrowRight') yaw += .1;
    if (event.key === 'ArrowUp') pitch = Math.max(-1.3, pitch - .1); if (event.key === 'ArrowDown') pitch = Math.min(1.3, pitch + .1);
    if (event.key === '+' || event.key === '=') distance = Math.max(.12, distance * .8); if (event.key === '-') distance = Math.min(1000, distance * 1.25);
    changed();
  };

  async function open(request) {
    const token = ++generation; retire(); resizeObserver.observe(panel); enable(false); status('Checking the pinned engine and admitting the exact HMC source in Rust…');
    const operation = serial.catch(() => {}).then(async () => {
      let staged;
      try {
        if (token !== generation) return;
        const engine = await loadVerifiedEngine(root);
        if (token !== generation) return;
        const Type = engine.wasm.SpeciesProductionPlaybackSession;
        if (typeof Type !== 'function') throw Error('The SHA-pinned retained engine lacks the current Rust production-playback exports; a coordinator-verified WASM rebuild is required');
        for (const name of ['reopen', 'current_asset', 'bind_portal', 'binding_state', 'play', 'pause', 'select_clip', 'seek_ticks', 'set_rate', 'set_looping', 'present_ticks', 'present_elapsed_ms', 'last_submission', 'state', 'cancel']) {
          if (typeof Type.prototype[name] !== 'function') throw Error(`Pinned engine lacks ${name}`);
        }
        if (!(request.bytes instanceof Uint8Array) || !request.bytes.length || request.bytes.length > MAX_PACKAGE
            || await sha256(request.bytes) !== request.packageSha) throw Error('Pinned HMC bytes differ');
        if (token !== generation) return;
        staged = { session: new Type(), engine: engine.receipt };
        staged.session.reopen(request.bytes);
        const asset = staged.session.current_asset();
        if (!(asset.model instanceof Uint8Array) || !asset.model.length || asset.model.length > MAX_MODEL
            || !(asset.package instanceof Uint8Array) || asset.package.length > MAX_PACKAGE) throw Error('Rust candidate projection exceeds this browser profile');
        const [modelSha, packageSha] = await Promise.all([sha256(asset.model), sha256(asset.package)]);
        if (token !== generation) return;
        if (modelSha !== request.modelSha || packageSha !== request.packageSha) throw Error('Reopened source/model/package differs from the selected artifact');
        const min = request.manifest?.bounds_min_m, max = request.manifest?.bounds_max_m;
        if (![min, max].every(v => Array.isArray(v) && v.length === 3 && v.every(n => Number.isFinite(n) && Math.abs(n) <= 1e8))
            || min.some((n, i) => n > max[i]) || max.every((n, i) => n === min[i])) throw Error('Pinned authored-frame view bounds are unavailable');
        const gpu = await engine.wasm.init_webgpu(canvas);
        if (token !== generation) return;
        if (!gpu) throw Error('This browser did not admit the WebGPU source-frame path');
        staged.portal = new engine.wasm.QualiaPortal(canvas);
        staged.portal.set_preserve_authored_frame(true);
        staged.portal.set_tensor_projection_enabled(false); staged.portal.set_ambient_enabled(false);
        staged.portal.set_sky_preset(1);
        // The sky preset also replaces lighting; apply the review key/fill last.
        staged.portal.set_lighting(.4, .85, .5, 1.5, .35);
        staged.portal.set_screen_space_ao_enabled(true);
        staged.portal.set_screen_space_ao(.05, .75, .001);
        staged.portal.set_screen_space_ao_sample_count(12);
        staged.portal.set_shadows_enabled(true);
        staged.admission = staged.portal.load_assets_colored([{ id: 'hmc-source:' + modelSha, bytes: asset.model, r: 1, g: 1, b: 1, a: 1 }]);
        const a = staged.admission;
        if (Number(a.assets_loaded) !== 1 || Number(a.assets_refused) !== 0 || !Number.isSafeInteger(a.vertex_count) || a.vertex_count <= 0
            || !Number.isSafeInteger(a.total_triangles) || a.total_triangles <= 0) throw Error('Rust refused complete authored-model upload');
        staged.session.bind_portal(staged.portal);
        staged.session.pause(); staged.session.present_ticks(staged.portal, '0');
        if (!staged.session.binding_state(staged.portal)) throw Error('Rust source-frame binding is absent');
        const state = staged.session.state();
        if (!Array.isArray(state.clips) || !state.clips.length || state.clips.length > 32
            || state.clips.some(s => typeof s !== 'string' || !s || s.length > 128)) throw Error('No bounded native clip inventory');
        if (token !== generation) return;
        $('motion-clip').replaceChildren();
        for (const clip of state.clips) { const option = document.createElement('option'); option.value = option.textContent = clip; $('motion-clip').append(option); }
        live = staged; staged = undefined;
        bounds = { min, max }; yaw = .7; pitch = .12; canvas.hidden = false;
        $('motion-rate').value = 'normal'; enable(true); update(nativeState()); resize(); fit(); changed();
        $('motion-identity').textContent = `HMC SHA-256 ${packageSha} · model ${modelSha} · engine ${engine.receipt.revision}. Rust reopens retained source/Q42/SKL1; texture residency, biological truth and acceptance remain separate.`;
        status('Exact HMC source and complete authored pose admitted. Paused; press Play for the Rust-owned motion clock.');
        return true;
      } catch (error) { if (token === generation) refuse(error); return false; }
      finally { release(staged); }
    });
    serial = operation; return operation;
  }
  function close() { generation++; retire(); }
  document.addEventListener('visibilitychange', () => { if (document.hidden && live) command(session => session.pause()); });
  window.addEventListener('pagehide', () => { close(); resizeObserver.disconnect(); });
  return { open, close };
}
