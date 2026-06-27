# 09AW Open Chat Test CMD GNU-Flag Contract Implementation Plan

## Goal

Make `bin/open-chat-test.cmd` accept the same GNU-style named flags that `bin/open-chat-test.sh` already uses for scripted validation.

## Implementation

1. Freeze the Windows `.cmd` scripted-validation surface in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Patch `bin/open-chat-test.ps1` to accept explicit aliases for the hyphenated operator flags
3. Keep the existing `.cmd` wrapper chain unchanged for this loop
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No generic `_cmd-forward-powershell.cmd` redesign
- No `open-chat-test.sh` behavior change
- No `chat-cli` parser change
- No new interactive feature
