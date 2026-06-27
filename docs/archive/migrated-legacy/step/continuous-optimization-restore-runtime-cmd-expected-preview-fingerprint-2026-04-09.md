# Continuous Optimization: Restore Runtime CMD Expected Preview Fingerprint

## Goal

- Close the Step 10 Windows restore-wrapper gap so `restore-runtime-local.cmd` preserves the documented preview fingerprint confirmation flag.

## Scope

- `bin/_cmd-forward-powershell.cmd`
- `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`

## Implementation

- Add a failing Windows wrapper regression for `bin\restore-runtime-local.cmd --expected-preview-fingerprint`.
- Freeze the shared forwarder contract so `_cmd-forward-powershell.cmd` must contain the restore fingerprint alias.
- Normalize the documented GNU-style flag to `-ExpectedPreviewFingerprint`.
- Re-run targeted, deployment-profile, package, and formatting verification.

## Expected State

- Windows operators can run the documented restore confirmation flow through `.cmd` without falling back to PowerShell-only parameter names.
- Restore preview fingerprint verification stays aligned across PowerShell, CMD, and Bash entrypoints.

## Boundary

- No change to restore preview generation, restore semantics, or profile topology.
- No new native Bash runtime proof is claimed from this Windows-only session.
