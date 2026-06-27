# 09BC Chat Window GUI CMD Help GNU-Surface Contract Implementation Plan

## Goal

Make `bin/chat-window-gui.cmd --help` explicitly surface the GNU-style named flags that the Windows GUI wrapper already accepts.

## Implementation

1. Freeze the Windows GUI `.cmd` help boundary in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Extend `bin/chat-window-gui.ps1` help text with a `.cmd` GNU-style usage line
3. Keep GUI runtime behavior unchanged for this loop
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No GUI runtime flow change
- No parser redesign
- No `chat-window.cmd` help change
- No `open-chat-test` help change
