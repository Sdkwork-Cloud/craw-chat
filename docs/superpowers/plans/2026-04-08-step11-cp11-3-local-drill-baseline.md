# Step 11 CP11-3 Local Drill Baseline Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Produce one repeatable local-minimal drill run for drain/rebalance, restore/recovery, and failover, then write the measured drill results into Step 11 review evidence.

**Architecture:** Add a machine-readable drill baseline under `tools/perf/`, and a focused `local-minimal-node` integration suite that reads the config, executes one HA/DR drill per scenario, and emits timing/results without inventing new infrastructure. Reuse the existing route-drain, runtime-restore, and cross-node resume-takeover assets as the canonical local drill surface.

**Tech Stack:** Rust integration tests, Axum in-process app execution, local runtime-dir restore helpers, serde_json, Markdown review docs.

---

### Task 1: Lock the CP11-3 drill contract with failing tests

**Files:**
- Create: `services/local-minimal-node/tests/performance_ha_dr_drill_test.rs`
- Test: `cargo test -p local-minimal-node --offline --test performance_ha_dr_drill_test`

- [ ] **Step 1: Write the failing contract and drill tests**

Cover:
- `tools/perf/step-11-cp11-3-local-drill-baseline.json` exists
- `docs/部署/性能与灾备演练场景.md` points to that drill baseline
- `drain-rebalance` drill emits metrics
- `restore-recovery` drill emits metrics
- `failover` drill emits metrics

- [ ] **Step 2: Run the suite to verify it fails**

Run: `cargo test -p local-minimal-node --offline --test performance_ha_dr_drill_test`
Expected: FAIL because the local drill baseline config and doc wiring do not exist yet.

### Task 2: Implement the repeatable local drill baseline

**Files:**
- Create: `tools/perf/step-11-cp11-3-local-drill-baseline.json`
- Modify: `tools/perf/step-11-scenario-catalog.json`
- Modify: `docs/部署/性能与灾备演练场景.md`

- [ ] **Step 1: Add the machine-readable local drill baseline**

Freeze:
- profile and tier
- drain/rebalance route count
- restore expected restored-file count
- failover takeover path

- [ ] **Step 2: Wire the operator doc and catalog to the local drill baseline**

Document:
- where the drill baseline lives
- which test executes it
- how it relates to later pre-release HA/DR drills

- [ ] **Step 3: Re-run the drill suite**

Run: `cargo test -p local-minimal-node --offline --test performance_ha_dr_drill_test -- --nocapture`
Expected: PASS and print drill metrics for all three scenarios.

### Task 3: Write Step 11 drill review evidence

**Files:**
- Create: `docs/review/step-11-cp11-3-高可用与恢复演练基线-执行卡-2026-04-08.md`
- Create: `docs/review/step-11-cp11-3-高可用与恢复演练基线-架构兑现与回写决议-2026-04-08.md`
- Create: `docs/review/step-11-cp11-3-高可用与恢复演练基线-质量审计与复盘-2026-04-08.md`
- Create: `docs/review/step-11-故障恢复复盘-2026-04-08.md`
- Modify: `docs/review/step-11-执行卡-2026-04-08.md`

- [ ] **Step 1: Capture the measured drill results**

Record:
- drain/rebalance duration and migrated route count
- restore preview/restore duration and restored-file count
- failover takeover duration and stale-owner rejection result

- [ ] **Step 2: Update the step-level execution card**

Reflect:
- `CP11-3` complete
- Step 11 still open only for report收口 and any remaining upgrade/rollback gap

### Task 4: Backwrite focused architecture evidence and verify

**Files:**
- Modify: `docs/架构/09-实施计划.md`
- Modify: `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md`
- Modify: `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- Modify: `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- Modify: `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`

- [ ] **Step 1: Append focused as-built entries**

Backwrite:
- Step 11 local drill baseline into `09`
- failover/drain route evidence into `131`
- local-minimal drill evidence into `137`
- HA/DR drill conclusions into `138`
- drill metric output into `140`

- [ ] **Step 2: Re-run focused verification**

Run:
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test performance_ha_dr_drill_test -- --nocapture`

Expected:
- PASS
- `CP11-3` closed
- next move is `CP11-4`
