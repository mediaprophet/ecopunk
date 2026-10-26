//! Explicit native commands use one owned worker with one queued request.
use super::controller::{EditorHost, EditorSnapshot, SessionEpoch};
use super::preview::{SourceRef, native_image_uri};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread::JoinHandle;
use std::time::Duration;

const MAX_REQUEST_BYTES: usize = 64 * 1024;

#[derive(Clone, PartialEq, Eq)]
pub struct Context {
    pub source: SourceRef,
    pub session_revision: u64,
    pub execution_epoch: SessionEpoch,
    pub reconciliation_required: bool,
}
impl Context {
    pub fn from_snapshot(snapshot: &EditorSnapshot) -> Self {
        Self {
            source: snapshot.source.clone(),
            session_revision: snapshot.session_revision,
            execution_epoch: snapshot.execution_epoch.clone(),
            reconciliation_required: snapshot.reconciliation_required,
        }
    }
}

pub enum Action {
    Check(String),
    Submit { source: String, expected: SourceRef },
    Undo(SourceRef),
    Redo(SourceRef),
    Rebuild(SourceRef),
    Save { name: String, expected: SourceRef },
    Refresh,
}

pub struct Reply {
    pub origin: Context,
    pub snapshot: Option<EditorSnapshot>,
    pub message: String,
    pub image_uri: Option<String>,
    pub current: bool,
}

struct Request {
    action: Action,
    origin: Context,
    sender: futures_channel::oneshot::Sender<Reply>,
}

#[derive(Clone)]
pub struct HostHandle {
    sender: mpsc::SyncSender<Request>,
    closing: Arc<AtomicBool>,
    // Derived completion guard only; authoritative state stays in EditorHost.
    latest: Arc<Mutex<Option<Context>>>,
}
impl PartialEq for HostHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.closing, &other.closing)
    }
}

/// Owned by launch(), outside the component. Its join runs only after Aether's
/// event loop returns. Started native effects cannot be forcibly cancelled.
pub struct Worker {
    closing: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}
impl Worker {
    pub fn start(mut host: Box<dyn EditorHost>) -> Result<(Self, HostHandle), String> {
        let (sender, receiver) = mpsc::sync_channel::<Request>(1);
        let closing = Arc::new(AtomicBool::new(false));
        let worker_closing = closing.clone();
        let latest = Arc::new(Mutex::new(Some(Context::from_snapshot(&host.snapshot()?))));
        let worker_latest = latest.clone();
        let thread = std::thread::Builder::new()
            .name("poet-native-authoring".into())
            .spawn(move || {
                loop {
                    if worker_closing.load(Ordering::Acquire) {
                        break;
                    }
                    let request = match receiver.recv_timeout(Duration::from_millis(50)) {
                        Ok(request) => request,
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    };
                    // A closed receiver/window abandons work that has not started.
                    if worker_closing.load(Ordering::Acquire) || request.sender.is_canceled() {
                        continue;
                    }
                    // Invalidate already-sent completions while the next job runs.
                    *worker_latest.lock().expect("completion cache poisoned") = None;
                    let reply = execute(host.as_mut(), request.action, request.origin);
                    *worker_latest.lock().expect("completion cache poisoned") = reply
                        .snapshot
                        .as_ref()
                        .filter(|_| reply.current)
                        .map(Context::from_snapshot);
                    let _ = request.sender.send(reply);
                }
            })
            .map_err(|error| format!("native authoring worker start: {error}"))?;
        Ok((
            Self {
                closing: closing.clone(),
                thread: Some(thread),
            },
            HostHandle {
                sender,
                closing,
                latest,
            },
        ))
    }

    pub fn finish(mut self) -> Result<(), String> {
        self.join()
    }
    fn join(&mut self) -> Result<(), String> {
        self.closing.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            thread.join().map_err(|_| "native authoring worker panicked; started host effects may require reconciliation".to_string())?;
        }
        Ok(())
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        // Also join during launch unwinding; never silently detach a started job.
        if let Err(error) = self.join() {
            eprintln!("{error}");
        }
    }
}

struct BoundImage {
    context: Context,
    uri: String,
}

fn image_for_snapshot(
    image: Option<BoundImage>,
    snapshot: Option<&EditorSnapshot>,
    current: bool,
    message: &mut String,
) -> Option<String> {
    image.and_then(|image| {
        if current
            && image.context.execution_epoch.is_current()
            && snapshot.is_some_and(|now| Context::from_snapshot(now) == image.context)
        {
            Some(image.uri)
        } else {
            message.push_str("; completed frame source/session changed; withheld");
            None
        }
    })
}

fn perform(
    host: &mut dyn EditorHost,
    action: Action,
) -> Result<(String, Option<BoundImage>), String> {
    let image_uri = None;
    let message = match action {
        Action::Check(source) => host.check(&source)?,
        Action::Submit { source, expected } => {
            let outcome = host.submit(&source, expected)?;
            return outcome_image(outcome);
        }
        Action::Rebuild(expected) => {
            if !host.snapshot()?.rebuild_supported {
                return Err("Execution rebuild is unsupported by this host profile".into());
            }
            return outcome_image(host.rebuild_execution(expected)?);
        }
        Action::Undo(expected) => {
            host.undo(expected)?;
            "Editable source restored; session reconciliation required".into()
        }
        Action::Redo(expected) => {
            host.redo(expected)?;
            "Editable source reapplied; session reconciliation required".into()
        }
        Action::Save { name, expected } => {
            let receipt=host.save_checkpoint(&name,
                qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile::Absent,expected)?;
            let digest=receipt.digest.iter().map(|b|format!("{b:02x}")).collect::<String>();
            format!("Accepted source/history saved as {name} ({} bytes, SHA256 {digest}); draft and human acceptance are separate",receipt.bytes)
        }
        Action::Refresh => "Current identified source read from canonical store".into(),
    };
    Ok((message, image_uri))
}

