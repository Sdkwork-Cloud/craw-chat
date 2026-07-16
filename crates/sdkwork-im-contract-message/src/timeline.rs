use sdkwork_im_contract_core::ContractError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjectionRecord {
    pub message_seq: u64,
    pub payload: String,
}

/// Tenant and organization scope for one durable timeline projection.
///
/// The database primary key includes all three values. Keeping them together
/// prevents adapters from inferring an organization from process state or from
/// falling back to a shared default scope. An explicit legacy personal alias
/// is canonicalized to organization `"0"`, matching event and projection keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjectionScope {
    tenant_id: String,
    organization_id: String,
    timeline_scope: String,
}

impl TimelineProjectionScope {
    pub fn new(
        tenant_id: impl Into<String>,
        organization_id: impl Into<String>,
        timeline_scope: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let tenant_id = required_scope_segment("tenant_id", tenant_id.into())?;
        let organization_id = im_domain_events::normalize_commit_organization_id(
            required_scope_segment("organization_id", organization_id.into())?.as_str(),
        );
        let timeline_scope = required_scope_segment("timeline_scope", timeline_scope.into())?;
        Ok(Self {
            tenant_id,
            organization_id,
            timeline_scope,
        })
    }

    pub fn tenant_id(&self) -> &str {
        self.tenant_id.as_str()
    }

    pub fn organization_id(&self) -> &str {
        self.organization_id.as_str()
    }

    pub fn timeline_scope(&self) -> &str {
        self.timeline_scope.as_str()
    }
}

fn required_scope_segment(field: &'static str, value: String) -> Result<String, ContractError> {
    if value.trim().is_empty() {
        return Err(ContractError::Conflict(format!(
            "timeline projection {field} is required; refusing to infer a default scope"
        )));
    }
    Ok(value)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjectionBatch {
    pub scope: TimelineProjectionScope,
    pub records: Vec<TimelineProjectionRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TimelineProjectionWindow {
    pub items: Vec<(u64, String)>,
    pub has_more: bool,
}

pub trait TimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        scope: &TimelineProjectionScope,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError>;

    fn load_timeline(
        &self,
        scope: &TimelineProjectionScope,
    ) -> Result<Vec<(u64, String)>, ContractError>;

    /// Load a bounded keyset window with `message_seq > after_seq`.
    fn load_timeline_window(
        &self,
        _scope: &TimelineProjectionScope,
        _after_seq: u64,
        _limit: usize,
    ) -> Result<TimelineProjectionWindow, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "timeline load_timeline_window requires an explicit store-level pagination implementation"
                .into(),
        ))
    }

    fn upsert_timeline_entries(
        &self,
        scope: &TimelineProjectionScope,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        for record in records {
            self.upsert_timeline_entry(scope, record.message_seq, record.payload.as_str())?;
        }
        Ok(())
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        for batch in batches {
            self.upsert_timeline_entries(&batch.scope, &batch.records)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UnpagedTimelineStore;

    impl TimelineProjectionStore for UnpagedTimelineStore {
        fn upsert_timeline_entry(
            &self,
            _scope: &TimelineProjectionScope,
            _message_seq: u64,
            _payload: &str,
        ) -> Result<(), ContractError> {
            Ok(())
        }

        fn load_timeline(
            &self,
            _scope: &TimelineProjectionScope,
        ) -> Result<Vec<(u64, String)>, ContractError> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn window_default_fails_closed() {
        let scope = TimelineProjectionScope::new("tenant", "organization", "conversation")
            .expect("scope should be valid");
        assert!(matches!(
            UnpagedTimelineStore.load_timeline_window(&scope, 0, 20),
            Err(ContractError::UnsupportedCapability(message)) if message.contains("load_timeline_window")
        ));
    }

    #[test]
    fn timeline_projection_scope_rejects_missing_organization_id() {
        assert!(matches!(
            TimelineProjectionScope::new("tenant", "", "conversation"),
            Err(ContractError::Conflict(message)) if message.contains("organization_id")
        ));
    }

    #[test]
    fn timeline_projection_scope_canonicalizes_explicit_personal_organization_alias() {
        let scope = TimelineProjectionScope::new("tenant", "default", "conversation")
            .expect("explicit personal organization scope should be valid");
        assert_eq!(scope.organization_id(), "0");
    }
}
