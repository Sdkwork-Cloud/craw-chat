> Migrated from `docs/架构/09CA-shell-process-identity-portability-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Shell Process Identity Portability Implementation Plan

## Goal

- Close the cross-platform lifecycle drift where Bash scripts can misclassify a managed `sdkwork-im-server` process on shells that truncate `ps -o comm=`.

## Steps

- Freeze the bug with a failing deployment-profile regression test that requires `ps -o args=` plus argv[0] basename extraction.
- Update `pnpm dev:server`, `bin/retired-lifecycle-status.sh`, and `bin/retired-lifecycle-stop.sh` to parse the executable basename from full argv output.
- Re-run the targeted regression, the full deployment-profile test file, formatting, and the `sdkwork-im-server` offline suite.
- Backwrite the bug, fix, boundary, and next verification gap into `docs/review` and `docs/step`.

## Boundary

- Keep the fix limited to Bash lifecycle identity detection.
- Do not change runtime profile selection, health semantics, or PowerShell/CMD flows in this loop.

