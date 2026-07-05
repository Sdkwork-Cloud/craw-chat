use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    CommitPosition, ContractError, OutboxEventRecord, StoredMessageRecord,
};

pub trait DurableMessagePostWriter: Send + Sync {
    fn persist_message_post(
        &self,
        envelope: CommitEnvelope,
        message: StoredMessageRecord,
        outbox: Option<OutboxEventRecord>,
    ) -> Result<CommitPosition, ContractError>;
}

impl DurableMessagePostWriter for im_adapters_postgres_journal::PostgresDurableMessagePostWriter {
    fn persist_message_post(
        &self,
        envelope: CommitEnvelope,
        message: StoredMessageRecord,
        outbox: Option<OutboxEventRecord>,
    ) -> Result<CommitPosition, ContractError> {
        im_adapters_postgres_journal::PostgresDurableMessagePostWriter::persist_message_post(
            self, envelope, message, outbox,
        )
    }
}
