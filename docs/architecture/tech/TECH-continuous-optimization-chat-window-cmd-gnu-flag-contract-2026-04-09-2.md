> Migrated from `docs/step/continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Chat Window CMD GNU-Flag Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `chat-window.sh` already defined a usable GNU-style operator launch surface for local interactive validation.
- The Windows `.cmd` entry still funneled through `chat-window.ps1`, which only accepted PowerShell-style parameter names.
- That made the Windows operator path unreliable even though the underlying `chat-session` flow already worked.

## Closure Target

1. Add a Windows e2e regression that calls `bin/chat-window.cmd` with GNU-style named flags.
2. Prove the command enters the interactive session and persists the prefixed message into the conversation timeline.
3. Patch only the smallest contract seam needed for `.cmd` parity.
4. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_window_cmd_wrapper_accepts_gnu_style_named_flags_for_interactive_session`
- Reproduced the real Windows failure first
- Added GNU-style aliases for the hyphenated `chat-window.ps1` operator parameters:
  - `base-url`
  - `tenant-id`
  - `conversation-id`
  - `user-id`
  - `session-id`
  - `device-id`
  - `message-prefix`
- Verified the Windows wrapper can now launch `chat-session` and keep `--message-prefix` effective in the stored timeline message
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_accepts_gnu_style_named_flags_for_interactive_session -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real contract mismatch in wrapper parity, scripted-validation ergonomics, or operator discoverability.

