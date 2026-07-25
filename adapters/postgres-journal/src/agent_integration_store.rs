use std::sync::Arc;

use im_platform_contracts::{ContractError, IdGenerator};
use r2d2_postgres::postgres::Transaction;
use sdkwork_im_contract_agent::{
    AgentAssignmentSource, AgentBindingStatus, AgentDispatchRecord, AgentDispatchStatus,
    AgentIntegrationStore, AgentMentionDispatchRequest, AgentReplyCommitResult,
    ConversationAgentAssignmentRecord, ConversationAgentBindingRecord,
    ReplaceConversationAgentAssignments,
};
use sdkwork_utils_rust::sha256_hash;

use crate::{
    PostgresJournalPool, postgres_pool_client, postgres_timestamptz, postgres_unavailable,
    run_postgres_io,
};

const MAX_ASSIGNMENTS: usize = 200;
const MAX_CLAIM_BATCH: usize = 100;

const LOCK_ASSIGNMENTS_SQL: &str = r#"
select source_aggregate_version, payload_hash
from im_conversation_agent_assignments
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
order by source_aggregate_version desc, id desc
limit 1 for update
"#;

const REMOVE_ASSIGNMENTS_SQL: &str = r#"
update im_conversation_agent_assignments
set enabled = false, status = 2, source_event_id = $4,
    source_aggregate_version = $5, payload_hash = $6, updated_at = $7
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

const UPSERT_ASSIGNMENT_SQL: &str = r#"
insert into im_conversation_agent_assignments (
    id, uuid, tenant_id, organization_id, conversation_id, agent_id,
    agent_revision_ref, assignment_source, assignment_generation, position,
    enabled, status, assigned_by, assigned_at, source_event_id,
    source_aggregate_version, payload_hash, created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
    true, 0, $11, $12, $13, $14, $15, $12, $12
)
on conflict (tenant_id, organization_id, conversation_id, agent_id)
do update set agent_revision_ref = excluded.agent_revision_ref,
    assignment_source = excluded.assignment_source,
    assignment_generation = excluded.assignment_generation,
    position = excluded.position, enabled = true, status = 0,
    assigned_by = excluded.assigned_by, assigned_at = excluded.assigned_at,
    source_event_id = excluded.source_event_id,
    source_aggregate_version = excluded.source_aggregate_version,
    payload_hash = excluded.payload_hash, updated_at = excluded.updated_at
where im_conversation_agent_assignments.source_aggregate_version <= excluded.source_aggregate_version
"#;

const LIST_ASSIGNMENTS_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, agent_id,
    agent_revision_ref, assignment_source, assignment_generation, position,
    enabled, status, source_aggregate_version
from im_conversation_agent_assignments
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and enabled = true and status = 0
order by position, id
limit $4
"#;

pub(crate) const INSERT_DISPATCH_SQL: &str = r#"
insert into im_agent_dispatch (
    id, uuid, dispatch_id, tenant_id, organization_id, conversation_id,
    source_message_id, source_message_seq, agent_id, agent_revision_ref,
    assignment_generation, status, idempotency_key, payload_hash,
    attempt_count, max_attempts, next_attempt_at, requested_by,
    created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
    $11, 0, $12, $13, 0, $14, $15, $16, $15, $15
)
on conflict (tenant_id, organization_id, conversation_id, source_message_id,
    agent_id, assignment_generation) do nothing
"#;

pub(crate) const SELECT_DISPATCH_BY_ID_SQL: &str = r#"
select dispatch_id, tenant_id, organization_id, conversation_id,
    source_message_id, source_message_seq, agent_id, agent_revision_ref,
    assignment_generation, binding_id, agents_session_id, agents_turn_id,
    status, idempotency_key, payload_hash, attempt_count, max_attempts,
    lease_owner, lease_expires_at, next_attempt_at, requested_by,
    reply_message_id, reply_message_seq, created_at, updated_at
from im_agent_dispatch
where tenant_id = $1 and organization_id = $2 and dispatch_id = $3
"#;

const CLAIM_DISPATCH_SQL: &str = r#"
with exhausted as materialized (
    update im_agent_dispatch
    set status = 7, lease_owner = null, lease_expires_at = null,
        last_error_code = 'lease_reconciliation_exhausted',
        last_error_detail = 'dispatch lease expired after the bounded reconciliation attempt',
        updated_at = $4
    where tenant_id = $1 and organization_id = $2
      and status in (1, 2, 3) and lease_expires_at <= $4
      and attempt_count > max_attempts
    returning id
), candidates as materialized (
    select id, status
    from im_agent_dispatch
    where tenant_id = $1 and organization_id = $2
      and next_attempt_at <= $4
      and (
        (status in (0, 5) and attempt_count < max_attempts)
        or (status in (1, 2, 3) and lease_expires_at <= $4
            and attempt_count <= max_attempts)
      )
      and (select count(*) from exhausted) >= 0
    order by next_attempt_at, lease_expires_at nulls first, id
    for update skip locked
    limit $6
), claimed as (
    update im_agent_dispatch dispatch
    set status = 1, lease_owner = $3, lease_expires_at = $5,
        attempt_count = attempt_count + case when candidates.status in (0, 5) then 1 else 0 end,
        started_at = coalesce(started_at, $4),
        updated_at = $4
    from candidates
    where dispatch.id = candidates.id
    returning dispatch.*
)
select dispatch_id, tenant_id, organization_id, conversation_id,
    source_message_id, source_message_seq, agent_id, agent_revision_ref,
    assignment_generation, binding_id, agents_session_id, agents_turn_id,
    status, idempotency_key, payload_hash, attempt_count, max_attempts,
    lease_owner, lease_expires_at, next_attempt_at, requested_by,
    reply_message_id, reply_message_seq, created_at, updated_at
