> Migrated from `docs/superpowers/plans/2026-04-05-im-review-and-deployment-hardening.md` on 2026-06-24.
> Owner: SDKWork maintainers

# IM Review And Deployment Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Verify the current `sdkwork-im` implementation against the documented architecture, harden the highest-risk runtime and deployment paths, and leave the local install/start/restart/stop flow directly usable on multiple operating systems.

**Architecture:** Work in short review-and-fix waves. First verify the actual deployment lifecycle scripts and runtime layout, then inspect shared domain semantics that can corrupt session state or break cross-service contracts, add regression tests for every confirmed defect, and finally document the hardened behavior under `docs/`.

**Tech Stack:** Rust workspace, Cargo tests, Axum services, PowerShell/Bash/CMD lifecycle scripts, Markdown architecture docs.

---

### Task 1: Verify Local Deployment Lifecycle

**Files:**
- Verify: `bin/install-local.ps1`
- Verify: `bin/start-local.ps1`
- Verify: `bin/status-local.ps1`
- Verify: `bin/restart-local.ps1`
- Verify: `bin/stop-local.ps1`
- Verify: `bin/*.sh`
- Verify: `bin/*.cmd`
- Verify: `deployments/docker-compose/local-minimal.yml`

- [ ] **Step 1: Run config initialization and install flow**

Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\install-local.ps1`
Expected: config file exists, `.runtime/local-minimal/{config,logs,pids}` exists, `cargo build --offline` succeeds.

- [ ] **Step 2: Run background startup and health verification**

Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\start-local.ps1`
Expected: pid file written, process stays alive, `/healthz` returns `200`.

- [ ] **Step 3: Run status, restart, and stop flow**

Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\status-local.ps1`
Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\restart-local.ps1`
Run: `powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\stop-local.ps1`
Expected: status reports correctly, restart replaces the running process cleanly, stop removes the pid file and leaves no stale process.

- [ ] **Step 4: Add or adjust tests only for confirmed lifecycle gaps**

Target: `services/local-minimal-node/tests/deployment_profile_test.rs`
Expected: asset tests encode the missing lifecycle behavior that was verified manually.

### Task 2: Review RTC State Machine Semantics

**Files:**
- Review/Modify: `services/im-call-runtime/src/lib.rs`
- Test: `services/im-call-runtime/tests/http_smoke_test.rs`
- Test: `services/local-minimal-node/tests/http_e2e_test.rs`

- [ ] **Step 1: Reproduce invalid RTC transitions with failing tests**

Add targeted tests for transitions such as `accepted -> rejected`, `ended -> accepted`, and duplicate terminal operations.

- [ ] **Step 2: Confirm root cause in runtime state mutation**

Trace `create_session`, `accept_session`, `reject_session`, `end_session`, and `post_signal` to verify whether state is overwritten without transition guards.

- [ ] **Step 3: Implement minimal state-machine enforcement**

Allow only valid transitions, preserve idempotent retries, and return explicit conflict or invalid-state errors for conflicting updates.

- [ ] **Step 4: Re-run targeted RTC tests**

Run: `cargo test -p im-call-runtime --offline`
Run: `cargo test -p local-minimal-node --offline`
Expected: new regression coverage passes.

### Task 3: Document Hardened Behavior

**Files:**
- Create: `docs/架构/38-RTC会话状态机与幂等更新标准-2026-04-05.md`
- Create: `docs/部署/生命周期脚本与运行目录标准-2026-04-05.md`

- [ ] **Step 1: Write RTC session state-machine standard**

Document valid transitions, idempotent retry rules, and conflict/error code expectations.

- [ ] **Step 2: Write deployment lifecycle standard**

Document runtime directory layout, required scripts, health checks, logs, pid handling, and OS-specific entrypoints.

### Task 4: Broad Verification

**Files:**
- Verify: entire workspace

- [ ] **Step 1: Format**

Run: `cargo fmt --all`
Expected: no formatting drift remains.

- [ ] **Step 2: Run workspace regression**

Run: `cargo test --workspace --offline`
Expected: all packages pass.

- [ ] **Step 3: Summarize findings and next wave**

Deliver findings first, ordered by severity, with file references, then record remaining risks and the next review target.

