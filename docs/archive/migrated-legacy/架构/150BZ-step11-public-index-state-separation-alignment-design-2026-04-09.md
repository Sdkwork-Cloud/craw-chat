# Step 11 Public Index State Separation Alignment Design

## Decision

- Public index docs must describe the shared current truth, not only carry partial state fragments.

## State Model

- `Pre-Release Tier`: `evidence_collected_gate_blocked`
- `Capacity Tier`: `template_only_pending_execution`
- shared boundary: only `Capacity Tier` still waits for real collection

## Contract

- Every public index must contain:
  - `Pre-Release Tier current state is now evidence_collected_gate_blocked`
  - `Capacity Tier current state remains template_only_pending_execution`
  - `Only Capacity Tier still waits for real collection; Pre-Release Tier already carries all seven truthful local artifacts.`

## Boundary

- This is a documentation-contract alignment.
- It does not change historical phase records or capacity execution status.
