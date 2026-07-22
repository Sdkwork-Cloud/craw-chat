use std::collections::{BTreeSet, HashMap, VecDeque};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, Mutex, MutexGuard};

use im_app_context::AppContext;
use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    CONVERSATION_AGGREGATE_PAGE_SIZE_MAX, ConversationAggregateStore,
    ConversationMemberPageCursor, normalize_realtime_organization_id,
};
use im_time::utc_now_rfc3339_millis;
use conversation_runtime::conversation_state::ConversationStateService;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitJournal, CommitPosition};
use sdkwork_im_contract_notification::{
    NotificationTaskListCursor, NotificationTaskRecord, NotificationTaskStore,
};
use sdkwork_utils_rust::{
    DEFAULT_LIST_PAGE_SIZE, MAX_LIST_PAGE_SIZE, SdkWorkCursorListQuery, SdkWorkPageData,
    base64url_decode, base64url_encode, cursor_list_page_data,
};
use tokio::sync::Semaphore;

use crate::dto::{
    NotificationRecipient, NotificationRequestDeliveryStatus, NotificationRequestResult,
    RequestAutomationResultNotification, RequestMessagePostedNotifications, RequestNotification,
    RequestNotificationFanout,
};
use crate::error::NotificationError;
use crate::helpers::{
    NotificationRecipientIndex, automation_notification_id,
    automation_notification_source_event_id, delivery_status_from_notification_status,
    ensure_notification_request_access, fanout_notification_id,
    insert_notification_recipient_index, notification_matches_request,
    notification_recipient_scope_key, notification_request_key, notification_scope_key,
    notification_visible_to_actor, remove_notification_recipient_index,
    validate_notification_request_payload_size,
};

#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<NotificationRuntime>,
}

#[derive(Clone)]
pub(crate) struct PublicAppGuardrails {
    pub(crate) request_gate: Arc<Semaphore>,
}

pub struct NotificationRuntime {
    pub(crate) tasks: Mutex<NotificationRuntimeTaskState>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    task_store: Arc<dyn NotificationTaskStore>,
    conversation_recipients: ConversationRecipientSource,
}

enum ConversationRecipientSource {
    Aggregate(Arc<dyn ConversationAggregateStore>),
    TestCache(Arc<ConversationStateService>),
}

#[derive(Default)]
pub(crate) struct NotificationRuntimeTaskState {
    tasks: HashMap<String, NotificationTask>,
    insertion_order: VecDeque<String>,
    estimated_bytes: usize,
}

fn decode_notification_cursor(
    auth: &AppContext,
    cursor: &str,
) -> Result<NotificationTaskListCursor, NotificationError> {
    let cursor = cursor.trim();
    if cursor.is_empty() || cursor.len() > 4 * 1024 {
        return Err(NotificationError::invalid_parameter(
            "cursor is empty or exceeds 4096 bytes",
        ));
    }
    let bytes = base64url_decode(cursor)
        .ok_or_else(|| NotificationError::invalid_parameter("cursor is not valid base64url"))?;
    let payload: NotificationCursorPayload = serde_json::from_slice(bytes.as_slice())
        .map_err(|_| NotificationError::invalid_parameter("cursor payload is invalid"))?;
    if payload.version != 1
        || payload.tenant_id != auth.tenant_id
        || payload.organization_id != auth.organization_id
        || payload.recipient_kind != auth.actor_kind
        || payload.recipient_id != auth.actor_id
        || payload.updated_at.trim().is_empty()
        || payload.notification_id.trim().is_empty()
    {
        return Err(NotificationError::invalid_parameter(
            "cursor does not match the authenticated notification scope",
        ));
    }
    Ok(NotificationTaskListCursor {
        updated_at: payload.updated_at,
        notification_id: payload.notification_id,
    })
}

