use super::*;
// Append to the existing zoological_loft_builder module (same private-helper scope).
// Reuses the current rich macropod construction; does not replace the scene fauna.
// Original numerical anatomy below is an authoring hypothesis pending reference review.

fn wallaby_review_smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Authored stockier/shorter-necked form in metres, +Y up, +Z forward.
/// This transformation retains the current rich builder's ears, limbs and tail;
/// it is not a measured allometric rule or taxonomic identification algorithm.
fn wallaby_review_form(p: [f32; 3]) -> [f32; 3] {
    let upper = wallaby_review_smooth((p[1] - 0.76) / 0.48);
    let neck = wallaby_review_smooth((p[1] - 0.88) / 0.44);
    let muzzle = wallaby_review_smooth((p[2] - 0.29) / 0.18)
        * wallaby_review_smooth((p[1] - 1.17) / 0.10);
    [
        p[0] * (1.00 - 0.12 * upper),
        p[1] * 0.78 - 0.06 * neck + 0.010,
        p[2] * 0.88 - 0.026 * muzzle,
    ]
}

/// Inverse-transpose normal transport for the authored nonuniform deformation.
/// Jacobian is sampled from this exact content function, not a replacement solver.
fn wallaby_review_normal(p: [f32; 3], n: [f32; 3]) -> [f32; 3] {
    let eps = 0.0002;
    let mut columns = [[0.0; 3]; 3];
    for i in 0..3 {
        let mut a = p;
        let mut b = p;
        a[i] += eps;
        b[i] -= eps;
        let a = wallaby_review_form(a);
        let b = wallaby_review_form(b);
        for j in 0..3 { columns[i][j] = (a[j] - b[j]) / (2.0 * eps); }
    }
    let cross = |a: [f32; 3], b: [f32; 3]| [
        a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0],
    ];
    let co = [cross(columns[1],columns[2]),cross(columns[2],columns[0]),cross(columns[0],columns[1])];
    let v = std::array::from_fn::<_,3,_>(|j| co[0][j]*n[0]+co[1][j]*n[1]+co[2][j]*n[2]);
    let length = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt().max(1e-8);
    v.map(|x| x / length)
}

