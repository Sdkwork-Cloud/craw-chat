//! Relays durable conversation outbox events to the embedded session-gateway realtime plane.

use std::sync::Arc;
use std::time::Duration;

use im_adapters_postgres_journal::{
    PostgresAggregateStore, PostgresJournalConfig, PostgresOutboxStore,
};
use im_platform_contracts::{
    CONVERSATION_AGGREGATE_PAGE_SIZE_MAX, ContractError, ConversationAggregateStore,
    ConversationMemberPageCursor, OutboxEventClaim, OutboxEventRecord, OutboxStore,
    RealtimeEventPublisher, RealtimeEventRecipient, RealtimeScopeEventPublishCommand,
};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use session_gateway::RealtimeDeliveryRuntime;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::outbox_relay_common::{DEFAULT_OUTBOX_CLAIM_LEASE, log_unexpected_aggregate_type};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const CONVERSATION_OUTBOX_RELAY_POLL_MS_ENV: &str = "SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_POLL_MS";
const CONVERSATION_OUTBOX_RELAY_TENANT_ID_ENV: &str =
    "SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_TENANT_ID";
const CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID_ENV: &str =
    "SDKWORK_IM_CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID";
const CONVERSATION_OUTBOX_AGGREGATE_TYPE: &str = "conversation";
const DEFAULT_CONVERSATION_OUTBOX_RELAY_POLL_MS: u64 = 50;
const DEFAULT_CONVERSATION_OUTBOX_RELAY_TENANT_ID: &str = "100001";
const DEFAULT_CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID: &str = "default";
const DEFAULT_CONVERSATION_OUTBOX_RELAY_BATCH_SIZE: usize = 64;
const DEFAULT_CONVERSATION_OUTBOX_RELAY_SCOPE_LIMIT: usize = 32;

pub struct ConversationOutboxRelayHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
}

struct ConversationOutboxRelayDependencies {
    outbox: Arc<dyn OutboxStore>,
    aggregate_store: Arc<dyn ConversationAggregateStore>,
}

impl ConversationOutboxRelayHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

pub fn spawn_conversation_outbox_relay_from_env(
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Option<ConversationOutboxRelayHandle> {
    let dependencies = resolve_conversation_outbox_dependencies_from_env()?;
    let poll_interval = resolve_conversation_outbox_relay_poll_interval();
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let task = tokio::spawn(async move {
        run_conversation_outbox_relay(
            dependencies.outbox,
            dependencies.aggregate_store,
            realtime_runtime,
            poll_interval,
            shutdown_rx,
        )
        .await;
    });
    info!("conversation outbox relay started");
    Some(ConversationOutboxRelayHandle {
        shutdown: shutdown_tx,
        task,
    })
}

fn resolve_conversation_outbox_dependencies_from_env() -> Option<ConversationOutboxRelayDependencies>
{
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            return PostgresJournalConfig::from_database_config(&config)
                .connect_pool()
                .ok()
                .map(conversation_outbox_dependencies_from_pool);
        }
    }

    let database_url = std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())?;
    PostgresJournalConfig::new(database_url)
        .connect_pool()
        .ok()
        .map(conversation_outbox_dependencies_from_pool)
}

fn conversation_outbox_dependencies_from_pool(
    pool: im_adapters_postgres_journal::PostgresJournalPool,
) -> ConversationOutboxRelayDependencies {
    ConversationOutboxRelayDependencies {
        outbox: Arc::new(PostgresOutboxStore::from_pool(pool.clone())),
        aggregate_store: Arc::new(PostgresAggregateStore::from_pool(pool)),
    }
}

fn resolve_conversation_outbox_relay_poll_interval() -> Duration {
    let millis = std::env::var(CONVERSATION_OUTBOX_RELAY_POLL_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_CONVERSATION_OUTBOX_RELAY_POLL_MS);
    Duration::from_millis(millis)
}

