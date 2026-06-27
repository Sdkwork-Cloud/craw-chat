> Migrated from `docs/架构/150BO-step11-tier-artifact-root-materialization-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Tier Artifact-Root Materialization Design

## Problem

The Step 11 catalog and tier gate templates already publish `artifacts/perf/step-11/pre-release` and `artifacts/perf/step-11/capacity`, but the repo had no such roots.

## Scope

- In scope:
  - materialize both artifact roots
  - add minimal README guidance inside each root
  - freeze the existence contract with a regression test
- Out of scope:
  - real collected high-tier evidence
  - new metrics or drill scenarios
  - a new Step 11 evidence schema

## Design

- Create `artifacts/perf/step-11/` as the Step 11 high-tier artifact area.
- Create one README per high-tier root.
- Keep both roots explicitly pre-evidence:
  - `template_only_pending_execution`
  - `pending_collection`
- Point operators back to the existing tier gate templates for slot-level detail.

## Acceptance

- `artifacts/perf/step-11/pre-release/README.md` exists.
- `artifacts/perf/step-11/capacity/README.md` exists.
- Both READMEs expose `artifactRoot`, `gateTemplate`, state, slot state, and naming rule.
- The new regression test passes.

