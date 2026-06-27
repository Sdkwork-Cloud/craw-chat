use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, MutexGuard};

use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

pub const PROVIDER_REGISTRY_INTERFACE_VERSION: &str = "provider-registry/v1";
const PROVIDER_POLICY_MAX_TENANT_ID_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderDomain {
    Rtc,
    ObjectStorage,
    PrincipalProfile,
}

impl ProviderDomain {
    pub const ALL: [Self; 3] = [Self::Rtc, Self::ObjectStorage, Self::PrincipalProfile];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rtc => "rtc",
            Self::ObjectStorage => "object-storage",
            Self::PrincipalProfile => "principal-profile",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPluginDescriptor {
    pub plugin_id: String,
    pub domain: ProviderDomain,
    pub provider_kind: String,
    pub display_name: String,
    pub interface_version: String,
    pub config_schema_ref: String,
    pub default_selected: bool,
    pub tenant_override_allowed: bool,
    pub required_capabilities: Vec<String>,
    pub optional_capabilities: Vec<String>,
    pub unsupported_features: Vec<String>,
    pub degraded_behaviors: Vec<String>,
}

impl ProviderPluginDescriptor {
    pub fn new(
        plugin_id: impl Into<String>,
        domain: ProviderDomain,
        provider_kind: impl Into<String>,
        display_name: impl Into<String>,
    ) -> Self {
        let plugin_id = plugin_id.into();
        Self {
            config_schema_ref: format!("providers/{plugin_id}.schema.json"),
            plugin_id,
            domain,
            provider_kind: provider_kind.into(),
            display_name: display_name.into(),
            interface_version: "v1".into(),
            default_selected: false,
            tenant_override_allowed: true,
            required_capabilities: Vec::new(),
            optional_capabilities: Vec::new(),
            unsupported_features: Vec::new(),
            degraded_behaviors: Vec::new(),
        }
    }

    pub fn with_default_selected(mut self, default_selected: bool) -> Self {
        self.default_selected = default_selected;
        self
    }

