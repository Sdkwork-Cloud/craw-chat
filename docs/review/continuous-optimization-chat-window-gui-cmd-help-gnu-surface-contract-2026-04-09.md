# Continuous Optimization: Chat Window GUI CMD Help GNU-Surface Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/chat-window-gui.cmd --help`
- Contract: the Windows GUI `.cmd` help surface must explicitly show the GNU-style named flags that the wrapper already accepts.

## Gap

- `chat-window-gui.cmd --help` already worked on Windows.
- Its help text still came entirely from `chat-window-gui.ps1` and only showed PowerShell-style parameter names such as `-ConversationId`, `-UserId`, and `-MessagePrefix`.
- That created a discoverability split:
  - runtime behavior accepted `--conversation-id`, `--user-id`, and `--message-prefix`
  - help output still taught operators a different PowerShell-only surface

## Decision

- Keep the existing GUI launch flow unchanged for this loop.
- Extend `chat-window-gui.ps1` help text with an explicit Windows `.cmd` usage line that shows the GNU-style named flags.
- Treat `.cmd --help` as part of the Step 12 visible Windows GUI wrapper contract.

## Changed Files

- `bin/chat-window-gui.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-chat-window-gui-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-chat-window-gui-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BC-chat-window-gui-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BC-chat-window-gui-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
```

- Failed before the patch because `chat-window-gui.cmd --help` only printed `-ConversationId`, `-UserId`, and `-MessagePrefix` style usage.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\chat-window-gui.cmd --help
```

## Remaining Gap

- This loop only closes the Windows `chat-window-gui.cmd --help` discoverability boundary.
- Other GUI wrapper runtime seams still need to be reviewed independently instead of assuming global consistency.
