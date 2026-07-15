//! Open API contact tags, preferences, and recommendations (`/im/v3/api/social/contacts/*`).

use std::sync::{Arc, OnceLock};

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use im_app_context::AppContext;
use im_platform_contracts::IdGenerator;
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_runtime_id::{build_runtime_id_generator, build_runtime_id_generator_blocking};
use sdkwork_utils_rust::{SdkWorkCursorListQuery, cursor_list_page_data};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::api_payload::resource_item;
use crate::contact_open_api_backend::{
    ContactPreferencesRecord, ContactRecommendationRecord, ContactTagRecord,
    create_contact_recommendation as backend_create_contact_recommendation,
    delete_contact_tag as backend_delete_contact_tag, encode_contact_tag_inventory_cursor,
    get_contact_preferences as backend_get_contact_preferences,
    get_contact_tag as backend_get_contact_tag, list_contact_tags as backend_list_contact_tags,
    parse_contact_tag_inventory_cursor, shared_contact_store,
    upsert_contact_preferences as backend_upsert_contact_preferences,
    upsert_contact_tag as backend_upsert_contact_tag,
};
use crate::envelope::{finish_created_enveloped_json, finish_enveloped_json, finish_no_content};
use crate::friendship::{AppState, SocialServiceError};

static CONTACT_OPEN_API_ID_GENERATOR: OnceLock<Arc<dyn IdGenerator>> = OnceLock::new();

/// Initialize the contact open-api ID generator from the database.
///
/// Must be called during async service startup before any request is served.
/// If not called, the generator falls back to lazy env-based initialization.
pub async fn init_contact_open_api_id_generator() {
    if CONTACT_OPEN_API_ID_GENERATOR.get().is_some() {
        return;
    }
    let generator = build_runtime_id_generator("social-service").await;
    let _ = CONTACT_OPEN_API_ID_GENERATOR.set(generator);
}

fn id_generator() -> &'static dyn IdGenerator {
    CONTACT_OPEN_API_ID_GENERATOR
        .get_or_init(|| build_runtime_id_generator_blocking("social-service"))
        .as_ref()
}

