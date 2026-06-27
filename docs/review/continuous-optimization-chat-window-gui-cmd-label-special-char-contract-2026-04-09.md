# Continuous Optimization: Chat Window GUI CMD Label Special-Character Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/chat-window-gui.cmd`
- Contract: the Windows GUI `.cmd` wrapper must preserve `-Label` / `--label` literally when launching `chat-window-gui.ps1`.

## Gap

- `chat-window-gui.cmd --help` already surfaced the Windows GNU-style named flags after the previous micro-loop.
- The runtime wrapper still passed through `_cmd-forward-powershell.cmd`.
- That forwarder enabled delayed expansion and stripped `!` from label values before they reached `chat-window-gui.ps1`.
- The resulting runtime split was observable in diagnostics:
  - direct `chat-window-gui.ps1 -Label guest!` preserved `guest!`
  - `chat-window-gui.cmd -Label guest!` degraded the same value to `guest`

## Decision

- Keep the GUI script implementation unchanged for this loop.
- Replace the `.cmd` forwarder hop with a direct PowerShell invocation, matching the already-fixed `chat-window.cmd` and `open-chat-test.cmd` wrappers.
- Treat literal label preservation as part of the Step 12 visible Windows GUI wrapper contract.

## Changed Files

- `bin/chat-window-gui.cmd`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-chat-window-gui-cmd-label-special-char-contract-2026-04-09.md`
- `docs/step/continuous-optimization-chat-window-gui-cmd-label-special-char-contract-2026-04-09.md`
- `docs/架构/09BD-chat-window-gui-cmd-label-special-char-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BD-chat-window-gui-cmd-label-special-char-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_preserves_exclamation_mark_in_label -- --exact --nocapture
```

- Failed before the patch because diagnostics showed `script start label=guest conversation=...` instead of preserving `guest!`.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_preserves_exclamation_mark_in_label -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\chat-window-gui.cmd --help
```

## Remaining Gap

- This loop only closes the Windows `chat-window-gui.cmd` literal label preservation boundary.
- Other GUI runtime seams still need to be reviewed independently instead of assuming global consistency.
