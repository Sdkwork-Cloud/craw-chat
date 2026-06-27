# 09BD Chat Window GUI CMD Label Special-Character Contract Implementation Plan

## Goal

Make `bin/chat-window-gui.cmd` preserve `-Label` / `--label` literally when launching the Windows GUI chat window.

## Implementation

1. Freeze the Windows GUI `.cmd` literal-fidelity boundary in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Replace `bin/chat-window-gui.cmd`'s forwarder hop with a direct PowerShell invocation
3. Keep `bin/chat-window-gui.ps1` unchanged for this loop
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No GUI script refactor
- No generic forwarder redesign
- No `chat-window-gui.ps1` parameter redesign
- No unrelated wrapper changes
