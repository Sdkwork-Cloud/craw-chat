#[test]
fn test_chat_cli_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "tools/chat-cli/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_chat_cli_command_parse_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub fn parse_cli_args<I, S>(",
        "fn parse_command_operation(",
        "fn build_command_context(global: GlobalOptions) -> CommandContext {",
        "fn parse_permissions(raw: String) -> Vec<String> {",
        "fn cli_usage() -> String {",
        "fn token_usage() -> String {",
        "fn watch_usage() -> String {",
        "struct ArgCursor {",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "tools/chat-cli/src/lib.rs should not keep command-parse symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_chat_cli_config_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "fn resolve_public_bearer_secret_from_config() -> Option<String> {",
        "fn resolve_base_url_from_config() -> Option<String> {",
        "fn find_local_env_file() -> Option<PathBuf> {",
        "fn env_file_candidates() -> Vec<PathBuf> {",
        "fn read_env_file_value(path: &Path, key: &str) -> Option<String> {",
        "fn bind_address_to_base_url(bind_address: &str) -> String {",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "tools/chat-cli/src/lib.rs should not keep config symbol: {forbidden_symbol}"
        );
    }
}
