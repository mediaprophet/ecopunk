use std::path::{Component, Path, PathBuf};

pub fn safe_target(base: &Path, raw: &str) -> Result<PathBuf, String> {
    if raw.is_empty() || raw.contains('\\') || raw.contains(':') || raw.starts_with('/') {
        return Err("unsafe path form".into());
    }
    if raw
        .split('/')
        .any(|s| s.is_empty() || s == "." || s == "..")
    {
        return Err("empty, dot, or parent path segment".into());
    }
    let rel = Path::new(raw);
    if rel.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err("non-normal path component".into());
    }
    let target = base
        .join(rel)
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize target: {e}"))?;
    if !target.starts_with(base) {
        return Err("canonical target escapes manifest directory".into());
    }
    Ok(target)
}
