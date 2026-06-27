# Continuous Optimization: Chat Window CMD GNU-Flag Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/chat-window.cmd -> bin/chat-window.ps1 -> bin/chat-cli.ps1`
- Contract: GNU-style named flags used by `chat-window.sh` must also work from the Windows `.cmd` entry for interactive validation.

## Gap

- `chat-window.sh` already exposed the operator-facing GNU flag surface for `--base-url`, `--tenant-id`, `--conversation-id`, `--user-id`, `--session-id`, `--device-id`, `--label`, and `--message-prefix`.
- `chat-window.cmd` still used the generic PowerShell forwarder while `chat-window.ps1` only accepted PowerShell-style parameter names.
- Calling the Windows wrapper with GNU-style required flags such as `--base-url`, `--conversation-id`, and `--user-id` exited on usage instead of launching the interactive chat session.

## Decision

- Keep the `.cmd` wrapper unchanged for this loop.
- Make `bin/chat-window.ps1` explicitly accept GNU-style aliases for the hyphenated operator parameters it owns.
- Treat `.ps1`, `.sh`, and `.cmd` as one launch contract for local interactive validation.

## Changed Files

- `bin/chat-window.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09.md`
- `docs/step/continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09.md`
- `docs/架构/09AX-chat-window-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150AX-chat-window-cmd-gnu-flag-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_accepts_gnu_style_named_flags_for_interactive_session -- --exact --nocapture
```

- Failed before the patch because the `.cmd` path printed `Usage: powershell -ExecutionPolicy Bypass -File bin/chat-window.ps1 ...` instead of entering `chat-session`.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_accepts_gnu_style_named_flags_for_interactive_session -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only closes the Windows `chat-window.cmd` GNU-flag boundary.
- Step 12 remains in continuous-optimization mode and should continue from the next smallest honest CLI or operator seam.
