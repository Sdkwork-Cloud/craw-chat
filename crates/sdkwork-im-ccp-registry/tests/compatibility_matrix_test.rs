use sdkwork_im_ccp_registry::CcpRegistry;

#[test]
fn test_control_plane_registry_freezes_schema_stage_and_client_compatibility_matrix() {
    let registry = CcpRegistry::control_plane_v1();

    let hello = registry
        .schema("ccp.control.hello")
        .expect("hello schema should be present in the registry");
    assert_eq!(hello.stage.as_str(), "stable");
    assert!(
        hello.supported_consumers.contains("web"),
        "stable control hello should declare web consumers"
    );
    assert!(
        hello.supported_consumers.contains("desktop"),
        "stable control hello should declare desktop consumers"
    );

    let web = registry
        .compatibility("web")
        .expect("web client compatibility should be present");
    assert_eq!(web.minimum_protocol_version, "ccp/1.0");
    assert!(
        web.supported_bindings.contains("ccp/ws/1"),
        "web clients should negotiate websocket binding"
    );
    assert!(
        web.supported_codecs.contains("json"),
        "web clients should expose json codec support"
    );
    assert!(
        !web.blocked_experimental_capabilities
            .contains("session.resume"),
        "baseline web client should not block stable session resume"
    );
}
