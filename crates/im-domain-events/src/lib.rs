use serde::{Deserialize, Serialize};

pub mod social;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregateType {
    Conversation,
    FriendRequest,
    Friendship,
    ExternalConnection,
    ExternalMemberLink,
    SharedChannelPolicy,
    StreamSession,
    RtcSession,
    TenantPolicy,
    DirectChat,
    Notification,
    AutomationExecution,
    UserBlock,
}

impl AggregateType {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Conversation => "conversation",
            Self::FriendRequest => "friend_request",
            Self::Friendship => "friendship",
            Self::ExternalConnection => "external_connection",
            Self::ExternalMemberLink => "external_member_link",
            Self::SharedChannelPolicy => "shared_channel_policy",
            Self::StreamSession => "stream_session",
            Self::RtcSession => "rtc_session",
            Self::TenantPolicy => "tenant_policy",
            Self::DirectChat => "direct_chat",
            Self::Notification => "notification",
            Self::AutomationExecution => "automation_execution",
            Self::UserBlock => "user_block",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventActor {
    pub actor_id: String,
    pub actor_kind: String,
    pub actor_session_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitEnvelope {
    pub event_id: String,
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub event_type: String,
    pub event_version: u16,
    pub aggregate_type: AggregateType,
    pub aggregate_id: String,
    pub scope_type: String,
    pub scope_id: String,
    pub ordering_key: String,
    pub ordering_seq: u64,
    pub causation_id: Option<String>,
    pub correlation_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub actor: EventActor,
    pub occurred_at: String,
    pub committed_at: String,
    pub payload_schema: Option<String>,
    pub payload: String,
    pub retention_class: String,
    pub audit_class: String,
}

impl CommitEnvelope {
    pub fn ordering_key(tenant_id: &str, scope_id: &str) -> String {
        encode_event_key_segments([tenant_id, scope_id])
    }

    pub fn normalized_organization_id(&self) -> String {
        normalize_commit_organization_id(self.organization_id.as_str())
    }

    pub fn with_organization_id(mut self, organization_id: &str) -> Self {
        self.organization_id = normalize_commit_organization_id(organization_id);
        self
    }

    pub fn minimal(
        event_id: &str,
        tenant_id: &str,
        event_type: &str,
        scope_type: &str,
        scope_id: &str,
        ordering_seq: u64,
    ) -> Self {
        Self {
            event_id: event_id.into(),
            tenant_id: tenant_id.into(),
            organization_id: default_organization_id(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: scope_id.into(),
            scope_type: scope_type.into(),
            scope_id: scope_id.into(),
            ordering_key: Self::ordering_key(tenant_id, scope_id),
            ordering_seq,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: "system".into(),
                actor_kind: "system".into(),
                actor_session_id: None,
            },
            occurred_at: "1970-01-01T00:00:00Z".into(),
            committed_at: "1970-01-01T00:00:00Z".into(),
            payload_schema: None,
            payload: "{}".into(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        }
    }

    pub fn with_payload(mut self, payload_schema: &str, payload: &str) -> Self {
        self.payload_schema = Some(payload_schema.into());
        self.payload = payload.into();
        self
    }
}

pub fn normalize_commit_organization_id(organization_id: &str) -> String {
    let trimmed = organization_id.trim();
    if trimmed.is_empty() || trimmed == "0" || trimmed == "default" {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

fn default_organization_id() -> String {
    "0".to_owned()
}

fn encode_event_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}
