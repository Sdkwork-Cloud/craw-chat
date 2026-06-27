> Migrated from `docs/superpowers/specs/2026-04-14-local-disk-persistence-recovery-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Local Disk Persistence Recovery Design

**Date:** 2026-04-14

**Goal:** Eliminate state-loss windows in the shared local-disk persistence layer that underpins commit journals, realtime checkpoints, subscriptions, stream state, RTC state, notifications, automation execution, and presence state.

## Problem

The shared JSON persistence helper in `adapters/local-disk/src/shared.rs` currently writes a temporary file, deletes the live file, and then renames the temporary file into place. That sequence creates a real failure window:

1. Temp file is fully written.
2. Live file is deleted.
3. Process exits or host crashes before rename.

After that sequence, only the `.tmp` file remains. Current reads ignore the temp file and return default state when the live file is missing, which can silently discard durable runtime state.

## Options Considered

### Option 1: Keep current write flow and rely on runtime repair tools

- Pros: No code change in storage layer.
- Cons: Runtime can still lose state between writes and repair is reactive, not preventive.

### Option 2: Recover `.tmp` files on read but keep delete-then-rename

- Pros: Fixes one interrupted-write scenario.
- Cons: Still leaves a deliberate state-loss window every write.

### Option 3: Replace shared write flow with atomic-ish temp write + direct rename, plus temp recovery on read

- Pros: Centralized fix, best protection, automatically benefits all local-disk stores.
- Cons: Requires touching shared persistence helpers and expanding regression coverage.

**Recommendation:** Option 3.

## Design

### Write Path

- Keep writing serialized JSON to a sibling temp file.
- Flush and sync the temp file before replacement.
- Replace the live file via `fs::rename(temp, live)` without deleting the live file first.
- On platforms where rename can replace the destination directly, this removes the empty-file window.

### Read Path

- Before reading, inspect the sibling temp file.
- If the live file is missing and the temp file exists, promote the temp file into place and continue reading.
- If both live and temp files exist, remove the stale temp file and keep the live file.

### Error Handling

- Preserve existing `ContractError::Unavailable(...)` behavior for unreadable or invalid JSON.
- Do not silently swallow parse failures for live files in the local-disk adapter. Recovery here targets interrupted writes, not data corruption masking.

### Test Coverage

- Add a failing regression test proving a commit journal recovers when only the temp file remains.
- Add a failing regression test proving a checkpoint store recovers when only the temp file remains.
- Add a failing regression test proving an existing live file wins over a stale temp file.

## Scope

In scope:

- `adapters/local-disk/src/shared.rs`
- `adapters/local-disk/src/tests.rs`

Out of scope:

- Runtime-dir repair report format changes
- Cross-process locking redesign
- Corrupt live-file quarantine for non-auth stores

