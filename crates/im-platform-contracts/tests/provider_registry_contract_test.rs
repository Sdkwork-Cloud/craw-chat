use std::collections::BTreeMap;

use im_platform_contracts::{
    EffectiveProviderBinding, ObjectStorageDownloadUrlRequest, ObjectStorageObjectDescriptor,
    ObjectStorageProvider, ObjectStoragePutRequest, ObjectStorageUploadSession,
    ObjectStorageUploadUrlRequest, PrincipalProfile, PrincipalProfileProvider, ProviderDomain,
    ProviderHealthSnapshot, ProviderPluginDescriptor, ProviderRegistry,
    RtcCreateMediaSessionRequest, RtcMediaSessionMode, RtcParticipantCredential,
    RtcProviderEventKind, RtcProviderPort, RtcProviderWebhookEvent, RtcProviderWebhookParseRequest,
    RtcRecordingArtifact, RtcSessionHandle, RuntimeProviderRegistry, StaticProviderRegistry,
    rtc_provider_payload_hash,
};
use sdkwork_communication_rtc_service::{
    ProviderDomain as RtcProviderDomain, ProviderHealthSnapshot as RtcProviderHealthSnapshot,
    ProviderPluginDescriptor as RtcProviderPluginDescriptor, RtcContractError,
};
use sdkwork_im_contract_core::ContractError;

#[derive(Clone)]
struct StubRtcProvider;

impl RtcProviderPort for StubRtcProvider {
    fn descriptor(&self) -> RtcProviderPluginDescriptor {
        RtcProviderPluginDescriptor::new(
            "rtc-volcengine",
            RtcProviderDomain::Rtc,
            "volcengine",
            "火山引擎",
        )
        .with_default_selected(true)
        .with_required_capabilities([
            "session",
            "credential",
            "provider.webhook",
            "health",
        ])
    }

    fn create_session(
        &self,
        request: RtcCreateMediaSessionRequest,
    ) -> Result<RtcSessionHandle, RtcContractError> {
        Ok(RtcSessionHandle {
            tenant_id: request.tenant_id,
            rtc_session_id: request.rtc_session_id,
            provider_session_id: "volc-room-demo".into(),
            access_endpoint: Some("wss://rtc.volcengine.example/session".into()),
            region: Some("cn-beijing".into()),
        })
    }

    fn close_session(
        &self,
        _tenant_id: &str,
        _rtc_session_id: &str,
    ) -> Result<bool, RtcContractError> {
        Ok(true)
    }

    fn issue_participant_credential(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
        participant_id: &str,
        _context: Option<&sdkwork_communication_rtc_service::RtcParticipantCredentialContext>,
    ) -> Result<RtcParticipantCredential, RtcContractError> {
        Ok(RtcParticipantCredential {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            participant_id: participant_id.into(),
            credential: "credential-demo".into(),
            expires_at: "2026-04-08T12:00:00Z".into(),
        })
    }

    fn refresh_participant_credential(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
        participant_id: &str,
        _context: Option<&sdkwork_communication_rtc_service::RtcParticipantCredentialContext>,
    ) -> Result<RtcParticipantCredential, RtcContractError> {
        self.issue_participant_credential(tenant_id, rtc_session_id, participant_id, None)
    }

