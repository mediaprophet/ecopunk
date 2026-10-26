//! Caller-selected native persistence. No default workspace/project destination.
use super::{codec::digest, decode, Checkpoint, CheckpointLimits};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug)]
pub enum ExpectedFile {
    Absent,
    Sha256([u8; 32]),
}
#[derive(Debug)]
pub struct SaveReceipt {
    pub path: PathBuf,
    pub digest: [u8; 32],
    pub bytes: usize,
}

fn destination(path: &Path) -> Result<PathBuf, String> {
    let name = path
        .file_name()
        .ok_or("checkpoint destination needs a file name")?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = parent.canonicalize().map_err(|e| e.to_string())?;
    let path = parent.join(name);
    match fs::symlink_metadata(&path) {
        Ok(meta) if meta.file_type().is_symlink() || !meta.is_file() => {
            Err("checkpoint destination must be a regular file, never a link/directory".into())
        }
        Ok(_) => Ok(path),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}

fn read_bounded(path: &Path, maximum: usize) -> Result<Vec<u8>, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let meta = file.metadata().map_err(|e| e.to_string())?;
    if !meta.is_file() || meta.len() > maximum as u64 {
        return Err("checkpoint input is not a regular file within caller budget".into());
    }
    let mut bytes = Vec::new();
    Read::by_ref(&mut file)
        .take((maximum as u64).saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > maximum {
        return Err("checkpoint input grew beyond caller budget".into());
    }
    Ok(bytes)
}

pub fn load(path: &Path, limits: CheckpointLimits) -> Result<Checkpoint, String> {
    let path = destination(path)?;
    decode(&read_bounded(&path, limits.file_bytes)?, limits)
}

fn check_expected(path: &Path, expected: ExpectedFile, budget: usize) -> Result<(), String> {
    match expected {
        ExpectedFile::Absent => match fs::symlink_metadata(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            _ => Err("checkpoint destination already exists or cannot be inspected".into()),
        },
        ExpectedFile::Sha256(expected) => {
            destination(path)?;
            if digest(&read_bounded(path, budget)?) != expected {
                return Err(
                    "checkpoint destination digest changed; existing bytes preserved".into(),
                );
            }
            Ok(())
        }
    }
}

/// Validate before filesystem mutation, stage/sync a same-directory RAII temp,
/// recheck expected bytes, then atomically install. Create never clobbers.
/// Replacement serializes cooperating writers through a stable OS-locked
/// sidecar. Arbitrary external writers must honour that lock; an OS filesystem
/// has no portable atomic compare-digest-and-rename primitive. Both checks fail
/// closed on observed changes; the final outside-writer race is not concealed.
/// File bytes are synced. Windows power-loss directory durability is unqualified.
pub fn save(
    path: &Path,
    bytes: &[u8],
    expected: ExpectedFile,
    limits: CheckpointLimits,
) -> Result<SaveReceipt, String> {
    decode(bytes, limits)?;
    let path = destination(path)?;
    let parent = path
        .parent()
        .ok_or("checkpoint destination parent absent")?;
    let mut lock_name = path
        .file_name()
        .ok_or("destination file name absent")?
        .to_os_string();
    lock_name.push(".poet-source.lock");
    let lock_path = parent.join(lock_name);
    if fs::symlink_metadata(&lock_path)
        .is_ok_and(|meta| meta.file_type().is_symlink() || !meta.is_file())
    {
        return Err("checkpoint lock path is not a regular file".into());
    }
    // Persistent coordination artifact, not stale temporary state. OS releases
    // its lock on process exit; never unlink it while another writer may wait.
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|e| e.to_string())?;
    lock.try_lock()
        .map_err(|e| format!("checkpoint writer busy: {e}"))?;
    check_expected(&path, expected, limits.file_bytes)?;
    let mut staged = tempfile::Builder::new()
        .prefix(".poet-source-")
        .tempfile_in(parent)
        .map_err(|e| e.to_string())?;
    staged.write_all(bytes).map_err(|e| e.to_string())?;
    staged.as_file().sync_all().map_err(|e| e.to_string())?;
    check_expected(&path, expected, limits.file_bytes)?;
    let installed = match expected {
        ExpectedFile::Absent => staged.persist_noclobber(&path),
        ExpectedFile::Sha256(_) => staged.persist(&path),
    }
    .map_err(|e| e.to_string())?;
    installed
        .sync_all()
        .map_err(|e| format!("checkpoint installed, file sync failed: {e}"))?;
    #[cfg(unix)]
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|e| format!("checkpoint installed, directory sync failed: {e}"))?;
    Ok(SaveReceipt {
        path,
        digest: digest(bytes),
        bytes: bytes.len(),
    })
}