from claimed order by next_attempt_at, id
"#;

const CLAIM_DISPATCH_GLOBAL_SQL: &str = r#"
with exhausted as materialized (
    update im_agent_dispatch
    set status = 7, lease_owner = null, lease_expires_at = null,
        last_error_code = 'lease_reconciliation_exhausted',
        last_error_detail = 'dispatch lease expired after the bounded reconciliation attempt',
        updated_at = $2
    where status in (1, 2, 3) and lease_expires_at <= $2
      and attempt_count > max_attempts
    returning id
), candidates as materialized (
    select id, status
    from im_agent_dispatch
    where next_attempt_at <= $2
      and (
        (status in (0, 5) and attempt_count < max_attempts)
        or (status in (1, 2, 3) and lease_expires_at <= $2
            and attempt_count <= max_attempts)
      )
      and (select count(*) from exhausted) >= 0
    order by next_attempt_at, lease_expires_at nulls first, id
    for update skip locked
    limit $4
), claimed as (
    update im_agent_dispatch dispatch
    set status = 1, lease_owner = $1, lease_expires_at = $3,
        attempt_count = attempt_count + case when candidates.status in (0, 5) then 1 else 0 end,
        started_at = coalesce(started_at, $2),
        updated_at = $2
    from candidates
    where dispatch.id = candidates.id
    returning dispatch.*
)
select dispatch_id, tenant_id, organization_id, conversation_id,
    source_message_id, source_message_seq, agent_id, agent_revision_ref,
    assignment_generation, binding_id, agents_session_id, agents_turn_id,
    status, idempotency_key, payload_hash, attempt_count, max_attempts,
    lease_owner, lease_expires_at, next_attempt_at, requested_by,
    reply_message_id, reply_message_seq, created_at, updated_at
from claimed order by next_attempt_at, tenant_id, organization_id, id
"#;

const RESOLVE_BINDING_SQL: &str = r#"
select binding_id, tenant_id, organization_id, conversation_id, agent_id,
    agent_revision_ref, assignment_generation, agents_session_id, status,
    idempotency_key, payload_hash, created_by, updated_by,
    last_error_code, last_error_detail, version, created_at, updated_at
from im_conversation_agent_binding
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and agent_id = $4 and assignment_generation = $5
limit 1
"#;

const INSERT_BINDING_SQL: &str = r#"
insert into im_conversation_agent_binding (
    id, uuid, binding_id, tenant_id, organization_id, conversation_id,
    agent_id, agent_revision_ref, assignment_generation, agents_session_id,
    status, idempotency_key, payload_hash, created_by, updated_by,
    last_error_code, last_error_detail, version, created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
    $11, $12, $13, $14, $15, $16, $17, 0, $18, $19
)
on conflict (tenant_id, organization_id, conversation_id, agent_id, assignment_generation)
do nothing
"#;

const UPDATE_BINDING_SQL: &str = r#"
update im_conversation_agent_binding
set agents_session_id = $1, status = $2::smallint, updated_by = $3,
    last_error_code = $4, last_error_detail = $5,
    last_used_at = case when $2::smallint = 1::smallint then $6 else last_used_at end,
    closed_at = case when $2::smallint in (3::smallint, 4::smallint) then $6 else closed_at end,
    version = version + 1, updated_at = $6
where tenant_id = $7 and organization_id = $8 and binding_id = $9
    and version = $10
"#;

const MARK_DISPATCH_RUNNING_SQL: &str = r#"
update im_agent_dispatch
set status = 3, binding_id = $5, agents_session_id = $6, updated_at = $7
where tenant_id = $1 and organization_id = $2 and dispatch_id = $3
  and lease_owner = $4 and status = 1 and lease_expires_at >= $7
"#;

pub(crate) const COMPLETE_DISPATCH_SQL: &str = r#"
update im_agent_dispatch
set status = 4, agents_turn_id = $5, reply_message_id = $6,
    reply_message_seq = $7, completed_at = $8, updated_at = $8,
    lease_owner = null, lease_expires_at = null, last_error_code = null,
    last_error_detail = null
where tenant_id = $1 and organization_id = $2 and dispatch_id = $3
  and lease_owner = $4 and status = 3 and lease_expires_at >= $8
"#;

pub(crate) const SELECT_DISPATCH_COMPLETION_SQL: &str = r#"
select status, agent_id, agents_session_id, agents_turn_id,
    reply_message_id, reply_message_seq, conversation_id
from im_agent_dispatch
where tenant_id = $1 and organization_id = $2 and dispatch_id = $3
"#;

