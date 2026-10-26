use super::*;

impl NativeAuthority {
    pub fn begin_session(
        &self,
        subject: &str,
        lifetime_seconds: u64,
    ) -> Result<SessionChallenge, String> {
        let mut state = self.state.lock().map_err(|_| "native authority poisoned")?;
        let now = now()?;
        self.require_admission(&state, now)?;
        if lifetime_seconds == 0 || lifetime_seconds > 3600 {
            return Err("native session lifetime exceeds bounds".into());
        }
        let admitted = state
            .durable
            .admission
            .snapshot
            .subjects
            .iter()
            .find(|s| s.subject == subject)
            .ok_or("subject has no signed project admission")?;
        if admitted.revoked || !live(now, admitted.valid_from_unix, admitted.valid_until_unix) {
            return Err("subject admission expired/revoked".into());
        }
        let expires = now
            .checked_add(lifetime_seconds)
            .ok_or("session expiry overflow")?
            .min(admitted.valid_until_unix)
            .min(state.durable.admission.snapshot.valid_until_unix);
        let nonce = random()?;
        let mut hash = Sha256::new();
        // Hash fixed-size fields plus bounded, length-delimited identifiers.
        hash.update(SESSION_CONTEXT);
        hash.update(self.incarnation);
        hash.update(nonce);
        hash.update(state.admission_digest);
        hash.update(admitted.controller_public_key);
        hash.update(expires.to_be_bytes());
        for value in [&self.anchor.project_id, &admitted.subject] {
            hash.update((value.len() as u64).to_be_bytes());
            hash.update(value.as_bytes());
        }
        let challenge = SessionChallenge {
            owner: self.incarnation,
            nonce,
            digest: hash.finalize().into(),
        };
        state.pending = Some(Pending {
            challenge: challenge.clone(),
            subject: subject.into(),
            expires,
            admission: state.admission_digest,
        });
        Ok(challenge)
    }
    pub fn complete_session(
        &self,
        challenge: SessionChallenge,
        signature: &[u8; 64],
    ) -> Result<SessionHandle, String> {
        let mut state = self.state.lock().map_err(|_| "native authority poisoned")?;
        let now = now()?;
        self.require_admission(&state, now)?;
        let pending = state
            .pending
            .take()
            .ok_or("no pending native session challenge")?;
        if challenge.owner != self.incarnation
            || challenge.nonce != pending.challenge.nonce
            || challenge.digest != pending.challenge.digest
            || now >= pending.expires
            || pending.admission != state.admission_digest
        {
            return Err("stale/foreign native session challenge".into());
        }
        let subject = state
            .durable
            .admission
            .snapshot
            .subjects
            .iter()
            .find(|s| s.subject == pending.subject)
            .ok_or("native subject admission disappeared")?;
        ed25519::verify_with_context(
            &subject.controller_public_key,
            &challenge.digest,
            SESSION_CONTEXT,
            signature,
        )
        .map_err(|_| "native session possession proof verification failed")?;
        // The native grant is installed only AFTER the adopted key proof above.
        let binding = AuthorityBinding::from_bytes(
            &subject.controller_public_key,
            &self.anchor.public_key,
            &state.admission_digest,
            self.anchor.project_id.as_bytes(),
            b"poet/native-authoring/v1",
            &challenge.nonce,
        )
        .map_err(|e| format!("native grant binding: {e:?}"))?;
        if let Some(previous) = state.session.take() {
            state
                .grants
                .revoke(previous.grant)
                .map_err(|e| format!("revoke: {e:?}"))?;
        }
        // QDNF's bounded grant table retains revoked slots. One fresh owner per
        // authenticated session prevents exhaustion without resurrecting grants.
        state.grants = GrantOwner::new();
        let (_, _, grant) = state
            .grants
            .install_grant(binding, now, pending.expires, ContactState::Active)
            .map_err(|e| format!("native grant install: {e:?}"))?;
        state.generation = state
            .generation
            .checked_add(1)
            .ok_or("authority generation exhausted")?;
        let handle = SessionHandle {
            owner: self.incarnation,
            nonce: challenge.nonce,
        };
        state.session = Some(Session {
            handle: handle.clone(),
            subject: pending.subject,
            expires: pending.expires,
            grant,
            binding,
            authentication_ref: format!(
                "urn:qualia:native-session-proof:sha256:{}",
                hex::encode(challenge.digest)
            ),
        });
        Ok(handle)
    }
    pub fn disconnect(&self, handle: &SessionHandle) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|_| "native authority poisoned")?;
        if state.session.as_ref().is_none_or(|s| &s.handle != handle) {
            return Err("foreign native session".into());
        }
        let session = state.session.take().ok_or("native session unavailable")?;
        state
            .grants
            .revoke(session.grant)
            .map_err(|e| format!("native revoke: {e:?}"))?;
        state.generation = state
            .generation
            .checked_add(1)
            .ok_or("authority generation exhausted")?;
        Ok(())
    }
}
