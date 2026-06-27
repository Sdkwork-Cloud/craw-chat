> Migrated from `docs/架构/09BE-chat-window-gui-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09BE Chat Window GUI CMD GNU-Flag Contract Implementation Plan

## Goal

Make `bin/chat-window-gui.cmd` actually accept the GNU-style named flags that the Windows GUI wrapper advertises.

## Implementation

1. Freeze the Windows GUI `.cmd` GNU-style runtime boundary in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Add GNU-style aliases to the hyphenated `chat-window-gui.ps1` parameters
3. Keep the GUI wrapper and GUI runtime flow unchanged for this loop
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No GUI runtime refactor
- No wrapper redesign
- No generic forwarder redesign
- No unrelated operator script changes

