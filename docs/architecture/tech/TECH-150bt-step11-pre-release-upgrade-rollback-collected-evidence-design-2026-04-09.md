> Migrated from `docs/架构/150BT-step11-pre-release-upgrade-rollback-collected-evidence-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Pre-Release Upgrade-Rollback Collected Evidence Design

## Decision

- Promote one already published CP11-3 local `upgrade-rollback` result into the `Pre-Release Tier` artifact root.
- Do not fake any missing `connection`, `message`, or `stream` artifacts.

## State Model

- previous `Pre-Release Tier` state: `evidence_partially_collected`
- new `Pre-Release Tier` state: `evidence_partially_collected`
- newly collected slot: `upgrade_rollback_drill`
- total collected slots: `4`
- `Capacity Tier` state: `template_only_pending_execution`

## Artifact Contract

- artifact path: `artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json`
- collected metric snapshot: `rollbackActivationMs = 0.007`
- required fields preserved:
  - `runId`
  - `compatibilityMatrixPassRate`
  - `rollbackActivationSeconds`
  - `postRollbackProtocolErrorRate`
- supporting fields:
  - `safeClientCount`
  - `compatibleClientCount`
  - `killSwitchPropagationSuccessRate`
  - `blockedBinding`
  - `disabledCapability`
  - `sourceBaselinePath`
  - `sourceTestPath`
  - `sourceReviewId`

## Boundary

- `rollbackActivationSeconds = 0.000007` is derived from the published local upgrade-rollback drill evidence.
- This artifact is a truthful partial collection record, not full `Pre-Release Tier` sign-off.
- It does not change `Capacity Tier`.

