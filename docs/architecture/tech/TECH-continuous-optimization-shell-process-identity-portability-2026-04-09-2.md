> Migrated from `docs/step/continuous-optimization-shell-process-identity-portability-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Shell Process Identity Portability

## Goal

- Make Bash lifecycle scripts identify managed `sdkwork-im-server` processes portably across Linux, Git Bash, and BSD/macOS.

## Scope

- Update only `pnpm dev:server`, `bin/retired-lifecycle-status.sh`, `bin/retired-lifecycle-stop.sh`, and the matching deployment-profile regression test.
- Do not change PowerShell or CMD lifecycle behavior in this loop.

## Implementation

- Reproduce the drift with a failing regression test that rejects `ps -o comm=`-based matching.
- Read process identity from `ps -o args=`, trim leading whitespace, isolate argv[0], and compare its basename with `sdkwork-im-server`.
- Re-run the targeted regression, the full `deployment_profile_test`, formatting, and the `sdkwork-im-server` offline suite.

## Expected State

- Bash lifecycle scripts no longer depend on truncation-prone `comm`.
- Managed-process detection uses full argv-derived basename matching.
- Native Linux/macOS runtime execution remains an explicit next verification step, not an implied completion claim.

## Boundary

- This loop fixes process identity only.
- It does not change runtime profile selection, health checks, or claim full native Bash proof from this session.

