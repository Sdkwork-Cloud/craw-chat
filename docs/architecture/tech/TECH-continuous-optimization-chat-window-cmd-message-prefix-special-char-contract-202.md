> Migrated from `docs/review/continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Chat Window CMD Message-Prefix Special-Char Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/chat-window.cmd -> bin/chat-window.ps1`
- Contract: the Windows `.cmd` interactive path must preserve `--message-prefix` byte-for-byte, including `!`.

## Gap

- `chat-window.cmd` still forwarded through `_cmd-forward-powershell.cmd`.
- That forwarder used delayed expansion while reconstructing the PowerShell argument string.
- Calling the Windows wrapper with `--message-prefix "[bang!] "` silently dropped `!` before the value reached `chat-window.ps1`.
- The visible failure mode was subtle but real:
  - the session still opened
  - the message still sent successfully
  - but the stored timeline summary became `[bang] hello ...` instead of `[bang!] hello ...`

## Decision

- Keep the existing `chat-window.ps1` launch behavior unchanged for this loop.
- Stop routing `chat-window.cmd` through `_cmd-forward-powershell.cmd`.
- Let `chat-window.cmd` invoke `chat-window.ps1` directly so Windows interactive inputs remain literal.

## Changed Files

- `bin/chat-window.cmd`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09.md`
- `docs/step/continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09.md`
- `docs/架构/09AZ-chat-window-cmd-message-prefix-special-char-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150AZ-chat-window-cmd-message-prefix-special-char-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_preserves_exclamation_mark_in_message_prefix -- --exact --nocapture
```

- Failed before the patch because the conversation timeline contained `[bang] hello from chat-window cmd bang` instead of `[bang!] hello from chat-window cmd bang`.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_preserves_exclamation_mark_in_message_prefix -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_accepts_gnu_style_named_flags_for_interactive_session -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\chat-window.cmd --help
```

## Remaining Gap

- This loop only closes the Windows `chat-window.cmd` special-character preservation boundary for interactive launches.
- `_cmd-forward-powershell.cmd` remains a shared helper for other scripts and should not be considered globally closed by this loop.

