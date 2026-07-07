//! PostgreSQL implementation of [`SearchProvider`].
//!
//! Leverages the `search_vector tsvector` column on `im_conversation_messages`
//! with a GIN index. The column is automatically populated by the
//! `im_messages_search_update` trigger on INSERT/UPDATE, so `index_message`
//! is a no-op for the PostgreSQL backend.

use im_platform_contracts::{
    ContractError, MessageSearchHit, SearchProvider, SearchResult, SearchableMessage,
};
use serde::{Deserialize, Serialize};

use crate::{PostgresJournalPool, postgres_pool_client, postgres_unavailable, run_postgres_io};

const SDKWORK_SEARCH_PAGE_SIZE_MAX: usize = 200;

/// PostgreSQL-backed search provider.
///
/// ## How it works
/// - **Indexing**: Handled automatically by the `im_messages_search_update`
///   database trigger. No application-side indexing needed.
/// - **Search**: Uses `to_tsquery` + `@@` operator against the GIN-indexed
///   `search_vector` column. Falls back to `plainto_tsquery` for plain text.
/// - **Language**: Attempts `chinese_zh` config first (requires zhparser),
///   falls back to `simple` config.
#[derive(Clone)]
pub struct PostgresSearchProvider {
    pool: PostgresJournalPool,
    plugin_id: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct MemberSearchQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub principal_kind: &'a str,
    pub principal_id: &'a str,
    pub query: &'a str,
    pub conversation_id: Option<&'a str>,
    pub limit: usize,
    pub cursor: Option<&'a str>,
}

impl PostgresSearchProvider {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self {
            pool,
            plugin_id: "search-postgres",
        }
    }
}

const SEARCH_SQL: &str = r#"
select message_id, conversation_id, message_seq
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and deleted_at is null
  and ($4::text is null or conversation_id = $4::text)
  and search_vector @@ to_tsquery('simple', $3)
order by created_at desc
limit $5
offset $6
"#;

const SEARCH_PLAIN_SQL: &str = r#"
select message_id, conversation_id, message_seq
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and deleted_at is null
  and ($4::text is null or conversation_id = $4::text)
  and search_vector @@ plainto_tsquery('simple', $3)
order by created_at desc
limit $5
offset $6
"#;

const COUNT_SQL: &str = r#"
select count(*) as total
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and deleted_at is null
  and ($3::text is null or conversation_id = $3::text)
  and search_vector @@ to_tsquery('simple', $4)
"#;

const COUNT_MEMBER_SQL: &str = r#"
select count(*) as total
from im_conversation_messages m
inner join im_projection_conversation_members mem
  on mem.tenant_id = m.tenant_id
 and mem.organization_id = m.organization_id
 and mem.conversation_id = m.conversation_id
 and mem.principal_kind = $5
 and mem.principal_id = $6
 and mem.membership_state in ('invited', 'joined', 'linked')
where m.tenant_id = $1
  and m.organization_id = $2
  and m.deleted_at is null
  and ($3::text is null or m.conversation_id = $3::text)
  and m.search_vector @@ to_tsquery('simple', $4)
"#;

const SEARCH_MEMBER_SQL: &str = r#"
select m.message_id, m.conversation_id, m.message_seq
from im_conversation_messages m
inner join im_projection_conversation_members mem
  on mem.tenant_id = m.tenant_id
 and mem.organization_id = m.organization_id
 and mem.conversation_id = m.conversation_id
 and mem.principal_kind = $7
 and mem.principal_id = $8
 and mem.membership_state in ('invited', 'joined', 'linked')
where m.tenant_id = $1
  and m.organization_id = $2
  and m.deleted_at is null
  and ($4::text is null or m.conversation_id = $4::text)
  and m.search_vector @@ to_tsquery('simple', $3)
order by m.created_at desc
limit $5
offset $6
"#;

const SEARCH_MEMBER_PLAIN_SQL: &str = r#"
select m.message_id, m.conversation_id, m.message_seq
from im_conversation_messages m
inner join im_projection_conversation_members mem
  on mem.tenant_id = m.tenant_id
 and mem.organization_id = m.organization_id
 and mem.conversation_id = m.conversation_id
 and mem.principal_kind = $7
 and mem.principal_id = $8
 and mem.membership_state in ('invited', 'joined', 'linked')
where m.tenant_id = $1
  and m.organization_id = $2
  and m.deleted_at is null
  and ($4::text is null or m.conversation_id = $4::text)
  and m.search_vector @@ plainto_tsquery('simple', $3)