const FAIL_DISPATCH_SQL: &str = r#"
update im_agent_dispatch
set status = case when attempt_count >= max_attempts then 7 else 5 end,
    last_error_code = $5, last_error_detail = $6, next_attempt_at = $7,
    updated_at = $8, lease_owner = null, lease_expires_at = null
where tenant_id = $1 and organization_id = $2 and dispatch_id = $3
  and lease_owner = $4 and status in (1, 2, 3)
returning status
"#;

const DEFER_DISPATCH_RECONCILIATION_SQL: &str = r#"
update im_agent_dispatch
set status = 3, agents_turn_id = coalesce($5, agents_turn_id),
    last_error_code = 'agents_turn_indeterminate', last_error_detail = $6,
    next_attempt_at = $7, lease_expires_at = $7, updated_at = $8,
    lease_owner = null
where tenant_id = $1 and organization_id = $2 and dispatch_id = $3
  and lease_owner = $4 and status in (1, 2, 3) and lease_expires_at >= $8
"#;

#[derive(Clone)]
pub struct PostgresAgentIntegrationStore {
    pool: PostgresJournalPool,
    id_generator: Arc<dyn IdGenerator>,
}

impl PostgresAgentIntegrationStore {
    pub fn from_pool(pool: PostgresJournalPool, id_generator: Arc<dyn IdGenerator>) -> Self {
        Self { pool, id_generator }
    }

    pub fn from_pool_with_runtime_ids(pool: PostgresJournalPool) -> Self {
        Self::from_pool(
            pool,
            sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "im-agent-integration-store",
            ),
        )
    }

    fn next_id(&self) -> Result<i64, ContractError> {
        self.id_generator.next_id()
    }
}

