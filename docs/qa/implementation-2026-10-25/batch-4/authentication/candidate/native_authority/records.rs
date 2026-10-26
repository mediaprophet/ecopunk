use super::*;

impl NativeAuthority {
    pub fn record_contribution(
        &self,
        handle: &SessionHandle,
        id: &str,
        description: &str,
        minutes: u32,
        status: WorkStatus,
    ) -> Result<ContributionRecord, String> {
        let mut state = self.state.lock().map_err(|_| "native authority poisoned")?;
        let now = now()?;
        if state.durable.contributions.len() >= 256 {
            return Err("contribution evidence budget exhausted".into());
        }
        let record = self.view(&state, handle, now, |view| {
            let permit = authorize(
                view,
                &AuthoringRequest {
                    action: AuthoringAction::ContributionRecord,
                    target: view.current_document,
                    expected_authority_generation: view.generation,
                },
            )
            .map_err(|e| e.to_string())?;
            let previous = state
                .durable
                .contributions
                .iter()
                .rev()
                .find(|r| r.contributor_subject == permit.principal_subject());
            permit
                .contribution_record(id, description, minutes, status, previous)
                .map_err(|e| e.to_string())
        })?;
        if state
            .durable
            .contributions
            .iter()
            .any(|r| r.contribution.id == id)
        {
            return Err("duplicate contribution identifier".into());
        }
        let mut next = state.durable.clone();
        next.contributions.push(record.clone());
        if let Err(e) = state.journal.append(&next) {
            state.reconciliation = true;
            return Err(e);
        }
        state.durable = next;
        Ok(record)
    }
    pub fn record_review(
        &self,
        handle: &SessionHandle,
        id: &str,
        decision: ReviewDecision,
        rationale: &str,
        evidence: &str,
    ) -> Result<ReviewRecord, String> {
        let mut state = self.state.lock().map_err(|_| "native authority poisoned")?;
        let now = now()?;
        if state.durable.reviews.len() >= 256 {
            return Err("review evidence budget exhausted".into());
        }
        let record = self.view(&state, handle, now, |view| {
            authorize(
                view,
                &AuthoringRequest {
                    action: AuthoringAction::ReviewDecision,
                    target: view.current_document,
                    expected_authority_generation: view.generation,
                },
            )
            .map_err(|e| e.to_string())?
            .review_record(id, decision, rationale, evidence)
            .map_err(|e| e.to_string())
        })?;
        if state.durable.reviews.iter().any(|r| r.id == id) {
            return Err("duplicate review identifier".into());
        }
        let mut next = state.durable.clone();
        next.reviews.push(record.clone());
        if let Err(e) = state.journal.append(&next) {
            state.reconciliation = true;
            return Err(e);
        }
        state.durable = next;
        Ok(record)
    }
}
