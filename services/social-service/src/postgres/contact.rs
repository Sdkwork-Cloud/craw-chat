//! Social contact inventory backed by normalized PostgreSQL relations.

use axum::extract::{Extension, Query, State};
use axum::response::Response;
use im_adapters_social_postgres::contact_inventory_store::{
    ContactInventoryQuery, ContactInventoryRecord,
};
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_utils_rust::{SdkWorkCursorListQuery, SdkWorkPageData, cursor_list_page_data};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::friendship::{decode_signed_inventory_cursor_payload, encode_signed_inventory_cursor};
use crate::postgres::access::social_principal_user_id;
use crate::postgres::http::PostgresAppState;

const CONTACT_CURSOR_VERSION: u8 = 1;
const CONTACT_LIST_MAX_PAGE_SIZE: usize = 200;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactInventoryCursor {
    version: u8,
    updated_at: String,
    friendship_id: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactResponse {
    tenant_id: String,
    owner_user_id: String,
    target_user_id: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
    chat_id: Option<String>,
    contact_type: &'static str,
    relationship_state: &'static str,
    friendship_id: String,
    direct_chat_id: Option<String>,
    conversation_id: Option<String>,
    established_at: String,
    last_interaction_at: String,
    is_starred: bool,
    is_blocked: bool,
    remark: Option<String>,
    updated_at: String,
}

impl From<ContactInventoryRecord> for ContactResponse {
    fn from(record: ContactInventoryRecord) -> Self {
        Self {
            tenant_id: record.tenant_id,
            owner_user_id: record.owner_user_id,
            target_user_id: record.target_user_id,
            display_name: record.display_name,
            avatar_url: record.avatar_url,
            chat_id: None,
            contact_type: "user",
            relationship_state: "active",
            friendship_id: record.friendship_id.to_string(),
            direct_chat_id: record.direct_chat_id.map(|value| value.to_string()),
            conversation_id: record.conversation_id,
            established_at: record.established_at,
            last_interaction_at: record.updated_at.clone(),
            is_starred: record.is_starred,
            is_blocked: record.is_blocked,
            remark: record.remark,
            updated_at: record.updated_at,
        }
    }
}

pub async fn list_contacts(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Query(query): Query<SdkWorkCursorListQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<ContactResponse>> =
        crate::postgres::http::run_blocking_postgres_call(state, move |state| {
            let page_size = query
                .resolve_page_size()
                .map_err(|_| ApiProblem::bad_request("page_size must be between 1 and 200"))?;
            if page_size > CONTACT_LIST_MAX_PAGE_SIZE {
                return Err(ApiProblem::bad_request(format!(
                    "page_size must be between 1 and {CONTACT_LIST_MAX_PAGE_SIZE}"
                )));
            }

            let cursor = query
                .cursor
                .as_deref()
                .map(decode_contact_cursor)
                .transpose()?;
            let owner_user_id = social_principal_user_id(&auth);
            let mut records = state
                .contact_inventory_store
                .list_contacts(ContactInventoryQuery {
                    tenant_id: auth.tenant_id.as_str(),
                    organization_id: auth.organization_id.as_str(),
                    owner_user_id,
                    cursor_updated_at: cursor.as_ref().map(|value| value.updated_at.as_str()),
                    cursor_friendship_id: cursor.as_ref().map(|value| value.friendship_id),
                    limit: i64::try_from(page_size.saturating_add(1)).unwrap_or(201),
                })
                .map_err(|_| ApiProblem::internal_server_error("failed to list social contacts"))?;

            let has_more = records.len() > page_size;
            if has_more {
                records.truncate(page_size);
            }
            let next_cursor = if has_more {
                records.last().map(encode_contact_cursor).transpose()?
            } else {
                None
            };
            let items = records.into_iter().map(ContactResponse::from).collect();
            Ok(cursor_list_page_data(
                items,
                page_size,
                next_cursor,
                has_more,
            ))
        })
        .await;
    finish_api_json(&ctx, result)
}

fn decode_contact_cursor(cursor: &str) -> Result<ContactInventoryCursor, ApiProblem> {
    let payload = decode_signed_inventory_cursor_payload(cursor)
        .map_err(|_| ApiProblem::bad_request("contact cursor is invalid"))?;
    let cursor: ContactInventoryCursor = serde_json::from_value(payload)
        .map_err(|_| ApiProblem::bad_request("contact cursor is invalid"))?;
    if cursor.version != CONTACT_CURSOR_VERSION
        || cursor.updated_at.trim().is_empty()
        || cursor.friendship_id <= 0
    {
        return Err(ApiProblem::bad_request("contact cursor is invalid"));
    }
    Ok(cursor)
}

fn encode_contact_cursor(record: &ContactInventoryRecord) -> Result<String, ApiProblem> {
    let payload = serde_json::to_value(ContactInventoryCursor {
        version: CONTACT_CURSOR_VERSION,
        updated_at: record.updated_at.clone(),
        friendship_id: record.friendship_id,
    })
    .map_err(|_| ApiProblem::internal_server_error("failed to encode contact cursor"))?;
    encode_signed_inventory_cursor(&payload)
        .map_err(|_| ApiProblem::internal_server_error("failed to encode contact cursor"))
}

#[cfg(test)]
mod tests {
    use super::{CONTACT_CURSOR_VERSION, ContactInventoryCursor};

    #[test]
    fn contact_cursor_wire_shape_is_versioned() {
        let value = serde_json::to_value(ContactInventoryCursor {
            version: CONTACT_CURSOR_VERSION,
            updated_at: "2026-07-22T00:00:00Z".into(),
            friendship_id: 42,
        })
        .expect("serialize contact cursor");
        assert_eq!(value["version"], CONTACT_CURSOR_VERSION);
        assert_eq!(value["friendshipId"], 42);
    }
}
