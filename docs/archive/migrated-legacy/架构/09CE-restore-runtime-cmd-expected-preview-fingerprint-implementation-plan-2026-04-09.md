# Restore Runtime CMD Expected Preview Fingerprint Implementation Plan

## Goal

- Close the Step 10 Windows restore wrapper drift so `.cmd` preserves the documented preview fingerprint confirmation flag.

## Steps

- Add a failing Windows wrapper regression in `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`.
- Extend the deployment asset contract assertion for `bin/_cmd-forward-powershell.cmd`.
- Normalize `--expected-preview-fingerprint` to `-ExpectedPreviewFingerprint` inside the shared `.cmd` forwarder.
- Re-run targeted, deployment-profile, package, and formatting verification.

## Boundary

- This plan only changes wrapper argument normalization.
- It does not change restore preview semantics, fingerprint validation logic, or runtime-dir topology ownership.