fn encode_notification_cursor(
    auth: &AppContext,
    record: &NotificationTaskRecord,
) -> Result<String, NotificationError> {
    let bytes = serde_json::to_vec(&NotificationCursorPayload {
        version: 1,
        tenant_id: auth.tenant_id.clone(),
        organization_id: auth.organization_id.clone(),
        recipient_kind: auth.actor_kind.clone(),
        recipient_id: auth.actor_id.clone(),
        updated_at: record.updated_at.clone(),
        notification_id: record.notification_id.clone(),
    })
    .map_err(|error| {
        NotificationError::internal(
            "notification_cursor_encode_failed",
            format!("failed to encode notification cursor: {error}"),
        )
    })?;
    Ok(base64url_encode(bytes.as_slice()))
}

const NOTIFICATION_CACHE_MAX_ENTRIES: usize = 1_024;
const NOTIFICATION_CACHE_MAX_BYTES: usize = 64 * 1024 * 1024;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct NotificationCursorPayload {
    version: u8,
    tenant_id: String,
    organization_id: String,
    recipient_kind: String,
    recipient_id: String,
    updated_at: String,
    notification_id: String,
}

#[derive(Default)]
struct NoopJournal;

impl CommitJournal for NoopJournal {
    fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("noop", 0))
    }
}

impl Default for NotificationRuntime {
    fn default() -> Self {
        Self::with_journal(Arc::new(NoopJournal))
    }
}

impl NotificationRuntime {
    pub fn with_journal<J>(journal: Arc<J>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_journal_and_store_and_conversation_state(
            journal,
            Arc::new(RuntimeMemoryNotificationTaskStore::default()),
            Arc::new(ConversationStateService::default()),
        )
    }

    pub fn with_journal_and_store<J, S>(journal: Arc<J>, task_store: Arc<S>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: NotificationTaskStore + 'static,
    {
        Self::with_journal_and_store_and_conversation_state(
            journal,
            task_store,
            Arc::new(ConversationStateService::default()),
        )
    }

    pub fn with_journal_and_conversation_state<J>(
        journal: Arc<J>,
        conversation_state_service: Arc<ConversationStateService>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_journal_and_store_and_conversation_state(
            journal,
            Arc::new(RuntimeMemoryNotificationTaskStore::default()),
            conversation_state_service,
        )
    }

    pub fn with_journal_and_store_and_conversation_state<J, S>(
        journal: Arc<J>,
        task_store: Arc<S>,
        conversation_state_service: Arc<ConversationStateService>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: NotificationTaskStore + 'static,
    {
        Self {
            tasks: Mutex::new(NotificationRuntimeTaskState::default()),
            journal,
            task_store,
            conversation_recipients: ConversationRecipientSource::TestCache(
                conversation_state_service,
            ),
        }
    }

    pub fn with_dyn_task_store_and_conversation_state<J>(
        journal: Arc<J>,
        task_store: Arc<dyn NotificationTaskStore>,
        conversation_state_service: Arc<ConversationStateService>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self {
            tasks: Mutex::new(NotificationRuntimeTaskState::default()),
            journal,
            task_store,
            conversation_recipients: ConversationRecipientSource::TestCache(
                conversation_state_service,
            ),
        }
    }

    pub fn with_dyn_task_store_and_aggregate_store<J>(
        journal: Arc<J>,
        task_store: Arc<dyn NotificationTaskStore>,
        aggregate_store: Arc<dyn ConversationAggregateStore>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self {
            tasks: Mutex::new(NotificationRuntimeTaskState::default()),
            journal,
            task_store,
            conversation_recipients: ConversationRecipientSource::Aggregate(aggregate_store),
        }
    }

    fn ensure_notification_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
    ) -> Result<(), NotificationError> {
        let scope_key = notification_scope_key(tenant_id, organization_id, notification_id);
        if self
            .tasks
            .lock_notification()
            .tasks
            .contains_key(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .task_store
            .load_task(tenant_id, organization_id, notification_id)
            .map_err(NotificationError::notification_store)?;
        if let Some(record) = restored {
            let mut state = self.tasks.lock_notification();
            insert_runtime_notification_task(
                &mut state,
                scope_key,
                record.task,
                record.organization_id.as_str(),
            );
        }

        Ok(())
    }

    pub fn request_notification(
        &self,
        auth: &AppContext,
        request: RequestNotification,
    ) -> Result<NotificationTask, NotificationError> {
        Ok(self.request_notification_with_outcome(auth, request)?.task)
    }

    pub fn request_notification_with_outcome(
        &self,
        auth: &AppContext,
        request: RequestNotification,
    ) -> Result<NotificationRequestResult, NotificationError> {
        validate_notification_request_payload_size(&request)?;
        self.ensure_notification_task(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            request.notification_id.as_str(),
        )?;
        let request_key = notification_request_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            request.notification_id.as_str(),
        );
        let notification_key = notification_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            request.notification_id.as_str(),
        );
        let mut state = self.tasks.lock_notification();

        if let Some(existing) = state.tasks.get(notification_key.as_str()).cloned() {
            if notification_matches_request(&existing, &request) {
                let delivery_status = delivery_status_from_notification_status(&existing.status);
                return Ok(NotificationRequestResult {
                    task: existing,
                    is_new: false,
                    request_key,
                    delivery_status,
                });
            }

            return Err(NotificationError::conflict(
                request.notification_id.as_str(),
            ));
        }

        let requested_at = utc_now_rfc3339_millis();
        let requested = NotificationTask {
            tenant_id: auth.tenant_id.clone(),
            notification_id: request.notification_id.clone(),
            source_event_id: request.source_event_id.clone(),
            source_event_type: request.source_event_type.clone(),
            category: request.category.clone(),
            channel: request.channel.clone(),
            recipient_id: request.recipient_id.clone(),
            recipient_kind: request.recipient_kind.clone(),
            status: NotificationStatus::Requested,
            title: request.title.clone(),
            body: request.body.clone(),
            payload: request.payload.clone(),
            requested_at: requested_at.clone(),
            dispatched_at: None,
            failure_reason: None,
        };
        self.append_event(auth, &requested, "notification.requested", 1)?;
        insert_runtime_notification_task(
            &mut state,
            notification_key.clone(),
            requested.clone(),
            auth.organization_id.as_str(),
        );
        if let Err(error) = self
            .task_store
            .save_task(self.task_record(auth, &requested))
        {
            remove_runtime_notification_task(
                &mut state,
                notification_key.as_str(),
                auth.organization_id.as_str(),
            );
            return Err(NotificationError::notification_store(error));
        }

        Ok(NotificationRequestResult {
            task: requested,
            is_new: true,
            request_key,
            delivery_status: NotificationRequestDeliveryStatus::Accepted,
        })
    }

