> Migrated from `docs/review/continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Chat Window GUI CMD GNU-Flag Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/chat-window-gui.cmd`
- Contract: the Windows GUI `.cmd` wrapper must actually accept the GNU-style named flags that its own help output advertises.

## Gap

- `chat-window-gui.cmd --help` already surfaced a GNU-style usage line on Windows.
- The runtime script `chat-window-gui.ps1` still lacked GNU-style aliases for hyphenated parameter names such as `--conversation-id` and `--user-id`.
- That created a contract split:
  - local help taught `--conversation-id`, `--user-id`, and `--message-prefix`
  - runtime invocation with those same flags fell back to usage instead of entering the GUI launch flow

## Decision

- Keep the current GUI wrapper and runtime flow unchanged for this loop.
- Add GNU-style aliases to the relevant `chat-window-gui.ps1` parameters so the advertised `.cmd` contract becomes executable.
- Treat GNU-style named flag acceptance as part of the Step 12 visible Windows GUI wrapper contract.

## Changed Files

- `bin/chat-window-gui.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09.md`
- `docs/step/continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09.md`
- `docs/架构/09BE-chat-window-gui-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BE-chat-window-gui-cmd-gnu-flag-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_accepts_gnu_style_named_flags_for_launch -- --exact --nocapture
```

- Failed before the patch because the GNU-style launch helper exited without entering the GUI script flow and produced no diagnostics.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_accepts_gnu_style_named_flags_for_launch -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_preserves_exclamation_mark_in_label -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only closes the Windows `chat-window-gui.cmd` GNU-style runtime argument contract.
- Other local operator wrappers that still rely on `_cmd-forward-powershell.cmd` still need to be audited independently instead of assuming generic safety.

