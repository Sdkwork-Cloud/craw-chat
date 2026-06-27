> Migrated from `docs/step/continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Open Chat Test CMD GNU-Flag Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `open-chat-test.ps1` and `open-chat-test.sh` already defined a usable scripted-validation surface.
- The Windows `.cmd` entry still accepted the command but silently discarded GNU-style named flags.
- That made the Windows operator path unreliable for automation even though the underlying scripted-validation flow already existed.

## Closure Target

1. Add a Windows e2e regression that calls `bin/open-chat-test.cmd` with GNU-style named flags.
2. Prove the command returns scripted-validation JSON instead of falling back to the interactive window flow.
3. Patch only the smallest contract seam needed for `.cmd` parity.
4. Backwrite the operator matrix and validation index.

## Actual Delivery

- Added `test_open_chat_test_cmd_wrapper_accepts_gnu_style_named_flags_for_scripted_validation`
- Reproduced the real Windows failure first
- Added GNU-style aliases for the hyphenated `open-chat-test.ps1` operator parameters:
  - `base-url`
  - `tenant-id`
  - `conversation-id`
  - `owner-user-id`
  - `guest-user-id`
  - `owner-label`
  - `guest-label`
  - `skip-start`
  - `use-console-windows`
  - `scripted-validation`
  - `validation-message`
- Updated the Step 12 operator guide and validation index
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_wrapper_accepts_gnu_style_named_flags_for_scripted_validation -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real contract mismatch in wrapper parity, scripted-validation ergonomics, or operator discoverability.