impl AgentIntegrationStore for PostgresAgentIntegrationStore {
    fn replace_conversation_agents(
        &self,
        command: ReplaceConversationAgentAssignments,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let generator = self.id_generator.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "replace conversation agents")?;
            let mut tx = client
                .transaction()
                .map_err(|error| postgres_unavailable("assignment transaction begin", error))?;
            replace_conversation_agents_in_transaction(&mut tx, &command, generator.as_ref())?;
            tx.commit()
                .map_err(|error| postgres_unavailable("assignment transaction commit", error))
        })
    }

    fn list_conversation_agents(
        &self,
        tenant_id: u64,
        organization_id: u64,
        conversation_id: &str,
        limit: usize,
    ) -> Result<Vec<ConversationAgentAssignmentRecord>, ContractError> {
        validate_signed_id(tenant_id, "tenantId", false)?;
        validate_signed_id(organization_id, "organizationId", true)?;
        if limit == 0 || limit > MAX_ASSIGNMENTS {
            return Err(ContractError::Invalid(
                "invalid assignment page size".into(),
            ));
        }
        let pool = self.pool.clone();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list conversation agents")?;
            client
                .query(
                    LIST_ASSIGNMENTS_SQL,
                    &[
                        &(tenant_id as i64),
                        &(organization_id as i64),
                        &conversation_id,
                        &(limit as i64),
                    ],
                )
                .map_err(|error| postgres_unavailable("list conversation agents", error))?
                .into_iter()
                .map(assignment_from_row)
                .collect()
        })
    }

    fn enqueue_dispatches(
        &self,
        request: &AgentMentionDispatchRequest,
        max_attempts: u32,
    ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
        request.validate()?;
        if max_attempts == 0 || max_attempts > 100 {
            return Err(ContractError::Invalid(
                "invalid dispatch max attempts".into(),
            ));
        }
        let tenant_id = parse_positive_id(&request.tenant_id, "tenantId")?;
        let organization_id = parse_id(&request.organization_id, "organizationId")?;
        let source_message_id = parse_positive_id(&request.message_id, "messageId")?;
        let requested_by = parse_positive_id(&request.sender_principal_id, "requestedBy")?;
        validate_signed_id(request.message_seq, "messageSeq", false)?;
        validate_signed_id(request.assignment_generation, "assignmentGeneration", false)?;
        let requested_at = postgres_timestamptz(&request.requested_at, "requested_at")?;
        let payload = serde_json::to_vec(request)
            .map_err(|_| ContractError::Invalid("invalid dispatch payload".into()))?;
        let payload_hash = sha256_hash(&payload);
        let pool = self.pool.clone();
        let targets = request.targets.clone();
        let conversation_id = request.conversation_id.clone();
        let source_message_seq = request.message_seq;
        let generation = request.assignment_generation;
        let generator = self.id_generator.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "enqueue agent dispatches")?;
            let mut tx = client
                .transaction()
                .map_err(|error| postgres_unavailable("dispatch enqueue begin", error))?;
            let mut records = Vec::with_capacity(targets.len());
            for target in targets {
                let id = generator.next_id()?;
                let uuid = storage_uuid("dispatch", id, &target.dispatch_id);
                let idempotency_key = format!("im-agent-dispatch:{}", target.dispatch_id);
                tx.execute(
                    INSERT_DISPATCH_SQL,
                    &[
                        &id,
                        &uuid,
                        &target.dispatch_id,
                        &(tenant_id as i64),
                        &(organization_id as i64),
                        &conversation_id,
                        &(source_message_id as i64),
                        &(source_message_seq as i64),
                        &target.agent_id,
                        &target.revision_id,
                        &(generation as i64),
                        &idempotency_key,
                        &payload_hash,
                        &(max_attempts as i32),
                        &requested_at,
                        &(requested_by as i64),
                    ],
                )
                .map_err(|error| postgres_unavailable("dispatch enqueue", error))?;
                let row = tx
                    .query_one(
                        SELECT_DISPATCH_BY_ID_SQL,
                        &[
                            &(tenant_id as i64),
                            &(organization_id as i64),
                            &target.dispatch_id,
                        ],
                    )
                    .map_err(|error| postgres_unavailable("dispatch replay load", error))?;
                let record = dispatch_from_row(row)?;
                if record.payload_hash != payload_hash {
                    return Err(ContractError::Conflict(
                        "dispatch idempotency payload conflict".into(),
                    ));
                }
                records.push(record);
            }
            tx.commit()
                .map_err(|error| postgres_unavailable("dispatch enqueue commit", error))?;
            Ok(records)
        })
    }

    fn claim_dispatches(
        &self,
        tenant_id: u64,
        organization_id: u64,
        lease_owner: &str,
        now: &str,
        lease_expires_at: &str,
        limit: usize,
    ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
        validate_signed_id(tenant_id, "tenantId", false)?;
        validate_signed_id(organization_id, "organizationId", true)?;
        if lease_owner.trim().is_empty() || limit == 0 || limit > MAX_CLAIM_BATCH {
            return Err(ContractError::Invalid(
                "invalid dispatch lease request".into(),
            ));
        }
        let now = postgres_timestamptz(now, "now")?;
        let lease_expires_at = postgres_timestamptz(lease_expires_at, "lease_expires_at")?;
        if lease_expires_at <= now {
            return Err(ContractError::Invalid(
                "dispatch lease must expire after now".into(),
            ));
        }
        let pool = self.pool.clone();
        let lease_owner = lease_owner.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "claim agent dispatches")?;
            client
                .query(
                    CLAIM_DISPATCH_SQL,
                    &[
                        &(tenant_id as i64),
                        &(organization_id as i64),
                        &lease_owner,
                        &now,
                        &lease_expires_at,
                        &(limit as i64),
                    ],
                )
                .map_err(|error| postgres_unavailable("claim agent dispatches", error))?
                .into_iter()
                .map(dispatch_from_row)
                .collect()
        })
    }

    fn claim_dispatches_global(
        &self,
        lease_owner: &str,
        now: &str,
        lease_expires_at: &str,
        limit: usize,
    ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
        if lease_owner.trim().is_empty() || limit == 0 || limit > MAX_CLAIM_BATCH {
            return Err(ContractError::Invalid(
                "invalid global dispatch lease request".into(),
            ));
        }
        let now = postgres_timestamptz(now, "now")?;
        let lease_expires_at = postgres_timestamptz(lease_expires_at, "lease_expires_at")?;
        if lease_expires_at <= now {
            return Err(ContractError::Invalid(
                "dispatch lease must expire after now".into(),
            ));
        }
        let pool = self.pool.clone();
        let lease_owner = lease_owner.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "claim global agent dispatches")?;
            client
                .query(
                    CLAIM_DISPATCH_GLOBAL_SQL,
                    &[&lease_owner, &now, &lease_expires_at, &(limit as i64)],
                )
                .map_err(|error| postgres_unavailable("claim global agent dispatches", error))?
                .into_iter()
                .map(dispatch_from_row)
                .collect()
        })
    }

    fn resolve_binding(
        &self,
        tenant_id: u64,
        organization_id: u64,
        conversation_id: &str,
        agent_id: &str,
        assignment_generation: u64,
    ) -> Result<Option<ConversationAgentBindingRecord>, ContractError> {
        validate_signed_id(tenant_id, "tenantId", false)?;
        validate_signed_id(organization_id, "organizationId", true)?;
        validate_signed_id(assignment_generation, "assignmentGeneration", false)?;
        let pool = self.pool.clone();
        let conversation_id = conversation_id.to_owned();
        let agent_id = agent_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "resolve agent binding")?;
            client
                .query_opt(
                    RESOLVE_BINDING_SQL,
                    &[
                        &(tenant_id as i64),
                        &(organization_id as i64),
                        &conversation_id,
                        &agent_id,
                        &(assignment_generation as i64),
                    ],
                )
                .map_err(|error| postgres_unavailable("resolve agent binding", error))?
                .map(binding_from_row)
                .transpose()
        })
    }

    fn save_binding(
        &self,
        binding: ConversationAgentBindingRecord,
    ) -> Result<ConversationAgentBindingRecord, ContractError> {
        validate_signed_id(binding.tenant_id, "tenantId", false)?;
        validate_signed_id(binding.organization_id, "organizationId", true)?;
        validate_signed_id(binding.assignment_generation, "assignmentGeneration", false)?;
        validate_signed_id(binding.created_by, "createdBy", false)?;
        validate_signed_id(binding.updated_by, "updatedBy", false)?;
        validate_signed_id(binding.version, "version", true)?;
        let created_at = postgres_timestamptz(&binding.created_at, "created_at")?;
        let updated_at = postgres_timestamptz(&binding.updated_at, "updated_at")?;
        let pool = self.pool.clone();
        let id = self.next_id()?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "save agent binding")?;
            if binding.version == 0 {
                let uuid = storage_uuid("binding", id, &binding.binding_id);
                client
                    .execute(
                        INSERT_BINDING_SQL,
                        &[
                            &id,
                            &uuid,
                            &binding.binding_id,
                            &(binding.tenant_id as i64),
                            &(binding.organization_id as i64),
                            &binding.conversation_id,
                            &binding.agent_id,
                            &binding.agent_revision_ref,
                            &(binding.assignment_generation as i64),
                            &binding.agents_session_id,
                            &binding.status.db_code(),
                            &binding.idempotency_key,
                            &binding.payload_hash,
                            &(binding.created_by as i64),
                            &(binding.updated_by as i64),
                            &binding.last_error_code,
                            &binding.last_error_detail,
                            &created_at,
                            &updated_at,
                        ],
                    )
                    .map_err(|error| postgres_unavailable("insert agent binding", error))?;
            } else {
                let affected = client
                    .execute(
                        UPDATE_BINDING_SQL,
                        &[
                            &binding.agents_session_id,
                            &binding.status.db_code(),
                            &(binding.updated_by as i64),
                            &binding.last_error_code,
                            &binding.last_error_detail,
                            &updated_at,
                            &(binding.tenant_id as i64),
                            &(binding.organization_id as i64),
                            &binding.binding_id,
                            &((binding.version - 1) as i64),
                        ],
                    )
                    .map_err(|error| postgres_unavailable("update agent binding", error))?;
                if affected == 0 {
                    return Err(ContractError::Conflict(
                        "agent binding version conflict".into(),
                    ));
                }
            }
            client
                .query_one(
                    RESOLVE_BINDING_SQL,
                    &[
                        &(binding.tenant_id as i64),
                        &(binding.organization_id as i64),
                        &binding.conversation_id,
                        &binding.agent_id,
                        &(binding.assignment_generation as i64),
                    ],
                )
                .map_err(|error| postgres_unavailable("reload agent binding", error))
                .and_then(binding_from_row)
        })
    }

    fn mark_dispatch_running(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        binding_id: &str,
        agents_session_id: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        validate_dispatch_ids(dispatch)?;
        let pool = self.pool.clone();
        let dispatch = dispatch.clone();
        let lease_owner = lease_owner.to_owned();
        let binding_id = binding_id.to_owned();
        let agents_session_id = agents_session_id.to_owned();
        let updated_at = postgres_timestamptz(updated_at, "updated_at")?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "mark dispatch running")?;
            let affected = client
                .execute(
                    MARK_DISPATCH_RUNNING_SQL,
                    &[
                        &(dispatch.tenant_id as i64),
                        &(dispatch.organization_id as i64),
                        &dispatch.dispatch_id,
                        &lease_owner,
                        &binding_id,
                        &agents_session_id,
                        &updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("mark dispatch running", error))?;
            if affected != 1 {
                return Err(ContractError::Conflict(
                    "dispatch lease fence rejected".into(),
                ));
            }
            Ok(())
        })
    }

    fn complete_dispatch(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        agents_turn_id: &str,
        reply: AgentReplyCommitResult,
        completed_at: &str,
    ) -> Result<(), ContractError> {
        validate_dispatch_ids(dispatch)?;
        validate_signed_id(reply.reply_message_id, "replyMessageId", false)?;
        validate_signed_id(reply.reply_message_seq, "replyMessageSeq", false)?;
        let pool = self.pool.clone();
        let dispatch = dispatch.clone();
        let lease_owner = lease_owner.to_owned();
        let agents_turn_id = agents_turn_id.to_owned();
        let completed_at = postgres_timestamptz(completed_at, "completed_at")?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "complete agent dispatch")?;
            let affected = client
                .execute(
                    COMPLETE_DISPATCH_SQL,
                    &[
                        &(dispatch.tenant_id as i64),
                        &(dispatch.organization_id as i64),
                        &dispatch.dispatch_id,
                        &lease_owner,
                        &agents_turn_id,
                        &(reply.reply_message_id as i64),
                        &(reply.reply_message_seq as i64),
                        &completed_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("complete agent dispatch", error))?;
            if affected != 1 {
                return Err(ContractError::Conflict(
                    "dispatch completion fence rejected".into(),
                ));
            }
            Ok(())
        })
    }

    fn defer_dispatch_reconciliation(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        agents_turn_id: Option<&str>,
        detail: &str,
        next_attempt_at: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        validate_dispatch_ids(dispatch)?;
        if agents_turn_id.is_some_and(|value| value.is_empty() || value.len() > 128)
            || detail.is_empty()
            || detail.len() > 2048
        {
            return Err(ContractError::Invalid(
                "invalid dispatch reconciliation detail".into(),
            ));
        }
        let pool = self.pool.clone();
        let dispatch = dispatch.clone();
        let lease_owner = lease_owner.to_owned();
        let agents_turn_id = agents_turn_id.map(ToOwned::to_owned);
        let detail = detail.to_owned();
        let next_attempt_at = postgres_timestamptz(next_attempt_at, "next_attempt_at")?;
        let updated_at = postgres_timestamptz(updated_at, "updated_at")?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "defer agent dispatch reconciliation")?;
            let affected = client
                .execute(
                    DEFER_DISPATCH_RECONCILIATION_SQL,
                    &[
                        &(dispatch.tenant_id as i64),
                        &(dispatch.organization_id as i64),
                        &dispatch.dispatch_id,
                        &lease_owner,
                        &agents_turn_id,
                        &detail,
                        &next_attempt_at,
                        &updated_at,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("defer agent dispatch reconciliation", error)
                })?;
            if affected != 1 {
                return Err(ContractError::Conflict(
                    "dispatch reconciliation fence rejected".into(),
                ));
            }
            Ok(())
        })
    }

    fn fail_dispatch(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        error_code: &str,
        error_detail: &str,
        next_attempt_at: &str,
        updated_at: &str,
    ) -> Result<AgentDispatchStatus, ContractError> {
        validate_dispatch_ids(dispatch)?;
        if error_code.is_empty() || error_code.len() > 128 || error_detail.len() > 2048 {
            return Err(ContractError::Invalid(
                "invalid dispatch failure detail".into(),
            ));
        }
        let pool = self.pool.clone();
        let dispatch = dispatch.clone();
        let lease_owner = lease_owner.to_owned();
        let error_code = error_code.to_owned();
        let error_detail = error_detail.to_owned();
        let next_attempt_at = postgres_timestamptz(next_attempt_at, "next_attempt_at")?;
        let updated_at = postgres_timestamptz(updated_at, "updated_at")?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "fail agent dispatch")?;
            let row = client
                .query_opt(
                    FAIL_DISPATCH_SQL,
                    &[
                        &(dispatch.tenant_id as i64),
                        &(dispatch.organization_id as i64),
                        &dispatch.dispatch_id,
                        &lease_owner,
                        &error_code,
                        &error_detail,
                        &next_attempt_at,
                        &updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("fail agent dispatch", error))?
                .ok_or_else(|| ContractError::Conflict("dispatch failure fence rejected".into()))?;
            AgentDispatchStatus::from_db_code(row.get(0))
        })
    }
}

