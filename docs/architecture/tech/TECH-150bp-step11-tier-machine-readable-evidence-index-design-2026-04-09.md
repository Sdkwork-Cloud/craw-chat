> Migrated from `docs/架构/150BP-step11-tier-machine-readable-evidence-index-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Tier Machine-Readable Evidence Index Design

## Problem

Step 11 high-tier machine-readable detail existed only in `tools/perf/step-11-*-tier-gate.json`, not beside the actual artifact roots.

## Scope

- In scope:
  - one shared Step 11 tier evidence-index schema
  - one co-located evidence index per high-tier artifact root
  - placeholder checksum and file-list entrypoints
- Out of scope:
  - real collected high-tier evidence
  - new Step 11 scenario families or metrics
  - changing the existing tier gate template semantics

## Design

- Keep the gate templates as the source authoring contract.
- Mirror their operator-relevant content into co-located tier evidence indexes under:
  - `artifacts/perf/step-11/pre-release`
  - `artifacts/perf/step-11/capacity`
- Add:
  - `checksum-manifest.txt`
  - `artifact-file-list.txt`
- Keep state explicit:
  - top-level `template_only_pending_execution`
  - slot-level `pending_collection`

## Acceptance

- `step-11-tier-evidence-index.schema.json` exists.
- Both high-tier artifact roots contain an evidence index JSON file.
- Both high-tier artifact roots contain placeholder checksum and file-list files.
- The new regression test passes.

