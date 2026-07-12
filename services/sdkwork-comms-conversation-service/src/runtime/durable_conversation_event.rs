use im_domain_events::CommitEnvelope;
use im_platform_contracts::{CommitPosition, ContractError, OutboxEventRecord};

/// Atomic persistence boundary for a conversation journal event and its
/// ordinary conversation outbox record.
///
/// The two records are the same logical commit: the journal is the source of
/// truth and the outbox is the durable realtime delivery handoff. Production
/// PostgreSQL wiring implements this port with one database transaction so a
/// process crash cannot leave a committed assignment change without a relay
/// record.
pub trait DurableConversationEventWriter: Send + Sync {
    fn persist_conversation_event(
        &self,
        envelope: CommitEnvelope,
        outbox: OutboxEventRecord,
    ) -> Result<CommitPosition, ContractError>;
}

impl DurableConversationEventWriter
    for im_adapters_postgres_journal::PostgresDurableConversationEventWriter
{
    fn persist_conversation_event(
        &self,
        envelope: CommitEnvelope,
        outbox: OutboxEventRecord,
    ) -> Result<CommitPosition, ContractError> {
        im_adapters_postgres_journal::PostgresDurableConversationEventWriter::persist_conversation_event(
            self, envelope, outbox,
        )
    }
}
