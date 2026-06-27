> Migrated from `docs/架构/09CF-inspect-runtime-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Inspect Runtime CMD Help GNU Surface Contract Implementation Plan

## Goal

- Close the Windows inspect-runtime help discoverability gap so `.cmd --help` matches the documented GNU-style operator contract.

## Steps

- Add a failing regression in `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs` for `inspect-runtime-local.cmd --help`.
- Update `bin/inspect-runtime-local.ps1` to print both PowerShell and `.cmd` usage lines in the help branch.
- Re-run targeted, deployment-profile, package, and formatting verification.

## Boundary

- This plan only changes help discoverability.
- It does not change inspection command execution, runtime-dir resolution, or repair/archive/restore behavior.