    fn parse_provider_webhook(
        &self,
        request: RtcProviderWebhookParseRequest,
    ) -> Result<RtcProviderWebhookEvent, RtcContractError> {
        Ok(RtcProviderWebhookEvent {
            provider: request.provider,
            provider_profile_id: request.provider_profile_id,
            external_event_id: Some("even100001".into()),
            event_type: "participant_joined".into(),
            event_kind: RtcProviderEventKind::ParticipantJoined,
            room_id: Some("room_demo".into()),
            rtc_session_id: Some("rtc_demo".into()),
            provider_session_id: Some("volc-room-demo".into()),
            participant_id: Some("1".into()),
            recording_id: None,
            occurred_at: Some("2026-04-08T12:00:00Z".into()),
            received_at: request.received_at,
            payload_hash: rtc_provider_payload_hash(request.raw_payload.as_str()),
            signature_header: request
                .headers
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("x-demo-signature"))
                .map(|(_, value)| value.clone()),
            raw_payload: request.raw_payload,
            normalized_event_json: "{\"eventKind\":\"participant_joined\"}".into(),
        })
    }

    fn export_recording_artifact(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcRecordingArtifact>, RtcContractError> {
        Ok(Some(RtcRecordingArtifact::drive_backed_recording(
            tenant_id,
            rtc_session_id,
            "space_rtc_recordings",
            format!("node_{rtc_session_id}"),
            Some("1".into()),
        )))
    }

    fn provider_health_snapshot(&self) -> RtcProviderHealthSnapshot {
        RtcProviderHealthSnapshot::healthy("rtc-volcengine", "2026-04-08T00:00:00Z")
    }
}

#[derive(Clone)]
struct StubObjectStorageProvider;

impl ObjectStorageProvider for StubObjectStorageProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "object-storage-aws",
            ProviderDomain::ObjectStorage,
            "aws",
            "Amazon Web Services",
        )
        .with_optional_capabilities(["presign", "multipart", "retention"])
    }

    fn put_object(
        &self,
        request: ObjectStoragePutRequest,
    ) -> Result<ObjectStorageObjectDescriptor, sdkwork_im_contract_core::ContractError> {
        Ok(ObjectStorageObjectDescriptor {
            bucket: request.bucket,
            object_key: request.object_key,
            content_length: request.content_length,
            etag: Some("etag-demo".into()),
        })
    }

    fn signed_upload_url(
        &self,
        request: ObjectStorageUploadUrlRequest,
    ) -> Result<ObjectStorageUploadSession, sdkwork_im_contract_core::ContractError> {
        Ok(ObjectStorageUploadSession {
            method: "PUT".into(),
            url: format!(
                "https://storage.example/{}/{}?ttl={}&upload=1",
                request.bucket, request.object_key, request.expires_in_seconds
            ),
            headers: BTreeMap::new(),
            expires_at: "2026-04-16T00:10:00.000Z".into(),
        })
    }

    fn signed_download_url(
        &self,
        request: ObjectStorageDownloadUrlRequest,
    ) -> Result<String, sdkwork_im_contract_core::ContractError> {
        Ok(format!(
            "https://storage.example/{}/{}?ttl={}",
            request.bucket, request.object_key, request.expires_in_seconds
        ))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy("object-storage-aws", "2026-04-08T00:00:00Z")
    }
}

#[derive(Clone)]
struct StubPrincipalProfileProvider;

impl PrincipalProfileProvider for StubPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-upstream-context",
            ProviderDomain::PrincipalProfile,
            "local",
            "本地实现",
        )
        .with_default_selected(true)
        .with_required_capabilities(["read", "profile"])
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, sdkwork_im_contract_core::ContractError> {
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            display_name: "Demo User".into(),
            external_system: None,
            external_principal_id: None,
            attributes: BTreeMap::from([("source".into(), "local".into())]),
            inactive: false,
        }))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, sdkwork_im_contract_core::ContractError> {
        principal_ids
            .iter()
            .map(|principal_id| self.get_profile(tenant_id, principal_id, principal_kind))
            .collect::<Result<Vec<_>, _>>()
            .map(|records| records.into_iter().flatten().collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, sdkwork_im_contract_core::ContractError> {
        Ok(self
            .get_profile(tenant_id, keyword, principal_kind)?
            .into_iter()
            .collect::<Vec<_>>())
    }

    fn map_external_principal(
        &self,
        tenant_id: &str,
        _principal_kind: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, sdkwork_im_contract_core::ContractError> {
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: "mapped-principal".into(),
            display_name: "Mapped User".into(),
            external_system: Some(external_system.into()),
            external_principal_id: Some(external_principal_id.into()),
            attributes: BTreeMap::new(),
            inactive: false,
        }))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(
            "principal-profile-upstream-context",
            "2026-04-08T00:00:00Z",
        )
    }
}