    pub fn request_notification_from_app_context(
        &self,
        auth: &AppContext,
        request: RequestNotification,
    ) -> Result<NotificationRequestResult, NotificationError> {
        ensure_notification_request_access(
            auth,
            request.recipient_id.as_str(),
            request.recipient_kind.as_str(),
        )?;
        self.request_notification_with_outcome(auth, request)
    }

    pub fn request_notification_fanout(
        &self,
        auth: &AppContext,
        request: RequestNotificationFanout,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        let mut tasks = Vec::new();

        for recipient in request.recipients.into_iter().filter(|recipient| {
            recipient.recipient_id != auth.actor_id || recipient.recipient_kind != auth.actor_kind
        }) {
            tasks.push(self.request_notification(
                auth,
                RequestNotification {
                    notification_id: fanout_notification_id(
                        request.notification_id_seed.as_str(),
                        &recipient,
                    ),
                    source_event_id: request.source_event_id.clone(),
                    source_event_type: request.source_event_type.clone(),
                    category: request.category.clone(),
                    channel: request.channel.clone(),
                    recipient_id: recipient.recipient_id,
                    recipient_kind: recipient.recipient_kind,
                    title: request.title.clone(),
                    body: request.body.clone(),
                    payload: request.payload.clone(),
                },
            )?);
        }

        Ok(tasks)
    }

    pub fn request_message_posted_notifications(
        &self,
        auth: &AppContext,
        request: RequestMessagePostedNotifications,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        let RequestMessagePostedNotifications {
            source_event_id,
            conversation_id,
            message_id,
            message_seq,
            message_type,
            summary,
        } = request;
        let category = if message_type == "signal" {
            "rtc.event"
        } else {
            "message.new"
        };
        let recipients =
            self.message_posted_notification_recipients(auth, conversation_id.as_str())?;
        let notification_id_seed = message_id.clone();
        let payload = serde_json::json!({
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "messageType": message_type,
        })
        .to_string();

        self.request_notification_fanout(
            auth,
            RequestNotificationFanout {
                notification_id_seed,
                source_event_id,
                source_event_type: "message.posted".into(),
                category: category.into(),
                channel: "inapp".into(),
                recipients,
                title: summary.clone(),
                body: summary,
                payload: Some(payload),
            },
        )
    }

