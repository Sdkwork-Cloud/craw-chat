# 09BG Status Local CMD Help GNU-Surface Contract Implementation Plan

## Goal

Make `bin/retired-lifecycle-status.cmd --help` surface the GNU-style Windows named flags that operators are expected to use.

## Implementation

1. Freeze the Windows help surface in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Patch the `-Help` branch in `bin/retired-lifecycle-status.ps1`
3. Keep runtime status behavior unchanged for this loop
4. Backwrite the operator quick-start doc and loop indexes

## Non-Goals

- No runtime status refactor
- No profile-resolution redesign
- No `_cmd-forward-powershell.cmd` redesign
- No unrelated local script changes
