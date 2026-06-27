//! IAM-backed social user search for add-friend flows.

use axum::Json;
use axum::extract::{Extension, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use im_adapters_social_postgres::{postgres_pool_client, SocialPostgresPool};
use serde::{Deserialize, Serialize};

use crate::postgres::http::PostgresAppState;
use crate::postgres::service_http::require_request_scope;

#[derive(Debug, Deserialize)]
pub struct SearchUsersQuery {
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialUserSearchResult {
    pub tenant_id: String,
    pub user_id: String,
    pub chat_id: String,
    pub display_name: String,
    pub relationship_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialUserSearchResponse {
    pub items: Vec<SocialUserSearchResult>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
struct IamUserRow {
    user_id: String,
    username: String,
    display_name: String,
    email: Option<String>,
    phone: Option<String>,
}

pub async fn search_users(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<SearchUsersQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    let keyword = query.q.unwrap_or_default().trim().to_owned();
    if keyword.is_empty() {
        return Ok(Json(SocialUserSearchResponse {
            items: Vec::new(),
            has_more: false,
            next_cursor: None,
        }));
    }

    let limit = query.limit.unwrap_or(20).clamp(1, 50);
    let pool = state.postgres_pool.clone();
    let tenant_id = scope.tenant_id.clone();
    let organization_id = scope.organization_id.clone();
    let current_user_id = scope.user_id.clone();
    let friendship_store = state.friendship_store.clone();
    let profile_store = state.user_profile_store.clone();
    let search_tenant_id = tenant_id.clone();

    let rows = tokio::task::spawn_blocking(move || {
        search_iam_users(
            &pool,
            search_tenant_id.as_str(),
            keyword.as_str(),
            limit,
        )
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let relationship_state = if row.user_id == current_user_id {
            "self".to_owned()
        } else {
            resolve_relationship_state(
                friendship_store.as_ref(),
                tenant_id.as_str(),
                organization_id.as_str(),
                current_user_id.as_str(),
                row.user_id.as_str(),
            )
        };

        let display_name = row.display_name.trim();
        let display_name = if display_name.is_empty() {
            row.username.clone()
        } else {
            display_name.to_owned()
        };

        let avatar_url = profile_store
            .get_by_user_id(
                tenant_id.as_str(),
                organization_id.as_str(),
                row.user_id.as_str(),
            )
            .ok()
            .flatten()
            .and_then(|profile| profile.im_avatar_url);

        items.push(SocialUserSearchResult {
            tenant_id: tenant_id.clone(),
            user_id: row.user_id.clone(),
            chat_id: resolve_chat_id(row.username.as_str(), row.user_id.as_str()),
            display_name,
            relationship_state,
            avatar_url,
            email: row.email,
            phone: row.phone,
        });
    }

    Ok(Json(SocialUserSearchResponse {
        items,
        has_more: false,
        next_cursor: None,
    }))
}

fn search_iam_users(
    pool: &SocialPostgresPool,
    tenant_id: &str,
    keyword: &str,
    limit: i64,
) -> Result<Vec<IamUserRow>, im_platform_contracts::ContractError> {
    let pool = pool.inner();
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                let mut client = postgres_pool_client(pool, "iam user search")?;
                let pattern = format!("%{keyword}%");
                let exact = keyword;
                let rows = client
                    .query(
                        r#"
SELECT id, username, display_name, email, phone
FROM iam_user
WHERE tenant_id = $1
  AND is_deleted = 0
  AND (
    id = $2
    OR username ILIKE $3
    OR display_name ILIKE $3
    OR COALESCE(email, '') ILIKE $3
    OR COALESCE(phone, '') ILIKE $3
  )
ORDER BY display_name, username, id
LIMIT $4
"#,
                        &[&tenant_id, &exact, &pattern, &limit],
                    )
                    .map_err(|error| {
                        im_platform_contracts::ContractError::Unavailable(format!(
                            "iam user search failed: {error}"
                        ))
                    })?;

                Ok(rows
                    .iter()
                    .map(|row| IamUserRow {
                        user_id: row.get("id"),
                        username: row.get("username"),
                        display_name: row.get("display_name"),
                        email: row.get("email"),
                        phone: row.get("phone"),
                    })
                    .collect())
            })
            .join()
            .map_err(|_| {
                im_platform_contracts::ContractError::Unavailable(
                    "iam user search worker panicked".into(),
                )
            })?
    })
}

fn resolve_relationship_state(
    friendship_store: &dyn im_adapters_social_postgres::friendship_store::FriendshipStore,
    tenant_id: &str,
    organization_id: &str,
    current_user_id: &str,
    target_user_id: &str,
) -> String {
    let (user_low_id, user_high_id) = canonical_user_pair(current_user_id, target_user_id);
    match friendship_store.find_by_pair(
        tenant_id,
        organization_id,
        user_low_id.as_str(),
        user_high_id.as_str(),
    ) {
        Ok(Some(record)) if record.status == "active" => "active".to_owned(),
        _ => "none".to_owned(),
    }
}

fn canonical_user_pair(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_owned(), right.to_owned())
    } else {
        (right.to_owned(), left.to_owned())
    }
}

fn resolve_chat_id(username: &str, user_id: &str) -> String {
    let normalized = username.trim().to_ascii_lowercase();
    if is_valid_chat_id(normalized.as_str()) {
        return normalized;
    }

    let mut slug: String = user_id
        .chars()
        .filter_map(|character| {
            if character.is_ascii_alphanumeric() {
                Some(character.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();
    if slug.is_empty() || !slug.starts_with(|character: char| character.is_ascii_lowercase()) {
        slug = format!("u{slug}");
    }
    if slug.len() > 24 {
        slug.truncate(24);
    }
    while slug.len() < 6 {
        slug.push('0');
    }
    slug
}

fn is_valid_chat_id(value: &str) -> bool {
    let Some(first) = value.chars().next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && (6..=24).contains(&value.len())
        && value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::{is_valid_chat_id, resolve_chat_id};

    #[test]
    fn resolve_chat_id_prefers_valid_username() {
        assert_eq!(resolve_chat_id("cc8k2m7q4x9p", "1138"), "cc8k2m7q4x9p");
    }

    #[test]
    fn resolve_chat_id_falls_back_to_user_id_slug() {
        let chat_id = resolve_chat_id("ALICE", "1138");
        assert!(is_valid_chat_id(chat_id.as_str()));
        assert!(chat_id.starts_with('u'));
    }
}
