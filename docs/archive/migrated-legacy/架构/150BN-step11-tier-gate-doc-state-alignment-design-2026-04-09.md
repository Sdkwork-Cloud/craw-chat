# Step 11 Tier-Gate Doc State Alignment Design

## Problem

The Step 11 catalog and schema already expose tier-level `artifactRoot`, but one step doc still described that contract as missing.

## Scope

- In scope:
  - correct the stale step narrative
  - add regression coverage for the corrected public wording
- Out of scope:
  - new Step 11 metrics
  - new gate templates
  - real `Pre-Release Tier` or `Capacity Tier` evidence collection

## Design

- Keep `tools/perf/step-11-scenario-catalog.json` unchanged.
- Add an addendum to the existing step doc instead of rewriting history.
- State explicitly that the earlier catalog-gap wording is stale and superseded.
- Freeze the corrected state with a focused test in `performance_drill_catalog_test.rs`.

## Acceptance

- The step doc explicitly says the catalog gap is closed.
- The step doc explicitly says the remaining gap is evidence collection.
- The new regression test passes.
