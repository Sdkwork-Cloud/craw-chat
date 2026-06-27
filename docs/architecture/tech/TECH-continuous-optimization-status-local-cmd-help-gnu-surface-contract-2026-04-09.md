> Migrated from `docs/review/continuous-optimization-status-local-cmd-help-gnu-surface-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Status Local CMD Help GNU-Surface Contract

## Context

- Current loop: continue Step 12 Windows operator seam tightening without claiming repo-wide closure.
- Surface: `bin/retired-lifecycle-status.cmd`
- Contract: the Windows `.cmd` help surface must expose the GNU-style named flags that operators use in local runtime inspection flows.

## Gap

- `docs/部署/快速启动脚本.md` already documented `retired-lifecycle-status` with `--profile` and `--runtime-dir`.
- `bin/retired-lifecycle-status.cmd --help` only showed the PowerShell usage line from `bin/retired-lifecycle-status.ps1`.
- That left a local discoverability split for Windows operators.

## Decision

- Keep the runtime status logic unchanged for this loop.
- Patch only the `-Help` branch in `bin/retired-lifecycle-status.ps1`.
- Add one explicit Windows `.cmd` usage line for `--profile` and `--runtime-dir`.

## Changed Files

- `bin/retired-lifecycle-status.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/快速启动脚本.md`
- `docs/review/continuous-optimization-retired-lifecycle-status-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-retired-lifecycle-status-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BG-retired-lifecycle-status-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BG-retired-lifecycle-status-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_status_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
```

- Failed before the patch because `retired-lifecycle-status.cmd --help` only emitted the PowerShell usage line.

Green:

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_status_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cmd /c .\bin\retired-lifecycle-status.cmd --help
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only closes help discoverability for `retired-lifecycle-status.cmd`.
- Other Windows wrappers such as `retired-lifecycle-install.cmd` and `retired-lifecycle-deploy.cmd` still need independent audits.