#[test]
fn test_provider_registry_platform_default_freezes_provider_matrix_and_override_precedence() {
    let registry = StaticProviderRegistry::platform_default().with_tenant_override(
        "t_rtc_aliyun",
        ProviderDomain::Rtc,
        "rtc-aliyun",
    );
    let snapshot = registry.snapshot();

    assert_eq!(snapshot.interface_version, "provider-registry/v1");
    assert_eq!(
        snapshot.precedence,
        vec![
            "tenant_override".to_string(),
            "deployment_profile".to_string(),
            "global_default".to_string(),
        ]
    );

    let rtc_binding = registry
        .effective_binding(ProviderDomain::Rtc, Some("t_rtc_aliyun"))
        .expect("rtc binding should exist");
    assert_eq!(
        rtc_binding,
        EffectiveProviderBinding {
            domain: ProviderDomain::Rtc,
            default_plugin_id: Some("rtc-volcengine".into()),
            selected_plugin_id: Some("rtc-aliyun".into()),
            selection_source: "tenant_override".into(),
            tenant_override_allowed: true,
        }
    );

    let storage_binding = registry
        .effective_binding(ProviderDomain::ObjectStorage, None)
        .expect("storage binding should exist");
    assert_eq!(storage_binding.default_plugin_id, None);
    assert_eq!(storage_binding.selected_plugin_id, None);
    assert_eq!(storage_binding.selection_source, "deployment_required");

    let user_binding = registry
        .effective_binding(ProviderDomain::PrincipalProfile, None)
        .expect("principal-profile binding should exist");
    assert_eq!(
        user_binding.selected_plugin_id.as_deref(),
        Some("principal-profile-upstream-context")
    );

    let plugins = snapshot.plugins;
    let rtc_volcengine = plugins
        .iter()
        .find(|plugin| plugin.plugin_id == "rtc-volcengine")
        .expect("rtc-volcengine plugin should exist");
    assert!(
        rtc_volcengine
            .required_capabilities
            .iter()
            .any(|capability| capability == "provider.webhook"),
        "RTC providers should advertise the current provider.webhook capability"
    );
    assert!(
        !rtc_volcengine
            .required_capabilities
            .iter()
            .any(|capability| capability == "callback"),
        "RTC providers must not advertise the retired callback capability"
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin.plugin_id == "rtc-volcengine")
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin.plugin_id == "rtc-aliyun")
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin.plugin_id == "rtc-tencent")
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin.plugin_id == "object-storage-aws")
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin.plugin_id == "object-storage-google")
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin.plugin_id == "principal-profile-external-catalog")
    );
}

#[test]
fn test_provider_registry_supports_deployment_profile_selection_between_tenant_override_and_global_default()
 {
    let registry = StaticProviderRegistry::platform_default()
        .with_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
        .with_tenant_override(
            "t_storage_aws",
            ProviderDomain::ObjectStorage,
            "object-storage-aws",
        );

    let deployment_binding = registry
        .effective_binding(ProviderDomain::ObjectStorage, None)
        .expect("deployment-profile object storage binding should exist");
    assert_eq!(
        deployment_binding,
        EffectiveProviderBinding {
            domain: ProviderDomain::ObjectStorage,
            default_plugin_id: None,
            selected_plugin_id: Some("object-storage-volcengine".into()),
            selection_source: "deployment_profile".into(),
            tenant_override_allowed: true,
        }
    );

    let tenant_binding = registry
        .effective_binding(ProviderDomain::ObjectStorage, Some("t_storage_aws"))
        .expect("tenant object storage binding should exist");
    assert_eq!(
        tenant_binding,
        EffectiveProviderBinding {
            domain: ProviderDomain::ObjectStorage,
            default_plugin_id: None,
            selected_plugin_id: Some("object-storage-aws".into()),
            selection_source: "tenant_override".into(),
            tenant_override_allowed: true,
        }
    );
}

