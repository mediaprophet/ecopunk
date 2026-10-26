//! Explicit native commands use one owned worker with one queued request.
use super::controller::{EditorHost, EditorSnapshot};
use super::preview::{SourceRef, native_image_uri};
use std::sync::{
    Arc,
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
}
impl Context {
    pub fn from_snapshot(snapshot: &EditorSnapshot) -> Self {
        Self {
            source: snapshot.source.clone(),
            session_revision: snapshot.session_revision,
        }
    }
}

pub enum Action {
    Check(String),
    Submit { source: String, expected: SourceRef },
    Undo(SourceRef),
    Redo(SourceRef),
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
                    let reply = execute(host.as_mut(), request.action, request.origin);
                    let _ = request.sender.send(reply);
                }
            })
            .map_err(|error| format!("native authoring worker start: {error}"))?;
        Ok((
            Self {
                closing: closing.clone(),
                thread: Some(thread),
            },
            HostHandle { sender, closing },
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
        if current && snapshot.is_some_and(|now| Context::from_snapshot(now) == image.context) {
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
    let mut image_uri = None;
    let message = match action {
        Action::Check(source) => host.check(&source)?,
        Action::Submit { source, expected } => {
            let outcome = host.submit(&source, expected)?;
            let mut message = outcome.message;
            if let Some(frame) = outcome.frame {
                if frame.source == outcome.snapshot.source
                    && frame.session_revision == Some(outcome.snapshot.session_revision)
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
            message
        }
        Action::Undo(expected) => {
            host.undo(expected)?;
            "Editable source restored; session reconciliation required".into()
        }
        Action::Redo(expected) => {
            host.redo(expected)?;
            "Editable source reapplied; session reconciliation required".into()
        }
        Action::Refresh => "Current identified source read from canonical store".into(),
    };
    Ok((message, image_uri))
}

fn execute(host: &mut dyn EditorHost, action: Action, origin: Context) -> Reply {
    let refresh = matches!(action, Action::Refresh);
    let result = host.snapshot().and_then(|snapshot| {
        if !refresh { origin.source.check_current()?; }
        let target = match &action {
            Action::Submit { expected, .. } | Action::Undo(expected) | Action::Redo(expected) => Some(expected),
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
        snapshot.source.check_current().is_ok()
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
pub fn completion_is_current(current_view: &EditorSnapshot, reply: &Reply) -> bool {
    reply.current
        && Context::from_snapshot(current_view) == reply.origin
        && reply.snapshot.as_ref().is_some_and(|snapshot| {
            qualia_core_db::poet_host::invoke::studio_store::artifact_stamp_is_current(
                &snapshot.source.artifact_id,
                &snapshot.source.stamp,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::super::controller::EditorOutcome;
    use super::*;
    use qualia_core_db::poet_host::invoke::studio_store::{self, StudioArtifact};
    use std::sync::{Barrier, Mutex, atomic::AtomicUsize};

    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct ProbeHost {
        snapshot: EditorSnapshot,
        calls: Arc<Mutex<Vec<std::thread::ThreadId>>>,
        gate: Option<Arc<Barrier>>,
    }
    impl EditorHost for ProbeHost {
        fn snapshot(&self) -> Result<EditorSnapshot, String> {
            Ok(self.snapshot.clone())
        }
        fn check(&self, source: &str) -> Result<String, String> {
            self.calls.lock().unwrap().push(std::thread::current().id());
            if source == "hold" || source == "panic" {
                if let Some(gate) = &self.gate {
                    gate.wait();
                    gate.wait();
                }
            }
            if source == "panic" {
                panic!("worker lifecycle probe");
            }
            Ok("checked without execution".into())
        }
        fn submit(&mut self, _: &str, _: SourceRef) -> Result<EditorOutcome, String> {
            self.snapshot.reconciliation_required = true;
            Err("host attempt failed; reconciliation required".into())
        }
        fn undo(&mut self, _: SourceRef) -> Result<EditorSnapshot, String> {
            Err("no source history".into())
        }
        fn redo(&mut self, _: SourceRef) -> Result<EditorSnapshot, String> {
            Err("no redo source".into())
        }
    }
    fn fixture(gate: Option<Arc<Barrier>>) -> (ProbeHost, Arc<Mutex<Vec<std::thread::ThreadId>>>) {
        let id = format!("poet-ui-worker-{}", NEXT.fetch_add(1, Ordering::Relaxed));
        let (artifact, stamp) = studio_store::create_artifact_if_absent(StudioArtifact::new(
            &id,
            "Geometry",
            "Worker test",
        ))
        .unwrap();
        let snapshot = EditorSnapshot {
            artifact_id: id.clone(),
            title: artifact.title,
            source_revision: artifact.lamport_clock,
            source: SourceRef {
                artifact_id: id,
                stamp,
            },
            session_revision: 0,
            latest_source: String::new(),
            reconciliation_required: false,
        };
        let calls = Arc::new(Mutex::new(Vec::new()));
        (
            ProbeHost {
                snapshot,
                calls: calls.clone(),
                gate,
            },
            calls,
        )
    }
    fn enqueue(
        handle: &HostHandle,
        action: Action,
        origin: Context,
    ) -> futures_channel::oneshot::Receiver<Reply> {
        let (sender, receiver) = futures_channel::oneshot::channel();
        assert!(
            handle
                .sender
                .try_send(Request {
                    action,
                    origin,
                    sender
                })
                .is_ok()
        );
        receiver
    }
    fn receive(mut receiver: futures_channel::oneshot::Receiver<Reply>) -> Reply {
        for _ in 0..200 {
            if let Some(reply) = receiver.try_recv().unwrap() {
                return reply;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        panic!("worker did not deliver bounded test reply");
    }

    #[test]
    fn one_worker_one_queue_and_cancelled_queued_work_never_starts() {
        let gate = Arc::new(Barrier::new(2));
        let (host, calls) = fixture(Some(gate.clone()));
        let origin = Context::from_snapshot(&host.snapshot);
        let (worker, handle) = Worker::start(Box::new(host)).unwrap();
        let first = enqueue(&handle, Action::Check("hold".into()), origin.clone());
        gate.wait(); // Started work is on the native worker, not this caller.
        let cancelled = enqueue(&handle, Action::Check("cancelled".into()), origin.clone());
        let (sender, _receiver) = futures_channel::oneshot::channel();
        assert!(matches!(
            handle.sender.try_send(Request {
                action: Action::Refresh,
                origin,
                sender
            }),
            Err(mpsc::TrySendError::Full(_))
        ));
        drop(cancelled);
        gate.wait();
        assert!(receive(first).current);
        worker.finish().unwrap();
        let calls = calls.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0], std::thread::current().id());
    }

    #[test]
    fn stale_session_or_replaced_incarnation_cannot_dispatch_or_install() {
        let (host, calls) = fixture(None);
        let view = host.snapshot.clone();
        let mut stale = Context::from_snapshot(&view);
        stale.session_revision += 1;
        let (worker, handle) = Worker::start(Box::new(host)).unwrap();
        let reply = receive(enqueue(&handle, Action::Check("unused".into()), stale));
        assert!(reply.message.contains("no command dispatched"));
        assert!(!completion_is_current(&view, &reply));
        assert!(calls.lock().unwrap().is_empty());
        // Identical id/revision replacement still has a different opaque stamp.
        let replacement = studio_store::get_artifact(&view.artifact_id).unwrap();
        studio_store::upsert_artifact(replacement);
        let reply = receive(enqueue(
            &handle,
            Action::Check("unused".into()),
            Context::from_snapshot(&view),
        ));
        assert!(!reply.current);
        assert!(!completion_is_current(&view, &reply));
        assert!(calls.lock().unwrap().is_empty());
        worker.finish().unwrap();
    }

    #[test]
    fn failed_host_attempt_delivers_reconciliation_snapshot_without_retry() {
        let (host, _) = fixture(None);
        let view = host.snapshot.clone();
        let (worker, handle) = Worker::start(Box::new(host)).unwrap();
        let reply = receive(enqueue(
            &handle,
            Action::Submit {
                source: "effect".into(),
                expected: view.source.clone(),
            },
            Context::from_snapshot(&view),
        ));
        assert!(completion_is_current(&view, &reply));
        assert!(reply.snapshot.unwrap().reconciliation_required);
        assert!(reply.image_uri.is_none());
        assert!(reply.message.contains("host attempt failed"));
        worker.finish().unwrap();
    }

    #[test]
    fn completed_image_cannot_follow_a_new_source_or_session_snapshot() {
        let (host, _) = fixture(None);
        let old = host.snapshot.clone();
        let image = || {
            Some(BoundImage {
                context: Context::from_snapshot(&old),
                uri: "old-frame".into(),
            })
        };
        let mut message = String::new();
        assert_eq!(
            image_for_snapshot(image(), Some(&old), true, &mut message).as_deref(),
            Some("old-frame")
        );
        let mut new_session = old.clone();
        new_session.session_revision += 1;
        assert!(image_for_snapshot(image(), Some(&new_session), true, &mut message).is_none());
        // Same artifact id and numeric revision replacement still changes incarnation.
        let artifact = studio_store::get_artifact(&old.artifact_id).unwrap();
        studio_store::upsert_artifact(artifact);
        let (_, stamp) = studio_store::get_artifact_stamped(&old.artifact_id).unwrap();
        let mut replacement = old.clone();
        replacement.source.stamp = stamp;
        assert!(image_for_snapshot(image(), Some(&replacement), true, &mut message).is_none());
        assert!(image_for_snapshot(image(), Some(&old), false, &mut message).is_none());
        assert!(message.contains("frame source/session changed; withheld"));
    }

    #[test]
    fn publication_rechecks_actual_store_after_worker_completion() {
        let (host, _) = fixture(None);
        let view = host.snapshot.clone();
        let reply = Reply {
            origin: Context::from_snapshot(&view),
            snapshot: Some(view.clone()),
            message: String::new(),
            image_uri: Some("frame".into()),
            current: true,
        };
        assert!(completion_is_current(&view, &reply));
        studio_store::upsert_artifact(studio_store::get_artifact(&view.artifact_id).unwrap());
        assert!(!completion_is_current(&view, &reply));
    }

    #[test]
    fn worker_panic_closes_reply_and_is_reported_when_owner_joins() {
        let gate = Arc::new(Barrier::new(2));
        let (host, calls) = fixture(Some(gate.clone()));
        let origin = Context::from_snapshot(&host.snapshot);
        let (worker, handle) = Worker::start(Box::new(host)).unwrap();
        let mut receiver = enqueue(&handle, Action::Check("panic".into()), origin);
        gate.wait();
        gate.wait();
        assert!(worker.finish().is_err());
        assert!(receiver.try_recv().is_err());
        assert_eq!(calls.lock().unwrap().len(), 1);
    }
}