/// Distinct swamp-wallaby-directed review candidate using existing rich native
/// fauna helpers. No species acceptance, rig, fur groom or measured body claim.
/// Estimated authored bounds x +-0.32, y 0..1.27, z -0.99..0.43 m;
/// coordinator must report the actual generated bounds after WASM execution.
pub fn build_swamp_wallaby_review_specimen(body: &mut SubmeshBuilder) {
    let first = body.positions.len();
    let triangle_start = body.indices.len();
    let dark = [0.115, 0.095, 0.078, 1.0];
    let flank = [0.24, 0.19, 0.14, 1.0];
    let warm = [0.43, 0.24, 0.105, 1.0];
    let keratin = [0.075, 0.058, 0.046, 1.0];
    // Distinct review archetype selects the existing macropod branch with denser
    // body sampling (integration patch); original kangaroo route is unchanged.
    add_lofted_fauna_specimen(body, [0.0; 3], 0.0, 1.0,
        "swamp_wallaby_review_base", dark, flank);

    let station = |center, rx, ry, color| AnatomicalLoftStation {
        center, rx, ry, dorsal_color: color, flank_color: color, ventral_color: color,
    };
    // Finger lobes continue the existing wrist/palm, retaining its folded arm.
    // Counts and landmark positions are provisional external anatomy, not a rig.
    for side in [-1.0_f32, 1.0] {
        for digit in 0..5 {
            let spread = (digit as f32 - 2.0) * 0.009;
            let length = 0.033 - (digit as f32 - 2.0).abs() * 0.005;
            let root = [side * (0.085 + spread), 0.612, 0.312];
            let mid = [root[0] + side * spread * 0.14, 0.597, 0.325 + length * 0.40];
            let tip = [mid[0], 0.587, 0.325 + length];
            let claw = [tip[0], tip[1] - 0.008, tip[2] + 0.006];
            append_anatomical_loft_sweep(body, &[
                station(root,0.0065,0.0065,dark),station(mid,0.0055,0.0055,dark),
                station(tip,0.004,0.004,keratin),station(claw,0.0012,0.0012,keratin),
            ],8,[0.0;3],0.0,1.0,true);
        }
        // Preserve the existing elongated main pes/digit. Add a smaller lateral
        // toe and the closely paired medial grooming-toe representation.
        for (offset,length,radius) in [(0.026_f32,0.090_f32,0.009_f32),(-0.025,0.052,0.005),(-0.034,0.049,0.005)] {
            let root=[side*(0.16+offset),0.035,0.105];
            let mid=[root[0],0.026,0.105+length*0.65];
            let tip=[root[0],0.020,0.105+length];
            append_anatomical_loft_sweep(body,&[
                station(root,radius,radius*0.8,dark),station(mid,radius*0.8,radius*0.65,dark),
                station(tip,radius*0.25,radius*0.2,keratin),
            ],8,[0.0;3],0.0,1.0,true);
        }
        // Eyelid ridge and cheek-to-muzzle marking follow the current face,
        // rather than floating opaque plates above the skin.
        append_anatomical_loft_sweep(body,&[
            station([side*0.058,1.341,0.305],0.006,0.006,dark),
            station([side*0.069,1.343,0.323],0.005,0.005,dark),
            station([side*0.061,1.337,0.339],0.003,0.003,dark),
        ],8,[0.0;3],0.0,1.0,true);
    }

    for i in first..body.positions.len() {
        let p=body.positions[i];
        let normal=body.normals[i];
        let old=body.colors[i];
        let is_eye=old[2]>old[0]+0.01 && old[0]<0.09;
        let is_inner_ear=p[1]>1.35 && old[0]>0.6;
        let is_dark_detail=old[0]<0.16 && old[1]<0.14 &&
            ((p[1]>1.20&&p[2]>0.39)||p[1]<0.10||(p[1]<0.66&&p[2]>0.29));
        if !is_eye && !is_dark_detail {
            let ventral=wallaby_review_smooth((normal[2]-0.1)/0.65)
                *wallaby_review_smooth((p[1]-0.35)/0.22)
                *(1.0-wallaby_review_smooth((p[1]-1.18)/0.11));
            let top=wallaby_review_smooth((normal[1]+0.15)/0.95);
            let mut color=lerp_color(flank,dark,top*0.68);
            color=lerp_color(color,warm,ventral*0.9);
            // Source-backed light cheek trait; exact mask is authored, unmeasured.
            let cheek_axis=1.277 + 0.12*(p[2]-0.27);
            let stripe=(-( (p[1]-cheek_axis)/0.016 ).powi(2)).exp()
                *wallaby_review_smooth((p[2]-0.24)/0.045)
                *(1.0-wallaby_review_smooth((p[2]-0.37)/0.055))
                *wallaby_review_smooth((p[0].abs()-0.027)/0.035);
            color=lerp_color(color,[0.62,0.47,0.29,1.0],stripe*0.9);
            if is_inner_ear {color=lerp_color([0.29,0.19,0.13,1.0],warm,0.35);}
            // Fine color breakup is a coat shading study, explicitly not a groom.
            let variation=1.0+0.045*(p[0]*137.0+p[1]*251.0).sin()*(p[2]*173.0-p[1]*91.0).sin();
            for channel in color.iter_mut().take(3){*channel=(*channel*variation).clamp(0.0,1.0);}
            body.colors[i]=color;
        }
        body.positions[i]=wallaby_review_form(p);
        body.normals[i]=wallaby_review_normal(p,normal);
    }
    // The existing extraction/loft sources have independent winding conventions.
    // Align candidate triangles against their own transported authoring normals.
    for triangle in body.indices[triangle_start..].chunks_exact_mut(3) {
        let a=body.positions[triangle[0]as usize];let b=body.positions[triangle[1]as usize];let c=body.positions[triangle[2]as usize];
        let u=std::array::from_fn::<_,3,_>(|i|b[i]-a[i]);let v=std::array::from_fn::<_,3,_>(|i|c[i]-a[i]);
        let cross=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]];
        let n=body.normals[triangle[0]as usize];
        if cross[0]*n[0]+cross[1]*n[1]+cross[2]*n[2]<0.0 {triangle.swap(1,2);}
    }
}