    fn message_posted_notification_recipients(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<BTreeSet<NotificationRecipient>, NotificationError> {
        match &self.conversation_recipients {
            ConversationRecipientSource::Aggregate(store) => {
                let organization_id =
                    normalize_realtime_organization_id(auth.organization_id.as_str());
                let sender = store
                    .load_member(
                        auth.tenant_id.as_str(),
                        organization_id.as_str(),
                        conversation_id,
                        auth.actor_kind.as_str(),
                        auth.actor_id.as_str(),
                    )
                    .map_err(NotificationError::notification_store)?;
                if !sender.is_some_and(|member| {
                    matches!(member.membership_state.as_str(), "joined" | "linked")
                }) {
                    return Err(NotificationError::forbidden(
                        "conversation_membership_required",
                        "active conversation membership is required for notification fanout",
                    ));
                }

                let joined_before_or_at = utc_now_rfc3339_millis();
                let mut cursor: Option<ConversationMemberPageCursor> = None;
                let mut recipients = BTreeSet::new();
                loop {
                    let page = store
                        .load_event_recipients_page(
                            auth.tenant_id.as_str(),
                            organization_id.as_str(),
                            conversation_id,
                            joined_before_or_at.as_str(),
                            cursor.as_ref(),
                            CONVERSATION_AGGREGATE_PAGE_SIZE_MAX,
                        )
                        .map_err(NotificationError::notification_store)?;
                    recipients.extend(page.items.into_iter().map(|member| {
                        NotificationRecipient {
                            recipient_id: member.principal_id,
                            recipient_kind: member.principal_kind,
                        }
                    }));
                    if !page.has_more {
                        break;
                    }
                    cursor = page.next_cursor;
                    if cursor.is_none() {
                        return Err(NotificationError::internal(
                            "conversation_recipient_cursor_missing",
                            "conversation recipient page is incomplete without a continuation cursor",
                        ));
                    }
                }
                Ok(recipients)
            }
            ConversationRecipientSource::TestCache(service) => Ok(service
                .message_posted_notification_recipients_from_auth_context(auth, conversation_id)?
                .into_iter()
                .map(|recipient| NotificationRecipient {
                    recipient_id: recipient.principal_id,
                    recipient_kind: recipient.principal_kind,
                })
                .collect()),
        }
    }

    pub fn request_automation_result_notification(
        &self,
        auth: &AppContext,
        request: RequestAutomationResultNotification,
    ) -> Result<NotificationTask, NotificationError> {
        self.request_notification(
            auth,
            RequestNotification {
                notification_id: automation_notification_id(
                    auth.actor_kind.as_str(),
                    request.execution_id.as_str(),
                ),
                source_event_id: automation_notification_source_event_id(
                    auth.actor_kind.as_str(),
                    request.execution_id.as_str(),
                ),
                source_event_type: "automation.execution_completed".into(),
                category: "automation.result".into(),
                channel: "inapp".into(),
                recipient_id: auth.actor_id.clone(),
                recipient_kind: auth.actor_kind.clone(),
                title: Some("Automation completed".into()),
                body: Some(request.target_ref),
                payload: request.output_payload,
            },
        )
    }

    pub fn list_notifications(
        &self,
        auth: &AppContext,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        Ok(self
            .list_notifications_page(auth, SdkWorkCursorListQuery::default())?
            .items)
    }

    pub fn list_notifications_page(
        &self,
        auth: &AppContext,
        query: SdkWorkCursorListQuery,
    ) -> Result<SdkWorkPageData<NotificationTask>, NotificationError> {
        let page_size = match query.page_size {
            None => usize::try_from(DEFAULT_LIST_PAGE_SIZE).unwrap_or(20),
            Some(value) if value > 0 && value <= MAX_LIST_PAGE_SIZE => usize::try_from(value)
                .map_err(|_| {
                    NotificationError::invalid_parameter("page_size exceeds platform range")
                })?,
            Some(value) => {
                return Err(NotificationError::invalid_parameter(format!(
                    "page_size must be between 1 and {MAX_LIST_PAGE_SIZE}, actual={value}"
                )));
            }
        };
        let cursor = query
            .cursor
            .as_deref()
            .map(|value| decode_notification_cursor(auth, value))
            .transpose()?;
        let mut records = self
            .task_store
            .list_tasks_for_recipient_page(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_kind.as_str(),
                auth.actor_id.as_str(),
                cursor.as_ref(),
                page_size,
            )
            .map_err(NotificationError::notification_store)?;
        let has_more = records.len() > page_size;
        if has_more {
            records.truncate(page_size);
        }
        let next_cursor = if has_more {
            records
                .last()
                .map(|record| encode_notification_cursor(auth, record))
                .transpose()?
        } else {
            None
        };
        let items = records.into_iter().map(|record| record.task).collect();
        Ok(cursor_list_page_data(
            items,
            page_size,
            next_cursor,
            has_more,
        ))
    }

    pub fn get_notification(
        &self,
        auth: &AppContext,
        notification_id: &str,
    ) -> Result<NotificationTask, NotificationError> {
        self.ensure_notification_task(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            notification_id,
        )?;
        self.tasks
            .lock_notification()
            .tasks
            .get(
                notification_scope_key(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    notification_id,
                )
                .as_str(),
            )
            .filter(|task| notification_visible_to_actor(task, auth))
            .cloned()
            .ok_or_else(|| NotificationError::not_found(notification_id))
    }

    fn task_record(&self, auth: &AppContext, task: &NotificationTask) -> NotificationTaskRecord {
        NotificationTaskRecord {
            tenant_id: task.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            notification_id: task.notification_id.clone(),
            task: task.clone(),
            updated_at: utc_now_rfc3339_millis(),
        }
    }

    fn append_event(
        &self,
        auth: &AppContext,
        task: &NotificationTask,
        event_type: &str,
        ordering_seq: u64,
    ) -> Result<(), NotificationError> {
        let committed_at = task
            .dispatched_at
            .clone()
            .unwrap_or_else(|| task.requested_at.clone());
        let envelope = CommitEnvelope {
            event_id: format!(
                "evt_{}_{}",
                task.notification_id,
                event_type.replace('.', "_")
            ),
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::Notification,
            aggregate_id: task.notification_id.clone(),
            scope_type: "notification".into(),
            scope_id: task.notification_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                auth.tenant_id.as_str(),
                task.notification_id.as_str(),
            ),
            ordering_seq,
            causation_id: Some(task.source_event_id.clone()),
            correlation_id: Some(task.source_event_id.clone()),
            idempotency_key: Some(format!(
                "{}:{}:{}",
                task.notification_id, event_type, ordering_seq
            )),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: task.requested_at.clone(),
            committed_at,
            payload_schema: Some("notification.task.v1".into()),
            payload: serde_json::to_string(task).map_err(|error| NotificationError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "notification_payload_invalid",
                message: format!(
                    "failed to serialize notification task into commit envelope: {error}"
                ),
            })?,
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub(crate) struct RuntimeMemoryNotificationTaskStore {
    pub(crate) state: Arc<Mutex<RuntimeMemoryNotificationTaskState>>,
}

#[derive(Default)]
pub(crate) struct RuntimeMemoryNotificationTaskState {
    tasks: HashMap<String, NotificationTaskRecord>,
    tasks_by_recipient: NotificationRecipientIndex,
}

impl NotificationTaskStore for RuntimeMemoryNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        Ok(self
            .state
            .lock_notification()
            .tasks
            .get(notification_scope_key(tenant_id, organization_id, notification_id).as_str())
            .cloned())
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let notification_key = notification_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.notification_id.as_str(),
        );
        let mut state = self.state.lock_notification();
        if let Some(previous) = state.tasks.get(notification_key.as_str()).cloned() {
            remove_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &previous,
            );
            let merged = previous.merge_monotonic(record);
            insert_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &merged,
            );
            state.tasks.insert(notification_key, merged);
            return Ok(());
        }
        insert_notification_recipient_index(
            &mut state.tasks_by_recipient,
            notification_key.as_str(),
            &record,
        );
        state.tasks.insert(notification_key, record);
        Ok(())
    }

    fn list_tasks_for_recipient_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
        cursor: Option<&NotificationTaskListCursor>,
        page_size: usize,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let state = self.state.lock_notification();
        let recipient_key = notification_recipient_scope_key(
            tenant_id,
            organization_id,
            recipient_kind,
            recipient_id,
        );
        let Some(index) = state.tasks_by_recipient.get(recipient_key.as_str()) else {
            return Ok(Vec::new());
        };
        let cursor_key = cursor.map(|value| {
            crate::helpers::NotificationRecipientSortKey(std::cmp::Reverse((
                value.updated_at.clone(),
                value.notification_id.clone(),
            )))
        });
        let values: Box<dyn Iterator<Item = &String> + '_> = match cursor_key.as_ref() {
            Some(key) => Box::new(
                index
                    .range((Excluded(key), Unbounded))
                    .map(|(_, value)| value),
            ),
            None => Box::new(index.values()),
        };
        Ok(values
            .take(page_size.saturating_add(1))
            .filter_map(|task_key| state.tasks.get(task_key.as_str()).cloned())
            .collect())
    }
}