#[test]
fn test_runtime_provider_registry_supports_policy_writes_and_rejects_cross_domain_plugin_ids() {
    let registry = RuntimeProviderRegistry::platform_default();

    let deployment_binding = registry
        .set_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
        .expect("deployment profile write should succeed");
    assert_eq!(
        deployment_binding,
        EffectiveProviderBinding {
            domain: ProviderDomain::ObjectStorage,
            default_plugin_id: None,
            selected_plugin_id: Some("object-storage-volcengine".into()),
            selection_source: "deployment_profile".into(),
            tenant_override_allowed: true,
        }
    );

    let tenant_binding = registry
        .set_tenant_override(
            "t_storage_aws",
            ProviderDomain::ObjectStorage,
            "object-storage-aws",
        )
        .expect("tenant override write should succeed");
    assert_eq!(
        tenant_binding,
        EffectiveProviderBinding {
            domain: ProviderDomain::ObjectStorage,
            default_plugin_id: None,
            selected_plugin_id: Some("object-storage-aws".into()),
            selection_source: "tenant_override".into(),
            tenant_override_allowed: true,
        }
    );

    let invalid = registry
        .set_deployment_profile(ProviderDomain::Rtc, "object-storage-aws")
        .expect_err("cross-domain plugin ids should be rejected");
    assert!(matches!(invalid, ContractError::UnsupportedCapability(_)));
}

#[test]
fn test_runtime_provider_registry_tracks_policy_versions_and_can_roll_back_to_previous_snapshot() {
    let registry = RuntimeProviderRegistry::platform_default();

    let initial_history = registry.policy_history();
    assert_eq!(initial_history.current_version, 1);
    assert_eq!(initial_history.items.len(), 1);
    assert!(initial_history.items[0].deployment_profiles.is_empty());
    assert!(initial_history.items[0].tenant_overrides.is_empty());
    assert_eq!(initial_history.items[0].rollback_from_version, None);

    registry
        .set_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
        .expect("deployment profile write should succeed");
    registry
        .set_tenant_override("t_provider_combo", ProviderDomain::Rtc, "rtc-aliyun")
        .expect("tenant override write should succeed");

    let updated_history = registry.policy_history();
    assert_eq!(updated_history.current_version, 3);
    assert_eq!(updated_history.items.len(), 3);
    assert!(
        updated_history.items[1]
            .deployment_profiles
            .iter()
            .any(|entry| entry.domain == ProviderDomain::ObjectStorage
                && entry.plugin_id == "object-storage-volcengine")
    );
    assert!(
        updated_history.items[2]
            .tenant_overrides
            .iter()
            .any(|entry| entry.tenant_id == "t_provider_combo"
                && entry
                    .bindings
                    .iter()
                    .any(|binding| binding.domain == ProviderDomain::Rtc
                        && binding.plugin_id == "rtc-aliyun"))
    );

    let rollback_snapshot = registry
        .rollback_to(1)
        .expect("rollback to baseline snapshot should succeed");
    assert_eq!(rollback_snapshot.version, 4);
    assert_eq!(rollback_snapshot.rollback_from_version, Some(1));
    assert!(rollback_snapshot.deployment_profiles.is_empty());
    assert!(rollback_snapshot.tenant_overrides.is_empty());

    let global_object_storage = registry
        .effective_binding(ProviderDomain::ObjectStorage, None)
        .expect("global object storage binding should exist after rollback");
    assert_eq!(global_object_storage.selected_plugin_id, None);
    assert_eq!(
        global_object_storage.selection_source,
        "deployment_required"
    );

    let tenant_rtc = registry
        .effective_binding(ProviderDomain::Rtc, Some("t_provider_combo"))
        .expect("tenant rtc binding should still resolve after rollback");
    assert_eq!(
        tenant_rtc.selected_plugin_id.as_deref(),
        Some("rtc-volcengine")
    );
    assert_eq!(tenant_rtc.selection_source, "global_default");

    let rollback_history = registry.policy_history();
    assert_eq!(rollback_history.current_version, 4);
    assert_eq!(rollback_history.items.len(), 4);
    assert_eq!(rollback_history.items[3].rollback_from_version, Some(1));
}

