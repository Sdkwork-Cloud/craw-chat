fn main() {
    let document = governance_service::export_openapi_document()
        .expect("control-plane openapi document should export");
    println!(
        "{}",
        serde_json::to_string_pretty(&document)
            .expect("control-plane openapi document should serialize"),
    );
}
