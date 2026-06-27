use std::collections::BTreeSet;
use std::sync::{Mutex, MutexGuard};

use im_domain_core::conversation::ClientRouteSyncFeedEntry;
use im_domain_core::conversation::{ConversationMember, ConversationReadCursor};
use im_platform_contracts::{
    MetadataSnapshotRecord, MetadataStore, TimelineProjectionBatch, TimelineProjectionRecord,
    TimelineProjectionStore,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::contacts::{contact_map_from_items, contact_runtime_scope, contact_snapshot_items};
use crate::interactions::{
    StoredMessageInteractionSummary, interaction_map_from_items, interaction_snapshot_items,
};
use crate::observability::ProjectionSnapshotOperation;
use crate::projection::ProjectionError;
use crate::scope::{
    ClientRouteFeedScopeKey, ClientRoutePrincipalScopeKey, client_route_feed_scope_key,
    client_route_principal_scope_key, encode_projection_key_segments,
};
use crate::{
    ContactView, ConversationSummaryView, TimelineProjectionService, TimelineViewEntry,
    model::ConversationCatalogEntry,
};

const CONVERSATION_SUMMARY_KEY: &str = "conversation-summary";
pub(crate) const CONVERSATION_SUMMARY_SNAPSHOT_KEY: &str = CONVERSATION_SUMMARY_KEY;
const CONVERSATION_CATALOG_KEY: &str = "conversation-catalog";
const CONVERSATION_MEMBERS_KEY: &str = "conversation-members";
const CONVERSATION_READ_CURSORS_KEY: &str = "conversation-read_cursors";
const MESSAGE_INTERACTIONS_KEY: &str = "message-interactions";
const CONTACTS_KEY: &str = "contacts";
const CONTACT_OWNERS_KEY: &str = "contact-owners";
const CONTACT_DIRECT_CHAT_BINDINGS_KEY: &str = "contact-direct-chat-bindings";
const REGISTERED_CLIENT_ROUTES_KEY: &str = "registered-client-routes";
const REGISTERED_CLIENT_ROUTE_PRINCIPALS_KEY: &str = "registered-client-route-principals";
const CLIENT_ROUTE_SYNC_SEQUENCE_KEY: &str = "client-route-sync-sequence";
const CLIENT_ROUTE_SYNC_SCOPE_CATALOG_KEY: &str = "client-route-sync-scopes";
const CONTACT_CATALOG_SCOPE: &str = "projection-contacts";
const PRINCIPAL_SNAPSHOT_SCOPE_PREFIX: &str = "principal";
const CLIENT_ROUTE_SYNC_SNAPSHOT_SCOPE_PREFIX: &str = "client-route-sync";
const CLIENT_ROUTE_SYNC_CATALOG_SCOPE: &str = "projection-client-route-sync";

#[derive(Default)]
struct ProjectionSnapshotWritePlan {
    metadata_snapshots: Vec<MetadataSnapshotRecord>,
    timeline_batches: Vec<TimelineProjectionBatch>,
}

impl ProjectionSnapshotWritePlan {
    fn push_metadata<T: Serialize>(
        &mut self,
        scope: &str,
        key: &str,
        value: &T,
    ) -> Result<(), ProjectionError> {
        self.metadata_snapshots.push(MetadataSnapshotRecord {
            scope: scope.to_owned(),
            key: key.to_owned(),
            value: serde_json::to_string(value).map_err(ProjectionError::InvalidSnapshot)?,
        });
        Ok(())
    }

    fn push_timeline_batch<T>(
        &mut self,
        tenant_id: &str,
        timeline_scope: &str,
        entries: impl IntoIterator<Item = T>,
    ) -> Result<(), ProjectionError>
    where
        T: Serialize + TimelineSequenceValue,
    {
        let records = entries
            .into_iter()
            .map(|entry| {
                Ok(TimelineProjectionRecord {
                    message_seq: entry.timeline_sequence(),
                    payload: serde_json::to_string(&entry)
                        .map_err(ProjectionError::InvalidSnapshot)?,
                })
            })
            .collect::<Result<Vec<_>, ProjectionError>>()?;
        if !records.is_empty() {
            self.timeline_batches.push(TimelineProjectionBatch {
                tenant_id: tenant_id.to_owned(),
                timeline_scope: timeline_scope.to_owned(),
                records,
            });
        }
        Ok(())
    }

    fn commit(
        self,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<(), ProjectionError> {
        if !self.metadata_snapshots.is_empty() {
            metadata_store
                .put_snapshots(&self.metadata_snapshots)
                .map_err(ProjectionError::StoreFailure)?;
        }
        if !self.timeline_batches.is_empty() {
            timeline_store
                .upsert_timeline_batches(&self.timeline_batches)
                .map_err(ProjectionError::StoreFailure)?;
        }
        Ok(())
    }

    fn commit_metadata_only(
        self,
        metadata_store: &dyn MetadataStore,
    ) -> Result<(), ProjectionError> {
        if !self.metadata_snapshots.is_empty() {
            metadata_store
                .put_snapshots(&self.metadata_snapshots)
                .map_err(ProjectionError::StoreFailure)?;
        }
        Ok(())
    }
}

trait TimelineSequenceValue {
    fn timeline_sequence(&self) -> u64;
}

impl TimelineSequenceValue for TimelineViewEntry {
    fn timeline_sequence(&self) -> u64 {
        self.message_seq
    }
}

impl TimelineSequenceValue for ClientRouteSyncFeedEntry {
    fn timeline_sequence(&self) -> u64 {
        self.sync_seq
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
struct PrincipalSnapshotCatalogEntry {
    tenant_id: String,
    organization_id: String,
    principal_kind: String,
    principal_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
struct ClientRouteSyncScopeCatalogEntry {
    tenant_id: String,
    organization_id: String,
    principal_kind: String,
    principal_id: String,
    device_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
struct ContactOwnerCatalogEntry {
    tenant_id: String,
    #[serde(default = "default_contact_catalog_organization_id")]
    organization_id: String,
    owner_user_id: String,
}

fn default_contact_catalog_organization_id() -> String {
    "default".to_owned()
}

trait ProjectionMutexExt<T> {
    fn lock_projection(&self, lock_name: &'static str) -> MutexGuard<'_, T>;
}

impl<T> ProjectionMutexExt<T> for Mutex<T> {
    fn lock_projection(&self, lock_name: &'static str) -> MutexGuard<'_, T> {
        super::lock_projection_mutex(self, lock_name)
    }
}

impl TimelineProjectionService {
    pub fn persist_conversation_snapshot(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let scope = conversation_snapshot_scope(tenant_id, organization_id, conversation_id);
        let mut persisted_client_route_sync_snapshot = false;
        let result = (|| {
            let mut write_plan = ProjectionSnapshotWritePlan::default();
            if !self.collect_conversation_snapshot_writes(
                tenant_id,
                organization_id,
                conversation_id,
                &mut write_plan,
            )? {
                return Ok(false);
            }
            self.collect_contact_snapshot_writes(&mut write_plan)?;
            match self.collect_client_route_sync_snapshot_writes(&mut write_plan) {
                Ok(persisted) => persisted_client_route_sync_snapshot = persisted,
                Err(error) => {
                    self.record_projection_snapshot_failure(
                        ProjectionSnapshotOperation::ClientRouteSyncSnapshotPersist,
                        "client-route-sync",
                        CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                        &error,
                    );
                    return Err(error);
                }
            }
            write_plan.commit(metadata_store, timeline_store)?;
            Ok(true)
        })();

        match &result {
            Ok(true) => {
                if persisted_client_route_sync_snapshot {
                    self.record_projection_snapshot_success(
                        ProjectionSnapshotOperation::ClientRouteSyncSnapshotPersist,
                        "client-route-sync",
                        CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                        "persisted client route sync projection snapshot catalog".to_owned(),
                    );
                }
                self.record_projection_snapshot_success(
                    ProjectionSnapshotOperation::ConversationSnapshotPersist,
                    "conversation",
                    scope.as_str(),
                    format!("persisted conversation projection snapshot for {scope}"),
                );
            }
            Ok(false) => {}
            Err(error) => {
                if persisted_client_route_sync_snapshot {
                    self.record_projection_snapshot_failure(
                        ProjectionSnapshotOperation::ClientRouteSyncSnapshotPersist,
                        "client-route-sync",
                        CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                        error,
                    );
                }
                self.record_projection_snapshot_failure(
                    ProjectionSnapshotOperation::ConversationSnapshotPersist,
                    "conversation",
                    scope.as_str(),
                    error,
                );
            }
        }

        result
    }

    pub fn restore_conversation_snapshot(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let scope = conversation_snapshot_scope(tenant_id, organization_id, conversation_id);
        let result = (|| {
            let Some(summary) = load_metadata_snapshot::<ConversationSummaryView>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_SUMMARY_KEY,
            )?
            else {
                return Ok(false);
            };
            let timeline = timeline_store
                .load_timeline(tenant_id, conversation_id)
                .map_err(ProjectionError::StoreFailure)?
                .into_iter()
                .map(|(_, payload)| {
                    serde_json::from_str::<TimelineViewEntry>(&payload)
                        .map_err(ProjectionError::InvalidSnapshot)
                })
                .map(|entry| entry.map(|entry| (entry.message_seq, entry)))
                .collect::<Result<std::collections::BTreeMap<_, _>, _>>()?;
            let conversation = load_metadata_snapshot::<ConversationCatalogEntry>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_CATALOG_KEY,
            )?;
            let members = load_metadata_snapshot::<Vec<ConversationMember>>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_MEMBERS_KEY,
            )?
            .unwrap_or_default();
            let read_cursors = load_metadata_snapshot::<Vec<ConversationReadCursor>>(
                metadata_store,
                scope.as_str(),
                CONVERSATION_READ_CURSORS_KEY,
            )?
            .unwrap_or_default()
            .into_iter()
            .map(|cursor| (cursor.member_id.clone(), cursor))
            .collect();
            let interactions = load_metadata_snapshot::<Vec<StoredMessageInteractionSummary>>(
                metadata_store,
                scope.as_str(),
                MESSAGE_INTERACTIONS_KEY,
            )?
            .map(interaction_map_from_items)
            .unwrap_or_default();

            self.summaries
                .lock_projection("summary store")
                .insert(scope.clone(), summary);
            self.received_messages
                .lock_projection("received message index")
                .rebuild_conversation(scope.as_str(), &timeline);
            self.entries
                .lock_projection("projection store")
                .insert(scope.clone(), timeline);
            match conversation {
                Some(conversation) => {
                    self.conversations
                        .lock_projection("conversation store")
                        .insert(scope.clone(), conversation);
                }
                None => {
                    self.conversations
                        .lock_projection("conversation store")
                        .remove(scope.as_str());
                }
            }
            {
                let mut member_store = self.members.lock_projection("member store");
                member_store.remove_conversation(scope.as_str());
                for member in members {
                    member_store.insert_member(scope.clone(), member);
                }
            }
            self.read_cursors
                .lock_projection("cursor store")
                .insert(scope.clone(), read_cursors);
            self.message_interactions
                .lock_projection("message interaction store")
                .insert(scope.clone(), interactions);
            self.restore_contact_snapshot(metadata_store)?;
            self.restore_client_route_sync_snapshot(metadata_store, timeline_store)?;

            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::ConversationSnapshotRestore,
                "conversation",
                scope.as_str(),
                format!("restored conversation projection snapshot for {scope}"),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::ConversationSnapshotRestore,
                "conversation",
                scope.as_str(),
                error,
            ),
        }

        result
    }

    pub fn persist_client_route_sync_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let result = (|| {
            let mut write_plan = ProjectionSnapshotWritePlan::default();
            if !self.collect_client_route_sync_snapshot_writes(&mut write_plan)? {
                return Ok(false);
            }
            write_plan.commit(metadata_store, timeline_store)?;
            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::ClientRouteSyncSnapshotPersist,
                "client-route-sync",
                CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                "persisted client route sync projection snapshot catalog".to_owned(),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::ClientRouteSyncSnapshotPersist,
                "client-route-sync",
                CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                error,
            ),
        }

        result
    }

    pub fn persist_contact_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
    ) -> Result<bool, ProjectionError> {
        (|| {
            let mut write_plan = ProjectionSnapshotWritePlan::default();
            if !self.collect_contact_snapshot_writes(&mut write_plan)? {
                return Ok(false);
            }
            write_plan.commit_metadata_only(metadata_store)?;
            Ok(true)
        })()
    }

    fn collect_conversation_snapshot_writes(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        write_plan: &mut ProjectionSnapshotWritePlan,
    ) -> Result<bool, ProjectionError> {
        let scope = conversation_snapshot_scope(tenant_id, organization_id, conversation_id);
        let Some(summary) = self.conversation_summary(tenant_id, organization_id, conversation_id) else {
            return Ok(false);
        };
        let conversation = self
            .conversations
            .lock_projection("conversation store")
            .get(scope.as_str())
            .cloned();
        let mut members = self
            .members
            .lock_projection("member store")
            .get(scope.as_str())
            .map(|scope_members| scope_members.values().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        members.sort_by(|left, right| {
            left.principal_id
                .cmp(&right.principal_id)
                .then_with(|| left.principal_kind.cmp(&right.principal_kind))
        });
        let mut read_cursors = self
            .read_cursors
            .lock_projection("cursor store")
            .get(scope.as_str())
            .map(|scope_cursors| scope_cursors.values().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        read_cursors.sort_by(|left, right| left.member_id.cmp(&right.member_id));
        let interaction_items = self
            .message_interactions
            .lock_projection("message interaction store")
            .get(scope.as_str())
            .map(interaction_snapshot_items)
            .unwrap_or_default();

        write_plan.push_metadata(scope.as_str(), CONVERSATION_SUMMARY_KEY, &summary)?;
        if let Some(conversation) = conversation.as_ref() {
            write_plan.push_metadata(scope.as_str(), CONVERSATION_CATALOG_KEY, conversation)?;
        }
        write_plan.push_metadata(scope.as_str(), CONVERSATION_MEMBERS_KEY, &members)?;
        write_plan.push_metadata(scope.as_str(), CONVERSATION_READ_CURSORS_KEY, &read_cursors)?;
        write_plan.push_metadata(scope.as_str(), MESSAGE_INTERACTIONS_KEY, &interaction_items)?;
        write_plan.push_timeline_batch(
            tenant_id,
            conversation_id,
            self.timeline(tenant_id, organization_id, conversation_id),
        )?;
        Ok(true)
    }

    fn collect_client_route_sync_snapshot_writes(
        &self,
        write_plan: &mut ProjectionSnapshotWritePlan,
    ) -> Result<bool, ProjectionError> {
        let registered_client_route_snapshots = self
            .registered_client_routes
            .lock_projection("registered client route store")
            .iter()
            .map(|(scope, devices)| {
                let mut items = devices.values().cloned().collect::<Vec<_>>();
                items.sort_by(|left, right| left.device_id.cmp(&right.device_id));
                (scope.clone(), items)
            })
            .collect::<Vec<_>>();
        let client_route_sync_feeds = self
            .client_route_sync_feeds
            .lock_projection("client route sync feed store")
            .clone();
        let client_route_sync_sequences = self
            .client_route_sync_sequences
            .lock_projection("client route sync sequence store")
            .clone();
        if registered_client_route_snapshots.is_empty()
            && client_route_sync_feeds.is_empty()
            && client_route_sync_sequences.is_empty()
        {
            return Ok(false);
        }

        let mut persisted_client_route_scopes = BTreeSet::new();
        let mut principal_catalog = BTreeSet::new();
        for (runtime_scope, devices) in &registered_client_route_snapshots {
            principal_catalog.insert(PrincipalSnapshotCatalogEntry {
                tenant_id: runtime_scope.tenant_id.clone(),
                organization_id: runtime_scope.organization_id.clone(),
                principal_kind: runtime_scope.principal_kind.clone(),
                principal_id: runtime_scope.principal_id.clone(),
            });
            let snapshot_scope = client_route_principal_snapshot_scope(runtime_scope);
            write_plan.push_metadata(
                snapshot_scope.as_str(),
                REGISTERED_CLIENT_ROUTES_KEY,
                devices,
            )?;

            for device in devices {
                persisted_client_route_scopes.insert(client_route_feed_scope_key(
                    runtime_scope.tenant_id.as_str(),
                    runtime_scope.organization_id.as_str(),
                    runtime_scope.principal_kind.as_str(),
                    runtime_scope.principal_id.as_str(),
                    device.device_id.as_str(),
                ));
            }
        }
        for runtime_scope in client_route_sync_feeds.keys() {
            persisted_client_route_scopes.insert(runtime_scope.clone());
        }
        for runtime_scope in client_route_sync_sequences.keys() {
            persisted_client_route_scopes.insert(runtime_scope.clone());
        }

        let principal_catalog = principal_catalog.into_iter().collect::<Vec<_>>();
        let client_route_scope_catalog = persisted_client_route_scopes
            .iter()
            .map(|scope| ClientRouteSyncScopeCatalogEntry {
                tenant_id: scope.tenant_id.clone(),
                organization_id: scope.organization_id.clone(),
                principal_kind: scope.principal_kind.clone(),
                principal_id: scope.principal_id.clone(),
                device_id: scope.device_id.clone(),
            })
            .collect::<Vec<_>>();
        write_plan.push_metadata(
            CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
            REGISTERED_CLIENT_ROUTE_PRINCIPALS_KEY,
            &principal_catalog,
        )?;
        write_plan.push_metadata(
            CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
            CLIENT_ROUTE_SYNC_SCOPE_CATALOG_KEY,
            &client_route_scope_catalog,
        )?;

        for runtime_scope in persisted_client_route_scopes {
            let snapshot_scope = client_route_sync_snapshot_scope(&runtime_scope);
            let feed_entries = client_route_sync_feeds
                .get(&runtime_scope)
                .map(|entries| entries.values().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            let latest_sync_seq = client_route_sync_sequences
                .get(&runtime_scope)
                .copied()
                .unwrap_or_else(|| {
                    feed_entries
                        .iter()
                        .map(|entry| entry.sync_seq)
                        .max()
                        .unwrap_or_default()
                });
            write_plan.push_metadata(
                snapshot_scope.as_str(),
                CLIENT_ROUTE_SYNC_SEQUENCE_KEY,
                &latest_sync_seq,
            )?;
            write_plan.push_timeline_batch(
                runtime_scope.tenant_id.as_str(),
                snapshot_scope.as_str(),
                feed_entries,
            )?;
        }

        Ok(true)
    }

    fn collect_contact_snapshot_writes(
        &self,
        write_plan: &mut ProjectionSnapshotWritePlan,
    ) -> Result<bool, ProjectionError> {
        let contacts = self.contacts.lock_projection("contact store").clone();
        let direct_chat_bindings = self
            .direct_chat_bindings
            .lock_projection("contact direct chat binding store")
            .values()
            .cloned()
            .collect::<Vec<_>>();

        if contacts.is_empty() && direct_chat_bindings.is_empty() {
            return Ok(false);
        }

        let mut owner_catalog = BTreeSet::new();
        for (runtime_scope, scope_contacts) in &contacts {
            owner_catalog.insert(ContactOwnerCatalogEntry {
                tenant_id: runtime_scope.tenant_id.clone(),
                organization_id: runtime_scope.organization_id.clone(),
                owner_user_id: runtime_scope.owner_user_id.clone(),
            });
            let snapshot_scope = contact_owner_snapshot_scope(
                runtime_scope.tenant_id.as_str(),
                runtime_scope.organization_id.as_str(),
                runtime_scope.owner_user_id.as_str(),
            );
            let items = contact_snapshot_items(scope_contacts);
            write_plan.push_metadata(snapshot_scope.as_str(), CONTACTS_KEY, &items)?;
        }

        write_plan.push_metadata(
            CONTACT_CATALOG_SCOPE,
            CONTACT_OWNERS_KEY,
            &owner_catalog.into_iter().collect::<Vec<_>>(),
        )?;
        write_plan.push_metadata(
            CONTACT_CATALOG_SCOPE,
            CONTACT_DIRECT_CHAT_BINDINGS_KEY,
            &direct_chat_bindings,
        )?;

        Ok(true)
    }

    pub fn restore_contact_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
    ) -> Result<bool, ProjectionError> {
        (|| {
            let owner_catalog = load_metadata_snapshot::<Vec<ContactOwnerCatalogEntry>>(
                metadata_store,
                CONTACT_CATALOG_SCOPE,
                CONTACT_OWNERS_KEY,
            )?
            .unwrap_or_default();
            let direct_chat_bindings =
                load_metadata_snapshot::<Vec<crate::model::ContactDirectChatBindingView>>(
                    metadata_store,
                    CONTACT_CATALOG_SCOPE,
                    CONTACT_DIRECT_CHAT_BINDINGS_KEY,
                )?
                .unwrap_or_default();

            if owner_catalog.is_empty() && direct_chat_bindings.is_empty() {
                return Ok(false);
            }

            for owner in owner_catalog {
                let snapshot_scope = contact_owner_snapshot_scope(
                    owner.tenant_id.as_str(),
                    owner.organization_id.as_str(),
                    owner.owner_user_id.as_str(),
                );
                let items = load_metadata_snapshot::<Vec<ContactView>>(
                    metadata_store,
                    snapshot_scope.as_str(),
                    CONTACTS_KEY,
                )?
                .unwrap_or_default();
                self.contacts.lock_projection("contact store").insert(
                    contact_runtime_scope(
                        owner.tenant_id.as_str(),
                        owner.organization_id.as_str(),
                        owner.owner_user_id.as_str(),
                    ),
                    contact_map_from_items(items),
                );
            }

            self.direct_chat_bindings
                .lock_projection("contact direct chat binding store")
                .extend(direct_chat_bindings);

            Ok(true)
        })()
    }

    pub fn restore_client_route_sync_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<bool, ProjectionError> {
        let result = (|| {
            let principal_catalog = load_metadata_snapshot::<Vec<PrincipalSnapshotCatalogEntry>>(
                metadata_store,
                CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                REGISTERED_CLIENT_ROUTE_PRINCIPALS_KEY,
            )?
            .unwrap_or_default();
            let client_route_scope_catalog =
                load_metadata_snapshot::<Vec<ClientRouteSyncScopeCatalogEntry>>(
                    metadata_store,
                    CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                    CLIENT_ROUTE_SYNC_SCOPE_CATALOG_KEY,
                )?
                .unwrap_or_default();
            if principal_catalog.is_empty() && client_route_scope_catalog.is_empty() {
                return Ok(false);
            }

            let mut restored_client_route_scopes = BTreeSet::new();
            for principal in principal_catalog {
                let runtime_scope = client_route_principal_scope_key(
                    principal.tenant_id.as_str(),
                    principal.organization_id.as_str(),
                    principal.principal_kind.as_str(),
                    principal.principal_id.as_str(),
                );
                let snapshot_scope = client_route_principal_snapshot_scope(&runtime_scope);
                let devices = load_metadata_snapshot::<Vec<crate::RegisteredClientRouteView>>(
                    metadata_store,
                    snapshot_scope.as_str(),
                    REGISTERED_CLIENT_ROUTES_KEY,
                )?
                .unwrap_or_default();
                let device_map = devices
                    .into_iter()
                    .map(|device| {
                        restored_client_route_scopes.insert(client_route_feed_scope_key(
                            principal.tenant_id.as_str(),
                            principal.organization_id.as_str(),
                            principal.principal_kind.as_str(),
                            principal.principal_id.as_str(),
                            device.device_id.as_str(),
                        ));
                        (device.device_id.clone(), device)
                    })
                    .collect();
                self.registered_client_routes
                    .lock_projection("registered client route store")
                    .insert(runtime_scope, device_map);
            }

            for client_route_scope in client_route_scope_catalog {
                restored_client_route_scopes.insert(client_route_feed_scope_key(
                    client_route_scope.tenant_id.as_str(),
                    client_route_scope.organization_id.as_str(),
                    client_route_scope.principal_kind.as_str(),
                    client_route_scope.principal_id.as_str(),
                    client_route_scope.device_id.as_str(),
                ));
            }

            for runtime_scope in restored_client_route_scopes {
                let snapshot_scope = client_route_sync_snapshot_scope(&runtime_scope);
                let mut feed_entries = timeline_store
                    .load_timeline(runtime_scope.tenant_id.as_str(), snapshot_scope.as_str())
                    .map_err(ProjectionError::StoreFailure)?
                    .into_iter()
                    .map(|(_, payload)| {
                        serde_json::from_str::<ClientRouteSyncFeedEntry>(&payload)
                            .map_err(ProjectionError::InvalidSnapshot)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                feed_entries.sort_by_key(|entry| entry.sync_seq);

                let restored_latest_sync_seq = load_metadata_snapshot::<u64>(
                    metadata_store,
                    snapshot_scope.as_str(),
                    CLIENT_ROUTE_SYNC_SEQUENCE_KEY,
                )?
                .unwrap_or_else(|| {
                    feed_entries
                        .iter()
                        .map(|entry| entry.sync_seq)
                        .max()
                        .unwrap_or_default()
                });
                self.client_route_sync_sequences
                    .lock_projection("client route sync sequence store")
                    .insert(runtime_scope.clone(), restored_latest_sync_seq);
                self.client_route_sync_feeds
                    .lock_projection("client route sync feed store")
                    .insert(
                        runtime_scope,
                        feed_entries
                            .into_iter()
                            .map(|entry| (entry.sync_seq, entry))
                            .collect(),
                    );
            }

            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::ClientRouteSyncSnapshotRestore,
                "client-route-sync",
                CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                "restored client route sync projection snapshot catalog".to_owned(),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::ClientRouteSyncSnapshotRestore,
                "client-route-sync",
                CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
                error,
            ),
        }

        result
    }

    pub fn restore_all_durable_snapshots(
        &self,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
        conversation_scopes: &[String],
    ) -> Result<(), ProjectionError> {
        self.restore_contact_snapshot(metadata_store)?;
        self.restore_client_route_sync_snapshot(metadata_store, timeline_store)?;
        for scope in conversation_scopes {
            let Some((tenant_id, organization_id, conversation_id)) =
                parse_conversation_snapshot_scope(scope.as_str())
            else {
                continue;
            };
            self.restore_conversation_snapshot(
                tenant_id.as_str(),
                organization_id.as_str(),
                conversation_id.as_str(),
                metadata_store,
                timeline_store,
            )?;
        }
        Ok(())
    }

    pub fn persist_all_durable_snapshots(
        &self,
        metadata_store: &dyn MetadataStore,
        timeline_store: &dyn TimelineProjectionStore,
    ) -> Result<(), ProjectionError> {
        self.persist_contact_snapshot(metadata_store)?;
        self.persist_client_route_sync_snapshot(metadata_store, timeline_store)?;
        let scopes = self
            .conversations
            .lock_projection("conversation store")
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for scope in scopes {
            let Some((tenant_id, organization_id, conversation_id)) =
                parse_conversation_snapshot_scope(scope.as_str())
            else {
                continue;
            };
            self.persist_conversation_snapshot(
                tenant_id.as_str(),
                organization_id.as_str(),
                conversation_id.as_str(),
                metadata_store,
                timeline_store,
            )?;
        }
        Ok(())
    }
}

fn conversation_snapshot_scope(tenant_id: &str, organization_id: &str, conversation_id: &str) -> String {
    super::scope_key(tenant_id, organization_id, conversation_id)
}

fn parse_conversation_snapshot_scope(
    scope: &str,
) -> Option<(String, String, String)> {
    let segments = crate::scope::decode_projection_key_segments(scope)?;
    match segments.as_slice() {
        [tenant_id, organization_id, conversation_id] => Some((
            tenant_id.clone(),
            organization_id.clone(),
            conversation_id.clone(),
        )),
        _ => None,
    }
}

fn contact_owner_snapshot_scope(
    tenant_id: &str,
    organization_id: &str,
    owner_user_id: &str,
) -> String {
    let normalized_organization_id =
        im_platform_contracts::normalize_realtime_organization_id(organization_id);
    encode_projection_key_segments([
        PRINCIPAL_SNAPSHOT_SCOPE_PREFIX,
        "contact",
        tenant_id,
        normalized_organization_id.as_str(),
        owner_user_id,
    ])
}

fn client_route_principal_snapshot_scope(scope: &ClientRoutePrincipalScopeKey) -> String {
    encode_projection_key_segments([
        PRINCIPAL_SNAPSHOT_SCOPE_PREFIX,
        "typed",
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.principal_kind.as_str(),
        scope.principal_id.as_str(),
    ])
}

fn client_route_sync_snapshot_scope(scope: &ClientRouteFeedScopeKey) -> String {
    encode_projection_key_segments([
        CLIENT_ROUTE_SYNC_SNAPSHOT_SCOPE_PREFIX,
        "typed",
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.principal_kind.as_str(),
        scope.principal_id.as_str(),
        scope.device_id.as_str(),
    ])
}

#[cfg(test)]
fn persist_metadata_snapshot<T: Serialize>(
    metadata_store: &dyn MetadataStore,
    scope: &str,
    key: &str,
    value: &T,
) -> Result<(), ProjectionError> {
    let payload = serde_json::to_string(value).map_err(ProjectionError::InvalidSnapshot)?;
    metadata_store
        .put_snapshot(scope, key, payload.as_str())
        .map_err(ProjectionError::StoreFailure)
}

fn load_metadata_snapshot<T: DeserializeOwned>(
    metadata_store: &dyn MetadataStore,
    scope: &str,
    key: &str,
) -> Result<Option<T>, ProjectionError> {
    metadata_store
        .load_snapshot(scope, key)
        .map_err(ProjectionError::StoreFailure)?
        .map(|payload| serde_json::from_str(&payload).map_err(ProjectionError::InvalidSnapshot))
        .transpose()
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::panic::{self, AssertUnwindSafe};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use im_adapters_local_memory::{MemoryMetadataStore, MemoryTimelineProjectionStore};
    use im_domain_core::conversation::{MembershipRole, build_conversation_member};
    use im_platform_contracts::{
        ContractError, MetadataSnapshotRecord, MetadataStore, TimelineProjectionBatch,
        TimelineProjectionRecord, TimelineProjectionStore,
    };

    use super::*;

    #[derive(Clone, Default)]
    struct CountingMetadataStore {
        snapshots: Arc<Mutex<HashMap<(String, String), String>>>,
        single_put_calls: Arc<AtomicUsize>,
        batch_put_calls: Arc<AtomicUsize>,
    }

    impl CountingMetadataStore {
        fn single_put_calls(&self) -> usize {
            self.single_put_calls.load(Ordering::Relaxed)
        }

        fn batch_put_calls(&self) -> usize {
            self.batch_put_calls.load(Ordering::Relaxed)
        }

        fn insert_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) {
            let mut stored = self.snapshots.lock().expect("metadata store should lock");
            for snapshot in snapshots {
                stored.insert(
                    (snapshot.scope.clone(), snapshot.key.clone()),
                    snapshot.value.clone(),
                );
            }
        }
    }

    impl MetadataStore for CountingMetadataStore {
        fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError> {
            self.single_put_calls.fetch_add(1, Ordering::Relaxed);
            self.snapshots
                .lock()
                .expect("metadata store should lock")
                .insert((scope.to_owned(), key.to_owned()), value.to_owned());
            Ok(())
        }

        fn load_snapshot(&self, scope: &str, key: &str) -> Result<Option<String>, ContractError> {
            Ok(self
                .snapshots
                .lock()
                .expect("metadata store should lock")
                .get(&(scope.to_owned(), key.to_owned()))
                .cloned())
        }

        fn put_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) -> Result<(), ContractError> {
            self.batch_put_calls.fetch_add(1, Ordering::Relaxed);
            self.insert_snapshots(snapshots);
            Ok(())
        }
    }

    type TimelineProjectionTestEntries =
        Arc<Mutex<HashMap<(String, String), BTreeMap<u64, String>>>>;

    #[derive(Clone, Default)]
    struct CountingTimelineProjectionStore {
        entries: TimelineProjectionTestEntries,
        single_upsert_calls: Arc<AtomicUsize>,
        batch_upsert_calls: Arc<AtomicUsize>,
    }

    impl CountingTimelineProjectionStore {
        fn single_upsert_calls(&self) -> usize {
            self.single_upsert_calls.load(Ordering::Relaxed)
        }

        fn batch_upsert_calls(&self) -> usize {
            self.batch_upsert_calls.load(Ordering::Relaxed)
        }

        fn upsert_records(
            &self,
            tenant_id: &str,
            timeline_scope: &str,
            records: &[TimelineProjectionRecord],
        ) {
            let mut stored = self.entries.lock().expect("timeline store should lock");
            let scope_entries = stored
                .entry((tenant_id.to_owned(), timeline_scope.to_owned()))
                .or_default();
            for record in records {
                scope_entries.insert(record.message_seq, record.payload.clone());
            }
        }
    }

    impl TimelineProjectionStore for CountingTimelineProjectionStore {
        fn upsert_timeline_entry(
            &self,
            tenant_id: &str,
            timeline_scope: &str,
            message_seq: u64,
            payload: &str,
        ) -> Result<(), ContractError> {
            self.single_upsert_calls.fetch_add(1, Ordering::Relaxed);
            self.entries
                .lock()
                .expect("timeline store should lock")
                .entry((tenant_id.to_owned(), timeline_scope.to_owned()))
                .or_default()
                .insert(message_seq, payload.to_owned());
            Ok(())
        }

        fn load_timeline(
            &self,
            tenant_id: &str,
            timeline_scope: &str,
        ) -> Result<Vec<(u64, String)>, ContractError> {
            Ok(self
                .entries
                .lock()
                .expect("timeline store should lock")
                .get(&(tenant_id.to_owned(), timeline_scope.to_owned()))
                .map(|items| {
                    items
                        .iter()
                        .map(|(message_seq, payload)| (*message_seq, payload.clone()))
                        .collect()
                })
                .unwrap_or_default())
        }

        fn upsert_timeline_batches(
            &self,
            batches: &[TimelineProjectionBatch],
        ) -> Result<(), ContractError> {
            self.batch_upsert_calls.fetch_add(1, Ordering::Relaxed);
            for batch in batches {
                self.upsert_records(
                    batch.tenant_id.as_str(),
                    batch.timeline_scope.as_str(),
                    &batch.records,
                );
            }
            Ok(())
        }
    }

    fn conversation_created_event(
        tenant_id: &str,
        conversation_id: &str,
        conversation_type: &str,
    ) -> im_domain_events::CommitEnvelope {
        im_domain_events::CommitEnvelope::minimal(
            &format!("evt_{tenant_id}_{conversation_id}_created"),
            tenant_id,
            "conversation.created",
            "conversation",
            conversation_id,
            0,
        )
        .with_payload(
            "conversation.created.v1",
            &serde_json::json!({
                "conversationType": conversation_type,
            })
            .to_string(),
        )
    }

    fn member_joined_event(
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
        role: MembershipRole,
    ) -> im_domain_events::CommitEnvelope {
        let member = build_conversation_member(
            tenant_id,
            conversation_id,
            format!("cm_{conversation_id}_{principal_id}"),
            principal_id,
            "user",
            role,
            Some("1".into()),
            "2026-04-08T10:00:00Z".into(),
        );

        im_domain_events::CommitEnvelope::minimal(
            &format!("evt_{tenant_id}_{conversation_id}_{principal_id}_joined"),
            tenant_id,
            "conversation.member_joined",
            "conversation",
            conversation_id,
            1,
        )
        .with_payload(
            "conversation.member_joined.v1",
            &serde_json::to_string(&member).expect("member should serialize"),
        )
    }

    fn message_posted_event(
        tenant_id: &str,
        conversation_id: &str,
        message_id: &str,
        message_seq: u64,
        sender_id: &str,
        summary: &str,
    ) -> im_domain_events::CommitEnvelope {
        im_domain_events::CommitEnvelope::minimal(
            &format!("evt_{tenant_id}_{conversation_id}_{message_seq}"),
            tenant_id,
            "message.posted",
            "conversation",
            conversation_id,
            message_seq,
        )
        .with_payload(
            "message.posted.v1",
            &serde_json::json!({
                "tenantId": tenant_id,
                "conversationId": conversation_id,
                "messageId": message_id,
                "messageSeq": message_seq,
                "sender": {
                    "id": sender_id,
                    "kind": "user",
                    "memberId": format!("cm_{sender_id}"),
                    "deviceId": format!("d_{sender_id}"),
                    "sessionId": format!("s_{sender_id}"),
                    "metadata": {}
                },
                "messageType": "standard",
                "deliveryMode": "discrete",
                "clientMsgId": format!("client_{message_id}"),
                "streamSessionId": null,
                "rtcSessionId": null,
                "body": {
                    "summary": summary,
                    "parts": [{
                        "kind": "text",
                        "text": summary
                    }],
                    "renderHints": {}
                },
                "attributes": {},
                "metadata": {},
                "occurredAt": "2026-04-08T10:00:01Z",
                "committedAt": "2026-04-08T10:00:01Z"
            })
            .to_string(),
        )
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    fn demo_summary(tenant_id: &str, conversation_id: &str) -> ConversationSummaryView {
        ConversationSummaryView {
            tenant_id: tenant_id.into(),
            conversation_id: conversation_id.into(),
            message_count: 0,
            last_message_id: None,
            last_message_seq: 0,
            last_sender_id: None,
            last_sender_kind: None,
            last_sender: None,
            last_summary: None,
            last_message_at: None,
            agent_handoff: None,
        }
    }

    #[test]
    fn test_persist_conversation_snapshot_batches_projection_store_flushes() {
        let projection = TimelineProjectionService::default();
        let metadata_store = CountingMetadataStore::default();
        let timeline_store = CountingTimelineProjectionStore::default();

        projection
            .apply(&conversation_created_event("100001", "c_batch", "group"))
            .expect("conversation projection should succeed");
        projection
            .apply(&member_joined_event(
                "100001",
                "c_batch",
                "1014",
                MembershipRole::Owner,
            ))
            .expect("member projection should succeed");
        projection.register_client_route("100001", "default", "1014", "d_phone");
        projection
            .apply(&message_posted_event(
                "100001",
                "c_batch",
                "msg_batch_1",
                1,
                "1014",
                "batched snapshot",
            ))
            .expect("message projection should succeed");

        assert!(
            projection
                .persist_conversation_snapshot(
                    "100001",
                    "default",
                    "c_batch",
                    &metadata_store,
                    &timeline_store,
                )
                .expect("snapshot should persist"),
            "snapshot should exist"
        );

        assert_eq!(
            metadata_store.single_put_calls(),
            0,
            "conversation snapshot persistence should avoid per-record metadata flushes"
        );
        assert_eq!(
            metadata_store.batch_put_calls(),
            1,
            "conversation snapshot persistence should flush metadata once per envelope"
        );
        assert_eq!(
            timeline_store.single_upsert_calls(),
            0,
            "conversation snapshot persistence should avoid per-entry timeline flushes"
        );
        assert_eq!(
            timeline_store.batch_upsert_calls(),
            1,
            "conversation snapshot persistence should flush timeline projections once per envelope"
        );
        assert_eq!(
            timeline_store
                .load_timeline("100001", "c_batch")
                .expect("conversation timeline should load")
                .len(),
            1,
            "conversation timeline should still persist its message entries"
        );
        assert_eq!(
            timeline_store
                .load_timeline(
                    "100001",
                    client_route_sync_snapshot_scope(&client_route_feed_scope_key(
                        "100001", "default", "user", "1014", "d_phone"
                    ))
                    .as_str(),
                )
                .expect("client route sync timeline should load")
                .len(),
            1,
            "client route sync timeline should still persist its feed entries"
        );
    }

    #[test]
    fn test_persist_contact_snapshot_recovers_from_poisoned_contact_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        poison_mutex(&projection.contacts);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.persist_contact_snapshot(&metadata_store)
        }));
        assert!(
            result.is_ok(),
            "persist_contact_snapshot should not panic when contact store lock is poisoned"
        );
        let persist_result = result.expect("panic status should be captured");
        assert!(
            persist_result.is_ok(),
            "persist_contact_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_restore_conversation_snapshot_recovers_from_poisoned_summary_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        let timeline_store = MemoryTimelineProjectionStore::default();
        let snapshot_scope = conversation_snapshot_scope("100001", "default", "c_demo");
        persist_metadata_snapshot(
            &metadata_store,
            snapshot_scope.as_str(),
            CONVERSATION_SUMMARY_KEY,
            &demo_summary("100001", "c_demo"),
        )
        .expect("summary snapshot should persist");

        poison_mutex(&projection.summaries);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.restore_conversation_snapshot(
                "100001",
                "default",
                "c_demo",
                &metadata_store,
                &timeline_store,
            )
        }));
        assert!(
            result.is_ok(),
            "restore_conversation_snapshot should not panic when summary store lock is poisoned"
        );
        let restore_result = result.expect("panic status should be captured");
        assert!(
            restore_result.is_ok(),
            "restore_conversation_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_persist_client_route_sync_snapshot_recovers_from_poisoned_registered_client_route_store_lock()
     {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        let timeline_store = MemoryTimelineProjectionStore::default();
        poison_mutex(&projection.registered_client_routes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.persist_client_route_sync_snapshot(&metadata_store, &timeline_store)
        }));
        assert!(
            result.is_ok(),
            "persist_client_route_sync_snapshot should not panic when registered-client-route lock is poisoned"
        );
        let persist_result = result.expect("panic status should be captured");
        assert!(
            persist_result.is_ok(),
            "persist_client_route_sync_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_restore_contact_snapshot_recovers_from_poisoned_binding_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        persist_metadata_snapshot(
            &metadata_store,
            CONTACT_CATALOG_SCOPE,
            CONTACT_OWNERS_KEY,
            &Vec::<ContactOwnerCatalogEntry>::new(),
        )
        .expect("contact owner catalog should persist");
        persist_metadata_snapshot(
            &metadata_store,
            CONTACT_CATALOG_SCOPE,
            CONTACT_DIRECT_CHAT_BINDINGS_KEY,
            &vec![crate::model::ContactDirectChatBindingView {
                tenant_id: Some("100001".into()),
                organization_id: Some("0".into()),
                direct_chat_id: "dc_demo".into(),
                conversation_id: "c_demo".into(),
                bound_at: "2026-04-01T00:00:00.000Z".into(),
                status: im_domain_core::social::DirectChatStatus::Active,
                updated_at: Some("2026-04-01T00:00:00.000Z".into()),
            }],
        )
        .expect("contact direct-chat binding snapshot should persist");
        poison_mutex(&projection.direct_chat_bindings);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.restore_contact_snapshot(&metadata_store)
        }));
        assert!(
            result.is_ok(),
            "restore_contact_snapshot should not panic when direct-chat-binding lock is poisoned"
        );
        let restore_result = result.expect("panic status should be captured");
        assert!(
            restore_result.is_ok(),
            "restore_contact_snapshot should recover from poisoned lock"
        );
    }

    #[test]
    fn test_restore_client_route_sync_snapshot_recovers_from_poisoned_sequence_store_lock() {
        let projection = TimelineProjectionService::default();
        let metadata_store = MemoryMetadataStore::default();
        let timeline_store = MemoryTimelineProjectionStore::default();
        let device_scope = ClientRouteSyncScopeCatalogEntry {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_id: "1".into(),
            principal_kind: "user".into(),
            device_id: "d_demo".into(),
        };
        persist_metadata_snapshot(
            &metadata_store,
            CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
            REGISTERED_CLIENT_ROUTE_PRINCIPALS_KEY,
            &Vec::<PrincipalSnapshotCatalogEntry>::new(),
        )
        .expect("client route sync principal catalog should persist");
        persist_metadata_snapshot(
            &metadata_store,
            CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
            CLIENT_ROUTE_SYNC_SCOPE_CATALOG_KEY,
            &vec![device_scope.clone()],
        )
        .expect("client route sync scope catalog should persist");
        persist_metadata_snapshot(
            &metadata_store,
            client_route_sync_snapshot_scope(&client_route_feed_scope_key(
                device_scope.tenant_id.as_str(),
                device_scope.organization_id.as_str(),
                device_scope.principal_kind.as_str(),
                device_scope.principal_id.as_str(),
                device_scope.device_id.as_str(),
            ))
            .as_str(),
            CLIENT_ROUTE_SYNC_SEQUENCE_KEY,
            &42_u64,
        )
        .expect("client route sync sequence snapshot should persist");
        poison_mutex(&projection.client_route_sync_sequences);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.restore_client_route_sync_snapshot(&metadata_store, &timeline_store)
        }));
        assert!(
            result.is_ok(),
            "restore_client_route_sync_snapshot should not panic when sequence lock is poisoned"
        );
        let restore_result = result.expect("panic status should be captured");
        assert!(
            restore_result.is_ok(),
            "restore_client_route_sync_snapshot should recover from poisoned lock"
        );
    }
}
