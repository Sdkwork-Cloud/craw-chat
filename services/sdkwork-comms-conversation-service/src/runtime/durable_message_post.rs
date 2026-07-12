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
    ) -> Result<CommitPosition, ContractError> {
        let positions =
            self.persist_message_post_batch(vec![envelope], message, outbox.into_iter().collect())?;
        match positions.as_slice() {
            [position] => Ok(position.clone()),
            _ => Err(ContractError::Invalid(
                "durable message post writer returned an invalid journal position count".into(),
            )),
        }
    }

    fn persist_message_post_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
    ) -> Result<Vec<CommitPosition>, ContractError>;
}

impl DurableMessagePostWriter for im_adapters_postgres_journal::PostgresDurableMessagePostWriter {
    fn persist_message_post_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        im_adapters_postgres_journal::PostgresDurableMessagePostWriter::persist_message_post_batch(
            self, envelopes, message, outboxes,
        )
    }
}
