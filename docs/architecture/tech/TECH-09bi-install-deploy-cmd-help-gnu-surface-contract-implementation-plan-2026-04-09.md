> Migrated from `docs/架构/09BI-install-deploy-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09BI Install/Deploy CMD Help GNU-Surface Contract Implementation Plan

## Goal

Make `(retired lifecycle script) --help` and `pnpm dev --help` surface the GNU-style Windows named flags that operators are expected to use.

## Implementation

1. Freeze both Windows help surfaces in `services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
2. Patch the `-Help` branches in `(retired lifecycle script)` and `pnpm dev`
3. Keep forwarding and runtime execution behavior unchanged for this loop
4. Backwrite review, step, and architecture indexes

## Non-Goals

- No launcher refactor
- No `_cmd-forward-powershell.cmd` redesign
- No install/deploy runtime behavior change
- No unrelated Windows wrapper edits

