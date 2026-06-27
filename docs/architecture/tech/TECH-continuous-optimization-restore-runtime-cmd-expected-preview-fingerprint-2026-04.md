> Migrated from `docs/review/continuous-optimization-restore-runtime-cmd-expected-preview-fingerprint-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Restore Runtime CMD Expected Preview Fingerprint

## Context

- `restore-runtime-local.cmd` is the documented Windows wrapper for runtime-dir restore.
- `docs/部署/快速启动脚本.md` already freezes `--expected-preview-fingerprint <fingerprint>` as part of the restore operator contract.
- `restore-runtime-local.ps1` and `restore-runtime-local.sh` already preserve this confirmation flag.

## Confirmed Bug

- Windows `.cmd` dropped `--expected-preview-fingerprint`, so the documented restore confirmation guard was not forwarded through the wrapper.

## Root Cause

- `bin/_cmd-forward-powershell.cmd` normalized `--backup-dir`, `--profile`, `--json`, and other GNU-style flags, but had no mapping for `--expected-preview-fingerprint`.

## Fix

- Add `/expectedPreviewFingerprint`, `--expected-preview-fingerprint`, and `--expectedPreviewFingerprint` aliases to `_cmd-forward-powershell.cmd`, all normalized to `-ExpectedPreviewFingerprint`.
- Extend the static deployment asset contract test so the forwarder must keep advertising this mapping.
- Add a Windows runtime wrapper regression test that executes `restore-runtime-local.cmd` against a stub `restore-runtime-local.ps1` and verifies the forwarded fingerprint value.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_restore_runtime_local_cmd_normalizes_expected_preview_fingerprint_switch -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_restore_runtime_local_cmd_normalizes_expected_preview_fingerprint_switch -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture
cargo fmt --all
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- Windows restore wrappers now preserve the same preview-confirmation guard already enforced on PowerShell and Bash.
- The shared `.cmd` forwarder can no longer silently strip this restore safety parameter.

## Boundary

- This loop proves Windows `.cmd` forwarding only.
- Native Bash runtime execution is still only partially evidenced in this Windows session because some `.sh` runtime regressions remain environment-skipped without a usable Bash runtime.

