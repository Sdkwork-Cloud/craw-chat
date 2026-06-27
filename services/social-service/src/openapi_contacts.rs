//! Open API contact tags, preferences, and recommendations (`/im/v3/api/social/contacts/*`).

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, patch, post};
use axum::Router;
use im_app_context::AppContext;
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;
use serde::{Deserialize, Serialize};

use crate::friendship::{self, AppState, SocialServiceError};

static CONTACT_OPEN_API_ID_GENERATOR: OnceLock<RuntimeSnowflakeIdGenerator> = OnceLock::new();
static CONTACT_OPEN_API_STORE: OnceLock<RwLock<ContactOpenApiStore>> = OnceLock::new();

/// Initialize the contact open-api ID generator from the database.
///
/// Must be called during async service startup before any request is served.
/// If not called, the generator falls back to lazy env-based initialization.
pub async fn init_contact_open_api_id_generator() {
    if CONTACT_OPEN_API_ID_GENERATOR.get().is_some() {
        return;
    }
    let generator = RuntimeSnowflakeIdGenerator::from_database_env("social-service")
        .await
        .unwrap_or_else(|error| {
            tracing::warn!(
                ?error,
                "database node_id allocation failed; falling back to env for social contact open-api"
            );
            RuntimeSnowflakeIdGenerator::from_env().unwrap_or_else(|_| {
                RuntimeSnowflakeIdGenerator::with_node_id(0)
                    .expect("snowflake node 0 must initialize")
            })
        });
    let _ = CONTACT_OPEN_API_ID_GENERATOR.set(generator);
}

fn id_generator() -> &'static RuntimeSnowflakeIdGenerator {
    CONTACT_OPEN_API_ID_GENERATOR.get_or_init(|| {
        // Fallback for lazy init (e.g., in tests without database)
        RuntimeSnowflakeIdGenerator::from_env().unwrap_or_else(|_| {
            RuntimeSnowflakeIdGenerator::with_node_id(0).expect("snowflake node 0 must initialize")
        })
    })
}

fn store() -> &'static RwLock<ContactOpenApiStore> {
    CONTACT_OPEN_API_STORE.get_or_init(|| RwLock::new(ContactOpenApiStore::default()))
}

fn next_entity_id() -> Result<String, SocialServiceError> {
    id_generator()
        .next_id()
        .map(|value| value.to_string())
        .map_err(|error| {
            SocialServiceError::invalid(
                "id_generation_failed",
                format!("contact open-api id generation failed: {error}"),
            )
        })
}

