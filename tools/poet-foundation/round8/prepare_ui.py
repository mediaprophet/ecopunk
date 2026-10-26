"""Stage exact native UI draft preservation changes; does not write siblings."""
from pathlib import Path
import hashlib,json

ROOT=Path(__file__).resolve().parent / 'ui'
assert not (ROOT/'manifest.json').exists(), 'One-shot staging already prepared; retain reviewed images and make a separate repair group'
LIVE=Path('C:/github/poet')
FILES=['native/src/graphical/draft_buffer.rs','native/src/graphical/ui.rs','native/src/graphical/authoring/controls.rs']
def sha(b): return hashlib.sha256(b).hexdigest()
def replace(s,a,b):
    assert s.count(a)==1, (a,s.count(a))
    return s.replace(a,b)
entries=[]
for name in FILES:
    raw=(LIVE/name).read_bytes()
    s=raw.decode().replace('\r\n','\n')
    if name.endswith('draft_buffer.rs'):
        s=replace(s,'pub struct DraftBuffer(Arc<Mutex<String>>);','''pub struct DraftBuffer(Arc<Mutex<DraftState>>);
struct DraftState { text: String, revision: u64 }

/// A completion may adopt accepted text only if no local edit occurred meanwhile.
/// The revision also detects typing followed by undoing back to the same string.
pub(crate) struct DraftCapture { text: String, revision: Option<u64> }
impl DraftCapture {
    pub fn capture(text: String, buffer: Option<&DraftBuffer>) -> Result<Self, String> {
        let revision = if let Some(buffer) = buffer {
            let state = buffer.0.lock().map_err(|_| "Local draft buffer unavailable")?;
            if state.text != text || state.revision == u64::MAX {
                return Err("Local draft changed; retry after refreshing the editor".into());
            }
            Some(state.revision)
        } else { None };
        Ok(Self { text, revision })
    }
    pub fn accept(&self, succeeded: bool, visible: &str, next: &str, buffer: Option<&DraftBuffer>) -> Option<String> {
        if !succeeded || visible != self.text { return None; }
        match (self.revision, buffer) {
            (Some(revision), Some(buffer)) => {
                let mut state = buffer.0.lock().ok()?;
                if state.revision != revision || state.text != self.text { return None; }
                let next_revision = revision.checked_add(1)?;
                state.text = next.to_owned();
                state.revision = next_revision;
            }
            (None, None) => {}
            _ => return None,
        }
        Some(next.to_owned())
    }
}''')
        s=replace(s,'Self(Arc::new(Mutex::new(text)))','Self(Arc::new(Mutex::new(DraftState { text, revision: 0 })))')
        s=replace(s,'.map(|s| s.clone())','.map(|s| s.text.clone())')
        s=replace(s,'*value = text;','''// Exhaustion fails closed for automatic completion adoption. Explicit
            // local typing remains retained even at the revision ceiling.
            value.revision = value.revision.saturating_add(1);
            value.text = text;''')
        s=replace(s,'    struct NoPixels;','''    #[test]
    fn pending_success_adopts_text_only_for_unchanged_private_draft() {
        let buffer = DraftBuffer::new("accepted".into());
        let capture = DraftCapture::capture("accepted".into(), Some(&buffer)).unwrap();
        assert!(capture.accept(false, "accepted", "refused", Some(&buffer)).is_none());
        assert_eq!(buffer.read().unwrap(), "accepted");
        assert_eq!(capture.accept(true, "accepted", "next", Some(&buffer)).unwrap(), "next");
        assert_eq!(buffer.read().unwrap(), "next");
        assert!(capture.accept(true, "accepted", "late", Some(&buffer)).is_none());
    }
    #[test]
    fn pending_completion_preserves_typing_and_detects_edit_then_undo() {
        let buffer = DraftBuffer::new("accepted".into());
        let capture = DraftCapture::capture("accepted".into(), Some(&buffer)).unwrap();
        buffer.set("new private work".into());
        assert!(capture.accept(true, "new private work", "next", Some(&buffer)).is_none());
        assert_eq!(buffer.read().unwrap(), "new private work");
        buffer.set("accepted".into());
        assert!(capture.accept(true, "accepted", "next", Some(&buffer)).is_none());
        assert_eq!(buffer.read().unwrap(), "accepted");
        assert!(DraftCapture::capture("different visible text".into(), Some(&buffer)).is_err());
    }
    #[test]
    fn standalone_completion_and_buffer_context_fail_closed() {
        let capture = DraftCapture::capture("accepted".into(), None).unwrap();
        assert!(capture.accept(false, "accepted", "next", None).is_none());
        assert!(capture.accept(true, "private", "next", None).is_none());
        assert_eq!(capture.accept(true, "accepted", "next", None).unwrap(), "next");
        let buffer = DraftBuffer::new("accepted".into());
        assert!(capture.accept(true, "accepted", "next", Some(&buffer)).is_none());
    }
    struct NoPixels;''')
    elif name.endswith('/ui.rs'):
        s=replace(s,'    busy.set(true);\n    image.set(None);','''    let preserve_draft = replace_draft && draft() != view().latest_source;
    let capture = if replace_draft && !preserve_draft {
        match super::draft_buffer::DraftCapture::capture(draft(), draft_buffer.as_ref()) {
            Ok(capture) => Some(capture),
            Err(error) => { status.set(error); return; }
        }
    } else { None };
    busy.set(true);''')
        s=replace(s,'    let preserve_draft = replace_draft && draft() != view().latest_source;\n    let guard', '    let guard')
        s=replace(s,'''                if let Some(snapshot) = reply.snapshot {
                    if replace_draft && !preserve_draft {
                        if let Some(buffer) = &draft_buffer {
                            buffer.set(snapshot.latest_source.clone());
                        }
                        draft.set(snapshot.latest_source.clone());
                    }
                    view.set(snapshot);
                }
                if reply.image_uri.is_some() {
                    image.set(reply.image_uri);
                }
                status.set(if preserve_draft {''','''                let mut retained_local_draft = preserve_draft;
                let mut context_changed = false;
                if let Some(snapshot) = reply.snapshot {
                    context_changed = snapshot.source != view().source
                        || snapshot.execution_epoch != view().execution_epoch
                        || snapshot.reconciliation_required;
                    if let Some(capture) = &capture {
                        if let Some(text) = capture.accept(reply.operation_succeeded, &draft(), &snapshot.latest_source, draft_buffer.as_ref()) {
                            draft.set(text);
                        } else if reply.operation_succeeded { retained_local_draft = true; }
                    }
                    view.set(snapshot);
                }
                if reply.image_uri.is_some() || context_changed {
                    image.set(reply.image_uri);
                }
                status.set(if retained_local_draft {''')
        # A transport error does not itself change the accepted source. Late
        # context failures still clear the image in the Ok(_) branch above.
        s=replace(s,'''            Err(error) => {
                image.set(None);
                status.set(error);
            }''','''            Err(error) => { status.set(error); }''')
    else:
        s=replace(s,'    let origin = Context::from_snapshot(&view());','''    let capture = if edit {
        match crate::graphical::draft_buffer::DraftCapture::capture(draft(), buffer.as_ref()) {
            Ok(capture) => Some(capture),
            Err(error) => { status.set(error); return; }
        }
    } else { None };
    let origin = Context::from_snapshot(&view());''')
        s=replace(s,'''                if let Some(snapshot) = reply.snapshot {
                    if edit {
                        if let Some(buffer) = buffer {
                            buffer.set(snapshot.latest_source.clone());
                        }
                        draft.set(snapshot.latest_source.clone());
                    }
                    view.set(snapshot);
                }
                if edit || epoch_changed {''','''                let mut retained_local_draft = false;
                let mut source_changed = false;
                if let Some(snapshot) = reply.snapshot {
                    source_changed = snapshot.source != view().source || snapshot.reconciliation_required;
                    if let Some(capture) = &capture {
                        if let Some(text) = capture.accept(reply.operation_succeeded, &draft(), &snapshot.latest_source, buffer.as_ref()) {
                            draft.set(text);
                        } else if reply.operation_succeeded { retained_local_draft = true; }
                    }
                    view.set(snapshot);
                }
                if source_changed || epoch_changed {''')
        s=replace(s,'                status.set(reply.message);','''                status.set(if retained_local_draft {
                    format!("{}; newer unsubmitted draft preserved", reply.message)
                } else { reply.message });''')
        s=replace(s,'Parameter rebuild regenerates parts from the retained recipe; explicit part overrides are replaced. Undo preserves the earlier source.','Parameters regenerate the recorded procedural base. Set up poet_parameter_parts in the recipe before part/surface edits. Later edits are preserved by refusing regeneration; merge them explicitly into source or create a separate derivation. UV/detail remapping is not yet qualified.')
    before=ROOT/'before'/'poet'/name
    after=ROOT/'after'/'poet'/name
    before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
    before.write_bytes(raw); after.write_text(s,encoding='utf-8',newline='\n')
    entries.append(dict(repository='poet',target=name,before_sha256=sha(raw),after_sha256=sha(after.read_bytes())))
(ROOT/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',scope='round8-ui-draft-preservation',files=entries),indent=2)+'\n')
print(json.dumps(dict(staged=len(entries),sibling_writes=0)))
