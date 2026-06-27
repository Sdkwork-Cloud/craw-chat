use sdkwork_im_ccp_registry::CcpRegistry;

#[test]
fn test_control_plane_registry_freezes_governance_snapshot_for_runtime_consumers() {
    let registry = CcpRegistry::control_plane_v1();

    let governance = registry
        .governance_snapshot()
        .expect("control plane registry should expose governance snapshot");

    assert_eq!(
        governance.capability_profile.profile_id,
        "control-plane-stable"
    );
    assert!(
        governance
            .capability_profile
            .enabled_capabilities
            .contains("payload.cbor"),
        "stable capability profile should enable cbor payload rollout before overrides"
    );
    assert_eq!(
        governance.quota_profile.max_concurrent_sessions_per_tenant,
        20_000
    );
    assert_eq!(governance.rollout_policy.release_channel.as_str(), "stable");
    assert_eq!(governance.rollout_policy.traffic_percent, 100);
    assert!(governance.kill_switch.active);
    assert!(
        governance
            .kill_switch
            .disabled_capabilities
            .contains("payload.cbor"),
        "kill switch should be able to turn off a risky protocol capability"
    );
    assert!(
        !governance
            .effective_snapshot
            .enabled_capabilities
            .contains("payload.cbor"),
        "effective snapshot must apply kill switch before runtime consumes the result"
    );
    assert!(
        governance
            .effective_snapshot
            .enabled_capabilities
            .contains("session.resume"),
        "effective snapshot must publish session.resume for runtime hello negotiation"
    );
    assert!(
        governance
            .effective_snapshot
            .allowed_bindings
            .contains("ccp/ws/1"),
        "effective snapshot should keep websocket binding for runtime readers"
    );
    assert!(
        governance
            .effective_snapshot
            .allowed_codecs
            .contains("json"),
        "effective snapshot should keep json codec for control plane consumers"
    );
    assert!(governance.effective_snapshot.kill_switch_active);
    assert_eq!(
        governance.effective_snapshot.precedence,
        vec![
            "emergency_kill_switch".to_string(),
            "operator_rollout_override".to_string(),
            "tenant_protocol_policy".to_string(),
            "cell_region_release_channel".to_string(),
            "global_stable_baseline".to_string(),
        ]
    );
    assert_eq!(
        governance.business_policy_vocabulary.policy_version_field,
        "policy_version"
    );
    assert_eq!(
        governance.business_policy_vocabulary.capability_flags_field,
        "capability_flags"
    );
    assert_eq!(
        governance
            .business_policy_vocabulary
            .history_visibility_field,
        "history_visibility"
    );
    assert_eq!(
        governance
            .business_policy_vocabulary
            .retention_policy_ref_field,
        "retention_policy_ref"
    );
    assert_eq!(
        governance
            .business_policy_vocabulary
            .history_visibility_modes,
        vec![
            "joined".to_string(),
            "invited".to_string(),
            "shared".to_string(),
            "world_readable".to_string()
        ]
    );
    assert_eq!(
        governance
            .business_policy_vocabulary
            .retention_policy_scopes,
        vec![
            "tenant".to_string(),
            "space".to_string(),
            "group".to_string(),
            "channel".to_string(),
            "thread".to_string(),
        ]
    );
    assert_eq!(
        governance.business_policy_vocabulary.retention_classes,
        vec![
            "ephemeral".to_string(),
            "standard".to_string(),
            "extended".to_string(),
            "legal_hold".to_string(),
        ]
    );
}
