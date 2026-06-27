> Migrated from `docs/superpowers/plans/2026-04-14-local-disk-persistence-recovery.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Local Disk Persistence Recovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove the local-disk shared persistence state-loss window and recover interrupted temp-file writes across restart.

**Architecture:** Centralize the fix in `adapters/local-disk/src/shared.rs` so every JSON-backed store inherits the safer behavior. Preserve current store-specific semantics while hardening the shared read and write lifecycle.

**Tech Stack:** Rust, serde_json, std::fs, unit tests in `adapters/local-disk`

---

### Task 1: Add Failing Persistence-Recovery Tests

**Files:**
- Modify: `adapters/local-disk/src/tests.rs`
- Test: `adapters/local-disk/src/tests.rs`

- [ ] **Step 1: Write the failing tests**

Add unit tests that:
- create only `<store>.json.tmp` and verify reopen/load recovers the pending write
- create both `<store>.json` and `<store>.json.tmp` and verify the live file remains authoritative

- [ ] **Step 2: Run tests to verify they fail**

Run:

```powershell
$env:CARGO_TARGET_DIR='<workspace-root>\sdkwork-im\target\tdd-local-disk-red'; cargo test -p im-adapters-local-disk pending_tmp -- --nocapture
```

Expected: failures because the current shared reader ignores pending temp files.

- [ ] **Step 3: Commit the test-only red phase if needed**

No commit required for the red-only checkpoint in this session.

### Task 2: Implement Shared Recovery and Safer Replacement

**Files:**
- Modify: `adapters/local-disk/src/shared.rs`
- Test: `adapters/local-disk/src/tests.rs`

- [ ] **Step 1: Implement minimal recovery helpers**

Add helpers that:
- derive the temp-file path
- promote temp to live when live is missing
- delete stale temp when live already exists

- [ ] **Step 2: Remove the delete-before-rename window**

Update the shared writer to:
- create and sync the temp file
- rename temp onto live without deleting live first

- [ ] **Step 3: Run the focused tests**

Run:

```powershell
$env:CARGO_TARGET_DIR='<workspace-root>\sdkwork-im\target\tdd-local-disk-green'; cargo test -p im-adapters-local-disk pending_tmp -- --nocapture
```

Expected: the new recovery tests pass.

### Task 3: Run Broader Regression Coverage

**Files:**
- Verify: `adapters/local-disk/src/tests.rs`
- Verify: `services/local-minimal-node/tests/real_auth_restart_e2e_test.rs`
- Verify: `services/local-minimal-node/tests/real_auth_e2e_test.rs`

- [ ] **Step 1: Run local-disk adapter tests**

Run:

```powershell
$env:CARGO_TARGET_DIR='<workspace-root>\sdkwork-im\target\verify-local-disk'; cargo test -p im-adapters-local-disk
```

- [ ] **Step 2: Run runtime smoke regressions that exercise persisted state**

Run:

```powershell
$env:CARGO_TARGET_DIR='<workspace-root>\sdkwork-im\target\verify-local-disk-restart'; cargo test -p local-minimal-node --test real_auth_restart_e2e_test
$env:CARGO_TARGET_DIR='<workspace-root>\sdkwork-im\target\verify-local-disk-auth'; cargo test -p local-minimal-node --test real_auth_e2e_test
```

- [ ] **Step 3: Commit**

```powershell
git add adapters/local-disk/src/shared.rs adapters/local-disk/src/tests.rs docs/superpowers/specs/2026-04-14-local-disk-persistence-recovery-design.md docs/superpowers/plans/2026-04-14-local-disk-persistence-recovery.md
git commit -m "fix(storage): recover pending local-disk temp writes"
```

