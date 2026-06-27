> Migrated from `docs/架构/09BA-chat-window-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09BA Chat Window CMD Help GNU-Surface Contract Implementation Plan

## Goal

Make `bin/chat-window.cmd --help` explicitly surface the GNU-style named flags that the Windows wrapper already accepts.

## Implementation

1. Freeze the Windows `.cmd` help boundary in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Extend `bin/chat-window.ps1` help text with a `.cmd` GNU-style usage line
3. Keep runtime parsing unchanged for this loop
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No parser change
- No generic help-system redesign
- No `open-chat-test` help change
- No GUI wrapper change

