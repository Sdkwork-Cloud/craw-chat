use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    CommitPosition, ContractError, OutboxEventRecord, StoredMessageMutation,
};

pub trait DurableMessageMutationWriter: Send + Sync {
    fn persist_message_mutation(
        &self,
        envelope: CommitEnvelope,
        mutation: StoredMessageMutation,
        outbox: OutboxEventRecord,
    ) -> Result<Option<CommitPosition>, ContractError>;
}

impl DurableMessageMutationWriter
    for im_adapters_postgres_journal::PostgresDurableMessageMutationWriter
{
    fn persist_message_mutation(
        &self,
        envelope: CommitEnvelope,
        mutation: StoredMessageMutation,
        outbox: OutboxEventRecord,
    ) -> Result<Option<CommitPosition>, ContractError> {
        im_adapters_postgres_journal::PostgresDurableMessageMutationWriter::persist_message_mutation(
            self, envelope, mutation, outbox,
        )
    }
}
