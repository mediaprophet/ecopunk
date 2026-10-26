from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent;base=Path('C:/github/poet/native/src/graphical');changes={}
def read(rel):
 if rel not in changes:
  p=base/rel;b=p.read_bytes()if p.exists()else None;changes[rel]=[b,b.decode('utf-8-sig')if b is not None else '']
 return changes[rel][1]
def replace(rel,old,new):
 s=read(rel)
 if s.count(old)!=1:raise RuntimeError('Unexpected source anchor '+rel+': '+old[:40])
 changes[rel][1]=s.replace(old,new)
read('native_dispatch.rs');changes['native_dispatch.rs'][1]=(root/'root-stage/native_dispatch.rs').read_text()
replace('mod.rs','pub mod preview;','pub mod preview;\npub mod poet_workspace;\nmod native_dispatch;')
replace('authoring/controls.rs','match jobs::run(host, action, origin).await {','match crate::graphical::native_dispatch::run(host, action, origin).await {')
replace('ui.rs','match jobs::run(host, action, origin).await {','match super::native_dispatch::run(host, action, origin).await {')
replace('geometry_host.rs','    pub(crate) fn actual_grants(&self) -> &[String] {','    pub(crate) fn vertex_limit(&self) -> usize { self.max_vertices }\n    pub(crate) fn actual_grants(&self) -> &[String] {')
replace('authoring/controller.rs','            crate::graphical::preview::PreviewLimits::default(),','            crate::graphical::preview::PreviewLimits {side:512,vertices:self.host.vertex_limit(),indices:self.host.vertex_limit().checked_mul(6).ok_or("Compiled assembly index budget overflow")?},')
replace('authoring/controller.rs','serde_json::to_vec(&self.source_cells(expected.clone())?).map_err(|e| e.to_string())?','self.source_cells(expected.clone())?.join("\n\n").into_bytes()')
replace('authoring/controller.rs','"vibe-source-cells-json"','"vibe"')
# A selected native projection of the ORIGINAL Studio seed, never a new product shell.
replace('ui.rs','    let current = view();','    let current = view();\n    let workspace = super::poet_workspace::WorkspaceProjection::for_source(&current.artifact_id).ok();')
replace('ui.rs','                strong { "Studio" }\n                p { "Manifold · source and scene" }','''                if let Some(workspace)=&workspace {
                    strong { "{workspace.manifold.label}" }
                    p { "Construct · {workspace.construct.honesty}" }
                    for container in &workspace.manifold.containers {
                        p { style:"font-size:12px;", "{container.title} · {container.honesty}" }
                    }
                }
                p { "Selected native 3D projection of Poet. Other tools retain their existing definitions and port status." }''')
rows=[];backup=root/'poet-projection-before'
for rel,(before,s) in changes.items():
 p=base/rel
 if before is None and p.exists():raise RuntimeError('New source exists '+rel)
 if before is not None and p.read_bytes()!=before:raise RuntimeError('Concurrent edit '+rel)
 if before is not None:
  b=backup/rel;b.parent.mkdir(parents=True,exist_ok=True)
  if b.exists():raise RuntimeError('Backup exists '+rel)
  b.write_bytes(before)
 after=s.encode();p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(after)
 rows.append(dict(path=str(p),before_sha256=hashlib.sha256(before).hexdigest()if before is not None else None,after_sha256=hashlib.sha256(after).hexdigest()))
(root/'poet-projection-saved.json').write_text(json.dumps(rows,indent=2)+'\n')
print('Saved existing Poet IntentBus/manifold integration and host-selected authoring budgets')