pub(crate) fn replace_conversation_agents_in_transaction(
    tx: &mut Transaction<'_>,
    command: &ReplaceConversationAgentAssignments,
    id_generator: &dyn IdGenerator,
) -> Result<(), ContractError> {
    validate_signed_id(command.tenant_id, "tenantId", false)?;
    validate_signed_id(command.organization_id, "organizationId", true)?;
    validate_signed_id(command.assigned_by, "assignedBy", true)?;
    validate_signed_id(command.assignment_generation, "assignmentGeneration", false)?;
    validate_signed_id(
        command.source_aggregate_version,
        "sourceAggregateVersion",
        true,
    )?;
    if command.items.len() > MAX_ASSIGNMENTS || command.assignment_generation == 0 {
        return Err(ContractError::Invalid(
            "invalid conversation agent assignments".into(),
        ));
    }

    let assigned_at = postgres_timestamptz(&command.assigned_at, "assigned_at")?;
    if let Some(row) = tx
        .query_opt(
            LOCK_ASSIGNMENTS_SQL,
            &[
                &(command.tenant_id as i64),
                &(command.organization_id as i64),
                &command.conversation_id,
            ],
        )
        .map_err(|error| postgres_unavailable("assignment version lock", error))?
    {
        let current_version: i64 = row.get(0);
        let current_hash: String = row.get(1);
        let next_version = command.source_aggregate_version as i64;
        if current_version > next_version {
            return Err(ContractError::Conflict(
                "stale assignment aggregate version".into(),
            ));
        }
        if current_version == next_version {
            if current_hash == command.payload_hash {
                return Ok(());
            }
            return Err(ContractError::Conflict(
                "assignment aggregate version payload conflict".into(),
            ));
        }
    }

    tx.execute(
        REMOVE_ASSIGNMENTS_SQL,
        &[
            &(command.tenant_id as i64),
            &(command.organization_id as i64),
            &command.conversation_id,
            &command.source_event_id,
            &(command.source_aggregate_version as i64),
            &command.payload_hash,
            &assigned_at,
        ],
    )
    .map_err(|error| postgres_unavailable("assignment remove previous", error))?;
    for item in &command.items {
        let id = id_generator.next_id()?;
        let uuid = storage_uuid("assignment", id, &item.agent_id);
        tx.execute(
            UPSERT_ASSIGNMENT_SQL,
            &[
                &id,
                &uuid,
                &(command.tenant_id as i64),
                &(command.organization_id as i64),
                &command.conversation_id,
                &item.agent_id,
                &item.agent_revision_ref,
                &command.assignment_source.db_code(),
                &(command.assignment_generation as i64),
                &item.position,
                &(command.assigned_by as i64),
                &assigned_at,
                &command.source_event_id,
                &(command.source_aggregate_version as i64),
                &command.payload_hash,
            ],
        )
        .map_err(|error| postgres_unavailable("assignment upsert", error))?;
    }
    Ok(())
}

