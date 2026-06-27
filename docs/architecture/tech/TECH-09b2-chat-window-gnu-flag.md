> Migrated from `docs/架构/09AX-chat-window-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09AX Chat Window CMD GNU-Flag Contract Implementation Plan

## Goal

Make `bin/chat-window.cmd` accept the same GNU-style named flags that `bin/chat-window.sh` already uses for interactive session launches.

## Implementation

1. Freeze the Windows `.cmd` launch surface in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Patch `bin/chat-window.ps1` to accept explicit aliases for the hyphenated operator flags
3. Keep the existing `.cmd` wrapper chain unchanged for this loop
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No generic `_cmd-forward-powershell.cmd` redesign
- No `chat-window.sh` behavior change
- No `chat-cli` interactive protocol change
- No GUI automation change

