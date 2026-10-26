// Verbatim fixture construction from actual assets/mod.rs, no model/rights claim.
const GLB_MAGIC:u32=0x46546c67;
const CHUNK_JSON:u32=0x4e4f534a;
const CHUNK_BIN:u32=0x004e4942;
pub fn living_fixture()->Vec<u8>{
        let mut bin = Vec::new();
        // bv 0: 3 positions (36 B)
        for v in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
            for c in v {
                bin.extend_from_slice(&c.to_le_bytes());
            }
        }
        // bv 1: 3 JOINTS_0 u8x4 (12 B)
        for _ in 0..3 {
            bin.extend_from_slice(&[0u8, 1, 0, 0]);
        }
        // bv 2: 3 WEIGHTS_0 f32x4 (48 B)
        for _ in 0..3 {
            for w in [0.75f32, 0.25, 0.0, 0.0] {
                bin.extend_from_slice(&w.to_le_bytes());
            }
        }
        // bv 3: 2 IBM MAT4 (128 B)
        let ident_mat: [f32; 16] = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        for _ in 0..2 {
            for f in ident_mat {
                bin.extend_from_slice(&f.to_le_bytes());
            }
        }
        // bv 4: anim times [0.0, 0.5] (8 B)
        for t in [0.0f32, 0.5] {
            bin.extend_from_slice(&t.to_le_bytes());
        }
        // bv 5: anim translations [[0,0,0], [0,2,0]] (24 B)
        for tr in [[0.0f32, 0.0, 0.0], [0.0, 2.0, 0.0]] {
            for c in tr {
                bin.extend_from_slice(&c.to_le_bytes());
            }
        }
        // bv 6: morph target POSITION deltas (36 B)
        for md in [[0.0f32, 0.5, 0.0], [0.0, 0.0, 0.0], [0.0, -0.25, 0.0]] {
            for c in md {
                bin.extend_from_slice(&c.to_le_bytes());
            }
        }
        while bin.len() % 4 != 0 {
            bin.push(0);
        }
        let json = serde_json::json!({
            "asset": { "version": "2.0", "extras": { "qualia": { "taxon_iri": "urn:qualia:flora:corymbia" } } },
            "buffers": [{ "byteLength": bin.len() }],
            "bufferViews": [
                { "buffer": 0, "byteOffset": 0, "byteLength": 36 },
                { "buffer": 0, "byteOffset": 36, "byteLength": 12 },
                { "buffer": 0, "byteOffset": 48, "byteLength": 48 },
                { "buffer": 0, "byteOffset": 96, "byteLength": 128 },
                { "buffer": 0, "byteOffset": 224, "byteLength": 8 },
                { "buffer": 0, "byteOffset": 232, "byteLength": 24 },
                { "buffer": 0, "byteOffset": 256, "byteLength": 36 }
            ],
            "accessors": [
                { "bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3" },
                { "bufferView": 1, "componentType": 5121, "count": 3, "type": "VEC4" },
                { "bufferView": 2, "componentType": 5126, "count": 3, "type": "VEC4" },
                { "bufferView": 3, "componentType": 5126, "count": 2, "type": "MAT4" },
                { "bufferView": 4, "componentType": 5126, "count": 2, "type": "SCALAR" },
                { "bufferView": 5, "componentType": 5126, "count": 2, "type": "VEC3" },
                { "bufferView": 6, "componentType": 5126, "count": 3, "type": "VEC3" }
            ],
            "materials": [{
                "extensions": {
                    "KHR_materials_clearcoat": { "clearcoatFactor": 0.8, "clearcoatRoughnessFactor": 0.2 },
                    "KHR_materials_ior": { "ior": 1.333 },
                    "KHR_materials_emissive_strength": { "emissiveStrength": 4.5 }
                }
            }],
            "extensions": {
                "KHR_lights_punctual": {
                    "lights": [{ "name": "SunSpot", "type": "spot", "color": [1.0, 0.9, 0.8], "intensity": 12.0 }]
                }
            },
            "meshes": [{
                "primitives": [{
                    "attributes": { "POSITION": 0, "JOINTS_0": 1, "WEIGHTS_0": 2 },
                    "targets": [{ "POSITION": 6 }],
                    "material": 0
                }]
            }],
            "nodes": [
                { "name": "RootJoint", "children": [1], "translation": [0.0, 1.0, 0.0] },
                { "name": "ChildJoint", "translation": [0.0, 0.5, 0.0], "extensions": { "KHR_lights_punctual": { "light": 0 } } },
                { "name": "SkinnedMeshNode", "mesh": 0, "skin": 0 }
            ],
            "scenes": [{ "nodes": [0, 2] }],
            "skins": [{ "joints": [0, 1], "skeleton": 0, "inverseBindMatrices": 3 }],
            "animations": [{
                "name": "Sway",
                "samplers": [{ "input": 4, "output": 5, "interpolation": "LINEAR" }],
                "channels": [{ "sampler": 0, "target": { "node": 1, "path": "translation" } }]
            }]
        });
        let mut jb = serde_json::to_vec(&json).unwrap();
        while jb.len() % 4 != 0 {
            jb.push(b' ');
        }
        let total = 12 + 8 + jb.len() + 8 + bin.len();
        let mut glb = Vec::with_capacity(total);
        glb.extend_from_slice(&GLB_MAGIC.to_le_bytes());
        glb.extend_from_slice(&2u32.to_le_bytes());
        glb.extend_from_slice(&(total as u32).to_le_bytes());
        glb.extend_from_slice(&(jb.len() as u32).to_le_bytes());
        glb.extend_from_slice(&CHUNK_JSON.to_le_bytes());
        glb.extend_from_slice(&jb);
        glb.extend_from_slice(&(bin.len() as u32).to_le_bytes());
        glb.extend_from_slice(&CHUNK_BIN.to_le_bytes());
        glb.extend_from_slice(&bin);


 glb
}