//! TECH-16 conversation-scope message change realtime fanout.

use std::sync::Arc;

use im_domain_core::message::Message;
use im_platform_contracts::{
    CommitJournal, ContractError, OutboxEventRecord, OutboxPublishStatus, RealtimeEventPublisher,
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

pub(crate) struct ConversationRealtimeEvent<'a> {
    pub(crate) tenant_id: &'a str,
    pub(crate) organization_id: &'a str,
    pub(crate) conversation_id: &'a str,
    pub(crate) event_type: &'a str,
    pub(crate) journal_event_id: &'a str,
    pub(crate) payload_json: String,
    pub(crate) occurred_at: &'a str,
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

    /// Publish a committed conversation event immediately when a realtime
    /// publisher is available, falling back to the durable conversation outbox
    /// when the publisher is unavailable or rejects the delivery. The journal
    /// remains authoritative: this method only handles the post-commit fanout.
    pub(crate) fn publish_or_enqueue_conversation_event(
        &self,
        event: ConversationRealtimeEvent<'_>,
    ) -> Result<(), RuntimeError> {
        serde_json::from_str::<serde_json::Value>(event.payload_json.as_str()).map_err(
            |error| {
                RuntimeError::InvalidInput(format!(
                    "{} realtime payload encode failed: {error}",
                    event.event_type
                ))
            },
        )?;

        let mut publisher_error = None;
        if let Some(publisher) = self.resolve_realtime_publisher() {
            match self.publish_durable_scope_event_to_active_members_in_batches(
                publisher.as_ref(),
                event.tenant_id,
                event.organization_id,
                event.conversation_id,
                event.event_type,
                event.payload_json.clone(),
            ) {
                Ok(()) => return Ok(()),
                Err(error) => {
                    tracing::warn!(
                        conversation_id = %event.conversation_id,
                        event_type = %event.event_type,
                        error = ?error,
                        "conversation realtime publish failed; falling back to outbox"
                    );
                    publisher_error = Some(error);
                }
            }
        }

        if self.outbox_store.is_none() {
            if publisher_error.is_some() && self.requires_realtime_delivery_fail_closed() {
                return Err(publisher_error.unwrap_or_else(|| {
                    RuntimeError::Contract(ContractError::Unavailable(
                        "conversation realtime publisher is unavailable".into(),
                    ))
                }));
            }
            if self.requires_realtime_delivery_fail_closed() {
                return Err(RuntimeError::Contract(ContractError::Unavailable(
                    "realtime publisher or outbox store is required in production".into(),
                )));
            }
            return Ok(());
        }

        let record = self.build_conversation_event_outbox_record(event)?;
        let outbox = self
            .outbox_store
            .as_ref()
            .expect("outbox record built only when outbox store is configured");
        match outbox.enqueue(record.clone()) {
            Ok(()) => Ok(()),
            Err(ContractError::Conflict(_)) => {
                let existing = outbox
                    .read_by_event_id(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.event_id.as_str(),
                    )?
                    .ok_or_else(|| {
                        RuntimeError::Conflict(format!(
                            "outbox event conflict without an existing record: {}",
                            record.event_id
                        ))
                    })?;
                if conversation_outbox_record_matches(&existing, &record) {
                    Ok(())
                } else {
                    Err(RuntimeError::Conflict(format!(
                        "outbox event identity already exists with a different payload: {}",
                        record.event_id
                    )))
                }
            }
            Err(error) => Err(RuntimeError::Contract(error)),
        }
    }

    pub(crate) fn build_conversation_event_outbox_record(
        &self,
        event: ConversationRealtimeEvent<'_>,
    ) -> Result<OutboxEventRecord, RuntimeError> {
        let payload_hash = sha256_hash(event.payload_json.as_bytes());
        let identity_seed = super::encode_conversation_key_segments([
            event.tenant_id,
            event.organization_id,
            event.conversation_id,
            event.event_type,
            event.journal_event_id,
        ]);
        let outbox_id = format!("conv_ob_{}", &sha256_hash(identity_seed.as_bytes())[..32]);
        let event_id = format!(
            "conversation:{}:{}",
            event.event_type, event.journal_event_id
        );
        // `available_at` is a delivery time, not the domain occurrence time.
        // A client/replay envelope may legitimately carry a historical or
        // slightly future timestamp; using it for scheduling could strand a
        // pending row until that timestamp arrives. Keep the original event
        // time out of the scheduling path and enqueue immediately.
        let now = utc_now_rfc3339_millis();
        let _ = event.occurred_at;
        Ok(OutboxEventRecord {
            tenant_id: event.tenant_id.to_owned(),
            organization_id: event.organization_id.to_owned(),
            outbox_id,
            aggregate_type: CONVERSATION_OUTBOX_AGGREGATE_TYPE.into(),
            aggregate_id: event.conversation_id.to_owned(),
            event_id,
            event_type: event.event_type.to_owned(),
            payload_json: event.payload_json,
            payload_hash,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: now.clone(),
            published_at: None,
            created_at: now.clone(),
            updated_at: now,
        })
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
                        recipients: recipients.clone(),
                    })
                    .map_err(RuntimeError::from)?;
                if event_type == "conversation.updated" {
                    publisher
                        .publish_durable_user_scope_event_to_recipients(
                            tenant_id,
                            organization_id,
                            event_type,
                            payload_json.clone(),
                            recipients,
                        )
                        .map_err(RuntimeError::from)?;
                }
            }
            if window.page_info.has_more != Some(true) {
                break;
            }
            cursor = window.page_info.next_cursor.clone();
        }
        Ok(())
    }

    pub(crate) fn publish_message_mutation_realtime_after_commit(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        event_type: &str,
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
        Ok(())
    }

    pub(crate) fn build_message_mutation_outbox_record(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        event_type: &str,
        event_id: &str,
        payload_body_json: String,
    ) -> Result<Option<OutboxEventRecord>, RuntimeError> {
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

fn conversation_outbox_record_matches(
    existing: &OutboxEventRecord,
    expected: &OutboxEventRecord,
) -> bool {
    existing.tenant_id == expected.tenant_id
        && existing.organization_id == expected.organization_id
        && existing.outbox_id == expected.outbox_id
        && existing.aggregate_type == expected.aggregate_type
        && existing.aggregate_id == expected.aggregate_id
        && existing.event_id == expected.event_id
        && existing.event_type == expected.event_type
        // `payload_json` is stored as PostgreSQL jsonb and its textual
        // representation may reorder object keys on read. The producer hash
        // is computed over the original serialized payload and is persisted
        // unchanged, so compare the hash rather than the round-tripped text.
        && existing.payload_hash == expected.payload_hash
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
    use std::time::Duration;

    use im_domain_core::conversation::{ConversationAgentAssignment, MembershipRole};
    use im_platform_contracts::{
        CommitPosition, ContractError, IdGenerator, NormalizedConversationCommit, OutboxEventClaim,
        OutboxStore,
    };

    use super::*;
    use crate::{
        AddConversationMemberCommand, CreateConversationCommand, DurableConversationEventWriter,
        ReplaceConversationAgentsCommand,
    };

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

        fn discover_pending_scopes(
            &self,
            _request: im_platform_contracts::OutboxScopeDiscoveryRequest<'_>,
        ) -> Result<Vec<(String, String)>, ContractError> {
            Ok(Vec::new())
        }
    }

    #[derive(Clone, Default)]
    struct RecordingOutboxStore {
        events: Arc<Mutex<Vec<OutboxEventRecord>>>,
    }

    impl RecordingOutboxStore {
        fn recorded(&self) -> Vec<OutboxEventRecord> {
            self.events.lock().expect("outbox should lock").clone()
        }
    }

    impl OutboxStore for RecordingOutboxStore {
        fn enqueue(&self, event: OutboxEventRecord) -> Result<(), ContractError> {
            let mut events = self.events.lock().expect("outbox should lock");
            if events.iter().any(|existing| {
                existing.tenant_id == event.tenant_id
                    && existing.organization_id == event.organization_id
                    && existing.event_id == event.event_id
            }) {
                return Err(ContractError::Conflict(format!(
                    "outbox event already exists: {}",
                    event.event_id
                )));
            }
            events.push(event);
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
                .expect("outbox should lock")
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
            tenant_id: &str,
            organization_id: &str,
        ) -> Result<u64, ContractError> {
            Ok(self
                .events
                .lock()
                .expect("outbox should lock")
                .iter()
                .filter(|event| {
                    event.tenant_id == tenant_id
                        && event.organization_id == organization_id
                        && event.publish_status == OutboxPublishStatus::Pending
                })
                .count() as u64)
        }

        fn discover_pending_scopes(
            &self,
            request: im_platform_contracts::OutboxScopeDiscoveryRequest<'_>,
        ) -> Result<Vec<(String, String)>, ContractError> {
            let aggregate_type = request.aggregate_type();
            let mut scopes = self
                .events
                .lock()
                .expect("outbox should lock")
                .iter()
                .filter(|event| {
                    event.aggregate_type == aggregate_type
                        && event.publish_status == OutboxPublishStatus::Pending
                })
                .map(|event| (event.tenant_id.clone(), event.organization_id.clone()))
                .collect::<Vec<_>>();
            scopes.sort_unstable();
            scopes.dedup();
            Ok(scopes)
        }
    }

    #[derive(Clone, Default)]
    struct RecordingDurableConversationEventWriter {
        commits: Arc<Mutex<Vec<(im_domain_events::CommitEnvelope, OutboxEventRecord)>>>,
    }

    impl RecordingDurableConversationEventWriter {
        fn recorded(&self) -> Vec<(im_domain_events::CommitEnvelope, OutboxEventRecord)> {
            self.commits.lock().expect("writer should lock").clone()
        }
    }

    impl DurableConversationEventWriter for RecordingDurableConversationEventWriter {
        fn persist_normalized_conversation_commit(
            &self,
            commit: NormalizedConversationCommit,
        ) -> Result<Vec<CommitPosition>, ContractError> {
            if commit.envelopes.len() != commit.outboxes.len() {
                return Err(ContractError::Invalid(
                    "test normalized commit cardinality mismatch".into(),
                ));
            }
            let positions = commit
                .envelopes
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    CommitPosition::new("atomic-conversation-event-test", (index + 1) as u64)
                })
                .collect();
            self.commits
                .lock()
                .expect("writer should lock")
                .extend(commit.envelopes.into_iter().zip(commit.outboxes));
            Ok(positions)
        }

        fn persist_conversation_event(
            &self,
            envelope: im_domain_events::CommitEnvelope,
            outbox: OutboxEventRecord,
        ) -> Result<CommitPosition, ContractError> {
            self.commits
                .lock()
                .expect("writer should lock")
                .push((envelope, outbox));
            Ok(CommitPosition::new("atomic-conversation-event-test", 1))
        }
    }

    struct FailingDurableConversationEventWriter;

    impl DurableConversationEventWriter for FailingDurableConversationEventWriter {
        fn persist_normalized_conversation_commit(
            &self,
            commit: NormalizedConversationCommit,
        ) -> Result<Vec<CommitPosition>, ContractError> {
            if commit
                .envelopes
                .iter()
                .any(|envelope| envelope.event_type == "conversation.agents_replaced")
            {
                return Err(ContractError::Unavailable(
                    "atomic conversation event write failed".into(),
                ));
            }
            Ok(commit
                .envelopes
                .iter()
                .enumerate()
                .map(|(index, _)| CommitPosition::new("atomic-create-test", (index + 1) as u64))
                .collect())
        }

        fn persist_conversation_event(
            &self,
            _envelope: im_domain_events::CommitEnvelope,
            _outbox: OutboxEventRecord,
        ) -> Result<CommitPosition, ContractError> {
            Err(ContractError::Unavailable(
                "atomic conversation event write failed".into(),
            ))
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct RecordedRealtimeEvent {
        tenant_id: String,
        organization_id: String,
        scope_type: String,
        scope_id: String,
        event_type: String,
        payload: String,
        recipients: Vec<RealtimeEventRecipient>,
    }

    #[derive(Clone, Default)]
    struct RecordingRealtimePublisher {
        events: Arc<Mutex<Vec<RecordedRealtimeEvent>>>,
    }

    impl RecordingRealtimePublisher {
        fn recorded(&self) -> Vec<RecordedRealtimeEvent> {
            self.events.lock().expect("publisher should lock").clone()
        }
    }

    impl RealtimeEventPublisher for RecordingRealtimePublisher {
        fn publish_ephemeral_scope_event_to_recipients(
            &self,
            _command: RealtimeScopeEventPublishCommand<'_>,
        ) -> Result<usize, ContractError> {
            Ok(0)
        }

        fn publish_durable_scope_event_to_recipients(
            &self,
            command: RealtimeScopeEventPublishCommand<'_>,
        ) -> Result<usize, ContractError> {
            let recipient_count = command.recipients.len();
            self.events
                .lock()
                .expect("publisher should lock")
                .push(RecordedRealtimeEvent {
                    tenant_id: command.tenant_id.to_owned(),
                    organization_id: command.organization_id.to_owned(),
                    scope_type: command.scope_type.to_owned(),
                    scope_id: command.scope_id.to_owned(),
                    event_type: command.event_type.to_owned(),
                    payload: command.payload,
                    recipients: command.recipients,
                });
            Ok(recipient_count)
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
            .with_id_generator(Arc::new(RealtimeTestIdGenerator::default()));
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_scope_only_outbox".into(),
                creator_id: "user_000".into(),
                conversation_type: "group".into(),
            })
            .expect("outbox test conversation should be created");
        for index in 1..=400 {
            runtime
                .add_member(AddConversationMemberCommand {
                    tenant_id: "100001".into(),
                    organization_id: "200001".into(),
                    conversation_id: "c_scope_only_outbox".into(),
                    principal_id: format!("user_{index:03}"),
                    principal_kind: "user".into(),
                    role: MembershipRole::Member,
                    invited_by: "user_000".into(),
                })
                .expect("outbox test member should be added");
        }
        let runtime = runtime.with_outbox_store(Arc::new(NoopOutboxStore));

        let record = runtime
            .build_message_mutation_outbox_record(
                "100001",
                "200001",
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

    #[test]
    fn replacing_group_agents_without_publisher_persists_deterministic_atomic_outbox_record() {
        let writer = RecordingDurableConversationEventWriter::default();
        let runtime = ConversationRuntime::new(RealtimeTestJournal::default())
            .with_durable_conversation_event_writer(Arc::new(writer.clone()));
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_outbox".into(),
                creator_id: "300001".into(),
                conversation_type: "group".into(),
            })
            .expect("outbox test group should be created");

        let replaced = runtime
            .replace_conversation_agents(ReplaceConversationAgentsCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_outbox".into(),
                replaced_by: "300001".into(),
                expected_generation: 1,
                agents: vec![
                    ConversationAgentAssignment::new("agent.im.reviewer", None),
                    ConversationAgentAssignment::new(
                        "agent.im.writer",
                        Some("revision.im.writer.2".into()),
                    ),
                ],
            })
            .expect("owner should replace group agents");

        let commits = writer.recorded();
        let records = commits
            .iter()
            .filter(|(event, _)| event.event_type == "conversation.agents_replaced")
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 1);
        let (event, record) = records[0];
        assert_eq!(event.event_id, replaced.event_id);
        assert_eq!(event.event_type, "conversation.agents_replaced");
        assert_eq!(record.tenant_id, "100001");
        assert_eq!(record.organization_id, "200001");
        assert_eq!(record.aggregate_type, "conversation");
        assert_eq!(record.aggregate_id, "c_agents_outbox");
        assert_eq!(record.event_type, "conversation.agents_replaced");
        assert_eq!(
            record.event_id,
            format!(
                "conversation:conversation.agents_replaced:{}",
                replaced.event_id
            )
        );
        let identity_seed = super::super::encode_conversation_key_segments([
            "100001",
            "200001",
            "c_agents_outbox",
            "conversation.agents_replaced",
            replaced.event_id.as_str(),
        ]);
        assert_eq!(
            record.outbox_id,
            format!("conv_ob_{}", &sha256_hash(identity_seed.as_bytes())[..32])
        );
        assert_eq!(
            record.payload_hash,
            sha256_hash(record.payload_json.as_bytes())
        );
        assert_eq!(record.publish_status, OutboxPublishStatus::Pending);
        assert_eq!(record.attempt_count, 0);
        assert!(record.published_at.is_none());

        let payload: serde_json::Value = serde_json::from_str(record.payload_json.as_str())
            .expect("assignment replacement outbox payload should decode");
        assert_eq!(payload["conversationId"], "c_agents_outbox");
        assert_eq!(payload["previousGeneration"], 1);
        assert_eq!(payload["agentAssignments"]["generation"], 2);
        assert_eq!(
            payload["agentAssignments"]["agents"][0]["agentId"],
            "agent.im.reviewer"
        );
        assert_eq!(
            payload["agentAssignments"]["agents"][1]["agentId"],
            "agent.im.writer"
        );
        assert!(payload.get("recipientPrincipalIds").is_none());
    }

    #[test]
    fn replacing_group_agents_uses_atomic_writer_without_second_delivery_path() {
        let writer = RecordingDurableConversationEventWriter::default();
        let fallback_outbox = RecordingOutboxStore::default();
        let publisher = RecordingRealtimePublisher::default();
        let runtime = ConversationRuntime::new(RealtimeTestJournal::default())
            .with_outbox_store(Arc::new(fallback_outbox.clone()))
            .with_realtime_publisher(Arc::new(publisher.clone()))
            .with_durable_conversation_event_writer(Arc::new(writer.clone()));
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_atomic_writer".into(),
                creator_id: "300001".into(),
                conversation_type: "group".into(),
            })
            .expect("atomic writer test group should be created");

        let replaced = runtime
            .replace_conversation_agents(ReplaceConversationAgentsCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_atomic_writer".into(),
                replaced_by: "300001".into(),
                expected_generation: 1,
                agents: vec![ConversationAgentAssignment::new(
                    "agent.im.reviewer",
                    Some("revision.im.reviewer.2".into()),
                )],
            })
            .expect("owner should replace group agents through the atomic writer");

        let commits = writer.recorded();
        let matching = commits
            .iter()
            .filter(|(event, _)| event.event_type == "conversation.agents_replaced")
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1);
        let (event, outbox) = matching[0];
        assert_eq!(event.event_id, replaced.event_id);
        assert_eq!(event.event_type, "conversation.agents_replaced");
        assert_eq!(outbox.event_type, event.event_type);
        assert_eq!(outbox.aggregate_type, "conversation");
        assert_eq!(outbox.aggregate_id, "c_agents_atomic_writer");
        assert_eq!(outbox.payload_hash, sha256_hash(event.payload.as_bytes()));
        assert!(
            fallback_outbox.recorded().is_empty(),
            "the runtime must not enqueue the same event again after an atomic commit"
        );
        assert!(
            publisher.recorded().is_empty(),
            "the durable outbox relay owns production delivery after an atomic commit"
        );
    }

    #[test]
    fn failed_atomic_group_agent_write_does_not_advance_assignment_generation() {
        let runtime = ConversationRuntime::new(RealtimeTestJournal::default())
            .with_durable_conversation_event_writer(Arc::new(
                FailingDurableConversationEventWriter,
            ));
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_atomic_failure".into(),
                creator_id: "300001".into(),
                conversation_type: "group".into(),
            })
            .expect("atomic failure test group should be created");

        let result = runtime.replace_conversation_agents(ReplaceConversationAgentsCommand {
            tenant_id: "100001".into(),
            organization_id: "200001".into(),
            conversation_id: "c_agents_atomic_failure".into(),
            replaced_by: "300001".into(),
            expected_generation: 1,
            agents: vec![ConversationAgentAssignment::new("agent.im.reviewer", None)],
        });

        assert!(matches!(result, Err(RuntimeError::Contract(_))));
        let snapshot = runtime
            .conversation_agent_assignments_snapshot("100001", "200001", "c_agents_atomic_failure")
            .expect("failed atomic write must retain the committed assignment snapshot");
        assert_eq!(snapshot.generation, 1);
        assert_eq!(snapshot.agents.len(), 1);
        assert_eq!(snapshot.agents[0].agent_id, "agent.im.default");
    }

    #[test]
    fn replacing_group_agents_publishes_only_to_human_conversation_members() {
        let publisher = RecordingRealtimePublisher::default();
        let runtime = ConversationRuntime::new(RealtimeTestJournal::default())
            .with_realtime_publisher(Arc::new(publisher.clone()));
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_realtime".into(),
                creator_id: "user.owner".into(),
                conversation_type: "group".into(),
            })
            .expect("realtime test group should be created");
        runtime
            .add_member(AddConversationMemberCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_realtime".into(),
                principal_id: "user.member".into(),
                principal_kind: "user".into(),
                role: MembershipRole::Member,
                invited_by: "user.owner".into(),
            })
            .expect("human member should be added");

        runtime
            .replace_conversation_agents(ReplaceConversationAgentsCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_agents_realtime".into(),
                replaced_by: "user.owner".into(),
                expected_generation: 1,
                agents: vec![
                    ConversationAgentAssignment::new("agent.im.reviewer", None),
                    ConversationAgentAssignment::new("agent.im.writer", None),
                ],
            })
            .expect("owner should replace group agents");

        let published = publisher.recorded();
        assert_eq!(published.len(), 1);
        let event = &published[0];
        assert_eq!(event.tenant_id, "100001");
        assert_eq!(event.organization_id, "200001");
        assert_eq!(event.scope_type, "conversation");
        assert_eq!(event.scope_id, "c_agents_realtime");
        assert_eq!(event.event_type, "conversation.agents_replaced");
        let mut recipients = event
            .recipients
            .iter()
            .map(|recipient| {
                (
                    recipient.principal_id.as_str(),
                    recipient.principal_kind.as_str(),
                )
            })
            .collect::<Vec<_>>();
        recipients.sort_unstable();
        assert_eq!(
            recipients,
            vec![("user.member", "user"), ("user.owner", "user")]
        );
        assert!(event.recipients.iter().all(|recipient| {
            recipient.principal_id != "agent.im.reviewer"
                && recipient.principal_id != "agent.im.writer"
        }));

        let payload: serde_json::Value = serde_json::from_str(event.payload.as_str())
            .expect("assignment replacement realtime payload should decode");
        assert_eq!(payload["agentAssignments"]["generation"], 2);
        assert_eq!(
            payload["agentAssignments"]["agents"]
                .as_array()
                .map(Vec::len),
            Some(2)
        );
    }

    #[test]
    fn conversation_profile_updates_reach_each_member_inbox_scope() {
        let publisher = RecordingRealtimePublisher::default();
        let runtime = ConversationRuntime::new(RealtimeTestJournal::default())
            .with_realtime_publisher(Arc::new(publisher.clone()));
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_profile_realtime".into(),
                creator_id: "user.owner".into(),
                conversation_type: "group".into(),
            })
            .expect("profile realtime group should be created");
        runtime
            .add_member(AddConversationMemberCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
                conversation_id: "c_profile_realtime".into(),
                principal_id: "user.member".into(),
                principal_kind: "user".into(),
                role: MembershipRole::Member,
                invited_by: "user.owner".into(),
            })
            .expect("profile realtime member should be added");

        runtime
            .publish_or_enqueue_conversation_event(ConversationRealtimeEvent {
                tenant_id: "100001",
                organization_id: "200001",
                conversation_id: "c_profile_realtime",
                event_type: "conversation.updated",
                journal_event_id: "profile-updated-1",
                payload_json: serde_json::json!({
                    "conversationId": "c_profile_realtime",
                    "displayName": "Renamed group",
                })
                .to_string(),
                occurred_at: "2026-07-14T10:00:00.000Z",
            })
            .expect("profile update should publish");

        let published = publisher.recorded();
        assert_eq!(published.len(), 3);
        assert!(published.iter().any(|event| {
            event.scope_type == "conversation"
                && event.scope_id == "c_profile_realtime"
                && event.recipients.len() == 2
        }));
        let mut inbox_scopes = published
            .iter()
            .filter(|event| event.scope_type == "user")
            .map(|event| event.scope_id.as_str())
            .collect::<Vec<_>>();
        inbox_scopes.sort_unstable();
        assert_eq!(inbox_scopes, vec!["user.member", "user.owner"]);
        assert!(
            published
                .iter()
                .all(|event| event.event_type == "conversation.updated")
        );
    }
}
