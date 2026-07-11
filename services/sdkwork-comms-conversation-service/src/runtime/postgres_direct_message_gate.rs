//! Postgres-backed direct message access gate for cloud conversation processes.

use std::sync::Arc;

use im_adapters_social_postgres::user_block_store::{PostgresUserBlockStore, UserBlockStore};

use super::DirectMessageAccessGate;

const BLOCK_SCOPE_ALL: &str = "all";
const BLOCK_SCOPE_DIRECT_CHAT: &str = "direct_chat";

#[derive(Clone)]
pub struct PostgresDirectMessageAccessGate {
    block_store: Arc<PostgresUserBlockStore>,
}

impl PostgresDirectMessageAccessGate {
    pub fn new(block_store: Arc<PostgresUserBlockStore>) -> Self {
        Self { block_store }
    }
}

impl DirectMessageAccessGate for PostgresDirectMessageAccessGate {
    fn ensure_direct_message_allowed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        sender_user_id: &str,
        peer_user_id: &str,
    ) -> Result<(), String> {
        if active_block_between_users(
            self.block_store.as_ref(),
            tenant_id,
            organization_id,
            peer_user_id,
            sender_user_id,
            BLOCK_SCOPE_ALL,
        )? {
            return Err("direct message blocked by user block".into());
        }
        if active_block_between_users(
            self.block_store.as_ref(),
            tenant_id,
            organization_id,
            peer_user_id,
            sender_user_id,
            BLOCK_SCOPE_DIRECT_CHAT,
        )? {
            return Err("direct message blocked by user block".into());
        }
        if active_block_between_users(
            self.block_store.as_ref(),
            tenant_id,
            organization_id,
            sender_user_id,
            peer_user_id,
            BLOCK_SCOPE_ALL,
        )? {
            return Err("direct message blocked by user block".into());
        }
        Ok(())
    }
}

fn active_block_between_users(
    store: &PostgresUserBlockStore,
    tenant_id: &str,
    organization_id: &str,
    blocker_user_id: &str,
    blocked_user_id: &str,
    scope: &str,
) -> Result<bool, String> {
    store
        .find_active_block(
            tenant_id,
            organization_id,
            blocker_user_id,
            blocked_user_id,
            scope,
        )
        .map(|record| record.is_some())
        .map_err(|error| format!("direct message block lookup failed: {error:?}"))
}
