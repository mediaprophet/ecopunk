//! Native texture controls emit a source recipe through the existing intent owner.
use crate::graphical::surfaces::detail::{
    DetailLimits, DetailRecipe, PaintLayer, PaintRecipe, PaintStroke, ProceduralRecipe,
};
use dioxus::prelude::*;

const STROKE_FIELDS: [&str; 5] = [
    "Diameter in pixels",
    "RGB bytes, comma separated",
    "Stroke opacity",
    "Hardness",
    "Dab points x,y; x,y (+Y down)",
];
const PROCEDURAL_FIELDS: [&str; 12] = [
    "Side",
    "Seed",
    "Tint red",
    "Tint green",
    "Tint blue",
    "Contrast",
    "Relief",
    "Bands U",
    "Bands V",
    "Band strength",
    "Roughness",
    "Metallic",
];
fn f32_value(s: &str) -> Result<f32, String> {
    let v = s
        .trim()
        .parse::<f32>()
        .map_err(|_| "Enter a finite number".to_owned())?;
    if !v.is_finite() {
        return Err("Enter a finite number".into());
    }
    Ok(v)
}
fn stroke(fields: &[String]) -> Result<PaintStroke, String> {
    if fields.len() != 5 || fields[4].len() > 32 * 1024 {
        return Err("Incomplete or oversized stroke fields".into());
    }
    let colour = fields[1]
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u8>()
                .map_err(|_| "RGB must be three bytes".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    if colour.len() != 3 {
        return Err("RGB must be three bytes".into());
    }
    let mut points = Vec::new();
    for point in fields[4].split(';') {
        if points.len() >= 4096 {
            return Err("Dab point bound exceeded".into());
        }
        let coordinates = point
            .split(',')
            .map(f32_value)
            .collect::<Result<Vec<_>, _>>()?;
        if coordinates.len() != 2 {
            return Err("Each dab needs x,y".into());
        }
        points.push([coordinates[0], coordinates[1]]);
    }
    Ok(PaintStroke {
        size: f32_value(&fields[0])?,
        colour: [colour[0], colour[1], colour[2], 255],
        opacity: f32_value(&fields[2])?,
        hardness: f32_value(&fields[3])?,
        points,
    })
}
fn stroke_fields(s: &PaintStroke) -> Vec<String> {
    vec![
        s.size.to_string(),
        format!("{},{},{}", s.colour[0], s.colour[1], s.colour[2]),
        s.opacity.to_string(),
        s.hardness.to_string(),
        s.points
            .iter()
            .map(|p| format!("{},{}", p[0], p[1]))
            .collect::<Vec<_>>()
            .join(";"),
    ]
}
fn procedural(xs: &[String]) -> Result<DetailRecipe, String> {
    if xs.len() != 12 {
        return Err("Incomplete procedural fields".into());
    }
    let integer = |i: usize| {
        xs[i]
            .trim()
            .parse::<u32>()
            .map_err(|_| "Side, seed and bands must be whole unsigned numbers".to_owned())
    };
    let p = DetailRecipe::Procedural(ProceduralRecipe {
        version: 1,
        side: integer(0)?,
        seed: integer(1)?,
        tint: [f32_value(&xs[2])?, f32_value(&xs[3])?, f32_value(&xs[4])?],
        contrast: f32_value(&xs[5])?,
        relief: f32_value(&xs[6])?,
        bands: [integer(7)?, integer(8)?],
        band_strength: f32_value(&xs[9])?,
        roughness: f32_value(&xs[10])?,
        metallic: f32_value(&xs[11])?,
    });
    p.validate(DetailLimits::default())?;
    Ok(p)
}
fn procedural_fields(p: &ProceduralRecipe) -> Vec<String> {
    vec![
        p.side.to_string(),
        p.seed.to_string(),
        p.tint[0].to_string(),
        p.tint[1].to_string(),
        p.tint[2].to_string(),
        p.contrast.to_string(),
        p.relief.to_string(),
        p.bands[0].to_string(),
        p.bands[1].to_string(),
        p.band_strength.to_string(),
        p.roughness.to_string(),
        p.metallic.to_string(),
    ]
}

#[component]
pub(super) fn DetailControls(
    detail: Option<DetailRecipe>,
    context_token: String,
    fresh: bool,
    preview_uri: Option<String>,
    onapply: EventHandler<Option<DetailRecipe>>,
    oninspect: EventHandler<()>,
) -> Element {
    let mut loaded = use_signal(|| false);
    let mut loaded_context = use_signal(String::new);
    let mut loaded_base = use_signal(|| None::<DetailRecipe>);
    let mut draft = use_signal(|| None::<DetailRecipe>);
    let mut layer_index = use_signal(|| 0usize);
    let mut stroke_index = use_signal(|| 0usize);
    let mut base_digest = use_signal(String::new);
    let mut side = use_signal(|| "64".to_owned());
    let mut fields = use_signal(|| {
        vec![
            "8".into(),
            "255,64,16".into(),
            "1".into(),
            "0.8".into(),
            "32,32".into(),
        ]
    });
    let mut procedural_values = use_signal(|| {
        vec![
            "64", "7", "1", "1", "1", "0.4", "0.01", "8", "1", "0.7", "0.8", "0",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>()
    });
    let mut message = use_signal(String::new);
    let ready = fresh && loaded() && loaded_base() == detail && loaded_context() == context_token;
    let reload_context = context_token.clone();
    let paint_context = context_token.clone();
    let procedural_context = context_token.clone();
    let reload_detail = detail.clone();
    let paint_base = detail.clone();
    let proc_base = detail.clone();
    rsx! {
        fieldset {style:"margin-top:10px;border:1px solid #405264;",
            legend {"Editable texture and detail source"}
            p {"Paint edits opaque imported base colour at its exact square size. Ordered normal layers use encoded sRGB sample compositing, discrete pixel dabs and +Y down; this is not 3D painting or linear-light compositing. Procedural maps are artistic colour, tangent normal and packed ORM, not measured physical properties."}
            button {disabled:!fresh,onclick:move |_|{
                loaded_base.set(reload_detail.clone());loaded_context.set(reload_context.clone());draft.set(reload_detail.clone());loaded.set(true);layer_index.set(0);stroke_index.set(0);
                if let Some(DetailRecipe::Procedural(p))=&reload_detail {procedural_values.set(procedural_fields(p));}
                message.set("Current accepted recipe loaded; local edits are retained until explicit reload.".into());
            },"Load accepted detail recipe"}
            label {"Imported base SHA-256" input {value:"{base_digest}",maxlength:64,oninput:move |e|base_digest.set(e.value())}}
            label {"Exact square side" input {value:"{side}",maxlength:4,oninput:move |e|side.set(e.value())}}
            button {disabled:!fresh,onclick:move |_|{
                let next=side().parse::<u32>().map_err(|_|"Enter the exact whole image side".to_owned()).and_then(|side|{
                    let p=DetailRecipe::Paint(PaintRecipe {version:1,side,base_sha256:base_digest(),layers:vec![]});p.validate(DetailLimits::default())?;Ok(p)
                });
                match next {Ok(p)=>{loaded_base.set(paint_base.clone());loaded_context.set(paint_context.clone());draft.set(Some(p));loaded.set(true);layer_index.set(0);},Err(e)=>message.set(e)}
            },"Start imported paint recipe"}
            button {disabled:!fresh,onclick:move |_|match procedural(&procedural_values()) {
                Ok(p)=>{loaded_base.set(proc_base.clone());loaded_context.set(procedural_context.clone());draft.set(Some(p));loaded.set(true);},Err(e)=>message.set(e)
            },"Start procedural detail recipe"}
            if loaded() && !ready {p {"Source or selection changed. Your local draft is retained; reload the current accepted recipe before applying."}}
            if let Some(DetailRecipe::Paint(p))=draft() {
                p {"{p.layers.len()} ordered paint layers; original base remains immutable."}
                button {disabled:!ready||p.layers.len()>=8,onclick:move |_|{
                    if let Some(DetailRecipe::Paint(p))=draft.write().as_mut() {p.layers.push(PaintLayer {name:format!("Paint {}",p.layers.len()+1),visible:true,opacity:1.0,strokes:vec![]});layer_index.set(p.layers.len()-1);stroke_index.set(0);}
                },"Add paint layer"}
                for (i,l) in p.layers.iter().enumerate() {
                    div {style:"padding:4px;border:1px solid #405264;",
                        button {disabled:!ready,onclick:move |_|{layer_index.set(i);stroke_index.set(0);},"Select layer {i}"}
                        input {disabled:!ready,value:"{l.name}",maxlength:80,aria_label:"Layer {i} name",oninput:move |e|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers[i].name=e.value();}}
                        label {"Visible" input {r#type:"checkbox",disabled:!ready,checked:l.visible,onchange:move |e|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers[i].visible=e.checked();}}}
                        label {"Layer opacity" input {disabled:!ready,value:"{l.opacity}",aria_label:"Layer {i} opacity",oninput:move |e|match f32_value(&e.value()){Ok(v)=>if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers[i].opacity=v;},Err(e)=>message.set(e)}}}
                        button {disabled:!ready||i==0,onclick:move |_|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers.swap(i,i-1);layer_index.set(i-1);},"Move layer up"}
                        button {disabled:!ready||i+1==p.layers.len(),onclick:move |_|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers.swap(i,i+1);layer_index.set(i+1);},"Move layer down"}
                        button {disabled:!ready,onclick:move |_|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers.remove(i);layer_index.set(0);stroke_index.set(0);},"Remove layer"}
                    }
                }
                if let Some(layer)=p.layers.get(layer_index()) {
                    p {"Editing layer {layer_index}: {layer.name}"}
                    for (i,s) in layer.strokes.iter().enumerate() {
                        div {
                            button {disabled:!ready,onclick:move |_|{
                                if let Some(DetailRecipe::Paint(p))=draft() {if let Some(s)=p.layers.get(layer_index()).and_then(|l|l.strokes.get(i)){fields.set(stroke_fields(s));stroke_index.set(i);}}
                            },"Load stroke {i} ({s.points.len()} dabs)"}
                            button {disabled:!ready||i==0,onclick:move |_|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers[layer_index()].strokes.swap(i,i-1);stroke_index.set(i-1);},"Move stroke up"}
                            button {disabled:!ready||i+1==layer.strokes.len(),onclick:move |_|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers[layer_index()].strokes.swap(i,i+1);stroke_index.set(i+1);},"Move stroke down"}
                            button {disabled:!ready,onclick:move |_|if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers[layer_index()].strokes.remove(i);stroke_index.set(0);},"Remove stroke"}
                        }
                    }
                    for (i,label) in STROKE_FIELDS.iter().enumerate() {label {"{label}" input {disabled:!ready,value:"{fields()[i]}",maxlength:32768,aria_label:"{label}",oninput:move |e|fields.write()[i]=e.value()}}}
                    button {disabled:!ready||layer.strokes.len()>=128,onclick:move |_|match stroke(&fields()) {
                        Ok(s)=>if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){p.layers[layer_index()].strokes.push(s);stroke_index.set(p.layers[layer_index()].strokes.len()-1);},Err(e)=>message.set(e)
                    },"Add discrete stroke"}
                    button {disabled:!ready||layer.strokes.is_empty(),onclick:move |_|match stroke(&fields()) {
                        Ok(s)=>if let Some(DetailRecipe::Paint(p))=draft.write().as_mut(){if let Some(slot)=p.layers[layer_index()].strokes.get_mut(stroke_index()){*slot=s;}},Err(e)=>message.set(e)
                    },"Replace loaded stroke"}
                }
            }
            if let Some(DetailRecipe::Procedural(_))=draft() {
                for (i,label) in PROCEDURAL_FIELDS.iter().enumerate() {label {"{label}" input {disabled:!ready,value:"{procedural_values()[i]}",maxlength:32,aria_label:"Procedural {label}",oninput:move |e|procedural_values.write()[i]=e.value()}}}
                button {disabled:!ready,onclick:move |_|match procedural(&procedural_values()){Ok(p)=>draft.set(Some(p)),Err(e)=>message.set(e)},"Update procedural draft"}
            }
            button {disabled:!ready||draft().is_none(),onclick:move |_|{
                if !ready {return;}
                if let Some(p)=draft(){match p.validate(DetailLimits::default()).and_then(|_|p.source_expression().map(|_|())){Ok(())=>onapply.call(Some(p)),Err(e)=>message.set(e)}}
            },"Apply shared texture/detail recipe"}
            button {disabled:!ready,onclick:move |_|onapply.call(None),"Remove detail recipe; restore authored map bindings"}
            button {disabled:!fresh,onclick:move |_|oninspect.call(()),"Inspect accepted texture pixels"}
            if fresh {if let Some(uri)=preview_uri {img {src:uri,alt:"Current accepted derived base-colour texture pixels",style:"display:block;max-width:100%;width:320px;image-rendering:pixelated;"}}}
            p {role:"status","{message}"}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_dab_fields_preserve_order_and_reject_invalid_coordinates() {
        let fields = vec![
            "4".into(),
            "255,0,10".into(),
            "0.5".into(),
            "1".into(),
            "2,3;4,5".into(),
        ];
        let s = stroke(&fields).unwrap();
        assert_eq!(s.points, vec![[2.0, 3.0], [4.0, 5.0]]);
        assert_eq!(stroke(&stroke_fields(&s)).unwrap(), s);
        let mut invalid = fields;
        invalid[4] = "NaN,2".into();
        assert!(stroke(&invalid).is_err());
    }
}
