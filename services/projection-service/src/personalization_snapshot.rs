//! Durable metadata snapshots for per-principal conversation preferences and message favorites.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use im_platform_contracts::MetadataStore;
use serde::{Deserialize, Serialize};

use crate::conversation_personalization::conversation_preferences_key;
use crate::message_favorites::message_favorites_scope_key;
use crate::model::{ConversationPreferencesView, MessageFavoriteView};
use crate::observability::ProjectionSnapshotOperation;
use crate::projection::ProjectionError;
use crate::scope::{decode_projection_key_segments, encode_projection_key_segments};
use crate::{TimelineProjectionService, lock_projection_mutex};

const PERSONALIZATION_CATALOG_SCOPE: &str = "projection-personalization";
const PERSONALIZATION_PRINCIPALS_KEY: &str = "personalization-principals";
const PERSONALIZATION_SNAPSHOT_KEY: &str = "principal-personalization";
const PRINCIPAL_SNAPSHOT_SCOPE_PREFIX: &str = "principal";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
struct PersonalizationPrincipalCatalogEntry {
    tenant_id: String,
    organization_id: String,
    principal_kind: String,
    principal_id: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PersonalizationPrincipalSnapshot {
    conversation_preferences: Vec<ConversationPreferencesView>,
    message_favorites: Vec<MessageFavoriteView>,
}

impl TimelineProjectionService {
    pub fn persist_personalization_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
    ) -> Result<bool, ProjectionError> {
        let result = (|| {
            let mut write_plan = super::snapshot::ProjectionSnapshotWritePlan::default();
            if !self.collect_personalization_snapshot_writes(&mut write_plan)? {
                return Ok(false);
            }
            write_plan.commit_metadata_only(metadata_store)?;
            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::PersonalizationSnapshotPersist,
                "personalization",
                PERSONALIZATION_CATALOG_SCOPE,
                "persisted personalization projection snapshot catalog".to_owned(),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::PersonalizationSnapshotPersist,
                "personalization",
                PERSONALIZATION_CATALOG_SCOPE,
                error,
            ),
        }

        result
    }

    pub fn restore_personalization_snapshot(
        &self,
        metadata_store: &dyn MetadataStore,
    ) -> Result<bool, ProjectionError> {
        let result = (|| {
            let principal_catalog = super::snapshot::load_metadata_snapshot::<
                Vec<PersonalizationPrincipalCatalogEntry>,
            >(
                metadata_store,
                PERSONALIZATION_CATALOG_SCOPE,
                PERSONALIZATION_PRINCIPALS_KEY,
            )?
            .unwrap_or_default();

            if principal_catalog.is_empty() {
                return Ok(false);
            }

            let mut preferences = lock_projection_mutex(
                &self.conversation_preferences,
                "conversation preferences store",
            );
            preferences.clear();
            let mut favorites =
                lock_projection_mutex(&self.message_favorites, "message favorites store");
            favorites.clear();
            lock_projection_mutex(&self.message_favorites_index, "message favorites index").clear();

            for principal in principal_catalog {
                let snapshot_scope = personalization_principal_snapshot_scope(
                    principal.tenant_id.as_str(),
                    principal.organization_id.as_str(),
                    principal.principal_kind.as_str(),
                    principal.principal_id.as_str(),
                );
                let snapshot =
                    super::snapshot::load_metadata_snapshot::<PersonalizationPrincipalSnapshot>(
                        metadata_store,
                        snapshot_scope.as_str(),
                        PERSONALIZATION_SNAPSHOT_KEY,
                    )?
                    .unwrap_or_default();

                for preference in snapshot.conversation_preferences {
                    let organization_id = im_platform_contracts::normalize_realtime_organization_id(
                        principal.organization_id.as_str(),
                    );
                    let key = conversation_preferences_key(
                        preference.tenant_id.as_str(),
                        organization_id.as_str(),
                        preference.conversation_id.as_str(),
                        preference.principal_kind.as_str(),
                        preference.principal_id.as_str(),
                    );
                    preferences.insert(key, preference);
                }

                let favorite_map = snapshot
                    .message_favorites
                    .into_iter()
                    .map(|favorite| (favorite.favorite_id.clone(), favorite))
                    .collect::<HashMap<_, _>>();
                if !favorite_map.is_empty() {
                    let key = message_favorites_scope_key(
                        principal.tenant_id.as_str(),
                        principal.organization_id.as_str(),
                        principal.principal_kind.as_str(),
                        principal.principal_id.as_str(),
                    );
                    favorites.insert(key.clone(), favorite_map.clone());
                    self.rebuild_message_favorites_index_for_scope(key.as_str(), &favorite_map);
                }
            }

            Ok(true)
        })();

        match &result {
            Ok(true) => self.record_projection_snapshot_success(
                ProjectionSnapshotOperation::PersonalizationSnapshotRestore,
                "personalization",
                PERSONALIZATION_CATALOG_SCOPE,
                "restored personalization projection snapshot catalog".to_owned(),
            ),
            Ok(false) => {}
            Err(error) => self.record_projection_snapshot_failure(
                ProjectionSnapshotOperation::PersonalizationSnapshotRestore,
                "personalization",
                PERSONALIZATION_CATALOG_SCOPE,
                error,
            ),
        }

        result
    }