fn storage_uuid(kind: &str, id: i64, resource_id: &str) -> String {
    let digest = sha256_hash(format!("{kind}:{id}:{resource_id}").as_bytes());
    format!("im-{kind}-{}", &digest[..48])
}

pub(crate) fn enqueue_dispatches_in_transaction(
    tx: &mut postgres::Transaction<'_>,
    request: &AgentMentionDispatchRequest,
    max_attempts: u32,
    id_generator: &dyn IdGenerator,
) -> Result<Vec<AgentDispatchRecord>, ContractError> {
    request.validate()?;
    if max_attempts == 0 || max_attempts > 100 {
        return Err(ContractError::Invalid(
            "invalid dispatch max attempts".into(),
        ));
    }
    let tenant_id = parse_positive_id(&request.tenant_id, "tenantId")?;
    let organization_id = parse_id(&request.organization_id, "organizationId")?;
    let source_message_id = parse_positive_id(&request.message_id, "messageId")?;
    let requested_by = parse_positive_id(&request.sender_principal_id, "requestedBy")?;
    validate_signed_id(request.message_seq, "messageSeq", false)?;
    validate_signed_id(request.assignment_generation, "assignmentGeneration", false)?;
    let requested_at = postgres_timestamptz(&request.requested_at, "requested_at")?;
    let payload = serde_json::to_vec(request)
        .map_err(|_| ContractError::Invalid("invalid dispatch payload".into()))?;
    let payload_hash = sha256_hash(&payload);
    let mut records = Vec::with_capacity(request.targets.len());
    for target in &request.targets {
        let id = id_generator.next_id()?;
        let uuid = storage_uuid("dispatch", id, &target.dispatch_id);
        let idempotency_key = format!("im-agent-dispatch:{}", target.dispatch_id);
        tx.execute(
            INSERT_DISPATCH_SQL,
            &[
                &id,
                &uuid,
                &target.dispatch_id,
                &(tenant_id as i64),
                &(organization_id as i64),
                &request.conversation_id,
                &(source_message_id as i64),
                &(request.message_seq as i64),
                &target.agent_id,
                &target.revision_id,
                &(request.assignment_generation as i64),
                &idempotency_key,
                &payload_hash,
                &(max_attempts as i32),
                &requested_at,
                &(requested_by as i64),
            ],
        )
        .map_err(|error| postgres_unavailable("dispatch transaction enqueue", error))?;
        let row = tx
            .query_one(
                SELECT_DISPATCH_BY_ID_SQL,
                &[
                    &(tenant_id as i64),
                    &(organization_id as i64),
                    &target.dispatch_id,
                ],
            )
            .map_err(|error| postgres_unavailable("dispatch transaction replay load", error))?;
        let record = dispatch_from_row(row)?;
        if record.payload_hash != payload_hash {
            return Err(ContractError::Conflict(
                "dispatch idempotency payload conflict".into(),
            ));
        }
        records.push(record);
    }
    Ok(records)
}

