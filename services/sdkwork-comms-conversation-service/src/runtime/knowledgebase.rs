//! Group knowledgebase orchestration owned by the Conversation aggregate.
//!
//! IM is deliberately not a second knowledgebase.  It persists only the
//! conversation-to-space conversation_state and the short-lived launch-ticket ledger;
//! `sdkwork-knowledgebase` remains the authority for spaces, documents, and
//! its group-space binding.  The coordinator has a narrow provider port so
//! the production adapter can use the generated Knowledgebase internal RPC SDK
//! without leaking transport/auth details into this module.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use im_app_context::AppContext;
use im_domain_core::conversation::{
    ConversationMember, ConversationRoster, MembershipRole, MembershipState,
};
use im_platform_contracts::{
    CONVERSATION_AGGREGATE_PAGE_SIZE_MAX, ContractError, ConversationMemberPageCursor,
    ConversationMemberRecord, IdGenerator, PrivilegedOperationActorKind,
    PrivilegedOperationContext,
};
use sdkwork_im_contract_message::CommitJournal;
use sdkwork_utils_rust::{
    aes_gcm_decrypt, aes_gcm_encrypt, base64url_encode, derive_aes_256_key, sha256_hash,
};
use serde::{Deserialize, Serialize, Serializer};

use super::*;

const GROUP_KNOWLEDGEBASE_TICKET_BYTES: usize = 32;
const GROUP_KNOWLEDGEBASE_TICKET_TTL_SECONDS: i64 = 300;
const GROUP_KNOWLEDGEBASE_TICKET_SECRET_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_LAUNCH_TICKET_SECRET";
const GROUP_KNOWLEDGEBASE_TICKET_SECRET_FILE_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_LAUNCH_TICKET_SECRET_FILE";
const GROUP_KNOWLEDGEBASE_TICKET_KEY_SALT: &[u8] = b"sdkwork-im.group-knowledgebase.ticket.v1";
const GROUP_KNOWLEDGEBASE_TICKET_KEY_INFO: &[u8] = b"launch-ticket-replay";
const GROUP_KNOWLEDGEBASE_MAX_CONVERSATION_ID_BYTES: usize = 256;
const GROUP_KNOWLEDGEBASE_MAX_INITIAL_GROUP_NAME_BYTES: usize = 256;
const GROUP_KNOWLEDGEBASE_MAX_MEMBER_ID_BYTES: usize = 256;
const GROUP_KNOWLEDGEBASE_MAX_ACTOR_ID_BYTES: usize = 256;
const GROUP_KNOWLEDGEBASE_MAX_SOURCE_EVENT_ID_BYTES: usize = 512;
const GROUP_KNOWLEDGEBASE_MAX_TARGET_UUID_BYTES: usize = 256;
const GROUP_KNOWLEDGEBASE_SUPPORTED_MEMBER_PRINCIPAL_KIND: &str = "user";
pub(crate) const KNOWLEDGEBASE_SERVICE_IDENTITY: &str = "sdkwork-knowledgebase";
pub const GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE: &str = "conversation_group_knowledgebase";
pub const GROUP_KNOWLEDGEBASE_MEMBERSHIP_SYNC_EVENT_TYPE: &str =
    "conversation.group_knowledgebase.members.synchronize";
pub const GROUP_KNOWLEDGEBASE_ARCHIVE_EVENT_TYPE: &str = "conversation.group_knowledgebase.archive";

const GROUP_KNOWLEDGEBASE_RECONCILIATION_ACTOR_ID: &str = "sdkwork-im-reconciliation";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupKnowledgebaseScope {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
}

impl GroupKnowledgebaseScope {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Self, RuntimeError> {
        let scope = Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: resolve_group_knowledgebase_organization_id(auth)?,
            conversation_id: conversation_id.to_owned(),
        };
        scope.validate()?;
        Ok(scope)
    }

    fn validate(&self) -> Result<(), RuntimeError> {
        validate_group_knowledgebase_tenant_id(self.tenant_id.as_str())?;
        validate_group_knowledgebase_organization_id(self.organization_id.as_str())?;
        validate_group_knowledgebase_required_identifier(
            self.conversation_id.as_str(),
            "conversation id",
            GROUP_KNOWLEDGEBASE_MAX_CONVERSATION_ID_BYTES,
        )
    }
}

/// Resolves the token-derived organization scope dimension. Tenant sessions
/// use the canonical `0` sentinel; organization sessions use a positive id.
pub(crate) fn resolve_group_knowledgebase_organization_id(
    auth: &AppContext,
) -> Result<String, RuntimeError> {
    validate_group_knowledgebase_tenant_id(auth.tenant_id.as_str())?;
    let organization_id = organization_id_from_auth_context(auth);
    validate_group_knowledgebase_organization_id(organization_id.as_str())?;
    Ok(organization_id)
}

pub(super) fn validate_group_knowledgebase_organization_id(
    organization_id: &str,
) -> Result<(), RuntimeError> {
    if organization_id == "0" {
        return Ok(());
    }
    validate_group_knowledgebase_positive_signed_scope_id(organization_id, "organization")
}

pub(super) fn validate_group_knowledgebase_tenant_id(tenant_id: &str) -> Result<(), RuntimeError> {
    validate_group_knowledgebase_positive_signed_scope_id(tenant_id, "tenant")
}

fn validate_group_knowledgebase_positive_signed_scope_id(
    value: &str,
    scope_label: &str,
) -> Result<(), RuntimeError> {
    let canonical_decimal = !value.is_empty()
        && value != "0"
        && !value.starts_with('0')
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && value.parse::<i64>().is_ok();
    if !canonical_decimal {
        return Err(RuntimeError::PermissionDenied(format!(
            "group knowledgebase requires a canonical positive signed-64-bit {scope_label} scope"
        )));
    }
    Ok(())
}

fn group_knowledgebase_u64_to_db_i64(value: u64, field: &str) -> Result<i64, RuntimeError> {
    i64::try_from(value).map_err(|_| {
        RuntimeError::InvalidInput(format!(
            "group knowledgebase {field} exceeds the signed-64-bit persistence limit"
        ))
    })
}

fn group_knowledgebase_db_i64_to_u64(value: i64, field: &str) -> Result<u64, RuntimeError> {
    u64::try_from(value).map_err(|_| {
        RuntimeError::Contract(ContractError::Unavailable(format!(
            "group knowledgebase persisted {field} is invalid"
        )))
    })
}

fn validate_group_knowledgebase_persisted_positive_i64(
    value: i64,
    field: &str,
) -> Result<i64, RuntimeError> {
    if value > 0 {
        return Ok(value);
    }
    Err(RuntimeError::Contract(ContractError::Unavailable(format!(
        "group knowledgebase persisted {field} is invalid"
    ))))
}

fn validate_group_knowledgebase_nonzero_u64(value: u64, field: &str) -> Result<(), RuntimeError> {
    if value == 0 {
        return Err(RuntimeError::InvalidInput(format!(
            "group knowledgebase {field} must be positive"
        )));
    }
    group_knowledgebase_u64_to_db_i64(value, field)?;
    Ok(())
}

fn next_group_knowledgebase_version(version: u64) -> Result<u64, RuntimeError> {
    let signed_version = group_knowledgebase_u64_to_db_i64(version, "link generation")?;
    let next = signed_version.checked_add(1).ok_or_else(|| {
        RuntimeError::Contract(ContractError::Unavailable(
            "group knowledgebase link generation exhausted the signed-64-bit range".into(),
        ))
    })?;
    u64::try_from(next).map_err(|_| {
        RuntimeError::Contract(ContractError::Unavailable(
            "group knowledgebase link generation is invalid".into(),
        ))
    })
}

fn validate_group_knowledgebase_required_identifier(
    value: &str,
    label: &str,
    max_bytes: usize,
) -> Result<(), RuntimeError> {
    if value.trim().is_empty() || value.len() > max_bytes {
        return Err(RuntimeError::InvalidInput(format!(
            "group knowledgebase {label} is invalid"
        )));
    }
    Ok(())
}

fn validate_group_knowledgebase_source_event_id(source_event_id: &str) -> Result<(), RuntimeError> {
    validate_group_knowledgebase_required_identifier(
        source_event_id,
        "source event id",
        GROUP_KNOWLEDGEBASE_MAX_SOURCE_EVENT_ID_BYTES,
    )
}