#[derive(Default)]
struct ContactOpenApiStore {
    tags: HashMap<TagKey, ContactTagRecord>,
    preferences: HashMap<PreferenceKey, ContactPreferencesRecord>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TagKey {
    tenant_id: String,
    owner_user_id: String,
    tag_id: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PreferenceKey {
    tenant_id: String,
    owner_user_id: String,
    target_user_id: String,
}

#[derive(Clone, Debug)]
struct ContactTagRecord {
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

#[derive(Clone, Debug)]
struct ContactPreferencesRecord {
    tenant_id: String,
    owner_user_id: String,
    target_user_id: String,
    is_starred: bool,
    remark: String,
    is_blocked: bool,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContactTagsListQuery {
    limit: Option<i64>,
    cursor: Option<String>,
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
struct ContactTagsResponse {
    items: Vec<ContactTagView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
    has_more: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteContactTagResponse {
    tag_id: String,
    deleted: bool,
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
    headers: HeaderMap,
    Query(query): Query<ContactTagsListQuery>,
    State(_state): State<AppState>,
) -> Result<Json<ContactTagsResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let limit = query.limit.unwrap_or(100).clamp(1, 200) as usize;
    let mut items = store()
        .read()
        .map_err(|_| store_lock_error())?
        .tags
        .values()
        .filter(|tag| tag.tenant_id == auth.tenant_id && tag.owner_user_id == auth.user_id)
        .cloned()
        .map(ContactTagView::from)
        .collect::<Vec<_>>();
    items.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
    let has_more = items.len() > limit;
    items.truncate(limit);
    Ok(Json(ContactTagsResponse {
        items,
        next_cursor: query.cursor,
        has_more,
    }))
}

async fn create_contact_tag(
    headers: HeaderMap,
    State(_state): State<AppState>,
    Json(request): Json<CreateContactTagRequest>,
) -> Result<Json<ContactTagView>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    validate_tag_name(request.name.as_str())?;
    let now = utc_now_rfc3339_millis();
    let tag_id = next_entity_id()?;
    let record = ContactTagRecord {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.user_id.clone(),
        tag_id: tag_id.clone(),
        name: request.name,
        color: request.color,
        count: request.count.unwrap_or(0),
        bg: request.bg.unwrap_or_default(),
        border: request.border.unwrap_or_default(),
        created_at: now.clone(),
        updated_at: now,
    };
    store()
        .write()
        .map_err(|_| store_lock_error())?
        .tags
        .insert(
            TagKey {
                tenant_id: auth.tenant_id.clone(),
                owner_user_id: auth.user_id.clone(),
                tag_id,
            },
            record.clone(),
        );
    Ok(Json(ContactTagView::from(record)))
}

async fn update_contact_tag(
    Path(tag_id): Path<String>,
    headers: HeaderMap,
    State(_state): State<AppState>,
    Json(request): Json<UpdateContactTagRequest>,
) -> Result<Json<ContactTagView>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    if let Some(name) = request.name.as_deref() {
        validate_tag_name(name)?;
    }
    let key = TagKey {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.user_id.clone(),
        tag_id: tag_id.clone(),
    };
    let mut guard = store().write().map_err(|_| store_lock_error())?;
    let record = guard.tags.get_mut(&key).ok_or_else(|| {
        SocialServiceError::not_found("contact_tag_not_found", format!("contact tag {tag_id} was not found"))
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
    Ok(Json(ContactTagView::from(record.clone())))
}

async fn delete_contact_tag(
    Path(tag_id): Path<String>,
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<DeleteContactTagResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let key = TagKey {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.user_id.clone(),
        tag_id: tag_id.clone(),
    };
    let removed = store()
        .write()
        .map_err(|_| store_lock_error())?
        .tags
        .remove(&key)
        .is_some();
    if !removed {
        return Err(SocialServiceError::not_found(
            "contact_tag_not_found",
            format!("contact tag {tag_id} was not found"),
        ));
    }
    Ok(Json(DeleteContactTagResponse {
        tag_id,
        deleted: true,
    }))
}

async fn retrieve_contact_preferences(
    Path(target_user_id): Path<String>,
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<ContactPreferencesView>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let key = PreferenceKey {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.user_id.clone(),
        target_user_id: target_user_id.clone(),
    };
    let record = store()
        .read()
        .map_err(|_| store_lock_error())?
        .preferences
        .get(&key)
        .cloned()
        .unwrap_or_else(|| default_preferences(&auth, target_user_id.as_str()));
    Ok(Json(ContactPreferencesView::from(record)))
}

async fn update_contact_preferences(
    Path(target_user_id): Path<String>,
    headers: HeaderMap,
    State(_state): State<AppState>,
    Json(request): Json<UpdateContactPreferencesRequest>,
) -> Result<Json<ContactPreferencesView>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let key = PreferenceKey {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.user_id.clone(),
        target_user_id: target_user_id.clone(),
    };
    let mut guard = store().write().map_err(|_| store_lock_error())?;
    let record = guard
        .preferences
        .entry(key)
        .or_insert_with(|| default_preferences(&auth, target_user_id.as_str()));
    if let Some(is_starred) = request.is_starred {
        record.is_starred = is_starred;
    }
    if let Some(remark) = request.remark {
        record.remark = remark;
    }
    if let Some(is_blocked) = request.is_blocked {
        record.is_blocked = is_blocked;
    }
    record.updated_at = utc_now_rfc3339_millis();
    Ok(Json(ContactPreferencesView::from(record.clone())))
}

async fn create_contact_recommendation(
    Path(target_user_id): Path<String>,
    headers: HeaderMap,
    State(_state): State<AppState>,
    Json(request): Json<CreateContactRecommendationRequest>,
) -> Result<Json<ContactRecommendationView>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    Ok(Json(ContactRecommendationView {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.user_id.clone(),
        target_user_id,
        recommendation_id: next_entity_id()?,
        target_conversation_id: request.target_conversation_id,
        created_at: utc_now_rfc3339_millis(),
    }))
}

fn default_preferences(auth: &AppContext, target_user_id: &str) -> ContactPreferencesRecord {
    ContactPreferencesRecord {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.user_id.clone(),
        target_user_id: target_user_id.to_owned(),
        is_starred: false,
        remark: String::new(),
        is_blocked: false,
        updated_at: utc_now_rfc3339_millis(),
    }
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

fn store_lock_error() -> SocialServiceError {
    SocialServiceError::invalid("contact_store_unavailable", "contact open-api store lock failed")
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