fn parse_id(value: &str, field: &str) -> Result<u64, ContractError> {
    let value = value
        .parse::<u64>()
        .map_err(|_| ContractError::Invalid(format!("{field} must be an int64 string")))?;
    if value > i64::MAX as u64 {
        return Err(ContractError::Invalid(format!(
            "{field} is outside int64 range"
        )));
    }
    Ok(value)
}

fn validate_signed_id(value: u64, field: &str, allow_zero: bool) -> Result<(), ContractError> {
    if value > i64::MAX as u64 || (!allow_zero && value == 0) {
        return Err(ContractError::Invalid(format!(
            "{field} is outside int64 range"
        )));
    }
    Ok(())
}

fn validate_dispatch_ids(dispatch: &AgentDispatchRecord) -> Result<(), ContractError> {
    validate_signed_id(dispatch.tenant_id, "tenantId", false)?;
    validate_signed_id(dispatch.organization_id, "organizationId", true)?;
    validate_signed_id(dispatch.source_message_id, "sourceMessageId", false)?;
    validate_signed_id(dispatch.source_message_seq, "sourceMessageSeq", false)?;
    validate_signed_id(
        dispatch.assignment_generation,
        "assignmentGeneration",
        false,
    )?;
    validate_signed_id(dispatch.requested_by, "requestedBy", false)?;
    if let Some(reply_message_id) = dispatch.reply_message_id {
        validate_signed_id(reply_message_id, "replyMessageId", false)?;
    }
    if let Some(reply_message_seq) = dispatch.reply_message_seq {
        validate_signed_id(reply_message_seq, "replyMessageSeq", false)?;
    }
    Ok(())
}

fn parse_positive_id(value: &str, field: &str) -> Result<u64, ContractError> {
    let value = parse_id(value, field)?;
    if value == 0 {
        return Err(ContractError::Invalid(format!(
            "{field} is outside int64 range"
        )));
    }
    Ok(value)
}

fn assignment_from_row(
    row: postgres::Row,
) -> Result<ConversationAgentAssignmentRecord, ContractError> {
    Ok(ConversationAgentAssignmentRecord {
        tenant_id: row.get::<_, i64>(0) as u64,
        organization_id: row.get::<_, i64>(1) as u64,
        conversation_id: row.get(2),
        agent_id: row.get(3),
        agent_revision_ref: row.get(4),
        assignment_source: AgentAssignmentSource::from_db_code(row.get(5))?,
        assignment_generation: row.get::<_, i64>(6) as u64,
        position: row.get(7),
        enabled: row.get(8),
        status: row.get(9),
        source_aggregate_version: row.get::<_, i64>(10) as u64,
    })
}

