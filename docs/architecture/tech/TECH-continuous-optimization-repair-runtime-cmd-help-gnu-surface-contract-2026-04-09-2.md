> Migrated from `docs/step/continuous-optimization-repair-runtime-cmd-help-gnu-surface-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Repair Runtime CMD Help GNU Surface Contract

## Goal

- Freeze the Windows `repair-runtime-local.cmd --help` surface so it exposes the documented GNU-style operator flags.

## Scope

- Modify:
  - `bin/repair-runtime-local.ps1`
  - `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`

## Implementation

- Add a failing Windows regression test for `repair-runtime-local.cmd --help`.
- Add the `.cmd` GNU-style usage line to the PowerShell help branch.
- Re-run the exact test, the deployment-profile suite, formatting, and the package tests.

## Expected State

- `repair-runtime-local.cmd --help` shows:
  - `cmd /c .\bin\repair-runtime-local.cmd [--profile ...] [--runtime-dir <path>] [--json] [--release]`
  - `powershell -ExecutionPolicy Bypass -File bin/repair-runtime-local.ps1 ...`

## Boundary

- This loop does not change repair behavior.
- This loop only fixes help discoverability for the Windows wrapper.

