use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};

use im_app_context::AppContext;
use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_time::utc_now_rfc3339_millis;
use projection_service::TimelineProjectionService;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitJournal, CommitPosition};
use sdkwork_im_contract_notification::{NotificationTaskRecord, NotificationTaskStore};
use tokio::sync::Semaphore;

use crate::dto::{
    NotificationRecipient, NotificationRequestDeliveryStatus, NotificationRequestResult,
    RequestAutomationResultNotification, RequestMessagePostedNotifications, RequestNotification,
    RequestNotificationFanout,
};
use crate::error::NotificationError;
use crate::helpers::{
    automation_notification_id, automation_notification_source_event_id,
    delivery_status_from_notification_status, ensure_notification_request_access,
    fanout_notification_id, insert_notification_recipient_index, notification_matches_request,
    notification_recipient_scope_key, notification_request_key, notification_scope_key,
    notification_sort_key, notification_visible_to_actor, remove_notification_recipient_index,
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
    pub(crate) restored_recipients: Mutex<HashSet<String>>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    task_store: Arc<dyn NotificationTaskStore>,
    projection_service: Arc<TimelineProjectionService>,
}

#[derive(Default)]
pub(crate) struct NotificationRuntimeTaskState {
    tasks: HashMap<String, NotificationTask>,
    tasks_by_recipient: HashMap<String, BTreeSet<String>>,
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
        Self::with_journal_and_store_and_projection(
            journal,
            Arc::new(RuntimeMemoryNotificationTaskStore::default()),
            Arc::new(TimelineProjectionService::default()),
        )
    }

    pub fn with_journal_and_store<J, S>(journal: Arc<J>, task_store: Arc<S>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: NotificationTaskStore + 'static,
    {
        Self::with_journal_and_store_and_projection(
            journal,
            task_store,
            Arc::new(TimelineProjectionService::default()),
        )
    }

    pub fn with_journal_and_projection<J>(
        journal: Arc<J>,
        projection_service: Arc<TimelineProjectionService>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_journal_and_store_and_projection(
            journal,
            Arc::new(RuntimeMemoryNotificationTaskStore::default()),
            projection_service,
        )
    }

    pub fn with_journal_and_store_and_projection<J, S>(
        journal: Arc<J>,
        task_store: Arc<S>,
        projection_service: Arc<TimelineProjectionService>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: NotificationTaskStore + 'static,
    {
        Self {
            tasks: Mutex::new(NotificationRuntimeTaskState::default()),
            restored_recipients: Mutex::new(HashSet::new()),
            journal,
            task_store,
            projection_service,
        }
    }

    fn ensure_notification_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<(), NotificationError> {
        let scope_key = notification_scope_key(tenant_id, notification_id);
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
            .load_task(tenant_id, notification_id)
            .map_err(NotificationError::notification_store)?;
        if let Some(record) = restored {
            let mut state = self.tasks.lock_notification();
            insert_runtime_notification_task(&mut state, scope_key, record.task);
        }

        Ok(())
    }

    fn ensure_recipient_tasks(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<(), NotificationError> {
        let recipient_key =
            notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id);
        if self
            .restored_recipients
            .lock_notification()
            .contains(recipient_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .task_store
            .list_tasks_for_recipient(tenant_id, recipient_kind, recipient_id)
            .map_err(NotificationError::notification_store)?;
        let mut state = self.tasks.lock_notification();
        for record in restored {
            insert_runtime_notification_task(
                &mut state,
                notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
                record.task,
            );
        }
        drop(state);
        self.restored_recipients
            .lock_notification()
            .insert(recipient_key);

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
        self.ensure_notification_task(auth.tenant_id.as_str(), request.notification_id.as_str())?;
        let request_key =
            notification_request_key(auth.tenant_id.as_str(), request.notification_id.as_str());
        let notification_key =
            notification_scope_key(auth.tenant_id.as_str(), request.notification_id.as_str());
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
        let dispatched_at = utc_now_rfc3339_millis();

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

        let dispatched = NotificationTask {
            status: NotificationStatus::Dispatched,
            dispatched_at: Some(dispatched_at),
            ..requested
        };
        self.append_event(auth, &dispatched, "notification.dispatched", 2)?;

        insert_runtime_notification_task(&mut state, notification_key.clone(), dispatched.clone());
        if let Err(error) = self.task_store.save_task(self.task_record(&dispatched)) {
            remove_runtime_notification_task(&mut state, notification_key.as_str());
            return Err(NotificationError::notification_store(error));
        }

        Ok(NotificationRequestResult {
            task: dispatched,
            is_new: true,
            request_key,
            delivery_status: NotificationRequestDeliveryStatus::Applied,
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
        let recipients = self
            .projection_service
            .message_posted_notification_recipients_from_auth_context(
                auth,
                conversation_id.as_str(),
            )?
            .into_iter()
            .map(|recipient| NotificationRecipient {
                recipient_id: recipient.principal_id,
                recipient_kind: recipient.principal_kind,
            })
            .collect::<BTreeSet<_>>();
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
        self.ensure_recipient_tasks(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        )?;
        let state = self.tasks.lock_notification();
        let mut items: Vec<_> = notification_keys_for_recipient(
            &state,
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        )
        .into_iter()
        .filter_map(|task_key| state.tasks.get(task_key.as_str()).cloned())
        .collect();
        items.sort_by(|left, right| {
            notification_sort_key(right)
                .cmp(&notification_sort_key(left))
                .then_with(|| right.notification_id.cmp(&left.notification_id))
        });
        Ok(items)
    }

    pub fn get_notification(
        &self,
        auth: &AppContext,
        notification_id: &str,
    ) -> Result<NotificationTask, NotificationError> {
        self.ensure_notification_task(auth.tenant_id.as_str(), notification_id)?;
        self.tasks
            .lock_notification()
            .tasks
            .get(notification_scope_key(auth.tenant_id.as_str(), notification_id).as_str())
            .filter(|task| notification_visible_to_actor(task, auth))
            .cloned()
            .ok_or_else(|| NotificationError::not_found(notification_id))
    }

    fn task_record(&self, task: &NotificationTask) -> NotificationTaskRecord {
        NotificationTaskRecord {
            tenant_id: task.tenant_id.clone(),
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
            organization_id: "0".into(),
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
    tasks_by_recipient: HashMap<String, BTreeSet<String>>,
}

impl NotificationTaskStore for RuntimeMemoryNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        Ok(self
            .state
            .lock_notification()
            .tasks
            .get(notification_scope_key(tenant_id, notification_id).as_str())
            .cloned())
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let notification_key =
            notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str());
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

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let state = self.state.lock_notification();
        let task_keys = state
            .tasks_by_recipient
            .get(notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id).as_str())
            .cloned()
            .unwrap_or_default();
        Ok(task_keys
            .into_iter()
            .filter_map(|task_key| state.tasks.get(task_key.as_str()).cloned())
            .collect())
    }
}

fn runtime_notification_recipient_scope_key(task: &NotificationTask) -> String {
    notification_recipient_scope_key(
        task.tenant_id.as_str(),
        task.recipient_kind.as_str(),
        task.recipient_id.as_str(),
    )
}

fn insert_runtime_notification_task(
    state: &mut NotificationRuntimeTaskState,
    notification_key: String,
    task: NotificationTask,
) {
    if let Some(previous) = state.tasks.get(notification_key.as_str()).cloned() {
        remove_runtime_notification_recipient_index(
            &mut state.tasks_by_recipient,
            notification_key.as_str(),
            &previous,
        );
    }
    insert_runtime_notification_recipient_index(
        &mut state.tasks_by_recipient,
        notification_key.as_str(),
        &task,
    );
    state.tasks.insert(notification_key, task);
}

fn remove_runtime_notification_task(
    state: &mut NotificationRuntimeTaskState,
    notification_key: &str,
) -> Option<NotificationTask> {
    let removed = state.tasks.remove(notification_key)?;
    remove_runtime_notification_recipient_index(
        &mut state.tasks_by_recipient,
        notification_key,
        &removed,
    );
    Some(removed)
}

fn insert_runtime_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    task: &NotificationTask,
) {
    index
        .entry(runtime_notification_recipient_scope_key(task))
        .or_default()
        .insert(notification_key.to_owned());
}

fn remove_runtime_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    task: &NotificationTask,
) {
    let recipient_key = runtime_notification_recipient_scope_key(task);
    if let Some(task_keys) = index.get_mut(recipient_key.as_str()) {
        task_keys.remove(notification_key);
        if task_keys.is_empty() {
            index.remove(recipient_key.as_str());
        }
    }
}

fn notification_keys_for_recipient(
    state: &NotificationRuntimeTaskState,
    tenant_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> BTreeSet<String> {
    state
        .tasks_by_recipient
        .get(notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id).as_str())
        .cloned()
        .unwrap_or_default()
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