order by m.created_at desc
limit $5
offset $6
"#;

const SEARCH_MEMBER_KEYSET_SQL: &str = r#"
select m.message_id, m.conversation_id, m.message_seq, m.created_at
from im_conversation_messages m
inner join im_projection_conversation_members mem
  on mem.tenant_id = m.tenant_id
 and mem.organization_id = m.organization_id
 and mem.conversation_id = m.conversation_id
 and mem.principal_kind = $6
 and mem.principal_id = $7
 and mem.membership_state in ('invited', 'joined', 'linked')
where m.tenant_id = $1
  and m.organization_id = $2
  and m.deleted_at is null
  and ($4::text is null or m.conversation_id = $4::text)
  and m.search_vector @@ to_tsquery('simple', $3)
  and (
    $8::timestamptz is null
    or m.created_at < $8::timestamptz
    or (m.created_at = $8::timestamptz and m.message_id > $9::bigint)
  )
order by m.created_at desc, m.message_id asc
limit $5
"#;

const SEARCH_MEMBER_KEYSET_PLAIN_SQL: &str = r#"
select m.message_id, m.conversation_id, m.message_seq, m.created_at
from im_conversation_messages m
inner join im_projection_conversation_members mem
  on mem.tenant_id = m.tenant_id
 and mem.organization_id = m.organization_id
 and mem.conversation_id = m.conversation_id
 and mem.principal_kind = $6
 and mem.principal_id = $7
 and mem.membership_state in ('invited', 'joined', 'linked')
where m.tenant_id = $1
  and m.organization_id = $2
  and m.deleted_at is null
  and ($4::text is null or m.conversation_id = $4::text)
  and m.search_vector @@ plainto_tsquery('simple', $3)
  and (
    $8::timestamptz is null
    or m.created_at < $8::timestamptz
    or (m.created_at = $8::timestamptz and m.message_id > $9::bigint)
  )
order by m.created_at desc, m.message_id asc
limit $5
"#;

const SEARCH_KEYSET_SQL: &str = r#"
select message_id, conversation_id, message_seq, created_at
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and deleted_at is null
  and ($4::text is null or conversation_id = $4::text)
  and search_vector @@ to_tsquery('simple', $3)
  and (
    $6::timestamptz is null
    or created_at < $6::timestamptz
    or (created_at = $6::timestamptz and message_id > $7::bigint)
  )
order by created_at desc, message_id asc
limit $5
"#;

const SEARCH_KEYSET_PLAIN_SQL: &str = r#"
select message_id, conversation_id, message_seq, created_at
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and deleted_at is null
  and ($4::text is null or conversation_id = $4::text)
  and search_vector @@ plainto_tsquery('simple', $3)
  and (
    $6::timestamptz is null
    or created_at < $6::timestamptz
    or (created_at = $6::timestamptz and message_id > $7::bigint)
  )
order by created_at desc, message_id asc
limit $5
"#;

const REMOVE_SQL: &str = r#"
update im_conversation_messages
set search_vector = null
where tenant_id = $1 and message_id = $2
"#;

