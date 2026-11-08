//! Admission of the scene builder's opaque base-colour PNG dependencies.
//!
//! `QualiaPortal::load_texture_encoded` exposes a numeric role, not a public
//! role enum. Its public contract assigns 0 to sRGB / `TextureMipSemantic::Color`,
//! matching the native scene builder's `TextureColorSpace::Srgb` / `Color` upload.
//! Keep codec, digest validation, mip generation and residency engine-owned.

/// Opaque sRGB base colour in `QualiaPortal::load_texture_encoded`.
const OPAQUE_BASE_COLOR_ROLE: u8 = 0;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum AdmissionFailure<E> {
    BackendUnavailable { index: usize },
    Upload { index: usize, source: E },
}

/// Upload all dependencies before scene submission. Neither an unavailable
/// backend nor a rejected resource may be reported as successful admission.
/// Previous successful uploads are not rolled back; callers must not submit
/// the scene after this returns an error.
pub(crate) fn admit_opaque_base_colors<T, E>(
    textures: &[T],
    mut upload: impl FnMut(&T, u8) -> Result<bool, E>,
) -> Result<(), AdmissionFailure<E>> {
    for (index, texture) in textures.iter().enumerate() {
        match upload(texture, OPAQUE_BASE_COLOR_ROLE) {
            Ok(true) => {}
            Ok(false) => return Err(AdmissionFailure::BackendUnavailable { index }),
            Err(source) => return Err(AdmissionFailure::Upload { index, source }),
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn admit_procedural_textures(
    portal: &mut qualia_core_db::QualiaPortal,
    textures: &[ecopunk_terrain::scene_builder_10d::ProceduralTextureAsset],
) -> Result<(), wasm_bindgen::JsValue> {
    use wasm_bindgen::JsValue;

    admit_opaque_base_colors(textures, |texture, role| {
        portal.load_texture_encoded(
            &texture.sha256_bytes,
            "image/png",
            &texture.png_bytes,
            role,
            0.5, // Unused for opaque colour; no alpha-mask coverage is requested.
        )
    })
    .map_err(|failure| {
        let (index, detail) = match failure {
            AdmissionFailure::BackendUnavailable { index } => (
                index,
                "GPU backend unavailable; texture was not admitted".to_owned(),
            ),
            AdmissionFailure::Upload { index, source } => {
                let detail = source.as_string().unwrap_or_else(|| format!("{source:?}"));
                (index, detail)
            }
        };
        let texture = &textures[index];
        JsValue::from_str(&format!(
            "opaque base-colour texture '{}' (sha256:{}): {detail}",
            texture.name, texture.sha256_hex
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_base_color_uses_role_zero_regardless_of_order_or_index() {
        let textures = ["soil", "foliage", "pebbles", "bark"];
        let mut uploaded = Vec::new();
        let result = admit_opaque_base_colors(&textures, |texture, role| {
            uploaded.push((*texture, role));
            Ok::<_, &str>(true)
        });
        assert_eq!(result, Ok(()));
        assert_eq!(
            uploaded,
            [("soil", 0), ("foliage", 0), ("pebbles", 0), ("bark", 0)]
        );
    }

    #[test]
    fn unavailable_backend_is_an_error_and_stops_further_uploads() {
        let mut attempted = Vec::new();
        let result = admit_opaque_base_colors(&[10, 20, 30], |texture, _| {
            attempted.push(*texture);
            Ok::<_, &str>(*texture != 20)
        });
        assert_eq!(
            result,
            Err(AdmissionFailure::BackendUnavailable { index: 1 })
        );
        assert_eq!(attempted, [10, 20]);
    }

    #[test]
    fn upload_error_preserves_source_and_stops_further_uploads() {
        // A structured source proves errors are not reduced to a success flag
        // or string by the admission control flow.
        let source = ("texture SHA-256 mismatch", [42_u8; 32]);
        let mut attempted = Vec::new();
        let result = admit_opaque_base_colors(&[10, 20, 30], |texture, _| {
            attempted.push(*texture);
            if *texture == 20 {
                Err(source)
            } else {
                Ok(true)
            }
        });
        assert_eq!(result, Err(AdmissionFailure::Upload { index: 1, source }));
        assert_eq!(attempted, [10, 20]);
    }

    #[test]
    fn empty_dependency_set_performs_no_upload() {
        let result =
            admit_opaque_base_colors::<u8, &str>(&[], |_, _| panic!("no resource to upload"));
        assert_eq!(result, Ok(()));
    }
}
