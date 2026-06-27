> Migrated from `docs/step/continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Open Chat Test CMD Help GNU-Surface Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `open-chat-test.cmd` already accepted the documented GNU-style named flags on Windows.
- The remaining real gap was operator discoverability, not runtime parsing or literal message fidelity.
- `open-chat-test.cmd --help` still surfaced only PowerShell-style parameter names, which made the Windows scripted-validation wrapper look inconsistent with the actual `.cmd` contract.

## Closure Target

1. Add a Windows regression that calls `bin/open-chat-test.cmd --help`.
2. Prove the help surface explicitly includes the GNU-style named flags used by the Windows wrapper.
3. Patch only the smallest help seam needed for discoverability parity.
4. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_open_chat_test_cmd_help_surfaces_gnu_style_named_flags`
- Reproduced the real Windows discoverability gap first
- Extended `open-chat-test.ps1` help text with an explicit Windows `.cmd` GNU-style usage line
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\open-chat-test.cmd --help
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real wrapper contract mismatch in help discoverability, literal argument fidelity, or cross-shell parity.

