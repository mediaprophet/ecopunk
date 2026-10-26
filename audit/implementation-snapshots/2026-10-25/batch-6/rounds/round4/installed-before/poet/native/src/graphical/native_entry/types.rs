use crate::graphical::{controller::EditorSnapshot, draft_buffer::DraftBuffer, jobs::HostHandle};
use qualia_cooperative_core::{authoring::PrincipalKind, projects::ProjectRole};
use qualia_core_db::poet_host::native_authority::enrollment::EnrollmentRecord;

pub enum LaunchAction {
    Adopt {
        fingerprint: String,
    },
    CreateProject {
        name: String,
        profile: String,
        kind: PrincipalKind,
        evidence: String,
    },
    CreateProfile {
        profile: String,
        kind: PrincipalKind,
    },
    SubmitEnrollment {
        profile: String,
        evidence: String,
    },
    Login {
        profile: String,
        filename: String,
    },
    Logout,
    Approve {
        id: String,
        evidence: String,
        human_evidence: Option<String>,
    },
    Decline {
        id: String,
        evidence: String,
    },
    GrantReview {
        subject: String,
        evidence: String,
    },
    Revoke {
        subject: String,
        evidence: String,
    },
}
#[derive(Clone)]
pub struct StudioLease {
    pub host: HostHandle,
    pub initial: EditorSnapshot,
    pub draft_buffer: DraftBuffer,
    pub epoch: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberView {
    pub subject: String,
    pub kind: PrincipalKind,
    pub role: ProjectRole,
    pub has_review: bool,
    pub revoked: bool,
}
#[derive(Clone)]
pub struct LaunchView {
    pub generation: u64,
    pub project_name: Option<String>,
    pub authority_fingerprint: Option<String>,
    pub adopted: bool,
    pub current_subject: Option<String>,
    pub pending: Vec<EnrollmentRecord>,
    pub members: Vec<MemberView>,
    pub studio: Option<StudioLease>,
}
impl PartialEq for LaunchView {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation
    }
}

pub fn validate_action(action: &LaunchAction) -> Result<(), String> {
    let values: Vec<&str> = match action {
        LaunchAction::Adopt { fingerprint } => vec![fingerprint],
        LaunchAction::CreateProject {
            name,
            profile,
            evidence,
            ..
        } => vec![name, profile, evidence],
        LaunchAction::CreateProfile { profile, .. } => vec![profile],
        LaunchAction::SubmitEnrollment { profile, evidence } => vec![profile, evidence],
        LaunchAction::Login { profile, filename } => vec![profile, filename],
        LaunchAction::Logout => vec![],
        LaunchAction::Approve {
            id,
            evidence,
            human_evidence,
        } => {
            let mut values = vec![id.as_str(), evidence.as_str()];
            if let Some(value) = human_evidence {
                values.push(value)
            }
            values
        }
        LaunchAction::Decline { id, evidence } => vec![id, evidence],
        LaunchAction::GrantReview { subject, evidence }
        | LaunchAction::Revoke { subject, evidence } => vec![subject, evidence],
    };
    if values
        .iter()
        .any(|value| value.len() > 4096 || value.contains('\0'))
    {
        return Err("Native project intent exceeds text bounds".into());
    }
    Ok(())
}