fn resolve_conversation_outbox_relay_tenant_id() -> String {
    std::env::var(CONVERSATION_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_CONVERSATION_OUTBOX_RELAY_TENANT_ID.to_owned())
}

fn resolve_conversation_outbox_relay_organization_id() -> String {
    std::env::var(CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_CONVERSATION_OUTBOX_RELAY_ORGANIZATION_ID.to_owned())
}

fn resolve_conversation_outbox_relay_scopes(
    outbox: &Arc<dyn OutboxStore>,
) -> Vec<(String, String)> {
    if std::env::var(CONVERSATION_OUTBOX_RELAY_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .is_some()
    {
        return vec![(
            resolve_conversation_outbox_relay_tenant_id(),
            resolve_conversation_outbox_relay_organization_id(),
        )];
    }

    match outbox.list_pending_scopes(
        CONVERSATION_OUTBOX_AGGREGATE_TYPE,
        DEFAULT_CONVERSATION_OUTBOX_RELAY_SCOPE_LIMIT,
    ) {
        Ok(scopes) => scopes,
        Err(error) => {
            warn!(error = ?error, "conversation outbox relay scope discovery failed");
            Vec::new()
        }
    }
}

async fn run_conversation_outbox_relay(
    outbox: Arc<dyn OutboxStore>,
    aggregate_store: Arc<dyn ConversationAggregateStore>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    poll_interval: Duration,
    mut shutdown: watch::Receiver<()>,
) {
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }

        for (tenant_id, organization_id) in resolve_conversation_outbox_relay_scopes(&outbox) {
            match outbox.claim_pending(
                tenant_id.as_str(),
                organization_id.as_str(),
                CONVERSATION_OUTBOX_AGGREGATE_TYPE,
                DEFAULT_CONVERSATION_OUTBOX_RELAY_BATCH_SIZE,
                DEFAULT_OUTBOX_CLAIM_LEASE,
            ) {
                Ok(claims) => {
                    for claim in claims {
                        let event = &claim.event;
                        if event.aggregate_type != CONVERSATION_OUTBOX_AGGREGATE_TYPE {
                            log_unexpected_aggregate_type(
                                &event,
                                CONVERSATION_OUTBOX_AGGREGATE_TYPE,
                                "conversation",
                            );
                            continue;
                        }
                        relay_conversation_outbox_event(
                            realtime_runtime.as_ref(),
                            &outbox,
                            aggregate_store.as_ref(),
                            &claim,
                        );
                    }
                }
                Err(error) => {
                    warn!(
                        tenant_id = tenant_id.as_str(),
                        organization_id = organization_id.as_str(),
                        error = ?error,
                        "conversation outbox relay drain failed"
                    );
                }
            }
        }

        tokio::select! {
            _ = shutdown.changed() => break,
            _ = tokio::time::sleep(poll_interval) => {}
        }
    }
}

fn relay_conversation_outbox_event(
    realtime_publisher: &dyn RealtimeEventPublisher,
    outbox: &Arc<dyn OutboxStore>,
    aggregate_store: &dyn ConversationAggregateStore,
    claim: &OutboxEventClaim,
) {
    let event = &claim.event;
    let payload = build_realtime_payload(event);
    let publish_result = publish_conversation_event_to_member_pages(
        realtime_publisher,
        aggregate_store,
        event,
        payload,
    );

    if let Err(error) = publish_result {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            event_type = event.event_type.as_str(),
            error = ?error,
            "conversation outbox relay publish failed"
        );
        let _ = outbox.mark_failed(claim, "conversation outbox relay publish failed");
        return;
    }

    if let Err(error) = outbox.mark_published(claim) {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            error = ?error,
            "conversation outbox relay mark_published failed"
        );
    }
}

