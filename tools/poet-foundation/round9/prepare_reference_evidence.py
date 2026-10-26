from pathlib import Path
import json,hashlib
group=Path(__file__).resolve().parent/'reference-evidence'
name='native/src/graphical/reference_authoring_tests.rs'
raw=(Path('C:/github/poet')/name).read_bytes();text=raw.decode().replace('\r\n','\n')
old='''        assert!(
            output.status.success(),
            "reference {phase} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );'''
assert text.count(old)==1
text=text.replace(old,old+'''
        assert!(root.path().join(HMC).is_file(), "child must execute the real workload");
        assert!(root.path().join(if phase=="write" {"observed.json"} else {"reopened.json"}).is_file(), "each child phase must produce its evidence");''')
old='''    }
}
#[test]
#[ignore = "child of explicit reference qualification; requires native GPU"]'''
assert text.count(old)==1
text=text.replace(old,'''    }
    if let Some(out) = std::env::var_os("POET_REFERENCE_QUALIFICATION_ARTIFACTS") {
        let out=std::fs::canonicalize(out).unwrap();
        let qa=std::fs::canonicalize("C:/github/ecopunk/docs/qa").unwrap();
        assert!(out.starts_with(qa));
        for name in ["observed.json","reopened.json"] {
            let bytes=std::fs::read(root.path().join(name)).unwrap();
            let mut file=std::fs::OpenOptions::new().create_new(true).write(true).open(out.join(name)).unwrap();
            file.write_all(&bytes).unwrap();
        }
    }
}
#[test]
#[ignore = "child of explicit reference qualification; requires native GPU"]''')
old='''        println!(
            "{}",
            serde_json::json!({"fixture_only":true,"production_acceptance":"unassessed-HOLD","phase":"fresh-reopen","source_cells":cells.len(),"parts":h.authoring_view(frame.state.source).unwrap().parts.len(),"resources":resource_count,"elapsed_ms":timer.elapsed().as_millis()})
        );'''
assert text.count(old)==1
text=text.replace(old,'''        let reopened=serde_json::json!({"fixture_only":true,"production_acceptance":"unassessed-HOLD","phase":"fresh-reopen","source_cells":cells.len(),"parts":h.authoring_view(frame.state.source).unwrap().parts.len(),"resources":resource_count,"elapsed_ms":timer.elapsed().as_millis()});
        std::fs::write(root.join("reopened.json"), serde_json::to_vec(&reopened).unwrap()).unwrap();
        println!("{reopened}");''')
before=group/'before/poet'/name;after=group/'after/poet'/name
before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
before.write_bytes(raw);after.write_text(text,encoding='utf-8',newline='\n')
rows=[dict(repository='poet',target=name,before_sha256=hashlib.sha256(raw).hexdigest(),after_sha256=hashlib.sha256(after.read_bytes()).hexdigest())]
(group/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',files=rows),indent=2)+'\n')
print(hashlib.sha256((group/'manifest.json').read_bytes()).hexdigest())
