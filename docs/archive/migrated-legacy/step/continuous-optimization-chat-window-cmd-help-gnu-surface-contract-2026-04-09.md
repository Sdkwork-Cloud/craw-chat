# Continuous Optimization: Chat Window CMD Help GNU-Surface Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `chat-window.cmd` already accepted the documented GNU-style named flags on Windows.
- The remaining real gap was operator discoverability, not runtime parsing.
- `chat-window.cmd --help` still surfaced only PowerShell-style parameter names, which made the Windows wrapper look inconsistent with the actual `.cmd` contract.

## Closure Target

1. Add a Windows regression that calls `bin/chat-window.cmd --help`.
2. Prove the help surface explicitly includes the GNU-style named flags used by the Windows wrapper.
3. Patch only the smallest help seam needed for discoverability parity.
4. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_window_cmd_help_surfaces_gnu_style_named_flags`
- Reproduced the real Windows discoverability gap first
- Extended `chat-window.ps1` help text with an explicit Windows `.cmd` GNU-style usage line
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\chat-window.cmd --help
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real wrapper contract mismatch in help discoverability, literal argument fidelity, or cross-shell parity.
