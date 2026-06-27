# Continuous Optimization: Chat CLI CMD Help Pass-Through Contract

## Context

- Current loop: continue after Step 12 closure, only close a real remaining gap.
- Surface: Windows CLI wrapper chain `bin/chat-cli.cmd -> bin/chat-cli-local.cmd -> bin/chat-cli-local.ps1`
- Contract: `bin/chat-cli.cmd --help`

## Gap

- `bin/chat-cli-local.cmd` reused the generic PowerShell forwarder `_cmd-forward-powershell.cmd`.
- That forwarder normalized `--help` into `-Help`.
- `sdkwork-im-cli` does not accept `-Help`, so `bin/chat-cli.cmd --help` failed with `unknown global flag: -Help`.

## Decision

- `chat-cli` wrappers must forward raw CLI arguments unchanged.
- Keep PowerShell bypass behavior in the `.cmd` entry.
- Remove the generic wrapper normalization step from `bin/chat-cli-local.cmd`.

## Changed Files

- `bin/chat-cli-local.cmd`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/step/continuous-optimization-chat-cli-cmd-help-pass-through-contract-2026-04-09.md`
- `docs/架构/09AV-chat-cli-cmd-help-pass-through-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150AV-chat-cli-cmd-help-pass-through-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_cli_cmd_wrapper_preserves_help_contract -- --exact --nocapture
```

- Failed before the patch because the `.cmd` wrapper rewrote `--help` to `-Help`.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_chat_cli_cmd_wrapper_preserves_help_contract -- --exact --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only fixed the Windows `.cmd` help pass-through seam.
- Step 12 remains in continuous-optimization mode and should continue from the next smallest honest CLI or operator gap.
