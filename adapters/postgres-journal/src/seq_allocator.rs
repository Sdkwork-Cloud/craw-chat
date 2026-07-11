//! Postgres-backed [`ConversationSeqAllocator`] using `im_conversation_seq_counters`.

use im_platform_contracts::ContractError;
use im_platform_contracts::ConversationSeqAllocator;

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_pool_client, postgres_timestamptz,
    postgres_unavailable, run_postgres_io,
};

const ALLOCATE_SEQ_SQL: &str = r#"
insert into im_conversation_seq_counters (tenant_id, organization_id, conversation_id, next_seq, updated_at)
values ($1, $2, $3, 1, $4)
on conflict (tenant_id, organization_id, conversation_id) do update
set next_seq = im_conversation_seq_counters.next_seq + 1, updated_at = $4
returning next_seq
"#;

/// Per-conversation sequence allocator backed by Postgres atomic counters.
#[derive(Clone)]
pub struct PostgresConversationSeqAllocator {
    pool: PostgresJournalPool,
}

impl PostgresConversationSeqAllocator {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

impl ConversationSeqAllocator for PostgresConversationSeqAllocator {
    fn allocate_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "allocate_conversation_seq")?;
            let now = postgres_timestamptz(&now_rfc3339(), "now")?;
            let row = client
                .query_one(
                    ALLOCATE_SEQ_SQL,
                    &[&tenant_id, &organization_id, &conversation_id, &now],
                )
                .map_err(|error| postgres_unavailable("allocate_conversation_seq", error))?;
            let seq: i64 = row.get(0);
            Ok(seq as u64)
        })
    }

    fn batch_size(&self) -> u32 {
        1
    }
}