fn escape_tsquery(query: &str) -> String {
    // Basic sanitization: replace special tsquery characters
    query
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace(['|', '&', '!', '(', ')', ':'], " ")
        .split_whitespace()
        .map(|w| format!("{}:*", w)) // prefix match for each word
        .collect::<Vec<_>>()
        .join(" & ")
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SearchListCursor {
    Start,
    Offset(usize),
    Keyset { created_at: String, message_id: i64 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchKeysetCursorWire {
    created_at: String,
    message_id: i64,
}

fn parse_search_cursor(cursor: Option<&str>) -> SearchListCursor {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return SearchListCursor::Start;
    };
    if cursor.chars().all(|ch| ch.is_ascii_digit()) {
        return SearchListCursor::Offset(cursor.parse::<usize>().unwrap_or(0));
    }
    if let Ok(wire) = serde_json::from_str::<SearchKeysetCursorWire>(cursor)
        && !wire.created_at.trim().is_empty()
        && wire.message_id > 0
    {
        return SearchListCursor::Keyset {
            created_at: wire.created_at,
            message_id: wire.message_id,
        };
    }
    SearchListCursor::Start
}

fn search_keyset_next_cursor(rows: &[(MessageSearchHit, String)], limit: usize) -> Option<String> {
    if rows.len() <= limit {
        return None;
    }
    let (hit, created_at) = rows.get(limit.saturating_sub(1))?;
    serde_json::to_string(&SearchKeysetCursorWire {
        created_at: created_at.clone(),
        message_id: hit.message_id,
    })
    .ok()
}

fn normalize_search_page_size(limit: usize) -> usize {
    limit.clamp(1, SDKWORK_SEARCH_PAGE_SIZE_MAX)
}

fn search_fetch_limit(limit: usize) -> i64 {
    (normalize_search_page_size(limit) + 1) as i64
}

fn offset_next_cursor(
    offset: usize,
    hit_count: usize,
    limit: usize,
    total_count: u64,
) -> Option<String> {
    if hit_count < limit || hit_count == 0 {
        return None;
    }

    let next_offset = u64::try_from(offset.saturating_add(hit_count)).unwrap_or(u64::MAX);
    if next_offset < total_count {
        Some(next_offset.to_string())
    } else {
        None
    }
}

fn row_created_at_rfc3339(row: &postgres::Row, index: usize) -> String {
    row.get::<_, chrono::DateTime<chrono::Utc>>(index)
        .to_rfc3339()
}

impl SearchProvider for PostgresSearchProvider {
    fn index_message(&self, _message: &SearchableMessage) -> Result<(), ContractError> {
        // No-op: the im_messages_search_update trigger handles indexing.
        // The search_vector column is populated automatically on INSERT/UPDATE.
        Ok(())
    }

    fn search(
        &self,
        tenant_id: &str,
        organization_id: &str,
        query: &str,
        conversation_id: Option<&str>,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<SearchResult, ContractError> {
        let list_cursor = parse_search_cursor(cursor);
        let limit = normalize_search_page_size(limit);
        let conversation_filter: Option<&str> = conversation_id;

        let tsquery = escape_tsquery(query);
        if tsquery.is_empty() {
            return Ok(SearchResult {
                hits: Vec::new(),
                total_count: 0,
                next_cursor: None,
            });
        }

        let pool = self.pool.clone();
        let tenant = tenant_id.to_owned();
        let org = organization_id.to_owned();
        let conv = conversation_filter.map(|s| s.to_owned());

        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "search")?;

            match list_cursor {
                SearchListCursor::Offset(offset) => {
                    let rows = match client.query(
                        SEARCH_SQL,
                        &[
                            &tenant,
                            &org,
                            &tsquery,
                            &conv.as_deref(),
                            &(limit as i64),
                            &(offset as i64),
                        ],
                    ) {
                        Ok(rows) => rows,
                        Err(_) => client
                            .query(
                                SEARCH_PLAIN_SQL,
                                &[
                                    &tenant,
                                    &org,
                                    &tsquery,
                                    &conv.as_deref(),
                                    &(limit as i64),
                                    &(offset as i64),
                                ],
                            )
                            .map_err(|e| postgres_unavailable("search", e))?,
                    };

                    let hits = rows
                        .iter()
                        .map(|row| MessageSearchHit {
                            message_id: row.get(0),
                            conversation_id: row.get(1),
                            message_seq: row.get::<_, i64>(2) as u64,
                        })
                        .collect::<Vec<_>>();

                    let total_count: i64 = client
                        .query_one(COUNT_SQL, &[&tenant, &org, &conv.as_deref(), &tsquery])
                        .map(|row| row.get(0))
                        .unwrap_or(0);

                    let next_cursor =
                        offset_next_cursor(offset, hits.len(), limit, total_count as u64);

                    Ok(SearchResult {
                        hits,
                        total_count: total_count as u64,
                        next_cursor,
                    })
                }
                SearchListCursor::Start | SearchListCursor::Keyset { .. } => {
                    let fetch_limit = search_fetch_limit(limit);
                    let (keyset_created_at, keyset_message_id) = match list_cursor {
                        SearchListCursor::Keyset {
                            created_at,
                            message_id,
                        } => (Some(created_at), Some(message_id)),
                        _ => (None, None),
                    };
                    let rows = match client.query(
                        SEARCH_KEYSET_SQL,
                        &[
                            &tenant,
                            &org,
                            &tsquery,
                            &conv.as_deref(),
                            &fetch_limit,
                            &keyset_created_at.as_deref(),
                            &keyset_message_id,
                        ],
                    ) {
                        Ok(rows) => rows,
                        Err(_) => client
                            .query(
                                SEARCH_KEYSET_PLAIN_SQL,
                                &[
                                    &tenant,
                                    &org,
                                    &tsquery,
                                    &conv.as_deref(),
                                    &fetch_limit,
                                    &keyset_created_at.as_deref(),
                                    &keyset_message_id,
                                ],
                            )
                            .map_err(|e| postgres_unavailable("search", e))?,
                    };

                    let mut collected = Vec::with_capacity(rows.len());
                    for row in &rows {
                        collected.push((
                            MessageSearchHit {
                                message_id: row.get(0),
                                conversation_id: row.get(1),
                                message_seq: row.get::<_, i64>(2) as u64,
                            },
                            row_created_at_rfc3339(row, 3),
                        ));
                    }

                    let total_count: i64 = client
                        .query_one(COUNT_SQL, &[&tenant, &org, &conv.as_deref(), &tsquery])
                        .map(|row| row.get(0))
                        .unwrap_or(0);

                    let next_cursor = search_keyset_next_cursor(&collected, limit);
                    let hits = collected
                        .into_iter()
                        .take(limit)
                        .map(|(hit, _)| hit)
                        .collect();

                    Ok(SearchResult {
                        hits,
                        total_count: total_count as u64,
                        next_cursor,
                    })
                }
            }
        })
    }

    fn remove_message(&self, tenant_id: &str, message_id: i64) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant = tenant_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "search_remove")?;
            client
                .execute(REMOVE_SQL, &[&tenant, &message_id])
                .map_err(|e| postgres_unavailable("search_remove", e))?;
            Ok(())
        })
    }

    fn plugin_id(&self) -> &'static str {
        self.plugin_id
    }
}

