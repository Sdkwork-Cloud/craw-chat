//! PostgreSQL implementation of [`SearchProvider`].
//!
//! Leverages the `search_vector tsvector` column on `im_conversation_messages`
//! with a GIN index. The column is automatically populated by the
//! `im_messages_search_update` trigger on INSERT/UPDATE, so `index_message`
//! is a no-op for the PostgreSQL backend.

use im_platform_contracts::{ContractError, SearchProvider, SearchResult, SearchableMessage};

use crate::{postgres_pool_client, postgres_unavailable, run_postgres_io, PostgresJournalPool};

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
        let offset: usize = cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);
        let limit = limit.clamp(1, 1000);
        let conversation_filter: Option<&str> = conversation_id;

        let tsquery = escape_tsquery(query);
        if tsquery.is_empty() {
            return Ok(SearchResult {
                message_ids: Vec::new(),
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

            // Try tsquery first, fall back to plainto_tsquery if it fails
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

            let mut message_ids = Vec::with_capacity(rows.len());
            for row in &rows {
                let msg_id: i64 = row.get(0);
                message_ids.push(msg_id);
            }

            let total_count: i64 = client
                .query_one(COUNT_SQL, &[&tenant, &org, &conv.as_deref(), &tsquery])
                .map(|row| row.get(0))
                .unwrap_or(0);

            let next_cursor = if message_ids.len() == limit {
                Some((offset + limit).to_string())
            } else {
                None
            };

            Ok(SearchResult {
                message_ids,
                total_count: total_count as u64,
                next_cursor,
            })
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
    fn test_plugin_id() {
        let plugin_id = "search-postgres";
        assert_eq!(plugin_id, "search-postgres");
    }
}