fn insert_runtime_notification_task(
    state: &mut NotificationRuntimeTaskState,
    notification_key: String,
    task: NotificationTask,
    _organization_id: &str,
) {
    if let Some(previous) = state.tasks.get(notification_key.as_str()).cloned() {
        state.estimated_bytes = state
            .estimated_bytes
            .saturating_sub(estimated_task_bytes(&previous));
    }
    state
        .insertion_order
        .retain(|key| key != notification_key.as_str());
    state.insertion_order.push_back(notification_key.clone());
    state.estimated_bytes = state
        .estimated_bytes
        .saturating_add(estimated_task_bytes(&task));
    state.tasks.insert(notification_key, task);
    while state.tasks.len() > NOTIFICATION_CACHE_MAX_ENTRIES
        || state.estimated_bytes > NOTIFICATION_CACHE_MAX_BYTES
    {
        let Some(evicted_key) = state.insertion_order.pop_front() else {
            break;
        };
        if let Some(evicted) = state.tasks.remove(evicted_key.as_str()) {
            state.estimated_bytes = state
                .estimated_bytes
                .saturating_sub(estimated_task_bytes(&evicted));
        }
    }
}

fn remove_runtime_notification_task(
    state: &mut NotificationRuntimeTaskState,
    notification_key: &str,
    _organization_id: &str,
) -> Option<NotificationTask> {
    let removed = state.tasks.remove(notification_key)?;
    state.insertion_order.retain(|key| key != notification_key);
    state.estimated_bytes = state
        .estimated_bytes
        .saturating_sub(estimated_task_bytes(&removed));
    Some(removed)
}

fn estimated_task_bytes(task: &NotificationTask) -> usize {
    task.tenant_id.len()
        + task.notification_id.len()
        + task.source_event_id.len()
        + task.source_event_type.len()
        + task.category.len()
        + task.channel.len()
        + task.recipient_id.len()
        + task.recipient_kind.len()
        + task.title.as_ref().map_or(0, String::len)
        + task.body.as_ref().map_or(0, String::len)
        + task.payload.as_ref().map_or(0, String::len)
        + task.requested_at.len()
        + task.dispatched_at.as_ref().map_or(0, String::len)
        + task.failure_reason.as_ref().map_or(0, String::len)
        + std::mem::size_of::<NotificationTask>()
}

trait NotificationMutexExt<T> {
    fn lock_notification(&self) -> MutexGuard<'_, T>;
}

impl<T> NotificationMutexExt<T> for Mutex<T> {
    fn lock_notification(&self) -> MutexGuard<'_, T> {
        match self.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!("recovering poisoned mutex in notification-service");
                poisoned.into_inner()
            }
        }
    }
}
