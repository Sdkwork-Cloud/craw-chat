> Migrated from `docs/架构/09CH-repair-runtime-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# repair-runtime-local.cmd Help GNU Surface Contract Implementation Plan

## Goal

- Close the Windows runtime-ops discoverability gap where `repair-runtime-local.cmd --help` did not expose the documented GNU-style wrapper usage.

## Steps

- Add a failing regression test for `repair-runtime-local.cmd --help`.
- Confirm the current output only contains the PowerShell usage line.
- Add the `.cmd` usage line to `bin/repair-runtime-local.ps1 -Help`.
- Re-run the exact test, `deployment_profile_test`, formatting, and the package tests.

## Boundary

- No runtime repair logic changes.
- No bundled fixes for the other runtime-ops wrappers in this pass.