    fn collect_personalization_snapshot_writes(
        &self,
        write_plan: &mut super::snapshot::ProjectionSnapshotWritePlan,
    ) -> Result<bool, ProjectionError> {
        let preferences = self
            .conversation_preferences
            .lock_projection("conversation preferences store")
            .clone();
        let favorites = self
            .message_favorites
            .lock_projection("message favorites store")
            .clone();

        if preferences.is_empty() && favorites.is_empty() {
            return Ok(false);
        }

        let mut principal_catalog = BTreeSet::new();
        let mut snapshots = BTreeMap::<
            PersonalizationPrincipalCatalogEntry,
            PersonalizationPrincipalSnapshot,
        >::new();

        for (storage_key, preference) in preferences {
            let Some((tenant_id, organization_id, conversation_id, principal_kind, principal_id)) =
                parse_conversation_preferences_storage_key(storage_key.as_str())
            else {
                continue;
            };
            let entry = PersonalizationPrincipalCatalogEntry {
                tenant_id: tenant_id.clone(),
                organization_id: organization_id.clone(),
                principal_kind: principal_kind.clone(),
                principal_id: principal_id.clone(),
            };
            principal_catalog.insert(entry.clone());
            snapshots
                .entry(entry)
                .or_default()
                .conversation_preferences
                .push(ConversationPreferencesView {
                    tenant_id,
                    conversation_id,
                    principal_kind,
                    principal_id,
                    is_pinned: preference.is_pinned,
                    is_muted: preference.is_muted,
                    is_marked_unread: preference.is_marked_unread,
                    is_hidden: preference.is_hidden,
                    updated_at: preference.updated_at,
                });
        }

        for (scope_key, favorite_map) in favorites {
            let Some((tenant_id, organization_id, principal_kind, principal_id)) =
                parse_message_favorites_storage_key(scope_key.as_str())
            else {
                continue;
            };
            let entry = PersonalizationPrincipalCatalogEntry {
                tenant_id: tenant_id.clone(),
                organization_id: organization_id.clone(),
                principal_kind: principal_kind.clone(),
                principal_id: principal_id.clone(),
            };
            principal_catalog.insert(entry.clone());
            snapshots.entry(entry).or_default().message_favorites =
                favorite_map.into_values().collect();
        }

        write_plan.push_metadata(
            PERSONALIZATION_CATALOG_SCOPE,
            PERSONALIZATION_PRINCIPALS_KEY,
            &principal_catalog.into_iter().collect::<Vec<_>>(),
        )?;

        for (principal, snapshot) in snapshots {
            let snapshot_scope = personalization_principal_snapshot_scope(
                principal.tenant_id.as_str(),
                principal.organization_id.as_str(),
                principal.principal_kind.as_str(),
                principal.principal_id.as_str(),
            );
            write_plan.push_metadata(
                snapshot_scope.as_str(),
                PERSONALIZATION_SNAPSHOT_KEY,
                &snapshot,
            )?;
        }

        Ok(true)
    }
}

fn personalization_principal_snapshot_scope(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    let normalized_organization_id =
        im_platform_contracts::normalize_realtime_organization_id(organization_id);
    encode_projection_key_segments([
        PRINCIPAL_SNAPSHOT_SCOPE_PREFIX,
        "personalization",
        tenant_id,
        normalized_organization_id.as_str(),
        principal_kind,
        principal_id,
    ])
}

fn parse_conversation_preferences_storage_key(
    storage_key: &str,
) -> Option<(String, String, String, String, String)> {
    let (scope_key, principal_kind, principal_id) = split_principal_storage_key(storage_key)?;
    let segments = decode_projection_key_segments(scope_key.as_str())?;
    let [tenant_id, organization_id, conversation_id] = segments.as_slice() else {
        return None;
    };
    Some((
        tenant_id.clone(),
        organization_id.clone(),
        conversation_id.clone(),
        principal_kind,
        principal_id,
    ))
}

fn parse_message_favorites_storage_key(
    storage_key: &str,
) -> Option<(String, String, String, String)> {
    let (scope_key, principal_kind, principal_id) = split_principal_storage_key(storage_key)?;
    let segments = decode_projection_key_segments(scope_key.as_str())?;
    let [tenant_id, organization_id, marker] = segments.as_slice() else {
        return None;
    };
    if marker != "message-favorites" {
        return None;
    }
    Some((
        tenant_id.clone(),
        organization_id.clone(),
        principal_kind,
        principal_id,
    ))
}

fn split_principal_storage_key(storage_key: &str) -> Option<(String, String, String)> {
    let principal_id = storage_key.rsplit_once(':')?.1.to_owned();
    let remainder = storage_key.rsplit_once(':')?.0;
    let principal_kind = remainder.rsplit_once(':')?.1.to_owned();
    let scope_key = remainder.rsplit_once(':')?.0.to_owned();
    Some((scope_key, principal_kind, principal_id))
}

trait ProjectionStoreLock<T> {
    fn lock_projection(&self, lock_name: &'static str) -> std::sync::MutexGuard<'_, T>;
}

impl<T> ProjectionStoreLock<T> for std::sync::Mutex<T> {
    fn lock_projection(&self, lock_name: &'static str) -> std::sync::MutexGuard<'_, T> {
        lock_projection_mutex(self, lock_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_conversation_preferences_storage_key_round_trip() {
        let key = conversation_preferences_key("100001", "default", "c_demo", "user", "42");
        let parsed =
            parse_conversation_preferences_storage_key(key.as_str()).expect("key should parse");
        assert_eq!(parsed.0, "100001");
        assert_eq!(parsed.1, "0");
        assert_eq!(parsed.2, "c_demo");
        assert_eq!(parsed.3, "user");
        assert_eq!(parsed.4, "42");
    }

    #[test]
    fn test_parse_message_favorites_storage_key_round_trip() {
        let key = message_favorites_scope_key("100001", "default", "user", "42");
        let parsed = parse_message_favorites_storage_key(key.as_str()).expect("key should parse");
        assert_eq!(parsed.0, "100001");
        assert_eq!(parsed.1, "0");
        assert_eq!(parsed.2, "user");
        assert_eq!(parsed.3, "42");
    }
}
