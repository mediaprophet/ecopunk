use super::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = env::temp_dir().join(format!(
            "asset-audit-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&p).unwrap();
        Self(p)
    }
    fn put(&self, p: &str, b: &[u8]) {
        let f = self.0.join(p);
        if let Some(d) = f.parent() {
            fs::create_dir_all(d).unwrap()
        }
        fs::write(f, b).unwrap();
    }
    fn manifest(&self, components: Vec<Value>) -> PathBuf {
        self.put("graph.json", b"graph");
        self.put("volume.q42", b"volume");
        let v = json!({"schema":"ecopunk-model-review-v1","models":[{"id":"m","components":components}],"graph":{"source":"graph.json","sourceSha256":format!("{:x}",Sha256::digest(b"graph")),"sourceBytes":5,"volume":"volume.q42","volumeSha256":format!("{:x}",Sha256::digest(b"volume")),"volumeBytes":6}});
        let p = self.0.join("model-set.json");
        fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
        p
    }
    fn decl(path: &str, expected: &[u8], bytes: u64) -> Value {
        json!({"file":path,"sha256":format!("{:x}",Sha256::digest(expected)),"bytes":bytes})
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn correct_metadata_passes() {
    let t = Temp::new();
    t.put("a.bin", b"asset");
    let r = run_audit(&t.manifest(vec![Temp::decl("a.bin", b"asset", 5)]), 10, 20).unwrap();
    assert_eq!(r["status"], "ok");
    let original = fs::read(t.0.join("model-set.json")).unwrap();
    assert_eq!(r["manifestBytes"], original.len());
    assert_eq!(
        r["manifestSha256"],
        format!("{:x}", Sha256::digest(original))
    );
}

#[test]
fn non_regular_or_oversized_manifest_is_rejected_before_reading() {
    let t = Temp::new();
    let error = run_audit(&t.0, 10, 20).unwrap_err();
    assert!(error.contains("regular file"));
    let path = t.0.join("oversized.json");
    fs::File::create(&path)
        .unwrap()
        .set_len(MAX_MANIFEST + 1)
        .unwrap();
    assert!(run_audit(&path, 10, 20).unwrap_err().contains("exceeds"));
}

#[test]
fn corrupt_resource_marks_every_repeated_reference_and_checks_graph() {
    let t = Temp::new();
    t.put("asset", b"bad");
    let manifest = t.manifest(vec![
        Temp::decl("asset", b"expected", 8),
        Temp::decl("asset", b"expected", 8),
    ]);
    t.put("graph.json", b"wrong");
    let result = run_audit(&manifest, 20, 100).unwrap();
    let refs = result["references"].as_array().unwrap();
    assert_eq!(refs[0]["status"], "integrity_issue");
    assert_eq!(refs[1]["status"], "integrity_issue");
    assert_eq!(refs[0]["resourceId"], refs[1]["resourceId"]);
    assert_eq!(refs[2]["status"], "integrity_issue");
    assert_eq!(refs[3]["status"], "ok");
    assert_eq!(result["distinctCanonicalPathCount"], 3);
}
#[test]
fn understated_actual_length_is_stopped_by_injected_cap() {
    let t = Temp::new();
    t.put("large", &vec![b'x'; 12]);
    let r = run_audit(&t.manifest(vec![Temp::decl("large", b"", 0)]), 5, 20).unwrap();
    assert!(r["issues"].to_string().contains("resource_budget_exceeded"));
}
#[test]
fn duplicate_conflicts_are_shared_not_quadratic() {
    let t = Temp::new();
    t.put("asset", b"bad");
    let cs = (0..3000)
        .map(|i| Temp::decl("asset", if i % 2 == 0 { b"one" } else { b"two" }, 3))
        .collect();
    let r = run_audit(&t.manifest(cs), 20, 100).unwrap();
    assert_eq!(r["referenceCount"], 3002);
    assert!(r["references"]
        .as_array()
        .unwrap()
        .iter()
        .take(3000)
        .all(|x| x["status"] == "integrity_issue"));
    let encoded = serde_json::to_vec(&r).unwrap();
    assert!(
        encoded.len() < 8_000_000,
        "report unexpectedly large: {}",
        encoded.len()
    );
}
#[test]
fn aggregate_actual_bytes_are_bounded() {
    let t = Temp::new();
    t.put("a", b"12345");
    t.put("b", b"67890");
    let r = run_audit(
        &t.manifest(vec![
            Temp::decl("a", b"12345", 5),
            Temp::decl("b", b"67890", 5),
        ]),
        10,
        15,
    )
    .unwrap();
    assert!(r["issues"].to_string().contains("total_budget_exceeded"));
}
#[test]
fn malformed_schema_fails() {
    let t = Temp::new();
    let p = t.0.join("bad.json");
    fs::write(&p, b"{}").unwrap();
    assert!(run_audit(&p, 10, 20).is_err());
}
#[test]
fn strict_lexical_paths_fail() {
    for p in [
        "",
        "a//b",
        "a/./b",
        "a/../b",
        "a/",
        "file:stream",
        "C:/file",
        "//server/share",
        "a\\b",
    ] {
        assert!(pathing::safe_target(Path::new("."), p).is_err(), "{p}");
    }
}
#[cfg(unix)]
#[test]
fn symlink_aliases_share_failure_and_budget_accounting() {
    use std::os::unix::fs::symlink;
    let t = Temp::new();
    t.put("real", b"bad!!");
    symlink(t.0.join("real"), t.0.join("alias")).unwrap();
    let p = t.manifest(vec![
        Temp::decl("real", b"right", 5),
        Temp::decl("alias", b"right", 5),
    ]);
    let r = run_audit(&p, 10, 16).unwrap();
    let refs = r["references"].as_array().unwrap();
    assert!(refs[0]["status"] == "integrity_issue" && refs[1]["status"] == "integrity_issue");
    assert_eq!(refs[0]["resourceId"], refs[1]["resourceId"]);
}