fn outcome_image(
    outcome: super::controller::EditorOutcome,
) -> Result<(String, Option<BoundImage>), String> {
    let mut image_uri = None;
    let mut message = outcome.message;
    if let Some(frame) = outcome.frame {
        if frame.source == outcome.snapshot.source
            && frame.session_revision == Some(outcome.snapshot.session_revision)
            && frame.execution_epoch.as_ref() == Some(&outcome.snapshot.execution_epoch)
            && outcome.snapshot.execution_epoch.is_current()
        {
            match native_image_uri(&frame) {
                Ok(uri) => {
                    image_uri = Some(BoundImage {
                        context: Context::from_snapshot(&outcome.snapshot),
                        uri,
                    })
                }
                Err(error) => message.push_str(&format!("; frame withheld: {error}")),
            }
        } else {
            message.push_str("; frame source/session mismatch; withheld");
        }
    }
    Ok((message, image_uri))
}

fn execute(host: &mut dyn EditorHost, action: Action, origin: Context) -> Reply {
    let refresh = matches!(action, Action::Refresh);
    let result = host.snapshot().and_then(|snapshot| {
        if !refresh { origin.source.check_current()?; }
        if !refresh && !origin.execution_epoch.is_current() {
            return Err("Execution epoch retired; no command dispatched. Refresh source.".into());
        }
        let target = match &action {
            Action::Submit { expected, .. } | Action::Undo(expected) | Action::Redo(expected) | Action::Rebuild(expected) | Action::Save { expected, .. } => Some(expected),
            _ => None,
        };
        if target.is_some_and(|target| target != &origin.source) {
            return Err("Command target differs from its admitted source context; no command dispatched".into());
        }
        if !refresh && Context::from_snapshot(&snapshot) != origin {
            return Err("Source/session changed before command start; no command dispatched. Refresh source.".into());
        }
        perform(host, action)
    });
    let snapshot = host.snapshot().ok();
    // Heavy host snapshots and encoding run on this worker. The UI repeats
    // only the nonblocking opaque-stamp read immediately before publication.
    let current = snapshot.as_ref().is_some_and(|snapshot| {
        snapshot.execution_epoch.is_current()
            && snapshot.source.check_current().is_ok()
            && host
                .snapshot()
                .is_ok_and(|now| Context::from_snapshot(&now) == Context::from_snapshot(snapshot))
    });
    let (mut message, bound_image) = match result {
        Ok(result) => result,
        Err(error) => (error, None),
    };
    let image_uri = image_for_snapshot(bound_image, snapshot.as_ref(), current, &mut message);
    Reply {
        origin,
        snapshot,
        message,
        image_uri,
        current,
    }
}

pub async fn run(host: HostHandle, action: Action, origin: Context) -> Result<Reply, String> {
    if host.closing.load(Ordering::Acquire) {
        return Err("Native authoring worker is closing".into());
    }
    if matches!(&action, Action::Check(source) | Action::Submit { source, .. } if source.len() > MAX_REQUEST_BYTES)
    {
        return Err("Source exceeds the native request limit of 64 KiB".into());
    }
    if matches!(&action, Action::Save { name, .. } if name.len()>128) {
        return Err("Checkpoint filename exceeds native request budget".into());
    }
    let (sender, receiver) = futures_channel::oneshot::channel();
    host.sender
        .try_send(Request {
            action,
            origin,
            sender,
        })
        .map_err(|error| match error {
            mpsc::TrySendError::Full(_) => {
                "Native authoring queue is full; no command admitted".to_string()
            }
            mpsc::TrySendError::Disconnected(_) => {
                "Native authoring worker ended; started effects may require host reconciliation"
                    .to_string()
            }
        })?;
    receiver.await.map_err(|_| "Native authoring worker ended without a result; started effects may require host reconciliation. No automatic retry.".to_string())
}

/// The UI can install a completion only into the exact view that requested it.
/// The worker separately validated the resulting source incarnation/session.
pub fn completion_is_current(
    host: &HostHandle,
    current_view: &EditorSnapshot,
    reply: &Reply,
) -> bool {
    !host.closing.load(Ordering::Acquire)
        && reply.current
        && Context::from_snapshot(current_view) == reply.origin
        && reply.snapshot.as_ref().is_some_and(|snapshot| {
            snapshot.execution_epoch.is_current()
                && host
                    .latest
                    .try_lock()
                    .is_ok_and(|latest| latest.as_ref() == Some(&Context::from_snapshot(snapshot)))
                && qualia_core_db::poet_host::invoke::studio_store::artifact_stamp_is_current(
                    &snapshot.source.artifact_id,
                    &snapshot.source.stamp,
                )
        })
}

#[cfg(test)]
#[path = "jobs_tests.rs"]
mod tests;
