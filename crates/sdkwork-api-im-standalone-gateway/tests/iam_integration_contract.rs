#[test]
fn embedded_iam_preserves_router_manifest_framework_binding() {
    let source = include_str!("../src/main.rs");

    assert!(source.contains("bootstrap_iam_app_for_application"));
    assert!(source.contains("ComposedApiAssembly::try_compose"));
    assert!(source.contains("vec![im_contribution, iam_contribution]"));
    assert!(!source.contains("iam_contribution.router"));
}