#[test]
fn test_runtime_provider_registry_can_diff_committed_policy_versions() {
    let registry = RuntimeProviderRegistry::platform_default();

    registry
        .set_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
        .expect("initial deployment profile write should succeed");
    registry
        .set_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-aws")
        .expect("deployment profile update should succeed");
    registry
        .set_tenant_override("t_provider_combo", ProviderDomain::Rtc, "rtc-aliyun")
        .expect("tenant override write should succeed");

    let changed_diff = registry
        .diff_versions(2, 4)
        .expect("diff between committed versions should succeed");
    assert_eq!(changed_diff.from_version, 2);
    assert_eq!(changed_diff.to_version, 4);
    assert_eq!(changed_diff.deployment_profile_changes.len(), 1);
    assert_eq!(
        changed_diff.deployment_profile_changes[0].domain,
        ProviderDomain::ObjectStorage
    );
    assert_eq!(
        changed_diff.deployment_profile_changes[0]
            .change_kind
            .as_str(),
        "changed"
    );
    assert_eq!(
        changed_diff.deployment_profile_changes[0]
            .from_plugin_id
            .as_deref(),
        Some("object-storage-volcengine")
    );
    assert_eq!(
        changed_diff.deployment_profile_changes[0]
            .to_plugin_id
            .as_deref(),
        Some("object-storage-aws")
    );
    assert_eq!(changed_diff.tenant_override_changes.len(), 1);
    assert_eq!(
        changed_diff.tenant_override_changes[0].tenant_id,
        "t_provider_combo"
    );
    assert_eq!(
        changed_diff.tenant_override_changes[0].domain,
        ProviderDomain::Rtc
    );
    assert_eq!(
        changed_diff.tenant_override_changes[0].change_kind.as_str(),
        "added"
    );
    assert_eq!(changed_diff.tenant_override_changes[0].from_plugin_id, None);
    assert_eq!(
        changed_diff.tenant_override_changes[0]
            .to_plugin_id
            .as_deref(),
        Some("rtc-aliyun")
    );

    let rollback_snapshot = registry
        .rollback_to(1)
        .expect("rollback to baseline snapshot should succeed");
    assert_eq!(rollback_snapshot.version, 5);

    let removed_diff = registry
        .diff_versions(4, 5)
        .expect("diff after rollback should succeed");
    assert_eq!(removed_diff.deployment_profile_changes.len(), 1);
    assert_eq!(
        removed_diff.deployment_profile_changes[0]
            .change_kind
            .as_str(),
        "removed"
    );
    assert_eq!(
        removed_diff.deployment_profile_changes[0]
            .from_plugin_id
            .as_deref(),
        Some("object-storage-aws")
    );
    assert_eq!(
        removed_diff.deployment_profile_changes[0].to_plugin_id,
        None
    );
    assert_eq!(removed_diff.tenant_override_changes.len(), 1);
    assert_eq!(
        removed_diff.tenant_override_changes[0].change_kind.as_str(),
        "removed"
    );
    assert_eq!(
        removed_diff.tenant_override_changes[0]
            .from_plugin_id
            .as_deref(),
        Some("rtc-aliyun")
    );
    assert_eq!(removed_diff.tenant_override_changes[0].to_plugin_id, None);
}

