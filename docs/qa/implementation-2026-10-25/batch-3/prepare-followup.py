from pathlib import Path
import hashlib,json
root=Path(__file__).parent/'native-followup'
root.mkdir(exist_ok=True)
changes=[]
def edit(relative, transform):
    target=Path('C:/github/poet/native')/relative
    old=target.read_bytes() if target.exists() else None
    source=transform(old.decode() if old else '')
    candidate=root/'candidate'/relative
    candidate.parent.mkdir(parents=True,exist_ok=True)
    candidate.write_text(source,encoding='utf-8',newline='\n')
    changes.append(dict(target=str(target),candidate='candidate/'+relative,
        original_sha256=hashlib.sha256(old).hexdigest() if old else None,
        candidate_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest()))
def replace(s,a,b):
    assert s.count(a)==1,a[:90]
    return s.replace(a,b)
def controller(s):
    s=replace(s,'        super::materials::authored_scene(value, self.material, self.limits).map(|_| ())','''        if let Value::Record(record)=value {
            if record.contains_key("geometry") || record.contains_key("material") {
                return super::materials::authored_scene(value,self.material,self.limits).map(|_| ());
            }
            if record.contains_key("base_color") && record.contains_key("metallic") && record.contains_key("roughness") {
                return super::materials::material_from_value(value).map(|_| ());
            }
        }
        // Ordinary helper/function/scalar cells remain valid persistent REPL
        // submissions. A non-scene result simply cannot produce a mesh frame.
        Ok(())''')
    s=replace(s,'impl EditorHost for CanonicalEditorHost {','''impl EditorHost for CanonicalEditorHost {
    fn material_edit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome,String> {
        let snapshot=self.snapshot()?;
        let id=snapshot.material_edit_id.ok_or("coherent mutable material/geometry bindings are required")?;
        let next=self.session.revision().checked_add(1).ok_or("material revision exhausted")?;
        super::material_edit::validate(source,next,id)?;
        self.submit(source,expected)
    }''')
    return s+'\n#[cfg(test)]\n#[path="material_admission_tests.rs"]\nmod material_admission_tests;\n'
edit('src/graphical/controller.rs',controller)
edit('src/graphical/mod.rs',lambda s:replace(s,'pub mod materials;','pub mod materials;\nmod material_edit;'))
helper=Path(__file__).parent/'native-integration/candidate/src/graphical/material_edit.rs'
assert helper.exists(),'Material admission helper is not ready'
edit('src/graphical/material_edit.rs',lambda _:helper.read_text())
tests=Path(__file__).parent/'material_admission_tests.rs'
edit('src/graphical/material_admission_tests.rs',lambda _:tests.read_text())
(root/'apply-manifest.json').write_text(json.dumps(dict(scope='Reviewed material-only admission and persistent REPL compatibility',mutations=changes),indent=2))
