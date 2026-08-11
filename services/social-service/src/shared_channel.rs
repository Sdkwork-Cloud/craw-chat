use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::response::Response;
use im_app_context::AppContext;
use im_domain_core::social::{SharedChannelPolicy, SharedChannelPolicyStatus};
use im_domain_events::social::{
    SharedChannelPolicyAppliedPayload, SocialCommitEnvelopeInput, SocialEventType,
    social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::SharedChannelLinkedMemberSyncRequest;
use crate::api_payload::resource_item;
use crate::envelope::finish_enveloped_json;
use crate::external::CommitEnvelopeResponse;
use crate::friendship::{AppState, SocialServiceError};
use crate::runtime::{
    SocialConnectionIndexKey, SocialControlState, SocialRuntime,
    SocialSharedChannelPolicyTargetIndexKey, SocialWritePersistence, StoredExternalMemberLink,
    StoredSharedChannelPolicy,
};

const MAX_ID_BYTES: usize = 256;
const MAX_HISTORY_VISIBILITY_BYTES: usize = 64;
const MAX_TIMESTAMP_BYTES: usize = 64;
const MAX_SHARED_CHANNEL_SYNC_TARGETS_PER_MUTATION: usize = 200;
const SHARED_CHANNEL_SYNC_LOOKUP_LIMIT: i64 = 201;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApplySharedChannelPolicyRequest {
    pub(crate) policy_id: String,
    pub(crate) event_id: String,
    pub(crate) connection_id: String,
    pub(crate) channel_id: String,
    pub(crate) conversation_id: Option<String>,
    pub(crate) policy_version: u64,
    pub(crate) history_visibility: String,
    pub(crate) applied_at: String,
}

// ---------------------------------------------------------------------------
// Result types (business logic layer)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct AppliedSharedChannelPolicy {
    pub(crate) shared_channel_policy: SharedChannelPolicy,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
    pub(crate) shared_channel_sync_requests: Vec<SharedChannelLinkedMemberSyncRequest>,
}

