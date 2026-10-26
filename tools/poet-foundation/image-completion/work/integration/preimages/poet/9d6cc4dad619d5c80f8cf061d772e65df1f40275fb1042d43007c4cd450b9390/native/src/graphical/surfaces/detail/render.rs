use super::{DetailLimits, DetailRecipe, TextureEffect};
use crate::graphical::surfaces::{binding::digest_hex, TextureRole};
use qualia_core_db::{
    hypermedia_authoring::image::{apply_filter, BrushStroke, ImageDocument, ImageFilter, ImageLayer},
    render::{
        assets::TextureDependency,
        surface_authoring::{encode_surface_png_into, generate_surface_maps_into, SurfaceRecipe},
        texture_decode::TextureColorSpace,
    },
};
use sha2::{Digest, Sha256};

/// Pixels decoded by the existing authorized resource owner, never a source path.
pub struct DetailBase<'a> {
    pub digest: [u8; 32],
    pub width: u32,
    pub height: u32,
    pub rgba: &'a [u8],
    pub colour_space: TextureColorSpace,
}
#[derive(Debug)]
pub struct DetailOutput {
    pub overrides: Vec<(TextureRole, [u8; 32])>,
    pub dependencies: Vec<TextureDependency>,
    pub base_digest: Option<[u8; 32]>,
    /// Source lineage witness, never authority or permission.
    pub recipe_sha256: [u8; 32],
}
fn encoded(side: u32, pixels: &[u8]) -> Result<TextureDependency, String> {
    let mut png = vec![0; pixels.len() + side as usize + 4096];
    let n = encode_surface_png_into(side, pixels, &mut png)
        .map_err(|e| format!("Texture detail PNG: {e:?}"))?;
    png.truncate(n);
    Ok(TextureDependency {
        digest: Sha256::digest(&png).into(),
        mime_type: "image/png".into(),
        bytes: png,
    })
}
/// Initial imported profile is opaque, sRGB base colour, exact square extent.
/// Canonical paint/composite operates on encoded samples, not linear-light RGB.
pub fn render_detail(
    recipe: &DetailRecipe,
    base: Option<DetailBase<'_>>,
    limits: DetailLimits,
) -> Result<DetailOutput, String> {
    recipe.validate(limits)?;
    let side = recipe.side();
    let len = side as usize * side as usize * 4;
    let mut maps = Vec::new();
    let base_digest = match recipe {
        DetailRecipe::Paint(p) => {
            let b = base.ok_or("Imported detail needs authorized original base pixels")?;
            if digest_hex(&b.digest) != p.base_sha256
                || b.width != side
                || b.height != side
                || b.rgba.len() != len
                || b.colour_space != TextureColorSpace::Srgb
                || b.rgba.chunks_exact(4).any(|p| p[3] != 255)
            {
                return Err(
                    "Imported detail base digest/extent/sRGB/opaque profile mismatch".into(),
                );
            }
            let mut document = ImageDocument::new("poet-detail", side, side);
            let original = document.add_layer("Original immutable imported base");
            for (dst, src) in original.pixels.iter_mut().zip(b.rgba.chunks_exact(4)) {
                dst.copy_from_slice(src);
            }
            for l in &p.layers {
                let layer = document.add_layer(&l.name);
                layer.opacity = l.opacity;
                layer.visible = l.visible;
                // Replay hidden layers too, preserving authored operation semantics.
                for s in &l.strokes {
                    let mut stroke = BrushStroke::new(s.size, s.colour);
                    stroke.opacity = s.opacity;
                    stroke.hardness = s.hardness;
                    for point in &s.points {
                        stroke.add_point(point[0], point[1]);
                    }
                    stroke.apply_to(layer);
                }
            }
            let pixels = document.composite();
            let rgba: Vec<u8> = if p.effects.is_empty() {
                pixels.iter().flatten().copied().collect()
            } else {
                // One temporary layer, never attached to the immutable document/source.
                let mut filtered = ImageLayer::new("poet-effects", "Post-composite effects", side, side);
                filtered.pixels.copy_from_slice(&pixels);
                for step in &p.effects {
                    if !step.enabled {
                        continue;
                    }
                    let (filter, intensity) = match step.effect {
                        TextureEffect::Grayscale => (ImageFilter::Grayscale, 1.0),
                        TextureEffect::Invert => (ImageFilter::Invert, 1.0),
                        TextureEffect::Sepia { intensity } => (ImageFilter::Sepia, intensity),
                    };
                    apply_filter(&mut filtered, filter, intensity);
                }
                filtered.pixels.iter().flatten().copied().collect()
            };
            maps.push((TextureRole::BaseColor, encoded(side, &rgba)?));
            Some(b.digest)
        }
        DetailRecipe::Procedural(p) => {
            if base.is_some() {
                return Err("Procedural detail has no imported base input".into());
            }
            let mut colour = vec![0; len];
            let mut normal = vec![0; len];
            let mut orm = vec![0; len];
            generate_surface_maps_into(
                SurfaceRecipe {
                    seed: p.seed,
                    tint: p.tint,
                    contrast: p.contrast,
                    relief: p.relief,
                    bands: p.bands,
                    band_strength: p.band_strength,
                    roughness: p.roughness,
                    metallic: p.metallic,
                },
                side,
                &mut colour,
                &mut normal,
                &mut orm,
            )
            .map_err(|e| format!("Canonical procedural detail: {e:?}"))?;
            maps.push((TextureRole::BaseColor, encoded(side, &colour)?));
            maps.push((TextureRole::Normal, encoded(side, &normal)?));
            // ORM's R=1 also explicitly supplies the separate AO map role.
            maps.push((TextureRole::MetallicRoughness, encoded(side, &orm)?));
            None
        }
    };
    let mut overrides: Vec<_> = maps
        .iter()
        .map(|(role, dependency)| (*role, dependency.digest))
        .collect();
    if let Some((_, digest)) = overrides
        .iter()
        .find(|(r, _)| *r == TextureRole::MetallicRoughness)
        .copied()
    {
        overrides.push((TextureRole::Occlusion, digest));
    }
    let mut dependencies: Vec<TextureDependency> = Vec::new();
    for (_, dependency) in maps {
        if !dependencies.iter().any(|d| d.digest == dependency.digest) {
            dependencies.push(dependency);
        }
    }
    Ok(DetailOutput {
        overrides,
        dependencies,
        base_digest,
        recipe_sha256: recipe.digest()?,
    })
}
