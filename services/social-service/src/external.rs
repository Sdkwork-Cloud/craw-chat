use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use im_app_context::AppContext;
use im_domain_core::social::{
    ExternalConnection, ExternalConnectionKind, ExternalConnectionStatus, ExternalMemberLink,
    ExternalMemberLinkStatus, ensure_cross_tenant_connection,
};
use im_domain_events::social::{
    ExternalConnectionEstablishedPayload, ExternalMemberLinkBoundPayload,
    SocialCommitEnvelopeInput, SocialEventType, social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use serde::{Deserialize, Serialize};

use crate::friendship::{AppState, SocialServiceError};
use crate::runtime::{
    SocialRuntime, SocialWritePersistence, StoredExternalConnection, StoredExternalMemberLink,
};

const MAX_ID_BYTES: usize = 256;
const MAX_ACTOR_KIND_BYTES: usize = 64;
const MAX_TIMESTAMP_BYTES: usize = 64;
const MAX_EXTERNAL_ORG_NAME_BYTES: usize = 256;
const MAX_EXTERNAL_DISPLAY_NAME_BYTES: usize = 512;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EstablishExternalConnectionRequest {
    pub(crate) connection_id: String,
    pub(crate) event_id: String,
    pub(crate) external_tenant_id: String,
    pub(crate) external_org_name: Option<String>,
    pub(crate) connection_kind: ExternalConnectionKind,
    pub(crate) established_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BindExternalMemberLinkRequest {
    pub(crate) link_id: String,
    pub(crate) event_id: String,
    pub(crate) connection_id: String,
    pub(crate) local_actor_id: String,
    pub(crate) local_actor_kind: String,
    pub(crate) external_member_id: String,
    pub(crate) external_display_name: Option<String>,
    pub(crate) linked_at: String,
}

// ---------------------------------------------------------------------------
// Result types (business logic layer)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct EstablishedExternalConnection {
    pub(crate) external_connection: ExternalConnection,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

#[derive(Clone, Debug)]
pub(crate) struct BoundExternalMemberLink {
    pub(crate) external_member_link: ExternalMemberLink,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
    pub(crate) shared_channel_sync_requests: Vec<crate::SharedChannelLinkedMemberSyncRequest>,
}

// ---------------------------------------------------------------------------
// HTTP response types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialExternalConnectionWriteStatus {
    Established,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialExternalConnectionReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialExternalConnectionCommitResponse {
    status: SocialExternalConnectionWriteStatus,
    external_connection: ExternalConnection,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialExternalConnectionSnapshotResponse {
    status: SocialExternalConnectionReadStatus,
    external_connection: ExternalConnection,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialExternalMemberLinkWriteStatus {
    Bound,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialExternalMemberLinkReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialExternalMemberLinkCommitResponse {
    status: SocialExternalMemberLinkWriteStatus,
    external_member_link: ExternalMemberLink,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialExternalMemberLinkSnapshotResponse {
    status: SocialExternalMemberLinkReadStatus,
    external_member_link: ExternalMemberLink,
    commits: Vec<CommitEnvelopeResponse>,
}

// ---------------------------------------------------------------------------
// CommitEnvelope response adapter
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommitEnvelopeResponse {
    event_id: String,
    tenant_id: String,
    event_type: String,
    event_version: u16,
    aggregate_type: String,
    aggregate_id: String,
    scope_type: String,
    scope_id: String,
    ordering_key: String,
    ordering_seq: u64,
    causation_id: Option<String>,
    correlation_id: Option<String>,
    idempotency_key: Option<String>,
    actor: EventActorResponse,
    occurred_at: String,
    committed_at: String,
    payload_schema: Option<String>,
    payload: String,
    retention_class: String,
    audit_class: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EventActorResponse {
    actor_id: String,
    actor_kind: String,
    actor_session_id: Option<String>,
}

impl From<CommitEnvelope> for CommitEnvelopeResponse {
    fn from(value: CommitEnvelope) -> Self {
        Self {
            event_id: value.event_id,
            tenant_id: value.tenant_id,
            event_type: value.event_type,
            event_version: value.event_version,
            aggregate_type: value.aggregate_type.as_wire_value().into(),
            aggregate_id: value.aggregate_id,
            scope_type: value.scope_type,
            scope_id: value.scope_id,
            ordering_key: value.ordering_key,
            ordering_seq: value.ordering_seq,
            causation_id: value.causation_id,
            correlation_id: value.correlation_id,
            idempotency_key: value.idempotency_key,
            actor: EventActorResponse {
                actor_id: value.actor.actor_id,
                actor_kind: value.actor.actor_kind,
                actor_session_id: value.actor.actor_session_id,
            },
            occurred_at: value.occurred_at,
            committed_at: value.committed_at,
            payload_schema: value.payload_schema,
            payload: value.payload,
            retention_class: value.retention_class,
            audit_class: value.audit_class,
        }
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), SocialServiceError> {
    if value.len() > max_bytes {
        return Err(SocialServiceError::payload_too_large(
            field,
            max_bytes,
            value.len(),
        ));
    }
    Ok(())
}

fn validate_optional_payload_size(
    field: &'static str,
    value: Option<&str>,
    max_bytes: usize,
) -> Result<(), SocialServiceError> {
    if let Some(value) = value {
        validate_payload_size(field, value, max_bytes)?;
    }
    Ok(())
}

fn validate_required_with_code(
    field: &'static str,
    value: &str,
    code: &'static str,
) -> Result<(), SocialServiceError> {
    if value.trim().is_empty() {
        return Err(SocialServiceError::invalid(
            code,
            format!("{field} cannot be empty"),
        ));
    }
    Ok(())
}

fn social_event_id_conflict_message(
    event_id: &str,
    existing: &crate::runtime::SocialCommittedEvent,
) -> String {
    let committed = existing.commit();
    format!(
        "eventId {} is already committed for {} {}",
        event_id,
        existing.aggregate_label(),
        committed.aggregate_id
    )
}

fn active_external_connection_record_for_target(
    state: &crate::runtime::SocialControlState,
    tenant_id: &str,
    external_tenant_id: &str,
    connection_kind: &ExternalConnectionKind,
) -> Option<StoredExternalConnection> {
    let key = crate::runtime::SocialExternalConnectionTargetIndexKey::new(
        tenant_id,
        external_tenant_id,
        connection_kind,
    );
    state
        .external_connections
        .get(state.active_external_connection_target_index.get(&key)?)
        .filter(|record| record.external_connection.status.is_active())
        .cloned()
}

fn active_external_member_link_record_for_mapping(
    state: &crate::runtime::SocialControlState,
    tenant_id: &str,
    connection_id: &str,
    external_member_id: &str,
) -> Option<StoredExternalMemberLink> {
    let key = crate::runtime::SocialExternalMemberMappingIndexKey::new(
        tenant_id,
        connection_id,
        external_member_id,
    );
    state
        .external_member_links
        .get(state.active_external_member_mapping_index.get(&key)?)
        .filter(|record| record.external_member_link.status.is_active())
        .cloned()
}

// ---------------------------------------------------------------------------
// Business logic: SocialRuntime external connection methods
// ---------------------------------------------------------------------------

impl SocialRuntime {
    pub(crate) fn establish_external_connection(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: EstablishExternalConnectionRequest,
    ) -> Result<EstablishedExternalConnection, SocialServiceError> {
        validate_payload_size("connectionId", request.connection_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "externalTenantId",
            request.external_tenant_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "externalOrgName",
            request.external_org_name.as_deref(),
            MAX_EXTERNAL_ORG_NAME_BYTES,
        )?;
        validate_payload_size(
            "establishedAt",
            request.established_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code(
            "connectionId",
            request.connection_id.as_str(),
            "invalid_external_connection",
        )?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_external_connection",
        )?;
        validate_required_with_code(
            "externalTenantId",
            request.external_tenant_id.as_str(),
            "invalid_external_connection",
        )?;
        validate_required_with_code(
            "establishedAt",
            request.established_at.as_str(),
            "invalid_external_connection",
        )?;
        ensure_cross_tenant_connection(tenant_id, request.external_tenant_id.as_str()).map_err(
            |error| SocialServiceError::invalid("invalid_external_connection", error.to_string()),
        )?;

        let payload = ExternalConnectionEstablishedPayload {
            connection_id: request.connection_id.clone(),
            external_tenant_id: request.external_tenant_id.clone(),
            external_org_name: request.external_org_name.clone(),
            connection_kind: serde_json::to_string(&request.connection_kind)
                .expect("external connection kind should serialize")
                .trim_matches('"')
                .to_owned(),
            established_at: request.established_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("external connection payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::ExternalConnection,
            aggregate_id: request.connection_id.as_str(),
            event_type: SocialEventType::ExternalConnectionEstablished,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.established_at.as_str(),
            committed_at: request.established_at.as_str(),
            payload: payload_json.as_str(),
        });
        let external_connection = ExternalConnection {
            tenant_id: tenant_id.into(),
            connection_id: request.connection_id.clone(),
            external_tenant_id: request.external_tenant_id.clone(),
            external_org_name: request.external_org_name,
            connection_kind: request.connection_kind.clone(),
            status: ExternalConnectionStatus::Active,
            established_at: request.established_at.clone(),
            updated_at: request.established_at,
        };

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::ExternalConnection { record, commit } => {
                        Ok(EstablishedExternalConnection {
                            external_connection: record.external_connection,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict_message(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .external_connections
            .contains_key(external_connection.connection_id.as_str())
        {
            return Err(SocialServiceError::conflict(
                "external_connection_conflict",
                format!(
                    "external connection {} already exists",
                    external_connection.connection_id
                ),
            ));
        }
        if active_external_connection_record_for_target(
            &next_state,
            tenant_id,
            external_connection.external_tenant_id.as_str(),
            &external_connection.connection_kind,
        )
        .is_some()
        {
            return Err(SocialServiceError::conflict(
                "external_connection_target_conflict",
                format!(
                    "active external connection already exists for tenant {} and kind {:?}",
                    external_connection.external_tenant_id, external_connection.connection_kind
                ),
            ));
        }

        next_state.insert_external_connection_record(
            external_connection.connection_id.clone(),
            StoredExternalConnection {
                external_connection: external_connection.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(EstablishedExternalConnection {
            external_connection,
            latest_commit: commit,
            persistence,
        })
    }

    pub(crate) fn bind_external_member_link(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: BindExternalMemberLinkRequest,
    ) -> Result<BoundExternalMemberLink, SocialServiceError> {
        validate_payload_size("linkId", request.link_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("connectionId", request.connection_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "localActorId",
            request.local_actor_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "localActorKind",
            request.local_actor_kind.as_str(),
            MAX_ACTOR_KIND_BYTES,
        )?;
        validate_payload_size(
            "externalMemberId",
            request.external_member_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "externalDisplayName",
            request.external_display_name.as_deref(),
            MAX_EXTERNAL_DISPLAY_NAME_BYTES,
        )?;
        validate_payload_size("linkedAt", request.linked_at.as_str(), MAX_TIMESTAMP_BYTES)?;
        validate_required_with_code(
            "linkId",
            request.link_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "connectionId",
            request.connection_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "localActorId",
            request.local_actor_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "localActorKind",
            request.local_actor_kind.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "externalMemberId",
            request.external_member_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "linkedAt",
            request.linked_at.as_str(),
            "invalid_external_member_link",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let connection = self
            .external_connection_snapshot(tenant_id, request.connection_id.as_str())
            .ok_or_else(|| {
                SocialServiceError::not_found(
                    "external_connection_not_found",
                    format!(
                        "external connection {} was not found",
                        request.connection_id
                    ),
                )
            })?;
        if !connection.external_connection.status.is_active() {
            return Err(SocialServiceError::conflict(
                "external_connection_inactive",
                format!(
                    "external connection {} is not active",
                    connection.external_connection.connection_id
                ),
            ));
        }

        let payload = ExternalMemberLinkBoundPayload {
            link_id: request.link_id.clone(),
            connection_id: request.connection_id.clone(),
            local_actor_id: request.local_actor_id.clone(),
            local_actor_kind: request.local_actor_kind.clone(),
            external_member_id: request.external_member_id.clone(),
            external_display_name: request.external_display_name.clone(),
            linked_at: request.linked_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("external member link payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::ExternalMemberLink,
            aggregate_id: request.link_id.as_str(),
            event_type: SocialEventType::ExternalMemberLinkBound,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.linked_at.as_str(),
            committed_at: request.linked_at.as_str(),
            payload: payload_json.as_str(),
        });
        let external_member_link = ExternalMemberLink {
            tenant_id: tenant_id.into(),
            link_id: request.link_id.clone(),
            connection_id: request.connection_id.clone(),
            local_actor_id: request.local_actor_id,
            local_actor_kind: request.local_actor_kind,
            external_member_id: request.external_member_id,
            external_display_name: request.external_display_name,
            status: ExternalMemberLinkStatus::Active,
            linked_at: request.linked_at.clone(),
            updated_at: request.linked_at,
        };

        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::ExternalMemberLink { record, commit } => {
                        Ok(BoundExternalMemberLink {
                            external_member_link: record.external_member_link,
                            latest_commit: commit,
                            persistence,
                            shared_channel_sync_requests: Vec::new(),
                        })
                    }
                    other => Err(social_event_id_conflict_message(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .external_member_links
            .contains_key(external_member_link.link_id.as_str())
        {
            return Err(SocialServiceError::conflict(
                "external_member_link_conflict",
                format!(
                    "external member link {} already exists",
                    external_member_link.link_id
                ),
            ));
        }
        if active_external_member_link_record_for_mapping(
            &next_state,
            tenant_id,
            external_member_link.connection_id.as_str(),
            external_member_link.external_member_id.as_str(),
        )
        .is_some()
        {
            return Err(SocialServiceError::conflict(
                "external_member_mapping_conflict",
                format!(
                    "active external member mapping already exists for {} on connection {}",
                    external_member_link.external_member_id, external_member_link.connection_id
                ),
            ));
        }

        next_state.insert_external_member_link_record(
            external_member_link.link_id.clone(),
            StoredExternalMemberLink {
                external_member_link: external_member_link.clone(),
                commits: vec![commit.clone()],
            },
        );
        let shared_channel_sync_requests =
            crate::shared_channel::shared_channel_sync_requests_for_external_member_link(
                &next_state,
                &external_member_link,
            );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(BoundExternalMemberLink {
            external_member_link,
            latest_commit: commit,
            persistence,
            shared_channel_sync_requests,
        })
    }

    pub(crate) fn external_member_link_snapshot(
        &self,
        tenant_id: &str,
        link_id: &str,
    ) -> Option<StoredExternalMemberLink> {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .external_member_links
            .get(link_id)
            .filter(|record| record.external_member_link.tenant_id == tenant_id)
            .cloned()
    }
}

// ---------------------------------------------------------------------------
// HTTP handler functions
// ---------------------------------------------------------------------------

pub(crate) async fn establish_external_connection(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<EstablishExternalConnectionRequest>,
) -> Result<Json<SocialExternalConnectionCommitResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let established = state.social_runtime.establish_external_connection(
        auth.tenant_id.as_str(),
        &auth,
        request,
    )?;

    Ok(Json(SocialExternalConnectionCommitResponse {
        status: SocialExternalConnectionWriteStatus::Established,
        external_connection: established.external_connection,
        latest_commit: established.latest_commit.into(),
        persistence: established.persistence,
    }))
}

pub(crate) async fn external_connection_snapshot(
    Path(connection_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialExternalConnectionSnapshotResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let snapshot = state
        .social_runtime
        .external_connection_snapshot(auth.tenant_id.as_str(), connection_id.as_str())
        .ok_or_else(|| {
            SocialServiceError::not_found(
                "external_connection_not_found",
                format!("external connection {connection_id} was not found"),
            )
        })?;

    Ok(Json(SocialExternalConnectionSnapshotResponse {
        status: SocialExternalConnectionReadStatus::Snapshot,
        external_connection: snapshot.external_connection,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

pub(crate) async fn bind_external_member_link(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BindExternalMemberLinkRequest>,
) -> Result<Json<SocialExternalMemberLinkCommitResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let bound =
        state
            .social_runtime
            .bind_external_member_link(auth.tenant_id.as_str(), &auth, request)?;

    state
        .social_runtime
        .dispatch_shared_channel_sync_requests(&bound.shared_channel_sync_requests)
        .map_err(|error| SocialServiceError::invalid("shared_channel_sync_failed", error))?;

    Ok(Json(SocialExternalMemberLinkCommitResponse {
        status: SocialExternalMemberLinkWriteStatus::Bound,
        external_member_link: bound.external_member_link,
        latest_commit: bound.latest_commit.into(),
        persistence: bound.persistence,
    }))
}

pub(crate) async fn external_member_link_snapshot(
    Path(link_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialExternalMemberLinkSnapshotResponse>, SocialServiceError> {
    let auth = crate::friendship::resolve_auth_from_headers(&headers)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let snapshot = state
        .social_runtime
        .external_member_link_snapshot(auth.tenant_id.as_str(), link_id.as_str())
        .ok_or_else(|| {
            SocialServiceError::not_found(
                "external_member_link_not_found",
                format!("external member link {link_id} was not found"),
            )
        })?;

    Ok(Json(SocialExternalMemberLinkSnapshotResponse {
        status: SocialExternalMemberLinkReadStatus::Snapshot,
        external_member_link: snapshot.external_member_link,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}
