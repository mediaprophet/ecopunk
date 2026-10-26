//! Public API qualification vectors for the nine blend modes, masks and offsets.
use qualia_core_db::hypermedia_authoring::image::{
    composite_into, validate_document, BlendMode, ImageDocument, ImageError, ImageLayer, ImageLimits,
};

fn limits() -> ImageLimits {
    ImageLimits { max_pixels: 64, max_work: 256, max_bytes: 1024 }
}

fn layer(id: &str, rgba: [u8; 4]) -> ImageLayer {
    let mut layer = ImageLayer::new(id, id, 1, 1);
    layer.pixels[0] = rgba;
    layer
}

fn pair(backdrop: [u8; 4], source: [u8; 4]) -> ImageDocument {
    let mut doc = ImageDocument::new("vector", 1, 1);
    doc.layers.push(layer("backdrop", backdrop));
    doc.layers.push(layer("source", source));
    doc
}

fn pixel(doc: &ImageDocument) -> [u8; 4] {
    let mut out = [[0; 4]];
    composite_into(doc, &mut out, limits()).unwrap();
    out[0]
}

#[test]
fn opaque_known_color_vectors_for_each_non_normal_mode() {
    // Independently evaluated W3C separable equations for Cb=(64,128,192)
    // and Cs=(192,96,32), both opaque. The modes must yield distinct results.
    let vectors = [
        (BlendMode::Multiply, [48, 48, 24, 255]),
        (BlendMode::Screen, [208, 176, 200, 255]),
        (BlendMode::Overlay, [96, 97, 145, 255]),
        (BlendMode::SoftLight, [96, 112, 156, 255]),
        (BlendMode::HardLight, [161, 96, 48, 255]),
        (BlendMode::ColorDodge, [255, 205, 220, 255]),
        (BlendMode::ColorBurn, [1, 0, 0, 255]),
        (BlendMode::Difference, [128, 32, 160, 255]),
        (BlendMode::Exclusion, [160, 128, 176, 255]),
    ];
    for (mode, expected) in vectors {
        let mut doc = pair([64, 128, 192, 255], [192, 96, 32, 255]);
        doc.layers[1].blend_mode = mode;
        assert_eq!(pixel(&doc), expected, "{mode:?}");
    }
}

#[test]
fn alpha_interpolation_and_transparent_backdrop() {
    let mut doc = pair([64, 128, 192, 128], [192, 96, 32, 128]);
    doc.layers[1].blend_mode = BlendMode::Multiply;
    assert_eq!(pixel(&doc), [101, 91, 83, 192]);
    doc.layers[0].pixels[0] = [250, 1, 2, 0];
    assert_eq!(pixel(&doc), [192, 96, 32, 128]);
}

#[test]
fn dodge_burn_endpoint_rules() {
    let mut doc = pair([0, 255, 128, 255], [255, 0, 255, 255]);
    doc.layers[1].blend_mode = BlendMode::ColorDodge;
    assert_eq!(pixel(&doc), [0, 255, 255, 255]);
    doc.layers[1].blend_mode = BlendMode::ColorBurn;
    assert_eq!(pixel(&doc), [0, 255, 128, 255]);
}

#[test]
fn mask_coverage_and_opacity_are_applied_once() {
    let mut doc = pair([64, 128, 192, 128], [192, 96, 32, 128]);
    doc.layers[1].mask = Some(vec![0]);
    assert_eq!(pixel(&doc), [64, 128, 192, 128]);
    doc.layers[1].mask = Some(vec![255]);
    doc.layers[1].opacity = 0.5;
    assert_eq!(pixel(&doc), [115, 115, 128, 160]);
    doc.layers[1].mask = Some(vec![128]);
    assert_eq!(pixel(&doc), [93, 121, 156, 144]);
}

#[test]
fn signed_offsets_clip_in_layer_coordinates() {
    let mut doc = ImageDocument::new("offset", 2, 1);
    let mut layer = ImageLayer::new("l", "l", 2, 1);
    layer.pixels = vec![[255, 0, 0, 255], [0, 255, 0, 255]];
    layer.offset_x = -1;
    doc.layers.push(layer);
    let mut out = [[0; 4]; 2];
    composite_into(&doc, &mut out, limits()).unwrap();
    assert_eq!(out, [[0, 255, 0, 255], [0; 4]]);
    doc.layers[0].offset_x = i32::MIN;
    composite_into(&doc, &mut out, limits()).unwrap();
    assert_eq!(out, [[0; 4]; 2]);
    doc.layers[0].offset_x = i32::MAX;
    composite_into(&doc, &mut out, limits()).unwrap();
    assert_eq!(out, [[0; 4]; 2]);
    doc.layers[0].offset_x = 1;
    composite_into(&doc, &mut out, limits()).unwrap();
    assert_eq!(out, [[0; 4], [255, 0, 0, 255]]);
}

#[test]
fn malformed_hidden_layers_and_all_budget_failures_leave_output_intact() {
    let mut doc = pair([1, 2, 3, 255], [4, 5, 6, 255]);
    let sentinel = [17, 18, 19, 20];
    let mut out = [sentinel];
    doc.layers[1].visible = false;
    doc.layers[1].mask = Some(vec![]);
    assert_eq!(composite_into(&doc, &mut out, limits()), Err(ImageError::Storage));
    assert_eq!(out[0], sentinel);
    doc.layers[1].mask = None;
    doc.layers[1].pixels.clear();
    assert_eq!(composite_into(&doc, &mut out, limits()), Err(ImageError::Storage));
    assert_eq!(out[0], sentinel);
    doc.layers[1].pixels.push([4, 5, 6, 255]);
    doc.layers[1].opacity = f32::NAN;
    assert_eq!(composite_into(&doc, &mut out, limits()), Err(ImageError::Parameters));
    assert_eq!(out[0], sentinel);
    doc.layers[1].opacity = 1.0;
    let tight = ImageLimits { max_pixels: 64, max_work: 2, max_bytes: 1024 };
    assert_eq!(composite_into(&doc, &mut out, tight), Err(ImageError::Budget));
    assert_eq!(out[0], sentinel);
    let tight = ImageLimits { max_pixels: 64, max_work: 256, max_bytes: 11 };
    assert_eq!(composite_into(&doc, &mut out, tight), Err(ImageError::Budget));
    assert_eq!(out[0], sentinel);
    doc.width = 0;
    assert_eq!(composite_into(&doc, &mut out, limits()), Err(ImageError::Dimensions));
    assert_eq!(out[0], sentinel);
    doc.width = u32::MAX;
    doc.height = u32::MAX;
    assert_eq!(validate_document(&doc, limits()), Err(ImageError::Dimensions));
    assert_eq!(out[0], sentinel);
}

#[test]
fn output_size_refusal_and_legacy_normal_rounding() {
    let doc = pair([64, 128, 192, 128], [192, 96, 32, 128]);
    let mut wrong = [[7; 4]; 2];
    assert_eq!(composite_into(&doc, &mut wrong, limits()), Err(ImageError::Output));
    assert_eq!(wrong, [[7; 4]; 2]);
    // Existing source_over::rgba, composed from transparent, has this exact
    // integer rounding for the partial-alpha pair.
    assert_eq!(pixel(&doc), [149, 107, 85, 192]);
}