    pub fn with_required_capabilities<I, S>(mut self, capabilities: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.required_capabilities = capabilities.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_optional_capabilities<I, S>(mut self, capabilities: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.optional_capabilities = capabilities.into_iter().map(Into::into).collect();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderHealthSnapshot {
    pub plugin_id: String,
    pub status: String,
    pub checked_at: String,
    pub details: BTreeMap<String, String>,
}

impl ProviderHealthSnapshot {
    pub fn healthy(plugin_id: impl Into<String>, checked_at: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            status: "healthy".into(),
            checked_at: checked_at.into(),
            details: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectiveProviderBinding {
    pub domain: ProviderDomain,
    pub default_plugin_id: Option<String>,
    pub selected_plugin_id: Option<String>,
    pub selection_source: String,
    pub tenant_override_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRegistrySnapshot {
    pub interface_version: String,
    pub plugins: Vec<ProviderPluginDescriptor>,
    pub effective_bindings: Vec<EffectiveProviderBinding>,
    pub precedence: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPolicySelection {
    pub domain: ProviderDomain,
    pub plugin_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantProviderPolicySelection {
    pub tenant_id: String,
    pub bindings: Vec<ProviderPolicySelection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPolicySnapshot {
    pub version: u64,
    pub recorded_at: String,
    pub rollback_from_version: Option<u64>,
    pub deployment_profiles: Vec<ProviderPolicySelection>,
    pub tenant_overrides: Vec<TenantProviderPolicySelection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPolicyHistory {
    pub current_version: u64,
    pub items: Vec<ProviderPolicySnapshot>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderPolicyChangeKind {
    Added,
    Removed,
    Changed,
}

impl ProviderPolicyChangeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPolicyChange {
    pub domain: ProviderDomain,
    pub change_kind: ProviderPolicyChangeKind,
    pub from_plugin_id: Option<String>,
    pub to_plugin_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantProviderPolicyChange {
    pub tenant_id: String,
    pub domain: ProviderDomain,
    pub change_kind: ProviderPolicyChangeKind,
    pub from_plugin_id: Option<String>,
    pub to_plugin_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPolicyDiff {
    pub from_version: u64,
    pub to_version: u64,
    pub from_recorded_at: String,
    pub to_recorded_at: String,
    pub deployment_profile_changes: Vec<ProviderPolicyChange>,
    pub tenant_override_changes: Vec<TenantProviderPolicyChange>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderPolicyResultStatus {
    Preview,
    Applied,
    Noop,
}

impl ProviderPolicyResultStatus {
    pub fn from_applied(applied: bool) -> Self {
        if applied { Self::Applied } else { Self::Noop }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPolicyPreview {
    pub status: ProviderPolicyResultStatus,
    pub base_version: u64,
    pub preview_version: u64,
    pub tenant_id: Option<String>,
    pub preview_binding: EffectiveProviderBinding,
    pub diff: ProviderPolicyDiff,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderPolicyCommit {
    pub status: ProviderPolicyResultStatus,
    pub applied: bool,
    pub current_version: u64,
    pub tenant_id: Option<String>,
    pub committed_binding: EffectiveProviderBinding,
    pub diff: ProviderPolicyDiff,
}

pub trait ProviderRegistry: Send + Sync {
    fn snapshot(&self) -> ProviderRegistrySnapshot;
    fn plugins_for_domain(&self, domain: ProviderDomain) -> Vec<ProviderPluginDescriptor>;
    fn effective_binding(
        &self,
        domain: ProviderDomain,
        tenant_id: Option<&str>,
    ) -> Option<EffectiveProviderBinding>;
}

#[derive(Clone, Debug, Default)]
pub struct StaticProviderRegistry {
    plugins: BTreeMap<String, ProviderPluginDescriptor>,
    defaults: BTreeMap<ProviderDomain, String>,
    deployment_profiles: BTreeMap<ProviderDomain, String>,
    tenant_overrides: BTreeMap<String, BTreeMap<ProviderDomain, String>>,
}

impl StaticProviderRegistry {
    pub fn new<I>(plugins: I) -> Self
    where
        I: IntoIterator<Item = ProviderPluginDescriptor>,
    {
        let mut registry = Self::default();
        for plugin in plugins {
            if plugin.default_selected {
                registry
                    .defaults
                    .insert(plugin.domain, plugin.plugin_id.clone());
            }
            registry.plugins.insert(plugin.plugin_id.clone(), plugin);
        }
        registry
    }

    pub fn platform_default() -> Self {
        Self::new([
            ProviderPluginDescriptor::new(
                "rtc-volcengine",
                ProviderDomain::Rtc,
                "volcengine",
                "火山引擎",
            )
            .with_default_selected(true)
            .with_required_capabilities([
                "session",
                "credential",
                "provider.webhook",
                "health",
            ]),
            ProviderPluginDescriptor::new("rtc-aliyun", ProviderDomain::Rtc, "aliyun", "阿里云")
                .with_required_capabilities([
                    "session",
                    "credential",
                    "provider.webhook",
                    "health",
                ]),
            ProviderPluginDescriptor::new("rtc-tencent", ProviderDomain::Rtc, "tencent", "腾讯云")
                .with_required_capabilities([
                    "session",
                    "credential",
                    "provider.webhook",
                    "health",
                ]),
            ProviderPluginDescriptor::new(
                "object-storage-aliyun",
                ProviderDomain::ObjectStorage,
                "aliyun",
                "阿里云",
            )
            .with_required_capabilities(["s3", "presign", "multipart"]),
            ProviderPluginDescriptor::new(
                "object-storage-tencent",
                ProviderDomain::ObjectStorage,
                "tencent",
                "腾讯云",
            )
            .with_required_capabilities(["s3", "presign", "multipart"]),
            ProviderPluginDescriptor::new(
                "object-storage-volcengine",
                ProviderDomain::ObjectStorage,
                "volcengine",
                "火山引擎",
            )
            .with_required_capabilities(["s3", "presign", "multipart"]),
            ProviderPluginDescriptor::new(
                "object-storage-aws",
                ProviderDomain::ObjectStorage,
                "aws",
                "Amazon Web Services",
            )
            .with_required_capabilities(["s3", "presign", "multipart"]),
            ProviderPluginDescriptor::new(
                "object-storage-google",
                ProviderDomain::ObjectStorage,
                "google",
                "Google",
            )
            .with_required_capabilities(["s3-gateway", "presign"]),
            ProviderPluginDescriptor::new(
                "object-storage-microsoft",
                ProviderDomain::ObjectStorage,
                "microsoft",
                "Microsoft",
            )
            .with_required_capabilities(["s3-gateway", "presign"]),
            ProviderPluginDescriptor::new(
                "principal-profile-upstream-context",
                ProviderDomain::PrincipalProfile,
                "upstream-context",
                "本地实现",
            )
            .with_default_selected(true)
            .with_required_capabilities(["read", "profile"]),
            ProviderPluginDescriptor::new(
                "principal-profile-external-catalog",
                ProviderDomain::PrincipalProfile,
                "external-catalog",
                "外部系统集成",
            )
            .with_required_capabilities(["read", "profile", "external-mapping"]),
        ])
    }

    pub fn with_tenant_override(
        mut self,
        tenant_id: impl Into<String>,
        domain: ProviderDomain,
        plugin_id: impl Into<String>,
    ) -> Self {
        self.tenant_overrides
            .entry(tenant_id.into())
            .or_default()
            .insert(domain, plugin_id.into());
        self
    }

    pub fn with_deployment_profile(
        mut self,
        domain: ProviderDomain,
        plugin_id: impl Into<String>,
    ) -> Self {
        self.deployment_profiles.insert(domain, plugin_id.into());
        self
    }

    fn plugin_matches_domain(&self, plugin_id: &str, domain: ProviderDomain) -> bool {
        self.plugins
            .get(plugin_id)
            .is_some_and(|plugin| plugin.domain == domain)
    }

    fn default_binding_for(&self, domain: ProviderDomain) -> EffectiveProviderBinding {
        let default_plugin_id = self.defaults.get(&domain).cloned();
        EffectiveProviderBinding {
            domain,
            default_plugin_id: default_plugin_id.clone(),
            selected_plugin_id: default_plugin_id,
            selection_source: if self.defaults.contains_key(&domain) {
                "global_default".into()
            } else {
                "deployment_required".into()
            },
            tenant_override_allowed: true,
        }
    }

    fn deployment_profile_binding_for(
        &self,
        domain: ProviderDomain,
        plugin_id: String,
    ) -> EffectiveProviderBinding {
        let tenant_override_allowed = self
            .plugins
            .get(plugin_id.as_str())
            .map(|plugin| plugin.tenant_override_allowed)
            .unwrap_or(true);
        EffectiveProviderBinding {
            domain,
            default_plugin_id: self.defaults.get(&domain).cloned(),
            selected_plugin_id: Some(plugin_id),
            selection_source: "deployment_profile".into(),
            tenant_override_allowed,
        }
    }

    pub fn into_runtime(self) -> RuntimeProviderRegistry {
        let mut state = RuntimeProviderPolicyState {
            deployment_profiles: self.deployment_profiles,
            tenant_overrides: self.tenant_overrides,
            history: Vec::new(),
            next_version: 1,
        };
        state.record_snapshot(None);
        RuntimeProviderRegistry {
            plugins: self.plugins,
            defaults: self.defaults,
            state: Mutex::new(state),
        }
    }
}

impl ProviderRegistry for StaticProviderRegistry {
    fn snapshot(&self) -> ProviderRegistrySnapshot {
        ProviderRegistrySnapshot {
            interface_version: PROVIDER_REGISTRY_INTERFACE_VERSION.into(),
            plugins: self.plugins.values().cloned().collect(),
            effective_bindings: ProviderDomain::ALL
                .into_iter()
                .filter_map(|domain| self.effective_binding(domain, None))
                .collect(),
            precedence: vec![
                "tenant_override".into(),
                "deployment_profile".into(),
                "global_default".into(),
            ],
        }
    }

    fn plugins_for_domain(&self, domain: ProviderDomain) -> Vec<ProviderPluginDescriptor> {
        self.plugins
            .values()
            .filter(|plugin| plugin.domain == domain)
            .cloned()
            .collect()
    }

    fn effective_binding(
        &self,
        domain: ProviderDomain,
        tenant_id: Option<&str>,
    ) -> Option<EffectiveProviderBinding> {
        if let Some(tenant_id) = tenant_id
            && let Some(plugin_id) = self
                .tenant_overrides
                .get(tenant_id)
                .and_then(|overrides| overrides.get(&domain))
                .cloned()
            && self.plugin_matches_domain(plugin_id.as_str(), domain)
        {
            let tenant_override_allowed = self
                .plugins
                .get(plugin_id.as_str())
                .map(|plugin| plugin.tenant_override_allowed)
                .unwrap_or(true);
            return Some(EffectiveProviderBinding {
                domain,
                default_plugin_id: self.defaults.get(&domain).cloned(),
                selected_plugin_id: Some(plugin_id),
                selection_source: "tenant_override".into(),
                tenant_override_allowed,
            });
        }

        if let Some(plugin_id) = self.deployment_profiles.get(&domain).cloned()
            && self.plugin_matches_domain(plugin_id.as_str(), domain)
        {
            return Some(self.deployment_profile_binding_for(domain, plugin_id));
        }

        Some(self.default_binding_for(domain))
    }
}

#[derive(Debug, Default)]
pub struct RuntimeProviderRegistry {
    plugins: BTreeMap<String, ProviderPluginDescriptor>,
    defaults: BTreeMap<ProviderDomain, String>,
    state: Mutex<RuntimeProviderPolicyState>,
}

#[derive(Clone, Debug, Default)]
struct RuntimeProviderPolicyState {
    deployment_profiles: BTreeMap<ProviderDomain, String>,
    tenant_overrides: BTreeMap<String, BTreeMap<ProviderDomain, String>>,
    history: Vec<ProviderPolicySnapshot>,
    next_version: u64,
}

fn lock_provider_registry_state<'a>(
    state: &'a Mutex<RuntimeProviderPolicyState>,
) -> MutexGuard<'a, RuntimeProviderPolicyState> {
    match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warning: recovering poisoned mutex in im-platform-contracts/provider-state");
            poisoned.into_inner()
        }
    }
}

fn lock_provider_registry_state_mut(
    state: &mut Mutex<RuntimeProviderPolicyState>,
) -> &mut RuntimeProviderPolicyState {
    match state.get_mut() {
        Ok(value) => value,
        Err(poisoned) => {
            eprintln!(
                "warning: recovering poisoned mutable mutex in im-platform-contracts/provider-state"
            );
            poisoned.into_inner()
        }
    }
}

impl RuntimeProviderPolicyState {
    fn current_version(&self) -> u64 {
        self.history
            .last()
            .map(|snapshot| snapshot.version)
            .unwrap_or(0)
    }

    fn ensure_expected_base_version(
        &self,
        expected_base_version: Option<u64>,
    ) -> Result<(), ContractError> {
        let Some(expected_base_version) = expected_base_version else {
            return Ok(());
        };
        let current_version = self.current_version();
        if current_version == expected_base_version {
            return Ok(());
        }

        Err(ContractError::Conflict(format!(
            "provider policy version drift: expected {expected_base_version}, current {current_version}"
        )))
    }

    fn record_snapshot(&mut self, rollback_from_version: Option<u64>) -> ProviderPolicySnapshot {
        let snapshot = self.compose_snapshot(
            self.next_version,
            utc_now_rfc3339_millis(),
            rollback_from_version,
        );
        self.history.push(snapshot.clone());
        self.next_version += 1;
        snapshot
    }

    fn compose_snapshot(
        &self,
        version: u64,
        recorded_at: String,
        rollback_from_version: Option<u64>,
    ) -> ProviderPolicySnapshot {
        ProviderPolicySnapshot {
            version,
            recorded_at,
            rollback_from_version,
            deployment_profiles: self
                .deployment_profiles
                .iter()
                .map(|(domain, plugin_id)| ProviderPolicySelection {
                    domain: *domain,
                    plugin_id: plugin_id.clone(),
                })
                .collect(),
            tenant_overrides: self
                .tenant_overrides
                .iter()
                .map(|(tenant_id, bindings)| TenantProviderPolicySelection {
                    tenant_id: tenant_id.clone(),
                    bindings: bindings
                        .iter()
                        .map(|(domain, plugin_id)| ProviderPolicySelection {
                            domain: *domain,
                            plugin_id: plugin_id.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    fn reset_history_to_current_state(&mut self) {
        self.history.clear();
        self.next_version = 1;
        self.record_snapshot(None);
    }

    fn restore_from_snapshot(&mut self, snapshot: &ProviderPolicySnapshot) {
        self.deployment_profiles = snapshot
            .deployment_profiles
            .iter()
            .map(|entry| (entry.domain, entry.plugin_id.clone()))
            .collect();
        self.tenant_overrides = snapshot
            .tenant_overrides
            .iter()
            .map(|entry| {
                (
                    entry.tenant_id.clone(),
                    entry
                        .bindings
                        .iter()
                        .map(|binding| (binding.domain, binding.plugin_id.clone()))
                        .collect(),
                )
            })
            .collect();
    }
}

impl RuntimeProviderRegistry {
    pub fn new<I>(plugins: I) -> Self
    where
        I: IntoIterator<Item = ProviderPluginDescriptor>,
    {
        StaticProviderRegistry::new(plugins).into_runtime()
    }

    pub fn platform_default() -> Self {
        StaticProviderRegistry::platform_default().into_runtime()
    }

    fn validate_tenant_override_id(tenant_id: Option<&str>) -> Result<(), ContractError> {
        if let Some(tenant_id) = tenant_id {
            if tenant_id.trim().is_empty() {
                return Err(ContractError::UnsupportedCapability(
                    "tenantId cannot be empty".into(),
                ));
            }
            if tenant_id.len() > PROVIDER_POLICY_MAX_TENANT_ID_BYTES {
                return Err(ContractError::UnsupportedCapability(format!(
                    "tenantId must be at most {PROVIDER_POLICY_MAX_TENANT_ID_BYTES} bytes",
                )));
            }
        }

        Ok(())
    }

    pub fn with_tenant_override(
        mut self,
        tenant_id: impl Into<String>,
        domain: ProviderDomain,
        plugin_id: impl Into<String>,
    ) -> Self {
        let tenant_id = tenant_id.into();
        let plugin_id = plugin_id.into();
        lock_provider_registry_state_mut(&mut self.state)
            .tenant_overrides
            .entry(tenant_id)
            .or_default()
            .insert(domain, plugin_id);
        lock_provider_registry_state_mut(&mut self.state).reset_history_to_current_state();
        self
    }

    pub fn with_deployment_profile(
        mut self,
        domain: ProviderDomain,
        plugin_id: impl Into<String>,
    ) -> Self {
        lock_provider_registry_state_mut(&mut self.state)
            .deployment_profiles
            .insert(domain, plugin_id.into());
        lock_provider_registry_state_mut(&mut self.state).reset_history_to_current_state();
        self
    }

    pub fn set_deployment_profile(
        &self,
        domain: ProviderDomain,
        plugin_id: &str,
    ) -> Result<EffectiveProviderBinding, ContractError> {
        self.set_deployment_profile_with_expected_version(domain, plugin_id, None)
    }

    pub fn set_deployment_profile_with_expected_version(
        &self,
        domain: ProviderDomain,
        plugin_id: &str,
        expected_base_version: Option<u64>,
    ) -> Result<EffectiveProviderBinding, ContractError> {
        Ok(self
            .commit_upsert(None, domain, plugin_id, expected_base_version)?
            .committed_binding)
    }

    pub fn set_tenant_override(
        &self,
        tenant_id: &str,
        domain: ProviderDomain,
        plugin_id: &str,
    ) -> Result<EffectiveProviderBinding, ContractError> {
        self.set_tenant_override_with_expected_version(tenant_id, domain, plugin_id, None)
    }

    pub fn set_tenant_override_with_expected_version(
        &self,
        tenant_id: &str,
        domain: ProviderDomain,
        plugin_id: &str,
        expected_base_version: Option<u64>,
    ) -> Result<EffectiveProviderBinding, ContractError> {
        Ok(self
            .commit_upsert(Some(tenant_id), domain, plugin_id, expected_base_version)?
            .committed_binding)
    }

    pub fn commit_upsert(
        &self,
        tenant_id: Option<&str>,
        domain: ProviderDomain,
        plugin_id: &str,
        expected_base_version: Option<u64>,
    ) -> Result<ProviderPolicyCommit, ContractError> {
        Self::validate_tenant_override_id(tenant_id)?;
        self.ensure_valid_plugin_for_domain(plugin_id, domain, tenant_id.is_some())?;
        let mut state = lock_provider_registry_state(&self.state);
        state.ensure_expected_base_version(expected_base_version)?;
        let base_snapshot = state.history.last().cloned().ok_or_else(|| {
            ContractError::Unavailable("provider policy history is not initialized".into())
        })?;
        let tenant_id = tenant_id.map(str::to_owned);
        let applied = if let Some(tenant_id) = tenant_id.as_deref() {
            match state
                .tenant_overrides
                .get(tenant_id)
                .and_then(|bindings| bindings.get(&domain))
            {
                Some(current_plugin_id) if current_plugin_id == plugin_id => false,
                _ => {
                    state
                        .tenant_overrides
                        .entry(tenant_id.into())
                        .or_default()
                        .insert(domain, plugin_id.into());
                    true
                }
            }
        } else {
            match state.deployment_profiles.get(&domain) {
                Some(current_plugin_id) if current_plugin_id == plugin_id => false,
                _ => {
                    state.deployment_profiles.insert(domain, plugin_id.into());
                    true
                }
            }
        };
        let committed_snapshot = if applied {
            state.record_snapshot(None)
        } else {
            base_snapshot.clone()
        };
        let committed_binding = if tenant_id.is_some() {
            self.tenant_override_binding_for(domain, plugin_id.into())
        } else {
            self.deployment_profile_binding_for(domain, plugin_id.into())
        };

        Ok(ProviderPolicyCommit {
            status: ProviderPolicyResultStatus::from_applied(applied),
            applied,
            current_version: committed_snapshot.version,
            tenant_id,
            committed_binding,
            diff: provider_policy_diff(&base_snapshot, &committed_snapshot),
        })
    }

    pub fn policy_history(&self) -> ProviderPolicyHistory {
        let state = lock_provider_registry_state(&self.state);
        ProviderPolicyHistory {
            current_version: state
                .history
                .last()
                .map(|snapshot| snapshot.version)
                .unwrap_or(0),
            items: state.history.clone(),
        }
    }

    pub fn diff_versions(
        &self,
        from_version: u64,
        to_version: u64,
    ) -> Result<ProviderPolicyDiff, ContractError> {
        if from_version > to_version {
            return Err(ContractError::UnsupportedCapability(
                "fromVersion must not exceed toVersion".into(),
            ));
        }

        let state = lock_provider_registry_state(&self.state);
        let from_snapshot = state
            .history
            .iter()
            .find(|snapshot| snapshot.version == from_version)
            .ok_or_else(|| {
                ContractError::Conflict(format!("unknown provider policy version: {from_version}"))
            })?;
        let to_snapshot = state
            .history
            .iter()
            .find(|snapshot| snapshot.version == to_version)
            .ok_or_else(|| {
                ContractError::Conflict(format!("unknown provider policy version: {to_version}"))
            })?;

        Ok(provider_policy_diff(from_snapshot, to_snapshot))
    }

    pub fn preview_upsert(
        &self,
        tenant_id: Option<&str>,
        domain: ProviderDomain,
        plugin_id: &str,
    ) -> Result<ProviderPolicyPreview, ContractError> {
        Self::validate_tenant_override_id(tenant_id)?;
        self.ensure_valid_plugin_for_domain(plugin_id, domain, tenant_id.is_some())?;
        let state = lock_provider_registry_state(&self.state);
        let base_snapshot = state.history.last().cloned().ok_or_else(|| {
            ContractError::Unavailable("provider policy history is not initialized".into())
        })?;
        let tenant_id = tenant_id.map(str::to_owned);
        let mut preview_state = state.clone();
        if let Some(tenant_id) = tenant_id.as_deref() {
            preview_state
                .tenant_overrides
                .entry(tenant_id.into())
                .or_default()
                .insert(domain, plugin_id.into());
        } else {
            preview_state
                .deployment_profiles
                .insert(domain, plugin_id.into());
        }
        let preview_snapshot =
            preview_state.compose_snapshot(state.next_version, utc_now_rfc3339_millis(), None);
        let preview_binding = if tenant_id.is_some() {
            self.tenant_override_binding_for(domain, plugin_id.into())
        } else {
            self.deployment_profile_binding_for(domain, plugin_id.into())
        };

        Ok(ProviderPolicyPreview {
            status: ProviderPolicyResultStatus::Preview,
            base_version: base_snapshot.version,
            preview_version: preview_snapshot.version,
            tenant_id,
            preview_binding,
            diff: provider_policy_diff(&base_snapshot, &preview_snapshot),
        })
    }

    pub fn rollback_to(
        &self,
        target_version: u64,
    ) -> Result<ProviderPolicySnapshot, ContractError> {
        let mut state = lock_provider_registry_state(&self.state);
        let Some(target_snapshot) = state
            .history
            .iter()
            .find(|snapshot| snapshot.version == target_version)
            .cloned()
        else {
            return Err(ContractError::Conflict(format!(
                "unknown provider policy version: {target_version}"
            )));
        };
        state.restore_from_snapshot(&target_snapshot);
        Ok(state.record_snapshot(Some(target_version)))
    }

    pub fn tenant_ids_with_overrides(&self) -> Vec<String> {
        lock_provider_registry_state(&self.state)
            .tenant_overrides
            .keys()
            .cloned()
            .collect()
    }

    fn ensure_valid_plugin_for_domain(
        &self,
        plugin_id: &str,
        domain: ProviderDomain,
        require_tenant_override_allowed: bool,
    ) -> Result<(), ContractError> {
        let Some(plugin) = self.plugins.get(plugin_id) else {
            return Err(ContractError::UnsupportedCapability(format!(
                "unknown provider plugin: {plugin_id}"
            )));
        };
        if plugin.domain != domain {
            return Err(ContractError::UnsupportedCapability(format!(
                "provider plugin {plugin_id} does not belong to domain {}",
                domain.as_str()
            )));
        }
        if require_tenant_override_allowed && !plugin.tenant_override_allowed {
            return Err(ContractError::Conflict(format!(
                "tenant override is not allowed for provider plugin: {plugin_id}"
            )));
        }
        Ok(())
    }

    fn default_binding_for(&self, domain: ProviderDomain) -> EffectiveProviderBinding {
        let default_plugin_id = self.defaults.get(&domain).cloned();
        EffectiveProviderBinding {
            domain,
            default_plugin_id: default_plugin_id.clone(),
            selected_plugin_id: default_plugin_id,
            selection_source: if self.defaults.contains_key(&domain) {
                "global_default".into()
            } else {
                "deployment_required".into()
            },
            tenant_override_allowed: true,
        }
    }

    fn deployment_profile_binding_for(
        &self,
        domain: ProviderDomain,
        plugin_id: String,
    ) -> EffectiveProviderBinding {
        let tenant_override_allowed = self
            .plugins
            .get(plugin_id.as_str())
            .map(|plugin| plugin.tenant_override_allowed)
            .unwrap_or(true);
        EffectiveProviderBinding {
            domain,
            default_plugin_id: self.defaults.get(&domain).cloned(),
            selected_plugin_id: Some(plugin_id),
            selection_source: "deployment_profile".into(),
            tenant_override_allowed,
        }
    }

    fn tenant_override_binding_for(
        &self,
        domain: ProviderDomain,
        plugin_id: String,
    ) -> EffectiveProviderBinding {
        let tenant_override_allowed = self
            .plugins
            .get(plugin_id.as_str())
            .map(|plugin| plugin.tenant_override_allowed)
            .unwrap_or(true);
        EffectiveProviderBinding {
            domain,
            default_plugin_id: self.defaults.get(&domain).cloned(),
            selected_plugin_id: Some(plugin_id),
            selection_source: "tenant_override".into(),
            tenant_override_allowed,
        }
    }
}

fn provider_policy_diff(
    from_snapshot: &ProviderPolicySnapshot,
    to_snapshot: &ProviderPolicySnapshot,
) -> ProviderPolicyDiff {
    let from_deployment_profiles: BTreeMap<ProviderDomain, String> = from_snapshot
        .deployment_profiles
        .iter()
        .map(|entry| (entry.domain, entry.plugin_id.clone()))
        .collect();
    let to_deployment_profiles: BTreeMap<ProviderDomain, String> = to_snapshot
        .deployment_profiles
        .iter()
        .map(|entry| (entry.domain, entry.plugin_id.clone()))
        .collect();

    let from_tenant_overrides: BTreeMap<String, BTreeMap<ProviderDomain, String>> = from_snapshot
        .tenant_overrides
        .iter()
        .map(|entry| {
            (
                entry.tenant_id.clone(),
                entry
                    .bindings
                    .iter()
                    .map(|binding| (binding.domain, binding.plugin_id.clone()))
                    .collect(),
            )
        })
        .collect();
    let to_tenant_overrides: BTreeMap<String, BTreeMap<ProviderDomain, String>> = to_snapshot
        .tenant_overrides
        .iter()
        .map(|entry| {
            (
                entry.tenant_id.clone(),
                entry
                    .bindings
                    .iter()
                    .map(|binding| (binding.domain, binding.plugin_id.clone()))
                    .collect(),
            )
        })
        .collect();

    let deployment_profile_changes = ProviderDomain::ALL
        .into_iter()
        .filter_map(|domain| {
            let from_plugin_id = from_deployment_profiles.get(&domain).cloned();
            let to_plugin_id = to_deployment_profiles.get(&domain).cloned();
            provider_policy_change_kind(&from_plugin_id, &to_plugin_id).map(|change_kind| {
                ProviderPolicyChange {
                    domain,
                    change_kind,
                    from_plugin_id,
                    to_plugin_id,
                }
            })
        })
        .collect();

    let tenant_ids: BTreeSet<String> = from_tenant_overrides
        .keys()
        .chain(to_tenant_overrides.keys())
        .cloned()
        .collect();
    let mut tenant_override_changes = Vec::new();
    for tenant_id in tenant_ids {
        let from_bindings = from_tenant_overrides.get(&tenant_id);
        let to_bindings = to_tenant_overrides.get(&tenant_id);
        for domain in ProviderDomain::ALL {
            let from_plugin_id = from_bindings
                .and_then(|bindings| bindings.get(&domain))
                .cloned();
            let to_plugin_id = to_bindings
                .and_then(|bindings| bindings.get(&domain))
                .cloned();
            if let Some(change_kind) = provider_policy_change_kind(&from_plugin_id, &to_plugin_id) {
                tenant_override_changes.push(TenantProviderPolicyChange {
                    tenant_id: tenant_id.clone(),
                    domain,
                    change_kind,
                    from_plugin_id,
                    to_plugin_id,
                });
            }
        }
    }

    ProviderPolicyDiff {
        from_version: from_snapshot.version,
        to_version: to_snapshot.version,
        from_recorded_at: from_snapshot.recorded_at.clone(),
        to_recorded_at: to_snapshot.recorded_at.clone(),
        deployment_profile_changes,
        tenant_override_changes,
    }
}

fn provider_policy_change_kind(
    from_plugin_id: &Option<String>,
    to_plugin_id: &Option<String>,
) -> Option<ProviderPolicyChangeKind> {
    match (from_plugin_id.as_deref(), to_plugin_id.as_deref()) {
        (None, None) => None,
        (None, Some(_)) => Some(ProviderPolicyChangeKind::Added),
        (Some(_), None) => Some(ProviderPolicyChangeKind::Removed),
        (Some(from_plugin_id), Some(to_plugin_id)) if from_plugin_id != to_plugin_id => {
            Some(ProviderPolicyChangeKind::Changed)
        }
        (Some(_), Some(_)) => None,
    }
}

impl ProviderRegistry for RuntimeProviderRegistry {
    fn snapshot(&self) -> ProviderRegistrySnapshot {
        ProviderRegistrySnapshot {
            interface_version: PROVIDER_REGISTRY_INTERFACE_VERSION.into(),
            plugins: self.plugins.values().cloned().collect(),
            effective_bindings: ProviderDomain::ALL
                .into_iter()
                .filter_map(|domain| self.effective_binding(domain, None))
                .collect(),
            precedence: vec![
                "tenant_override".into(),
                "deployment_profile".into(),
                "global_default".into(),
            ],
        }
    }

    fn plugins_for_domain(&self, domain: ProviderDomain) -> Vec<ProviderPluginDescriptor> {
        self.plugins
            .values()
            .filter(|plugin| plugin.domain == domain)
            .cloned()
            .collect()
    }

    fn effective_binding(
        &self,
        domain: ProviderDomain,
        tenant_id: Option<&str>,
    ) -> Option<EffectiveProviderBinding> {
        let state = lock_provider_registry_state(&self.state);
        if let Some(tenant_id) = tenant_id
            && let Some(plugin_id) = state
                .tenant_overrides
                .get(tenant_id)
                .and_then(|overrides| overrides.get(&domain))
                .cloned()
            && self
                .plugins
                .get(plugin_id.as_str())
                .is_some_and(|plugin| plugin.domain == domain)
        {
            return Some(self.tenant_override_binding_for(domain, plugin_id));
        }

        if let Some(plugin_id) = state.deployment_profiles.get(&domain).cloned()
            && self
                .plugins
                .get(plugin_id.as_str())
                .is_some_and(|plugin| plugin.domain == domain)
        {
            return Some(self.deployment_profile_binding_for(domain, plugin_id));
        }

        Some(self.default_binding_for(domain))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStoragePutRequest {
    pub bucket: String,
    pub object_key: String,
    pub content_length: u64,
    pub content_type: Option<String>,
    pub storage_class: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStorageObjectDescriptor {
    pub bucket: String,
    pub object_key: String,
    pub content_length: u64,
    pub etag: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStorageDownloadUrlRequest {
    pub bucket: String,
    pub object_key: String,
    pub expires_in_seconds: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStorageUploadUrlRequest {
    pub bucket: String,
    pub object_key: String,
    pub expires_in_seconds: u32,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStorageUploadSession {
    pub method: String,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub expires_at: String,
}

pub trait ObjectStorageProvider: Send + Sync {
    fn descriptor(&self) -> ProviderPluginDescriptor;
    fn put_object(
        &self,
        request: ObjectStoragePutRequest,
    ) -> Result<ObjectStorageObjectDescriptor, ContractError>;
    fn signed_upload_url(
        &self,
        request: ObjectStorageUploadUrlRequest,
    ) -> Result<ObjectStorageUploadSession, ContractError>;
    fn signed_download_url(
        &self,
        request: ObjectStorageDownloadUrlRequest,
    ) -> Result<String, ContractError>;
    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrincipalProfile {
    pub tenant_id: String,
    pub principal_id: String,
    pub display_name: String,
    pub external_system: Option<String>,
    pub external_principal_id: Option<String>,
    pub attributes: BTreeMap<String, String>,
    pub inactive: bool,
}

pub trait PrincipalProfileProvider: Send + Sync {
    fn descriptor(&self) -> ProviderPluginDescriptor;
    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError>;
    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError>;
    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError>;
    fn map_external_principal(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError>;
    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_effective_binding_recovers_from_poisoned_provider_registry_state_lock() {
        let registry = RuntimeProviderRegistry::platform_default();
        poison_mutex(&registry.state);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            registry.effective_binding(ProviderDomain::Rtc, None)
        }));
        assert!(
            result.is_ok(),
            "effective_binding should not panic when provider registry state lock is poisoned"
        );
        let binding = result.expect("panic status should be captured");
        assert!(binding.is_some());
    }

    #[test]
    fn test_set_deployment_profile_recovers_from_poisoned_provider_registry_state_lock() {
        let registry = RuntimeProviderRegistry::platform_default();
        poison_mutex(&registry.state);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            registry.set_deployment_profile(ProviderDomain::Rtc, "rtc-aliyun")
        }));
        assert!(
            result.is_ok(),
            "set_deployment_profile should not panic when provider registry state lock is poisoned"
        );
        let commit_result = result.expect("panic status should be captured");
        assert!(commit_result.is_ok());
    }

    #[test]
    fn test_with_tenant_override_recovers_from_poisoned_mutable_provider_registry_state() {
        let registry = RuntimeProviderRegistry::platform_default();
        poison_mutex(&registry.state);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            registry.with_tenant_override("100001", ProviderDomain::Rtc, "rtc-aliyun")
        }));
        assert!(
            result.is_ok(),
            "with_tenant_override should not panic when provider registry mutable state is poisoned"
        );
        let registry = result.expect("panic status should be captured");
        let binding = registry
            .effective_binding(ProviderDomain::Rtc, Some("100001"))
            .expect("tenant override binding should exist");
        assert_eq!(binding.selection_source, "tenant_override");
        assert_eq!(binding.selected_plugin_id.as_deref(), Some("rtc-aliyun"));
    }
}
