# Continuous Optimization: Chat Window GUI CMD GNU-Flag Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `chat-window-gui.cmd --help` had already been aligned to show a GNU-style usage line.
- The remaining real gap was runtime parameter acceptance, not help discoverability.
- `chat-window-gui.cmd` still rejected the hyphenated GNU-style named flags it advertised, which made the visible Windows GUI wrapper contract internally inconsistent.

## Closure Target

1. Add a Windows regression that launches `bin/chat-window-gui.cmd` with GNU-style named flags.
2. Prove the wrapper actually enters the GUI script and writes diagnostics instead of falling back to usage.
3. Patch only the smallest parameter-binding seam needed for runtime parity.
4. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_window_gui_cmd_wrapper_accepts_gnu_style_named_flags_for_launch`
- Reproduced the real Windows GNU-style runtime gap first
- Added GNU-style aliases to `chat-window-gui.ps1` for the hyphenated wrapper parameters
- Re-ran the adjacent GUI help and label-fidelity regressions to confirm previous contracts stayed green
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_accepts_gnu_style_named_flags_for_launch -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_window_gui_cmd_wrapper_preserves_exclamation_mark_in_label -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real wrapper contract mismatch in runtime argument acceptance, literal argument fidelity, or cross-shell parity.
