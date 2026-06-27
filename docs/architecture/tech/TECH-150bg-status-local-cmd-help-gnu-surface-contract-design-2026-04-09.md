> Migrated from `docs/架构/150BG-status-local-cmd-help-gnu-surface-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150BG Status Local CMD Help GNU-Surface Contract Design

## Problem

The Windows `retired-lifecycle-status.cmd` wrapper is a documented local operator entrypoint, but its local `--help` output only surfaced PowerShell-style arguments.
That created a discoverability mismatch:

- docs taught `--profile` and `--runtime-dir`
- local Windows help only showed `-ProfileName` and `-RuntimeDir`

## Decision

- Keep the current runtime status flow unchanged.
- Define GNU-style help discoverability as part of the visible Windows `.cmd` operator contract.
- Add one explicit `.cmd` usage line to `bin/retired-lifecycle-status.ps1` help output.

## Rationale

- This is the minimum change that makes the published Windows operator surface truthful.
- It keeps the contract near the script that owns help generation.
- It avoids a broader wrapper redesign while still closing the real user-facing gap.

## Boundary

- This design only covers `retired-lifecycle-status.cmd --help` discoverability on Windows.
- It does not redefine the generic strategy for unrelated wrappers.

