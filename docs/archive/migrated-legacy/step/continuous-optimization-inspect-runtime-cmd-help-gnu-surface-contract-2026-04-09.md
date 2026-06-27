# Continuous Optimization: Inspect Runtime CMD Help GNU Surface Contract

## Goal

- Close the Step 10 Windows discoverability gap so `inspect-runtime-local.cmd --help` exposes the documented GNU-style operator usage.

## Scope

- `bin/inspect-runtime-local.ps1`
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`

## Implementation

- Add a failing Windows help-surface regression for `inspect-runtime-local.cmd --help`.
- Extend `inspect-runtime-local.ps1 -Help` with a `.cmd` GNU-style usage line.
- Re-run targeted, deployment-profile, package, and formatting verification.

## Expected State

- Windows operators can discover `--profile`, `--runtime-dir`, `--json`, and `--release` directly from the `.cmd` help surface.
- The existing PowerShell usage remains visible.

## Boundary

- This loop does not change runtime inspection semantics.
- This loop does not yet cover `repair/list/archive/prune/preview/restore` `.cmd --help` surfaces.
