from pathlib import Path
import hashlib,json,re
root=Path(__file__).parent/'core-forge-repairs'
root.mkdir(exist_ok=True)
owner=Path('C:/github/qualiaDB/crates/qualia-core-db/src')
changes=[]
def edit(relative,transform):
    target=owner/relative
    old=target.read_bytes()
    candidate=root/'candidate'/relative
    candidate.parent.mkdir(parents=True,exist_ok=True)
    candidate.write_text(transform(old.decode()),encoding='utf-8',newline='\n')
    changes.append(dict(target=str(target),candidate='candidate/'+relative,original_sha256=hashlib.sha256(old).hexdigest(),candidate_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest()))
def extensions(s):
    assert 'enable wgpu_' in s
    return s.replace('enable wgpu_cooperative_matrix','enable etalon_cooperative_matrix').replace('enable wgpu_ray_query','enable etalon_ray_query')
edit('wgsl_forge/emit/coopmat.rs',extensions)
edit('wgsl_forge/emit/wgsl.rs',extensions)
edit('wgsl_forge/oracle/evaluate.rs',extensions)
def seam(s):
    marker='#[cfg(not(all(not(target_arch = "wasm32"), feature = "wgsl-forge")))]'
    starts=[m.start() for m in re.finditer(re.escape(marker),s)]
    assert len(starts)==6
    for start in reversed(starts):
        end=s.find('\n#[cfg',start+len(marker))
        assert end>start
        block=s[start:end]
        name=re.search(r'pub fn (\w+)\(',block).group(1)
        header=f'{marker}\npub fn {name}(_args_v: &Value, span: Span) -> Result<Value, Diagnostic> {{\n'
        if name=='list_kernels':
            body='''    let _ = span;
    // Discovery reports actual target availability, not a synthetic catalogue.
    Ok(args::record([
        ("op", Value::String("list_kernels".into())),
        ("id", Value::String(ids::WGSL_FORGE_LIST_KERNELS.into())),
        ("available", Value::Bool(false)),
        ("native_service_required", Value::Bool(true)),
        ("kernel_count", Value::U64(0)),
        ("kernels", Value::List(Vec::new())),
        ("viewport_pipelines", Value::List(Vec::new())),
    ]))
'''
        else:
            body=f'''    Err(args::bad(span, "WgslForge.{name} requires the enabled native WGSL Forge service; no shader, validation or execution result was produced"))
'''
        s=s[:start]+header+body+'}\n'+s[end:]
    # Use an actual listed kernel rather than an invented non-native catalogue ID.
    s=s.replace('Value::String("rmsnorm-f32".into())','Value::String("ray-probe".into())')
    assert s.count('#[cfg(test)]\nmod tests {')==1
    s=s.replace('#[cfg(test)]\nmod tests {','#[cfg(all(test, not(target_arch = "wasm32"), feature = "wgsl-forge"))]\nmod tests {')
    s+='''
#[cfg(all(test, not(all(not(target_arch = "wasm32"), feature = "wgsl-forge"))))]
mod unavailable_tests {
    use super::*;
    #[test]
    fn disabled_native_service_never_fabricates_a_kernel_or_validation() {
        let value=Value::Record(Default::default());
        let span=Span::point(0);
        let Value::Record(catalogue)=list_kernels(&value,span).unwrap() else { panic!("catalogue record") };
        assert_eq!(catalogue.get("available"),Some(&Value::Bool(false)));
        assert_eq!(catalogue.get("kernel_count"),Some(&Value::U64(0)));
        assert!(generate(&value,span).is_err());
        assert!(validate(&value,span).is_err());
        assert!(roofline(&value,span).is_err());
        assert!(schedule_candidates(&value,span).is_err());
        assert!(attention_synthesize(&value,span).is_err());
    }
}
'''
    return s
edit('poet_host/invoke/wgsl_forge_seam.rs',seam)
(root/'apply-manifest.json').write_text(json.dumps(dict(scope='Selected Etalon extension compatibility and truthful native Forge availability',mutations=changes),indent=2))
print(json.dumps(dict(staged=len(changes))))