fn build_realtime_payload(event: &OutboxEventRecord) -> String {
    serde_json::json!({
        "eventId": event.event_id,
        "eventType": event.event_type,
        "aggregateType": event.aggregate_type,
        "aggregateId": event.aggregate_id,
        "tenantId": event.tenant_id,
        "organizationId": event.organization_id,
        "payload": serde_json::from_str::<serde_json::Value>(event.payload_json.as_str())
            .unwrap_or_else(|_| serde_json::json!(event.payload_json)),
    })
    .to_string()
}

fn publish_conversation_event_to_member_pages(
    realtime_publisher: &dyn RealtimeEventPublisher,
    aggregate_store: &dyn ConversationAggregateStore,
    event: &OutboxEventRecord,
    payload: String,
) -> Result<usize, ContractError> {
    let mut cursor: Option<ConversationMemberPageCursor> = None;
    let mut delivered = 0usize;
    loop {
        let page = aggregate_store.load_event_recipients_page(
            event.tenant_id.as_str(),
            event.organization_id.as_str(),
            event.aggregate_id.as_str(),
            event.created_at.as_str(),
            cursor.as_ref(),
            CONVERSATION_AGGREGATE_PAGE_SIZE_MAX,
        )?;
        let recipients = page
            .items
            .into_iter()
            .map(|member| RealtimeEventRecipient::new(member.principal_id, member.principal_kind))
            .collect::<Vec<_>>();
        if !recipients.is_empty() {
            delivered = delivered.saturating_add(
                realtime_publisher.publish_durable_scope_event_to_recipients(
                    RealtimeScopeEventPublishCommand {
                        tenant_id: event.tenant_id.as_str(),
                        organization_id: event.organization_id.as_str(),
                        scope_type: "conversation",
                        scope_id: event.aggregate_id.as_str(),
                        event_type: event.event_type.as_str(),
                        payload: payload.clone(),
                        recipients: recipients.clone(),
                    },
                )?,
            );
            if event.event_type == "conversation.updated" {
                delivered = delivered.saturating_add(
                    realtime_publisher.publish_durable_user_scope_event_to_recipients(
                        event.tenant_id.as_str(),
                        event.organization_id.as_str(),
                        event.event_type.as_str(),
                        payload.clone(),
                        recipients,
                    )?,
                );
            }
        }
        if !page.has_more {
            break;
        }
        cursor = Some(page.next_cursor.ok_or_else(|| {
            ContractError::Invalid(
                "conversation event recipient page returned has_more without next_cursor".into(),
            )
        })?);
    }
    Ok(delivered)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use im_platform_contracts::{
        ConversationAggregateStore, ConversationMemberPage, ConversationMemberPageCursor,
        ConversationMemberRecord, ReadCursorPage, ReadCursorPageCursor, ReadCursorRecord,
        RealtimeEventPublisher,
    };

    use super::*;
    use im_platform_contracts::AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE;

    #[test]
    fn conversation_realtime_relay_does_not_claim_agent_dispatch_outbox_rows() {
        assert_ne!(
            CONVERSATION_OUTBOX_AGGREGATE_TYPE,
            AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE
        );
    }

    struct PagedConversationMembers {
        members: Vec<ConversationMemberRecord>,
    }

    impl ConversationAggregateStore for PagedConversationMembers {
        fn load_members_page(
            &self,
            tenant_id: &str,
            organization_id: &str,
            conversation_id: &str,
            cursor: Option<&ConversationMemberPageCursor>,
            page_size: usize,
        ) -> Result<ConversationMemberPage, im_platform_contracts::ContractError> {
            let mut members = self
                .members
                .iter()
                .filter(|member| {
                    member.tenant_id == tenant_id
                        && member.organization_id == organization_id
                        && member.conversation_id == conversation_id
                        && cursor.is_none_or(|cursor| {
                            (member.principal_kind.as_str(), member.principal_id.as_str())
                                > (cursor.principal_kind.as_str(), cursor.principal_id.as_str())
                        })
                })
                .cloned()
                .collect::<Vec<_>>();
            members.sort_by(|left, right| {
                (&left.principal_kind, &left.principal_id)
                    .cmp(&(&right.principal_kind, &right.principal_id))
            });
            let has_more = members.len() > page_size;
            members.truncate(page_size);
            let next_cursor = has_more.then(|| {
                let last = members
                    .last()
                    .expect("paged member result should not be empty");
                ConversationMemberPageCursor {
                    principal_kind: last.principal_kind.clone(),
                    principal_id: last.principal_id.clone(),
                }
            });
            Ok(ConversationMemberPage {
                items: members,
                next_cursor,
                has_more,
            })
        }

        fn load_member(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
        ) -> Result<Option<ConversationMemberRecord>, im_platform_contracts::ContractError>
        {
            Ok(None)
        }

        fn load_member_by_id(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _member_id: i64,
        ) -> Result<Option<ConversationMemberRecord>, im_platform_contracts::ContractError>
        {
            Ok(None)
        }

        fn load_event_recipients_page(
            &self,
            tenant_id: &str,
            organization_id: &str,
            conversation_id: &str,
            _joined_before_or_at: &str,
            cursor: Option<&ConversationMemberPageCursor>,
            page_size: usize,
        ) -> Result<ConversationMemberPage, im_platform_contracts::ContractError> {
            self.load_members_page(
                tenant_id,
                organization_id,
                conversation_id,
                cursor,
                page_size,
            )
        }

        fn upsert_member(
            &self,
            _member: ConversationMemberRecord,
        ) -> Result<(), im_platform_contracts::ContractError> {
            Ok(())
        }

        fn remove_member(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
            _removed_at: &str,
        ) -> Result<(), im_platform_contracts::ContractError> {
            Ok(())
        }

        fn load_read_cursors_page(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _cursor: Option<&ReadCursorPageCursor>,
            _page_size: usize,
        ) -> Result<ReadCursorPage, im_platform_contracts::ContractError> {
            Ok(ReadCursorPage {
                items: Vec::new(),
                next_cursor: None,
                has_more: false,
            })
        }

        fn load_read_cursor(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _member_id: i64,
        ) -> Result<Option<ReadCursorRecord>, im_platform_contracts::ContractError> {
            Ok(None)
        }

        fn upsert_read_cursor(
            &self,
            _cursor: ReadCursorRecord,
        ) -> Result<(), im_platform_contracts::ContractError> {
            Ok(())
        }

        fn load_high_watermark(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<u64, im_platform_contracts::ContractError> {
            Ok(0)
        }

        fn allocate_member_id(&self) -> Result<i64, im_platform_contracts::ContractError> {
            Ok(1)
        }

        fn conversation_exists(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<bool, im_platform_contracts::ContractError> {
            Ok(true)
        }
    }

    #[derive(Default)]
    struct RecordingPublisher {
        durable_batch_sizes: Mutex<Vec<usize>>,
        durable_scopes: Mutex<Vec<(String, String)>>,
    }

    impl RealtimeEventPublisher for RecordingPublisher {
        fn publish_ephemeral_scope_event_to_recipients(
            &self,
            _command: RealtimeScopeEventPublishCommand<'_>,
        ) -> Result<usize, im_platform_contracts::ContractError> {
            Ok(0)
        }

        fn publish_durable_scope_event_to_recipients(
            &self,
            command: RealtimeScopeEventPublishCommand<'_>,
        ) -> Result<usize, im_platform_contracts::ContractError> {
            let batch_size = command.recipients.len();
            self.durable_batch_sizes
                .lock()
                .expect("durable batch sizes should lock")
                .push(batch_size);
            self.durable_scopes
                .lock()
                .expect("durable scopes should lock")
                .push((command.scope_type.into(), command.scope_id.into()));
            Ok(batch_size)
        }
    }

    #[test]
    fn conversation_scope_outbox_relay_pages_recipients_from_durable_membership() {
        let members = (0..401_i64)
            .map(|index| ConversationMemberRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_relay_paged_members".into(),
                principal_kind: "user".into(),
                principal_id: format!("user_{index:03}"),
                member_id: 10_000 + index,
                membership_role: "member".into(),
                membership_state: "joined".into(),
                invited_by: None,
                joined_at: "2026-07-10T00:00:00.000Z".into(),
                removed_at: None,
                attributes_json: "{}".into(),
            })
            .collect();
        let member_store = PagedConversationMembers { members };
        let publisher = RecordingPublisher::default();
        let event = OutboxEventRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            outbox_id: "1".into(),
            aggregate_type: "conversation".into(),
            aggregate_id: "c_relay_paged_members".into(),
            event_id: "conversation:message.posted:1".into(),
            event_type: "message.posted".into(),
            payload_json: serde_json::json!({
                "conversationId": "c_relay_paged_members",
                "messageId": "42",
            })
            .to_string(),
            payload_hash: "hash".into(),
            publish_status: im_platform_contracts::OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: "2026-07-10T00:00:00.000Z".into(),
            published_at: None,
            created_at: "2026-07-10T00:00:00.000Z".into(),
            updated_at: "2026-07-10T00:00:00.000Z".into(),
        };

        let delivered = publish_conversation_event_to_member_pages(
            &publisher,
            &member_store,
            &event,
            build_realtime_payload(&event),
        )
        .expect("conversation relay should publish every durable member page");

        assert_eq!(delivered, 401);
        assert_eq!(
            *publisher
                .durable_batch_sizes
                .lock()
                .expect("durable batch sizes should lock"),
            vec![200, 200, 1]
        );
    }

    #[test]
    fn conversation_updated_outbox_relay_reaches_member_inbox_scopes() {
        let members = vec![
            ConversationMemberRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_profile_updated".into(),
                principal_kind: "user".into(),
                principal_id: "user.owner".into(),
                member_id: 1,
                membership_role: "owner".into(),
                membership_state: "joined".into(),
                invited_by: None,
                joined_at: "2026-07-10T00:00:00.000Z".into(),
                removed_at: None,
                attributes_json: "{}".into(),
            },
            ConversationMemberRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_profile_updated".into(),
                principal_kind: "user".into(),
                principal_id: "user.member".into(),
                member_id: 2,
                membership_role: "member".into(),
                membership_state: "joined".into(),
                invited_by: Some("user.owner".into()),
                joined_at: "2026-07-10T00:00:00.000Z".into(),
                removed_at: None,
                attributes_json: "{}".into(),
            },
        ];
        let member_store = PagedConversationMembers { members };
        let publisher = RecordingPublisher::default();
        let event = OutboxEventRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            outbox_id: "profile-1".into(),
            aggregate_type: "conversation".into(),
            aggregate_id: "c_profile_updated".into(),
            event_id: "conversation:conversation.updated:profile-1".into(),
            event_type: "conversation.updated".into(),
            payload_json: serde_json::json!({
                "conversationId": "c_profile_updated",
                "displayName": "Renamed group",
            })
            .to_string(),
            payload_hash: "hash".into(),
            publish_status: im_platform_contracts::OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: "2026-07-10T00:00:00.000Z".into(),
            published_at: None,
            created_at: "2026-07-10T00:00:00.000Z".into(),
            updated_at: "2026-07-10T00:00:00.000Z".into(),
        };

        let delivered = publish_conversation_event_to_member_pages(
            &publisher,
            &member_store,
            &event,
            build_realtime_payload(&event),
        )
        .expect("profile event should publish to conversation and inbox scopes");

        assert_eq!(delivered, 4);
        let mut scopes = publisher
            .durable_scopes
            .lock()
            .expect("durable scopes should lock")
            .clone();
        scopes.sort_unstable();
        assert_eq!(
            scopes,
            vec![
                ("conversation".into(), "c_profile_updated".into()),
                ("user".into(), "user.member".into()),
                ("user".into(), "user.owner".into()),
            ]
        );
    }
}
