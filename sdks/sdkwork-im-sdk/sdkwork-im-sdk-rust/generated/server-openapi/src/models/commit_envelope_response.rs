use serde::{Deserialize, Serialize};

use crate::models::{EventActor};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommitEnvelopeResponse {
    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "eventType")]
    pub event_type: String,

    #[serde(rename = "eventVersion")]
    pub event_version: i64,

    #[serde(rename = "aggregateType")]
    pub aggregate_type: String,

    #[serde(rename = "aggregateId")]
    pub aggregate_id: String,

    #[serde(rename = "scopeType")]
    pub scope_type: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "orderingKey")]
    pub ordering_key: String,

    #[serde(rename = "orderingSeq")]
    pub ordering_seq: i64,

    #[serde(rename = "causationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<String>,

    #[serde(rename = "correlationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,

    #[serde(rename = "idempotencyKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,

    pub actor: EventActor,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,

    #[serde(rename = "committedAt")]
    pub committed_at: String,

    #[serde(rename = "payloadSchema")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_schema: Option<String>,

    pub payload: String,

    #[serde(rename = "retentionClass")]
    pub retention_class: String,

    #[serde(rename = "auditClass")]
    pub audit_class: String,
}
