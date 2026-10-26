//! Inert unverified source declarations stored in existing HMC resource CBOR metadata.
use serde::{Deserialize, Serialize};
use std::io::Write;

pub const SOURCE_ANNOTATION_BYTES: usize = 16 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Synthetic,
    Provided,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceDeclaration {
    pub status: SourceStatus,
    pub description: String,
    /// A supplied reference, not a verified observation, right or authority.
    pub evidence_ref: Option<String>,
    pub observation_claim: bool,
}
impl SourceDeclaration {
    pub fn validate(&self) -> Result<(), String> {
        let bounded =
            |value: &str| !value.trim().is_empty() && value.len() <= 4096 && !value.contains('\0');
        if !bounded(&self.description) || self.evidence_ref.as_deref().is_some_and(|v| !bounded(v))
        {
            return Err("Raster source description/reference requires one to 4096 bytes".into());
        }
        if self.observation_claim
            && (self.status != SourceStatus::Provided || self.evidence_ref.is_none())
        {
            return Err("An observation claim requires explicit provided-source status and evidence reference; it remains unverified".into());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceQualification {
    Unverified,
}

/// App-owned resource annotation, not a scientific provenance graph or attestation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceAnnotation {
    pub source_declaration: SourceDeclaration,
    pub qualification: SourceQualification,
}

pub(super) fn encode_annotation(source: &SourceDeclaration) -> Result<Vec<u8>, String> {
    source.validate()?;
    struct Bounded(Vec<u8>);
    impl Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self
                .0
                .len()
                .checked_add(bytes.len())
                .is_none_or(|n| n > SOURCE_ANNOTATION_BYTES)
            {
                return Err(std::io::Error::other(
                    "Raster source annotation byte budget exceeded",
                ));
            }
            self.0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Bounded(Vec::new());
    writer
        .0
        .try_reserve_exact(SOURCE_ANNOTATION_BYTES)
        .map_err(|_| "Source annotation allocation refused")?;
    let annotation = SourceAnnotation {
        source_declaration: source.clone(),
        qualification: SourceQualification::Unverified,
    };
    ciborium::ser::into_writer(&annotation, &mut writer)
        .map_err(|e| format!("Resource source metadata encoding refused: {e}"))?;
    Ok(writer.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn georaster_source_declarations_remain_unverified_bounded_and_distinct() {
        for status in [
            SourceStatus::Synthetic,
            SourceStatus::Provided,
            SourceStatus::Unknown,
        ] {
            let source = SourceDeclaration {
                status,
                description: "explicit fixture declaration".into(),
                evidence_ref: Some("urn:fixture:unverified-source".into()),
                observation_claim: false,
            };
            let bytes = encode_annotation(&source).unwrap();
            assert!(bytes.len() <= SOURCE_ANNOTATION_BYTES);
            let decoded: SourceAnnotation = ciborium::de::from_reader(bytes.as_slice()).unwrap();
            assert_eq!(decoded.source_declaration, source);
            assert_eq!(decoded.qualification, SourceQualification::Unverified);
        }
        let mut source = SourceDeclaration {
            status: SourceStatus::Provided,
            description: "declared observation; not verified".into(),
            evidence_ref: None,
            observation_claim: true,
        };
        assert!(source.validate().is_err());
        source.evidence_ref = Some("urn:fixture:claim-only".into());
        assert!(source.validate().is_ok());
        source.status = SourceStatus::Synthetic;
        assert!(source.validate().is_err());
        source.observation_claim = false;
        source.description = "d".repeat(4096);
        source.evidence_ref = Some("e".repeat(4096));
        assert!(encode_annotation(&source).unwrap().len() <= SOURCE_ANNOTATION_BYTES);
        source.description = "x".repeat(4097);
        assert!(encode_annotation(&source).is_err());
        source.description = "bounded".into();
        source.evidence_ref = Some(" ".into());
        assert!(encode_annotation(&source).is_err());
    }
}