#[test]
fn test_runtime_provider_registry_rejects_reversed_policy_diff_version_range() {
    let registry = RuntimeProviderRegistry::platform_default();

    registry
        .set_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
        .expect("deployment profile update should succeed");

    let invalid = registry
        .diff_versions(2, 1)
        .expect_err("reversed diff version ranges should be rejected");
    match invalid {
        ContractError::UnsupportedCapability(message) => {
            assert!(
                message.contains("fromVersion must not exceed toVersion"),
                "reversed diff error should explain the invalid version range"
            );
        }
        other => panic!("expected unsupported capability error, got {other:?}"),
    }
}

#[test]
fn test_runtime_provider_registry_rejects_empty_tenant_override_id() {
    let registry = RuntimeProviderRegistry::platform_default();

    let invalid = registry
        .set_tenant_override("", ProviderDomain::Rtc, "rtc-aliyun")
        .expect_err("empty tenant override ids should be rejected");
    match invalid {
        ContractError::UnsupportedCapability(message) => {
            assert!(
                message.contains("tenantId cannot be empty"),
                "empty tenant override error should explain why the tenant id is invalid"
            );
        }
        other => panic!("expected unsupported capability error, got {other:?}"),
    }
}

#[test]
fn test_runtime_provider_registry_rejects_oversized_tenant_override_id() {
    let registry = RuntimeProviderRegistry::platform_default();
    let tenant_id = "t".repeat(257);

    let invalid = registry
        .set_tenant_override(tenant_id.as_str(), ProviderDomain::Rtc, "rtc-aliyun")
        .expect_err("oversized tenant override ids should be rejected");
    match invalid {
        ContractError::UnsupportedCapability(message) => {
            assert!(
                message.contains("tenantId"),
                "oversized tenant override error should name the rejected field"
            );
        }
        other => panic!("expected unsupported capability error, got {other:?}"),
    }
}

#[test]
fn test_runtime_provider_registry_can_preview_policy_write_without_mutating_history() {
    let registry = RuntimeProviderRegistry::platform_default();

    let preview = registry
        .preview_upsert(
            None,
            ProviderDomain::ObjectStorage,
            "object-storage-volcengine",
        )
        .expect("deployment profile preview should succeed");
    assert_eq!(
        preview.status,
        im_platform_contracts::ProviderPolicyResultStatus::Preview
    );
    assert_eq!(preview.base_version, 1);
    assert_eq!(preview.preview_version, 2);
    assert_eq!(preview.tenant_id, None);
    assert_eq!(
        preview.preview_binding.domain,
        ProviderDomain::ObjectStorage
    );
    assert_eq!(
        preview.preview_binding.selected_plugin_id.as_deref(),
        Some("object-storage-volcengine")
    );
    assert_eq!(
        preview.preview_binding.selection_source,
        "deployment_profile"
    );
    assert_eq!(preview.diff.from_version, 1);
    assert_eq!(preview.diff.to_version, 2);
    assert_eq!(preview.diff.deployment_profile_changes.len(), 1);
    assert_eq!(
        preview.diff.deployment_profile_changes[0]
            .change_kind
            .as_str(),
        "added"
    );
    assert_eq!(
        preview.diff.deployment_profile_changes[0]
            .to_plugin_id
            .as_deref(),
        Some("object-storage-volcengine")
    );
    assert!(preview.diff.tenant_override_changes.is_empty());

    let history_after_preview = registry.policy_history();
    assert_eq!(history_after_preview.current_version, 1);
    assert_eq!(history_after_preview.items.len(), 1);
    assert!(
        history_after_preview.items[0]
            .deployment_profiles
            .is_empty()
    );

    let global_binding = registry
        .effective_binding(ProviderDomain::ObjectStorage, None)
        .expect("global object storage binding should still resolve");
    assert_eq!(global_binding.selected_plugin_id, None);
    assert_eq!(global_binding.selection_source, "deployment_required");
}

