//! TECH-16 conversation-scope message change realtime fanout.

use std::sync::Arc;

use im_domain_core::message::{Message, MessageEdited, MessageRecalled};
use im_platform_contracts::{
    CommitJournal, OutboxEventRecord, OutboxPublishStatus, RealtimeEventPublisher,
    RealtimeEventRecipient, RealtimeScopeEventPublishCommand,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::sha256_hash;
use serde::Serialize;

use super::{
    CONVERSATION_MEMBER_LIST_MAX_LIMIT, ConversationRuntime, DirectMessageAccessGate, RuntimeError,
};

const CONVERSATION_SCOPE_TYPE: &str = "conversation";
const CONVERSATION_OUTBOX_AGGREGATE_TYPE: &str = "conversation";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MessagePostedRealtimePayload {
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    message_type: String,
    summary: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageEditedRealtimePayload {
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    summary: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageRecalledRealtimePayload {
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    summary: String,
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn resolve_realtime_publisher(&self) -> Option<Arc<dyn RealtimeEventPublisher>> {
        self.realtime_publisher
            .clone()
            .or_else(crate::embedded_wiring::resolve_embedded_realtime_publisher)
    }

    pub fn with_realtime_publisher(mut self, publisher: Arc<dyn RealtimeEventPublisher>) -> Self {
        self.realtime_publisher = Some(publisher);
        self
    }

    pub fn with_direct_message_access_gate(
        mut self,
        gate: Arc<dyn DirectMessageAccessGate>,
    ) -> Self {
        self.direct_message_access_gate = Some(gate);
        self
    }

    pub(crate) fn resolve_direct_message_access_gate(
        &self,
    ) -> Option<Arc<dyn DirectMessageAccessGate>> {
        crate::embedded_wiring::resolve_embedded_direct_message_access_gate()
            .or_else(|| self.direct_message_access_gate.clone())
    }

    pub(crate) fn publish_message_posted_realtime(
        &self,
        tenant_id: &str,
        organization_id: &str,
        message: &Message,
    ) -> Result<(), RuntimeError> {
        let Some(publisher) = self.resolve_realtime_publisher() else {
            if self.requires_realtime_delivery_fail_closed() {
                return Err(RuntimeError::Contract(
                    sdkwork_im_contract_core::ContractError::Unavailable(
                        "realtime publisher is required in production when outbox delivery is not configured"
                            .into(),
                    ),
                ));
            }
            return Ok(());
        };
        let payload = MessagePostedRealtimePayload {
            conversation_id: message.conversation_id.clone(),
            message_id: message.message_id.clone(),
            message_seq: message.message_seq,
            message_type: message.message_type.as_wire_value().to_owned(),
            summary: message
                .body
                .summary_or_derived()
                .unwrap_or_else(|| "[message]".into()),
        };
        let payload_json = serde_json::to_string(&payload).map_err(|error| {
            RuntimeError::InvalidInput(format!(
                "message.posted realtime payload encode failed: {error}"
            ))
        })?;
        // The journal commit has already persisted the message; realtime push is a
        // best-effort side-effect. If the publisher is temporarily unavailable the
        // outbox relay (when configured) will eventually deliver the event. Logging
        // the error and returning Ok avoids cascading 503 (code 50301) failures for
        // every message send when the realtime backend blips.
        if let Err(error) = self.publish_durable_conversation_event(
            publisher.as_ref(),
            tenant_id,
            organization_id,
            message.conversation_id.as_str(),
            "message.posted",
            payload_json,
        ) {
            tracing::warn!(
                conversation_id = %message.conversation_id,
                message_id = %message.message_id,
                error = %error,
                "message.posted realtime publish failed; relying on outbox relay for eventual delivery"
            );
        }
        Ok(())
    }

    pub(crate) fn publish_message_edited_realtime(
        &self,
        tenant_id: &str,
        organization_id: &str,
        edited: &MessageEdited,
        event_id: &str,
    ) -> Result<(), RuntimeError> {
        let payload_body = MessageEditedRealtimePayload {
            conversation_id: edited.conversation_id.clone(),
            message_id: edited.message_id.clone(),
            message_seq: edited.message_seq,
            summary: edited
                .body
                .summary_or_derived()
                .unwrap_or_else(|| "[message]".into()),
        };
        let payload_json = serde_json::to_string(&payload_body).map_err(|error| {
            RuntimeError::InvalidInput(format!(
                "message.edited realtime payload encode failed: {error}"
            ))
        })?;
        self.publish_or_enqueue_message_mutation_realtime(
            tenant_id,
            organization_id,
            edited.conversation_id.as_str(),
            "message.edited",
            event_id,
            payload_json,
        )
    }

    pub(crate) fn publish_message_recalled_realtime(
        &self,
        tenant_id: &str,
        organization_id: &str,
        recalled: &MessageRecalled,
        event_id: &str,
    ) -> Result<(), RuntimeError> {
        let payload_body = MessageRecalledRealtimePayload {
            conversation_id: recalled.conversation_id.clone(),
            message_id: recalled.message_id.clone(),
            message_seq: recalled.message_seq,
            summary: "[recalled]".into(),
        };
        let payload_json = serde_json::to_string(&payload_body).map_err(|error| {
            RuntimeError::InvalidInput(format!(
                "message.recalled realtime payload encode failed: {error}"
            ))
        })?;
        self.publish_or_enqueue_message_mutation_realtime(
            tenant_id,
            organization_id,
            recalled.conversation_id.as_str(),
            "message.recalled",
            event_id,
            payload_json,
        )
    }

    fn publish_durable_scope_event_to_active_members_in_batches(
        &self,
        publisher: &dyn RealtimeEventPublisher,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        event_type: &str,
        payload_json: String,
    ) -> Result<(), RuntimeError> {
        let mut cursor: Option<String> = None;
        loop {
            let window = self.list_members_window(
                tenant_id,
                organization_id,
                conversation_id,
                Some(CONVERSATION_MEMBER_LIST_MAX_LIMIT),
                cursor.as_deref(),
            )?;
            let recipients = window
                .items
                .into_iter()
                .map(|member| {
                    RealtimeEventRecipient::new(member.principal_id, member.principal_kind)
                })
                .collect::<Vec<_>>();
            if !recipients.is_empty() {
                publisher
                    .publish_durable_scope_event_to_recipients(RealtimeScopeEventPublishCommand {
                        tenant_id,
                        organization_id,
                        scope_type: CONVERSATION_SCOPE_TYPE,
                        scope_id: conversation_id,
                        event_type,
                        payload: payload_json.clone(),
                        recipients,
                    })
                    .map_err(RuntimeError::from)?;
            }
            if window.page_info.has_more != Some(true) {
                break;
            }
            cursor = window.page_info.next_cursor.clone();
        }
        Ok(())
    }

    fn publish_or_enqueue_message_mutation_realtime(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        event_type: &str,
        event_id: &str,
        payload_json: String,
    ) -> Result<(), RuntimeError> {
        if let Some(publisher) = self.resolve_realtime_publisher() {
            // The journal commit has already persisted the mutation; realtime push
            // is a best-effort side-effect. If the publisher is temporarily
            // unavailable, log and continue rather than failing the request with
            // 503 (code 50301). The outbox relay provides eventual delivery when
            // configured.
            if let Err(error) = self.publish_durable_scope_event_to_active_members_in_batches(
                publisher.as_ref(),
                tenant_id,
                organization_id,
                conversation_id,
                event_type,
                payload_json,
            ) {
                tracing::warn!(
                    conversation_id = %conversation_id,
                    event_type = %event_type,
                    error = %error,
                    "message mutation realtime publish failed; relying on outbox relay for eventual delivery"
                );
            }
            return Ok(());
        }

        if let Some(record) = self.build_message_mutation_outbox_record(
            tenant_id,
            organization_id,
            conversation_id,
            event_type,
            event_id,
            payload_json,
        )? {
            self.outbox_store
                .as_ref()
                .expect("outbox record built only when outbox store is configured")
                .enqueue(record)
                .map_err(RuntimeError::from)?;
            return Ok(());
        }

        if self.requires_realtime_delivery_fail_closed() {
            return Err(RuntimeError::Contract(
                sdkwork_im_contract_core::ContractError::Unavailable(
                    "realtime publisher or outbox store is required in production".into(),
                ),
            ));
        }
        Ok(())
    }

    fn build_message_mutation_outbox_record(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        event_type: &str,
        event_id: &str,
        payload_body_json: String,
    ) -> Result<Option<OutboxEventRecord>, RuntimeError> {
        if self.resolve_realtime_publisher().is_some() {
            return Ok(None);
        }
        if self.outbox_store.is_none() || self.id_generator.is_none() {
            return Ok(None);
        }
        serde_json::from_str::<serde_json::Value>(payload_body_json.as_str()).map_err(|error| {
            RuntimeError::InvalidInput(format!(
                "{event_type} outbox payload encode failed: {error}"
            ))
        })?;
        let payload_json = payload_body_json;
        let payload_hash = sha256_hash(payload_json.as_bytes());
        let now = utc_now_rfc3339_millis();
        let id_generator = self
            .id_generator
            .as_ref()
            .expect("id_generator checked above");
        let outbox_id = id_generator
            .next_id()
            .map_err(RuntimeError::from)?
            .to_string();
        let outbox_event_id = format!("conversation:{event_type}:{event_id}");
        Ok(Some(OutboxEventRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            outbox_id,
            aggregate_type: CONVERSATION_OUTBOX_AGGREGATE_TYPE.into(),
            aggregate_id: conversation_id.to_owned(),
            event_id: outbox_event_id,
            event_type: event_type.into(),
            payload_json,
            payload_hash,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: now.clone(),
            published_at: None,
            created_at: now.clone(),
            updated_at: now,
        }))
    }

    fn publish_durable_conversation_event(
        &self,
        publisher: &dyn RealtimeEventPublisher,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        event_type: &str,
        payload: String,
    ) -> Result<(), RuntimeError> {
        self.publish_durable_scope_event_to_active_members_in_batches(
            publisher,
            tenant_id,
            organization_id,
            conversation_id,
            event_type,
            payload,
        )
    }

    pub(crate) fn build_message_posted_outbox_record(
        &self,
        tenant_id: &str,
        organization_id: &str,
        message: &Message,
    ) -> Result<Option<OutboxEventRecord>, RuntimeError> {
        if self.resolve_realtime_publisher().is_some() {
            return Ok(None);
        }
        if self.outbox_store.is_none() || self.id_generator.is_none() {
            return Ok(None);
        }
        let payload_body = MessagePostedRealtimePayload {
            conversation_id: message.conversation_id.clone(),
            message_id: message.message_id.clone(),
            message_seq: message.message_seq,
            message_type: message.message_type.as_wire_value().to_owned(),
            summary: message
                .body
                .summary_or_derived()
                .unwrap_or_else(|| "[message]".into()),
        };
        let payload_json = serde_json::json!({
            "conversationId": payload_body.conversation_id,
            "messageId": payload_body.message_id,
            "messageSeq": payload_body.message_seq,
            "messageType": payload_body.message_type,
            "summary": payload_body.summary,
        });
        let payload_json = serde_json::to_string(&payload_json).map_err(|error| {
            RuntimeError::InvalidInput(format!(
                "message.posted outbox payload encode failed: {error}"
            ))
        })?;
        let payload_hash = sha256_hash(payload_json.as_bytes());
        let now = utc_now_rfc3339_millis();
        let id_generator = self
            .id_generator
            .as_ref()
            .expect("id_generator checked above");
        let outbox_id = id_generator
            .next_id()
            .map_err(RuntimeError::from)?
            .to_string();
        let event_id = format!("conversation:message.posted:{outbox_id}");
        Ok(Some(OutboxEventRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            outbox_id,
            aggregate_type: CONVERSATION_OUTBOX_AGGREGATE_TYPE.into(),
            aggregate_id: message.conversation_id.clone(),
            event_id,
            event_type: "message.posted".into(),
            payload_json,
            payload_hash,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: now.clone(),
            published_at: None,
            created_at: now.clone(),
            updated_at: now,
        }))
    }

    fn requires_realtime_delivery_fail_closed(&self) -> bool {
        if !env_flag_enabled("SDKWORK_IM_REQUIRE_REALTIME_PUBLISHER") {
            return false;
        }
        self.resolve_realtime_publisher().is_none() && self.outbox_store.is_none()
    }
}

fn env_flag_enabled(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
    use std::time::Duration;

    use im_domain_core::conversation::MembershipRole;
    use im_platform_contracts::{
        CommitPosition, ContractError, IdGenerator, OutboxEventClaim, OutboxStore,
    };

    use super::*;
    use crate::{AddConversationMemberCommand, CreateConversationCommand};

    #[derive(Default)]
    struct RealtimeTestJournal {
        offset: AtomicU64,
    }

    impl CommitJournal for RealtimeTestJournal {
        fn append(
            &self,
            _envelope: im_domain_events::CommitEnvelope,
        ) -> Result<CommitPosition, ContractError> {
            Ok(CommitPosition::new(
                "message-realtime-test",
                self.offset.fetch_add(1, Ordering::Relaxed) + 1,
            ))
        }
    }

    #[derive(Default)]
    struct NoopOutboxStore;

    impl OutboxStore for NoopOutboxStore {
        fn enqueue(&self, _event: OutboxEventRecord) -> Result<(), ContractError> {
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
            _tenant_id: &str,
            _organization_id: &str,
            _event_id: &str,
        ) -> Result<Option<OutboxEventRecord>, ContractError> {
            Ok(None)
        }

        fn count_pending(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
        ) -> Result<u64, ContractError> {
            Ok(0)
        }

        fn list_pending_scopes(
            &self,
            _aggregate_type: &str,
            _limit: usize,
        ) -> Result<Vec<(String, String)>, ContractError> {
            Ok(Vec::new())
        }
    }

    #[derive(Default)]
    struct RealtimeTestIdGenerator {
        next: AtomicI64,
    }

    impl IdGenerator for RealtimeTestIdGenerator {
        fn next_id(&self) -> Result<i64, ContractError> {
            Ok(self.next.fetch_add(1, Ordering::Relaxed) + 1)
        }

        fn node_id(&self) -> u16 {
            0
        }

        fn next_id_at(&self, _timestamp_millis: u64) -> Result<i64, ContractError> {
            self.next_id()
        }
    }

    #[test]
    fn conversation_outbox_payload_does_not_embed_unbounded_recipient_inventory() {
        let runtime = ConversationRuntime::new(RealtimeTestJournal::default())
            .with_outbox_store(Arc::new(NoopOutboxStore))
            .with_id_generator(Arc::new(RealtimeTestIdGenerator::default()));
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_scope_only_outbox".into(),
                creator_id: "user_000".into(),
                conversation_type: "group".into(),
            })
            .expect("outbox test conversation should be created");
        for index in 1..=400 {
            runtime
                .add_member(AddConversationMemberCommand {
                    tenant_id: "100001".into(),
                    organization_id: "0".into(),
                    conversation_id: "c_scope_only_outbox".into(),
                    principal_id: format!("user_{index:03}"),
                    principal_kind: "user".into(),
                    role: MembershipRole::Member,
                    invited_by: "user_000".into(),
                })
                .expect("outbox test member should be added");
        }

        let record = runtime
            .build_message_mutation_outbox_record(
                "100001",
                "0",
                "c_scope_only_outbox",
                "message.edited",
                "evt_message_edited",
                serde_json::json!({
                    "conversationId": "c_scope_only_outbox",
                    "messageId": "42",
                })
                .to_string(),
            )
            .expect("scope-only outbox record should build")
            .expect("outbox record should be present");
        let payload: serde_json::Value = serde_json::from_str(record.payload_json.as_str())
            .expect("outbox payload should be valid json");

        assert!(payload.get("recipientPrincipalIds").is_none());
        assert!(payload.get("recipientPrincipalKinds").is_none());
        assert!(record.payload_json.len() < 1024);
    }
}
