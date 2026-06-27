//! HTTP operator surfaces for shared-channel sync runtime control.

use axum::extract::State;
use axum::{Json, Router};
use serde::Deserialize;

use crate::friendship::{AppState, SocialServiceError};
use crate::runtime::SocialRuntimeRepairResponse;
use crate::shared_channel_sync_runtime::{
    SharedChannelSyncOwnerConflict, SocialSharedChannelSyncDeadLetterInventoryResponse,
    SocialSharedChannelSyncDeadLetterRequeueResponse,
    SocialSharedChannelSyncDeliveredInventoryResponse,
    SocialSharedChannelSyncDeliveryStateInventoryResponse,
    SocialSharedChannelSyncPendingInventoryResponse,
    SocialSharedChannelSyncPendingStaleReclaimResponse, SocialSharedChannelSyncRepairResponse,
    SocialSharedChannelSyncRepublishResponse, SocialSharedChannelSyncTargetedMutationResponse,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TargetedRequestKeysBody {
    pub request_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TargetedTakeoverBody {
    pub request_keys: Vec<String>,
    #[serde(default)]
    pub legacy_override: bool,
}

pub fn build_runtime_control_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/backend/v3/api/control/social/runtime/pending_shared_channel_sync",
            axum::routing::get(list_pending_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync",
            axum::routing::get(list_dead_letter_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/delivered_shared_channel_sync",
            axum::routing::get(list_delivered_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync",
            axum::routing::get(list_delivery_state_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/reclaim_stale_pending_shared_channel_sync",
            axum::routing::post(reclaim_stale_pending_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/repair_derived_snapshot",
            axum::routing::post(repair_derived_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/runtime/repair_shared_channel_sync",
            axum::routing::post(repair_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync",
            axum::routing::post(requeue_dead_letter_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted",
            axum::routing::post(requeue_dead_letter_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted",
            axum::routing::post(claim_pending_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/release_pending_shared_channel_sync_targeted",
            axum::routing::post(release_pending_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/takeover_pending_shared_channel_sync_targeted",
            axum::routing::post(takeover_pending_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/republish_pending_shared_channel_sync_targeted",
            axum::routing::post(republish_pending_shared_channel_sync_targeted),
        )
}

async fn list_pending_shared_channel_sync(
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncPendingInventoryResponse>, SocialServiceError> {
    Ok(Json(
        state
            .social_runtime
            .pending_shared_channel_sync_inventory("system", "system"),
    ))
}

async fn list_dead_letter_shared_channel_sync(
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterInventoryResponse>, SocialServiceError> {
    Ok(Json(
        state
            .social_runtime
            .dead_letter_shared_channel_sync_inventory("system", "system"),
    ))
}

async fn list_delivered_shared_channel_sync(
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeliveredInventoryResponse>, SocialServiceError> {
    Ok(Json(
        state.social_runtime.delivered_shared_channel_sync_inventory(),
    ))
}

async fn list_delivery_state_shared_channel_sync(
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeliveryStateInventoryResponse>, SocialServiceError> {
    Ok(Json(
        state
            .social_runtime
            .delivery_state_shared_channel_sync_inventory(),
    ))
}

async fn reclaim_stale_pending_shared_channel_sync(
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncPendingStaleReclaimResponse>, SocialServiceError> {
    state
        .social_runtime
        .reclaim_stale_pending_shared_channel_sync_claims_persisted()
        .map(Json)
        .map_err(|error| SocialServiceError::invalid("shared_channel_sync_reclaim_failed", error))
}

async fn repair_derived_snapshot(
    State(state): State<AppState>,
) -> Result<Json<SocialRuntimeRepairResponse>, SocialServiceError> {
    state
        .social_runtime
        .repair_derived_snapshot()
        .map(Json)
        .map_err(|error| SocialServiceError::invalid("social_runtime_repair_failed", error))
}

async fn repair_shared_channel_sync(
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncRepairResponse>, SocialServiceError> {
    state
        .social_runtime
        .repair_shared_channel_sync()
        .map(Json)
        .map_err(|error| SocialServiceError::invalid("shared_channel_sync_repair_failed", error))
}

async fn requeue_dead_letter_shared_channel_sync(
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterRequeueResponse>, SocialServiceError> {
    state
        .social_runtime
        .requeue_dead_letter_shared_channel_sync_persisted(None)
        .map(Json)
        .map_err(|error| SocialServiceError::invalid("shared_channel_sync_requeue_failed", error))
}

async fn requeue_dead_letter_shared_channel_sync_targeted(
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterRequeueResponse>, SocialServiceError> {
    state
        .social_runtime
        .requeue_dead_letter_shared_channel_sync_persisted(Some(body.request_keys.as_slice()))
        .map(Json)
        .map_err(|error| SocialServiceError::invalid("shared_channel_sync_requeue_failed", error))
}

async fn claim_pending_shared_channel_sync_targeted(
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Result<Json<SocialSharedChannelSyncTargetedMutationResponse>, SocialServiceError> {
    state
        .social_runtime
        .claim_pending_shared_channel_sync_targeted_persisted(
            body.request_keys.as_slice(),
            "system",
            "system",
        )
        .map(Json)
        .map_err(owner_conflict_into_service_error)
}

async fn release_pending_shared_channel_sync_targeted(
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Result<Json<SocialSharedChannelSyncTargetedMutationResponse>, SocialServiceError> {
    state
        .social_runtime
        .release_pending_shared_channel_sync_targeted_persisted(
            body.request_keys.as_slice(),
            "system",
            "system",
        )
        .map(Json)
        .map_err(owner_conflict_into_service_error)
}

async fn takeover_pending_shared_channel_sync_targeted(
    State(state): State<AppState>,
    Json(body): Json<TargetedTakeoverBody>,
) -> Result<Json<SocialSharedChannelSyncTargetedMutationResponse>, SocialServiceError> {
    state
        .social_runtime
        .takeover_pending_shared_channel_sync_targeted_persisted(
            body.request_keys.as_slice(),
            "system",
            "system",
            body.legacy_override,
        )
        .map(Json)
        .map_err(owner_conflict_into_service_error)
}

async fn republish_pending_shared_channel_sync_targeted(
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Result<Json<SocialSharedChannelSyncRepublishResponse>, SocialServiceError> {
    state
        .social_runtime
        .republish_pending_shared_channel_sync_targeted("system", "system", body.request_keys)
        .map(Json)
        .map_err(owner_conflict_into_service_error)
}

fn owner_conflict_into_service_error(error: SharedChannelSyncOwnerConflict) -> SocialServiceError {
    SocialServiceError::conflict_with_details(error.code, error.message, error.details)
}
