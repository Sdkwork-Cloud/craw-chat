//! IAM-backed social user search for add-friend flows.

use axum::extract::{Extension, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use im_domain_core::social::normalize_user_pair;
use im_adapters_social_postgres::{postgres_pool_client, SocialPostgresPool};
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_utils_rust::{cursor_list_page_data, SdkWorkCursorListQuery, SdkWorkPageData};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::postgres::http::PostgresAppState;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchUsersQuery {
    pub q: Option<String>,
    #[serde(flatten)]
    pub paging: SdkWorkCursorListQuery,
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

#[derive(Debug, Clone)]
struct IamUserRow {
    user_id: String,
    username: String,
    display_name: String,
    email: Option<String>,
    phone: Option<String>,
}

pub async fn search_users(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Query(query): Query<SearchUsersQuery>,
) -> Response {
    let keyword = query.q.unwrap_or_default().trim().to_owned();
    let page_size = query
        .paging
        .resolve()
        .map(|params| params.page_size.clamp(1, 50))
        .unwrap_or(20);
    if keyword.is_empty() {
        let result: ApiResult<SdkWorkPageData<SocialUserSearchResult>> =
            Ok(cursor_list_page_data(Vec::new(), page_size, None, false));
        return finish_api_json(&ctx, result);
    }

    let limit = page_size as i64;
    let pool = state.postgres_pool.clone();
    let tenant_id = auth.tenant_id.clone();
    let organization_id = auth.organization_id.clone();
    let current_user_id = auth.actor_id.clone();
    let friendship_store = state.friendship_store.clone();
    let profile_store = state.user_profile_store.clone();
    let search_tenant_id = tenant_id.clone();

    let rows = match tokio::task::spawn_blocking(move || {
        search_iam_users(
            &pool,
            search_tenant_id.as_str(),
            keyword.as_str(),
            limit,
        )
    })
    .await
    {
        Ok(Ok(rows)) => rows,
        Ok(Err(_)) => {
            return finish_api_json::<SdkWorkPageData<SocialUserSearchResult>>(
                &ctx,
                Err(ApiProblem::dependency_unavailable("iam user search failed")),
            );
        }
        Err(_) => {
            return finish_api_json::<SdkWorkPageData<SocialUserSearchResult>>(
                &ctx,
                Err(ApiProblem::internal_server_error("iam user search worker panicked")),
            );
        }
    };

    let user_block_store = state.user_block_store.clone();
    let candidate_user_ids: Vec<String> = rows.iter().map(|row| row.user_id.clone()).collect();

    let relationship_context = match tokio::task::spawn_blocking({
        let friendship_store = friendship_store.clone();
        let user_block_store = user_block_store.clone();
        let tenant_id = tenant_id.clone();
        let organization_id = organization_id.clone();
        let current_user_id = current_user_id.clone();
        move || {
            let mut active_friend_ids = std::collections::HashSet::new();
            let mut blocked_user_ids = std::collections::HashSet::new();
            for user_id in candidate_user_ids {
                if user_id == current_user_id {
                    continue;
                }
                if blocked_user_ids.contains(user_id.as_str()) {
                    continue;
                }
                let pair = match normalize_user_pair(current_user_id.as_str(), user_id.as_str()) {
                    Ok(pair) => pair,
                    Err(_) => continue,
                };
                if user_block_store
                    .find_active_block(
                        tenant_id.as_str(),
                        organization_id.as_str(),
                        current_user_id.as_str(),
                        user_id.as_str(),
                        "social",
                    )
                    .ok()
                    .flatten()
                    .is_some()
                    || user_block_store
                        .find_active_block(
                            tenant_id.as_str(),
                            organization_id.as_str(),
                            user_id.as_str(),
                            current_user_id.as_str(),
                            "social",
                        )
                        .ok()
                        .flatten()
                        .is_some()
                {
                    blocked_user_ids.insert(user_id);
                    continue;
                }
                if friendship_store
                    .find_by_pair(
                        tenant_id.as_str(),
                        organization_id.as_str(),
                        pair.user_low_id.as_str(),
                        pair.user_high_id.as_str(),
                    )
                    .ok()
                    .flatten()
                    .is_some_and(|record| record.status == "active")
                {
                    active_friend_ids.insert(user_id);
                }
            }
            (active_friend_ids, blocked_user_ids)
        }
    })
    .await
    {
        Ok(context) => context,
        _ => (std::collections::HashSet::new(), std::collections::HashSet::new()),
    };
    let (active_friend_ids, blocked_user_ids) = relationship_context;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        if blocked_user_ids.contains(row.user_id.as_str()) {
            continue;
        }
        let relationship_state = if row.user_id == current_user_id {
            "self".to_owned()
        } else if active_friend_ids.contains(row.user_id.as_str()) {
            "active".to_owned()
        } else {
            "none".to_owned()
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

    let has_more = items.len() >= page_size;
    let result: ApiResult<SdkWorkPageData<SocialUserSearchResult>> =
        Ok(cursor_list_page_data(items, page_size, None, has_more));
    finish_api_json(&ctx, result)
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
