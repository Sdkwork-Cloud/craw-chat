# Continuous Optimization: Chat CLI CMD Help Pass-Through Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `bin/chat-cli.ps1` already had wrapper coverage, but `bin/chat-cli.cmd` still lacked a real contract test.
- The `.cmd` wrapper chain rewrote `--help` into `-Help`, which broke the most basic CLI entry contract on Windows.
- This was the next smallest honest wrapper gap after the token-surface fixes.

## Closure Target

1. Add a Windows e2e regression that proves `bin/chat-cli.cmd --help` exits successfully and preserves usage output.
2. Patch the `.cmd` wrapper with the minimum pass-through change.
3. Backwrite the Step 12 operator guide and validation index.

## Actual Delivery

- Added `test_chat_cli_cmd_wrapper_preserves_help_contract`
- Fixed `bin/chat-cli-local.cmd` to forward raw CLI arguments directly into `chat-cli-local.ps1`
- Preserved PowerShell `-NoProfile` and `-ExecutionPolicy Bypass` behavior
- Updated:
  - `docs/部署/CLI聊天验证与兼容矩阵.md`
  - `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_cli_cmd_wrapper_preserves_help_contract -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 CLI/operator seams.
- Prefer the next smallest real wrapper, discoverability, or behavior gap instead of broad doc expansion.
