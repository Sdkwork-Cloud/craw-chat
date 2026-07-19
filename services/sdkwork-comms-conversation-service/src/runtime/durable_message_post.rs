use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    AgentDispatchReplyCompletion, AgentMentionDispatchRequest, CommitPosition, ContractError,
    OutboxEventRecord, StoredMessageRecord,
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

    fn persist_message_post_batch_with_agent_dispatch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
        dispatch_request: Option<AgentMentionDispatchRequest>,
        max_dispatch_attempts: u32,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let _ = (dispatch_request, max_dispatch_attempts);
        self.persist_message_post_batch(envelopes, message, outboxes)
    }

    fn persist_agent_reply_and_complete_dispatch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
        completion: AgentDispatchReplyCompletion,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let _ = completion;
        self.persist_message_post_batch(envelopes, message, outboxes)
    }
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

    fn persist_message_post_batch_with_agent_dispatch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
        dispatch_request: Option<AgentMentionDispatchRequest>,
        max_dispatch_attempts: u32,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        im_adapters_postgres_journal::PostgresDurableMessagePostWriter::persist_message_post_batch_with_agent_dispatch(
            self,
            envelopes,
            message,
            outboxes,
            dispatch_request,
            max_dispatch_attempts,
        )
    }

    fn persist_agent_reply_and_complete_dispatch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
        completion: AgentDispatchReplyCompletion,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        im_adapters_postgres_journal::PostgresDurableMessagePostWriter::persist_agent_reply_and_complete_dispatch(
            self,
            envelopes,
            message,
            outboxes,
            completion,
        )
    }
}
