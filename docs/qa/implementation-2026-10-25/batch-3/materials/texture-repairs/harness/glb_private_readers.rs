// Verbatim actual private accessor readers; harness only, no substitute engine.
use serde_json::Value;
use super::AssetError;
pub(super) fn read_vec3(
    accessor: &Value,
    bvs: &[Value],
    bin: &[u8],
    semantic: &str,
) -> Result<Vec<[f32; 3]>, AssetError> {
    if accessor["componentType"].as_u64() != Some(5126) {
        return Err(AssetError::Parse(format!(
            "{semantic} componentType must be FLOAT (5126)"
        )));
    }
    if accessor["type"].as_str() != Some("VEC3") {
        return Err(AssetError::Parse(format!(
            "{semantic} accessor type must be VEC3"
        )));
    }
    let count = accessor["count"]
        .as_u64()
        .ok_or_else(|| AssetError::Parse("accessor.count missing".into()))?
        as usize;
    let acc_off = accessor["byteOffset"].as_u64().unwrap_or(0) as usize;
    let bv_idx = accessor["bufferView"]
        .as_u64()
        .ok_or_else(|| AssetError::Parse("accessor.bufferView missing".into()))?
        as usize;
    let bv = bvs
        .get(bv_idx)
        .ok_or_else(|| AssetError::Parse("bufferView index out of range".into()))?;
    let bv_off = bv["byteOffset"].as_u64().unwrap_or(0) as usize;
    let stride = match bv["byteStride"].as_u64().unwrap_or(0) {
        0 => 12, // tightly packed VEC3 f32
        s => s as usize,
    };
    let start = bv_off + acc_off;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let o = start + i * stride;
        if o + 12 > bin.len() {
            return Err(AssetError::Parse(format!(
                "{semantic} read past end of BIN"
            )));
        }
        let rd = |k: usize| {
            f32::from_le_bytes([bin[o + k], bin[o + k + 1], bin[o + k + 2], bin[o + k + 3]])
        };
        out.push([rd(0), rd(4), rd(8)]);
    }
    Ok(out)
}

pub(super) fn read_vec4(
    accessor: &Value,
    bvs: &[Value],
    bin: &[u8],
    semantic: &str,
) -> Result<Vec<[f32; 4]>, AssetError> {
    if accessor["componentType"].as_u64() != Some(5126) {
        return Err(AssetError::Parse(format!(
            "{semantic} componentType must be FLOAT (5126)"
        )));
    }
    if accessor["type"].as_str() != Some("VEC4") {
        return Err(AssetError::Parse(format!(
            "{semantic} accessor type must be VEC4"
        )));
    }
    let count = accessor["count"]
        .as_u64()
        .ok_or_else(|| AssetError::Parse("accessor.count missing".into()))?
        as usize;
    let acc_off = accessor["byteOffset"].as_u64().unwrap_or(0) as usize;
    let bv_idx = accessor["bufferView"]
        .as_u64()
        .ok_or_else(|| AssetError::Parse("accessor.bufferView missing".into()))?
        as usize;
    let bv = bvs
        .get(bv_idx)
        .ok_or_else(|| AssetError::Parse("bufferView index out of range".into()))?;
    let bv_off = bv["byteOffset"].as_u64().unwrap_or(0) as usize;
    let stride = match bv["byteStride"].as_u64().unwrap_or(0) {
        0 => 16,
        s => s as usize,
    };
    let start = bv_off + acc_off;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let o = start + i * stride;
        if o + 16 > bin.len() {
            return Err(AssetError::Parse(format!(
                "{semantic} read past end of BIN"
            )));
        }
        let rd = |k: usize| {
            f32::from_le_bytes([bin[o + k], bin[o + k + 1], bin[o + k + 2], bin[o + k + 3]])
        };
        out.push([rd(0), rd(4), rd(8), rd(12)]);
    }
    Ok(out)
}

