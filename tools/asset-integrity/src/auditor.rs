use crate::{hashing, pathing};
#[cfg(test)]
#[path = "auditor_tests.rs"]
mod tests;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

const MAX_MANIFEST: u64 = 4 * 1024 * 1024;
const MAX_RESOURCE: u64 = 512 * 1024 * 1024;
const MAX_TOTAL: u64 = 2 * 1024 * 1024 * 1024;

#[derive(Clone, Debug)]
struct Decl {
    path: String,
    sha: String,
    bytes: u64,
    context: String,
}
#[derive(Default)]
struct Resource {
    decls: Vec<Decl>,
    refs: Vec<usize>,
    issues: Vec<Value>,
}
fn get<'a>(v: &'a Value, k: &str) -> Result<&'a Value, String> {
    v.get(k).ok_or_else(|| format!("missing field {k}"))
}
fn s(v: &Value, k: &str) -> Result<String, String> {
    get(v, k)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("{k} must be a string"))
}
fn n(v: &Value, k: &str) -> Result<u64, String> {
    get(v, k)?
        .as_u64()
        .ok_or_else(|| format!("{k} must be a nonnegative integer"))
}
fn declaration(v: &Value, p: &str, h: &str, b: &str, context: String) -> Result<Decl, String> {
    let path = s(v, p)?;
    let sha = s(v, h)?;
    let bytes = n(v, b)?;
    if sha.len() != 64 || !sha.bytes().all(|x| x.is_ascii_hexdigit()) {
        return Err(format!("{context}: {h} must be 64 hex characters"));
    }
    Ok(Decl {
        path,
        sha: sha.to_ascii_lowercase(),
        bytes,
        context,
    })
}
fn declarations(root: &Value) -> Result<Vec<Decl>, String> {
    if s(root, "schema")? != "ecopunk-model-review-v1" {
        return Err("unsupported schema".into());
    }
    let models = get(root, "models")?
        .as_array()
        .ok_or("models must be array")?;
    let mut out = Vec::new();
    for (mi, m) in models.iter().enumerate() {
        let id = m
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<missing-id>");
        let cs = get(m, "components")
            .map_err(|e| format!("models[{mi}] {id}: {e}"))?
            .as_array()
            .ok_or("components must be array")?;
        for (ci, c) in cs.iter().enumerate() {
            let context = format!("model:{id}:component:{ci}");
            out.push(
                declaration(c, "file", "sha256", "bytes", context.clone())
                    .map_err(|e| format!("{context}: {e}"))?,
            );
        }
    }
    let g = get(root, "graph")?;
    out.push(declaration(
        g,
        "source",
        "sourceSha256",
        "sourceBytes",
        "graph:source".into(),
    )?);
    out.push(declaration(
        g,
        "volume",
        "volumeSha256",
        "volumeBytes",
        "graph:volume".into(),
    )?);
    Ok(out)
}
fn run_audit(manifest: &Path, max_resource: u64, max_total: u64) -> Result<Value, String> {
    let cm = manifest
        .canonicalize()
        .map_err(|e| format!("cannot resolve manifest: {e}"))?;
    let base = cm
        .parent()
        .ok_or("manifest lacks parent")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let initial = std::fs::metadata(&cm).map_err(|e| format!("cannot stat manifest: {e}"))?;
    if !initial.is_file() {
        return Err("manifest must be a regular file".into());
    }
    if initial.len() > MAX_MANIFEST {
        return Err("manifest exceeds 4 MiB".into());
    }
    let f = File::open(&cm).map_err(|e| format!("cannot read manifest: {e}"))?;
    let opened = f.metadata().map_err(|e| e.to_string())?;
    if !opened.is_file() || opened.len() != initial.len() {
        return Err("manifest changed before reading".into());
    }
    let mut bytes = Vec::new();
    f.take(MAX_MANIFEST + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_MANIFEST {
        return Err("manifest grew beyond 4 MiB".into());
    }
    if bytes.len() as u64 != opened.len() {
        return Err("manifest size changed while reading".into());
    }
    let manifest_sha256 = format!("{:x}", Sha256::digest(&bytes));
    let root: Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid JSON/schema: {e}"))?;
    let ds = declarations(&root)?;
    let mut groups: BTreeMap<PathBuf, Resource> = BTreeMap::new();
    let mut refs = Vec::new();
    let mut global = Vec::new();
    for (i, d) in ds.into_iter().enumerate() {
        let rid = format!("R{i:06}");
        let target = match pathing::safe_target(&base, &d.path) {
            Ok(p) => p,
            Err(e) => {
                global.push(json!({"id":rid,"reference":i,"context":d.context,"path":d.path,"status":"unsafe_or_missing","detail":e}));
                refs.push(json!({"id":rid,"context":d.context,"path":d.path,"status":"unsafe_or_missing"}));
                continue;
            }
        };
        let g = groups.entry(target.clone()).or_default();
        g.refs.push(i);
        g.decls.push(d.clone());
        refs.push(json!({"id":rid,"context":d.context,"path":d.path,"status":"pending"}));
    }
    let mut used = 0u64;
    let mut path_to_issue_ids = BTreeMap::new();
    for (path, g) in groups.iter_mut() {
        let issue_start = global.len();
        let metadata_conflict = g
            .decls
            .iter()
            .any(|d| d.sha != g.decls[0].sha || d.bytes != g.decls[0].bytes);
        if metadata_conflict {
            for d in &g.decls {
                g.issues.push(json!({"status":"conflicting_metadata","context":d.context,"path":d.path,"expectedSha256":d.sha,"expectedBytes":d.bytes}));
            }
        }
        match std::fs::metadata(path) {
            Err(e) => g
                .issues
                .push(json!({"status":"resource_stat_error","detail":e.to_string()})),
            Ok(pm) if !pm.is_file() => g.issues.push(json!({"status":"not_regular_file"})),
            Ok(pm) if pm.len() > max_resource => g.issues.push(json!({"status":"resource_budget_exceeded","actualBytes":pm.len(),"limit":max_resource})),
            Ok(pm) if used.checked_add(pm.len()).filter(|x| *x <= max_total).is_none() => g.issues.push(json!({"status":"total_budget_exceeded","actualBytes":pm.len(),"usedBytes":used,"limit":max_total})),
            Ok(pm) => {
                let reserved = pm.len();
                used += reserved;
                match File::open(path) {
                    Err(e) => g.issues.push(json!({"status":"resource_open_error","detail":e.to_string()})),
                    Ok(mut file) => {
                        match file.metadata() {
                            Err(e) => g.issues.push(json!({"status":"resource_stat_error","detail":e.to_string()})),
                            Ok(md) if !md.is_file() => g.issues.push(json!({"status":"not_regular_file"})),
                            Ok(md) if md.len() != reserved => g.issues.push(json!({"status":"resource_changed_before_hash","statBytes":reserved,"openedBytes":md.len()})),
                            Ok(_) => match hashing::hash_limited(&mut file, reserved) {
                                Err(e) => g.issues.push(json!({"status":"resource_read_error","detail":e.to_string()})),
                                Ok((hash, count, changed)) => {
                                    let handle_after = file.metadata();
                                    let path_after = std::fs::metadata(path);
                                    let stable = matches!((&handle_after, &path_after), (Ok(h), Ok(p))
                                        if h.is_file() && p.is_file() && h.len() == reserved && p.len() == reserved);
                                    if changed || count != reserved || !stable {
                                        g.issues.push(json!({"status":"resource_changed_during_hash","actualBytes":count,"reservedBytes":reserved}));
                                    } else {
                                        for d in &g.decls {
                                            if count != d.bytes { g.issues.push(json!({"status":"byte_count_mismatch","context":d.context,"path":d.path,"expectedBytes":d.bytes,"actualBytes":count})); }
                                            if hash != d.sha { g.issues.push(json!({"status":"sha256_mismatch","context":d.context,"path":d.path,"expectedSha256":d.sha,"actualSha256":hash})); }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
        if g.issues.is_empty() {
            path_to_issue_ids.insert(path.clone(), Vec::<String>::new());
        } else {
            let ids: Vec<String> = (0..g.issues.len())
                .map(|j| format!("I{:06}", issue_start + j))
                .collect();
            for (id, issue) in ids.iter().zip(g.issues.drain(..)) {
                global.push(json!({"id":id,"canonicalResource":path,"finding":issue}));
            }
            path_to_issue_ids.insert(path.clone(), ids);
        }
    }
    let mut resources = Vec::new();
    for (path, g) in &groups {
        let ids = &path_to_issue_ids[path];
        for i in &g.refs {
            let r = &mut refs[*i];
            r["resourceId"] = json!(path.to_string_lossy());
            r["status"] = if ids.is_empty() {
                "ok"
            } else {
                "integrity_issue"
            }
            .into();
        }
        resources.push(json!({"resourceId":path.to_string_lossy(),"status":if ids.is_empty(){"ok"}else{"integrity_issue"},"issueIds":ids}));
    }
    Ok(
        json!({"schema":"asset-integrity-report-v1","manifest":cm,"manifestSha256":manifest_sha256,"manifestBytes":bytes.len(),"referenceCount":refs.len(),"distinctCanonicalPathCount":groups.len(),"issues":global,"resources":resources,"references":refs,"status":if global.is_empty(){"ok"}else{"issues"}}),
    )
}
pub fn run() {
    let a: Vec<String> = env::args().skip(1).collect();
    if a.len() != 1 {
        eprintln!("usage: asset-integrity <model-set.json>");
        std::process::exit(2)
    }
    match run_audit(Path::new(&a[0]), MAX_RESOURCE, MAX_TOTAL) {
        Ok(v) => {
            let bad = !v["issues"].as_array().unwrap().is_empty();
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
            std::process::exit(if bad { 1 } else { 0 })
        }
        Err(e) => {
            eprintln!("asset-integrity: {e}");
            std::process::exit(2)
        }
    }
}
