> Migrated from `docs/review/continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Open Chat Test CMD Help GNU-Surface Contract

## Context

- Current loop: continue after Step 12 closure and only close the next real Windows operator gap.
- Surface: `bin/open-chat-test.cmd --help`
- Contract: the Windows `.cmd` help surface must explicitly show the GNU-style scripted-validation flags that the wrapper already accepts.

## Gap

- `open-chat-test.cmd` already accepted the documented GNU-style named flags on Windows.
- It also already preserved `--validation-message` literally across the `.cmd` wrapper boundary.
- Its help text still came entirely from `open-chat-test.ps1` and only showed PowerShell-style parameter names such as `-OwnerUserId`, `-ScriptedValidation`, and `-ValidationMessage`.
- That created a discoverability split:
  - runtime behavior accepted `--owner-user-id`, `--guest-user-id`, `--scripted-validation`, `--validation-message`, and `--json`
  - help output still taught operators a different PowerShell-only surface

## Decision

- Keep the existing parameter parsing unchanged for this loop.
- Extend `open-chat-test.ps1` help text with an explicit Windows `.cmd` usage line that shows the GNU-style named flags.
- Treat `.cmd --help` as part of the Step 12 Windows scripted-validation contract.

## Changed Files

- `bin/open-chat-test.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- `docs/review/continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BB-open-chat-test-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BB-open-chat-test-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_open_chat_test_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
```

- Failed before the patch because `open-chat-test.cmd --help` only printed `-OwnerUserId`, `-ScriptedValidation`, and `-ValidationMessage` style usage.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
cmd /c .\bin\open-chat-test.cmd --help
```

## Remaining Gap

- This loop only closes the Windows `open-chat-test.cmd --help` discoverability boundary.
- Other wrapper help surfaces still need to be reviewed independently instead of assuming global consistency.

