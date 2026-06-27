> Migrated from `docs/架构/150CA-shell-process-identity-portability-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Shell Process Identity Portability Design

## Decision

- Bash lifecycle scripts must identify managed processes from full argv, not from `ps` command-name fields that may be truncated by platform.

## State Model

- source field: `ps -p "$pid" -o args=`
- normalization: trim leading whitespace, take argv[0], then take basename
- expected identity: `sdkwork-im-server`

## Contract

- `pnpm dev:server`, `bin/retired-lifecycle-status.sh`, and `bin/retired-lifecycle-stop.sh` must use `ps -o args=`.
- They must derive argv[0] before basename comparison.
- A managed process remains managed even when the platform truncates `comm`.
- PID ownership checks stay strict: only basename `sdkwork-im-server` is accepted.

## Boundary

- This design standardizes Bash lifecycle identity matching only.
- Native Linux/macOS runtime proof remains a separate execution concern.

