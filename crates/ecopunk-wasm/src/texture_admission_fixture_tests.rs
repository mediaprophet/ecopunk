//! Source-to-adapter regression for the actual four generated scene textures.
//! This checks admission requests, not codec execution or GPU residency.

use super::texture_admission::admit_opaque_base_colors;
use ecopunk_terrain::scene_builder_10d::build_procedural_textures;
use sha2::{Digest, Sha256};

#[test]
fn procedural_texture_admission_contract() {
    let mut textures = build_procedural_textures();
    assert_eq!(
        textures
            .iter()
            .map(|texture| texture.name)
            .collect::<Vec<_>>(),
        [
            "foliage_leaf_venation",
            "bark_fibrous_mottled",
            "riverbed_pebbles_lithology",
            "slga_soil_strata_litter",
        ]
    );
    // Reordering must not change semantics or detach identities from payloads.
    textures.reverse();
    let mut admitted = 0;
    admit_opaque_base_colors(&textures, |texture, role| {
        assert_eq!(role, 0, "{} must request sRGB colour mips", texture.name);
        assert!(texture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
        let actual_digest: [u8; 32] = Sha256::digest(&texture.png_bytes).into();
        assert_eq!(texture.sha256_bytes, actual_digest);
        assert_eq!(
            texture.rgba8.len(),
            (texture.width * texture.height * 4) as usize
        );
        assert!(texture.rgba8.chunks_exact(4).all(|pixel| pixel[3] == 255));
        admitted += 1;
        Ok::<_, &str>(true)
    })
    .unwrap();
    assert_eq!(admitted, 4);
}
