// Browser projection only. Container parsing and rendering stay in QualiaDB.
export const HISTORICAL = Object.freeze({
  path: "assets/production-studies/eucalypt-spray-r1/revisions/2026-10-29/",
  mesh: "eucalypt-spray.10d",
  meshSha256: "e5af9e12b7ac3e8ab6cbc40f5bc234793ae6a0424090040eb8501e951ef5fa62",
  bundle: "eucalypt-spray.hmc",
  bundleSha256: "270458f0145e40f654a179fbdd2292d620f0f7100a2187c2040dc8ecf21f9eb9",
});
export const ENGINE = Object.freeze({
  base: "viewer/model-revisions/23405c2d31927038bcad441e138f4a91/",
  revision: "23405c2d31927038bcad441e138f4a91",
  wasmSha256: "23405c2d319270382eb79a8512d7c43bb50f98ac6b83127aea0a02d836a3ff3d",
  jsSha256: "bcad441e138f4a9119f5e2614e91b2a910eb41a873f17884926fac3a48758cde",
  scope: "R6 exact native/WASM HMC replay and transport verified. Browser pixels/device interaction and asset acceptance remain pending.",
});
export async function sha256(bytes) {
  if (!globalThis.crypto?.subtle) throw Error("SHA-256 requires HTTPS or a loopback HTTP origin.");
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return [...new Uint8Array(digest)].map(b => b.toString(16).padStart(2, "0")).join("");
}
export function expectedDigest(value, name) {
  if (!value) return null;
  if (!/^[a-fA-F0-9]{64}$/.test(value)) throw Error(name + " must contain exactly 64 hexadecimal SHA-256 digits.");
  return value.toLowerCase();
}
export function resolveAssetUrl(input, root) {
  if (typeof input !== "string" || input.length > 4096 || /[\x00-\x1f\x7f\\]/.test(input)) throw Error("Asset URL exceeds the review profile.");
  // /assets means this application's asset root, including a Pages prefix.
  const mapped = input.startsWith("/assets/") ? input.slice(1) : input;
  const url = new URL(mapped, root);
  if (url.origin !== root.origin || url.username || url.password || url.search || url.hash
      || !url.pathname.startsWith(root.pathname + "assets/") || /%(?:2e|2f|5c|00)/i.test(url.pathname)
      || !/\.(?:10d|hmc)$/i.test(url.pathname)) {
    throw Error("Choose a .10d or .hmc under this application's assets directory. External URLs, query strings and traversal are refused.");
  }
  return url;
}
export async function fetchBounded(url, maxBytes, signal) {
  const response = await fetch(url, { cache: "no-store", credentials: "same-origin", redirect: "error", signal });
  if (!response.ok) throw Error("Fetch refused (" + response.status + ") for " + new URL(url).pathname + ".");
  const length = response.headers.get("content-length");
  if (length && (!/^\d+$/.test(length) || Number(length) > maxBytes)) {
    await response.body?.cancel();
    throw Error("File exceeds this review's " + maxBytes.toLocaleString() + " byte profile.");
  }
  if (!response.body?.getReader) throw Error("This browser cannot bound streamed asset reads.");
  const reader = response.body.getReader();
  const chunks = []; let size = 0;
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      size += value.byteLength;
      if (size > maxBytes) { await reader.cancel(); throw Error("Stream exceeds the bounded review profile."); }
      if (chunks.length >= 4096) { await reader.cancel(); throw Error("Stream exceeds the 4096-chunk review allocation profile."); }
      chunks.push(value);
    }
  } finally { reader.releaseLock(); }
  if (!size) throw Error("The selected file is empty.");
  const bytes = new Uint8Array(size); let offset = 0;
  for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.byteLength; }
  return bytes;
}
let initialized;
export function loadVerifiedEngine(root) {
  if (initialized) return initialized;
  initialized = (async () => {
    const base = new URL(ENGINE.base, root);
    const [js, binary] = await Promise.all([
      fetchBounded(new URL("ecopunk_wasm.js", base), 1024 * 1024),
      fetchBounded(new URL("ecopunk_wasm_bg.wasm", base), 16 * 1024 * 1024),
    ]);
    const [jsHash, wasmHash] = await Promise.all([sha256(js), sha256(binary)]);
    if (jsHash !== ENGINE.jsSha256 || wasmHash !== ENGINE.wasmSha256) throw Error("The pinned engine changed. Review its new receipt before updating this route.");
    // Generated driver has no relative imports. Execute its checked bytes and
    // instantiate checked WASM bytes, avoiding a second unchecked fetch.
    const moduleUrl = URL.createObjectURL(new Blob([js], { type: "text/javascript" }));
    try {
      const wasm = await import(moduleUrl);
      await wasm.default({ module_or_path: binary });
      if (typeof wasm.QualiaPortal !== "function") throw Error("Pinned build has no QualiaPortal.");
      return { wasm, receipt: { ...ENGINE, js_bytes: js.byteLength, wasm_bytes: binary.byteLength, integrity: "Both SHA-256 digests matched before module initialization." } };
    } finally { URL.revokeObjectURL(moduleUrl); }
  })().catch(error => { initialized = undefined; throw error; });
  return initialized;
}
// Validate the Rust reader's metadata projection, never container bytes.
export function candidateEntries(value, fileBytes) {
  if (!Array.isArray(value) || value.length === 0 || value.length > 256) throw Error("Canonical HMC inventory exceeds the 256-entry review profile.");
  const seen = new Set(), ranges = [], candidates = [];
  const count = n => {
    if (typeof n === "bigint") { if (n < 0n || n > BigInt(fileBytes)) throw Error("HMC resource range exceeds file."); return Number(n); }
    if (!Number.isSafeInteger(n) || n < 0 || n > fileBytes) throw Error("HMC resource range is invalid.");
    return n;
  };
  for (const entry of value) {
    if (!entry || typeof entry.key !== "string" || !entry.key || entry.key.length > 512 || /[\x00-\x1f\x7f]/.test(entry.key)
        || typeof entry.kind !== "string" || entry.kind.length > 256 || seen.has(entry.key)) throw Error("HMC inventory has ambiguous or out-of-profile resource keys.");
    seen.add(entry.key);
    const offset = count(entry.offset), length = count(entry.length);
    if (offset + length > fileBytes) throw Error("HMC resource exceeds its exact input bytes.");
    if (length) ranges.push([offset, offset + length]);
    if (entry.kind === "10d" || entry.key.toLowerCase().endsWith(".10d")) candidates.push({ key: entry.key, bytes: length });
  }
  ranges.sort((a, b) => a[0] - b[0]);
  for (let i = 1; i < ranges.length; i++) if (ranges[i][0] < ranges[i - 1][1]) throw Error("Overlapping HMC resources are outside this review profile.");
  return candidates;
}