// ---------------------------------------------------------------------------
// HTTP response types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialSharedChannelPolicyWriteStatus {
    Applied,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialSharedChannelPolicyReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialSharedChannelPolicyCommitResponse {
    status: SocialSharedChannelPolicyWriteStatus,
    shared_channel_policy: SharedChannelPolicy,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialSharedChannelPolicySnapshotResponse {
    status: SocialSharedChannelPolicyReadStatus,
    shared_channel_policy: SharedChannelPolicy,
    commits: Vec<CommitEnvelopeResponse>,
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

fn active_shared_channel_policy_record_for_target(
    state: &crate::runtime::SocialControlState,
    tenant_id: &str,
    connection_id: &str,
    channel_id: &str,
) -> Option<StoredSharedChannelPolicy> {
    let key = SocialSharedChannelPolicyTargetIndexKey::new(tenant_id, connection_id, channel_id);
    state
        .shared_channel_policies
        .get(state.active_shared_channel_policy_target_index.get(&key)?)
        .filter(|record| record.shared_channel_policy.status.is_active())
        .cloned()
}

// ---------------------------------------------------------------------------
// Business logic: SocialRuntime shared channel methods
// ---------------------------------------------------------------------------

impl SocialRuntime {
    pub(crate) fn apply_shared_channel_policy(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: ApplySharedChannelPolicyRequest,
    ) -> Result<AppliedSharedChannelPolicy, SocialServiceError> {
        validate_payload_size("policyId", request.policy_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("connectionId", request.connection_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("channelId", request.channel_id.as_str(), MAX_ID_BYTES)?;
        validate_optional_payload_size(
            "conversationId",
            request.conversation_id.as_deref(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "historyVisibility",
            request.history_visibility.as_str(),
            MAX_HISTORY_VISIBILITY_BYTES,
        )?;
        validate_payload_size(
            "appliedAt",
            request.applied_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code(
            "policyId",
            request.policy_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "connectionId",
            request.connection_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "channelId",
            request.channel_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "historyVisibility",
            request.history_visibility.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "appliedAt",
            request.applied_at.as_str(),
            "invalid_shared_channel_policy",
        )?;
        if request.policy_version == 0 {
            return Err(SocialServiceError::invalid(
                "invalid_shared_channel_policy",
                "policyVersion must be greater than 0",
            ));
        }
        if request.history_visibility != "shared" {
            return Err(SocialServiceError::invalid(
                "invalid_shared_channel_policy",
                format!(
                    "shared_channel_policy only supports historyVisibility=shared, got {}",
                    request.history_visibility
                ),
            ));
        }

        self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        self.external_connection_snapshot(
            tenant_id,
            auth.organization_id.as_str(),
            request.connection_id.as_str(),
        )
        .map_err(|error| {
            SocialServiceError::dependency_unavailable(
                "external_connection_store_unavailable",
                error,
            )
        })?
        .filter(|record| record.external_connection.status.is_active())
        .ok_or_else(|| {
            SocialServiceError::not_found(
                "external_connection_not_found",
                format!(
                    "external connection {} was not found or is inactive",
                    request.connection_id
                ),
            )
        })?;

        let payload = SharedChannelPolicyAppliedPayload {
            policy_id: request.policy_id.clone(),
            connection_id: request.connection_id.clone(),
            channel_id: request.channel_id.clone(),
            conversation_id: request.conversation_id.clone(),
            policy_version: request.policy_version,
            history_visibility: request.history_visibility.clone(),
            applied_at: request.applied_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("shared channel policy payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::SharedChannelPolicy,
            aggregate_id: request.policy_id.as_str(),
            event_type: SocialEventType::SharedChannelPolicyApplied,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.applied_at.as_str(),
            committed_at: request.applied_at.as_str(),
            payload: payload_json.as_str(),
        });
        let shared_channel_policy = SharedChannelPolicy {
            tenant_id: tenant_id.into(),
            policy_id: request.policy_id.clone(),
            connection_id: request.connection_id.clone(),
            channel_id: request.channel_id,
            conversation_id: request.conversation_id,
            policy_version: request.policy_version,
            history_visibility: request.history_visibility,
            status: SharedChannelPolicyStatus::Active,
            applied_at: request.applied_at.clone(),
            updated_at: request.applied_at,
        };

        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.resolve_committed_social_event_retry(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::SharedChannelPolicy {
                        record,
                        commit,
                    } => Ok(AppliedSharedChannelPolicy {
                        shared_channel_sync_requests: self
                            .shared_channel_sync_requests_for_shared_channel_policy(
                                &state,
                                auth.organization_id.as_str(),
                                &record.shared_channel_policy,
                            )?,
                        shared_channel_policy: record.shared_channel_policy,
                        latest_commit: commit,
                        persistence,
                    }),
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
            .shared_channel_policies
            .contains_key(shared_channel_policy.policy_id.as_str())
        {
            return Err(SocialServiceError::conflict(
                "shared_channel_policy_conflict",
                format!(
                    "shared channel policy {} already exists",
                    shared_channel_policy.policy_id
                ),
            ));
        }
        if self
            .active_shared_channel_policy_for_target(
                &next_state,
                tenant_id,
                auth.organization_id.as_str(),
                shared_channel_policy.connection_id.as_str(),
                shared_channel_policy.channel_id.as_str(),
            )
            .map_err(|error| {
                SocialServiceError::dependency_unavailable(
                    "shared_channel_policy_store_unavailable",
                    error,
                )
            })?
            .is_some()
        {
            return Err(SocialServiceError::conflict(
                "shared_channel_policy_target_conflict",
                format!(
                    "active shared channel policy already exists for channel {} on connection {}",
                    shared_channel_policy.channel_id, shared_channel_policy.connection_id
                ),
            ));
        }

        next_state.insert_shared_channel_policy_record(
            shared_channel_policy.policy_id.clone(),
            StoredSharedChannelPolicy {
                shared_channel_policy: shared_channel_policy.clone(),
                commits: vec![commit.clone()],
            },
        );
        let shared_channel_sync_requests = self
            .shared_channel_sync_requests_for_shared_channel_policy(
                &next_state,
                auth.organization_id.as_str(),
                &shared_channel_policy,
            )
            .map_err(|error| {
                SocialServiceError::dependency_unavailable(
                    "external_member_link_store_unavailable",
                    error,
                )
            })?;
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(AppliedSharedChannelPolicy {
            shared_channel_policy,
            latest_commit: commit,
            persistence,
            shared_channel_sync_requests,
        })
    }

    pub(crate) fn shared_channel_policy_snapshot(
        &self,
        tenant_id: &str,
        organization_id: &str,
        policy_id: &str,
    ) -> Result<Option<StoredSharedChannelPolicy>, String> {
        if let Some(store) = self.shared_channel_policy_authority_store() {
            return store
                .get_by_id(
                    tenant_id,
                    organization_id,
                    im_adapters_social_postgres::wire_id::parse_social_entity_id(policy_id)
                        .map_err(|error| format!("invalid shared-channel-policy id: {error:?}"))?,
                )
                .map_err(|error| {
                    format!("normalized shared-channel-policy lookup failed: {error:?}")
                })?
                .map(shared_channel_policy_from_authority_record)
                .transpose()
                .map(|record| {
                    record.map(|mut shared_channel_policy| {
                        shared_channel_policy.policy_id = policy_id.to_owned();
                        StoredSharedChannelPolicy {
                            shared_channel_policy,
                            commits: Vec::new(),
                        }
                    })
                });
        }

        Ok(self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .shared_channel_policies
            .get(policy_id)
            .filter(|record| record.shared_channel_policy.tenant_id == tenant_id)
            .cloned())
    }

    fn active_shared_channel_policy_for_target(
        &self,
        state: &SocialControlState,
        tenant_id: &str,
        organization_id: &str,
        connection_id: &str,
        channel_id: &str,
    ) -> Result<Option<StoredSharedChannelPolicy>, String> {
        if let Some(store) = self.shared_channel_policy_authority_store() {
            return store
                .find_by_target(
                    tenant_id,
                    organization_id,
                    im_adapters_social_postgres::wire_id::parse_social_entity_id(connection_id)
                        .map_err(|error| format!("invalid external-connection id: {error:?}"))?,
                    channel_id,
                )
                .map_err(|error| {
                    format!("normalized shared-channel-policy target lookup failed: {error:?}")
                })?
                .map(shared_channel_policy_from_authority_record)
                .transpose()
                .map(|record| {
                    record
                        .filter(|policy| policy.status.is_active())
                        .map(|shared_channel_policy| StoredSharedChannelPolicy {
                            shared_channel_policy,
                            commits: Vec::new(),
                        })
                });
        }

        Ok(active_shared_channel_policy_record_for_target(
            state,
            tenant_id,
            connection_id,
            channel_id,
        ))
    }
}

fn shared_channel_policy_from_authority_record(
    record: im_adapters_social_postgres::shared_channel_store::SharedChannelPolicyRecord,
) -> Result<SharedChannelPolicy, String> {
    let status = match record.status.as_str() {
        "active" => SharedChannelPolicyStatus::Active,
        "suspended" => SharedChannelPolicyStatus::Suspended,
        other => {
            return Err(format!(
                "normalized shared-channel-policy status is invalid: {other}"
            ));
        }
    };
    let policy_version = u64::try_from(record.policy_version).map_err(|_| {
        format!(
            "normalized shared-channel-policy version is invalid: {}",
            record.policy_version
        )
    })?;
    Ok(SharedChannelPolicy {
        tenant_id: record.tenant_id,
        policy_id: record.policy_id.to_string(),
        connection_id: record.connection_id.to_string(),
        channel_id: record.channel_id,
        conversation_id: record.conversation_id,
        policy_version,
        history_visibility: record.history_visibility,
        status,
        applied_at: record.applied_at,
        updated_at: record.updated_at,
    })
}

// ---------------------------------------------------------------------------
// Sync request generation
// ---------------------------------------------------------------------------

fn non_empty_string(value: Option<&str>) -> Option<String> {
    value.and_then(|value| {
        if value.trim().is_empty() {
            None
        } else {
            Some(value.to_owned())
        }
    })
}

fn active_external_member_link_records_for_connection(
    state: &SocialControlState,
    tenant_id: &str,
    connection_id: &str,
) -> Vec<StoredExternalMemberLink> {
    let key = SocialConnectionIndexKey::new(tenant_id, connection_id);
    state
        .active_external_member_connection_index
        .get(&key)
        .into_iter()
        .flat_map(|link_ids| link_ids.iter())
        .filter_map(|link_id| {
            state
                .external_member_links
                .get(link_id)
                .filter(|record| record.external_member_link.status.is_active())
                .cloned()
        })
        .collect()
}

fn active_shared_channel_policy_records_for_connection(
    state: &SocialControlState,
    tenant_id: &str,
    connection_id: &str,
) -> Vec<StoredSharedChannelPolicy> {
    let key = SocialConnectionIndexKey::new(tenant_id, connection_id);
    state
        .active_shared_channel_policy_connection_index
        .get(&key)
        .into_iter()
        .flat_map(|policy_ids| policy_ids.iter())
        .filter_map(|policy_id| {
            state
                .shared_channel_policies
                .get(policy_id)
                .filter(|record| record.shared_channel_policy.status.is_active())
                .cloned()
        })
        .collect()
}

impl SocialRuntime {
    pub(crate) fn shared_channel_sync_requests_for_external_member_link(
        &self,
        state: &SocialControlState,
        organization_id: &str,
        link: &im_domain_core::social::ExternalMemberLink,
    ) -> Result<Vec<SharedChannelLinkedMemberSyncRequest>, String> {
        let policies = if let Some(store) = self.shared_channel_policy_authority_store() {
            let records = store
                .list_by_connection(
                    link.tenant_id.as_str(),
                    organization_id,
                    im_adapters_social_postgres::wire_id::parse_social_entity_id(
                        link.connection_id.as_str(),
                    )
                    .map_err(|error| format!("invalid external-connection id: {error:?}"))?,
                    "active",
                    SHARED_CHANNEL_SYNC_LOOKUP_LIMIT,
                )
                .map_err(|error| {
                    format!("normalized shared-channel-policy sync lookup failed: {error:?}")
                })?;
            ensure_complete_shared_channel_sync_lookup(records.len(), "shared-channel policies")?;
            records
                .into_iter()
                .map(shared_channel_policy_from_authority_record)
                .map(|result| {
                    result.map(|shared_channel_policy| StoredSharedChannelPolicy {
                        shared_channel_policy,
                        commits: Vec::new(),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            active_shared_channel_policy_records_for_connection(
                state,
                link.tenant_id.as_str(),
                link.connection_id.as_str(),
            )
        };

        Ok(policies
            .into_iter()
            .filter_map(|record| {
                let policy = &record.shared_channel_policy;
                let conversation_id = non_empty_string(policy.conversation_id.as_deref())?;
                if policy.history_visibility != "shared" {
                    return None;
                }

                Some(SharedChannelLinkedMemberSyncRequest {
                    tenant_id: link.tenant_id.clone(),
                    conversation_id,
                    shared_channel_policy_id: policy.policy_id.clone(),
                    external_connection_id: link.connection_id.clone(),
                    local_actor_id: link.local_actor_id.clone(),
                    local_actor_kind: link.local_actor_kind.clone(),
                    external_member_id: link.external_member_id.clone(),
                })
            })
            .collect())
    }

    fn shared_channel_sync_requests_for_shared_channel_policy(
        &self,
        state: &SocialControlState,
        organization_id: &str,
        policy: &SharedChannelPolicy,
    ) -> Result<Vec<SharedChannelLinkedMemberSyncRequest>, String> {
        let Some(conversation_id) = non_empty_string(policy.conversation_id.as_deref()) else {
            return Ok(Vec::new());
        };
        if !policy.status.is_active() || policy.history_visibility != "shared" {
            return Ok(Vec::new());
        }

        let member_links = if let Some(store) = self.external_member_link_authority_store() {
            let records = store
                .list_by_connection(
                    policy.tenant_id.as_str(),
                    organization_id,
                    im_adapters_social_postgres::wire_id::parse_social_entity_id(
                        policy.connection_id.as_str(),
                    )
                    .map_err(|error| format!("invalid external-connection id: {error:?}"))?,
                    "active",
                    SHARED_CHANNEL_SYNC_LOOKUP_LIMIT,
                )
                .map_err(|error| {
                    format!("normalized external-member-link sync lookup failed: {error:?}")
                })?;
            ensure_complete_shared_channel_sync_lookup(records.len(), "external member links")?;
            records
                .into_iter()
                .map(crate::external::external_member_link_from_authority_record)
                .map(|result| {
                    result.map(|external_member_link| StoredExternalMemberLink {
                        external_member_link,
                        commits: Vec::new(),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            active_external_member_link_records_for_connection(
                state,
                policy.tenant_id.as_str(),
                policy.connection_id.as_str(),
            )
        };

        Ok(member_links
            .into_iter()
            .filter_map(|record| {
                if record.external_member_link.status
                    != im_domain_core::social::ExternalMemberLinkStatus::Active
                {
                    return None;
                }
                Some(SharedChannelLinkedMemberSyncRequest {
                    tenant_id: policy.tenant_id.clone(),
                    conversation_id: conversation_id.clone(),
                    shared_channel_policy_id: policy.policy_id.clone(),
                    external_connection_id: policy.connection_id.clone(),
                    local_actor_id: record.external_member_link.local_actor_id.clone(),
                    local_actor_kind: record.external_member_link.local_actor_kind.clone(),
                    external_member_id: record.external_member_link.external_member_id.clone(),
                })
            })
            .collect())
    }
}

fn ensure_complete_shared_channel_sync_lookup(
    fetched: usize,
    resource: &str,
) -> Result<(), String> {
    if fetched > MAX_SHARED_CHANNEL_SYNC_TARGETS_PER_MUTATION {
        return Err(format!(
            "shared-channel sync requires more than {MAX_SHARED_CHANNEL_SYNC_TARGETS_PER_MUTATION} {resource}; use the durable paged sync worker"
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// HTTP handler functions
// ---------------------------------------------------------------------------

pub(crate) async fn apply_shared_channel_policy(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<ApplySharedChannelPolicyRequest>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        let applied = state.social_runtime.apply_shared_channel_policy(
            auth.tenant_id.as_str(),
            &auth,
            request,
        )?;

        state
            .social_runtime
            .dispatch_shared_channel_sync_requests(&applied.shared_channel_sync_requests)
            .map_err(|error| SocialServiceError::invalid("shared_channel_sync_failed", error))?;

        Ok(resource_item(SocialSharedChannelPolicyCommitResponse {
            status: SocialSharedChannelPolicyWriteStatus::Applied,
            shared_channel_policy: applied.shared_channel_policy,
            latest_commit: applied.latest_commit.into(),
            persistence: applied.persistence,
        }))
    })
    .await;
    crate::envelope::finish_created_enveloped_json(&ctx, result)
}

pub(crate) async fn shared_channel_policy_snapshot(
    Path(policy_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        state.social_runtime.acquire_cross_instance_read_lock()?;
        state
            .social_runtime
            .refresh_state_from_authority_for_read()?;
        let snapshot = state
            .social_runtime
            .shared_channel_policy_snapshot(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                policy_id.as_str(),
            )
            .map_err(|error| {
                SocialServiceError::dependency_unavailable(
                    "shared_channel_policy_store_unavailable",
                    error,
                )
            })?
            .ok_or_else(|| {
                SocialServiceError::not_found(
                    "shared_channel_policy_not_found",
                    format!("shared channel policy {policy_id} was not found"),
                )
            })?;

        Ok(resource_item(SocialSharedChannelPolicySnapshotResponse {
            status: SocialSharedChannelPolicyReadStatus::Snapshot,
            shared_channel_policy: snapshot.shared_channel_policy,
            commits: snapshot.commits.into_iter().map(Into::into).collect(),
        }))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}
