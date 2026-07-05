//! Contact open-api persistence with Postgres authority and in-memory dev/test fallback.

use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, OnceLock, RwLock};

use im_adapters_social_postgres::contact_store::{
    ContactPreferencesRecord as PostgresContactPreferencesRecord,
    ContactRecommendationRecord as PostgresContactRecommendationRecord, ContactStore,
    ContactTagRecord as PostgresContactTagRecord, PostgresContactStore,
    contact_recommendation_id_to_i64, contact_tag_id_to_i64,
};
use im_app_context::{resolve_web_environment_from_process_env, AppContext};
use sdkwork_utils_rust::offset_limit_page_from_iter;
use sdkwork_web_core::WebEnvironment;

use crate::friendship::SocialServiceError;

fn contact_owner_user_id(auth: &AppContext) -> Result<&str, SocialServiceError> {
    auth.ensure_user_actor_principal().map_err(|error| {
        SocialServiceError::invalid("social_principal_invalid", error.message())
    })
}

static CONTACT_MEMORY_STORE: OnceLock<RwLock<ContactMemoryStore>> = OnceLock::new();
static CONTACT_POSTGRES_STORE: OnceLock<Option<Arc<dyn ContactStore>>> = OnceLock::new();

pub async fn init_contact_postgres_store() {
    if CONTACT_POSTGRES_STORE.get().is_some() {
        return;
    }
    let store = crate::journal_bootstrap::resolve_social_postgres_pool_from_env().map(|pool| {
        Arc::new(PostgresContactStore::new(Arc::new(pool.inner().clone())))
            as Arc<dyn ContactStore>
    });
    if store.is_none() && !allows_contact_memory_fallback() {
        tracing::error!(
            "contact open-api fail-closed: IM postgres pool is required in production for contact tags, preferences, and recommendations"
        );
    }
    let _ = CONTACT_POSTGRES_STORE.set(store);
}

pub fn shared_contact_store() -> Option<Arc<dyn ContactStore>> {
    CONTACT_POSTGRES_STORE
        .get()
        .and_then(|store| store.clone())
}

