use sdkwork_im_cli::{CommandOutput, execute_command, parse_cli_args};

fn command_output_json(output: CommandOutput) -> serde_json::Value {
    match output {
        CommandOutput::Json(value) => value,
        other => panic!("expected json output, got {other:?}"),
    }
}

#[test]
fn test_chat_cli_rejects_removed_login_command() {
    let error = parse_cli_args([
        "sdkwork-im-cli",
        "--base-url",
        "http://127.0.0.1:18079",
        "--tenant-id",
        "100001",
        "--session-id",
        "s_guest",
        "--device-id",
        "d_guest",
        "login",
        "--login",
        "2",
        "--password",
        "Guest#2026",
    ])
    .expect_err("sdkwork-im CLI must not expose a sdkwork-im-owned login command");

    assert!(
        error.message().contains("unknown command: login"),
        "unexpected parse error: {}",
        error.message()
    );
}

#[tokio::test]
async fn test_chat_cli_token_command_uses_supplied_bearer_token_without_login() {
    let output = execute_command(
        parse_cli_args([
            "sdkwork-im-cli",
            "--base-url",
            "http://127.0.0.1:18079",
            "--tenant-id",
            "100001",
            "--user-id",
            "2",
            "--session-id",
            "chat_session_guest",
            "--device-id",
            "d_guest",
            "--bearer-token",
            "external-appbase-token",
            "token",
            "--token-only",
        ])
        .expect("token args should parse"),
    )
    .await
    .expect("token command should not call sdkwork-im login");

    let json = command_output_json(output);
    assert_eq!(json["source"], "providedBearerToken");
    assert_eq!(json["token"], "external-appbase-token");
    assert!(json["claims"].is_null());
}
