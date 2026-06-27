# Continuous Optimization: Chat Window GUI CMD Label Special-Character Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `chat-window-gui.cmd --help` had already been aligned in the previous micro-loop.
- The next real gap was runtime literal fidelity, not help discoverability.
- `chat-window-gui.cmd` still stripped `!` from `-Label` at the wrapper boundary, which made the visible Windows GUI wrapper inconsistent with direct PowerShell launches.

## Closure Target

1. Add a Windows regression that launches `bin/chat-window-gui.cmd` with `-Label guest!`.
2. Prove diagnostics preserve the exact label value across the wrapper boundary.
3. Patch only the smallest wrapper seam needed for literal fidelity.
4. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_window_gui_cmd_wrapper_preserves_exclamation_mark_in_label`
- Reproduced the real Windows label-fidelity gap first
- Replaced `chat-window-gui.cmd`'s forwarder hop with a direct PowerShell invocation
- Re-ran the GUI help regression to confirm the previous help contract stayed green
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_preserves_exclamation_mark_in_label -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\chat-window-gui.cmd --help
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real wrapper contract mismatch in help discoverability, literal argument fidelity, or cross-shell parity.
