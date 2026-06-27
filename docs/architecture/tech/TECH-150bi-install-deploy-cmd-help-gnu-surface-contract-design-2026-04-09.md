> Migrated from `docs/架构/150BI-install-deploy-cmd-help-gnu-surface-contract-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150BI Install/Deploy CMD Help GNU-Surface Contract Design

## Problem

The Windows `retired-lifecycle-install.cmd` and `retired-lifecycle-deploy.cmd` wrappers are documented operator entrypoints, but their local `--help` output only surfaced PowerShell-style arguments.
That created a discoverability mismatch:

- docs taught GNU-style Windows flags
- local Windows help only showed PowerShell syntax

## Decision

- Keep the current forwarding flow unchanged.
- Define GNU-style help discoverability as part of the visible Windows `.cmd` operator contract.
- Add one explicit `.cmd` usage line to each help branch in `(retired lifecycle script)` and `pnpm dev`.

## Rationale

- This is the minimum change that makes the published Windows operator surface truthful.
- It keeps help generation beside the scripts that own the contract.
- It closes the real user-facing gap without introducing another wrapper layer or a generic redesign.

## Boundary

- This design only covers `retired-lifecycle-install.cmd --help` and `retired-lifecycle-deploy.cmd --help` discoverability on Windows.
- It does not redefine the global help strategy for every remaining wrapper.

