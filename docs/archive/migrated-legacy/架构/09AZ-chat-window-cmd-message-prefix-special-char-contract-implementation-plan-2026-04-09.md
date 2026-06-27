# 09AZ Chat Window CMD Message-Prefix Special-Char Contract Implementation Plan

## Goal

Make `bin/chat-window.cmd` preserve `--message-prefix` literally on Windows interactive runs, including `!`.

## Implementation

1. Freeze the Windows `.cmd` special-character boundary in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Replace the `chat-window.cmd -> _cmd-forward-powershell.cmd` hop with a direct call to `chat-window.ps1`
3. Re-verify the existing GNU-style named flag regression after the wrapper change
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No generic `_cmd-forward-powershell.cmd` redesign
- No `chat-window.ps1` launch behavior change
- No `open-chat-test` wrapper change
- No GUI automation change