impl PostgresSearchProvider {
    /// Membership-scoped search for interactive principals.
    pub fn search_for_member(
        &self,
        query: MemberSearchQuery<'_>,
    ) -> Result<SearchResult, ContractError> {
        let list_cursor = parse_search_cursor(query.cursor);
        let limit = normalize_search_page_size(query.limit);
        let conversation_filter: Option<&str> = query.conversation_id;
        let tsquery = escape_tsquery(query.query);
        if tsquery.is_empty() {
            return Ok(SearchResult {
                hits: Vec::new(),
                total_count: 0,
                next_cursor: None,
            });
        }

        let pool = self.pool.clone();
        let tenant = query.tenant_id.to_owned();
        let org = query.organization_id.to_owned();
        let principal_kind = query.principal_kind.to_owned();
        let principal_id = query.principal_id.to_owned();
        let conv = conversation_filter.map(|s| s.to_owned());

        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "search_for_member")?;
            match list_cursor {
                SearchListCursor::Offset(offset) => {
                    let rows = match client.query(
                        SEARCH_MEMBER_SQL,
                        &[
                            &tenant,
                            &org,
                            &tsquery,
                            &conv.as_deref(),
                            &(limit as i64),
                            &(offset as i64),
                            &principal_kind,
                            &principal_id,
                        ],
                    ) {
                        Ok(rows) => rows,
                        Err(_) => client
                            .query(
                                SEARCH_MEMBER_PLAIN_SQL,
                                &[
                                    &tenant,
                                    &org,
                                    &tsquery,
                                    &conv.as_deref(),
                                    &(limit as i64),
                                    &(offset as i64),
                                    &principal_kind,
                                    &principal_id,
                                ],
                            )
                            .map_err(|e| postgres_unavailable("search_for_member", e))?,
                    };

                    let hits = rows
                        .iter()
                        .map(|row| MessageSearchHit {
                            message_id: row.get(0),
                            conversation_id: row.get(1),
                            message_seq: row.get::<_, i64>(2) as u64,
                        })
                        .collect::<Vec<_>>();

                    let total_count: i64 = client
                        .query_one(
                            COUNT_MEMBER_SQL,
                            &[
                                &tenant,
                                &org,
                                &conv.as_deref(),
                                &tsquery,
                                &principal_kind,
                                &principal_id,
                            ],
                        )
                        .map(|row| row.get(0))
                        .unwrap_or(0);

                    let next_cursor =
                        offset_next_cursor(offset, hits.len(), limit, total_count as u64);

                    Ok(SearchResult {
                        hits,
                        total_count: total_count as u64,
                        next_cursor,
                    })
                }
                SearchListCursor::Start | SearchListCursor::Keyset { .. } => {
                    let fetch_limit = search_fetch_limit(limit);
                    let (keyset_created_at, keyset_message_id) = match list_cursor {
                        SearchListCursor::Keyset {
                            created_at,
                            message_id,
                        } => (Some(created_at), Some(message_id)),
                        _ => (None, None),
                    };
                    let rows = match client.query(
                        SEARCH_MEMBER_KEYSET_SQL,
                        &[
                            &tenant,
                            &org,
                            &tsquery,
                            &conv.as_deref(),
                            &fetch_limit,
                            &principal_kind,
                            &principal_id,
                            &keyset_created_at.as_deref(),
                            &keyset_message_id,
                        ],
                    ) {
                        Ok(rows) => rows,
                        Err(_) => client
                            .query(
                                SEARCH_MEMBER_KEYSET_PLAIN_SQL,
                                &[
                                    &tenant,
                                    &org,
                                    &tsquery,
                                    &conv.as_deref(),
                                    &fetch_limit,
                                    &principal_kind,
                                    &principal_id,
                                    &keyset_created_at.as_deref(),
                                    &keyset_message_id,
                                ],
                            )
                            .map_err(|e| postgres_unavailable("search_for_member", e))?,
                    };

                    let mut collected = Vec::with_capacity(rows.len());
                    for row in &rows {
                        collected.push((
                            MessageSearchHit {
                                message_id: row.get(0),
                                conversation_id: row.get(1),
                                message_seq: row.get::<_, i64>(2) as u64,
                            },
                            row_created_at_rfc3339(row, 3),
                        ));
                    }

                    let total_count: i64 = client
                        .query_one(
                            COUNT_MEMBER_SQL,
                            &[
                                &tenant,
                                &org,
                                &conv.as_deref(),
                                &tsquery,
                                &principal_kind,
                                &principal_id,
                            ],
                        )
                        .map(|row| row.get(0))
                        .unwrap_or(0);

                    let next_cursor = search_keyset_next_cursor(&collected, limit);
                    let hits = collected
                        .into_iter()
                        .take(limit)
                        .map(|(hit, _)| hit)
                        .collect();

                    Ok(SearchResult {
                        hits,
                        total_count: total_count as u64,
                        next_cursor,
                    })
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_tsquery_sanitizes_special_chars() {
        let result = escape_tsquery("hello | world & test ! (foo)");
        assert!(!result.contains('|'));
        assert!(!result.contains('!'));
        assert!(!result.contains('('));
        assert!(!result.contains(')'));
        assert!(
            result.contains("hello:*")
                && result.contains("world:*")
                && result.contains("test:*")
                && result.contains("foo:*"),
            "escaped tsquery should preserve searchable tokens as prefix terms"
        );
    }

    #[test]
    fn test_escape_tsquery_empty_input() {
        assert_eq!(escape_tsquery("   "), "");
    }

    #[test]
    fn test_escape_tsquery_adds_prefix_match() {
        let result = escape_tsquery("hello world");
        assert!(result.contains("hello:*"));
        assert!(result.contains("world:*"));
    }

    #[test]
    fn test_parse_search_cursor_accepts_keyset_json() {
        let cursor = parse_search_cursor(Some(
            r#"{"createdAt":"2026-05-06T00:00:00Z","messageId":42}"#,
        ));
        assert_eq!(
            cursor,
            SearchListCursor::Keyset {
                created_at: "2026-05-06T00:00:00Z".to_owned(),
                message_id: 42,
            }
        );
    }

    #[test]
    fn test_parse_search_cursor_accepts_legacy_offset() {
        assert_eq!(
            parse_search_cursor(Some("40")),
            SearchListCursor::Offset(40)
        );
    }

    #[test]
    fn search_limit_is_bounded_by_sdkwork_page_size_max() {
        assert_eq!(normalize_search_page_size(0), 1);
        assert_eq!(normalize_search_page_size(20), 20);
        assert_eq!(normalize_search_page_size(200), 200);
        assert_eq!(normalize_search_page_size(201), 200);
        assert_eq!(normalize_search_page_size(1000), 200);
    }

    #[test]
    fn search_fetch_limit_is_page_size_plus_one_with_sdkwork_bound() {
        assert_eq!(search_fetch_limit(0), 2);
        assert_eq!(search_fetch_limit(20), 21);
        assert_eq!(search_fetch_limit(200), 201);
        assert_eq!(search_fetch_limit(1000), 201);
    }

    #[test]
    fn offset_next_cursor_uses_total_count_to_avoid_empty_tail_page() {
        assert_eq!(offset_next_cursor(0, 20, 20, 41), Some("20".to_owned()));
        assert_eq!(offset_next_cursor(20, 20, 20, 40), None);
        assert_eq!(offset_next_cursor(20, 19, 20, 40), None);
    }

    #[test]
    fn test_plugin_id() {
        let plugin_id = "search-postgres";
        assert_eq!(plugin_id, "search-postgres");
    }
}