#[derive(Clone, Debug)]
pub struct ContactTagRecord {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub tag_id: String,
    pub name: String,
    pub color: String,
    pub count: i32,
    pub bg: String,
    pub border: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct ContactPreferencesRecord {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub target_user_id: String,
    pub is_starred: bool,
    pub remark: String,
    pub is_blocked: bool,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct ContactRecommendationRecord {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub target_user_id: String,
    pub recommendation_id: String,
    pub target_conversation_id: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ContactTagOwnerKey {
    tenant_id: String,
    owner_user_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ContactTagSortKey {
    owner: ContactTagOwnerKey,
    updated_at: Reverse<String>,
    tag_id: Reverse<String>,
}

#[derive(Default)]
struct ContactMemoryStore {
    tag_index: BTreeMap<ContactTagSortKey, ContactTagRecord>,
    tag_identity: HashMap<(String, String, String), ContactTagSortKey>,
    preferences: Vec<ContactPreferencesRecord>,
    recommendations: Vec<ContactRecommendationRecord>,
}

pub fn list_contact_tags(
    store: Option<&Arc<dyn ContactStore>>,
    auth: &AppContext,
    limit: usize,
    offset: usize,
) -> Result<(Vec<ContactTagRecord>, bool), SocialServiceError> {
    ensure_contact_memory_fallback_allowed(store)?;
    if let Some(store) = store {
        let records = store
            .list_tags_by_owner(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                contact_owner_user_id(auth)?,
                (limit.saturating_add(1)) as i64,
                offset as i64,
            )
            .map_err(|_| store_unavailable("list_contact_tags"))?;
        let has_more = records.len() > limit;
        let items = records
            .into_iter()
            .take(limit)
            .map(tag_record_from_postgres)
            .collect();
        return Ok((items, has_more));
    }

    let guard = memory_store().read().map_err(|_| store_lock_error())?;
    let owner_key = ContactTagOwnerKey {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: contact_owner_user_id(auth)?.to_owned(),
    };
    let start = ContactTagSortKey {
        owner: owner_key.clone(),
        updated_at: Reverse(String::new()),
        tag_id: Reverse(String::new()),
    };
    let end = ContactTagSortKey {
        owner: owner_key,
        updated_at: Reverse(String::from("\u{10FFFF}")),
        tag_id: Reverse(String::from("\u{10FFFF}")),
    };
    let page = offset_limit_page_from_iter(
        guard.tag_index.range(start..=end).map(|(_, record)| record.clone()),
        limit,
        offset,
    );
    Ok((page.items, page.has_more))
}

pub fn upsert_contact_tag(
    store: Option<&Arc<dyn ContactStore>>,
    auth: &AppContext,
    record: ContactTagRecord,
) -> Result<(), SocialServiceError> {
    ensure_contact_memory_fallback_allowed(store)?;
    if let Some(store) = store {
        return store
            .upsert_tag(&tag_record_to_postgres(auth, &record))
            .map_err(|_| store_unavailable("upsert_contact_tag"));
    }
    let mut guard = memory_store().write().map_err(|_| store_lock_error())?;
    upsert_contact_tag_in_memory(&mut guard, record);
    Ok(())
}

fn upsert_contact_tag_in_memory(store: &mut ContactMemoryStore, record: ContactTagRecord) {
    let identity = (
        record.tenant_id.clone(),
        record.owner_user_id.clone(),
        record.tag_id.clone(),
    );
    if let Some(previous_key) = store.tag_identity.remove(&identity) {
        store.tag_index.remove(&previous_key);
    }
    let sort_key = contact_tag_sort_key(&record);
    store.tag_identity.insert(identity, sort_key.clone());
    store.tag_index.insert(sort_key, record);
}

pub fn get_contact_tag(
    store: Option<&Arc<dyn ContactStore>>,
    auth: &AppContext,
    tag_id: &str,
) -> Result<Option<ContactTagRecord>, SocialServiceError> {
    ensure_contact_memory_fallback_allowed(store)?;
    if let Some(store) = store {
        let record = store
            .get_tag(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                contact_owner_user_id(auth)?,
                contact_tag_id_to_i64(tag_id),
            )
            .map_err(|_| store_unavailable("get_contact_tag"))?;
        return Ok(record.map(tag_record_from_postgres));
    }
    let guard = memory_store().read().map_err(|_| store_lock_error())?;
    let owner_user_id = contact_owner_user_id(auth)?;
    Ok(find_contact_tag_in_memory(&guard, auth, owner_user_id, tag_id))
}

fn find_contact_tag_in_memory(
    store: &ContactMemoryStore,
    auth: &AppContext,
    owner_user_id: &str,
    tag_id: &str,
) -> Option<ContactTagRecord> {
    let identity = (
        auth.tenant_id.clone(),
        owner_user_id.to_owned(),
        tag_id.to_owned(),
    );
    store
        .tag_identity
        .get(&identity)
        .and_then(|sort_key| store.tag_index.get(sort_key).cloned())
}

pub fn delete_contact_tag(
    store: Option<&Arc<dyn ContactStore>>,
    auth: &AppContext,
    tag_id: &str,
) -> Result<bool, SocialServiceError> {
    ensure_contact_memory_fallback_allowed(store)?;
    if let Some(store) = store {
        let exists = store
            .get_tag(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                contact_owner_user_id(auth)?,
                contact_tag_id_to_i64(tag_id),
            )
            .map_err(|_| store_unavailable("delete_contact_tag"))?
            .is_some();
        if !exists {
            return Ok(false);
        }
        store
            .delete_tag(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                contact_owner_user_id(auth)?,
                contact_tag_id_to_i64(tag_id),
            )
            .map_err(|_| store_unavailable("delete_contact_tag"))?;
        return Ok(true);
    }
    let mut guard = memory_store().write().map_err(|_| store_lock_error())?;
    let owner_user_id = contact_owner_user_id(auth)?;
    Ok(remove_contact_tag_from_memory(
        &mut guard,
        auth,
        owner_user_id,
        tag_id,
    ))
}

fn remove_contact_tag_from_memory(
    store: &mut ContactMemoryStore,
    auth: &AppContext,
    owner_user_id: &str,
    tag_id: &str,
) -> bool {
    let identity = (
        auth.tenant_id.clone(),
        owner_user_id.to_owned(),
        tag_id.to_owned(),
    );
    let Some(sort_key) = store.tag_identity.remove(&identity) else {
        return false;
    };
    store.tag_index.remove(&sort_key);
    true
}

pub fn get_contact_preferences(
    store: Option<&Arc<dyn ContactStore>>,
    auth: &AppContext,
    target_user_id: &str,
) -> Result<ContactPreferencesRecord, SocialServiceError> {
    ensure_contact_memory_fallback_allowed(store)?;
    let owner_user_id = contact_owner_user_id(auth)?;
    if let Some(store) = store {
        let record = store
            .get_preferences(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                owner_user_id,
                target_user_id,
            )
            .map_err(|_| store_unavailable("get_contact_preferences"))?;
        return Ok(record
            .map(preferences_record_from_postgres)
            .unwrap_or_else(|| default_preferences(auth, target_user_id)));
    }
    let guard = memory_store().read().map_err(|_| store_lock_error())?;
    Ok(guard
        .preferences
        .iter()
        .find(|record| {
            record.tenant_id == auth.tenant_id
                && record.owner_user_id == owner_user_id
                && record.target_user_id == target_user_id
        })
        .cloned()
        .unwrap_or_else(|| default_preferences(auth, target_user_id)))
}

pub fn upsert_contact_preferences(
    store: Option<&Arc<dyn ContactStore>>,
    auth: &AppContext,
    record: ContactPreferencesRecord,
) -> Result<(), SocialServiceError> {
    ensure_contact_memory_fallback_allowed(store)?;
    if let Some(store) = store {
        return store
            .upsert_preferences(&preferences_record_to_postgres(auth, &record))
            .map_err(|_| store_unavailable("upsert_contact_preferences"));
    }
    let mut guard = memory_store().write().map_err(|_| store_lock_error())?;
    if let Some(index) = guard.preferences.iter().position(|existing| {
        existing.tenant_id == record.tenant_id
            && existing.owner_user_id == record.owner_user_id
            && existing.target_user_id == record.target_user_id
    }) {
        guard.preferences[index] = record;
    } else {
        guard.preferences.push(record);
    }
    Ok(())
}

pub fn create_contact_recommendation(
    store: Option<&Arc<dyn ContactStore>>,
    auth: &AppContext,
    target_user_id: &str,
    recommendation_id: &str,
    target_conversation_id: Option<String>,
) -> Result<ContactRecommendationRecord, SocialServiceError> {
    let target_user_id = target_user_id.trim();
    if target_user_id.is_empty() {
        return Err(SocialServiceError::invalid(
            "contact_recommendation_target_required",
            "target user id is required",
        ));
    }
    if target_user_id == contact_owner_user_id(auth)? {
        return Err(SocialServiceError::invalid(
            "contact_recommendation_self_target",
            "cannot create a contact recommendation for yourself",
        ));
    }

    ensure_contact_memory_fallback_allowed(store)?;

    let record = ContactRecommendationRecord {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: contact_owner_user_id(auth)?.to_owned(),
        target_user_id: target_user_id.to_owned(),
        recommendation_id: recommendation_id.to_owned(),
        target_conversation_id,
        created_at: im_time::utc_now_rfc3339_millis(),
    };

    if let Some(store) = store {
        store
            .insert_recommendation(&recommendation_record_to_postgres(auth, &record))
            .map_err(|_| store_unavailable("create_contact_recommendation"))?;
        return Ok(record);
    }

    let mut guard = memory_store().write().map_err(|_| store_lock_error())?;
    guard.recommendations.push(record.clone());
    Ok(record)
}

pub fn default_preferences(auth: &AppContext, target_user_id: &str) -> ContactPreferencesRecord {
    ContactPreferencesRecord {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.social_principal_user_id().to_owned(),
        target_user_id: target_user_id.to_owned(),
        is_starred: false,
        remark: String::new(),
        is_blocked: false,
        updated_at: im_time::utc_now_rfc3339_millis(),
    }
}

fn memory_store() -> &'static RwLock<ContactMemoryStore> {
    CONTACT_MEMORY_STORE.get_or_init(|| RwLock::new(ContactMemoryStore::default()))
}

fn contact_tag_sort_key(record: &ContactTagRecord) -> ContactTagSortKey {
    ContactTagSortKey {
        owner: ContactTagOwnerKey {
            tenant_id: record.tenant_id.clone(),
            owner_user_id: record.owner_user_id.clone(),
        },
        updated_at: Reverse(record.updated_at.clone()),
        tag_id: Reverse(record.tag_id.clone()),
    }
}

fn tag_record_to_postgres(auth: &AppContext, record: &ContactTagRecord) -> PostgresContactTagRecord {
    PostgresContactTagRecord {
        tenant_id: record.tenant_id.clone(),
        organization_id: auth.organization_id.clone(),
        owner_user_id: record.owner_user_id.clone(),
        tag_id: contact_tag_id_to_i64(record.tag_id.as_str()),
        name: record.name.clone(),
        color: record.color.clone(),
        count: record.count,
        bg: record.bg.clone(),
        border: record.border.clone(),
        created_at: record.created_at.clone(),
        updated_at: record.updated_at.clone(),
    }
}

fn tag_record_from_postgres(record: PostgresContactTagRecord) -> ContactTagRecord {
    ContactTagRecord {
        tenant_id: record.tenant_id,
        owner_user_id: record.owner_user_id,
        tag_id: record.tag_id.to_string(),
        name: record.name,
        color: record.color,
        count: record.count,
        bg: record.bg,
        border: record.border,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}

fn preferences_record_to_postgres(
    auth: &AppContext,
    record: &ContactPreferencesRecord,
) -> PostgresContactPreferencesRecord {
    PostgresContactPreferencesRecord {
        tenant_id: record.tenant_id.clone(),
        organization_id: auth.organization_id.clone(),
        owner_user_id: record.owner_user_id.clone(),
        target_user_id: record.target_user_id.clone(),
        is_starred: record.is_starred,
        is_blocked: record.is_blocked,
        remark: if record.remark.is_empty() {
            None
        } else {
            Some(record.remark.clone())
        },
        updated_at: record.updated_at.clone(),
    }
}

fn preferences_record_from_postgres(record: PostgresContactPreferencesRecord) -> ContactPreferencesRecord {
    ContactPreferencesRecord {
        tenant_id: record.tenant_id,
        owner_user_id: record.owner_user_id,
        target_user_id: record.target_user_id,
        is_starred: record.is_starred,
        is_blocked: record.is_blocked,
        remark: record.remark.unwrap_or_default(),
        updated_at: record.updated_at,
    }
}

fn recommendation_record_to_postgres(
    auth: &AppContext,
    record: &ContactRecommendationRecord,
) -> PostgresContactRecommendationRecord {
    PostgresContactRecommendationRecord {
        tenant_id: record.tenant_id.clone(),
        organization_id: auth.organization_id.clone(),
        owner_user_id: record.owner_user_id.clone(),
        target_user_id: record.target_user_id.clone(),
        recommendation_id: contact_recommendation_id_to_i64(record.recommendation_id.as_str()),
        target_conversation_id: record.target_conversation_id.clone(),
        created_at: record.created_at.clone(),
    }
}

fn running_under_rust_test_harness() -> bool {
    std::env::var("RUST_TEST_THREADS").is_ok()
}

fn allows_contact_memory_fallback() -> bool {
    cfg!(test)
        || running_under_rust_test_harness()
        || matches!(
            resolve_web_environment_from_process_env(),
            WebEnvironment::Dev | WebEnvironment::Test
        )
}

fn ensure_contact_memory_fallback_allowed(
    store: Option<&Arc<dyn ContactStore>>,
) -> Result<(), SocialServiceError> {
    if store.is_some() || allows_contact_memory_fallback() {
        return Ok(());
    }
    Err(SocialServiceError::dependency_unavailable(
        "contact_store_unavailable",
        "contact open-api durable postgres store is required in production",
    ))
}

fn store_lock_error() -> SocialServiceError {
    SocialServiceError::dependency_unavailable(
        "contact_store_unavailable",
        "contact open-api store lock failed",
    )
}

fn store_unavailable(operation: &str) -> SocialServiceError {
    SocialServiceError::dependency_unavailable(
        "contact_store_unavailable",
        format!("contact postgres store unavailable during {operation}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use crate::friendship::SocialServiceError;

    #[test]
    fn contact_store_unavailable_returns_service_unavailable() {
        let response = SocialServiceError::dependency_unavailable(
            "contact_store_unavailable",
            "contact postgres store unavailable during test",
        )
        .into_response();
        assert_eq!(response.status(), axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn create_contact_recommendation_rejects_self_target_in_memory_store() {
        let auth = AppContext {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            user_id: "9001".into(),
            session_id: None,
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            actor_id: "9001".into(),
            actor_kind: "user".into(),
            device_id: None,
        };
        let error = create_contact_recommendation(
            None,
            &auth,
            "9001",
            "rec_1",
            None,
        )
        .expect_err("self recommendation should fail");
        assert!(format!("{error:?}").contains("contact_recommendation_self_target"));
    }

    #[test]
    fn create_contact_recommendation_persists_in_memory_store() {
        let auth = AppContext {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            user_id: "9001".into(),
            session_id: None,
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            actor_id: "9001".into(),
            actor_kind: "user".into(),
            device_id: None,
        };
        let record = create_contact_recommendation(
            None,
            &auth,
            "9002",
            "rec_2",
            Some("c_direct_001".into()),
        )
        .expect("recommendation should persist");
        assert_eq!(record.target_user_id, "9002");
        assert_eq!(
            record.target_conversation_id.as_deref(),
            Some("c_direct_001")
        );
    }
}
