# Continuous Optimization: Step 11 Public Index State Separation Alignment

## Context

- Public index docs already exposed `Pre-Release Tier current state is now evidence_collected_gate_blocked`.
- The same public surfaces still kept older template-era wording that did not clearly separate `Pre-Release Tier` from `Capacity Tier`.

## Confirmed Bug

- Shared index docs could be read as if both high tiers were still template-only.
- The public contract did not explicitly say that only `Capacity Tier` still remains `template_only_pending_execution`.

## Root Cause

- Earlier loops added the final `Pre-Release Tier` state but did not add an equally explicit shared-state line for `Capacity Tier`.
- Public indexes kept partial truth instead of the full current split-state summary.

## Fix

- Add the same two shared-state lines to:
  - `README.md`
  - `docs/部署/README.md`
  - `docs/review/README.md`
  - `docs/架构/README.md`
  - `docs/step/README.md`
- New shared-state lines:
  - `Capacity Tier current state remains template_only_pending_execution`
  - `Only Capacity Tier still waits for real collection; Pre-Release Tier already carries all seven truthful local artifacts.`
- Add a regression test that requires all five public indexes to carry the split-state summary.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_separates_pre_release_and_capacity_current_states_in_public_indexes -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_separates_pre_release_and_capacity_current_states_in_public_indexes -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- Public index docs now separate the current states of `Pre-Release Tier` and `Capacity Tier`.
- The shared contract is harder to misread during later review loops.

## Next Gap

- Continue the review loop on cross-platform runtime claims, especially Bash-script verification outside the current Windows environment.
