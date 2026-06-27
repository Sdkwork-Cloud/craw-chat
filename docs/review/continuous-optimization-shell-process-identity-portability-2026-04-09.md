# Continuous Optimization: Shell Process Identity Portability

## Context

- `pnpm dev:server`, `bin/retired-lifecycle-status.sh`, and `bin/retired-lifecycle-stop.sh` used `ps -p "$pid" -o comm=` and exact `sdkwork-im-server` matching.
- On BSD/macOS, `comm` may truncate long command names, so a live managed process can be misread as stale or foreign.

## Confirmed Bug

- `retired-lifecycle-status.sh` could report `stopped` for a live managed process.
- `retired-lifecycle-start.sh` could ignore a live managed process and proceed with a conflicting restart path.
- `retired-lifecycle-stop.sh` could refuse to stop a live managed process because the truncated name no longer matched.

## Root Cause

- The scripts matched a truncation-prone process field instead of full argv.
- They also assumed the returned field already was the executable basename.

## Fix

- Switch the three Bash lifecycle scripts from `ps -o comm=` to `ps -o args=`.
- Trim leading spaces, isolate argv[0], then compare only its basename with `sdkwork-im-server`.
- Add regression test `test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- Bash lifecycle identity matching now follows the executable basename derived from full argv and is safer across Bash environments.
- Same-root-cause scan found no remaining lifecycle `ps -o comm=` check in repo scripts.

## Next Gap

- Run native Bash lifecycle smoke on Linux/macOS or Git Bash to prove runtime behavior beyond the content-level contract verified in this Windows session.
