# Continuous Optimization: Chat Window CMD Message-Prefix Special-Char Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `chat-window.cmd` already accepted the documented GNU-style named flags on Windows.
- The remaining real gap was argument fidelity, not argument recognition.
- When the interactive message prefix contained `!`, the Windows `.cmd` path silently stripped it before the prefix reached `chat-window.ps1`.

## Closure Target

1. Add a Windows e2e regression that calls `bin/chat-window.cmd` with `--message-prefix` containing `!`.
2. Prove the stored conversation timeline keeps the exact prefix.
3. Patch only the smallest wrapper seam needed for literal prefix preservation.
4. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_window_cmd_wrapper_preserves_exclamation_mark_in_message_prefix`
- Reproduced the real Windows failure first
- Changed `chat-window.cmd` to invoke `chat-window.ps1` directly instead of routing through `_cmd-forward-powershell.cmd`
- Re-verified the existing GNU-style named flag interactive regression stayed green after the wrapper change
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_preserves_exclamation_mark_in_message_prefix -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_wrapper_accepts_gnu_style_named_flags_for_interactive_session -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\chat-window.cmd --help
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real wrapper or operator contract mismatch, especially where shared Windows forwarding can still alter user-supplied values.
