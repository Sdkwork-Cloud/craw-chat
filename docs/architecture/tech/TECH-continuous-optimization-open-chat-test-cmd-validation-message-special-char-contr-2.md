> Migrated from `docs/step/continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Open Chat Test CMD Validation-Message Special-Char Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `open-chat-test.cmd` already accepted the documented GNU-style named flags on Windows.
- The remaining real gap was argument fidelity, not argument recognition.
- When the scripted-validation message contained `!`, the Windows `.cmd` path silently stripped it before the message reached `open-chat-test.ps1`.

## Closure Target

1. Add a Windows e2e regression that calls `bin/open-chat-test.cmd` with `--validation-message` containing `!`.
2. Prove the returned JSON summary keeps the exact operator input.
3. Patch only the smallest wrapper seam needed for literal message preservation.
4. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_open_chat_test_cmd_wrapper_preserves_exclamation_mark_in_validation_message`
- Reproduced the real Windows failure first
- Changed `open-chat-test.cmd` to invoke `open-chat-test.ps1` directly instead of routing through `_cmd-forward-powershell.cmd`
- Re-verified the existing GNU-style named flag scripted-validation regression stayed green after the wrapper change
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_preserves_exclamation_mark_in_validation_message -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_accepts_gnu_style_named_flags_for_scripted_validation -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real wrapper or operator contract mismatch, especially where shared Windows forwarding can still alter user-supplied values.

