# Continuous Optimization: Chat Window CMD Help GNU-Surface Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/chat-window.cmd --help`
- Contract: the Windows `.cmd` help surface must explicitly show the GNU-style named flags that the wrapper actually accepts.

## Gap

- `chat-window.cmd` already accepted the documented GNU-style launch parameters on Windows.
- Its help text still came entirely from `chat-window.ps1` and only showed PowerShell-style parameter names such as `-ConversationId` and `-UserId`.
- That created a discoverability split:
  - runtime behavior accepted `--conversation-id`, `--user-id`, and `--message-prefix`
  - help output still taught operators a different parameter surface

## Decision

- Keep the existing parameter parsing unchanged for this loop.
- Extend `chat-window.ps1` help text with an explicit Windows `.cmd` usage line that shows the GNU-style named flags.
- Treat `.cmd` help output as part of the Step 12 Windows operator contract.

## Changed Files

- `bin/chat-window.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-chat-window-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-chat-window-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BA-chat-window-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BA-chat-window-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
```

- Failed before the patch because `chat-window.cmd --help` only printed `-ConversationId`, `-UserId`, and `-MessagePrefix` style usage.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\chat-window.cmd --help
```

## Remaining Gap

- This loop only closes the Windows `chat-window.cmd --help` discoverability boundary.
- Other wrapper help surfaces should still be reviewed separately instead of assuming global consistency.
