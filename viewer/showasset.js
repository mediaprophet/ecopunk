import { HISTORICAL, ENGINE, sha256, expectedDigest, resolveAssetUrl, fetchBounded, loadVerifiedEngine, candidateEntries } from "./showasset-engine.js";

const root = new URL("../", import.meta.url);
// Embedded review keeps the same verified loader, camera and evidence path.
document.body.classList.toggle("embedded-review",new URLSearchParams(location.search).get("embed")==="1");
const el = id => document.getElementById(id);
const canvas = el("canvas"), status = el("status");
const MAX_INPUT = 64 * 1024 * 1024, MAX_MESH = 16 * 1024 * 1024;
let generation = 0, controller, portal, portalReady, engine, current, active = false, fileUrl;
let yaw = .7, pitch = .12, distance = 3, viewVersion = 0, lastTime = performance.now(), firstFrame = false, frameDeadline = 0;
let presented, captureBusy = false;

function message(text, error = false) { status.textContent = text; status.dataset.error = String(error); }
function controls(ready, evidence = false) {
  document.querySelectorAll("[data-view],#zoom-in,#zoom-out").forEach(b => b.disabled = !ready);
  el("save-image").disabled = el("save-record").disabled = !evidence || captureBusy;
}
function retire(text) {
  active = false; current = undefined; firstFrame = false; presented = undefined; viewVersion++;
  for (const id of pointers.keys()) { try { if (canvas.hasPointerCapture(id)) canvas.releasePointerCapture(id); } catch {} }
  pointers.clear(); pinch = undefined;
  controls(false); canvas.hidden = true; el("placeholder").hidden = false;
  el("placeholder").querySelector("p").textContent = text;
  el("render-label").textContent = "No current rendered evidence";
  el("download").hidden = true; el("copy-link").disabled = true;
  el("facts").replaceChildren(); el("historical-links").hidden = true;
  if (fileUrl) { URL.revokeObjectURL(fileUrl); fileUrl = undefined; }
}
function evidence(value) { el("receipt").textContent = JSON.stringify(value, null, 2); }
function fact(name, value) {
  const dt = document.createElement("dt"), dd = document.createElement("dd");
  dt.textContent = name; dd.textContent = String(value); el("facts").append(dt, dd);
}
function resize() {
  const box = el("viewport").getBoundingClientRect();
  const ratio = Math.min(devicePixelRatio || 1, 2, 2048 / Math.max(box.width, box.height), Math.sqrt(3_000_000 / Math.max(1, box.width * box.height)));
  const width = Math.max(1, Math.round(box.width * ratio)), height = Math.max(1, Math.round(box.height * ratio));
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width; canvas.height = height; viewVersion++;
    if (active) { fit(); controls(true, false); }
  }
}
new ResizeObserver(resize).observe(el("viewport"));
function fit() {
  // Rust's existing normalized orbit frame is bounded by a 1.7-wide cube.
  // This fits that display cube, without reading or interpreting mesh bytes.
  const aspect = canvas.width / canvas.height, s = Math.sin(yaw), c = Math.cos(yaw), sp = Math.sin(pitch), cp = Math.cos(pitch);
  let result = .1;
  for (const x of [-.85, .85]) for (const y of [-.85, .85]) for (const z of [-.85, .85]) {
    const right = c * x - s * z, up = -sp * s * x + cp * y - sp * c * z, depth = cp * s * x + sp * y + cp * c * z;
    result = Math.max(result, depth + Math.abs(right) / (.41421357 * aspect), depth + Math.abs(up) / .41421357);
  }
  distance = result * 1.1; viewVersion++;
}
async function ensurePortal() {
  if (portalReady) return portalReady;
  portalReady = (async () => {
    engine = await loadVerifiedEngine(root); resize();
    let gpu = false; try { gpu = await engine.wasm.init_webgpu(canvas); } catch {}
    if (!gpu && !engine.wasm.init_webgl2(canvas)) throw Error("This browser did not admit WebGPU or WebGL2. No rendered evidence is available.");
    portal = new engine.wasm.QualiaPortal(canvas);
    portal.set_preserve_authored_frame(false);
    portal.set_tensor_projection_enabled(false); portal.set_ambient_enabled(false);
    portal.set_environment_probe_data(new Float32Array(0), new Float32Array([0, 0, 0]));
    portal.set_lighting(.4, .85, .5, 1.5, .35); portal.set_exposure_compensation(.05);
    portal.set_sky_preset(1); portal.set_shadows_enabled(true);
    portal.set_screen_space_ao_enabled(true); portal.set_screen_space_ao(.35, .45, .018);
  })().catch(error => {
    try { portal?.free(); } catch {}
    portal = undefined; portalReady = undefined; throw error;
  });
  return portalReady;
}
function pinnedLink(record) {
  const url = new URL(location.href); url.search = "";
  url.searchParams.set("asset", record.asset_url);
  url.searchParams.set("sha256", record.input_sha256);
  if (record.resource) { url.searchParams.set("resource", record.resource); url.searchParams.set("resource_sha256", record.mesh_sha256); }
  return url.href;
}
function snapshot(rendered) {
  return { ...current.record, built_engine: engine.receipt, admission: current.admission, rendered,
    camera: { yaw, pitch, distance, target: [0, 0, 0], display_frame: "Rust normalized orbit frame; authored source bytes retained unchanged." },
    canvas: { width: canvas.width, height: canvas.height }, captured_utc: new Date().toISOString(),
    last_image_capture: current.lastImageEvidence || null, human_acceptance: "pending" };
}
function currentRender() {
  const rendered = portal.body_render_receipt();
  if (rendered.success !== true || rendered.uploaded !== true || Number(rendered.frames_presented) < 1
      || Number(rendered.vertex_count) !== Number(current.admission.vertex_count)
      || Number(rendered.index_count) !== Number(current.admission.total_triangles) * 3) {
    throw Error("The current renderer has not presented the complete admitted mesh. No current image evidence is established.");
  }
  return rendered;
}
function renderFailure(error) {
  const failed = current ? { ...current.record, admission: current.admission, built_engine: engine?.receipt } : {};
  retire("The selected asset has no current verified frame.");
  message("Render refused: " + String(error?.message || error), true);
  evidence({ ...failed, render_error: String(error?.message || error).slice(0, 2000), human_acceptance: "pending" });
}
async function openFromUrl() {
  const token = ++generation; controller?.abort(); controller = new AbortController();
  retire("Loading and verifying the selected artifact…"); message("Checking artifact and retained engine…");
  el("resource-options").replaceChildren(); el("history").textContent = "Unreviewed URL candidate. Loading is not acceptance.";
  let partial = { schema: "ecopunk.asset-url-review/1", human_acceptance: "pending" };
  evidence(partial);
  try {
    const params = new URL(location.href).searchParams;
    const input = params.get("asset") || HISTORICAL.path + HISTORICAL.mesh;
    const url = resolveAssetUrl(input, root), isBundle = url.pathname.toLowerCase().endsWith(".hmc");
    const resource = params.get("resource") || "";
    if (resource.length > 512 || /[\x00-\x1f\x7f]/.test(resource)) throw Error("HMC resource key exceeds this review's explicit selection profile.");
    let expected = expectedDigest(params.get("sha256"), "File SHA-256");
    if (!expected && !params.has("asset")) expected = HISTORICAL.meshSha256;
    const expectedMesh = expectedDigest(params.get("resource_sha256"), "Resource SHA-256");
    if (!isBundle && (resource || expectedMesh)) throw Error("A loose .10d does not accept an HMC resource selector.");
    el("asset").value = url.pathname; el("expected-sha").value = expected || ""; el("resource").value = resource;
    partial = { ...partial, asset_url: url.pathname, resource: resource || null, expected_input_sha256: expected, expected_mesh_sha256: expectedMesh };
    const [bytes] = await Promise.all([fetchBounded(url, isBundle ? MAX_INPUT : MAX_MESH, controller.signal), ensurePortal()]);
    if (token !== generation) return;
    const inputHash = await sha256(bytes);
    if (token !== generation) return;
    partial = { ...partial, input_bytes: bytes.byteLength, input_sha256: inputHash, built_engine: engine.receipt };
    if (expected && inputHash !== expected) throw Error("Input SHA-256 does not match the requested artifact. Nothing was admitted.");
    let mesh = bytes, inventoryCount = null;
    if (isBundle) {
      // This old API has no caller-selected index/entry allocation admission.
      // Restrict it to byte-identical, already inspected historical content
      // BEFORE invoking the canonical reader; arbitrary hashes are not grants.
      if (inputHash !== HISTORICAL.bundleSha256) throw Error("Generic HMC review requires the later strict Rust inspector build. This retained build only opens the exact pinned historical eucalypt HMC; no bundle parser was invoked.");
      if (typeof engine.wasm.list_hmc_bundle_entries_wasm !== "function" || typeof engine.wasm.read_hmc_bundle_entry_wasm !== "function") throw Error("This retained build lacks canonical HMC inventory/read support.");
      const inventory = engine.wasm.list_hmc_bundle_entries_wasm(bytes);
      const candidates = candidateEntries(inventory, bytes.byteLength); inventoryCount = inventory.length;
      for (const entry of candidates) { const option = document.createElement("option"); option.value = entry.key; el("resource-options").append(option); }
      if (!resource) {
        evidence({ ...partial, inventory_entries: inventoryCount, mesh_candidates: candidates, selection_required: true });
        message("Choose an explicit HMC mesh resource above, then open it. No first entry was selected.");
        el("placeholder").querySelector("p").textContent = "Choose an HMC mesh resource to inspect.";
        return;
      }
      const selected = candidates.find(entry => entry.key === resource);
      if (!selected || !selected.bytes || selected.bytes > MAX_MESH) throw Error("Selected resource is absent, not declared .10d, or exceeds the 8 MiB mesh review profile.");
      mesh = engine.wasm.read_hmc_bundle_entry_wasm(bytes, resource);
      if (!(mesh instanceof Uint8Array) || mesh.byteLength !== selected.bytes) throw Error("Canonical HMC reader returned an inconsistent selected resource.");
    }
    const meshHash = await sha256(mesh);
    if (token !== generation) return;
    partial = { ...partial, mesh_sha256: meshHash, mesh_bytes: mesh.byteLength, inventory_entries: inventoryCount,
      integrity_scope: isBundle ? "Entire input SHA-256 observed; selected HMC entry checksum verified by the canonical Rust reader, then .10d admitted by Rust Portal. Other HMC entries and whole-package semantics unassessed." : "Exact input SHA-256 observed; .10d admitted by the canonical Rust Portal.",
      compatibility: "Current retained build uses existing canonical readers/Portal. The new strict game candidate inspector is not present in this build.",
      hmc_profile: isBundle ? "Exact pinned historical package only; no generic HMC parser allocation claim." : "Not applicable to a loose .10d.",
      texture_dependency_residency: "Unassessed by this historical generic route; a material-pipeline receipt does not establish complete texture/dependency residency or appearance.",
      package_status: "Unreviewed display derivative; complete asset/source/rights/target qualification remains open." };
    if (expectedMesh && meshHash !== expectedMesh) throw Error("Selected mesh SHA-256 differs from the requested resource.");
    // Retire before any renderer mutation. Only a complete, current receipt
    // below can make this source visible or enable evidence capture.
    const admission = portal.load_assets_colored([{ id: "url-review:" + meshHash, bytes: mesh, r: 1, g: 1, b: 1, a: 1 }]);
    if (Number(admission.assets_loaded) !== 1 || Number(admission.assets_refused) !== 0
        || !Number.isSafeInteger(admission.vertex_count) || admission.vertex_count <= 0
        || !Number.isSafeInteger(admission.total_triangles) || admission.total_triangles <= 0) throw Error("Rust Portal refused complete mesh admission.");
    current = { record: partial, admission }; yaw = .7; pitch = .12; resize(); fit();
    canvas.hidden = false; el("placeholder").hidden = true; active = true; firstFrame = true; frameDeadline = performance.now() + 15000; controls(true);
    fileUrl = URL.createObjectURL(new Blob([bytes], { type: "application/octet-stream" }));
    el("download").href = fileUrl; el("download").download = url.pathname.split("/").pop(); el("download").hidden = false;
    el("copy-link").disabled = false;
    const historicalPath = new URL(HISTORICAL.path, root).pathname;
    const historical = url.pathname === historicalPath + HISTORICAL.mesh && inputHash === HISTORICAL.meshSha256
      || url.pathname === historicalPath + HISTORICAL.bundle && inputHash === HISTORICAL.bundleSha256;
    el("history").textContent = historical
      ? "Historical illustrative eucalypt branch study, 29 October 2026. It is not a species-accurate or accepted asset and does not contain the latest implementation."
      : "Unreviewed URL candidate. No biological accuracy, source rights, human acceptance or completed asset package is established.";
    el("historical-links").hidden = !historical;
    el("production-receipt").href = new URL(HISTORICAL.path + "PRODUCTION-RECEIPT.md", root).href;
    el("source-link").href = new URL(HISTORICAL.path + "authoring/spray.vibe", root).href;
    fact("Exact input SHA-256", inputHash); fact("Mesh SHA-256", meshHash);
    fact("Selected resource", resource || "Loose .10d input"); fact("Geometry admitted", admission.vertex_count.toLocaleString() + " vertices · " + admission.total_triangles.toLocaleString() + " triangles");
    fact("Built engine", ENGINE.revision + " · historical"); fact("Material path", admission.material_pipeline ? "Declared material pipeline admitted; textures/dependency residency and appearance unassessed." : "Colour fallback; material fidelity and texture/dependency residency unassessed.");
    evidence({ ...partial, admission, built_engine: engine.receipt, render_status: "Waiting for a current frame." });
    message("Mesh admitted; waiting for a current renderer frame. Human review remains pending.");
  } catch (error) {
    if (token !== generation) return;
    retire("The requested asset was not admitted.");
    message(String(error?.message || error), true);
    evidence({ ...partial, error: String(error?.message || error).slice(0, 2000), human_acceptance: "pending" });
  }
}
function frame(time) {
  if (active && current) {
    try {
      const before = Number(portal.body_render_receipt().frames_presented);
      if (!Number.isSafeInteger(before) || before < 0) throw Error("Renderer frame counter is unavailable.");
      portal.set_camera_target(yaw, pitch, distance, 0, 0, 0); portal.tick(canvas, Math.min(50, time - lastTime));
      const after = Number(portal.body_render_receipt().frames_presented);
      if (!Number.isSafeInteger(after) || after < before) throw Error("Renderer frame counter changed unexpectedly.");
      if (after > before) presented = { generation, version: viewVersion, counter: after, receipt: currentRender() };
      const fresh = presented?.generation === generation && presented.version === viewVersion;
      controls(true, fresh);
      if (firstFrame) {
        if (fresh) {
          const rendered = presented.receipt; firstFrame = false;
          el("render-label").textContent = rendered.renderer + " · current frame · pending review";
          message("Current frame presented · candidate awaiting human review. " + (current.admission.material_pipeline ? "" : "Colour fallback; full materials unqualified."));
          evidence(snapshot(rendered));
        } else if (time > frameDeadline) throw Error("No successful current frame was reported within 15 seconds.");
      } else if (fresh) el("render-label").textContent = presented.receipt.renderer + " · current view frame · pending review";
      else el("render-label").textContent = "View changed; waiting for a new renderer frame";
    } catch (error) { renderFailure(error); }
  }
  lastTime = time; requestAnimationFrame(frame);
}
requestAnimationFrame(frame);
function moveCamera(update) { if (!active) return; update(); viewVersion++; controls(true, false); }
document.querySelectorAll("[data-view]").forEach(button => button.onclick = () => moveCamera(() => {
  const view = button.dataset.view; yaw = view === "front" ? 0 : view === "side" ? Math.PI / 2 : .7; pitch = view === "fit" ? .12 : 0; fit();
}));
el("zoom-in").onclick = () => moveCamera(() => distance = Math.max(.12, distance * .8));
el("zoom-out").onclick = () => moveCamera(() => distance = Math.min(30, distance * 1.25));
const pointers = new Map(); let pinch;
canvas.onpointerdown = event => { if (!active) return; canvas.focus(); pointers.set(event.pointerId, [event.clientX, event.clientY]); canvas.setPointerCapture(event.pointerId); pinch = undefined; };
canvas.onpointermove = event => {
  const old = pointers.get(event.pointerId); if (!old || !active) return;
  pointers.set(event.pointerId, [event.clientX, event.clientY]);
  moveCamera(() => {
    if (pointers.size === 1) { yaw += (event.clientX - old[0]) * .006; pitch = Math.max(-1.3, Math.min(1.3, pitch + (event.clientY - old[1]) * .006)); }
    else { const [a, b] = [...pointers.values()]; const span = Math.hypot(a[0] - b[0], a[1] - b[1]); if (pinch && span > 0) distance = Math.max(.12, Math.min(30, distance * pinch / span)); pinch = span; }
  });
};
canvas.onpointerup = canvas.onpointercancel = event => { pointers.delete(event.pointerId); pinch = undefined; };
canvas.addEventListener("wheel", event => { if (!active) return; event.preventDefault(); moveCamera(() => distance = Math.max(.12, Math.min(30, distance * Math.exp(event.deltaY * .001)))); }, { passive: false });
canvas.onkeydown = event => {
  if (!active) return; const key = event.key;
  if (!["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "+", "-", "="].includes(key)) return; event.preventDefault();
  moveCamera(() => { if (key === "ArrowLeft") yaw -= .1; if (key === "ArrowRight") yaw += .1; if (key === "ArrowUp") pitch = Math.max(-1.3, pitch - .1); if (key === "ArrowDown") pitch = Math.min(1.3, pitch + .1); if (key === "+" || key === "=") distance = Math.max(.12, distance * .8); if (key === "-") distance = Math.min(30, distance * 1.25); });
};
function saveBlob(blob, name) { const url = URL.createObjectURL(blob), anchor = document.createElement("a"); anchor.href = url; anchor.download = name; anchor.click(); setTimeout(() => URL.revokeObjectURL(url), 1000); }
async function withFreshEvidence(save) {
  if (!active || !current || captureBusy) return;
  const token = generation, version = viewVersion;
  const deadline = performance.now() + 5000;
  captureBusy = true; controls(true, false);
  try {
    const previousCounter = Number(portal.body_render_receipt().frames_presented);
    if (!Number.isSafeInteger(previousCounter)) throw Error("No valid renderer counter for evidence capture.");
    while (performance.now() < deadline) {
      await new Promise(resolve => requestAnimationFrame(resolve));
      if (!active || token !== generation || version !== viewVersion) return;
      if (performance.now() >= deadline) break;
      if (presented?.generation === token && presented.version === version && presented.counter > previousCounter) {
        const record = snapshot(presented.receipt);
        await save(record, () => active && token === generation && version === viewVersion);
        return;
      }
    }
    presented = undefined;
    throw Error("No NEW renderer frame arrived within five seconds; cached pixels were not saved as current evidence.");
  } catch (error) {
    if (token === generation) message("Evidence not saved: " + String(error?.message || error), true);
  } finally {
    captureBusy = false;
    controls(active, presented?.generation === generation && presented.version === viewVersion);
  }
}
el("save-record").onclick = () => withFreshEvidence(async (record, stillCurrent) => {
  if (!stillCurrent()) return;
  evidence(record); saveBlob(new Blob([JSON.stringify(record, null, 2)], { type: "application/json" }), "asset-review-" + record.mesh_sha256.slice(0, 12) + ".json");
});
el("save-image").onclick = () => withFreshEvidence(async (record, stillCurrent) => {
  const blob = await new Promise(resolve => canvas.toBlob(resolve, "image/png"));
  if (!stillCurrent()) return;
  if (!blob) throw Error("PNG capture returned no bytes.");
  const pngHash = await sha256(await blob.arrayBuffer());
  if (!stillCurrent()) return;
  const name = "asset-" + record.mesh_sha256.slice(0, 12) + ".png";
  const { last_image_capture, ...imageRecord } = record;
  current.lastImageEvidence = { ...imageRecord, image_sha256: pngHash, image_filename: name };
  evidence(current.lastImageEvidence); saveBlob(blob, name);
});
el("copy-link").onclick = async () => {
  if (!current) return; const url = pinnedLink(current.record), token = generation;
  try { await navigator.clipboard.writeText(url); if (token === generation) message("Pinned artifact review link copied. Acceptance remains pending."); }
  catch { if (token === generation) { message("Clipboard unavailable. Copy the pinned link from the address bar."); history.replaceState(null, "", url); } }
};
el("asset-form").onsubmit = event => {
  event.preventDefault(); const url = new URL(location.href); url.search = "";
  url.searchParams.set("asset", el("asset").value); if (el("expected-sha").value) url.searchParams.set("sha256", el("expected-sha").value);
  if (el("resource").value) url.searchParams.set("resource", el("resource").value);
  history.pushState(null, "", url); openFromUrl();
};
addEventListener("popstate", openFromUrl);
addEventListener("pagehide", () => { generation++; controller?.abort(); active = false; });
addEventListener("pageshow", event => { if (event.persisted) openFromUrl(); });
openFromUrl();