#[test]
fn test_runtime_provider_registry_rejects_stale_expected_base_version_without_mutation() {
    let registry = RuntimeProviderRegistry::platform_default();

    let preview = registry
        .preview_upsert(Some("t_provider_combo"), ProviderDomain::Rtc, "rtc-aliyun")
        .expect("tenant override preview should succeed");
    assert_eq!(preview.base_version, 1);

    registry
        .set_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
        .expect("concurrent deployment profile write should succeed");

    let conflict = registry
        .set_tenant_override_with_expected_version(
            "t_provider_combo",
            ProviderDomain::Rtc,
            "rtc-aliyun",
            Some(preview.base_version),
        )
        .expect_err("stale expected base version should be rejected");
    assert!(matches!(conflict, ContractError::Conflict(_)));
    let ContractError::Conflict(message) = conflict else {
        unreachable!("stale writes should return a conflict");
    };
    assert!(
        message.contains("expected 1") && message.contains("current 2"),
        "conflict message should describe version drift"
    );

    let history = registry.policy_history();
    assert_eq!(history.current_version, 2);
    assert_eq!(history.items.len(), 2);
    assert!(
        history.items[1].tenant_overrides.is_empty(),
        "stale write must not append tenant override history"
    );

    let tenant_binding = registry
        .effective_binding(ProviderDomain::Rtc, Some("t_provider_combo"))
        .expect("tenant rtc binding should still resolve");
    assert_eq!(
        tenant_binding.selected_plugin_id.as_deref(),
        Some("rtc-volcengine")
    );
    assert_eq!(tenant_binding.selection_source, "global_default");
}

#[test]
fn test_runtime_provider_registry_commit_upsert_returns_committed_version_binding_and_diff() {
    let registry = RuntimeProviderRegistry::platform_default();

    let preview = registry
        .preview_upsert(Some("t_provider_combo"), ProviderDomain::Rtc, "rtc-aliyun")
        .expect("tenant override preview should succeed");
    assert_eq!(preview.base_version, 1);

    let commit = registry
        .commit_upsert(
            Some("t_provider_combo"),
            ProviderDomain::Rtc,
            "rtc-aliyun",
            Some(preview.base_version),
        )
        .expect("preview confirmation write should succeed");
    assert_eq!(commit.current_version, 2);
    assert_eq!(commit.tenant_id.as_deref(), Some("t_provider_combo"));
    assert_eq!(commit.committed_binding.domain, ProviderDomain::Rtc);
    assert_eq!(
        commit.committed_binding.selected_plugin_id.as_deref(),
        Some("rtc-aliyun")
    );
    assert_eq!(commit.committed_binding.selection_source, "tenant_override");
    assert_eq!(commit.diff.from_version, 1);
    assert_eq!(commit.diff.to_version, 2);
    assert_eq!(commit.diff.deployment_profile_changes.len(), 0);
    assert_eq!(commit.diff.tenant_override_changes.len(), 1);
    assert_eq!(
        commit.diff.tenant_override_changes[0].tenant_id,
        "t_provider_combo"
    );
    assert_eq!(
        commit.diff.tenant_override_changes[0]
            .to_plugin_id
            .as_deref(),
        Some("rtc-aliyun")
    );

    let history = registry.policy_history();
    assert_eq!(history.current_version, 2);
    assert_eq!(history.items.len(), 2);
}