fn dispatch_from_row(row: postgres::Row) -> Result<AgentDispatchRecord, ContractError> {
    Ok(AgentDispatchRecord {
        dispatch_id: row.get(0),
        tenant_id: row.get::<_, i64>(1) as u64,
        organization_id: row.get::<_, i64>(2) as u64,
        conversation_id: row.get(3),
        source_message_id: row.get::<_, i64>(4) as u64,
        source_message_seq: row.get::<_, i64>(5) as u64,
        agent_id: row.get(6),
        agent_revision_ref: row.get(7),
        assignment_generation: row.get::<_, i64>(8) as u64,
        binding_id: row.get(9),
        agents_session_id: row.get(10),
        agents_turn_id: row.get(11),
        status: AgentDispatchStatus::from_db_code(row.get(12))?,
        idempotency_key: row.get(13),
        payload_hash: row.get(14),
        attempt_count: row.get::<_, i32>(15) as u32,
        max_attempts: row.get::<_, i32>(16) as u32,
        lease_owner: row.get(17),
        lease_expires_at: row
            .get::<_, Option<chrono::DateTime<chrono::Utc>>>(18)
            .map(|value| value.to_rfc3339()),
        next_attempt_at: row.get::<_, chrono::DateTime<chrono::Utc>>(19).to_rfc3339(),
        requested_by: row.get::<_, i64>(20) as u64,
        reply_message_id: row.get::<_, Option<i64>>(21).map(|value| value as u64),
        reply_message_seq: row.get::<_, Option<i64>>(22).map(|value| value as u64),
        created_at: row.get::<_, chrono::DateTime<chrono::Utc>>(23).to_rfc3339(),
        updated_at: row.get::<_, chrono::DateTime<chrono::Utc>>(24).to_rfc3339(),
    })
}

fn binding_from_row(row: postgres::Row) -> Result<ConversationAgentBindingRecord, ContractError> {
    Ok(ConversationAgentBindingRecord {
        binding_id: row.get(0),
        tenant_id: row.get::<_, i64>(1) as u64,
        organization_id: row.get::<_, i64>(2) as u64,
        conversation_id: row.get(3),
        agent_id: row.get(4),
        agent_revision_ref: row.get(5),
        assignment_generation: row.get::<_, i64>(6) as u64,
        agents_session_id: row.get(7),
        status: AgentBindingStatus::from_db_code(row.get(8))?,
        idempotency_key: row.get(9),
        payload_hash: row.get(10),
        created_by: row.get::<_, i64>(11) as u64,
        updated_by: row.get::<_, i64>(12) as u64,
        last_error_code: row.get(13),
        last_error_detail: row.get(14),
        version: row.get::<_, i64>(15) as u64,
        created_at: row.get::<_, chrono::DateTime<chrono::Utc>>(16).to_rfc3339(),
        updated_at: row.get::<_, chrono::DateTime<chrono::Utc>>(17).to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_sql_is_scoped_leased_and_store_paginated() {
        assert!(LIST_ASSIGNMENTS_SQL.contains("tenant_id = $1"));
        assert!(LIST_ASSIGNMENTS_SQL.contains("organization_id = $2"));
        assert!(LIST_ASSIGNMENTS_SQL.contains("limit $4"));
        assert!(CLAIM_DISPATCH_SQL.contains("for update skip locked"));
        assert!(CLAIM_DISPATCH_SQL.contains("attempt_count < max_attempts"));
        assert!(
            CLAIM_DISPATCH_SQL.contains("case when candidates.status in (0, 5) then 1 else 0 end")
        );
        assert!(CLAIM_DISPATCH_GLOBAL_SQL.contains("for update skip locked"));
        assert!(
            CLAIM_DISPATCH_GLOBAL_SQL
                .contains("case when candidates.status in (0, 5) then 1 else 0 end")
        );
        assert!(DEFER_DISPATCH_RECONCILIATION_SQL.contains("agents_turn_indeterminate"));
        assert!(DEFER_DISPATCH_RECONCILIATION_SQL.contains("lease_expires_at = $7"));
        assert!(UPDATE_BINDING_SQL.contains("$2::smallint"));
        assert!(!INSERT_DISPATCH_SQL.contains("ai_agent_"));
    }

    #[test]
    fn integration_ids_reject_unsigned_values_outside_postgres_bigint() {
        assert!(parse_id("9223372036854775807", "tenantId").is_ok());
        assert!(parse_id("9223372036854775808", "tenantId").is_err());
        assert!(parse_positive_id("0", "requestedBy").is_err());
        assert!(validate_signed_id(i64::MAX as u64, "id", false).is_ok());
        assert!(validate_signed_id(i64::MAX as u64 + 1, "id", false).is_err());
        assert!(validate_signed_id(0, "systemActorId", true).is_ok());
        assert!(validate_signed_id(0, "userActorId", false).is_err());
        assert!(validate_signed_id(0, "sourceAggregateVersion", true).is_ok());
    }
}
