use super::*;
use qualia_core_db::render::skeletal::SkinWeight;

pub(super) fn copy(a:&AnatomyMeshAccumulator,extra_vertices:usize,extra_indices:usize)->Result<AnatomyMeshAccumulator,String>{
    let n=a.positions.len();
    if n>750_000 || a.indices.len()>9_000_000 || a.indices.len()%3!=0
        || [a.normals.len(),a.colors.len(),a.uvs.len(),a.layer_tags.len(),a.joint_indices.len(),a.joint_weights.len()].iter().any(|&v|v!=n) {
        return Err("Junction input stream correspondence/count invalid".into());
    }
    let mut out=AnatomyMeshAccumulator::with_numeric_profile(a.numeric_profile());
    macro_rules! stream {($field:ident,$extra:expr)=>{{
        out.$field.try_reserve_exact(sum(a.$field.len(),$extra)?).map_err(|_|"Junction detached stream allocation refused")?;
        out.$field.extend_from_slice(&a.$field);
    }}}
    stream!(positions,extra_vertices);stream!(normals,extra_vertices);stream!(colors,extra_vertices);
    stream!(uvs,extra_vertices);stream!(layer_tags,extra_vertices);stream!(joint_indices,extra_vertices);
    stream!(joint_weights,extra_vertices);stream!(indices,extra_indices);
    Ok(out)
}

pub(super) fn append(a:&mut AnatomyMeshAccumulator,p:[f32;3],uv:[f32;2],body:u32,limb:u32,t:f64,max_loss:f64)
    ->Result<(u32,f64),String>{
    let b=body as usize;let l=limb as usize;
    let weights=|index:usize|SkinWeight{joints:a.joint_indices[index],weights:a.joint_weights[index]};
    let (weight,loss)=if t==0.0{(weights(b),0.0)}else if t==1.0{(weights(l),0.0)}else{
        super::super::skin_profile::blend_weights(&[weights(b),weights(l)],&[1.0-t,t],max_loss)?
    };
    let color=if t==0.0{a.colors[b]}else if t==1.0{a.colors[l]}else{
        std::array::from_fn(|c|(f64::from(a.colors[b][c])*(1.0-t)+f64::from(a.colors[l][c])*t)as f32)
    };
    if p.iter().any(|v|!v.is_finite()||v.abs()>100.0) || color.iter().any(|v|!v.is_finite()) {
        return Err("Junction generated nonfinite/out-of-domain stream".into());
    }
    let index=u32::try_from(a.positions.len()).map_err(|_|"Junction vertex index overflow")?;
    a.positions.push(p);a.normals.push([0.0;3]);a.colors.push(color);a.uvs.push(uv);
    a.layer_tags.push(Layer::ExternalIntegument);
    a.joint_indices.push(weight.joints);a.joint_weights.push(weight.weights);
    Ok((index,loss))
}

pub(super) fn normal(p:[f32;3],q:[f32;3],r:[f32;3])->[f64;3]{
    let a=std::array::from_fn::<_,3,_>(|i|f64::from(q[i])-f64::from(p[i]));
    let b=std::array::from_fn::<_,3,_>(|i|f64::from(r[i])-f64::from(p[i]));
    [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
}
pub(super) fn dot64(a:[f64;3],b:[f64;3])->f64{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
pub(super) fn unit(a:[f64;3])->Result<[f64;3],String>{
    let length=dot64(a,a).sqrt();
    if !length.is_finite()||length<1e-15{return Err("Junction degenerate geometric direction".into())}
    Ok(a.map(|x|x/length))
}
