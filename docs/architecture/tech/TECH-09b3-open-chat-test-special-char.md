> Migrated from `docs/架构/09AY-open-chat-test-cmd-validation-message-special-char-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09AY Open Chat Test CMD Validation-Message Special-Char Contract Implementation Plan

## Goal

Make `bin/open-chat-test.cmd` preserve `--validation-message` literally on Windows scripted-validation runs, including `!`.

## Implementation

1. Freeze the Windows `.cmd` special-character boundary in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Replace the `open-chat-test.cmd -> _cmd-forward-powershell.cmd` hop with a direct call to `open-chat-test.ps1`
3. Re-verify the existing GNU-style named flag regression after the wrapper change
4. Backwrite the Step 12 operator guide and validation index

## Non-Goals

- No generic `_cmd-forward-powershell.cmd` redesign
- No `open-chat-test.ps1` scripted-validation behavior change
- No `chat-window` wrapper change
- No new operator mode

