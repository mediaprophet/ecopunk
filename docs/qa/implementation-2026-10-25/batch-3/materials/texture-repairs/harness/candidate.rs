//! Exact proposed importer modules with actual current engine types/APIs.
pub use qualia_core_db::*;
pub mod render {
    pub use qualia_core_db::render::*;
    #[path="C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/materials/texture-repairs/candidate/src/render/texture_transcode.rs"]
    pub mod texture_transcode;
    pub mod assets {
        pub use qualia_core_db::render::assets::*;
        const GLB_MAGIC:u32=0x46546c67;
        const CHUNK_JSON:u32=0x4e4f534a;
        const CHUNK_BIN:u32=0x004e4942;
        mod glb {
            pub use qualia_core_db::render::assets::import_glb_with_normals;
            include!("C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/materials/texture-repairs/harness/glb_private_readers.rs");
        }
        #[path="C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/materials/texture-repairs/candidate/src/render/assets/glb_skins_anims.rs"]
        pub mod glb_skins_anims;
    }
}
