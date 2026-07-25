//! HTTP operator surfaces for shared-channel sync runtime control.

use axum::extract::{Extension, State};
use axum::response::Response;
use axum::{Json, Router};
use im_app_context::AppContext;
use sdkwork_web_core::WebRequestContext;
use serde::Deserialize;

use crate::api_payload::{full_inventory_page, resource_item};
use crate::control_access::{ensure_control_read_access, ensure_control_write_access};
use crate::envelope::finish_enveloped_json;
use crate::friendship::{AppState, SocialServiceError};
use crate::shared_channel_sync_runtime::SharedChannelSyncOwnerConflict;

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
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_read_access(&auth)?;
        let inventory = state.social_runtime.pending_shared_channel_sync_inventory(
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        Ok(full_inventory_page(inventory.items))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn list_dead_letter_shared_channel_sync(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_read_access(&auth)?;
        let inventory = state
            .social_runtime
            .dead_letter_shared_channel_sync_inventory(
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            );
        Ok(full_inventory_page(inventory.items))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn list_delivered_shared_channel_sync(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_read_access(&auth)?;
        let inventory = state
            .social_runtime
            .delivered_shared_channel_sync_inventory();
        Ok(full_inventory_page(inventory.items))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn list_delivery_state_shared_channel_sync(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_read_access(&auth)?;
        let inventory = state
            .social_runtime
            .delivery_state_shared_channel_sync_inventory();
        Ok(resource_item(inventory))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn reclaim_stale_pending_shared_channel_sync(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .reclaim_stale_pending_shared_channel_sync_claims_persisted()
            .map(resource_item)
            .map_err(|error| {
                SocialServiceError::invalid("shared_channel_sync_reclaim_failed", error)
            })
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn repair_shared_channel_sync(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .repair_shared_channel_sync()
            .map(resource_item)
            .map_err(|error| {
                SocialServiceError::invalid("shared_channel_sync_repair_failed", error)
            })
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn requeue_dead_letter_shared_channel_sync(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .requeue_dead_letter_shared_channel_sync_persisted(None)
            .map(resource_item)
            .map_err(|error| {
                SocialServiceError::invalid("shared_channel_sync_requeue_failed", error)
            })
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn requeue_dead_letter_shared_channel_sync_targeted(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .requeue_dead_letter_shared_channel_sync_persisted(Some(body.request_keys.as_slice()))
            .map(resource_item)
            .map_err(|error| {
                SocialServiceError::invalid("shared_channel_sync_requeue_failed", error)
            })
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn claim_pending_shared_channel_sync_targeted(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .claim_pending_shared_channel_sync_targeted_persisted(
                body.request_keys.as_slice(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            )
            .map(resource_item)
            .map_err(owner_conflict_into_service_error)
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn release_pending_shared_channel_sync_targeted(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .release_pending_shared_channel_sync_targeted_persisted(
                body.request_keys.as_slice(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            )
            .map(resource_item)
            .map_err(owner_conflict_into_service_error)
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn takeover_pending_shared_channel_sync_targeted(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(body): Json<TargetedTakeoverBody>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .takeover_pending_shared_channel_sync_targeted_persisted(
                body.request_keys.as_slice(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                body.legacy_override,
            )
            .map(resource_item)
            .map_err(owner_conflict_into_service_error)
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn republish_pending_shared_channel_sync_targeted(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(body): Json<TargetedRequestKeysBody>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        ensure_control_write_access(&auth)?;
        state
            .social_runtime
            .republish_pending_shared_channel_sync_targeted(
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                body.request_keys,
            )
            .map(resource_item)
            .map_err(owner_conflict_into_service_error)
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

fn owner_conflict_into_service_error(error: SharedChannelSyncOwnerConflict) -> SocialServiceError {
    SocialServiceError::conflict_with_details(error.code, error.message, error.details)
}
