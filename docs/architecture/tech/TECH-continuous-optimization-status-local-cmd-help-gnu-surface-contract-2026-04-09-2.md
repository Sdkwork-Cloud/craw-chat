> Migrated from `docs/step/continuous-optimization-status-local-cmd-help-gnu-surface-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Status Local CMD Help GNU-Surface Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `retired-lifecycle-status` belongs to the same local operator flow as `retired-lifecycle-start`.
- The docs already exposed GNU-style Windows flags for status inspection.
- The remaining real gap was that `retired-lifecycle-status.cmd --help` still only surfaced PowerShell syntax.

## Closure Target

1. Add a Windows regression for `bin/retired-lifecycle-status.cmd --help`.
2. Prove the wrapper currently hides the GNU-style named flags.
3. Patch only the help text surface.
4. Backwrite review, step, architecture, and operator docs for this micro-loop.

## Actual Delivery

- Added `test_status_local_cmd_help_surfaces_gnu_style_named_flags`
- Reproduced the real help-surface gap first
- Added a `.cmd` GNU-style usage line to the `-Help` branch in `bin/retired-lifecycle-status.ps1`
- Updated the local operator quick-start doc
- Backwrote review and architecture docs for this micro-loop

## Verification

```powershell
cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test test_status_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cmd /c .\bin\retired-lifecycle-status.cmd --help
cargo test -p sdkwork-im-cli --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from Step 12 Windows wrapper seams.
- Prefer the next smallest real mismatch in help discoverability, GNU-style runtime acceptance, or literal argument fidelity.