#[test]
fn test_runtime_provider_registry_suppresses_noop_commit_without_advancing_version() {
    let registry = RuntimeProviderRegistry::platform_default();

    let first_commit = registry
        .commit_upsert(
            None,
            ProviderDomain::ObjectStorage,
            "object-storage-volcengine",
            None,
        )
        .expect("first deployment profile commit should succeed");
    assert_eq!(
        first_commit.status,
        im_platform_contracts::ProviderPolicyResultStatus::Applied
    );
    assert!(first_commit.applied);
    assert_eq!(first_commit.current_version, 2);

    let noop_commit = registry
        .commit_upsert(
            None,
            ProviderDomain::ObjectStorage,
            "object-storage-volcengine",
            Some(first_commit.current_version),
        )
        .expect("same deployment profile commit should become no-op");
    assert_eq!(
        noop_commit.status,
        im_platform_contracts::ProviderPolicyResultStatus::Noop
    );
    assert!(!noop_commit.applied);
    assert_eq!(noop_commit.current_version, 2);
    assert_eq!(
        noop_commit.committed_binding.selected_plugin_id.as_deref(),
        Some("object-storage-volcengine")
    );
    assert_eq!(noop_commit.diff.from_version, 2);
    assert_eq!(noop_commit.diff.to_version, 2);
    assert!(noop_commit.diff.deployment_profile_changes.is_empty());
    assert!(noop_commit.diff.tenant_override_changes.is_empty());

    let history = registry.policy_history();
    assert_eq!(history.current_version, 2);
    assert_eq!(history.items.len(), 2);
}

#[test]
fn test_provider_ports_can_be_implemented_without_vendor_sdk_types_leaking_into_contracts() {
    let rtc = StubRtcProvider;
    let storage = StubObjectStorageProvider;
    let principal_profile = StubPrincipalProfileProvider;

    let rtc_session = rtc
        .create_session(RtcCreateMediaSessionRequest {
            tenant_id: "100001".into(),
            rtc_session_id: "rtc_demo".into(),
            media_mode: RtcMediaSessionMode::Video,
            room_id: Some("c_demo".into()),
            region: Some("cn-beijing".into()),
        })
        .expect("rtc create_session should succeed");
    assert_eq!(rtc_session.provider_session_id, "volc-room-demo");

    let object_descriptor = storage
        .put_object(ObjectStoragePutRequest {
            bucket: "media".into(),
            object_key: "rtc/demo.mp4".into(),
            content_length: 128,
            content_type: Some("video/mp4".into()),
            storage_class: Some("standard".into()),
        })
        .expect("object put should succeed");
    assert_eq!(object_descriptor.bucket, "media");

    let upload_session = storage
        .signed_upload_url(ObjectStorageUploadUrlRequest {
            bucket: "media".into(),
            object_key: "uploads/demo.mp4".into(),
            expires_in_seconds: 600,
            content_type: Some("video/mp4".into()),
            content_length: Some(128),
        })
        .expect("signed_upload_url should succeed");
    assert_eq!(upload_session.method, "PUT");
    assert!(upload_session.url.contains("upload=1"));

    let profile = principal_profile
        .get_profile("100001", "1", "user")
        .expect("get_profile should succeed")
        .expect("profile should exist");
    assert_eq!(profile.display_name, "Demo User");

    let signed_url = storage
        .signed_download_url(ObjectStorageDownloadUrlRequest {
            bucket: "media".into(),
            object_key: "rtc/demo.mp4".into(),
            expires_in_seconds: 600,
        })
        .expect("signed_download_url should succeed");
    assert!(signed_url.contains("ttl=600"));

    let mapped_profile = principal_profile
        .map_external_principal("100001", "user", "iam", "ext_demo")
        .expect("map_external_principal should succeed")
        .expect("mapped profile should exist");
    assert_eq!(mapped_profile.external_system.as_deref(), Some("iam"));

    assert_eq!(
        rtc.provider_health_snapshot(),
        RtcProviderHealthSnapshot::healthy("rtc-volcengine", "2026-04-08T00:00:00Z")
    );
    assert_eq!(
        storage.provider_health_snapshot(),
        ProviderHealthSnapshot::healthy("object-storage-aws", "2026-04-08T00:00:00Z")
    );
    assert_eq!(
        principal_profile.provider_health_snapshot(),
        ProviderHealthSnapshot::healthy(
            "principal-profile-upstream-context",
            "2026-04-08T00:00:00Z"
        )
    );
}
