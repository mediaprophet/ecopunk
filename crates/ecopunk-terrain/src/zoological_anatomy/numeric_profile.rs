//! Explicit generator arithmetic policy; this does not change stored assets.
//! Legacy sources retain platform std math. The portable profile selects the
//! pinned software owner; exact cross-target artifact qualification is separate.
//! Selected callers use sin/cos on [0, TAU] and pow exponents .65/.70/.75/.85/8.
//! In libm 0.2.16 these bounded paths use k_sinf/k_cosf and the pow polynomial;
//! they do not enter general trig reduction or the exponent-.5 sqrt dispatch.
//! No global libm feature change or general all-input determinism is claimed.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum NumericProfile {
    #[serde(rename = "portable-software-v1")]
    PortableSoftwareV1,
}

impl NumericProfile {
    #[inline]
    pub(crate) fn sin(profile: Option<Self>, x: f32) -> f32 {
        match profile {
            None => x.sin(),
            Some(Self::PortableSoftwareV1) => libm::sinf(x),
        }
    }
    #[inline]
    pub(crate) fn cos(profile: Option<Self>, x: f32) -> f32 {
        match profile {
            None => x.cos(),
            Some(Self::PortableSoftwareV1) => libm::cosf(x),
        }
    }
    #[inline]
    pub(crate) fn pow(profile: Option<Self>, x: f32, exponent: f32) -> f32 {
        match profile {
            None => x.powf(exponent),
            Some(Self::PortableSoftwareV1) => libm::powf(x, exponent),
        }
    }
}
