# Continuous Optimization: Repair Runtime CMD Help GNU Surface Contract

## Context

- `repair-runtime-local.cmd` is part of the documented Windows runtime-ops surface.
- Earlier loops already closed the same help-surface gap for `start/status/install/deploy/inspect`.

## Confirmed Bug

- `cmd /c .\bin\repair-runtime-local.cmd --help` only printed the PowerShell usage line.
- Windows operators could not discover the documented GNU-style `.cmd` flags from the wrapper entrypoint itself.

## Root Cause

- `bin/repair-runtime-local.ps1 -Help` printed only the PowerShell usage.
- The `.cmd` forwarder preserved `--help`, but it can only expose what the PowerShell help branch emits.

## Fix

- Add a `.cmd` GNU-style usage line to `bin/repair-runtime-local.ps1`.
- Add a regression test that executes `repair-runtime-local.cmd --help` and requires both:
  - Windows `.cmd` usage
  - native PowerShell usage

## Verification

- Red:
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_repair_runtime_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture`
- Green:
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_repair_runtime_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture`

## Next

- Continue the same evidence-driven pass for the adjacent runtime-ops wrappers:
  - `list-runtime-backups-local`
  - `archive-runtime-backup-local`
  - `prune-runtime-archives-local`
  - `preview-runtime-restore-local`
  - `restore-runtime-local`
