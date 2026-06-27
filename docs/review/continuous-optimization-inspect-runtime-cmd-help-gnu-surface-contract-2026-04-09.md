# Continuous Optimization: Inspect Runtime CMD Help GNU Surface Contract

## Context

- `inspect-runtime-local.cmd` is the documented Windows wrapper for runtime-dir inspection.
- `docs/架构/09-实施计划.md` already includes `cmd /c bin\\inspect-runtime-local.cmd --help` in the Windows verification path.
- `inspect-runtime-local.cmd` already delegates runtime execution through the shared `.cmd` forwarder.

## Confirmed Bug

- `inspect-runtime-local.cmd --help` only surfaced PowerShell usage, so Windows operators could not discover the documented GNU-style flags from the `.cmd` help surface.

## Root Cause

- `bin/inspect-runtime-local.ps1 -Help` printed only the PowerShell usage line.
- The `.cmd` wrapper forwards `--help` correctly, but it can only echo what the PowerShell help branch exposes.

## Fix

- Add an explicit Windows `.cmd` GNU-style usage line to `bin/inspect-runtime-local.ps1 -Help`.
- Add a Windows regression test that executes `bin\\inspect-runtime-local.cmd --help` and requires both:
  - the GNU-style `.cmd` usage
  - the existing PowerShell usage

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_inspect_runtime_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_inspect_runtime_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture
cargo fmt --all
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `inspect-runtime-local.cmd --help` now exposes the same Windows GNU-style operator contract that the repo documents elsewhere.
- The native PowerShell help line remains visible, so no existing PowerShell guidance was removed.

## Boundary

- This loop only closes `inspect-runtime-local.cmd --help`.
- The adjacent runtime-ops wrappers `repair/list/archive/prune/preview/restore` still need their own evidence-driven review instead of being assumed fixed.
