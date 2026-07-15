use im_platform_contracts::{ContractError, OutboxEventRecord, OutboxPublishStatus};
use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::sha256_hash;

use crate::{ConversationProfileView, ProjectionError, TimelineProjectionService};

const CONVERSATION_OUTBOX_AGGREGATE_TYPE: &str = "conversation";
const CONVERSATION_UPDATED_EVENT_TYPE: &str = "conversation.updated";

impl TimelineProjectionService {
    pub(crate) fn enqueue_conversation_profile_updated(
        &self,
        tenant_id: &str,
        organization_id: &str,
        profile: &ConversationProfileView,
    ) -> Result<(), ProjectionError> {
        let Some(outbox) = self.conversation_event_outbox.get() else {
            return Ok(());
        };
        let record =
            build_conversation_profile_updated_record(tenant_id, organization_id, profile)?;
        match outbox.enqueue(record.clone()) {
            Ok(()) => Ok(()),
            Err(ContractError::Conflict(_)) => {
                let existing = outbox
                    .read_by_event_id(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.event_id.as_str(),
                    )
                    .map_err(ProjectionError::StoreFailure)?;
                if existing
                    .as_ref()
                    .is_some_and(|value| outbox_records_match(value, &record))
                {
                    Ok(())
                } else {
                    Err(ProjectionError::StoreFailure(ContractError::Conflict(
                        format!(
                            "conversation profile event identity conflicts with another payload: {}",
                            record.event_id
                        ),
                    )))
                }
            }
            Err(error) => Err(ProjectionError::StoreFailure(error)),
        }
    }
}

fn build_conversation_profile_updated_record(
    tenant_id: &str,
    organization_id: &str,
    profile: &ConversationProfileView,
) -> Result<OutboxEventRecord, ProjectionError> {
    let payload_json = serde_json::to_string(&serde_json::json!({
        "conversationId": profile.conversation_id,
        "displayName": profile.display_name,
        "avatarUrl": profile.avatar_url,
        "notice": profile.notice,
        "updatedAt": profile.updated_at,
    }))
    .map_err(ProjectionError::InvalidSnapshot)?;
    let journal_event_id = format!(
        "conversation:profile.updated:{}:{}",
        profile.conversation_id, profile.updated_at
    );
    let event_id = format!("conversation:{CONVERSATION_UPDATED_EVENT_TYPE}:{journal_event_id}");
    let identity_seed = encode_segments([
        tenant_id,
        organization_id,
        profile.conversation_id.as_str(),
        CONVERSATION_UPDATED_EVENT_TYPE,
        journal_event_id.as_str(),
    ]);
    let outbox_id = format!("conv_ob_{}", &sha256_hash(identity_seed.as_bytes())[..32]);
    let now = utc_now_rfc3339_millis();
    Ok(OutboxEventRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        outbox_id,
        aggregate_type: CONVERSATION_OUTBOX_AGGREGATE_TYPE.into(),
        aggregate_id: profile.conversation_id.clone(),
        event_id,
        event_type: CONVERSATION_UPDATED_EVENT_TYPE.into(),
        payload_hash: sha256_hash(payload_json.as_bytes()),
        payload_json,
        publish_status: OutboxPublishStatus::Pending,
        attempt_count: 0,
        available_at: now.clone(),
        published_at: None,
        created_at: now.clone(),
        updated_at: now,
    })
}

fn encode_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

fn outbox_records_match(existing: &OutboxEventRecord, expected: &OutboxEventRecord) -> bool {
    existing.tenant_id == expected.tenant_id
        && existing.organization_id == expected.organization_id
        && existing.outbox_id == expected.outbox_id
        && existing.aggregate_type == expected.aggregate_type
        && existing.aggregate_id == expected.aggregate_id
        && existing.event_id == expected.event_id
        && existing.event_type == expected.event_type
        && existing.payload_json == expected.payload_json
        && existing.payload_hash == expected.payload_hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_platform_contracts::{OutboxEventClaim, OutboxStore};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[derive(Default)]
    struct RecordingOutboxStore {
        events: Mutex<Vec<OutboxEventRecord>>,
    }

    impl OutboxStore for RecordingOutboxStore {
        fn enqueue(&self, event: OutboxEventRecord) -> Result<(), ContractError> {
            self.events.lock().expect("events should lock").push(event);
            Ok(())
        }

        fn claim_pending(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _aggregate_type: &str,
            _batch_size: usize,
            _lease_duration: Duration,
        ) -> Result<Vec<OutboxEventClaim>, ContractError> {
            Ok(Vec::new())
        }

        fn mark_published(&self, _claim: &OutboxEventClaim) -> Result<(), ContractError> {
            Ok(())
        }

        fn mark_failed(
            &self,
            _claim: &OutboxEventClaim,
            _reason: &str,
        ) -> Result<(), ContractError> {
            Ok(())
        }

        fn retry_failed(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _outbox_id: &str,
        ) -> Result<(), ContractError> {
            Ok(())
        }

        fn read_by_event_id(
            &self,
            tenant_id: &str,
            organization_id: &str,
            event_id: &str,
        ) -> Result<Option<OutboxEventRecord>, ContractError> {
            Ok(self
                .events
                .lock()
                .expect("events should lock")
                .iter()
                .find(|event| {
                    event.tenant_id == tenant_id
                        && event.organization_id == organization_id
                        && event.event_id == event_id
                })
                .cloned())
        }

        fn count_pending(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
        ) -> Result<u64, ContractError> {
            Ok(self.events.lock().expect("events should lock").len() as u64)
        }

        fn list_pending_scopes(
            &self,
            _aggregate_type: &str,
            _limit: usize,
        ) -> Result<Vec<(String, String)>, ContractError> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn conversation_profile_event_carries_authoritative_display_name() {
        let profile = ConversationProfileView {
            tenant_id: "100001".into(),
            conversation_id: "group-1".into(),
            display_name: "Renamed group".into(),
            avatar_url: "https://cdn.example.test/group.png".into(),
            notice: "Updated notice".into(),
            updated_at: "2026-07-14T10:00:00.000Z".into(),
            updated_by_principal_kind: Some("user".into()),
            updated_by_principal_id: Some("owner-1".into()),
        };

        let record = build_conversation_profile_updated_record("100001", "0", &profile)
            .expect("profile event should build");
        let payload: serde_json::Value =
            serde_json::from_str(record.payload_json.as_str()).expect("payload should be JSON");

        assert_eq!(record.aggregate_type, "conversation");
        assert_eq!(record.event_type, "conversation.updated");
        assert_eq!(payload["conversationId"], "group-1");
        assert_eq!(payload["displayName"], "Renamed group");
    }

    #[test]
    fn configured_outbox_receives_profile_update_event() {
        let service = TimelineProjectionService::default();
        let outbox = Arc::new(RecordingOutboxStore::default());
        service.configure_conversation_event_outbox(outbox.clone());
        let profile = ConversationProfileView {
            tenant_id: "100001".into(),
            conversation_id: "group-2".into(),
            display_name: "Commercial group".into(),
            avatar_url: String::new(),
            notice: String::new(),
            updated_at: "2026-07-14T11:00:00.000Z".into(),
            updated_by_principal_kind: Some("user".into()),
            updated_by_principal_id: Some("owner-2".into()),
        };

        service
            .enqueue_conversation_profile_updated("100001", "0", &profile)
            .expect("configured outbox should accept profile event");

        let events = outbox.events.lock().expect("events should lock");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "conversation.updated");
        assert_eq!(events[0].aggregate_id, "group-2");
    }
}