fn validate_group_knowledgebase_actor_id(actor_id: &str) -> Result<(), RuntimeError> {
    validate_group_knowledgebase_required_identifier(
        actor_id,
        "actor id",
        GROUP_KNOWLEDGEBASE_MAX_ACTOR_ID_BYTES,
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupKnowledgebaseLifecycleState {
    /// Returned only by the IM read model. It is never persisted in the link
    /// table, whose rows exist only after an Owner/Admin starts provisioning.
    Absent,
    Provisioning,
    Active,
    Failed,
    Archived,
    Deleted,
}

impl GroupKnowledgebaseLifecycleState {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Provisioning => "provisioning",
            Self::Active => "active",
            Self::Failed => "failed",
            Self::Archived => "archived",
            Self::Deleted => "deleted",
        }
    }

    fn from_db(value: &str) -> Result<Self, RuntimeError> {
        match value {
            "absent" => Ok(Self::Absent),
            "provisioning" => Ok(Self::Provisioning),
            "active" => Ok(Self::Active),
            "failed" => Ok(Self::Failed),
            "archived" => Ok(Self::Archived),
            "deleted" => Ok(Self::Deleted),
            _ => Err(RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase lifecycle state is invalid".into(),
            ))),
        }
    }

    fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    fn requires_durable_reconciliation(&self) -> bool {
        matches!(self, Self::Provisioning | Self::Active | Self::Archived)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupKnowledgebaseLink {
    pub id: i64,
    pub link_uuid: String,
    pub scope: GroupKnowledgebaseScope,
    pub knowledge_space_id: Option<i64>,
    pub knowledge_space_uuid: Option<String>,
    pub knowledgebase_binding_id: Option<i64>,
    pub knowledgebase_binding_uuid: Option<String>,
    pub lifecycle_state: GroupKnowledgebaseLifecycleState,
    pub provisioning_operation_id: Option<String>,
    pub creation_idempotency_key: String,
    pub last_source_event_id: Option<String>,
    pub membership_epoch: u64,
    pub last_synchronized_membership_epoch: u64,
    pub last_error_code: Option<String>,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
}

impl GroupKnowledgebaseLink {
    fn new(
        id: i64,
        link_uuid: String,
        scope: GroupKnowledgebaseScope,
        actor_id: String,
        now: DateTime<Utc>,
    ) -> Self {
        let creation_idempotency_key = format!(
            "im-group-knowledgebase:{}",
            sha256_hash(
                format!(
                    "{}:{}:{}",
                    scope.tenant_id, scope.organization_id, scope.conversation_id
                )
                .as_bytes(),
            )
        );
        Self {
            id,
            link_uuid,
            scope,
            knowledge_space_id: None,
            knowledge_space_uuid: None,
            knowledgebase_binding_id: None,
            knowledgebase_binding_uuid: None,
            lifecycle_state: GroupKnowledgebaseLifecycleState::Provisioning,
            provisioning_operation_id: None,
            creation_idempotency_key,
            last_source_event_id: None,
            // Normalized Conversation membership is the source of truth. A
            // newly reserved link has not synchronized that current state yet.
            membership_epoch: 0,
            last_synchronized_membership_epoch: 0,
            last_error_code: None,
            created_by: actor_id.clone(),
            updated_by: actor_id,
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }

    pub fn view(&self) -> GroupKnowledgebaseLinkView {
        GroupKnowledgebaseLinkView {
            conversation_id: self.scope.conversation_id.clone(),
            space_id: self.knowledge_space_id.map(|id| id.to_string()),
            space_uuid: self.knowledge_space_uuid.clone(),
            lifecycle_state: self.lifecycle_state.clone(),
            provisioning_operation_id: self.provisioning_operation_id.clone(),
            membership_epoch: self.membership_epoch,
            upstream_link_generation: self.version,
            last_error_code: self.last_error_code.clone(),
        }
    }

    fn validate_for_persistence(&self) -> Result<(), RuntimeError> {
        self.scope.validate()?;
        if self.id <= 0 {
            return Err(RuntimeError::InvalidInput(
                "group knowledgebase link id must be positive".into(),
            ));
        }
        group_knowledgebase_u64_to_db_i64(self.membership_epoch, "membership epoch")?;
        group_knowledgebase_u64_to_db_i64(
            self.last_synchronized_membership_epoch,
            "last synchronized membership epoch",
        )?;
        validate_group_knowledgebase_nonzero_u64(self.version, "link generation")?;
        if self.last_synchronized_membership_epoch > self.membership_epoch {
            return Err(RuntimeError::Conflict(
                "group knowledgebase synchronized membership epoch exceeds its membership epoch"
                    .into(),
            ));
        }
        let has_target_reference = self.knowledge_space_id.is_some()
            || self.knowledge_space_uuid.is_some()
            || self.knowledgebase_binding_id.is_some()
            || self.knowledgebase_binding_uuid.is_some();
        if has_target_reference {
            GroupKnowledgebaseTargetFence::from_link(self)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupKnowledgebaseLinkView {
    pub conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_uuid: Option<String>,
    pub lifecycle_state: GroupKnowledgebaseLifecycleState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_operation_id: Option<String>,
    #[serde(serialize_with = "serialize_u64_as_decimal_string")]
    pub membership_epoch: u64,
    #[serde(serialize_with = "serialize_u64_as_decimal_string")]
    pub upstream_link_generation: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
}

impl GroupKnowledgebaseLinkView {
    fn absent(conversation_id: &str) -> Self {
        Self {
            conversation_id: conversation_id.to_owned(),
            space_id: None,
            space_uuid: None,
            lifecycle_state: GroupKnowledgebaseLifecycleState::Absent,
            provisioning_operation_id: None,
            membership_epoch: 0,
            upstream_link_generation: 0,
            last_error_code: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupKnowledgebaseLaunchView {
    pub conversation_id: String,
    pub space_id: String,
    pub space_uuid: String,
    pub launch_ticket: String,
    pub expires_at: String,
    #[serde(serialize_with = "serialize_u64_as_decimal_string")]
    pub membership_epoch: u64,
    #[serde(serialize_with = "serialize_u64_as_decimal_string")]
    pub upstream_link_generation: u64,
}

/// The public launch command returns one stable item shape. A missing ticket
/// means provisioning is still in progress; callers must never invent a KB
/// URL or bypass the ticket-consumption boundary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupKnowledgebaseLaunchResponse {
    pub conversation_id: String,
    pub lifecycle_state: GroupKnowledgebaseLifecycleState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(serialize_with = "serialize_u64_as_decimal_string")]
    pub membership_epoch: u64,
    #[serde(serialize_with = "serialize_u64_as_decimal_string")]
    pub upstream_link_generation: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_operation_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupKnowledgebaseLaunchResult {
    Ready(GroupKnowledgebaseLaunchView),
    Provisioning(GroupKnowledgebaseLinkView),
}

impl From<GroupKnowledgebaseLaunchResult> for GroupKnowledgebaseLaunchResponse {
    fn from(result: GroupKnowledgebaseLaunchResult) -> Self {
        match result {
            GroupKnowledgebaseLaunchResult::Ready(launch) => Self {
                conversation_id: launch.conversation_id,
                lifecycle_state: GroupKnowledgebaseLifecycleState::Active,
                space_id: Some(launch.space_id),
                space_uuid: Some(launch.space_uuid),
                launch_ticket: Some(launch.launch_ticket),
                expires_at: Some(launch.expires_at),
                membership_epoch: launch.membership_epoch,
                upstream_link_generation: launch.upstream_link_generation,
                provisioning_operation_id: None,
            },
            GroupKnowledgebaseLaunchResult::Provisioning(link) => Self {
                conversation_id: link.conversation_id,
                lifecycle_state: link.lifecycle_state,
                space_id: link.space_id,
                space_uuid: link.space_uuid,
                launch_ticket: None,
                expires_at: None,
                membership_epoch: link.membership_epoch,
                upstream_link_generation: link.upstream_link_generation,
                provisioning_operation_id: link.provisioning_operation_id,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupKnowledgebaseEnsureResult {
    Created(GroupKnowledgebaseLinkView),
    Existing(GroupKnowledgebaseLinkView),
    Provisioning(GroupKnowledgebaseLinkView),
}

impl GroupKnowledgebaseEnsureResult {
    fn view(&self) -> &GroupKnowledgebaseLinkView {
        match self {
            Self::Created(view) | Self::Existing(view) | Self::Provisioning(view) => view,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupKnowledgebaseMembership {
    pub principal_id: String,
    pub principal_kind: String,
    /// IM sends its authoritative role unchanged. Knowledgebase owns the
    /// corresponding resource-permission conversation_state, so there is one mapping
    /// boundary rather than independently maintained role translations.
    pub role: MembershipRole,
}

impl GroupKnowledgebaseMembership {
    fn validate(&self) -> Result<(), RuntimeError> {
        validate_group_knowledgebase_required_identifier(
            self.principal_id.as_str(),
            "member principal id",
            GROUP_KNOWLEDGEBASE_MAX_MEMBER_ID_BYTES,
        )?;
        validate_group_knowledgebase_required_identifier(
            self.principal_kind.as_str(),
            "member principal kind",
            GROUP_KNOWLEDGEBASE_MAX_MEMBER_ID_BYTES,
        )?;
        if self.principal_kind != GROUP_KNOWLEDGEBASE_SUPPORTED_MEMBER_PRINCIPAL_KIND {
            return Err(RuntimeError::InvalidInput(
                "group knowledgebase supports user members only".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnsureGroupKnowledgebaseRequest {
    pub scope: GroupKnowledgebaseScope,
    pub group_name: String,
    pub idempotency_key: String,
    pub source_event_id: String,
    pub membership_epoch: u64,
    pub members: Vec<GroupKnowledgebaseMembership>,
}

impl EnsureGroupKnowledgebaseRequest {
    fn validate(&self) -> Result<(), RuntimeError> {
        self.scope.validate()?;
        validate_group_knowledgebase_source_event_id(self.source_event_id.as_str())?;
        group_knowledgebase_u64_to_db_i64(self.membership_epoch, "membership epoch")?;
        for member in &self.members {
            member.validate()?;
        }
        Ok(())
    }
}

/// Full, authoritative roster replacement request. IM preserves its canonical
/// membership role (`owner|admin|member|guest`) and Knowledgebase owns the
/// resource ACL conversation_state, including guest grant revocation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynchronizeGroupKnowledgebaseMembersRequest {
    pub scope: GroupKnowledgebaseScope,
    pub knowledge_space_id: i64,
    pub knowledge_space_uuid: String,
    pub knowledgebase_binding_id: i64,
    pub knowledgebase_binding_uuid: String,
    pub upstream_link_generation: u64,
    pub membership_epoch: u64,
    pub source_event_id: String,
    pub members: Vec<GroupKnowledgebaseMembership>,
}

impl SynchronizeGroupKnowledgebaseMembersRequest {
    fn validate(&self) -> Result<(), RuntimeError> {
        self.scope.validate()?;
        GroupKnowledgebaseTargetFence {
            knowledge_space_id: self.knowledge_space_id,
            knowledge_space_uuid: self.knowledge_space_uuid.clone(),
            knowledgebase_binding_id: self.knowledgebase_binding_id,
            knowledgebase_binding_uuid: self.knowledgebase_binding_uuid.clone(),
        }
        .validate()?;
        validate_group_knowledgebase_nonzero_u64(
            self.upstream_link_generation,
            "upstream link generation",
        )?;
        group_knowledgebase_u64_to_db_i64(self.membership_epoch, "membership epoch")?;
        validate_group_knowledgebase_source_event_id(self.source_event_id.as_str())?;
        for member in &self.members {
            member.validate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveGroupKnowledgebaseRequest {
    pub scope: GroupKnowledgebaseScope,
    pub knowledge_space_id: i64,
    pub knowledge_space_uuid: String,
    pub knowledgebase_binding_id: i64,
    pub knowledgebase_binding_uuid: String,
    pub membership_epoch: u64,
    pub upstream_link_generation: u64,
    pub source_event_id: String,
    pub archived_by: String,
}

impl ArchiveGroupKnowledgebaseRequest {
    fn validate(&self) -> Result<(), RuntimeError> {
        self.scope.validate()?;
        GroupKnowledgebaseTargetFence {
            knowledge_space_id: self.knowledge_space_id,
            knowledge_space_uuid: self.knowledge_space_uuid.clone(),
            knowledgebase_binding_id: self.knowledgebase_binding_id,
            knowledgebase_binding_uuid: self.knowledgebase_binding_uuid.clone(),
        }
        .validate()?;
        validate_group_knowledgebase_nonzero_u64(
            self.upstream_link_generation,
            "upstream link generation",
        )?;
        group_knowledgebase_u64_to_db_i64(self.membership_epoch, "membership epoch")?;
        validate_group_knowledgebase_source_event_id(self.source_event_id.as_str())?;
        validate_group_knowledgebase_actor_id(self.archived_by.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupKnowledgebaseOutboxOperation {
    SynchronizeMembers,
    Archive,
}

/// Durable payload held in IM's outbox. It intentionally contains no launch
/// ticket, end-user credential, or mutable caller context.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupKnowledgebaseOutboxPayload {
    pub operation: GroupKnowledgebaseOutboxOperation,
    pub source_event_id: String,
    pub scope: GroupKnowledgebaseScope,
    pub knowledge_space_id: i64,
    pub knowledge_space_uuid: String,
    pub knowledgebase_binding_id: i64,
    pub knowledgebase_binding_uuid: String,
    pub upstream_link_generation: u64,
    pub membership_epoch: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<GroupKnowledgebaseMembership>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<String>,
}

impl GroupKnowledgebaseOutboxPayload {
    fn validate(&self) -> Result<(), RuntimeError> {
        self.scope.validate()?;
        GroupKnowledgebaseTargetFence {
            knowledge_space_id: self.knowledge_space_id,
            knowledge_space_uuid: self.knowledge_space_uuid.clone(),
            knowledgebase_binding_id: self.knowledgebase_binding_id,
            knowledgebase_binding_uuid: self.knowledgebase_binding_uuid.clone(),
        }
        .validate()?;
        validate_group_knowledgebase_nonzero_u64(
            self.upstream_link_generation,
            "upstream link generation",
        )?;
        group_knowledgebase_u64_to_db_i64(self.membership_epoch, "membership epoch")?;
        validate_group_knowledgebase_source_event_id(self.source_event_id.as_str())?;
        for member in &self.members {
            member.validate()?;
        }
        match (&self.operation, &self.archived_by) {
            (GroupKnowledgebaseOutboxOperation::SynchronizeMembers, None) => Ok(()),
            (GroupKnowledgebaseOutboxOperation::Archive, Some(archived_by)) => {
                validate_group_knowledgebase_actor_id(archived_by)
            }
            _ => Err(RuntimeError::Conflict(
                "group knowledgebase outbox payload does not match its operation".into(),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnsuredGroupKnowledgebase {
    pub knowledge_space_id: i64,
    pub knowledge_space_uuid: String,
    pub knowledgebase_binding_id: i64,
    pub knowledgebase_binding_uuid: String,
    pub provisioning_operation_id: Option<String>,
    pub membership_epoch: u64,
}

impl EnsuredGroupKnowledgebase {
    fn validate(&self) -> Result<(), RuntimeError> {
        GroupKnowledgebaseTargetFence {
            knowledge_space_id: self.knowledge_space_id,
            knowledge_space_uuid: self.knowledge_space_uuid.clone(),
            knowledgebase_binding_id: self.knowledgebase_binding_id,
            knowledgebase_binding_uuid: self.knowledgebase_binding_uuid.clone(),
        }
        .validate()?;
        group_knowledgebase_u64_to_db_i64(self.membership_epoch, "membership epoch")?;
        if let Some(operation_id) = self.provisioning_operation_id.as_deref() {
            validate_group_knowledgebase_required_identifier(
                operation_id,
                "provisioning operation id",
                GROUP_KNOWLEDGEBASE_MAX_SOURCE_EVENT_ID_BYTES,
            )?;
        }
        Ok(())
    }
}

/// Immutable Knowledgebase target identity. IM owns the link generation used
/// for its own optimistic fences; Knowledgebase owns this target quartet and
/// verifies it before accepting ACL or archive lifecycle work.
#[derive(Clone, Debug, PartialEq, Eq)]
struct GroupKnowledgebaseTargetFence {
    knowledge_space_id: i64,
    knowledge_space_uuid: String,
    knowledgebase_binding_id: i64,
    knowledgebase_binding_uuid: String,
}

impl GroupKnowledgebaseTargetFence {
    fn from_link(link: &GroupKnowledgebaseLink) -> Result<Self, RuntimeError> {
        let target = Self {
            knowledge_space_id: link.knowledge_space_id.ok_or_else(|| {
                RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase link is missing its space reference".into(),
                ))
            })?,
            knowledge_space_uuid: link.knowledge_space_uuid.clone().ok_or_else(|| {
                RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase link is missing its space reference".into(),
                ))
            })?,
            knowledgebase_binding_id: link.knowledgebase_binding_id.ok_or_else(|| {
                RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase link is missing its binding reference".into(),
                ))
            })?,
            knowledgebase_binding_uuid: link.knowledgebase_binding_uuid.clone().ok_or_else(
                || {
                    RuntimeError::Contract(ContractError::Unavailable(
                        "group knowledgebase link is missing its binding reference".into(),
                    ))
                },
            )?,
        };
        target.validate()?;
        Ok(target)
    }

    fn validate(&self) -> Result<(), RuntimeError> {
        if self.knowledge_space_id <= 0 || self.knowledgebase_binding_id <= 0 {
            return Err(RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase target contains an invalid identifier".into(),
            )));
        }
        validate_group_knowledgebase_required_identifier(
            self.knowledge_space_uuid.as_str(),
            "knowledge space uuid",
            GROUP_KNOWLEDGEBASE_MAX_TARGET_UUID_BYTES,
        )?;
        validate_group_knowledgebase_required_identifier(
            self.knowledgebase_binding_uuid.as_str(),
            "knowledgebase binding uuid",
            GROUP_KNOWLEDGEBASE_MAX_TARGET_UUID_BYTES,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupKnowledgebasePortError {
    Unavailable,
    Conflict,
    Rejected,
}

/// Archive completion is intentionally distinct from successful acceptance.
/// Large ACL revocations are paged by Knowledgebase and must remain in IM's
/// durable outbox until the remote binding reaches a terminal state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupKnowledgebaseArchiveDeliveryState {
    Archiving,
    Archived,
    Deleted,
}

/// The only IM -> Knowledgebase integration boundary.  The production
/// implementation is injected by runtime composition and uses the generated
/// Knowledgebase internal RPC SDK; this contract intentionally has no HTTP
/// types.
#[async_trait]
pub trait GroupKnowledgebasePort: Send + Sync {
    /// Construction-time configuration and credential preflight. Production
    /// composition calls this before the durable outbox worker begins to claim
    /// events, so an unavailable provider cannot turn a healthy IM process
    /// into a permanent retry loop.
    async fn ensure_delivery_ready(&self) -> Result<(), GroupKnowledgebasePortError>;

    async fn ensure_group_knowledgebase(
        &self,
        request: EnsureGroupKnowledgebaseRequest,
    ) -> Result<EnsuredGroupKnowledgebase, GroupKnowledgebasePortError>;

    async fn synchronize_group_members(
        &self,
        request: SynchronizeGroupKnowledgebaseMembersRequest,
    ) -> Result<(), GroupKnowledgebasePortError>;

    async fn archive_group_knowledgebase(
        &self,
        request: ArchiveGroupKnowledgebaseRequest,
    ) -> Result<GroupKnowledgebaseArchiveDeliveryState, GroupKnowledgebasePortError>;
}

#[derive(Default)]
pub struct UnavailableGroupKnowledgebasePort;

#[async_trait]
impl GroupKnowledgebasePort for UnavailableGroupKnowledgebasePort {
    async fn ensure_delivery_ready(&self) -> Result<(), GroupKnowledgebasePortError> {
        Err(GroupKnowledgebasePortError::Unavailable)
    }

    async fn ensure_group_knowledgebase(
        &self,
        _request: EnsureGroupKnowledgebaseRequest,
    ) -> Result<EnsuredGroupKnowledgebase, GroupKnowledgebasePortError> {
        Err(GroupKnowledgebasePortError::Unavailable)
    }

    async fn synchronize_group_members(
        &self,
        _request: SynchronizeGroupKnowledgebaseMembersRequest,
    ) -> Result<(), GroupKnowledgebasePortError> {
        Err(GroupKnowledgebasePortError::Unavailable)
    }

    async fn archive_group_knowledgebase(
        &self,
        _request: ArchiveGroupKnowledgebaseRequest,
    ) -> Result<GroupKnowledgebaseArchiveDeliveryState, GroupKnowledgebasePortError> {
        Err(GroupKnowledgebasePortError::Unavailable)
    }
}

#[derive(Clone, Debug)]
struct GroupKnowledgebaseLaunchTicket {
    id: i64,
    ticket_hash: String,
    scope: GroupKnowledgebaseScope,
    knowledge_space_id: i64,
    knowledge_space_uuid: String,
    knowledgebase_binding_id: i64,
    knowledgebase_binding_uuid: String,
    upstream_link_generation: u64,
    membership_epoch: u64,
    actor_kind: String,
    actor_id: String,
    principal_kind: String,
    principal_id: String,
    session_id: String,
    issuing_app_id: Option<String>,
    issued_by: String,
    idempotency_key_hash: String,
    request_fingerprint_hash: String,
    ticket_ciphertext: String,
    expires_at: DateTime<Utc>,
    consumed_at: Option<DateTime<Utc>>,
    consumed_by_service: Option<String>,
    consumed_trace_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GroupKnowledgebaseLaunchTicketIdempotencyScope {
    scope: GroupKnowledgebaseScope,
    actor_kind: String,
    actor_id: String,
    principal_kind: String,
    principal_id: String,
    session_id: String,
    idempotency_key_hash: String,
}

impl GroupKnowledgebaseLaunchTicket {
    fn idempotency_scope(&self) -> GroupKnowledgebaseLaunchTicketIdempotencyScope {
        GroupKnowledgebaseLaunchTicketIdempotencyScope {
            scope: self.scope.clone(),
            actor_kind: self.actor_kind.clone(),
            actor_id: self.actor_id.clone(),
            principal_kind: self.principal_kind.clone(),
            principal_id: self.principal_id.clone(),
            session_id: self.session_id.clone(),
            idempotency_key_hash: self.idempotency_key_hash.clone(),
        }
    }
}

enum GroupKnowledgebaseLaunchTicketReservation {
    Created,
    Existing(Box<GroupKnowledgebaseLaunchTicket>),
}

/// Encrypts the one raw ticket value retained solely for a same-key launch
/// replay. The database stores a SHA-256 verifier plus this AEAD ciphertext;
/// it never stores a recoverable capability in plaintext.
#[derive(Clone)]
struct GroupKnowledgebaseLaunchTicketCipher {
    key: [u8; 32],
}

impl GroupKnowledgebaseLaunchTicketCipher {
    fn from_runtime_env() -> Result<Self, RuntimeError> {
        let secret = resolve_group_knowledgebase_ticket_secret()?;
        Ok(Self {
            key: derive_aes_256_key(
                secret.as_bytes(),
                GROUP_KNOWLEDGEBASE_TICKET_KEY_SALT,
                GROUP_KNOWLEDGEBASE_TICKET_KEY_INFO,
            ),
        })
    }

    #[cfg(test)]
    fn for_test() -> Self {
        Self {
            key: derive_aes_256_key(
                b"group-knowledgebase-test-ticket-secret-32-bytes",
                GROUP_KNOWLEDGEBASE_TICKET_KEY_SALT,
                GROUP_KNOWLEDGEBASE_TICKET_KEY_INFO,
            ),
        }
    }

    fn encrypt(&self, ticket: &str) -> Result<String, RuntimeError> {
        aes_gcm_encrypt(&self.key, ticket.as_bytes()).map_err(|_| {
            RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase launch ticket encryption failed".into(),
            ))
        })
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, RuntimeError> {
        let bytes = aes_gcm_decrypt(&self.key, ciphertext).map_err(|_| {
            RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase launch ticket replay state is invalid".into(),
            ))
        })?;
        String::from_utf8(bytes).map_err(|_| {
            RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase launch ticket replay state is invalid".into(),
            ))
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsumedGroupKnowledgebaseLaunchTicket {
    pub conversation_id: String,
    pub space_id: String,
    pub space_uuid: String,
    pub knowledgebase_binding_id: String,
    pub knowledgebase_binding_uuid: String,
    pub lifecycle_state: GroupKnowledgebaseLifecycleState,
    pub membership_role: MembershipRole,
    pub membership_epoch: u64,
    pub upstream_link_generation: u64,
    pub expires_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GroupKnowledgebaseReconciliationScope {
    tenant_id: String,
    organization_id: String,
}

#[derive(Clone, Copy, Debug)]
struct GroupKnowledgebaseReconciliationScopeRequest<'a> {
    context: &'a PrivilegedOperationContext,
    after: Option<&'a GroupKnowledgebaseReconciliationScope>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct GroupKnowledgebaseReconciliationCursor {
    completed_scope: Option<GroupKnowledgebaseReconciliationScope>,
    active_scope: Option<GroupKnowledgebaseReconciliationScope>,
    link_after_id: Option<i64>,
    pending_provisioning_recovery: Option<GroupKnowledgebaseProvisioningRecovery>,
}

#[derive(Clone, Debug)]
struct GroupKnowledgebaseReconciliationLinkPage {
    links: Vec<GroupKnowledgebaseLink>,
    next_link_id: Option<i64>,
}

#[derive(Clone, Debug)]
struct GroupKnowledgebaseDurableSnapshot {
    membership_epoch: u64,
    roster: ConversationRoster,
    archive: Option<GroupKnowledgebaseArchiveReconciliation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GroupKnowledgebaseArchiveReconciliation {
    source_event_id: String,
    actor_id: String,
}

#[derive(Clone, Debug)]
struct GroupKnowledgebaseProvisioningRecovery {
    link: GroupKnowledgebaseLink,
}

enum GroupKnowledgebaseReconciliationLinkOutcome {
    Reconciled,
    ProvisioningRecovery(Box<GroupKnowledgebaseProvisioningRecovery>),
}

trait GroupKnowledgebaseStore: Send + Sync {
    fn get_link(
        &self,
        scope: &GroupKnowledgebaseScope,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError>;
    fn reserve_link(
        &self,
        candidate: GroupKnowledgebaseLink,
    ) -> Result<GroupKnowledgebaseLinkReservation, RuntimeError>;
    fn begin_retry_provisioning(
        &self,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        source_event_id: &str,
    ) -> Result<GroupKnowledgebaseLink, RuntimeError>;
    fn activate_link(
        &self,
        scope: &GroupKnowledgebaseScope,
        ensured: EnsuredGroupKnowledgebase,
        membership_epoch: u64,
        actor_id: &str,
        source_event_id: &str,
        archive_outbox_id: &str,
    ) -> Result<GroupKnowledgebaseLink, RuntimeError>;
    fn fail_link(
        &self,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        error_code: &str,
    ) -> Result<(), RuntimeError>;
    fn enqueue_membership_synchronization(
        &self,
        request: GroupKnowledgebaseMembershipSyncEnqueue,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError>;
    fn mark_membership_synchronized(
        &self,
        scope: &GroupKnowledgebaseScope,
        membership_epoch: u64,
        upstream_link_generation: u64,
        actor_id: &str,
    ) -> Result<bool, RuntimeError>;
    fn archive_link_and_enqueue(
        &self,
        request: GroupKnowledgebaseArchiveEnqueue,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError>;
    fn next_reconciliation_scope(
        &self,
        request: GroupKnowledgebaseReconciliationScopeRequest<'_>,
    ) -> Result<Option<GroupKnowledgebaseReconciliationScope>, RuntimeError>;
    fn list_reconciliation_links(
        &self,
        scope: &GroupKnowledgebaseReconciliationScope,
        after_link_id: Option<i64>,
        limit: usize,
    ) -> Result<GroupKnowledgebaseReconciliationLinkPage, RuntimeError>;
    fn reserve_ticket(
        &self,
        ticket: GroupKnowledgebaseLaunchTicket,
    ) -> Result<GroupKnowledgebaseLaunchTicketReservation, RuntimeError>;
    fn find_unconsumed_ticket_for_consumer(
        &self,
        ticket_hash: &str,
        auth: &AppContext,
    ) -> Result<Option<GroupKnowledgebaseLaunchTicket>, RuntimeError>;
    fn consume_ticket_if_current(
        &self,
        ticket: &GroupKnowledgebaseLaunchTicket,
        auth: &AppContext,
        consumed_trace_id: &str,
    ) -> Result<bool, RuntimeError>;
}

#[derive(Clone, Debug)]
struct GroupKnowledgebaseMembershipSyncEnqueue {
    scope: GroupKnowledgebaseScope,
    actor_id: String,
    source_event_id: String,
    target_membership_epoch: u64,
    members: Vec<GroupKnowledgebaseMembership>,
    outbox_id: String,
    occurred_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
struct GroupKnowledgebaseArchiveEnqueue {
    scope: GroupKnowledgebaseScope,
    actor_id: String,
    source_event_id: String,
    outbox_id: String,
    occurred_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
struct GroupKnowledgebaseLinkReservation {
    link: GroupKnowledgebaseLink,
    newly_reserved: bool,
}

#[derive(Default)]
struct InMemoryGroupKnowledgebaseStore {
    links: Mutex<HashMap<GroupKnowledgebaseScope, GroupKnowledgebaseLink>>,
    tickets: Mutex<InMemoryGroupKnowledgebaseTickets>,
    outbox: Mutex<BTreeMap<String, GroupKnowledgebaseOutboxPayload>>,
}

#[derive(Default)]
struct InMemoryGroupKnowledgebaseTickets {
    by_hash: HashMap<String, GroupKnowledgebaseLaunchTicket>,
    by_idempotency: HashMap<GroupKnowledgebaseLaunchTicketIdempotencyScope, String>,
}

impl InMemoryGroupKnowledgebaseStore {
    fn enqueue_outbox_payload(
        &self,
        payload: GroupKnowledgebaseOutboxPayload,
    ) -> Result<(), RuntimeError> {
        let event_id = group_knowledgebase_outbox_event_id(
            &payload.operation,
            &payload.scope,
            payload.source_event_id.as_str(),
        );
        lock_knowledgebase_mutex(&self.outbox, "knowledgebase-outbox")
            .entry(event_id)
            .or_insert(payload);
        Ok(())
    }

    #[cfg(test)]
    fn pending_outbox_payloads(&self) -> Vec<GroupKnowledgebaseOutboxPayload> {
        lock_knowledgebase_mutex(&self.outbox, "knowledgebase-outbox")
            .values()
            .cloned()
            .collect()
    }
}

impl GroupKnowledgebaseStore for InMemoryGroupKnowledgebaseStore {
    fn get_link(
        &self,
        scope: &GroupKnowledgebaseScope,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        scope.validate()?;
        Ok(lock_knowledgebase_mutex(&self.links, "knowledgebase-links")
            .get(scope)
            .cloned())
    }

    fn reserve_link(
        &self,
        candidate: GroupKnowledgebaseLink,
    ) -> Result<GroupKnowledgebaseLinkReservation, RuntimeError> {
        candidate.scope.validate()?;
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        match links.entry(candidate.scope.clone()) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(candidate.clone());
                Ok(GroupKnowledgebaseLinkReservation {
                    link: candidate,
                    newly_reserved: true,
                })
            }
            std::collections::hash_map::Entry::Occupied(entry) => {
                Ok(GroupKnowledgebaseLinkReservation {
                    link: entry.get().clone(),
                    newly_reserved: false,
                })
            }
        }
    }

    fn begin_retry_provisioning(
        &self,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        source_event_id: &str,
    ) -> Result<GroupKnowledgebaseLink, RuntimeError> {
        scope.validate()?;
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        let link = links.get_mut(scope).ok_or_else(|| {
            RuntimeError::ConversationBindingNotFound(scope.conversation_id.clone())
        })?;
        match link.lifecycle_state {
            GroupKnowledgebaseLifecycleState::Failed => {
                link.lifecycle_state = GroupKnowledgebaseLifecycleState::Provisioning;
                link.provisioning_operation_id = None;
                link.last_source_event_id = Some(source_event_id.to_owned());
                link.last_error_code = None;
                link.updated_by = actor_id.to_owned();
                link.updated_at = Utc::now();
                link.version = next_group_knowledgebase_version(link.version)?;
                Ok(link.clone())
            }
            GroupKnowledgebaseLifecycleState::Provisioning => Ok(link.clone()),
            GroupKnowledgebaseLifecycleState::Archived
            | GroupKnowledgebaseLifecycleState::Deleted => Err(RuntimeError::Conflict(
                "group knowledgebase lifecycle does not permit automatic reprovisioning".into(),
            )),
            GroupKnowledgebaseLifecycleState::Active | GroupKnowledgebaseLifecycleState::Absent => {
                Err(RuntimeError::Conflict(
                    "group knowledgebase is not eligible for provisioning retry".into(),
                ))
            }
        }
    }

    fn activate_link(
        &self,
        scope: &GroupKnowledgebaseScope,
        ensured: EnsuredGroupKnowledgebase,
        membership_epoch: u64,
        actor_id: &str,
        source_event_id: &str,
        _archive_outbox_id: &str,
    ) -> Result<GroupKnowledgebaseLink, RuntimeError> {
        scope.validate()?;
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        let link = links.get_mut(scope).ok_or_else(|| {
            RuntimeError::ConversationBindingNotFound(scope.conversation_id.clone())
        })?;
        match link.lifecycle_state {
            GroupKnowledgebaseLifecycleState::Provisioning => {
                link.knowledge_space_id = Some(ensured.knowledge_space_id);
                link.knowledge_space_uuid = Some(ensured.knowledge_space_uuid);
                link.knowledgebase_binding_id = Some(ensured.knowledgebase_binding_id);
                link.knowledgebase_binding_uuid = Some(ensured.knowledgebase_binding_uuid);
                link.provisioning_operation_id = ensured.provisioning_operation_id;
                link.membership_epoch = membership_epoch;
                // Ensure creates the KB binding synchronously, but durable
                // full-roster synchronization still runs after activation.
                // Until that succeeds, launch-ticket issuance remains
                // fail-closed.
                link.last_synchronized_membership_epoch = 0;
                link.last_source_event_id = Some(source_event_id.to_owned());
                link.last_error_code = None;
                link.lifecycle_state = GroupKnowledgebaseLifecycleState::Active;
                link.updated_by = actor_id.to_owned();
                link.updated_at = Utc::now();
                link.version = next_group_knowledgebase_version(link.version)?;
                Ok(link.clone())
            }
            GroupKnowledgebaseLifecycleState::Archived => {
                if let Some(existing_space_id) = link.knowledge_space_id
                    && (existing_space_id != ensured.knowledge_space_id
                        || link.knowledge_space_uuid.as_deref()
                            != Some(ensured.knowledge_space_uuid.as_str())
                        || link.knowledgebase_binding_id != Some(ensured.knowledgebase_binding_id)
                        || link.knowledgebase_binding_uuid.as_deref()
                            != Some(ensured.knowledgebase_binding_uuid.as_str()))
                {
                    return Err(RuntimeError::Conflict(
                        "archived group knowledgebase link conflicts with a provisioning result"
                            .into(),
                    ));
                }
                if link.knowledge_space_id.is_none() {
                    link.knowledge_space_id = Some(ensured.knowledge_space_id);
                    link.knowledge_space_uuid = Some(ensured.knowledge_space_uuid);
                    link.knowledgebase_binding_id = Some(ensured.knowledgebase_binding_id);
                    link.knowledgebase_binding_uuid = Some(ensured.knowledgebase_binding_uuid);
                    link.provisioning_operation_id = ensured.provisioning_operation_id;
                    link.membership_epoch = link.membership_epoch.max(membership_epoch);
                    link.last_error_code = None;
                    // Preserve archive actor/source for the required archive
                    // handoff; an ensure completion must never overwrite it.
                    link.updated_at = Utc::now();
                    link.version = next_group_knowledgebase_version(link.version)?;
                }
                let archived = link.clone();
                let source_event_id = archived.last_source_event_id.clone().ok_or_else(|| {
                    RuntimeError::Conflict(
                        "archived group knowledgebase link is missing its durable archive source"
                            .into(),
                    )
                })?;
                let payload = group_knowledgebase_archive_outbox_payload(
                    &archived,
                    source_event_id,
                    archived.updated_by.clone(),
                )?;
                drop(links);
                self.enqueue_outbox_payload(payload)?;
                Ok(archived)
            }
            GroupKnowledgebaseLifecycleState::Active => {
                if link.knowledge_space_id == Some(ensured.knowledge_space_id)
                    && link.knowledge_space_uuid.as_deref()
                        == Some(ensured.knowledge_space_uuid.as_str())
                    && link.knowledgebase_binding_id == Some(ensured.knowledgebase_binding_id)
                    && link.knowledgebase_binding_uuid.as_deref()
                        == Some(ensured.knowledgebase_binding_uuid.as_str())
                {
                    Ok(link.clone())
                } else {
                    Err(RuntimeError::Conflict(
                        "active group knowledgebase link conflicts with a provisioning result"
                            .into(),
                    ))
                }
            }
            GroupKnowledgebaseLifecycleState::Failed
            | GroupKnowledgebaseLifecycleState::Deleted
            | GroupKnowledgebaseLifecycleState::Absent => Err(RuntimeError::Conflict(
                "group knowledgebase lifecycle does not permit provisioning completion".into(),
            )),
        }
    }

    fn fail_link(
        &self,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        error_code: &str,
    ) -> Result<(), RuntimeError> {
        scope.validate()?;
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        if let Some(link) = links.get_mut(scope)
            && matches!(
                link.lifecycle_state,
                GroupKnowledgebaseLifecycleState::Provisioning
            )
        {
            link.lifecycle_state = GroupKnowledgebaseLifecycleState::Failed;
            link.last_error_code = Some(error_code.to_owned());
            link.updated_by = actor_id.to_owned();
            link.updated_at = Utc::now();
            link.version = next_group_knowledgebase_version(link.version)?;
        }
        Ok(())
    }

    fn enqueue_membership_synchronization(
        &self,
        request: GroupKnowledgebaseMembershipSyncEnqueue,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        request.scope.validate()?;
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        let Some(link) = links.get_mut(&request.scope) else {
            return Ok(None);
        };
        if !link.lifecycle_state.is_active()
            || request.target_membership_epoch < link.membership_epoch
            || (request.target_membership_epoch == link.membership_epoch
                && link.last_synchronized_membership_epoch >= request.target_membership_epoch)
        {
            return Ok(Some(link.clone()));
        }
        if link.last_source_event_id.as_deref() != Some(request.source_event_id.as_str()) {
            link.membership_epoch = request.target_membership_epoch;
            link.last_source_event_id = Some(request.source_event_id.clone());
            link.updated_by = request.actor_id.clone();
            link.updated_at = request.occurred_at;
            link.version = next_group_knowledgebase_version(link.version)?;
        }
        let updated = link.clone();
        let payload = group_knowledgebase_membership_outbox_payload(
            &updated,
            request.source_event_id,
            request.members,
        )?;
        let _outbox_id = request.outbox_id;
        drop(links);
        self.enqueue_outbox_payload(payload)?;
        Ok(Some(updated))
    }

    fn mark_membership_synchronized(
        &self,
        scope: &GroupKnowledgebaseScope,
        membership_epoch: u64,
        upstream_link_generation: u64,
        actor_id: &str,
    ) -> Result<bool, RuntimeError> {
        scope.validate()?;
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        let Some(link) = links.get_mut(scope) else {
            return Ok(false);
        };
        if !link.lifecycle_state.is_active()
            || link.membership_epoch != membership_epoch
            || link.version != upstream_link_generation
        {
            return Ok(false);
        }
        link.last_synchronized_membership_epoch = membership_epoch;
        link.updated_by = actor_id.to_owned();
        link.updated_at = Utc::now();
        Ok(true)
    }

    fn archive_link_and_enqueue(
        &self,
        request: GroupKnowledgebaseArchiveEnqueue,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        request.scope.validate()?;
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        let Some(link) = links.get_mut(&request.scope) else {
            return Ok(None);
        };
        if matches!(
            link.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Archived
        ) {
            if link.last_source_event_id.as_deref() != Some(request.source_event_id.as_str()) {
                return Err(RuntimeError::Conflict(
                    "archived group knowledgebase link conflicts with the durable archive source"
                        .into(),
                ));
            }
            let archived = link.clone();
            let payload = match archived.knowledge_space_id {
                Some(_) => Some(group_knowledgebase_archive_outbox_payload(
                    &archived,
                    request.source_event_id,
                    request.actor_id,
                )?),
                None => None,
            };
            drop(links);
            if let Some(payload) = payload {
                self.enqueue_outbox_payload(payload)?;
            }
            return Ok(Some(archived));
        }
        if !matches!(
            link.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Active
                | GroupKnowledgebaseLifecycleState::Provisioning
                | GroupKnowledgebaseLifecycleState::Failed
        ) {
            return Err(RuntimeError::Conflict(
                "group knowledgebase lifecycle does not permit archival".into(),
            ));
        }
        link.lifecycle_state = GroupKnowledgebaseLifecycleState::Archived;
        link.last_source_event_id = Some(request.source_event_id.clone());
        link.updated_by = request.actor_id.clone();
        link.updated_at = request.occurred_at;
        link.version = next_group_knowledgebase_version(link.version)?;
        let archived = link.clone();
        let payload = match archived.knowledge_space_id {
            Some(_) => Some(group_knowledgebase_archive_outbox_payload(
                &archived,
                request.source_event_id,
                request.actor_id,
            )?),
            None => None,
        };
        let _outbox_id = request.outbox_id;
        drop(links);
        if let Some(payload) = payload {
            self.enqueue_outbox_payload(payload)?;
        }
        Ok(Some(archived))
    }

    fn next_reconciliation_scope(
        &self,
        request: GroupKnowledgebaseReconciliationScopeRequest<'_>,
    ) -> Result<Option<GroupKnowledgebaseReconciliationScope>, RuntimeError> {
        let after = request.after;
        let links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        let mut scopes = BTreeMap::new();
        for link in links.values() {
            if !link.lifecycle_state.requires_durable_reconciliation() {
                continue;
            }
            link.scope.validate()?;
            scopes.insert(
                GroupKnowledgebaseReconciliationScope {
                    tenant_id: link.scope.tenant_id.clone(),
                    organization_id: link.scope.organization_id.clone(),
                },
                (),
            );
        }
        Ok(scopes
            .into_keys()
            .find(|scope| after.is_none_or(|after| scope > after)))
    }

    fn list_reconciliation_links(
        &self,
        scope: &GroupKnowledgebaseReconciliationScope,
        after_link_id: Option<i64>,
        limit: usize,
    ) -> Result<GroupKnowledgebaseReconciliationLinkPage, RuntimeError> {
        validate_group_knowledgebase_tenant_id(scope.tenant_id.as_str())?;
        validate_group_knowledgebase_organization_id(scope.organization_id.as_str())?;
        let page_size = limit.max(1);
        let mut links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links")
            .values()
            .filter(|link| {
                link.scope.tenant_id == scope.tenant_id
                    && link.scope.organization_id == scope.organization_id
                    && link.lifecycle_state.requires_durable_reconciliation()
                    && after_link_id.is_none_or(|after_link_id| link.id > after_link_id)
            })
            .cloned()
            .collect::<Vec<_>>();
        links.sort_by_key(|link| link.id);
        let has_more = links.len() > page_size;
        links.truncate(page_size);
        Ok(GroupKnowledgebaseReconciliationLinkPage {
            next_link_id: has_more.then(|| links.last().map(|link| link.id)).flatten(),
            links,
        })
    }

    fn reserve_ticket(
        &self,
        ticket: GroupKnowledgebaseLaunchTicket,
    ) -> Result<GroupKnowledgebaseLaunchTicketReservation, RuntimeError> {
        ticket.scope.validate()?;
        let mut tickets = lock_knowledgebase_mutex(&self.tickets, "knowledgebase-tickets");
        let idempotency_scope = ticket.idempotency_scope();
        if let Some(existing_hash) = tickets.by_idempotency.get(&idempotency_scope)
            && let Some(existing) = tickets.by_hash.get(existing_hash)
        {
            return Ok(GroupKnowledgebaseLaunchTicketReservation::Existing(
                Box::new(existing.clone()),
            ));
        }
        tickets
            .by_idempotency
            .insert(idempotency_scope, ticket.ticket_hash.clone());
        tickets.by_hash.insert(ticket.ticket_hash.clone(), ticket);
        Ok(GroupKnowledgebaseLaunchTicketReservation::Created)
    }

    fn find_unconsumed_ticket_for_consumer(
        &self,
        ticket_hash: &str,
        auth: &AppContext,
    ) -> Result<Option<GroupKnowledgebaseLaunchTicket>, RuntimeError> {
        let organization_id = resolve_group_knowledgebase_organization_id(auth)?;
        let session_id = require_group_knowledgebase_ticket_session(auth)?;
        Ok(
            lock_knowledgebase_mutex(&self.tickets, "knowledgebase-tickets")
                .by_hash
                .get(ticket_hash)
                .filter(|ticket| {
                    ticket.scope.tenant_id == auth.tenant_id
                        && ticket.scope.organization_id == organization_id
                        && ticket.actor_id == auth.actor_id
                        && ticket.actor_kind == auth.actor_kind
                        && ticket.principal_id == auth.user_id
                        && ticket.principal_kind == auth.actor_kind
                        && ticket.session_id == session_id
                        && ticket.consumed_at.is_none()
                        && ticket.expires_at > Utc::now()
                })
                .cloned(),
        )
    }

    fn consume_ticket_if_current(
        &self,
        ticket: &GroupKnowledgebaseLaunchTicket,
        auth: &AppContext,
        consumed_trace_id: &str,
    ) -> Result<bool, RuntimeError> {
        ticket.scope.validate()?;
        let organization_id = resolve_group_knowledgebase_organization_id(auth)?;
        let links = lock_knowledgebase_mutex(&self.links, "knowledgebase-links");
        let mut tickets = lock_knowledgebase_mutex(&self.tickets, "knowledgebase-tickets");
        let Some(stored) = tickets.by_hash.get_mut(ticket.ticket_hash.as_str()) else {
            return Ok(false);
        };
        let link_is_current = links.get(&stored.scope).is_some_and(|link| {
            link.lifecycle_state.is_active()
                && link.knowledge_space_id == Some(stored.knowledge_space_id)
                && link.knowledge_space_uuid.as_deref()
                    == Some(stored.knowledge_space_uuid.as_str())
                && link.knowledgebase_binding_id == Some(stored.knowledgebase_binding_id)
                && link.knowledgebase_binding_uuid.as_deref()
                    == Some(stored.knowledgebase_binding_uuid.as_str())
                && link.version == stored.upstream_link_generation
                && link.membership_epoch == stored.membership_epoch
                && link.last_synchronized_membership_epoch == stored.membership_epoch
        });
        if stored.scope != ticket.scope
            || stored.actor_kind != ticket.actor_kind
            || stored.actor_id != ticket.actor_id
            || stored.principal_kind != ticket.principal_kind
            || stored.principal_id != ticket.principal_id
            || stored.session_id != ticket.session_id
            || stored.scope.tenant_id != auth.tenant_id
            || stored.scope.organization_id != organization_id
            || stored.actor_kind != auth.actor_kind
            || stored.actor_id != auth.actor_id
            || stored.principal_kind != auth.actor_kind
            || stored.principal_id != auth.user_id
            || auth
                .session_id
                .as_deref()
                .is_none_or(|session_id| stored.session_id != session_id)
            || stored.knowledge_space_id != ticket.knowledge_space_id
            || stored.knowledge_space_uuid != ticket.knowledge_space_uuid
            || stored.knowledgebase_binding_id != ticket.knowledgebase_binding_id
            || stored.knowledgebase_binding_uuid != ticket.knowledgebase_binding_uuid
            || stored.upstream_link_generation != ticket.upstream_link_generation
            || stored.membership_epoch != ticket.membership_epoch
            || stored.consumed_at.is_some()
            || stored.expires_at <= Utc::now()
            || !link_is_current
        {
            return Ok(false);
        }
        stored.consumed_at = Some(Utc::now());
        stored.consumed_by_service = Some(KNOWLEDGEBASE_SERVICE_IDENTITY.to_owned());
        stored.consumed_trace_id = Some(consumed_trace_id.to_owned());
        Ok(true)
    }
}

/// Production PostgreSQL store. It consumes the shared IM process pool and is
/// called only from the existing blocking conversation runtime boundary.
#[derive(Clone)]
struct PostgresGroupKnowledgebaseStore {
    pool: im_adapters_postgres_journal::PostgresJournalPool,
}

impl PostgresGroupKnowledgebaseStore {
    fn from_shared_process_pool() -> Option<Self> {
        sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool().map(|pool| Self {
            pool: im_adapters_postgres_journal::PostgresJournalPool::from_pool(pool),
        })
    }

    fn client(
        &self,
    ) -> Result<
        r2d2::PooledConnection<im_adapters_postgres_journal::PostgresJournalConnectionManager>,
        RuntimeError,
    > {
        self.pool.get().map_err(|_| {
            RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase storage is unavailable".into(),
            ))
        })
    }
}

const SELECT_LINK_SQL: &str = r#"
select id, link_uuid, tenant_id, organization_id, conversation_id,
       knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
       lifecycle_state, provisioning_operation_id, creation_idempotency_key,
       last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
       last_error_code, created_by,
       updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
from im_conversation_knowledge_space_link
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

const INSERT_LINK_SQL: &str = r#"
insert into im_conversation_knowledge_space_link (
    id, link_uuid, tenant_id, organization_id, conversation_id,
    lifecycle_state, creation_idempotency_key, membership_epoch,
    created_by, updated_by, created_at, updated_at, version
) values ($1, $2, $3, $4, $5, 'provisioning', $6, $7, $8, $8, $9, $9, 1)
on conflict (tenant_id, organization_id, conversation_id) do nothing
returning id, link_uuid, tenant_id, organization_id, conversation_id,
          knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
          lifecycle_state, provisioning_operation_id, creation_idempotency_key,
          last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
          last_error_code, created_by,
          updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
"#;

const ACTIVATE_PROVISIONING_LINK_SQL: &str = r#"
update im_conversation_knowledge_space_link
set knowledge_space_id = $4,
    knowledge_space_uuid = $5,
    knowledgebase_binding_id = $6,
    knowledgebase_binding_uuid = $7,
    lifecycle_state = 'active',
    provisioning_operation_id = $8,
    membership_epoch = greatest(membership_epoch, $9),
    last_synchronized_membership_epoch = 0,
    last_source_event_id = $10,
    last_error_code = null,
    last_error_at = null,
    updated_by = $11,
    updated_at = $12,
    version = version + 1
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and lifecycle_state = 'provisioning'
returning id, link_uuid, tenant_id, organization_id, conversation_id,
          knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
          lifecycle_state, provisioning_operation_id, creation_idempotency_key,
          last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
          last_error_code, created_by,
          updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
"#;

const ACTIVATE_ARCHIVED_LINK_SQL: &str = r#"
update im_conversation_knowledge_space_link
set knowledge_space_id = $4,
    knowledge_space_uuid = $5,
    knowledgebase_binding_id = $6,
    knowledgebase_binding_uuid = $7,
    provisioning_operation_id = $8,
    membership_epoch = greatest(membership_epoch, $9),
    last_error_code = null,
    last_error_at = null,
    updated_at = $10,
    version = version + 1
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and lifecycle_state = 'archived'
returning id, link_uuid, tenant_id, organization_id, conversation_id,
          knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
          lifecycle_state, provisioning_operation_id, creation_idempotency_key,
          last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
          last_error_code, created_by,
          updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
"#;

const FAIL_LINK_SQL: &str = r#"
update im_conversation_knowledge_space_link
set lifecycle_state = 'failed',
    last_error_code = $4,
    last_error_at = $5,
    updated_by = $6,
    updated_at = $5,
    version = version + 1
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and lifecycle_state = 'provisioning'
"#;

const BEGIN_RETRY_PROVISIONING_SQL: &str = r#"
update im_conversation_knowledge_space_link
set lifecycle_state = 'provisioning',
    provisioning_operation_id = null,
    last_source_event_id = $4,
    last_error_code = null,
    last_error_at = null,
    updated_by = $5,
    updated_at = $6,
    version = version + 1
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and lifecycle_state = 'failed'
returning id, link_uuid, tenant_id, organization_id, conversation_id,
          knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
          lifecycle_state, provisioning_operation_id, creation_idempotency_key,
          last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
          last_error_code, created_by,
          updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
"#;

const SELECT_LINK_FOR_UPDATE_SQL: &str = r#"
select id, link_uuid, tenant_id, organization_id, conversation_id,
       knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
       lifecycle_state, provisioning_operation_id, creation_idempotency_key,
       last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
       last_error_code, created_by,
       updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
from im_conversation_knowledge_space_link
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
for update
"#;

const UPDATE_MEMBERSHIP_SYNC_LINK_SQL: &str = r#"
update im_conversation_knowledge_space_link
set membership_epoch = $4,
    last_source_event_id = $5,
    updated_by = $6,
    updated_at = $7,
    version = version + 1
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and lifecycle_state = 'active'
returning id, link_uuid, tenant_id, organization_id, conversation_id,
          knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
          lifecycle_state, provisioning_operation_id, creation_idempotency_key,
          last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
          last_error_code, created_by,
          updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
"#;

const MARK_MEMBERSHIP_SYNCHRONIZED_SQL: &str = r#"
update im_conversation_knowledge_space_link
set last_synchronized_membership_epoch = $4,
    updated_by = $6,
    updated_at = $7
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and lifecycle_state = 'active'
  and membership_epoch = $4
  and version = $5
  and last_synchronized_membership_epoch < $4
"#;

const ARCHIVE_LINK_SQL: &str = r#"
update im_conversation_knowledge_space_link
set lifecycle_state = 'archived',
    archived_at = $4,
    last_source_event_id = $5,
    updated_by = $6,
    updated_at = $4,
    version = version + 1
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and lifecycle_state in ('active', 'provisioning', 'failed')
returning id, link_uuid, tenant_id, organization_id, conversation_id,
          knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
          lifecycle_state, provisioning_operation_id, creation_idempotency_key,
          last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
          last_error_code, created_by,
          updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
"#;

const SELECT_NEXT_RECONCILIATION_SCOPE_SQL: &str = r#"
/* sdkwork:cross-organization-operation=knowledgebase-reconciliation-scope-discovery */
select tenant_id, organization_id
from im_conversation_knowledge_space_link
where lifecycle_state in ('provisioning', 'active', 'archived')
  and ($1::text is null or (tenant_id, organization_id) > ($1, $2))
group by tenant_id, organization_id
order by tenant_id asc, organization_id asc
limit 1
"#;

const SELECT_RECONCILIATION_LINKS_SQL: &str = r#"
select id, link_uuid, tenant_id, organization_id, conversation_id,
       knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
       lifecycle_state, provisioning_operation_id, creation_idempotency_key,
       last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
       last_error_code, created_by,
       updated_by, created_at, updated_at, version, knowledgebase_binding_uuid
from im_conversation_knowledge_space_link
where tenant_id = $1
  and organization_id = $2
  and lifecycle_state in ('provisioning', 'active', 'archived')
  and ($3::bigint is null or id > $3)
order by id asc
limit $4
"#;

const INSERT_GROUP_KNOWLEDGEBASE_OUTBOX_SQL: &str = r#"
insert into im_outbox_events (
    tenant_id, organization_id, outbox_id, aggregate_type, aggregate_id,
    event_id, event_type, payload_json, payload_hash, publish_status,
    attempt_count, available_at, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, 'pending', 0, $10, $10, $10)
on conflict (tenant_id, organization_id, event_id) do update
set publish_status = 'pending',
    attempt_count = 0,
    available_at = excluded.available_at,
    published_at = null,
    updated_at = excluded.updated_at
where im_outbox_events.publish_status = 'failed'
  and im_outbox_events.aggregate_type = excluded.aggregate_type
  and im_outbox_events.payload_hash = excluded.payload_hash
returning aggregate_type, payload_hash, publish_status
"#;

const SELECT_GROUP_KNOWLEDGEBASE_OUTBOX_INTEGRITY_SQL: &str = r#"
select aggregate_type, payload_hash, publish_status
from im_outbox_events
where tenant_id = $1 and organization_id = $2 and event_id = $3
"#;

const INSERT_TICKET_SQL: &str = r#"
insert into im_group_knowledge_launch_tickets (
    id, ticket_hash, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
    knowledgebase_binding_uuid, upstream_link_generation, membership_epoch,
    actor_kind, actor_id, principal_kind, principal_id, session_id,
    issuing_app_id, issued_by, idempotency_key_hash, request_fingerprint_hash,
    ticket_ciphertext, expires_at, created_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
    $15, $16, $17, $18, $19, $20, $21, $22, $23
)
on conflict (
    tenant_id, organization_id, conversation_id, actor_kind, actor_id,
    principal_kind, principal_id, session_id, idempotency_key_hash
)
do nothing
returning id, ticket_hash, tenant_id, organization_id, conversation_id,
          knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
          knowledgebase_binding_uuid, upstream_link_generation, membership_epoch,
          actor_kind, actor_id, principal_kind, principal_id, session_id,
          issuing_app_id, issued_by, idempotency_key_hash, request_fingerprint_hash,
          ticket_ciphertext, expires_at, consumed_at, consumed_by_service, consumed_trace_id
"#;

const SELECT_TICKET_FOR_ACTOR_SQL: &str = r#"
select id, ticket_hash, tenant_id, organization_id, conversation_id,
       knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
       knowledgebase_binding_uuid, upstream_link_generation, membership_epoch,
       actor_kind, actor_id, principal_kind, principal_id, session_id,
       issuing_app_id, issued_by, idempotency_key_hash, request_fingerprint_hash,
       ticket_ciphertext, expires_at, consumed_at, consumed_by_service, consumed_trace_id
from im_group_knowledge_launch_tickets
where ticket_hash = $1
  and tenant_id = $2
  and organization_id = $3
  and actor_kind = $4
  and actor_id = $5
  and principal_kind = $6
  and principal_id = $7
  and session_id = $8
  and consumed_at is null
  and expires_at > $9
"#;

const SELECT_TICKET_BY_IDEMPOTENCY_SQL: &str = r#"
select id, ticket_hash, tenant_id, organization_id, conversation_id,
       knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
       knowledgebase_binding_uuid, upstream_link_generation, membership_epoch,
       actor_kind, actor_id, principal_kind, principal_id, session_id,
       issuing_app_id, issued_by, idempotency_key_hash, request_fingerprint_hash,
       ticket_ciphertext, expires_at, consumed_at, consumed_by_service, consumed_trace_id
from im_group_knowledge_launch_tickets
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and actor_kind = $4
  and actor_id = $5
  and principal_kind = $6
  and principal_id = $7
  and session_id = $8
  and idempotency_key_hash = $9
"#;

const CONSUME_TICKET_SQL: &str = r#"
update im_group_knowledge_launch_tickets as ticket
set consumed_at = $2,
    consumed_by_service = $3,
    consumed_trace_id = $4
from im_conversation_knowledge_space_link as link
where ticket.ticket_hash = $1
  and ticket.tenant_id = $5
  and ticket.organization_id = $6
  and ticket.actor_kind = $7
  and ticket.actor_id = $8
  and ticket.principal_kind = $9
  and ticket.principal_id = $10
  and ticket.session_id = $11
  and ticket.knowledgebase_binding_id = $12
  and ticket.knowledgebase_binding_uuid = $13
  and ticket.upstream_link_generation = $14
  and ticket.membership_epoch = $15
  and ticket.conversation_id = $16
  and ticket.knowledge_space_id = $17
  and ticket.knowledge_space_uuid = $18
  and ticket.consumed_at is null
  and ticket.expires_at > $2
  and link.tenant_id = ticket.tenant_id
  and link.organization_id = ticket.organization_id
  and link.conversation_id = ticket.conversation_id
  and link.lifecycle_state = 'active'
  and link.knowledge_space_id = ticket.knowledge_space_id
  and link.knowledge_space_uuid = ticket.knowledge_space_uuid
  and link.knowledgebase_binding_id = ticket.knowledgebase_binding_id
  and link.knowledgebase_binding_uuid = ticket.knowledgebase_binding_uuid
  and link.version = ticket.upstream_link_generation
  and link.membership_epoch = ticket.membership_epoch
  and link.last_synchronized_membership_epoch = ticket.membership_epoch
"#;

fn postgres_store_error(operation: &str) -> RuntimeError {
    RuntimeError::Contract(ContractError::Unavailable(format!(
        "group knowledgebase storage {operation} failed"
    )))
}

fn group_knowledgebase_outbox_event_id(
    operation: &GroupKnowledgebaseOutboxOperation,
    scope: &GroupKnowledgebaseScope,
    source_event_id: &str,
) -> String {
    let operation = match operation {
        GroupKnowledgebaseOutboxOperation::SynchronizeMembers => "members",
        GroupKnowledgebaseOutboxOperation::Archive => "archive",
    };
    let digest = sha256_hash(
        format!(
            "{operation}:{}:{}:{}:{source_event_id}",
            scope.tenant_id, scope.organization_id, scope.conversation_id
        )
        .as_bytes(),
    );
    format!("im.group-knowledgebase.{operation}.{digest}")
}

fn active_group_knowledgebase_reference(
    link: &GroupKnowledgebaseLink,
) -> Result<GroupKnowledgebaseTargetFence, RuntimeError> {
    if !link.lifecycle_state.is_active() {
        return Err(RuntimeError::Contract(ContractError::Unavailable(
            "active group knowledgebase link is missing its space reference".into(),
        )));
    }
    group_knowledgebase_target_fence(link)
}

fn group_knowledgebase_target_fence(
    link: &GroupKnowledgebaseLink,
) -> Result<GroupKnowledgebaseTargetFence, RuntimeError> {
    GroupKnowledgebaseTargetFence::from_link(link)
}

fn group_knowledgebase_membership_outbox_payload(
    link: &GroupKnowledgebaseLink,
    source_event_id: String,
    members: Vec<GroupKnowledgebaseMembership>,
) -> Result<GroupKnowledgebaseOutboxPayload, RuntimeError> {
    let target = active_group_knowledgebase_reference(link)?;
    Ok(GroupKnowledgebaseOutboxPayload {
        operation: GroupKnowledgebaseOutboxOperation::SynchronizeMembers,
        source_event_id,
        scope: link.scope.clone(),
        knowledge_space_id: target.knowledge_space_id,
        knowledge_space_uuid: target.knowledge_space_uuid,
        knowledgebase_binding_id: target.knowledgebase_binding_id,
        knowledgebase_binding_uuid: target.knowledgebase_binding_uuid,
        upstream_link_generation: link.version,
        membership_epoch: link.membership_epoch,
        members,
        archived_by: None,
    })
}

fn group_knowledgebase_archive_outbox_payload(
    link: &GroupKnowledgebaseLink,
    source_event_id: String,
    archived_by: String,
) -> Result<GroupKnowledgebaseOutboxPayload, RuntimeError> {
    let target = group_knowledgebase_target_fence(link)?;
    Ok(GroupKnowledgebaseOutboxPayload {
        operation: GroupKnowledgebaseOutboxOperation::Archive,
        source_event_id,
        scope: link.scope.clone(),
        knowledge_space_id: target.knowledge_space_id,
        knowledge_space_uuid: target.knowledge_space_uuid,
        knowledgebase_binding_id: target.knowledgebase_binding_id,
        knowledgebase_binding_uuid: target.knowledgebase_binding_uuid,
        upstream_link_generation: link.version,
        membership_epoch: link.membership_epoch,
        members: Vec::new(),
        archived_by: Some(archived_by),
    })
}

fn serialize_group_knowledgebase_outbox_payload(
    payload: &GroupKnowledgebaseOutboxPayload,
) -> Result<(String, String), RuntimeError> {
    let payload_json = serde_json::to_string(payload).map_err(|_| {
        RuntimeError::Contract(ContractError::Unavailable(
            "group knowledgebase outbox payload serialization failed".into(),
        ))
    })?;
    let payload_hash = sha256_hash(payload_json.as_bytes());
    Ok((payload_json, payload_hash))
}

fn persist_group_knowledgebase_outbox<C>(
    client: &mut C,
    payload: &GroupKnowledgebaseOutboxPayload,
    outbox_id: &str,
    occurred_at: &DateTime<Utc>,
) -> Result<(), RuntimeError>
where
    C: postgres::GenericClient,
{
    let (payload_json, payload_hash) = serialize_group_knowledgebase_outbox_payload(payload)?;
    let event_id = group_knowledgebase_outbox_event_id(
        &payload.operation,
        &payload.scope,
        payload.source_event_id.as_str(),
    );
    let event_type = match payload.operation {
        GroupKnowledgebaseOutboxOperation::SynchronizeMembers => {
            GROUP_KNOWLEDGEBASE_MEMBERSHIP_SYNC_EVENT_TYPE
        }
        GroupKnowledgebaseOutboxOperation::Archive => GROUP_KNOWLEDGEBASE_ARCHIVE_EVENT_TYPE,
    };
    let row = client
        .query_opt(
            INSERT_GROUP_KNOWLEDGEBASE_OUTBOX_SQL,
            &[
                &payload.scope.tenant_id,
                &payload.scope.organization_id,
                &outbox_id,
                &GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE,
                &payload.scope.conversation_id,
                &event_id,
                &event_type,
                &payload_json,
                &payload_hash,
                occurred_at,
            ],
        )
        .map_err(|_| postgres_store_error("outbox enqueue"))?;
    let row = match row {
        Some(row) => row,
        None => client
            .query_one(
                SELECT_GROUP_KNOWLEDGEBASE_OUTBOX_INTEGRITY_SQL,
                &[
                    &payload.scope.tenant_id,
                    &payload.scope.organization_id,
                    &event_id,
                ],
            )
            .map_err(|_| postgres_store_error("outbox conflict validation"))?,
    };
    let stored_aggregate_type: String = row.get(0);
    let stored_payload_hash: String = row.get(1);
    let stored_publish_status: String = row.get(2);
    if stored_aggregate_type != GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE
        || stored_payload_hash != payload_hash
        || stored_publish_status == "failed"
    {
        return Err(RuntimeError::Conflict(
            "group knowledgebase deterministic outbox event conflicts with stored integrity state"
                .into(),
        ));
    }
    Ok(())
}

fn row_to_link(row: &postgres::Row) -> Result<GroupKnowledgebaseLink, RuntimeError> {
    let lifecycle_state: String = row.get(8);
    let membership_epoch = group_knowledgebase_db_i64_to_u64(row.get(12), "membership epoch")?;
    let last_synchronized_membership_epoch =
        group_knowledgebase_db_i64_to_u64(row.get(13), "last synchronized membership epoch")?;
    let version = group_knowledgebase_db_i64_to_u64(
        validate_group_knowledgebase_persisted_positive_i64(row.get(19), "link generation")?,
        "link generation",
    )?;
    Ok(GroupKnowledgebaseLink {
        id: row.get(0),
        link_uuid: row.get(1),
        scope: GroupKnowledgebaseScope {
            tenant_id: row.get(2),
            organization_id: row.get(3),
            conversation_id: row.get(4),
        },
        knowledge_space_id: row.get(5),
        knowledge_space_uuid: row.get(6),
        knowledgebase_binding_id: row.get(7),
        knowledgebase_binding_uuid: row.get(20),
        lifecycle_state: GroupKnowledgebaseLifecycleState::from_db(lifecycle_state.as_str())?,
        provisioning_operation_id: row.get(9),
        creation_idempotency_key: row.get(10),
        last_source_event_id: row.get(11),
        membership_epoch,
        last_synchronized_membership_epoch,
        last_error_code: row.get(14),
        created_by: row.get(15),
        updated_by: row.get(16),
        created_at: row.get(17),
        updated_at: row.get(18),
        version,
    })
}

fn row_to_ticket(row: &postgres::Row) -> Result<GroupKnowledgebaseLaunchTicket, RuntimeError> {
    let upstream_link_generation = group_knowledgebase_db_i64_to_u64(
        validate_group_knowledgebase_persisted_positive_i64(
            row.get(9),
            "upstream link generation",
        )?,
        "upstream link generation",
    )?;
    let membership_epoch = group_knowledgebase_db_i64_to_u64(row.get(10), "membership epoch")?;
    Ok(GroupKnowledgebaseLaunchTicket {
        id: row.get(0),
        ticket_hash: row.get(1),
        scope: GroupKnowledgebaseScope {
            tenant_id: row.get(2),
            organization_id: row.get(3),
            conversation_id: row.get(4),
        },
        knowledge_space_id: row.get(5),
        knowledge_space_uuid: row.get(6),
        knowledgebase_binding_id: row.get(7),
        knowledgebase_binding_uuid: row.get(8),
        upstream_link_generation,
        membership_epoch,
        actor_kind: row.get(11),
        actor_id: row.get(12),
        principal_kind: row.get(13),
        principal_id: row.get(14),
        session_id: row.get(15),
        issuing_app_id: row.get(16),
        issued_by: row.get(17),
        idempotency_key_hash: row.get(18),
        request_fingerprint_hash: row.get(19),
        ticket_ciphertext: row.get(20),
        expires_at: row.get(21),
        consumed_at: row.get(22),
        consumed_by_service: row.get(23),
        consumed_trace_id: row.get(24),
    })
}

impl GroupKnowledgebaseStore for PostgresGroupKnowledgebaseStore {
    fn get_link(
        &self,
        scope: &GroupKnowledgebaseScope,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        scope.validate()?;
        let mut client = self.client()?;
        client
            .query_opt(
                SELECT_LINK_SQL,
                &[
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.conversation_id,
                ],
            )
            .map_err(|_| postgres_store_error("link lookup"))?
            .map(|row| row_to_link(&row))
            .transpose()
    }

    fn reserve_link(
        &self,
        candidate: GroupKnowledgebaseLink,
    ) -> Result<GroupKnowledgebaseLinkReservation, RuntimeError> {
        candidate.validate_for_persistence()?;
        let membership_epoch =
            group_knowledgebase_u64_to_db_i64(candidate.membership_epoch, "membership epoch")?;
        let mut client = self.client()?;
        let row = client
            .query_opt(
                INSERT_LINK_SQL,
                &[
                    &candidate.id,
                    &candidate.link_uuid,
                    &candidate.scope.tenant_id,
                    &candidate.scope.organization_id,
                    &candidate.scope.conversation_id,
                    &candidate.creation_idempotency_key,
                    &membership_epoch,
                    &candidate.created_by,
                    &candidate.created_at,
                ],
            )
            .map_err(|_| postgres_store_error("link reservation"))?;
        if let Some(row) = row {
            return Ok(GroupKnowledgebaseLinkReservation {
                link: row_to_link(&row)?,
                newly_reserved: true,
            });
        }
        self.get_link(&candidate.scope)?
            .map(|link| GroupKnowledgebaseLinkReservation {
                link,
                newly_reserved: false,
            })
            .ok_or_else(|| {
                RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase link reservation did not persist".into(),
                ))
            })
    }

    fn begin_retry_provisioning(
        &self,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        source_event_id: &str,
    ) -> Result<GroupKnowledgebaseLink, RuntimeError> {
        scope.validate()?;
        let mut client = self.client()?;
        let now = Utc::now();
        if let Some(row) = client
            .query_opt(
                BEGIN_RETRY_PROVISIONING_SQL,
                &[
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.conversation_id,
                    &source_event_id,
                    &actor_id,
                    &now,
                ],
            )
            .map_err(|_| postgres_store_error("link provisioning retry"))?
        {
            return row_to_link(&row);
        }

        let existing = self.get_link(scope)?.ok_or_else(|| {
            RuntimeError::ConversationBindingNotFound(scope.conversation_id.clone())
        })?;
        match existing.lifecycle_state {
            GroupKnowledgebaseLifecycleState::Provisioning => Ok(existing),
            GroupKnowledgebaseLifecycleState::Archived
            | GroupKnowledgebaseLifecycleState::Deleted => Err(RuntimeError::Conflict(
                "group knowledgebase lifecycle does not permit automatic reprovisioning".into(),
            )),
            GroupKnowledgebaseLifecycleState::Active | GroupKnowledgebaseLifecycleState::Absent => {
                Err(RuntimeError::Conflict(
                    "group knowledgebase is not eligible for provisioning retry".into(),
                ))
            }
            GroupKnowledgebaseLifecycleState::Failed => {
                Err(RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase provisioning retry did not advance the failed link".into(),
                )))
            }
        }
    }

    fn activate_link(
        &self,
        scope: &GroupKnowledgebaseScope,
        ensured: EnsuredGroupKnowledgebase,
        membership_epoch: u64,
        actor_id: &str,
        source_event_id: &str,
        archive_outbox_id: &str,
    ) -> Result<GroupKnowledgebaseLink, RuntimeError> {
        scope.validate()?;
        ensured.validate()?;
        let membership_epoch =
            group_knowledgebase_u64_to_db_i64(membership_epoch, "membership epoch")?;
        let mut client = self.client()?;
        let now = Utc::now();
        let mut transaction = client
            .transaction()
            .map_err(|_| postgres_store_error("link activation transaction begin"))?;
        let existing = transaction
            .query_opt(
                SELECT_LINK_FOR_UPDATE_SQL,
                &[
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.conversation_id,
                ],
            )
            .map_err(|_| postgres_store_error("link activation lock"))?
            .map(|row| row_to_link(&row))
            .transpose()?
            .ok_or_else(|| {
                RuntimeError::ConversationBindingNotFound(scope.conversation_id.clone())
            })?;

        let result = match existing.lifecycle_state {
            GroupKnowledgebaseLifecycleState::Provisioning => {
                let row = transaction
                    .query_one(
                        ACTIVATE_PROVISIONING_LINK_SQL,
                        &[
                            &scope.tenant_id,
                            &scope.organization_id,
                            &scope.conversation_id,
                            &ensured.knowledge_space_id,
                            &ensured.knowledge_space_uuid,
                            &ensured.knowledgebase_binding_id,
                            &ensured.knowledgebase_binding_uuid,
                            &ensured.provisioning_operation_id,
                            &membership_epoch,
                            &source_event_id,
                            &actor_id,
                            &now,
                        ],
                    )
                    .map_err(|_| postgres_store_error("link activation"))?;
                row_to_link(&row)?
            }
            GroupKnowledgebaseLifecycleState::Archived => {
                let link = if let Some(existing_space_id) = existing.knowledge_space_id {
                    if existing_space_id != ensured.knowledge_space_id
                        || existing.knowledge_space_uuid.as_deref()
                            != Some(ensured.knowledge_space_uuid.as_str())
                        || existing.knowledgebase_binding_id
                            != Some(ensured.knowledgebase_binding_id)
                        || existing.knowledgebase_binding_uuid.as_deref()
                            != Some(ensured.knowledgebase_binding_uuid.as_str())
                    {
                        return Err(RuntimeError::Conflict(
                            "archived group knowledgebase link conflicts with a provisioning result"
                                .into(),
                        ));
                    }
                    existing
                } else {
                    let row = transaction
                        .query_one(
                            ACTIVATE_ARCHIVED_LINK_SQL,
                            &[
                                &scope.tenant_id,
                                &scope.organization_id,
                                &scope.conversation_id,
                                &ensured.knowledge_space_id,
                                &ensured.knowledge_space_uuid,
                                &ensured.knowledgebase_binding_id,
                                &ensured.knowledgebase_binding_uuid,
                                &ensured.provisioning_operation_id,
                                &membership_epoch,
                                &now,
                            ],
                        )
                        .map_err(|_| postgres_store_error("archived link reference persistence"))?;
                    row_to_link(&row)?
                };
                let archive_source_event_id =
                    link.last_source_event_id.clone().ok_or_else(|| {
                        RuntimeError::Conflict(
                        "archived group knowledgebase link is missing its durable archive source"
                            .into(),
                    )
                    })?;
                let payload = group_knowledgebase_archive_outbox_payload(
                    &link,
                    archive_source_event_id,
                    link.updated_by.clone(),
                )?;
                persist_group_knowledgebase_outbox(
                    &mut transaction,
                    &payload,
                    archive_outbox_id,
                    &now,
                )?;
                link
            }
            GroupKnowledgebaseLifecycleState::Active => {
                if existing.knowledge_space_id == Some(ensured.knowledge_space_id)
                    && existing.knowledge_space_uuid.as_deref()
                        == Some(ensured.knowledge_space_uuid.as_str())
                    && existing.knowledgebase_binding_id == Some(ensured.knowledgebase_binding_id)
                    && existing.knowledgebase_binding_uuid.as_deref()
                        == Some(ensured.knowledgebase_binding_uuid.as_str())
                {
                    existing
                } else {
                    return Err(RuntimeError::Conflict(
                        "active group knowledgebase link conflicts with a provisioning result"
                            .into(),
                    ));
                }
            }
            GroupKnowledgebaseLifecycleState::Failed
            | GroupKnowledgebaseLifecycleState::Deleted
            | GroupKnowledgebaseLifecycleState::Absent => {
                return Err(RuntimeError::Conflict(
                    "group knowledgebase lifecycle does not permit provisioning completion".into(),
                ));
            }
        };
        transaction
            .commit()
            .map_err(|_| postgres_store_error("link activation transaction commit"))?;
        Ok(result)
    }

    fn fail_link(
        &self,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        error_code: &str,
    ) -> Result<(), RuntimeError> {
        scope.validate()?;
        let mut client = self.client()?;
        let now = Utc::now();
        client
            .execute(
                FAIL_LINK_SQL,
                &[
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.conversation_id,
                    &error_code,
                    &now,
                    &actor_id,
                ],
            )
            .map_err(|_| postgres_store_error("link failure update"))?;
        Ok(())
    }

    fn enqueue_membership_synchronization(
        &self,
        request: GroupKnowledgebaseMembershipSyncEnqueue,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        request.scope.validate()?;
        let target_membership_epoch =
            group_knowledgebase_u64_to_db_i64(request.target_membership_epoch, "membership epoch")?;
        let mut client = self.client()?;
        let mut transaction = client
            .transaction()
            .map_err(|_| postgres_store_error("membership synchronization transaction begin"))?;
        let existing = transaction
            .query_opt(
                SELECT_LINK_FOR_UPDATE_SQL,
                &[
                    &request.scope.tenant_id,
                    &request.scope.organization_id,
                    &request.scope.conversation_id,
                ],
            )
            .map_err(|_| postgres_store_error("membership synchronization link lock"))?
            .map(|row| row_to_link(&row))
            .transpose()?;
        let Some(existing) = existing else {
            transaction
                .commit()
                .map_err(|_| postgres_store_error("membership synchronization empty commit"))?;
            return Ok(None);
        };
        if !existing.lifecycle_state.is_active()
            || request.target_membership_epoch < existing.membership_epoch
            || (request.target_membership_epoch == existing.membership_epoch
                && existing.last_synchronized_membership_epoch >= request.target_membership_epoch)
        {
            transaction.commit().map_err(|_| {
                postgres_store_error("membership synchronization idempotent commit")
            })?;
            return Ok(Some(existing));
        }

        let link = if existing.membership_epoch == request.target_membership_epoch
            && existing.last_source_event_id.as_deref() == Some(request.source_event_id.as_str())
        {
            existing
        } else {
            let updated = transaction
                .query_one(
                    UPDATE_MEMBERSHIP_SYNC_LINK_SQL,
                    &[
                        &request.scope.tenant_id,
                        &request.scope.organization_id,
                        &request.scope.conversation_id,
                        &target_membership_epoch,
                        &request.source_event_id,
                        &request.actor_id,
                        &request.occurred_at,
                    ],
                )
                .map_err(|_| postgres_store_error("membership synchronization link update"))?;
            row_to_link(&updated)?
        };
        let payload = group_knowledgebase_membership_outbox_payload(
            &link,
            request.source_event_id,
            request.members,
        )?;
        persist_group_knowledgebase_outbox(
            &mut transaction,
            &payload,
            request.outbox_id.as_str(),
            &request.occurred_at,
        )?;
        transaction
            .commit()
            .map_err(|_| postgres_store_error("membership synchronization transaction commit"))?;
        Ok(Some(link))
    }

    fn mark_membership_synchronized(
        &self,
        scope: &GroupKnowledgebaseScope,
        membership_epoch: u64,
        upstream_link_generation: u64,
        actor_id: &str,
    ) -> Result<bool, RuntimeError> {
        scope.validate()?;
        let membership_epoch =
            group_knowledgebase_u64_to_db_i64(membership_epoch, "membership epoch")?;
        validate_group_knowledgebase_nonzero_u64(
            upstream_link_generation,
            "upstream link generation",
        )?;
        let upstream_link_generation = group_knowledgebase_u64_to_db_i64(
            upstream_link_generation,
            "upstream link generation",
        )?;
        let mut client = self.client()?;
        let now = Utc::now();
        let affected = client
            .execute(
                MARK_MEMBERSHIP_SYNCHRONIZED_SQL,
                &[
                    &scope.tenant_id,
                    &scope.organization_id,
                    &scope.conversation_id,
                    &membership_epoch,
                    &upstream_link_generation,
                    &actor_id,
                    &now,
                ],
            )
            .map_err(|_| postgres_store_error("membership synchronization acknowledgement"))?;
        Ok(affected == 1)
    }

    fn archive_link_and_enqueue(
        &self,
        request: GroupKnowledgebaseArchiveEnqueue,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        request.scope.validate()?;
        let mut client = self.client()?;
        let mut transaction = client
            .transaction()
            .map_err(|_| postgres_store_error("group knowledgebase archive transaction begin"))?;
        let existing = transaction
            .query_opt(
                SELECT_LINK_FOR_UPDATE_SQL,
                &[
                    &request.scope.tenant_id,
                    &request.scope.organization_id,
                    &request.scope.conversation_id,
                ],
            )
            .map_err(|_| postgres_store_error("group knowledgebase archive link lock"))?
            .map(|row| row_to_link(&row))
            .transpose()?;
        let Some(existing) = existing else {
            transaction
                .commit()
                .map_err(|_| postgres_store_error("group knowledgebase archive empty commit"))?;
            return Ok(None);
        };
        let link = if matches!(
            existing.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Archived
        ) {
            if existing.last_source_event_id.as_deref() != Some(request.source_event_id.as_str()) {
                return Err(RuntimeError::Conflict(
                    "archived group knowledgebase link conflicts with the durable archive source"
                        .into(),
                ));
            }
            existing
        } else {
            if !matches!(
                existing.lifecycle_state,
                GroupKnowledgebaseLifecycleState::Active
                    | GroupKnowledgebaseLifecycleState::Provisioning
                    | GroupKnowledgebaseLifecycleState::Failed
            ) {
                return Err(RuntimeError::Conflict(
                    "group knowledgebase lifecycle does not permit archival".into(),
                ));
            }

            let updated = transaction
                .query_one(
                    ARCHIVE_LINK_SQL,
                    &[
                        &request.scope.tenant_id,
                        &request.scope.organization_id,
                        &request.scope.conversation_id,
                        &request.occurred_at,
                        &request.source_event_id,
                        &request.actor_id,
                    ],
                )
                .map_err(|_| postgres_store_error("group knowledgebase archive link update"))?;
            row_to_link(&updated)?
        };
        if link.knowledge_space_id.is_none() {
            transaction.commit().map_err(|_| {
                postgres_store_error("group knowledgebase provisional archive transaction commit")
            })?;
            return Ok(Some(link));
        }
        let payload = group_knowledgebase_archive_outbox_payload(
            &link,
            request.source_event_id,
            request.actor_id,
        )?;
        persist_group_knowledgebase_outbox(
            &mut transaction,
            &payload,
            request.outbox_id.as_str(),
            &request.occurred_at,
        )?;
        transaction
            .commit()
            .map_err(|_| postgres_store_error("group knowledgebase archive transaction commit"))?;
        Ok(Some(link))
    }

    fn next_reconciliation_scope(
        &self,
        request: GroupKnowledgebaseReconciliationScopeRequest<'_>,
    ) -> Result<Option<GroupKnowledgebaseReconciliationScope>, RuntimeError> {
        let context = request.context;
        let after = request.after;
        let mut client = self.client()?;
        let after_tenant_id = after.map(|scope| scope.tenant_id.as_str());
        let after_organization_id = after.map(|scope| scope.organization_id.as_str());
        let result = client
            .query_opt(
                SELECT_NEXT_RECONCILIATION_SCOPE_SQL,
                &[&after_tenant_id, &after_organization_id],
            )
            .map_err(|_| postgres_store_error("reconciliation scope lookup"))?
            .map(|row| {
                let scope = GroupKnowledgebaseReconciliationScope {
                    tenant_id: row.get(0),
                    organization_id: row.get(1),
                };
                validate_group_knowledgebase_tenant_id(scope.tenant_id.as_str())?;
                validate_group_knowledgebase_organization_id(scope.organization_id.as_str())?;
                Ok(scope)
            })
            .transpose();
        match &result {
            Ok(scope) => tracing::info!(
                target: "sdkwork.im.security",
                event = "im.knowledgebase_reconciliation_scope.operation_completed",
                actor_kind = context.actor_kind().as_str(),
                actor_id = context.actor_id(),
                trace_id = context.trace_id(),
                outcome = "succeeded",
                scope_found = scope.is_some(),
                "cross-organization knowledgebase reconciliation scope discovery completed"
            ),
            Err(error) => tracing::warn!(
                target: "sdkwork.im.security",
                event = "im.knowledgebase_reconciliation_scope.operation_completed",
                actor_kind = context.actor_kind().as_str(),
                actor_id = context.actor_id(),
                trace_id = context.trace_id(),
                outcome = "failed",
                error = ?error,
                "cross-organization knowledgebase reconciliation scope discovery failed"
            ),
        }
        result
    }

    fn list_reconciliation_links(
        &self,
        scope: &GroupKnowledgebaseReconciliationScope,
        after_link_id: Option<i64>,
        limit: usize,
    ) -> Result<GroupKnowledgebaseReconciliationLinkPage, RuntimeError> {
        validate_group_knowledgebase_tenant_id(scope.tenant_id.as_str())?;
        validate_group_knowledgebase_organization_id(scope.organization_id.as_str())?;
        let page_size = limit.max(1);
        let query_limit = i64::try_from(page_size.saturating_add(1)).map_err(|_| {
            RuntimeError::InvalidInput(
                "group knowledgebase reconciliation page size is invalid".into(),
            )
        })?;
        let mut client = self.client()?;
        let rows = client
            .query(
                SELECT_RECONCILIATION_LINKS_SQL,
                &[
                    &scope.tenant_id,
                    &scope.organization_id,
                    &after_link_id,
                    &query_limit,
                ],
            )
            .map_err(|_| postgres_store_error("reconciliation link page lookup"))?;
        let mut links = rows
            .iter()
            .map(row_to_link)
            .collect::<Result<Vec<_>, _>>()?;
        let has_more = links.len() > page_size;
        links.truncate(page_size);
        Ok(GroupKnowledgebaseReconciliationLinkPage {
            next_link_id: has_more.then(|| links.last().map(|link| link.id)).flatten(),
            links,
        })
    }

    fn reserve_ticket(
        &self,
        ticket: GroupKnowledgebaseLaunchTicket,
    ) -> Result<GroupKnowledgebaseLaunchTicketReservation, RuntimeError> {
        ticket.scope.validate()?;
        GroupKnowledgebaseTargetFence {
            knowledge_space_id: ticket.knowledge_space_id,
            knowledge_space_uuid: ticket.knowledge_space_uuid.clone(),
            knowledgebase_binding_id: ticket.knowledgebase_binding_id,
            knowledgebase_binding_uuid: ticket.knowledgebase_binding_uuid.clone(),
        }
        .validate()?;
        validate_group_knowledgebase_nonzero_u64(
            ticket.upstream_link_generation,
            "upstream link generation",
        )?;
        let upstream_link_generation = group_knowledgebase_u64_to_db_i64(
            ticket.upstream_link_generation,
            "upstream link generation",
        )?;
        let membership_epoch =
            group_knowledgebase_u64_to_db_i64(ticket.membership_epoch, "membership epoch")?;
        let mut client = self.client()?;
        let now = Utc::now();
        let inserted = client
            .query_opt(
                INSERT_TICKET_SQL,
                &[
                    &ticket.id,
                    &ticket.ticket_hash,
                    &ticket.scope.tenant_id,
                    &ticket.scope.organization_id,
                    &ticket.scope.conversation_id,
                    &ticket.knowledge_space_id,
                    &ticket.knowledge_space_uuid,
                    &ticket.knowledgebase_binding_id,
                    &ticket.knowledgebase_binding_uuid,
                    &upstream_link_generation,
                    &membership_epoch,
                    &ticket.actor_kind,
                    &ticket.actor_id,
                    &ticket.principal_kind,
                    &ticket.principal_id,
                    &ticket.session_id,
                    &ticket.issuing_app_id,
                    &ticket.issued_by,
                    &ticket.idempotency_key_hash,
                    &ticket.request_fingerprint_hash,
                    &ticket.ticket_ciphertext,
                    &ticket.expires_at,
                    &now,
                ],
            )
            .map_err(|_| postgres_store_error("ticket reservation"))?;
        if inserted.is_some() {
            return Ok(GroupKnowledgebaseLaunchTicketReservation::Created);
        }
        let existing = client
            .query_opt(
                SELECT_TICKET_BY_IDEMPOTENCY_SQL,
                &[
                    &ticket.scope.tenant_id,
                    &ticket.scope.organization_id,
                    &ticket.scope.conversation_id,
                    &ticket.actor_kind,
                    &ticket.actor_id,
                    &ticket.principal_kind,
                    &ticket.principal_id,
                    &ticket.session_id,
                    &ticket.idempotency_key_hash,
                ],
            )
            .map_err(|_| postgres_store_error("ticket idempotency lookup"))?
            .map(|row| row_to_ticket(&row))
            .transpose()?
            .ok_or_else(|| {
                RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase ticket reservation did not persist".into(),
                ))
            })?;
        Ok(GroupKnowledgebaseLaunchTicketReservation::Existing(
            Box::new(existing),
        ))
    }

    fn find_unconsumed_ticket_for_consumer(
        &self,
        ticket_hash: &str,
        auth: &AppContext,
    ) -> Result<Option<GroupKnowledgebaseLaunchTicket>, RuntimeError> {
        let organization_id = resolve_group_knowledgebase_organization_id(auth)?;
        let session_id = require_group_knowledgebase_ticket_session(auth)?;
        let mut client = self.client()?;
        client
            .query_opt(
                SELECT_TICKET_FOR_ACTOR_SQL,
                &[
                    &ticket_hash,
                    &auth.tenant_id,
                    &organization_id,
                    &auth.actor_kind,
                    &auth.actor_id,
                    &auth.actor_kind,
                    &auth.user_id,
                    &session_id,
                    &Utc::now(),
                ],
            )
            .map_err(|_| postgres_store_error("ticket lookup"))?
            .map(|row| row_to_ticket(&row))
            .transpose()
    }

    fn consume_ticket_if_current(
        &self,
        ticket: &GroupKnowledgebaseLaunchTicket,
        auth: &AppContext,
        consumed_trace_id: &str,
    ) -> Result<bool, RuntimeError> {
        ticket.scope.validate()?;
        validate_group_knowledgebase_nonzero_u64(
            ticket.upstream_link_generation,
            "upstream link generation",
        )?;
        let upstream_link_generation = group_knowledgebase_u64_to_db_i64(
            ticket.upstream_link_generation,
            "upstream link generation",
        )?;
        let membership_epoch =
            group_knowledgebase_u64_to_db_i64(ticket.membership_epoch, "membership epoch")?;
        let organization_id = resolve_group_knowledgebase_organization_id(auth)?;
        let mut client = self.client()?;
        let session_id = require_group_knowledgebase_ticket_session(auth)?;
        let affected = client
            .execute(
                CONSUME_TICKET_SQL,
                &[
                    &ticket.ticket_hash,
                    &Utc::now(),
                    &KNOWLEDGEBASE_SERVICE_IDENTITY,
                    &consumed_trace_id,
                    &ticket.scope.tenant_id,
                    &organization_id,
                    &auth.actor_kind,
                    &auth.actor_id,
                    &auth.actor_kind,
                    &auth.user_id,
                    &session_id,
                    &ticket.knowledgebase_binding_id,
                    &ticket.knowledgebase_binding_uuid,
                    &upstream_link_generation,
                    &membership_epoch,
                    &ticket.scope.conversation_id,
                    &ticket.knowledge_space_id,
                    &ticket.knowledge_space_uuid,
                ],
            )
            .map_err(|_| postgres_store_error("ticket consume"))?;
        Ok(affected == 1)
    }
}

#[derive(Clone)]
pub struct GroupKnowledgebaseCoordinator {
    store: Arc<dyn GroupKnowledgebaseStore>,
    port: Arc<dyn GroupKnowledgebasePort>,
    id_generator: Arc<dyn IdGenerator>,
    launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher,
}

enum GroupKnowledgebaseEnsurePlan {
    Complete(GroupKnowledgebaseEnsureResult),
    Provision {
        scope: GroupKnowledgebaseScope,
        actor_id: String,
        newly_reserved: bool,
        source_event_id: String,
        request: EnsureGroupKnowledgebaseRequest,
    },
}

enum GroupKnowledgebaseLaunchPlan {
    Ready {
        scope: GroupKnowledgebaseScope,
        link: Box<GroupKnowledgebaseLink>,
    },
    Provisioning(GroupKnowledgebaseLinkView),
    Ensure,
}

impl GroupKnowledgebaseCoordinator {
    pub fn with_production_store(
        port: Arc<dyn GroupKnowledgebasePort>,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Result<Self, RuntimeError> {
        let store =
            PostgresGroupKnowledgebaseStore::from_shared_process_pool().ok_or_else(|| {
                RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase requires the shared PostgreSQL process pool".into(),
                ))
            })?;
        Ok(Self {
            store: Arc::new(store),
            port,
            id_generator,
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::from_runtime_env()?,
        })
    }

    /// Development/test-only construction is intentionally explicit. Production
    /// composition must call `with_production_store` and fail closed if its
    /// PostgreSQL pool is absent.
    pub fn with_development_memory_store(
        port: Arc<dyn GroupKnowledgebasePort>,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Result<Self, RuntimeError> {
        if !im_app_context::allows_header_only_app_context_fallback() {
            return Err(RuntimeError::Contract(ContractError::Unavailable(
                "in-memory group knowledgebase storage is development/test only".into(),
            )));
        }
        Ok(Self {
            store: Arc::new(InMemoryGroupKnowledgebaseStore::default()),
            port,
            id_generator,
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::from_runtime_env()?,
        })
    }

    #[cfg(test)]
    pub(super) fn with_memory_store(port: Arc<dyn GroupKnowledgebasePort>) -> Self {
        Self {
            store: Arc::new(InMemoryGroupKnowledgebaseStore::default()),
            port,
            id_generator: sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "conversation-knowledgebase-test",
            ),
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::for_test(),
        }
    }

    /// Validates that the injected generated Knowledgebase SDK adapter was
    /// fully constructed before an outbox worker leases durable events.
    pub async fn ensure_outbox_delivery_ready(&self) -> Result<(), RuntimeError> {
        self.port
            .ensure_delivery_ready()
            .await
            .map_err(group_knowledgebase_port_error_runtime_error)
    }

    /// Reconciles a bounded number of group-KB links from normalized
    /// Conversation and membership state. The cursor advances only after a
    /// link has been reconciled successfully.
    pub(crate) fn reconcile_durable_state(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        cursor: &mut GroupKnowledgebaseReconciliationCursor,
        max_links: usize,
    ) -> Result<usize, RuntimeError> {
        self.reconcile_durable_state_in_scope(runtime, cursor, max_links, None)
    }

    /// Variant used by the relay's optional tenant/organization override.
    /// It preserves the same cursor semantics while preventing a scoped worker
    /// from accidentally inspecting another organization's links.
    pub(crate) fn reconcile_durable_state_in_scope(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        cursor: &mut GroupKnowledgebaseReconciliationCursor,
        max_links: usize,
        scope_override: Option<(&str, &str)>,
    ) -> Result<usize, RuntimeError> {
        if max_links == 0 {
            return Err(RuntimeError::InvalidInput(
                "group knowledgebase reconciliation link limit must be positive".into(),
            ));
        }
        if cursor.pending_provisioning_recovery.is_some() {
            // The async relay must finish the idempotent remote ensure before
            // this keyset cursor may advance past a provisioning link.
            return Ok(0);
        }

        let configured_scope = scope_override
            .map(|(tenant_id, organization_id)| -> Result<GroupKnowledgebaseReconciliationScope, RuntimeError> {
                validate_group_knowledgebase_tenant_id(tenant_id)?;
                validate_group_knowledgebase_organization_id(organization_id)?;
                Ok(GroupKnowledgebaseReconciliationScope {
                    tenant_id: tenant_id.to_owned(),
                    organization_id: organization_id.to_owned(),
                })
            })
            .transpose()?;

        if let Some(configured_scope) = configured_scope.as_ref()
            && cursor.active_scope.as_ref() != Some(configured_scope)
        {
            cursor.active_scope = Some(configured_scope.clone());
            cursor.completed_scope = None;
            cursor.link_after_id = None;
        }

        let mut reconciled = 0;
        while reconciled < max_links {
            let scope = match cursor.active_scope.clone() {
                Some(scope) => scope,
                None => match configured_scope.clone() {
                    Some(scope) => {
                        cursor.active_scope = Some(scope.clone());
                        scope
                    }
                    None => {
                        let context = PrivilegedOperationContext::try_new(
                            PrivilegedOperationActorKind::ServiceWorker,
                            GROUP_KNOWLEDGEBASE_RECONCILIATION_ACTOR_ID,
                            sdkwork_utils_rust::id::uuid(),
                        )
                        .map_err(RuntimeError::Contract)?;
                        let request = GroupKnowledgebaseReconciliationScopeRequest {
                            context: &context,
                            after: cursor.completed_scope.as_ref(),
                        };
                        match self.store.next_reconciliation_scope(request)? {
                            Some(scope) => {
                                cursor.active_scope = Some(scope.clone());
                                cursor.link_after_id = None;
                                scope
                            }
                            None => {
                                // Completing a full pass resets the selector for
                                // the next recurring pass without retaining an
                                // unbounded historical cursor.
                                cursor.completed_scope = None;
                                return Ok(reconciled);
                            }
                        }
                    }
                },
            };

            let page = self.store.list_reconciliation_links(
                &scope,
                cursor.link_after_id,
                max_links.saturating_sub(reconciled),
            )?;
            if page.links.is_empty() && page.next_link_id.is_some() {
                return Err(RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase reconciliation page has an invalid cursor".into(),
                )));
            }

            for link in page.links {
                match self.reconcile_durable_link(runtime, &link)? {
                    GroupKnowledgebaseReconciliationLinkOutcome::Reconciled => {
                        cursor.active_scope = Some(scope.clone());
                        cursor.link_after_id = Some(link.id);
                        reconciled = reconciled.saturating_add(1);
                    }
                    GroupKnowledgebaseReconciliationLinkOutcome::ProvisioningRecovery(recovery) => {
                        cursor.pending_provisioning_recovery = Some(*recovery);
                        return Ok(reconciled);
                    }
                }
            }

            if page.next_link_id.is_some() {
                continue;
            }

            cursor.active_scope = None;
            cursor.link_after_id = None;
            if configured_scope.is_some() {
                cursor.completed_scope = None;
                return Ok(reconciled);
            }
            cursor.completed_scope = Some(scope);
        }

        Ok(reconciled)
    }

    fn reconcile_durable_link(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        link: &GroupKnowledgebaseLink,
    ) -> Result<GroupKnowledgebaseReconciliationLinkOutcome, RuntimeError> {
        let snapshot = group_knowledgebase_durable_snapshot(runtime, &link.scope)?;
        let archived_missing_space = matches!(
            link.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Archived
        ) && link.knowledge_space_id.is_none();
        if matches!(
            link.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Provisioning
        ) || (archived_missing_space && snapshot.archive.is_some())
        {
            // If remote ensure committed before this process crashed, the
            // stable creation idempotency key is the only safe way to recover
            // the remote space reference. Normalized Conversation lifecycle
            // decides whether the recovered reference is archived immediately.
            return Ok(
                GroupKnowledgebaseReconciliationLinkOutcome::ProvisioningRecovery(Box::new(
                    GroupKnowledgebaseProvisioningRecovery { link: link.clone() },
                )),
            );
        }
        if let Some(archive) = snapshot.archive.as_ref() {
            self.store
                .archive_link_and_enqueue(GroupKnowledgebaseArchiveEnqueue {
                    scope: link.scope.clone(),
                    actor_id: archive.actor_id.clone(),
                    source_event_id: archive.source_event_id.clone(),
                    outbox_id: self.next_id()?.to_string(),
                    occurred_at: Utc::now(),
                })?;
            return Ok(GroupKnowledgebaseReconciliationLinkOutcome::Reconciled);
        }

        if !link.lifecycle_state.is_active() {
            return Err(RuntimeError::Conflict(
                "group knowledgebase link is archived while its normalized Conversation is active"
                    .into(),
            ));
        }
        if link.membership_epoch > snapshot.membership_epoch
            || link.last_synchronized_membership_epoch > snapshot.membership_epoch
        {
            return Err(RuntimeError::Conflict(
                "group knowledgebase link membership epoch exceeds normalized Conversation state"
                    .into(),
            ));
        }
        if link.membership_epoch < snapshot.membership_epoch
            || link.last_synchronized_membership_epoch < snapshot.membership_epoch
        {
            self.enqueue_durable_membership_snapshot(
                &link.scope,
                GROUP_KNOWLEDGEBASE_RECONCILIATION_ACTOR_ID,
                &snapshot,
            )?;
        }
        Ok(GroupKnowledgebaseReconciliationLinkOutcome::Reconciled)
    }

    /// Completes one idempotent remote ensure for a provisioning link selected
    /// by durable reconciliation. The cursor remains pinned to this link until
    /// this operation succeeds, preventing a crash after remote creation from
    /// becoming a permanently stuck local `provisioning` state.
    pub(super) async fn recover_pending_provisioning(
        &self,
        runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
        cursor: &mut GroupKnowledgebaseReconciliationCursor,
    ) -> Result<bool, RuntimeError> {
        let Some(recovery) = cursor.pending_provisioning_recovery.take() else {
            return Ok(false);
        };
        let coordinator = self.clone();
        let snapshot_runtime = runtime.clone();
        let scope = recovery.link.scope.clone();
        let current =
            run_group_knowledgebase_blocking("provisioning recovery preparation", move || {
                let snapshot =
                    group_knowledgebase_durable_snapshot(snapshot_runtime.as_ref(), &scope)?;
                Ok(snapshot)
            })
            .await;
        let snapshot = match current {
            Ok(snapshot) => snapshot,
            Err(error) => {
                cursor.pending_provisioning_recovery = Some(recovery);
                return Err(error);
            }
        };

        let source_event_id = format!(
            "im.group-knowledgebase.provisioning.reconcile:{}:{}",
            recovery.link.id, recovery.link.version
        );
        let request = EnsureGroupKnowledgebaseRequest {
            scope: recovery.link.scope.clone(),
            group_name: group_knowledgebase_initial_group_name(
                recovery.link.scope.conversation_id.as_str(),
            ),
            idempotency_key: recovery.link.creation_idempotency_key.clone(),
            source_event_id: source_event_id.clone(),
            membership_epoch: snapshot.membership_epoch,
            members: group_knowledgebase_snapshot_members(&snapshot),
        };
        request.validate()?;
        let ensured = match self.port.ensure_group_knowledgebase(request).await {
            Ok(ensured) => ensured,
            Err(error) => {
                cursor.pending_provisioning_recovery = Some(recovery);
                return Err(group_knowledgebase_port_error_runtime_error(error));
            }
        };
        let recovery_runtime = runtime.clone();
        let recovery_link = recovery.link.clone();
        let completed =
            run_group_knowledgebase_blocking("provisioning recovery activation", move || {
                coordinator.complete_ensure(
                    recovery_runtime.as_ref(),
                    &recovery_link.scope,
                    recovery_link.created_by.as_str(),
                    false,
                    source_event_id.as_str(),
                    ensured,
                )
            })
            .await;
        if let Err(error) = completed {
            cursor.pending_provisioning_recovery = Some(recovery);
            return Err(error);
        }
        Ok(true)
    }

    fn enqueue_durable_membership_snapshot(
        &self,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        snapshot: &GroupKnowledgebaseDurableSnapshot,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        scope.validate()?;
        self.store
            .enqueue_membership_synchronization(GroupKnowledgebaseMembershipSyncEnqueue {
                scope: scope.clone(),
                actor_id: actor_id.to_owned(),
                source_event_id: group_knowledgebase_membership_reconciliation_source_event_id(
                    scope,
                    snapshot.membership_epoch,
                ),
                target_membership_epoch: snapshot.membership_epoch,
                members: group_knowledgebase_snapshot_members(snapshot),
                outbox_id: self.next_id()?.to_string(),
                occurred_at: Utc::now(),
            })
    }

    pub fn retrieve(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<GroupKnowledgebaseLinkView, RuntimeError> {
        runtime.ensure_active_group_conversation_from_auth_context(auth, conversation_id)?;
        let member = runtime.require_active_member_from_auth_context(auth, conversation_id)?;
        ensure_group_knowledgebase_access(&member)?;
        let scope = GroupKnowledgebaseScope::from_auth_context(auth, conversation_id)?;
        self.store.get_link(&scope).map(|link| {
            link.map(|link| link.view())
                .unwrap_or_else(|| GroupKnowledgebaseLinkView::absent(conversation_id))
        })
    }

    pub async fn ensure(
        &self,
        runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
        auth: AppContext,
        conversation_id: String,
    ) -> Result<GroupKnowledgebaseEnsureResult, RuntimeError> {
        let coordinator = self.clone();
        let prepare_runtime = runtime.clone();
        let prepare_auth = auth.clone();
        let prepare_conversation_id = conversation_id.clone();
        let plan = run_group_knowledgebase_blocking("ensure preparation", move || {
            coordinator.prepare_ensure(
                prepare_runtime.as_ref(),
                &prepare_auth,
                prepare_conversation_id.as_str(),
            )
        })
        .await?;

        let (scope, actor_id, newly_reserved, source_event_id, request) = match plan {
            GroupKnowledgebaseEnsurePlan::Complete(result) => return Ok(result),
            GroupKnowledgebaseEnsurePlan::Provision {
                scope,
                actor_id,
                newly_reserved,
                source_event_id,
                request,
            } => (scope, actor_id, newly_reserved, source_event_id, request),
        };

        request.validate()?;
        match self.port.ensure_group_knowledgebase(request).await {
            Ok(ensured) => {
                let coordinator = self.clone();
                let activate_runtime = runtime.clone();
                run_group_knowledgebase_blocking("ensure activation", move || {
                    coordinator.complete_ensure(
                        activate_runtime.as_ref(),
                        &scope,
                        actor_id.as_str(),
                        newly_reserved,
                        source_event_id.as_str(),
                        ensured,
                    )
                })
                .await
            }
            Err(error) => {
                let error_code = group_knowledgebase_port_error_code(&error).to_owned();
                let store = self.store.clone();
                let failed_scope = scope.clone();
                let failed_actor_id = actor_id.clone();
                run_group_knowledgebase_blocking("ensure failure recording", move || {
                    store.fail_link(&failed_scope, failed_actor_id.as_str(), error_code.as_str())
                })
                .await?;
                Err(group_knowledgebase_port_error_runtime_error(error))
            }
        }
    }

    fn prepare_ensure(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<GroupKnowledgebaseEnsurePlan, RuntimeError> {
        runtime.ensure_active_group_conversation_from_auth_context(auth, conversation_id)?;
        let member = runtime.require_active_member_from_auth_context(auth, conversation_id)?;
        ensure_group_knowledgebase_access(&member)?;
        ensure_group_knowledgebase_owner(&member)?;
        let scope = GroupKnowledgebaseScope::from_auth_context(auth, conversation_id)?;
        if let Some(link) = self.store.get_link(&scope)? {
            match link.lifecycle_state {
                GroupKnowledgebaseLifecycleState::Active => {
                    let link = self.schedule_initial_membership_sync_if_needed(
                        runtime,
                        &scope,
                        auth.actor_id.as_str(),
                        link,
                    )?;
                    return Ok(GroupKnowledgebaseEnsurePlan::Complete(
                        GroupKnowledgebaseEnsureResult::Existing(link.view()),
                    ));
                }
                GroupKnowledgebaseLifecycleState::Provisioning => {
                    return Ok(GroupKnowledgebaseEnsurePlan::Complete(
                        GroupKnowledgebaseEnsureResult::Provisioning(link.view()),
                    ));
                }
                GroupKnowledgebaseLifecycleState::Archived
                | GroupKnowledgebaseLifecycleState::Deleted => {
                    return Err(RuntimeError::Conflict(
                        "group knowledgebase lifecycle does not permit automatic reprovisioning"
                            .into(),
                    ));
                }
                GroupKnowledgebaseLifecycleState::Failed
                | GroupKnowledgebaseLifecycleState::Absent => {}
            }
        }

        let (link, newly_reserved) = match self.store.get_link(&scope)? {
            Some(link)
                if matches!(
                    link.lifecycle_state,
                    GroupKnowledgebaseLifecycleState::Failed
                ) =>
            {
                let retry_source_event_id =
                    format!("im.group-knowledgebase.retry:{}:{}", link.id, link.version);
                (
                    self.store.begin_retry_provisioning(
                        &scope,
                        auth.actor_id.as_str(),
                        retry_source_event_id.as_str(),
                    )?,
                    false,
                )
            }
            Some(link)
                if matches!(
                    link.lifecycle_state,
                    GroupKnowledgebaseLifecycleState::Provisioning
                ) =>
            {
                return Ok(GroupKnowledgebaseEnsurePlan::Complete(
                    GroupKnowledgebaseEnsureResult::Provisioning(link.view()),
                ));
            }
            Some(link) if link.lifecycle_state.is_active() => {
                let link = self.schedule_initial_membership_sync_if_needed(
                    runtime,
                    &scope,
                    auth.actor_id.as_str(),
                    link,
                )?;
                return Ok(GroupKnowledgebaseEnsurePlan::Complete(
                    GroupKnowledgebaseEnsureResult::Existing(link.view()),
                ));
            }
            Some(link) => {
                return Err(RuntimeError::Conflict(format!(
                    "group knowledgebase lifecycle {:?} does not permit automatic provisioning",
                    link.lifecycle_state
                )));
            }
            None => {
                let candidate = GroupKnowledgebaseLink::new(
                    self.next_id()?,
                    random_opaque_value("gkl_")?,
                    scope.clone(),
                    auth.actor_id.clone(),
                    Utc::now(),
                );
                let reservation = self.store.reserve_link(candidate)?;
                if !reservation.newly_reserved {
                    match reservation.link.lifecycle_state {
                        GroupKnowledgebaseLifecycleState::Active => {
                            let link = self.schedule_initial_membership_sync_if_needed(
                                runtime,
                                &scope,
                                auth.actor_id.as_str(),
                                reservation.link,
                            )?;
                            return Ok(GroupKnowledgebaseEnsurePlan::Complete(
                                GroupKnowledgebaseEnsureResult::Existing(link.view()),
                            ));
                        }
                        GroupKnowledgebaseLifecycleState::Provisioning => {
                            return Ok(GroupKnowledgebaseEnsurePlan::Complete(
                                GroupKnowledgebaseEnsureResult::Provisioning(
                                    reservation.link.view(),
                                ),
                            ));
                        }
                        GroupKnowledgebaseLifecycleState::Archived
                        | GroupKnowledgebaseLifecycleState::Deleted => {
                            return Err(RuntimeError::Conflict(
                                "group knowledgebase lifecycle does not permit automatic reprovisioning"
                                    .into(),
                            ));
                        }
                        GroupKnowledgebaseLifecycleState::Failed
                        | GroupKnowledgebaseLifecycleState::Absent => {
                            return Err(RuntimeError::Conflict(
                                "group knowledgebase link changed while provisioning was reserved"
                                    .into(),
                            ));
                        }
                    }
                }
                (reservation.link, reservation.newly_reserved)
            }
        };

        let snapshot = group_knowledgebase_durable_snapshot(runtime, &scope)?;
        if snapshot.archive.is_some() {
            return Err(RuntimeError::Conflict(
                "group conversation is archived and its knowledgebase cannot be provisioned".into(),
            ));
        }
        let source_event_id = format!("im.group-knowledgebase.ensure:{}:{}", link.id, link.version);
        let request = EnsureGroupKnowledgebaseRequest {
            scope: scope.clone(),
            // Conversation currently does not persist an independent group title
            // in its aggregate. The KB owns the editable display title; using a
            // stable conversation-derived initial title avoids a client authority.
            group_name: group_knowledgebase_initial_group_name(scope.conversation_id.as_str()),
            idempotency_key: link.creation_idempotency_key.clone(),
            source_event_id: source_event_id.clone(),
            membership_epoch: snapshot.membership_epoch,
            members: group_knowledgebase_snapshot_members(&snapshot),
        };
        Ok(GroupKnowledgebaseEnsurePlan::Provision {
            scope,
            actor_id: auth.actor_id.clone(),
            newly_reserved,
            source_event_id,
            request,
        })
    }

    fn complete_ensure(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        newly_reserved: bool,
        source_event_id: &str,
        ensured: EnsuredGroupKnowledgebase,
    ) -> Result<GroupKnowledgebaseEnsureResult, RuntimeError> {
        scope.validate()?;
        let snapshot = group_knowledgebase_durable_snapshot(runtime, scope)?;
        if let Some(archive) = snapshot.archive.as_ref() {
            self.store
                .archive_link_and_enqueue(GroupKnowledgebaseArchiveEnqueue {
                    scope: scope.clone(),
                    actor_id: archive.actor_id.clone(),
                    source_event_id: archive.source_event_id.clone(),
                    outbox_id: self.next_id()?.to_string(),
                    occurred_at: Utc::now(),
                })?;
        }
        let archive_outbox_id = self.next_id()?.to_string();
        let activated = self.store.activate_link(
            scope,
            ensured,
            snapshot.membership_epoch,
            actor_id,
            source_event_id,
            archive_outbox_id.as_str(),
        )?;
        let link = if snapshot.archive.is_some() {
            activated
        } else {
            self.schedule_initial_membership_sync_if_needed(runtime, scope, actor_id, activated)?
        };
        let view = link.view();
        Ok(if newly_reserved && link.lifecycle_state.is_active() {
            GroupKnowledgebaseEnsureResult::Created(view)
        } else {
            GroupKnowledgebaseEnsureResult::Existing(view)
        })
    }

    /// Persist a complete authoritative roster snapshot after a group member
    /// mutation. The link epoch and outbox record share one transaction in
    /// PostgreSQL, so a committed invalidation always has a retryable KB ACL
    /// synchronization request.
    pub fn record_membership_change(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        auth: &AppContext,
        conversation_id: &str,
        source_event_id: &str,
    ) -> Result<Option<GroupKnowledgebaseLinkView>, RuntimeError> {
        match runtime.ensure_active_group_conversation_from_auth_context(auth, conversation_id) {
            Ok(()) => {}
            Err(RuntimeError::ConversationTypeInvalid(_)) => return Ok(None),
            Err(error) => return Err(error),
        }
        if source_event_id.trim().is_empty() || source_event_id.len() > 256 {
            return Err(RuntimeError::InvalidInput(
                "group knowledgebase membership source event is invalid".into(),
            ));
        }
        let scope = match GroupKnowledgebaseScope::from_auth_context(auth, conversation_id) {
            Ok(scope) => scope,
            // Membership mutation is an IM capability, not a group-KB
            // capability. A malformed token-derived scope must not prevent the
            // Conversation mutation from completing.
            Err(RuntimeError::PermissionDenied(_)) => return Ok(None),
            Err(error) => return Err(error),
        };
        self.enqueue_current_membership_snapshot(runtime, &scope, auth.actor_id.as_str())
            .map(|link| link.map(|link| link.view()))
    }

    pub async fn launch(
        &self,
        runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
        auth: AppContext,
        conversation_id: String,
        idempotency_key: String,
    ) -> Result<GroupKnowledgebaseLaunchResult, RuntimeError> {
        validate_group_knowledgebase_launch_idempotency_key(idempotency_key.as_str())?;
        require_group_knowledgebase_ticket_session(&auth)?;
        loop {
            let coordinator = self.clone();
            let prepare_runtime = runtime.clone();
            let prepare_auth = auth.clone();
            let prepare_conversation_id = conversation_id.clone();
            let plan = run_group_knowledgebase_blocking("launch preparation", move || {
                coordinator.prepare_launch(
                    prepare_runtime.as_ref(),
                    &prepare_auth,
                    prepare_conversation_id.as_str(),
                )
            })
            .await?;

            match plan {
                GroupKnowledgebaseLaunchPlan::Provisioning(view) => {
                    return Ok(GroupKnowledgebaseLaunchResult::Provisioning(view));
                }
                GroupKnowledgebaseLaunchPlan::Ensure => {
                    let ensured = self
                        .ensure(runtime.clone(), auth.clone(), conversation_id.clone())
                        .await?;
                    if !matches!(
                        ensured.view().lifecycle_state,
                        GroupKnowledgebaseLifecycleState::Active
                    ) {
                        return Ok(GroupKnowledgebaseLaunchResult::Provisioning(
                            ensured.view().clone(),
                        ));
                    }
                }
                GroupKnowledgebaseLaunchPlan::Ready { scope, link } => {
                    let coordinator = self.clone();
                    let issue_auth = auth.clone();
                    let issue_idempotency_key = idempotency_key.clone();
                    return run_group_knowledgebase_blocking("launch ticket issuance", move || {
                        coordinator.issue_launch_ticket(
                            scope,
                            *link,
                            &issue_auth,
                            issue_idempotency_key.as_str(),
                        )
                    })
                    .await;
                }
            }
        }
    }

    fn prepare_launch(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<GroupKnowledgebaseLaunchPlan, RuntimeError> {
        runtime.ensure_active_group_conversation_from_auth_context(auth, conversation_id)?;
        let member = runtime.require_active_member_from_auth_context(auth, conversation_id)?;
        ensure_group_knowledgebase_access(&member)?;
        let scope = GroupKnowledgebaseScope::from_auth_context(auth, conversation_id)?;
        match self.store.get_link(&scope)? {
            Some(link) if link.lifecycle_state.is_active() => {
                Ok(GroupKnowledgebaseLaunchPlan::Ready {
                    scope,
                    link: Box::new(link),
                })
            }
            Some(link)
                if matches!(
                    link.lifecycle_state,
                    GroupKnowledgebaseLifecycleState::Provisioning
                ) =>
            {
                Ok(GroupKnowledgebaseLaunchPlan::Provisioning(link.view()))
            }
            Some(_) | None => {
                ensure_group_knowledgebase_owner(&member)?;
                Ok(GroupKnowledgebaseLaunchPlan::Ensure)
            }
        }
    }

    fn issue_launch_ticket(
        &self,
        scope: GroupKnowledgebaseScope,
        link: GroupKnowledgebaseLink,
        auth: &AppContext,
        idempotency_key: &str,
    ) -> Result<GroupKnowledgebaseLaunchResult, RuntimeError> {
        scope.validate()?;
        let session_id = require_group_knowledgebase_ticket_session(auth)?.to_owned();
        let target = active_group_knowledgebase_reference(&link)?;
        if link.last_synchronized_membership_epoch != link.membership_epoch {
            return Ok(GroupKnowledgebaseLaunchResult::Provisioning(link.view()));
        }
        let raw_ticket = random_opaque_value("gklt_")?;
        let expires_at = Utc::now() + Duration::seconds(GROUP_KNOWLEDGEBASE_TICKET_TTL_SECONDS);
        let ticket = GroupKnowledgebaseLaunchTicket {
            id: self.next_id()?,
            ticket_hash: sha256_hash(raw_ticket.as_bytes()),
            scope: scope.clone(),
            knowledge_space_id: target.knowledge_space_id,
            knowledge_space_uuid: target.knowledge_space_uuid.clone(),
            knowledgebase_binding_id: target.knowledgebase_binding_id,
            knowledgebase_binding_uuid: target.knowledgebase_binding_uuid.clone(),
            upstream_link_generation: link.version,
            membership_epoch: link.membership_epoch,
            actor_kind: auth.actor_kind.clone(),
            actor_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
            principal_id: auth.user_id.clone(),
            session_id,
            issuing_app_id: auth.app_id.clone(),
            issued_by: auth.actor_id.clone(),
            idempotency_key_hash: sha256_hash(idempotency_key.as_bytes()),
            request_fingerprint_hash: group_knowledgebase_launch_request_fingerprint(
                &scope, &link, auth,
            ),
            ticket_ciphertext: self.launch_ticket_cipher.encrypt(raw_ticket.as_str())?,
            expires_at,
            consumed_at: None,
            consumed_by_service: None,
            consumed_trace_id: None,
        };
        match self.store.reserve_ticket(ticket)? {
            GroupKnowledgebaseLaunchTicketReservation::Created => Ok(
                GroupKnowledgebaseLaunchResult::Ready(GroupKnowledgebaseLaunchView {
                    conversation_id: scope.conversation_id,
                    space_id: target.knowledge_space_id.to_string(),
                    space_uuid: target.knowledge_space_uuid,
                    launch_ticket: raw_ticket,
                    expires_at: expires_at.to_rfc3339(),
                    membership_epoch: link.membership_epoch,
                    upstream_link_generation: link.version,
                }),
            ),
            GroupKnowledgebaseLaunchTicketReservation::Existing(existing) => {
                self.replay_launch_ticket(*existing, &scope, &link, auth, idempotency_key)
            }
        }
    }

    fn replay_launch_ticket(
        &self,
        ticket: GroupKnowledgebaseLaunchTicket,
        scope: &GroupKnowledgebaseScope,
        link: &GroupKnowledgebaseLink,
        auth: &AppContext,
        idempotency_key: &str,
    ) -> Result<GroupKnowledgebaseLaunchResult, RuntimeError> {
        scope.validate()?;
        ticket.scope.validate()?;
        let session_id = require_group_knowledgebase_ticket_session(auth)?;
        let expected_fingerprint =
            group_knowledgebase_launch_request_fingerprint(scope, link, auth);
        if ticket.idempotency_key_hash != sha256_hash(idempotency_key.as_bytes())
            || ticket.request_fingerprint_hash != expected_fingerprint
        {
            return Err(RuntimeError::Conflict(
                "group knowledgebase launch idempotency key conflicts with a different request"
                    .into(),
            ));
        }
        if ticket.consumed_at.is_some() || ticket.expires_at <= Utc::now() {
            return Err(RuntimeError::Conflict(
                "group knowledgebase launch idempotency key cannot replay a consumed or expired ticket"
                    .into(),
            ));
        }
        if !link.lifecycle_state.is_active()
            || link.knowledge_space_id != Some(ticket.knowledge_space_id)
            || link.knowledge_space_uuid.as_deref() != Some(ticket.knowledge_space_uuid.as_str())
            || link.knowledgebase_binding_id != Some(ticket.knowledgebase_binding_id)
            || link.knowledgebase_binding_uuid.as_deref()
                != Some(ticket.knowledgebase_binding_uuid.as_str())
            || link.version != ticket.upstream_link_generation
            || link.membership_epoch != ticket.membership_epoch
            || link.last_synchronized_membership_epoch != ticket.membership_epoch
            || ticket.actor_kind != auth.actor_kind
            || ticket.actor_id != auth.actor_id
            || ticket.principal_kind != auth.actor_kind
            || ticket.principal_id != auth.user_id
            || ticket.session_id != session_id
        {
            return Err(RuntimeError::Conflict(
                "group knowledgebase launch idempotency key references a stale ticket".into(),
            ));
        }
        let raw_ticket = self
            .launch_ticket_cipher
            .decrypt(ticket.ticket_ciphertext.as_str())?;
        validate_launch_ticket(raw_ticket.as_str())?;
        if sha256_hash(raw_ticket.as_bytes()) != ticket.ticket_hash {
            return Err(RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase launch ticket replay integrity check failed".into(),
            )));
        }
        Ok(GroupKnowledgebaseLaunchResult::Ready(
            GroupKnowledgebaseLaunchView {
                conversation_id: ticket.scope.conversation_id,
                space_id: ticket.knowledge_space_id.to_string(),
                space_uuid: ticket.knowledge_space_uuid,
                launch_ticket: raw_ticket,
                expires_at: ticket.expires_at.to_rfc3339(),
                membership_epoch: ticket.membership_epoch,
                upstream_link_generation: ticket.upstream_link_generation,
            },
        ))
    }

    /// Consumes a ticket after the internal RPC adapter has authenticated the
    /// caller as `sdkwork-knowledgebase` and constructed `auth` exclusively
    /// from an mTLS-bound, signed caller context. The service identity and
    /// delegated principal are deliberately not accepted from this API's
    /// caller-controlled payload.
    pub fn consume_launch_ticket_from_trusted_knowledgebase(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        auth: &AppContext,
        ticket: &str,
        consumed_trace_id: &str,
    ) -> Result<ConsumedGroupKnowledgebaseLaunchTicket, RuntimeError> {
        resolve_group_knowledgebase_organization_id(auth)?;
        validate_launch_ticket(ticket)?;
        validate_ticket_consumer_trace_id(consumed_trace_id)?;
        require_group_knowledgebase_ticket_session(auth)?;
        let ticket_hash = sha256_hash(ticket.as_bytes());
        let ticket = self
            .store
            .find_unconsumed_ticket_for_consumer(ticket_hash.as_str(), auth)?
            .ok_or_else(|| {
                RuntimeError::PermissionDenied("group knowledgebase launch denied".into())
            })?;
        ticket.scope.validate()?;
        ensure_group_knowledgebase_ticket_consumer_matches(&ticket, auth)?;
        runtime.ensure_active_group_conversation_from_auth_context(
            auth,
            ticket.scope.conversation_id.as_str(),
        )?;
        let member = runtime
            .require_active_member_from_auth_context(auth, ticket.scope.conversation_id.as_str())?;
        ensure_group_knowledgebase_access(&member)?;
        if !self
            .store
            .consume_ticket_if_current(&ticket, auth, consumed_trace_id)?
        {
            return Err(RuntimeError::PermissionDenied(
                "group knowledgebase launch denied".into(),
            ));
        }
        Ok(ConsumedGroupKnowledgebaseLaunchTicket {
            conversation_id: ticket.scope.conversation_id,
            space_id: ticket.knowledge_space_id.to_string(),
            space_uuid: ticket.knowledge_space_uuid,
            knowledgebase_binding_id: ticket.knowledgebase_binding_id.to_string(),
            knowledgebase_binding_uuid: ticket.knowledgebase_binding_uuid,
            lifecycle_state: GroupKnowledgebaseLifecycleState::Active,
            membership_role: member.role,
            membership_epoch: ticket.membership_epoch,
            upstream_link_generation: ticket.upstream_link_generation,
            expires_at: ticket.expires_at.to_rfc3339(),
        })
    }

    /// Deliver one durable IM outbox payload through the injected generated
    /// Knowledgebase SDK adapter. The outbox relay owns retry/backoff; this
    /// method is deterministic and intentionally has no HTTP concerns.
    pub async fn deliver_outbox_payload(
        &self,
        payload: GroupKnowledgebaseOutboxPayload,
    ) -> Result<(), RuntimeError> {
        payload.validate()?;
        let target = GroupKnowledgebaseTargetFence {
            knowledge_space_id: payload.knowledge_space_id,
            knowledge_space_uuid: payload.knowledge_space_uuid.clone(),
            knowledgebase_binding_id: payload.knowledgebase_binding_id,
            knowledgebase_binding_uuid: payload.knowledgebase_binding_uuid.clone(),
        };
        target.validate()?;
        match payload.operation.clone() {
            GroupKnowledgebaseOutboxOperation::SynchronizeMembers => {
                let request = SynchronizeGroupKnowledgebaseMembersRequest {
                    scope: payload.scope.clone(),
                    knowledge_space_id: target.knowledge_space_id,
                    knowledge_space_uuid: target.knowledge_space_uuid,
                    knowledgebase_binding_id: target.knowledgebase_binding_id,
                    knowledgebase_binding_uuid: target.knowledgebase_binding_uuid,
                    upstream_link_generation: payload.upstream_link_generation,
                    membership_epoch: payload.membership_epoch,
                    source_event_id: payload.source_event_id,
                    members: payload.members,
                };
                request.validate()?;
                self.port
                    .synchronize_group_members(request)
                    .await
                    .map_err(group_knowledgebase_port_error_runtime_error)?;
                // A later membership change can supersede this event while it
                // was leased. That is successful delivery; the newer event
                // remains the current ACL authority.
                let store = self.store.clone();
                let scope = payload.scope;
                let membership_epoch = payload.membership_epoch;
                let upstream_link_generation = payload.upstream_link_generation;
                let _ = run_group_knowledgebase_blocking(
                    "membership synchronization acknowledgement",
                    move || {
                        store.mark_membership_synchronized(
                            &scope,
                            membership_epoch,
                            upstream_link_generation,
                            KNOWLEDGEBASE_SERVICE_IDENTITY,
                        )
                    },
                )
                .await?;
                Ok(())
            }
            GroupKnowledgebaseOutboxOperation::Archive => {
                let archived_by = payload.archived_by.as_deref().ok_or_else(|| {
                    RuntimeError::Conflict(
                        "group knowledgebase archive outbox payload is missing its archived-by actor"
                            .into(),
                    )
                })?;
                validate_group_knowledgebase_actor_id(archived_by)?;
                let request = ArchiveGroupKnowledgebaseRequest {
                    scope: payload.scope,
                    knowledge_space_id: target.knowledge_space_id,
                    knowledge_space_uuid: target.knowledge_space_uuid,
                    knowledgebase_binding_id: target.knowledgebase_binding_id,
                    knowledgebase_binding_uuid: target.knowledgebase_binding_uuid,
                    membership_epoch: payload.membership_epoch,
                    upstream_link_generation: payload.upstream_link_generation,
                    source_event_id: payload.source_event_id,
                    archived_by: archived_by.to_owned(),
                };
                request.validate()?;
                let archive_state = self
                    .port
                    .archive_group_knowledgebase(request)
                    .await
                    .map_err(group_knowledgebase_port_error_runtime_error)?;
                match archive_state {
                    GroupKnowledgebaseArchiveDeliveryState::Archived
                    | GroupKnowledgebaseArchiveDeliveryState::Deleted => Ok(()),
                    GroupKnowledgebaseArchiveDeliveryState::Archiving => {
                        Err(RuntimeError::Contract(ContractError::Unavailable(
                            "group knowledgebase archive is still processing".into(),
                        )))
                    }
                }
            }
        }
    }

    fn schedule_initial_membership_sync_if_needed(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
        link: GroupKnowledgebaseLink,
    ) -> Result<GroupKnowledgebaseLink, RuntimeError> {
        scope.validate()?;
        if !link.lifecycle_state.is_active() {
            return Ok(link);
        }
        let snapshot = group_knowledgebase_durable_snapshot(runtime, scope)?;
        if let Some(archive) = snapshot.archive.as_ref() {
            return self
                .store
                .archive_link_and_enqueue(GroupKnowledgebaseArchiveEnqueue {
                    scope: scope.clone(),
                    actor_id: archive.actor_id.clone(),
                    source_event_id: archive.source_event_id.clone(),
                    outbox_id: self.next_id()?.to_string(),
                    occurred_at: Utc::now(),
                })?
                .ok_or_else(|| {
                    RuntimeError::Contract(ContractError::Unavailable(
                        "active group knowledgebase link disappeared before archival".into(),
                    ))
                });
        }
        if link.membership_epoch > snapshot.membership_epoch
            || link.last_synchronized_membership_epoch > snapshot.membership_epoch
        {
            return Err(RuntimeError::Conflict(
                "group knowledgebase link membership epoch exceeds normalized Conversation state"
                    .into(),
            ));
        }
        if link.membership_epoch == snapshot.membership_epoch
            && link.last_synchronized_membership_epoch >= snapshot.membership_epoch
        {
            return Ok(link);
        }
        self.enqueue_current_membership_snapshot(runtime, scope, actor_id)?
            .ok_or_else(|| {
                RuntimeError::Contract(ContractError::Unavailable(
                    "active group knowledgebase link disappeared before membership synchronization"
                        .into(),
                ))
            })
    }

    fn enqueue_current_membership_snapshot(
        &self,
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        scope: &GroupKnowledgebaseScope,
        actor_id: &str,
    ) -> Result<Option<GroupKnowledgebaseLink>, RuntimeError> {
        scope.validate()?;
        let snapshot = group_knowledgebase_durable_snapshot(runtime, scope)?;
        if snapshot.archive.is_some() {
            return Err(RuntimeError::Conflict(
                "group knowledgebase membership synchronization cannot target an archived conversation"
                    .into(),
            ));
        }
        self.enqueue_durable_membership_snapshot(scope, actor_id, &snapshot)
    }

    fn next_id(&self) -> Result<i64, RuntimeError> {
        self.id_generator.next_id().map_err(RuntimeError::Contract)
    }

    /// Applies the IM aggregate's already-committed archive event to the
    /// local group-to-space conversation_state and atomically queues the KB archive
    /// handoff when a space exists. The source event id makes retries safe.
    pub fn archive_after_group_conversation_archive(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        source_event_id: &str,
    ) -> Result<bool, RuntimeError> {
        if source_event_id.trim().is_empty() || source_event_id.len() > 256 {
            return Err(RuntimeError::InvalidInput(
                "group knowledgebase archive source event is invalid".into(),
            ));
        }
        let scope = match GroupKnowledgebaseScope::from_auth_context(auth, conversation_id) {
            Ok(scope) => scope,
            // Archiving a conversation is independent of group-KB support.
            // Deliberately do not enqueue a conversation_state for a malformed
            // token-derived scope.
            Err(RuntimeError::PermissionDenied(_)) => return Ok(false),
            Err(error) => return Err(error),
        };
        let link = self
            .store
            .archive_link_and_enqueue(GroupKnowledgebaseArchiveEnqueue {
                scope,
                actor_id: auth.actor_id.clone(),
                source_event_id: source_event_id.to_owned(),
                outbox_id: self.next_id()?.to_string(),
                occurred_at: Utc::now(),
            })?;
        Ok(link.is_some())
    }
}

fn group_knowledgebase_durable_snapshot(
    runtime: &ConversationRuntime<ConversationCommitJournal>,
    scope: &GroupKnowledgebaseScope,
) -> Result<GroupKnowledgebaseDurableSnapshot, RuntimeError> {
    scope.validate()?;
    let Some(aggregate_store) = runtime.aggregate_store.as_ref() else {
        // Dev/test runtimes without a durable aggregate store treat the hot
        // conversation state as authoritative, matching the runtime's fallback
        // rule for aggregate-store-less processes. KB membership
        // synchronization is a best-effort side channel: it must never block
        // the IM membership mutation.
        return group_knowledgebase_hot_snapshot(runtime, scope);
    };
    let conversation = aggregate_store
        .load_conversation(
            scope.tenant_id.as_str(),
            scope.organization_id.as_str(),
            scope.conversation_id.as_str(),
        )?
        .ok_or_else(|| RuntimeError::ConversationNotFound(scope.conversation_id.clone()))?;
    if conversation.tenant_id != scope.tenant_id
        || conversation.organization_id != scope.organization_id
        || conversation.conversation_id != scope.conversation_id
    {
        return Err(RuntimeError::Conflict(
            "group knowledgebase normalized Conversation does not match its requested scope".into(),
        ));
    }
    if conversation.conversation_type != "group" {
        return Err(RuntimeError::ConversationTypeInvalid(
            "group knowledgebase normalized state targets a non-group Conversation".into(),
        ));
    }
    if conversation.member_epoch > conversation.commit_seq {
        return Err(RuntimeError::Conflict(
            "group knowledgebase normalized membership epoch exceeds the Conversation commit sequence"
                .into(),
        ));
    }
    let archive = match conversation.lifecycle_state.as_str() {
        "active" => None,
        "archived" if conversation.commit_seq > 0 => {
            Some(GroupKnowledgebaseArchiveReconciliation {
                source_event_id: group_knowledgebase_archive_reconciliation_source_event_id(
                    scope,
                    conversation.commit_seq,
                ),
                actor_id: GROUP_KNOWLEDGEBASE_RECONCILIATION_ACTOR_ID.into(),
            })
        }
        "archived" => {
            return Err(RuntimeError::Conflict(
                "group knowledgebase normalized archived Conversation has no committed transition"
                    .into(),
            ));
        }
        _ => {
            return Err(RuntimeError::Conflict(
                "group knowledgebase normalized Conversation lifecycle is invalid".into(),
            ));
        }
    };

    let mut roster = ConversationRoster::default();
    let mut cursor: Option<ConversationMemberPageCursor> = None;
    loop {
        let page = aggregate_store.load_members_page(
            scope.tenant_id.as_str(),
            scope.organization_id.as_str(),
            scope.conversation_id.as_str(),
            cursor.as_ref(),
            CONVERSATION_AGGREGATE_PAGE_SIZE_MAX,
        )?;
        if page.items.len() > CONVERSATION_AGGREGATE_PAGE_SIZE_MAX
            || page.has_more != page.next_cursor.is_some()
            || (page.items.is_empty() && page.has_more)
        {
            return Err(RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase normalized member page is invalid".into(),
            )));
        }

        let mut previous_key = cursor
            .as_ref()
            .map(|cursor| (cursor.principal_kind.as_str(), cursor.principal_id.as_str()));
        for record in &page.items {
            let current_key = (record.principal_kind.as_str(), record.principal_id.as_str());
            if previous_key.is_some_and(|previous_key| current_key <= previous_key) {
                return Err(RuntimeError::Contract(ContractError::Unavailable(
                    "group knowledgebase normalized member page is not strictly ordered".into(),
                )));
            }
            roster.upsert_member(group_knowledgebase_member_from_normalized_record(
                scope, record,
            )?);
            previous_key = Some(current_key);
        }
        if roster.active_principal_count()
            > im_domain_core::space::MAX_CHAT_GROUP_MAX_MEMBERS as usize
        {
            return Err(RuntimeError::Conflict(
                "group knowledgebase normalized roster exceeds the maximum group size".into(),
            ));
        }

        let Some(next_cursor) = page.next_cursor else {
            break;
        };
        let last_record = page.items.last().ok_or_else(|| {
            RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase normalized member cursor has no source row".into(),
            ))
        })?;
        if next_cursor.principal_kind != last_record.principal_kind
            || next_cursor.principal_id != last_record.principal_id
            || cursor.as_ref() == Some(&next_cursor)
        {
            return Err(RuntimeError::Contract(ContractError::Unavailable(
                "group knowledgebase normalized member cursor did not advance".into(),
            )));
        }
        cursor = Some(next_cursor);
    }

    Ok(GroupKnowledgebaseDurableSnapshot {
        membership_epoch: conversation.member_epoch,
        roster,
        archive,
    })
}

/// Builds the membership snapshot from the hot conversation state for
/// aggregate-store-less runtimes (dev/test fixtures and embedded wiring).
/// The hot aggregate and roster are authoritative in that configuration,
/// which mirrors `ensure_conversation_loaded`'s in-memory authority rule.
fn group_knowledgebase_hot_snapshot(
    runtime: &ConversationRuntime<ConversationCommitJournal>,
    scope: &GroupKnowledgebaseScope,
) -> Result<GroupKnowledgebaseDurableSnapshot, RuntimeError> {
    runtime.ensure_conversation_loaded(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.conversation_id.as_str(),
    )?;
    let scope_key = conversation_scope_key(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.conversation_id.as_str(),
    );
    let state = read_runtime_state(
        &runtime.state,
        "conversation-runtime.state.knowledgebase.hot-snapshot",
    );
    let conversation = state
        .conversations
        .get(scope_key.as_str())
        .ok_or_else(|| RuntimeError::ConversationNotFound(scope.conversation_id.clone()))?;
    if conversation.aggregate.conversation_type() != "group" {
        return Err(RuntimeError::ConversationTypeInvalid(
            "group knowledgebase hot state targets a non-group Conversation".into(),
        ));
    }
    let commit_seq = conversation.aggregate.commit_seq();
    let archive = match conversation.aggregate.lifecycle_state() {
        ConversationLifecycleState::Active => None,
        ConversationLifecycleState::Archived if commit_seq > 0 => {
            Some(GroupKnowledgebaseArchiveReconciliation {
                source_event_id: group_knowledgebase_archive_reconciliation_source_event_id(
                    scope,
                    commit_seq,
                ),
                actor_id: GROUP_KNOWLEDGEBASE_RECONCILIATION_ACTOR_ID.into(),
            })
        }
        ConversationLifecycleState::Archived => {
            return Err(RuntimeError::Conflict(
                "group knowledgebase hot archived Conversation has no committed transition".into(),
            ));
        }
    };
    let mut roster = ConversationRoster::default();
    for member in conversation.roster.members().values() {
        if !member.is_active() {
            continue;
        }
        let record = member_to_record(
            scope.tenant_id.as_str(),
            scope.organization_id.as_str(),
            scope.conversation_id.as_str(),
            member,
        );
        roster.upsert_member(group_knowledgebase_member_from_normalized_record(scope, &record)?);
    }
    if roster.active_principal_count() > im_domain_core::space::MAX_CHAT_GROUP_MAX_MEMBERS as usize
    {
        return Err(RuntimeError::Conflict(
            "group knowledgebase hot roster exceeds the maximum group size".into(),
        ));
    }
    Ok(GroupKnowledgebaseDurableSnapshot {
        membership_epoch: conversation.aggregate.member_epoch(),
        roster,
        archive,
    })
}

fn group_knowledgebase_member_from_normalized_record(
    scope: &GroupKnowledgebaseScope,
    record: &ConversationMemberRecord,
) -> Result<ConversationMember, RuntimeError> {
    if record.tenant_id != scope.tenant_id
        || record.organization_id != scope.organization_id
        || record.conversation_id != scope.conversation_id
        || record.member_id <= 0
        || record.principal_id.trim().is_empty()
        || record.principal_kind.trim().is_empty()
        || record.principal_id.len() > GROUP_KNOWLEDGEBASE_MAX_MEMBER_ID_BYTES
        || record.principal_kind.len() > GROUP_KNOWLEDGEBASE_MAX_MEMBER_ID_BYTES
    {
        return Err(RuntimeError::Conflict(
            "group knowledgebase normalized member does not match its requested scope".into(),
        ));
    }
    let role = match record.membership_role.as_str() {
        "owner" => MembershipRole::Owner,
        "admin" => MembershipRole::Admin,
        "member" => MembershipRole::Member,
        "guest" => MembershipRole::Guest,
        _ => {
            return Err(RuntimeError::Conflict(
                "group knowledgebase normalized member role is invalid".into(),
            ));
        }
    };
    let state = match record.membership_state.as_str() {
        "joined" => MembershipState::Joined,
        "invited" => MembershipState::Invited,
        "linked" => MembershipState::Linked,
        "left" => MembershipState::Left,
        "removed" => MembershipState::Removed,
        _ => {
            return Err(RuntimeError::Conflict(
                "group knowledgebase normalized membership state is invalid".into(),
            ));
        }
    };
    let attributes = serde_json::from_str::<BTreeMap<String, String>>(
        record.attributes_json.as_str(),
    )
    .map_err(|_| {
        RuntimeError::Conflict(
            "group knowledgebase normalized member attributes are invalid".into(),
        )
    })?;
    Ok(ConversationMember {
        tenant_id: record.tenant_id.clone(),
        conversation_id: record.conversation_id.clone(),
        member_id: record.member_id.to_string(),
        principal_id: record.principal_id.clone(),
        principal_kind: record.principal_kind.clone(),
        role,
        state,
        invited_by: record.invited_by.clone(),
        joined_at: record.joined_at.clone(),
        removed_at: record.removed_at.clone(),
        attributes,
    })
}

fn group_knowledgebase_snapshot_members(
    snapshot: &GroupKnowledgebaseDurableSnapshot,
) -> Vec<GroupKnowledgebaseMembership> {
    let mut members = snapshot
        .roster
        .members()
        .values()
        .filter(|member| {
            member.is_active()
                && member.principal_kind == GROUP_KNOWLEDGEBASE_SUPPORTED_MEMBER_PRINCIPAL_KIND
        })
        .map(group_knowledgebase_membership_from_conversation_member)
        .collect::<Vec<_>>();
    members.sort_by(|left, right| {
        (left.principal_kind.as_str(), left.principal_id.as_str())
            .cmp(&(right.principal_kind.as_str(), right.principal_id.as_str()))
    });
    members
}

fn group_knowledgebase_membership_reconciliation_source_event_id(
    scope: &GroupKnowledgebaseScope,
    membership_epoch: u64,
) -> String {
    // Conversation identifiers can be valid at 256 bytes. Keep the derived
    // idempotency event below the cross-service input bound without weakening
    // its tenant/organization/conversation uniqueness fence.
    let scope_hash = sha256_hash(
        format!(
            "{}:{}:{}",
            scope.tenant_id, scope.organization_id, scope.conversation_id
        )
        .as_bytes(),
    );
    format!(
        "im.group-knowledgebase.members.reconcile:{}:{}",
        scope_hash, membership_epoch
    )
}

fn group_knowledgebase_archive_reconciliation_source_event_id(
    scope: &GroupKnowledgebaseScope,
    commit_seq: u64,
) -> String {
    let scope_hash = sha256_hash(
        format!(
            "{}:{}:{}",
            scope.tenant_id, scope.organization_id, scope.conversation_id
        )
        .as_bytes(),
    );
    format!(
        "im.group-knowledgebase.archive.reconcile:{}:{}",
        scope_hash, commit_seq
    )
}

pub(super) async fn run_group_knowledgebase_blocking<T, F>(
    operation: &'static str,
    work: F,
) -> Result<T, RuntimeError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, RuntimeError> + Send + 'static,
{
    tokio::task::spawn_blocking(work).await.map_err(|_| {
        RuntimeError::Contract(ContractError::Unavailable(format!(
            "group knowledgebase {operation} worker did not complete"
        )))
    })?
}

/// Initial provisioning is a group-owner action. The roster is the
/// authoritative ownership conversation_state: group creation assigns its creator the
/// owner role, and an explicit ownership transfer updates that role before a
/// later initialization attempt.
fn ensure_group_knowledgebase_owner(member: &ConversationMember) -> Result<(), RuntimeError> {
    if group_knowledgebase_owner(member) {
        return Ok(());
    }
    Err(RuntimeError::PermissionDenied(
        "only the group owner can initialize a group knowledgebase".into(),
    ))
}

fn group_knowledgebase_owner(member: &ConversationMember) -> bool {
    matches!(member.role, MembershipRole::Owner)
}

fn ensure_group_knowledgebase_access(member: &ConversationMember) -> Result<(), RuntimeError> {
    if matches!(member.role, MembershipRole::Guest) {
        return Err(RuntimeError::PermissionDenied(
            "group guests cannot access a group knowledgebase".into(),
        ));
    }
    Ok(())
}

fn group_knowledgebase_membership_from_conversation_member(
    member: &ConversationMember,
) -> GroupKnowledgebaseMembership {
    GroupKnowledgebaseMembership {
        principal_id: member.principal_id.clone(),
        principal_kind: member.principal_kind.clone(),
        role: member.role.clone(),
    }
}

/// Conversation does not own a mutable title, so the first Knowledgebase
/// provision receives a deterministic initial display name. The remote API
/// permits at most 256 bytes; a hash-only fallback keeps an otherwise valid
/// maximum-length conversation id from failing at the service boundary.
pub(super) fn group_knowledgebase_initial_group_name(conversation_id: &str) -> String {
    const PREFIX: &str = "Group ";
    if conversation_id.len() <= GROUP_KNOWLEDGEBASE_MAX_INITIAL_GROUP_NAME_BYTES - PREFIX.len() {
        return format!("{PREFIX}{conversation_id}");
    }
    format!("{PREFIX}{}", sha256_hash(conversation_id.as_bytes()))
}

fn group_knowledgebase_port_error_code(error: &GroupKnowledgebasePortError) -> &'static str {
    match error {
        GroupKnowledgebasePortError::Unavailable => "knowledgebase_unavailable",
        GroupKnowledgebasePortError::Conflict => "knowledgebase_conflict",
        GroupKnowledgebasePortError::Rejected => "knowledgebase_rejected",
    }
}

fn group_knowledgebase_port_error_runtime_error(
    error: GroupKnowledgebasePortError,
) -> RuntimeError {
    match error {
        GroupKnowledgebasePortError::Unavailable => RuntimeError::Contract(
            ContractError::Unavailable("group knowledgebase service is unavailable".into()),
        ),
        GroupKnowledgebasePortError::Conflict => {
            RuntimeError::Conflict("group knowledgebase provisioning conflicted".into())
        }
        GroupKnowledgebasePortError::Rejected => {
            RuntimeError::PermissionDenied("group knowledgebase provisioning was rejected".into())
        }
    }
}

fn resolve_group_knowledgebase_ticket_secret() -> Result<String, RuntimeError> {
    let secret = if let Ok(file_path) = std::env::var(GROUP_KNOWLEDGEBASE_TICKET_SECRET_FILE_ENV)
        && !file_path.trim().is_empty()
    {
        std::fs::read_to_string(file_path.trim())
            .map_err(|_| {
                RuntimeError::Contract(ContractError::Unavailable(format!(
                    "failed to read {GROUP_KNOWLEDGEBASE_TICKET_SECRET_FILE_ENV}"
                )))
            })?
            .trim()
            .to_owned()
    } else {
        std::env::var(GROUP_KNOWLEDGEBASE_TICKET_SECRET_ENV)
            .ok()
            .map(|value| value.trim().to_owned())
            .unwrap_or_default()
    };
    if secret.len() >= 32 {
        return Ok(secret);
    }
    if !im_app_context::allows_header_only_app_context_fallback() {
        return Err(RuntimeError::Contract(ContractError::Unavailable(format!(
            "{GROUP_KNOWLEDGEBASE_TICKET_SECRET_ENV} or {GROUP_KNOWLEDGEBASE_TICKET_SECRET_FILE_ENV} with at least 32 bytes is required in production"
        ))));
    }
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|_| {
        RuntimeError::Contract(ContractError::Unavailable(
            "local group knowledgebase ticket secret generation failed".into(),
        ))
    })?;
    tracing::warn!(
        secret_env = GROUP_KNOWLEDGEBASE_TICKET_SECRET_ENV,
        "using an ephemeral group knowledgebase launch-ticket encryption secret in development/test"
    );
    Ok(base64url_encode(bytes.as_slice()))
}

fn validate_group_knowledgebase_launch_idempotency_key(
    idempotency_key: &str,
) -> Result<(), RuntimeError> {
    let key = idempotency_key.trim();
    let valid = key == idempotency_key
        && (8..=128).contains(&key.len())
        && key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'));
    if valid {
        return Ok(());
    }
    Err(RuntimeError::InvalidInput(
        "group knowledgebase launch idempotency key is invalid".into(),
    ))
}

fn group_knowledgebase_launch_request_fingerprint(
    scope: &GroupKnowledgebaseScope,
    link: &GroupKnowledgebaseLink,
    auth: &AppContext,
) -> String {
    let session_id = auth.session_id.as_deref().unwrap_or_default();
    sha256_hash(
        format!(
            "launch:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            scope.tenant_id,
            scope.organization_id,
            scope.conversation_id,
            auth.actor_kind,
            auth.actor_id,
            auth.user_id,
            session_id,
            auth.app_id.as_deref().unwrap_or_default(),
            link.knowledge_space_id.unwrap_or_default(),
            link.knowledge_space_uuid.as_deref().unwrap_or_default(),
            link.version,
            link.membership_epoch,
        )
        .as_bytes(),
    )
}

/// A launch capability is delegated to one authenticated human session. This
/// keeps an otherwise valid opaque ticket from crossing a same-user session
/// boundary when the Knowledgebase resolver exchanges it with IM.
fn require_group_knowledgebase_ticket_session(auth: &AppContext) -> Result<&str, RuntimeError> {
    if auth.actor_kind != "user" || auth.actor_id != auth.user_id {
        return Err(RuntimeError::PermissionDenied(
            "group knowledgebase launch requires an authenticated user principal".into(),
        ));
    }
    let session_id = auth
        .session_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .filter(|value| value.trim() == *value)
        .filter(|value| value.len() <= 256)
        .filter(|value| value.bytes().all(|byte| byte.is_ascii_graphic()))
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(
                "group knowledgebase launch requires an authenticated session".into(),
            )
        })?;
    Ok(session_id)
}

fn ensure_group_knowledgebase_ticket_consumer_matches(
    ticket: &GroupKnowledgebaseLaunchTicket,
    auth: &AppContext,
) -> Result<(), RuntimeError> {
    ticket.scope.validate()?;
    let organization_id = resolve_group_knowledgebase_organization_id(auth)?;
    let session_id = require_group_knowledgebase_ticket_session(auth)?;
    if ticket.scope.tenant_id != auth.tenant_id
        || ticket.scope.organization_id != organization_id
        || ticket.actor_kind != auth.actor_kind
        || ticket.actor_id != auth.actor_id
        || ticket.principal_kind != auth.actor_kind
        || ticket.principal_id != auth.user_id
        || ticket.session_id != session_id
    {
        return Err(RuntimeError::PermissionDenied(
            "group knowledgebase launch denied".into(),
        ));
    }
    Ok(())
}

fn random_opaque_value(prefix: &str) -> Result<String, RuntimeError> {
    let mut bytes = [0u8; GROUP_KNOWLEDGEBASE_TICKET_BYTES];
    getrandom::fill(&mut bytes).map_err(|_| {
        RuntimeError::Contract(ContractError::Unavailable(
            "secure group knowledgebase token generation failed".into(),
        ))
    })?;
    Ok(format!("{prefix}{}", base64url_encode(bytes.as_slice())))
}

fn validate_launch_ticket(ticket: &str) -> Result<(), RuntimeError> {
    let ticket = ticket.trim();
    let encoded = ticket.strip_prefix("gklt_");
    if encoded.is_none_or(|encoded| {
        encoded.len() != 43
            || !encoded
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    }) {
        return Err(RuntimeError::PermissionDenied(
            "group knowledgebase launch denied".into(),
        ));
    }
    Ok(())
}

fn validate_ticket_consumer_trace_id(trace_id: &str) -> Result<(), RuntimeError> {
    let trace_id = trace_id.trim();
    if trace_id.is_empty()
        || trace_id.len() > 256
        || !trace_id
            .bytes()
            .all(|byte| byte.is_ascii_graphic() || byte == b' ')
    {
        return Err(RuntimeError::InvalidInput(
            "group knowledgebase ticket consumer trace is invalid".into(),
        ));
    }
    Ok(())
}

fn serialize_u64_as_decimal_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value.to_string().as_str())
}

fn lock_knowledgebase_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    label: &'static str,
) -> std::sync::MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned group knowledgebase mutex lock={label}");
            poisoned.into_inner()
        }
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn ensure_group_conversation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<(), RuntimeError> {
        let organization_id = organization_id_from_auth_context(auth);
        self.ensure_conversation_loaded(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
        )?;
        let scope_key = conversation_scope_key(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
        );
        let state = read_runtime_state(
            &self.state,
            "conversation-runtime.state.group-knowledgebase",
        );
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        if conversation.aggregate.conversation_type() != "group" {
            return Err(RuntimeError::ConversationTypeInvalid(
                "group knowledgebase requires a group conversation".into(),
            ));
        }
        Ok(())
    }

    /// Knowledgebase reads, creation, launch, and ticket consumption all use
    /// this stricter aggregate gate. An archived group must never regain KB
    /// access merely because an old link, ticket, or roster snapshot exists.
    pub fn ensure_active_group_conversation_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<(), RuntimeError> {
        self.ensure_group_conversation_from_auth_context(auth, conversation_id)?;
        let scope_key = conversation_scope_key(
            auth.tenant_id.as_str(),
            organization_id_from_auth_context(auth).as_str(),
            conversation_id,
        );
        let state = read_runtime_state(
            &self.state,
            "conversation-runtime.state.active-group-knowledgebase",
        );
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        if conversation.aggregate.is_archived() {
            return Err(RuntimeError::Conflict(
                "group conversation is archived and its knowledgebase is unavailable".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_platform_contracts::{ConversationMemberPage, ReadCursorPage, ReadCursorPageCursor};

    #[test]
    fn group_knowledgebase_signed_i64_persistence_rejects_overflow_and_negative_rows() {
        let signed_max = i64::MAX as u64;
        assert_eq!(
            group_knowledgebase_u64_to_db_i64(signed_max, "membership epoch")
                .expect("signed maximum should persist"),
            i64::MAX
        );
        assert!(
            group_knowledgebase_u64_to_db_i64(signed_max + 1, "membership epoch").is_err(),
            "values above signed BIGINT must be rejected before persistence"
        );
        assert_eq!(
            group_knowledgebase_db_i64_to_u64(i64::MAX, "membership epoch")
                .expect("signed maximum should decode"),
            signed_max
        );
        assert!(
            group_knowledgebase_db_i64_to_u64(-1, "membership epoch").is_err(),
            "negative persisted epochs must not be silently coerced"
        );
        assert!(
            validate_group_knowledgebase_persisted_positive_i64(0, "link generation").is_err(),
            "link generation zero must not be silently coerced"
        );
    }

    #[test]
    fn group_knowledgebase_link_generation_fails_when_signed_i64_range_is_exhausted() {
        let signed_max = i64::MAX as u64;
        assert_eq!(
            next_group_knowledgebase_version(signed_max - 1)
                .expect("last valid increment should remain representable"),
            signed_max
        );
        assert!(
            next_group_knowledgebase_version(signed_max).is_err(),
            "link generation must not saturate at the signed persistence limit"
        );
    }

    #[test]
    fn group_knowledgebase_launch_waits_for_membership_sync_before_issuing_a_ticket() {
        let scope = test_scope("g-first-admin-launch");
        let store = Arc::new(InMemoryGroupKnowledgebaseStore::default());
        let port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
            GroupKnowledgebaseArchiveDeliveryState::Archived,
        ));
        let coordinator = GroupKnowledgebaseCoordinator {
            store: store.clone(),
            port,
            id_generator: sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "conversation-knowledgebase-launch-convergence-test",
            ),
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::for_test(),
        };
        let mut link = active_test_link(scope.clone(), 1);
        link.last_synchronized_membership_epoch = 0;
        let mut auth =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        auth.organization_id = scope.organization_id.clone();
        auth.session_id = Some("session-first-admin-launch".into());

        let first = coordinator
            .issue_launch_ticket(scope.clone(), link.clone(), &auth, "first-admin-launch-1")
            .expect("unsynchronized launch should be handled");
        assert!(matches!(
            first,
            GroupKnowledgebaseLaunchResult::Provisioning(_)
        ));

        link.last_synchronized_membership_epoch = link.membership_epoch;
        let second = coordinator
            .issue_launch_ticket(scope, link, &auth, "first-admin-launch-2")
            .expect("synchronized launch should issue a ticket");
        let GroupKnowledgebaseLaunchResult::Ready(ready) = second else {
            panic!("membership convergence must permit one-time ticket issuance");
        };
        assert!(validate_launch_ticket(ready.launch_ticket.as_str()).is_ok());
    }

    #[tokio::test]
    async fn group_knowledgebase_first_owner_launch_provisions_synchronizes_and_then_issues_ticket()
    {
        let scope = test_scope("g-first-owner-lifecycle");
        let runtime = Arc::new(normalized_group_runtime(
            &scope,
            "active",
            1,
            1,
            vec![test_member(&scope, "m-owner", "42", MembershipRole::Owner)],
        ));
        let store = Arc::new(InMemoryGroupKnowledgebaseStore::default());
        let port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
            GroupKnowledgebaseArchiveDeliveryState::Archived,
        ));
        let coordinator = GroupKnowledgebaseCoordinator {
            store: store.clone(),
            port: port.clone(),
            id_generator: sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "conversation-knowledgebase-first-owner-lifecycle-test",
            ),
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::for_test(),
        };
        let mut auth =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        auth.organization_id = scope.organization_id.clone();
        auth.session_id = Some("session-first-owner-lifecycle".into());

        let first = coordinator
            .launch(
                runtime.clone(),
                auth.clone(),
                scope.conversation_id.clone(),
                "first-owner-launch-1".into(),
            )
            .await
            .expect("the first owner launch should provision the group knowledgebase");
        let GroupKnowledgebaseLaunchResult::Provisioning(first_view) = first else {
            panic!("the first launch must wait for durable ACL synchronization");
        };
        assert_eq!(
            first_view.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Active
        );
        assert_eq!(first_view.conversation_id, scope.conversation_id);

        {
            let ensure_requests = lock_knowledgebase_mutex(
                &port.ensure_requests,
                "recording-knowledgebase-ensure-requests",
            );
            assert_eq!(ensure_requests.len(), 1);
            assert_eq!(ensure_requests[0].scope, scope);
            assert_eq!(ensure_requests[0].membership_epoch, 1);
            assert_eq!(ensure_requests[0].members.len(), 1);
            assert_eq!(ensure_requests[0].members[0].principal_id, "42");
        }

        let reserved_link = store
            .get_link(&scope)
            .expect("the local group knowledgebase link should be readable")
            .expect("the first launch should reserve exactly one local link");
        assert_eq!(
            reserved_link.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Active
        );
        assert!(reserved_link.link_uuid.starts_with("gkl_"));
        assert!(
            reserved_link
                .creation_idempotency_key
                .starts_with("im-group-knowledgebase:")
        );
        assert_eq!(reserved_link.membership_epoch, 1);
        assert_eq!(reserved_link.last_synchronized_membership_epoch, 0);

        let pending_payloads = store.pending_outbox_payloads();
        assert_eq!(pending_payloads.len(), 1);
        let synchronization_payload = pending_payloads
            .into_iter()
            .next()
            .expect("the initial ACL synchronization payload should exist");
        assert_eq!(
            synchronization_payload.operation,
            GroupKnowledgebaseOutboxOperation::SynchronizeMembers
        );
        assert_eq!(synchronization_payload.scope, scope);
        assert_eq!(
            synchronization_payload.upstream_link_generation,
            reserved_link.version
        );
        assert_eq!(synchronization_payload.membership_epoch, 1);
        assert_eq!(synchronization_payload.members.len(), 1);

        coordinator
            .deliver_outbox_payload(synchronization_payload)
            .await
            .expect(
                "the generated Knowledgebase port should accept the initial ACL synchronization",
            );
        {
            let synchronization_requests = lock_knowledgebase_mutex(
                &port.synchronization_requests,
                "recording-knowledgebase-sync-requests",
            );
            assert_eq!(synchronization_requests.len(), 1);
            assert_eq!(synchronization_requests[0].scope, scope);
            assert_eq!(synchronization_requests[0].membership_epoch, 1);
            assert_eq!(synchronization_requests[0].members.len(), 1);
        }

        let synchronized_link = store
            .get_link(&scope)
            .expect("the synchronized local link should be readable")
            .expect("the synchronized local link should remain present");
        assert_eq!(
            synchronized_link.last_synchronized_membership_epoch,
            synchronized_link.membership_epoch
        );

        let second = coordinator
            .launch(
                runtime,
                auth,
                scope.conversation_id.clone(),
                "first-owner-launch-2".into(),
            )
            .await
            .expect("the second owner launch should issue a ticket after ACL convergence");
        let GroupKnowledgebaseLaunchResult::Ready(ready) = second else {
            panic!("a synchronized group knowledgebase must issue a launch ticket");
        };
        assert_eq!(ready.conversation_id, scope.conversation_id);
        assert_eq!(ready.membership_epoch, synchronized_link.membership_epoch);
        assert_eq!(ready.upstream_link_generation, synchronized_link.version);
        assert!(validate_launch_ticket(ready.launch_ticket.as_str()).is_ok());
    }

    #[tokio::test]
    async fn group_knowledgebase_initialization_allows_owner_and_denies_admin_or_member() {
        let scope = test_scope("g-owner-only-initialization");
        let runtime = Arc::new(normalized_group_runtime(
            &scope,
            "active",
            3,
            3,
            vec![
                test_member(&scope, "m-owner", "42", MembershipRole::Owner),
                test_member(&scope, "m-admin", "43", MembershipRole::Admin),
                test_member(&scope, "m-member", "44", MembershipRole::Member),
            ],
        ));
        let store = Arc::new(InMemoryGroupKnowledgebaseStore::default());
        let port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
            GroupKnowledgebaseArchiveDeliveryState::Archived,
        ));
        let coordinator = GroupKnowledgebaseCoordinator {
            store: store.clone(),
            port: port.clone(),
            id_generator: sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "conversation-knowledgebase-owner-only-initialization-test",
            ),
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::for_test(),
        };

        for (actor_id, idempotency_key) in [
            ("43", "admin-cannot-initialize"),
            ("44", "member-cannot-initialize"),
        ] {
            let mut auth =
                im_app_context::local_service_app_context("100001", actor_id, "user", None, ["*"]);
            auth.organization_id = scope.organization_id.clone();
            auth.session_id = Some(format!("session-{actor_id}"));

            let error = coordinator
                .launch(
                    runtime.clone(),
                    auth,
                    scope.conversation_id.clone(),
                    idempotency_key.into(),
                )
                .await
                .expect_err("a non-owner must not initialize the group knowledgebase");
            assert!(matches!(error, RuntimeError::PermissionDenied(_)));
        }

        assert!(
            store
                .get_link(&scope)
                .expect("link lookup should succeed")
                .is_none(),
            "a denied initialization must not reserve a local link"
        );
        assert!(
            lock_knowledgebase_mutex(
                &port.ensure_requests,
                "recording-knowledgebase-ensure-requests",
            )
            .is_empty(),
            "a denied initialization must not call Knowledgebase"
        );

        let mut owner_auth =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        owner_auth.organization_id = scope.organization_id.clone();
        owner_auth.session_id = Some("session-owner".into());
        let result = coordinator
            .launch(
                runtime,
                owner_auth,
                scope.conversation_id.clone(),
                "owner-can-initialize".into(),
            )
            .await
            .expect("the group owner should initialize the group knowledgebase");
        assert!(matches!(
            result,
            GroupKnowledgebaseLaunchResult::Provisioning(_)
        ));
        assert_eq!(
            lock_knowledgebase_mutex(
                &port.ensure_requests,
                "recording-knowledgebase-ensure-requests",
            )
            .len(),
            1,
            "only the owner initiation should reach Knowledgebase"
        );
    }

    #[tokio::test]
    async fn group_knowledgebase_active_launch_remains_available_to_a_non_owner_member() {
        let scope = test_scope("g-active-member-launch");
        let runtime = Arc::new(normalized_group_runtime(
            &scope,
            "active",
            2,
            2,
            vec![
                test_member(&scope, "m-owner", "42", MembershipRole::Owner),
                test_member(&scope, "m-member", "44", MembershipRole::Member),
            ],
        ));
        let store = Arc::new(InMemoryGroupKnowledgebaseStore::default());
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links")
            .insert(scope.clone(), active_test_link(scope.clone(), 2));
        let port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
            GroupKnowledgebaseArchiveDeliveryState::Archived,
        ));
        let coordinator = GroupKnowledgebaseCoordinator {
            store,
            port: port.clone(),
            id_generator: sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "conversation-knowledgebase-active-member-launch-test",
            ),
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::for_test(),
        };
        let mut member_auth =
            im_app_context::local_service_app_context("100001", "44", "user", None, ["*"]);
        member_auth.organization_id = scope.organization_id.clone();
        member_auth.session_id = Some("session-member".into());

        let result = coordinator
            .launch(
                runtime,
                member_auth,
                scope.conversation_id.clone(),
                "member-can-open-active-knowledgebase".into(),
            )
            .await
            .expect("an active group knowledgebase should remain launchable by a member");
        let GroupKnowledgebaseLaunchResult::Ready(ready) = result else {
            panic!("an active, synchronized group knowledgebase must issue a ticket");
        };
        assert_eq!(ready.conversation_id, scope.conversation_id);
        assert!(validate_launch_ticket(ready.launch_ticket.as_str()).is_ok());
        assert!(
            lock_knowledgebase_mutex(
                &port.ensure_requests,
                "recording-knowledgebase-ensure-requests",
            )
            .is_empty(),
            "opening an active group knowledgebase must not invoke initialization"
        );
    }

    struct RecordingGroupKnowledgebasePort {
        archive_state: GroupKnowledgebaseArchiveDeliveryState,
        archive_requests: Mutex<Vec<ArchiveGroupKnowledgebaseRequest>>,
        ensure_requests: Mutex<Vec<EnsureGroupKnowledgebaseRequest>>,
        synchronization_requests: Mutex<Vec<SynchronizeGroupKnowledgebaseMembersRequest>>,
    }

    impl RecordingGroupKnowledgebasePort {
        fn with_archive_state(archive_state: GroupKnowledgebaseArchiveDeliveryState) -> Self {
            Self {
                archive_state,
                archive_requests: Mutex::new(Vec::new()),
                ensure_requests: Mutex::new(Vec::new()),
                synchronization_requests: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl GroupKnowledgebasePort for RecordingGroupKnowledgebasePort {
        async fn ensure_delivery_ready(&self) -> Result<(), GroupKnowledgebasePortError> {
            Ok(())
        }

        async fn ensure_group_knowledgebase(
            &self,
            request: EnsureGroupKnowledgebaseRequest,
        ) -> Result<EnsuredGroupKnowledgebase, GroupKnowledgebasePortError> {
            let membership_epoch = request.membership_epoch;
            lock_knowledgebase_mutex(
                &self.ensure_requests,
                "recording-knowledgebase-ensure-requests",
            )
            .push(request);
            Ok(EnsuredGroupKnowledgebase {
                knowledge_space_id: 701,
                knowledge_space_uuid: "space-701".into(),
                knowledgebase_binding_id: 801,
                knowledgebase_binding_uuid: "binding-801".into(),
                provisioning_operation_id: None,
                membership_epoch,
            })
        }

        async fn synchronize_group_members(
            &self,
            request: SynchronizeGroupKnowledgebaseMembersRequest,
        ) -> Result<(), GroupKnowledgebasePortError> {
            lock_knowledgebase_mutex(
                &self.synchronization_requests,
                "recording-knowledgebase-sync-requests",
            )
            .push(request);
            Ok(())
        }

        async fn archive_group_knowledgebase(
            &self,
            request: ArchiveGroupKnowledgebaseRequest,
        ) -> Result<GroupKnowledgebaseArchiveDeliveryState, GroupKnowledgebasePortError> {
            lock_knowledgebase_mutex(
                &self.archive_requests,
                "recording-knowledgebase-archive-requests",
            )
            .push(request);
            Ok(self.archive_state)
        }
    }

    fn test_scope(conversation_id: &str) -> GroupKnowledgebaseScope {
        GroupKnowledgebaseScope {
            tenant_id: "100001".into(),
            organization_id: "200001".into(),
            conversation_id: conversation_id.into(),
        }
    }

    fn test_member(
        scope: &GroupKnowledgebaseScope,
        member_id: &str,
        principal_id: &str,
        role: MembershipRole,
    ) -> ConversationMember {
        ConversationMember {
            tenant_id: scope.tenant_id.clone(),
            conversation_id: scope.conversation_id.clone(),
            member_id: member_id.into(),
            principal_id: principal_id.into(),
            principal_kind: "user".into(),
            role,
            state: im_domain_core::conversation::MembershipState::Joined,
            invited_by: None,
            joined_at: "2026-07-13T00:00:00Z".into(),
            removed_at: None,
            attributes: BTreeMap::new(),
        }
    }

    struct NormalizedGroupConversationStore {
        conversation: NormalizedConversationRecord,
        members: Vec<ConversationMemberRecord>,
    }

    impl ConversationAggregateStore for NormalizedGroupConversationStore {
        fn load_conversation(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<Option<NormalizedConversationRecord>, ContractError> {
            Ok(Some(self.conversation.clone()))
        }

        fn load_members_page(
            &self,
            tenant_id: &str,
            organization_id: &str,
            conversation_id: &str,
            cursor: Option<&ConversationMemberPageCursor>,
            page_size: usize,
        ) -> Result<ConversationMemberPage, ContractError> {
            let mut items = self
                .members
                .iter()
                .filter(|member| {
                    member.tenant_id == tenant_id
                        && member.organization_id == organization_id
                        && member.conversation_id == conversation_id
                        && cursor.is_none_or(|cursor| {
                            (member.principal_kind.as_str(), member.principal_id.as_str())
                                > (cursor.principal_kind.as_str(), cursor.principal_id.as_str())
                        })
                })
                .cloned()
                .collect::<Vec<_>>();
            items.sort_by(|left, right| {
                (&left.principal_kind, &left.principal_id)
                    .cmp(&(&right.principal_kind, &right.principal_id))
            });
            let has_more = items.len() > page_size;
            items.truncate(page_size);
            let next_cursor = has_more.then(|| {
                let last = items
                    .last()
                    .expect("a paged normalized member result must contain one row");
                ConversationMemberPageCursor {
                    principal_kind: last.principal_kind.clone(),
                    principal_id: last.principal_id.clone(),
                }
            });
            Ok(ConversationMemberPage {
                items,
                next_cursor,
                has_more,
            })
        }

        fn load_member(
            &self,
            tenant_id: &str,
            organization_id: &str,
            conversation_id: &str,
            principal_kind: &str,
            principal_id: &str,
        ) -> Result<Option<ConversationMemberRecord>, ContractError> {
            Ok(self
                .members
                .iter()
                .find(|member| {
                    member.tenant_id == tenant_id
                        && member.organization_id == organization_id
                        && member.conversation_id == conversation_id
                        && member.principal_kind == principal_kind
                        && member.principal_id == principal_id
                })
                .cloned())
        }

        fn load_member_by_id(
            &self,
            tenant_id: &str,
            organization_id: &str,
            conversation_id: &str,
            member_id: i64,
        ) -> Result<Option<ConversationMemberRecord>, ContractError> {
            Ok(self
                .members
                .iter()
                .find(|member| {
                    member.tenant_id == tenant_id
                        && member.organization_id == organization_id
                        && member.conversation_id == conversation_id
                        && member.member_id == member_id
                })
                .cloned())
        }

        fn load_event_recipients_page(
            &self,
            tenant_id: &str,
            organization_id: &str,
            conversation_id: &str,
            _joined_before_or_at: &str,
            cursor: Option<&ConversationMemberPageCursor>,
            page_size: usize,
        ) -> Result<ConversationMemberPage, ContractError> {
            self.load_members_page(
                tenant_id,
                organization_id,
                conversation_id,
                cursor,
                page_size,
            )
        }

        fn upsert_member(&self, _member: ConversationMemberRecord) -> Result<(), ContractError> {
            normalized_group_test_store_unsupported()
        }

        fn remove_member(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
            _removed_at: &str,
        ) -> Result<(), ContractError> {
            normalized_group_test_store_unsupported()
        }

        fn load_read_cursors_page(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _cursor: Option<&ReadCursorPageCursor>,
            _page_size: usize,
        ) -> Result<ReadCursorPage, ContractError> {
            Ok(ReadCursorPage {
                items: Vec::new(),
                next_cursor: None,
                has_more: false,
            })
        }

        fn load_read_cursor(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _member_id: i64,
        ) -> Result<Option<ReadCursorRecord>, ContractError> {
            Ok(None)
        }

        fn upsert_read_cursor(&self, _cursor: ReadCursorRecord) -> Result<(), ContractError> {
            normalized_group_test_store_unsupported()
        }

        fn load_high_watermark(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<u64, ContractError> {
            Ok(0)
        }

        fn allocate_member_id(&self) -> Result<i64, ContractError> {
            normalized_group_test_store_unsupported()
        }

        fn conversation_exists(
            &self,
            tenant_id: &str,
            organization_id: &str,
            conversation_id: &str,
        ) -> Result<bool, ContractError> {
            Ok(self.conversation.tenant_id == tenant_id
                && self.conversation.organization_id == organization_id
                && self.conversation.conversation_id == conversation_id)
        }
    }

    fn normalized_group_test_store_unsupported<T>() -> Result<T, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "operation is not used by normalized group knowledgebase tests".into(),
        ))
    }

    fn normalized_group_runtime(
        scope: &GroupKnowledgebaseScope,
        lifecycle_state: &str,
        commit_seq: u64,
        membership_epoch: u64,
        members: Vec<ConversationMember>,
    ) -> ConversationRuntime<ConversationCommitJournal> {
        let records = members
            .into_iter()
            .enumerate()
            .map(|(index, member)| ConversationMemberRecord {
                tenant_id: scope.tenant_id.clone(),
                organization_id: scope.organization_id.clone(),
                conversation_id: scope.conversation_id.clone(),
                principal_kind: member.principal_kind,
                principal_id: member.principal_id,
                member_id: index as i64 + 1,
                membership_role: match member.role {
                    MembershipRole::Owner => "owner",
                    MembershipRole::Admin => "admin",
                    MembershipRole::Member => "member",
                    MembershipRole::Guest => "guest",
                }
                .into(),
                membership_state: match member.state {
                    MembershipState::Joined => "joined",
                    MembershipState::Invited => "invited",
                    MembershipState::Linked => "linked",
                    MembershipState::Left => "left",
                    MembershipState::Removed => "removed",
                }
                .into(),
                invited_by: member.invited_by,
                joined_at: member.joined_at,
                removed_at: member.removed_at,
                attributes_json: serde_json::to_string(&member.attributes)
                    .expect("test member attributes should serialize"),
            })
            .collect();
        normalized_group_runtime_with_store(NormalizedGroupConversationStore {
            conversation: NormalizedConversationRecord {
                tenant_id: scope.tenant_id.clone(),
                organization_id: scope.organization_id.clone(),
                conversation_id: scope.conversation_id.clone(),
                conversation_type: "group".into(),
                lifecycle_state: lifecycle_state.into(),
                archived_at: (lifecycle_state == "archived").then(|| "2026-07-13T00:00:00Z".into()),
                archive_event_id: (lifecycle_state == "archived")
                    .then(|| "evt_normalized_group_archived".into()),
                commit_seq,
                member_epoch: membership_epoch,
                last_activity_at: "2026-07-13T00:00:00Z".into(),
                retention_until: None,
            },
            members: records,
        })
    }

    fn normalized_group_runtime_with_store(
        store: NormalizedGroupConversationStore,
    ) -> ConversationRuntime<ConversationCommitJournal> {
        ConversationRuntime::new(ConversationCommitJournal::Memory(InMemoryJournal::default()))
            .with_aggregate_store(Arc::new(store))
    }

    fn active_test_link(
        scope: GroupKnowledgebaseScope,
        membership_epoch: u64,
    ) -> GroupKnowledgebaseLink {
        let mut link = GroupKnowledgebaseLink::new(
            11,
            "link-reconciliation".into(),
            scope,
            "42".into(),
            Utc::now(),
        );
        link.lifecycle_state = GroupKnowledgebaseLifecycleState::Active;
        link.knowledge_space_id = Some(101);
        link.knowledge_space_uuid = Some("space-101".into());
        link.knowledgebase_binding_id = Some(201);
        link.knowledgebase_binding_uuid = Some("binding-201".into());
        link.membership_epoch = membership_epoch;
        link.last_synchronized_membership_epoch = membership_epoch;
        link
    }

    fn archive_test_payload(archived_by: Option<String>) -> GroupKnowledgebaseOutboxPayload {
        GroupKnowledgebaseOutboxPayload {
            operation: GroupKnowledgebaseOutboxOperation::Archive,
            source_event_id: "evt-archive-outbox".into(),
            scope: test_scope("g-archive-outbox"),
            knowledge_space_id: 101,
            knowledge_space_uuid: "space-101".into(),
            knowledgebase_binding_id: 201,
            knowledgebase_binding_uuid: "binding-201".into(),
            upstream_link_generation: 7,
            membership_epoch: 3,
            members: Vec::new(),
            archived_by,
        }
    }

    fn test_launch_ticket(
        scope: GroupKnowledgebaseScope,
        link: &GroupKnowledgebaseLink,
        knowledgebase_binding_uuid: &str,
    ) -> GroupKnowledgebaseLaunchTicket {
        GroupKnowledgebaseLaunchTicket {
            id: 1,
            ticket_hash: sha256_hash(b"gklt_ticket-currentness-test"),
            scope,
            knowledge_space_id: link.knowledge_space_id.expect("active test link space id"),
            knowledge_space_uuid: link
                .knowledge_space_uuid
                .clone()
                .expect("active test link space uuid"),
            knowledgebase_binding_id: link
                .knowledgebase_binding_id
                .expect("active test link binding id"),
            knowledgebase_binding_uuid: knowledgebase_binding_uuid.into(),
            upstream_link_generation: link.version,
            membership_epoch: link.membership_epoch,
            actor_kind: "user".into(),
            actor_id: "42".into(),
            principal_kind: "user".into(),
            principal_id: "42".into(),
            session_id: "session-a".into(),
            issuing_app_id: Some("sdkwork-im".into()),
            issued_by: "42".into(),
            idempotency_key_hash: sha256_hash(b"ticket-currentness"),
            request_fingerprint_hash: sha256_hash(b"ticket-currentness-request"),
            ticket_ciphertext: "not-used-by-currentness-test".into(),
            expires_at: Utc::now() + Duration::seconds(60),
            consumed_at: None,
            consumed_by_service: None,
            consumed_trace_id: None,
        }
    }

    fn reconciliation_coordinator(
        store: Arc<InMemoryGroupKnowledgebaseStore>,
    ) -> GroupKnowledgebaseCoordinator {
        GroupKnowledgebaseCoordinator {
            store,
            port: Arc::new(UnavailableGroupKnowledgebasePort),
            id_generator: sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "conversation-knowledgebase-reconciliation-test",
            ),
            launch_ticket_cipher: GroupKnowledgebaseLaunchTicketCipher::for_test(),
        }
    }

    #[test]
    fn launch_ticket_format_is_opaque_and_bounded() {
        let value = random_opaque_value("gklt_").expect("ticket should generate");
        assert!(value.starts_with("gklt_"));
        assert!(value.len() <= 256);
        assert!(validate_launch_ticket(value.as_str()).is_ok());
    }

    #[test]
    fn ticket_validation_rejects_non_opaque_input() {
        assert!(validate_launch_ticket("42").is_err());
        assert!(validate_launch_ticket("gklt_short").is_err());
    }

    #[test]
    fn launch_ticket_requires_a_nonblank_user_session() {
        let mut auth =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        auth.session_id = None;
        assert!(require_group_knowledgebase_ticket_session(&auth).is_err());

        auth.session_id = Some(" ".into());
        assert!(require_group_knowledgebase_ticket_session(&auth).is_err());

        auth.session_id = Some("session-42".into());
        assert_eq!(
            require_group_knowledgebase_ticket_session(&auth).expect("valid session"),
            "session-42"
        );
    }

    #[test]
    fn group_knowledgebase_accepts_token_derived_tenant_and_organization_scopes() {
        let mut auth =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        let tenant_scope = GroupKnowledgebaseScope::from_auth_context(&auth, "g-tenant-group")
            .expect("tenant-scoped group knowledgebase should be valid");
        assert_eq!(tenant_scope.organization_id, "0");

        auth.organization_id = "200001".into();
        let scope = GroupKnowledgebaseScope::from_auth_context(&auth, "g-organization-bound")
            .expect("organization-scoped group knowledgebase should be valid");
        assert_eq!(scope.organization_id, "200001");

        auth.organization_id = "org-200001".into();
        let error = GroupKnowledgebaseScope::from_auth_context(&auth, "g-organization-bound")
            .expect_err("opaque organization IDs must not cross into Knowledgebase");
        assert!(matches!(error, RuntimeError::PermissionDenied(_)));
    }

    #[test]
    fn group_knowledgebase_organization_scope_uses_the_signed_i64_cross_service_boundary() {
        assert!(validate_group_knowledgebase_organization_id("0").is_ok());
        assert!(validate_group_knowledgebase_organization_id("9223372036854775807").is_ok());
        assert!(matches!(
            validate_group_knowledgebase_organization_id("9223372036854775808"),
            Err(RuntimeError::PermissionDenied(_))
        ));
        assert!(matches!(
            validate_group_knowledgebase_organization_id(" 1 "),
            Err(RuntimeError::PermissionDenied(_))
        ));
    }

    #[test]
    fn group_knowledgebase_identifier_limits_are_enforced_before_cross_service_delivery() {
        let conversation_id = "c".repeat(GROUP_KNOWLEDGEBASE_MAX_CONVERSATION_ID_BYTES);
        assert!(test_scope(conversation_id.as_str()).validate().is_ok());
        assert!(matches!(
            test_scope(
                "c".repeat(GROUP_KNOWLEDGEBASE_MAX_CONVERSATION_ID_BYTES + 1)
                    .as_str()
            )
            .validate(),
            Err(RuntimeError::InvalidInput(_))
        ));

        assert!(
            validate_group_knowledgebase_source_event_id(
                "e".repeat(GROUP_KNOWLEDGEBASE_MAX_SOURCE_EVENT_ID_BYTES)
                    .as_str()
            )
            .is_ok()
        );
        assert!(matches!(
            validate_group_knowledgebase_source_event_id(
                "e".repeat(GROUP_KNOWLEDGEBASE_MAX_SOURCE_EVENT_ID_BYTES + 1)
                    .as_str()
            ),
            Err(RuntimeError::InvalidInput(_))
        ));

        assert!(
            validate_group_knowledgebase_actor_id(
                "a".repeat(GROUP_KNOWLEDGEBASE_MAX_ACTOR_ID_BYTES).as_str()
            )
            .is_ok()
        );
        assert!(matches!(
            validate_group_knowledgebase_actor_id(
                "a".repeat(GROUP_KNOWLEDGEBASE_MAX_ACTOR_ID_BYTES + 1)
                    .as_str()
            ),
            Err(RuntimeError::InvalidInput(_))
        ));

        let member = GroupKnowledgebaseMembership {
            principal_id: "m".repeat(GROUP_KNOWLEDGEBASE_MAX_MEMBER_ID_BYTES),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
        };
        assert!(member.validate().is_ok());
        let oversized_member = GroupKnowledgebaseMembership {
            principal_id: "m".repeat(GROUP_KNOWLEDGEBASE_MAX_MEMBER_ID_BYTES + 1),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
        };
        assert!(matches!(
            oversized_member.validate(),
            Err(RuntimeError::InvalidInput(_))
        ));
        let unsupported_member = GroupKnowledgebaseMembership {
            principal_id: "agent-1".into(),
            principal_kind: "agent".into(),
            role: MembershipRole::Member,
        };
        assert!(matches!(
            unsupported_member.validate(),
            Err(RuntimeError::InvalidInput(_))
        ));

        let maximum_name = group_knowledgebase_initial_group_name(conversation_id.as_str());
        assert!(maximum_name.len() <= GROUP_KNOWLEDGEBASE_MAX_INITIAL_GROUP_NAME_BYTES);
        assert_eq!(
            group_knowledgebase_initial_group_name("group-1"),
            "Group group-1"
        );
    }

    #[tokio::test]
    async fn archive_delivery_rejects_missing_blank_and_oversized_actors_before_port_invocation() {
        for archived_by in [
            None,
            Some(" ".to_owned()),
            Some("a".repeat(GROUP_KNOWLEDGEBASE_MAX_ACTOR_ID_BYTES + 1)),
        ] {
            let port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
                GroupKnowledgebaseArchiveDeliveryState::Archived,
            ));
            let coordinator = GroupKnowledgebaseCoordinator::with_memory_store(port.clone());
            let error = coordinator
                .deliver_outbox_payload(archive_test_payload(archived_by))
                .await
                .expect_err("an invalid archive actor must not cross the port boundary");
            assert!(matches!(
                error,
                RuntimeError::Conflict(_) | RuntimeError::InvalidInput(_)
            ));
            assert!(
                lock_knowledgebase_mutex(
                    &port.archive_requests,
                    "recording-knowledgebase-archive-requests",
                )
                .is_empty()
            );
        }
    }

    #[tokio::test]
    async fn archive_delivery_preserves_its_generation_and_retries_until_a_terminal_state() {
        let archiving_port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
            GroupKnowledgebaseArchiveDeliveryState::Archiving,
        ));
        let archiving_coordinator =
            GroupKnowledgebaseCoordinator::with_memory_store(archiving_port.clone());
        let error = archiving_coordinator
            .deliver_outbox_payload(archive_test_payload(Some("archive-owner".into())))
            .await
            .expect_err("an archiving response must retain the durable retry");
        assert!(matches!(
            error,
            RuntimeError::Contract(ContractError::Unavailable(_))
        ));
        {
            let archiving_requests = lock_knowledgebase_mutex(
                &archiving_port.archive_requests,
                "recording-knowledgebase-archive-requests",
            );
            assert_eq!(archiving_requests.len(), 1);
            assert_eq!(archiving_requests[0].membership_epoch, 3);
            assert_eq!(archiving_requests[0].upstream_link_generation, 7);
        }

        let archived_port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
            GroupKnowledgebaseArchiveDeliveryState::Archived,
        ));
        let archived_coordinator =
            GroupKnowledgebaseCoordinator::with_memory_store(archived_port.clone());
        archived_coordinator
            .deliver_outbox_payload(archive_test_payload(Some("archive-owner".into())))
            .await
            .expect("a terminal archived response should complete delivery");
        assert_eq!(
            lock_knowledgebase_mutex(
                &archived_port.archive_requests,
                "recording-knowledgebase-archive-requests",
            )
            .len(),
            1
        );
    }

    #[tokio::test]
    async fn outbox_delivery_rejects_an_incomplete_immutable_target_before_port_invocation() {
        let port = Arc::new(RecordingGroupKnowledgebasePort::with_archive_state(
            GroupKnowledgebaseArchiveDeliveryState::Archived,
        ));
        let coordinator = GroupKnowledgebaseCoordinator::with_memory_store(port.clone());
        let mut payload = archive_test_payload(Some("archive-owner".into()));
        payload.knowledgebase_binding_uuid = " ".into();
        let error = coordinator
            .deliver_outbox_payload(payload)
            .await
            .expect_err("a blank target UUID must fail before delivery");
        assert!(matches!(error, RuntimeError::InvalidInput(_)));
        assert!(
            lock_knowledgebase_mutex(
                &port.archive_requests,
                "recording-knowledgebase-archive-requests",
            )
            .is_empty()
        );
    }

    #[tokio::test]
    async fn group_knowledgebase_accepts_a_tenant_scoped_outbox_payload() {
        let coordinator = GroupKnowledgebaseCoordinator::with_memory_store(Arc::new(
            UnavailableGroupKnowledgebasePort,
        ));
        let error = coordinator
            .deliver_outbox_payload(GroupKnowledgebaseOutboxPayload {
                operation: GroupKnowledgebaseOutboxOperation::SynchronizeMembers,
                source_event_id: "group-kb-tenant-scope-sync".into(),
                scope: GroupKnowledgebaseScope {
                    tenant_id: "100001".into(),
                    organization_id: "0".into(),
                    conversation_id: "g-tenant-scope".into(),
                },
                knowledge_space_id: 101,
                knowledge_space_uuid: "space-tenant-scope".into(),
                knowledgebase_binding_id: 201,
                knowledgebase_binding_uuid: "binding-tenant-scope".into(),
                upstream_link_generation: 1,
                membership_epoch: 1,
                members: Vec::new(),
                archived_by: None,
            })
            .await
            .expect_err("the unavailable port should receive the valid tenant-scoped payload");
        assert!(matches!(error, RuntimeError::Contract(_)));
    }

    #[test]
    fn trusted_ticket_consume_rejects_noncanonical_delegated_organization_contexts() {
        let coordinator = GroupKnowledgebaseCoordinator::with_memory_store(Arc::new(
            UnavailableGroupKnowledgebasePort,
        ));
        let runtime =
            ConversationRuntime::new(ConversationCommitJournal::Memory(InMemoryJournal::default()));
        let mut auth =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        auth.session_id = Some("session-malformed-organization".into());

        for organization_id in ["0200001", "org-200001"] {
            auth.organization_id = organization_id.into();
            let ticket = random_opaque_value("gklt_").expect("valid ticket");
            let error = coordinator
                .consume_launch_ticket_from_trusted_knowledgebase(
                    &runtime,
                    &auth,
                    ticket.as_str(),
                    "trace-noncanonical-organization",
                )
                .expect_err(
                    "noncanonical delegated organization scope must be rejected before ticket lookup",
                );
            assert!(matches!(error, RuntimeError::PermissionDenied(_)));
        }
    }

    #[test]
    fn launch_ticket_cannot_cross_a_same_user_session_boundary() {
        let store = InMemoryGroupKnowledgebaseStore::default();
        let now = Utc::now();
        let scope = GroupKnowledgebaseScope {
            tenant_id: "100001".into(),
            organization_id: "200001".into(),
            conversation_id: "g-session-bound".into(),
        };
        let mut link = GroupKnowledgebaseLink::new(
            1,
            "link-session-bound".into(),
            scope.clone(),
            "42".into(),
            now,
        );
        link.lifecycle_state = GroupKnowledgebaseLifecycleState::Active;
        link.knowledge_space_id = Some(101);
        link.knowledge_space_uuid = Some("space-session-bound".into());
        link.knowledgebase_binding_id = Some(201);
        link.knowledgebase_binding_uuid = Some("binding-session-bound".into());
        link.membership_epoch = 7;
        link.last_synchronized_membership_epoch = 7;
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links")
            .insert(scope.clone(), link.clone());

        let raw_ticket = "gklt_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let ticket = GroupKnowledgebaseLaunchTicket {
            id: 1,
            ticket_hash: sha256_hash(raw_ticket.as_bytes()),
            scope,
            knowledge_space_id: 101,
            knowledge_space_uuid: "space-session-bound".into(),
            knowledgebase_binding_id: 201,
            knowledgebase_binding_uuid: "binding-session-bound".into(),
            upstream_link_generation: link.version,
            membership_epoch: 7,
            actor_kind: "user".into(),
            actor_id: "42".into(),
            principal_kind: "user".into(),
            principal_id: "42".into(),
            session_id: "session-a".into(),
            issuing_app_id: Some("sdkwork-im".into()),
            issued_by: "42".into(),
            idempotency_key_hash: sha256_hash(b"ticket-session-bound"),
            request_fingerprint_hash: sha256_hash(b"ticket-session-bound-request"),
            ticket_ciphertext: "not-used-by-session-test".into(),
            expires_at: now + Duration::seconds(60),
            consumed_at: None,
            consumed_by_service: None,
            consumed_trace_id: None,
        };
        store
            .reserve_ticket(ticket.clone())
            .expect("ticket reservation should succeed");

        let mut wrong_session =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        wrong_session.organization_id = "200001".into();
        wrong_session.session_id = Some("session-b".into());

        assert!(
            store
                .find_unconsumed_ticket_for_consumer(ticket.ticket_hash.as_str(), &wrong_session)
                .expect("lookup should not fail")
                .is_none(),
            "the same user in another session must not resolve the ticket"
        );
        assert!(
            !store
                .consume_ticket_if_current(&ticket, &wrong_session, "trace-wrong-session")
                .expect("atomic consume should not fail"),
            "the final consume fence must reject the wrong session even with a current link"
        );

        let mut correct_session = wrong_session;
        correct_session.session_id = Some("session-a".into());
        let remaining = store
            .find_unconsumed_ticket_for_consumer(ticket.ticket_hash.as_str(), &correct_session)
            .expect("correct-session lookup should not fail")
            .expect("wrong-session attempt must not consume the ticket");
        assert!(remaining.consumed_at.is_none());
    }

    #[test]
    fn launch_ticket_currentness_rejects_a_rebound_knowledgebase_binding_uuid() {
        let store = InMemoryGroupKnowledgebaseStore::default();
        let scope = test_scope("g-ticket-binding-fence");
        let link = active_test_link(scope.clone(), 7);
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links")
            .insert(scope.clone(), link.clone());
        let ticket = test_launch_ticket(scope, &link, "binding-rebound");
        store
            .reserve_ticket(ticket.clone())
            .expect("ticket reservation should succeed");

        let mut auth =
            im_app_context::local_service_app_context("100001", "42", "user", None, ["*"]);
        auth.organization_id = "200001".into();
        auth.session_id = Some("session-a".into());

        assert!(
            !store
                .consume_ticket_if_current(&ticket, &auth, "trace-binding-fence")
                .expect("ticket currentness check should not fail"),
            "a ticket must not survive a binding UUID mismatch even when its numeric IDs match"
        );
        let stored = store
            .find_unconsumed_ticket_for_consumer(ticket.ticket_hash.as_str(), &auth)
            .expect("ticket lookup should not fail")
            .expect(
                "rejected ticket consume must leave the ticket available for audit/retry handling",
            );
        assert_eq!(stored.knowledgebase_binding_uuid, "binding-rebound");
        assert_eq!(stored.upstream_link_generation, link.version);
    }

    #[test]
    fn group_knowledgebase_initialization_gate_requires_the_group_owner() {
        let mut member = ConversationMember {
            tenant_id: "100001".into(),
            conversation_id: "g_test".into(),
            member_id: "1".into(),
            principal_id: "1".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            state: im_domain_core::conversation::MembershipState::Joined,
            invited_by: None,
            joined_at: Utc::now().to_rfc3339(),
            removed_at: None,
            attributes: Default::default(),
        };
        assert!(ensure_group_knowledgebase_owner(&member).is_err());
        member.role = MembershipRole::Admin;
        assert!(ensure_group_knowledgebase_owner(&member).is_err());
        member.role = MembershipRole::Owner;
        assert!(ensure_group_knowledgebase_owner(&member).is_ok());
    }

    #[test]
    fn durable_reconciliation_recreates_one_membership_snapshot_after_commit_window() {
        let scope = test_scope("g-reconcile-members");
        let runtime = normalized_group_runtime(
            &scope,
            "active",
            2,
            2,
            vec![
                test_member(&scope, "m-owner", "42", MembershipRole::Owner),
                test_member(&scope, "m-member", "43", MembershipRole::Member),
            ],
        );
        let store = Arc::new(InMemoryGroupKnowledgebaseStore::default());
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links")
            .insert(scope.clone(), active_test_link(scope.clone(), 1));
        let coordinator = reconciliation_coordinator(store.clone());
        let mut cursor = GroupKnowledgebaseReconciliationCursor::default();

        assert_eq!(
            coordinator
                .reconcile_durable_state(&runtime, &mut cursor, 8)
                .expect("reconciliation should succeed"),
            1
        );
        let pending = store.pending_outbox_payloads();
        assert_eq!(pending.len(), 1);
        assert_eq!(
            pending[0].operation,
            GroupKnowledgebaseOutboxOperation::SynchronizeMembers
        );
        assert_eq!(pending[0].membership_epoch, 2);
        assert_eq!(pending[0].members.len(), 2);
        assert_eq!(
            pending[0].source_event_id,
            group_knowledgebase_membership_reconciliation_source_event_id(&scope, 2)
        );

        // The second call completes the scope pass and the third revisits the
        // link. The deterministic event id must keep the recovery idempotent.
        coordinator
            .reconcile_durable_state(&runtime, &mut cursor, 8)
            .expect("scope completion should succeed");
        coordinator
            .reconcile_durable_state(&runtime, &mut cursor, 8)
            .expect("repeated reconciliation should succeed");
        assert_eq!(store.pending_outbox_payloads().len(), 1);
    }

    #[test]
    fn durable_reconciliation_recreates_archive_from_normalized_lifecycle() {
        let scope = test_scope("g-reconcile-archive");
        let runtime = normalized_group_runtime(
            &scope,
            "archived",
            2,
            1,
            vec![test_member(&scope, "m-owner", "42", MembershipRole::Owner)],
        );
        let store = Arc::new(InMemoryGroupKnowledgebaseStore::default());
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links")
            .insert(scope.clone(), active_test_link(scope.clone(), 1));
        let coordinator = reconciliation_coordinator(store.clone());
        let mut cursor = GroupKnowledgebaseReconciliationCursor::default();

        coordinator
            .reconcile_durable_state(&runtime, &mut cursor, 8)
            .expect("archive reconciliation should succeed");
        let pending = store.pending_outbox_payloads();
        assert_eq!(pending.len(), 1);
        assert_eq!(
            pending[0].operation,
            GroupKnowledgebaseOutboxOperation::Archive
        );
        assert_eq!(
            pending[0].source_event_id,
            group_knowledgebase_archive_reconciliation_source_event_id(&scope, 2)
        );
        assert_eq!(
            pending[0].archived_by.as_deref(),
            Some(GROUP_KNOWLEDGEBASE_RECONCILIATION_ACTOR_ID)
        );
        assert!(matches!(
            store
                .get_link(&scope)
                .expect("link lookup should succeed")
                .expect("link should exist")
                .lifecycle_state,
            GroupKnowledgebaseLifecycleState::Archived
        ));

        coordinator
            .reconcile_durable_state(&runtime, &mut cursor, 8)
            .expect("scope completion should succeed");
        coordinator
            .reconcile_durable_state(&runtime, &mut cursor, 8)
            .expect("repeated archive reconciliation should succeed");
        assert_eq!(store.pending_outbox_payloads().len(), 1);
    }

    #[test]
    fn durable_reconciliation_rejects_archived_link_when_normalized_conversation_is_active() {
        let scope = test_scope("g-reconcile-corrupt-archive");
        let runtime = normalized_group_runtime(&scope, "active", 1, 0, Vec::new());
        let store = Arc::new(InMemoryGroupKnowledgebaseStore::default());
        let mut link = active_test_link(scope.clone(), 0);
        link.lifecycle_state = GroupKnowledgebaseLifecycleState::Archived;
        link.knowledge_space_id = None;
        link.knowledge_space_uuid = None;
        link.knowledgebase_binding_id = None;
        link.knowledgebase_binding_uuid = None;
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links").insert(scope, link);
        let coordinator = reconciliation_coordinator(store);
        let mut cursor = GroupKnowledgebaseReconciliationCursor::default();

        let error = coordinator
            .reconcile_durable_state(&runtime, &mut cursor, 8)
            .expect_err(
                "an archived link must not recover while normalized Conversation state is active",
            );
        assert!(matches!(error, RuntimeError::Conflict(_)));
        assert!(
            cursor.pending_provisioning_recovery.is_none(),
            "the relay must not schedule a remote ensure for a corrupt archived conversation_state"
        );
    }

    #[test]
    fn durable_reconciliation_rejects_a_normalized_organization_collision() {
        let scope = test_scope("g-reconcile-collision");
        let runtime = normalized_group_runtime_with_store(NormalizedGroupConversationStore {
            conversation: NormalizedConversationRecord {
                tenant_id: scope.tenant_id.clone(),
                organization_id: "200002".into(),
                conversation_id: scope.conversation_id.clone(),
                conversation_type: "group".into(),
                lifecycle_state: "active".into(),
                archived_at: None,
                archive_event_id: None,
                commit_seq: 1,
                member_epoch: 0,
                last_activity_at: "2026-07-13T00:00:00Z".into(),
                retention_until: None,
            },
            members: Vec::new(),
        });
        let error = group_knowledgebase_durable_snapshot(&runtime, &scope)
            .expect_err("wrong normalized organization must fail closed");
        assert!(matches!(error, RuntimeError::Conflict(_)));
    }

    #[test]
    fn membership_sync_never_lowers_epoch_and_reuses_the_same_epoch_outbox() {
        let scope = test_scope("g-sync-epoch");
        let store = InMemoryGroupKnowledgebaseStore::default();
        let mut link = active_test_link(scope.clone(), 5);
        link.last_synchronized_membership_epoch = 4;
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links").insert(scope.clone(), link);

        let stale = store
            .enqueue_membership_synchronization(GroupKnowledgebaseMembershipSyncEnqueue {
                scope: scope.clone(),
                actor_id: "42".into(),
                source_event_id: "stale-membership".into(),
                target_membership_epoch: 4,
                members: Vec::new(),
                outbox_id: "1".into(),
                occurred_at: Utc::now(),
            })
            .expect("stale enqueue should not fail")
            .expect("link should remain available");
        assert_eq!(stale.membership_epoch, 5);
        assert!(store.pending_outbox_payloads().is_empty());

        let first = store
            .enqueue_membership_synchronization(GroupKnowledgebaseMembershipSyncEnqueue {
                scope: scope.clone(),
                actor_id: "42".into(),
                source_event_id: "same-epoch-membership".into(),
                target_membership_epoch: 5,
                members: Vec::new(),
                outbox_id: "2".into(),
                occurred_at: Utc::now(),
            })
            .expect("same epoch enqueue should succeed")
            .expect("link should remain available");
        let replay = store
            .enqueue_membership_synchronization(GroupKnowledgebaseMembershipSyncEnqueue {
                scope: scope.clone(),
                actor_id: "42".into(),
                source_event_id: "same-epoch-membership".into(),
                target_membership_epoch: 5,
                members: Vec::new(),
                outbox_id: "3".into(),
                occurred_at: Utc::now(),
            })
            .expect("same epoch replay should succeed")
            .expect("link should remain available");
        assert_eq!(first.membership_epoch, 5);
        assert_eq!(replay.membership_epoch, 5);
        assert_eq!(store.pending_outbox_payloads().len(), 1);
        assert!(
            store
                .mark_membership_synchronized(&scope, 5, replay.version, "sdkwork-knowledgebase")
                .expect("acknowledgement should succeed")
        );
        assert_eq!(
            store
                .get_link(&scope)
                .expect("link lookup should succeed")
                .expect("link should exist")
                .last_synchronized_membership_epoch,
            5
        );
    }

    #[test]
    fn provisioning_completion_preserves_archive_and_enqueues_archive_handoff() {
        let scope = test_scope("g-provisioning-archive-race");
        let store = InMemoryGroupKnowledgebaseStore::default();
        let link = GroupKnowledgebaseLink::new(
            99,
            "link-provisioning-archive-race".into(),
            scope.clone(),
            "42".into(),
            Utc::now(),
        );
        lock_knowledgebase_mutex(&store.links, "knowledgebase-links").insert(scope.clone(), link);
        store
            .archive_link_and_enqueue(GroupKnowledgebaseArchiveEnqueue {
                scope: scope.clone(),
                actor_id: "archive-owner".into(),
                source_event_id: "evt-archive-during-provisioning".into(),
                outbox_id: "archive-before-space".into(),
                occurred_at: Utc::now(),
            })
            .expect("archive before remote ensure return should succeed");
        let completed = store
            .activate_link(
                &scope,
                EnsuredGroupKnowledgebase {
                    knowledge_space_id: 701,
                    knowledge_space_uuid: "space-701".into(),
                    knowledgebase_binding_id: 801,
                    knowledgebase_binding_uuid: "binding-801".into(),
                    provisioning_operation_id: None,
                    membership_epoch: 1,
                },
                1,
                "42",
                "im.group-knowledgebase.ensure:99:1",
                "archive-after-space",
            )
            .expect("completion should retain archived lifecycle");
        assert!(matches!(
            completed.lifecycle_state,
            GroupKnowledgebaseLifecycleState::Archived
        ));
        let pending = store.pending_outbox_payloads();
        assert_eq!(pending.len(), 1);
        assert_eq!(
            pending[0].operation,
            GroupKnowledgebaseOutboxOperation::Archive
        );
        assert_eq!(
            pending[0].source_event_id,
            "evt-archive-during-provisioning"
        );
    }
}
