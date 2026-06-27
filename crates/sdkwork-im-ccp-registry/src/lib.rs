use std::collections::{BTreeMap, BTreeSet};

use sdkwork_im_ccp_core::TransportBinding;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseStage {
    Experimental,
    Beta,
    Stable,
    Deprecated,
    Removed,
}

impl ReleaseStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Experimental => "experimental",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::Removed => "removed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseChannel {
    Stable,
    Canary,
    Emergency,
}

impl ReleaseChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Canary => "canary",
            Self::Emergency => "emergency",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaDescriptor {
    pub schema: String,
    pub kind: String,
    pub stage: ReleaseStage,
    pub binding_protocols: BTreeSet<String>,
    pub required_capabilities: BTreeSet<String>,
    pub supported_consumers: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientCompatibilityDescriptor {
    pub client_type: String,
    pub minimum_protocol_version: String,
    pub supported_bindings: BTreeSet<String>,
    pub supported_codecs: BTreeSet<String>,
    pub supported_capabilities: BTreeSet<String>,
    pub blocked_experimental_capabilities: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityProfile {
    pub profile_id: String,
    pub release_channel: ReleaseChannel,
    pub enabled_capabilities: BTreeSet<String>,
    pub experimental_capabilities: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuotaProfile {
    pub profile_id: String,
    pub max_concurrent_sessions_per_tenant: u32,
    pub max_subscriptions_per_session: u32,
    pub max_inflight_messages: u32,
    pub max_payload_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RolloutPolicy {
    pub policy_id: String,
    pub release_channel: ReleaseChannel,
    pub traffic_percent: u8,
    pub cell_selector: String,
    pub region_selector: String,
    pub operator_override: bool,
    pub tenant_allowlist: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KillSwitchRule {
    pub rule_id: String,
    pub active: bool,
    pub reason: String,
    pub disabled_capabilities: BTreeSet<String>,
    pub disabled_bindings: BTreeSet<String>,
    pub disabled_codecs: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectiveProtocolSnapshot {
    pub protocol_version: String,
    pub release_channel: ReleaseChannel,
    pub enabled_capabilities: BTreeSet<String>,
    pub allowed_bindings: BTreeSet<String>,
    pub allowed_codecs: BTreeSet<String>,
    pub quota_profile_id: String,
    pub kill_switch_active: bool,
    pub precedence: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusinessPolicyVocabulary {
    pub policy_version_field: String,
    pub capability_flags_field: String,
    pub history_visibility_field: String,
    pub history_visibility_modes: Vec<String>,
    pub retention_policy_ref_field: String,
    pub retention_policy_scopes: Vec<String>,
    pub retention_classes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolGovernanceSnapshot {
    pub capability_profile: CapabilityProfile,
    pub quota_profile: QuotaProfile,
    pub rollout_policy: RolloutPolicy,
    pub kill_switch: KillSwitchRule,
    pub effective_snapshot: EffectiveProtocolSnapshot,
    pub business_policy_vocabulary: BusinessPolicyVocabulary,
}

impl SchemaDescriptor {
    pub fn new<I, J, K, S, T>(
        schema: impl Into<String>,
        kind: impl Into<String>,
        bindings: I,
        required_capabilities: J,
        stage: ReleaseStage,
        supported_consumers: K,
    ) -> Self
    where
        I: IntoIterator<Item = TransportBinding>,
        J: IntoIterator<Item = S>,
        K: IntoIterator<Item = T>,
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            schema: schema.into(),
            kind: kind.into(),
            stage,
            binding_protocols: bindings
                .into_iter()
                .map(|binding| binding.protocol_id().to_string())
                .collect(),
            required_capabilities: required_capabilities.into_iter().map(Into::into).collect(),
            supported_consumers: supported_consumers.into_iter().map(Into::into).collect(),
        }
    }

    pub fn supports_binding(&self, binding: &TransportBinding) -> bool {
        self.binding_protocols.contains(binding.protocol_id())
    }

    pub fn requires_capability(&self, capability: &str) -> bool {
        self.required_capabilities.contains(capability)
    }
}

impl ClientCompatibilityDescriptor {
    pub fn new<I, J, K, L, S, T, U, V>(
        client_type: impl Into<String>,
        minimum_protocol_version: impl Into<String>,
        supported_bindings: I,
        supported_codecs: J,
        supported_capabilities: K,
        blocked_experimental_capabilities: L,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        J: IntoIterator<Item = T>,
        K: IntoIterator<Item = U>,
        L: IntoIterator<Item = V>,
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        Self {
            client_type: client_type.into(),
            minimum_protocol_version: minimum_protocol_version.into(),
            supported_bindings: supported_bindings.into_iter().map(Into::into).collect(),
            supported_codecs: supported_codecs.into_iter().map(Into::into).collect(),
            supported_capabilities: supported_capabilities.into_iter().map(Into::into).collect(),
            blocked_experimental_capabilities: blocked_experimental_capabilities
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl EffectiveProtocolSnapshot {
    fn from_registry_inputs(
        protocol_version: impl Into<String>,
        bindings: &BTreeSet<String>,
        codecs: &BTreeSet<String>,
        capability_profile: &CapabilityProfile,
        quota_profile: &QuotaProfile,
        rollout_policy: &RolloutPolicy,
        kill_switch: &KillSwitchRule,
    ) -> Self {
        let enabled_capabilities = capability_profile
            .enabled_capabilities
            .iter()
            .filter(|capability| {
                !kill_switch.active || !kill_switch.disabled_capabilities.contains(*capability)
            })
            .cloned()
            .collect();
        let allowed_bindings = bindings
            .iter()
            .filter(|binding| {
                !kill_switch.active || !kill_switch.disabled_bindings.contains(*binding)
            })
            .cloned()
            .collect();
        let allowed_codecs = codecs
            .iter()
            .filter(|codec| !kill_switch.active || !kill_switch.disabled_codecs.contains(*codec))
            .cloned()
            .collect();

        Self {
            protocol_version: protocol_version.into(),
            release_channel: rollout_policy.release_channel.clone(),
            enabled_capabilities,
            allowed_bindings,
            allowed_codecs,
            quota_profile_id: quota_profile.profile_id.clone(),
            kill_switch_active: kill_switch.active,
            precedence: vec![
                "emergency_kill_switch".into(),
                "operator_rollout_override".into(),
                "tenant_protocol_policy".into(),
                "cell_region_release_channel".into(),
                "global_stable_baseline".into(),
            ],
        }
    }
}

impl ProtocolGovernanceSnapshot {
    fn from_registry_inputs(inputs: ProtocolGovernanceSnapshotInputs<'_>) -> Self {
        let ProtocolGovernanceSnapshotInputs {
            protocol_version,
            bindings,
            codecs,
            capability_profile,
            quota_profile,
            rollout_policy,
            kill_switch,
            business_policy_vocabulary,
        } = inputs;
        let effective_snapshot = EffectiveProtocolSnapshot::from_registry_inputs(
            protocol_version,
            bindings,
            codecs,
            &capability_profile,
            &quota_profile,
            &rollout_policy,
            &kill_switch,
        );

        Self {
            capability_profile,
            quota_profile,
            rollout_policy,
            kill_switch,
            effective_snapshot,
            business_policy_vocabulary,
        }
    }
}

struct ProtocolGovernanceSnapshotInputs<'a> {
    protocol_version: String,
    bindings: &'a BTreeSet<String>,
    codecs: &'a BTreeSet<String>,
    capability_profile: CapabilityProfile,
    quota_profile: QuotaProfile,
    rollout_policy: RolloutPolicy,
    kill_switch: KillSwitchRule,
    business_policy_vocabulary: BusinessPolicyVocabulary,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CcpRegistry {
    protocol_version: String,
    bindings: BTreeSet<String>,
    codecs: BTreeSet<String>,
    capabilities: BTreeSet<String>,
    schemas: BTreeMap<String, SchemaDescriptor>,
    compatibility_matrix: BTreeMap<String, ClientCompatibilityDescriptor>,
    governance_snapshot: Option<ProtocolGovernanceSnapshot>,
}

impl CcpRegistry {
    pub fn new() -> Self {
        Self {
            protocol_version: "ccp/1.0".into(),
            ..Self::default()
        }
    }

    pub fn control_plane_v1() -> Self {
        let mut registry = Self::new();

        for binding in all_bindings() {
            registry.register_binding(binding);
        }
        registry.register_codec("json");
        registry.register_codec("cbor");

        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.hello",
            "hello",
            all_bindings(),
            ["control", "negotiation"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.hello_ack",
            "hello_ack",
            all_bindings(),
            ["control", "negotiation"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.auth_bind",
            "auth_bind",
            all_bindings(),
            ["control", "auth"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.auth_ok",
            "auth_ok",
            all_bindings(),
            ["control", "auth"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.session_resume",
            "session_resume",
            all_bindings(),
            ["control", "session"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.session_resumed",
            "session_resumed",
            all_bindings(),
            ["control", "session"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.heartbeat",
            "heartbeat",
            all_bindings(),
            ["control", "session"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.goaway",
            "goaway",
            all_bindings(),
            ["control"],
            ReleaseStage::Stable,
            all_consumers(),
        ));
        registry.register_schema(SchemaDescriptor::new(
            "ccp.control.error",
            "error",
            all_bindings(),
            ["control"],
            ReleaseStage::Stable,
            all_consumers(),
        ));

        registry.register_compatibility(ClientCompatibilityDescriptor::new(
            "web",
            "ccp/1.0",
            ["ccp/http/1", "ccp/ws/1", "ccp/sse/1"],
            ["json"],
            ["control", "negotiation", "auth", "session"],
            ["agent.tool_call", "device.signature"],
        ));
        registry.register_compatibility(ClientCompatibilityDescriptor::new(
            "desktop",
            "ccp/1.0",
            ["ccp/http/1", "ccp/ws/1", "ccp/sse/1"],
            ["json", "cbor"],
            ["control", "negotiation", "auth", "session"],
            ["agent.tool_call"],
        ));
        registry.register_compatibility(ClientCompatibilityDescriptor::new(
            "mobile",
            "ccp/1.0",
            ["ccp/http/1", "ccp/ws/1", "ccp/sse/1"],
            ["json", "cbor"],
            ["control", "negotiation", "auth", "session"],
            ["device.signature"],
        ));
        registry.register_compatibility(ClientCompatibilityDescriptor::new(
            "backend",
            "ccp/1.0",
            [
                "ccp/http/1",
                "ccp/ws/1",
                "ccp/sse/1",
                "ccp/mqtt/1",
                "ccp/tcp/1",
                "ccp/udp/1",
                "ccp/quic/1",
            ],
            ["json", "cbor"],
            ["control", "negotiation", "auth", "session"],
            std::iter::empty::<&str>(),
        ));
        registry.register_compatibility(ClientCompatibilityDescriptor::new(
            "iot-edge",
            "ccp/1.0",
            ["ccp/mqtt/1", "ccp/tcp/1", "ccp/udp/1", "ccp/quic/1"],
            ["json", "cbor"],
            ["control", "negotiation", "auth", "session", "device.signature"],
            std::iter::empty::<&str>(),
        ));

        for capability in [
            "payload.json",
            "payload.cbor",
            "realtime.pull",
            "realtime.push",
            "session.resume",
        ] {
            registry.register_capability(capability);
        }

        let capability_profile = CapabilityProfile {
            profile_id: "control-plane-stable".into(),
            release_channel: ReleaseChannel::Stable,
            enabled_capabilities: registry.capabilities.iter().cloned().collect(),
            experimental_capabilities: ["agent.tool_call", "device.signature"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
        };
        let quota_profile = QuotaProfile {
            profile_id: "control-plane-default".into(),
            max_concurrent_sessions_per_tenant: 20_000,
            max_subscriptions_per_session: 4_096,
            max_inflight_messages: 2_048,
            max_payload_bytes: 512 * 1024,
        };
        let rollout_policy = RolloutPolicy {
            policy_id: "control-plane-global-stable".into(),
            release_channel: ReleaseChannel::Stable,
            traffic_percent: 100,
            cell_selector: "cell-*".into(),
            region_selector: "region-*".into(),
            operator_override: false,
            tenant_allowlist: BTreeSet::new(),
        };
        let kill_switch = KillSwitchRule {
            rule_id: "pause-cbor-and-mqtt".into(),
            active: true,
            reason:
                "runtime readers must consume effective protocol snapshot before cbor/mqtt rollout"
                    .into(),
            disabled_capabilities: ["payload.cbor"].into_iter().map(str::to_owned).collect(),
            disabled_bindings: ["ccp/mqtt/1"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            disabled_codecs: ["cbor"].into_iter().map(str::to_owned).collect(),
        };
        let business_policy_vocabulary = BusinessPolicyVocabulary {
            policy_version_field: "policy_version".into(),
            capability_flags_field: "capability_flags".into(),
            history_visibility_field: "history_visibility".into(),
            history_visibility_modes: ["joined", "invited", "shared", "world_readable"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            retention_policy_ref_field: "retention_policy_ref".into(),
            retention_policy_scopes: ["tenant", "space", "group", "channel", "thread"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            retention_classes: ["ephemeral", "standard", "extended", "legal_hold"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
        };
        let protocol_version = registry.protocol_version.clone();
        let bindings = registry.bindings.clone();
        let codecs = registry.codecs.clone();
        registry.set_governance_snapshot(ProtocolGovernanceSnapshot::from_registry_inputs(
            ProtocolGovernanceSnapshotInputs {
                protocol_version,
                bindings: &bindings,
                codecs: &codecs,
                capability_profile,
                quota_profile,
                rollout_policy,
                kill_switch,
                business_policy_vocabulary,
            },
        ));

        registry
    }

    pub fn register_binding(&mut self, binding: TransportBinding) -> bool {
        self.bindings.insert(binding.protocol_id().to_string())
    }

    pub fn register_capability(&mut self, capability: impl Into<String>) -> bool {
        self.capabilities.insert(capability.into())
    }

    pub fn register_codec(&mut self, codec: impl Into<String>) -> bool {
        self.codecs.insert(codec.into())
    }

    pub fn register_schema(&mut self, descriptor: SchemaDescriptor) -> Option<SchemaDescriptor> {
        self.bindings
            .extend(descriptor.binding_protocols.iter().cloned());
        self.capabilities
            .extend(descriptor.required_capabilities.iter().cloned());
        self.schemas.insert(descriptor.schema.clone(), descriptor)
    }

    pub fn register_compatibility(
        &mut self,
        descriptor: ClientCompatibilityDescriptor,
    ) -> Option<ClientCompatibilityDescriptor> {
        self.bindings
            .extend(descriptor.supported_bindings.iter().cloned());
        self.codecs
            .extend(descriptor.supported_codecs.iter().cloned());
        self.capabilities
            .extend(descriptor.supported_capabilities.iter().cloned());
        self.compatibility_matrix
            .insert(descriptor.client_type.clone(), descriptor)
    }

    pub fn set_governance_snapshot(&mut self, governance_snapshot: ProtocolGovernanceSnapshot) {
        self.governance_snapshot = Some(governance_snapshot);
    }

    pub fn supports_binding(&self, binding: &TransportBinding) -> bool {
        self.bindings.contains(binding.protocol_id())
    }

    pub fn supports_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn schema(&self, schema: &str) -> Option<&SchemaDescriptor> {
        self.schemas.get(schema)
    }

    pub fn compatibility(&self, client_type: &str) -> Option<&ClientCompatibilityDescriptor> {
        self.compatibility_matrix.get(client_type)
    }

    pub fn protocol_version(&self) -> &str {
        self.protocol_version.as_str()
    }

    pub fn bindings(&self) -> &BTreeSet<String> {
        &self.bindings
    }

    pub fn codecs(&self) -> &BTreeSet<String> {
        &self.codecs
    }

    pub fn schemas(&self) -> &BTreeMap<String, SchemaDescriptor> {
        &self.schemas
    }

    pub fn compatibility_matrix(&self) -> &BTreeMap<String, ClientCompatibilityDescriptor> {
        &self.compatibility_matrix
    }

    pub fn governance_snapshot(&self) -> Option<&ProtocolGovernanceSnapshot> {
        self.governance_snapshot.as_ref()
    }
}

fn all_bindings() -> [TransportBinding; 7] {
    [
        TransportBinding::Http1,
        TransportBinding::Ws1,
        TransportBinding::Sse1,
        TransportBinding::Mqtt1,
        TransportBinding::Tcp1,
        TransportBinding::Udp1,
        TransportBinding::Quic1,
    ]
}

fn all_consumers() -> [&'static str; 6] {
    ["web", "desktop", "mobile", "backend", "agent", "iot-edge"]
}