fn next_entity_id() -> Result<String, SocialServiceError> {
    id_generator()
        .next_id()
        .map(|value| value.to_string())
        .map_err(|error| {
            SocialServiceError::invalid(
                "id_generation_failed",
                format!("contact open-api id generation failed: {error:?}"),
            )
        })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContactTagsListQuery {
    #[serde(flatten)]
    paging: SdkWorkCursorListQuery,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateContactTagRequest {
    name: String,
    color: String,
    count: Option<i32>,
    bg: Option<String>,
    border: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateContactTagRequest {
    name: Option<String>,
    color: Option<String>,
    count: Option<i32>,
    bg: Option<String>,
    border: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateContactPreferencesRequest {
    is_starred: Option<bool>,
    remark: Option<String>,
    is_blocked: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateContactRecommendationRequest {
    target_conversation_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactTagView {
    tenant_id: String,
    owner_user_id: String,
    tag_id: String,
    name: String,
    color: String,
    count: i32,
    bg: String,
    border: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactPreferencesView {
    tenant_id: String,
    owner_user_id: String,
    target_user_id: String,
    is_starred: bool,
    remark: String,
    is_blocked: bool,
    updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactRecommendationView {
    tenant_id: String,
    owner_user_id: String,
    target_user_id: String,
    recommendation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_conversation_id: Option<String>,
    created_at: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/im/v3/api/social/contacts/tags",
            get(list_contact_tags).post(create_contact_tag),
        )
        .route(
            "/im/v3/api/social/contacts/tags/{tag_id}",
            patch(update_contact_tag).delete(delete_contact_tag),
        )
        .route(
            "/im/v3/api/social/contacts/{target_user_id}/preferences",
            get(retrieve_contact_preferences).patch(update_contact_preferences),
        )
        .route(
            "/im/v3/api/social/contacts/{target_user_id}/recommendations",
            post(create_contact_recommendation),
        )
}

async fn list_contact_tags(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<ContactTagsListQuery>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |_state| {
        let paging = query.paging.resolve().map_err(|_| {
            SocialServiceError::invalid("cursor_invalid", "contact tag list cursor is invalid")
        })?;
        let cursor = if let Some(raw) = query.paging.cursor.as_deref() {
            Some(parse_contact_tag_inventory_cursor(raw)?)
        } else {
            None
        };
        let contact_store = shared_contact_store();
        let store = contact_store.as_ref();
        let (items, has_more) =
            backend_list_contact_tags(store, &auth, paging.page_size, cursor.as_ref())?;
        let next_cursor = if has_more {
            items
                .last()
                .map(encode_contact_tag_inventory_cursor)
                .transpose()?
        } else {
            None
        };
        let views = items.into_iter().map(ContactTagView::from).collect();
        Ok(cursor_list_page_data(
            views,
            paging.page_size,
            next_cursor,
            has_more,
        ))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn create_contact_tag(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CreateContactTagRequest>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |_state| {
        validate_tag_name(request.name.as_str())?;
        let now = utc_now_rfc3339_millis();
        let tag_id = next_entity_id()?;
        let owner_user_id = auth
            .ensure_user_actor_principal()
            .map_err(|error| {
                SocialServiceError::invalid("social_principal_invalid", error.message())
            })?
            .to_owned();
        let record = ContactTagRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            owner_user_id,
            tag_id: tag_id.clone(),
            name: request.name,
            color: request.color,
            count: request.count.unwrap_or(0),
            bg: request.bg.unwrap_or_default(),
            border: request.border.unwrap_or_default(),
            created_at: now.clone(),
            updated_at: now,
        };
        let contact_store = shared_contact_store();
        backend_upsert_contact_tag(contact_store.as_ref(), &auth, record.clone())?;
        Ok(resource_item(ContactTagView::from(record)))
    })
    .await;
    finish_created_enveloped_json(&ctx, result)
}

async fn update_contact_tag(
    Path(tag_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<UpdateContactTagRequest>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |_state| {
        if let Some(name) = request.name.as_deref() {
            validate_tag_name(name)?;
        }
        let contact_store = shared_contact_store();
        let store = contact_store.as_ref();
        let mut record =
            backend_get_contact_tag(store, &auth, tag_id.as_str())?.ok_or_else(|| {
                SocialServiceError::not_found(
                    "contact_tag_not_found",
                    format!("contact tag {tag_id} was not found"),
                )
            })?;
        if let Some(name) = request.name {
            record.name = name;
        }
        if let Some(color) = request.color {
            record.color = color;
        }
        if let Some(count) = request.count {
            record.count = count;
        }
        if let Some(bg) = request.bg {
            record.bg = bg;
        }
        if let Some(border) = request.border {
            record.border = border;
        }
        record.updated_at = utc_now_rfc3339_millis();
        backend_upsert_contact_tag(store, &auth, record.clone())?;
        Ok(resource_item(ContactTagView::from(record)))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn delete_contact_tag(
    Path(tag_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |_state| {
        let store_handle = shared_contact_store();
        let deleted = backend_delete_contact_tag(store_handle.as_ref(), &auth, tag_id.as_str())?;
        if !deleted {
            return Err(SocialServiceError::not_found(
                "contact_tag_not_found",
                format!("contact tag {tag_id} was not found"),
            ));
        }
        Ok(())
    })
    .await;
    finish_no_content(&ctx, result)
}

async fn retrieve_contact_preferences(
    Path(target_user_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        let store_handle = shared_contact_store();
        let mut record =
            backend_get_contact_preferences(store_handle.as_ref(), &auth, target_user_id.as_str())?;
        record.is_blocked = state.social_runtime.contact_is_blocked_all_scope(
            auth.tenant_id.as_str(),
            auth.social_principal_user_id(),
            target_user_id.as_str(),
        );
        Ok(resource_item(ContactPreferencesView::from(record)))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn update_contact_preferences(
    Path(target_user_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<UpdateContactPreferencesRequest>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |state| {
        let contact_store = shared_contact_store();
        let store = contact_store.as_ref();
        let mut record = backend_get_contact_preferences(store, &auth, target_user_id.as_str())?;
        if let Some(is_starred) = request.is_starred {
            record.is_starred = is_starred;
        }
        if let Some(remark) = request.remark {
            record.remark = remark;
        }
        if let Some(is_blocked) = request.is_blocked {
            state.social_runtime.sync_contact_block_preference(
                auth.tenant_id.as_str(),
                &auth,
                target_user_id.as_str(),
                is_blocked,
                next_entity_id()?,
                next_entity_id()?,
            )?;
        }
        record.is_blocked = state.social_runtime.contact_is_blocked_all_scope(
            auth.tenant_id.as_str(),
            auth.social_principal_user_id(),
            target_user_id.as_str(),
        );
        record.updated_at = utc_now_rfc3339_millis();
        backend_upsert_contact_preferences(store, &auth, record.clone())?;
        Ok(resource_item(ContactPreferencesView::from(record)))
    })
    .await;
    finish_enveloped_json(&ctx, result)
}

async fn create_contact_recommendation(
    Path(target_user_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CreateContactRecommendationRequest>,
) -> Response {
    let result = crate::envelope::run_blocking_social_call(state, move |_state| {
        let recommendation_id = next_entity_id()?;
        let record = backend_create_contact_recommendation(
            shared_contact_store().as_ref(),
            &auth,
            target_user_id.as_str(),
            recommendation_id.as_str(),
            request.target_conversation_id,
        )?;
        Ok(resource_item(ContactRecommendationView::from(record)))
    })
    .await;
    finish_created_enveloped_json(&ctx, result)
}

fn validate_tag_name(name: &str) -> Result<(), SocialServiceError> {
    if name.trim().is_empty() {
        return Err(SocialServiceError::invalid(
            "contact_tag_name_required",
            "contact tag name is required",
        ));
    }
    Ok(())
}

impl From<ContactTagRecord> for ContactTagView {
    fn from(record: ContactTagRecord) -> Self {
        Self {
            tenant_id: record.tenant_id,
            owner_user_id: record.owner_user_id,
            tag_id: record.tag_id,
            name: record.name,
            color: record.color,
            count: record.count,
            bg: record.bg,
            border: record.border,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

impl From<ContactPreferencesRecord> for ContactPreferencesView {
    fn from(record: ContactPreferencesRecord) -> Self {
        Self {
            tenant_id: record.tenant_id,
            owner_user_id: record.owner_user_id,
            target_user_id: record.target_user_id,
            is_starred: record.is_starred,
            remark: record.remark,
            is_blocked: record.is_blocked,
            updated_at: record.updated_at,
        }
    }
}

impl From<ContactRecommendationRecord> for ContactRecommendationView {
    fn from(record: ContactRecommendationRecord) -> Self {
        Self {
            tenant_id: record.tenant_id,
            owner_user_id: record.owner_user_id,
            target_user_id: record.target_user_id,
            recommendation_id: record.recommendation_id,
            target_conversation_id: record.target_conversation_id,
            created_at: record.created_at,
        }
    }
}
