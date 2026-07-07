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

    fn collect_active_member_principal_pairs(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(Vec<String>, Vec<String>), RuntimeError> {
        let mut principal_ids = Vec::new();
        let mut principal_kinds = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let window = self.list_members_window(
                tenant_id,
                organization_id,
                conversation_id,
                Some(CONVERSATION_MEMBER_LIST_MAX_LIMIT),
                cursor.as_deref(),
            )?;
            for member in window.items {
                principal_ids.push(member.principal_id);
                principal_kinds.push(member.principal_kind);
            }
            if window.page_info.has_more != Some(true) {
                break;
            }
            cursor = window.page_info.next_cursor.clone();
        }
        Ok((principal_ids, principal_kinds))
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
        let members = self.list_members(tenant_id, organization_id, conversation_id)?;
        if members.is_empty() {
            return Ok(None);
        }
        let recipient_principal_ids = members
            .iter()
            .map(|member| member.principal_id.clone())
            .collect::<Vec<_>>();
        let recipient_principal_kinds = members
            .iter()
            .map(|member| member.principal_kind.clone())
            .collect::<Vec<_>>();
        let payload_value = serde_json::from_str::<serde_json::Value>(&payload_body_json)
            .unwrap_or_else(|_| serde_json::json!({}));
        let mut payload_object = match payload_value {
            serde_json::Value::Object(map) => map,
            other => {
                let mut map = serde_json::Map::new();
                map.insert("payload".into(), other);
                map
            }
        };
        payload_object.insert(
            "recipientPrincipalIds".into(),
            serde_json::Value::Array(
                recipient_principal_ids
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            ),
        );
        payload_object.insert(
            "recipientPrincipalKinds".into(),
            serde_json::Value::Array(
                recipient_principal_kinds
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            ),
        );
        let payload_json = serde_json::to_string(&serde_json::Value::Object(payload_object))
            .map_err(|error| {
                RuntimeError::InvalidInput(format!(
                    "{event_type} outbox payload encode failed: {error}"
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
        let (recipient_principal_ids, recipient_principal_kinds) = self
            .collect_active_member_principal_pairs(
                tenant_id,
                organization_id,
                message.conversation_id.as_str(),
            )?;
        if recipient_principal_ids.is_empty() {
            return Ok(None);
        }
        let payload_json = serde_json::json!({
            "conversationId": payload_body.conversation_id,
            "messageId": payload_body.message_id,
            "messageSeq": payload_body.message_seq,
            "messageType": payload_body.message_type,
            "summary": payload_body.summary,
            "recipientPrincipalIds": recipient_principal_ids,
            "recipientPrincipalKinds": recipient_principal_kinds,
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
