from pathlib import Path
import hashlib,json
root=Path(__file__).parent/'publication-integration'
root.mkdir(exist_ok=True)
changes=[]
def edit(relative, transform):
    target=Path('C:/github/poet/native')/relative
    old=target.read_bytes()
    candidate=root/'candidate'/relative
    candidate.parent.mkdir(parents=True,exist_ok=True)
    candidate.write_text(transform(old.decode()),encoding='utf-8',newline='\n')
    changes.append(dict(target=str(target),candidate='candidate/'+relative,
        original_sha256=hashlib.sha256(old).hexdigest(),candidate_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest()))
def replace(s,a,b):
    assert s.count(a)==1,a[:100]
    return s.replace(a,b)
def controller(s):
    s=replace(s,'    pub checkpoint_supported: bool,','    pub checkpoint_supported: bool,\n    pub authorisation_stamp: Option<super::cooperation::AuthorisationStamp>,')
    s=replace(s,'pub struct EditorOutcome {','''impl EditorSnapshot {
    /// Nonblocking source/execution/authority validation at publication.
    pub fn publication_is_current(&self)->bool {
        self.execution_epoch.is_current() && self.source.check_current().is_ok()
            && self.authorisation_stamp.as_ref().is_none_or(|stamp|stamp.is_current())
    }
}

pub struct EditorOutcome {''')
    s=replace(s,'                checkpoint_supported: false,','                checkpoint_supported: false,\n                authorisation_stamp: None,')
    return replace(s,'\n            checkpoint_supported: false,','\n            checkpoint_supported: false,\n            authorisation_stamp: None,')
edit('src/graphical/controller.rs',controller)
edit('src/graphical/jobs_tests.rs',lambda s:replace(s,'        checkpoint_supported: false,','        checkpoint_supported: false,\n        authorisation_stamp: None,'))
def jobs(s):
    s=replace(s,'    pub reconciliation_required: bool,','    pub reconciliation_required: bool,\n    pub authorisation_stamp: Option<super::cooperation::AuthorisationStamp>,')
    s=replace(s,'            reconciliation_required: snapshot.reconciliation_required,','            reconciliation_required: snapshot.reconciliation_required,\n            authorisation_stamp: snapshot.authorisation_stamp.clone(),')
    s=replace(s,'        if !refresh { origin.source.check_current()?; }','''        if !refresh {
            origin.source.check_current()?;
            if origin.authorisation_stamp.as_ref().is_some_and(|stamp|!stamp.is_current()) {
                return Err("Native authority changed; no command dispatched. Refresh authorised source.".into());
            }
        }''')
    s=replace(s,'        snapshot.execution_epoch.is_current()\n            && snapshot.source.check_current().is_ok()','        snapshot.publication_is_current()')
    s=replace(s,'        && reply.current\n','        && reply.current\n        && current_view.publication_is_current()\n')
    return replace(s,'            snapshot.execution_epoch.is_current()\n                && host','            snapshot.publication_is_current()\n                && host')
edit('src/graphical/jobs.rs',jobs)
edit('src/graphical/ui.rs',lambda s:replace(s,'            && self.checkpoint_supported == other.checkpoint_supported','            && self.checkpoint_supported == other.checkpoint_supported\n            && self.authorisation_stamp == other.authorisation_stamp'))
def mcp(s):
    s=replace(s,'        snapshot.source.check_current().map_err(editor_error)?;','''        if !snapshot.publication_is_current() { return Err(failure("stale_authority", "Native publication authority/source is no longer current")); }''')
    s=replace(s,'        let expected = snapshot.clone();','''        let expected = snapshot.clone();
        if !expected.publication_is_current() { return Err(failure("stale_authority", "Native publication authority/source is no longer current")); }''')
    s=replace(s,'fn outcome(result: EditorOutcome) -> Value {','''fn receipt_authority(stamp:&Option<super::cooperation::AuthorisationStamp>)->Result<(),ToolError> {
    if stamp.as_ref().is_none_or(|stamp|!stamp.is_current()) {
        return Err(failure("stale_authority", "Native authority changed after the operation; effects may have committed; read before retrying"));
    }
    Ok(())
}
fn outcome(result: EditorOutcome, stamp:&Option<super::cooperation::AuthorisationStamp>) -> Result<Value,ToolError> {
    receipt_authority(stamp)?;
    if !result.snapshot.execution_epoch.is_current() || result.snapshot.source.check_current().is_err() {
        return Err(failure("stale_context", "Native completion is stale; effects may have committed; read before retrying"));
    }''')
    s=replace(s,'    json!({"source_revision":result.snapshot.source_revision,','    Ok(json!({"source_revision":result.snapshot.source_revision,')
    s=replace(s,'        "read_required":true})\n}','        "read_required":true}))\n}')
    s=replace(s,'                let message=self.host()?.check(&edit.source).map_err(editor_error)?;\n                return Ok(json!({"checked":true,"executed":false,"message":message}));','''                self.host()?.check(&edit.source).map_err(editor_error)?;
                receipt_authority(&expected.authorisation_stamp)?;
                return Ok(json!({"checked":true,"executed":false,"message":"Canonical check completed; no execution attempted"}));''')
    s=replace(s,'            let result=if name=="poet_material_apply" {','            let authority=expected.authorisation_stamp.clone();\n            let result=if name=="poet_material_apply" {')
    s=replace(s,'            return result.map(outcome).map_err(editor_error);','            return result.map_err(editor_error).and_then(|result|outcome(result,&authority));')
    s=replace(s,'            let receipt=self.host()?.save_checkpoint(&save.name,file,expected.source).map_err(editor_error)?;','''            let authority=expected.authorisation_stamp.clone();
            let receipt=self.host()?.save_checkpoint(&save.name,file,expected.source).map_err(editor_error)?;
            receipt_authority(&authority)?;''')
    s=replace(s,'            if name=="poet_execution_rebuild" {\n                return self.host()?.rebuild_execution(expected.source).map(outcome).map_err(editor_error);','''            let authority=expected.authorisation_stamp.clone();
            if name=="poet_execution_rebuild" {
                return self.host()?.rebuild_execution(expected.source).map_err(editor_error).and_then(|result|outcome(result,&authority));''')
    return replace(s,'            let snapshot=result.map_err(editor_error)?;','            let snapshot=result.map_err(editor_error)?;\n            receipt_authority(&authority)?;')
edit('src/graphical/mcp/mod.rs',mcp)
helper=Path(__file__).parent/'native-integration/candidate/src/graphical/material_edit.rs'
edit('src/graphical/material_edit.rs',lambda _:helper.read_text())
(root/'apply-manifest.json').write_text(json.dumps(dict(scope='Native source/execution/authority publication guards and material-parser ingress hardening',mutations=changes),indent=2))
